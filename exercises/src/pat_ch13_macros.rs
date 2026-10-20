//! Patterns ch13 Macros (book: rust-patterns-book/src/ch13-macros-code-that-writes-code.md)
//! (ch19_macros covers the basics; here: tt munching and generating items.)

/// count!(a b c) -> 3usize. Recursive tt munching: count!() = 0, count!(h rest...) = 1 + count!(rest...)
macro_rules! count {
    () => {
        0usize
    };
    ($h:tt $($t:tt)*) => {
        todo!()
    };
}

/// strs!["a", "b"] -> Vec<String>
macro_rules! strs {
    ($($x:expr),* $(,)?) => {
        todo!()
    };
}

/// Generate an enum plus as_str() (variant name via stringify!) and all() (every variant, in order).
macro_rules! enum_str {
    ($name:ident { $($v:ident),+ $(,)? }) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $name { $($v),+ }
        impl $name {
            pub fn as_str(&self) -> &'static str { match self { $(Self::$v => todo!()),+ } }
            pub fn all() -> Vec<$name> { todo!() }
        }
    };
}
enum_str!(Color { Red, Green, Blue });

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat13_count() {
        let a: usize = count!();
        let b: usize = count!(x y z);
        assert_eq!((a, b), (0, 3));
    }
    #[test]
    fn pat13_strs() {
        let v: Vec<String> = strs!["a", "b",];
        assert_eq!(v, ["a", "b"]);
    }
    #[test]
    fn pat13_enum() {
        assert_eq!(Color::Green.as_str(), "Green");
        assert_eq!(Color::all(), [Color::Red, Color::Green, Color::Blue]);
    }
}
