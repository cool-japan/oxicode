//! Advanced char encoding tests for OxiCode (set advanced2).
//! Focuses on specific chars and encoding properties not covered in char_test.rs,
//! char_advanced_test.rs, or char_advanced2_test.rs.

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

// 1. ASCII 'a' roundtrip
#[test]
fn test_char_ascii_lowercase_a_roundtrip() {
    let ch = 'a';
    let enc = encode_to_vec(&ch).expect("encode 'a'");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode 'a'");
    assert_eq!(val, ch);
}

// 2. ASCII 'z' roundtrip
#[test]
fn test_char_ascii_lowercase_z_roundtrip() {
    let ch = 'z';
    let enc = encode_to_vec(&ch).expect("encode 'z'");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode 'z'");
    assert_eq!(val, ch);
}

// 3. ASCII '0' roundtrip
#[test]
fn test_char_ascii_digit_zero_roundtrip() {
    let ch = '0';
    let enc = encode_to_vec(&ch).expect("encode '0'");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode '0'");
    assert_eq!(val, ch);
}

// 4. ASCII ' ' (space) roundtrip
#[test]
fn test_char_ascii_space_roundtrip() {
    let ch = ' ';
    let enc = encode_to_vec(&ch).expect("encode space");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode space");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x20);
}

// 5. Null char '\0' roundtrip
#[test]
fn test_char_null_roundtrip() {
    let ch = '\0';
    let enc = encode_to_vec(&ch).expect("encode null char");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode null char");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x00);
}

// 6. Max ASCII '\x7F' (DEL) roundtrip
#[test]
fn test_char_max_ascii_del_roundtrip() {
    let ch = '\x7F';
    let enc = encode_to_vec(&ch).expect("encode DEL (0x7F)");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode DEL (0x7F)");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x7F);
}

// 7. Unicode 'é' (U+00E9) roundtrip
#[test]
fn test_char_unicode_e_acute_u00e9_roundtrip() {
    let ch = 'é'; // U+00E9
    let enc = encode_to_vec(&ch).expect("encode 'é' U+00E9");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode 'é' U+00E9");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x00E9);
}

// 8. Unicode 'ñ' (U+00F1) roundtrip
#[test]
fn test_char_unicode_n_tilde_u00f1_roundtrip() {
    let ch = 'ñ'; // U+00F1
    let enc = encode_to_vec(&ch).expect("encode 'ñ' U+00F1");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode 'ñ' U+00F1");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x00F1);
}

// 9. Unicode '中' (Chinese, U+4E2D) roundtrip
#[test]
fn test_char_unicode_cjk_zhong_u4e2d_roundtrip() {
    let ch = '中'; // U+4E2D
    let enc = encode_to_vec(&ch).expect("encode '中' U+4E2D");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode '中' U+4E2D");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x4E2D);
}

// 10. Unicode '日' (Japanese, U+65E5) roundtrip
#[test]
fn test_char_unicode_cjk_nichi_u65e5_roundtrip() {
    let ch = '日'; // U+65E5
    let enc = encode_to_vec(&ch).expect("encode '日' U+65E5");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode '日' U+65E5");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x65E5);
}

// 11. Unicode '😀' (grinning face, U+1F600) roundtrip
#[test]
fn test_char_unicode_grinning_face_u1f600_roundtrip() {
    let ch = '😀'; // U+1F600
    let enc = encode_to_vec(&ch).expect("encode '😀' U+1F600");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode '😀' U+1F600");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x1F600);
}

// 12. Unicode '🦀' (crab emoji, U+1F980) roundtrip
#[test]
fn test_char_unicode_crab_u1f980_roundtrip() {
    let ch = '🦀'; // U+1F980
    let enc = encode_to_vec(&ch).expect("encode '🦀' U+1F980");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode '🦀' U+1F980");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x1F980);
}

// 13. Unicode max char '\u{10FFFF}' roundtrip
#[test]
fn test_char_unicode_max_u10ffff_roundtrip() {
    let ch = '\u{10FFFF}';
    let enc = encode_to_vec(&ch).expect("encode U+10FFFF");
    let (val, _): (char, usize) = decode_from_slice(&enc).expect("decode U+10FFFF");
    assert_eq!(val, ch);
    assert_eq!(ch as u32, 0x10FFFF);
}

// 14. Vec<char> roundtrip with ASCII chars only
#[test]
fn test_vec_char_ascii_only_roundtrip() {
    let chars: Vec<char> = vec!['a', 'b', 'c', 'x', 'y', 'z', '0', '9'];
    let enc = encode_to_vec(&chars).expect("encode Vec<char> ASCII");
    let (val, _): (Vec<char>, usize) = decode_from_slice(&enc).expect("decode Vec<char> ASCII");
    assert_eq!(val, chars);
}

