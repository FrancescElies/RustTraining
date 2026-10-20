//! Ch10 Traits & generics (book: ch10-traits.md, ch10-1-generics.md)
//! Goals: define/implement traits, default methods, generics + bounds, trait objects (dyn).

pub trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;
    /// Default method: "<name> says <sound>"
    fn speak(&self) -> String {
        todo!()
    }
}

pub struct Dog;
pub struct Cat {
    pub name: String,
}
// Dog: name "Dog", sound "woof".  Cat: its name field, "meow".
impl Animal for Dog {
    fn name(&self) -> String {
        todo!()
    }
    fn sound(&self) -> String {
        todo!()
    }
}
impl Animal for Cat {
    fn name(&self) -> String {
        todo!()
    }
    fn sound(&self) -> String {
        todo!()
    }
}

/// Static dispatch via generics. Returns "<speak> <speak>".
pub fn speak_twice<T: Animal>(a: &T) -> String {
    todo!()
}

/// Dynamic dispatch: a heterogeneous slice of trait objects. Return all speak() strings.
pub fn chorus(zoo: &[Box<dyn Animal>]) -> Vec<String> {
    todo!()
}

/// Generic: largest element of a non-empty slice. Add the right bound(s) on T.
pub fn largest<T: PartialOrd>(v: &[T]) -> &T {
    todo!()
}

/// Implement a trait on a foreign type: add `double()` to i32.
pub trait Double {
    fn double(&self) -> Self;
}
impl Double for i32 {
    fn double(&self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch10_speak() {
        assert_eq!(Dog.speak(), "Dog says woof");
        assert_eq!(
            speak_twice(&Cat { name: "Tom".into() }),
            "Tom says meow Tom says meow"
        );
    }
    #[test]
    fn ch10_dyn() {
        let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat { name: "Kit".into() })];
        assert_eq!(chorus(&zoo), ["Dog says woof", "Kit says meow"]);
    }
    #[test]
    fn ch10_largest() {
        assert_eq!(*largest(&[1, 9, 3]), 9);
        assert_eq!(*largest(&["a", "c", "b"]), "c");
    }
    #[test]
    fn ch10_double() {
        assert_eq!(4.double(), 8);
    }
}
