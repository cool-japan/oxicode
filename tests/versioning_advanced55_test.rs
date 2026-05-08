//! Advanced/large-payload-focused tests for the versioning module (split from versioning_test.rs).

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
use oxicode::versioning::Version;

/// 12. test_versioned_large_payload
///
/// Migrate a struct that contains a 100-element Vec<u32>.  Verifies that the
/// versioned encode/decode pipeline handles payloads significantly larger than
/// the 11-byte header without corruption.
#[cfg(feature = "derive")]
#[test]
fn test_versioned_large_payload() {
    use oxicode::versioning::Version;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LargeRecord {
        id: u64,
        values: Vec<u32>,
    }

    let version = Version::new(1, 0, 0);
    let original = LargeRecord {
        id: 12345,
        values: (0u32..100).collect(),
    };

    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode");
    let (decoded, ver, consumed): (LargeRecord, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    // The versioned envelope must be larger than the plain encoded form.
    let plain = oxicode::encode_to_vec(&original).expect("plain encode");
    assert!(encoded.len() > plain.len());
}

/// NEW-3. encode_versioned_value + decode_versioned_value roundtrip for a
/// large struct with many fields (derive feature).
#[cfg(feature = "derive")]
#[test]
fn test_encode_versioned_value_large_struct_roundtrip() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct BigRecord {
        id: u64,
        name: String,
        score: f64,
        active: bool,
        tags: Vec<String>,
        counts: Vec<u32>,
    }

    let version = Version::new(3, 2, 1);
    let original = BigRecord {
        id: 999_999_999,
        name: "large record test".to_string(),
        score: 1234.5678_f64,
        active: true,
        tags: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
        counts: (0u32..50).collect(),
    };

    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, consumed): (BigRecord, _, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");

    assert_eq!(decoded, original, "decoded value must equal original");
    assert_eq!(ver, version, "decoded version must match encoded version");
    assert!(consumed > 0, "consumed must be positive");
}

