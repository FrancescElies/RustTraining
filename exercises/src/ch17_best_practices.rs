//! Ch17 Best practices (book: ch17-*.md)
//! 17.1 avoid clone: borrow, Cow, Weak.  17.2 avoid [] indexing / unwrap: get, ?, unwrap_or, and_then, find_map.
//! 17.3 collapse pyramids into iterator chains / Option sums.  (17.4 logging needs crates: read it, skip here.)
//! Rule: no .clone(), no .unwrap(), no [] indexing in your solutions.
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// 17.1 Return the longest line (by bytes), borrowed from the input. First wins on ties. "" if empty.
pub fn longest_line(text: &str) -> &str {
    todo!()
}

/// 17.1 Cow: replace each '\t' with 4 spaces. If there are no tabs, return the input BORROWED (no allocation).
pub fn expand_tabs(s: &str) -> Cow<'_, str> {
    todo!()
}

/// 17.1 Weak: child -> parent must not be a strong link (cycle = leak).
pub struct TreeNode {
    pub val: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}
impl TreeNode {
    pub fn new(val: i32) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            val,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
}
/// Create a child, link both ways (parent holds Rc, child holds Weak), return the child.
pub fn add_child(parent: &Rc<TreeNode>, val: i32) -> Rc<TreeNode> {
    todo!()
}
/// Parent's val if the parent is still alive. (Weak::upgrade)
pub fn parent_val(n: &TreeNode) -> Option<i32> {
    todo!()
}
pub fn child_count(n: &TreeNode) -> usize {
    n.children.borrow().len()
}

/// 17.2 Big-endian u16 at byte offset. Out of range -> None. No panics.
pub fn read_u16_be(data: &[u8], off: usize) -> Option<u16> {
    todo!()
}

/// 17.2 "host:8080/path" -> Some(8080). No port / bad port -> None. (split_once, and_then, parse().ok())
pub fn port_from_url(url: &str) -> Option<u16> {
    todo!()
}

/// 17.2 unwrap_or family: threads from an optional string; None/garbage -> 4; 0 -> 1.
pub fn parse_threads(s: Option<&str>) -> usize {
    todo!()
}

/// 17.2 find_map: first word that parses as i32.
pub fn first_number(words: &[&str]) -> Option<i32> {
    todo!()
}

/// 17.3 No loop+early return pyramid: total = sum qty*price, None if ANY item has no price.
/// (hint: map to Option<u32>, then .sum::<Option<u32>>())
pub fn cart_total(cart: &[(&str, u32)], prices: &HashMap<&str, u32>) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch17_longest() {
        assert_eq!(longest_line("ab\nabc\nxyz\n"), "abc");
        assert_eq!(longest_line(""), "");
    }
    #[test]
    fn ch17_cow() {
        assert!(matches!(expand_tabs("no tabs"), Cow::Borrowed("no tabs")));
        let e = expand_tabs("a\tb");
        assert!(matches!(e, Cow::Owned(_)));
        assert_eq!(e, "a    b");
    }
    #[test]
    fn ch17_weak() {
        let root = TreeNode::new(1);
        let kid = add_child(&root, 2);
        assert_eq!(parent_val(&kid), Some(1));
        assert_eq!(parent_val(&root), None);
        assert_eq!(child_count(&root), 1);
        assert_eq!(Rc::strong_count(&root), 1); // kid only holds Weak
        drop(root);
        assert_eq!(parent_val(&kid), None);
    }
    #[test]
    fn ch17_u16() {
        assert_eq!(read_u16_be(&[0, 1, 0x12, 0x34], 2), Some(0x1234));
        assert_eq!(read_u16_be(&[1, 2], 1), None);
        assert_eq!(read_u16_be(&[], usize::MAX), None);
    }
    #[test]
    fn ch17_port() {
        assert_eq!(port_from_url("example.com:8080/x"), Some(8080));
        assert_eq!(port_from_url("nohost"), None);
        assert_eq!(port_from_url("h:abc"), None);
        assert_eq!(port_from_url("h:80"), Some(80));
    }
    #[test]
    fn ch17_threads() {
        assert_eq!(parse_threads(None), 4);
        assert_eq!(parse_threads(Some("x")), 4);
        assert_eq!(parse_threads(Some("0")), 1);
        assert_eq!(parse_threads(Some("8")), 8);
    }
    #[test]
    fn ch17_first_number() {
        assert_eq!(first_number(&["a", "12", "7"]), Some(12));
        assert_eq!(first_number(&["a"]), None);
    }
    #[test]
    fn ch17_cart() {
        let prices = HashMap::from([("apple", 2), ("pear", 3)]);
        assert_eq!(cart_total(&[("apple", 3), ("pear", 1)], &prices), Some(9));
        assert_eq!(cart_total(&[("apple", 3), ("kiwi", 1)], &prices), None);
        assert_eq!(cart_total(&[], &prices), Some(0));
    }
}
