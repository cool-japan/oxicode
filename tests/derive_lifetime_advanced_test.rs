//! Advanced lifetime-related derive tests for OxiCode.
//!
//! Covers:
//! - Structs with `'de` lifetime and borrowed `&'de str` / `&'de [u8]` fields
//! - Multiple lifetime fields in a single struct
//! - Enum variants borrowing `&'de str`
//! - Generic wrapper structs with lifetimes
//! - Mixed owned (`String`) and borrowed (`&'de str`) fields
//! - Nested structs with borrowed inner fields
//! - Lifetime of decoded `&str` references tied to source buffer
//! - Borrow-decode vs. owned-decode value equivalence
//! - Multiple borrows from the same encoded buffer
//! - `Cow<'de, str>` and `Cow<'de, [u8]>` zero-copy decode
//! - Byte count verification for `borrow_decode_from_slice`
//! - `Vec<u8>` and `&'de str` coexisting in the same struct
//! - Large and empty string/slice borrow-decode
//! - Re-encode after borrow-decode produces identical bytes

#![allow(
    clippy::approx_constant,
    clippy::useless_vec,
    clippy::len_zero,
    clippy::unnecessary_cast,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_borrow,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms,
    clippy::inconsistent_digit_grouping,
    clippy::unit_cmp,
    clippy::assertions_on_constants,
    clippy::iter_on_single_items,
    clippy::expect_fun_call,
    clippy::redundant_pattern_matching,
    variant_size_differences,
    clippy::absurd_extreme_comparisons,
    clippy::nonminimal_bool,
    clippy::for_kv_map,
    clippy::needless_range_loop,
    clippy::single_match,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::map_entry,
    clippy::match_single_binding,
    clippy::bool_comparison,
    clippy::derivable_impls,
    clippy::manual_range_contains,
    clippy::needless_borrows_for_generic_args,
    clippy::manual_map,
    clippy::vec_init_then_push,
    clippy::identity_op,
    clippy::manual_flatten,
    clippy::single_char_pattern,
    clippy::search_is_some,
    clippy::option_map_unit_fn,
    clippy::while_let_on_iterator,
    clippy::clone_on_copy,
    clippy::box_collection,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::large_enum_variant,
    clippy::match_ref_pats,
    clippy::needless_pass_by_value,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::manual_strip,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::wrong_self_convention,
    clippy::inherent_to_string,
    clippy::new_without_default,
    clippy::unnecessary_wraps,
    clippy::field_reassign_with_default,
    clippy::manual_find,
    clippy::unnecessary_lazy_evaluations,
    clippy::should_implement_trait,
    clippy::missing_safety_doc,
    clippy::unusual_byte_groupings,
    clippy::bool_assert_comparison,
    clippy::zero_prefixed_literal,
    clippy::await_holding_lock,
    clippy::manual_saturating_arithmetic,
    clippy::explicit_counter_loop,
    clippy::needless_lifetimes,
    clippy::single_component_path_imports,
    clippy::uninlined_format_args,
    clippy::iter_cloned_collect,
    clippy::manual_str_repeat,
    clippy::excessive_precision,
    clippy::precedence,
    clippy::unnecessary_literal_unwrap
)]
extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use oxicode::{encode_to_vec, BorrowDecode, Decode, Encode};

// ── Type definitions ──────────────────────────────────────────────────────────

/// Test 1: Struct with `'de` lifetime and a single `&'de str` field.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct SimpleStrBorrow<'de> {
    text: &'de str,
}

/// Test 2: Struct with `'de` lifetime and a single `&'de [u8]` field.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct SimpleSliceBorrow<'de> {
    data: &'de [u8],
}

/// Test 3: Struct with two distinct borrowed fields.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct TwoBorrows<'de> {
    key: &'de str,
    value: &'de [u8],
}

/// Test 4: Enum where one variant borrows `&'de str`, another is unit.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
enum BorrowEnum<'de> {
    Unit,
    Text(&'de str),
}

/// Test 5: Generic wrapper with lifetime parameter.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct Wrapper<'de, T>
where
    T: BorrowDecode<'de> + PartialEq + core::fmt::Debug,
{
    inner: T,
    tag: u32,
    _marker: core::marker::PhantomData<&'de ()>,
}

