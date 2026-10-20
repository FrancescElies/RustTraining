//! Ch18 C++ -> Rust semantic deep dives (book: ch18-cpp-rust-semantic-deep-dives.md)
//! casts, cfg, volatile/atomics, static/const/OnceLock, const fn, trait bounds (vs SFINAE),
//! Box<dyn Fn> (vs std::function), collections, panic safety.
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

/// Casts: checked narrowing. i64 -> i32, None if it does not fit. (TryFrom, not 'as')
pub fn narrow(x: i64) -> Option<i32> {
    todo!()
}
/// Casts: what does 'as' do float -> int? Predict first: 300.5, -1.0, NaN. Then implement with 'as' (u8).
pub fn float_to_u8(f: f64) -> u8 {
    todo!()
}
/// Casts: reinterpret bits of an f32 (C++ memcpy/bit_cast). No unsafe needed.
pub fn float_bits(f: f32) -> u32 {
    todo!()
}

/// Preprocessor -> cfg!. "windows" / "unix" / "other".
pub fn os_family() -> &'static str {
    todo!()
}

/// static: a global counter via atomics (C++ static int + volatile/atomic). First call returns 1, then 2...
pub fn next_id() -> u32 {
    static NEXT: AtomicU32 = AtomicU32::new(1);
    todo!() // NEXT.fetch_add(1, Ordering::Relaxed)
}

/// OnceLock: lazy global initialised exactly once; returns the same &'static each time.
pub fn app_name() -> &'static str {
    static NAME: OnceLock<String> = OnceLock::new();
    todo!() // NAME.get_or_init(|| "rustlab".to_string())
}

/// constexpr -> const fn. Implement with a while loop (for is not allowed in const fn).
pub const fn fact(n: u64) -> u64 {
    0 /* stub: replace */
}

/// SFINAE/enable_if -> trait bounds. Sum a slice of any numeric-like T. Fill in the where clause.
pub fn sum_all<T>(v: &[T]) -> T
where
    T: Copy, // + ??? (Add with Output=T, Default)
{
    todo!()
}

/// std::function -> impl Fn. compose(f, g)(x) == g(f(x)).
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |_a| todo!()
}
/// std::function in a table -> Box<dyn Fn>. Names: "add", "sub", "mul".
pub fn ops() -> Vec<(&'static str, Box<dyn Fn(i32, i32) -> i32>)> {
    vec![] /* stub */
}

/// std::map<size_t, vector<string>> -> BTreeMap (ordered). Group words by length.
pub fn group_by_len(words: &[&str]) -> BTreeMap<usize, Vec<String>> {
    todo!()
}
/// std::set -> BTreeSet. Sorted unique values.
pub fn sorted_unique(v: &[i32]) -> Vec<i32> {
    todo!()
}
/// std::deque -> VecDeque. Keep only the last n items of an iterator (bounded memory).
pub fn last_n(it: impl Iterator<Item = i32>, n: usize) -> Vec<i32> {
    todo!()
}

/// Exception safety -> panic safety. a / b panics on b == 0. Catch it at this boundary: Err("panicked").
/// (std::panic::catch_unwind; closure must be UnwindSafe, hint: AssertUnwindSafe or just Copy ints)
pub fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch18_narrow() {
        assert_eq!(narrow(5), Some(5));
        assert_eq!(narrow(1 << 40), None);
        assert_eq!(narrow(-1), Some(-1));
    }
    #[test]
    fn ch18_float() {
        assert_eq!(float_to_u8(300.5), 255);
        assert_eq!(float_to_u8(-1.0), 0);
        assert_eq!(float_to_u8(f64::NAN), 0);
        assert_eq!(float_to_u8(7.9), 7);
    }
    #[test]
    fn ch18_bits() {
        assert_eq!(float_bits(1.0), 0x3f80_0000);
    }
    #[test]
    fn ch18_os() {
        assert_eq!(
            os_family(),
            if cfg!(windows) {
                "windows"
            } else if cfg!(unix) {
                "unix"
            } else {
                "other"
            }
        );
    }
    #[test]
    fn ch18_id() {
        let a = next_id();
        assert_eq!(a, 1);
        assert_eq!(next_id(), 2);
    }
    #[test]
    fn ch18_once() {
        assert_eq!(app_name(), "rustlab");
        assert!(std::ptr::eq(app_name(), app_name()));
    }
    #[test]
    fn ch18_const() {
        const F5: u64 = fact(5);
        assert_eq!(F5, 120);
        assert_eq!(fact(0), 1);
    }
    #[test]
    fn ch18_sum() {
        assert_eq!(sum_all(&[1, 2, 3]), 6);
        assert_eq!(sum_all(&[0.5, 0.25]), 0.75);
    }
    #[test]
    fn ch18_compose() {
        let f = compose(|x: i32| x + 1, |y: i32| y.to_string());
        assert_eq!(f(4), "5");
    }
    #[test]
    fn ch18_ops() {
        let t = ops();
        let call = |n: &str, a, b| (t.iter().find(|(k, _)| *k == n).unwrap().1)(a, b);
        assert_eq!(
            (call("add", 6, 3), call("sub", 6, 3), call("mul", 6, 3)),
            (9, 3, 18)
        );
    }
    #[test]
    fn ch18_group() {
        let g = group_by_len(&["a", "bb", "c", "dd", "eee"]);
        assert_eq!(g[&1], ["a", "c"]);
        assert_eq!(g[&2], ["bb", "dd"]);
        assert_eq!(g.keys().copied().collect::<Vec<_>>(), [1, 2, 3]);
    }
    #[test]
    fn ch18_unique() {
        assert_eq!(sorted_unique(&[3, 1, 3, 2, 1]), [1, 2, 3]);
    }
    #[test]
    fn ch18_lastn() {
        assert_eq!(last_n(1..=10, 3), [8, 9, 10]);
        assert_eq!(last_n(1..=2, 5), [1, 2]);
    }
    #[test]
    fn ch18_panic() {
        assert_eq!(safe_div(6, 3), Ok(2));
        assert_eq!(safe_div(1, 0), Err("panicked".to_string()));
    }
}
