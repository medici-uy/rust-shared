use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::{bail, Result};
use chrono::NaiveDate;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::helpers::{format_text, full_image_path};
use super::traits::Hashable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CourseData {
    pub key: String,

    pub name: String,
    pub short_name: String,
    pub price_in_uyu: Option<Decimal>,
    pub tags: Vec<String>,
    pub image_file_name: Option<PathBuf>,
    pub year: Option<i16>,
    pub order: Option<i16>,
    #[serde(skip)]
    pub questions: Vec<QuestionData>,
    #[serde(skip)]
    pub evaluations: Vec<CourseEvaluationData>,

    pub hash: String,
}

impl CourseData {
    pub fn new(
        key: String,
        name: String,
        short_name: String,
        price_in_uyu: Option<Decimal>,
        tags: Vec<String>,
        image_file_name: Option<PathBuf>,
        year: Option<i16>,
        order: Option<i16>,
        questions: Vec<QuestionData>,
        evaluations: Vec<CourseEvaluationData>,
    ) -> Self {
        Self {
            key,
            name,
            short_name,
            price_in_uyu,
            tags,
            image_file_name,
            year,
            order,
            questions,
            evaluations,
            hash: Default::default(),
        }
    }

    pub fn process(&mut self) -> Result<()> {
        self.clean();
        self.format();
        self.sort();
        self.deduplicate();
        self.check()?;
        self.set_hash();

        Ok(())
    }

    fn sort(&mut self) {
        self.questions
            .sort_by(|a, b| match a.evaluation.cmp(&b.evaluation) {
                Ordering::Equal => match a.asked_at.cmp(&b.asked_at) {
                    Ordering::Equal => match a.text.cmp(&b.text) {
                        Ordering::Equal => a.id.cmp(&b.id),
                        ordering => ordering,
                    },
                    ordering => ordering,
                },
                ordering => ordering,
            });

        for question in &mut self.questions {
            question.sort_options();
        }
    }

    fn clean(&mut self) {
        for question in &mut self.questions {
            question.clean();
        }
    }

    fn deduplicate(&mut self) {
        self.questions.dedup_by(|a, b| a.eq_data(b));

        for question in &mut self.questions {
            question.deduplicate_options();
        }
    }

    fn check(&self) -> Result<()> {
        for question in &self.questions {
            question.check()?;
        }

        Ok(())
    }

    fn format(&mut self) {
        for question in &mut self.questions {
            question.format();
        }
    }

    pub fn full_image_path(&self) -> Option<String> {
        Some(full_image_path(&self.key, self.image_file_name.as_ref()?))
    }
}

impl Hashable for CourseData {
    fn hashable_data(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.key.as_bytes());
        bytes.extend(self.name.as_bytes());
        bytes.extend(self.short_name.as_bytes());

        if let Some(price_in_uyu) = &self.price_in_uyu {
            bytes.extend(price_in_uyu.to_string().as_bytes());
        }

        bytes.extend(self.tags.join(",").as_bytes());

        if let Some(image_file_name) = &self.image_file_name {
            bytes.extend(image_file_name.to_string_lossy().as_bytes());
        }

        if let Some(year) = self.year {
            bytes.extend(&year.to_be_bytes());
        }

        if let Some(order) = self.order {
            bytes.extend(&order.to_be_bytes());
        }

        bytes.extend(
            self.questions
                .iter()
                .flat_map(|question| question.hash.as_bytes()),
        );
        bytes.extend(
            self.evaluations
                .iter()
                .flat_map(|evaluation| evaluation.hash.as_bytes()),
        );

        bytes
    }

    fn set_hash(&mut self) {
        for question in &mut self.questions {
            question.set_hash();
        }

        for evaluation in &mut self.evaluations {
            evaluation.set_hash();
        }

        self.hash = self.hash_data();
    }
}

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

    fn sort_options(&mut self) {
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

    fn clean(&mut self) {
        self.remove_empty_options();
    }

    fn deduplicate_options(&mut self) {
        self.question_options.dedup_by(|a, b| a.eq_data(b));
    }

    fn remove_empty_options(&mut self) {
        self.question_options
            .retain(|question_option| !question_option.text.is_empty());
    }

    fn eq_data(&self, other: &Self) -> bool {
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

    fn format(&mut self) {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuestionOptionData {
    pub id: Uuid,

    pub question_id: Uuid,
    pub text: String,
    pub correct: bool,
    pub explanation: Option<String>,

    pub hash: String,
}

impl QuestionOptionData {
    pub fn new(
        id: Uuid,
        question_id: Uuid,
        text: String,
        correct: bool,
        explanation: Option<String>,
    ) -> Self {
        Self {
            id,
            question_id,
            text,
            correct,
            explanation,
            hash: Default::default(),
        }
    }

    fn eq_data(&self, other: &Self) -> bool {
        self.text == other.text
            && self.correct == other.correct
            && self.explanation == other.explanation
    }

    fn format(&mut self) {
        self.text = format_text(&self.text);
        self.ensure_text_ends_with_period();
    }

    fn ensure_text_ends_with_period(&mut self) {
        const PERIOD: char = '.';

        if !self.text.ends_with(PERIOD) {
            self.text.push(PERIOD);
        }
    }
}

impl Hashable for QuestionOptionData {
    fn hashable_data(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.id.as_bytes());
        bytes.extend(self.question_id.as_bytes());
        bytes.extend(self.text.as_bytes());
        bytes.extend(&[self.correct as u8]);

        if let Some(explanation) = &self.explanation {
            bytes.extend(explanation.as_bytes());
        }

        bytes
    }

    fn set_hash(&mut self) {
        self.hash = self.hash_data();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct CourseEvaluationData {
    pub key: String,

    pub course_key: String,
    pub name: String,
    pub order: i16,

    pub hash: String,
}

impl CourseEvaluationData {
    pub fn new(key: String, course_key: String, name: String, order: i16) -> Self {
        Self {
            key,
            course_key,
            name,
            order,
            hash: Default::default(),
        }
    }

    pub fn full_key(&self) -> String {
        Self::do_full_key(&self.course_key, &self.key)
    }

    pub fn do_full_key(course_key: &str, key: &str) -> String {
        format!("{}{COURSE_EVALUATION_KEY_SEPARATOR}{}", course_key, key)
    }
}

impl Hashable for CourseEvaluationData {
    fn hashable_data(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.course_key.as_bytes());
        bytes.extend(self.name.as_bytes());
        bytes.extend(self.order.to_be_bytes());

        bytes
    }

    fn set_hash(&mut self) {
        self.hash = self.hash_data();
    }
}

pub const COURSE_EVALUATION_KEY_SEPARATOR: &str = "/";
