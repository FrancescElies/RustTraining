//! Patterns ch12 Unsafe Rust: controlled danger (book: rust-patterns-book/src/ch12-unsafe-rust-controlled-danger.md)
//! Build SAFE APIs on top of unsafe blocks. Each unsafe block needs a // Safety: comment proving why.
//! Tip: cargo +nightly miri test -p exercises pat12   (if you have miri)

/// Reimplement slice::split_at_mut. The borrow checker can't see the halves are disjoint; you can.
/// Panics if mid > len (assert!, so the unsafe code can't be reached with a bad mid).
/// (std::slice::from_raw_parts_mut, as_mut_ptr, add)
pub fn split_at_mut_manual(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    todo!()
}

/// Safe wrapper over get_unchecked: do the bounds check yourself, then use unsafe for the access.
pub fn get_checked(s: &[u8], i: usize) -> Option<u8> {
    todo!()
}

/// View u32s as bytes, no copy. len in bytes = len * 4. (from_raw_parts on a *const u8)
pub fn as_bytes(v: &[u32]) -> &[u8] {
    todo!()
}

/// Swap first and last element IN PLACE using raw pointers (ptr::swap). Empty/1-element: do nothing.
pub fn swap_ends(s: &mut [i32]) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pat12_split() {
        let mut v = [1, 2, 3, 4, 5];
        let (a, b) = split_at_mut_manual(&mut v, 2);
        a[0] = 10;
        b[0] = 30;
        assert_eq!((a.len(), b.len()), (2, 3));
        assert_eq!(v, [10, 2, 30, 4, 5]);
        let mut e: [i32; 0] = [];
        let (x, y) = split_at_mut_manual(&mut e, 0);
        assert!(x.is_empty() && y.is_empty());
    }
    #[test]
    fn pat12_split_oob() {
        // must be YOUR assert!, not the stub's todo!()
        let r = std::panic::catch_unwind(|| {
            let mut v = [1];
            split_at_mut_manual(&mut v, 2);
        });
        let p = r.unwrap_err();
        let msg = p
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        assert!(
            !msg.contains("not yet implemented") && !msg.is_empty(),
            "expected an assert! panic, got {msg:?}"
        );
    }
    #[test]
    fn pat12_get() {
        assert_eq!(get_checked(&[1, 2], 1), Some(2));
        assert_eq!(get_checked(&[1, 2], 2), None);
    }
    #[test]
    fn pat12_bytes() {
        let v = [1u32, 0x0102_0304];
        let mut want = 1u32.to_ne_bytes().to_vec();
        want.extend(0x0102_0304u32.to_ne_bytes());
        assert_eq!(as_bytes(&v), &want[..]);
    }
    #[test]
    fn pat12_swap() {
        let mut v = [1, 2, 3];
        swap_ends(&mut v);
        assert_eq!(v, [3, 2, 1]);
        let mut w = [9];
        swap_ends(&mut w);
        assert_eq!(w, [9]);
        swap_ends(&mut []);
    }
}
