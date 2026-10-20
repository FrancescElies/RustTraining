//! Type-driven ch16 Send & Sync: compile-time concurrency proofs (book: ...ch16-send-sync-*.md)
use std::marker::PhantomData;
use std::sync::Mutex;
use std::thread;

/// Compile-time theorems: these only compile if the bound holds.
pub const fn require_send<T: Send>() {}
pub const fn require_sync<T: Sync>() {}

pub struct Counter {
    inner: Mutex<u32>,
}
impl Counter {
    pub fn new() -> Self {
        Counter {
            inner: Mutex::new(0),
        }
    }
    pub fn inc(&self) {
        todo!()
    }
    pub fn get(&self) -> u32 {
        todo!()
    }
}
// Mutex<u32> turns interior mutation into something Sync: prove it.
const _: () = require_sync::<Counter>();
const _: () = require_send::<Counter>();

/// A handle that must stay on its creating thread (like a thread-bound OS/GPU context).
/// The PhantomData<*const ()> makes it !Send and !Sync.
pub struct ThreadBound {
    pub id: u32,
    _not_send: PhantomData<*const ()>,
}
impl ThreadBound {
    pub fn new(id: u32) -> Self {
        ThreadBound {
            id,
            _not_send: PhantomData,
        }
    }
}
// Uncomment to see the error: const _: () = require_send::<ThreadBound>();

/// Spawn n SCOPED threads (thread::scope, so they can borrow the counter), each doing 100 incs. Return the total.
pub fn count_in_parallel(counter: &Counter, n: usize) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td16_counter() {
        let c = Counter::new();
        c.inc();
        c.inc();
        assert_eq!(c.get(), 2);
    }
    #[test]
    fn td16_parallel() {
        let c = Counter::new();
        assert_eq!(count_in_parallel(&c, 4), 400);
    }
    #[test]
    fn td16_bound() {
        assert_eq!(ThreadBound::new(3).id, 3);
    }
}
