//! Ch06 Enums & pattern matching (book: ch06-enums-and-pattern-matching.md)
//! Goals: enums with data, exhaustive match, Option, if let, match guards, destructuring.

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64), // radius
    Rect { w: f64, h: f64 },
    Triangle(f64, f64, f64), // sides
}

/// Area. Triangle via Heron's formula. std::f64::consts::PI.
pub fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect { w, h } => w * h,
        Shape::Triangle(a, b, c) => {
            let p = (a + b + c) / 2.0;
            (p * (p - a) * (p - b) * (p - c)).sqrt()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Cmd {
    Push(i32),
    Pop,
    Add,
    Dup,
}

/// Run commands on a stack. Pop/Add/Dup on too-small stack: return None. Else Some(final stack).
pub fn run(cmds: &[Cmd]) -> Option<Vec<i32>> {
    let mut stack: Vec<i32> = Vec::new();
    for c in cmds {
        match c {
            Cmd::Push(x) => {
                stack.push(*x);
            }
            Cmd::Pop => {
                if stack.is_empty() {
                    return None;
                }
                stack.pop();
            }
            Cmd::Add => {
                let lhs = stack.pop()?;
                let rhs = stack.pop()?;
                stack.push(lhs + rhs);
            }
            Cmd::Dup => {
                stack.push(*stack.last()?);
            }
        };
    }
    Some(stack)
}

/// Classify using match guards: "negative", "zero", "small" (1..=9), "large".
pub fn classify(n: i32) -> &'static str {
    match n {
        n if n < 0 => "negative",
        0 => "zero",
        n if n >= 1 && n <= 9 => "small",
        _ => "large",
    }
}

/// Use `if let` / combinators: parse the string as i32 and double it; None if not a number.
pub fn double_parsed(s: &str) -> Option<i32> {
    s.parse::<i32>().map(|x| x * 2).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch06_area() {
        assert!((area(&Shape::Circle(1.0)) - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(area(&Shape::Rect { w: 2.0, h: 3.0 }), 6.0);
        assert!((area(&Shape::Triangle(3.0, 4.0, 5.0)) - 6.0).abs() < 1e-9);
    }
    #[test]
    fn ch06_run() {
        assert_eq!(run(&[Cmd::Push(2), Cmd::Dup, Cmd::Add]), Some(vec![4]));
        assert_eq!(run(&[Cmd::Push(1), Cmd::Add]), None);
        assert_eq!(run(&[Cmd::Pop]), None);
    }
    #[test]
    fn ch06_classify() {
        assert_eq!(classify(-3), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(5), "small");
        assert_eq!(classify(10), "large");
    }
    #[test]
    fn ch06_parse() {
        assert_eq!(double_parsed("21"), Some(42));
        assert_eq!(double_parsed("x"), None);
    }
}
