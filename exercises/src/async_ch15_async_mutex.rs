//! Async ch15 Exercise 4: build an async Mutex from scratch (book: async-book/src/ch15-exercises.md)
//! Hard (3 stars). Waiters register their Waker; unlock wakes ONE waiter. Never hold the std Mutex across .await.
use std::cell::UnsafeCell;
use std::collections::VecDeque;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

struct State {
    locked: bool,
    waiters: VecDeque<Waker>,
}

pub struct AsyncMutex<T> {
    state: Mutex<State>,
    value: UnsafeCell<T>,
}
// Safety: access to value is serialized by the logical lock in state.locked (only the Guard holder touches it).
unsafe impl<T: Send> Sync for AsyncMutex<T> {}

impl<T> AsyncMutex<T> {
    pub fn new(v: T) -> Self {
        AsyncMutex {
            state: Mutex::new(State {
                locked: false,
                waiters: VecDeque::new(),
            }),
            value: UnsafeCell::new(v),
        }
    }
    pub fn lock(&self) -> LockFuture<'_, T> {
        LockFuture { m: self }
    }
}

pub struct LockFuture<'a, T> {
    m: &'a AsyncMutex<T>,
}
impl<'a, T> Future for LockFuture<'a, T> {
    type Output = Guard<'a, T>;
    /// Lock state. Not locked: set locked, Ready(Guard). Locked: push cx.waker().clone() and return Pending.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Guard<'a, T>> {
        todo!()
    }
}

pub struct Guard<'a, T> {
    m: &'a AsyncMutex<T>,
}
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        todo!() // Safety comment please: why is this access exclusive?
    }
}
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        todo!()
    }
}
impl<T> Drop for Guard<'_, T> {
    /// Unlock and wake ONE waiter (pop_front().wake()). Drop the std lock before waking.
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test(start_paused = true)]
    async fn async15_no_lost_updates() {
        let m = Arc::new(AsyncMutex::new(0u32));
        let mut hs = vec![];
        for _ in 0..10 {
            let m = m.clone();
            hs.push(tokio::spawn(async move {
                for _ in 0..100 {
                    let mut g = m.lock().await;
                    let v = *g;
                    tokio::task::yield_now().await; // guard held across an await: only an async mutex allows this
                    *g = v + 1;
                }
            }));
        }
        // a lost wake-up would hang forever: the timeout turns that into a failure
        tokio::time::timeout(Duration::from_secs(5), async {
            for h in hs {
                h.await.unwrap();
            }
        })
        .await
        .expect("deadlock / lost wake-up");
        assert_eq!(*m.lock().await, 1000);
    }
}
