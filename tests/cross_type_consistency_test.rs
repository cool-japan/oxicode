//! Cross-type encoding consistency tests.
//!
//! Verifies that the same data encoded via different container types
//! produces identical or predictably different byte representations.

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
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};
use std::cell::{Cell, RefCell};
use std::cmp::Reverse;
use std::mem::ManuallyDrop;
use std::num::Wrapping;
use std::rc::Rc;
use std::sync::Arc;

// Test 1: Vec<u8> and &[u8] encode to the same bytes
#[test]
fn test_vec_u8_and_slice_u8_same_encoding() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let slice: &[u8] = &[1, 2, 3, 4, 5];

    let vec_encoded = encode_to_vec(&data).expect("encode Vec<u8>");
    let slice_encoded = encode_to_vec(&slice).expect("encode &[u8]");

    assert_eq!(vec_encoded, slice_encoded);
}

// Test 2: Box<u32> encodes the same as raw u32
#[test]
fn test_box_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let boxed: Box<u32> = Box::new(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let boxed_encoded = encode_to_vec(&boxed).expect("encode Box<u32>");

    assert_eq!(raw_encoded, boxed_encoded);
}

// Test 3: Rc<u32> encodes the same as raw u32
#[test]
fn test_rc_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let rc: Rc<u32> = Rc::new(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let rc_encoded = encode_to_vec(&rc).expect("encode Rc<u32>");

    assert_eq!(raw_encoded, rc_encoded);
}

// Test 4: Arc<u32> encodes the same as raw u32
#[test]
fn test_arc_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let arc: Arc<u32> = Arc::new(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let arc_encoded = encode_to_vec(&arc).expect("encode Arc<u32>");

    assert_eq!(raw_encoded, arc_encoded);
}

// Test 5: Cell<u32> encodes the same as raw u32
#[test]
fn test_cell_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let cell: Cell<u32> = Cell::new(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let cell_encoded = encode_to_vec(&cell).expect("encode Cell<u32>");

    assert_eq!(raw_encoded, cell_encoded);
}

// Test 6: RefCell<u32> encodes the same as raw u32
#[test]
fn test_refcell_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let refcell: RefCell<u32> = RefCell::new(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let refcell_encoded = encode_to_vec(&refcell).expect("encode RefCell<u32>");

    assert_eq!(raw_encoded, refcell_encoded);
}

// Test 7: Wrapping<u32> encodes the same as raw u32
#[test]
fn test_wrapping_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let wrapping: Wrapping<u32> = Wrapping(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let wrapping_encoded = encode_to_vec(&wrapping).expect("encode Wrapping<u32>");

    assert_eq!(raw_encoded, wrapping_encoded);
}

// Test 8: Reverse<u32> encodes the same as raw u32
#[test]
fn test_reverse_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let reverse: Reverse<u32> = Reverse(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let reverse_encoded = encode_to_vec(&reverse).expect("encode Reverse<u32>");

    assert_eq!(raw_encoded, reverse_encoded);
}

// Test 9: ManuallyDrop<u32> encodes the same as raw u32
#[test]
fn test_manually_drop_u32_same_as_raw_u32() {
    let raw: u32 = 42;
    let md: ManuallyDrop<u32> = ManuallyDrop::new(42);

    let raw_encoded = encode_to_vec(&raw).expect("encode u32");
    let md_encoded = encode_to_vec(&md).expect("encode ManuallyDrop<u32>");

    assert_eq!(raw_encoded, md_encoded);
}

// Test 10: Option<u32> None encodes as a single zero byte
#[test]
fn test_option_none_encodes_as_single_zero_byte() {
    let none: Option<u32> = None;
    let encoded = encode_to_vec(&none).expect("encode Option::None");

    assert_eq!(encoded, vec![0x00u8]);
    assert_eq!(encoded.len(), 1);
}

// Test 11: Option<u32> Some(0) encodes as [0x01, 0x00] — discriminant + value
#[test]
fn test_option_some_zero_encodes_as_discriminant_plus_value() {
    let some: Option<u32> = Some(0);
    let encoded = encode_to_vec(&some).expect("encode Option::Some(0)");

    assert_eq!(encoded, vec![0x01u8, 0x00u8]);
}

// Test 12: Result<u32, String> Ok(0) encodes as [0x00, 0x00]
// Result uses u32 varint discriminant; varint(0u32) = [0x00], then varint(0u32) = [0x00]
#[test]
fn test_result_ok_zero_encodes_as_two_zero_bytes() {
    let ok: Result<u32, String> = Ok(0);
    let encoded = encode_to_vec(&ok).expect("encode Result::Ok(0)");

    assert_eq!(encoded, vec![0x00u8, 0x00u8]);
}

// Test 13: Result<u32, String> Err("") encodes as [0x01, 0x00]
// varint(1u32) = [0x01] for Err discriminant, empty String length varint(0u64) = [0x00]
#[test]
fn test_result_err_empty_string_encodes_as_err_discriminant_plus_zero() {
    let err: Result<u32, String> = Err(String::new());
    let encoded = encode_to_vec(&err).expect("encode Result::Err(\"\")");

    assert_eq!(encoded, vec![0x01u8, 0x00u8]);
}

// Test 14: Box<Vec<u8>> adds no overhead compared to Vec<u8>
#[test]
fn test_box_vec_u8_no_overhead_vs_vec_u8() {
    let data: Vec<u8> = vec![10, 20, 30];
    let boxed: Box<Vec<u8>> = Box::new(vec![10, 20, 30]);

    let vec_encoded = encode_to_vec(&data).expect("encode Vec<u8>");
    let boxed_encoded = encode_to_vec(&boxed).expect("encode Box<Vec<u8>>");

    assert_eq!(vec_encoded, boxed_encoded);
}

