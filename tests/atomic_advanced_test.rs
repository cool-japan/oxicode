//! Advanced roundtrip tests for atomic types, Mutex, RwLock, and derive-based structs.

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
mod atomic_advanced_tests {
    use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
    use std::sync::atomic::{
        AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
        AtomicU64, AtomicU8, AtomicUsize, Ordering,
    };
    use std::sync::{Mutex, RwLock};

    // ===== Test 1: AtomicBool true roundtrip =====

    #[cfg(target_has_atomic = "8")]
    #[test]
    fn test_atomic_bool_true_advanced_roundtrip() {
        let original = AtomicBool::new(true);
        let bytes = encode_to_vec(&original).expect("encode AtomicBool(true)");
        let (decoded, _): (AtomicBool, _) =
            decode_from_slice(&bytes).expect("decode AtomicBool(true)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicBool(true) roundtrip failed"
        );
    }

    // ===== Test 2: AtomicBool false roundtrip =====

    #[cfg(target_has_atomic = "8")]
    #[test]
    fn test_atomic_bool_false_advanced_roundtrip() {
        let original = AtomicBool::new(false);
        let bytes = encode_to_vec(&original).expect("encode AtomicBool(false)");
        let (decoded, _): (AtomicBool, _) =
            decode_from_slice(&bytes).expect("decode AtomicBool(false)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicBool(false) roundtrip failed"
        );
    }

    // ===== Test 3: AtomicU8 max value roundtrip =====

