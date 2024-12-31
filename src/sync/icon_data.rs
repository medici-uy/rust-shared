use anyhow::{bail, Result};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{
    helpers::{format_text, full_image_path},
    ICON_IMAGES_DIR_NAME,
};
use crate::traits::Hashable;

#[non_exhaustive]
#[derive(medici_macros::Hashable, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct IconData {
    pub key: String,

    pub is_initial: bool,
    pub description: Option<String>,
    pub price_in_uyu: Option<Decimal>,
    pub image_file_name: PathBuf,

    pub hash: String,
}

impl IconData {
    pub fn new(
        key: String,
        is_initial: bool,
        description: Option<String>,
        price_in_uyu: Option<Decimal>,
        image_file_name: PathBuf,
    ) -> Result<Self> {
        let mut data = Self {
            key,
            is_initial,
            description,
            price_in_uyu,
            image_file_name,
            hash: Default::default(),
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
        if let Some(price_in_uyu) = self.price_in_uyu {
            if price_in_uyu <= Decimal::ZERO {
                bail!("invalid icon price");
            }
        }

        Ok(())
    }

    fn format(&mut self) {
        self.key = self.key.trim().to_string();

        self.description = self.description.as_deref().map(format_text);
    }

    pub fn full_image_path(&self) -> String {
        full_image_path(&ICON_IMAGES_DIR_NAME, &self.image_file_name)
    }
}
