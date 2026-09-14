//! Harnesses over `oxicode`'s public fixed-array varint codec (Phase 2b
//! package E4; design v1.1 §4.2 item E4, harness inventory `I0-d.md` §5.4).
//!
//! # The codec, in one paragraph
//!
//! `oxicode`'s varint is the bincode-compatible tag-byte form: a value up to
//! 250 is one byte; larger values use a tag byte (251/252/253/254 for
//! 16/32/64/128-bit payloads) followed by the value in the configured
//! endianness. The worst case is `1 + size_of::<T>()` bytes: 3 for `u16`,
//! 5 for `u32`, 9 for `u64`. Signed values go through zigzag first
//! (`(v << 1) ^ (v >> (BITS - 1))`, branchless and total, including at
//! `i32::MIN`/`i64::MIN`), so they share the unsigned bounds.
//!
//! # Measured reality: 218 proved / 0 refuted / 6 unknown over 224
//!
//! Measured 2026-09-14 (release CLI + release driver with dependency-body
//! lowering (D1) and on-demand monomorphic instance lowering (D2), OxiZ
//! 0.3.3, rustc nightly-2026-06-20; 224 VCs, 0 cached, 6 s wall, exit 0).
//! Every harness below encodes and reaches the solver: `bmc` reports
//! **218 proved / 0 refuted / 6 unknown / 0 timeout / 0 unsupported /
//! 0 unverifiable**, with 42 dependency bodies lowered (6 reachable) and
//! 68 monomorphic instances lowered (all 68 reachable) -- the latter
//! including every `varint_encode_*`/`varint_decode_*` function, every
//! per-type `Encode::encode`/`Decode::decode` impl for
//! `u16`/`u32`/`u64`/`i32`/`i64`, `EncoderImpl::new`/`into_writer`,
//! `DecoderImpl::new`/`claim_bytes_read`, `SliceWriter::new`/`bytes_written`,
//! `SliceReader::new`, every `config::*` helper this package calls, and the
//! two `Encoder::writer`/`Decoder::reader` accessors that used to block
//! everything (see "History" below).
//!
//! # The six `unknown`s: the OxiZ 0.3.3 pin, nothing else
//!
//! All six carry the same `report.json` message -- `solver-model-rejected:
//! OxiZ 0.3.3 returned a model that does not satisfy the verification
//! condition`. cargo-formal pins OxiZ `=0.3.3`, whose model gate (upstream
//! U-Z10) turns a model failing the driver's mandatory model check into
//! `unknown` rather than an invented counterexample; that is why none of the
//! six is reported as a refutation. U-Z10 is fixed in the `../oxiz` working
//! tree but unreleased, so the pin stays; the six rows are *expected* to move
//! to `proved` when it ships (an expectation -- no 0.3.4 run was measured).
//!
//! Where they sit differs by harness, and it matters:
//!
//! * In the five round-trip harnesses the `unknown` is the `Err(_) =>
//!   assert(false)` arm of the **inner** `decode_from_slice` match -- the
//!   claim "decoding the bytes the encoder just produced cannot fail". The
//!   headline equalities `decoded == value` and `consumed == written` are
//!   **proved** at every width, zigzag extremes included. The *outer*
//!   `Err(_) => assert(false)` arm (encode into a worst-case-size buffer)
//!   raises no item in `report.json` at all.
//! * In `decode_rejects_a_wide_tag_harness` the `unknown` is the harness's
//!   only assertion and its whole point, so that property is *stated and
//!   encoded but not established on this pin*.
//!
//! No `unwinding-assertion` obligation was raised anywhere in this run, so it
//! gives no evidence either way about the `unwind` bounds (12 package-wide,
//! 16 for `decode_never_panics_on_arbitrary_bytes_harness`).
//!
//! # History: why this package once reported nine `unsupported(no-body)`
//!
//! Before Phase 2b's `P2-14` landed, every harness here was a whole-harness
//! `unsupported(no-body)` and no VC from this package ever reached OxiZ.
//! `oxicode` reaches its varint codec only through `encoder.writer()` /
//! `decoder.reader()` (`<EncoderImpl<W, C> as Encoder>::writer`,
//! `<DecoderImpl<R, C> as Decoder>::reader`), called from inside every
//! `Encode`/`Decode` impl the crate ships. Both impls define their associated
//! type as exactly their own generic parameter (`type W = W;`/`type R = R;`),
//! and the driver did **not** normalize associated-type projections in a
//! *monomorphic instance* signature, so at `W = SliceWriter<'_>`,
//! `C = Configuration` the signature still carried
//! `<EncoderImpl<SliceWriter<'_>, Configuration> as Encoder>::W`
//! (symmetrically `::R`) un-normalized, and the instance was refused before
//! its body was ever requested. As those accessors are `oxicode`'s only
//! dispatch path into the codec, that one gap blocked all nine harnesses
//! identically and no harness rewrite could have avoided it.
//!
//! `P2-14` normalizes an instance signature with the **fallible** normalizer
//! under `TypingEnv::fully_monomorphized()` (see cargo-formal's
//! `CHANGELOG.md`, `P2-14` bullet, which names this exact projection shape),
//! and all nine harnesses now encode. That bullet also settles the design
//! v1.1 §3.2 open question these docs used to raise: `Callee::path` is
//! rewritten at **queue** time, so a callee path names a `Function` entry.
//!
//! The vendored twin `examples/ecosystem/oxicode-varint` (176 proved /
//! 1 refuted / 2 unknown) drives five vendored `pub(crate)` varint files
//! through its *own* hand-rolled `Writer`/`Reader` generics, bypassing
//! `oxicode`'s real `Encoder`/`Decoder` dispatch; this package goes through
//! that dispatch. Both now encode. Its `varint_u64_bound_is_tight_harness` is
//! `refuted` where this one is `proved` because the two state different
//! propositions -- see `EXPECTED.toml`.
//!
//! # Why `match` and never `assert(.. == Ok(..))`
//!
//! `Result<T, oxicode::error::Error>`'s `PartialEq` lives in `oxicode` itself
//! (a `#[derive]`), and a direct `assert(encode(..) == Ok((value, n)))` is
//! needless risk for zero benefit here: every harness below already needs to
//! open the `Ok` case to read out `value`/`n`, so it is written as a `match`
//! throughout, exactly as `examples/ecosystem/oxicode-varint/src/harness.rs`
//! documents doing. Every comparison that *is* asserted is between two
//! primitive integers or two `bool`s (`core::cmp::impls`, builtin registry).