    #[cfg(target_has_atomic = "8")]
    #[test]
    fn test_atomic_u8_max_roundtrip() {
        let original = AtomicU8::new(u8::MAX);
        let bytes = encode_to_vec(&original).expect("encode AtomicU8(MAX)");
        let (decoded, _): (AtomicU8, _) = decode_from_slice(&bytes).expect("decode AtomicU8(MAX)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicU8(MAX) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), u8::MAX);
    }

    // ===== Test 4: AtomicU16 max value roundtrip =====

    #[cfg(target_has_atomic = "16")]
    #[test]
    fn test_atomic_u16_max_roundtrip() {
        let original = AtomicU16::new(u16::MAX);
        let bytes = encode_to_vec(&original).expect("encode AtomicU16(MAX)");
        let (decoded, _): (AtomicU16, _) =
            decode_from_slice(&bytes).expect("decode AtomicU16(MAX)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicU16(MAX) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), u16::MAX);
    }

    // ===== Test 5: AtomicU32 max value roundtrip =====

    #[cfg(target_has_atomic = "32")]
    #[test]
    fn test_atomic_u32_max_roundtrip() {
        let original = AtomicU32::new(u32::MAX);
        let bytes = encode_to_vec(&original).expect("encode AtomicU32(MAX)");
        let (decoded, _): (AtomicU32, _) =
            decode_from_slice(&bytes).expect("decode AtomicU32(MAX)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicU32(MAX) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), u32::MAX);
    }

    // ===== Test 6: AtomicU64 max value roundtrip =====

    #[cfg(target_has_atomic = "64")]
    #[test]
    fn test_atomic_u64_max_roundtrip() {
        let original = AtomicU64::new(u64::MAX);
        let bytes = encode_to_vec(&original).expect("encode AtomicU64(MAX)");
        let (decoded, _): (AtomicU64, _) =
            decode_from_slice(&bytes).expect("decode AtomicU64(MAX)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicU64(MAX) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), u64::MAX);
    }

    // ===== Test 7: AtomicI8 min value roundtrip =====

    #[cfg(target_has_atomic = "8")]
    #[test]
    fn test_atomic_i8_min_roundtrip() {
        let original = AtomicI8::new(i8::MIN);
        let bytes = encode_to_vec(&original).expect("encode AtomicI8(MIN)");
        let (decoded, _): (AtomicI8, _) = decode_from_slice(&bytes).expect("decode AtomicI8(MIN)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicI8(MIN) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), i8::MIN);
    }

    // ===== Test 8: AtomicI16 min value roundtrip =====

    #[cfg(target_has_atomic = "16")]
    #[test]
    fn test_atomic_i16_min_roundtrip() {
        let original = AtomicI16::new(i16::MIN);
        let bytes = encode_to_vec(&original).expect("encode AtomicI16(MIN)");
        let (decoded, _): (AtomicI16, _) =
            decode_from_slice(&bytes).expect("decode AtomicI16(MIN)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicI16(MIN) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), i16::MIN);
    }

    // ===== Test 9: AtomicI32 min value roundtrip =====

    #[cfg(target_has_atomic = "32")]
    #[test]
    fn test_atomic_i32_min_roundtrip() {
        let original = AtomicI32::new(i32::MIN);
        let bytes = encode_to_vec(&original).expect("encode AtomicI32(MIN)");
        let (decoded, _): (AtomicI32, _) =
            decode_from_slice(&bytes).expect("decode AtomicI32(MIN)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicI32(MIN) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), i32::MIN);
    }

    // ===== Test 10: AtomicI64 min value roundtrip =====

    #[cfg(target_has_atomic = "64")]
    #[test]
    fn test_atomic_i64_min_roundtrip() {
        let original = AtomicI64::new(i64::MIN);
        let bytes = encode_to_vec(&original).expect("encode AtomicI64(MIN)");
        let (decoded, _): (AtomicI64, _) =
            decode_from_slice(&bytes).expect("decode AtomicI64(MIN)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicI64(MIN) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), i64::MIN);
    }

    // ===== Test 11: AtomicUsize max value roundtrip =====

    #[cfg(target_has_atomic = "ptr")]
    #[test]
    fn test_atomic_usize_max_roundtrip() {
        let original = AtomicUsize::new(usize::MAX);
        let bytes = encode_to_vec(&original).expect("encode AtomicUsize(MAX)");
        let (decoded, _): (AtomicUsize, _) =
            decode_from_slice(&bytes).expect("decode AtomicUsize(MAX)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicUsize(MAX) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), usize::MAX);
    }

    // ===== Test 12: AtomicIsize min value roundtrip =====

    #[cfg(target_has_atomic = "ptr")]
    #[test]
    fn test_atomic_isize_min_roundtrip() {
        let original = AtomicIsize::new(isize::MIN);
        let bytes = encode_to_vec(&original).expect("encode AtomicIsize(MIN)");
        let (decoded, _): (AtomicIsize, _) =
            decode_from_slice(&bytes).expect("decode AtomicIsize(MIN)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicIsize(MIN) roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), isize::MIN);
    }

    // ===== Test 13: Mutex<u32> roundtrip =====

    #[test]
    fn test_mutex_u32_roundtrip() {
        let original = Mutex::new(0xDEAD_BEEFu32);
        let bytes = encode_to_vec(&original).expect("encode Mutex<u32>");
        let (decoded, _): (Mutex<u32>, _) = decode_from_slice(&bytes).expect("decode Mutex<u32>");
        let original_val = *original.lock().expect("lock original Mutex<u32>");
        let decoded_val = *decoded.lock().expect("lock decoded Mutex<u32>");
        assert_eq!(original_val, decoded_val, "Mutex<u32> roundtrip failed");
    }

    // ===== Test 14: Mutex<String> roundtrip =====

    #[test]
    fn test_mutex_string_roundtrip() {
        let original = Mutex::new(String::from("oxicode mutex string test"));
        let bytes = encode_to_vec(&original).expect("encode Mutex<String>");
        let (decoded, _): (Mutex<String>, _) =
            decode_from_slice(&bytes).expect("decode Mutex<String>");
        let original_val = original
            .lock()
            .expect("lock original Mutex<String>")
            .clone();
        let decoded_val = decoded.lock().expect("lock decoded Mutex<String>").clone();
        assert_eq!(original_val, decoded_val, "Mutex<String> roundtrip failed");
    }

    // ===== Test 15: Mutex<Vec<u8>> roundtrip =====

    #[test]
    fn test_mutex_vec_u8_roundtrip() {
        let data: Vec<u8> = (0u8..=255u8).collect();
        let original = Mutex::new(data.clone());
        let bytes = encode_to_vec(&original).expect("encode Mutex<Vec<u8>>");
        let (decoded, _): (Mutex<Vec<u8>>, _) =
            decode_from_slice(&bytes).expect("decode Mutex<Vec<u8>>");
        let original_val = original
            .lock()
            .expect("lock original Mutex<Vec<u8>>")
            .clone();
        let decoded_val = decoded.lock().expect("lock decoded Mutex<Vec<u8>>").clone();
        assert_eq!(original_val, decoded_val, "Mutex<Vec<u8>> roundtrip failed");
        assert_eq!(decoded_val.len(), 256);
    }

    // ===== Test 16: RwLock<u32> roundtrip =====

    #[test]
    fn test_rwlock_u32_roundtrip() {
        let original = RwLock::new(0xCAFE_BABEu32);
        let bytes = encode_to_vec(&original).expect("encode RwLock<u32>");
        let (decoded, _): (RwLock<u32>, _) = decode_from_slice(&bytes).expect("decode RwLock<u32>");
        let original_val = *original.read().expect("read original RwLock<u32>");
        let decoded_val = *decoded.read().expect("read decoded RwLock<u32>");
        assert_eq!(original_val, decoded_val, "RwLock<u32> roundtrip failed");
    }

    // ===== Test 17: RwLock<String> roundtrip =====

    #[test]
    fn test_rwlock_string_roundtrip() {
        let original = RwLock::new(String::from("oxicode rwlock string test"));
        let bytes = encode_to_vec(&original).expect("encode RwLock<String>");
        let (decoded, _): (RwLock<String>, _) =
            decode_from_slice(&bytes).expect("decode RwLock<String>");
        let original_val = original
            .read()
            .expect("read original RwLock<String>")
            .clone();
        let decoded_val = decoded.read().expect("read decoded RwLock<String>").clone();
        assert_eq!(original_val, decoded_val, "RwLock<String> roundtrip failed");
    }

    // ===== Test 18: Struct with AtomicU32 field derive roundtrip =====

    #[cfg(target_has_atomic = "32")]
    #[derive(Encode, Decode)]
    struct AtomicCounter {
        value: AtomicU32,
        name: String,
    }

    #[cfg(target_has_atomic = "32")]
    #[test]
    fn test_struct_with_atomic_u32_field_derive_roundtrip() {
        // Use PI bits as an interesting non-trivial value
        let pi_bits = (std::f64::consts::PI * 1_000_000.0) as u32;
        let original = AtomicCounter {
            value: AtomicU32::new(pi_bits),
            name: String::from("pi_counter"),
        };
        let bytes = encode_to_vec(&original).expect("encode AtomicCounter");
        let (decoded, _): (AtomicCounter, _) =
            decode_from_slice(&bytes).expect("decode AtomicCounter");
        assert_eq!(
            original.value.load(Ordering::SeqCst),
            decoded.value.load(Ordering::SeqCst),
            "AtomicCounter.value mismatch"
        );
        assert_eq!(original.name, decoded.name, "AtomicCounter.name mismatch");
    }

    // ===== Test 19: Vec of AtomicU64 values roundtrip =====

    #[cfg(target_has_atomic = "64")]
    #[test]
    fn test_vec_of_atomic_u64_roundtrip() {
        // Use E and PI bits as interesting values alongside boundary values
        let e_bits = (std::f64::consts::E * 1_000_000_000.0) as u64;
        let pi_bits = (std::f64::consts::PI * 1_000_000_000.0) as u64;
        let original_values: Vec<u64> =
            vec![0, 1, e_bits, pi_bits, u64::MAX / 2, u64::MAX - 1, u64::MAX];
        let original: Vec<AtomicU64> = original_values.iter().map(|&v| AtomicU64::new(v)).collect();

        let bytes = encode_to_vec(&original).expect("encode Vec<AtomicU64>");
        let (decoded, _): (Vec<AtomicU64>, _) =
            decode_from_slice(&bytes).expect("decode Vec<AtomicU64>");

        assert_eq!(
            original.len(),
            decoded.len(),
            "Vec<AtomicU64> length mismatch"
        );
        for (i, (orig, dec)) in original.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(
                orig.load(Ordering::SeqCst),
                dec.load(Ordering::SeqCst),
                "Vec<AtomicU64>[{}] mismatch",
                i
            );
        }
    }

    // ===== Test 20: AtomicU32 value 0 roundtrip (boundary) =====

    #[cfg(target_has_atomic = "32")]
    #[test]
    fn test_atomic_u32_zero_boundary_roundtrip() {
        let original = AtomicU32::new(0u32);
        let bytes = encode_to_vec(&original).expect("encode AtomicU32(0)");
        let (decoded, _): (AtomicU32, _) = decode_from_slice(&bytes).expect("decode AtomicU32(0)");
        assert_eq!(
            original.load(Ordering::SeqCst),
            decoded.load(Ordering::SeqCst),
            "AtomicU32(0) boundary roundtrip failed"
        );
        assert_eq!(decoded.load(Ordering::SeqCst), 0u32);
    }
}
