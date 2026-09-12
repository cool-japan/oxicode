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
//! # Measured reality: 0 proved / 0 refuted / 9 unsupported, and why
//!
//! `encode_to_fixed_array::<const N: usize, E: Encode>` builds `[0u8; N]`
//! for a **const generic parameter**, and `decode_from_slice::<D: Decode>`
//! reaches `SliceReader::read` through a `R: Reader` trait bound. D2
//! (on-demand monomorphic instance lowering) handles both of those cases
//! well: a real `cargo formal check` run (release CLI + release driver with
//! D1/D2, `--target-dir` recorded in the Phase 2b report) shows **68
//! monomorphic instances lowered, all 68 reachable**, including every
//! `varint_encode_*`/`varint_decode_*` function, every per-type
//! `Encode::encode`/`Decode::decode` impl for `u16`/`u32`/`u64`/`i32`/`i64`,
//! `EncoderImpl::new`/`into_writer`, `DecoderImpl::new`/`claim_bytes_read`,
//! `SliceWriter::new`/`bytes_written`, `SliceReader::new`, and every
//! `config::*` helper this package calls.
//!
//! What D2 does **not** yet lower is two specific instances that every one
//! of the nine harnesses below transitively calls:
//! `<oxicode::enc::EncoderImpl<W, C> as oxicode::enc::Encoder>::writer`
//! (`oxicode/src/enc/encoder.rs:46`) and
//! `<oxicode::de::DecoderImpl<R, C> as oxicode::de::Decoder>::reader`
//! (`oxicode/src/de/decoder.rs`, the symmetric accessor). Both impls define
//! their associated type as exactly their own generic parameter
//! (`type W = W;` / `type R = R;`), and at the monomorphic instance
//! (`W = SliceWriter<'_>`, `C = Configuration`) the driver reports:
//!
//! ```text
//! the driver lowered no body for `oxicode::enc::encoder::{impl-1}::writer::{inst-...}`:
//! unsupported-type(<oxicode::enc::EncoderImpl<oxicode::enc::SliceWriter<'_>,
//! oxicode::config::Configuration> as oxicode::enc::Encoder>::W)
//! ```
//!
//! (symmetrically for `{impl-2}::reader` and `::R`). `FORMAL_LOG=debug`
//! shows the **generic definition** of `writer`/`reader` lowering fine as a
//! dependency body (`lowered dependency oxicode::enc::encoder::{impl-1}::writer
//! (body: true)`) -- its body is the trivial `&mut self.writer` field access
//! -- but that generic body has no encodable layout for an abstract `W`, so
//! the encoder needs the *instance*, and the instance's return type
//! (`Self::W`) is left as an un-normalized associated-type projection rather
//! than reduced to the concrete `SliceWriter<'_>` the substitution implies.
//! Both accessors are called from every `Encode`/`Decode` impl in this
//! crate (`encoder.writer().write(..)`, `decoder.reader().read(..)`,
//! `oxicode/src/enc/impls.rs`, `oxicode/src/de/impls.rs`), so this one gap
//! blocks every harness in this package identically -- it is not a property
//! of any one harness, and no harness rewrite avoids it: `writer()`/
//! `reader()` are `oxicode`'s own dispatch mechanism, unreachable from
//! outside it, and there is no public entry point into the varint codec
//! that does not go through them.
//!
//! **This is a different, more specific finding than the harness inventory
//! (`I0-d.md` §5.4) predicted.** That inventory's "proved (post-D2)"
//! prediction was calibrated against the in-repository vendored twin,
//! `examples/ecosystem/oxicode-varint`, which measured 176 proved / 1
//! refuted / 2 unknown once D2 landed -- but the vendored twin's harnesses
//! call the raw `varint_encode_*`/`varint_decode_*` functions through its
//! *own* hand-rolled `Writer`/`Reader` generics (`ArrayWriter`/
//! `SliceReader` defined in the trial itself), never through `oxicode`'s
//! real `Encoder`/`Decoder`/`EncoderImpl`/`DecoderImpl` dispatch, because
//! `oxicode`'s `varint` module is `pub(crate)` and was vendored rather than
//! called. This package calls the **real public API**
//! (`encode_to_fixed_array`, `decode_from_slice`), which is exactly what
//! routes every harness through the two accessors above -- an indirection
//! layer the vendored twin structurally could not exercise. Put plainly:
//! this package found a real, narrowly-localized driver gap that the
//! vendored trial's design made invisible.
//!
//! One open question this package cannot resolve from the release binary
//! alone, for the D2 owner: design v1.1 §3.2 item 2 says a call site's
//! `Callee::path` becomes the instance path "only after the driver has
//! queued/lowered that instance". The debug log is consistent with either
//! reading of "lowered" -- "attempted" (current behaviour is as designed;
//! the fix is purely normalizing the projection) or "succeeded" (the
//! redirect onto a bodyless instance is itself a deviation, and a
//! successful-instance-only redirect would have kept the call at the
//! definition path, which has a body, even though that body still could
//! not be encoded for an abstract `W`/`R`). This package does not guess
//! which; see `EXPECTED.toml` and the Phase 2b report for the raw evidence.
//!
//! # Why `match` and never `assert(.. == Ok(..))`
//!
//! `Result<T, oxicode::error::Error>`'s `PartialEq` lives in `oxicode`
//! itself (a `#[derive]`), and even where D1 lowers the derived impl, a
//! direct `assert(encode(..) == Ok((value, n)))` is needless risk for zero
//! benefit here: every harness below already needs to open the `Ok` case to
//! read out `value`/`n`, so it is written as a `match` throughout, exactly
//! as `examples/ecosystem/oxicode-varint/src/harness.rs` documents doing.
//! Every comparison that *is* asserted is between two primitive integers or
//! two `bool`s (`core::cmp::impls`, always in the builtin registry).

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

