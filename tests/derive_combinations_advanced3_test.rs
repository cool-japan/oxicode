//! 22 tests combining multiple derive attributes in patterns not covered by
//! derive_combinations_test.rs.
//!
//! All tests are top-level `#[test]` functions with no `#[cfg(test)]` wrapper.
//! No `unwrap()` — all Results are handled with `.expect("msg")`.

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
// Helper / transform functions used by encode_with / decode_with tests
// ---------------------------------------------------------------------------

mod adv3_transforms {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    /// Encode a String by reversing its characters on the wire.
    #[allow(clippy::ptr_arg)]
    pub fn encode_reversed<E: Encoder>(s: &String, encoder: &mut E) -> Result<(), Error> {
        let reversed: String = s.chars().rev().collect();
        reversed.encode(encoder)
    }

    /// Decode a String and reverse it back.
    pub fn decode_reversed<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
        let s = String::decode(decoder)?;
        Ok(s.chars().rev().collect())
    }

    /// Encode a u64 by multiplying by 3.
    pub fn encode_tripled<E: Encoder>(n: &u64, encoder: &mut E) -> Result<(), Error> {
        (n * 3).encode(encoder)
    }

    /// Decode a u64 by dividing by 3.
    pub fn decode_third<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u64, Error> {
        Ok(u64::decode(decoder)? / 3)
    }
}

// ---------------------------------------------------------------------------
// Test 1: rename_all = "camelCase" on container + skip on one field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CamelCaseWithSkip {
    first_name: String,
    last_name: String,
    #[oxicode(skip)]
    internal_id: u64,
}

#[test]
fn test_01_rename_all_camel_case_plus_skip() {
    let original = CamelCaseWithSkip {
        first_name: "John".into(),
        last_name: "Doe".into(),
        internal_id: 0xDEAD_BEEF,
    };
    let enc = encode_to_vec(&original).expect("encode CamelCaseWithSkip");
    let (dec, _): (CamelCaseWithSkip, _) =
        decode_from_slice(&enc).expect("decode CamelCaseWithSkip");
    assert_eq!(dec.first_name, "John");
    assert_eq!(dec.last_name, "Doe");
    assert_eq!(dec.internal_id, 0u64, "skipped field must be Default (0)");
}

// ---------------------------------------------------------------------------
// Test 2: tag_type = "u8" on enum + rename_all = "snake_case"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8", rename_all = "snake_case")]
enum SnakeCaseTaggedEnum {
    UserLogin,
    UserLogout,
    SessionExpired { reason: String },
}

#[test]
fn test_02_tag_type_u8_with_rename_all_snake_case() {
    let cases = [
        SnakeCaseTaggedEnum::UserLogin,
        SnakeCaseTaggedEnum::UserLogout,
        SnakeCaseTaggedEnum::SessionExpired {
            reason: "timeout".into(),
        },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode SnakeCaseTaggedEnum");
        let (dec, _): (SnakeCaseTaggedEnum, _) =
            decode_from_slice(&enc).expect("decode SnakeCaseTaggedEnum");
        assert_eq!(case, &dec);
    }
    // Verify u8 tag in legacy (fixed-int) mode: 1 byte for unit variant.
    let enc_unit = oxicode::encode_to_vec_with_config(
        &SnakeCaseTaggedEnum::UserLogin,
        oxicode::config::legacy(),
    )
    .expect("encode legacy");
    assert_eq!(
        enc_unit.len(),
        1,
        "u8 tag = 1 byte for unit variant in legacy mode"
    );
}

// ---------------------------------------------------------------------------
// Test 3: seq_len + rename on same field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SeqLenWithRename {
    #[oxicode(seq_len = "u16", rename = "packet_list")]
    packets: Vec<u32>,
    label: String,
}

#[test]
fn test_03_seq_len_and_rename_on_same_field() {
    let original = SeqLenWithRename {
        packets: vec![10, 20, 30, 40],
        label: "stream".into(),
    };
    let enc = encode_to_vec(&original).expect("encode SeqLenWithRename");
    let (dec, _): (SeqLenWithRename, _) = decode_from_slice(&enc).expect("decode SeqLenWithRename");
    assert_eq!(dec.packets, vec![10u32, 20, 30, 40]);
    assert_eq!(dec.label, "stream");
}

// ---------------------------------------------------------------------------
// Test 4: encode_with + decode_with on struct field
// ---------------------------------------------------------------------------