/// Test 6: Struct mixing owned `String` and borrowed `&'de str`.
/// We keep ownership on the `owned` side via a separate owned struct, then
/// borrow-decode the combination.  To avoid the orphan problem we do NOT call
/// `impl_borrow_decode!(String)`; instead we wrap the mixed pattern in a
/// struct where the owned field is `u64` (a primitive that already has
/// `impl_borrow_decode` in the library) and the borrowed field is `&'de str`.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct MixedPrimitiveBorrow<'de> {
    id: u64,
    label: &'de str,
}

/// Test 7: Outer struct with owned fields (no lifetime).
#[derive(Debug, PartialEq, Encode, Decode)]
struct OuterOwned {
    x: u32,
    y: u32,
}

// Provide BorrowDecode for OuterOwned via the macro (local type — OK).
oxicode::impl_borrow_decode!(OuterOwned);

/// Inner struct embeds OuterOwned and adds a borrowed &'de str field.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct InnerBorrowed<'de> {
    outer: OuterOwned,
    note: &'de str,
}

/// Tests 11 & 12: Struct holding `Cow<'de, str>`.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct CowStrHolder<'de> {
    text: Cow<'de, str>,
    id: u64,
}

/// Test 12: Struct holding `Cow<'de, [u8]>`.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct CowSliceHolder<'de> {
    data: Cow<'de, [u8]>,
}

/// Test 14: Struct with `&'de [u8]` (borrowed) AND `&'de str` (also borrowed) to
/// test that two borrowed heterogeneous types coexist. We treat the "Vec-like"
/// scenario by encoding a `&[u8]` value that the test constructs from a Vec.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct SliceAndStr<'de> {
    payload: &'de [u8],
    name: &'de str,
}

/// Test 18: Struct with three different borrowed fields.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct ThreeBorrows<'de> {
    alpha: &'de str,
    beta: &'de [u8],
    gamma: &'de str,
}