// A `#[harness]` body exists only under `formal` or under
// `all(test, oxiformal_runtime_checks)` (see `oxiformal_macros::harness`). In
// a plain `cargo build`/`cargo test` these imports are otherwise unused,
// exactly as `examples/ecosystem/oxicode-varint/src/harness.rs` documents.
#[cfg_attr(
    not(any(formal, all(test, oxiformal_runtime_checks))),
    allow(unused_imports)
)]
use oxicode::config;
#[cfg_attr(
    not(any(formal, all(test, oxiformal_runtime_checks))),
    allow(unused_imports)
)]
use oxicode::{
    decode_from_slice, decode_from_slice_with_config, encode_to_fixed_array,
    encode_to_fixed_array_with_config,
};
use oxiformal::prelude::*;

/// Worst-case encoded size of a `u16`/`i16`: one tag byte plus two payload
/// bytes.
pub const U16_BOUND: usize = 3;
/// Worst-case encoded size of a `u32`/`i32`.
pub const U32_BOUND: usize = 5;
/// Worst-case encoded size of a `u64`/`i64`.
pub const U64_BOUND: usize = 9;

/// Property: `assert`, three source sites. **Measured L1 verdict: unknown**
/// -- 23 obligations, 22 proved and 1 unknown. The round trip itself is
/// established: `decoded == value` (`:148`) and `consumed == written`
/// (`:149`) are both proved for every `u16`. The unknown is the `Err(_) =>
/// assert(false)` arm at `:151` -- "decoding the bytes the encoder just
/// produced cannot fail" -- `solver-model-rejected` on the OxiZ 0.3.3 pin
/// (U-Z10), not refuted, and expected to prove once the pin moves. The
/// outer `Err` arm at `:153` raises no item at all.
///
/// Stated in full: for every `u16`, `encode_to_fixed_array::<U16_BOUND, u16>`
/// under the default (variable-width, little-endian) configuration succeeds,
/// and `decode_from_slice::<u16>` on exactly the bytes written recovers the
/// value and consumes exactly that many bytes.
#[harness]
fn varint_u16_roundtrip_harness() {
    let value: u16 = any();
    match encode_to_fixed_array::<U16_BOUND, u16>(&value) {
        Ok((buf, written)) => match decode_from_slice::<u16>(&buf[..written]) {
            Ok((decoded, consumed)) => {
                assert(decoded == value);
                assert(consumed == written);
            }
            Err(_) => assert(false),
        },
        Err(_) => assert(false),
    }
}

