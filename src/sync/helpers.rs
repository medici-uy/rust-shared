use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

const UNITS_TO_SEPARATE: [&str; 1] = ["%"];

static WHITESPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s\s+").unwrap());
static WHITESPACE_BEFORE_END_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s(\.|:|\?)$").unwrap());
static DOUBLE_QUOTE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[“”]").unwrap());
static SPACE_BEFORE_PERCENT_SIGN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"(\d)({})", UNITS_TO_SEPARATE.join("|"))).unwrap());
static END_PERIOD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\.$").unwrap());

pub fn format_text(text: &str) -> String {
    let mut formatted = text.trim().to_owned();

    formatted = WHITESPACE_REGEX.replace_all(&formatted, " ").into();
    formatted = WHITESPACE_BEFORE_END_REGEX.replace(&formatted, "$1").into();
    formatted = DOUBLE_QUOTE_REGEX.replace_all(&formatted, "\"").into();
    formatted = SPACE_BEFORE_PERCENT_SIGN_REGEX
        .replace_all(&formatted, "$1 $2")
        .into();

    formatted
}

pub fn remove_end_period(text: &str) -> String {
    END_PERIOD_REGEX.replace(text, "").into()
}

pub fn capitalize_first_char(text: &mut str) {
    if let Some(char) = text.get_mut(0..1) {
        char.make_ascii_uppercase();
    }
}

pub fn full_image_path<P>(key: &str, image_file_name: P) -> String
where
    P: AsRef<Path>,
{
    format!(
        "{key}/{}",
        image_file_name
            .as_ref()
            .as_os_str()
            .to_str()
            .expect("invalid image file name")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_text() {
        assert_eq!(
            format_text(" test  “text”   12.34%  . "),
            "test \"text\" 12.34 %."
        );
    }
}
