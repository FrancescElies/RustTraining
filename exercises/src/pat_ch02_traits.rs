//! Patterns ch02 Traits in depth (book: rust-patterns-book/src/ch02-traits-in-depth.md)
//! associated types, supertraits, blanket impls, HRTB, impl Trait.
use std::fmt::Display;

/// Associated type: a Source yields Items. (Why not Source<Item> generic? read the book section.)
pub trait Source {
    type Item;
    fn next_item(&mut self) -> Option<Self::Item>;
    /// Default method: drain everything.
    fn collect_all(&mut self) -> Vec<Self::Item> {
        todo!()
    }
}
/// Counts down n, n-1, ..., 1.
pub struct Countdown(pub u32);
impl Source for Countdown {
    type Item = u32;
    fn next_item(&mut self) -> Option<u32> {
        todo!()
    }
}

/// Supertrait: every Greeter must also be Named.
pub trait Named {
    fn name(&self) -> String;
}
pub trait Greeter: Named {
    /// "Hello, <name>!"
    fn greet(&self) -> String {
        todo!()
    }
}
pub struct Person(pub String);
impl Named for Person {
    fn name(&self) -> String {
        self.0.clone()
    }
}
impl Greeter for Person {}

/// Blanket impl: anything Display can shout.
pub trait Shout {
    fn shout(&self) -> String;
}
// "hi" -> "HI!", 42 -> "42!"
impl<T: Display> Shout for T {
    fn shout(&self) -> String {
        todo!()
    }
}

/// HRTB: f works for ANY lifetime of its input. Return the length of f(item) for each item.
pub fn lens_after<F>(f: F, items: &[String]) -> Vec<usize>
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    todo!()
}

/// Return-position impl Trait: 0, 2, 4, ... (infinite). No Box.
pub fn evens() -> impl Iterator<Item = u32> {
    std::iter::empty() /* stub */
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat02_source() {
        assert_eq!(Countdown(3).collect_all(), [3, 2, 1]);
        assert_eq!(Countdown(0).collect_all(), Vec::<u32>::new());
    }
    #[test]
    fn pat02_greeter() {
        assert_eq!(Person("Ana".into()).greet(), "Hello, Ana!");
    }
    #[test]
    fn pat02_shout() {
        assert_eq!("hi".shout(), "HI!");
        assert_eq!(42.shout(), "42!");
    }
    #[test]
    fn pat02_hrtb() {
        assert_eq!(
            lens_after(|s| s.trim(), &["  ab ".to_string(), "c".to_string()]),
            [2, 1]
        );
    }
    #[test]
    fn pat02_evens() {
        assert_eq!(evens().take(3).collect::<Vec<_>>(), [0, 2, 4]);
    }
}
