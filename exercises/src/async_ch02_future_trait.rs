//! Async ch02 The Future trait (book: async-book/src/ch02-the-future-trait.md)
//! Goals: implement Future by hand: poll, Pending/Ready, and the Waker contract
//! ("if you return Pending you MUST arrange for cx.waker() to be woken, or nobody will poll you again").
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Needs n+1 polls: the first n return Pending (and wake the task so it is polled again), the next returns Ready("liftoff").
pub struct Countdown {
    n: u32,
}
impl Countdown {
    pub fn new(n: u32) -> Self {
        Countdown { n }
    }
}
impl Future for Countdown {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        todo!()
    }
}

/// A future that is immediately ready with its value. Polling twice is a bug: panic with "polled after completion".
/// (Option::take; T: Unpin lets you use Pin::get_mut)
pub struct Ready<T>(Option<T>);
impl<T> Ready<T> {
    pub fn new(v: T) -> Self {
        Ready(Some(v))
    }
}
impl<T: Unpin> Future for Ready<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::task::{waker, ArcWake};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct CountWakes(AtomicUsize);
    impl ArcWake for CountWakes {
        fn wake_by_ref(a: &Arc<Self>) {
            a.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn async02_countdown_polls_and_wakes() {
        let counter = Arc::new(CountWakes(AtomicUsize::new(0)));
        let w = waker(counter.clone());
        let mut cx = Context::from_waker(&w);
        let mut f = Countdown::new(3);
        let mut polls = 0;
        let out = loop {
            polls += 1;
            if let Poll::Ready(v) = Pin::new(&mut f).poll(&mut cx) {
                break v;
            }
        };
        assert_eq!(out, "liftoff");
        assert_eq!(polls, 4);
        assert_eq!(
            counter.0.load(Ordering::SeqCst),
            3,
            "each Pending must wake the task"
        );
    }
    #[test]
    fn async02_countdown_in_executor() {
        assert_eq!(futures::executor::block_on(Countdown::new(5)), "liftoff");
        assert_eq!(futures::executor::block_on(Countdown::new(0)), "liftoff");
    }
    #[test]
    fn async02_ready() {
        assert_eq!(futures::executor::block_on(Ready::new(7)), 7);
    }
}
