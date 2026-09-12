//! `oxicode-formal`: `cargo-formal` harnesses over `oxicode`'s public
//! fixed-array codec API (Phase 2b package E4, design v1.1 §4 / §7.3 item 5,
//! harness inventory `I0-d.md` §5.4).
//!
//! This package is **not** vendored code: every function it calls belongs to
//! the real `oxicode` crate, reached by an ordinary path dependency
//! (`oxicode = { path = "..", default-features = false }`). It replaces the
//! in-repository trial `examples/ecosystem/oxicode-varint` (cargo-formal
//! TODO P2-11), which vendored five files of `oxicode/src/varint/` because,
//! before Phase 2b, the driver could not lower a path dependency's function
//! bodies at all. Two driver features landed to make this package possible:
//!
//! * **D1** (dependency-body lowering): `oxicode`'s own functions --
//!   `encode_to_fixed_array`, `decode_from_slice`, `SliceWriter::write`,
//!   `SliceReader::read`, the `Encode`/`Decode` impls for the integer types,
//!   and the private `varint_encode_*`/`varint_decode_*` functions they
//!   call -- now carry MIR in this package's `.fir` because `oxicode` is
//!   listed in `dep-crates`.
//! * **D2** (on-demand monomorphic instance lowering): every harness below
//!   calls a **generic** entry point (`encode_to_fixed_array::<N, E>`,
//!   `decode_from_slice::<D>`). Before D2 the const generic `N` in
//!   `[0u8; N]` (`oxicode/src/lib.rs:295`) had no evaluable value in the
//!   polymorphic definition body, and the `R: Reader` bound on
//!   `SliceReader::read` named a trait method declaration with no
//!   substitution -- both `unsupported(no-body)`. D2 lowers the concrete
//!   instance each call site actually resolves to
//!   (`encode_to_fixed_array::<3, u16>`, `<SliceReader as Reader>::read`,
//!   ...), whose bodies carry the evaluated array length and the resolved
//!   impl method. The vendored trial (`examples/ecosystem/oxicode-varint`)
//!   measured this exact transition: eight whole-harness
//!   `unsupported(no-body)` results before D2, 176 proved / 1 refuted /
//!   2 unknown obligations after. This package is `oxicode`'s own public
//!   API and was measured only once D2 was in the tree.
//!
//!   **Measured for this package: all nine harnesses `unsupported(no-body)`,
//!   0 proved / 0 refuted.** D1/D2 otherwise worked (68 monomorphic
//!   instances lowered, all reachable, including every
//!   `varint_encode_*`/`varint_decode_*` function and every per-type
//!   `Encode`/`Decode` impl this package reaches); what remains unlowered is
//!   a narrower, single shared gap -- the `Self::W`/`Self::R`
//!   associated-type projection on `EncoderImpl::writer`/`DecoderImpl::reader`'s
//!   own monomorphic instances, which every harness here transitively calls
//!   because it is `oxicode`'s only dispatch path into the codec. See
//!   [`harness`]'s module docs and `EXPECTED.toml`/`README.md` for the full
//!   account and the exact driver messages.
//!
//! No self-host note applies here: `oxicode` does not depend on
//! `cargo-formal` or `oxiformal`.
//!
//! # The three builds
//!
//! Exactly as `examples/checked-arith`'s module docs describe in full (see
//! that crate for the complete explanation of each mode):
//!
//! 1. **`cargo build` / `cargo test`** (plain, stable) -- harnesses vanish;
//!    only ordinary Rust type-checks, calling straight into the real
//!    `oxicode` crate.
//! 2. **`RUSTFLAGS="--cfg oxiformal_runtime_checks" cargo test`** -- every
//!    harness becomes a randomized `#[test]` (256 draws by default). A
//!    harness whose counterexample is dense under uniform random input is
//!    additionally `#[should_panic]`; a single-point witness in a huge
//!    domain is left unmarked (documented per-harness).
//! 3. **`cargo +nightly-2026-06-20 check --cfg formal
//!    -Zcrate-attr=feature(register_tool)
//!    -Zcrate-attr=register_tool(formal_tool)`** with a separate
//!    `--target-dir` -- type-checks the `#[cfg(formal)]` copy of every
//!    harness. It does not run the driver or the solver; the verdicts in
//!    [`harness`] and `EXPECTED.toml` are **measurements** from a real
//!    `cargo formal check` run (release CLI + release driver with D1/D2,
//!    OxiZ 0.3.3 pinned per cargo-formal's own rules), never predictions.

#![forbid(unsafe_code)]

pub mod harness;
