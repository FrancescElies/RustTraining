//! Patterns ch14 Testing patterns: proptest, mocking without frameworks
//! (book: rust-patterns-book/src/ch14-testing-and-benchmarking-patterns.md)
//! The tests are property-based: proptest throws hundreds of random inputs at YOUR code and shrinks failures.
//! Part 3 is different: YOU write the property.
use std::cell::Cell;

// ---- 1. Invariant: a Vec that is always sorted ----
#[derive(Debug, Default)]
pub struct SortedVec<T: Ord> {
    items: Vec<T>,
}
impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        SortedVec { items: Vec::new() }
    }
    /// Insert keeping order (duplicates allowed). Use binary_search to find the position.
    pub fn insert(&mut self, v: T) {
        todo!()
    }
    pub fn contains(&self, v: &T) -> bool {
        todo!()
    }
    /// Remove one occurrence. true if something was removed.
    pub fn remove(&mut self, v: &T) -> bool {
        todo!()
    }
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }
}

// ---- 2. Round-trip property: decode(encode(x)) == x ----
/// Run-length encoding as (count, byte) pairs, count 1..=255. Runs longer than 255 split into several pairs.
/// encode(&[7,7,7,1]) == [3,7,1,1]
pub fn rle_encode(data: &[u8]) -> Vec<u8> {
    todo!()
}
/// Odd length or a zero count -> Err.
pub fn rle_decode(data: &[u8]) -> Result<Vec<u8>, String> {
    todo!()
}

// ---- 3. YOU write the property ----
/// Does `output` look like a correct sort of `input`? (Two conditions. Think: what could a buggy sort get wrong
/// while still returning something that "looks sorted"?)
pub fn is_valid_sort(input: &[i32], output: &[i32]) -> bool {
    todo!()
}
/// Provided, correct.
pub fn good_sort(v: &[i32]) -> Vec<i32> {
    let mut o = v.to_vec();
    o.sort();
    o
}
/// Provided, BUGGY: sorted output, but it silently drops duplicates. Your property must catch this.
pub fn buggy_sort(v: &[i32]) -> Vec<i32> {
    let mut o = v.to_vec();
    o.sort();
    o.dedup();
    o
}

// ---- 4. Mocking without a framework: inject the dependency as a trait ----
pub trait Clock {
    fn now(&self) -> u64;
}
/// Allow at most `max` calls per `window` seconds (sliding window). A hit older than `window` no longer counts:
/// a hit at t counts while now - t < window.
pub struct RateLimiter<C: Clock> {
    clock: C,
    max: usize,
    window: u64,
    hits: Vec<u64>,
}
impl<C: Clock> RateLimiter<C> {
    pub fn new(clock: C, max: usize, window: u64) -> Self {
        RateLimiter {
            clock,
            max,
            window,
            hits: vec![],
        }
    }
    pub fn allow(&mut self) -> bool {
        todo!()
    }
}
pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}
/// Test double: time only moves when the test says so.
pub struct FakeClock(pub Cell<u64>);
impl Clock for &FakeClock {
    fn now(&self) -> u64 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::{Config, TestRunner};

    fn cfg() -> ProptestConfig {
        // no regression files written into the repo
        ProptestConfig {
            failure_persistence: None,
            ..ProptestConfig::default()
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn pat14_sorted_always_sorted(values in proptest::collection::vec(-1000i32..1000, 0..100)) {
            let mut s = SortedVec::new();
            for v in &values { s.insert(*v); }
            prop_assert!(s.as_slice().windows(2).all(|w| w[0] <= w[1]));
            prop_assert_eq!(s.as_slice().len(), values.len());
        }

        #[test]
        fn pat14_sorted_contains_and_remove(values in proptest::collection::vec(0i32..50, 1..30), probe in 0i32..50) {
            let mut s = SortedVec::new();
            for v in &values { s.insert(*v); }
            prop_assert_eq!(s.contains(&probe), values.contains(&probe));
            let had = values.contains(&probe);
            prop_assert_eq!(s.remove(&probe), had);
            prop_assert_eq!(s.as_slice().len(), values.len() - had as usize);
        }

        #[test]
        fn pat14_rle_roundtrip(data in proptest::collection::vec(0u8..3, 0..700)) {
            // small alphabet => long runs, which is where the 255 limit bites
            prop_assert_eq!(rle_decode(&rle_encode(&data)), Ok(data));
        }

        #[test]
        fn pat14_rle_pairs_valid(data in proptest::collection::vec(any::<u8>(), 0..200)) {
            let e = rle_encode(&data);
            prop_assert_eq!(e.len() % 2, 0);
            prop_assert!(e.chunks(2).all(|p| p[0] >= 1));
        }
    }

    #[test]
    fn pat14_rle_examples() {
        assert_eq!(rle_encode(&[7, 7, 7, 1]), [3, 7, 1, 1]);
        assert_eq!(rle_encode(&vec![9; 300]), [255, 9, 45, 9]);
        assert_eq!(rle_encode(&[]), Vec::<u8>::new());
        assert!(rle_decode(&[1]).is_err());
        assert!(rle_decode(&[0, 5]).is_err());
    }

    #[test]
    fn pat14_your_property_catches_the_bug() {
        let strat = proptest::collection::vec(-5i32..5, 0..20);
        let mut runner = TestRunner::new(Config {
            failure_persistence: None,
            ..Config::default()
        });
        // A correct property accepts the good implementation...
        let good = runner.run(&strat, |v| {
            prop_assert!(is_valid_sort(&v, &good_sort(&v)));
            Ok(())
        });
        assert!(
            good.is_ok(),
            "your property rejects a correct sort: {good:?}"
        );
        // ...and rejects the buggy one.
        let bad = runner.run(&strat, |v| {
            prop_assert!(is_valid_sort(&v, &buggy_sort(&v)));
            Ok(())
        });
        assert!(
            bad.is_err(),
            "your property did not notice the dedup bug: strengthen it"
        );
    }

    #[test]
    fn pat14_rate_limiter_with_fake_clock() {
        let clock = FakeClock(Cell::new(100));
        let mut rl = RateLimiter::new(&clock, 2, 10);
        assert!(rl.allow());
        assert!(rl.allow());
        assert!(!rl.allow()); // 2 in window already
        clock.0.set(109);
        assert!(!rl.allow()); // 109 - 100 = 9 < 10: still counts
        clock.0.set(110);
        assert!(rl.allow()); // the hits at 100 expired
        assert!(rl.allow());
        assert!(!rl.allow());
    }
}
