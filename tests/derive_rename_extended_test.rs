//! Extended tests for rename and rename_all derive attributes in OxiCode.
//!
//! All tests verify that `#[oxicode(rename = "...")]` and
//! `#[oxicode(rename_all = "...")]` are accepted by the derive macros and that
//! the binary format remains positional (field names do not appear on the wire).
//!
//! Coverage:
//! 1.  Single renamed field roundtrip
//! 2.  Renamed field produces same bytes as non-renamed field (positional wire)
//! 3.  Multiple renamed fields in one struct
//! 4.  All fields renamed, all-bytes-consumed verification
//! 5.  Enum variant with rename roundtrip
//! 6.  All enum variants renamed, discriminant ordering
//! 7.  rename + skip combined on the same struct
//! 8.  rename + default combined on the same struct
//! 9.  Nested struct with renamed fields (inner and outer)
//! 10. Roundtrip consistency: renamed vs non-renamed produce same bytes
//! 11. Enum discriminant values preserved with renamed variants
//! 12. Vec<StructWithRenamedFields> roundtrip
//! 13. Option<StructWithRenamedFields> Some roundtrip
//! 14. Option<StructWithRenamedFields> None roundtrip
//! 15. Generic struct with renamed field
//! 16. Tuple struct with renamed fields
//! 17. Unit struct (no fields) after rename_all on container
//! 18. rename_all = "camelCase" on struct fields
//! 19. rename_all = "SCREAMING_SNAKE_CASE" on struct fields
//! 20. rename_all on enum + individual rename overrides rename_all
//! 21. rename_all = "kebab-case" on struct fields
//! 22. Enum with mixed rename and non-rename variants, full roundtrip sweep

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
// Helper default functions used by attribute tests
// ---------------------------------------------------------------------------

fn default_zero_u32() -> u32 {
    0
}

// ---------------------------------------------------------------------------
// 1. Single renamed field roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SingleRename {
    #[oxicode(rename = "userId")]
    user_id: u64,
    score: u32,
}

