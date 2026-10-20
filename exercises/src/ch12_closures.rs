//! Ch12 Closures & iterators (book: ch12-closures.md, ch12-1-iterator-power-tools.md)
//! Goals: closures capturing, Fn/FnMut/FnOnce, returning closures, iterator chains.

/// Apply `f` twice.
pub fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    todo!()
}

/// Return a closure that adds `n`. (hint: `impl Fn` + `move`)
pub fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |_x| todo!()
}

/// Counter closure: first call 1, then 2, ... (FnMut, boxed)
pub fn counter() -> Box<dyn FnMut() -> u32> {
    todo!()
}

/// Squares of the even numbers, no loops: filter + map + collect.
pub fn even_squares(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// Words longer than `n` chars, uppercased, joined by ",". Iterator chain.
pub fn long_words(s: &str, n: usize) -> String {
    todo!()
}

/// Sum of the first `k` Fibonacci numbers (0,1,1,2,...). (hint: std::iter::successors, take, sum)
pub fn fib_sum(k: usize) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch12_twice() {
        assert_eq!(apply_twice(|x| x * 3, 2), 18);
    }
    #[test]
    fn ch12_adder() {
        assert_eq!(adder(5)(1), 6);
    }
    #[test]
    fn ch12_counter() {
        let mut c = counter();
        assert_eq!((c(), c(), c()), (1, 2, 3));
    }
    #[test]
    fn ch12_squares() {
        assert_eq!(even_squares(&[1, 2, 3, 4]), [4, 16]);
    }
    #[test]
    fn ch12_words() {
        assert_eq!(long_words("a bbb cc dddd", 2), "BBB,DDDD");
    }
    #[test]
    fn ch12_fib() {
        assert_eq!(fib_sum(7), 12);
    }
}
