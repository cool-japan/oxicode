//! Tests for derived structs with lifetime parameters and BorrowDecode.
//!
//! Covers:
//! - Structs borrowing `&str` via `BorrowDecode`
//! - Structs borrowing `&[u8]` via `BorrowDecode`
//! - Owned structs with `Encode + Decode` roundtrip
//! - Mixed owned structs with multiple field types
//! - Tuple structs with lifetime and `BorrowDecode`
//! - Byte-level equality between derived and manual encoding of `&str` fields
//! - `Vec<&str>` zero-copy BorrowDecode roundtrip
//! - Nested structs with BorrowDecode

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
use oxicode::{BorrowDecode, Decode, Encode};

// ── 1. Struct borrowing &str ──────────────────────────────────────────────────

/// Named struct holding a borrowed string slice and an owned integer.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct StrRef<'a> {
    s: &'a str,
    n: u32,
}

#[test]
fn test_derive_lifetime_str_ref_borrow_decode() {
    // Encode the owned version using a String / u32 pair that matches the wire layout.
    // We encode StrRef by encoding its fields in order: &str then u32.
    let original_s = "hello lifetime";
    let original_n: u32 = 42;

    // Build the owned encoding: encode &str and u32 individually to mimic field order.
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&original_s).expect("encode s"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&original_n).expect("encode n"));

    // Now borrow-decode the entire StrRef from that buffer.
    let (decoded, consumed): (StrRef<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode StrRef");

    assert_eq!(decoded.s, original_s);
    assert_eq!(decoded.n, original_n);
    assert_eq!(consumed, buf.len());
}

#[test]
fn test_derive_lifetime_str_ref_borrow_decode_unicode() {
    let s = "こんにちは 🦀";
    let n: u32 = 0xCAFE;

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&s).expect("encode s"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&n).expect("encode n"));

    let (decoded, _): (StrRef<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode StrRef unicode");

    assert_eq!(decoded.s, s);
    assert_eq!(decoded.n, n);
}

// ── 2. Struct borrowing &[u8] ─────────────────────────────────────────────────

/// Named struct holding a borrowed byte slice and an owned tag byte.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct SliceRef<'a> {
    data: &'a [u8],
    tag: u8,
}

#[test]
fn test_derive_lifetime_slice_ref_borrow_decode() {
    let original_data: &[u8] = &[10, 20, 30, 40, 50, 255];
    let original_tag: u8 = 7;

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&original_data).expect("encode data"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&original_tag).expect("encode tag"));

    let (decoded, consumed): (SliceRef<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode SliceRef");

    assert_eq!(decoded.data, original_data);
    assert_eq!(decoded.tag, original_tag);
    assert_eq!(consumed, buf.len());
}

#[test]
fn test_derive_lifetime_slice_ref_borrow_decode_empty() {
    let original_data: &[u8] = &[];
    let original_tag: u8 = 0;

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&original_data).expect("encode data"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&original_tag).expect("encode tag"));

    let (decoded, _): (SliceRef<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode SliceRef empty");

    assert_eq!(decoded.data, original_data);
    assert_eq!(decoded.tag, original_tag);
}

// ── 3. Owned struct Encode + Decode roundtrip ─────────────────────────────────

/// Owned struct with no lifetime parameter; exercises Encode + Decode derive.
#[derive(Debug, PartialEq, Encode, Decode)]
struct OwnedPair {
    label: String,
    value: u64,
}

