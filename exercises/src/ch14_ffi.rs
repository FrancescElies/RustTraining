//! Ch14 Unsafe & FFI (book: ch14-unsafe-rust-and-ffi.md)
//! Goals: raw pointers, calling C (libc), CString/CStr, #[repr(C)], extern "C" fns,
//! opaque handles with Box::into_raw/from_raw, C callbacks.
//! Note: a panic inside an `extern "C"` fn aborts the process, so those stubs return dummies, not todo!().
//! Rule: every `unsafe` block gets a `// Safety:` comment saying why it's sound.
use std::ffi::{c_char, c_int, c_void, CStr, CString, NulError};

// Real C functions from the C runtime, already linked into every Rust binary.
extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn abs(x: c_int) -> c_int;
    fn qsort(
        base: *mut c_void,
        n: usize,
        size: usize,
        cmp: extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
}

/// 1. Swap two i32 through raw pointers (no std::mem::swap). Caller guarantees validity.
///    Why is this fn `unsafe`?
pub unsafe fn swap_raw(a: *mut i32, b: *mut i32) {
    todo!()
}

/// 2. Call C `abs` from a safe wrapper.
pub fn c_abs(x: i32) -> i32 {
    todo!()
}

/// 3. Safe wrapper over C `strlen`: build a CString (fails on interior NUL -> propagate the error),
///    pass `.as_ptr()`. Keep the CString alive across the call!
pub fn c_strlen(s: &str) -> Result<usize, NulError> {
    todo!()
}

/// 4. Receive a C string: copy it into a Rust String (lossy UTF-8). Null pointer -> None.
///    (hint: CStr::from_ptr, to_string_lossy)
/// # Safety
/// `p` must be null or point to a valid NUL-terminated string.
pub unsafe fn from_c_str(p: *const c_char) -> Option<String> {
    todo!()
}

/// 5. Transparent struct shared with C: layout must match C.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Exported to C: C can call this by name. Reads a Point through a pointer; null -> -1.
#[no_mangle]
pub extern "C" fn point_manhattan(p: *const Point) -> i64 {
    -1 // stub: replace (panics can't cross extern "C", so no todo!() here)
}

/// 6. Opaque handle: C only holds a `*mut Counter`, never looks inside.
pub struct Counter {
    n: u32,
}
/// Allocate on the heap and give ownership to the caller as a raw pointer. (Box::into_raw)
#[no_mangle]
pub extern "C" fn counter_new() -> *mut Counter {
    std::ptr::null_mut() // stub: replace
}
/// Increment and return the new value. Null -> 0.
#[no_mangle]
pub extern "C" fn counter_inc(c: *mut Counter) -> u32 {
    0 // stub: replace
}
/// Take ownership back and free. Null is a no-op. (Box::from_raw) Calling twice = double free!
#[no_mangle]
pub extern "C" fn counter_free(c: *mut Counter) {
    // stub: replace
}

/// 7. Callback: sort a slice of i32 with C `qsort`. Write the `extern "C"` comparator below.
extern "C" fn cmp_i32(a: *const c_void, b: *const c_void) -> c_int {
    0 // stub: replace. read both as *const i32; return -1/0/1 (don't subtract: overflow!)
}
pub fn c_sort(v: &mut [i32]) {
    todo!() // qsort(v.as_mut_ptr().cast(), v.len(), size_of::<i32>(), cmp_i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch14_swap() {
        let (mut a, mut b) = (1, 2);
        unsafe { swap_raw(&mut a, &mut b) };
        assert_eq!((a, b), (2, 1));
    }
    #[test]
    fn ch14_abs() {
        assert_eq!(c_abs(-7), 7);
    }
    #[test]
    fn ch14_strlen() {
        assert_eq!(c_strlen("hello"), Ok(5));
        assert!(c_strlen("a\0b").is_err());
    }
    #[test]
    fn ch14_cstr() {
        let c = CString::new("héllo").unwrap();
        assert_eq!(unsafe { from_c_str(c.as_ptr()) }.as_deref(), Some("héllo"));
        assert_eq!(unsafe { from_c_str(std::ptr::null()) }, None);
    }
    #[test]
    fn ch14_point() {
        assert_eq!(point_manhattan(&Point { x: -3, y: 4 }), 7);
        assert_eq!(point_manhattan(std::ptr::null()), -1);
        assert_eq!(size_of::<Point>(), 8);
    }
    #[test]
    fn ch14_counter() {
        let c = counter_new();
        assert_eq!(counter_inc(c), 1);
        assert_eq!(counter_inc(c), 2);
        counter_free(c);
        counter_free(std::ptr::null_mut());
        assert_eq!(counter_inc(std::ptr::null_mut()), 0);
    }
    #[test]
    fn ch14_sort() {
        let mut v = [3, -1, i32::MAX, i32::MIN, 0];
        c_sort(&mut v);
        assert_eq!(v, [i32::MIN, -1, 0, 3, i32::MAX]);
    }
}