/// Same statement as `varint_u16_roundtrip_harness` for `u32`, the width
/// whose encoding needs the two-byte-tag branch (`251 <= value <= 65535`) as
/// well as the four-byte-tag branch. Property: `assert`, three sites.
/// **Measured L1 verdict: unknown** -- 30 obligations, 29 proved and 1
/// unknown. `decoded == value` (`:170`) and `consumed == written` (`:171`)
/// are proved; the unknown is the same inner-decode `Err` arm, here at
/// `:173`, `solver-model-rejected` on OxiZ 0.3.3.
#[harness]
fn varint_u32_roundtrip_harness() {
    let value: u32 = any();
    match encode_to_fixed_array::<U32_BOUND, u32>(&value) {
        Ok((buf, written)) => match decode_from_slice::<u32>(&buf[..written]) {
            Ok((decoded, consumed)) => {
                assert(decoded == value);
                assert(consumed == written);
            }
            Err(_) => assert(false),
        },
        Err(_) => assert(false),
    }
}

/// Same statement for `u64`, the widest value this package encodes. The
/// encoder's four-way branch on `value` (`<= 250`, `<= u16::MAX`,
/// `<= u32::MAX`, else) is the only control flow in `varint_encode_u64`.
/// Property: `assert`, three sites. **Measured L1 verdict: unknown** -- 41
/// obligations, 40 proved and 1 unknown. `decoded == value` (`:192`) and
/// `consumed == written` (`:193`) are proved; the unknown is the inner-decode
/// `Err` arm at `:195`, `solver-model-rejected` on OxiZ 0.3.3.
#[harness]
fn varint_u64_roundtrip_harness() {
    let value: u64 = any();
    match encode_to_fixed_array::<U64_BOUND, u64>(&value) {
        Ok((buf, written)) => match decode_from_slice::<u64>(&buf[..written]) {
            Ok((decoded, consumed)) => {
                assert(decoded == value);
                assert(consumed == written);
            }
            Err(_) => assert(false),
        },
        Err(_) => assert(false),
    }
}

/// Property: `assert`, one source site (`:220`). **Measured L1 verdict:
/// proved** -- and so is every other obligation of this harness: 20 proved,
/// nothing else. This is the strongest result in the package.
///
/// It states the boundary that makes [`U64_BOUND`] *tight*: an eight-byte
/// buffer (`U64_BOUND - 1`) holds `varint_encode_u64`'s output exactly when
/// `value <= u32::MAX` (single byte, or the two/four-byte-tag forms, all of
/// which fit in eight bytes) and cannot when `value > u32::MAX` (the encoder
/// then needs the nine-byte `U64_BYTE` tag form). Proved as a biconditional
/// over every `u64`, not sampled.
///
/// Deliberately a `Result::is_err` check, never a panic: `SliceWriter::write`
/// returns `Err(Error::UnexpectedEnd { .. })` when the buffer is too small
/// (`oxicode/src/enc/write.rs:80-88`); it does not trap. The 0 refutations
/// measured across this package are consistent with that reading.
#[harness]
fn varint_u64_bound_is_tight_harness() {
    let value: u64 = any();
    let result = encode_to_fixed_array::<{ U64_BOUND - 1 }, u64>(&value);
    assert(result.is_err() == (value > u32::MAX as u64));
}

/// Property: `assert`, three source sites (`:239`, `:242`, `:243`).
/// **Measured L1 verdict: proved** -- 13 obligations, all proved. Neither
/// `Err` arm (`:245`, `:248`) raises an item.
///
/// Stated in full: under `config::standard().with_fixed_int_encoding()`,
/// encoding a `u64` always writes exactly 8 bytes (the `IntEncoding::Fixed`
/// arm of `u64`'s `Encode` impl always calls `to_le_bytes`/`to_be_bytes`,
/// never the varint tag logic), and decoding those 8 bytes with the same
/// configuration recovers the value. The `Configuration` type parameter is
/// carried through the lowered instances like any other generic argument.
#[harness]
fn fixed_int_roundtrip_harness() {
    let value: u64 = any();
    let cfg = config::standard().with_fixed_int_encoding();
    match encode_to_fixed_array_with_config::<8, u64, _>(&value, cfg) {
        Ok((buf, written)) => {
            assert(written == 8);
            match decode_from_slice_with_config::<u64, _>(&buf[..written], cfg) {
                Ok((decoded, consumed)) => {
                    assert(decoded == value);
                    assert(consumed == written);
                }
                Err(_) => assert(false),
            }
        }
        Err(_) => assert(false),
    }
}

