use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::helpers::{format_text, remove_end_period};
use crate::traits::Hashable;

#[non_exhaustive]
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct QuestionTopicData {
    pub course_key: String,
    pub name: String,
}

impl QuestionTopicData {
    pub const KEY_SEPARATOR: &'static str = "::";
    pub const DEFAULT_NAME: &'static str = "_";

    pub fn new(course_key: String, name: String) -> Result<Self> {
        let mut data = Self { course_key, name };

        data.format();

        Ok(data)
    }

    pub fn key(&self) -> String {
        format!("{}{}{}", self.course_key, Self::KEY_SEPARATOR, self.name)
    }

    pub fn is_blank(&self) -> bool {
        self.name.is_empty()
    }

    pub fn is_default(&self) -> bool {
        Self::is_default_topic_name(&self.name)
    }

    fn format(&mut self) {
        self.name = remove_end_period(&format_text(&self.name));
    }

    pub fn is_default_topic_name(name: &str) -> bool {
        name == Self::DEFAULT_NAME
    }
}

impl Hashable for QuestionTopicData {
    fn to_bytes(&self) -> Vec<u8> {
        self.key().to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let data = QuestionTopicData::new("test".into(), "topic  1.".into()).unwrap();

        assert_eq!(data.name, "topic 1");
    }
}
