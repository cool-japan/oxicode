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
    config, decode_from_slice, encode_to_vec, encode_to_vec_with_config, Decode, Encode,
};
use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

proptest! {
    #[test]
    fn prop_usize_roundtrip(val: usize) {
        let enc = encode_to_vec(&val).expect("encode usize");
        let (dec, _): (usize, usize) = decode_from_slice(&enc).expect("decode usize");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_isize_roundtrip(val: isize) {
        let enc = encode_to_vec(&val).expect("encode isize");
        let (dec, _): (isize, usize) = decode_from_slice(&enc).expect("decode isize");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_u128_roundtrip(val: u128) {
        let enc = encode_to_vec(&val).expect("encode u128");
        let (dec, _): (u128, usize) = decode_from_slice(&enc).expect("decode u128");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_i128_roundtrip(val: i128) {
        let enc = encode_to_vec(&val).expect("encode i128");
        let (dec, _): (i128, usize) = decode_from_slice(&enc).expect("decode i128");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_char_roundtrip(val: char) {
        let enc = encode_to_vec(&val).expect("encode char");
        let (dec, _): (char, usize) = decode_from_slice(&enc).expect("decode char");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_vec_bool_roundtrip(val: Vec<bool>) {
        let enc = encode_to_vec(&val).expect("encode Vec<bool>");
        let (dec, _): (Vec<bool>, usize) = decode_from_slice(&enc).expect("decode Vec<bool>");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_vec_i32_roundtrip(val: Vec<i32>) {
        let enc = encode_to_vec(&val).expect("encode Vec<i32>");
        let (dec, _): (Vec<i32>, usize) = decode_from_slice(&enc).expect("decode Vec<i32>");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_tuple_i32_i32_roundtrip(a: i32, b: i32) {
        let val = (a, b);
        let enc = encode_to_vec(&val).expect("encode (i32, i32)");
        let (dec, _): ((i32, i32), usize) = decode_from_slice(&enc).expect("decode (i32, i32)");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_point_roundtrip(x: i32, y: i32) {
        let val = Point { x, y };
        let enc = encode_to_vec(&val).expect("encode Point");
        let (dec, _): (Point, usize) = decode_from_slice(&enc).expect("decode Point");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_color_unit_variants_roundtrip(variant in 0u8..3u8) {
        let val = match variant {
            0 => Color::Red,
            1 => Color::Green,
            _ => Color::Blue,
        };
        let enc = encode_to_vec(&val).expect("encode Color unit variant");
        let (dec, _): (Color, usize) = decode_from_slice(&enc).expect("decode Color unit variant");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_color_custom_roundtrip(r: u8, g: u8, b: u8) {
        let val = Color::Custom(r, g, b);
        let enc = encode_to_vec(&val).expect("encode Color::Custom");
        let (dec, _): (Color, usize) = decode_from_slice(&enc).expect("decode Color::Custom");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_option_vec_u32_roundtrip(val: Option<Vec<u32>>) {
        let enc = encode_to_vec(&val).expect("encode Option<Vec<u32>>");
        let (dec, _): (Option<Vec<u32>>, usize) =
            decode_from_slice(&enc).expect("decode Option<Vec<u32>>");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_vec_option_i32_roundtrip(val: Vec<Option<i32>>) {
        let enc = encode_to_vec(&val).expect("encode Vec<Option<i32>>");
        let (dec, _): (Vec<Option<i32>>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Option<i32>>");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_array_u8_4_roundtrip(a: u8, b: u8, c: u8, d: u8) {
        let val: [u8; 4] = [a, b, c, d];
        let enc = encode_to_vec(&val).expect("encode [u8; 4]");
        let (dec, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8; 4]");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_array_u32_8_roundtrip(
        v0: u32, v1: u32, v2: u32, v3: u32,
        v4: u32, v5: u32, v6: u32, v7: u32
    ) {
        let val: [u32; 8] = [v0, v1, v2, v3, v4, v5, v6, v7];
        let enc = encode_to_vec(&val).expect("encode [u32; 8]");
        let (dec, _): ([u32; 8], usize) = decode_from_slice(&enc).expect("decode [u32; 8]");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_encode_length_consistent(val: u64) {
        let enc1 = encode_to_vec(&val).expect("encode u64 first");
        let enc2 = encode_to_vec(&val).expect("encode u64 second");
        prop_assert_eq!(enc1.len(), enc2.len());
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_u64_big_endian_fixed_int_always_8_bytes(val: u64) {
        let cfg = config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode u64 big_endian fixed_int");
        prop_assert_eq!(enc.len(), 8usize, "u64 with fixed_int encoding must always be 8 bytes");
    }
}

proptest! {
    #[test]
    fn prop_i32_fixed_int_always_4_bytes(val: i32) {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode i32 fixed_int");
        prop_assert_eq!(enc.len(), 4usize, "i32 with fixed_int encoding must always be 4 bytes");
    }
}

proptest! {
    #[test]
    fn prop_string_alphanumeric_roundtrip(val in "[a-zA-Z0-9]{0,64}") {
        let enc = encode_to_vec(&val).expect("encode String");
        let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode String");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_vec_tuple_u32_string_roundtrip(
        val in prop::collection::vec(
            (any::<u32>(), "[a-zA-Z0-9]{0,16}"),
            0..8
        )
    ) {
        let enc = encode_to_vec(&val).expect("encode Vec<(u32, String)>");
        let (dec, _): (Vec<(u32, String)>, usize) =
            decode_from_slice(&enc).expect("decode Vec<(u32, String)>");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_tuple_u8_u16_u32_u64_roundtrip(a: u8, b: u16, c: u32, d: u64) {
        let val = (a, b, c, d);
        let enc = encode_to_vec(&val).expect("encode (u8, u16, u32, u64)");
        let (dec, _): ((u8, u16, u32, u64), usize) =
            decode_from_slice(&enc).expect("decode (u8, u16, u32, u64)");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_encode_sequence_same_as_individual(x: u32, y: u32) {
        let enc_x = encode_to_vec(&x).expect("encode x");
        let enc_y = encode_to_vec(&y).expect("encode y");
        let mut combined = enc_x.clone();
        combined.extend_from_slice(&enc_y);

        let (dec_x, consumed_x): (u32, usize) =
            decode_from_slice(&combined).expect("decode x from combined");
        let (dec_y, _): (u32, usize) =
            decode_from_slice(&combined[consumed_x..]).expect("decode y from combined");

        prop_assert_eq!(x, dec_x);
        prop_assert_eq!(y, dec_y);
        prop_assert_eq!(combined.len(), enc_x.len() + enc_y.len());
    }
}
