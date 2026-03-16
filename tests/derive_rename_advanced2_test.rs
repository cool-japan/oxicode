//! Advanced tests for OxiCode `#[oxicode(rename = "...")]` and
//! `#[oxicode(rename_all = "...")]` attributes — set 2.
//!
//! Key invariant: OxiCode is a **binary** format where fields are encoded
//! positionally.  The `rename` / `rename_all` attributes are therefore no-ops
//! on the wire; they are parsed and stored for future text-layer use but do
//! NOT change the byte layout.
//!
//! All 22 tests are top-level `#[test]` functions; no `#[cfg(test)]` wrappers.
//! No `unwrap()` — every fallible call uses `.expect("msg")`.

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
// Top-level type definitions shared across all tests
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct OriginalNames {
    first_name: String,
    last_name: String,
    age: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamedFields {
    #[oxicode(rename = "firstName")]
    first_name: String,
    #[oxicode(rename = "lastName")]
    last_name: String,
    #[oxicode(rename = "userAge")]
    age: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    #[oxicode(rename = "ACTIVE")]
    Active,
    #[oxicode(rename = "INACTIVE")]
    Inactive,
    #[oxicode(rename = "PENDING")]
    Pending,
}

// ---------------------------------------------------------------------------
// Test 1: RenamedFields struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_01_renamed_fields_struct_roundtrip() {
    let original = RenamedFields {
        first_name: "Alice".to_string(),
        last_name: "Wonderland".to_string(),
        age: 30,
    };
    let encoded = encode_to_vec(&original).expect("encode RenamedFields");
    let (decoded, consumed): (RenamedFields, usize) =
        decode_from_slice(&encoded).expect("decode RenamedFields");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: RenamedFields and OriginalNames produce same wire bytes
// ---------------------------------------------------------------------------

#[test]
fn test_02_renamed_and_original_produce_identical_bytes() {
    let renamed = RenamedFields {
        first_name: "Bob".to_string(),
        last_name: "Builder".to_string(),
        age: 42,
    };
    let original = OriginalNames {
        first_name: "Bob".to_string(),
        last_name: "Builder".to_string(),
        age: 42,
    };
    let renamed_bytes = encode_to_vec(&renamed).expect("encode RenamedFields");
    let original_bytes = encode_to_vec(&original).expect("encode OriginalNames");
    assert_eq!(
        renamed_bytes, original_bytes,
        "rename must not alter binary wire bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 3: RenamedFields consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_03_renamed_fields_consumed_equals_encoded_len() {
    let original = RenamedFields {
        first_name: "Carol".to_string(),
        last_name: "Danvers".to_string(),
        age: 35,
    };
    let encoded = encode_to_vec(&original).expect("encode for consumed check");
    let (_decoded, consumed): (RenamedFields, usize) =
        decode_from_slice(&encoded).expect("decode for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Status::Active roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_04_status_active_roundtrip() {
    let original = Status::Active;
    let encoded = encode_to_vec(&original).expect("encode Status::Active");
    let (decoded, consumed): (Status, usize) =
        decode_from_slice(&encoded).expect("decode Status::Active");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Status::Inactive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_05_status_inactive_roundtrip() {
    let original = Status::Inactive;
    let encoded = encode_to_vec(&original).expect("encode Status::Inactive");
    let (decoded, consumed): (Status, usize) =
        decode_from_slice(&encoded).expect("decode Status::Inactive");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Status::Pending roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_06_status_pending_roundtrip() {
    let original = Status::Pending;
    let encoded = encode_to_vec(&original).expect("encode Status::Pending");
    let (decoded, consumed): (Status, usize) =
        decode_from_slice(&encoded).expect("decode Status::Pending");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Vec<Status> with all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_07_vec_status_all_variants_roundtrip() {
    let original: Vec<Status> = vec![Status::Active, Status::Inactive, Status::Pending];
    let encoded = encode_to_vec(&original).expect("encode Vec<Status>");
    let (decoded, consumed): (Vec<Status>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Status>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Option<RenamedFields> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_08_option_renamed_fields_some_roundtrip() {
    let inner = RenamedFields {
        first_name: "Dave".to_string(),
        last_name: "Eastwood".to_string(),
        age: 50,
    };
    let original: Option<RenamedFields> = Some(inner);
    let encoded = encode_to_vec(&original).expect("encode Option<RenamedFields> Some");
    let (decoded, consumed): (Option<RenamedFields>, usize) =
        decode_from_slice(&encoded).expect("decode Option<RenamedFields> Some");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: RenamedFields with fixed-int config
// ---------------------------------------------------------------------------

#[test]
fn test_09_renamed_fields_fixed_int_config_roundtrip() {
    let original = RenamedFields {
        first_name: "Eve".to_string(),
        last_name: "Fitzgerald".to_string(),
        age: 28,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode RenamedFields fixed-int");
    let (decoded, consumed): (RenamedFields, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode RenamedFields fixed-int");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: RenamedFields with big-endian config
// ---------------------------------------------------------------------------

#[test]
fn test_10_renamed_fields_big_endian_config_roundtrip() {
    let original = RenamedFields {
        first_name: "Frank".to_string(),
        last_name: "Garrison".to_string(),
        age: 44,
    };
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode RenamedFields big-endian");
    let (decoded, consumed): (RenamedFields, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode RenamedFields big-endian");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Renamed field with skip — define new struct
//   `#[oxicode(rename = "id", skip)]` — the field is absent from the wire
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamedWithSkip {
    name: String,
    #[oxicode(rename = "id", skip)]
    internal_id: u64,
    score: u32,
}

#[test]
fn test_11_renamed_field_with_skip_absent_from_wire() {
    let original = RenamedWithSkip {
        name: "Grace".to_string(),
        internal_id: u64::MAX,
        score: 100,
    };
    let encoded = encode_to_vec(&original).expect("encode rename+skip");
    let (decoded, consumed): (RenamedWithSkip, usize) =
        decode_from_slice(&encoded).expect("decode rename+skip");

    assert_eq!(decoded.name, "Grace");
    // Skipped field must decode as Default (0 for u64)
    assert_eq!(
        decoded.internal_id, 0u64,
        "skipped field must be Default (0)"
    );
    assert_eq!(decoded.score, 100);
    assert_eq!(consumed, encoded.len());

    // Verify the skipped field leaves no bytes in the stream by comparing
    // against a struct without that field at all.
    #[derive(Encode)]
    struct WithoutId {
        name: String,
        score: u32,
    }
    let minimal = WithoutId {
        name: "Grace".to_string(),
        score: 100,
    };
    let minimal_bytes = encode_to_vec(&minimal).expect("encode minimal");
    assert_eq!(
        encoded, minimal_bytes,
        "rename+skip must encode identically to struct without that field"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Multiple renamed structs with different renames, same types —
//          same wire bytes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameSetA {
    #[oxicode(rename = "alpha")]
    value_x: u32,
    #[oxicode(rename = "beta")]
    value_y: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameSetB {
    #[oxicode(rename = "foo")]
    value_x: u32,
    #[oxicode(rename = "bar")]
    value_y: String,
}

#[test]
fn test_12_different_renames_same_types_produce_identical_bytes() {
    let a = RenameSetA {
        value_x: 77,
        value_y: "hello".to_string(),
    };
    let b = RenameSetB {
        value_x: 77,
        value_y: "hello".to_string(),
    };
    let bytes_a = encode_to_vec(&a).expect("encode RenameSetA");
    let bytes_b = encode_to_vec(&b).expect("encode RenameSetB");
    assert_eq!(
        bytes_a, bytes_b,
        "different rename strings on same-typed structs must produce identical wire bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Renamed enum variants decode from correct bytes
// ---------------------------------------------------------------------------

#[test]
fn test_13_renamed_enum_variants_decode_from_correct_bytes() {
    // Encode each variant and verify it decodes back to the right variant.
    for variant in &[Status::Active, Status::Inactive, Status::Pending] {
        let encoded = encode_to_vec(variant).expect("encode Status variant");
        let (decoded, consumed): (Status, usize) =
            decode_from_slice(&encoded).expect("decode Status variant");
        assert_eq!(&decoded, variant);
        assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 14: Different enum variants after rename produce different bytes
// ---------------------------------------------------------------------------

#[test]
fn test_14_different_renamed_variants_produce_different_bytes() {
    let active_bytes = encode_to_vec(&Status::Active).expect("encode Active");
    let inactive_bytes = encode_to_vec(&Status::Inactive).expect("encode Inactive");
    let pending_bytes = encode_to_vec(&Status::Pending).expect("encode Pending");

    assert_ne!(
        active_bytes, inactive_bytes,
        "Active vs Inactive must differ"
    );
    assert_ne!(active_bytes, pending_bytes, "Active vs Pending must differ");
    assert_ne!(
        inactive_bytes, pending_bytes,
        "Inactive vs Pending must differ"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Struct with single renamed field roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SingleRenamedField {
    #[oxicode(rename = "theValue")]
    value: u64,
}

#[test]
fn test_15_single_renamed_field_roundtrip() {
    let original = SingleRenamedField {
        value: 0xDEAD_BEEF_CAFE_F00D,
    };
    let encoded = encode_to_vec(&original).expect("encode SingleRenamedField");
    let (decoded, consumed): (SingleRenamedField, usize) =
        decode_from_slice(&encoded).expect("decode SingleRenamedField");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Struct with rename + default combination
// ---------------------------------------------------------------------------

fn default_threshold() -> u32 {
    42
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameWithDefault {
    label: String,
    #[oxicode(rename = "threshold", default = "default_threshold")]
    threshold: u32,
    active: bool,
}

#[test]
fn test_16_rename_with_default_combination_roundtrip() {
    let original = RenameWithDefault {
        label: "test".to_string(),
        threshold: 999, // NOT encoded; default_threshold() returns 42
        active: true,
    };
    let encoded = encode_to_vec(&original).expect("encode rename+default");
    let (decoded, consumed): (RenameWithDefault, usize) =
        decode_from_slice(&encoded).expect("decode rename+default");

    assert_eq!(decoded.label, "test");
    assert_eq!(
        decoded.threshold, 42,
        "default_threshold() must return 42 for renamed+default field"
    );
    assert!(decoded.active);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Vec<RenamedFields> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_17_vec_renamed_fields_roundtrip() {
    let original: Vec<RenamedFields> = vec![
        RenamedFields {
            first_name: "Hannah".to_string(),
            last_name: "Ingram".to_string(),
            age: 25,
        },
        RenamedFields {
            first_name: "Ivan".to_string(),
            last_name: "Jenkins".to_string(),
            age: 31,
        },
        RenamedFields {
            first_name: "Julia".to_string(),
            last_name: "Kim".to_string(),
            age: 19,
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<RenamedFields>");
    let (decoded, consumed): (Vec<RenamedFields>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<RenamedFields>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Rename does not affect wire byte count
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct NoRenameBaseline {
    first_name: String,
    last_name: String,
    age: u32,
}

#[test]
fn test_18_rename_does_not_affect_wire_byte_count() {
    let renamed = RenamedFields {
        first_name: "Karl".to_string(),
        last_name: "Lambert".to_string(),
        age: 38,
    };
    let baseline = NoRenameBaseline {
        first_name: "Karl".to_string(),
        last_name: "Lambert".to_string(),
        age: 38,
    };
    let renamed_len = encode_to_vec(&renamed)
        .expect("encode renamed byte count")
        .len();
    let baseline_len = encode_to_vec(&baseline)
        .expect("encode baseline byte count")
        .len();
    assert_eq!(
        renamed_len, baseline_len,
        "rename must not change the wire byte count"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Renamed struct re-encode after decode produces same bytes
// ---------------------------------------------------------------------------

#[test]
fn test_19_renamed_struct_reencode_after_decode_produces_same_bytes() {
    let original = RenamedFields {
        first_name: "Mia".to_string(),
        last_name: "Nelson".to_string(),
        age: 22,
    };
    let first_encoded = encode_to_vec(&original).expect("first encode");
    let (decoded, _consumed): (RenamedFields, usize) =
        decode_from_slice(&first_encoded).expect("decode for re-encode test");
    let second_encoded = encode_to_vec(&decoded).expect("re-encode after decode");
    assert_eq!(
        first_encoded, second_encoded,
        "re-encoding a decoded RenamedFields must produce the same bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Status roundtrip all 3 variants in order
// ---------------------------------------------------------------------------

#[test]
fn test_20_status_all_three_variants_in_order() {
    let variants = [Status::Active, Status::Inactive, Status::Pending];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode Status in order");
        let (decoded, consumed): (Status, usize) =
            decode_from_slice(&encoded).expect("decode Status in order");
        assert_eq!(&decoded, variant);
        assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 21: Struct with numeric rename (rename = "field1") roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct NumericRename {
    #[oxicode(rename = "field1")]
    value_a: u32,
    #[oxicode(rename = "field2")]
    value_b: String,
    #[oxicode(rename = "field3")]
    value_c: bool,
}

#[test]
fn test_21_numeric_rename_string_roundtrip() {
    let original = NumericRename {
        value_a: 111,
        value_b: "numeric_rename".to_string(),
        value_c: false,
    };
    let encoded = encode_to_vec(&original).expect("encode NumericRename");
    let (decoded, consumed): (NumericRename, usize) =
        decode_from_slice(&encoded).expect("decode NumericRename");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: Nested struct with renamed outer fields roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerData {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OuterWithRenamedFields {
    #[oxicode(rename = "outerLabel")]
    label: String,
    #[oxicode(rename = "coordinates")]
    coords: InnerData,
    #[oxicode(rename = "outerStatus")]
    status: Status,
}

#[test]
fn test_22_nested_struct_with_renamed_outer_fields_roundtrip() {
    let original = OuterWithRenamedFields {
        label: "nested_rename_test".to_string(),
        coords: InnerData { x: -10, y: 25 },
        status: Status::Pending,
    };
    let encoded = encode_to_vec(&original).expect("encode OuterWithRenamedFields");
    let (decoded, consumed): (OuterWithRenamedFields, usize) =
        decode_from_slice(&encoded).expect("decode OuterWithRenamedFields");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}
