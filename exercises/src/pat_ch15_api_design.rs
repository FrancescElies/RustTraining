//! Patterns ch15 Crate architecture & API design (book: rust-patterns-book/src/ch15-*.md)
//! Ergonomic params (Into/AsRef), parse-don't-validate, FromStr/TryFrom.
use std::convert::TryFrom;
use std::path::Path;
use std::str::FromStr;

/// Accept &str, String, or &String without the caller converting. "Hello, <name>!"
pub fn greet(name: impl Into<String>) -> String {
    todo!()
}

/// Accept a slice of anything string-like. join with "/".
pub fn join_paths<P: AsRef<str>>(parts: &[P]) -> String {
    todo!()
}

/// Accept &str, String, PathBuf, &Path: file extension if any. (AsRef<Path>, Path::extension, to_str)
pub fn extension<P: AsRef<Path>>(p: P) -> Option<String> {
    todo!()
}

/// A validated percentage: 0..=100 and nothing else can exist.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percent(u8);
impl Percent {
    pub fn get(self) -> u8 {
        self.0
    }
}
impl TryFrom<i32> for Percent {
    type Error = String;
    fn try_from(v: i32) -> Result<Self, String> {
        todo!()
    }
}
/// "50%" or "50" -> Percent. Reuse TryFrom. Garbage -> Err.
impl FromStr for Percent {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn pat15_greet() {
        let s = String::from("B");
        assert_eq!(
            (greet("A"), greet(s.clone()), greet(s)),
            ("Hello, A!".into(), "Hello, B!".into(), "Hello, B!".into())
        );
    }
    #[test]
    fn pat15_join() {
        assert_eq!(join_paths(&["a", "b"]), "a/b");
        assert_eq!(join_paths(&[String::from("x")]), "x");
    }
    #[test]
    fn pat15_ext() {
        assert_eq!(extension("a/b.rs"), Some("rs".into()));
        assert_eq!(extension(PathBuf::from("c")), None);
    }
    #[test]
    fn pat15_percent() {
        assert_eq!(Percent::try_from(50).unwrap().get(), 50);
        assert!(Percent::try_from(101).is_err());
        assert!(Percent::try_from(-1).is_err());
        assert_eq!("75%".parse::<Percent>(), Ok(Percent(75)));
        assert_eq!("75".parse::<Percent>(), Ok(Percent(75)));
        assert!("abc".parse::<Percent>().is_err());
        assert!("200%".parse::<Percent>().is_err());
    }
}
