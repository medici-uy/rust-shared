use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::helpers::format_text;
use crate::traits::Hashable;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct QuestionOptionData {
    pub id: Uuid,

    pub question_id: Uuid,
    pub text: String,
    pub correct: bool,

    pub hash: String,
}

impl QuestionOptionData {
    pub fn new(id: Uuid, question_id: Uuid, text: String, correct: bool) -> Result<Self> {
        let mut data = Self {
            id,
            question_id,
            text,
            correct,
            hash: Default::default(),
        };

        data.format();
        data.check()?;

        data.hash = data.hash();

        Ok(data)
    }

    pub fn is_blank(&self) -> bool {
        self.text.is_empty()
    }

    pub fn eq_data(&self, other: &Self) -> bool {
        self.question_id == other.question_id
            && self.text == other.text
            && self.correct == other.correct
    }

    fn check(&self) -> Result<()> {
        if self.id == self.question_id {
            bail!("invalid question option with ID {}", self.id);
        }

        Ok(())
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
        bytes.push(self.correct as u8);

        bytes
    }
}

impl std::fmt::Display for QuestionOptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let data =
            QuestionOptionData::new(Uuid::new_v4(), Uuid::new_v4(), "  option  1 ".into(), true)
                .unwrap();

        assert_eq!(data.text, "option 1.");
    }

    #[test]
    fn test_hash() {
        let id = Uuid::new_v4();
        let question_id = Uuid::new_v4();

        let data_1 = QuestionOptionData::new(id, question_id, "opt 1".into(), false).unwrap();

        let data_2 = QuestionOptionData::new(id, question_id, "opt 2".into(), false).unwrap();

        assert_ne!(data_1.hash, data_2.hash);
    }
}
