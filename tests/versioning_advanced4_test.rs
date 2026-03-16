//! Advanced versioning tests — fourth batch (22 tests).
//!
//! Focuses on scenarios not covered by prior batches:
//! 1-4:   Forward compatibility via Option<T> fields (added fields default to None).
//! 5-8:   Backward compat — old encoder omitting new fields, new decoder sees None.
//! 9-12:  Version mismatch detection edge cases with min_compatible thresholds.
//! 13-16: Versioned enums — adding optional-data variants, roundtrip stability.
//! 17-19: Manual migration converter pattern (v1→v2 transformation function).
//! 20-22: Nested versioned structs — inner version tag preserved through outer encoding.
//!
//! All tests are top-level; no `#[cfg(test)]` module wrapper.

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
use oxicode::versioning::{
    can_migrate, check_compatibility, decode_versioned_with_check, encode_versioned,
    extract_version, is_versioned, migration_path, CompatibilityLevel, Version,
};
use oxicode::{decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value};
use oxicode::{Decode, Encode};

// ─────────────────────────────────────────────────────────────────────────────
// Shared types
// ─────────────────────────────────────────────────────────────────────────────

/// V1 of a user profile — baseline schema.
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct UserProfileV1 {
    id: u64,
    username: String,
}

/// V2 of the same profile — adds an optional email field.
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct UserProfileV2 {
    id: u64,
    username: String,
    email: Option<String>,
}

/// V3 adds a second optional field on top of V2.
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct UserProfileV3 {
    id: u64,
    username: String,
    email: Option<String>,
    age: Option<u8>,
}

/// A simple event type used in enum tests.
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
enum EventKind {
    Created,
    Updated { field: String },
    Deleted,
}

/// Outer struct that embeds a version-tagged payload as raw bytes.
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct Envelope {
    schema_id: u32,
    payload: Vec<u8>,
}

