//! Async ch12 Common pitfalls (book: async-book/src/ch12-common-pitfalls.md)
//! Accidental sequential execution, blocking the executor, cancellation, no async Drop.
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub async fn slow(id: u32) -> u32 {
    sleep(Duration::from_millis(100)).await;
    id
}

/// BUG to fix: this runs the three calls one after another (300ms). Make them concurrent (100ms),
/// results still in input order. (futures::future::join_all, or tokio::join!)
pub async fn fetch_three(ids: [u32; 3]) -> Vec<u32> {
    let mut out = vec![];
    for id in ids {
        out.push(slow(id).await);
    }
    out
}

/// Give up after 'ms' milliseconds: None on timeout. (tokio::time::timeout)
pub async fn with_timeout<F: Future>(f: F, ms: u64) -> Option<F::Output> {
    todo!()
}

/// Race two sleeps with tokio::select!. Return "a" or "b", whichever finishes first. The loser is cancelled (dropped).
pub async fn first_of(a_ms: u64, b_ms: u64) -> &'static str {
    todo!()
}

/// There is no async Drop. Cleanup on cancellation goes in a plain Drop guard.
pub struct SetOnDrop(pub Arc<AtomicBool>);
impl Drop for SetOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}
/// Create the guard, then sleep for an hour. When the task is aborted, the guard must run.
pub async fn long_task(flag: Arc<AtomicBool>) {
    todo!()
}

/// Sum 1..=n is CPU work. Don't do it on the async worker thread: tokio::task::spawn_blocking(..).await.unwrap().
pub async fn heavy_sum(n: u64) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(start_paused = true)]
    async fn async12_concurrent() {
        let t = tokio::time::Instant::now();
        assert_eq!(fetch_three([1, 2, 3]).await, [1, 2, 3]);
        assert_eq!(t.elapsed(), Duration::from_millis(100));
    }
    #[tokio::test(start_paused = true)]
    async fn async12_timeout() {
        assert_eq!(with_timeout(slow(5), 200).await, Some(5));
        assert_eq!(with_timeout(slow(5), 50).await, None);
    }
    #[tokio::test(start_paused = true)]
    async fn async12_select() {
        assert_eq!(first_of(50, 10).await, "b");
        assert_eq!(first_of(10, 50).await, "a");
    }
    #[tokio::test(start_paused = true)]
    async fn async12_cleanup_on_cancel() {
        let flag = Arc::new(AtomicBool::new(false));
        let h = tokio::spawn(long_task(flag.clone()));
        sleep(Duration::from_millis(10)).await;
        assert!(!flag.load(Ordering::SeqCst));
        h.abort();
        let _ = h.await;
        assert!(
            flag.load(Ordering::SeqCst),
            "Drop guard must run on cancellation"
        );
    }
    #[tokio::test]
    async fn async12_heavy() {
        assert_eq!(heavy_sum(1000).await, 500_500);
    }
}
