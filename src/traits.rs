use std::{num::NonZeroU16, path::PathBuf};

use chrono::{DateTime, Utc};
use rust_decimal::prelude::*;
use serde::Serialize;
use uuid::Uuid;

pub trait Hashable {
    fn to_bytes(&self) -> Vec<u8>;

    fn hash(&self) -> String {
        if let Some(hash) = self.get_hash() {
            return hash.into();
        }

        self.compute_hash()
    }

    fn compute_hash(&self) -> String {
        blake3::hash(&self.to_bytes()).to_string()
    }

    fn refresh_hash(&mut self) {
        self.set_hash(self.compute_hash())
    }

    fn set_hash(&mut self, _hash: String) {
        panic!("cannot store hash in this type");
    }

    fn get_hash(&self) -> Option<&str> {
        None
    }
}

impl Hashable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().into()
    }
}

impl Hashable for Uuid {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().into()
    }
}

impl Hashable for DateTime<Utc> {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_rfc3339().to_bytes()
    }
}

impl<T: Hashable> Hashable for Option<T> {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_ref().map(|a| a.to_bytes()).unwrap_or_default()
    }
}

impl<T: Hashable> Hashable for Vec<T> {
    fn to_bytes(&self) -> Vec<u8> {
        self.iter().flat_map(|a| a.to_bytes()).collect()
    }
}

impl Hashable for Decimal {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().to_bytes()
    }
}

impl Hashable for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }
}

impl Hashable for bool {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().to_bytes()
    }
}

impl Hashable for NonZeroU16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.get().to_bytes()
    }
}

impl Hashable for PathBuf {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string_lossy().as_bytes().into()
    }
}

pub trait EmailTemplate: Serialize + Sized {
    const TEMPLATE_NAME: &'static str;

    fn data(&self) -> String {
        serde_json::to_string(self).expect("failed to serialize template data")
    }

    fn email_content(self) -> aws_sdk_sesv2::types::EmailContent {
        let template = aws_sdk_sesv2::types::Template::builder()
            .template_name(Self::TEMPLATE_NAME)
            .template_data(self.data())
            .build();

        aws_sdk_sesv2::types::EmailContent::builder()
            .template(template)
            .build()
    }
}