#[derive(Debug, Encode, Decode)]
struct ReversedStringStruct {
    id: u32,
    #[oxicode(
        encode_with = "adv3_transforms::encode_reversed",
        decode_with = "adv3_transforms::decode_reversed"
    )]
    tag: String,
}

#[test]
fn test_04_encode_with_decode_with_on_struct_field() {
    let original = ReversedStringStruct {
        id: 7,
        tag: "hello".into(),
    };
    let enc = encode_to_vec(&original).expect("encode ReversedStringStruct");
    let (dec, _): (ReversedStringStruct, _) =
        decode_from_slice(&enc).expect("decode ReversedStringStruct");
    // "hello" reversed → "olleh" on wire; reversed back → "hello"
    assert_eq!(dec.id, 7);
    assert_eq!(dec.tag, "hello");

    // Verify what's on the wire: decode as plain String to confirm reversal
    #[derive(Debug, Decode)]
    #[allow(dead_code)]
    struct WireView {
        id: u32,
        tag: String,
    }
    let (wire, _): (WireView, _) = decode_from_slice(&enc).expect("wire decode");
    assert_eq!(wire.tag, "olleh", "wire should hold the reversed string");
}

// ---------------------------------------------------------------------------
// Test 5: Container rename_all + field-level rename override
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct ContainerAndFieldRename {
    first_name: String,
    // field rename overrides the container rename_all
    #[oxicode(rename = "user_score")]
    user_score_val: u32,
    last_name: String,
}

#[test]
fn test_05_container_rename_all_and_field_rename_override() {
    let original = ContainerAndFieldRename {
        first_name: "Alice".into(),
        user_score_val: 99,
        last_name: "Smith".into(),
    };
    let enc = encode_to_vec(&original).expect("encode ContainerAndFieldRename");
    let (dec, _): (ContainerAndFieldRename, _) =
        decode_from_slice(&enc).expect("decode ContainerAndFieldRename");
    assert_eq!(dec.first_name, "Alice");
    assert_eq!(dec.user_score_val, 99);
    assert_eq!(dec.last_name, "Smith");
}

// ---------------------------------------------------------------------------
// Test 6: skip + custom default function
// ---------------------------------------------------------------------------

fn default_priority() -> u8 {
    42u8
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipWithCustomDefault {
    name: String,
    #[oxicode(default = "default_priority")]
    priority: u8,
    value: u32,
}

#[test]
fn test_06_skip_with_custom_default_function() {
    let original = SkipWithCustomDefault {
        name: "task".into(),
        priority: 255, // not encoded
        value: 1000,
    };
    let enc = encode_to_vec(&original).expect("encode SkipWithCustomDefault");
    let (dec, _): (SkipWithCustomDefault, _) =
        decode_from_slice(&enc).expect("decode SkipWithCustomDefault");
    assert_eq!(dec.name, "task");
    assert_eq!(
        dec.priority, 42u8,
        "default_priority() must be used on decode"
    );
    assert_eq!(dec.value, 1000);
}

// ---------------------------------------------------------------------------
// Test 7: Generic struct with bound = "..." attribute
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode + PartialEq + Clone + std::fmt::Debug")]
struct TripleGeneric<T> {
    first: T,
    second: T,
    third: T,
}

#[test]
fn test_07_generic_struct_with_bound_attribute() {
    let original: TripleGeneric<u32> = TripleGeneric {
        first: 1,
        second: 2,
        third: 3,
    };
    let enc = encode_to_vec(&original).expect("encode TripleGeneric<u32>");
    let (dec, _): (TripleGeneric<u32>, _) =
        decode_from_slice(&enc).expect("decode TripleGeneric<u32>");
    assert_eq!(original, dec);

    let original_str: TripleGeneric<String> = TripleGeneric {
        first: "a".into(),
        second: "b".into(),
        third: "c".into(),
    };
    let enc2 = encode_to_vec(&original_str).expect("encode TripleGeneric<String>");
    let (dec2, _): (TripleGeneric<String>, _) =
        decode_from_slice(&enc2).expect("decode TripleGeneric<String>");
    assert_eq!(original_str, dec2);
}

// ---------------------------------------------------------------------------
// Test 8: variant = N on enum + tag_type = "u16"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum U16TaggedWithVariants {
    #[oxicode(variant = 500)]
    Request { method: String },
    #[oxicode(variant = 501)]
    Response { status: u16 },
    #[oxicode(variant = 999)]
    Error(String),
}

