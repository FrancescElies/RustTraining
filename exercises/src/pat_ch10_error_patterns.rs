//! Patterns ch10 Error handling patterns (book: rust-patterns-book/src/ch10-error-handling-patterns.md)
//! Hand-written equivalent of what thiserror/anyhow generate: Display, Error::source, From, context wrapping.
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),
    Privileged(u16),
}
// Display: Parse -> "bad port: <inner>"   Privileged(p) -> "port <p> is privileged"
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
// source(): Parse exposes the ParseIntError, Privileged has none.
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        todo!()
    }
}

/// Parse a u16 port with '?'. Ports below 1024 -> Privileged.
pub fn load_port(s: &str) -> Result<u16, AppError> {
    todo!()
}

/// Context wrapper (anyhow's .context()): message + the underlying cause.
#[derive(Debug)]
pub struct ContextError {
    msg: String,
    cause: Box<dyn Error>,
}
impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!() /* just msg */
    }
}
impl Error for ContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}
pub fn with_context<T, E: Error + 'static>(r: Result<T, E>, msg: &str) -> Result<T, ContextError> {
    todo!()
}

/// Walk the chain: e's message, then its source's message, and so on.
pub fn chain(e: &dyn Error) -> Vec<String> {
    todo!()
}

/// Application level: Box<dyn Error> accepts any error through '?'.
pub fn run(s: &str) -> Result<u16, Box<dyn Error>> {
    let p = with_context(load_port(s), "loading config")?;
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat10_ok() {
        assert_eq!(load_port("8080").unwrap(), 8080);
    }
    #[test]
    fn pat10_privileged() {
        assert_eq!(
            load_port("80").unwrap_err().to_string(),
            "port 80 is privileged"
        );
    }
    #[test]
    fn pat10_parse() {
        let e = load_port("x").unwrap_err();
        assert!(e.to_string().starts_with("bad port: "));
        assert!(e.source().is_some());
        assert!(load_port("99999").is_err()); // overflow is a parse error too
    }
    #[test]
    fn pat10_chain() {
        let e = run("x").unwrap_err();
        let c = chain(e.as_ref());
        assert_eq!(c.len(), 3);
        assert_eq!(c[0], "loading config");
        assert!(c[1].starts_with("bad port: "));
        assert_eq!(run("8080").unwrap(), 8080);
    }
}
