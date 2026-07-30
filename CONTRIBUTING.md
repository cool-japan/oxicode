# Contributing to oxicode

Thanks for your interest in contributing! oxicode is a binary
serialization library with a strict "no warnings" policy and a large
compatibility/fuzz test surface, so please read this before opening a
pull request.

## Prerequisites

- Rust **1.81.0** or newer (this is the MSRV — `rust-version` in
  `Cargo.toml`). Use `rustup show` to confirm your toolchain, or
  `rustup install 1.81.0` to add it alongside `stable`.
- [`cargo-nextest`](https://nexte.st/) for running the test suite:
  `cargo install cargo-nextest --locked`.
- `rustfmt` and `clippy` components (`rustup component add rustfmt
  clippy`).

## Building

```bash
cargo build --workspace --all-features
```

The workspace has three members: the `oxicode` crate itself, the
`derive` proc-macro crate (`oxicode_derive`), and the internal
`compatibility` crate (test-only, `publish = false`, never depend on it
from application code).

## Testing

Run the full test suite with all features enabled before submitting a
change:

```bash
cargo nextest run --workspace --all-features
```

Useful narrower invocations while iterating:

```bash
# Just the oxicode crate
cargo nextest run -p oxicode --all-features

# A subset by name filter
cargo nextest run -p oxicode --all-features -E 'test(compat)'

# Doc tests (nextest does not run these)
cargo test --doc --all-features
```

Also exercise the feature matrix that CI checks, at least for the
feature(s) your change touches: `--no-default-features`,
`--no-default-features --features alloc`, and each of `simd`,
`compression-lz4`, `compression-zstd`, `serde`, `async-tokio`,
`checksum` individually.

If you touch any `unsafe` code (currently confined to
`src/de/impls.rs`, `src/de/borrow_slice.rs`,
`src/features/impl_alloc.rs`, `src/simd/aligned.rs`), also run it under
Miri:

```bash
cargo +nightly miri test -p oxicode --lib
```

## Lints and formatting

This project enforces a **zero-warnings policy**. Both of the following
must be clean before a PR is merged:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Please run `cargo fmt --all` (without `--check`) to auto-format your
changes rather than hand-formatting.

## Code conventions

- `snake_case` for variables/functions, `UpperCamelCase` for
  types/traits, matching standard Rust naming conventions (`clippy`'s
  naming lints are part of the `-D warnings` gate above).
- No new `unwrap()` / `expect()` / `panic!()` / `unreachable!()` in
  production (`src/`) code paths — use the existing `Error` variants
  (`src/error.rs`) and propagate `Result` instead. Test and example
  code may use them where a failure should abort the test/example.
- Keep individual source files under ~2000 lines; split large modules
  rather than letting one file grow unbounded.
- Any test that writes to disk must use `std::env::temp_dir()` (or
  `tempfile`, already a dev-dependency) rather than hard-coded paths.
- **Wire-format stability:** oxicode's `standard()`/`legacy()` configs
  are byte-for-byte compatible with bincode 2.0.1 / bincode 1.x
  respectively for currently-valid inputs (see the `compatibility/`
  crate and `tests/bincode_compat_test.rs`). Do not change the
  serialized byte layout for any input that is valid today; rejecting
  previously-unvalidated/malicious input, fixing panics, and adding new
  APIs are all fine. If you are unsure whether a change affects the
  wire format, run the compatibility suite:

  ```bash
  cargo test -p oxicode_compatibility
  cargo nextest run -p oxicode -E 'test(compat)'
  ```

## Dependencies

Follow the workspace-dependency policy: shared dependencies belong in
`[workspace.dependencies]` at the repository root and are inherited via
`dep = { workspace = true }` in each crate's `Cargo.toml` — avoid
re-pinning a literal version in an individual crate manifest. Prefer the
latest version available on crates.io when adding or bumping a
dependency, with the sole intentional exception of the `bincode`
dev-dependency, which is pinned to `=2.0.1` because it is the
compatibility oracle used by the compat test suite (bincode 3.x has an
incompatible API and must never be introduced — this is also enforced
by `deny.toml`).

## Submitting changes

1. Fork the repository and create a feature branch.
2. Make your change, following the conventions above.
3. Add or update tests covering the change.
4. Run the build/test/lint commands in this document.
5. Update `CHANGELOG.md` under an "Unreleased" heading if the change is
   user-visible.
6. Open a pull request describing the change and its motivation.

## Reporting bugs and security issues

Regular bugs: please open a GitHub issue with a minimal reproduction
(ideally a byte sequence plus the type/config used to decode it).

Security vulnerabilities (panics, memory-safety issues, or
resource-exhaustion bugs triggerable by decoding untrusted input) should
**not** be filed as public issues — see [SECURITY.md](SECURITY.md) for
the private reporting process.