#[test]
fn test_08_variant_n_with_tag_type_u16() {
    let cases = [
        U16TaggedWithVariants::Request {
            method: "GET".into(),
        },
        U16TaggedWithVariants::Response { status: 200 },
        U16TaggedWithVariants::Error("not found".into()),
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode U16TaggedWithVariants");
        let (dec, _): (U16TaggedWithVariants, _) =
            decode_from_slice(&enc).expect("decode U16TaggedWithVariants");
        assert_eq!(case, &dec);
    }
    // Verify discriminant in legacy mode
    let enc_req = oxicode::encode_to_vec_with_config(
        &U16TaggedWithVariants::Request {
            method: "GET".into(),
        },
        oxicode::config::legacy(),
    )
    .expect("encode legacy Request");
    // u16 LE = [500 % 256, 500 / 256] = [244, 1]
    let disc = u16::from_le_bytes([enc_req[0], enc_req[1]]);
    assert_eq!(disc, 500u16, "Request discriminant should be 500");
}

// ---------------------------------------------------------------------------
// Test 9: bytes attribute + rename on same field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct BytesWithRename {
    id: u32,
    #[oxicode(bytes, rename = "raw_payload")]
    raw_payload_data: Vec<u8>,
    version: u8,
}

#[test]
fn test_09_bytes_attribute_with_rename() {
    let original = BytesWithRename {
        id: 42,
        raw_payload_data: vec![0xCA, 0xFE, 0xBA, 0xBE],
        version: 3,
    };
    let enc = encode_to_vec(&original).expect("encode BytesWithRename");
    let (dec, _): (BytesWithRename, _) = decode_from_slice(&enc).expect("decode BytesWithRename");
    assert_eq!(dec.id, 42);
    assert_eq!(dec.raw_payload_data, vec![0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(dec.version, 3);
}

// ---------------------------------------------------------------------------
// Test 10: Struct with rename + skip + seq_len combined across multiple fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllAttributesCombined {
    #[oxicode(rename = "event_id")]
    id: u32,
    #[oxicode(skip)]
    cached_hash: u64,
    #[oxicode(seq_len = "u8", rename = "item_list")]
    items: Vec<u16>,
    label: String,
}

#[test]
fn test_10_rename_skip_seq_len_combined_in_struct() {
    let original = AllAttributesCombined {
        id: 55,
        cached_hash: 0xFFFF_FFFF_FFFF,
        items: vec![100, 200, 300],
        label: "combined".into(),
    };
    let enc = encode_to_vec(&original).expect("encode AllAttributesCombined");
    let (dec, _): (AllAttributesCombined, _) =
        decode_from_slice(&enc).expect("decode AllAttributesCombined");
    assert_eq!(dec.id, 55);
    assert_eq!(dec.cached_hash, 0u64, "skipped field must be Default (0)");
    assert_eq!(dec.items, vec![100u16, 200, 300]);
    assert_eq!(dec.label, "combined");
}

// ---------------------------------------------------------------------------
// Test 11: Nested struct — outer has rename_all, inner has rename on a field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerWithRename {
    #[oxicode(rename = "item_count")]
    count: u32,
    tag: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct OuterWithRenameAll {
    outer_id: u32,
    inner_data: InnerWithRename,
    outer_name: String,
}

#[test]
fn test_11_nested_outer_rename_all_inner_field_rename() {
    let original = OuterWithRenameAll {
        outer_id: 1,
        inner_data: InnerWithRename {
            count: 5,
            tag: "inner_tag".into(),
        },
        outer_name: "outer".into(),
    };
    let enc = encode_to_vec(&original).expect("encode OuterWithRenameAll");
    let (dec, _): (OuterWithRenameAll, _) =
        decode_from_slice(&enc).expect("decode OuterWithRenameAll");
    assert_eq!(dec.outer_id, 1);
    assert_eq!(dec.inner_data.count, 5);
    assert_eq!(dec.inner_data.tag, "inner_tag");
    assert_eq!(dec.outer_name, "outer");
}

// ---------------------------------------------------------------------------
// Test 12: Enum where some variants have variant = N and others use default
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum MixedVariantAssignment {
    Auto0, // gets discriminant 0
    #[oxicode(variant = 50)]
    Custom50(u32),
    Auto51, // gets discriminant 51 (next after 50)
    #[oxicode(variant = 200)]
    Custom200 {
        x: u32,
        y: u32,
    },
}

#[test]
fn test_12_mixed_custom_and_auto_variant_discriminants() {
    let cases = [
        MixedVariantAssignment::Auto0,
        MixedVariantAssignment::Custom50(99),
        MixedVariantAssignment::Auto51,
        MixedVariantAssignment::Custom200 { x: 10, y: 20 },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode MixedVariantAssignment");
        let (dec, _): (MixedVariantAssignment, _) =
            decode_from_slice(&enc).expect("decode MixedVariantAssignment");
        assert_eq!(case, &dec);
    }
    // Verify Auto0 discriminant = 0 in legacy mode
    let enc_auto0 = oxicode::encode_to_vec_with_config(
        &MixedVariantAssignment::Auto0,
        oxicode::config::legacy(),
    )
    .expect("encode legacy Auto0");
    assert_eq!(enc_auto0[0], 0u8, "Auto0 should have discriminant 0");

    // Verify Custom50 discriminant = 50
    let enc_custom50 = oxicode::encode_to_vec_with_config(
        &MixedVariantAssignment::Custom50(0),
        oxicode::config::legacy(),
    )
    .expect("encode legacy Custom50");
    assert_eq!(
        enc_custom50[0], 50u8,
        "Custom50 should have discriminant 50"
    );
}

// ---------------------------------------------------------------------------
// Test 13: with_default on Vec field (skip + default = "Vec::new")
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct VecWithDefaultSkip {
    id: u32,
    name: String,
    #[oxicode(default = "Vec::new")]
    tags: Vec<String>,
}

#[test]
fn test_13_default_fn_vec_field_yields_empty_on_decode() {
    let original = VecWithDefaultSkip {
        id: 9,
        name: "item".into(),
        tags: vec!["a".into(), "b".into()], // not encoded (skipped via default)
    };
    let enc = encode_to_vec(&original).expect("encode VecWithDefaultSkip");
    let (dec, _): (VecWithDefaultSkip, _) =
        decode_from_slice(&enc).expect("decode VecWithDefaultSkip");
    assert_eq!(dec.id, 9);
    assert_eq!(dec.name, "item");
    assert!(
        dec.tags.is_empty(),
        "Vec field with default = Vec::new should be empty"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Generic enum with type constraints
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum GenericEnumResult<T, E>
where
    T: Encode + Decode + PartialEq + std::fmt::Debug,
    E: Encode + Decode + PartialEq + std::fmt::Debug,
{
    Ok(T),
    Err(E),
    Pending,
}

#[test]
fn test_14_generic_enum_with_type_constraints() {
    let ok_val: GenericEnumResult<u32, String> = GenericEnumResult::Ok(42);
    let err_val: GenericEnumResult<u32, String> = GenericEnumResult::Err("fail".into());
    let pending_val: GenericEnumResult<u32, String> = GenericEnumResult::Pending;

    for val in [&ok_val, &err_val, &pending_val] {
        let enc = encode_to_vec(val).expect("encode GenericEnumResult");
        let (dec, _): (GenericEnumResult<u32, String>, _) =
            decode_from_slice(&enc).expect("decode GenericEnumResult");
        assert_eq!(val, &dec);
    }
}

// ---------------------------------------------------------------------------
// Test 15: Struct implementing custom encode_with/decode_with for a complex type
// ---------------------------------------------------------------------------

#[derive(Debug, Encode, Decode)]
struct ComplexTransformStruct {
    version: u32,
    #[oxicode(
        encode_with = "adv3_transforms::encode_tripled",
        decode_with = "adv3_transforms::decode_third"
    )]
    timestamp: u64,
    #[oxicode(
        encode_with = "adv3_transforms::encode_reversed",
        decode_with = "adv3_transforms::decode_reversed"
    )]
    message: String,
}

