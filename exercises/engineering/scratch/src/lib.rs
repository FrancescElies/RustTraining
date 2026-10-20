//! Scratch crate for the engineering-book tasks (see ../README.md). Edit freely.

/// 8-bit wrapping checksum.
pub fn checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |a, b| a.wrapping_add(*b))
}

/// "key=value" -> (key, value). Trims spaces. Empty key or no '=' -> None.
pub fn parse_kv(s: &str) -> Option<(&str, &str)> {
    let (k, v) = s.split_once('=')?;
    let k = k.trim();
    if k.is_empty() {
        return None;
    }
    Some((k, v.trim()))
}

/// First two bytes as little-endian u16. Task 5 (Miri): this has undefined behaviour. Find it, fix it safely.
pub fn first_two(v: &[u8]) -> u16 {
    unsafe {
        let p = v.as_ptr();
        (*p as u16) | ((*p.add(1) as u16) << 8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_wraps() {
        assert_eq!(checksum(&[200, 100]), 44);
    }

    #[test]
    fn parse_kv_ok() {
        assert_eq!(parse_kv("a = 1"), Some(("a", "1")));
    }

    #[test]
    fn first_two_ok() {
        assert_eq!(first_two(&[1, 2, 3]), 0x0201);
    }

    /// Passes on a normal run (the UB is silent!). Run: cargo +nightly miri test -- --ignored
    #[test]
    #[ignore]
    fn first_two_short_input() {
        let _ = first_two(&[1]);
    }
}
