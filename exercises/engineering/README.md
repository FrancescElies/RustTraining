# Engineering exercises

Source: [engineering-book](https://github.com/microsoft/RustTraining/tree/main/engineering-book).
These chapters are about the toolchain, so the exercises are guided tasks on a scratch crate
(`scratch/`, its own cargo workspace) instead of unit tests. Each task says how to know you are done.
Work inside `scratch/`: `cd exercises/engineering/scratch`. Commit when a task is done so you can diff the next one.

Difficulty from the book: 🟢 starter, 🟡 intermediate, 🔴 advanced. Tools you install as you go.

---

## 1. build.rs 🟢 — [ch01](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch01-build-scripts-buildrs-in-depth.md)
Add `build.rs` that exports `GIT_HASH` (from `git rev-parse --short HEAD`) and `BUILD_PROFILE` (from the `PROFILE` env var)
with `cargo:rustc-env=...`. Add `cargo:rerun-if-changed=../../../.git/HEAD` (or similar) so it does not rerun every build.
Handle `git` not being available (fall back to `unknown`; a build script must not fail the build for this).
**Done when:** `cargo run` prints a real hash, `cargo run --release` prints `release`, and a build outside git still compiles.

## 2. Cross-compilation 🟡 — [ch02](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch02-cross-compilation-one-source-many-target.md)
`rustup target add x86_64-unknown-linux-gnu aarch64-pc-windows-msvc wasm32-unknown-unknown`.
`cargo check --target <each>` the **lib** (`--lib`). Then try the full `cargo build --target x86_64-unknown-linux-gnu`.
**Done when:** you can say which targets only need `check`, which need a linker, and why. Write the answer in `NOTES.md`.
Bonus: add `.cargo/config.toml` with a `[target.<triple>] linker = ...` entry (read the chapter on `cross`/`zig cc`).

## 3. Benchmarking 🟡 — [ch03](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch03-benchmarking-measuring-what-matters.md)
`cargo add --dev criterion`, create `benches/checksum.rs` (`[[bench]] harness = false`), benchmark `checksum` on 1 KiB and 1 MiB.
Use `std::hint::black_box` on the input. Rewrite `checksum` with `chunks_exact(8)` and compare.
**Done when:** `cargo bench` reports both sizes and criterion tells you whether the rewrite is a change (read the confidence interval, not just the mean).

## 4. Coverage 🟡 — [ch04](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch04-code-coverage-seeing-what-tests-miss.md)
`cargo install cargo-llvm-cov` (+ `rustup component add llvm-tools-preview`), then `cargo llvm-cov --html --open`.
Find the uncovered lines in `parse_kv` (empty key, missing `=`) and add tests.
**Done when:** `cargo llvm-cov --fail-under-lines 95` passes. That flag is your future CI gate.

## 5. Miri / sanitizers 🔴 — [ch05](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch05-miri-valgrind-and-sanitizers-verifying-u.md)
`rustup +nightly component add miri`, then `cargo +nightly miri test -- --ignored`. Miri should report undefined behaviour in `first_two`.
Fix it **without `unsafe`** (`v.get(..2)`, return `Option<u16>` or similar) and update the tests.
**Done when:** `cargo +nightly miri test -- --include-ignored` is clean. Write one sentence on why the original test passed on a normal run.

## 6. Dependencies & supply chain 🟡 — [ch06](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch06-dependency-management-and-supply-chain-s.md)
`cargo install cargo-deny cargo-audit`. Run `cargo tree -d` (duplicates), `cargo audit`, `cargo deny init` then `cargo deny check`.
Edit `deny.toml`: allow only licenses you accept (MIT/Apache-2.0 for example) and ban a crate of your choice.
**Done when:** `cargo deny check` passes, and you have seen it **fail** after adding the banned crate.

## 7. Release profile & binary size 🟡 — [ch07](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch07-release-profiles-and-binary-size.md)
Record `target/release/scratch(.exe)` size. Then set in `[profile.release]`: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`.
Rebuild and compare; try `cargo install cargo-bloat` + `cargo bloat --release`.
**Done when:** you have a before/after table in `NOTES.md` and can say what each setting costs (build time, panics, debuggability).

## 8. Compile-time & dev tools 🟢 — [ch08](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch08-compile-time-and-developer-tools.md)
`cargo build --timings` (open the HTML), compare `cargo check` vs `cargo build`, install `cargo-nextest` and run `cargo nextest run`.
**Done when:** you can name the slowest crate in the timings report and your preferred fast inner loop.

## 9. no_std & feature verification 🔴 — [ch09](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch09-no-std-and-feature-verification.md)
Add a `std` feature (default on). Make the lib `#![cfg_attr(not(feature = "std"), no_std)]`. `checksum` and `parse_kv` only need `core`.
Add a std-only function (e.g. one returning `String`) behind `#[cfg(feature = "std")]`.
`cargo install cargo-hack`.
**Done when:** `cargo check --no-default-features --lib --target thumbv7em-none-eabihf` and `cargo hack check --feature-powerset --lib` both pass.

## 10. Windows & conditional compilation 🟡 — [ch10](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch10-windows-and-conditional-compilation.md)
Add `pub fn temp_dir_name() -> &'static str` with `#[cfg(windows)]` returning `"TEMP"` and `#[cfg(unix)]` returning `"TMPDIR"`,
plus a `compile_error!` fallback for other targets via `#[cfg(not(any(windows, unix)))]`.
**Done when:** `cargo check --lib --target x86_64-pc-windows-msvc`, `--target x86_64-unknown-linux-gnu` and `--target wasm32-unknown-unknown`
behave as you predicted (write the prediction first!).

## 11. CI pipeline 🔴 — [ch11](https://github.com/microsoft/RustTraining/blob/main/engineering-book/src/ch11-putting-it-all-together-a-production-cic.md)
Write `.github/workflows/ci.yml` in `scratch/` (it will not run from this subfolder; that is fine) that does:
fmt check, clippy `-D warnings`, test (+nextest), `cargo deny check`, the coverage gate from task 4, a `--no-default-features` check from task 9,
and a Miri job from task 5. Use a build matrix for `ubuntu-latest` + `windows-latest` where it makes sense and cache `~/.cargo` and `target`.
**Done when:** `actionlint` (or `yamllint`) accepts it and you can explain what each job protects against.

---
Chapters 12 (tricks) and 13 (quick reference) are reading material: skim them after task 11 and add 2 tricks you would adopt to `NOTES.md`.
