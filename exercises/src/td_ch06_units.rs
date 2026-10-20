//! Type-driven ch06 Dimensional analysis: the compiler checks your units
//! (book: ...ch06-dimensional-analysis-*.md)
use std::ops::{Add, Div, Mul};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Volts(pub f64);
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Amps(pub f64);
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Ohms(pub f64);
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Watts(pub f64);

/// P = V * I
impl Mul<Amps> for Volts {
    type Output = Watts;
    fn mul(self, i: Amps) -> Watts {
        todo!()
    }
}
/// I = V / R
impl Div<Ohms> for Volts {
    type Output = Amps;
    fn div(self, r: Ohms) -> Amps {
        todo!()
    }
}
impl Add for Watts {
    type Output = Watts;
    fn add(self, o: Watts) -> Watts {
        todo!()
    }
}

pub fn total_power(loads: &[Watts]) -> Watts {
    todo!()
}
/// True if the loads fit in the budget (<=).
pub fn within_budget(loads: &[Watts], budget: Watts) -> bool {
    todo!()
}
/// Power drawn by a resistive load on a rail: P = V * (V / R). Express it only with the operators above.
pub fn resistive_power(v: Volts, r: Ohms) -> Watts {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td06_ops() {
        assert_eq!(Volts(12.0) * Amps(2.0), Watts(24.0));
        assert_eq!(Volts(12.0) / Ohms(4.0), Amps(3.0));
        assert_eq!(Watts(1.0) + Watts(2.0), Watts(3.0));
    }
    #[test]
    fn td06_budget() {
        let l = [Watts(10.0), Watts(20.0)];
        assert_eq!(total_power(&l), Watts(30.0));
        assert!(within_budget(&l, Watts(30.0)));
        assert!(!within_budget(&l, Watts(29.9)));
        assert_eq!(total_power(&[]), Watts(0.0));
    }
    #[test]
    fn td06_resistive() {
        assert_eq!(resistive_power(Volts(10.0), Ohms(5.0)), Watts(20.0));
    }
    // Does not compile (mixing units): let _ = Volts(1.0) + Amps(1.0);
}
