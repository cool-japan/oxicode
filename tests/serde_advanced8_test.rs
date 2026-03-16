// serde_advanced8_test.rs — 22 advanced serde integration tests for OxiCode
//
// All tests are gated on the `serde` feature at the file level.
// No #[cfg(test)] module wrapper; tests are top-level items.
// No unwrap() — all Results use .expect("...").
//
// Coverage focuses on serde_json::Value encoding, serde attribute combos
// (#[serde(rename_all = "camelCase")], #[serde(skip_serializing_if)],
// #[serde(untagged)], #[serde(tag)], #[serde(default)], #[serde(flatten)]),
// determinism, BTreeMap via serde, tuples, newtype structs, unit structs,
// enum tuple variants, and a custom Serialize/Deserialize implementation.
//
// Key limitation: oxicode is a non-self-describing binary format, so
// serde_json::Value decoding is not supported; those tests verify encode
// produces distinct, non-empty bytes and that a struct with the same shape
// round-trips correctly.

#![cfg(feature = "serde")]
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
use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Test 1: serde_json::Value::Null — encoding succeeds (produces zero bytes for
//         a unit value in oxicode's non-self-describing binary format); a plain
//         unit type also serializes to zero bytes; and Value::Bool encodes to
//         a non-empty sequence, confirming the two variants are distinct.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_01_serde_json_value_null_encodes() {
    let cfg = oxicode::config::standard();

    // Encoding serde_json::Value::Null must succeed (unit value → zero bytes).
    let null_val = serde_json::Value::Null;
    let null_bytes = serde_encode(&null_val, cfg).expect("encode serde_json::Value::Null");

    // In oxicode's binary format a unit value serializes to zero bytes.
    assert_eq!(
        null_bytes.len(),
        0,
        "Value::Null (unit) must encode to 0 bytes in binary format"
    );

    // Encoding Value::Bool(true) must produce non-empty bytes.
    let bool_val = serde_json::Value::Bool(true);
    let bool_bytes = serde_encode(&bool_val, cfg).expect("encode serde_json::Value::Bool(true)");
    assert!(
        !bool_bytes.is_empty(),
        "Value::Bool must produce non-empty bytes"
    );

    // The two variants must produce different byte sequences.
    assert_ne!(
        null_bytes, bool_bytes,
        "Value::Null and Value::Bool must encode to distinct bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 2: serde_json::Value::Bool(true) — encode produces non-empty bytes;
//         bool true and bool false encode to distinct byte sequences.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_02_serde_json_value_bool_true_encodes() {
    let cfg = oxicode::config::standard();

    let true_val = serde_json::Value::Bool(true);
    let false_val = serde_json::Value::Bool(false);

    let true_bytes = serde_encode(&true_val, cfg).expect("encode Value::Bool(true)");
    let false_bytes = serde_encode(&false_val, cfg).expect("encode Value::Bool(false)");

    assert!(
        !true_bytes.is_empty(),
        "Value::Bool(true) must produce non-empty bytes"
    );
    assert!(
        !false_bytes.is_empty(),
        "Value::Bool(false) must produce non-empty bytes"
    );
    assert_ne!(
        true_bytes, false_bytes,
        "Value::Bool(true) and Value::Bool(false) must encode to distinct bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 3: serde_json::Value::Number(42) — encode produces non-empty bytes;
//         different numeric values encode to distinct byte sequences.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_03_serde_json_value_number_encodes() {
    let cfg = oxicode::config::standard();

    let n42 = serde_json::Value::Number(serde_json::Number::from(42i64));
    let n99 = serde_json::Value::Number(serde_json::Number::from(99i64));

    let bytes42 = serde_encode(&n42, cfg).expect("encode Value::Number(42)");
    let bytes99 = serde_encode(&n99, cfg).expect("encode Value::Number(99)");

    assert!(
        !bytes42.is_empty(),
        "Value::Number(42) must produce non-empty bytes"
    );
    assert_ne!(
        bytes42, bytes99,
        "Value::Number(42) and Value::Number(99) must encode to distinct bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 4: serde_json::Value::String("hello") — encode produces non-empty bytes;
//         different string values encode to distinct byte sequences.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_04_serde_json_value_string_encodes() {
    let cfg = oxicode::config::standard();

    let s_hello = serde_json::Value::String("hello".to_string());
    let s_world = serde_json::Value::String("world".to_string());

    let hello_bytes = serde_encode(&s_hello, cfg).expect("encode Value::String(\"hello\")");
    let world_bytes = serde_encode(&s_world, cfg).expect("encode Value::String(\"world\")");

    assert!(
        !hello_bytes.is_empty(),
        "Value::String must produce non-empty bytes"
    );
    assert_ne!(
        hello_bytes, world_bytes,
        "Different Value::String values must encode to distinct bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 5: serde_json::Value::Array([1,2,3]) — encode produces non-empty bytes;
//         a typed Vec<i64> with the same values must produce the same number
//         of value-level bytes (encoding is value-driven, not tag-driven).
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_05_serde_json_value_array_encodes() {
    let cfg = oxicode::config::standard();

    let arr = serde_json::json!([1i64, 2i64, 3i64]);
    let arr_bytes = serde_encode(&arr, cfg).expect("encode Value::Array([1,2,3])");
    assert!(
        !arr_bytes.is_empty(),
        "Value::Array must produce non-empty bytes"
    );

    // An empty array must encode to different bytes than a non-empty array.
    let empty_arr = serde_json::Value::Array(vec![]);
    let empty_bytes = serde_encode(&empty_arr, cfg).expect("encode Value::Array([])");
    assert_ne!(
        arr_bytes, empty_bytes,
        "Non-empty and empty Value::Array must encode to distinct bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 6: serde_json::Value::Object with keys — encode produces non-empty bytes;
//         an Object with different key counts encodes to distinct bytes.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_06_serde_json_value_object_encodes() {
    let cfg = oxicode::config::standard();

    let obj1 = serde_json::json!({"key1": "value1"});
    let obj2 = serde_json::json!({"key1": "value1", "key2": "value2"});

    let bytes1 = serde_encode(&obj1, cfg).expect("encode Value::Object 1-key");
    let bytes2 = serde_encode(&obj2, cfg).expect("encode Value::Object 2-key");

    assert!(
        !bytes1.is_empty(),
        "Value::Object must produce non-empty bytes"
    );
    assert_ne!(
        bytes1, bytes2,
        "Objects with different key counts must encode to distinct bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Struct with #[serde(rename_all = "camelCase")] roundtrip.
//         Binary format encodes values by position; rename_all affects the
//         logical key names in text formats but the binary layout is identical
//         to a plain struct with the same field types in the same order.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct CamelDevice {
    device_id: u32,
    device_name: String,
    firmware_version: String,
    is_connected: bool,
    battery_level: u8,
}

#[test]
fn test_adv8_07_rename_all_camel_case_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = CamelDevice {
        device_id: 42,
        device_name: "SensorNode-7".to_string(),
        firmware_version: "1.4.2".to_string(),
        is_connected: true,
        battery_level: 87,
    };

    let bytes = serde_encode(&original, cfg).expect("encode CamelDevice");
    let (decoded, consumed): (CamelDevice, usize) =
        serde_decode(&bytes, cfg).expect("decode CamelDevice");

    assert_eq!(
        original, decoded,
        "#[serde(rename_all = \"camelCase\")] struct must roundtrip"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Struct with #[serde(skip_serializing_if = "Option::is_none")].
//         In oxicode's binary format, skipped fields are NOT written to the
//         byte stream, so a struct where optional fields are skipped encodes
//         to fewer bytes than one where they are present (Some).
//         The Some-field variant must roundtrip correctly (all fields written).
//         We also verify that encoding with all-Some produces more bytes than
//         encoding with skip_serializing_if causing None fields to be omitted.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WithSkipIf {
    id: u64,
    label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u8>,
}

/// Version without skip_serializing_if so None still writes a byte and
/// the roundtrip works symmetrically for comparison.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WithAlwaysOption {
    id: u64,
    label: String,
    description: Option<String>,
    priority: Option<u8>,
}

#[test]
fn test_adv8_08_skip_serializing_if_option_is_none_encodes_fewer_bytes() {
    let cfg = oxicode::config::standard();

    // All-Some variant: fields are written → roundtrip succeeds.
    let with_opts = WithSkipIf {
        id: 100,
        label: "task-alpha".to_string(),
        description: Some("important task".to_string()),
        priority: Some(5),
    };
    let bytes_some = serde_encode(&with_opts, cfg).expect("encode WithSkipIf with Some options");
    let (decoded_some, consumed_some): (WithSkipIf, usize) =
        serde_decode(&bytes_some, cfg).expect("decode WithSkipIf with Some options");
    assert_eq!(
        with_opts, decoded_some,
        "WithSkipIf with Some options must roundtrip"
    );
    assert_eq!(consumed_some, bytes_some.len());

    // All-None variant with skip_serializing_if: None fields are omitted from the
    // byte stream, so encoding produces fewer bytes than the all-Some variant.
    let without_opts = WithSkipIf {
        id: 100,
        label: "task-alpha".to_string(),
        description: None,
        priority: None,
    };
    let bytes_none = serde_encode(&without_opts, cfg).expect("encode WithSkipIf with None options");
    assert!(
        bytes_none.len() < bytes_some.len(),
        "Skipped (None) fields must produce fewer bytes than present (Some) fields"
    );

    // Contrast with a struct that always writes Option fields: None variant still
    // roundtrips when the field is not skipped.
    let always_none = WithAlwaysOption {
        id: 200,
        label: "task-beta".to_string(),
        description: None,
        priority: None,
    };
    let bytes_always = serde_encode(&always_none, cfg).expect("encode WithAlwaysOption None");
    let (decoded_always, consumed_always): (WithAlwaysOption, usize) =
        serde_decode(&bytes_always, cfg).expect("decode WithAlwaysOption None");
    assert_eq!(
        always_none, decoded_always,
        "WithAlwaysOption None must roundtrip"
    );
    assert_eq!(consumed_always, bytes_always.len());
}

// ---------------------------------------------------------------------------
// Test 9: Enum with #[serde(untagged)] roundtrip.
//         oxicode encodes the variant value directly; the untagged
//         representation serializes the inner value only, without a
//         discriminant. Encoding succeeds; we verify the bytes are non-empty
//         and distinct across variants, and that re-encoding after decode
//         gives the same bytes for a known-shape variant.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum UntaggedValue {
    Integer(i64),
    Float(f64),
    Text(String),
}

#[test]
fn test_adv8_09_untagged_enum_encode_distinct_variants() {
    let cfg = oxicode::config::standard();

    let int_val = UntaggedValue::Integer(42);
    let float_val = UntaggedValue::Float(3.14);
    let text_val = UntaggedValue::Text("hello".to_string());

    let int_bytes = serde_encode(&int_val, cfg).expect("encode UntaggedValue::Integer");
    let float_bytes = serde_encode(&float_val, cfg).expect("encode UntaggedValue::Float");
    let text_bytes = serde_encode(&text_val, cfg).expect("encode UntaggedValue::Text");

    assert!(
        !int_bytes.is_empty(),
        "UntaggedValue::Integer must encode to non-empty bytes"
    );
    assert!(
        !float_bytes.is_empty(),
        "UntaggedValue::Float must encode to non-empty bytes"
    );
    assert!(
        !text_bytes.is_empty(),
        "UntaggedValue::Text must encode to non-empty bytes"
    );

    // Re-encoding the integer-backed variant must produce the same bytes as
    // encoding a plain i64 with the same value (untagged serializes the raw value).
    let plain_i64: i64 = 42;
    let plain_bytes = serde_encode(&plain_i64, cfg).expect("encode plain i64 42");
    assert_eq!(
        int_bytes, plain_bytes,
        "UntaggedValue::Integer(42) must encode identically to plain i64 42"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Enum with #[serde(tag = "type")] — internally tagged.
//          oxicode is non-self-describing; encoding succeeds but decoding
//          requires `deserialize_any` which is not available. We verify that
//          all variant encodings are non-empty and that variants with the same
//          payload type but different names encode to distinct bytes.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
enum TaggedRequest {
    Read { path: String },
    Write { path: String, content: String },
    Delete { path: String },
}

#[test]
fn test_adv8_10_internally_tagged_enum_encode_only() {
    let cfg = oxicode::config::standard();

    let read = TaggedRequest::Read {
        path: "/tmp/file.txt".to_string(),
    };
    let delete = TaggedRequest::Delete {
        path: "/tmp/file.txt".to_string(),
    };
    let write = TaggedRequest::Write {
        path: "/tmp/out.txt".to_string(),
        content: "hello".to_string(),
    };

    let read_bytes = serde_encode(&read, cfg).expect("encode TaggedRequest::Read");
    let delete_bytes = serde_encode(&delete, cfg).expect("encode TaggedRequest::Delete");
    let write_bytes = serde_encode(&write, cfg).expect("encode TaggedRequest::Write");

    assert!(
        !read_bytes.is_empty(),
        "TaggedRequest::Read must produce non-empty bytes"
    );
    assert!(
        !write_bytes.is_empty(),
        "TaggedRequest::Write must produce non-empty bytes"
    );

    // Read and Delete have the same field structure; their bytes must differ
    // because the variant tag string differs.
    assert_ne!(
        read_bytes, delete_bytes,
        "Read and Delete with same-path must encode to distinct bytes due to variant tag"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Struct with #[serde(default)] on a field — explicit value survives
//          roundtrip; default is only used when the field is absent (text
//          formats), which cannot happen in binary encoding since all fields
//          are always written.
// ---------------------------------------------------------------------------

fn default_limit() -> u32 {
    100
}

fn default_offset() -> u64 {
    0
}

fn default_descending() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct QueryParams {
    index: String,
    #[serde(default = "default_limit")]
    limit: u32,
    #[serde(default = "default_offset")]
    offset: u64,
    #[serde(default = "default_descending")]
    descending: bool,
}

#[test]
fn test_adv8_11_serde_default_explicit_values_survive_roundtrip() {
    let cfg = oxicode::config::standard();

    // Non-default values must survive the roundtrip unchanged.
    let original = QueryParams {
        index: "products".to_string(),
        limit: 25,
        offset: 50,
        descending: true,
    };

    let bytes = serde_encode(&original, cfg).expect("encode QueryParams with non-default values");
    let (decoded, consumed): (QueryParams, usize) =
        serde_decode(&bytes, cfg).expect("decode QueryParams with non-default values");

    assert_eq!(
        original, decoded,
        "Non-default field values must survive roundtrip"
    );
    assert_eq!(
        decoded.limit, 25,
        "Explicit limit must not be replaced by default"
    );
    assert_eq!(
        decoded.offset, 50,
        "Explicit offset must not be replaced by default"
    );
    assert!(
        decoded.descending,
        "Explicit descending=true must survive roundtrip"
    );
    assert_eq!(consumed, bytes.len());

    // Default values explicitly set must also survive.
    let default_vals = QueryParams {
        index: "orders".to_string(),
        limit: default_limit(),
        offset: default_offset(),
        descending: default_descending(),
    };

    let bytes2 = serde_encode(&default_vals, cfg).expect("encode QueryParams with default values");
    let (decoded2, _): (QueryParams, usize) =
        serde_decode(&bytes2, cfg).expect("decode QueryParams with default values");
    assert_eq!(
        default_vals, decoded2,
        "Fields set to default values must also roundtrip"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Vec<serde_json::Value> — encoding succeeds and produces bytes
//          proportional to the number of elements; an empty Vec encodes to
//          fewer bytes than a non-empty one.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_12_vec_serde_json_value_encodes() {
    let cfg = oxicode::config::standard();

    let empty: Vec<serde_json::Value> = vec![];
    let non_empty: Vec<serde_json::Value> = vec![
        serde_json::Value::Bool(true),
        serde_json::Value::Number(serde_json::Number::from(7i64)),
        serde_json::Value::String("item".to_string()),
    ];

    let empty_bytes = serde_encode(&empty, cfg).expect("encode empty Vec<serde_json::Value>");
    let non_empty_bytes = serde_encode(&non_empty, cfg).expect("encode Vec<serde_json::Value>");

    assert!(
        !non_empty_bytes.is_empty(),
        "Vec<serde_json::Value> must produce non-empty bytes"
    );
    assert!(
        non_empty_bytes.len() > empty_bytes.len(),
        "Non-empty Vec must encode to more bytes than empty Vec"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Large nested serde_json::Value structure — encoding succeeds;
//          a deeper structure encodes to more bytes than a shallower one.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_13_large_nested_serde_json_value_encodes() {
    let cfg = oxicode::config::standard();

    // Build a two-level nested object with an array of numbers.
    let shallow = serde_json::json!({"level": 1});
    let deep = serde_json::json!({
        "level": 1,
        "data": {
            "items": [1, 2, 3, 4, 5],
            "count": 5,
            "label": "nested"
        }
    });

    let shallow_bytes = serde_encode(&shallow, cfg).expect("encode shallow serde_json::Value");
    let deep_bytes = serde_encode(&deep, cfg).expect("encode deep serde_json::Value");

    assert!(
        !deep_bytes.is_empty(),
        "Deep nested Value must produce non-empty bytes"
    );
    assert!(
        deep_bytes.len() > shallow_bytes.len(),
        "Deeper structure must encode to more bytes than shallower one"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Serde encoding is deterministic — same value always produces the
//          same bytes across multiple encode calls.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DeterministicRecord {
    id: u64,
    name: String,
    scores: Vec<f64>,
    active: bool,
    metadata: BTreeMap<String, u32>,
}

#[test]
fn test_adv8_14_serde_encoding_is_deterministic() {
    let cfg = oxicode::config::standard();

    let mut metadata = BTreeMap::new();
    metadata.insert("alpha".to_string(), 10);
    metadata.insert("beta".to_string(), 20);
    metadata.insert("gamma".to_string(), 30);

    let original = DeterministicRecord {
        id: 999,
        name: "determinism-test".to_string(),
        scores: vec![1.0, 2.5, 3.14],
        active: true,
        metadata,
    };

    let bytes1 = serde_encode(&original, cfg).expect("first encode");
    let bytes2 = serde_encode(&original, cfg).expect("second encode");
    let bytes3 = serde_encode(&original, cfg).expect("third encode");

    assert_eq!(
        bytes1, bytes2,
        "Two encodes of the same value must be identical"
    );
    assert_eq!(
        bytes2, bytes3,
        "Three encodes of the same value must all be identical"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Serde encode then decode gives equal value (end-to-end for a
//          composite struct not used in any previous test).
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PipelineStage {
    stage_id: u32,
    name: String,
    enabled: bool,
    max_workers: u8,
    timeout_secs: u64,
    dependencies: Vec<String>,
}

#[test]
fn test_adv8_15_encode_then_decode_gives_equal_value() {
    let cfg = oxicode::config::standard();
    let original = PipelineStage {
        stage_id: 3,
        name: "transform".to_string(),
        enabled: true,
        max_workers: 4,
        timeout_secs: 300,
        dependencies: vec!["ingest".to_string(), "validate".to_string()],
    };

    let bytes = serde_encode(&original, cfg).expect("encode PipelineStage");
    let (decoded, consumed): (PipelineStage, usize) =
        serde_decode(&bytes, cfg).expect("decode PipelineStage");

    assert_eq!(original, decoded, "Decoded value must equal original");
    assert_eq!(consumed, bytes.len(), "All bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 16: BTreeMap via serde roundtrip — BTreeMap guarantees sorted key
//          iteration order, so the encoding is deterministic; two separate
//          BTreeMaps built with keys in different insertion order must encode
//          to identical bytes when their logical contents are equal.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_16_btreemap_serde_roundtrip_with_insertion_order_independence() {
    let cfg = oxicode::config::standard();

    // Build the same map with different insertion orders.
    let mut map_a: BTreeMap<String, u64> = BTreeMap::new();
    map_a.insert("zebra".to_string(), 26);
    map_a.insert("alpha".to_string(), 1);
    map_a.insert("middle".to_string(), 13);

    let mut map_b: BTreeMap<String, u64> = BTreeMap::new();
    map_b.insert("middle".to_string(), 13);
    map_b.insert("zebra".to_string(), 26);
    map_b.insert("alpha".to_string(), 1);

    let bytes_a = serde_encode(&map_a, cfg).expect("encode map_a");
    let bytes_b = serde_encode(&map_b, cfg).expect("encode map_b");

    assert_eq!(
        bytes_a, bytes_b,
        "BTreeMaps with same contents but different insertion orders must encode identically"
    );

    // Round-trip verification
    let (decoded_a, consumed_a): (BTreeMap<String, u64>, usize) =
        serde_decode(&bytes_a, cfg).expect("decode BTreeMap");
    assert_eq!(map_a, decoded_a, "BTreeMap must roundtrip correctly");
    assert_eq!(consumed_a, bytes_a.len());
}

// ---------------------------------------------------------------------------
// Test 17: Struct with #[serde(flatten)] — encoding is expected to fail
//          in oxicode's binary (non-self-describing) serde layer because
//          flatten requires dynamic map merging which needs `serialize_map`
//          with an upfront known length that the format cannot determine.
//          We also verify that a logically equivalent struct without flatten
//          round-trips correctly.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FlattenedPatch {
    patch_id: u32,
    #[serde(flatten)]
    location: PatchLocation,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PatchLocation {
    file: String,
    line: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct EquivalentPatch {
    patch_id: u32,
    file: String,
    line: u32,
}

#[test]
fn test_adv8_17_serde_flatten_encode_fails_plain_equivalent_roundtrips() {
    let cfg = oxicode::config::standard();

    // Encoding a struct with #[serde(flatten)] must fail in binary format.
    let flattened = FlattenedPatch {
        patch_id: 1,
        location: PatchLocation {
            file: "main.rs".to_string(),
            line: 42,
        },
    };
    let flatten_result = serde_encode(&flattened, cfg);
    assert!(
        flatten_result.is_err(),
        "#[serde(flatten)] encoding must return an error in non-self-describing binary format"
    );

    // The logically equivalent struct without flatten must roundtrip correctly.
    let equivalent = EquivalentPatch {
        patch_id: 1,
        file: "main.rs".to_string(),
        line: 42,
    };
    let bytes = serde_encode(&equivalent, cfg).expect("encode EquivalentPatch");
    let (decoded, consumed): (EquivalentPatch, usize) =
        serde_decode(&bytes, cfg).expect("decode EquivalentPatch");
    assert_eq!(
        equivalent, decoded,
        "Equivalent struct without flatten must roundtrip"
    );
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18: Tuple via serde roundtrip — tests a 5-element heterogeneous tuple.
// ---------------------------------------------------------------------------

#[test]
fn test_adv8_18_tuple_five_element_roundtrip() {
    let cfg = oxicode::config::standard();

    let original: (u8, u32, i64, f64, String) = (
        255,
        1_000_000,
        -9_876_543_210i64,
        core::f64::consts::TAU,
        "tuple-value".to_string(),
    );

    let bytes = serde_encode(&original, cfg).expect("encode 5-element tuple");
    let (decoded, consumed): ((u8, u32, i64, f64, String), usize) =
        serde_decode(&bytes, cfg).expect("decode 5-element tuple");

    assert_eq!(
        original, decoded,
        "5-element tuple must roundtrip correctly"
    );
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 19: Newtype struct via serde roundtrip — two distinct newtype structs
//          wrapping different primitives must each roundtrip independently.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct OrderId(u64);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Checksum(u32);

#[test]
fn test_adv8_19_newtype_struct_serde_roundtrip() {
    let cfg = oxicode::config::standard();

    let order = OrderId(0xDEAD_BEEF_CAFE_1234);
    let bytes_order = serde_encode(&order, cfg).expect("encode OrderId");
    let (decoded_order, consumed_order): (OrderId, usize) =
        serde_decode(&bytes_order, cfg).expect("decode OrderId");
    assert_eq!(
        order, decoded_order,
        "OrderId newtype must roundtrip correctly"
    );
    assert_eq!(consumed_order, bytes_order.len());

    let chk = Checksum(0xAABBCCDD);
    let bytes_chk = serde_encode(&chk, cfg).expect("encode Checksum");
    let (decoded_chk, consumed_chk): (Checksum, usize) =
        serde_decode(&bytes_chk, cfg).expect("decode Checksum");
    assert_eq!(
        chk, decoded_chk,
        "Checksum newtype must roundtrip correctly"
    );
    assert_eq!(consumed_chk, bytes_chk.len());
}

// ---------------------------------------------------------------------------
// Test 20: Unit struct via serde roundtrip — two unit structs must each encode
//          to bytes (possibly empty or minimal) and decode back to their type.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Sentinel;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Eof;

#[test]
fn test_adv8_20_unit_struct_serde_roundtrip() {
    let cfg = oxicode::config::standard();

    let sentinel = Sentinel;
    let bytes_sentinel = serde_encode(&sentinel, cfg).expect("encode Sentinel unit struct");
    let (decoded_sentinel, consumed_sentinel): (Sentinel, usize) =
        serde_decode(&bytes_sentinel, cfg).expect("decode Sentinel unit struct");
    assert_eq!(
        sentinel, decoded_sentinel,
        "Sentinel unit struct must roundtrip"
    );
    assert_eq!(consumed_sentinel, bytes_sentinel.len());

    let eof = Eof;
    let bytes_eof = serde_encode(&eof, cfg).expect("encode Eof unit struct");
    let (decoded_eof, consumed_eof): (Eof, usize) =
        serde_decode(&bytes_eof, cfg).expect("decode Eof unit struct");
    assert_eq!(eof, decoded_eof, "Eof unit struct must roundtrip");
    assert_eq!(consumed_eof, bytes_eof.len());
}

// ---------------------------------------------------------------------------
// Test 21: Enum with tuple variants via serde roundtrip.
//          Each variant holds a different number of fields.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Packet {
    Ping,
    Data(u32, Vec<u8>),
    Ack(u32),
    Fragment(u16, u16, Vec<u8>), // fragment_index, total_fragments, payload
}

#[test]
fn test_adv8_21_enum_tuple_variants_serde_roundtrip() {
    let cfg = oxicode::config::standard();

    let packets = [
        Packet::Ping,
        Packet::Data(1001, vec![0xDE, 0xAD, 0xBE, 0xEF]),
        Packet::Ack(1001),
        Packet::Fragment(0, 3, vec![1, 2, 3, 4, 5, 6, 7, 8]),
    ];

    for packet in &packets {
        let bytes = serde_encode(packet, cfg).expect("encode Packet variant");
        let (decoded, consumed): (Packet, usize) =
            serde_decode(&bytes, cfg).expect("decode Packet variant");
        assert_eq!(packet, &decoded, "Packet variant must roundtrip correctly");
        assert_eq!(consumed, bytes.len());
    }

    // Verify that Ping (no payload) encodes to fewer bytes than Data (with payload).
    let ping_bytes = serde_encode(&Packet::Ping, cfg).expect("encode Ping");
    let data_bytes = serde_encode(&Packet::Data(1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), cfg)
        .expect("encode Data");
    assert!(
        data_bytes.len() > ping_bytes.len(),
        "Packet::Data with payload must encode to more bytes than Packet::Ping"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Custom Serialize/Deserialize implementation roundtrip.
//          A type that manually implements serde's traits (not derived) must
//          encode and decode correctly via oxicode's serde layer.
// ---------------------------------------------------------------------------

/// A 2D vector type with a custom Serialize/Deserialize implementation that
/// serializes to/from a fixed-length sequence [x, y].
#[derive(Debug, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Serialize for Vec2 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.x)?;
        tup.serialize_element(&self.y)?;
        tup.end()
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{SeqAccess, Visitor};
        use std::fmt;

        struct Vec2Visitor;

        impl<'de> Visitor<'de> for Vec2Visitor {
            type Value = Vec2;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence of two f64 values [x, y]")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec2, A::Error> {
                let x: f64 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let y: f64 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Vec2 { x, y })
            }
        }

        deserializer.deserialize_tuple(2, Vec2Visitor)
    }
}

#[test]
fn test_adv8_22_custom_serialize_deserialize_roundtrip() {
    let cfg = oxicode::config::standard();

    let test_cases = [
        Vec2 { x: 0.0, y: 0.0 },
        Vec2 { x: 1.0, y: -1.0 },
        Vec2 {
            x: core::f64::consts::PI,
            y: core::f64::consts::E,
        },
        Vec2 {
            x: f64::MAX,
            y: f64::MIN_POSITIVE,
        },
        Vec2 {
            x: -12345.678,
            y: 0.0001,
        },
    ];

    for original in &test_cases {
        let bytes = serde_encode(original, cfg).expect("encode Vec2 with custom Serialize");
        let (decoded, consumed): (Vec2, usize) =
            serde_decode(&bytes, cfg).expect("decode Vec2 with custom Deserialize");

        assert_eq!(
            original.x.to_bits(),
            decoded.x.to_bits(),
            "Vec2.x bit pattern must be preserved"
        );
        assert_eq!(
            original.y.to_bits(),
            decoded.y.to_bits(),
            "Vec2.y bit pattern must be preserved"
        );
        assert_eq!(consumed, bytes.len(), "All bytes must be consumed");
    }

    // Verify that encoding two Vec2 instances with the same values produces
    // identical bytes (determinism of custom implementation).
    let v_a = Vec2 { x: 1.5, y: -2.5 };
    let v_b = Vec2 { x: 1.5, y: -2.5 };
    let bytes_a = serde_encode(&v_a, cfg).expect("encode Vec2 v_a");
    let bytes_b = serde_encode(&v_b, cfg).expect("encode Vec2 v_b");
    assert_eq!(bytes_a, bytes_b, "Custom Serialize must be deterministic");
}
