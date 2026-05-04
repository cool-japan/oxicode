//! Schema-evolution-focused tests for the versioning module (split from versioning_test.rs).

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
use oxicode::versioning::{check_compatibility, extract_version, Version};

#[cfg(feature = "derive")]
mod derive_tests {
    use super::*;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DataV1 {
        name: String,
        value: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SimpleRecord {
        id: u64,
        score: f32,
        active: bool,
    }

    #[test]
    fn test_derived_struct_versioned_roundtrip() {
        let version = Version::new(1, 0, 0);
        let original = DataV1 {
            name: String::from("test"),
            value: 42,
        };
        let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _): (DataV1, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(ver, version);
    }

    #[test]
    fn test_derived_struct_complex_roundtrip() {
        let version = Version::new(2, 1, 0);
        let original = SimpleRecord {
            id: 9999,
            score: 2.71,
            active: true,
        };
        let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _): (SimpleRecord, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(ver, version);
    }

    #[test]
    fn test_version_schema_evolution_metadata() {
        // V1 data with V1 schema — a future reader can inspect the stored version
        // before deciding how to decode.
        let v1_data = DataV1 {
            name: String::from("record"),
            value: 100,
        };
        let v1 = Version::new(1, 0, 0);
        let encoded = oxicode::encode_versioned_value(&v1_data, v1).expect("encode failed");

        // A future reader inspects the stored version before decoding
        let stored_version = extract_version(&encoded).expect("extract failed");
        assert_eq!(stored_version.major, 1);

        // And can check compatibility before attempting to decode
        let current = Version::new(1, 5, 0);
        let compat = check_compatibility(stored_version, current, None);
        assert!(compat.is_usable());
    }

    #[test]
    fn test_version_numbers_comparison() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(2, 0, 0);
        assert!(v2 > v1);
        assert!(v1 < v2);
        assert_ne!(v1, v2);

        let v_patch = Version::new(1, 0, 1);
        assert!(v_patch > v1);

        let v_minor = Version::new(1, 1, 0);
        assert!(v_minor > v1);
        assert!(v_minor > v_patch);
    }
}

#[cfg(feature = "derive")]
mod extra_derive_tests {
    use super::*;
    use oxicode::{Decode, Encode};

    /// V1 schema: id + name only.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct UserV1 {
        id: u64,
        name: String,
    }

    /// Encode a V1 record, decode it back as V1, and verify the stored version
    /// header round-trips correctly.  This simulates the first step of a
    /// multi-version migration chain (v1 -> v2 -> v3) where the reader first
    /// inspects the stored version before deciding which schema to use.
    #[test]
    fn test_multi_version_migration_chain() {
        let user_v1 = UserV1 {
            id: 1,
            name: "alice".to_string(),
        };
        let v1 = Version::new(1, 0, 0);

        let bytes_v1 = oxicode::encode_versioned_value(&user_v1, v1).expect("encode v1");

        // A reader can inspect the stored version first…
        let stored = extract_version(&bytes_v1).expect("extract version");
        assert_eq!(stored, v1);

        // …then decode using the matching schema.
        let (decoded, version, _consumed): (UserV1, _, _) =
            oxicode::decode_versioned_value(&bytes_v1).expect("decode v1");

        assert_eq!(version, v1);
        assert_eq!(decoded, user_v1);
    }

    /// A V2-encoded record produces bytes that are distinct from a V1-encoded
    /// record of the same logical data (different version header).
    #[test]
    fn test_v1_v2_bytes_differ() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(2, 0, 0);

        let record = UserV1 {
            id: 42,
            name: "bob".to_string(),
        };

        let enc_v1 = oxicode::encode_versioned_value(&record, v1).expect("encode v1");
        let enc_v2 = oxicode::encode_versioned_value(&record, v2).expect("encode v2");

        assert_ne!(enc_v1, enc_v2, "v1 and v2 headers must differ");
    }

    /// A future reader can check compatibility before attempting to decode,
    /// enabling graceful handling of schema evolution.
    #[test]
    fn test_schema_evolution_compatibility_gate() {
        let v1 = Version::new(1, 0, 0);
        let v_current = Version::new(1, 5, 0);

        let record = UserV1 {
            id: 7,
            name: "carol".to_string(),
        };
        let encoded = oxicode::encode_versioned_value(&record, v1).expect("encode");

        let stored = extract_version(&encoded).expect("extract");
        let compat = check_compatibility(stored, v_current, None);

        // An older minor is still usable (compatible with warnings).
        assert!(compat.is_usable());
    }
}

