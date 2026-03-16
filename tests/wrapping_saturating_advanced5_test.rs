//! Advanced roundtrip tests for Wrapping<T>/Saturating<T>/Cell<T>/RefCell<T>
//! transparent encoding in OxiCode.
//! All four wrappers encode transparently — same wire bytes as their inner T.

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
use std::num::{Saturating, Wrapping};

// ===== Test 1: Wrapping<u32> roundtrip value=42 =====

#[test]
fn test_wrapping_u32_value_42_roundtrip() {
    let val = Wrapping(42u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u32>(42)");
    let (decoded, _): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u32>(42)");
    assert_eq!(val, decoded);
}

// ===== Test 2: Wrapping<u32> overflow wrap (u32::MAX) =====

#[test]
fn test_wrapping_u32_max_overflow_roundtrip() {
    let val = Wrapping(u32::MAX);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u32>(u32::MAX)");
    let (decoded, _): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u32>(u32::MAX)");
    assert_eq!(val, decoded);
    // Confirm the inner value is preserved exactly, not clamped/wrapped further
    assert_eq!(decoded.0, u32::MAX);
}

// ===== Test 3: Wrapping<i32> with negative =====

#[test]
fn test_wrapping_i32_negative_roundtrip() {
    let val = Wrapping(-42_i32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<i32>(-42)");
    let (decoded, _): (Wrapping<i32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<i32>(-42)");
    assert_eq!(val, decoded);
    assert_eq!(decoded.0, -42_i32);
}

// ===== Test 4: Wrapping<u64> roundtrip =====

#[test]
fn test_wrapping_u64_roundtrip() {
    let val = Wrapping(0x_DEAD_BEEF_CAFE_BABEu64);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u64>");
    let (decoded, _): (Wrapping<u64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u64>");
    assert_eq!(val, decoded);
}

// ===== Test 5: Wrapping<u32> same wire bytes as raw u32 =====

#[test]
fn test_wrapping_u32_same_wire_bytes_as_raw_u32() {
    let inner = 42u32;
    let raw_bytes = encode_to_vec(&inner).expect("Failed to encode raw u32");
    let wrapped_bytes = encode_to_vec(&Wrapping(inner)).expect("Failed to encode Wrapping<u32>");
    assert_eq!(
        raw_bytes, wrapped_bytes,
        "Wrapping<u32> must produce identical bytes to raw u32"
    );
}

// ===== Test 6: Saturating<u32> roundtrip value=42 =====

#[test]
fn test_saturating_u32_value_42_roundtrip() {
    let val = Saturating(42u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<u32>(42)");
    let (decoded, _): (Saturating<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<u32>(42)");
    assert_eq!(val, decoded);
}

// ===== Test 7: Saturating<u32> roundtrip max value =====

#[test]
fn test_saturating_u32_max_roundtrip() {
    let val = Saturating(u32::MAX);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<u32>(u32::MAX)");
    let (decoded, _): (Saturating<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<u32>(u32::MAX)");
    assert_eq!(val, decoded);
    assert_eq!(decoded.0, u32::MAX);
}

// ===== Test 8: Saturating<i32> with negative =====

#[test]
fn test_saturating_i32_negative_roundtrip() {
    let val = Saturating(-999_i32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<i32>(-999)");
    let (decoded, _): (Saturating<i32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<i32>(-999)");
    assert_eq!(val, decoded);
    assert_eq!(decoded.0, -999_i32);
}

// ===== Test 9: Saturating<u64> roundtrip =====

#[test]
fn test_saturating_u64_roundtrip() {
    let val = Saturating(u64::MAX / 3);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<u64>");
    let (decoded, _): (Saturating<u64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<u64>");
    assert_eq!(val, decoded);
}

// ===== Test 10: Saturating<u32> same wire bytes as raw u32 =====

#[test]
fn test_saturating_u32_same_wire_bytes_as_raw_u32() {
    let inner = 42u32;
    let raw_bytes = encode_to_vec(&inner).expect("Failed to encode raw u32");
    let saturating_bytes =
        encode_to_vec(&Saturating(inner)).expect("Failed to encode Saturating<u32>");
    assert_eq!(
        raw_bytes, saturating_bytes,
        "Saturating<u32> must produce identical bytes to raw u32"
    );
}

// ===== Test 11: Cell<u32> roundtrip =====

#[test]
fn test_cell_u32_roundtrip() {
    let val = Cell::new(12345u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Cell<u32>");
    let (decoded, _): (Cell<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Cell<u32>");
    assert_eq!(val.get(), decoded.get());
}

// ===== Test 12: Cell<bool> roundtrip =====

#[test]
fn test_cell_bool_roundtrip() {
    for flag in [true, false] {
        let val = Cell::new(flag);
        let encoded = encode_to_vec(&val).expect("Failed to encode Cell<bool>");
        let (decoded, _): (Cell<bool>, usize) =
            decode_from_slice(&encoded).expect("Failed to decode Cell<bool>");
        assert_eq!(
            val.get(),
            decoded.get(),
            "Cell<bool> roundtrip failed for value={flag}"
        );
    }
}

// ===== Test 13: Cell<u64> roundtrip =====

#[test]
fn test_cell_u64_roundtrip() {
    let val = Cell::new(u64::MAX / 7);
    let encoded = encode_to_vec(&val).expect("Failed to encode Cell<u64>");
    let (decoded, _): (Cell<u64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Cell<u64>");
    assert_eq!(val.get(), decoded.get());
}

// ===== Test 14: Cell<u32> same wire bytes as raw u32 =====

#[test]
fn test_cell_u32_same_wire_bytes_as_raw_u32() {
    let inner = 77u32;
    let raw_bytes = encode_to_vec(&inner).expect("Failed to encode raw u32");
    let cell_bytes = encode_to_vec(&Cell::new(inner)).expect("Failed to encode Cell<u32>");
    assert_eq!(
        raw_bytes, cell_bytes,
        "Cell<u32> must produce identical bytes to raw u32"
    );
}

// ===== Test 15: RefCell<u32> roundtrip =====

#[test]
fn test_refcell_u32_roundtrip() {
    let val = RefCell::new(99999u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode RefCell<u32>");
    let (decoded, _): (RefCell<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode RefCell<u32>");
    assert_eq!(*val.borrow(), *decoded.borrow());
}

// ===== Test 16: RefCell<String> roundtrip =====

#[test]
fn test_refcell_string_roundtrip() {
    let val = RefCell::new("transparent encoding rocks".to_string());
    let encoded = encode_to_vec(&val).expect("Failed to encode RefCell<String>");
    let (decoded, _): (RefCell<String>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode RefCell<String>");
    assert_eq!(*val.borrow(), *decoded.borrow());
}

// ===== Test 17: RefCell<Vec<u8>> roundtrip =====

#[test]
fn test_refcell_vec_u8_roundtrip() {
    let val = RefCell::new(vec![0u8, 1, 2, 3, 127, 128, 254, 255]);
    let encoded = encode_to_vec(&val).expect("Failed to encode RefCell<Vec<u8>>");
    let (decoded, _): (RefCell<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode RefCell<Vec<u8>>");
    assert_eq!(*val.borrow(), *decoded.borrow());
}

// ===== Test 18: RefCell<u32> same wire bytes as raw u32 =====

#[test]
fn test_refcell_u32_same_wire_bytes_as_raw_u32() {
    let inner = 77u32;
    let raw_bytes = encode_to_vec(&inner).expect("Failed to encode raw u32");
    let refcell_bytes = encode_to_vec(&RefCell::new(inner)).expect("Failed to encode RefCell<u32>");
    assert_eq!(
        raw_bytes, refcell_bytes,
        "RefCell<u32> must produce identical bytes to raw u32"
    );
}

// ===== Test 19: Vec<Wrapping<u32>> roundtrip =====

#[test]
fn test_vec_wrapping_u32_roundtrip() {
    let val: Vec<Wrapping<u32>> = vec![
        Wrapping(0u32),
        Wrapping(1u32),
        Wrapping(42u32),
        Wrapping(u32::MAX - 1),
        Wrapping(u32::MAX),
    ];
    let encoded = encode_to_vec(&val).expect("Failed to encode Vec<Wrapping<u32>>");
    let (decoded, _): (Vec<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Wrapping<u32>>");
    assert_eq!(val, decoded);
}

// ===== Test 20: Vec<Saturating<u32>> roundtrip =====

#[test]
fn test_vec_saturating_u32_roundtrip() {
    let val: Vec<Saturating<u32>> = vec![
        Saturating(0u32),
        Saturating(1u32),
        Saturating(42u32),
        Saturating(u32::MAX - 1),
        Saturating(u32::MAX),
    ];
    let encoded = encode_to_vec(&val).expect("Failed to encode Vec<Saturating<u32>>");
    let (decoded, _): (Vec<Saturating<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Saturating<u32>>");
    assert_eq!(val, decoded);
}

// ===== Test 21: Wrapping<u32> with fixed-int config (4 bytes) =====

#[test]
fn test_wrapping_u32_fixed_int_config_four_bytes() {
    let val = Wrapping(9999u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&val, cfg)
        .expect("Failed to encode Wrapping<u32> with fixed-int config");
    // With fixed-int encoding, u32 always occupies exactly 4 bytes
    assert_eq!(
        encoded.len(),
        4,
        "Wrapping<u32> with fixed-int config must encode to exactly 4 bytes"
    );
    let (decoded, _): (Wrapping<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Wrapping<u32> with fixed-int config");
    assert_eq!(val, decoded);
}

// ===== Test 22: Saturating<u32> with big-endian config =====

#[test]
fn test_saturating_u32_big_endian_config() {
    let val = Saturating(0x_0102_0304u32);
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&val, cfg)
        .expect("Failed to encode Saturating<u32> with big-endian config");
    // With big-endian fixed-int, the bytes should appear in big-endian order
    assert_eq!(
        encoded.len(),
        4,
        "Saturating<u32> with big-endian fixed-int config must encode to exactly 4 bytes"
    );
    assert_eq!(
        encoded[0], 0x01,
        "Most-significant byte must be 0x01 in big-endian encoding"
    );
    assert_eq!(
        encoded[1], 0x02,
        "Second byte must be 0x02 in big-endian encoding"
    );
    assert_eq!(
        encoded[2], 0x03,
        "Third byte must be 0x03 in big-endian encoding"
    );
    assert_eq!(
        encoded[3], 0x04,
        "Least-significant byte must be 0x04 in big-endian encoding"
    );
    let (decoded, _): (Saturating<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Saturating<u32> with big-endian config");
    assert_eq!(val, decoded);
}
