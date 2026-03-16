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
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct AuditLog {
    event_id: u64,
    user: String,
    action: String,
    success: bool,
    metadata: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Permission {
    Read,
    Write,
    Execute,
    Admin { level: u8 },
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct AccessPolicy {
    resource: String,
    permissions: Vec<Permission>,
    deny_list: Vec<String>,
}

fn make_audit_log() -> AuditLog {
    AuditLog {
        event_id: 100001,
        user: "alice".to_string(),
        action: "login".to_string(),
        success: true,
        metadata: vec![
            ("ip".to_string(), "127.0.0.1".to_string()),
            ("session".to_string(), "abc123".to_string()),
        ],
    }
}

fn make_access_policy() -> AccessPolicy {
    AccessPolicy {
        resource: "/api/admin".to_string(),
        permissions: vec![
            Permission::Read,
            Permission::Write,
            Permission::Admin { level: 3 },
        ],
        deny_list: vec!["guest".to_string(), "bot".to_string()],
    }
}

#[test]
fn test_audit_log_basic_roundtrip() {
    let cfg = config::standard();
    let original = make_audit_log();
    let bytes = encode_to_vec(&original, cfg).expect("encode AuditLog");
    let (decoded, _): (AuditLog, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuditLog");
    assert_eq!(original, decoded);
}

#[test]
fn test_audit_log_event_id_preserved() {
    let cfg = config::standard();
    let original = AuditLog {
        event_id: u64::MAX - 7,
        user: "bob".to_string(),
        action: "delete".to_string(),
        success: false,
        metadata: vec![],
    };
    let bytes = encode_to_vec(&original, cfg).expect("encode AuditLog with large event_id");
    let (decoded, _): (AuditLog, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuditLog with large event_id");
    assert_eq!(original.event_id, decoded.event_id);
    assert_eq!(original.success, decoded.success);
}

#[test]
fn test_permission_read_roundtrip() {
    let cfg = config::standard();
    let original = Permission::Read;
    let bytes = encode_to_vec(&original, cfg).expect("encode Permission::Read");
    let (decoded, _): (Permission, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Permission::Read");
    assert_eq!(original, decoded);
}

#[test]
fn test_permission_write_roundtrip() {
    let cfg = config::standard();
    let original = Permission::Write;
    let bytes = encode_to_vec(&original, cfg).expect("encode Permission::Write");
    let (decoded, _): (Permission, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Permission::Write");
    assert_eq!(original, decoded);
}

#[test]
fn test_permission_execute_roundtrip() {
    let cfg = config::standard();
    let original = Permission::Execute;
    let bytes = encode_to_vec(&original, cfg).expect("encode Permission::Execute");
    let (decoded, _): (Permission, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Permission::Execute");
    assert_eq!(original, decoded);
}

#[test]
fn test_permission_admin_roundtrip() {
    let cfg = config::standard();
    let original = Permission::Admin { level: 255 };
    let bytes = encode_to_vec(&original, cfg).expect("encode Permission::Admin");
    let (decoded, _): (Permission, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Permission::Admin");
    assert_eq!(original, decoded);
}

#[test]
fn test_permission_admin_zero_level() {
    let cfg = config::standard();
    let original = Permission::Admin { level: 0 };
    let bytes = encode_to_vec(&original, cfg).expect("encode Permission::Admin level=0");
    let (decoded, _): (Permission, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Permission::Admin level=0");
    assert_eq!(original, decoded);
}

#[test]
fn test_access_policy_roundtrip() {
    let cfg = config::standard();
    let original = make_access_policy();
    let bytes = encode_to_vec(&original, cfg).expect("encode AccessPolicy");
    let (decoded, _): (AccessPolicy, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AccessPolicy");
    assert_eq!(original, decoded);
}

#[test]
fn test_access_policy_empty_deny_list() {
    let cfg = config::standard();
    let original = AccessPolicy {
        resource: "/public".to_string(),
        permissions: vec![Permission::Read],
        deny_list: vec![],
    };
    let bytes = encode_to_vec(&original, cfg).expect("encode AccessPolicy with empty deny_list");
    let (decoded, _): (AccessPolicy, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AccessPolicy with empty deny_list");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_audit_log_roundtrip() {
    let cfg = config::standard();
    let original = vec![
        AuditLog {
            event_id: 1,
            user: "alice".to_string(),
            action: "read".to_string(),
            success: true,
            metadata: vec![("key".to_string(), "val".to_string())],
        },
        AuditLog {
            event_id: 2,
            user: "bob".to_string(),
            action: "write".to_string(),
            success: false,
            metadata: vec![],
        },
        AuditLog {
            event_id: 3,
            user: "carol".to_string(),
            action: "execute".to_string(),
            success: true,
            metadata: vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string()),
                ("c".to_string(), "3".to_string()),
            ],
        },
    ];
    let bytes = encode_to_vec(&original, cfg).expect("encode Vec<AuditLog>");
    let (decoded, _): (Vec<AuditLog>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<AuditLog>");
    assert_eq!(original, decoded);
}

#[test]
fn test_audit_log_empty_metadata_roundtrip() {
    let cfg = config::standard();
    let original = AuditLog {
        event_id: 0,
        user: String::new(),
        action: String::new(),
        success: false,
        metadata: vec![],
    };
    let bytes = encode_to_vec(&original, cfg).expect("encode AuditLog with all-empty fields");
    let (decoded, _): (AuditLog, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuditLog with all-empty fields");
    assert_eq!(original, decoded);
}

#[test]
fn test_access_policy_empty_permissions_roundtrip() {
    let cfg = config::standard();
    let original = AccessPolicy {
        resource: String::new(),
        permissions: vec![],
        deny_list: vec![],
    };
    let bytes =
        encode_to_vec(&original, cfg).expect("encode AccessPolicy with all-empty collections");
    let (decoded, _): (AccessPolicy, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode AccessPolicy with all-empty collections");
    assert_eq!(original, decoded);
}

#[test]
fn test_audit_log_consumed_bytes_equals_encoded_len() {
    let cfg = config::standard();
    let original = make_audit_log();
    let bytes = encode_to_vec(&original, cfg).expect("encode AuditLog for consumed-bytes check");
    let (_decoded, consumed): (AuditLog, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuditLog for consumed-bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

#[test]
fn test_access_policy_consumed_bytes_equals_encoded_len() {
    let cfg = config::standard();
    let original = make_access_policy();
    let bytes =
        encode_to_vec(&original, cfg).expect("encode AccessPolicy for consumed-bytes check");
    let (_decoded, consumed): (AccessPolicy, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AccessPolicy for consumed-bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length for AccessPolicy"
    );
}

#[test]
fn test_audit_log_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = make_audit_log();
    let bytes = encode_to_vec(&original, cfg).expect("encode AuditLog with fixed_int config");
    let (decoded, _): (AuditLog, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuditLog with fixed_int config");
    assert_eq!(original, decoded);
}

#[test]
fn test_access_policy_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = make_access_policy();
    let bytes = encode_to_vec(&original, cfg).expect("encode AccessPolicy with fixed_int config");
    let (decoded, _): (AccessPolicy, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AccessPolicy with fixed_int config");
    assert_eq!(original, decoded);
}

#[test]
fn test_audit_log_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original = make_audit_log();
    let bytes = encode_to_vec(&original, cfg).expect("encode AuditLog with big_endian config");
    let (decoded, _): (AuditLog, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuditLog with big_endian config");
    assert_eq!(original, decoded);
}

#[test]
fn test_access_policy_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original = make_access_policy();
    let bytes = encode_to_vec(&original, cfg).expect("encode AccessPolicy with big_endian config");
    let (decoded, _): (AccessPolicy, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AccessPolicy with big_endian config");
    assert_eq!(original, decoded);
}

#[test]
fn test_audit_log_big_endian_fixed_int_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = make_audit_log();
    let bytes =
        encode_to_vec(&original, cfg).expect("encode AuditLog with big_endian + fixed_int config");
    let (decoded, _): (AuditLog, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode AuditLog with big_endian + fixed_int config");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_audit_log_some_roundtrip() {
    let cfg = config::standard();
    let original: Option<AuditLog> = Some(make_audit_log());
    let bytes = encode_to_vec(&original, cfg).expect("encode Option<AuditLog> Some");
    let (decoded, _): (Option<AuditLog>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Option<AuditLog> Some");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_audit_log_none_roundtrip() {
    let cfg = config::standard();
    let original: Option<AuditLog> = None;
    let bytes = encode_to_vec(&original, cfg).expect("encode Option<AuditLog> None");
    let (decoded, _): (Option<AuditLog>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Option<AuditLog> None");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_permission_all_variants_roundtrip() {
    let cfg = config::standard();
    let original = vec![
        Permission::Read,
        Permission::Write,
        Permission::Execute,
        Permission::Admin { level: 1 },
        Permission::Admin { level: 127 },
        Permission::Admin { level: 255 },
        Permission::Read,
    ];
    let bytes = encode_to_vec(&original, cfg).expect("encode Vec<Permission> with all variants");
    let (decoded, _): (Vec<Permission>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Permission> with all variants");
    assert_eq!(original, decoded);
}

#[test]
fn test_audit_log_unicode_fields_roundtrip() {
    let cfg = config::standard();
    let original = AuditLog {
        event_id: 42,
        user: "用户_тест_ユーザー".to_string(),
        action: "アクション_действие_动作".to_string(),
        success: true,
        metadata: vec![
            ("キー".to_string(), "値".to_string()),
            ("ключ".to_string(), "значение".to_string()),
        ],
    };
    let bytes = encode_to_vec(&original, cfg).expect("encode AuditLog with unicode fields");
    let (decoded, _): (AuditLog, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuditLog with unicode fields");
    assert_eq!(original, decoded);
}
