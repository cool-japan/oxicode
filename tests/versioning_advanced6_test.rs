//! Wire format stability and versioning tests for OxiCode (set 6).
//!
//! Covers 22 scenarios exercising schema evolution, wire format stability,
//! prefix compatibility between struct versions, enum encoding, and collections.

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

// V1: original schema
#[derive(Debug, PartialEq, Encode, Decode)]
struct UserV1 {
    id: u32,
    name: String,
}

// V2: added field (simulate by having a "full" version)
#[derive(Debug, PartialEq, Encode, Decode)]
struct UserV2 {
    id: u32,
    name: String,
    email: String,
}

// V3: different types
#[derive(Debug, PartialEq, Encode, Decode)]
struct UserV3 {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Config {
    version: u32,
    debug: bool,
    timeout_ms: u64,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Action {
    Create,
    Update { id: u32, field: String },
    Delete(u32),
    Noop,
}

// ── Test 1 ─────────────────────────────────────────────────────────────────
// UserV1 roundtrip basic
#[test]
fn test_userv1_roundtrip_basic() {
    let original = UserV1 {
        id: 42,
        name: String::from("alice"),
    };
    let encoded = encode_to_vec(&original).expect("encode UserV1 failed");
    let (decoded, _consumed): (UserV1, usize) =
        decode_from_slice(&encoded).expect("decode UserV1 failed");
    assert_eq!(decoded, original);
}

// ── Test 2 ─────────────────────────────────────────────────────────────────
// UserV2 roundtrip
#[test]
fn test_userv2_roundtrip() {
    let original = UserV2 {
        id: 7,
        name: String::from("bob"),
        email: String::from("bob@example.com"),
    };
    let encoded = encode_to_vec(&original).expect("encode UserV2 failed");
    let (decoded, _consumed): (UserV2, usize) =
        decode_from_slice(&encoded).expect("decode UserV2 failed");
    assert_eq!(decoded, original);
}

// ── Test 3 ─────────────────────────────────────────────────────────────────
// UserV3 roundtrip
#[test]
fn test_userv3_roundtrip() {
    let original = UserV3 {
        id: 999_999_999_999u64,
        name: String::from("carol"),
        email: String::from("carol@domain.org"),
        active: true,
    };
    let encoded = encode_to_vec(&original).expect("encode UserV3 failed");
    let (decoded, _consumed): (UserV3, usize) =
        decode_from_slice(&encoded).expect("decode UserV3 failed");
    assert_eq!(decoded, original);
}

// ── Test 4 ─────────────────────────────────────────────────────────────────
// UserV1 encoding is a prefix of UserV2 encoding (first N bytes of V2 match V1)
// Structs are encoded by concatenating fields, so V1{id,name} bytes should be
// a prefix of V2{id,name,email=""} bytes (empty string appends [0x00]).
#[test]
fn test_userv1_bytes_are_prefix_of_userv2_bytes() {
    let v1 = UserV1 {
        id: 1,
        name: String::from("bob"),
    };
    let v2 = UserV2 {
        id: 1,
        name: String::from("bob"),
        email: String::from(""),
    };
    let v1_bytes = encode_to_vec(&v1).expect("encode V1 failed");
    let v2_bytes = encode_to_vec(&v2).expect("encode V2 failed");
    // V1 bytes must be a prefix of V2 bytes
    assert!(
        v2_bytes.len() >= v1_bytes.len(),
        "V2 encoded ({} bytes) must be at least as long as V1 ({} bytes)",
        v2_bytes.len(),
        v1_bytes.len()
    );
    assert_eq!(
        &v2_bytes[..v1_bytes.len()],
        v1_bytes.as_slice(),
        "V1 bytes must be a prefix of V2 bytes"
    );
}

// ── Test 5 ─────────────────────────────────────────────────────────────────
// V1 bytes can be decoded as just the first two fields of V2 manually
// (decode V1 bytes and check id+name match what V2 would have)
#[test]
fn test_v1_bytes_decode_as_v1_fields_of_v2() {
    let v1 = UserV1 {
        id: 55,
        name: String::from("dave"),
    };
    let v1_bytes = encode_to_vec(&v1).expect("encode V1 failed");
    // Decode V1 bytes back as V1
    let (decoded_v1, consumed): (UserV1, usize) =
        decode_from_slice(&v1_bytes).expect("decode V1 as V1 failed");
    assert_eq!(decoded_v1.id, 55);
    assert_eq!(decoded_v1.name, "dave");
    assert_eq!(consumed, v1_bytes.len());
    // Confirm V2 with empty email has the same id+name prefix
    let v2 = UserV2 {
        id: 55,
        name: String::from("dave"),
        email: String::from(""),
    };
    let v2_bytes = encode_to_vec(&v2).expect("encode V2 failed");
    assert_eq!(&v2_bytes[..v1_bytes.len()], v1_bytes.as_slice());
}

// ── Test 6 ─────────────────────────────────────────────────────────────────
// Config roundtrip with all fields
#[test]
fn test_config_roundtrip_all_fields() {
    let original = Config {
        version: 3,
        debug: true,
        timeout_ms: 5000,
        tags: vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode Config failed");
    let (decoded, _consumed): (Config, usize) =
        decode_from_slice(&encoded).expect("decode Config failed");
    assert_eq!(decoded, original);
}

// ── Test 7 ─────────────────────────────────────────────────────────────────
// Config consumed bytes == encoded len
#[test]
fn test_config_consumed_bytes_equals_encoded_len() {
    let original = Config {
        version: 1,
        debug: false,
        timeout_ms: 1000,
        tags: vec![String::from("tag1"), String::from("tag2")],
    };
    let encoded = encode_to_vec(&original).expect("encode Config failed");
    let (_decoded, consumed): (Config, usize) =
        decode_from_slice(&encoded).expect("decode Config failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ── Test 8 ─────────────────────────────────────────────────────────────────
// Action::Create roundtrip
#[test]
fn test_action_create_roundtrip() {
    let original = Action::Create;
    let encoded = encode_to_vec(&original).expect("encode Action::Create failed");
    let (decoded, _consumed): (Action, usize) =
        decode_from_slice(&encoded).expect("decode Action::Create failed");
    assert_eq!(decoded, original);
}

// ── Test 9 ─────────────────────────────────────────────────────────────────
// Action::Update roundtrip
#[test]
fn test_action_update_roundtrip() {
    let original = Action::Update {
        id: 100,
        field: String::from("email"),
    };
    let encoded = encode_to_vec(&original).expect("encode Action::Update failed");
    let (decoded, _consumed): (Action, usize) =
        decode_from_slice(&encoded).expect("decode Action::Update failed");
    assert_eq!(decoded, original);
}

// ── Test 10 ────────────────────────────────────────────────────────────────
// Action::Delete roundtrip
#[test]
fn test_action_delete_roundtrip() {
    let original = Action::Delete(42);
    let encoded = encode_to_vec(&original).expect("encode Action::Delete failed");
    let (decoded, _consumed): (Action, usize) =
        decode_from_slice(&encoded).expect("decode Action::Delete failed");
    assert_eq!(decoded, original);
}

// ── Test 11 ────────────────────────────────────────────────────────────────
// Action::Noop roundtrip
#[test]
fn test_action_noop_roundtrip() {
    let original = Action::Noop;
    let encoded = encode_to_vec(&original).expect("encode Action::Noop failed");
    let (decoded, _consumed): (Action, usize) =
        decode_from_slice(&encoded).expect("decode Action::Noop failed");
    assert_eq!(decoded, original);
}

// ── Test 12 ────────────────────────────────────────────────────────────────
// Vec<Action> all four variants roundtrip
#[test]
fn test_vec_action_all_variants_roundtrip() {
    let original: Vec<Action> = vec![
        Action::Create,
        Action::Update {
            id: 1,
            field: String::from("name"),
        },
        Action::Delete(7),
        Action::Noop,
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Action> failed");
    let (decoded, _consumed): (Vec<Action>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Action> failed");
    assert_eq!(decoded, original);
}

// ── Test 13 ────────────────────────────────────────────────────────────────
// UserV2 fixed int config roundtrip
#[test]
fn test_userv2_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = UserV2 {
        id: 1234,
        name: String::from("eve"),
        email: String::from("eve@mail.com"),
    };
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode UserV2 with fixed int failed");
    let (decoded, _consumed): (UserV2, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode UserV2 with fixed int failed");
    assert_eq!(decoded, original);
    // fixed int encoding: u32 always 4 bytes
    assert!(encoded.len() >= 4);
}

// ── Test 14 ────────────────────────────────────────────────────────────────
// Two Users with same id but different names produce different encodings
#[test]
fn test_two_users_same_id_different_names_different_encodings() {
    let user_a = UserV1 {
        id: 10,
        name: String::from("frank"),
    };
    let user_b = UserV1 {
        id: 10,
        name: String::from("grace"),
    };
    let bytes_a = encode_to_vec(&user_a).expect("encode user_a failed");
    let bytes_b = encode_to_vec(&user_b).expect("encode user_b failed");
    assert_ne!(
        bytes_a, bytes_b,
        "different names must produce different bytes"
    );
}

// ── Test 15 ────────────────────────────────────────────────────────────────
// Encoding V1 gives fewer bytes than V2 (V2 has more fields)
#[test]
fn test_v1_encoding_smaller_than_v2() {
    let v1 = UserV1 {
        id: 1,
        name: String::from("henry"),
    };
    let v2 = UserV2 {
        id: 1,
        name: String::from("henry"),
        email: String::from("henry@example.com"),
    };
    let v1_bytes = encode_to_vec(&v1).expect("encode V1 failed");
    let v2_bytes = encode_to_vec(&v2).expect("encode V2 failed");
    assert!(
        v1_bytes.len() < v2_bytes.len(),
        "V1 ({} bytes) must be smaller than V2 ({} bytes) when email is non-empty",
        v1_bytes.len(),
        v2_bytes.len()
    );
}

// ── Test 16 ────────────────────────────────────────────────────────────────
// Config tags empty roundtrip
#[test]
fn test_config_tags_empty_roundtrip() {
    let original = Config {
        version: 0,
        debug: false,
        timeout_ms: 0,
        tags: vec![],
    };
    let encoded = encode_to_vec(&original).expect("encode Config empty tags failed");
    let (decoded, consumed): (Config, usize) =
        decode_from_slice(&encoded).expect("decode Config empty tags failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.tags.len(), 0);
    assert_eq!(consumed, encoded.len());
}

// ── Test 17 ────────────────────────────────────────────────────────────────
// Config tags 5 items roundtrip
#[test]
fn test_config_tags_five_items_roundtrip() {
    let original = Config {
        version: 2,
        debug: true,
        timeout_ms: 9999,
        tags: vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
            String::from("five"),
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode Config 5 tags failed");
    let (decoded, consumed): (Config, usize) =
        decode_from_slice(&encoded).expect("decode Config 5 tags failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.tags.len(), 5);
    assert_eq!(consumed, encoded.len());
}

// ── Test 18 ────────────────────────────────────────────────────────────────
// UserV3 with unicode name roundtrip
#[test]
fn test_userv3_unicode_name_roundtrip() {
    let original = UserV3 {
        id: 88,
        name: String::from("日本語テスト"),
        email: String::from("unicode@test.jp"),
        active: false,
    };
    let encoded = encode_to_vec(&original).expect("encode UserV3 unicode failed");
    let (decoded, consumed): (UserV3, usize) =
        decode_from_slice(&encoded).expect("decode UserV3 unicode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── Test 19 ────────────────────────────────────────────────────────────────
// Vec<UserV1> 10 users roundtrip
#[test]
fn test_vec_userv1_ten_users_roundtrip() {
    let original: Vec<UserV1> = (0..10)
        .map(|i| UserV1 {
            id: i,
            name: format!("user_{}", i),
        })
        .collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<UserV1> failed");
    let (decoded, consumed): (Vec<UserV1>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<UserV1> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 10);
    assert_eq!(consumed, encoded.len());
}

// ── Test 20 ────────────────────────────────────────────────────────────────
// Option<UserV1> Some roundtrip
#[test]
fn test_option_userv1_some_roundtrip() {
    let original: Option<UserV1> = Some(UserV1 {
        id: 77,
        name: String::from("ivan"),
    });
    let encoded = encode_to_vec(&original).expect("encode Option<UserV1> Some failed");
    let (decoded, consumed): (Option<UserV1>, usize) =
        decode_from_slice(&encoded).expect("decode Option<UserV1> Some failed");
    assert_eq!(decoded, original);
    assert!(decoded.is_some());
    assert_eq!(consumed, encoded.len());
}

// ── Test 21 ────────────────────────────────────────────────────────────────
// Option<Action> None roundtrip
#[test]
fn test_option_action_none_roundtrip() {
    let original: Option<Action> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Action> None failed");
    let (decoded, consumed): (Option<Action>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Action> None failed");
    assert_eq!(decoded, original);
    assert!(decoded.is_none());
    assert_eq!(consumed, encoded.len());
}

// ── Test 22 ────────────────────────────────────────────────────────────────
// Nested: Vec<(UserV1, Action)> roundtrip
#[test]
fn test_vec_tuple_userv1_action_roundtrip() {
    let original: Vec<(UserV1, Action)> = vec![
        (
            UserV1 {
                id: 1,
                name: String::from("judy"),
            },
            Action::Create,
        ),
        (
            UserV1 {
                id: 2,
                name: String::from("karl"),
            },
            Action::Update {
                id: 2,
                field: String::from("name"),
            },
        ),
        (
            UserV1 {
                id: 3,
                name: String::from("lena"),
            },
            Action::Delete(3),
        ),
        (
            UserV1 {
                id: 4,
                name: String::from("mike"),
            },
            Action::Noop,
        ),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<(UserV1, Action)> failed");
    let (decoded, consumed): (Vec<(UserV1, Action)>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<(UserV1, Action)> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 4);
    assert_eq!(consumed, encoded.len());
}
