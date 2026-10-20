//! Ch11 From / Into (book: ch11-from-and-into-traits.md)
//! Goals: implement From (get Into free), TryFrom for fallible conversion.
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Celsius(pub f64);
#[derive(Debug, PartialEq)]
pub struct Fahrenheit(pub f64);
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);
impl From<(u8, u8, u8)> for Rgb {
    fn from(t: (u8, u8, u8)) -> Self {
        todo!()
    }
}

/// Takes anything convertible into Rgb; returns it.
pub fn make_color(c: impl Into<Rgb>) -> Rgb {
    todo!()
}

#[derive(Debug, PartialEq)]
pub struct Even(pub u32);
// odd -> Err
impl TryFrom<u32> for Even {
    type Error = String;
    fn try_from(n: u32) -> Result<Self, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch11_temp() {
        let f: Fahrenheit = Celsius(100.0).into();
        assert_eq!(f, Fahrenheit(212.0));
    }
    #[test]
    fn ch11_rgb() {
        assert_eq!(make_color((1, 2, 3)), Rgb(1, 2, 3));
    }
    #[test]
    fn ch11_even() {
        assert_eq!(Even::try_from(4), Ok(Even(4)));
        assert!(Even::try_from(5).is_err());
    }
}
