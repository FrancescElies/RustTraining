//! Patterns ch11 Serialization, zero-copy, binary data (book: rust-patterns-book/src/ch11-*.md)
//! No serde here: hand-parse bytes, return borrowed slices instead of copies.

pub const MAGIC: u32 = 0xCAFE_BABE;

/// Wire format (little endian): magic u32 | version u16 | len u16 | payload (len bytes)
#[derive(Debug, PartialEq)]
pub struct Header {
    pub version: u16,
    pub len: u16,
}

/// Parse the 8-byte header. Too short or wrong magic -> None. (u32::from_le_bytes, try_into on slices)
pub fn parse_header(b: &[u8]) -> Option<Header> {
    todo!()
}

pub fn write_header(h: &Header) -> [u8; 8] {
    todo!()
}

/// Zero-copy: borrow the payload out of the buffer. Truncated payload -> None.
pub fn payload(b: &[u8]) -> Option<&[u8]> {
    todo!()
}

/// Length-prefixed (u8) UTF-8 strings back to back. Return borrowed &str, no allocation per string.
/// Truncated or invalid UTF-8 -> Err.
pub fn parse_strings(b: &[u8]) -> Result<Vec<&str>, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn msg(version: u16, body: &[u8]) -> Vec<u8> {
        let mut v = write_header(&Header {
            version,
            len: body.len() as u16,
        })
        .to_vec();
        v.extend_from_slice(body);
        v
    }
    #[test]
    fn pat11_header_bytes() {
        assert_eq!(
            write_header(&Header { version: 1, len: 2 }),
            [0xBE, 0xBA, 0xFE, 0xCA, 1, 0, 2, 0]
        );
    }
    #[test]
    fn pat11_parse() {
        let m = msg(3, b"hey");
        assert_eq!(parse_header(&m), Some(Header { version: 3, len: 3 }));
        assert_eq!(parse_header(&m[..7]), None);
        let mut bad = m.clone();
        bad[0] = 0;
        assert_eq!(parse_header(&bad), None);
    }
    #[test]
    fn pat11_payload() {
        let m = msg(1, b"hello");
        let p = payload(&m).unwrap();
        assert_eq!(p, b"hello");
        assert!(std::ptr::eq(p.as_ptr(), m[8..].as_ptr())); // really borrowed, not copied
        assert_eq!(payload(&m[..10]), None);
    }
    #[test]
    fn pat11_strings() {
        assert_eq!(
            parse_strings(&[2, b'h', b'i', 1, b'x']),
            Ok(vec!["hi", "x"])
        );
        assert!(parse_strings(&[5, b'a']).is_err());
        assert!(parse_strings(&[1, 0xFF]).is_err());
        assert_eq!(parse_strings(&[]), Ok(vec![]));
    }
}
