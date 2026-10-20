//! Async ch13 Production patterns (book: async-book/src/ch13-production-patterns.md)
//! retry, backpressure, structured concurrency (JoinSet), worker pool, graceful shutdown.
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, watch, Mutex};
use tokio::task::JoinSet;
use tokio::time::sleep;

/// Call f up to 'attempts' times (attempts >= 1). Return the first Ok; otherwise the LAST error.
/// Sleep 10ms between attempts (not after the last).
pub async fn retry<T, E, F, Fut>(attempts: usize, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    todo!()
}

/// Backpressure: producer task sends 0..n into a bounded mpsc::channel(cap) (send().await blocks when full),
/// this function consumes and returns everything received, in order.
pub async fn produce_consume(n: u32, cap: usize) -> Vec<u32> {
    todo!()
}

/// Structured concurrency with JoinSet: one task per input: 13 -> Err("unlucky"), else Ok(x*2).
/// All fine -> Ok(sorted results). Any error -> return the first Err you see (remaining tasks are aborted when the JoinSet drops).
pub async fn run_all(inputs: Vec<u32>) -> Result<Vec<u32>, String> {
    todo!()
}

/// Worker pool: 'workers' tasks share ONE receiver (Arc<tokio::sync::Mutex<Receiver>>), each loops
/// { lock, recv().await, release lock, process job*2 }, until the channel is closed and drained.
/// Return all results sorted. (Release the lock BEFORE doing the work: don't hold a guard across the processing!)
pub async fn worker_pool(jobs: mpsc::Receiver<u32>, workers: usize) -> Vec<u32> {
    todo!()
}

/// Graceful shutdown: count one tick every 10ms until the watch value becomes true, then return the tick count.
/// (tokio::select! between shutdown.changed() and sleep)
pub async fn run_until_shutdown(mut shutdown: watch::Receiver<bool>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test(start_paused = true)]
    async fn async13_retry() {
        let calls = AtomicUsize::new(0);
        let r: Result<u32, String> = retry(5, || {
            let n = calls.fetch_add(1, Ordering::SeqCst);
            async move {
                if n < 2 {
                    Err(format!("fail {n}"))
                } else {
                    Ok(7)
                }
            }
        })
        .await;
        assert_eq!(r, Ok(7));
        assert_eq!(calls.load(Ordering::SeqCst), 3);

        let r: Result<u32, String> = retry(3, || async { Err("nope".to_string()) }).await;
        assert_eq!(r, Err("nope".to_string()));
    }
    #[tokio::test(start_paused = true)]
    async fn async13_retry_sleeps_between() {
        let t = tokio::time::Instant::now();
        let _: Result<(), ()> = retry(3, || async { Err(()) }).await;
        assert_eq!(t.elapsed(), Duration::from_millis(20));
    }
    #[tokio::test]
    async fn async13_backpressure() {
        assert_eq!(produce_consume(50, 2).await, (0..50).collect::<Vec<_>>());
    }
    #[tokio::test]
    async fn async13_run_all() {
        assert_eq!(run_all(vec![3, 1, 2]).await, Ok(vec![2, 4, 6]));
        assert_eq!(run_all(vec![1, 13, 2]).await, Err("unlucky".to_string()));
    }
    #[tokio::test]
    async fn async13_pool() {
        let (tx, rx) = mpsc::channel(4);
        let pool = tokio::spawn(worker_pool(rx, 3));
        for i in 0..20 {
            tx.send(i).await.unwrap();
        }
        drop(tx); // closing the channel is the shutdown signal
        let out = pool.await.unwrap();
        assert_eq!(out, (0..20).map(|x| x * 2).collect::<Vec<_>>());
        let _ = Arc::new(Mutex::new(0)); // (hint: that's the type you need)
    }
    #[tokio::test(start_paused = true)]
    async fn async13_shutdown() {
        let (tx, rx) = watch::channel(false);
        let h = tokio::spawn(run_until_shutdown(rx));
        sleep(Duration::from_millis(55)).await;
        tx.send(true).unwrap();
        assert_eq!(h.await.unwrap(), 5);
    }
    #[allow(dead_code)]
    fn _uses(_: JoinSet<()>) {}
}
