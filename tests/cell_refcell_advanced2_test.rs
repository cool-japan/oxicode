//! Advanced roundtrip tests for Cell<T> and RefCell<T> encoding in OxiCode.
//!
//! Cell<T> and RefCell<T> are transparent wrappers — their wire format is
//! identical to the inner value T.

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

// ===== 1. Cell<u32> roundtrip value=42 =====

#[test]
fn test_cell_u32_roundtrip_42() {
    let val = Cell::new(42u32);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<u32>(42)");
    let (decoded, _): (u32, usize) = decode_from_slice(&enc).expect("decode Cell<u32>(42)");
    assert_eq!(val.get(), decoded);
}

// ===== 2. Cell<u32> roundtrip value=0 =====

#[test]
fn test_cell_u32_roundtrip_zero() {
    let val = Cell::new(0u32);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<u32>(0)");
    let (decoded, _): (u32, usize) = decode_from_slice(&enc).expect("decode Cell<u32>(0)");
    assert_eq!(val.get(), decoded);
}

// ===== 3. Cell<u32> same wire bytes as raw u32 =====

#[test]
fn test_cell_u32_same_wire_bytes_as_raw_u32() {
    let value = 99_999u32;
    let val = Cell::new(value);
    let cell_enc = encode_to_vec(&val.get()).expect("encode Cell<u32> inner");
    let raw_enc = encode_to_vec(&value).expect("encode raw u32");
    assert_eq!(
        cell_enc, raw_enc,
        "Cell<u32> inner must encode identically to raw u32"
    );
}

// ===== 4. Cell<bool> roundtrip true =====

#[test]
fn test_cell_bool_roundtrip_true() {
    let val = Cell::new(true);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<bool>(true)");
    let (decoded, _): (bool, usize) = decode_from_slice(&enc).expect("decode Cell<bool>(true)");
    assert_eq!(val.get(), decoded);
}

// ===== 5. Cell<bool> roundtrip false =====

#[test]
fn test_cell_bool_roundtrip_false() {
    let val = Cell::new(false);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<bool>(false)");
    let (decoded, _): (bool, usize) = decode_from_slice(&enc).expect("decode Cell<bool>(false)");
    assert_eq!(val.get(), decoded);
}

// ===== 6. Cell<u64> consumed bytes equals encoded len =====

#[test]
fn test_cell_u64_consumed_bytes_equals_encoded_len() {
    let val = Cell::new(123_456_789_u64);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<u64>");
    let (_, consumed): (u64, usize) = decode_from_slice(&enc).expect("decode Cell<u64>");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the total encoded length"
    );
}

// ===== 7. Cell<u8> all values 0-255 via a sample roundtrip =====

#[test]
fn test_cell_u8_all_values_sample_roundtrip() {
    for byte_val in 0u8..=255u8 {
        let val = Cell::new(byte_val);
        let enc = encode_to_vec(&val.get()).expect("encode Cell<u8>");
        let (decoded, _): (u8, usize) = decode_from_slice(&enc).expect("decode Cell<u8>");
        assert_eq!(
            val.get(),
            decoded,
            "Cell<u8>({}) roundtrip failed",
            byte_val
        );
    }
}

// ===== 8. RefCell<u32> roundtrip =====

#[test]
fn test_refcell_u32_roundtrip() {
    let val = RefCell::new(42u32);
    let enc = encode_to_vec(&*val.borrow()).expect("encode RefCell<u32>");
    let (decoded, _): (u32, usize) = decode_from_slice(&enc).expect("decode RefCell<u32>");
    assert_eq!(*val.borrow(), decoded);
}

// ===== 9. RefCell<String> roundtrip =====

#[test]
fn test_refcell_string_roundtrip() {
    let val = RefCell::new(String::from("oxicode refcell string test"));
    let enc = encode_to_vec(&*val.borrow()).expect("encode RefCell<String>");
    let (decoded, _): (String, usize) = decode_from_slice(&enc).expect("decode RefCell<String>");
    assert_eq!(*val.borrow(), decoded);
}

// ===== 10. RefCell<Vec<u8>> roundtrip =====

#[test]
fn test_refcell_vec_u8_roundtrip() {
    let val = RefCell::new(vec![0xca_u8, 0xfe, 0xba, 0xbe]);
    let enc = encode_to_vec(&*val.borrow()).expect("encode RefCell<Vec<u8>>");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode RefCell<Vec<u8>>");
    assert_eq!(*val.borrow(), decoded);
}

// ===== 11. RefCell<u32> same wire bytes as raw u32 =====

#[test]
fn test_refcell_u32_same_wire_bytes_as_raw_u32() {
    let value = 55_555u32;
    let val = RefCell::new(value);
    let refcell_enc = encode_to_vec(&*val.borrow()).expect("encode RefCell<u32> inner");
    let raw_enc = encode_to_vec(&value).expect("encode raw u32");
    assert_eq!(
        refcell_enc, raw_enc,
        "RefCell<u32> inner must encode identically to raw u32"
    );
}

// ===== 12. RefCell<u64> consumed bytes equals encoded len =====

#[test]
fn test_refcell_u64_consumed_bytes_equals_encoded_len() {
    let val = RefCell::new(987_654_321_u64);
    let enc = encode_to_vec(&*val.borrow()).expect("encode RefCell<u64>");
    let (_, consumed): (u64, usize) = decode_from_slice(&enc).expect("decode RefCell<u64>");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the total encoded length"
    );
}