#[test]
fn test_derive_lifetime_owned_roundtrip() {
    let original = OwnedPair {
        label: "roundtrip".to_string(),
        value: 0xDEAD_BEEF_CAFE_BABEu64,
    };

    let encoded = oxicode::encode_to_vec(&original).expect("encode OwnedPair");
    let (decoded, consumed): (OwnedPair, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode OwnedPair");

    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_derive_lifetime_owned_roundtrip_empty_string() {
    let original = OwnedPair {
        label: String::new(),
        value: 0,
    };

    let encoded = oxicode::encode_to_vec(&original).expect("encode OwnedPair empty");
    let (decoded, _): (OwnedPair, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode OwnedPair empty");

    assert_eq!(original, decoded);
}

// ── 4. Mixed owned struct with multiple field types ───────────────────────────

/// Struct mixing String and u64; no lifetime, purely owned.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Mixed {
    owned: String,
    count: u64,
}

#[test]
fn test_derive_lifetime_mixed_struct_roundtrip() {
    let original = Mixed {
        owned: "mixed owned string".to_string(),
        count: 123_456_789,
    };

    let encoded = oxicode::encode_to_vec(&original).expect("encode Mixed");
    let (decoded, consumed): (Mixed, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Mixed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_derive_lifetime_mixed_struct_zero_count() {
    let original = Mixed {
        owned: "zero".to_string(),
        count: 0,
    };

    let encoded = oxicode::encode_to_vec(&original).expect("encode Mixed zero");
    let (decoded, _): (Mixed, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Mixed zero");

    assert_eq!(original, decoded);
}

// ── 5. Tuple struct with lifetime and BorrowDecode ────────────────────────────

/// Tuple struct borrowing a `&str` and carrying a `u32` tag.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct Tagged<'a>(&'a str, u32);

#[test]
fn test_derive_lifetime_tagged_tuple_borrow_decode() {
    let label = "tagged-tuple";
    let id: u32 = 99;

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&label).expect("encode label"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&id).expect("encode id"));

    let (decoded, consumed): (Tagged<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode Tagged");

    assert_eq!(decoded.0, label);
    assert_eq!(decoded.1, id);
    assert_eq!(consumed, buf.len());
}

#[test]
fn test_derive_lifetime_tagged_tuple_empty_str() {
    let label = "";
    let id: u32 = 0;

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&label).expect("encode empty label"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&id).expect("encode zero id"));

    let (decoded, _): (Tagged<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode Tagged empty");

    assert_eq!(decoded.0, label);
    assert_eq!(decoded.1, id);
}

// ── 6. Byte-level equality: derived vs manual encode for &str field ───────────

/// Verify that the derived Encode for StrRef produces the same bytes as
/// manually encoding each field in sequence.
#[test]
fn test_derive_lifetime_str_ref_encode_matches_manual() {
    let s = "manual-vs-derived";
    let n: u32 = 7;

    // Derived encode of the whole struct.
    let derived_bytes = {
        let instance = StrRef { s, n };
        oxicode::encode_to_vec(&instance).expect("encode StrRef derived")
    };

    // Manual field-by-field encoding in the same order.
    let manual_bytes = {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&oxicode::encode_to_vec(&s).expect("encode s manual"));
        buf.extend_from_slice(&oxicode::encode_to_vec(&n).expect("encode n manual"));
        buf
    };

    assert_eq!(
        derived_bytes, manual_bytes,
        "derived Encode must produce same bytes as manual field encoding"
    );
}

// ── 7. Vec<&str> BorrowDecode roundtrip ──────────────────────────────────────

/// Verify that Vec<&str> supports zero-copy BorrowDecode: encode a Vec<String>,
/// then borrow-decode as Vec<&str>.
#[test]
fn test_derive_lifetime_vec_str_borrow_decode() {
    let owned: Vec<String> = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];

    // Encode as Vec<String> (wire format identical to Vec<&str>).
    let encoded = oxicode::encode_to_vec(&owned).expect("encode Vec<String>");

    // BorrowDecode as Vec<&str> — each &str borrows from `encoded`.
    let (decoded, consumed): (Vec<&str>, usize) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Vec<&str>");

    let expected: Vec<&str> = owned.iter().map(String::as_str).collect();
    assert_eq!(decoded, expected);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_derive_lifetime_vec_str_borrow_decode_empty() {
    let owned: Vec<String> = Vec::new();
    let encoded = oxicode::encode_to_vec(&owned).expect("encode empty Vec<String>");
    let (decoded, _): (Vec<&str>, usize) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode empty Vec<&str>");
    assert!(decoded.is_empty());
}

// ── 8. Nested struct with BorrowDecode ───────────────────────────────────────

/// Inner struct that owns data; implements Encode + Decode.
#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerOwned {
    id: u32,
    score: u64,
}

/// Outer struct that borrows a `&str` and embeds InnerOwned.
/// InnerOwned implements BorrowDecode via the blanket impl (delegates to Decode).
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct OuterBorrow<'a> {
    name: &'a str,
    inner: InnerOwned,
}

// Provide the BorrowDecode impl for InnerOwned so it can be used inside OuterBorrow.
oxicode::impl_borrow_decode!(InnerOwned);

#[test]
fn test_derive_lifetime_nested_borrow_decode() {
    let name = "nested-borrow";
    let inner = InnerOwned { id: 5, score: 1000 };

    // Encode field by field: name (&str), then inner (InnerOwned).
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&name).expect("encode name"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&inner).expect("encode inner"));

    let (decoded, consumed): (OuterBorrow<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode OuterBorrow");

    assert_eq!(decoded.name, name);
    assert_eq!(decoded.inner, inner);
    assert_eq!(consumed, buf.len());
}

#[test]
fn test_derive_lifetime_nested_borrow_decode_large_score() {
    let name = "big-score";
    let inner = InnerOwned {
        id: u32::MAX,
        score: u64::MAX,
    };

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&name).expect("encode name"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&inner).expect("encode inner"));

    let (decoded, _): (OuterBorrow<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode OuterBorrow large");

    assert_eq!(decoded.name, name);
    assert_eq!(decoded.inner.id, u32::MAX);
    assert_eq!(decoded.inner.score, u64::MAX);
}

// ── Additional: verify zero-copy pointer alignment for &str field ─────────────

#[test]
fn test_derive_lifetime_str_ref_zero_copy_ptr() {
    // The &str decoded from StrRef should point into the encoded buffer directly.
    let s = "zero-copy-check";
    let n: u32 = 1;

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&oxicode::encode_to_vec(&s).expect("encode s"));
    buf.extend_from_slice(&oxicode::encode_to_vec(&n).expect("encode n"));

    let (decoded, _): (StrRef<'_>, usize) =
        oxicode::borrow_decode_from_slice(&buf).expect("borrow_decode StrRef zero-copy");

    // The encoded &str format: 1-byte varint length prefix, then raw UTF-8 bytes.
    // So decoded.s should point to buf[1].
    let expected_ptr = unsafe { buf.as_ptr().add(1) };
    assert_eq!(
        decoded.s.as_ptr(),
        expected_ptr,
        "&str field must borrow directly from the encoded buffer (zero-copy)"
    );
}
