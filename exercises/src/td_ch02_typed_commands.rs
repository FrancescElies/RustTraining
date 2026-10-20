//! Type-driven ch02 Typed command interfaces: the request determines the response type
//! (book: type-driven-correctness-book/src/ch02-*.md)

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Celsius(pub f32);
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rpm(pub u32);

pub trait Command {
    type Response;
    fn code(&self) -> u8;
    /// Decode the raw reply bytes. Wrong length -> None.
    fn parse(&self, raw: &[u8]) -> Option<Self::Response>;
}

/// Reply: i16 big-endian, in hundredths of a degree. [0x09, 0xC4] = 2500 -> Celsius(25.0)
pub struct ReadTemp {
    pub sensor: u8,
}
/// Reply: u16 big-endian RPM.
pub struct ReadFan {
    pub fan: u8,
}

impl Command for ReadTemp {
    type Response = Celsius;
    fn code(&self) -> u8 {
        0x10 + self.sensor
    }
    fn parse(&self, raw: &[u8]) -> Option<Celsius> {
        todo!()
    }
}
impl Command for ReadFan {
    type Response = Rpm;
    fn code(&self) -> u8 {
        0x20 + self.fan
    }
    fn parse(&self, raw: &[u8]) -> Option<Rpm> {
        todo!()
    }
}

/// The executor is generic: ONE function, and the return type follows from the command.
/// wire: sends the command code, returns raw reply bytes.
pub fn execute<C: Command>(cmd: &C, wire: impl Fn(u8) -> Vec<u8>) -> Option<C::Response> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fake_wire(code: u8) -> Vec<u8> {
        match code {
            0x10 => vec![0x09, 0xC4],
            0x20 => vec![0x03, 0xE8],
            _ => vec![],
        }
    }
    #[test]
    fn td02_temp() {
        assert_eq!(
            execute(&ReadTemp { sensor: 0 }, fake_wire),
            Some(Celsius(25.0))
        );
    }
    #[test]
    fn td02_fan() {
        assert_eq!(execute(&ReadFan { fan: 0 }, fake_wire), Some(Rpm(1000)));
    }
    #[test]
    fn td02_bad_reply() {
        assert_eq!(execute(&ReadTemp { sensor: 5 }, fake_wire), None);
    }
    // The point: this does not compile (Rpm is not Celsius). Try it, then delete it.
    // fn wrong() { let _t: Option<Celsius> = execute(&ReadFan { fan: 0 }, fake_wire); }
}
