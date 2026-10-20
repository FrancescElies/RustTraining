//! Async ch08 Tokio deep dive (book: async-book/src/ch08-tokio-deep-dive.md)
//! spawn + JoinHandle, oneshot, Semaphore, abort/cancellation.
//! Tests use start_paused = true: virtual time, so sleeps are instant and results are deterministic.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{oneshot, Semaphore};

/// Spawn one task per element (tokio::spawn needs 'static: move the value in), await the handles IN ORDER.
pub async fn parallel_squares(v: Vec<u64>) -> Vec<u64> {
    todo!()
}

/// Request/response with a oneshot channel: spawn a task that computes x*2 and replies through the oneshot Sender.
pub async fn ask(x: u32) -> u32 {
    todo!()
}

/// Spawn 'tasks' tasks. Each: acquire a permit from a Semaphore(limit) (acquire_owned), bump a shared "running" counter,
/// record the max seen (fetch_max), sleep 10ms, decrement, release. Return the max concurrency observed (== limit).
pub async fn max_concurrency(tasks: usize, limit: usize) -> usize {
    todo!()
}

/// Spawn a task that sleeps for an hour, abort it, await the handle, and report whether the JoinError says "cancelled".
pub async fn was_cancelled() -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(start_paused = true)]
    async fn async08_squares() {
        assert_eq!(parallel_squares(vec![1, 2, 3, 4]).await, [1, 4, 9, 16]);
    }
    #[tokio::test(start_paused = true)]
    async fn async08_ask() {
        assert_eq!(ask(21).await, 42);
    }
    #[tokio::test(start_paused = true)]
    async fn async08_semaphore() {
        let t = tokio::time::Instant::now();
        assert_eq!(max_concurrency(10, 3).await, 3);
        // 10 tasks, 3 at a time, 10ms each => 4 waves
        assert_eq!(t.elapsed(), Duration::from_millis(40));
    }
    #[tokio::test(start_paused = true)]
    async fn async08_abort() {
        assert!(was_cancelled().await);
    }
}