#[test]
fn test_15_complex_struct_custom_encode_decode_with() {
    let original = ComplexTransformStruct {
        version: 1,
        timestamp: 1000,
        message: "world".into(),
    };
    let enc = encode_to_vec(&original).expect("encode ComplexTransformStruct");

    // Verify wire representation: timestamp * 3 = 3000, message reversed = "dlrow"
    #[derive(Debug, Decode)]
    struct WireRepr {
        version: u32,
        timestamp: u64,
        message: String,
    }
    let (wire, _): (WireRepr, _) = decode_from_slice(&enc).expect("decode wire repr");
    assert_eq!(wire.version, 1);
    assert_eq!(wire.timestamp, 3000u64, "wire timestamp should be tripled");
    assert_eq!(wire.message, "dlrow", "wire message should be reversed");

    // Roundtrip via original type
    let (dec, _): (ComplexTransformStruct, _) =
        decode_from_slice(&enc).expect("decode ComplexTransformStruct");
    assert_eq!(dec.version, 1);
    assert_eq!(
        dec.timestamp, 1000u64,
        "timestamp should be un-tripled on decode"
    );
    assert_eq!(
        dec.message, "world",
        "message should be un-reversed on decode"
    );
}

// ---------------------------------------------------------------------------
// Test 16: flatten attribute on nested struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Address {
    street: String,
    city: String,
    zip: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Person {
    name: String,
    age: u32,
    #[oxicode(flatten)]
    address: Address,
}

