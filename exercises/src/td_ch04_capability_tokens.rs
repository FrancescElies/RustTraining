//! Type-driven ch04 Capability tokens: proof of authority as a value (book: ...ch04-capability-tokens-*.md)
//! Idea: a function that needs a permission takes a reference to a token that only login() can create.

/// Zero-sized: costs nothing at runtime. The private field means code outside this module cannot forge one.
pub struct AdminCap(());
pub struct ReadCap(());

/// Right password is "hunter2".
pub fn login(password: &str) -> Option<AdminCap> {
    todo!()
}

impl AdminCap {
    /// Hierarchy: an admin can derive a weaker read-only capability.
    pub fn downgrade(&self) -> ReadCap {
        todo!()
    }
}

/// Needs admin. Returns "rebooting".
pub fn reboot(_proof: &AdminCap) -> &'static str {
    todo!()
}
/// Needs only read. Returns 42.
pub fn read_temp(_proof: &ReadCap) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td04_login() {
        assert!(login("nope").is_none());
        assert!(login("hunter2").is_some());
    }
    #[test]
    fn td04_use() {
        let admin = login("hunter2").unwrap();
        let ro = admin.downgrade();
        assert_eq!(read_temp(&ro), 42);
        assert_eq!(reboot(&admin), "rebooting");
        assert_eq!(size_of::<AdminCap>(), 0);
        // reboot(&ro); // does not compile: ReadCap is not AdminCap
    }
}