/// Inner struct used inside nested-versioning tests.
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct InnerRecord {
    value: i64,
    label: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests 1-4: Forward compatibility — added Option fields default to None
// ─────────────────────────────────────────────────────────────────────────────

/// Test 1 — A V2 struct with email=None round-trips cleanly via encode/decode.
#[test]
fn test_forward_compat_v2_none_email_roundtrip() {
    let original = UserProfileV2 {
        id: 1,
        username: String::from("alice"),
        email: None,
    };
    let encoded = encode_to_vec(&original).expect("encode V2 failed");
    let (decoded, _): (UserProfileV2, _) = decode_from_slice(&encoded).expect("decode V2 failed");
    assert_eq!(decoded, original, "V2 with None email must round-trip");
    assert!(decoded.email.is_none(), "email must remain None");
}

/// Test 2 — A V2 struct with email=Some(...) round-trips, preserving the value.
#[test]
fn test_forward_compat_v2_some_email_roundtrip() {
    let original = UserProfileV2 {
        id: 2,
        username: String::from("bob"),
        email: Some(String::from("bob@example.com")),
    };
    let encoded = encode_to_vec(&original).expect("encode V2 failed");
    let (decoded, _): (UserProfileV2, _) = decode_from_slice(&encoded).expect("decode V2 failed");
    assert_eq!(decoded, original, "V2 with Some email must round-trip");
    assert_eq!(
        decoded.email.as_deref(),
        Some("bob@example.com"),
        "email content must be preserved"
    );
}

/// Test 3 — V3 with both optional fields present round-trips cleanly.
#[test]
fn test_forward_compat_v3_both_optionals_present() {
    let original = UserProfileV3 {
        id: 3,
        username: String::from("carol"),
        email: Some(String::from("carol@example.com")),
        age: Some(30),
    };
    let encoded = encode_to_vec(&original).expect("encode V3 failed");
    let (decoded, _): (UserProfileV3, _) = decode_from_slice(&encoded).expect("decode V3 failed");
    assert_eq!(
        decoded, original,
        "V3 must round-trip with both Option fields"
    );
    assert_eq!(decoded.age, Some(30), "age must survive roundtrip");
}

/// Test 4 — V3 encoded under version tag 3.0.0, then version is verified.
#[test]
fn test_forward_compat_v3_with_version_tag_3_0_0() {
    let original = UserProfileV3 {
        id: 4,
        username: String::from("dave"),
        email: None,
        age: None,
    };
    let v3 = Version::new(3, 0, 0);
    let encoded = encode_versioned_value(&original, v3).expect("encode versioned V3 failed");
    let (decoded, ver, _): (UserProfileV3, _, _) =
        decode_versioned_value(&encoded).expect("decode versioned V3 failed");
    assert_eq!(
        decoded, original,
        "V3 must round-trip through versioned encoding"
    );
    assert_eq!(ver, v3, "version tag 3.0.0 must be preserved");
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests 5-8: Backward compat — V1 bytes decode safely into V2 via Option
// ─────────────────────────────────────────────────────────────────────────────

/// Test 5 — Encoding V1 and decoding as V1 is lossless.
#[test]
fn test_backward_compat_v1_encode_v1_decode_lossless() {
    let original = UserProfileV1 {
        id: 10,
        username: String::from("eve"),
    };
    let encoded = encode_to_vec(&original).expect("encode V1 failed");
    let (decoded, _): (UserProfileV1, _) = decode_from_slice(&encoded).expect("decode V1 failed");
    assert_eq!(decoded, original, "V1 decode must be lossless");
}

/// Test 6 — V1 fields survive when wrapped in a versioned envelope under 1.0.0.
#[test]
fn test_backward_compat_v1_versioned_tag_1_0_0() {
    let original = UserProfileV1 {
        id: 11,
        username: String::from("frank"),
    };
    let v1 = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&original, v1).expect("encode versioned V1 failed");
    let (decoded, ver, _): (UserProfileV1, _, _) =
        decode_versioned_value(&encoded).expect("decode versioned V1 failed");
    assert_eq!(decoded, original, "V1 must survive versioned roundtrip");
    assert_eq!(ver.major, 1, "major version must be 1");
}

/// Test 7 — V2 with email=None is byte-compatible in the portion V1 consumes.
/// Encodes V2(email=None), decodes as V1 — should fail gracefully (trailing bytes OK or error).
/// The test verifies the encoded V1 portion is a strict prefix of the V2 bytes.
#[test]
fn test_backward_compat_v2_none_prefix_equals_v1_encoding() {
    let v1 = UserProfileV1 {
        id: 20,
        username: String::from("grace"),
    };
    let v2 = UserProfileV2 {
        id: 20,
        username: String::from("grace"),
        email: None,
    };
    let v1_bytes = encode_to_vec(&v1).expect("encode V1 failed");
    let v2_bytes = encode_to_vec(&v2).expect("encode V2 failed");
    // V2 bytes must be longer or equal (Option<None> adds at least a tag byte).
    assert!(
        v2_bytes.len() >= v1_bytes.len(),
        "V2 encoding must be at least as long as V1"
    );
    // The V1 prefix must appear at the start of V2 bytes.
    assert_eq!(
        &v2_bytes[..v1_bytes.len()],
        v1_bytes.as_slice(),
        "V1 fields must be a byte-identical prefix of V2 encoding with None optional"
    );
}

/// Test 8 — V3 with all Nones encodes with a prefix identical to V2(email=None).
#[test]
fn test_backward_compat_v3_all_nones_prefix_equals_v2_none() {
    let v2 = UserProfileV2 {
        id: 30,
        username: String::from("heidi"),
        email: None,
    };
    let v3 = UserProfileV3 {
        id: 30,
        username: String::from("heidi"),
        email: None,
        age: None,
    };
    let v2_bytes = encode_to_vec(&v2).expect("encode V2 failed");
    let v3_bytes = encode_to_vec(&v3).expect("encode V3 failed");
    assert!(
        v3_bytes.len() >= v2_bytes.len(),
        "V3 encoding must be at least as long as V2"
    );
    assert_eq!(
        &v3_bytes[..v2_bytes.len()],
        v2_bytes.as_slice(),
        "V2 bytes must be a prefix of V3 bytes when age=None"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests 9-12: Version mismatch detection — min_compatible thresholds
// ─────────────────────────────────────────────────────────────────────────────

/// Test 9 — Data at 1.2.0 with min_compatible=1.3.0 is Incompatible.
#[test]
fn test_version_mismatch_below_min_compat_is_incompatible() {
    let data_ver = Version::new(1, 2, 0);
    let current = Version::new(1, 5, 0);
    let min_compat = Some(Version::new(1, 3, 0));
    let level = check_compatibility(data_ver, current, min_compat);
    assert_eq!(
        level,
        CompatibilityLevel::Incompatible,
        "data version 1.2.0 below min_compatible 1.3.0 must be Incompatible"
    );
}

/// Test 10 — Data at exactly min_compatible is NOT Incompatible.
#[test]
fn test_version_mismatch_at_min_compat_boundary_is_not_incompatible() {
    let data_ver = Version::new(1, 3, 0);
    let current = Version::new(1, 5, 0);
    let min_compat = Some(Version::new(1, 3, 0));
    let level = check_compatibility(data_ver, current, min_compat);
    assert!(
        level.is_usable(),
        "data version 1.3.0 equal to min_compatible must be usable"
    );
    assert_ne!(
        level,
        CompatibilityLevel::Incompatible,
        "data version at min_compatible boundary must not be Incompatible"
    );
}

/// Test 11 — decode_versioned_with_check returns error when data version is too old.
#[test]
fn test_version_mismatch_decode_with_check_returns_error_for_old_data() {
    let old_ver = Version::new(1, 0, 0);
    let current = Version::new(2, 0, 0);
    let payload = b"old payload bytes";

    let encoded = encode_versioned(payload, old_ver).expect("encode failed");
    let result = decode_versioned_with_check(&encoded, current, None);
    assert!(
        result.is_err(),
        "decoding major-1 data with major-2 current must fail"
    );
}

/// Test 12 — Version satisfies check: 2.5.3 satisfies minimum 2.0.0.
#[test]
fn test_version_satisfies_minimum_requirement() {
    let v = Version::new(2, 5, 3);
    let min = Version::new(2, 0, 0);
    assert!(
        v.satisfies(&min),
        "2.5.3 must satisfy minimum requirement 2.0.0"
    );
    // Reverse: 2.0.0 does NOT satisfy 2.5.3 minimum.
    assert!(
        !min.satisfies(&v),
        "2.0.0 must not satisfy minimum requirement 2.5.3"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests 13-16: Versioned enums — adding variants, roundtrip stability
// ─────────────────────────────────────────────────────────────────────────────

/// Test 13 — EventKind::Created round-trips under version tag 1.0.0.
#[test]
fn test_versioned_enum_created_roundtrip() {
    let event = EventKind::Created;
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&event, ver).expect("encode EventKind::Created failed");
    let (decoded, recovered_ver, _): (EventKind, _, _) =
        decode_versioned_value(&encoded).expect("decode EventKind::Created failed");
    assert_eq!(
        decoded, event,
        "EventKind::Created must survive versioned roundtrip"
    );
    assert_eq!(recovered_ver, ver, "version must be preserved for enum");
}

/// Test 14 — EventKind::Deleted round-trips under version tag 1.0.0.
#[test]
fn test_versioned_enum_deleted_roundtrip() {
    let event = EventKind::Deleted;
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&event, ver).expect("encode EventKind::Deleted failed");
    let (decoded, _, _): (EventKind, _, _) =
        decode_versioned_value(&encoded).expect("decode EventKind::Deleted failed");
    assert_eq!(
        decoded, event,
        "EventKind::Deleted must survive versioned roundtrip"
    );
}

/// Test 15 — EventKind::Updated with a field value round-trips correctly.
#[test]
fn test_versioned_enum_updated_with_field_roundtrip() {
    let event = EventKind::Updated {
        field: String::from("title"),
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&event, ver).expect("encode EventKind::Updated failed");
    let (decoded, recovered_ver, _): (EventKind, _, _) =
        decode_versioned_value(&encoded).expect("decode EventKind::Updated failed");
    assert_eq!(
        decoded, event,
        "EventKind::Updated must survive versioned roundtrip"
    );
    assert_eq!(recovered_ver, ver, "version 2.0.0 must be preserved");
    if let EventKind::Updated { field } = decoded {
        assert_eq!(field, "title", "field value must be preserved");
    } else {
        panic!("expected EventKind::Updated variant");
    }
}

/// Test 16 — All three EventKind variants encode to distinct byte sequences.
#[test]
fn test_versioned_enum_variants_produce_distinct_bytes() {
    let ver = Version::new(1, 0, 0);

    let enc_created =
        encode_versioned_value(&EventKind::Created, ver).expect("encode Created failed");
    let enc_deleted =
        encode_versioned_value(&EventKind::Deleted, ver).expect("encode Deleted failed");
    let enc_updated = encode_versioned_value(
        &EventKind::Updated {
            field: String::from("x"),
        },
        ver,
    )
    .expect("encode Updated failed");

    assert_ne!(
        enc_created, enc_deleted,
        "Created and Deleted must have different bytes"
    );
    assert_ne!(
        enc_created, enc_updated,
        "Created and Updated must have different bytes"
    );
    assert_ne!(
        enc_deleted, enc_updated,
        "Deleted and Updated must have different bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests 17-19: Manual migration converter pattern (v1 → v2 transform)
// ─────────────────────────────────────────────────────────────────────────────

/// Test 17 — Migrating from V1 to V2 via an explicit converter function works.
#[test]
fn test_migration_v1_to_v2_converter_function() {
    /// Simulate a v1→v2 migration by re-encoding under the v2 schema.
    fn migrate_v1_to_v2(v1: UserProfileV1) -> UserProfileV2 {
        UserProfileV2 {
            id: v1.id,
            username: v1.username,
            email: None, // new field defaults to None
        }
    }

    let v1_data = UserProfileV1 {
        id: 100,
        username: String::from("ivan"),
    };
    let v2_data = migrate_v1_to_v2(v1_data.clone());

    // V2 must carry over the V1 fields intact.
    assert_eq!(
        v2_data.id, v1_data.id,
        "id must be preserved through migration"
    );
    assert_eq!(
        v2_data.username, v1_data.username,
        "username must be preserved through migration"
    );
    assert!(
        v2_data.email.is_none(),
        "email defaults to None after migration"
    );

    // The migrated V2 must round-trip under the v2 version tag.
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&v2_data, ver).expect("encode migrated V2 failed");
    let (decoded, recovered_ver, _): (UserProfileV2, _, _) =
        decode_versioned_value(&encoded).expect("decode migrated V2 failed");
    assert_eq!(decoded, v2_data, "migrated V2 must round-trip");
    assert_eq!(recovered_ver, ver, "v2 version tag must be preserved");
}

/// Test 18 — can_migrate returns false for backward-version migration (downgrade).
#[test]
fn test_migration_can_migrate_backward_returns_false() {
    let newer = Version::new(3, 0, 0);
    let older = Version::new(2, 0, 0);
    assert!(
        !can_migrate(newer, older),
        "downgrade migration 3.0.0 -> 2.0.0 must not be allowed"
    );
}

/// Test 19 — migration_path for three-major jump contains exactly two intermediates.
#[test]
fn test_migration_path_three_major_jump_has_two_intermediates() {
    let from = Version::new(1, 0, 0);
    let to = Version::new(4, 0, 0);
    let path = migration_path(from, to);
    assert_eq!(
        path.len(),
        2,
        "1.x -> 4.x migration must contain exactly 2 intermediate versions"
    );
    assert_eq!(
        path[0],
        Version::new(2, 0, 0),
        "first intermediate must be 2.0.0"
    );
    assert_eq!(
        path[1],
        Version::new(3, 0, 0),
        "second intermediate must be 3.0.0"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests 20-22: Nested versioned structs
// ─────────────────────────────────────────────────────────────────────────────

/// Test 20 — Encoding an InnerRecord with a version header, then wrapping in Envelope.
/// The inner version tag survives within the Envelope payload bytes.
#[test]
fn test_nested_versioned_inner_record_in_envelope() {
    let inner = InnerRecord {
        value: -42,
        label: String::from("nested"),
    };
    let inner_ver = Version::new(1, 0, 0);
    let inner_encoded = encode_versioned_value(&inner, inner_ver).expect("encode inner failed");

    let envelope = Envelope {
        schema_id: 7,
        payload: inner_encoded.clone(),
    };
    let env_encoded = encode_to_vec(&envelope).expect("encode envelope failed");
    let (decoded_env, _): (Envelope, _) =
        decode_from_slice(&env_encoded).expect("decode envelope failed");

    assert_eq!(
        decoded_env.schema_id, 7,
        "schema_id must survive outer encoding"
    );
    assert_eq!(
        decoded_env.payload, inner_encoded,
        "inner payload bytes must be preserved verbatim"
    );

    // Verify the inner payload is still recognised as versioned.
    assert!(
        is_versioned(&decoded_env.payload),
        "inner payload must still be recognised as versioned after outer roundtrip"
    );
}

/// Test 21 — Extracting the inner version from a nested encoded envelope.
#[test]
fn test_nested_versioned_extract_inner_version_from_envelope_payload() {
    let inner = InnerRecord {
        value: 999,
        label: String::from("deep"),
    };
    let inner_ver = Version::new(5, 2, 1);
    let inner_encoded = encode_versioned_value(&inner, inner_ver).expect("encode inner failed");

    let envelope = Envelope {
        schema_id: 42,
        payload: inner_encoded,
    };
    let env_encoded = encode_to_vec(&envelope).expect("encode envelope failed");
    let (decoded_env, _): (Envelope, _) =
        decode_from_slice(&env_encoded).expect("decode envelope failed");

    let extracted_inner_ver =
        extract_version(&decoded_env.payload).expect("extract_version from inner payload failed");
    assert_eq!(
        extracted_inner_ver, inner_ver,
        "version extracted from inner payload must match original inner version"
    );
}

/// Test 22 — Round-trip an Envelope that itself is versioned-encoded (double versioning).
#[test]
fn test_nested_versioned_double_versioned_envelope_roundtrip() {
    let inner = InnerRecord {
        value: 0,
        label: String::from("root"),
    };
    let inner_ver = Version::new(1, 0, 0);
    let inner_encoded = encode_versioned_value(&inner, inner_ver).expect("encode inner failed");

    let envelope = Envelope {
        schema_id: 1,
        payload: inner_encoded,
    };
    // Version the envelope itself.
    let outer_ver = Version::new(2, 0, 0);
    let outer_encoded = encode_versioned_value(&envelope, outer_ver).expect("encode outer failed");

    let (decoded_env, recovered_outer_ver, _): (Envelope, _, _) =
        decode_versioned_value(&outer_encoded).expect("decode outer failed");

    assert_eq!(
        recovered_outer_ver, outer_ver,
        "outer version 2.0.0 must survive double-versioned roundtrip"
    );
    assert_eq!(
        decoded_env.schema_id, 1,
        "schema_id must survive double-versioned roundtrip"
    );
    // Inner payload is still a valid versioned blob.
    assert!(
        is_versioned(&decoded_env.payload),
        "inner payload must remain versioned after double-versioned roundtrip"
    );
    // Decode the inner value to confirm end-to-end integrity.
    let (decoded_inner, recovered_inner_ver, _): (InnerRecord, _, _) =
        decode_versioned_value(&decoded_env.payload)
            .expect("decode inner from double-versioned envelope failed");
    assert_eq!(
        decoded_inner, inner,
        "InnerRecord must survive double-versioned roundtrip"
    );
    assert_eq!(
        recovered_inner_ver, inner_ver,
        "inner version 1.0.0 must survive double-versioned roundtrip"
    );
}