// 15. Vec<char> roundtrip with mixed ASCII and Unicode
#[test]
fn test_vec_char_mixed_ascii_unicode_roundtrip() {
    let chars: Vec<char> = vec!['A', 'é', '中', '😀', 'z', '🦀', '\0', '\u{10FFFF}'];
    let enc = encode_to_vec(&chars).expect("encode Vec<char> mixed");
    let (val, _): (Vec<char>, usize) = decode_from_slice(&enc).expect("decode Vec<char> mixed");
    assert_eq!(val, chars);
}

// 16. Option<char> Some('A') roundtrip
#[test]
fn test_option_char_some_capital_a_roundtrip() {
    let opt: Option<char> = Some('A');
    let enc = encode_to_vec(&opt).expect("encode Some('A')");
    let (val, _): (Option<char>, usize) = decode_from_slice(&enc).expect("decode Some('A')");
    assert_eq!(val, opt);
}

// 17. Option<char> None roundtrip
#[test]
fn test_option_char_none_roundtrip() {
    let opt: Option<char> = None;
    let enc = encode_to_vec(&opt).expect("encode None::<char>");
    let (val, _): (Option<char>, usize) = decode_from_slice(&enc).expect("decode None::<char>");
    assert_eq!(val, opt);
}

// 18. Struct containing char field roundtrip
#[derive(Debug, PartialEq, Encode, Decode)]
struct CharHolder {
    label: char,
    value: u32,
}

#[test]
fn test_struct_with_char_field_roundtrip() {
    let holder = CharHolder {
        label: '🦀',
        value: 42,
    };
    let enc = encode_to_vec(&holder).expect("encode CharHolder");
    let (val, consumed): (CharHolder, usize) = decode_from_slice(&enc).expect("decode CharHolder");
    assert_eq!(val, holder);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

// 19. char encoding produces same bytes as u32 encoding for the same code point
// Chars encode as their u32 code point (varint). Verify 'A' (0x41) == encode(41u32).
#[test]
fn test_char_same_bytes_as_u32_codepoint() {
    let char_enc = encode_to_vec(&'A').expect("encode 'A' as char");
    let u32_enc = encode_to_vec(&(b'A' as u32)).expect("encode 'A' code point as u32");
    assert_eq!(
        char_enc, u32_enc,
        "char 'A' must encode identically to u32 code point 65"
    );
}

// 20. Supplementary plane chars (U+10000..=U+10FFFF) always encode as exactly 4 UTF-8 bytes
#[test]
fn test_char_supplementary_plane_encodes_four_bytes() {
    // Chars in U+10000..=U+10FFFF use 4-byte UTF-8 encoding regardless of config
    let chars: &[char] = &['😀', '🦀', '\u{10000}', '\u{10FFFF}', '\u{1D11E}'];
    for &ch in chars {
        let enc = encode_to_vec(&ch).expect("encode supplementary plane char");
        assert_eq!(
            enc.len(),
            4,
            "char {:?} (U+{:04X}) must encode as exactly 4 UTF-8 bytes",
            ch,
            ch as u32
        );
        // Also verify roundtrip with fixed_int config (char encoding is always UTF-8)
        let cfg = config::standard().with_fixed_int_encoding();
        let enc_fi = encode_to_vec_with_config(&ch, cfg)
            .expect("encode supplementary plane char with fixed_int");
        assert_eq!(enc_fi.len(), 4,
            "char {:?} must still be 4 bytes with fixed_int config (UTF-8 is independent of int config)",
            ch
        );
        let (val, _): (char, usize) =
            decode_from_slice_with_config(&enc_fi, cfg).expect("decode with fixed_int config");
        assert_eq!(val, ch);
    }
}

// 21. '\n' (newline, 0x0A) and '\t' (tab, 0x09) roundtrip
#[test]
fn test_char_newline_and_tab_roundtrip() {
    let newline = '\n';
    let enc_n = encode_to_vec(&newline).expect("encode newline");
    let (val_n, _): (char, usize) = decode_from_slice(&enc_n).expect("decode newline");
    assert_eq!(val_n, newline);
    assert_eq!(newline as u32, 0x0A);

    let tab = '\t';
    let enc_t = encode_to_vec(&tab).expect("encode tab");
    let (val_t, _): (char, usize) = decode_from_slice(&enc_t).expect("decode tab");
    assert_eq!(val_t, tab);
    assert_eq!(tab as u32, 0x09);
}

// 22. consumed bytes == encoded length for various chars
#[test]
fn test_char_consumed_equals_encoded_length() {
    let chars = [
        'a',
        'é',
        '中',
        '😀',
        '\0',
        '\x7F',
        '\u{10FFFF}',
        '🦀',
        'ñ',
        '日',
    ];
    for ch in chars {
        let enc = encode_to_vec(&ch).expect("encode char for consumed check");
        let (_, consumed): (char, usize) =
            decode_from_slice(&enc).expect("decode char for consumed check");
        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes must equal encoded length for char {:?} (U+{:04X})",
            ch,
            ch as u32
        );
    }
}
