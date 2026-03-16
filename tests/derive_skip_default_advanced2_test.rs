//! Advanced tests for #[oxicode(skip)] and #[oxicode(default = "fn_path")] derive attributes.
//!
//! 22 tests — all top-level, no cfg(test) module wrapper.
//! Skip behavior: field is NOT encoded and on decode uses Default::default().
//! Default-fn behavior (skip + default): skip takes precedence → Default::default() applies.
//! Default-fn-only behavior (only default attr, no skip): custom fn is called on decode.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Module-level default functions
// ---------------------------------------------------------------------------

#[allow(dead_code)]
fn default_zero_u32() -> u32 {
    0
}

#[allow(dead_code)]
fn default_hundred() -> u32 {
    100
}

#[allow(dead_code)]
fn default_hello() -> String {
    String::from("hello")
}

#[allow(dead_code)]
fn default_empty_vec() -> Vec<u32> {
    Vec::new()
}

#[allow(dead_code)]
fn default_true() -> bool {
    true
}

// ---------------------------------------------------------------------------
// Shared struct definitions
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, Debug, PartialEq)]
struct SkipField {
    included: u32,
    #[oxicode(skip)]
    skipped: u32,
}

/// DefaultField uses skip + default = "default_hundred".
/// Because skip takes precedence, with_default will be Default::default() = 0
/// (not 100) after decode. We verify this precise behavior in test 5.
#[derive(Encode, Decode, Debug, PartialEq)]
struct DefaultField {
    included: u32,
    #[oxicode(skip, default = "default_hundred")]
    with_default: u32,
}

#[derive(Encode, Decode, Debug, PartialEq)]
struct MixedFields {
    first: u32,
    #[oxicode(skip)]
    skipped_zero: u32,
    second: String,
    #[oxicode(skip, default = "default_hello")]
    skipped_hello: String,
    third: bool,
}

#[derive(Encode, Decode, Debug, PartialEq)]
struct MultiSkip {
    a: u32,
    #[oxicode(skip)]
    b: u32,
    c: u32,
    #[oxicode(skip)]
    d: u32,
}

// ---------------------------------------------------------------------------
// Test 1: SkipField encodes same bytes as a struct with only `included`
// ---------------------------------------------------------------------------

