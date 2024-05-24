use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::traits::Hashable;

#[non_exhaustive]
#[derive(medici_macros::Hashable, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct ExplanationData {
    pub text: String,
    pub by: String,
    pub date: DateTime<Utc>,

    pub hash: String,
    #[serde(skip)]
    pub _bytes: Vec<u8>,
}

impl ExplanationData {
    pub fn new(text: String, by: String, date: DateTime<Utc>) -> Result<Self> {
        let mut data = Self {
            text,
            by,
            date,
            hash: Default::default(),
            _bytes: Default::default(),
        };

        data.process()?;

        Ok(data)
    }

    fn process(&mut self) -> Result<()> {
        self.format();
        self.check()?;

        self.refresh_hash();

        Ok(())
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
