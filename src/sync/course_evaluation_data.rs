use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::traits::Hashable;

#[non_exhaustive]
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct CourseEvaluationData {
    pub key: String,

    pub course_key: String,
    pub name: String,
    pub order: i16,

    pub hash: String,
}

impl CourseEvaluationData {
    pub fn new(key: String, course_key: String, name: String, order: i16) -> Result<Self> {
        let mut data = Self {
            key,
            course_key,
            name,
            order,
            hash: Default::default(),
        };

        data.hash = data.hash();

        Ok(data)
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
}

pub const COURSE_EVALUATION_KEY_SEPARATOR: &str = "/";
