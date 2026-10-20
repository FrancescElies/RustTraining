//! Ch09 Error handling (book: ch09-*.md)
//! Goals: Result, `?`, custom error enum, Display, From for `?` conversion.
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Missing(String),          // key not found
    BadNumber(ParseIntError), // value not an integer
}

// Display: "missing key: x" / "bad number: <inner>"
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
// From makes `?` convert ParseIntError automatically
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        todo!()
    }
}

/// Parse "k1=v1;k2=v2" and return the integer for `key`.
/// Missing key -> Missing(key). Non-integer -> BadNumber via `?`.
pub fn get_int(cfg: &str, key: &str) -> Result<i64, ConfigError> {
    todo!()
}

/// Sum of get_int for each key; first error wins.
pub fn sum_keys(cfg: &str, keys: &[&str]) -> Result<i64, ConfigError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch09_ok() {
        assert_eq!(get_int("a=1;b=22", "b"), Ok(22));
    }
    #[test]
    fn ch09_missing() {
        assert_eq!(get_int("a=1", "z"), Err(ConfigError::Missing("z".into())));
    }
    #[test]
    fn ch09_bad() {
        assert!(matches!(
            get_int("a=x", "a"),
            Err(ConfigError::BadNumber(_))
        ));
    }
    #[test]
    fn ch09_sum() {
        assert_eq!(sum_keys("a=1;b=2", &["a", "b"]), Ok(3));
        assert!(sum_keys("a=1", &["a", "q"]).is_err());
    }
    #[test]
    fn ch09_display() {
        assert_eq!(
            ConfigError::Missing("k".into()).to_string(),
            "missing key: k"
        );
    }
}