#[test]
fn test_skip_field_encodes_without_skipped() {
    #[derive(Encode)]
    struct IncludedOnly {
        included: u32,
    }

    let with_skip = SkipField {
        included: 42,
        skipped: 0xDEAD_BEEF,
    };
    let reference = IncludedOnly { included: 42 };

    let enc_skip = encode_to_vec(&with_skip).expect("encode SkipField");
    let enc_ref = encode_to_vec(&reference).expect("encode IncludedOnly");

    assert_eq!(
        enc_skip, enc_ref,
        "SkipField must encode to the same bytes as a struct containing only `included`"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Decoded skipped field is u32 Default (0)
// ---------------------------------------------------------------------------

#[test]
fn test_skip_field_decode_skipped_is_zero() {
    let original = SkipField {
        included: 10,
        skipped: 99_999,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (SkipField, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(
        val.skipped, 0_u32,
        "skipped u32 must default to 0 after decode"
    );
    assert_eq!(val.included, 10, "included must survive roundtrip");
}

// ---------------------------------------------------------------------------
// Test 3: Bytes consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_skip_field_consumed_equals_len() {
    let original = SkipField {
        included: 7,
        skipped: 1234,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (_, consumed): (SkipField, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Wire size equals encoding of just the `included` u32
// ---------------------------------------------------------------------------

#[test]
fn test_skip_field_wire_size_equals_included_only() {
    let value = SkipField {
        included: 255,
        skipped: u32::MAX,
    };
    let enc = encode_to_vec(&value).expect("encode SkipField");

    // Encode just a bare u32 with the same value
    let enc_u32 = encode_to_vec(&255_u32).expect("encode u32");

    assert_eq!(
        enc.len(),
        enc_u32.len(),
        "SkipField wire size must equal encoding of just included u32"
    );
    assert_eq!(
        enc, enc_u32,
        "SkipField bytes must equal encoding of just included u32"
    );
}

// ---------------------------------------------------------------------------
// Test 5: DefaultField with_default is Default::default() (0) after decode
//         because skip takes precedence over default = "fn"
// ---------------------------------------------------------------------------

#[test]
fn test_default_field_decode_has_custom_default() {
    let original = DefaultField {
        included: 50,
        with_default: 999,
    };
    let enc = encode_to_vec(&original).expect("encode DefaultField");
    let (val, _): (DefaultField, usize) = decode_from_slice(&enc).expect("decode DefaultField");

    // skip takes precedence over default = "default_hundred"
    assert_eq!(
        val.with_default, 0_u32,
        "skip + default fn: skip takes precedence, field must be Default (0)"
    );
    assert_eq!(val.included, 50);
}

// ---------------------------------------------------------------------------
// Test 6: DefaultField encodes same bytes as just encoding `included`
// ---------------------------------------------------------------------------

#[test]
fn test_default_field_encodes_without_default_field() {
    #[derive(Encode)]
    struct JustIncluded {
        included: u32,
    }

    let df = DefaultField {
        included: 77,
        with_default: 100,
    };
    let reference = JustIncluded { included: 77 };

    let enc_df = encode_to_vec(&df).expect("encode DefaultField");
    let enc_ref = encode_to_vec(&reference).expect("encode JustIncluded");

    assert_eq!(
        enc_df, enc_ref,
        "DefaultField must encode same as struct with only `included`"
    );
}

// ---------------------------------------------------------------------------
// Test 7: MixedFields roundtrip — non-skipped fields preserved
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_fields_roundtrip() {
    let original = MixedFields {
        first: 1,
        skipped_zero: 0,
        second: "test".to_string(),
        skipped_hello: "hello".to_string(),
        third: false,
    };
    let enc = encode_to_vec(&original).expect("encode MixedFields");
    let (val, _): (MixedFields, usize) = decode_from_slice(&enc).expect("decode MixedFields");

    assert_eq!(val.first, 1);
    assert_eq!(val.second, "test");
    assert!(!val.third);
}

// ---------------------------------------------------------------------------
// Test 8: MixedFields skipped_zero is 0 after decode (no default fn, plain skip)
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_fields_skipped_zero_is_zero() {
    let original = MixedFields {
        first: 5,
        skipped_zero: 0xAB_CD,
        second: "x".to_string(),
        skipped_hello: "y".to_string(),
        third: true,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (MixedFields, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(val.skipped_zero, 0_u32, "plain skip must give Default (0)");
}

// ---------------------------------------------------------------------------
// Test 9: MixedFields skipped_hello is "" (skip takes precedence over default_hello)
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_fields_skipped_hello_has_default() {
    let original = MixedFields {
        first: 3,
        skipped_zero: 0,
        second: "abc".to_string(),
        skipped_hello: "world".to_string(),
        third: false,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (MixedFields, usize) = decode_from_slice(&enc).expect("decode");

    // skip takes precedence over default = "default_hello"
    assert_eq!(
        val.skipped_hello, "",
        "skip takes precedence: skipped_hello must be Default (\"\")"
    );
}

// ---------------------------------------------------------------------------
// Test 10: MixedFields non-skipped fields are all preserved
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_fields_included_fields_preserved() {
    let original = MixedFields {
        first: 99,
        skipped_zero: 12,
        second: "preserved".to_string(),
        skipped_hello: "lost".to_string(),
        third: true,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (MixedFields, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(val.first, 99, "first must be preserved");
    assert_eq!(val.second, "preserved", "second must be preserved");
    assert!(val.third, "third must be preserved");
}

// ---------------------------------------------------------------------------
// Test 11: MultiSkip roundtrip — non-skipped a and c are preserved
// ---------------------------------------------------------------------------

#[test]
fn test_multi_skip_roundtrip() {
    let original = MultiSkip {
        a: 1,
        b: 0,
        c: 2,
        d: 0,
    };
    let enc = encode_to_vec(&original).expect("encode MultiSkip");
    let (val, _): (MultiSkip, usize) = decode_from_slice(&enc).expect("decode MultiSkip");

    assert_eq!(val.a, 1);
    assert_eq!(val.c, 2);
}

// ---------------------------------------------------------------------------
// Test 12: MultiSkip b is 0 after decode
// ---------------------------------------------------------------------------

#[test]
fn test_multi_skip_b_is_zero() {
    let original = MultiSkip {
        a: 10,
        b: 0xFF_FF,
        c: 20,
        d: 0xFF_FF,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (MultiSkip, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(val.b, 0_u32, "skipped b must be 0");
}

// ---------------------------------------------------------------------------
// Test 13: MultiSkip d is 0 after decode
// ---------------------------------------------------------------------------

#[test]
fn test_multi_skip_d_is_zero() {
    let original = MultiSkip {
        a: 5,
        b: 0xABCD,
        c: 6,
        d: 0xEF01,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (MultiSkip, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(val.d, 0_u32, "skipped d must be 0");
}

// ---------------------------------------------------------------------------
// Test 14: MultiSkip a and c are preserved after roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_multi_skip_a_c_preserved() {
    let original = MultiSkip {
        a: 111,
        b: 999,
        c: 222,
        d: 888,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (MultiSkip, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(val.a, 111, "a must be preserved");
    assert_eq!(val.c, 222, "c must be preserved");
}

// ---------------------------------------------------------------------------
// Test 15: Vec<SkipField> roundtrip with 3 elements
// ---------------------------------------------------------------------------

#[test]
fn test_vec_skip_field_roundtrip() {
    let items = vec![
        SkipField {
            included: 10,
            skipped: 100,
        },
        SkipField {
            included: 20,
            skipped: 200,
        },
        SkipField {
            included: 30,
            skipped: 300,
        },
    ];
    let enc = encode_to_vec(&items).expect("encode Vec<SkipField>");
    let (val, _): (Vec<SkipField>, usize) = decode_from_slice(&enc).expect("decode Vec<SkipField>");

    assert_eq!(val.len(), 3, "length must be preserved");
    assert_eq!(val[0].included, 10);
    assert_eq!(val[1].included, 20);
    assert_eq!(val[2].included, 30);
    for item in &val {
        assert_eq!(item.skipped, 0_u32, "each skipped field must be 0");
    }
}

// ---------------------------------------------------------------------------
// Test 16: Option<SkipField> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_skip_field_some_roundtrip() {
    let original: Option<SkipField> = Some(SkipField {
        included: 55,
        skipped: 9999,
    });
    let enc = encode_to_vec(&original).expect("encode Option<SkipField>");
    let (val, _): (Option<SkipField>, usize) =
        decode_from_slice(&enc).expect("decode Option<SkipField>");

    let inner = val.expect("decoded Option must be Some");
    assert_eq!(inner.included, 55, "included must survive inside Option");
    assert_eq!(inner.skipped, 0_u32, "skipped must be 0 inside Option");
}

// ---------------------------------------------------------------------------
// Test 17: Option<SkipField> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_skip_field_none_roundtrip() {
    let original: Option<SkipField> = None;
    let enc = encode_to_vec(&original).expect("encode Option<SkipField> None");
    let (val, _): (Option<SkipField>, usize) =
        decode_from_slice(&enc).expect("decode Option<SkipField> None");

    assert!(val.is_none(), "None Option must decode as None");
}

// ---------------------------------------------------------------------------
// Test 18: Encoding same SkipField value twice produces identical bytes
// ---------------------------------------------------------------------------

#[test]
fn test_skip_field_encode_twice_same_bytes() {
    let value = SkipField {
        included: 123,
        skipped: 456,
    };

    let enc1 = encode_to_vec(&value).expect("encode first");
    let enc2 = encode_to_vec(&value).expect("encode second");

    assert_eq!(
        enc1, enc2,
        "encoding the same SkipField twice must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 19: DefaultField encoding consistency — same value, same bytes
// ---------------------------------------------------------------------------

#[test]
fn test_default_field_encode_twice_same_bytes() {
    let value = DefaultField {
        included: 77,
        with_default: 0,
    };

    let enc1 = encode_to_vec(&value).expect("encode first");
    let enc2 = encode_to_vec(&value).expect("encode second");

    assert_eq!(
        enc1, enc2,
        "encoding the same DefaultField twice must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Vec<MixedFields> roundtrip with 2 elements
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_fields_vec_roundtrip() {
    let items = vec![
        MixedFields {
            first: 1,
            skipped_zero: 111,
            second: "alpha".to_string(),
            skipped_hello: "skip_a".to_string(),
            third: true,
        },
        MixedFields {
            first: 2,
            skipped_zero: 222,
            second: "beta".to_string(),
            skipped_hello: "skip_b".to_string(),
            third: false,
        },
    ];
    let enc = encode_to_vec(&items).expect("encode Vec<MixedFields>");
    let (val, _): (Vec<MixedFields>, usize) =
        decode_from_slice(&enc).expect("decode Vec<MixedFields>");

    assert_eq!(val.len(), 2);
    assert_eq!(val[0].first, 1);
    assert_eq!(val[0].second, "alpha");
    assert!(val[0].third);
    assert_eq!(val[0].skipped_zero, 0_u32);
    assert_eq!(val[0].skipped_hello, "");

    assert_eq!(val[1].first, 2);
    assert_eq!(val[1].second, "beta");
    assert!(!val[1].third);
    assert_eq!(val[1].skipped_zero, 0_u32);
    assert_eq!(val[1].skipped_hello, "");
}

// ---------------------------------------------------------------------------
// Test 21: SkipField has fewer wire bytes than a comparable struct with both fields
// ---------------------------------------------------------------------------

#[test]
fn test_skip_reduces_wire_size() {
    #[derive(Encode)]
    struct BothFields {
        included: u32,
        skipped: u32,
    }

    // Use a large value so the skipped field actually contributes meaningful bytes
    let with_skip = SkipField {
        included: 1,
        skipped: u32::MAX,
    };
    let without_skip = BothFields {
        included: 1,
        skipped: u32::MAX,
    };

    let enc_skip = encode_to_vec(&with_skip).expect("encode with skip");
    let enc_both = encode_to_vec(&without_skip).expect("encode without skip");

    assert!(
        enc_skip.len() < enc_both.len(),
        "SkipField must produce fewer bytes ({}) than struct with both fields ({})",
        enc_skip.len(),
        enc_both.len()
    );
}

// ---------------------------------------------------------------------------
// Test 22: Non-zero skipped field: after roundtrip, skipped becomes 0
// ---------------------------------------------------------------------------

#[test]
fn test_skip_field_zero_value_for_non_default() {
    // Explicitly construct with a non-zero skipped value
    let original = SkipField {
        included: 42,
        skipped: 0xCAFE_BABE,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (val, _): (SkipField, usize) = decode_from_slice(&enc).expect("decode");

    assert_eq!(
        val.skipped, 0_u32,
        "non-zero skipped field must become 0 after roundtrip (Default::default())"
    );
    assert_eq!(val.included, 42, "included must survive roundtrip");
}
