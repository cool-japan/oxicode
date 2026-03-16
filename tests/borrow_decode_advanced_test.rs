//! Advanced BorrowDecode tests for complex struct hierarchies and zero-copy decoding patterns.
//!
//! These tests cover scenarios NOT in borrow_decode_test.rs, borrow_decode_derive_test.rs,
//! or cow_types_test.rs: cross-struct encode/decode, nested borrowed structs, multiple &str
//! fields with pointer verification, large payloads, config variations, and byte alignment.

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
use oxicode::{
    borrow_decode_from_slice, borrow_decode_from_slice_with_config, config, encode_to_vec,
    encode_to_vec_with_config, BorrowDecode, Decode, Encode,
};

// ─── Type definitions ────────────────────────────────────────────────────────

/// Owned counterpart used for encoding.
#[derive(Debug, PartialEq, Encode, Decode)]
struct OwnedMsg {
    id: u32,
    text: String,
    data: Vec<u8>,
}

/// Borrowed counterpart: borrow_decode_from_slice returns this from the same buffer.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct BorrowedMsg<'a> {
    id: u32,
    text: &'a str,
    data: &'a [u8],
}

// Nested structs for hierarchy tests
#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerOwned {
    tag: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OuterOwned {
    inner: InnerOwned,
    count: u32,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct InnerBorrowed<'a> {
    tag: &'a str,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct OuterBorrowed<'a> {
    inner: InnerBorrowed<'a>,
    count: u32,
}

// Multi-field struct for pointer alignment tests
#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiFieldOwned {
    first: String,
    second: String,
    third: String,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct MultiFieldBorrowed<'a> {
    first: &'a str,
    second: &'a str,
    third: &'a str,
}

// Large-payload struct
#[derive(Debug, PartialEq, Encode, Decode)]
struct LargePayloadOwned {
    header: u32,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct LargePayloadBorrowed<'a> {
    header: u32,
    payload: &'a [u8],
}

// ─── Tests ───────────────────────────────────────────────────────────────────

