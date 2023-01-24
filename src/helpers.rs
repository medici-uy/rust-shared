use std::path::Path;

use once_cell::sync::Lazy;
use regex::Regex;

static WHITESPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s\s+").unwrap());
static WHITESPACE_BEFORE_END_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s(\.|:|\?)$").unwrap());

pub fn format_text(text: &str) -> String {
    let mut formatted = text.trim().to_owned();

    formatted = WHITESPACE_REGEX.replace_all(&formatted, " ").to_string();
    formatted = WHITESPACE_BEFORE_END_REGEX
        .replace(&formatted, "$1")
        .to_string();

    formatted
}

pub fn full_image_path<P>(course_key: &str, image_file_name: P) -> String
where
    P: AsRef<Path>,
{
    format!(
        "{course_key}/{}",
        image_file_name
            .as_ref()
            .as_os_str()
            .to_str()
            .expect("invalid image file name")
    )
}
