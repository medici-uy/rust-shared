use anyhow::Result;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct QuestionTopicData {
    pub course_key: String,
    pub name: String,
}

impl QuestionTopicData {
    pub const KEY_SEPARATOR: &'static str = "::";

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

    fn format(&mut self) {
        self.name = self.name.trim().into();
    }
}
