//! Ch13 Concurrency (book: ch13-concurrency.md)
//! Goals: thread::spawn + move, join, Arc<Mutex<T>>, mpsc channels.
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// Spawn `n` threads, each adds 1 to a shared counter 1000 times. Return the final value.
pub fn parallel_count(n: usize) -> u64 {
    todo!()
}

/// Split `v` into `parts` chunks, sum each in its own thread, return total.
/// (hint: thread::scope lets threads borrow the slice)
pub fn parallel_sum(v: &[u64], parts: usize) -> u64 {
    todo!()
}

/// Spawn a thread that sends 0..n on a channel; collect them on this thread.
pub fn produce(n: u32) -> Vec<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch13_count() {
        assert_eq!(parallel_count(8), 8000);
    }
    #[test]
    fn ch13_sum() {
        let v: Vec<u64> = (1..=100).collect();
        assert_eq!(parallel_sum(&v, 4), 5050);
    }
    #[test]
    fn ch13_chan() {
        assert_eq!(produce(4), [0, 1, 2, 3]);
    }
}
