//! Ch07 Ownership, borrowing, lifetimes (book: ch07-*.md)
//! Goals: move vs borrow, &mut exclusivity, lifetimes in signatures, Rc/RefCell/Box.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Take ownership of the String, return it with "!" appended.
pub fn shout(mut s: String) -> String {
    s.push_str("!");
    s
}

/// Borrow immutably: count vowels (aeiou, lowercase).
pub fn vowels(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, x| if "aeiou".contains(x) { acc + 1 } else { acc })
}

/// Borrow mutably: double every element in place.
pub fn double_all(v: &mut Vec<i32>) {
    todo!()
}

/// Return the longer of two strings. Lifetimes are given: explain to yourself why 'a is needed. Then try removing them and read the error.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// Struct that borrows: holds a slice of the original text.
pub struct Parser<'a> {
    text: &'a str,
}
impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!()
    }
    /// Return the next whitespace-delimited token and advance; None at the end.
    /// (hint: trim_start, find(char::is_whitespace), split_at)
    pub fn next_token(&mut self) -> Option<&'a str> {
        todo!()
    }
}

/// Shared mutable counter: two handles to the same count. Use Rc<RefCell<i32>>.
pub fn shared_counter() -> (Rc<RefCell<i32>>, Rc<RefCell<i32>>) {
    todo!()
}

/// Singly linked list with Box.
#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
pub fn prepend(l: List, x: i32) -> List {
    todo!()
}
pub fn sum(l: &List) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch07_shout() {
        assert_eq!(shout("hi".to_string()), "hi!");
    }
    #[test]
    fn ch07_vowels() {
        assert_eq!(vowels("education"), 5);
    }
    #[test]
    fn ch07_double() {
        let mut v = vec![1, 2];
        double_all(&mut v);
        assert_eq!(v, [2, 4]);
    }
    #[test]
    fn ch07_longest() {
        assert_eq!(longest("abc", "de"), "abc");
    }
    #[test]
    fn ch07_parser() {
        let mut p = Parser::new("  let x  = 5");
        assert_eq!(p.next_token(), Some("let"));
        assert_eq!(p.next_token(), Some("x"));
        assert_eq!(p.next_token(), Some("="));
        assert_eq!(p.next_token(), Some("5"));
        assert_eq!(p.next_token(), None);
    }
    #[test]
    fn ch07_rc() {
        let (a, b) = shared_counter();
        *a.borrow_mut() += 5;
        assert_eq!(*b.borrow(), 5);
    }
    #[test]
    fn ch07_list() {
        let l = prepend(prepend(List::Nil, 2), 1);
        assert_eq!(sum(&l), 3);
    }
}
