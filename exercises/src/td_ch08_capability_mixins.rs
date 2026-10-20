//! Type-driven ch08 Capability mixins: behaviour appears only when the hardware has the ingredients
//! (book: ...ch08-capability-mixins-*.md)

// Ingredient traits: what the hardware CAN do.
pub trait HasTemp {
    fn temp(&self) -> f32;
}
pub trait HasFan {
    fn fan_rpm(&self) -> u32;
}

// Mixin trait 1: needs only a temperature sensor. Blanket impl gives it to EVERY HasTemp type.
pub trait ThermalCheck {
    fn overheated(&self) -> bool;
}
/// overheated = temp > 85.0
impl<T: HasTemp> ThermalCheck for T {
    fn overheated(&self) -> bool {
        todo!()
    }
}

// Mixin trait 2: needs BOTH temperature and a fan.
pub trait CoolingReport {
    fn report(&self) -> String;
}
/// "temp=70.5C fan=1200rpm"
impl<T: HasTemp + HasFan> CoolingReport for T {
    fn report(&self) -> String {
        todo!()
    }
}

pub struct Server {
    pub t: f32,
    pub rpm: u32,
}
impl HasTemp for Server {
    fn temp(&self) -> f32 {
        self.t
    }
}
impl HasFan for Server {
    fn fan_rpm(&self) -> u32 {
        self.rpm
    }
}

pub struct FanlessSwitch {
    pub t: f32,
}
impl HasTemp for FanlessSwitch {
    fn temp(&self) -> f32 {
        self.t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td08_server() {
        let s = Server { t: 70.5, rpm: 1200 };
        assert!(!s.overheated());
        assert!(Server { t: 90.0, rpm: 0 }.overheated());
        assert_eq!(s.report(), "temp=70.5C fan=1200rpm");
    }
    #[test]
    fn td08_switch() {
        assert!(FanlessSwitch { t: 99.0 }.overheated());
        // FanlessSwitch { t: 1.0 }.report(); // does not compile: no HasFan. That's the point.
    }
}
