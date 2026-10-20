//! Patterns ch04 PhantomData (book: rust-patterns-book/src/ch04-phantomdata-types-that-carry-no-data.md)
use std::marker::PhantomData;
use std::ops::Add;

/// Unit-of-measure: the unit exists only in the type.
pub struct Meters;
pub struct Feet;
#[derive(Debug, PartialEq)]
pub struct Quantity<U> {
    pub value: f64,
    _u: PhantomData<U>,
}
impl<U> Quantity<U> {
    pub fn new(value: f64) -> Self {
        Quantity {
            value,
            _u: PhantomData,
        }
    }
}
/// Only same-unit addition compiles. (Quantity<Meters> + Quantity<Feet> must be an error: try it.)
impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
/// 1 m = 3.28084 ft
pub fn to_feet(m: Quantity<Meters>) -> Quantity<Feet> {
    todo!()
}

/// Typed id: Id<User> and Id<Order> are different types but cost one u32.
/// #[derive(Clone, Copy, PartialEq)] would wrongly require T: Clone/Copy/PartialEq. Implement by hand.
/// (PhantomData<fn() -> T> keeps Send/Sync independent of T.)
pub struct Id<T> {
    pub raw: u32,
    _t: PhantomData<fn() -> T>,
}
impl<T> Id<T> {
    pub fn new(raw: u32) -> Self {
        Id {
            raw,
            _t: PhantomData,
        }
    }
}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat04_add() {
        assert_eq!(
            (Quantity::<Meters>::new(1.5) + Quantity::new(2.0)).value,
            3.5
        );
    }
    #[test]
    fn pat04_convert() {
        assert!((to_feet(Quantity::new(100.0)).value - 328.084).abs() < 1e-9);
    }
    #[test]
    fn pat04_id() {
        struct NoTraits; // deliberately not Clone/PartialEq/Debug
        let a: Id<NoTraits> = Id::new(7);
        let b = a; // Copy
        assert!(a == b);
        assert!(a != Id::new(8));
    }
}
