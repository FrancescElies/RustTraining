//! Async ch05 The state machine reveal (book: async-book/src/ch05-the-state-machine-reveal.md)
//! The compiler turns an async fn into an enum with one variant per .await point. Do it by hand.
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Provided: returns Pending once (waking itself), then Ready.
pub struct YieldOnce {
    yielded: bool,
}
impl YieldOnce {
    pub fn new() -> Self {
        YieldOnce { yielded: false }
    }
}
impl Future for YieldOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// The async fn we want to hand-compile:
pub async fn add_double(a: u32, b: u32) -> u32 {
    let sum = a + b;
    YieldOnce::new().await; // <- the single suspension point
    sum * 2
}

/// Hand-written equivalent. One variant per state; data that lives across the .await (sum, the inner future) lives in the variant.
pub enum AddDouble {
    Start { a: u32, b: u32 },
    Waiting { sum: u32, y: YieldOnce },
    Done,
}
impl AddDouble {
    pub fn new(a: u32, b: u32) -> Self {
        AddDouble::Start { a, b }
    }
}
impl Future for AddDouble {
    type Output = u32;
    /// Start -> compute sum, create YieldOnce, go to Waiting and continue (no Pending yet).
    /// Waiting -> poll y; Pending => return Pending (stay), Ready => Done, return Ready(sum*2).
    /// Done -> panic "polled after completion" (what real async fns do).
    /// (All fields are Unpin, so self.get_mut() is fine. std::mem::replace helps to move out of the enum.)
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn polls<F: Future + Unpin>(mut f: F) -> (usize, F::Output) {
        let w = futures::task::noop_waker();
        let mut cx = Context::from_waker(&w);
        let mut n = 0;
        loop {
            n += 1;
            if let Poll::Ready(v) = Pin::new(&mut f).poll(&mut cx) {
                return (n, v);
            }
        }
    }
    #[test]
    fn async05_same_result_same_polls_as_async_fn() {
        let (n_hand, v_hand) = polls(AddDouble::new(2, 3));
        let (n_async, v_async) = polls(Box::pin(add_double(2, 3)));
        assert_eq!((n_hand, v_hand), (n_async, v_async));
        assert_eq!((n_hand, v_hand), (2, 10));
    }
    #[test]
    #[should_panic(expected = "polled after completion")]
    fn async05_poll_after_done() {
        let mut f = AddDouble::new(1, 1);
        let w = futures::task::noop_waker();
        let mut cx = Context::from_waker(&w);
        while Pin::new(&mut f).poll(&mut cx).is_pending() {}
        let _ = Pin::new(&mut f).poll(&mut cx);
    }
}
