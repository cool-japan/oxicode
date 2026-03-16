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
use proptest::prelude::*;

// 1. prop: any u8 roundtrips
proptest! {
    #[test]
    fn prop_u8_roundtrip(val: u8) {
        let enc = encode_to_vec(&val).expect("encode u8");
        let (dec, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8");
        prop_assert_eq!(val, dec);
    }
}

// 2. prop: any u16 roundtrips
proptest! {
    #[test]
    fn prop_u16_roundtrip(val: u16) {
        let enc = encode_to_vec(&val).expect("encode u16");
        let (dec, _): (u16, usize) = decode_from_slice(&enc).expect("decode u16");
        prop_assert_eq!(val, dec);
    }
}

// 3. prop: any u32 roundtrips
proptest! {
    #[test]
    fn prop_u32_roundtrip(val: u32) {
        let enc = encode_to_vec(&val).expect("encode u32");
        let (dec, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32");
        prop_assert_eq!(val, dec);
    }
}

// 4. prop: any u64 roundtrips
proptest! {
    #[test]
    fn prop_u64_roundtrip(val: u64) {
        let enc = encode_to_vec(&val).expect("encode u64");
        let (dec, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64");
        prop_assert_eq!(val, dec);
    }
}

// 5. prop: any i8 roundtrips
proptest! {
    #[test]
    fn prop_i8_roundtrip(val: i8) {
        let enc = encode_to_vec(&val).expect("encode i8");
        let (dec, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8");
        prop_assert_eq!(val, dec);
    }
}

// 6. prop: any i16 roundtrips
proptest! {
    #[test]
    fn prop_i16_roundtrip(val: i16) {
        let enc = encode_to_vec(&val).expect("encode i16");
        let (dec, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16");
        prop_assert_eq!(val, dec);
    }
}

// 7. prop: any i32 roundtrips
proptest! {
    #[test]
    fn prop_i32_roundtrip(val: i32) {
        let enc = encode_to_vec(&val).expect("encode i32");
        let (dec, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32");
        prop_assert_eq!(val, dec);
    }
}

// 8. prop: any i64 roundtrips
proptest! {
    #[test]
    fn prop_i64_roundtrip(val: i64) {
        let enc = encode_to_vec(&val).expect("encode i64");
        let (dec, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64");
        prop_assert_eq!(val, dec);
    }
}

// 9. prop: any bool roundtrips
proptest! {
    #[test]
    fn prop_bool_roundtrip(val: bool) {
        let enc = encode_to_vec(&val).expect("encode bool");
        let (dec, _): (bool, usize) = decode_from_slice(&enc).expect("decode bool");
        prop_assert_eq!(val, dec);
    }
}

// 10. prop: any f32 (non-NaN) roundtrips
proptest! {
    #[test]
    fn prop_f32_roundtrip(val in prop::num::f32::NORMAL | prop::num::f32::SUBNORMAL | prop::num::f32::ZERO | prop::num::f32::INFINITE) {
        let enc = encode_to_vec(&val).expect("encode f32");
        let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32");
        prop_assert_eq!(val.to_bits(), dec.to_bits());
    }
}

// 11. prop: any f64 (non-NaN) roundtrips
proptest! {
    #[test]
    fn prop_f64_roundtrip(val in prop::num::f64::NORMAL | prop::num::f64::SUBNORMAL | prop::num::f64::ZERO | prop::num::f64::INFINITE) {
        let enc = encode_to_vec(&val).expect("encode f64");
        let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64");
        prop_assert_eq!(val.to_bits(), dec.to_bits());
    }
}

// 12. prop: any String (printable ASCII) roundtrips
proptest! {
    #[test]
    fn prop_string_printable_ascii_roundtrip(val in proptest::string::string_regex("[!-~]{0,64}").expect("string regex")) {
        let enc = encode_to_vec(&val).expect("encode String");
        let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode String");
        prop_assert_eq!(val, dec);
    }
}

// 13. prop: any Vec<u8> roundtrips
proptest! {
    #[test]
    fn prop_vec_u8_roundtrip(val in proptest::collection::vec(any::<u8>(), 0..128)) {
        let enc = encode_to_vec(&val).expect("encode Vec<u8>");
        let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8>");
        prop_assert_eq!(val, dec);
    }
}

// 14. prop: any Vec<u32> roundtrips
proptest! {
    #[test]
    fn prop_vec_u32_roundtrip(val in proptest::collection::vec(any::<u32>(), 0..64)) {
        let enc = encode_to_vec(&val).expect("encode Vec<u32>");
        let (dec, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode Vec<u32>");
        prop_assert_eq!(val, dec);
    }
}

// 15. prop: any Option<u32> roundtrips
proptest! {
    #[test]
    fn prop_option_u32_roundtrip(val: Option<u32>) {
        let enc = encode_to_vec(&val).expect("encode Option<u32>");
        let (dec, _): (Option<u32>, usize) = decode_from_slice(&enc).expect("decode Option<u32>");
        prop_assert_eq!(val, dec);
    }
}

// 16. prop: encode then decode consumed bytes == encoded length for u64
proptest! {
    #[test]
    fn prop_u64_consumed_bytes_equals_encoded_length(val: u64) {
        let enc = encode_to_vec(&val).expect("encode u64 for consumed bytes");
        let expected_len = enc.len();
        let (_, consumed) = decode_from_slice::<u64>(&enc).expect("decode u64 for consumed bytes");
        prop_assert_eq!(expected_len, consumed);
    }
}

// 17. prop: two different u32 values produce different encodings unless equal
proptest! {
    #[test]
    fn prop_u32_distinct_values_distinct_encodings(a: u32, b: u32) {
        let enc_a = encode_to_vec(&a).expect("encode u32 a");
        let enc_b = encode_to_vec(&b).expect("encode u32 b");
        if a == b {
            prop_assert_eq!(enc_a, enc_b);
        } else {
            prop_assert_ne!(enc_a, enc_b);
        }
    }
}

// 18. prop: any (u32, String) tuple roundtrips
proptest! {
    #[test]
    fn prop_u32_string_tuple_roundtrip(
        n: u32,
        s in proptest::string::string_regex("[!-~]{0,32}").expect("string regex")
    ) {
        let val = (n, s);
        let enc = encode_to_vec(&val).expect("encode (u32, String)");
        let (dec, _): ((u32, String), usize) = decode_from_slice(&enc).expect("decode (u32, String)");
        prop_assert_eq!(val, dec);
    }
}

// 19. prop: any Vec<String> roundtrips
proptest! {
    #[test]
    fn prop_vec_string_roundtrip(
        val in proptest::collection::vec(
            proptest::string::string_regex("[!-~]{0,32}").expect("string regex"),
            0..16
        )
    ) {
        let enc = encode_to_vec(&val).expect("encode Vec<String>");
        let (dec, _): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode Vec<String>");
        prop_assert_eq!(val, dec);
    }
}

// 20. prop: u32 with fixed_int_encoding always produces 4 bytes
proptest! {
    #[test]
    fn prop_u32_fixed_int_encoding_always_4_bytes(val: u32) {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode u32 with fixed_int");
        prop_assert_eq!(enc.len(), 4usize, "u32 with fixed_int_encoding must be exactly 4 bytes");
        let (dec, _): (u32, usize) = decode_from_slice_with_config(&enc, cfg).expect("decode u32 with fixed_int");
        prop_assert_eq!(val, dec);
    }
}

// 21. prop: encode(decode(encode(x))) == encode(x) (idempotency)
proptest! {
    #[test]
    fn prop_u32_encode_decode_encode_idempotency(val: u32) {
        let enc1 = encode_to_vec(&val).expect("first encode u32");
        let (dec, _): (u32, usize) = decode_from_slice(&enc1).expect("decode u32 for idempotency");
        let enc2 = encode_to_vec(&dec).expect("second encode u32");
        prop_assert_eq!(enc1, enc2);
    }
}

// 22. prop: any i64 roundtrips with big_endian + fixed_int config
proptest! {
    #[test]
    fn prop_i64_big_endian_fixed_int_roundtrip(val: i64) {
        let cfg = config::standard().with_big_endian().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode i64 big_endian+fixed_int");
        prop_assert_eq!(enc.len(), 8usize, "i64 with fixed_int_encoding must be exactly 8 bytes");
        let (dec, _): (i64, usize) = decode_from_slice_with_config(&enc, cfg).expect("decode i64 big_endian+fixed_int");
        prop_assert_eq!(val, dec);
    }
}
