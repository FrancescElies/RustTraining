//! Ch15 no_std mindset (book: ch15-no_std-*.md, ch15-1-embedded-deep-dive.md)
//! Real no_std needs a bare-metal target, so here we practise the *style*:
//! only core:: (no heap, no String/Vec/HashMap), fixed-size buffers, const generics, bit twiddling, volatile.
//! Rule for this file: do NOT use std:: or alloc types in your solutions. Use core:: paths.
use core::fmt;

/// 1. Fixed-capacity FIFO ring buffer on the stack. No heap!
pub struct RingBuffer<const N: usize> {
    buf: [u8; N],
    head: usize, // index of the oldest element
    len: usize,
}
impl<const N: usize> RingBuffer<N> {
    pub const fn new() -> Self {
        RingBuffer {
            buf: [0; N],
            head: 0,
            len: 0,
        }
    }
    /// Push to the back. Full -> Err(value) (give it back, don't panic).
    pub fn push(&mut self, v: u8) -> Result<(), u8> {
        todo!()
    }
    /// Pop from the front.
    pub fn pop(&mut self) -> Option<u8> {
        todo!()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

/// 2. Register field helpers. Replace the bits selected by mask with value (value is unshifted; mask is shifted into place).
///    set_field(0b1111_0000, shift=4, width=2, value=0b01) -> 0b0101_0000
pub fn set_field(reg: u32, shift: u32, width: u32, value: u32) -> u32 {
    todo!()
}
pub fn get_field(reg: u32, shift: u32, width: u32) -> u32 {
    todo!()
}

/// 3. Memory-mapped register access: set one bit with a volatile read-modify-write.
///    (core::ptr::read_volatile / write_volatile; why not a plain *reg |= ... ?)
/// # Safety
/// reg must be valid for reads and writes and properly aligned.
pub unsafe fn set_bit_volatile(reg: *mut u32, bit: u32) {
    todo!()
}

/// 4. Wrapping 8-bit checksum (sum of bytes, wrapping). Common in protocols.
pub fn checksum(data: &[u8]) -> u8 {
    todo!()
}

/// 5. Formatting without alloc: implement fmt::Write so write!(buf, ...) works on a stack buffer.
pub struct FixedBuf<const N: usize> {
    buf: [u8; N],
    len: usize,
}
impl<const N: usize> FixedBuf<N> {
    pub const fn new() -> Self {
        FixedBuf {
            buf: [0; N],
            len: 0,
        }
    }
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.len]).unwrap()
    }
}
impl<const N: usize> fmt::Write for FixedBuf<N> {
    /// Append s. If it does not fit, return Err(fmt::Error) and write nothing.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Write;
    #[test]
    fn ch15_ring() {
        let mut r = RingBuffer::<3>::new();
        assert_eq!(r.push(1), Ok(()));
        r.push(2).unwrap();
        r.push(3).unwrap();
        assert_eq!(r.push(4), Err(4));
        assert_eq!(r.pop(), Some(1));
        r.push(5).unwrap(); // wraps around
        assert_eq!(
            (r.pop(), r.pop(), r.pop(), r.pop()),
            (Some(2), Some(3), Some(5), None)
        );
        assert_eq!(r.len(), 0);
    }
    #[test]
    fn ch15_fields() {
        assert_eq!(set_field(0b1111_0000, 4, 2, 0b01), 0b0101_0000);
        assert_eq!(get_field(0b0101_0000, 4, 2), 0b01);
        assert_eq!(get_field(u32::MAX, 0, 32), u32::MAX); // careful: 1 << 32 overflows!
    }
    #[test]
    fn ch15_volatile() {
        let mut r = 0u32;
        unsafe { set_bit_volatile(&mut r, 3) };
        assert_eq!(r, 8);
    }
    #[test]
    fn ch15_checksum() {
        assert_eq!(checksum(&[200, 100, 1]), 45);
    }
    #[test]
    fn ch15_fmt() {
        let mut b = FixedBuf::<8>::new();
        write!(b, "x={}", 42).unwrap();
        assert_eq!(b.as_str(), "x=42");
        assert!(write!(b, "toolongvalue").is_err());
        assert_eq!(b.as_str(), "x=42");
    }
}
