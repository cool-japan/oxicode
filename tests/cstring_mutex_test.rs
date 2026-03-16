//! Roundtrip and error tests for CString, Mutex, and RwLock types.

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
use oxicode::Encode;
use std::ffi::CString;
use std::sync::{Mutex, RwLock};

// Helper: encode a value to bytes then decode it back, asserting equality
// via a caller-supplied comparison closure.
fn roundtrip_bytes<T: Encode>(val: &T) -> Vec<u8> {
    oxicode::encode_to_vec(val).expect("encode must succeed")
}

// ===== CString roundtrip tests =====

#[test]
fn cstring_mutex_cstring_ascii_roundtrip() {
    let original = CString::new("hello world").expect("valid cstring");
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (CString, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    assert_eq!(original, decoded);
}

#[test]
fn cstring_mutex_cstring_empty_roundtrip() {
    let original = CString::new("").expect("valid cstring");
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (CString, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    assert_eq!(original, decoded);
}

#[test]
fn cstring_mutex_cstring_special_chars_roundtrip() {
    // Mix of control chars, printable ASCII, and arbitrary high bytes (as raw Vec<u8>).
    // Rust string literals only accept \x00-\x7f; high bytes must come via Vec<u8>.
    let bytes: Vec<u8> = b"tab:\there\nnewline\\backslash\x01\x7f"
        .iter()
        .copied()
        .chain([0xffu8, 0xfe])
        .collect();
    let original = CString::new(bytes).expect("valid cstring");
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (CString, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    assert_eq!(original, decoded);
}

#[test]
fn cstring_mutex_cstring_null_byte_decode_error() {
    // Build a valid CString first to get the length-prefixed wire format,
    // then patch one byte in the payload area to be NUL so the decoder
    // should reject it.
    let original = CString::new("abc").expect("valid cstring");
    let mut encoded = roundtrip_bytes(&original);

    // The encoding is: varint(len) ++ bytes.
    // For len=3, the varint is a single byte 0x03 (small value, no multi-byte).
    // The payload starts at byte index 1.  Overwrite the second payload byte
    // (index 2) with 0x00 to inject a null.
    assert!(
        encoded.len() >= 3,
        "encoded buffer too short: {}",
        encoded.len()
    );
    encoded[2] = 0x00;

    let result: Result<(CString, usize), _> = oxicode::decode_from_slice(&encoded);
    assert!(
        result.is_err(),
        "decoding a CString with an embedded null byte must return Err"
    );
}

// ===== Mutex roundtrip tests =====

#[test]
fn cstring_mutex_mutex_u32_roundtrip() {
    let original = Mutex::new(0xdeadbeef_u32);
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (Mutex<u32>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    let orig_val = *original.lock().expect("lock original");
    let dec_val = *decoded.lock().expect("lock decoded");
    assert_eq!(orig_val, dec_val);
}

#[test]
fn cstring_mutex_mutex_string_roundtrip() {
    let original = Mutex::new(String::from("oxicode serialisation"));
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (Mutex<String>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    let orig_val = original.lock().expect("lock original").clone();
    let dec_val = decoded.lock().expect("lock decoded").clone();
    assert_eq!(orig_val, dec_val);
}

// ===== RwLock roundtrip tests =====

#[test]
fn cstring_mutex_rwlock_u32_roundtrip() {
    let original = RwLock::new(42_u32);
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (RwLock<u32>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    let orig_val = *original.read().expect("read original");
    let dec_val = *decoded.read().expect("read decoded");
    assert_eq!(orig_val, dec_val);
}

#[test]
fn cstring_mutex_rwlock_vec_u8_roundtrip() {
    let original = RwLock::new(vec![0xca_u8, 0xfe, 0xba, 0xbe]);
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (RwLock<Vec<u8>>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    let orig_val = original.read().expect("read original").clone();
    let dec_val = decoded.read().expect("read decoded").clone();
    assert_eq!(orig_val, dec_val);
}

// ===== Compound / nested tests =====

#[test]
fn cstring_mutex_mutex_vec_string_roundtrip() {
    let data = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ];
    let original = Mutex::new(data);
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (Mutex<Vec<String>>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    let orig_val = original.lock().expect("lock original").clone();
    let dec_val = decoded.lock().expect("lock decoded").clone();
    assert_eq!(orig_val, dec_val);
}

#[test]
fn cstring_mutex_mutex_option_string_some_roundtrip() {
    let original: Mutex<Option<String>> = Mutex::new(Some(String::from("present")));
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (Mutex<Option<String>>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    let orig_val = original.lock().expect("lock original").clone();
    let dec_val = decoded.lock().expect("lock decoded").clone();
    assert_eq!(orig_val, dec_val);
}

#[test]
fn cstring_mutex_mutex_option_string_none_roundtrip() {
    let original: Mutex<Option<String>> = Mutex::new(None);
    let encoded = roundtrip_bytes(&original);
    let (decoded, _): (Mutex<Option<String>>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode must succeed");
    let orig_val = original.lock().expect("lock original").clone();
    let dec_val = decoded.lock().expect("lock decoded").clone();
    assert_eq!(orig_val, dec_val);
}
