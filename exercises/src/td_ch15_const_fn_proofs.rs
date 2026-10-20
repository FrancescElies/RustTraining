//! Type-driven ch15 Const fn: compile-time correctness proofs (book: ...ch15-const-fn-*.md)
//! A bad memory map should fail the BUILD, not a test.

#[derive(Clone, Copy)]
pub struct Region {
    pub start: u32,
    pub end: u32,
} // [start, end)

impl Region {
    pub const fn new(start: u32, len: u32) -> Region {
        Region {
            start,
            end: start + len,
        }
    }
}

/// True if the two half-open ranges share any address.
pub const fn overlaps(a: Region, b: Region) -> bool {
    true /* stub: replace */
}

/// True if no two regions overlap. const fn: use while loops and indices (no iterators in const fn).
pub const fn all_disjoint(regs: &[Region]) -> bool {
    true /* stub: replace */
}

const GOOD: [Region; 3] = [
    Region::new(0x0000, 0x100),
    Region::new(0x100, 0x100),
    Region::new(0x400, 0x80),
];
// Compile-time proof (passes with the stub only because the stub says true).
// Once all_disjoint is correct, edit GOOD to overlap two regions and watch the BUILD break. Then revert.
const _: () = assert!(all_disjoint(&GOOD));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td15_overlaps() {
        assert!(overlaps(Region::new(0, 10), Region::new(5, 10)));
        assert!(!overlaps(Region::new(0, 10), Region::new(10, 10))); // touching is fine
        assert!(overlaps(Region::new(0, 100), Region::new(10, 5))); // containment
    }
    #[test]
    fn td15_disjoint() {
        assert!(all_disjoint(&GOOD));
        assert!(!all_disjoint(&[
            Region::new(0, 10),
            Region::new(20, 5),
            Region::new(24, 5)
        ]));
        assert!(all_disjoint(&[]));
    }
}
