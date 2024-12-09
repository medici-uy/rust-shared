use anyhow::Result;
#[cfg(test)]
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{capitalize_first_char, helpers::format_text};
use crate::traits::Hashable;

#[non_exhaustive]
#[derive(medici_macros::Hashable, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(test, derive(Dummy))]
pub struct QuestionOptionData {
    pub id: Uuid,

    pub question_id: Uuid,
    pub text: String,
    #[cfg_attr(test, dummy(default))]
    pub correct: bool,
    #[medici(skip_hash)]
    pub reference: u16,
    #[medici(skip_hash)]
    #[cfg_attr(test, dummy(default))]
    pub preserve_case: bool,

    pub hash: String,
}

impl QuestionOptionData {
    pub fn new(
        id: Uuid,
        question_id: Uuid,
        text: String,
        correct: bool,
        reference: u16,
        preserve_case: bool,
    ) -> Result<Self> {
        let mut data = Self {
            id,
            question_id,
            text,
            correct,
            hash: Default::default(),
            reference,
            preserve_case,
        };

        data.process()?;

        Ok(data)
    }

    pub fn process(&mut self) -> Result<()> {
        self.format();
        self.check()?;

        self.refresh_hash();

        Ok(())
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
        Ok(())
    }

    fn format(&mut self) {
        self.text = format_text(&self.text);

        if !self.text.is_empty() {
            self.ensure_text_ends_with_period();
        }

        if !self.preserve_case {
            capitalize_first_char(&mut self.text);
        }
    }

    fn ensure_text_ends_with_period(&mut self) {
        const PERIOD: char = '.';

        if !self.text.ends_with(PERIOD) {
            self.text.push(PERIOD);
        }
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
    fn test_process() {
        let mut data: QuestionOptionData = Faker.fake();
        data.text = "  option  1  ".into();
        data.process().unwrap();

        assert_eq!(data.text, "Option 1.");
    }

    #[test]
    fn test_process_preserve_case() {
        let mut data: QuestionOptionData = Faker.fake();
        data.text = "o".into();
        data.preserve_case = true;
        data.process().unwrap();

        assert_eq!(data.text, "o.");
    }

    #[test]
    fn test_hash() {
        let mut data1: QuestionOptionData = Faker.fake();
        data1.process().unwrap();

        let mut data2: QuestionOptionData = Faker.fake();
        data2.process().unwrap();

        assert_ne!(data1.hash, data2.hash);
    }
}