/// Property: `assert`, three source sites. **Measured L1 verdict: unknown**
/// -- 33 obligations, 32 proved and 1 unknown. The zigzag round trip itself
/// is established: `(v << 1) ^ (v >> 31)` maps every `i32` (including
/// `i32::MIN`, whose image is `u32::MAX`) onto a `u32` the unsigned codec
/// round-trips, and the inverse `(n >> 1) ^ -(n & 1)` recovers `v` exactly --
/// `decoded == value` (`:267`) and `consumed == written` (`:268`) are proved,
/// as are the `shift-overflow` and `neg-overflow` checks of the zigzag
/// arithmetic. The unknown is the inner-decode `Err` arm at `:270`,
/// `solver-model-rejected` on OxiZ 0.3.3.
#[harness]
fn zigzag_i32_roundtrip_harness() {
    let value: i32 = any();
    match encode_to_fixed_array::<U32_BOUND, i32>(&value) {
        Ok((buf, written)) => match decode_from_slice::<i32>(&buf[..written]) {
            Ok((decoded, consumed)) => {
                assert(decoded == value);
                assert(consumed == written);
            }
            Err(_) => assert(false),
        },
        Err(_) => assert(false),
    }
}

/// The `i64` half of the same statement, at the widest width this package
/// encodes. Property: `assert`, three sites. **Measured L1 verdict: unknown**
/// -- 44 obligations, 43 proved and 1 unknown; `decoded == value` (`:287`)
/// and `consumed == written` (`:288`) are proved, and the unknown is the
/// inner-decode `Err` arm at `:290`, `solver-model-rejected` on OxiZ 0.3.3.
#[harness]
fn zigzag_i64_roundtrip_harness() {
    let value: i64 = any();
    match encode_to_fixed_array::<U64_BOUND, i64>(&value) {
        Ok((buf, written)) => match decode_from_slice::<i64>(&buf[..written]) {
            Ok((decoded, consumed)) => {
                assert(decoded == value);
                assert(consumed == written);
            }
            Err(_) => assert(false),
        },
        Err(_) => assert(false),
    }
}

/// Property: `assert`, one source site (`:315`), which is this harness's
/// whole point. **Measured L1 verdict: unknown** -- and unlike the round-trip
/// harnesses, the unknown *is* the stated property, so it is **not
/// established on the OxiZ 0.3.3 pin**: `solver-model-rejected` (U-Z10), not
/// refuted, and expected to prove once the pin moves. The harness's other
/// 11 obligations (`arith-overflow`, `bounds-check`, `slice-range`) are all
/// proved; 12 in total.
///
/// Stated in full: a `u16` decode must refuse a stream whose tag byte
/// announces a wider integer (`U32_BYTE = 252`, `U64_BYTE = 253`,
/// `U128_BYTE = 254`) -- every one of those three first bytes is an error,
/// never a silently truncated value. Written over a fixed-size `[u8; 4]`
/// (not `any_vec`, per `I0-d.md` §5.4): the property is about the first byte
/// only, and the too-short-slice case is
/// `decode_never_panics_on_arbitrary_bytes_harness`'s statement, not this.
#[harness]
fn decode_rejects_a_wide_tag_harness() {
    let src: [u8; 4] = any();
    assume(src[0] == 252 || src[0] == 253 || src[0] == 254);
    assert(decode_from_slice::<u16>(&src).is_err());
}

/// No assertion of its own, so the whole-harness pseudo-key applies.
/// **Measured L1 verdict: `harness` proved** -- 8 obligations, every one
/// proved: 1 `arith-overflow` (`oxicode/src/lib.rs:951`), 4 `bounds-check` in
/// `varint/decode_unsigned.rs`, 3 `slice-range` in `de/read.rs`.
///
/// Stated in full: `decode_from_slice::<u64>` never panics for *any* twelve
/// bytes, whatever tag byte they start with -- it either returns a value or
/// an `Err`. `SliceReader::read` checks the requested length against the
/// remaining slice before it copies (`oxicode/src/de/read.rs:50-54`), and
/// `decode_from_slice`'s own `bytes_read` computation
/// (`src.len() - decoder.reader().slice.len()`, `oxicode/src/lib.rs:951`) is
/// the `arith-overflow` obligation this harness was written to cover. Both
/// are discharged.
///
/// `unwind = 16`, not the package default of 12: the nine-byte `U64_BYTE`
/// path plus the loop-header visit that reads the discriminant is the longest
/// control-flow walk this harness reaches. The run raised no
/// `unwinding-assertion` obligation here, so it gives no evidence either way
/// about whether that bound binds.
#[harness(unwind = 16)]
fn decode_never_panics_on_arbitrary_bytes_harness() {
    let src: [u8; 12] = any();
    let _ = decode_from_slice::<u64>(&src);
}

