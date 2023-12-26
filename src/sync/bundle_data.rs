use anyhow::{bail, Result};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{helpers::full_image_path, BUNDLE_IMAGES_DIR_NAME};
use crate::traits::Hashable;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct BundleData {
    pub key: String,

    pub name: String,
    pub course_keys: Vec<String>,
    pub discount: Decimal,
    pub image_file_name: Option<PathBuf>,

    pub hash: String,
}

impl BundleData {
    pub fn new(
        key: String,
        name: String,
        course_keys: Vec<String>,
        discount: Decimal,
        image_file_name: Option<PathBuf>,
    ) -> Result<Self> {
        let mut data = Self {
            key,
            name,
            course_keys,
            discount,
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
        if self.name.is_empty() {
            bail!("invalid bundle name");
        }

        if self.discount <= Decimal::ZERO {
            bail!("invalid bundle discount");
        }

        Ok(())
    }

    fn format(&mut self) {
        self.key = self.key.trim().to_string();
        self.name = self.name.trim().to_string();
    }

    pub fn full_image_path(&self) -> Option<String> {
        Some(full_image_path(
            BUNDLE_IMAGES_DIR_NAME,
            self.image_file_name.as_ref()?,
        ))
    }
}

impl Hashable for BundleData {
    fn hashable_data(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.name.as_bytes());
        bytes.extend(self.course_keys.iter().flat_map(|key| key.as_bytes()));
        bytes.extend(format!("discount {}", self.discount).as_bytes());

        if let Some(image_file_name) = &self.image_file_name {
            bytes.extend(
                format!("image_file_name {}", image_file_name.to_string_lossy()).as_bytes(),
            );
        }

        bytes
    }

    fn refresh_hash(&mut self) {
        self.hash = self.hash();
    }
}
