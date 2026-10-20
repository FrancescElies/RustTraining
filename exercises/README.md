# Exercises

Hands-on exercises for the books in this repo ([microsoft/RustTraining](https://github.com/microsoft/RustTraining)).
Read the chapter, then open the matching file and replace each `todo!()` until the tests pass.

```
cargo test -p exercises ch05      # one chapter (prefixes: ch05, pat04, td06 ...)
cargo test -p exercises           # everything
```

Suggested order: core, patterns, type-driven, async. The engineering track is separate (tool tasks, not tests). Each file's `//!` header names the chapter too.
Not covered: ch17.4 logging (needs tracing crates) and the reading-only async chapters (Pin, runtimes, when tokio doesn't fit).

## Core track
Source: [c-cpp-book](https://github.com/microsoft/RustTraining/tree/main/c-cpp-book)

| Exercise file | Book chapter(s) |
|---|---|
| [`ch03_types.rs`](src/ch03_types.rs) | [ch03-built-in-types](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch03-built-in-types.md) |
| [`ch04_control_flow.rs`](src/ch04_control_flow.rs) | [ch04-control-flow](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch04-control-flow.md) |
| [`ch05_data_structures.rs`](src/ch05_data_structures.rs) | [ch05-data-structures](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch05-data-structures.md) |
| [`ch06_enums.rs`](src/ch06_enums.rs) | [ch06-enums-and-pattern-matching](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch06-enums-and-pattern-matching.md) |
| [`ch07_ownership.rs`](src/ch07_ownership.rs) | [ch07-1-lifetimes-and-borrowing-deep-dive](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch07-1-lifetimes-and-borrowing-deep-dive.md), [ch07-2-smart-pointers-and-interior-mutability](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch07-2-smart-pointers-and-interior-mutability.md), [ch07-ownership-and-borrowing](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch07-ownership-and-borrowing.md) |
| [`ch09_errors.rs`](src/ch09_errors.rs) | [ch09-1-error-handling-best-practices](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch09-1-error-handling-best-practices.md), [ch09-error-handling](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch09-error-handling.md) |
| [`ch10_traits.rs`](src/ch10_traits.rs) | [ch10-1-generics](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch10-1-generics.md), [ch10-traits](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch10-traits.md) |
| [`ch11_from_into.rs`](src/ch11_from_into.rs) | [ch11-from-and-into-traits](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch11-from-and-into-traits.md) |
| [`ch12_closures.rs`](src/ch12_closures.rs) | [ch12-1-iterator-power-tools](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch12-1-iterator-power-tools.md), [ch12-closures](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch12-closures.md) |
| [`ch13_concurrency.rs`](src/ch13_concurrency.rs) | [ch13-concurrency](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch13-concurrency.md) |
| [`ch14_ffi.rs`](src/ch14_ffi.rs) | [ch14-unsafe-rust-and-ffi](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch14-unsafe-rust-and-ffi.md) |
| [`ch15_no_std.rs`](src/ch15_no_std.rs) | [ch15-1-embedded-deep-dive](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch15-1-embedded-deep-dive.md), [ch15-no_std-rust-without-the-standard-library](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch15-no_std-rust-without-the-standard-library.md) |
| [`ch16_case_studies.rs`](src/ch16_case_studies.rs) | [ch16-case-studies](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch16-case-studies.md), [ch16-cases-3-5-lifetime-borrowing](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch16-cases-3-5-lifetime-borrowing.md) |
| [`ch17_best_practices.rs`](src/ch17_best_practices.rs) | [ch17-1-avoiding-excessive-clone](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch17-1-avoiding-excessive-clone.md), [ch17-2-avoiding-unchecked-indexing](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch17-2-avoiding-unchecked-indexing.md), [ch17-3-collapsing-assignment-pyramids](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch17-3-collapsing-assignment-pyramids.md), [ch17-4-logging-and-tracing-ecosystem](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch17-4-logging-and-tracing-ecosystem.md), [ch17-best-practices](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch17-best-practices.md) |
| [`ch18_semantics.rs`](src/ch18_semantics.rs) | [ch18-cpp-rust-semantic-deep-dives](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch18-cpp-rust-semantic-deep-dives.md) |
| [`ch19_macros.rs`](src/ch19_macros.rs) | [ch19-macros](https://github.com/microsoft/RustTraining/blob/main/c-cpp-book/src/ch19-macros.md) |

## Patterns track
Source: [rust-patterns-book](https://github.com/microsoft/RustTraining/tree/main/rust-patterns-book)

| Exercise file | Book chapter(s) |
|---|---|
| [`pat_ch01_generics.rs`](src/pat_ch01_generics.rs) | [ch01-generics-the-full-picture](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch01-generics-the-full-picture.md) |
| [`pat_ch02_traits.rs`](src/pat_ch02_traits.rs) | [ch02-traits-in-depth](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch02-traits-in-depth.md) |
| [`pat_ch03_newtype_typestate.rs`](src/pat_ch03_newtype_typestate.rs) | [ch03-the-newtype-and-type-state-patterns](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch03-the-newtype-and-type-state-patterns.md) |
| [`pat_ch04_phantomdata.rs`](src/pat_ch04_phantomdata.rs) | [ch04-phantomdata-types-that-carry-no-data](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch04-phantomdata-types-that-carry-no-data.md) |
| [`pat_ch05_channels.rs`](src/pat_ch05_channels.rs) | [ch05-channels-and-message-passing](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch05-channels-and-message-passing.md) |
| [`pat_ch09_smart_pointers.rs`](src/pat_ch09_smart_pointers.rs) | [ch09-smart-pointers-and-interior-mutability](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch09-smart-pointers-and-interior-mutability.md) |
| [`pat_ch10_error_patterns.rs`](src/pat_ch10_error_patterns.rs) | [ch10-error-handling-patterns](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch10-error-handling-patterns.md) |
| [`pat_ch11_binary_data.rs`](src/pat_ch11_binary_data.rs) | [ch11-serialization-zero-copy-and-binary-data](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch11-serialization-zero-copy-and-binary-data.md) |
| [`pat_ch12_unsafe.rs`](src/pat_ch12_unsafe.rs) | [ch12-unsafe-rust-controlled-danger](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch12-unsafe-rust-controlled-danger.md) |
| [`pat_ch13_macros.rs`](src/pat_ch13_macros.rs) | [ch13-macros-code-that-writes-code](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch13-macros-code-that-writes-code.md) |
| [`pat_ch14_testing.rs`](src/pat_ch14_testing.rs) | [ch14-testing-and-benchmarking-patterns](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch14-testing-and-benchmarking-patterns.md) |
| [`pat_ch15_api_design.rs`](src/pat_ch15_api_design.rs) | [ch15-crate-architecture-and-api-design](https://github.com/microsoft/RustTraining/blob/main/rust-patterns-book/src/ch15-crate-architecture-and-api-design.md) |

## Type-driven track
Source: [type-driven-correctness-book](https://github.com/microsoft/RustTraining/tree/main/type-driven-correctness-book)

| Exercise file | Book chapter(s) |
|---|---|
| [`td_ch02_typed_commands.rs`](src/td_ch02_typed_commands.rs) | [ch02-typed-command-interfaces-request-determi](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch02-typed-command-interfaces-request-determi.md) |
| [`td_ch03_single_use.rs`](src/td_ch03_single_use.rs) | [ch03-single-use-types-cryptographic-guarantee](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch03-single-use-types-cryptographic-guarantee.md) |
| [`td_ch04_capability_tokens.rs`](src/td_ch04_capability_tokens.rs) | [ch04-capability-tokens-zero-cost-proof-of-aut](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch04-capability-tokens-zero-cost-proof-of-aut.md) |
| [`td_ch05_typestate_protocol.rs`](src/td_ch05_typestate_protocol.rs) | [ch05-protocol-state-machines-type-state-for-r](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch05-protocol-state-machines-type-state-for-r.md) |
| [`td_ch06_units.rs`](src/td_ch06_units.rs) | [ch06-dimensional-analysis-making-the-compiler](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch06-dimensional-analysis-making-the-compiler.md) |
| [`td_ch07_parse_dont_validate.rs`](src/td_ch07_parse_dont_validate.rs) | [ch07-validated-boundaries-parse-dont-validate](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch07-validated-boundaries-parse-dont-validate.md) |
| [`td_ch08_capability_mixins.rs`](src/td_ch08_capability_mixins.rs) | [ch08-capability-mixins-compile-time-hardware-](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch08-capability-mixins-compile-time-hardware-.md) |
| [`td_ch09_phantom_resources.rs`](src/td_ch09_phantom_resources.rs) | [ch09-phantom-types-for-resource-tracking](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch09-phantom-types-for-resource-tracking.md) |
| [`td_ch15_const_fn_proofs.rs`](src/td_ch15_const_fn_proofs.rs) | [ch15-const-fn-compile-time-correctness-proofs](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch15-const-fn-compile-time-correctness-proofs.md) |
| [`td_ch16_send_sync.rs`](src/td_ch16_send_sync.rs) | [ch16-send-sync-compile-time-concurrency-proofs](https://github.com/microsoft/RustTraining/blob/main/type-driven-correctness-book/src/ch16-send-sync-compile-time-concurrency-proofs.md) |

## Async track
Source: [async-book](https://github.com/microsoft/RustTraining/tree/main/async-book). Prefix `async`, e.g. `cargo test -p exercises async05`.
Most tests use `start_paused = true` (virtual time), so they run instantly and deterministically.

| Exercise file | Book chapter(s) |
|---|---|
| [`async_ch02_future_trait.rs`](src/async_ch02_future_trait.rs) | [ch02-the-future-trait](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch02-the-future-trait.md) |
| [`async_ch03_executor.rs`](src/async_ch03_executor.rs) | [ch03-how-poll-works](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch03-how-poll-works.md) |
| [`async_ch05_state_machine.rs`](src/async_ch05_state_machine.rs) | [ch05-the-state-machine-reveal](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch05-the-state-machine-reveal.md) |
| [`async_ch06_futures_by_hand.rs`](src/async_ch06_futures_by_hand.rs) | [ch06-building-futures-by-hand](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch06-building-futures-by-hand.md) |
| [`async_ch08_tokio.rs`](src/async_ch08_tokio.rs) | [ch08-tokio-deep-dive](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch08-tokio-deep-dive.md) |
| [`async_ch10_async_traits.rs`](src/async_ch10_async_traits.rs) | [ch10-async-traits](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch10-async-traits.md) |
| [`async_ch11_streams.rs`](src/async_ch11_streams.rs) | [ch11-streams-and-asynciterator](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch11-streams-and-asynciterator.md) |
| [`async_ch12_pitfalls.rs`](src/async_ch12_pitfalls.rs) | [ch12-common-pitfalls](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch12-common-pitfalls.md) |
| [`async_ch13_production.rs`](src/async_ch13_production.rs) | [ch13-production-patterns](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch13-production-patterns.md) |
| [`async_ch14_sync_core.rs`](src/async_ch14_sync_core.rs) | [ch14-async-is-an-optimization-not-an-architecture](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch14-async-is-an-optimization-not-an-architecture.md) |
| [`async_ch15_async_mutex.rs`](src/async_ch15_async_mutex.rs) | [ch15-exercises](https://github.com/microsoft/RustTraining/blob/main/async-book/src/ch15-exercises.md) |

## Engineering track
Source: [engineering-book](https://github.com/microsoft/RustTraining/tree/main/engineering-book). Guided tasks on a scratch crate: [engineering/README.md](engineering/README.md).