#[cfg(all(test, not(oxiformal_runtime_checks), not(formal)))]
mod plain_tests {
    use super::*;

    #[test]
    fn small_u16_values_take_one_byte() {
        let (buf, written) = encode_to_fixed_array::<U16_BOUND, u16>(&42u16).expect("encode");
        assert_eq!(written, 1);
        assert_eq!(buf[0], 42);
        assert_eq!(
            decode_from_slice::<u16>(&buf[..written]),
            Ok((42u16, 1usize))
        );
    }

    #[test]
    fn u32_roundtrips_across_every_tag_width() {
        for value in [0u32, 250, 251, u16::MAX as u32, 70_000, u32::MAX] {
            let (buf, written) = encode_to_fixed_array::<U32_BOUND, u32>(&value).expect("encode");
            assert_eq!(
                decode_from_slice::<u32>(&buf[..written]),
                Ok((value, written))
            );
        }
    }

    #[test]
    fn u64_max_uses_the_full_nine_byte_form() {
        let (buf, written) = encode_to_fixed_array::<U64_BOUND, u64>(&u64::MAX).expect("encode");
        assert_eq!(written, U64_BOUND);
        assert_eq!(
            decode_from_slice::<u64>(&buf[..written]),
            Ok((u64::MAX, written))
        );
    }

    /// Concrete witness for the boundary `super::varint_u64_bound_is_tight_harness`
    /// states: the value one past `u32::MAX` needs nine bytes, so an
    /// eight-byte buffer rejects it, and `u32::MAX` itself still fits.
    #[test]
    fn nine_bytes_are_necessary_above_u32_max() {
        let value = u64::from(u32::MAX) + 1;
        assert!(encode_to_fixed_array::<{ U64_BOUND - 1 }, u64>(&value).is_err());
        assert!(encode_to_fixed_array::<{ U64_BOUND - 1 }, u64>(&u64::from(u32::MAX)).is_ok());
    }

    #[test]
    fn fixed_int_encoding_is_always_eight_bytes() {
        let cfg = config::standard().with_fixed_int_encoding();
        for value in [0u64, 1, 250, 251, u32::MAX as u64, u64::MAX] {
            let (buf, written) =
                encode_to_fixed_array_with_config::<8, u64, _>(&value, cfg).expect("encode");
            assert_eq!(written, 8);
            assert_eq!(
                decode_from_slice_with_config::<u64, _>(&buf[..written], cfg),
                Ok((value, written))
            );
        }
    }

    #[test]
    fn zigzag_maps_i32_min_to_u32_max() {
        let (buf, written) = encode_to_fixed_array::<U32_BOUND, i32>(&i32::MIN).expect("encode");
        assert_eq!(
            decode_from_slice::<u32>(&buf[..written]),
            Ok((u32::MAX, written))
        );
        assert_eq!(
            decode_from_slice::<i32>(&buf[..written]),
            Ok((i32::MIN, written))
        );
    }

    #[test]
    fn zigzag_i64_roundtrips_at_the_extremes() {
        for value in [i64::MIN, -1, 0, 1, i64::MAX] {
            let (buf, written) = encode_to_fixed_array::<U64_BOUND, i64>(&value).expect("encode");
            assert_eq!(
                decode_from_slice::<i64>(&buf[..written]),
                Ok((value, written))
            );
        }
    }

    #[test]
    fn a_wide_tag_is_rejected_by_the_u16_decoder() {
        for tag in [252u8, 253, 254] {
            let src = [tag, 0, 0, 0];
            assert!(decode_from_slice::<u16>(&src).is_err());
        }
    }

    #[test]
    fn u16_still_accepts_its_own_two_byte_tag() {
        // 251 (U16_BYTE) is u16's own wide-value tag, not a rejection case.
        let src = [251u8, 0x34, 0x12, 0];
        assert_eq!(decode_from_slice::<u16>(&src), Ok((0x1234u16, 3)));
    }

    #[test]
    fn decode_u64_never_panics_on_short_or_garbage_input() {
        let cases: &[[u8; 12]] = &[
            [0u8; 12],
            [255u8; 12],
            [253, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [253, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0],
        ];
        for src in cases {
            let _ = decode_from_slice::<u64>(src);
        }
    }
}