#[test]
fn test_16_flatten_attribute_on_nested_struct() {
    let original = Person {
        name: "Bob".into(),
        age: 30,
        address: Address {
            street: "Main St".into(),
            city: "Springfield".into(),
            zip: 12345,
        },
    };
    let enc = encode_to_vec(&original).expect("encode Person");
    let (dec, _): (Person, _) = decode_from_slice(&enc).expect("decode Person");
    assert_eq!(dec.name, "Bob");
    assert_eq!(dec.age, 30);
    assert_eq!(dec.address.street, "Main St");
    assert_eq!(dec.address.city, "Springfield");
    assert_eq!(dec.address.zip, 12345);

    // Verify flatten binary-compatibility with manually flat struct
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FlatPerson {
        name: String,
        age: u32,
        street: String,
        city: String,
        zip: u32,
    }
    let flat = FlatPerson {
        name: "Bob".into(),
        age: 30,
        street: "Main St".into(),
        city: "Springfield".into(),
        zip: 12345,
    };
    let flat_enc = encode_to_vec(&flat).expect("encode FlatPerson");
    assert_eq!(
        enc, flat_enc,
        "flatten must produce byte-identical output to manually flat struct"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Tuple struct with rename attribute
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TupleWithFieldRename(u32, #[oxicode(rename = "tag_name")] String, u16);

#[test]
fn test_17_tuple_struct_with_rename() {
    let original = TupleWithFieldRename(100, "hello".into(), 42);
    let enc = encode_to_vec(&original).expect("encode TupleWithFieldRename");
    let (dec, _): (TupleWithFieldRename, _) =
        decode_from_slice(&enc).expect("decode TupleWithFieldRename");
    assert_eq!(dec.0, 100);
    assert_eq!(dec.1, "hello");
    assert_eq!(dec.2, 42);
}

// ---------------------------------------------------------------------------
// Test 18: Unit struct combination — unit struct with rename_all (no-op) and plain roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct UnitStruct;

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum UnitEnumContainer {
    #[oxicode(variant = 7)]
    OnlyUnit,
    WithPayload(u32),
}

#[test]
fn test_18_unit_struct_and_unit_enum_variants() {
    // Unit struct roundtrip
    let unit = UnitStruct;
    let enc_unit = encode_to_vec(&unit).expect("encode UnitStruct");
    let (dec_unit, _): (UnitStruct, _) = decode_from_slice(&enc_unit).expect("decode UnitStruct");
    assert_eq!(unit, dec_unit);
    assert!(enc_unit.is_empty(), "unit struct should produce 0 bytes");

    // Unit enum variant roundtrip
    let only_unit = UnitEnumContainer::OnlyUnit;
    let enc_ou = encode_to_vec(&only_unit).expect("encode OnlyUnit");
    let (dec_ou, _): (UnitEnumContainer, _) = decode_from_slice(&enc_ou).expect("decode OnlyUnit");
    assert_eq!(only_unit, dec_ou);

    let with_payload = UnitEnumContainer::WithPayload(999);
    let enc_wp = encode_to_vec(&with_payload).expect("encode WithPayload");
    let (dec_wp, _): (UnitEnumContainer, _) =
        decode_from_slice(&enc_wp).expect("decode WithPayload");
    assert_eq!(with_payload, dec_wp);
}

// ---------------------------------------------------------------------------
// Test 19: Two structs that cross-decode (same wire layout, different field names)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct LayoutA {
    x: u32,
    y: u32,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LayoutB {
    width: u32,
    height: u32,
    name: String,
}

#[test]
fn test_19_two_structs_cross_decode_same_wire_layout() {
    let a = LayoutA {
        x: 800,
        y: 600,
        label: "screen".into(),
    };
    let enc_a = encode_to_vec(&a).expect("encode LayoutA");

    // Decode LayoutA bytes as LayoutB (same wire layout, different field names)
    let (b, _): (LayoutB, _) = decode_from_slice(&enc_a).expect("cross-decode as LayoutB");
    assert_eq!(b.width, 800, "width should match x");
    assert_eq!(b.height, 600, "height should match y");
    assert_eq!(b.name, "screen", "name should match label");

    // Verify symmetry: encode B, decode as A
    let enc_b = encode_to_vec(&b).expect("encode LayoutB");
    let (a2, _): (LayoutA, _) = decode_from_slice(&enc_b).expect("cross-decode as LayoutA");
    assert_eq!(a2, a);
}

// ---------------------------------------------------------------------------
// Test 20: Struct with 8+ fields, many with attributes
// ---------------------------------------------------------------------------

fn default_max_retries() -> u8 {
    3u8
}

fn default_timeout_ms() -> u32 {
    5000u32
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct LargeAttributedStruct {
    request_id: u64,
    #[oxicode(rename = "user_name")]
    user_name_field: String,
    #[oxicode(skip)]
    session_token: u64,
    #[oxicode(seq_len = "u16")]
    tags: Vec<String>,
    priority: u8,
    #[oxicode(default = "default_max_retries")]
    max_retries: u8,
    #[oxicode(default = "default_timeout_ms")]
    timeout_ms: u32,
    active: bool,
    #[oxicode(bytes)]
    payload: Vec<u8>,
}

#[test]
fn test_20_struct_with_8_plus_fields_many_attributes() {
    let original = LargeAttributedStruct {
        request_id: 42,
        user_name_field: "alice".into(),
        session_token: 0xCAFE_BABE,
        tags: vec!["rust".into(), "oxicode".into()],
        priority: 1,
        max_retries: 10,  // not encoded
        timeout_ms: 9999, // not encoded
        active: true,
        payload: vec![0xDE, 0xAD],
    };
    let enc = encode_to_vec(&original).expect("encode LargeAttributedStruct");
    let (dec, _): (LargeAttributedStruct, _) =
        decode_from_slice(&enc).expect("decode LargeAttributedStruct");

    assert_eq!(dec.request_id, 42);
    assert_eq!(dec.user_name_field, "alice");
    assert_eq!(dec.session_token, 0u64, "skipped field must be Default");
    assert_eq!(dec.tags, vec!["rust".to_string(), "oxicode".to_string()]);
    assert_eq!(dec.priority, 1);
    assert_eq!(dec.max_retries, 3u8, "default_max_retries() = 3");
    assert_eq!(dec.timeout_ms, 5000u32, "default_timeout_ms() = 5000");
    assert!(dec.active);
    assert_eq!(dec.payload, vec![0xDE_u8, 0xAD]);
}

// ---------------------------------------------------------------------------
// Test 21: Enum with unit variants, tuple variants, and struct variants,
//          mixed attributes: tag_type, rename_all, variant = N, seq_len
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16", rename_all = "snake_case")]
enum MixedVariantEnum {
    #[oxicode(variant = 1)]
    Connect,
    #[oxicode(variant = 2)]
    Disconnect,
    #[oxicode(variant = 100)]
    SendMessage {
        recipient: String,
        #[oxicode(seq_len = "u8")]
        data: Vec<u8>,
    },
    #[oxicode(variant = 200)]
    BroadcastAlert(String),
    Error {
        code: u32,
        detail: String,
    },
}

#[test]
fn test_21_enum_unit_and_struct_variants_mixed_attributes() {
    let cases: Vec<MixedVariantEnum> = vec![
        MixedVariantEnum::Connect,
        MixedVariantEnum::Disconnect,
        MixedVariantEnum::SendMessage {
            recipient: "bob".into(),
            data: vec![1, 2, 3],
        },
        MixedVariantEnum::BroadcastAlert("emergency".into()),
        MixedVariantEnum::Error {
            code: 404,
            detail: "not found".into(),
        },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode MixedVariantEnum");
        let (dec, _): (MixedVariantEnum, _) =
            decode_from_slice(&enc).expect("decode MixedVariantEnum");
        assert_eq!(case, &dec);
    }

    // Verify discriminants in legacy mode
    let enc_connect =
        oxicode::encode_to_vec_with_config(&MixedVariantEnum::Connect, oxicode::config::legacy())
            .expect("encode legacy Connect");
    let disc_connect = u16::from_le_bytes([enc_connect[0], enc_connect[1]]);
    assert_eq!(disc_connect, 1u16, "Connect discriminant should be 1");

    let enc_broadcast = oxicode::encode_to_vec_with_config(
        &MixedVariantEnum::BroadcastAlert("x".into()),
        oxicode::config::legacy(),
    )
    .expect("encode legacy BroadcastAlert");
    let disc_broadcast = u16::from_le_bytes([enc_broadcast[0], enc_broadcast[1]]);
    assert_eq!(
        disc_broadcast, 200u16,
        "BroadcastAlert discriminant should be 200"
    );

    // Verify seq_len = u8 is used in SendMessage.data:
    // legacy: 2 bytes (u16 tag) + string(3 bytes recipient) + 1 byte (u8 len) + 3 bytes data
    let enc_send = oxicode::encode_to_vec_with_config(
        &MixedVariantEnum::SendMessage {
            recipient: "bob".into(),
            data: vec![1, 2, 3],
        },
        oxicode::config::legacy(),
    )
    .expect("encode legacy SendMessage");
    // tag=2, "bob"=4(len_u64)+3, data_len=1(u8), data=3 => 2+4+3+1+3=13
    // Actually with legacy: string len is u64 (8 bytes) in fixed-int mode
    // We just check the discriminant
    let disc_send = u16::from_le_bytes([enc_send[0], enc_send[1]]);
    assert_eq!(disc_send, 100u16, "SendMessage discriminant should be 100");
}

// ---------------------------------------------------------------------------
// Test 22: Deep nesting: 3 levels with attributes at each level
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level3 {
    #[oxicode(rename = "deep_value")]
    value: u32,
    #[oxicode(skip)]
    internal: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct Level2 {
    node_id: u32,
    #[oxicode(seq_len = "u8")]
    node_tags: Vec<u16>,
    nested_level3: Level3,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Level1 {
    #[oxicode(variant = 10)]
    Data {
        #[oxicode(flatten)]
        content: Level2,
    },
    #[oxicode(variant = 20)]
    Empty,
}

#[test]
fn test_22_deep_nesting_3_levels_with_attributes() {
    let level3 = Level3 {
        value: 42,
        internal: 0xDEAD_BEEF,
    };
    let level2 = Level2 {
        node_id: 1,
        node_tags: vec![10, 20, 30],
        nested_level3: level3,
    };
    let original = Level1::Data { content: level2 };

    let enc = encode_to_vec(&original).expect("encode Level1::Data");
    let (dec, _): (Level1, _) = decode_from_slice(&enc).expect("decode Level1::Data");

    match dec {
        Level1::Data { content } => {
            assert_eq!(content.node_id, 1);
            assert_eq!(content.node_tags, vec![10u16, 20, 30]);
            assert_eq!(content.nested_level3.value, 42);
            assert_eq!(
                content.nested_level3.internal, 0u64,
                "skipped deep field must be Default"
            );
        }
        other => panic!("expected Level1::Data, got {:?}", other),
    }

    // Verify Empty variant also roundtrips
    let empty = Level1::Empty;
    let enc_empty = encode_to_vec(&empty).expect("encode Level1::Empty");
    let (dec_empty, _): (Level1, _) = decode_from_slice(&enc_empty).expect("decode Level1::Empty");
    assert_eq!(empty, dec_empty);

    // Verify discriminant of Data in legacy mode = 10
    let enc_data = oxicode::encode_to_vec_with_config(
        &Level1::Data {
            content: Level2 {
                node_id: 0,
                node_tags: vec![],
                nested_level3: Level3 {
                    value: 0,
                    internal: 0,
                },
            },
        },
        oxicode::config::legacy(),
    )
    .expect("encode legacy Data");
    assert_eq!(enc_data[0], 10u8, "Data discriminant should be 10");
}