// Test 15: u32 standard (varint) vs fixed-int config — different encoded sizes
#[test]
fn test_u32_varint_vs_fixedint_different_sizes() {
    let value: u32 = 1;

    let varint_config = config::standard().with_variable_int_encoding();
    let fixint_config = config::standard().with_fixed_int_encoding();

    let varint_encoded = encode_to_vec_with_config(&value, varint_config).expect("encode varint");
    let fixint_encoded = encode_to_vec_with_config(&value, fixint_config).expect("encode fixint");

    // varint(1u32) = 1 byte, fixint u32 = 4 bytes
    assert_eq!(varint_encoded.len(), 1);
    assert_eq!(fixint_encoded.len(), 4);
    assert!(varint_encoded.len() < fixint_encoded.len());
}

// Test 16: Little-endian vs big-endian u32 — same size, different byte order
#[test]
fn test_little_endian_vs_big_endian_same_size_different_bytes() {
    let value: u32 = 0x01020304;

    let le_config = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let be_config = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let le_encoded = encode_to_vec_with_config(&value, le_config).expect("encode little-endian");
    let be_encoded = encode_to_vec_with_config(&value, be_config).expect("encode big-endian");

    assert_eq!(le_encoded.len(), be_encoded.len());
    assert_ne!(le_encoded, be_encoded);
    assert_eq!(le_encoded, vec![0x04u8, 0x03, 0x02, 0x01]);
    assert_eq!(be_encoded, vec![0x01u8, 0x02, 0x03, 0x04]);
}

// Test 17: Empty Vec<u32> and empty Vec<String> both encode as a single varint(0) byte
#[test]
fn test_empty_vecs_of_different_types_encode_as_single_byte() {
    let empty_u32: Vec<u32> = vec![];
    let empty_str: Vec<String> = vec![];

    let encoded_u32 = encode_to_vec(&empty_u32).expect("encode empty Vec<u32>");
    let encoded_str = encode_to_vec(&empty_str).expect("encode empty Vec<String>");

    assert_eq!(encoded_u32.len(), 1);
    assert_eq!(encoded_str.len(), 1);
    assert_eq!(encoded_u32, vec![0x00u8]);
    assert_eq!(encoded_str, vec![0x00u8]);
}

// Test 18: String and Vec<u8> with same ASCII content encode to the same bytes
#[test]
fn test_string_and_vec_u8_same_ascii_content_same_encoding() {
    let s = String::from("hello");
    let v: Vec<u8> = b"hello".to_vec();

    let string_encoded = encode_to_vec(&s).expect("encode String");
    let vec_encoded = encode_to_vec(&v).expect("encode Vec<u8>");

    assert_eq!(string_encoded, vec_encoded);
}

// Test 19: [u8; 5] fixed array has no length prefix; Vec<u8> with same 5 elements has a prefix
#[test]
fn test_fixed_array_vs_vec_length_prefix_difference() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let vec: Vec<u8> = vec![1, 2, 3, 4, 5];

    let arr_encoded = encode_to_vec(&arr).expect("encode [u8; 5]");
    let vec_encoded = encode_to_vec(&vec).expect("encode Vec<u8>");

    // Fixed array: 5 raw bytes (no length prefix)
    assert_eq!(arr_encoded.len(), 5);
    // Vec<u8>: 1 byte varint length prefix + 5 bytes = 6
    assert_eq!(vec_encoded.len(), 6);

    // The array bytes are the raw payload; vec bytes include length prefix byte
    assert_eq!(&arr_encoded[..], &[1u8, 2, 3, 4, 5]);
    assert_eq!(&vec_encoded[1..], &[1u8, 2, 3, 4, 5]);
}

// Test 20: bool true encodes as [0x01], bool false encodes as [0x00]
#[test]
fn test_bool_true_and_false_single_byte_encoding() {
    let t_encoded = encode_to_vec(&true).expect("encode true");
    let f_encoded = encode_to_vec(&false).expect("encode false");

    assert_eq!(t_encoded, vec![0x01u8]);
    assert_eq!(f_encoded, vec![0x00u8]);
    assert_eq!(t_encoded.len(), 1);
    assert_eq!(f_encoded.len(), 1);
}

// Test 21: u8 value 1 and bool true encode identically
#[test]
fn test_u8_one_and_bool_true_same_encoding() {
    let u8_encoded = encode_to_vec(&1u8).expect("encode 1u8");
    let bool_encoded = encode_to_vec(&true).expect("encode true");

    assert_eq!(u8_encoded, bool_encoded);
    assert_eq!(u8_encoded, vec![0x01u8]);
}

// Test 22: () unit type encodes to zero bytes
#[test]
fn test_unit_type_encodes_to_zero_bytes() {
    let unit_encoded = encode_to_vec(&()).expect("encode ()");

    assert!(unit_encoded.is_empty());

    // Round-trip verify
    let ((), consumed): ((), _) = decode_from_slice(&unit_encoded).expect("decode ()");
    assert_eq!(consumed, 0);

    // Also verify with a custom config for completeness
    let config = config::standard().with_fixed_int_encoding();
    let unit_fixed = encode_to_vec_with_config(&(), config).expect("encode () fixed");
    assert!(unit_fixed.is_empty());
    let ((), consumed_fixed): ((), _) =
        decode_from_slice_with_config(&unit_fixed, config).expect("decode () fixed");
    assert_eq!(consumed_fixed, 0);
}
