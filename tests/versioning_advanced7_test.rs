//! Wire format stability and versioning tests for OxiCode (set 7).
//!
//! Covers 22 scenarios exercising schema evolution between UserProfileV1/V2/V3,
//! wire format prefix relationships, ActionV1 enum encoding, collection round-
//! trips, size comparisons, and config variants.

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
// Versioned structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserProfileV1 {
    id: u64,
    name: String,
    age: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserProfileV2 {
    id: u64,
    name: String,
    age: u8,
    email: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserProfileV3 {
    id: u64,
    name: String,
    age: u8,
    email: Option<String>,
    active: Option<bool>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ActionV1 {
    Create(String),
    Delete(u64),
    Update { id: u64, field: String },
}

// ---------------------------------------------------------------------------
// Test 1: UserProfileV1 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_user_profile_v1_roundtrip() {
    let original = UserProfileV1 {
        id: 1001,
        name: String::from("Alice"),
        age: 30,
    };
    let encoded = encode_to_vec(&original).expect("encode UserProfileV1 failed");
    let (decoded, consumed): (UserProfileV1, usize) =
        decode_from_slice(&encoded).expect("decode UserProfileV1 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: UserProfileV2 with email Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_user_profile_v2_with_email_some_roundtrip() {
    let original = UserProfileV2 {
        id: 2002,
        name: String::from("Bob"),
        age: 25,
        email: Some(String::from("bob@example.com")),
    };
    let encoded = encode_to_vec(&original).expect("encode UserProfileV2 (Some email) failed");
    let (decoded, consumed): (UserProfileV2, usize) =
        decode_from_slice(&encoded).expect("decode UserProfileV2 (Some email) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: UserProfileV2 with email None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_user_profile_v2_with_email_none_roundtrip() {
    let original = UserProfileV2 {
        id: 3003,
        name: String::from("Carol"),
        age: 28,
        email: None,
    };
    let encoded = encode_to_vec(&original).expect("encode UserProfileV2 (None email) failed");
    let (decoded, consumed): (UserProfileV2, usize) =
        decode_from_slice(&encoded).expect("decode UserProfileV2 (None email) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: UserProfileV3 all fields roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_user_profile_v3_all_fields_roundtrip() {
    let original = UserProfileV3 {
        id: 4004,
        name: String::from("Dave"),
        age: 35,
        email: Some(String::from("dave@domain.org")),
        active: Some(true),
    };
    let encoded = encode_to_vec(&original).expect("encode UserProfileV3 (all fields) failed");
    let (decoded, consumed): (UserProfileV3, usize) =
        decode_from_slice(&encoded).expect("decode UserProfileV3 (all fields) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: V1 bytes + appended None byte decode as V2 (email: None)
//
// UserProfileV1 fields are a binary prefix of UserProfileV2 when V2's email
// is None.  Option::None encodes as a single 0x00 byte.  We manually construct
// the V2 bytes by appending 0x00 to the V1 bytes and verify decoding gives
// email: None.
// ---------------------------------------------------------------------------
#[test]
fn test_v1_bytes_extended_decode_as_v2_with_none_email() {
    let v1 = UserProfileV1 {
        id: 5005,
        name: String::from("Eve"),
        age: 22,
    };
    let v1_bytes = encode_to_vec(&v1).expect("encode V1 failed");

    // Append one 0x00 byte representing Option::None for the email field.
    let mut v2_bytes = v1_bytes.clone();
    v2_bytes.push(0x00u8);

    let (decoded_v2, consumed): (UserProfileV2, usize) =
        decode_from_slice(&v2_bytes).expect("decode V2 from extended V1 bytes failed");

    assert_eq!(decoded_v2.id, v1.id);
    assert_eq!(decoded_v2.name, v1.name);
    assert_eq!(decoded_v2.age, v1.age);
    assert_eq!(decoded_v2.email, None);
    assert_eq!(consumed, v2_bytes.len());
}

// ---------------------------------------------------------------------------
// Test 6: V1 bytes extended with two None bytes decode as V3 (both None)
//
// UserProfileV3 adds email: Option<String> and active: Option<bool>.
// Both absent from V1; appending two 0x00 bytes yields a valid V3 encoding
// with both optional fields set to None.
// ---------------------------------------------------------------------------
#[test]
fn test_v1_bytes_extended_decode_as_v3_with_none_fields() {
    let v1 = UserProfileV1 {
        id: 6006,
        name: String::from("Frank"),
        age: 40,
    };
    let v1_bytes = encode_to_vec(&v1).expect("encode V1 failed");

    // Append 0x00 for email (None) and 0x00 for active (None).
    let mut v3_bytes = v1_bytes.clone();
    v3_bytes.push(0x00u8); // email: None
    v3_bytes.push(0x00u8); // active: None

    let (decoded_v3, consumed): (UserProfileV3, usize) =
        decode_from_slice(&v3_bytes).expect("decode V3 from extended V1 bytes failed");

    assert_eq!(decoded_v3.id, v1.id);
    assert_eq!(decoded_v3.name, v1.name);
    assert_eq!(decoded_v3.age, v1.age);
    assert_eq!(decoded_v3.email, None);
    assert_eq!(decoded_v3.active, None);
    assert_eq!(consumed, v3_bytes.len());
}

// ---------------------------------------------------------------------------
// Test 7: V2 bytes with None email + appended None byte decode as V3
//
// Verify that V2 bytes (email: None) can be extended with one 0x00 byte
// (active: None) and decoded as V3 with both optional fields None.
// ---------------------------------------------------------------------------
#[test]
fn test_v2_none_email_bytes_extended_decode_as_v3() {
    let v2 = UserProfileV2 {
        id: 7007,
        name: String::from("Grace"),
        age: 31,
        email: None,
    };
    let v2_bytes = encode_to_vec(&v2).expect("encode V2 (None email) failed");

    // Append 0x00 for active (None).
    let mut v3_bytes = v2_bytes.clone();
    v3_bytes.push(0x00u8); // active: None

    let (decoded_v3, consumed): (UserProfileV3, usize) =
        decode_from_slice(&v3_bytes).expect("decode V3 from extended V2 bytes failed");

    assert_eq!(decoded_v3.id, v2.id);
    assert_eq!(decoded_v3.name, v2.name);
    assert_eq!(decoded_v3.age, v2.age);
    assert_eq!(decoded_v3.email, None);
    assert_eq!(decoded_v3.active, None);
    assert_eq!(consumed, v3_bytes.len());
}

// ---------------------------------------------------------------------------
// Test 8: ActionV1::Create roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_action_v1_create_roundtrip() {
    let original = ActionV1::Create(String::from("new-resource"));
    let encoded = encode_to_vec(&original).expect("encode ActionV1::Create failed");
    let (decoded, consumed): (ActionV1, usize) =
        decode_from_slice(&encoded).expect("decode ActionV1::Create failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: ActionV1::Delete roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_action_v1_delete_roundtrip() {
    let original = ActionV1::Delete(9876543210u64);
    let encoded = encode_to_vec(&original).expect("encode ActionV1::Delete failed");
    let (decoded, consumed): (ActionV1, usize) =
        decode_from_slice(&encoded).expect("decode ActionV1::Delete failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: ActionV1::Update roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_action_v1_update_roundtrip() {
    let original = ActionV1::Update {
        id: 1234567890u64,
        field: String::from("display_name"),
    };
    let encoded = encode_to_vec(&original).expect("encode ActionV1::Update failed");
    let (decoded, consumed): (ActionV1, usize) =
        decode_from_slice(&encoded).expect("decode ActionV1::Update failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Vec<UserProfileV1> roundtrip with 5 items
// ---------------------------------------------------------------------------
#[test]
fn test_vec_user_profile_v1_roundtrip_five_items() {
    let original: Vec<UserProfileV1> = vec![
        UserProfileV1 {
            id: 1,
            name: String::from("Alice"),
            age: 20,
        },
        UserProfileV1 {
            id: 2,
            name: String::from("Bob"),
            age: 21,
        },
        UserProfileV1 {
            id: 3,
            name: String::from("Carol"),
            age: 22,
        },
        UserProfileV1 {
            id: 4,
            name: String::from("Dave"),
            age: 23,
        },
        UserProfileV1 {
            id: 5,
            name: String::from("Eve"),
            age: 24,
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<UserProfileV1> failed");
    let (decoded, consumed): (Vec<UserProfileV1>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<UserProfileV1> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 5);
}

// ---------------------------------------------------------------------------
// Test 12: Vec<ActionV1> roundtrip (all variants)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_action_v1_all_variants_roundtrip() {
    let original: Vec<ActionV1> = vec![
        ActionV1::Create(String::from("resource-a")),
        ActionV1::Delete(42u64),
        ActionV1::Update {
            id: 99u64,
            field: String::from("title"),
        },
        ActionV1::Create(String::from("resource-b")),
        ActionV1::Delete(0u64),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<ActionV1> failed");
    let (decoded, consumed): (Vec<ActionV1>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<ActionV1> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: V1 encoding is smaller than V2 with Some email
//
// V2 with a non-None email contains more bytes than V1.
// ---------------------------------------------------------------------------
#[test]
fn test_v1_encoding_smaller_than_v2_with_some_email() {
    let v1 = UserProfileV1 {
        id: 100,
        name: String::from("Henry"),
        age: 45,
    };
    let v2 = UserProfileV2 {
        id: 100,
        name: String::from("Henry"),
        age: 45,
        email: Some(String::from("henry@mail.com")),
    };
    let v1_bytes = encode_to_vec(&v1).expect("encode V1 failed");
    let v2_bytes = encode_to_vec(&v2).expect("encode V2 (Some email) failed");
    assert!(
        v1_bytes.len() < v2_bytes.len(),
        "V1 ({} bytes) must be smaller than V2 with Some email ({} bytes)",
        v1_bytes.len(),
        v2_bytes.len()
    );
}

// ---------------------------------------------------------------------------
// Test 14: V2 with Some email is larger than V2 with None email
// ---------------------------------------------------------------------------
#[test]
fn test_v2_some_email_larger_than_v2_none_email() {
    let v2_none = UserProfileV2 {
        id: 200,
        name: String::from("Irene"),
        age: 33,
        email: None,
    };
    let v2_some = UserProfileV2 {
        id: 200,
        name: String::from("Irene"),
        age: 33,
        email: Some(String::from("irene@example.com")),
    };
    let bytes_none = encode_to_vec(&v2_none).expect("encode V2 (None) failed");
    let bytes_some = encode_to_vec(&v2_some).expect("encode V2 (Some) failed");
    assert!(
        bytes_some.len() > bytes_none.len(),
        "V2 with Some email ({} bytes) must be larger than V2 with None email ({} bytes)",
        bytes_some.len(),
        bytes_none.len()
    );
}

// ---------------------------------------------------------------------------
// Test 15: Fixed-int config with UserProfileV1 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_config_user_profile_v1_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = UserProfileV1 {
        id: 300,
        name: String::from("Jake"),
        age: 50,
    };
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode V1 with fixed int failed");
    let (decoded, consumed): (UserProfileV1, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode V1 with fixed int failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // With fixed int encoding u64 is always 8 bytes, so the total must be >= 8.
    assert!(
        encoded.len() >= 8,
        "fixed-int u64 must occupy at least 8 bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Big-endian config with UserProfileV1 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_big_endian_config_user_profile_v1_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original = UserProfileV1 {
        id: 400,
        name: String::from("Karen"),
        age: 27,
    };
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode V1 with big-endian failed");
    let (decoded, consumed): (UserProfileV1, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode V1 with big-endian failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Option<UserProfileV1> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_user_profile_v1_some_roundtrip() {
    let inner = UserProfileV1 {
        id: 500,
        name: String::from("Leo"),
        age: 19,
    };
    let original: Option<UserProfileV1> = Some(inner);
    let encoded = encode_to_vec(&original).expect("encode Option<UserProfileV1> Some failed");
    let (decoded, consumed): (Option<UserProfileV1>, usize) =
        decode_from_slice(&encoded).expect("decode Option<UserProfileV1> Some failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_some());
}

// ---------------------------------------------------------------------------
// Test 18: Option<UserProfileV1> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_user_profile_v1_none_roundtrip() {
    let original: Option<UserProfileV1> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<UserProfileV1> None failed");
    let (decoded, consumed): (Option<UserProfileV1>, usize) =
        decode_from_slice(&encoded).expect("decode Option<UserProfileV1> None failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_none());
    // Option::None must encode to a single zero byte.
    assert_eq!(encoded.len(), 1);
    assert_eq!(encoded[0], 0x00u8);
}

// ---------------------------------------------------------------------------
// Test 19: Consumed bytes equals encoded length for V3
// ---------------------------------------------------------------------------
#[test]
fn test_v3_consumed_bytes_equals_encoded_length() {
    let original = UserProfileV3 {
        id: 600,
        name: String::from("Mia"),
        age: 32,
        email: Some(String::from("mia@test.io")),
        active: Some(false),
    };
    let encoded = encode_to_vec(&original).expect("encode V3 failed");
    let (_decoded, consumed): (UserProfileV3, usize) =
        decode_from_slice(&encoded).expect("decode V3 consumed bytes failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal full encoded length for V3"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Deterministic encoding — same value encodes identically twice
// ---------------------------------------------------------------------------
#[test]
fn test_v1_encoding_is_deterministic() {
    let val = UserProfileV1 {
        id: 700,
        name: String::from("Noah"),
        age: 44,
    };
    let first = encode_to_vec(&val).expect("first encode V1 failed");
    let second = encode_to_vec(&val).expect("second encode V1 failed");
    assert_eq!(
        first, second,
        "encoding must be deterministic for identical values"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Different user IDs produce different encodings
// ---------------------------------------------------------------------------
#[test]
fn test_different_user_ids_produce_different_encodings() {
    let user_a = UserProfileV1 {
        id: 801,
        name: String::from("Olivia"),
        age: 29,
    };
    let user_b = UserProfileV1 {
        id: 802,
        name: String::from("Olivia"),
        age: 29,
    };
    let bytes_a = encode_to_vec(&user_a).expect("encode user_a failed");
    let bytes_b = encode_to_vec(&user_b).expect("encode user_b failed");
    assert_ne!(
        bytes_a, bytes_b,
        "different user IDs must produce different byte encodings"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Vec<UserProfileV3> roundtrip with 3 items, mixed email/active
// ---------------------------------------------------------------------------
#[test]
fn test_vec_user_profile_v3_mixed_fields_roundtrip() {
    let original: Vec<UserProfileV3> = vec![
        UserProfileV3 {
            id: 901,
            name: String::from("Paul"),
            age: 38,
            email: Some(String::from("paul@example.com")),
            active: Some(true),
        },
        UserProfileV3 {
            id: 902,
            name: String::from("Quinn"),
            age: 26,
            email: None,
            active: Some(false),
        },
        UserProfileV3 {
            id: 903,
            name: String::from("Rita"),
            age: 55,
            email: Some(String::from("rita@domain.net")),
            active: None,
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<UserProfileV3> failed");
    let (decoded, consumed): (Vec<UserProfileV3>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<UserProfileV3> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
    // Spot-check mixed field values.
    assert_eq!(decoded[0].email, Some(String::from("paul@example.com")));
    assert_eq!(decoded[1].email, None);
    assert_eq!(decoded[2].active, None);
}
