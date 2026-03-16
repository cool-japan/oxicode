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
    encode_to_vec_with_config, Decode, Encode,
};
use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Pending(u32),
}

// 1. prop: any NonZeroU32 (1..=u32::MAX) encodes/decodes correctly
proptest! {
    #[test]
    fn prop_nonzero_u32_roundtrip(val in 1u32..=u32::MAX) {
        let enc = encode_to_vec(&val).expect("encode NonZeroU32-range u32");
        let (dec, _): (u32, usize) = decode_from_slice(&enc).expect("decode NonZeroU32-range u32");
        prop_assert_eq!(val, dec);
        prop_assert!(dec >= 1u32, "decoded value must remain non-zero");
    }
}

// 2. prop: any i64 with big_endian config roundtrips
proptest! {
    #[test]
    fn prop_i64_big_endian_roundtrip(val: i64) {
        let cfg = config::standard().with_big_endian().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode i64 big_endian");
        let (dec, _): (i64, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode i64 big_endian");
        prop_assert_eq!(val, dec);
    }
}

// 3. prop: Rectangle with any width/height roundtrips
proptest! {
    #[test]
    fn prop_rectangle_roundtrip(width: u32, height: u32) {
        let val = Rectangle { width, height };
        let enc = encode_to_vec(&val).expect("encode Rectangle");
        let (dec, _): (Rectangle, usize) = decode_from_slice(&enc).expect("decode Rectangle");
        prop_assert_eq!(val, dec);
    }
}

// 4. prop: Status::Active/Inactive/Pending roundtrips
proptest! {
    #[test]
    fn prop_status_roundtrip(variant in 0u8..3u8, pending_val: u32) {
        let val = match variant {
            0 => Status::Active,
            1 => Status::Inactive,
            _ => Status::Pending(pending_val),
        };
        let enc = encode_to_vec(&val).expect("encode Status");
        let (dec, _): (Status, usize) = decode_from_slice(&enc).expect("decode Status");
        prop_assert_eq!(val, dec);
    }
}

// 5. prop: any BTreeMap<u32, u32> roundtrips
proptest! {
    #[test]
    fn prop_btreemap_u32_u32_roundtrip(
        val in proptest::collection::btree_map(any::<u32>(), any::<u32>(), 0..16)
    ) {
        let enc = encode_to_vec(&val).expect("encode BTreeMap<u32, u32>");
        let (dec, _): (std::collections::BTreeMap<u32, u32>, usize) =
            decode_from_slice(&enc).expect("decode BTreeMap<u32, u32>");
        prop_assert_eq!(val, dec);
    }
}

// 6. prop: any Vec<(u32, u32)> roundtrips
proptest! {
    #[test]
    fn prop_vec_tuple_u32_u32_roundtrip(
        val in proptest::collection::vec((any::<u32>(), any::<u32>()), 0..32)
    ) {
        let enc = encode_to_vec(&val).expect("encode Vec<(u32, u32)>");
        let (dec, _): (Vec<(u32, u32)>, usize) =
            decode_from_slice(&enc).expect("decode Vec<(u32, u32)>");
        prop_assert_eq!(val, dec);
    }
}

// 7. prop: any char roundtrips (use valid Unicode)
proptest! {
    #[test]
    fn prop_char_valid_unicode_roundtrip(val: char) {
        let enc = encode_to_vec(&val).expect("encode char");
        let (dec, _): (char, usize) = decode_from_slice(&enc).expect("decode char");
        prop_assert_eq!(val, dec);
    }
}

// 8. prop: encode then re-encode is idempotent for bool
proptest! {
    #[test]
    fn prop_bool_encode_idempotent(val: bool) {
        let enc1 = encode_to_vec(&val).expect("first encode bool");
        let (decoded, _): (bool, usize) = decode_from_slice(&enc1).expect("decode bool for idempotency");
        let enc2 = encode_to_vec(&decoded).expect("second encode bool");
        prop_assert_eq!(enc1, enc2);
    }
}

// 9. prop: u32 encoded with standard config then decoded gives same value
proptest! {
    #[test]
    fn prop_u32_standard_config_roundtrip(val: u32) {
        let cfg = config::standard();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode u32 standard config");
        let (dec, _): (u32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode u32 standard config");
        prop_assert_eq!(val, dec);
    }
}

// 10. prop: String with ascii chars roundtrips
proptest! {
    #[test]
    fn prop_string_ascii_roundtrip(val in "[a-zA-Z0-9 .,!?]{0,64}") {
        let enc = encode_to_vec(&val).expect("encode ascii String");
        let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode ascii String");
        prop_assert_eq!(val, dec);
    }
}

// 11. prop: any i8 with fixed_int_encoding roundtrips
proptest! {
    #[test]
    fn prop_i8_fixed_int_roundtrip(val: i8) {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode i8 fixed_int");
        let (dec, _): (i8, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode i8 fixed_int");
        prop_assert_eq!(val, dec);
        prop_assert_eq!(enc.len(), 1usize, "i8 with fixed_int_encoding must always be 1 byte");
    }
}

// 12. prop: any u64 roundtrips
proptest! {
    #[test]
    fn prop_u64_standard_roundtrip(val: u64) {
        let enc = encode_to_vec(&val).expect("encode u64");
        let (dec, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64");
        prop_assert_eq!(val, dec);
    }
}

// 13. prop: Option<u32> roundtrips
proptest! {
    #[test]
    fn prop_option_u32_standard_roundtrip(val: Option<u32>) {
        let enc = encode_to_vec(&val).expect("encode Option<u32>");
        let (dec, _): (Option<u32>, usize) =
            decode_from_slice(&enc).expect("decode Option<u32>");
        prop_assert_eq!(val, dec);
    }
}

// 14. prop: Vec<bool> roundtrips
proptest! {
    #[test]
    fn prop_vec_bool_standard_roundtrip(
        val in proptest::collection::vec(any::<bool>(), 0..64)
    ) {
        let enc = encode_to_vec(&val).expect("encode Vec<bool>");
        let (dec, _): (Vec<bool>, usize) = decode_from_slice(&enc).expect("decode Vec<bool>");
        prop_assert_eq!(val, dec);
    }
}

// 15. prop: any i64 roundtrips
proptest! {
    #[test]
    fn prop_i64_standard_roundtrip(val: i64) {
        let enc = encode_to_vec(&val).expect("encode i64");
        let (dec, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64");
        prop_assert_eq!(val, dec);
    }
}

// 16. prop: (u32, u32, String) triple roundtrips
proptest! {
    #[test]
    fn prop_triple_u32_u32_string_roundtrip(
        a: u32,
        b: u32,
        s in "[a-zA-Z0-9]{0,32}"
    ) {
        let val = (a, b, s);
        let enc = encode_to_vec(&val).expect("encode (u32, u32, String)");
        let (dec, _): ((u32, u32, String), usize) =
            decode_from_slice(&enc).expect("decode (u32, u32, String)");
        prop_assert_eq!(val, dec);
    }
}

// 17. prop: encode(a) != encode(b) when a != b for u64
proptest! {
    #[test]
    fn prop_u64_distinct_values_distinct_encodings(a: u64, b: u64) {
        let enc_a = encode_to_vec(&a).expect("encode u64 a");
        let enc_b = encode_to_vec(&b).expect("encode u64 b");
        if a == b {
            prop_assert_eq!(&enc_a, &enc_b);
        } else {
            prop_assert_ne!(&enc_a, &enc_b);
        }
    }
}

// 18. prop: u8 always encodes to 1 byte (standard config)
proptest! {
    #[test]
    fn prop_u8_always_1_byte(val: u8) {
        let enc = encode_to_vec(&val).expect("encode u8 for size check");
        prop_assert_eq!(enc.len(), 1usize, "u8 must always encode to exactly 1 byte");
    }
}

// 19. prop: i32 with fixed_int_encoding always 4 bytes
proptest! {
    #[test]
    fn prop_i32_fixed_int_encoding_always_4_bytes(val: i32) {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode i32 fixed_int for size check");
        prop_assert_eq!(enc.len(), 4usize, "i32 with fixed_int_encoding must always be 4 bytes");
        let (dec, _): (i32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode i32 fixed_int for size check");
        prop_assert_eq!(val, dec);
    }
}

// 20. prop: any Vec<u8> of length <= 100 roundtrips
proptest! {
    #[test]
    fn prop_vec_u8_max100_roundtrip(
        val in proptest::collection::vec(any::<u8>(), 0..=100)
    ) {
        prop_assume!(val.len() <= 100);
        let enc = encode_to_vec(&val).expect("encode Vec<u8> max100");
        let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> max100");
        prop_assert_eq!(val, dec);
    }
}

// 21. prop: any (i32, String) pair roundtrips
proptest! {
    #[test]
    fn prop_i32_string_pair_roundtrip(
        n: i32,
        s in "[a-zA-Z0-9_\\-]{0,48}"
    ) {
        let val = (n, s);
        let enc = encode_to_vec(&val).expect("encode (i32, String)");
        let (dec, _): ((i32, String), usize) =
            decode_from_slice(&enc).expect("decode (i32, String)");
        prop_assert_eq!(val, dec);
    }
}

// 22. prop: any [u32; 4] array roundtrips
proptest! {
    #[test]
    fn prop_array_u32_4_roundtrip(a: u32, b: u32, c: u32, d: u32) {
        let val: [u32; 4] = [a, b, c, d];
        let enc = encode_to_vec(&val).expect("encode [u32; 4]");
        let (dec, _): ([u32; 4], usize) = decode_from_slice(&enc).expect("decode [u32; 4]");
        prop_assert_eq!(val, dec);
    }
}
