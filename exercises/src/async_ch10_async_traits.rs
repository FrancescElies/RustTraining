//! Async ch10 Async traits (book: async-book/src/ch10-async-traits.md)
//! native async fn in traits, the boxed-future form that works with dyn, async closures.
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

/// Native async fn in trait (static dispatch only).
#[allow(async_fn_in_trait)]
pub trait Storage {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: &str);
}

#[derive(Default)]
pub struct Mem(Mutex<HashMap<String, String>>);

impl Storage for Mem {
    async fn get(&self, key: &str) -> Option<String> {
        todo!()
    }
    async fn set(&self, key: &str, value: &str) {
        todo!()
    }
}

/// Generic over any Storage: value, or default if missing.
pub async fn get_or_default<S: Storage>(s: &S, key: &str, default: &str) -> String {
    todo!()
}

/// dyn-compatible version: async fn in traits can't be used as dyn, so return a boxed future by hand.
pub trait DynStorage {
    fn get_boxed<'a>(
        &'a self,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>>;
}
impl DynStorage for Mem {
    fn get_boxed<'a>(
        &'a self,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        todo!() // Box::pin(async move { ... })
    }
}
pub async fn lookup_dyn(s: &dyn DynStorage, key: &str) -> Option<String> {
    s.get_boxed(key).await
}

/// Async closure (Rust 1.85+): apply f twice.
pub async fn apply_twice(f: impl AsyncFn(i32) -> i32, x: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn async10_storage() {
        let m = Mem::default();
        assert_eq!(m.get("a").await, None);
        m.set("a", "1").await;
        assert_eq!(m.get("a").await, Some("1".into()));
        assert_eq!(get_or_default(&m, "a", "?").await, "1");
        assert_eq!(get_or_default(&m, "zz", "?").await, "?");
    }
    #[tokio::test]
    async fn async10_dyn() {
        let m = Mem::default();
        m.set("k", "v").await;
        let d: &dyn DynStorage = &m;
        assert_eq!(lookup_dyn(d, "k").await, Some("v".into()));
        // The boxed future is Send, so it can cross tasks:
        let m = std::sync::Arc::new(m);
        let m2 = m.clone();
        let r = tokio::spawn(async move { DynStorage::get_boxed(&*m2, "k").await })
            .await
            .unwrap();
        assert_eq!(r, Some("v".into()));
    }
    #[tokio::test]
    async fn async10_async_closure() {
        let add = async |x: i32| {
            tokio::task::yield_now().await;
            x + 10
        };
        assert_eq!(apply_twice(add, 1).await, 21);
    }
}
