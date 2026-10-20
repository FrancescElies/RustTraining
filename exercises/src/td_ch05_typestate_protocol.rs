//! Type-driven ch05 Protocol state machines with type-state (book: ...ch05-protocol-state-machines-*.md)
use std::marker::PhantomData;

pub struct Disconnected;
pub struct Connected;
pub struct Authed;

/// Session<State>: methods exist only in the states where they make sense.
pub struct Session<S> {
    sent: u32,
    _s: PhantomData<S>,
}

impl Session<Disconnected> {
    pub fn new() -> Self {
        Session {
            sent: 0,
            _s: PhantomData,
        }
    }
    pub fn connect(self) -> Session<Connected> {
        todo!()
    }
}
impl Session<Connected> {
    /// Password "secret" -> Ok(authed). Wrong -> Err(self) so the caller can retry.
    pub fn authenticate(self, password: &str) -> Result<Session<Authed>, Session<Connected>> {
        todo!()
    }
}
impl Session<Authed> {
    /// Count of messages sent so far (including this one).
    pub fn send(&mut self, _msg: &str) -> u32 {
        todo!()
    }
    /// Back to Disconnected, keeping the sent count (so totals survive reconnects).
    pub fn close(self) -> Session<Disconnected> {
        todo!()
    }
}
impl<S> Session<S> {
    /// Available in every state.
    pub fn sent(&self) -> u32 {
        self.sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn td05_flow() {
        let c = Session::new().connect();
        let c = match c.authenticate("wrong") {
            Err(c) => c,
            Ok(_) => panic!("accepted bad password"),
        };
        let mut a = match c.authenticate("secret") {
            Ok(a) => a,
            Err(_) => panic!("rejected good password"),
        };
        assert_eq!((a.send("hi"), a.send("there")), (1, 2));
        let d = a.close();
        assert_eq!(d.sent(), 2);
        // Session::new().send("x") // does not compile: no send() on Session<Disconnected>
    }
}
