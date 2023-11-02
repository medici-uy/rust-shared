use std::cmp::Ordering;
use std::path::PathBuf;

use anyhow::Result;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

use super::course_evaluation_data::CourseEvaluationData;
use super::helpers::full_image_path;
use super::question_data::QuestionData;
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