/// NEW-6. encode_versioned_value for Vec<String>.
///
/// Verifies that a heap-allocated collection of strings can be versioned
/// and recovered without corruption.
#[test]
fn test_encode_versioned_value_vec_string() {
    let version = Version::new(2, 0, 0);
    let original: Vec<String> = vec![
        "hello".to_string(),
        "versioned".to_string(),
        "world".to_string(),
        "".to_string(), // empty string edge case
    ];
    let encoded =
        oxicode::encode_versioned_value(&original, version).expect("encode Vec<String> failed");
    let (decoded, ver, _): (Vec<String>, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode Vec<String> failed");

    assert_eq!(
        decoded, original,
        "Vec<String> must survive versioned roundtrip"
    );
    assert_eq!(ver, version);
}

/// NEW-7. encode_versioned_value for nested structs (derive feature).
///
/// A three-level nesting (outer → middle → inner) must encode and decode
/// correctly through the versioned API.
#[cfg(feature = "derive")]
#[test]
fn test_encode_versioned_value_nested_structs() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct InnerNode {
        value: u64,
        label: String,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MiddleNode {
        children: Vec<InnerNode>,
        count: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct OuterNode {
        name: String,
        middle: MiddleNode,
        active: bool,
    }

    let version = Version::new(1, 5, 0);
    let original = OuterNode {
        name: "root".to_string(),
        middle: MiddleNode {
            children: vec![
                InnerNode {
                    value: 1,
                    label: "first".to_string(),
                },
                InnerNode {
                    value: 2,
                    label: "second".to_string(),
                },
            ],
            count: 2,
        },
        active: true,
    };

    let encoded =
        oxicode::encode_versioned_value(&original, version).expect("encode nested structs failed");
    let (decoded, ver, _): (OuterNode, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode nested structs failed");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

/// NEW-8. Multiple sequential versioned encode/decode operations.
///
/// Performs 5 consecutive encode/decode cycles on different values and
/// versions, verifying that each round-trip is independent.
#[test]
fn test_multiple_sequential_versioned_operations() {
    let pairs: &[(u64, Version)] = &[
        (0, Version::new(1, 0, 0)),
        (1, Version::new(1, 1, 0)),
        (u64::MAX, Version::new(2, 0, 0)),
        (42, Version::new(0, 5, 3)),
        (100, Version::new(10, 0, 255)),
    ];

    for (value, version) in pairs {
        let encoded =
            oxicode::encode_versioned_value(value, *version).expect("encode in sequence failed");
        let (decoded, ver, _): (u64, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode in sequence failed");
        assert_eq!(decoded, *value, "sequential value mismatch");
        assert_eq!(ver, *version, "sequential version mismatch");
    }
}

/// NEW-9. Versioned encoding of primitives: u64, bool, String.
///
/// Each primitive is independently encoded with a distinct version and
/// decoded back, verifying the type-level roundtrip for all three cases.
#[test]
fn test_versioned_primitives_u64_bool_string() {
    // u64
    {
        let v = Version::new(1, 0, 0);
        let encoded = oxicode::encode_versioned_value(&u64::MAX, v).expect("encode u64");
        let (val, ver, _): (u64, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode u64");
        assert_eq!(val, u64::MAX);
        assert_eq!(ver, v);
    }
    // bool
    {
        let v = Version::new(2, 1, 0);
        for b in [true, false] {
            let encoded = oxicode::encode_versioned_value(&b, v).expect("encode bool");
            let (val, ver, _): (bool, _, _) =
                oxicode::decode_versioned_value(&encoded).expect("decode bool");
            assert_eq!(val, b);
            assert_eq!(ver, v);
        }
    }
    // String
    {
        let v = Version::new(3, 0, 7);
        let s = "hello versioned string".to_string();
        let encoded = oxicode::encode_versioned_value(&s, v).expect("encode String");
        let (val, ver, _): (String, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode String");
        assert_eq!(val, s);
        assert_eq!(ver, v);
    }
}

/// NEW-12. encode_versioned_value produces more bytes than plain encode_to_vec.
///
/// The version header overhead (11 bytes) must make the versioned output
/// strictly larger than the plain serialised form.
#[test]
fn test_versioned_value_larger_than_plain() {
    let value = 12345u64;
    let version = Version::new(1, 0, 0);

    let plain = oxicode::encode_to_vec(&value).expect("plain encode");
    let versioned = oxicode::encode_versioned_value(&value, version).expect("versioned encode");

    assert!(
        versioned.len() > plain.len(),
        "versioned output ({} bytes) must be larger than plain ({} bytes)",
        versioned.len(),
        plain.len()
    );
    // The difference should be exactly the 11-byte header.
    assert_eq!(
        versioned.len() - plain.len(),
        11,
        "version header overhead must be exactly 11 bytes"
    );
}

/// NEW-15. Multiple structs each with different version numbers.
///
/// Three distinct structs are each encoded with their own version.  After
/// decoding, both the struct data and the stored version must match
/// what was supplied at encoding time.
#[cfg(feature = "derive")]
#[test]
fn test_multiple_structs_different_versions() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TypeA {
        x: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TypeB {
        name: String,
        count: u64,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TypeC {
        flag: bool,
        values: Vec<u8>,
    }

    let va = Version::new(1, 0, 0);
    let vb = Version::new(2, 3, 0);
    let vc = Version::new(0, 9, 5);

    let a = TypeA { x: 42 };
    let b = TypeB {
        name: "struct_b".to_string(),
        count: 100,
    };
    let c = TypeC {
        flag: false,
        values: vec![10, 20, 30],
    };

    let enc_a = oxicode::encode_versioned_value(&a, va).expect("encode A");
    let enc_b = oxicode::encode_versioned_value(&b, vb).expect("encode B");
    let enc_c = oxicode::encode_versioned_value(&c, vc).expect("encode C");

    let (da, vera, _): (TypeA, _, _) = oxicode::decode_versioned_value(&enc_a).expect("decode A");
    let (db, verb, _): (TypeB, _, _) = oxicode::decode_versioned_value(&enc_b).expect("decode B");
    let (dc, verc, _): (TypeC, _, _) = oxicode::decode_versioned_value(&enc_c).expect("decode C");

    assert_eq!(da, a);
    assert_eq!(vera, va);
    assert_eq!(db, b);
    assert_eq!(verb, vb);
    assert_eq!(dc, c);
    assert_eq!(verc, vc);
}

/// NEW-16. Decode versioned then re-encode to a different version.
///
/// A value encoded as V1 is decoded, then immediately re-encoded as V2.
/// The re-encoded bytes must have the V2 header and decode correctly.
#[test]
fn test_decode_versioned_then_reencode_different_version() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    let original = 55555u32;
    let enc_v1 = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // Decode from V1 encoding.
    let (val, ver1, _): (u32, _, _) = oxicode::decode_versioned_value(&enc_v1).expect("decode v1");
    assert_eq!(val, original);
    assert_eq!(ver1, v1);

    // Re-encode as V2.
    let enc_v2 = oxicode::encode_versioned_value(&val, v2).expect("re-encode as v2");
    let (val2, ver2, _): (u32, _, _) = oxicode::decode_versioned_value(&enc_v2).expect("decode v2");

    assert_eq!(
        val2, original,
        "value must be unchanged after version migration"
    );
    assert_eq!(ver2, v2, "re-encoded data must carry V2 header");
    assert_ne!(enc_v1, enc_v2, "V1 and V2 encoded bytes must differ");
}

/// NEW-19. Batch: encode 100 items with version, decode all back.
///
/// Produces 100 individual versioned payloads (one per u64 value) and decodes
/// each one, confirming that all values and version numbers are preserved.
#[test]
fn test_batch_100_items_versioned() {
    let version = Version::new(1, 0, 0);

    let mut encoded_items: Vec<Vec<u8>> = Vec::with_capacity(100);
    for i in 0u64..100 {
        let enc = oxicode::encode_versioned_value(&i, version).expect("batch encode");
        encoded_items.push(enc);
    }

    for (i, enc) in encoded_items.iter().enumerate() {
        let (val, ver, _): (u64, _, _) =
            oxicode::decode_versioned_value(enc).expect("batch decode");
        assert_eq!(val, i as u64, "batch item {i}: value mismatch");
        assert_eq!(ver, version, "batch item {i}: version mismatch");
    }
}
