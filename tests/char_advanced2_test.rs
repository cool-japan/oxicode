//! Advanced char encoding tests for OxiCode (set 2)

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

#[test]
fn test_char_ascii_roundtrip() {
    let ch = 'A';
    let enc = encode_to_vec(&ch).expect("encode 'A' failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode 'A' failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_zero_roundtrip() {
    let ch = '\0';
    let enc = encode_to_vec(&ch).expect("encode null char failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode null char failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_max_roundtrip() {
    let ch = char::MAX;
    let enc = encode_to_vec(&ch).expect("encode char::MAX failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode char::MAX failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_space_roundtrip() {
    let ch = ' ';
    let enc = encode_to_vec(&ch).expect("encode space failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode space failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_newline_roundtrip() {
    let ch = '\n';
    let enc = encode_to_vec(&ch).expect("encode newline failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode newline failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_tab_roundtrip() {
    let ch = '\t';
    let enc = encode_to_vec(&ch).expect("encode tab failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode tab failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_unicode_emoji_roundtrip() {
    let ch = '🦀';
    let enc = encode_to_vec(&ch).expect("encode crab emoji failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode crab emoji failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_unicode_cjk_roundtrip() {
    let ch = '日';
    let enc = encode_to_vec(&ch).expect("encode CJK char failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode CJK char failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_unicode_arabic_roundtrip() {
    let ch = 'ع';
    let enc = encode_to_vec(&ch).expect("encode Arabic letter failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode Arabic letter failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_consumed_equals_encoded_len() {
    let ch = 'Z';
    let enc = encode_to_vec(&ch).expect("encode 'Z' failed");
    let (_, consumed): (char, usize) = decode_from_slice(&enc).expect("decode 'Z' failed");
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_char_ascii_vs_unicode_size() {
    let ascii_enc = encode_to_vec(&'A').expect("encode 'A' failed");
    let emoji_enc = encode_to_vec(&'🦀').expect("encode crab emoji failed");
    // Both must encode successfully and produce non-empty bytes.
    assert!(!ascii_enc.is_empty());
    assert!(!emoji_enc.is_empty());
    // With variable-length varint encoding, a larger codepoint takes at least as many bytes.
    assert!(emoji_enc.len() >= ascii_enc.len());
}

#[test]
fn test_vec_char_roundtrip() {
    let chars: Vec<char> = vec!['H', 'e', 'l', 'l', 'o', '🦀', '日'];
    let enc = encode_to_vec(&chars).expect("encode Vec<char> failed");
    let (val, _): (Vec<char>, usize) = decode_from_slice(&enc).expect("decode Vec<char> failed");
    assert_eq!(val, chars);
}

#[test]
fn test_option_char_some_roundtrip() {
    let opt: Option<char> = Some('X');
    let enc = encode_to_vec(&opt).expect("encode Option<char> Some failed");
    let (val, _): (Option<char>, usize) =
        decode_from_slice(&enc).expect("decode Option<char> Some failed");
    assert_eq!(val, opt);
}

#[test]
fn test_option_char_none_roundtrip() {
    let opt: Option<char> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<char> None failed");
    let (val, _): (Option<char>, usize) =
        decode_from_slice(&enc).expect("decode Option<char> None failed");
    assert_eq!(val, opt);
}

#[test]
fn test_char_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let ch = 'Q';
    let enc = encode_to_vec_with_config(&ch, cfg).expect("encode with fixed_int failed");
    let (val, _): (char, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with fixed_int failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_big_endian_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let ch = 'Ω';
    let enc = encode_to_vec_with_config(&ch, cfg).expect("encode with big_endian+fixed_int failed");
    let (val, _): (char, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with big_endian+fixed_int failed");
    assert_eq!(val, ch);
}

#[test]
fn test_char_tuple_roundtrip() {
    let tup = ('a', 'b', 'c');
    let enc = encode_to_vec(&tup).expect("encode (char, char, char) failed");
    let (val, _): ((char, char, char), usize) =
        decode_from_slice(&enc).expect("decode (char, char, char) failed");
    assert_eq!(val, tup);
}

#[test]
fn test_char_array_roundtrip() {
    let arr: [char; 4] = ['R', 'u', 's', 't'];
    let enc = encode_to_vec(&arr).expect("encode [char; 4] failed");
    let (val, _): ([char; 4], usize) = decode_from_slice(&enc).expect("decode [char; 4] failed");
    assert_eq!(val, arr);
}

#[test]
fn test_char_latin_supplement_roundtrip() {
    // U+00E9 LATIN SMALL LETTER E WITH ACUTE
    let ch = '\u{00E9}';
    let enc = encode_to_vec(&ch).expect("encode U+00E9 failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode U+00E9 failed");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x00E9);
}

#[test]
fn test_char_latin_extended_roundtrip() {
    // U+03A9 GREEK CAPITAL LETTER OMEGA
    let ch = '\u{03A9}';
    let enc = encode_to_vec(&ch).expect("encode U+03A9 failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode U+03A9 failed");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x03A9);
}

#[test]
fn test_char_surrogate_boundary() {
    // U+D7FF is the last valid scalar before the surrogate range (U+D800..=U+DFFF)
    let ch = '\u{D7FF}';
    let enc = encode_to_vec(&ch).expect("encode U+D7FF failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode U+D7FF failed");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0xD7FF);
}

#[test]
fn test_char_supplementary_plane() {
    // U+1D11E MUSICAL SYMBOL G CLEF
    let ch = '\u{1D11E}';
    let enc = encode_to_vec(&ch).expect("encode U+1D11E failed");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode U+1D11E failed");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x1D11E);
}