/// Test 19: Enum where some variants borrow and some own.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
enum MixedBorrowEnum<'de> {
    OwnedCount(u64),
    BorrowedText(&'de str),
    BorrowedSlice(&'de [u8]),
    Both { name: &'de str, id: u32 },
}

// ── Helper ─────────────────────────────────────────────────────────────────────

/// Build an encoded buffer by concatenating individually-encoded values.
/// Each value must implement `Encode` by value (not by reference).
fn concat_encoded(parts: &[Vec<u8>]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    for part in parts {
        buf.extend_from_slice(part);
    }
    buf
}

// ── Tests ──────────────────────────────────────────────────────────────────────

mod derive_lifetime_advanced_tests {
    use super::*;

    // ── Test 1: &'de str field BorrowDecode roundtrip ────────────────────────

    #[test]
    fn test_adv_str_field_borrow_decode_roundtrip() {
        let text = "lifetime-str";
        let buf = encode_to_vec(&text).expect("encode str");

        let (decoded, consumed): (SimpleStrBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode SimpleStrBorrow");

        assert_eq!(decoded.text, "lifetime-str");
        assert_eq!(consumed, buf.len());
    }

    // ── Test 2: &'de [u8] field BorrowDecode roundtrip ───────────────────────

    #[test]
    fn test_adv_slice_field_borrow_decode_roundtrip() {
        let data: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
        let buf = encode_to_vec(&data).expect("encode slice");

        let (decoded, consumed): (SimpleSliceBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode SimpleSliceBorrow");

        assert_eq!(decoded.data, data);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 3: Multiple lifetime fields in one struct ────────────────────────

    #[test]
    fn test_adv_two_borrows_borrow_decode() {
        let key = "my-key";
        let value: &[u8] = &[1, 2, 3, 4, 5];
        let buf = concat_encoded(&[
            encode_to_vec(&key).expect("encode key"),
            encode_to_vec(&value).expect("encode value"),
        ]);

        let (decoded, consumed): (TwoBorrows<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode TwoBorrows");

        assert_eq!(decoded.key, key);
        assert_eq!(decoded.value, value);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 4: Enum with 'de lifetime variant containing &'de str ───────────

    #[test]
    fn test_adv_enum_borrowed_variant() {
        // Encode the `Text` variant: discriminant (u32) = 1, then the str payload.
        let text = "enum-borrow";
        let discriminant: u32 = 1;
        let buf = concat_encoded(&[
            encode_to_vec(&discriminant).expect("encode discriminant"),
            encode_to_vec(&text).expect("encode text"),
        ]);

        let (decoded, consumed): (BorrowEnum<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode BorrowEnum::Text");

        assert_eq!(decoded, BorrowEnum::Text("enum-borrow"));
        assert_eq!(consumed, buf.len());
    }

    // ── Test 5: Generic struct Wrapper<'de, T> ────────────────────────────────

    #[test]
    fn test_adv_generic_wrapper_borrow_decode() {
        let inner: &str = "wrapped-value";
        let tag: u32 = 42;
        let buf = concat_encoded(&[
            encode_to_vec(&inner).expect("encode inner"),
            encode_to_vec(&tag).expect("encode tag"),
        ]);

        let (decoded, consumed): (Wrapper<'_, &str>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode Wrapper<&str>");

        assert_eq!(decoded.inner, "wrapped-value");
        assert_eq!(decoded.tag, 42);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 6: Mixed primitive owned field and borrowed &'de str ────────────

    #[test]
    fn test_adv_mixed_primitive_and_borrowed() {
        let id: u64 = 0xCAFE_BABE_DEAD_BEEF;
        let label = "borrowed-label";
        let buf = concat_encoded(&[
            encode_to_vec(&id).expect("encode id"),
            encode_to_vec(&label).expect("encode label"),
        ]);

        let (decoded, consumed): (MixedPrimitiveBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode MixedPrimitiveBorrow");

        assert_eq!(decoded.id, 0xCAFE_BABE_DEAD_BEEF);
        assert_eq!(decoded.label, "borrowed-label");
        assert_eq!(consumed, buf.len());
    }

    // ── Test 7: Nested outer owns, inner borrows ──────────────────────────────

    #[test]
    fn test_adv_nested_outer_owned_inner_borrowed() {
        let outer = OuterOwned { x: 10, y: 20 };
        let note = "inner-note";
        let buf = concat_encoded(&[
            encode_to_vec(&outer).expect("encode outer"),
            encode_to_vec(&note).expect("encode note"),
        ]);

        let (decoded, consumed): (InnerBorrowed<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode InnerBorrowed");

        assert_eq!(decoded.outer.x, 10);
        assert_eq!(decoded.outer.y, 20);
        assert_eq!(decoded.note, "inner-note");
        assert_eq!(consumed, buf.len());
    }

    // ── Test 8: Decoded &str reference lives as long as the source buffer ─────

    #[test]
    fn test_adv_decoded_str_lifetime_tied_to_buffer() {
        let source_string = "borrow-lifetime-check".to_string();
        let buf = encode_to_vec(&source_string.as_str()).expect("encode str for lifetime");

        let (decoded, _): (SimpleStrBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode for lifetime check");

        // The str bytes must overlap with `buf`'s memory (zero-copy).
        let buf_start = buf.as_ptr() as usize;
        let buf_end = buf_start + buf.len();
        let str_start = decoded.text.as_ptr() as usize;
        let str_end = str_start + decoded.text.len();

        assert!(
            str_start >= buf_start && str_end <= buf_end,
            "decoded &str should point inside the source buffer (zero-copy)"
        );
    }

    // ── Test 9: Borrow-decode vs. owned-decode produce the same string value ──

    #[test]
    fn test_adv_borrow_vs_owned_decode_same_value() {
        let original = "compare-borrow-vs-owned";
        let buf = encode_to_vec(&original).expect("encode str for comparison");

        // BorrowDecode path — yields a &str
        let (borrowed, _): (&str, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode &str");

        // Decode path — yields an owned String
        let (owned, _): (String, usize) = oxicode::decode_from_slice(&buf).expect("decode String");

        assert_eq!(borrowed, owned.as_str());
    }

    // ── Test 10: Multiple borrows from the same encoded buffer ────────────────

    #[test]
    fn test_adv_multiple_borrows_from_same_buffer() {
        let s1 = "first";
        let s2: &[u8] = &[10, 20, 30];
        let buf = concat_encoded(&[
            encode_to_vec(&s1).expect("encode s1"),
            encode_to_vec(&s2).expect("encode s2"),
        ]);

        let (decoded, _): (TwoBorrows<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode TwoBorrows");

        // Both fields should reference memory inside `buf`.
        let buf_ptr = buf.as_ptr() as usize;
        let buf_end = buf_ptr + buf.len();

        let str_ptr = decoded.key.as_ptr() as usize;
        assert!(
            str_ptr >= buf_ptr && str_ptr < buf_end,
            "key &str must borrow from the buffer"
        );

        let slice_ptr = decoded.value.as_ptr() as usize;
        assert!(
            slice_ptr >= buf_ptr && slice_ptr < buf_end,
            "value &[u8] must borrow from the buffer"
        );
    }

    // ── Test 11: Cow<'de, str> borrowed on borrow_decode ─────────────────────

    #[test]
    fn test_adv_cow_str_borrowed_on_borrow_decode() {
        let text = "cow-borrowed-str";
        let id: u64 = 0xCAFE_BABE_1234_5678;
        let buf = concat_encoded(&[
            encode_to_vec(&text).expect("encode cow text"),
            encode_to_vec(&id).expect("encode id"),
        ]);

        let (decoded, consumed): (CowStrHolder<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode CowStrHolder");

        // After borrow_decode the Cow must be the Borrowed variant, not Owned.
        assert!(
            matches!(decoded.text, Cow::Borrowed(_)),
            "Cow<str> should be Borrowed after borrow_decode"
        );
        assert_eq!(decoded.text.as_ref(), "cow-borrowed-str");
        assert_eq!(decoded.id, 0xCAFE_BABE_1234_5678);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 12: Cow<'de, [u8]> borrowed on borrow_decode ────────────────────

    #[test]
    fn test_adv_cow_slice_borrowed_on_borrow_decode() {
        let data: &[u8] = &[0xAA, 0xBB, 0xCC, 0xDD];
        let buf = encode_to_vec(&data).expect("encode cow slice");

        let (decoded, consumed): (CowSliceHolder<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode CowSliceHolder");

        assert!(
            matches!(decoded.data, Cow::Borrowed(_)),
            "Cow<[u8]> should be Borrowed after borrow_decode"
        );
        assert_eq!(decoded.data.as_ref(), data);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 13: borrow_decode_from_slice returns correct consumed byte count ─

    #[test]
    fn test_adv_borrow_decode_correct_consumed_count() {
        // Encode `first` alone, then append `second` — decode only the first
        // field and verify the consumed count equals the first field's size.
        let first = "count-check";
        let second: u32 = 999;
        let first_bytes = encode_to_vec(&first).expect("encode first");
        let first_encoded_len = first_bytes.len();
        let buf = concat_encoded(&[first_bytes, encode_to_vec(&second).expect("encode second")]);

        let (_, consumed): (SimpleStrBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode count check");

        assert_eq!(
            consumed, first_encoded_len,
            "consumed byte count should match bytes used for the first field only"
        );
    }

    // ── Test 14: &'de [u8] and &'de str coexist in the same struct ───────────

    #[test]
    fn test_adv_slice_and_str_in_same_struct() {
        let payload: Vec<u8> = vec![1, 2, 3, 4, 5];
        let name = "payload-name";
        let buf = concat_encoded(&[
            encode_to_vec(&payload.as_slice()).expect("encode payload"),
            encode_to_vec(&name).expect("encode name"),
        ]);

        let (decoded, consumed): (SliceAndStr<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode SliceAndStr");

        assert_eq!(decoded.payload, payload.as_slice());
        assert_eq!(decoded.name, "payload-name");
        assert_eq!(consumed, buf.len());
    }

    // ── Test 15: BorrowDecode of long string (1 KB) ───────────────────────────

    #[test]
    fn test_adv_long_string_borrow_decode() {
        let long_str: String = "A".repeat(1024);
        let buf = encode_to_vec(&long_str.as_str()).expect("encode 1KB str");

        let (decoded, consumed): (SimpleStrBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode 1KB str");

        assert_eq!(decoded.text.len(), 1024);
        assert_eq!(decoded.text, long_str.as_str());
        assert_eq!(consumed, buf.len());

        // Verify zero-copy: the decoded &str must point into the encoded buffer.
        let buf_ptr = buf.as_ptr() as usize;
        let buf_end = buf_ptr + buf.len();
        let str_ptr = decoded.text.as_ptr() as usize;
        assert!(
            str_ptr >= buf_ptr && str_ptr < buf_end,
            "1 KB &str should zero-copy from buffer"
        );
    }

    // ── Test 16: BorrowDecode of empty string ─────────────────────────────────

    #[test]
    fn test_adv_empty_str_borrow_decode() {
        let empty_str: &str = "";
        let buf = encode_to_vec(&empty_str).expect("encode empty str");

        let (decoded, consumed): (SimpleStrBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode empty str");

        assert_eq!(decoded.text, "");
        assert_eq!(decoded.text.len(), 0);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 17: BorrowDecode of empty &[u8] ──────────────────────────────────

    #[test]
    fn test_adv_empty_slice_borrow_decode() {
        let empty: &[u8] = &[];
        let buf = encode_to_vec(&empty).expect("encode empty slice");

        let (decoded, consumed): (SimpleSliceBorrow<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode empty slice");

        assert_eq!(decoded.data, empty);
        assert_eq!(decoded.data.len(), 0);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 18: Struct with 3 different borrowed fields ─────────────────────

    #[test]
    fn test_adv_three_borrowed_fields() {
        let alpha = "alpha-text";
        let beta: &[u8] = &[0xFF, 0x00, 0xAB];
        let gamma = "gamma-text";
        let buf = concat_encoded(&[
            encode_to_vec(&alpha).expect("encode alpha"),
            encode_to_vec(&beta).expect("encode beta"),
            encode_to_vec(&gamma).expect("encode gamma"),
        ]);

        let (decoded, consumed): (ThreeBorrows<'_>, usize) =
            oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode ThreeBorrows");

        assert_eq!(decoded.alpha, alpha);
        assert_eq!(decoded.beta, beta);
        assert_eq!(decoded.gamma, gamma);
        assert_eq!(consumed, buf.len());
    }

    // ── Test 19: Enum: some variants borrow, some own ────────────────────────

    #[test]
    fn test_adv_enum_mixed_borrow_and_own_variants() {
        // Variant OwnedCount (discriminant 0)
        let owned_buf = concat_encoded(&[
            encode_to_vec(&0u32).expect("encode disc 0"),
            encode_to_vec(&0xDEAD_BEEFu64).expect("encode count"),
        ]);
        let (decoded_owned, _): (MixedBorrowEnum<'_>, usize) =
            oxicode::borrow_decode_from_slice(&owned_buf).expect("borrow_decode OwnedCount");
        assert_eq!(decoded_owned, MixedBorrowEnum::OwnedCount(0xDEAD_BEEF));

        // Variant BorrowedText (discriminant 1)
        let text_buf = concat_encoded(&[
            encode_to_vec(&1u32).expect("encode disc 1"),
            encode_to_vec(&"mixed-text").expect("encode text"),
        ]);
        let (decoded_text, _): (MixedBorrowEnum<'_>, usize) =
            oxicode::borrow_decode_from_slice(&text_buf).expect("borrow_decode BorrowedText");
        assert_eq!(decoded_text, MixedBorrowEnum::BorrowedText("mixed-text"));

        // Variant BorrowedSlice (discriminant 2)
        let slice: &[u8] = &[0x01, 0x02, 0x03];
        let slice_buf = concat_encoded(&[
            encode_to_vec(&2u32).expect("encode disc 2"),
            encode_to_vec(&slice).expect("encode slice"),
        ]);
        let (decoded_slice, _): (MixedBorrowEnum<'_>, usize) =
            oxicode::borrow_decode_from_slice(&slice_buf).expect("borrow_decode BorrowedSlice");
        assert_eq!(decoded_slice, MixedBorrowEnum::BorrowedSlice(slice));
    }

    // ── Test 20: BorrowDecode then re-encode produces same bytes ─────────────

    #[test]
    fn test_adv_borrow_decode_then_reencode_same_bytes() {
        let original_key = "reencode-key";
        let original_value: &[u8] = &[10, 20, 30, 40, 50];
        let original_buf = concat_encoded(&[
            encode_to_vec(&original_key).expect("encode key for reencode"),
            encode_to_vec(&original_value).expect("encode value for reencode"),
        ]);

        // Borrow-decode from the buffer.
        let (decoded, _): (TwoBorrows<'_>, usize) =
            oxicode::borrow_decode_from_slice(&original_buf)
                .expect("borrow_decode for reencode test");

        // Re-encode the decoded struct.
        let reencoded = encode_to_vec(&decoded).expect("re-encode TwoBorrows");

        // The re-encoded bytes must be byte-for-byte identical to the original.
        assert_eq!(
            reencoded, original_buf,
            "re-encoding a borrow-decoded struct must produce identical bytes"
        );
    }
}
