//! Version struct basics-focused tests for the versioning module (split from versioning_test.rs).

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

// ── Version struct tests ──────────────────────────────────────────────────────

#[test]
fn test_version_comparison_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v2 > v1);
    assert!(v1 < v2);
}

#[test]
fn test_version_new_fields() {
    let v = Version::new(3, 7, 11);
    assert_eq!(v.major, 3);
    assert_eq!(v.minor, 7);
    assert_eq!(v.patch, 11);
}

#[test]
fn test_version_equality() {
    assert_eq!(Version::new(1, 2, 3), Version::new(1, 2, 3));
    assert_ne!(Version::new(1, 2, 3), Version::new(1, 2, 4));
}

#[test]
fn test_version_zero() {
    let v = Version::zero();
    assert_eq!(v.major, 0);
    assert_eq!(v.minor, 0);
    assert_eq!(v.patch, 0);
}

#[test]
fn test_version_parse_valid() {
    assert_eq!(Version::parse("1.2.3"), Some(Version::new(1, 2, 3)));
    assert_eq!(Version::parse("0.0.0"), Some(Version::new(0, 0, 0)));
    assert_eq!(
        Version::parse("65535.65535.65535"),
        Some(Version::new(65535, 65535, 65535))
    );
}

#[test]
fn test_version_parse_invalid() {
    assert_eq!(Version::parse("1.2"), None);
    assert_eq!(Version::parse("1.2.3.4"), None);
    assert_eq!(Version::parse("not.a.version"), None);
    assert_eq!(Version::parse(""), None);
}

#[test]
fn test_version_bytes_roundtrip() {
    let v = Version::new(10, 20, 30);
    let bytes = v.to_bytes();
    let v2 = Version::from_bytes(&bytes).expect("from_bytes failed");
    assert_eq!(v, v2);
}

#[test]
fn test_version_tuple() {
    let v = Version::new(5, 6, 7);
    assert_eq!(v.tuple(), (5, 6, 7));
}

#[test]
fn test_version_satisfies() {
    let v = Version::new(1, 5, 0);
    assert!(v.satisfies(&Version::new(1, 0, 0)));
    assert!(!v.satisfies(&Version::new(2, 0, 0)));
}

#[test]
fn test_version_is_compatible_with() {
    // Same major (>=1) → compatible
    assert!(Version::new(1, 0, 0).is_compatible_with(&Version::new(1, 5, 0)));
    // Different major → not compatible
    assert!(!Version::new(1, 0, 0).is_compatible_with(&Version::new(2, 0, 0)));
    // 0.x same minor → compatible
    assert!(Version::new(0, 1, 0).is_compatible_with(&Version::new(0, 1, 5)));
    // 0.x different minor → not compatible
    assert!(!Version::new(0, 1, 0).is_compatible_with(&Version::new(0, 2, 0)));
}

#[test]
fn test_version_breaking_change() {
    assert!(Version::new(2, 0, 0).is_breaking_change_from(&Version::new(1, 0, 0)));
    assert!(!Version::new(1, 5, 0).is_breaking_change_from(&Version::new(1, 0, 0)));
}

#[test]
fn test_version_minor_update() {
    assert!(Version::new(1, 2, 0).is_minor_update_from(&Version::new(1, 0, 0)));
    assert!(!Version::new(2, 0, 0).is_minor_update_from(&Version::new(1, 0, 0)));
}

#[test]
fn test_version_patch_update() {
    assert!(Version::new(1, 0, 1).is_patch_update_from(&Version::new(1, 0, 0)));
    assert!(!Version::new(1, 1, 0).is_patch_update_from(&Version::new(1, 0, 0)));
}