/// Property intended: for every `u16`, `encode_to_fixed_array::<U16_BOUND,
/// u16>` under the default (variable-width, little-endian) configuration
/// succeeds, and `decode_from_slice::<u16>` on exactly the bytes written
/// recovers the value and consumes exactly that many bytes.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Blocked at
/// `oxicode/src/enc/impls.rs:44` (`encoder.writer()`, the `u16::encode`
/// call site) by the un-normalized `Self::W` projection on
/// `<EncoderImpl<SliceWriter<'_>, Configuration> as Encoder>::writer`'s
/// monomorphic instance -- see the module docs above for the full finding.
/// Not reached by the solver: this row costs nothing against the
/// `solver-model-rejected`/OxiZ-0.3.3 budget, and moving the OxiZ pin does
/// not change it.
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

/// Property intended: same statement as
/// [`varint_u16_roundtrip_harness`] for `u32`, the width whose encoding
/// needs the two-byte-tag branch (`251 <= value <= 65535`) as well as the
/// four-byte-tag branch.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Same root
/// cause, blocked at `oxicode/src/enc/impls.rs:58`.
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

/// Property intended: same statement for `u64`, the widest value this
/// package encodes. The encoder's four-way branch on `value` (`<= 250`,
/// `<= u16::MAX`, `<= u32::MAX`, else) is the only control flow in
/// `varint_encode_u64`.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Same root
/// cause, blocked at `oxicode/src/enc/impls.rs:72`.
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

/// Property intended: states the boundary that makes [`U64_BOUND`] *tight*:
/// an eight-byte buffer (`U64_BOUND - 1`) can hold `varint_encode_u64`'s
/// output exactly when `value <= u32::MAX` (single byte, or the
/// two/four-byte-tag forms all fit in eight bytes) and cannot when
/// `value > u32::MAX` (the encoder then needs the nine-byte `U64_BYTE` tag
/// form). Deliberately a `Result::is_err` check, never a panic:
/// `SliceWriter::write` returns `Err(Error::UnexpectedEnd { .. })` when the
/// buffer is too small (`oxicode/src/enc/write.rs:80-88`); it does not trap.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Same root
/// cause as [`varint_u16_roundtrip_harness`], blocked at
/// `oxicode/src/enc/impls.rs:72` (`u64::encode`'s call to `encoder.writer()`,
/// reached with `N = U64_BOUND - 1` here). The `Result::is_err`-vs-panic
/// design point above is a source-reading claim, not something this run
/// verified.
#[harness]
fn varint_u64_bound_is_tight_harness() {
    let value: u64 = any();
    let result = encode_to_fixed_array::<{ U64_BOUND - 1 }, u64>(&value);
    assert(result.is_err() == (value > u32::MAX as u64));
}

