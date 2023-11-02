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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuestionData {
    pub id: Uuid,

    pub course_key: String,
    pub evaluation: String,
    pub source: String,
    pub asked_at: Option<NaiveDate>,
    pub text: String,
    pub topic: Option<String>,
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
        topic: Option<String>,
        image_file_name: Option<PathBuf>,
        question_options: Vec<QuestionOptionData>,
        evaluation: String,
        source: String,
        asked_at: Option<NaiveDate>,
    ) -> Self {
        Self {
            id,
            course_key,
            evaluation,
            source,
            asked_at,
            text,
            topic,
            image_file_name,
            question_options,
            hash: Default::default(),
        }
    }

    pub fn sort_options(&mut self) {
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

    pub fn clean(&mut self) {
        self.remove_empty_options();
    }

    pub fn deduplicate_options(&mut self) {
        self.question_options.dedup_by(|a, b| a.eq_data(b));
    }

    fn remove_empty_options(&mut self) {
        self.question_options
            .retain(|question_option| !question_option.text.is_empty());
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

    pub fn check(&self) -> Result<()> {
        self.check_question_option_count()?;
        self.check_duplicates_in_question_options()?;
        self.check_correct_count()?;

        Ok(())
    }

    fn check_question_option_count(&self) -> Result<()> {
        if self.question_options.len() < 2 || self.question_options.len() > 5 {
            bail!(
                "Question {} has {} option(s)",
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
                bail!("Duplicate question option. Text: \"{text}\"");
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

        if correct_count != 1 {
            bail!("Question {} has {correct_count} correct options", self.id)
        }

        Ok(())
    }

    pub fn format(&mut self) {
        self.text = format_text(&self.text);

        self.topic = self.topic.as_ref().map(|topic| topic.trim().to_string());

        for question_option in self.question_options.iter_mut() {
            question_option.format();
        }
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

        if let Some(topic) = &self.topic {
            bytes.extend(topic.as_bytes());
        }

        if let Some(image_file_name) = &self.image_file_name {
            bytes.extend(image_file_name.to_string_lossy().as_bytes());
        }

        bytes.extend(
            self.question_options
                .iter()
                .flat_map(|question_option| question_option.hash.as_bytes()),
        );

        bytes.extend(self.evaluation.as_bytes());
        bytes.extend(self.source.as_bytes());

        if let Some(asked_at) = self.asked_at {
            bytes.extend(asked_at.to_string().as_bytes());
        }

        bytes
    }

    fn set_hash(&mut self) {
        for question_option in &mut self.question_options {
            question_option.set_hash();
        }

        self.hash = self.hash_data();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        let data = QuestionData::new(
            Uuid::new_v4(),
            "course".into(),
            "text".into(),
            None,
            None,
            vec![],
            "eva".into(),
            "source".into(),
            None,
        );

        assert!(data.check().is_err());
    }
}
