//! Type-driven ch03 Single-use types: ownership as linear types (book: ...ch03-single-use-*.md)

/// A nonce can be used once because it is not Clone/Copy and encrypt takes it BY VALUE.
#[derive(Debug)]
pub struct Nonce(u64);
impl Nonce {
    pub fn value(&self) -> u64 {
        self.0
    }
}

pub struct NonceGen {
    next: u64,
}
impl NonceGen {
    pub fn new() -> Self {
        NonceGen { next: 0 }
    }
    /// Hand out 0, 1, 2, ... each wrapped in a Nonce.
    pub fn issue(&mut self) -> Nonce {
        todo!()
    }
}

/// "Encrypt": xor each byte with key; return (nonce value, ciphertext). Consumes the nonce.
pub fn encrypt(key: u8, nonce: Nonce, msg: &[u8]) -> (u64, Vec<u8>) {
    todo!()
}

/// One-time signing authority: the token can be issued once, and signing consumes it.
pub struct Authority {
    issued: bool,
}
pub struct SigningToken(());
impl Authority {
    pub fn new() -> Self {
        Authority { issued: false }
    }
    /// First call Some(token); every later call None.
    pub fn issue(&mut self) -> Option<SigningToken> {
        todo!()
    }
}
impl SigningToken {
    /// Consumes self. "signature" = wrapping sum of bytes as u32.
    pub fn sign(self, firmware: &[u8]) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td03_nonce() {
        let mut g = NonceGen::new();
        let (n1, n2) = (g.issue(), g.issue());
        assert_eq!((n1.value(), n2.value()), (0, 1));
        let (v, ct) = encrypt(0xFF, n1, &[0x00, 0x0F]);
        assert_eq!((v, ct), (0, vec![0xFF, 0xF0]));
    }
    #[test]
    fn td03_token() {
        let mut a = Authority::new();
        let t = a.issue().expect("first issue works");
        assert!(a.issue().is_none());
        assert_eq!(t.sign(&[1, 2, 3]), 6);
        // let again = t.sign(&[]); // does not compile: t was moved. Try it.
    }
}
