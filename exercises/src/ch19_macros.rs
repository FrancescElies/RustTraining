//! Ch19 Macros (book: ch19-macros.md)
//! Goals: macro_rules! patterns, repetition $(...),*, recursion, hygiene, stringify!, generating impls.
//! Macros must be defined BEFORE use in the file. Stubs expand to todo!(): fix the bodies.
use std::collections::HashMap;

/// 1. square!(x) -> x * x. Hygiene trap: square!(2 + 1) must be 9, not 5. (why does $x:expr save you?)
macro_rules! square {
    ($x:expr) => {
        todo!()
    };
}

/// 2. max!(a), max!(a, b), max!(a, b, c, ...): recursive. Use std::cmp::max.
macro_rules! max {
    ($x:expr) => {
        todo!()
    };
    ($x:expr, $($rest:expr),+) => {
        todo!()
    };
}

/// 3. hashmap!{ "a" => 1, "b" => 2 } -> HashMap. Trailing comma allowed. Repetition: $( $k:expr => $v:expr ),* $(,)?
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {
        todo!()
    };
}

/// 4. check!(cond, "fmt {}", args...) -> Result<(), String>. Err(format!(...)) when cond is false.
macro_rules! check {
    ($cond:expr, $($arg:tt)+) => {
        todo!()
    };
}

/// 5. Generate trait impls for many types at once; describe() returns the type name via stringify!.
pub trait Describe {
    fn describe(&self) -> String;
}
macro_rules! impl_describe {
    ($($t:ty),* $(,)?) => {
        $( impl Describe for $t { fn describe(&self) -> String { todo!() /* stringify!($t) */ } } )*
    };
}
impl_describe!(i32, f64, bool);

/// 6. Plain function: when is a function better than a macro? Implement it, then answer in a comment below.
pub fn square_fn(x: i32) -> i32 {
    todo!()
}
// ANSWER: (write 2 sentences: when do you actually need a macro?)

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch19_square() {
        let a: i32 = square!(4);
        let b: i32 = square!(2 + 1);
        assert_eq!((a, b), (16, 9));
    }
    #[test]
    fn ch19_max() {
        let a: i32 = max!(7);
        let b: i32 = max!(1, 5, 3);
        assert_eq!((a, b), (7, 5));
    }
    #[test]
    fn ch19_hashmap() {
        let m: HashMap<&str, i32> = hashmap! { "a" => 1, "b" => 2, };
        assert_eq!((m.len(), m["a"], m["b"]), (2, 1, 2));
        let e: HashMap<&str, i32> = hashmap! {};
        assert!(e.is_empty());
    }
    #[test]
    fn ch19_check() {
        let ok: Result<(), String> = check!(1 < 2, "never {}", 1);
        let bad: Result<(), String> = check!(1 > 2, "bad value {}", 42);
        assert_eq!((ok, bad), (Ok(()), Err("bad value 42".to_string())));
    }
    #[test]
    fn ch19_describe() {
        assert_eq!(
            (1i32.describe(), 1.0f64.describe(), true.describe()),
            ("i32".into(), "f64".into(), "bool".into())
        );
    }
    #[test]
    fn ch19_fn() {
        assert_eq!(square_fn(5), 25);
    }
}