/// Property intended: under `config::standard().with_fixed_int_encoding()`,
/// encoding a `u64` always writes exactly 8 bytes (the `IntEncoding::Fixed`
/// arm of `u64`'s `Encode` impl always calls `to_le_bytes`/`to_be_bytes`,
/// never the varint tag logic), and decoding those 8 bytes with the same
/// configuration recovers the value.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Same root
/// cause, blocked at `oxicode/src/enc/impls.rs:76` -- the `Self::W`
/// projection gap does not depend on which `IntEncoding`/`Endianness` the
/// `Configuration` type parameter carries.
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

/// Property intended: zigzag round trip for `i32`: `(v << 1) ^ (v >> 31)`
/// maps every `i32` (including `i32::MIN`, whose image is `u32::MAX`) onto
/// a `u32` that the unsigned varint codec above already round-trips, and
/// the inverse `(n >> 1) ^ -(n & 1)` recovers `v` exactly.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Same root
/// cause, blocked at `oxicode/src/enc/impls.rs:136` (`i32::encode`'s call
/// to `encoder.writer()`, after the zigzag transform has already produced
/// the `u32` to write).
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

/// Property intended: the `i64` half of the same statement, at the widest
/// width this package encodes.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Same root
/// cause, blocked at `oxicode/src/enc/impls.rs:150`.
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

/// Property intended: a `u16` decode must refuse a stream whose tag byte
/// announces a wider integer (`U32_BYTE = 252`, `U64_BYTE = 253`,
/// `U128_BYTE = 254`): every one of those three first bytes is an error,
/// never a silently truncated value. Written over a fixed-size `[u8; 4]`
/// (not `any_vec`, per `I0-d.md` §5.4's own recommendation on that point):
/// the property is about the first byte only, and `decode_from_slice`'s
/// bound-check on a too-short slice is exactly
/// [`decode_never_panics_on_arbitrary_bytes_harness`]'s statement, not this
/// one's.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Blocked at
/// `oxicode/src/de/impls.rs:60` (`u16::decode`'s call to `decoder.reader()`)
/// by the symmetric `Self::R` projection gap on
/// `<DecoderImpl<SliceReader<'_>, Configuration> as Decoder>::reader`'s
/// monomorphic instance -- see the module docs above.
#[harness]
fn decode_rejects_a_wide_tag_harness() {
    let src: [u8; 4] = any();
    assume(src[0] == 252 || src[0] == 253 || src[0] == 254);
    assert(decode_from_slice::<u16>(&src).is_err());
}

/// Property intended, no assertion of its own: `decode_from_slice::<u64>`
/// never panics for *any* twelve bytes, whatever tag byte they start with --
/// it either returns a value or an `Err`. `SliceReader::read` checks the
/// requested length against the remaining slice before it copies
/// (`oxicode/src/de/read.rs:50-54`), and `decode_from_slice`'s own
/// `bytes_read` computation (`src.len() - decoder.reader().slice.len()`,
/// `oxicode/src/lib.rs:951`) is an `arith-overflow` obligation this harness
/// was also meant to cover.
///
/// `unwind = 16`, not the package default of 12: the nine-byte `U64_BYTE`
/// path plus the loop-header visit that reads the discriminant is the
/// longest control-flow walk this harness would have reached.
///
/// **MEASURED L1 verdict: `harness` unsupported (`no-body`).** Blocked at
/// `oxicode/src/de/impls.rs:98` (`u64::decode`'s call to `decoder.reader()`),
/// the same `Self::R` projection gap as
/// [`decode_rejects_a_wide_tag_harness`]. The `unwind` bound was never
/// exercised: the driver refused to lower a body before any unwinding
/// happened.
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

    /// Concrete witness for the boundary [`super::varint_u64_bound_is_tight_harness`]
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
