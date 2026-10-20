//! Patterns ch03 Newtype and type-state (book: rust-patterns-book/src/ch03-the-newtype-and-type-state-patterns.md)
use std::ops::Deref;

/// Newtype: a validated Email. Exactly one '@', non-empty on both sides.
#[derive(Debug, PartialEq)]
pub struct Email(String);
impl Email {
    pub fn parse(s: &str) -> Result<Email, String> {
        todo!()
    }
}
/// Deref to str so e.len(), e.contains(..) work. (book: power and pitfalls of Deref on newtypes)
impl Deref for Email {
    type Target = str;
    fn deref(&self) -> &str {
        todo!()
    }
}

/// Type-state door: illegal transitions do not compile.
pub struct Open;
pub struct Closed;
pub struct Locked {
    code: u32,
}
pub struct Door<S> {
    state: S,
}
impl Door<Closed> {
    pub fn new() -> Self {
        Door { state: Closed }
    }
    pub fn open(self) -> Door<Open> {
        todo!()
    }
    pub fn lock(self, code: u32) -> Door<Locked> {
        todo!()
    }
}
impl Door<Open> {
    pub fn close(self) -> Door<Closed> {
        todo!()
    }
}
impl Door<Locked> {
    /// Right code -> Ok(closed door). Wrong code -> Err(still locked door), giving the door back.
    pub fn unlock(self, code: u32) -> Result<Door<Closed>, Door<Locked>> {
        todo!()
    }
}
// Experiment (do not keep): try Door::new().lock(1).open(). Read the compile error.

/// Builder with defaults.
#[derive(Debug, PartialEq)]
pub struct Request {
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub timeout_secs: u32,
}
pub struct RequestBuilder {
    url: String,
    headers: Vec<(String, String)>,
    timeout_secs: u32,
}
impl RequestBuilder {
    /// Default timeout is 30.
    pub fn new(url: &str) -> Self {
        todo!()
    }
    pub fn header(mut self, k: &str, v: &str) -> Self {
        todo!()
    }
    pub fn timeout(mut self, secs: u32) -> Self {
        todo!()
    }
    pub fn build(self) -> Request {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat03_email() {
        let e = Email::parse("a@b.com").unwrap();
        assert_eq!(e.len(), 7);
        assert!(e.contains("b.com"));
        for bad in ["", "ab.com", "@b.com", "a@", "a@b@c"] {
            assert!(Email::parse(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn pat03_door() {
        let d = Door::new().open().close().lock(7);
        let d = match d.unlock(1) {
            Err(d) => d,
            Ok(_) => panic!("wrong code accepted"),
        };
        assert!(d.unlock(7).is_ok());
    }
    #[test]
    fn pat03_builder() {
        let r = RequestBuilder::new("http://x")
            .header("a", "1")
            .timeout(5)
            .build();
        assert_eq!(
            r,
            Request {
                url: "http://x".into(),
                headers: vec![("a".into(), "1".into())],
                timeout_secs: 5
            }
        );
        assert_eq!(RequestBuilder::new("u").build().timeout_secs, 30);
    }
}
