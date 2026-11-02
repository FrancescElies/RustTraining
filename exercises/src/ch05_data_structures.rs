//! Ch05 Data structures (book: ch05-data-structures.md)
//! Goals: structs, impl blocks, methods (&self/&mut self/self), associated fns, Vec, HashMap, derive.
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub w: f64,
    pub h: f64,
}

impl Rect {
    /// Associated function (no self): a square of side `s`.
    pub fn square(s: f64) -> Rect {
        Rect { w: s, h: s }
    }
    /// &self method.
    pub fn area(&self) -> f64 {
        self.w * self.h
    }
    /// &mut self method: multiply both sides by `k`.
    pub fn scale(&mut self, k: f64) {
        self.w *= k;
        self.h *= k;
    }
    /// self by value: consume and return rect with w and h swapped.
    pub fn rotated(self) -> Rect {
        Rect {
            w: self.h,
            h: self.w,
        }
    }
}

/// Count how often each word appears (split_whitespace). Use HashMap + the entry API.
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut h = HashMap::new();
    for x in text.split_whitespace() {
        *h.entry(x.to_owned()).or_insert(0) += 1;
    }
    h
}

/// A stack backed by a Vec.
#[derive(Default)]
pub struct Stack {
    items: Vec<i32>,
}

impl Stack {
    pub fn push(&mut self, x: i32) {
        self.items.push(x);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
    pub fn peek(&self) -> Option<&i32> {
        self.items.last()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch05_rect() {
        let mut r = Rect::square(2.0);
        assert_eq!(r.area(), 4.0);
        r.scale(2.0);
        assert_eq!(r, Rect { w: 4.0, h: 4.0 });
        assert_eq!(Rect { w: 1.0, h: 2.0 }.rotated(), Rect { w: 2.0, h: 1.0 });
    }
    #[test]
    fn ch05_words() {
        let m = word_count("a b a c a b");
        assert_eq!(m["a"], 3);
        assert_eq!(m["b"], 2);
        assert_eq!(m["c"], 1);
    }
    #[test]
    fn ch05_stack() {
        let mut s = Stack::default();
        s.push(1);
        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.len(), 1);
        s.pop();
        assert_eq!(s.pop(), None);
    }
}
