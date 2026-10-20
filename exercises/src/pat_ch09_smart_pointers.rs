//! Patterns ch09 Smart pointers & interior mutability (book: rust-patterns-book/src/ch09-*.md)
//! Cell, RefCell, Rc, Box<dyn Fn>.
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// Observer pattern: handlers are boxed closures. emit takes &self (handlers are Fn).
pub struct EventBus {
    handlers: Vec<Box<dyn Fn(&str)>>,
}
impl EventBus {
    pub fn new() -> Self {
        EventBus { handlers: vec![] }
    }
    pub fn subscribe(&mut self, f: impl Fn(&str) + 'static) {
        todo!()
    }
    pub fn emit(&self, ev: &str) {
        todo!()
    }
}

/// Cell: mutate through &self for Copy data.
pub struct Stats {
    hits: Cell<u32>,
}
impl Stats {
    pub fn new() -> Self {
        Stats { hits: Cell::new(0) }
    }
    pub fn hit(&self) {
        todo!()
    }
    pub fn hits(&self) -> u32 {
        todo!()
    }
}

/// RefCell: a memoizing wrapper whose call() takes &self. Only call f on a cache miss.
pub struct Memo<F: Fn(u64) -> u64> {
    f: F,
    cache: RefCell<HashMap<u64, u64>>,
}
impl<F: Fn(u64) -> u64> Memo<F> {
    pub fn new(f: F) -> Self {
        Memo {
            f,
            cache: RefCell::new(HashMap::new()),
        }
    }
    pub fn call(&self, x: u64) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    #[test]
    fn pat09_bus() {
        let log = Rc::new(RefCell::new(Vec::<String>::new()));
        let mut bus = EventBus::new();
        let l1 = Rc::clone(&log);
        bus.subscribe(move |e| l1.borrow_mut().push(format!("A:{e}")));
        let l2 = Rc::clone(&log);
        bus.subscribe(move |e| l2.borrow_mut().push(format!("B:{e}")));
        bus.emit("x");
        assert_eq!(*log.borrow(), ["A:x", "B:x"]);
    }
    #[test]
    fn pat09_stats() {
        let s = Stats::new();
        s.hit();
        s.hit();
        assert_eq!(s.hits(), 2);
    }
    #[test]
    fn pat09_memo() {
        let calls = Rc::new(Cell::new(0));
        let c = Rc::clone(&calls);
        let m = Memo::new(move |x| {
            c.set(c.get() + 1);
            x * x
        });
        assert_eq!((m.call(4), m.call(4), m.call(5)), (16, 16, 25));
        assert_eq!(calls.get(), 2);
    }
}
