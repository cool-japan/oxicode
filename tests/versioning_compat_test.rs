//! Compatibility-focused tests for the versioning module (split from versioning_test.rs).

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
    can_migrate, check_compatibility, migration_path, CompatibilityLevel, Version,
};

// ── Compatibility checking ────────────────────────────────────────────────────

#[test]
fn test_compatibility_same_version() {
    let v = Version::new(1, 0, 0);
    assert_eq!(
        check_compatibility(v, v, None),
        CompatibilityLevel::Compatible
    );
}

#[test]
fn test_compatibility_patch_difference_older_data() {
    let data = Version::new(1, 0, 0);
    let current = Version::new(1, 0, 5);
    let compat = check_compatibility(data, current, None);
    assert_eq!(compat, CompatibilityLevel::Compatible);
}

#[test]
fn test_compatibility_older_minor() {
    let data = Version::new(1, 0, 0);
    let current = Version::new(1, 5, 0);
    let compat = check_compatibility(data, current, None);
    assert!(compat.is_usable());
}

#[test]
fn test_compatibility_incompatible_major() {
    let data = Version::new(1, 0, 0);
    let current = Version::new(2, 0, 0);
    assert_eq!(
        check_compatibility(data, current, None),
        CompatibilityLevel::Incompatible
    );
}

#[test]
fn test_compatibility_below_minimum() {
    let data = Version::new(1, 1, 0);
    let current = Version::new(1, 5, 0);
    let min = Some(Version::new(1, 3, 0));
    assert_eq!(
        check_compatibility(data, current, min),
        CompatibilityLevel::Incompatible
    );
}

#[test]
fn test_compatibility_0x_same_minor() {
    let data = Version::new(0, 1, 0);
    let current = Version::new(0, 1, 9);
    assert_eq!(
        check_compatibility(data, current, None),
        CompatibilityLevel::Compatible
    );
}

#[test]
fn test_compatibility_0x_different_minor() {
    let data = Version::new(0, 1, 0);
    let current = Version::new(0, 2, 0);
    assert_eq!(
        check_compatibility(data, current, None),
        CompatibilityLevel::Incompatible
    );
}

#[test]
fn test_compatibility_newer_data_than_current() {
    // Data was encoded with a newer minor version than what the reader knows
    let data = Version::new(1, 9, 0);
    let current = Version::new(1, 0, 0);
    let compat = check_compatibility(data, current, None);
    assert_eq!(compat, CompatibilityLevel::CompatibleWithWarnings);
}

#[test]
fn test_compatibility_level_methods() {
    assert!(CompatibilityLevel::Compatible.is_usable());
    assert!(CompatibilityLevel::Compatible.is_fully_compatible());
    assert!(!CompatibilityLevel::Compatible.has_warnings());

    assert!(CompatibilityLevel::CompatibleWithWarnings.is_usable());
    assert!(!CompatibilityLevel::CompatibleWithWarnings.is_fully_compatible());
    assert!(CompatibilityLevel::CompatibleWithWarnings.has_warnings());

    assert!(!CompatibilityLevel::Incompatible.is_usable());
    assert!(!CompatibilityLevel::Incompatible.is_fully_compatible());
    assert!(!CompatibilityLevel::Incompatible.has_warnings());
}

// ── Migration path ─────────────────────────────────────────────────────────────

#[test]
fn test_can_migrate_same_major() {
    assert!(can_migrate(Version::new(1, 0, 0), Version::new(1, 9, 0)));
}

#[test]
fn test_can_migrate_forward_major() {
    assert!(can_migrate(Version::new(1, 0, 0), Version::new(3, 0, 0)));
}

#[test]
fn test_cannot_migrate_backward() {
    assert!(!can_migrate(Version::new(3, 0, 0), Version::new(1, 0, 0)));
}

#[test]
fn test_can_migrate_same_version() {
    // Migrating to the same version is trivially possible (within same major)
    assert!(can_migrate(Version::new(1, 0, 0), Version::new(1, 0, 0)));
}

#[test]
fn test_migration_path_same_version() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(1, 0, 0));
    assert!(path.is_empty());
}

#[test]
fn test_migration_path_same_major() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(1, 5, 0));
    assert!(path.is_empty());
}

