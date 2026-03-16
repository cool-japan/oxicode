//! Roundtrip and encoding tests for `Mutex<T>` and `RwLock<T>`.

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
use std::sync::{Mutex, RwLock};

// ===== Mutex roundtrip tests =====

#[test]
fn mutex_rwlock_mutex_u32_roundtrip() {
    let original = Mutex::new(42u32);
    let encoded = encode_to_vec(&original).expect("encode Mutex<u32>");
    let (decoded, _): (Mutex<u32>, usize) = decode_from_slice(&encoded).expect("decode Mutex<u32>");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
}

#[test]
fn mutex_rwlock_mutex_string_roundtrip() {
    let original = Mutex::new("hello".to_string());
    let encoded = encode_to_vec(&original).expect("encode Mutex<String>");
    let (decoded, _): (Mutex<String>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<String>");
    assert_eq!(
        original.lock().expect("lock").clone(),
        decoded.lock().expect("lock").clone()
    );
}

#[test]
fn mutex_rwlock_mutex_vec_u8_roundtrip() {
    let original = Mutex::new(vec![1u8, 2, 3]);
    let encoded = encode_to_vec(&original).expect("encode Mutex<Vec<u8>>");
    let (decoded, _): (Mutex<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<Vec<u8>>");
    assert_eq!(
        original.lock().expect("lock").clone(),
        decoded.lock().expect("lock").clone()
    );
}

#[test]
fn mutex_rwlock_mutex_bool_true_roundtrip() {
    let original = Mutex::new(true);
    let encoded = encode_to_vec(&original).expect("encode Mutex<bool> true");
    let (decoded, _): (Mutex<bool>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<bool> true");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
}

#[test]
fn mutex_rwlock_mutex_bool_false_roundtrip() {
    let original = Mutex::new(false);
    let encoded = encode_to_vec(&original).expect("encode Mutex<bool> false");
    let (decoded, _): (Mutex<bool>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<bool> false");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
}

#[test]
fn mutex_rwlock_mutex_u64_max_roundtrip() {
    let original = Mutex::new(u64::MAX);
    let encoded = encode_to_vec(&original).expect("encode Mutex<u64::MAX>");
    let (decoded, _): (Mutex<u64>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<u64::MAX>");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
}

#[test]
fn mutex_rwlock_mutex_i64_min_roundtrip() {
    let original = Mutex::new(i64::MIN);
    let encoded = encode_to_vec(&original).expect("encode Mutex<i64::MIN>");
    let (decoded, _): (Mutex<i64>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<i64::MIN>");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
}

#[test]
fn mutex_rwlock_mutex_u32_same_bytes_as_u32() {
    let value = 12345u32;
    let mutex_bytes = encode_to_vec(&Mutex::new(value)).expect("encode Mutex<u32>");
    let plain_bytes = encode_to_vec(&value).expect("encode u32");
    assert_eq!(
        mutex_bytes, plain_bytes,
        "Mutex<u32> must encode identically to u32"
    );
}

#[test]
fn mutex_rwlock_option_mutex_u32_some_roundtrip() {
    let original: Option<Mutex<u32>> = Some(Mutex::new(99u32));
    let encoded = encode_to_vec(&original).expect("encode Option<Mutex<u32>> Some");
    let (decoded, _): (Option<Mutex<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Mutex<u32>> Some");
    let orig_val = original.as_ref().map(|m| *m.lock().expect("lock orig"));
    let dec_val = decoded.as_ref().map(|m| *m.lock().expect("lock dec"));
    assert_eq!(orig_val, dec_val);
}

#[test]
fn mutex_rwlock_option_mutex_u32_none_roundtrip() {
    let original: Option<Mutex<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Mutex<u32>> None");
    let (decoded, _): (Option<Mutex<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Mutex<u32>> None");
    assert!(decoded.is_none(), "decoded Option must be None");
}

#[test]
fn mutex_rwlock_mutex_fixed_int_encoding_roundtrip() {
    let original = Mutex::new(1000u32);
    let cfg = config::legacy();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode Mutex<u32> fixed");
    let (decoded, _): (Mutex<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Mutex<u32> fixed");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
    // legacy (fixed) encoding of u32 is always 4 bytes
    assert_eq!(
        encoded.len(),
        4,
        "fixed int encoding of u32 must be 4 bytes"
    );
}

#[test]
fn mutex_rwlock_mutex_big_endian_config_roundtrip() {
    use oxicode::config::standard;
    // Build a big-endian config: standard with big endian set
    let cfg = standard().with_big_endian();
    let original = Mutex::new(0xCAFEu32);
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode Mutex<u32> BE");
    let (decoded, _): (Mutex<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Mutex<u32> BE");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
}

// ===== RwLock roundtrip tests =====

#[test]
fn mutex_rwlock_rwlock_u32_roundtrip() {
    let original = RwLock::new(42u32);
    let encoded = encode_to_vec(&original).expect("encode RwLock<u32>");
    let (decoded, _): (RwLock<u32>, usize) =
        decode_from_slice(&encoded).expect("decode RwLock<u32>");
    assert_eq!(
        *original.read().expect("read"),
        *decoded.read().expect("read")
    );
}

#[test]
fn mutex_rwlock_rwlock_string_roundtrip() {
    let original = RwLock::new("world".to_string());
    let encoded = encode_to_vec(&original).expect("encode RwLock<String>");
    let (decoded, _): (RwLock<String>, usize) =
        decode_from_slice(&encoded).expect("decode RwLock<String>");
    assert_eq!(
        original.read().expect("read").clone(),
        decoded.read().expect("read").clone()
    );
}

#[test]
fn mutex_rwlock_rwlock_vec_u8_roundtrip() {
    let original = RwLock::new(vec![10u8, 20, 30]);
    let encoded = encode_to_vec(&original).expect("encode RwLock<Vec<u8>>");
    let (decoded, _): (RwLock<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode RwLock<Vec<u8>>");
    assert_eq!(
        original.read().expect("read").clone(),
        decoded.read().expect("read").clone()
    );
}

#[test]
fn mutex_rwlock_rwlock_bool_false_roundtrip() {
    let original = RwLock::new(false);
    let encoded = encode_to_vec(&original).expect("encode RwLock<bool> false");
    let (decoded, _): (RwLock<bool>, usize) =
        decode_from_slice(&encoded).expect("decode RwLock<bool> false");
    assert_eq!(
        *original.read().expect("read"),
        *decoded.read().expect("read")
    );
}

#[test]
fn mutex_rwlock_rwlock_u64_large_roundtrip() {
    let large_val = u64::MAX / 3 * 2;
    let original = RwLock::new(large_val);
    let encoded = encode_to_vec(&original).expect("encode RwLock<u64> large");
    let (decoded, _): (RwLock<u64>, usize) =
        decode_from_slice(&encoded).expect("decode RwLock<u64> large");
    assert_eq!(
        *original.read().expect("read"),
        *decoded.read().expect("read")
    );
}

#[test]
fn mutex_rwlock_rwlock_u32_same_bytes_as_u32() {
    let value = 12345u32;
    let rwlock_bytes = encode_to_vec(&RwLock::new(value)).expect("encode RwLock<u32>");
    let plain_bytes = encode_to_vec(&value).expect("encode u32");
    assert_eq!(
        rwlock_bytes, plain_bytes,
        "RwLock<u32> must encode identically to u32"
    );
}

#[test]
fn mutex_rwlock_option_rwlock_string_some_roundtrip() {
    let original: Option<RwLock<String>> = Some(RwLock::new("oxicode".to_string()));
    let encoded = encode_to_vec(&original).expect("encode Option<RwLock<String>> Some");
    let (decoded, _): (Option<RwLock<String>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<RwLock<String>> Some");
    let orig_val = original
        .as_ref()
        .map(|r| r.read().expect("read orig").clone());
    let dec_val = decoded
        .as_ref()
        .map(|r| r.read().expect("read dec").clone());
    assert_eq!(orig_val, dec_val);
}

#[test]
fn mutex_rwlock_rwlock_fixed_int_encoding_roundtrip() {
    let original = RwLock::new(1000u32);
    let cfg = config::legacy();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode RwLock<u32> fixed");
    let (decoded, _): (RwLock<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode RwLock<u32> fixed");
    assert_eq!(
        *original.read().expect("read"),
        *decoded.read().expect("read")
    );
    assert_eq!(
        encoded.len(),
        4,
        "fixed int encoding of u32 must be 4 bytes"
    );
}

#[test]
fn mutex_rwlock_rwlock_big_endian_config_roundtrip() {
    use oxicode::config::standard;
    let cfg = standard().with_big_endian();
    let original = RwLock::new(0xBEEFu32);
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode RwLock<u32> BE");
    let (decoded, _): (RwLock<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode RwLock<u32> BE");
    assert_eq!(
        *original.read().expect("read"),
        *decoded.read().expect("read")
    );
}

// ===== Combined / nested tests =====

#[test]
fn mutex_rwlock_mutex_tuple_roundtrip() {
    let original = Mutex::new((42u32, "test".to_string()));
    let encoded = encode_to_vec(&original).expect("encode Mutex<(u32, String)>");
    let (decoded, _): (Mutex<(u32, String)>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<(u32, String)>");
    assert_eq!(
        original.lock().expect("lock").clone(),
        decoded.lock().expect("lock").clone()
    );
}

#[test]
fn mutex_rwlock_rwlock_vec_string_roundtrip() {
    let original = RwLock::new(vec!["a".to_string(), "b".to_string()]);
    let encoded = encode_to_vec(&original).expect("encode RwLock<Vec<String>>");
    let (decoded, _): (RwLock<Vec<String>>, usize) =
        decode_from_slice(&encoded).expect("decode RwLock<Vec<String>>");
    assert_eq!(
        original.read().expect("read").clone(),
        decoded.read().expect("read").clone()
    );
}

#[test]
fn mutex_rwlock_mutex_option_none_roundtrip() {
    let original = Mutex::new(Option::<u32>::None);
    let encoded = encode_to_vec(&original).expect("encode Mutex<Option<u32>> None");
    let (decoded, _): (Mutex<Option<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Mutex<Option<u32>> None");
    assert_eq!(
        *original.lock().expect("lock"),
        *decoded.lock().expect("lock")
    );
}
