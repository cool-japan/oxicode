//! Roundtrip tests for Cell<T> and RefCell<T> encoding/decoding.

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
use oxicode::{config, decode_from_slice, encode_to_vec};
use std::cell::{Cell, RefCell};

// ===== Cell<T> tests =====

#[test]
fn test_cell_u32_roundtrip() {
    let original = Cell::new(42u32);
    let bytes = encode_to_vec(&original).expect("encode Cell<u32>");
    let (decoded, _): (Cell<u32>, _) = decode_from_slice(&bytes).expect("decode Cell<u32>");
    assert_eq!(decoded.get(), 42u32);
}

#[test]
fn test_cell_bool_roundtrip() {
    let original = Cell::new(true);
    let bytes = encode_to_vec(&original).expect("encode Cell<bool>");
    let (decoded, _): (Cell<bool>, _) = decode_from_slice(&bytes).expect("decode Cell<bool>");
    assert!(decoded.get());
}

#[test]
fn test_cell_u64_max_roundtrip() {
    let original = Cell::new(u64::MAX);
    let bytes = encode_to_vec(&original).expect("encode Cell<u64::MAX>");
    let (decoded, _): (Cell<u64>, _) = decode_from_slice(&bytes).expect("decode Cell<u64>");
    assert_eq!(decoded.get(), u64::MAX);
}

#[test]
fn test_cell_negative_i32_roundtrip() {
    let original = Cell::new(-999i32);
    let bytes = encode_to_vec(&original).expect("encode Cell<i32>");
    let (decoded, _): (Cell<i32>, _) = decode_from_slice(&bytes).expect("decode Cell<i32>");
    assert_eq!(decoded.get(), -999i32);
}

#[test]
fn test_cell_u8_max_roundtrip() {
    let original = Cell::new(255u8);
    let bytes = encode_to_vec(&original).expect("encode Cell<u8>");
    let (decoded, _): (Cell<u8>, _) = decode_from_slice(&bytes).expect("decode Cell<u8>");
    assert_eq!(decoded.get(), 255u8);
}

#[test]
fn test_cell_char_roundtrip() {
    let original = Cell::new('A');
    let bytes = encode_to_vec(&original).expect("encode Cell<char>");
    let (decoded, _): (Cell<char>, _) = decode_from_slice(&bytes).expect("decode Cell<char>");
    assert_eq!(decoded.get(), 'A');
}

#[test]
fn test_cell_f64_pi_roundtrip() {
    let original = Cell::new(std::f64::consts::PI);
    let bytes = encode_to_vec(&original).expect("encode Cell<f64>");
    let (decoded, _): (Cell<f64>, _) = decode_from_slice(&bytes).expect("decode Cell<f64>");
    assert!((decoded.get() - std::f64::consts::PI).abs() < f64::EPSILON);
}

#[test]
fn test_cell_usize_zero_roundtrip() {
    let original = Cell::new(0usize);
    let bytes = encode_to_vec(&original).expect("encode Cell<usize>");
    let (decoded, _): (Cell<usize>, _) = decode_from_slice(&bytes).expect("decode Cell<usize>");
    assert_eq!(decoded.get(), 0usize);
}

#[test]
fn test_cell_u32_same_bytes_as_plain_u32() {
    let value = 12345u32;
    let cell_bytes = encode_to_vec(&Cell::new(value)).expect("encode Cell<u32>");
    let plain_bytes = encode_to_vec(&value).expect("encode u32");
    assert_eq!(
        cell_bytes, plain_bytes,
        "Cell<u32> and u32 should produce identical bytes"
    );
}

#[test]
fn test_option_cell_u32_some_roundtrip() {
    let original: Option<Cell<u32>> = Some(Cell::new(99u32));
    let bytes = encode_to_vec(&original).expect("encode Option<Cell<u32>> Some");
    let (decoded, _): (Option<Cell<u32>>, _) =
        decode_from_slice(&bytes).expect("decode Option<Cell<u32>>");
    let inner = decoded.expect("expected Some");
    assert_eq!(inner.get(), 99u32);
}

#[test]
fn test_option_cell_u32_none_roundtrip() {
    let original: Option<Cell<u32>> = None;
    let bytes = encode_to_vec(&original).expect("encode Option<Cell<u32>> None");
    let (decoded, _): (Option<Cell<u32>>, _) =
        decode_from_slice(&bytes).expect("decode Option<Cell<u32>>");
    assert!(decoded.is_none());
}

