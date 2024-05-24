use std::cmp::Ordering;
use std::collections::HashSet;
use std::num::NonZeroU16;
use std::path::PathBuf;

use anyhow::{bail, Result};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

use super::helpers::full_image_path;
use super::question_data::QuestionData;
use super::question_source_data::QuestionSourceData;
use super::question_topic_data::QuestionTopicData;
use crate::traits::Hashable;

#[non_exhaustive]
#[derive(medici_macros::Hashable, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct CourseData {
    pub key: String,

    pub name: String,
    pub short_name: String,
    pub description: Option<String>,
    pub price_in_uyu: Option<Decimal>,
    pub tags: Vec<String>,
    pub image_file_name: Option<PathBuf>,
    pub year: Option<u16>,
    pub order: Option<u16>,
    pub questions_per_test: Option<NonZeroU16>,
    #[serde(skip)]
    pub questions: Vec<QuestionData>,
    #[serde(skip)]
    pub topics: Vec<String>,

    pub hash: String,
}

impl CourseData {
    pub fn new(
        key: String,
        name: String,
        short_name: String,
        description: Option<String>,
        price_in_uyu: Option<Decimal>,
        tags: Vec<String>,
        image_file_name: Option<PathBuf>,
        year: Option<u16>,
        order: Option<u16>,
        questions_per_test: Option<NonZeroU16>,
        questions: Vec<QuestionData>,
        topics: Vec<String>,
    ) -> Result<Self> {
        let mut data = Self {
            key,
            name,
            short_name,
            description,
            price_in_uyu,
            tags,
            image_file_name,
            year,
            order,
            questions_per_test,
            questions,
            topics,
            hash: Default::default(),
        };

        data.process()?;

        Ok(data)
    }

    pub fn process(&mut self) -> Result<()> {
        self.remove_blank_questions();
        self.format();
        self.sort();
        self.deduplicate();
        self.check()?;

        self.refresh_hash();

        Ok(())
    }

    pub fn replace_question(&mut self, new_question: QuestionData) -> Result<()> {
        let old_question = self
            .questions
            .iter_mut()
            .find(|question| question.id == new_question.id)
            .expect("question to replace not found");

        *old_question = new_question;

        self.process()
    }

    fn sort(&mut self) {
        self.questions
            .sort_by(|a, b| match a.source.r#type.cmp(&b.source.r#type) {
                Ordering::Equal => match a.source.date.cmp(&b.source.date) {
                    Ordering::Equal => match a.text.cmp(&b.text) {
                        Ordering::Equal => a.id.cmp(&b.id),
                        ordering => ordering,
                    },
                    ordering => ordering,
                },
                ordering => ordering,
            });
    }

    fn remove_blank_questions(&mut self) {
        self.questions.retain(|question| !question.is_blank());
    }

    fn deduplicate(&mut self) {
        self.questions.dedup_by(|a, b| a.eq_data(b));
    }

    fn check(&self) -> Result<()> {
        if self.key.is_empty() || self.name.is_empty() || self.short_name.is_empty() {
            bail!("invalid course with key {}", self.key);
        }

        Ok(())
    }

    fn format(&mut self) {
        self.name = self.name.trim().into();
        self.short_name = self.short_name.trim().into();
        self.description = self
            .description
            .as_ref()
            .map(|description| description.trim().into());
        self.tags = self.tags.iter().map(|tag| tag.trim().into()).collect();
    }

    pub fn full_image_path(&self) -> Option<String> {
        Some(full_image_path(&self.key, self.image_file_name.as_ref()?))
    }

    pub fn question_topics(&self) -> HashSet<QuestionTopicData> {
        HashSet::from_iter(self.questions.iter().map(|question| question.topic.clone()))
    }

    pub fn question_sources(&self) -> HashSet<QuestionSourceData> {
        HashSet::from_iter(
            self.questions
                .iter()
                .map(|question| question.source.clone()),
        )
    }
}
