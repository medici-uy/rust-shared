use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::course_evaluation_data::CourseEvaluationData;
use super::helpers::{format_text, full_image_path};
use super::question_option_data::QuestionOptionData;
use super::traits::Hashable;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuestionData {
    pub id: Uuid,

    pub course_key: String,
    pub evaluation: String,
    pub source: String,
    pub asked_at: Option<NaiveDate>,
    pub text: String,
    pub explanation: Option<String>,
    pub topic: Option<String>,
    pub tags: Vec<String>,
    pub image_file_name: Option<PathBuf>,
    #[serde(skip)]
    pub question_options: Vec<QuestionOptionData>,

    pub hash: String,
}

impl QuestionData {
    pub fn new(
        id: Uuid,
        course_key: String,
        text: String,
        explanation: Option<String>,
        topic: Option<String>,
        tags: Vec<String>,
        image_file_name: Option<PathBuf>,
        question_options: Vec<QuestionOptionData>,
        evaluation: String,
        source: String,
        asked_at: Option<NaiveDate>,
    ) -> Result<Self> {
        let mut data = Self {
            id,
            course_key,
            evaluation,
            source,
            asked_at,
            text,
            explanation,
            topic,
            tags,
            image_file_name,
            question_options,
            hash: Default::default(),
        };

        data.remove_blank_options();
        data.format();
        data.sort();
        data.deduplicate();
        data.check()?;

        data.hash = data.hash();

        Ok(data)
    }

    pub fn is_blank(&self) -> bool {
        self.text.is_empty() && self.question_options.is_empty()
    }

    fn sort(&mut self) {
        self.question_options.sort_by(|a, b| {
            if a.correct {
                Ordering::Less
            } else if b.correct {
                Ordering::Greater
            } else {
                a.text.cmp(&b.text)
            }
        })
    }

    fn deduplicate(&mut self) {
        self.question_options.dedup_by(|a, b| a.eq_data(b));
    }

    fn remove_blank_options(&mut self) {
        self.question_options
            .retain(|question_option| !question_option.is_blank());
    }

    pub fn eq_data(&self, other: &Self) -> bool {
        self.text == other.text
            && self.evaluation == other.evaluation
            && self.question_options.len() == other.question_options.len()
            && self
                .question_options
                .iter()
                .all(|a| other.question_options.iter().any(|b| a.eq_data(b)))
    }

    fn check(&self) -> Result<()> {
        self.check_question_option_count()?;
        self.check_duplicates_in_question_options()?;
        self.check_correct_count()?;

        Ok(())
    }

    fn check_question_option_count(&self) -> Result<()> {
        if !self.is_blank() && (self.question_options.len() < 2 || self.question_options.len() > 5)
        {
            bail!(
                "question with ID {} has {} option(s)",
                self.id,
                self.question_options.len()
            );
        }

        Ok(())
    }

    fn check_duplicates_in_question_options(&self) -> Result<()> {
        let texts_iter = self
            .question_options
            .iter()
            .map(|question_option| question_option.text.as_str());

        let mut texts_set = HashSet::<&str>::with_capacity(self.question_options.len());

        for text in texts_iter {
            if texts_set.contains(text) {
                bail!("duplicate question option. Text: \"{text}\"");
            } else {
                texts_set.insert(text);
            }
        }

        Ok(())
    }

    fn check_correct_count(&self) -> Result<()> {
        let correct_count = self
            .question_options
            .iter()
            .filter(|option| option.correct)
            .count();

        if !self.is_blank() && correct_count != 1 {
            bail!(
                "question with ID {} has {correct_count} correct options",
                self.id
            )
        }

        Ok(())
    }

    fn format(&mut self) {
        self.text = format_text(&self.text);

        self.explanation = self.explanation.as_mut().and_then(|original_explanation| {
            let explanation = original_explanation.trim().to_string();

            if explanation.is_empty() {
                None
            } else {
                Some(explanation)
            }
        });

        self.topic = self.topic.as_ref().map(|topic| topic.trim().to_string());

        self.tags = self
            .tags
            .iter()
            .map(|tag| tag.trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect();
    }

    pub fn full_evaluation_key(&self) -> String {
        CourseEvaluationData::do_full_key(&self.course_key, &self.evaluation)
    }

    pub fn full_image_path(&self) -> Option<String> {
        Some(full_image_path(
            &self.course_key,
            self.image_file_name.as_ref()?,
        ))
    }
}

impl Hashable for QuestionData {
    fn hashable_data(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.id.as_bytes());

        bytes.extend(self.course_key.as_bytes());
        bytes.extend(self.text.as_bytes());

        if let Some(explanation) = &self.explanation {
            bytes.extend(format!("explanation {explanation}").as_bytes());
        }

        if let Some(topic) = &self.topic {
            bytes.extend(format!("topic {topic}").as_bytes());
        }

        bytes.extend(self.tags.iter().flat_map(|tag| tag.as_bytes()));

        if let Some(image_file_name) = &self.image_file_name {
            bytes.extend(
                format!("image_file_name {}", image_file_name.to_string_lossy()).as_bytes(),
            );
        }

        bytes.extend(
            self.question_options
                .iter()
                .flat_map(|question_option| question_option.hash.as_bytes()),
        );

        bytes.extend(self.evaluation.as_bytes());
        bytes.extend(self.source.as_bytes());

        if let Some(asked_at) = self.asked_at {
            bytes.extend(format!("asked_at {}", asked_at).as_bytes());
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        let result = QuestionData::new(
            Uuid::new_v4(),
            "course".into(),
            "text".into(),
            None,
            None,
            vec![],
            None,
            vec![],
            "eva".into(),
            "source".into(),
            None,
        );

        assert!(result.is_err());
    }
}
