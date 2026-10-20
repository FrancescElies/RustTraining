//! Type-driven ch07 Validated boundaries: parse, don't validate (book: ...ch07-validated-boundaries-*.md)
//! Check ONCE at the edge; afterwards the type itself is the proof.
use std::convert::TryFrom;
use std::str::FromStr;

/// A Vec that can never be empty, so first() and average() have no failure case.
#[derive(Debug, PartialEq)]
pub struct NonEmpty<T>(Vec<T>);
impl<T> NonEmpty<T> {
    pub fn new(v: Vec<T>) -> Option<Self> {
        todo!()
    }
    pub fn first(&self) -> &T {
        todo!()
    } // no Option!
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
pub fn average(v: &NonEmpty<f64>) -> f64 {
    todo!()
} // no divide-by-zero check needed

/// TCP port 1..=65535 (0 is invalid).
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Port(u16);
impl Port {
    pub fn get(self) -> u16 {
        self.0
    }
}
impl TryFrom<u32> for Port {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, String> {
        todo!()
    }
}

/// "aa:bb:cc:dd:ee:ff" -> 6 bytes. Any deviation -> Err.
#[derive(Debug, PartialEq)]
pub struct Mac([u8; 6]);
impl FromStr for Mac {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td07_nonempty() {
        assert!(NonEmpty::<i32>::new(vec![]).is_none());
        let n = NonEmpty::new(vec![2.0, 4.0]).unwrap();
        assert_eq!((*n.first(), n.len(), average(&n)), (2.0, 2, 3.0));
    }
    #[test]
    fn td07_port() {
        assert_eq!(Port::try_from(8080).unwrap().get(), 8080);
        assert!(Port::try_from(0).is_err());
        assert!(Port::try_from(65536).is_err());
        assert!(Port::try_from(65535).is_ok());
    }
    #[test]
    fn td07_mac() {
        assert_eq!(
            "aa:bb:cc:00:ff:01".parse::<Mac>(),
            Ok(Mac([0xaa, 0xbb, 0xcc, 0x00, 0xff, 0x01]))
        );
        for bad in [
            "",
            "aa:bb",
            "aa:bb:cc:dd:ee:ff:00",
            "aa:bb:cc:dd:ee:gg",
            "aabbccddeeff",
        ] {
            assert!(bad.parse::<Mac>().is_err(), "{bad}");
        }
    }
}
