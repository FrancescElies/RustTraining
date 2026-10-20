//! Async ch14 Async is an optimization, not an architecture (book: async-book/src/ch14-*.md)
//! Put the logic in plain sync functions (trivially testable, no runtime), and keep async in a thin I/O shell.
//! Also covers the book's "Async Echo Server" exercise idea, over an in-memory stream so no sockets are needed.
use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

pub type Store = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub enum Cmd {
    Set(String, String),
    Get(String),
    Del(String),
}

/// "SET k some value" / "GET k" / "DEL k" (case-sensitive). The value may contain spaces (splitn(3, ' ')).
/// Anything else -> Err("unknown command"). Missing arguments -> Err("missing argument").
pub fn parse(line: &str) -> Result<Cmd, String> {
    todo!()
}

/// SET -> "OK". GET -> "VALUE <v>" or "NOT_FOUND". DEL -> "OK" if it existed, else "NOT_FOUND".
pub fn apply(store: &mut Store, cmd: Cmd) -> String {
    todo!()
}

/// parse + apply. A parse error becomes "ERR <message>".
pub fn handle_line(store: &mut Store, line: &str) -> String {
    todo!()
}

/// The async shell: read lines until EOF, answer each with handle_line + '\n', flush at the end.
/// No logic here: just I/O. (read_line into a String, trim_end, write_all)
pub async fn serve<R, W>(mut reader: R, mut writer: W) -> std::io::Result<()>
where
    R: AsyncBufRead + Unpin,
    W: AsyncWrite + Unpin,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, BufReader};

    // No tokio, no runtime: that's the payoff of a sync core.
    #[test]
    fn async14_parse() {
        assert_eq!(
            parse("SET a hello world"),
            Ok(Cmd::Set("a".into(), "hello world".into()))
        );
        assert_eq!(parse("GET a"), Ok(Cmd::Get("a".into())));
        assert_eq!(parse("DEL a"), Ok(Cmd::Del("a".into())));
        assert_eq!(parse("NOPE"), Err("unknown command".into()));
        assert_eq!(parse("GET"), Err("missing argument".into()));
        assert_eq!(parse("SET k"), Err("missing argument".into()));
    }
    #[test]
    fn async14_core_logic() {
        let mut s = Store::new();
        assert_eq!(handle_line(&mut s, "SET a 1"), "OK");
        assert_eq!(handle_line(&mut s, "GET a"), "VALUE 1");
        assert_eq!(handle_line(&mut s, "GET b"), "NOT_FOUND");
        assert_eq!(handle_line(&mut s, "DEL a"), "OK");
        assert_eq!(handle_line(&mut s, "DEL a"), "NOT_FOUND");
        assert_eq!(handle_line(&mut s, "WAT"), "ERR unknown command");
    }
    #[tokio::test]
    async fn async14_shell_over_duplex() {
        let (mut client, server) = tokio::io::duplex(1024);
        let (sr, sw) = tokio::io::split(server);
        let task = tokio::spawn(serve(BufReader::new(sr), sw));
        client
            .write_all(b"SET a 1\nGET a\nGET zz\nWAT\n")
            .await
            .unwrap();
        client.shutdown().await.unwrap(); // EOF for the server
        let mut out = String::new();
        client.read_to_string(&mut out).await.unwrap();
        assert_eq!(out, "OK\nVALUE 1\nNOT_FOUND\nERR unknown command\n");
        task.await.unwrap().unwrap();
    }
}
