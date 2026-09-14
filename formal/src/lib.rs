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
//!   impl method. An instance signature is normalized with the **fallible**
//!   normalizer under `TypingEnv::fully_monomorphized()` (cargo-formal
//!   `P2-14`), without which an associated-type projection such as
//!   `<EncoderImpl<SliceWriter<'_>, Configuration> as Encoder>::W` survives
//!   into the signature and every instance is refused -- which is exactly
//!   what used to block all nine harnesses here, since `encoder.writer()`/
//!   `decoder.reader()` are `oxicode`'s only dispatch path into the varint
//!   codec. See [`harness`]'s module docs for that history in full.
//!
//! **Measured L1 verdict for this package** (2026-09-14, release CLI +
//! release driver with D1/D2, OxiZ 0.3.3, rustc nightly-2026-06-20): `bmc`
//! **218 proved / 0 refuted / 6 unknown / 0 timeout / 0 unsupported /
//! 0 unverifiable over 224 obligations**, 0 cached, 6 s wall, exit 0;
//! 42 dependency bodies lowered (6 reachable) and 68 monomorphic instances
//! lowered (all 68 reachable); `contract` 0 proved / 0 refuted; `theorem` not
//! run; `hygiene` pass. Per harness, the measured L1 verdicts are:
//!
//! * `varint_u16_roundtrip_harness` -- `assert` **unknown** (23 obligations,
//!   22 proved);
//! * `varint_u32_roundtrip_harness` -- `assert` **unknown** (30, 29 proved);
//! * `varint_u64_roundtrip_harness` -- `assert` **unknown** (41, 40 proved);
//! * `varint_u64_bound_is_tight_harness` -- `assert` **proved** (20, all);
//! * `fixed_int_roundtrip_harness` -- `assert` **proved** (13, all);
//! * `zigzag_i32_roundtrip_harness` -- `assert` **unknown** (33, 32 proved);
//! * `zigzag_i64_roundtrip_harness` -- `assert` **unknown** (44, 43 proved);
//! * `decode_rejects_a_wide_tag_harness` -- `assert` **unknown** (12, 11
//!   proved);
//! * `decode_never_panics_on_arbitrary_bytes_harness` -- `harness` **proved**
//!   (8, all).
//!
//! All six `unknown`s are `solver-model-rejected` on the OxiZ `=0.3.3` pin
//! (upstream U-Z10, fixed in the `../oxiz` working tree but unreleased), not
//! refutations, and are expected to prove once that release ships. The
//! vendored trial (`examples/ecosystem/oxicode-varint`) measured 176 proved /
//! 1 refuted / 2 unknown on the same driver, but reaches the codec through
//! hand-rolled `Writer`/`Reader` generics rather than `oxicode`'s real
//! `Encoder`/`Decoder` dispatch; this package exercises that dispatch. Both
//! now encode. See [`harness`]'s module docs, `EXPECTED.toml` and `README.md`
//! for the per-site detail.
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
