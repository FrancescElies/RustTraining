//! Patterns ch05 Channels and message passing (book: rust-patterns-book/src/ch05-channels-and-message-passing.md)
//! std::sync::mpsc only (the book also covers crossbeam).
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Fan-in: n producer threads each send their own id (0..n). Collect, sorted.
/// Remember: the receiver loop ends only when ALL senders are dropped (including your original!).
pub fn fan_in(n: u32) -> Vec<u32> {
    todo!()
}

/// Worker pool: 'workers' threads pull jobs from ONE shared queue (Arc<Mutex<Receiver<u32>>>),
/// send job*job back on a result channel. Return all results sorted.
pub fn square_pool(jobs: Vec<u32>, workers: usize) -> Vec<u32> {
    todo!()
}

/// Pipeline: stage 1 thread doubles, stage 2 thread adds 1, connected by channels. Order preserved.
pub fn pipeline(input: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Backpressure: sync_channel(cap) blocks the producer when full.
/// Producer sends 0..n on a bounded channel of capacity 1; consumer sums them. Return the sum.
pub fn bounded_sum(n: u64) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat05_fan_in() {
        assert_eq!(fan_in(4), [0, 1, 2, 3]);
    }
    #[test]
    fn pat05_pool() {
        assert_eq!(square_pool(vec![3, 1, 2, 4, 5], 3), [1, 4, 9, 16, 25]);
        assert_eq!(square_pool(vec![], 2), Vec::<u32>::new());
    }
    #[test]
    fn pat05_pipeline() {
        assert_eq!(pipeline(vec![1, 2, 3]), [3, 5, 7]);
    }
    #[test]
    fn pat05_bounded() {
        assert_eq!(bounded_sum(100), 4950);
    }
}
