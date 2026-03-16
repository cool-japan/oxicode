//! Tests for #[derive(BorrowDecode)]

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
use oxicode::{BorrowDecode, Encode};

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct ZeroCopyBytes<'a> {
    data: &'a [u8],
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct ZeroCopyStr<'a> {
    name: &'a str,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct OwnedFields {
    id: u32,
    name: String,
    count: u64,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct Mixed<'a> {
    raw: &'a [u8],
    label: String,
    value: u32,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
enum Commands<'a> {
    Ping,
    Data(&'a [u8]),
    Named { key: &'a str, value: u32 },
}

#[test]
fn test_borrow_decode_bytes_zero_copy() {
    let data = b"hello zero copy";
    let original = ZeroCopyBytes { data };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _) =
        oxicode::borrow_decode_from_slice::<ZeroCopyBytes>(&encoded).expect("borrow_decode failed");
    assert_eq!(original, decoded);
    // Verify the pointer points into the encoded buffer (zero-copy)
    assert_eq!(
        decoded.data.as_ptr(),
        encoded[encoded.len() - data.len()..].as_ptr()
    );
}

#[test]
fn test_borrow_decode_str_zero_copy() {
    let original = ZeroCopyStr {
        name: "zero copy string",
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _) =
        oxicode::borrow_decode_from_slice::<ZeroCopyStr>(&encoded).expect("borrow_decode failed");
    assert_eq!(original.name, decoded.name);
}

#[test]
fn test_borrow_decode_owned_fields() {
    let original = OwnedFields {
        id: 42,
        name: "owned string".to_string(),
        count: 100,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _) =
        oxicode::borrow_decode_from_slice::<OwnedFields>(&encoded).expect("borrow_decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_mixed() {
    let bytes = b"raw bytes here";
    let original = Mixed {
        raw: bytes,
        label: "mixed".to_string(),
        value: 42,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _) =
        oxicode::borrow_decode_from_slice::<Mixed>(&encoded).expect("borrow_decode failed");
    assert_eq!(original.raw, decoded.raw);
    assert_eq!(original.label, decoded.label);
    assert_eq!(original.value, decoded.value);
}

#[test]
fn test_borrow_decode_enum() {
    // Ping variant
    let cmd = Commands::Ping;
    let encoded = oxicode::encode_to_vec(&cmd).expect("encode failed");
    let (decoded, _) =
        oxicode::borrow_decode_from_slice::<Commands>(&encoded).expect("borrow_decode Ping failed");
    assert_eq!(cmd, decoded);

    // Data variant with zero-copy bytes
    let data_bytes = b"payload";
    let cmd = Commands::Data(data_bytes);
    let encoded = oxicode::encode_to_vec(&cmd).expect("encode failed");
    let (decoded, _) =
        oxicode::borrow_decode_from_slice::<Commands>(&encoded).expect("borrow_decode Data failed");
    if let Commands::Data(d) = decoded {
        assert_eq!(d, data_bytes);
    } else {
        panic!("expected Commands::Data");
    }
}