#[test]
fn test_vec_of_cells_roundtrip() {
    let original: Vec<Cell<u32>> = vec![Cell::new(10u32), Cell::new(20u32), Cell::new(30u32)];
    let bytes = encode_to_vec(&original).expect("encode Vec<Cell<u32>>");
    let (decoded, _): (Vec<Cell<u32>>, _) =
        decode_from_slice(&bytes).expect("decode Vec<Cell<u32>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].get(), 10u32);
    assert_eq!(decoded[1].get(), 20u32);
    assert_eq!(decoded[2].get(), 30u32);
}

#[test]
fn test_cell_u32_fixed_int_encoding() {
    let original = Cell::new(1u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode Cell<u32> fixed-int");
    // Fixed-int encoding: u32 always 4 bytes
    assert_eq!(bytes.len(), 4, "fixed-int Cell<u32> must be 4 bytes");
    let (decoded, _): (Cell<u32>, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode Cell<u32> fixed-int");
    assert_eq!(decoded.get(), 1u32);
}

#[test]
fn test_cell_u32_big_endian_config() {
    let original = Cell::new(0x0102_0304u32);
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode Cell<u32> big-endian");
    // Big-endian wire: 0x01 0x02 0x03 0x04
    assert_eq!(bytes, [0x01, 0x02, 0x03, 0x04]);
    let (decoded, _): (Cell<u32>, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode Cell<u32> big-endian");
    assert_eq!(decoded.get(), 0x0102_0304u32);
}

// ===== RefCell<T> tests =====

#[test]
fn test_refcell_string_roundtrip() {
    let original = RefCell::new("hello".to_string());
    let bytes = encode_to_vec(&original).expect("encode RefCell<String>");
    let (decoded, _): (RefCell<String>, _) =
        decode_from_slice(&bytes).expect("decode RefCell<String>");
    assert_eq!(*decoded.borrow(), "hello");
}

#[test]
fn test_refcell_vec_u8_roundtrip() {
    let original = RefCell::new(vec![1u8, 2, 3]);
    let bytes = encode_to_vec(&original).expect("encode RefCell<Vec<u8>>");
    let (decoded, _): (RefCell<Vec<u8>>, _) =
        decode_from_slice(&bytes).expect("decode RefCell<Vec<u8>>");
    assert_eq!(*decoded.borrow(), vec![1u8, 2, 3]);
}

#[test]
fn test_refcell_u32_roundtrip() {
    let original = RefCell::new(42u32);
    let bytes = encode_to_vec(&original).expect("encode RefCell<u32>");
    let (decoded, _): (RefCell<u32>, _) = decode_from_slice(&bytes).expect("decode RefCell<u32>");
    assert_eq!(*decoded.borrow(), 42u32);
}

#[test]
fn test_refcell_option_string_roundtrip() {
    let original = RefCell::new(Some("text".to_string()));
    let bytes = encode_to_vec(&original).expect("encode RefCell<Option<String>>");
    let (decoded, _): (RefCell<Option<String>>, _) =
        decode_from_slice(&bytes).expect("decode RefCell<Option<String>>");
    assert_eq!(*decoded.borrow(), Some("text".to_string()));
}

#[test]
fn test_refcell_vec_strings_roundtrip() {
    let original = RefCell::new(vec!["a".to_string(), "b".to_string()]);
    let bytes = encode_to_vec(&original).expect("encode RefCell<Vec<String>>");
    let (decoded, _): (RefCell<Vec<String>>, _) =
        decode_from_slice(&bytes).expect("decode RefCell<Vec<String>>");
    let borrowed = decoded.borrow();
    assert_eq!(borrowed.len(), 2);
    assert_eq!(borrowed[0], "a");
    assert_eq!(borrowed[1], "b");
}

#[test]
fn test_cell_refcell_tuple_roundtrip() {
    let original = (Cell::new(100u32), RefCell::new("world".to_string()));
    let bytes = encode_to_vec(&original).expect("encode (Cell<u32>, RefCell<String>)");
    let (decoded, _): ((Cell<u32>, RefCell<String>), _) =
        decode_from_slice(&bytes).expect("decode (Cell<u32>, RefCell<String>)");
    assert_eq!(decoded.0.get(), 100u32);
    assert_eq!(*decoded.1.borrow(), "world");
}

#[test]
fn test_refcell_tuple_inner_roundtrip() {
    let original = RefCell::new((42u32, 99u64, false));
    let bytes = encode_to_vec(&original).expect("encode RefCell<(u32, u64, bool)>");
    let (decoded, _): (RefCell<(u32, u64, bool)>, _) =
        decode_from_slice(&bytes).expect("decode RefCell<(u32, u64, bool)>");
    let inner = *decoded.borrow();
    assert_eq!(inner.0, 42u32);
    assert_eq!(inner.1, 99u64);
    assert!(!inner.2);
}

#[test]
fn test_cell_u32_with_limit_config() {
    let original = Cell::new(7u32);
    let cfg = config::standard().with_limit::<1000>();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode Cell<u32> with limit");
    let (decoded, _): (Cell<u32>, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode Cell<u32> with limit");
    assert_eq!(decoded.get(), 7u32);
}

#[test]
fn test_refcell_vec_u64_roundtrip() {
    let original = RefCell::new(vec![10u64, 20, 30, 40, 50]);
    let bytes = encode_to_vec(&original).expect("encode RefCell<Vec<u64>>");
    let (decoded, _): (RefCell<Vec<u64>>, _) =
        decode_from_slice(&bytes).expect("decode RefCell<Vec<u64>>");
    let borrowed = decoded.borrow();
    assert_eq!(borrowed.len(), 5);
    assert_eq!(*borrowed, vec![10u64, 20, 30, 40, 50]);
}
