use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct QuestionSourceData {
    pub course_key: String,
    pub r#type: QuestionSourceType,
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
}

impl QuestionSourceData {
    pub const KEY_SEPARATOR: &'static str = "::";
    pub const EMPTY_FIELD_KEY_VALUE: &'static str = "!";

    pub fn new(
        course_key: String,
        r#type: QuestionSourceType,
        name: Option<String>,
        date: Option<NaiveDate>,
    ) -> Result<Self> {
        let mut data = Self {
            course_key,
            r#type,
            name,
            date,
        };

        data.format();
        data.check()?;

        Ok(data)
    }

    pub fn key(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            self.course_key,
            Self::KEY_SEPARATOR,
            self.r#type,
            Self::KEY_SEPARATOR,
            self.name.as_deref().unwrap_or(Self::EMPTY_FIELD_KEY_VALUE),
            Self::KEY_SEPARATOR,
            self.date
                .map(|date| date.to_string())
                .unwrap_or(Self::EMPTY_FIELD_KEY_VALUE.into())
        )
    }

    fn check(&self) -> Result<()> {
        if (self.date.is_none()
            && (self.r#type == QuestionSourceType::Exam
                || self.r#type == QuestionSourceType::Partial))
            || (self.name.is_none() && self.r#type == QuestionSourceType::Partial)
        {
            bail!("invalid data in question source with key {}", self.key());
        }

        Ok(())
    }

    fn format(&mut self) {
        if let Some(name) = &self.name {
            self.name.replace(name.trim().into());
        }
    }
}

#[derive(
    sqlx::Type,
    strum::Display,
    Serialize,
    Deserialize,
    PartialEq,
    Hash,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Debug,
)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum QuestionSourceType {
    Exam,
    Partial,
    SelfAssessment,
    Other,
}
