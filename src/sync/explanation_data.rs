use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::traits::Hashable;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct ExplanationData {
    pub text: String,
    pub by: String,
    pub date: DateTime<Utc>,

    pub hash: String,
}

impl ExplanationData {
    pub fn new(text: String, explained_by: String, explained_at: DateTime<Utc>) -> Result<Self> {
        let mut data = Self {
            text,
            by: explained_by,
            date: explained_at,
            hash: Default::default(),
        };

        data.format();
        data.check()?;

        data.refresh_hash();

        Ok(data)
    }

    fn check(&self) -> Result<()> {
        if self.text.is_empty() || self.by.is_empty() {
            bail!("invalid explanation");
        }

        Ok(())
    }

    fn format(&mut self) {
        self.text = self.text.trim().to_string();
        self.by = self.by.trim().to_string();
    }
}

impl Hashable for ExplanationData {
    fn hashable_data(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.text.as_bytes());
        bytes.extend(self.by.as_bytes());
        bytes.extend(self.date.to_rfc3339().as_bytes());

        bytes
    }

    fn refresh_hash(&mut self) {
        self.hash = self.hash();
    }
}
