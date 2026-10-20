//! Patterns ch01 Generics (book: rust-patterns-book/src/ch01-generics-the-full-picture.md)
//! const generics, trait-bound generics, const fn.

/// Fixed-capacity cache with least-recently-used eviction. N is a const generic.
/// items is ordered oldest -> newest.
pub struct Cache<K, V, const N: usize> {
    items: Vec<(K, V)>,
}
impl<K: PartialEq, V, const N: usize> Cache<K, V, N> {
    pub fn new() -> Self {
        Cache {
            items: Vec::with_capacity(N),
        }
    }
    /// Insert or update (an updated key becomes newest). If the cache would exceed N, evict the oldest
    /// and return it.
    pub fn put(&mut self, k: K, v: V) -> Option<(K, V)> {
        todo!()
    }
    /// Lookup; a hit becomes newest.
    pub fn get(&mut self, k: &K) -> Option<&V> {
        todo!()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

/// Element with the largest key. None if empty. (generic over T and K: Ord, closure Fn(&T) -> K)
pub fn max_by_key_ref<T, K: Ord>(v: &[T], key: impl Fn(&T) -> K) -> Option<&T> {
    todo!()
}

/// const fn: number of bits needed to represent n (0 -> 0, 1 -> 1, 255 -> 8, 256 -> 9). Use a while loop.
pub const fn bit_width(n: u32) -> u32 {
    0 /* stub: replace */
}

/// Matrix with const-generic dimensions: a transpose changes the type from R x C to C x R.
#[derive(Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize>(pub [[i32; C]; R]);
impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn transpose(&self) -> Matrix<C, R> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat01_cache() {
        let mut c: Cache<&str, i32, 2> = Cache::new();
        assert_eq!(c.put("a", 1), None);
        assert_eq!(c.put("b", 2), None);
        assert_eq!(c.get(&"a"), Some(&1)); // a is now newest, b oldest
        assert_eq!(c.put("c", 3), Some(("b", 2)));
        assert_eq!(c.put("a", 10), None); // update, no eviction
        assert_eq!(c.len(), 2);
        assert_eq!(c.get(&"a"), Some(&10));
        assert_eq!(c.get(&"b"), None);
    }
    #[test]
    fn pat01_max_by_key() {
        assert_eq!(
            max_by_key_ref(&["aa", "b", "cccc"], |s| s.len()),
            Some(&"cccc")
        );
        assert_eq!(max_by_key_ref(&[] as &[i32], |x| *x), None);
    }
    #[test]
    fn pat01_bit_width() {
        const W: u32 = bit_width(255);
        assert_eq!(
            (bit_width(0), bit_width(1), W, bit_width(256)),
            (0, 1, 8, 9)
        );
    }
    #[test]
    fn pat01_matrix() {
        let m = Matrix([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(m.transpose(), Matrix([[1, 4], [2, 5], [3, 6]]));
    }
}