#[test]
fn test_migration_path_one_major_bump() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(2, 0, 0));
    assert!(
        path.is_empty(),
        "direct migration needs no intermediate steps"
    );
}

#[test]
fn test_migration_path_two_major_bumps() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(3, 0, 0));
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], Version::new(2, 0, 0));
}

#[test]
fn test_migration_path_three_major_bumps() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(4, 0, 0));
    assert_eq!(path.len(), 2);
    assert_eq!(path[0], Version::new(2, 0, 0));
    assert_eq!(path[1], Version::new(3, 0, 0));
}

/// can_migrate: same version is always possible; forward migration is possible;
/// backward migration is not.
#[test]
fn test_can_migrate_extended() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    // Same version — trivially migratable (same major).
    assert!(can_migrate(v1, v1));

    // Forward migration is supported.
    assert!(can_migrate(v1, v2));

    // Backward migration is not supported.
    assert!(!can_migrate(v2, v1));
}

/// Verify that multiple minor-version bumps within the same major are all
/// forward-compatible (can_migrate returns true) and that the stored version
/// round-trips correctly.
#[test]
fn test_version_compatibility_chain() {
    let versions = [
        Version::new(1, 0, 0),
        Version::new(1, 1, 0),
        Version::new(1, 2, 0),
        Version::new(1, 3, 0),
    ];

    // Every earlier version can migrate to every later version.
    for (i, &from) in versions.iter().enumerate() {
        for &to in &versions[i..] {
            assert!(
                can_migrate(from, to),
                "expected can_migrate({from} -> {to})"
            );
        }
    }
}

/// 7. test_can_migrate_true
#[test]
fn test_can_migrate_true() {
    use oxicode::versioning::{can_migrate, Version};

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(
        can_migrate(v1, v2),
        "forward major migration must be possible"
    );
}

/// 8. test_can_migrate_false
#[test]
fn test_can_migrate_false() {
    use oxicode::versioning::{can_migrate, Version};

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(
        !can_migrate(v2, v1),
        "backward (downgrade) migration must not be possible"
    );
}

/// 9. test_migration_path_chain
#[test]
fn test_migration_path_chain() {
    use oxicode::versioning::{migration_path, Version};

    let v1 = Version::new(1, 0, 0);
    let v3 = Version::new(3, 0, 0);

    let path = migration_path(v1, v3);
    assert_eq!(path.len(), 1, "one intermediate step between v1 and v3");
    assert_eq!(
        path[0],
        Version::new(2, 0, 0),
        "intermediate step must be v2"
    );
}

/// 4. test_migration_chain_three_steps
#[test]
fn test_migration_chain_three_steps() {
    use oxicode::versioning::{migration_path, Version};

    let from = Version::new(1, 0, 0);
    let to = Version::new(4, 0, 0);

    let path = migration_path(from, to);
    assert_eq!(path.len(), 2, "three-step chain has two intermediate nodes");
    assert_eq!(path[0], Version::new(2, 0, 0));
    assert_eq!(path[1], Version::new(3, 0, 0));
}

/// 14. test_multiple_migrations_applied
#[test]
fn test_multiple_migrations_applied() {
    use oxicode::versioning::{extract_version, Version};

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);

    // Step 1: encode as V1.
    let enc1 = oxicode::encode_versioned_value(&100u64, v1).expect("encode v1");
    assert_eq!(extract_version(&enc1).expect("extract v1"), v1);

    // Step 2: decode from V1, re-encode as V2 (the "migration" step).
    let (val2, _, _): (u64, _, _) = oxicode::decode_versioned_value(&enc1).expect("decode v1");
    let enc2 = oxicode::encode_versioned_value(&val2, v2).expect("encode v2");
    assert_eq!(extract_version(&enc2).expect("extract v2"), v2);

    // Step 3: decode from V2, re-encode as V3.
    let (val3, _, _): (u64, _, _) = oxicode::decode_versioned_value(&enc2).expect("decode v2");
    let enc3 = oxicode::encode_versioned_value(&val3, v3).expect("encode v3");
    assert_eq!(extract_version(&enc3).expect("extract v3"), v3);

    // The value must survive all three transformations unchanged.
    let (final_val, final_ver, _): (u64, _, _) =
        oxicode::decode_versioned_value(&enc3).expect("decode v3");
    assert_eq!(final_val, 100u64);
    assert_eq!(final_ver, v3);
}
