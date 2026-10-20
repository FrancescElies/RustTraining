//! Ch03 Built-in types  (book: c-cpp-book/src/ch03-built-in-types.md)
//! Goals: integer types, no implicit casts, tuples, arrays, slices, &str vs String.

/// Convert Celsius to Fahrenheit. (hint: f64 * 9/5 + 32; literals need to be floats)
pub fn c_to_f(c: f64) -> f64 {
    todo!()
}

/// Add two u8 without overflow panic/wrap: return None on overflow. (hint: checked_add)
pub fn safe_add(a: u8, b: u8) -> Option<u8> {
    todo!()
}

/// Return (min, max) of a non-empty slice. Tuples!
pub fn min_max(v: &[i32]) -> (i32, i32) {
    todo!()
}

/// Return the first word of a string slice (split on ' '); "" if empty. No allocation: return &str.
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// Convert an i64 to u8, saturating: <0 -> 0, >255 -> 255. (hint: `as` truncates! use comparisons or clamp)
pub fn to_u8_saturating(x: i64) -> u8 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch03_temp() {
        assert_eq!(c_to_f(100.0), 212.0);
        assert_eq!(c_to_f(0.0), 32.0);
    }
    #[test]
    fn ch03_add() {
        assert_eq!(safe_add(200, 55), Some(255));
        assert_eq!(safe_add(200, 56), None);
    }
    #[test]
    fn ch03_minmax() {
        assert_eq!(min_max(&[3, -1, 7, 2]), (-1, 7));
    }
    #[test]
    fn ch03_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("one"), "one");
    }
    #[test]
    fn ch03_sat() {
        assert_eq!(to_u8_saturating(-5), 0);
        assert_eq!(to_u8_saturating(300), 255);
        assert_eq!(to_u8_saturating(42), 42);
    }
}
