//! Type-driven ch09 Phantom types for resource tracking (book: ...ch09-phantom-types-*.md)
use std::marker::PhantomData;

pub struct ReadOnly;
pub struct ReadWrite;

/// A memory region whose access permission lives in the type.
pub struct Region<P> {
    data: Vec<u8>,
    _p: PhantomData<P>,
}

impl Region<ReadWrite> {
    pub fn new(len: usize) -> Self {
        Region {
            data: vec![0; len],
            _p: PhantomData,
        }
    }
    /// false if out of bounds (and nothing written).
    pub fn write(&mut self, off: usize, byte: u8) -> bool {
        todo!()
    }
    /// Lock it: same data, now ReadOnly. Consumes the writable handle.
    pub fn freeze(self) -> Region<ReadOnly> {
        todo!()
    }
}
/// Reading is allowed in every permission state.
impl<P> Region<P> {
    pub fn read(&self, off: usize) -> Option<u8> {
        todo!()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td09_flow() {
        let mut r = Region::new(4);
        assert!(r.write(1, 7));
        assert!(!r.write(4, 1));
        let ro = r.freeze();
        assert_eq!(
            (ro.read(1), ro.read(0), ro.read(9), ro.len()),
            (Some(7), Some(0), None, 4)
        );
        // ro.write(0, 1); // does not compile: no write() on Region<ReadOnly>
    }
}