/// Verify all four ordering relationships across patch / minor / major boundaries.
#[test]
fn test_version_ordering() {
    let v1_0_0 = Version::new(1, 0, 0);
    let v1_0_1 = Version::new(1, 0, 1);
    let v1_1_0 = Version::new(1, 1, 0);
    let v2_0_0 = Version::new(2, 0, 0);

    assert!(v1_0_0 < v1_0_1);
    assert!(v1_0_1 < v1_1_0);
    assert!(v1_1_0 < v2_0_0);
    assert!(v1_0_0 < v2_0_0);
    assert_eq!(v1_0_0, Version::new(1, 0, 0));
}

/// NEW-1. Version::new stores major, minor, patch correctly.
#[test]
fn test_version_new_creates_correct_fields() {
    let v = Version::new(100, 200, 300);
    assert_eq!(v.major, 100, "major must be 100");
    assert_eq!(v.minor, 200, "minor must be 200");
    assert_eq!(v.patch, 300, "patch must be 300");

    let v2 = Version::new(0, 0, 1);
    assert_eq!(v2.major, 0);
    assert_eq!(v2.minor, 0);
    assert_eq!(v2.patch, 1);
}

/// NEW-2. Strict ordering: 1.0.0 < 1.1.0 < 2.0.0.
#[test]
fn test_version_ordering_chain() {
    let v1_0_0 = Version::new(1, 0, 0);
    let v1_1_0 = Version::new(1, 1, 0);
    let v2_0_0 = Version::new(2, 0, 0);

    assert!(v1_0_0 < v1_1_0, "1.0.0 must be less than 1.1.0");
    assert!(v1_1_0 < v2_0_0, "1.1.0 must be less than 2.0.0");
    assert!(v1_0_0 < v2_0_0, "1.0.0 must be less than 2.0.0");
    assert_eq!(
        v1_0_0,
        Version::new(1, 0, 0),
        "identical versions must be equal"
    );
}

/// NEW-5. Version with u16 major, minor, patch boundary values.
#[test]
fn test_version_u16_boundary_values() {
    let max = Version::new(u16::MAX, u16::MAX, u16::MAX);
    let bytes = max.to_bytes();
    let restored = Version::from_bytes(&bytes).expect("from_bytes must succeed for max version");
    assert_eq!(
        max, restored,
        "max-value version must survive bytes roundtrip"
    );

    let zero = Version::new(0, 0, 0);
    let bytes0 = zero.to_bytes();
    let restored0 = Version::from_bytes(&bytes0).expect("from_bytes must succeed for zero version");
    assert_eq!(zero, restored0, "zero version must survive bytes roundtrip");
}

/// NEW-14. Version max values (65535.65535.65535) roundtrip through
/// encode_versioned_value / decode_versioned_value.
#[test]
fn test_version_max_values_roundtrip() {
    let max_version = Version::new(u16::MAX, u16::MAX, u16::MAX);
    let payload = 77u32;

    let encoded =
        oxicode::encode_versioned_value(&payload, max_version).expect("encode max version");
    let (val, ver, _): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode max version");

    assert_eq!(val, payload);
    assert_eq!(
        ver, max_version,
        "max version (65535.65535.65535) must roundtrip"
    );
    assert_eq!(ver.major, u16::MAX);
    assert_eq!(ver.minor, u16::MAX);
    assert_eq!(ver.patch, u16::MAX);
}

/// NEW-20. Version Display format produces "major.minor.patch" string.
#[test]
fn test_version_display_format() {
    let cases: &[(Version, &str)] = &[
        (Version::new(1, 2, 3), "1.2.3"),
        (Version::new(0, 0, 0), "0.0.0"),
        (Version::new(10, 20, 30), "10.20.30"),
        (
            Version::new(u16::MAX, u16::MAX, u16::MAX),
            "65535.65535.65535",
        ),
        (Version::new(1, 0, 0), "1.0.0"),
    ];

    for (v, expected) in cases {
        let formatted = format!("{}", v);
        assert_eq!(
            formatted, *expected,
            "display format for {:?} must be {:?}, got {:?}",
            v, expected, formatted
        );
    }
}
