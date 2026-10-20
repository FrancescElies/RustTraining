//! Ch04 Control flow (book: ch04-control-flow.md)
//! Goals: if/else as expression, loop/while/for, ranges, break with value, match.

/// FizzBuzz for one number: "Fizz" (3), "Buzz" (5), "FizzBuzz" (both), else the number. Use `match` on a tuple.
pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

/// Sum of all multiples of 3 or 5 below `n`. Use a `for` over a range.
pub fn sum_multiples(n: u32) -> u32 {
    todo!()
}

/// Collatz steps to reach 1. Use `loop` and `break` with a value (or a while).
pub fn collatz_steps(mut n: u64) -> u32 {
    todo!()
}

/// Return the first index where v[i] == target, using a labeled-free loop with early return.
pub fn find(v: &[i32], target: i32) -> Option<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch04_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }
    #[test]
    fn ch04_sum() {
        assert_eq!(sum_multiples(10), 23);
    }
    #[test]
    fn ch04_collatz() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(6), 8);
    }
    #[test]
    fn ch04_find() {
        assert_eq!(find(&[4, 5, 6], 6), Some(2));
        assert_eq!(find(&[4], 9), None);
    }
}
