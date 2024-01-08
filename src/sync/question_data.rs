use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::{bail, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::explanation_data::ExplanationData;
use super::helpers::{format_text, full_image_path};
use super::question_option_data::QuestionOptionData;
use super::question_source_data::QuestionSourceData;
use super::question_topic_data::QuestionTopicData;
use crate::traits::Hashable;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct QuestionData {
    pub id: Uuid,

    pub course_key: String,
    pub source: QuestionSourceData,
    pub text: String,
    pub explanation: Option<ExplanationData>,
    pub topic: QuestionTopicData,
    pub topic_by: Option<String>,
    pub tags: Vec<String>,
    pub image_file_name: Option<PathBuf>,
    #[serde(skip)]
    pub question_options: Vec<QuestionOptionData>,

    pub hash: String,
}

impl QuestionData {
    pub const TOPIC_KEY_SEPARATOR: &'static str = "::";

    pub fn new(
        id: Uuid,
        course_key: String,
        text: String,
        explanation: Option<ExplanationData>,
        topic: String,
        topic_by: Option<String>,
        tags: Vec<String>,
        image_file_name: Option<PathBuf>,
        question_options: Vec<QuestionOptionData>,
        source: QuestionSourceData,
    ) -> Result<Self> {
        let mut data = Self {
            id,
            course_key: course_key.clone(),
            source,
            text,
            explanation,
            topic: QuestionTopicData::new(course_key, topic)?,
            topic_by,
            tags,
            image_file_name,
            question_options,
            hash: Default::default(),
        };

        data.process()?;

        Ok(data)
    }

    pub fn is_blank(&self) -> bool {
        self.text.is_empty() && self.question_options.is_empty()
    }

    pub fn process(&mut self) -> Result<()> {
        self.remove_blank_options();
        self.format();
        self.sort();
        self.deduplicate();
        self.check()?;

        self.refresh_hash();

        Ok(())
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
            && self.source == other.source
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
                bail!("duplicate question option. {self:?}");
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

        self.tags = self
            .tags
            .iter()
            .map(|tag| tag.trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect();
    }

    pub fn topic_key(&self) -> String {
        self.topic.key()
    }

    pub fn source_key(&self) -> String {
        self.source.key()
    }

    pub fn set_topic(&mut self, name: String, topic_by: Option<String>) -> Result<()> {
        self.topic = QuestionTopicData::new(self.course_key.clone(), name)?;
        self.topic_by = topic_by;

        self.process()
    }

    pub fn set_explanation(&mut self, text: String, by: String) -> Result<()> {
        self.explanation
            .replace(ExplanationData::new(text, by, Utc::now())?);

        self.process()
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

        bytes.extend(self.course_key.as_bytes());
        bytes.extend(self.text.as_bytes());

        if let Some(explanation) = &self.explanation {
            bytes.extend(format!("explanation {}", explanation.hash).as_bytes());
        }

        bytes.extend(self.topic_key().as_bytes());

        if let Some(topic_by) = &self.topic_by {
            bytes.extend(format!("topic_by {topic_by}").as_bytes());
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

        bytes.extend(self.source_key().as_bytes());

        bytes
    }

    fn refresh_hash(&mut self) {
        self.hash = self.hash();
    }
}

impl std::fmt::Display for QuestionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let question_options_string = self.question_options.iter().enumerate().fold(
            String::new(),
            |acc, (index, question_option)| {
                format!("{acc}\n{}. {question_option}", (97 + index as u8) as char)
            },
        );

        write!(f, "{}\n{}", self.text, question_options_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::QuestionSourceType;

    use super::*;

    #[test]
    fn test_check() {
        let course_key = "course".to_string();

        let result = QuestionData::new(
            Uuid::new_v4(),
            course_key.clone(),
            "text".into(),
            None,
            "topic".into(),
            None,
            vec![],
            None,
            vec![],
            QuestionSourceData::new(
                course_key.clone(),
                QuestionSourceType::SelfAssessment,
                None,
                None,
            )
            .unwrap(),
        );

        assert!(result.is_err());
    }
}
