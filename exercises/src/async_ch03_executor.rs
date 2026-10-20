//! Async ch03 How poll works: write a minimal executor (book: async-book/src/ch03-how-poll-works.md)
//! Goals: Waker from the Wake trait, park/unpark, the poll loop, spurious wakes.
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::Duration;

/// A waker that unparks the thread that is blocked in block_on.
struct ThreadWaker(Thread);
impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        todo!() // self.0.unpark()
    }
}

/// Run a future to completion on the current thread. Loop: poll; if Pending, park the thread
/// (wake() will unpark it). Pin the future on the stack with std::pin::pin!.
/// Spurious wakes are legal: always re-poll after park() returns, never assume the future is ready.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    todo!()
}

/// Provided: a future that completes after a real delay, woken from another thread.
pub struct ThreadDelay {
    shared: Arc<Mutex<Shared>>,
}
struct Shared {
    done: bool,
    waker: Option<Waker>,
}
impl ThreadDelay {
    pub fn new(ms: u64) -> Self {
        let shared = Arc::new(Mutex::new(Shared {
            done: false,
            waker: None,
        }));
        let s2 = shared.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(ms));
            let mut s = s2.lock().unwrap();
            s.done = true;
            if let Some(w) = s.waker.take() {
                w.wake();
            }
        });
        ThreadDelay { shared }
    }
}
impl Future for ThreadDelay {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut s = self.shared.lock().unwrap();
        if s.done {
            Poll::Ready(())
        } else {
            s.waker = Some(cx.waker().clone()); // always store the LATEST waker
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    #[test]
    fn async03_ready_future() {
        assert_eq!(block_on(async { 1 + 1 }), 2);
    }
    #[test]
    fn async03_waits_for_wake() {
        let t = Instant::now();
        let v = block_on(async {
            ThreadDelay::new(30).await;
            ThreadDelay::new(30).await;
            "done"
        });
        assert_eq!(v, "done");
        assert!(t.elapsed() >= Duration::from_millis(60));
    }
}
