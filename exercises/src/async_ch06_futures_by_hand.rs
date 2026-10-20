//! Async ch06 Building futures by hand: Join and Select (book: async-book/src/ch06-building-futures-by-hand.md)
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Provided: Pending n times, then Ready(value). Polling after Ready panics (like real futures).
pub struct Delay<T> {
    left: u32,
    value: Option<T>,
}
impl<T> Delay<T> {
    pub fn new(left: u32, value: T) -> Self {
        Delay {
            left,
            value: Some(value),
        }
    }
}
impl<T: Unpin> Future for Delay<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        if self.left == 0 {
            Poll::Ready(self.value.take().expect("polled after completion"))
        } else {
            self.left -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Run both futures concurrently, finish when BOTH are done, return both outputs.
/// Keep each result in its Option once available and never poll a finished future again.
pub struct Join2<A: Future, B: Future> {
    a: A,
    b: B,
    ra: Option<A::Output>,
    rb: Option<B::Output>,
}
impl<A: Future, B: Future> Join2<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Join2 {
            a,
            b,
            ra: None,
            rb: None,
        }
    }
}
impl<A, B> Future for Join2<A, B>
where
    A: Future + Unpin,
    B: Future + Unpin,
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
/// Finish as soon as EITHER is done. Poll a first, then b (so on a tie, Left wins). The loser is simply dropped.
pub struct Select2<A, B> {
    pub a: A,
    pub b: B,
}
impl<A: Future + Unpin, B: Future + Unpin> Future for Select2<A, B> {
    type Output = Either<A::Output, B::Output>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
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
    fn async06_join() {
        assert_eq!(
            block_on(Join2::new(Delay::new(2, "a"), Delay::new(1, 7))),
            ("a", 7)
        );
    }
    #[test]
    fn async06_join_is_concurrent() {
        // max(2,1)+1 = 3 polls, not 2+1+2 sequential
        let (n, _) = polls(Join2::new(Delay::new(2, 'a'), Delay::new(1, 'b')));
        assert_eq!(n, 3);
    }
    #[test]
    fn async06_select() {
        assert_eq!(
            block_on(Select2 {
                a: Delay::new(3, "a"),
                b: Delay::new(1, "b")
            }),
            Either::Right("b")
        );
        assert_eq!(
            block_on(Select2 {
                a: Delay::new(1, "a"),
                b: Delay::new(1, "b")
            }),
            Either::Left("a")
        );
    }
}