// ===== 13. Cell<u32> fixed int config roundtrip (4 bytes) =====

#[test]
fn test_cell_u32_fixed_int_config_roundtrip() {
    let val = Cell::new(7u32);
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val.get(), fixed_cfg).expect("encode Cell<u32> fixed-int");
    assert_eq!(
        enc.len(),
        4,
        "Cell<u32> with fixed-int encoding must produce exactly 4 bytes"
    );
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&enc, fixed_cfg).expect("decode Cell<u32> fixed-int");
    assert_eq!(val.get(), decoded);
}

// ===== 14. RefCell<u32> fixed int config (4 bytes) =====

#[test]
fn test_refcell_u32_fixed_int_config_four_bytes() {
    let val = RefCell::new(13u32);
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&*val.borrow(), fixed_cfg)
        .expect("encode RefCell<u32> fixed-int");
    assert_eq!(
        enc.len(),
        4,
        "RefCell<u32> with fixed-int encoding must produce exactly 4 bytes"
    );
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&enc, fixed_cfg).expect("decode RefCell<u32> fixed-int");
    assert_eq!(*val.borrow(), decoded);
}

// ===== 15. Cell<i32> negative value roundtrip =====

#[test]
fn test_cell_i32_negative_roundtrip() {
    let val = Cell::new(-42i32);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<i32>(-42)");
    let (decoded, _): (i32, usize) = decode_from_slice(&enc).expect("decode Cell<i32>(-42)");
    assert_eq!(val.get(), decoded);
}

// ===== 16. Cell<f64> roundtrip (bit-exact) =====

#[test]
fn test_cell_f64_roundtrip_bit_exact() {
    let pi = core::f64::consts::PI;
    let val = Cell::new(pi);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<f64>(PI)");
    let (decoded, _): (f64, usize) = decode_from_slice(&enc).expect("decode Cell<f64>(PI)");
    assert_eq!(
        val.get().to_bits(),
        decoded.to_bits(),
        "Cell<f64> PI must decode bit-exactly"
    );
}

// ===== 17. RefCell<bool> roundtrip =====

#[test]
fn test_refcell_bool_roundtrip() {
    let val = RefCell::new(true);
    let enc = encode_to_vec(&*val.borrow()).expect("encode RefCell<bool>");
    let (decoded, _): (bool, usize) = decode_from_slice(&enc).expect("decode RefCell<bool>");
    assert_eq!(*val.borrow(), decoded);
}

// ===== 18. Vec decode after Cell encode (Cell<Vec<u8>> transparent) =====

#[test]
fn test_vec_decode_after_cell_encode_transparent() {
    let inner: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
    let val = Cell::new(inner.clone());
    // Cell<Vec<u8>> — use get() which returns a clone
    let enc = encode_to_vec(&val.into_inner()).expect("encode Cell<Vec<u8>> inner");
    // Decode back as Vec<u8> — wire format must be identical
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&enc).expect("decode Vec<u8> from Cell-encoded bytes");
    assert_eq!(
        inner, decoded,
        "Vec<u8> decoded from Cell<Vec<u8>> bytes must match original"
    );
}

// ===== 19. Cell<u128> roundtrip =====

#[test]
fn test_cell_u128_roundtrip() {
    let val = Cell::new(u128::MAX / 7);
    let enc = encode_to_vec(&val.get()).expect("encode Cell<u128>");
    let (decoded, _): (u128, usize) = decode_from_slice(&enc).expect("decode Cell<u128>");
    assert_eq!(val.get(), decoded);
}

// ===== 20. RefCell<u128> roundtrip =====

#[test]
fn test_refcell_u128_roundtrip() {
    let val = RefCell::new(u128::MAX / 13);
    let enc = encode_to_vec(&*val.borrow()).expect("encode RefCell<u128>");
    let (decoded, _): (u128, usize) = decode_from_slice(&enc).expect("decode RefCell<u128>");
    assert_eq!(*val.borrow(), decoded);
}

// ===== 21. Cell<u32> and raw u32 produce identical encoding for same value =====

#[test]
fn test_cell_u32_and_raw_u32_identical_encoding() {
    let value = 0xDEAD_BEEFu32;
    let val = Cell::new(value);
    let cell_enc = encode_to_vec(&val.get()).expect("encode Cell<u32> inner");
    let raw_enc = encode_to_vec(&value).expect("encode raw u32");
    assert_eq!(
        cell_enc, raw_enc,
        "Cell<u32>.get() and raw u32 must produce identical encoding for the same value"
    );
}

// ===== 22. RefCell<u32> big-endian fixed int config roundtrip =====

#[test]
fn test_refcell_u32_big_endian_fixed_int_roundtrip() {
    let val = RefCell::new(0x0102_0304u32);
    let be_cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&*val.borrow(), be_cfg)
        .expect("encode RefCell<u32> big-endian fixed-int");
    // Big-endian wire layout for 0x01020304: [0x01, 0x02, 0x03, 0x04]
    assert_eq!(
        enc,
        [0x01, 0x02, 0x03, 0x04],
        "big-endian wire layout must be MSB-first"
    );
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&enc, be_cfg)
        .expect("decode RefCell<u32> big-endian fixed-int");
    assert_eq!(*val.borrow(), decoded);
}