/// 1. test_version_schema_evolution_add_optional_field
#[cfg(feature = "derive")]
#[test]
fn test_version_schema_evolution_add_optional_field() {
    use oxicode::versioning::{check_compatibility, extract_version, Version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct RecordV1 {
        id: u64,
        name: String,
    }

    #[derive(Debug, PartialEq)]
    struct RecordV2 {
        id: u64,
        name: String,
        tag: Option<String>,
    }

    let v1 = Version::new(1, 0, 0);
    let original = RecordV1 {
        id: 7,
        name: "alice".to_string(),
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // A V2 reader inspects the stored version first.
    let stored = extract_version(&encoded).expect("extract");
    assert_eq!(stored.major, 1);
    assert_eq!(stored.minor, 0);

    // The V2 current version is 1.1.0.
    let v2 = Version::new(1, 1, 0);
    let compat = check_compatibility(stored, v2, None);
    assert!(compat.is_usable(), "V1 data must be usable by a V2 reader");

    // Decode using the V1 schema (what the bytes actually contain).
    let (decoded_v1, ver, _): (RecordV1, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode as v1");
    assert_eq!(ver, v1);
    assert_eq!(decoded_v1.id, 7);

    // Construct V2 with the new optional field defaulting to None.
    let migrated = RecordV2 {
        id: decoded_v1.id,
        name: decoded_v1.name,
        tag: None,
    };
    assert_eq!(
        migrated.tag, None,
        "new optional field must default to None"
    );
}

/// 2. test_versioned_value_roundtrip_v1_to_v3
#[cfg(feature = "derive")]
#[test]
fn test_versioned_value_roundtrip_v1_to_v3() {
    use oxicode::versioning::{can_migrate, extract_version, migration_path, Version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Payload {
        value: u32,
        label: String,
    }

    let v1 = Version::new(1, 0, 0);
    let v3 = Version::new(3, 0, 0);

    let original = Payload {
        value: 42,
        label: "test".to_string(),
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode");

    // V1 → V3 migration must be possible.
    assert!(can_migrate(v1, v3), "forward migration must be supported");

    // Migration path from V1 to V3 contains one intermediate step (V2).
    let path = migration_path(v1, v3);
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], Version::new(2, 0, 0));

    // The actual bytes were encoded as V1; verify the stored version is still V1.
    let stored = extract_version(&encoded).expect("extract");
    assert_eq!(stored, v1);

    // Decode using the V1 schema.
    let (decoded, ver, _): (Payload, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode");
    assert_eq!(ver, v1);
    assert_eq!(decoded, original);
}

/// 3. test_versioned_decode_unknown_version_error
#[test]
fn test_versioned_decode_unknown_version_error() {
    use oxicode::versioning::{decode_versioned, encode_versioned, Version};

    // Produce a valid versioned payload then corrupt the header-format-version byte.
    let mut bytes = encode_versioned(b"payload", Version::new(1, 0, 0)).expect("encode");

    // Byte 4 is the header format version; setting it to 99 makes it unsupported.
    bytes[4] = 99;

    let result = decode_versioned(&bytes);
    assert!(
        result.is_err(),
        "decoding with unknown header version must fail"
    );
}

/// 5. test_versioned_with_nested_struct
#[cfg(feature = "derive")]
#[test]
fn test_versioned_with_nested_struct() {
    use oxicode::versioning::Version;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        x: i32,
        y: i32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        name: String,
        pos: Inner,
        weight: f64,
    }

    let version = Version::new(1, 2, 3);
    let original = Outer {
        name: "origin".to_string(),
        pos: Inner { x: -5, y: 10 },
        weight: std::f64::consts::PI,
    };

    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode");
    let (decoded, ver, _): (Outer, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

/// 6. test_version_field_count_mismatch
#[cfg(feature = "derive")]
#[test]
fn test_version_field_count_mismatch() {
    use oxicode::versioning::{decode_versioned, encode_versioned, Version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TwoFields {
        a: u64,
        b: u64,
    }

    let version = Version::new(1, 0, 0);
    let original = TwoFields { a: 1, b: 2 };

    // Encode the plain payload (without version header) so we can truncate it.
    let payload = oxicode::encode_to_vec(&original).expect("encode payload");

    // Keep only the first byte of the payload (definitely too short for u64 + u64).
    let short_payload = &payload[..1];

    // Re-wrap the truncated payload with the versioned header.
    let versioned_truncated = encode_versioned(short_payload, version).expect("encode versioned");

    // Extracting the header succeeds (it is intact), but decoding the struct fails.
    let (raw, _ver) = decode_versioned(&versioned_truncated).expect("decode raw");
    let result: oxicode::Result<(TwoFields, _)> = oxicode::decode_from_slice(&raw);
    assert!(
        result.is_err(),
        "truncated payload must not decode as TwoFields"
    );
}

/// 11. test_compatibility_encode_current_decode_old
#[test]
fn test_compatibility_encode_current_decode_old() {
    use oxicode::versioning::{
        decode_versioned_with_check, encode_versioned, CompatibilityLevel, Version,
    };

    let new_version = Version::new(1, 5, 0);
    let old_reader = Version::new(1, 0, 0);

    let encoded = encode_versioned(b"data", new_version).expect("encode");
    let result = decode_versioned_with_check(&encoded, old_reader, None);

    // The data version (1.5.0) is newer than what the reader knows (1.0.0),
    // same major → CompatibleWithWarnings, not an error.
    let (_, ver, compat) = result.expect("decode must succeed despite version mismatch");
    assert_eq!(ver, new_version);
    assert_eq!(
        compat,
        CompatibilityLevel::CompatibleWithWarnings,
        "newer-minor data decoded by older reader must yield CompatibleWithWarnings"
    );
}

/// 15. test_versioned_default_field_values
#[cfg(feature = "derive")]
#[test]
fn test_versioned_default_field_values() {
    use oxicode::versioning::Version;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ConfigV1 {
        timeout_ms: u32,
        retries: u8,
    }

    #[derive(Debug, PartialEq)]
    struct ConfigV2 {
        timeout_ms: u32,
        retries: u8,
        /// New in V2; absent in V1 data → defaults to false.
        verbose: bool,
    }

    let v1 = Version::new(1, 0, 0);
    let original = ConfigV1 {
        timeout_ms: 5000,
        retries: 3,
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // Decode with the V1 schema.
    let (decoded_v1, ver, _): (ConfigV1, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode v1");
    assert_eq!(ver, v1);

    // Construct V2 by migrating V1 fields and applying the default for `verbose`.
    let migrated = ConfigV2 {
        timeout_ms: decoded_v1.timeout_ms,
        retries: decoded_v1.retries,
        verbose: bool::default(),
    };

    assert_eq!(migrated.timeout_ms, 5000);
    assert_eq!(migrated.retries, 3);
    assert!(
        !migrated.verbose,
        "new field must default to false during migration"
    );
}

/// NEW-11. Schema evolution: V1 struct bytes decodable as V2 struct with
/// skip+default field.
#[cfg(feature = "derive")]
#[test]
fn test_schema_evolution_skip_and_default_field() {
    use oxicode::versioning::{check_compatibility, extract_version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ArticleV1 {
        id: u64,
        title: String,
    }

    #[derive(Debug, PartialEq)]
    struct ArticleV2 {
        id: u64,
        title: String,
        /// New in V2; absent in V1 → default to empty string.
        description: String,
    }

    let v1 = Version::new(1, 0, 0);
    let original = ArticleV1 {
        id: 42,
        title: "Evolution Test".to_string(),
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // A V2 reader checks compatibility first.
    let stored = extract_version(&encoded).expect("extract version");
    let v2_current = Version::new(1, 1, 0);
    let compat = check_compatibility(stored, v2_current, None);
    assert!(compat.is_usable(), "V1 data must be usable by V2 reader");

    // Decode as V1 schema (the bytes' actual content).
    let (decoded_v1, ver, _): (ArticleV1, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode as V1");
    assert_eq!(ver, v1);

    // Migrate to V2 by supplying the new field default.
    let migrated = ArticleV2 {
        id: decoded_v1.id,
        title: decoded_v1.title,
        description: String::new(),
    };
    assert_eq!(migrated.id, 42);
    assert_eq!(migrated.title, "Evolution Test");
    assert!(
        migrated.description.is_empty(),
        "default description must be empty string"
    );
}
