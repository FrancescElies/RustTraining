//! Ch16 Case studies: C++ patterns -> idiomatic Rust (book: ch16-case-studies.md, ch16-cases-3-5-*.md)
//! 1 enum dispatch instead of class hierarchy; 2 arena + indices instead of shared_ptr trees;
//! 3 borrowed context with lifetimes instead of raw pointer; 4 composable state structs (disjoint borrows).

// ---- Case 1: enum dispatch ----
#[derive(Debug, PartialEq)]
pub enum GpuEvent {
    Throttle { temp: u32 },
    EccError { count: u32, fatal: bool },
    Fan(u8), // percent
}
impl GpuEvent {
    /// Throttle: >90 -> 3 else 1.  EccError: fatal -> 4 else 2.  Fan: 0% -> 3 else 0.
    pub fn severity(&self) -> u8 {
        todo!()
    }
    /// "throttle@95C", "ecc x3 fatal", "ecc x1", "fan 40%"  (one match, no trait objects)
    pub fn describe(&self) -> String {
        todo!()
    }
}
pub fn max_severity(events: &[GpuEvent]) -> u8 {
    todo!()
}

// ---- Case 2: arena tree (no Rc<RefCell>, no cycles) ----
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeId(usize);
struct Node {
    value: i32,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}
#[derive(Default)]
pub struct Arena {
    nodes: Vec<Node>,
}
impl Arena {
    pub fn add_root(&mut self, value: i32) -> NodeId {
        todo!()
    }
    pub fn add_child(&mut self, parent: NodeId, value: i32) -> NodeId {
        todo!()
    } // also record in parent.children
    /// Number of edges up to the root (root = 0).
    pub fn depth(&self, id: NodeId) -> usize {
        todo!()
    }
    /// Sum of value over id and all descendants.
    pub fn sum_subtree(&self, id: NodeId) -> i32 {
        todo!()
    }
}

// ---- Case 3: borrowed context (C++ held a raw pointer to the framework) ----
pub struct DiagContext<'a> {
    log: &'a mut Vec<String>,
    name: &'a str,
}
impl<'a> DiagContext<'a> {
    pub fn new(log: &'a mut Vec<String>, name: &'a str) -> Self {
        todo!()
    }
    /// Pushes "[name] msg".
    pub fn log(&mut self, msg: &str) {
        todo!()
    }
}

// ---- Case 4: composable state: split the monolith so borrows are disjoint ----
pub struct Config {
    pub limit: u32,
}
#[derive(Default, Debug, PartialEq)]
pub struct Counters {
    pub ok: u32,
    pub fail: u32,
}
/// v <= limit: ok += 1, return true. Else fail += 1, return false.
pub fn step(cfg: &Config, c: &mut Counters, v: u32) -> bool {
    todo!()
}

pub struct App {
    pub cfg: Config,
    pub counters: Counters,
    pub log: Vec<String>,
}
impl App {
    /// Run step() for each input. On failure push "fail: <v>" to self.log.
    /// Observe: you borrow &self.cfg and &mut self.counters at once. Why does that compile?
    pub fn run(&mut self, inputs: &[u32]) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ch16_events() {
        assert_eq!(GpuEvent::Throttle { temp: 95 }.severity(), 3);
        assert_eq!(GpuEvent::Throttle { temp: 80 }.severity(), 1);
        assert_eq!(
            GpuEvent::EccError {
                count: 3,
                fatal: true
            }
            .severity(),
            4
        );
        assert_eq!(GpuEvent::Fan(0).severity(), 3);
        assert_eq!(GpuEvent::Fan(40).severity(), 0);
        assert_eq!(GpuEvent::Throttle { temp: 95 }.describe(), "throttle@95C");
        assert_eq!(
            GpuEvent::EccError {
                count: 3,
                fatal: true
            }
            .describe(),
            "ecc x3 fatal"
        );
        assert_eq!(
            GpuEvent::EccError {
                count: 1,
                fatal: false
            }
            .describe(),
            "ecc x1"
        );
        assert_eq!(GpuEvent::Fan(40).describe(), "fan 40%");
        assert_eq!(max_severity(&[GpuEvent::Fan(40), GpuEvent::Fan(0)]), 3);
        assert_eq!(max_severity(&[]), 0);
    }
    #[test]
    fn ch16_arena() {
        let mut a = Arena::default();
        let r = a.add_root(1);
        let c1 = a.add_child(r, 2);
        let c2 = a.add_child(r, 3);
        let g = a.add_child(c1, 4);
        assert_eq!((a.depth(r), a.depth(c1), a.depth(g)), (0, 1, 2));
        assert_eq!(a.sum_subtree(r), 10);
        assert_eq!(a.sum_subtree(c1), 6);
        assert_eq!(a.sum_subtree(c2), 3);
    }
    #[test]
    fn ch16_ctx() {
        let mut v = Vec::new();
        {
            let mut c = DiagContext::new(&mut v, "gpu");
            c.log("ok");
            c.log("again");
        }
        assert_eq!(v, ["[gpu] ok", "[gpu] again"]);
    }
    #[test]
    fn ch16_app() {
        let mut app = App {
            cfg: Config { limit: 10 },
            counters: Counters::default(),
            log: vec![],
        };
        app.run(&[1, 20, 5]);
        assert_eq!(app.counters, Counters { ok: 2, fail: 1 });
        assert_eq!(app.log, ["fail: 20"]);
    }
}
