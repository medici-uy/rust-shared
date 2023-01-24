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
