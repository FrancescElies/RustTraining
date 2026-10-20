//! Async ch11 Streams (book: async-book/src/ch11-streams-and-asynciterator.md)
//! A Stream is an async Iterator. Uses futures::stream. Exercise 5 of the book's exercise chapter lives here too.
use futures::stream::{self, Stream, StreamExt};
use std::time::Duration;

/// Provided: pretend I/O. 100ms per item, returns id*10.
pub async fn work(id: u32) -> u32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    id * 10
}

/// Squares of the even numbers in 0..n. (stream::iter + filter + map; closures in filter/map are sync, use futures::future::ready in filter)
pub fn evens_squared(n: u32) -> impl Stream<Item = u32> {
    stream::empty() // stub
}

/// n, n-1, ..., 1 using stream::unfold (state = current number).
pub fn countdown(n: u32) -> impl Stream<Item = u32> {
    stream::empty() // stub
}

/// Sum all items (fold).
pub async fn total(s: impl Stream<Item = u32>) -> u32 {
    todo!()
}

/// Group items into Vecs of at most 'size'. (StreamExt::chunks)
pub async fn batches(s: impl Stream<Item = u32>, size: usize) -> Vec<Vec<u32>> {
    todo!()
}

/// Run work() for every id with at most 'parallel' in flight, results IN INPUT ORDER.
/// (stream::iter(ids).map(work).buffered(parallel).collect())
pub async fn fetch_all(ids: Vec<u32>, parallel: usize) -> Vec<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn async11_evens() {
        assert_eq!(evens_squared(7).collect::<Vec<_>>().await, [0, 4, 16, 36]);
    }
    #[tokio::test]
    async fn async11_countdown() {
        assert_eq!(countdown(3).collect::<Vec<_>>().await, [3, 2, 1]);
        assert_eq!(countdown(0).collect::<Vec<_>>().await, Vec::<u32>::new());
    }
    #[tokio::test]
    async fn async11_total_and_batches() {
        assert_eq!(total(stream::iter([1, 2, 3])).await, 6);
        assert_eq!(
            batches(stream::iter(1..=5), 2).await,
            [vec![1, 2], vec![3, 4], vec![5]]
        );
    }
    #[tokio::test(start_paused = true)]
    async fn async11_buffered() {
        let t = tokio::time::Instant::now();
        let out = fetch_all(vec![1, 2, 3, 4, 5, 6], 3).await;
        assert_eq!(out, [10, 20, 30, 40, 50, 60]);
        assert_eq!(t.elapsed(), Duration::from_millis(200)); // 2 waves of 3
    }
}