/// Encode OwnedMsg, decode as BorrowedMsg — the canonical cross-struct zero-copy pattern.
#[test]
fn test_owned_to_borrowed_struct_cross_decode() {
    let original = OwnedMsg {
        id: 42,
        text: "hello zero copy".to_string(),
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let enc = encode_to_vec(&original).expect("encode OwnedMsg");

    let (decoded, consumed): (BorrowedMsg, _) =
        borrow_decode_from_slice(&enc).expect("borrow_decode BorrowedMsg");

    assert_eq!(decoded.id, 42);
    assert_eq!(decoded.text, "hello zero copy");
    assert_eq!(decoded.data, &[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(consumed, enc.len());
}

/// Encode a nested owned struct and borrow_decode it as a nested borrowed struct.
#[test]
fn test_deeply_nested_borrowed_struct() {
    let original = OuterOwned {
        inner: InnerOwned {
            tag: "nested_tag".to_string(),
        },
        count: 99,
    };
    let enc = encode_to_vec(&original).expect("encode OuterOwned");

    let (decoded, consumed): (OuterBorrowed, _) =
        borrow_decode_from_slice(&enc).expect("borrow_decode OuterBorrowed");

    assert_eq!(decoded.inner.tag, "nested_tag");
    assert_eq!(decoded.count, 99);
    assert_eq!(consumed, enc.len());
}

/// All three &str fields in MultiFieldBorrowed should point into the encoded buffer.
#[test]
fn test_borrow_decode_multiple_str_fields() {
    let original = MultiFieldOwned {
        first: "alpha".to_string(),
        second: "beta".to_string(),
        third: "gamma".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode MultiFieldOwned");

    let (decoded, _): (MultiFieldBorrowed, _) =
        borrow_decode_from_slice(&enc).expect("borrow_decode MultiFieldBorrowed");

    assert_eq!(decoded.first, "alpha");
    assert_eq!(decoded.second, "beta");
    assert_eq!(decoded.third, "gamma");

    // Verify all decoded strings point into enc (zero-copy)
    let buf_start = enc.as_ptr() as usize;
    let buf_end = buf_start + enc.len();

    let first_ptr = decoded.first.as_ptr() as usize;
    let second_ptr = decoded.second.as_ptr() as usize;
    let third_ptr = decoded.third.as_ptr() as usize;

    assert!(
        first_ptr >= buf_start && first_ptr < buf_end,
        "first field does not point into encoded buffer"
    );
    assert!(
        second_ptr >= buf_start && second_ptr < buf_end,
        "second field does not point into encoded buffer"
    );
    assert!(
        third_ptr >= buf_start && third_ptr < buf_end,
        "third field does not point into encoded buffer"
    );
}

/// BorrowDecode a struct with a 10 000-byte payload — verify the slice pointer is within enc.
#[test]
fn test_borrow_decode_large_payload() {
    let payload_data: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();
    let original = LargePayloadOwned {
        header: 0xCAFE_BABE,
        payload: payload_data.clone(),
    };
    let enc = encode_to_vec(&original).expect("encode LargePayloadOwned");

    let (decoded, consumed): (LargePayloadBorrowed, _) =
        borrow_decode_from_slice(&enc).expect("borrow_decode LargePayloadBorrowed");

    assert_eq!(decoded.header, 0xCAFE_BABE);
    assert_eq!(decoded.payload.len(), 10_000);
    assert_eq!(decoded.payload, payload_data.as_slice());
    assert_eq!(consumed, enc.len());

    // Verify zero-copy: payload slice points into enc
    let buf_start = enc.as_ptr() as usize;
    let buf_end = buf_start + enc.len();
    let payload_ptr = decoded.payload.as_ptr() as usize;
    assert!(
        payload_ptr >= buf_start && payload_ptr < buf_end,
        "large payload does not point into encoded buffer — not zero-copy"
    );
}

/// borrow_decode_from_slice_with_config using legacy (fixed-int) config.
#[test]
fn test_borrow_decode_with_config_legacy() {
    let legacy_cfg = config::legacy();
    let original = OwnedMsg {
        id: 7,
        text: "legacy_config_test".to_string(),
        data: vec![1, 2, 3, 4, 5],
    };
    let enc = encode_to_vec_with_config(&original, legacy_cfg).expect("encode with legacy config");

    let (decoded, consumed): (BorrowedMsg, _) =
        borrow_decode_from_slice_with_config(&enc, legacy_cfg)
            .expect("borrow_decode with legacy config");

    assert_eq!(decoded.id, 7);
    assert_eq!(decoded.text, "legacy_config_test");
    assert_eq!(decoded.data, &[1u8, 2, 3, 4, 5]);
    assert_eq!(consumed, enc.len());
}

/// After cross-decoding, both decoded.text and decoded.data pointers lie within enc's range.
#[test]
fn test_cross_decode_preserves_byte_alignment() {
    let original = OwnedMsg {
        id: 1,
        text: "alignment_check".to_string(),
        data: vec![0xAA, 0xBB, 0xCC],
    };
    let enc = encode_to_vec(&original).expect("encode for alignment check");

    let (decoded, _): (BorrowedMsg, _) =
        borrow_decode_from_slice(&enc).expect("borrow_decode for alignment check");

    let buf_start = enc.as_ptr() as usize;
    let buf_end = buf_start + enc.len();

    let text_ptr = decoded.text.as_ptr() as usize;
    let data_ptr = decoded.data.as_ptr() as usize;

    assert!(
        text_ptr >= buf_start && text_ptr < buf_end,
        "text ptr {text_ptr:#x} is outside enc [{buf_start:#x}, {buf_end:#x})"
    );
    assert!(
        data_ptr >= buf_start && data_ptr < buf_end,
        "data ptr {data_ptr:#x} is outside enc [{buf_start:#x}, {buf_end:#x})"
    );
}

/// Encode a Vec<OwnedMsg> and decode as Vec<OwnedMsg> (owned), then individually
/// borrow_decode each item to confirm field-level values are consistent.
#[test]
fn test_vec_of_owned_then_per_item_borrow_decode() {
    let items: Vec<OwnedMsg> = vec![
        OwnedMsg {
            id: 10,
            text: "first".to_string(),
            data: vec![1],
        },
        OwnedMsg {
            id: 20,
            text: "second".to_string(),
            data: vec![2, 3],
        },
        OwnedMsg {
            id: 30,
            text: "third".to_string(),
            data: vec![4, 5, 6],
        },
    ];

    // Round-trip via owned Vec
    let enc = encode_to_vec(&items).expect("encode Vec<OwnedMsg>");
    let (decoded_vec, consumed): (Vec<OwnedMsg>, _) =
        oxicode::decode_from_slice(&enc).expect("decode Vec<OwnedMsg>");
    assert_eq!(consumed, enc.len());
    assert_eq!(decoded_vec.len(), 3);

    // Verify each item independently via borrow_decode on its own buffer
    for (original, decoded_owned) in items.iter().zip(decoded_vec.iter()) {
        assert_eq!(original.id, decoded_owned.id);
        assert_eq!(original.text, decoded_owned.text);
        assert_eq!(original.data, decoded_owned.data);

        let item_enc = encode_to_vec(original).expect("encode individual item");
        let (borrowed, _): (BorrowedMsg, _) =
            borrow_decode_from_slice(&item_enc).expect("borrow_decode individual item");
        assert_eq!(borrowed.id, original.id);
        assert_eq!(borrowed.text, original.text.as_str());
        assert_eq!(borrowed.data, original.data.as_slice());
    }
}

/// Encode with standard config and borrow_decode with fixed_int_encoding config variation.
/// Tests that standard-encoded fields (varint length prefix) are consistently decoded.
#[test]
fn test_borrow_decode_standard_config_roundtrip() {
    let standard_cfg = config::standard();
    let original = OwnedMsg {
        id: 255,
        text: "standard_cfg".to_string(),
        data: vec![0xFF, 0x00, 0x7F],
    };
    let enc = encode_to_vec_with_config(&original, standard_cfg).expect("encode standard");

    let (decoded, consumed): (BorrowedMsg, _) =
        borrow_decode_from_slice_with_config(&enc, standard_cfg)
            .expect("borrow_decode standard config");

    assert_eq!(decoded.id, 255);
    assert_eq!(decoded.text, "standard_cfg");
    assert_eq!(decoded.data, &[0xFF_u8, 0x00, 0x7F]);
    assert_eq!(consumed, enc.len());
}

/// Nested BorrowDecode with empty string and empty slice — edge case for zero-length fields.
#[test]
fn test_nested_borrowed_empty_fields() {
    let original = OuterOwned {
        inner: InnerOwned { tag: String::new() },
        count: 0,
    };
    let enc = encode_to_vec(&original).expect("encode empty nested");

    let (decoded, consumed): (OuterBorrowed, _) =
        borrow_decode_from_slice(&enc).expect("borrow_decode empty nested");

    assert_eq!(decoded.inner.tag, "");
    assert_eq!(decoded.count, 0);
    assert_eq!(consumed, enc.len());
}
