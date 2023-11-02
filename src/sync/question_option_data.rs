use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::helpers::format_text;
use super::traits::Hashable;

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

    pub fn eq_data(&self, other: &Self) -> bool {
        self.text == other.text
            && self.correct == other.correct
            && self.explanation == other.explanation
    }

    pub fn format(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let text = "  option  1 ".to_string();
        let mut data = QuestionOptionData::new(Uuid::new_v4(), Uuid::new_v4(), text, true, None);

        data.format();

        assert_eq!(data.text, "option 1.");
    }

    #[test]
    fn test_hash() {
        let id = Uuid::new_v4();
        let question_id = Uuid::new_v4();

        let mut data_before =
            QuestionOptionData::new(id, question_id, "opt old".into(), false, None);

        data_before.format();
        data_before.set_hash();

        let mut data_after = data_before.clone();
        data_after.text = "opt new".into();

        data_after.format();
        data_after.set_hash();

        assert_ne!(data_before.hash, data_after.hash);
    }
}
