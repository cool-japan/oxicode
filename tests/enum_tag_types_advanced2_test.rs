//! Advanced tests for OxiCode derive macro `#[oxicode(tag_type = "...")]` container attribute.
//!
//! Covers discriminant width variants (u8, u16, u32), data-carrying variants, config interactions,
//! collection wrappers, and byte-level encoding properties.

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

// ---------------------------------------------------------------------------
// Top-level enum definitions
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, Debug, PartialEq)]
#[oxicode(tag_type = "u8")]
enum SmallTag {
    A,
    B,
    C,
}

#[derive(Encode, Decode, Debug, PartialEq)]
#[oxicode(tag_type = "u16")]
enum MediumTag {
    First,
    Second,
    Third,
}

#[derive(Encode, Decode, Debug, PartialEq)]
#[oxicode(tag_type = "u32")]
enum LargeTag {
    One,
    Two,
    Three,
}

#[derive(Encode, Decode, Debug, PartialEq)]
#[oxicode(tag_type = "u8")]
enum TaggedPayload {
    Empty,
    WithU32(u32),
    WithString(String),
    WithTuple(u32, u64),
}

// ---------------------------------------------------------------------------
// Test 1: SmallTag::A roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_a_roundtrip() {
    let original = SmallTag::A;
    let enc = encode_to_vec(&original).expect("encode SmallTag::A");
    let (decoded, _): (SmallTag, usize) = decode_from_slice(&enc).expect("decode SmallTag::A");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 2: SmallTag::B roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_b_roundtrip() {
    let original = SmallTag::B;
    let enc = encode_to_vec(&original).expect("encode SmallTag::B");
    let (decoded, _): (SmallTag, usize) = decode_from_slice(&enc).expect("decode SmallTag::B");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 3: SmallTag::C roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_c_roundtrip() {
    let original = SmallTag::C;
    let enc = encode_to_vec(&original).expect("encode SmallTag::C");
    let (decoded, _): (SmallTag, usize) = decode_from_slice(&enc).expect("decode SmallTag::C");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 4: SmallTag u8 discriminant size with fixed_int config
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_u8_discriminant_size() {
    let cfg = config::legacy();
    let enc_a = encode_to_vec_with_config(&SmallTag::A, cfg).expect("encode SmallTag::A fixed");
    assert_eq!(
        enc_a.len(),
        1,
        "u8 tag_type with fixed-int config must be 1 byte, got {:?}",
        enc_a
    );
    assert_eq!(enc_a[0], 0u8, "discriminant of A must be 0");

    let enc_b = encode_to_vec_with_config(&SmallTag::B, cfg).expect("encode SmallTag::B fixed");
    assert_eq!(enc_b.len(), 1);
    assert_eq!(enc_b[0], 1u8, "discriminant of B must be 1");

    let enc_c = encode_to_vec_with_config(&SmallTag::C, cfg).expect("encode SmallTag::C fixed");
    assert_eq!(enc_c.len(), 1);
    assert_eq!(enc_c[0], 2u8, "discriminant of C must be 2");
}

// ---------------------------------------------------------------------------
// Test 5: MediumTag roundtrip (u16 discriminant)
// ---------------------------------------------------------------------------

#[test]
fn test_medium_tag_roundtrip() {
    for variant in [MediumTag::First, MediumTag::Second, MediumTag::Third] {
        let enc = encode_to_vec(&variant).expect("encode MediumTag");
        let (decoded, _): (MediumTag, usize) = decode_from_slice(&enc).expect("decode MediumTag");
        assert_eq!(decoded, variant);
    }
}

// ---------------------------------------------------------------------------
// Test 6: LargeTag roundtrip (u32 discriminant)
// ---------------------------------------------------------------------------

#[test]
fn test_large_tag_roundtrip() {
    for variant in [LargeTag::One, LargeTag::Two, LargeTag::Three] {
        let enc = encode_to_vec(&variant).expect("encode LargeTag");
        let (decoded, _): (LargeTag, usize) = decode_from_slice(&enc).expect("decode LargeTag");
        assert_eq!(decoded, variant);
    }
}

// ---------------------------------------------------------------------------
// Test 7: TaggedPayload::Empty roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_payload_empty_roundtrip() {
    let original = TaggedPayload::Empty;
    let enc = encode_to_vec(&original).expect("encode TaggedPayload::Empty");
    let (decoded, _): (TaggedPayload, usize) =
        decode_from_slice(&enc).expect("decode TaggedPayload::Empty");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 8: TaggedPayload::WithU32 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_payload_with_u32_roundtrip() {
    let original = TaggedPayload::WithU32(42);
    let enc = encode_to_vec(&original).expect("encode TaggedPayload::WithU32");
    let (decoded, _): (TaggedPayload, usize) =
        decode_from_slice(&enc).expect("decode TaggedPayload::WithU32");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 9: TaggedPayload::WithString roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_payload_with_string_roundtrip() {
    let original = TaggedPayload::WithString("hello".to_string());
    let enc = encode_to_vec(&original).expect("encode TaggedPayload::WithString");
    let (decoded, _): (TaggedPayload, usize) =
        decode_from_slice(&enc).expect("decode TaggedPayload::WithString");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 10: TaggedPayload::WithTuple roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_payload_with_tuple_roundtrip() {
    let original = TaggedPayload::WithTuple(1, 2);
    let enc = encode_to_vec(&original).expect("encode TaggedPayload::WithTuple");
    let (decoded, _): (TaggedPayload, usize) =
        decode_from_slice(&enc).expect("decode TaggedPayload::WithTuple");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 11: SmallTag — bytes consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_consumed_equals_len() {
    for variant in [SmallTag::A, SmallTag::B, SmallTag::C] {
        let enc = encode_to_vec(&variant).expect("encode SmallTag for consumed check");
        let (_, consumed): (SmallTag, usize) =
            decode_from_slice(&enc).expect("decode SmallTag for consumed check");
        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes must equal encoded length for {:?}",
            variant
        );
    }
}

// ---------------------------------------------------------------------------
// Test 12: TaggedPayload — bytes consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_payload_consumed_equals_len() {
    let variants = vec![
        TaggedPayload::Empty,
        TaggedPayload::WithU32(999),
        TaggedPayload::WithString("world".to_string()),
        TaggedPayload::WithTuple(100, 200),
    ];
    for variant in &variants {
        let enc = encode_to_vec(variant).expect("encode TaggedPayload for consumed check");
        let (_, consumed): (TaggedPayload, usize) =
            decode_from_slice(&enc).expect("decode TaggedPayload for consumed check");
        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes must equal encoded length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 13: Vec<SmallTag> roundtrip with all variants
// ---------------------------------------------------------------------------

#[test]
fn test_vec_small_tag_roundtrip() {
    let original = vec![SmallTag::A, SmallTag::B, SmallTag::C, SmallTag::A];
    let enc = encode_to_vec(&original).expect("encode Vec<SmallTag>");
    let (decoded, _): (Vec<SmallTag>, usize) =
        decode_from_slice(&enc).expect("decode Vec<SmallTag>");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 14: Option<SmallTag> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_small_tag_some_roundtrip() {
    let original: Option<SmallTag> = Some(SmallTag::B);
    let enc = encode_to_vec(&original).expect("encode Option<SmallTag> Some");
    let (decoded, _): (Option<SmallTag>, usize) =
        decode_from_slice(&enc).expect("decode Option<SmallTag> Some");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 15: Option<SmallTag> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_small_tag_none_roundtrip() {
    let original: Option<SmallTag> = None;
    let enc = encode_to_vec(&original).expect("encode Option<SmallTag> None");
    let (decoded, _): (Option<SmallTag>, usize) =
        decode_from_slice(&enc).expect("decode Option<SmallTag> None");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 16: SmallTag — A, B, C produce different encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_different_variants_different_bytes() {
    let enc_a = encode_to_vec(&SmallTag::A).expect("encode SmallTag::A");
    let enc_b = encode_to_vec(&SmallTag::B).expect("encode SmallTag::B");
    let enc_c = encode_to_vec(&SmallTag::C).expect("encode SmallTag::C");
    assert_ne!(enc_a, enc_b, "A and B must encode differently");
    assert_ne!(enc_b, enc_c, "B and C must encode differently");
    assert_ne!(enc_a, enc_c, "A and C must encode differently");
}

// ---------------------------------------------------------------------------
// Test 17: MediumTag — First, Second, Third produce different encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_medium_tag_different_variants_different_bytes() {
    let enc_first = encode_to_vec(&MediumTag::First).expect("encode MediumTag::First");
    let enc_second = encode_to_vec(&MediumTag::Second).expect("encode MediumTag::Second");
    let enc_third = encode_to_vec(&MediumTag::Third).expect("encode MediumTag::Third");
    assert_ne!(
        enc_first, enc_second,
        "First and Second must encode differently"
    );
    assert_ne!(
        enc_second, enc_third,
        "Second and Third must encode differently"
    );
    assert_ne!(
        enc_first, enc_third,
        "First and Third must encode differently"
    );
}

// ---------------------------------------------------------------------------
// Test 18: TaggedPayload — all 4 variants produce different encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_payload_all_variants_different() {
    let enc_empty = encode_to_vec(&TaggedPayload::Empty).expect("encode Empty");
    let enc_u32 = encode_to_vec(&TaggedPayload::WithU32(0)).expect("encode WithU32");
    let enc_str =
        encode_to_vec(&TaggedPayload::WithString(String::new())).expect("encode WithString");
    let enc_tup = encode_to_vec(&TaggedPayload::WithTuple(0, 0)).expect("encode WithTuple");

    assert_ne!(
        enc_empty, enc_u32,
        "Empty and WithU32 must encode differently"
    );
    assert_ne!(
        enc_empty, enc_str,
        "Empty and WithString must encode differently"
    );
    assert_ne!(
        enc_empty, enc_tup,
        "Empty and WithTuple must encode differently"
    );
    assert_ne!(
        enc_u32, enc_str,
        "WithU32 and WithString must encode differently"
    );
    assert_ne!(
        enc_u32, enc_tup,
        "WithU32 and WithTuple must encode differently"
    );
    assert_ne!(
        enc_str, enc_tup,
        "WithString and WithTuple must encode differently"
    );
}

// ---------------------------------------------------------------------------
// Test 19: SmallTag with fixed_int_encoding config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    for variant in [SmallTag::A, SmallTag::B, SmallTag::C] {
        let enc = encode_to_vec_with_config(&variant, cfg).expect("encode SmallTag fixed_int");
        let (decoded, _): (SmallTag, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode SmallTag fixed_int");
        assert_eq!(decoded, variant);
    }
}

// ---------------------------------------------------------------------------
// Test 20: TaggedPayload with big_endian config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_payload_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let variants = vec![
        TaggedPayload::Empty,
        TaggedPayload::WithU32(12345),
        TaggedPayload::WithString("oxicode".to_string()),
        TaggedPayload::WithTuple(9999, 888888),
    ];
    for variant in &variants {
        let enc = encode_to_vec_with_config(variant, cfg).expect("encode TaggedPayload big_endian");
        let (decoded, _): (TaggedPayload, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode TaggedPayload big_endian");
        assert_eq!(&decoded, variant);
    }
}

// ---------------------------------------------------------------------------
// Test 21: Vec<TaggedPayload> with all 4 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_tagged_payload_all_variants() {
    let original = vec![
        TaggedPayload::Empty,
        TaggedPayload::WithU32(777),
        TaggedPayload::WithString("test_string".to_string()),
        TaggedPayload::WithTuple(42, 9876543210),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<TaggedPayload>");
    let (decoded, _): (Vec<TaggedPayload>, usize) =
        decode_from_slice(&enc).expect("decode Vec<TaggedPayload>");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 22: SmallTag same variant encodes to same bytes consistently
// ---------------------------------------------------------------------------

#[test]
fn test_small_tag_encode_twice_consistent() {
    for variant in [SmallTag::A, SmallTag::B, SmallTag::C] {
        let enc1 = encode_to_vec(&variant).expect("first encode SmallTag");
        let enc2 = encode_to_vec(&variant).expect("second encode SmallTag");
        assert_eq!(
            enc1, enc2,
            "encoding {:?} twice must produce identical bytes",
            variant
        );
    }
}