#[test]
fn test_single_renamed_field_roundtrip() {
    let original = SingleRename {
        user_id: 1234567890,
        score: 42,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SingleRename, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.user_id, 1234567890);
    assert_eq!(dec.score, 42);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 2. Renamed field produces the same bytes as non-renamed (positional wire)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithRename {
    #[oxicode(rename = "myField")]
    my_field: u32,
    other: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithoutRename {
    my_field: u32,
    other: String,
}

#[test]
fn test_renamed_same_bytes_as_non_renamed() {
    let with_rename = WithRename {
        my_field: 99,
        other: "hello".to_string(),
    };
    let without_rename = WithoutRename {
        my_field: 99,
        other: "hello".to_string(),
    };
    let enc_renamed = encode_to_vec(&with_rename).expect("encode renamed");
    let enc_plain = encode_to_vec(&without_rename).expect("encode plain");
    assert_eq!(
        enc_renamed, enc_plain,
        "rename should not affect binary representation"
    );
}

// ---------------------------------------------------------------------------
// 3. Multiple renamed fields in one struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiRename {
    #[oxicode(rename = "firstName")]
    first_name: String,
    #[oxicode(rename = "lastName")]
    last_name: String,
    #[oxicode(rename = "emailAddress")]
    email: String,
    age: u8,
}

#[test]
fn test_multiple_renamed_fields_roundtrip() {
    let original = MultiRename {
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        email: "jane@example.com".to_string(),
        age: 30,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (MultiRename, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 4. All fields renamed, verify all bytes consumed
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllRenamed {
    #[oxicode(rename = "a")]
    field_a: i32,
    #[oxicode(rename = "b")]
    field_b: i32,
    #[oxicode(rename = "c")]
    field_c: i32,
}

#[test]
fn test_all_fields_renamed_bytes_consumed() {
    let original = AllRenamed {
        field_a: -1,
        field_b: 0,
        field_c: 1,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (AllRenamed, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len(), "all encoded bytes must be consumed");
}

// ---------------------------------------------------------------------------
// 5. Enum variant with rename roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum RenamedVariantEnum {
    #[oxicode(rename = "start_event")]
    Start {
        timestamp: u64,
    },
    #[oxicode(rename = "stop_event")]
    Stop,
    Pause(u32),
}

#[test]
fn test_enum_renamed_named_variant_roundtrip() {
    let original = RenamedVariantEnum::Start { timestamp: 99999 };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (RenamedVariantEnum, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

#[test]
fn test_enum_renamed_unit_variant_roundtrip() {
    let original = RenamedVariantEnum::Stop;
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (RenamedVariantEnum, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
}

// ---------------------------------------------------------------------------
// 6. All enum variants renamed, discriminant ordering preserved
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum AllVariantsRenamed {
    #[oxicode(rename = "first")]
    Alpha,
    #[oxicode(rename = "second")]
    Beta,
    #[oxicode(rename = "third")]
    Gamma,
}

#[test]
fn test_all_variants_renamed_discriminant_order() {
    // Encode each variant and check that different variants produce different bytes.
    let enc_a = encode_to_vec(&AllVariantsRenamed::Alpha).expect("encode Alpha");
    let enc_b = encode_to_vec(&AllVariantsRenamed::Beta).expect("encode Beta");
    let enc_g = encode_to_vec(&AllVariantsRenamed::Gamma).expect("encode Gamma");
    assert_ne!(
        enc_a, enc_b,
        "Alpha and Beta must have different discriminants"
    );
    assert_ne!(
        enc_b, enc_g,
        "Beta and Gamma must have different discriminants"
    );
    assert_ne!(
        enc_a, enc_g,
        "Alpha and Gamma must have different discriminants"
    );

    // Each must also roundtrip correctly.
    for (original, enc) in [
        (AllVariantsRenamed::Alpha, &enc_a),
        (AllVariantsRenamed::Beta, &enc_b),
        (AllVariantsRenamed::Gamma, &enc_g),
    ] {
        let (dec, _): (AllVariantsRenamed, _) = decode_from_slice(enc).expect("decode");
        assert_eq!(dec, original);
    }
}

// ---------------------------------------------------------------------------
// 7. rename + skip combined on the same struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameAndSkip {
    #[oxicode(rename = "primaryId")]
    primary_id: u32,
    #[oxicode(rename = "cache", skip)]
    cache: u64,
    payload: Vec<u8>,
}

#[test]
fn test_rename_and_skip_combined() {
    let original = RenameAndSkip {
        primary_id: 7,
        cache: 0xDEAD_BEEF_CAFE_BABE,
        payload: vec![10, 20, 30],
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (RenameAndSkip, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.primary_id, 7);
    assert_eq!(
        dec.cache, 0u64,
        "skipped field must decode as Default::default()"
    );
    assert_eq!(dec.payload, vec![10, 20, 30]);
    assert_eq!(bytes_read, enc.len());
}

#[test]
fn test_rename_and_skip_smaller_than_unskipped() {
    #[derive(Encode)]
    struct NoSkip {
        primary_id: u32,
        cache: u64,
        payload: Vec<u8>,
    }

    let with_skip = RenameAndSkip {
        primary_id: 1,
        cache: u64::MAX,
        payload: vec![1, 2, 3],
    };
    let no_skip = NoSkip {
        primary_id: 1,
        cache: u64::MAX,
        payload: vec![1, 2, 3],
    };
    let skip_len = encode_to_vec(&with_skip).expect("encode skip").len();
    let full_len = encode_to_vec(&no_skip).expect("encode no_skip").len();
    assert!(
        skip_len < full_len,
        "skipped renamed field should produce fewer bytes: {skip_len} vs {full_len}"
    );
}

// ---------------------------------------------------------------------------
// 8. rename + default combined on the same struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameAndDefault {
    name: String,
    #[oxicode(rename = "defaultScore", default = "default_zero_u32")]
    default_score: u32,
    active: bool,
}

#[test]
fn test_rename_and_default_combined() {
    let original = RenameAndDefault {
        name: "Tester".to_string(),
        default_score: 500, // not encoded; default_zero_u32() will be used
        active: true,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (RenameAndDefault, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.name, "Tester");
    assert_eq!(dec.default_score, 0, "default_zero_u32() should be applied");
    assert!(dec.active);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 9. Nested struct with renamed fields (inner and outer)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerRenamed {
    #[oxicode(rename = "xCoord")]
    x: f32,
    #[oxicode(rename = "yCoord")]
    y: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OuterWithInner {
    #[oxicode(rename = "label")]
    name: String,
    #[oxicode(rename = "position")]
    pos: InnerRenamed,
}

#[test]
fn test_nested_struct_renamed_fields_roundtrip() {
    let original = OuterWithInner {
        name: "point_A".to_string(),
        pos: InnerRenamed { x: 1.5, y: -2.5 },
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (OuterWithInner, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 10. Roundtrip consistency: renamed vs non-renamed produce identical bytes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct FullRenameStruct {
    #[oxicode(rename = "fieldOne")]
    field_one: u8,
    #[oxicode(rename = "fieldTwo")]
    field_two: u16,
    #[oxicode(rename = "fieldThree")]
    field_three: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FullPlainStruct {
    field_one: u8,
    field_two: u16,
    field_three: u32,
}

#[test]
fn test_renamed_struct_bytes_match_plain_struct() {
    let renamed = FullRenameStruct {
        field_one: 1,
        field_two: 2,
        field_three: 3,
    };
    let plain = FullPlainStruct {
        field_one: 1,
        field_two: 2,
        field_three: 3,
    };
    let enc_renamed = encode_to_vec(&renamed).expect("encode renamed");
    let enc_plain = encode_to_vec(&plain).expect("encode plain");
    assert_eq!(
        enc_renamed, enc_plain,
        "rename attributes must not alter wire bytes"
    );
}

// ---------------------------------------------------------------------------
// 11. Enum discriminant values are preserved with renamed variants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum RenamedTaggedEnum {
    #[oxicode(variant = 5, rename = "alpha_event")]
    Alpha,
    #[oxicode(variant = 15, rename = "beta_event")]
    Beta(u32),
    #[oxicode(variant = 25, rename = "gamma_event")]
    Gamma { value: i64 },
}

#[test]
fn test_renamed_enum_discriminant_values_preserved() {
    // With legacy (fixed-int) config u8 discriminant is written as a single byte.
    let enc_alpha =
        oxicode::encode_to_vec_with_config(&RenamedTaggedEnum::Alpha, oxicode::config::legacy())
            .expect("encode Alpha");
    assert_eq!(
        enc_alpha.len(),
        1,
        "unit variant should be 1 byte with u8 tag"
    );
    assert_eq!(enc_alpha[0], 5u8, "Alpha discriminant must be 5");

    let enc_beta =
        oxicode::encode_to_vec_with_config(&RenamedTaggedEnum::Beta(0), oxicode::config::legacy())
            .expect("encode Beta");
    assert_eq!(enc_beta[0], 15u8, "Beta discriminant must be 15");

    let enc_gamma = oxicode::encode_to_vec_with_config(
        &RenamedTaggedEnum::Gamma { value: 0 },
        oxicode::config::legacy(),
    )
    .expect("encode Gamma");
    assert_eq!(enc_gamma[0], 25u8, "Gamma discriminant must be 25");
}

// ---------------------------------------------------------------------------
// 12. Vec<StructWithRenamedFields> roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamedItem {
    #[oxicode(rename = "itemId")]
    id: u32,
    #[oxicode(rename = "itemName")]
    name: String,
}

#[test]
fn test_vec_of_renamed_structs_roundtrip() {
    let original: Vec<RenamedItem> = vec![
        RenamedItem {
            id: 1,
            name: "alpha".to_string(),
        },
        RenamedItem {
            id: 2,
            name: "beta".to_string(),
        },
        RenamedItem {
            id: 3,
            name: "gamma".to_string(),
        },
    ];
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (Vec<RenamedItem>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 13. Option<StructWithRenamedFields> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_renamed_struct_some_roundtrip() {
    let original: Option<RenamedItem> = Some(RenamedItem {
        id: 42,
        name: "optionalItem".to_string(),
    });
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (Option<RenamedItem>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 14. Option<StructWithRenamedFields> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_renamed_struct_none_roundtrip() {
    let original: Option<RenamedItem> = None;
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (Option<RenamedItem>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, None);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 15. Generic struct with renamed field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenericRenamed<T> {
    #[oxicode(rename = "payload")]
    data: T,
    #[oxicode(rename = "version")]
    ver: u8,
}

#[test]
fn test_generic_struct_renamed_field_roundtrip() {
    let original = GenericRenamed::<Vec<u8>> {
        data: vec![1, 2, 3, 4, 5],
        ver: 2,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (GenericRenamed<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

#[test]
fn test_generic_struct_renamed_field_string_roundtrip() {
    let original = GenericRenamed::<String> {
        data: "serialized_payload".to_string(),
        ver: 7,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (GenericRenamed<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 16. Tuple struct with renamed fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamedTuple(
    #[oxicode(rename = "first")] u32,
    #[oxicode(rename = "second")] String,
    #[oxicode(rename = "third")] bool,
);

#[test]
fn test_tuple_struct_renamed_fields_roundtrip() {
    let original = RenamedTuple(123, "hello_tuple".to_string(), true);
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (RenamedTuple, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 17. Unit struct after rename_all on container
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct UnitRenameAll;

#[test]
fn test_unit_struct_with_rename_all_roundtrip() {
    let original = UnitRenameAll;
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (UnitRenameAll, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    // Unit struct has no fields; the encoding must be empty.
    assert!(
        enc.is_empty(),
        "unit struct must produce zero bytes; got {}",
        enc.len()
    );
    assert_eq!(bytes_read, 0);
}

// ---------------------------------------------------------------------------
// 18. rename_all = "camelCase" on struct fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CamelCaseStruct {
    first_name: String,
    last_name: String,
    birth_year: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CamelCasePlain {
    first_name: String,
    last_name: String,
    birth_year: u16,
}

#[test]
fn test_rename_all_camel_case_roundtrip() {
    let original = CamelCaseStruct {
        first_name: "John".to_string(),
        last_name: "Smith".to_string(),
        birth_year: 1990,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (CamelCaseStruct, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

#[test]
fn test_rename_all_camel_case_same_bytes_as_plain() {
    let camel = CamelCaseStruct {
        first_name: "Alice".to_string(),
        last_name: "Wonder".to_string(),
        birth_year: 1985,
    };
    let plain = CamelCasePlain {
        first_name: "Alice".to_string(),
        last_name: "Wonder".to_string(),
        birth_year: 1985,
    };
    let enc_camel = encode_to_vec(&camel).expect("encode camelCase");
    let enc_plain = encode_to_vec(&plain).expect("encode plain");
    assert_eq!(
        enc_camel, enc_plain,
        "rename_all=camelCase must not change wire bytes"
    );
}

// ---------------------------------------------------------------------------
// 19. rename_all = "SCREAMING_SNAKE_CASE" on struct fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingSnakeStruct {
    request_id: u64,
    response_code: u32,
    error_message: String,
}

#[test]
fn test_rename_all_screaming_snake_roundtrip() {
    let original = ScreamingSnakeStruct {
        request_id: 9876543210,
        response_code: 404,
        error_message: "not_found".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (ScreamingSnakeStruct, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 20. rename_all on enum + individual rename overrides rename_all
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)]
enum MixedRenameEnum {
    // rename_all would make this "alphaEvent"; individual rename overrides it.
    #[oxicode(rename = "ALPHA_OVERRIDE")]
    AlphaEvent,
    // rename_all applies here: "betaEvent"
    BetaEvent,
    // rename_all applies here: "gammaEvent"
    GammaEvent(u32),
}

#[test]
fn test_rename_all_with_individual_override_roundtrip() {
    let cases = [
        MixedRenameEnum::AlphaEvent,
        MixedRenameEnum::BetaEvent,
        MixedRenameEnum::GammaEvent(777),
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, bytes_read): (MixedRenameEnum, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(&dec, case);
        assert_eq!(bytes_read, enc.len());
    }
}

#[test]
fn test_rename_all_override_different_discriminants() {
    // Each variant must have a distinct discriminant even with mixed rename strategies.
    let enc_a = encode_to_vec(&MixedRenameEnum::AlphaEvent).expect("encode Alpha");
    let enc_b = encode_to_vec(&MixedRenameEnum::BetaEvent).expect("encode Beta");
    let enc_g = encode_to_vec(&MixedRenameEnum::GammaEvent(0)).expect("encode Gamma");
    // Discriminant bytes differ, so at least the first bytes must differ.
    assert_ne!(
        enc_a[0], enc_b[0],
        "AlphaEvent and BetaEvent must have different discriminants"
    );
    assert_ne!(
        enc_b[0], enc_g[0],
        "BetaEvent and GammaEvent must have different discriminants"
    );
}

// ---------------------------------------------------------------------------
// 21. rename_all = "kebab-case" on struct fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
struct KebabCaseStruct {
    max_retries: u8,
    base_url: String,
    timeout_ms: u64,
}

#[test]
fn test_rename_all_kebab_case_roundtrip() {
    let original = KebabCaseStruct {
        max_retries: 3,
        base_url: "https://example.com".to_string(),
        timeout_ms: 5000,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (KebabCaseStruct, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec, original);
    assert_eq!(bytes_read, enc.len());
}

// ---------------------------------------------------------------------------
// 22. Enum with mixed rename and non-rename variants, full roundtrip sweep
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum MixedVariantRename {
    // Not renamed — uses default positional discriminant.
    Plain,
    // Renamed — still positional discriminant on wire.
    #[oxicode(rename = "specialConnect")]
    Connect {
        host: String,
        port: u16,
    },
    // Not renamed.
    Disconnect(u32),
    // Renamed.
    #[oxicode(rename = "errorCode")]
    Error {
        code: i32,
        message: String,
    },
    // Not renamed.
    Ping,
}

#[test]
fn test_mixed_variant_rename_full_sweep() {
    let cases = vec![
        MixedVariantRename::Plain,
        MixedVariantRename::Connect {
            host: "localhost".to_string(),
            port: 8080,
        },
        MixedVariantRename::Disconnect(42),
        MixedVariantRename::Error {
            code: -1,
            message: "something went wrong".to_string(),
        },
        MixedVariantRename::Ping,
    ];

    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, bytes_read): (MixedVariantRename, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(&dec, case);
        assert_eq!(
            bytes_read,
            enc.len(),
            "all encoded bytes must be consumed for variant"
        );
    }
}

#[test]
fn test_mixed_variant_rename_all_unique_discriminants() {
    // Ensure that all five variants produce different first-discriminant bytes.
    let variants = [
        MixedVariantRename::Plain,
        MixedVariantRename::Connect {
            host: String::new(),
            port: 0,
        },
        MixedVariantRename::Disconnect(0),
        MixedVariantRename::Error {
            code: 0,
            message: String::new(),
        },
        MixedVariantRename::Ping,
    ];

    let discriminants: Vec<u8> = variants
        .iter()
        .map(|v| {
            let enc = encode_to_vec(v).expect("encode variant");
            enc[0]
        })
        .collect();

    // All discriminants must be unique.
    let mut sorted = discriminants.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        discriminants.len(),
        "all variant discriminants must be unique: {discriminants:?}"
    );
}
