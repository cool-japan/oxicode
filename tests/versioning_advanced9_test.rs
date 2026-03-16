//! Advanced versioning tests — user profile schema evolution (set 9).
//!
//! Exercises encode_versioned_value / decode_versioned_value with V1/V2/V3
//! UserProfile structs to validate forward-compatible storage patterns.

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
use oxicode::versioning::{Version, VERSIONED_MAGIC};
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, decode_versioned_value,
    encode_to_vec, encode_to_vec_with_config, encode_versioned_value, Decode, Encode,
};

// ── Data structures ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserProfileV1 {
    id: u64,
    username: String,
    email: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserProfileV2 {
    id: u64,
    username: String,
    email: String,
    bio: Option<String>,
    created_at: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserProfileV3 {
    id: u64,
    username: String,
    email: String,
    bio: Option<String>,
    created_at: u64,
    avatar_url: Option<String>,
    follower_count: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AccountStatus {
    Active,
    Suspended { reason: String },
    Deleted,
    PendingVerification,
}

// ── Helper constructors ──────────────────────────────────────────────────────

fn sample_v1() -> UserProfileV1 {
    UserProfileV1 {
        id: 1001,
        username: String::from("alice"),
        email: String::from("alice@example.com"),
    }
}

fn sample_v2_with_bio() -> UserProfileV2 {
    UserProfileV2 {
        id: 2002,
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        bio: Some(String::from("Rustacean and coffee enthusiast")),
        created_at: 1_700_000_000,
    }
}

fn sample_v2_no_bio() -> UserProfileV2 {
    UserProfileV2 {
        id: 3003,
        username: String::from("carol"),
        email: String::from("carol@example.com"),
        bio: None,
        created_at: 1_710_000_000,
    }
}

fn sample_v3_full() -> UserProfileV3 {
    UserProfileV3 {
        id: 4004,
        username: String::from("dan"),
        email: String::from("dan@example.com"),
        bio: Some(String::from("Open-source contributor")),
        created_at: 1_720_000_000,
        avatar_url: Some(String::from("https://cdn.example.com/dan.png")),
        follower_count: 512,
    }
}

fn sample_v3_minimal() -> UserProfileV3 {
    UserProfileV3 {
        id: 5005,
        username: String::from("eve"),
        email: String::from("eve@example.com"),
        bio: None,
        created_at: 1_730_000_000,
        avatar_url: None,
        follower_count: 0,
    }
}

// ── Test 1: UserProfileV1 basic roundtrip ───────────────────────────────────

#[test]
fn test_user_profile_v1_basic_roundtrip() {
    let original = sample_v1();
    let bytes = encode_to_vec(&original).expect("encode UserProfileV1 failed");
    let (decoded, _): (UserProfileV1, usize) =
        decode_from_slice(&bytes).expect("decode UserProfileV1 failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.id, 1001);
    assert_eq!(decoded.username, "alice");
    assert_eq!(decoded.email, "alice@example.com");
}

// ── Test 2: UserProfileV2 basic roundtrip ───────────────────────────────────

#[test]
fn test_user_profile_v2_basic_roundtrip() {
    let original = sample_v2_with_bio();
    let bytes = encode_to_vec(&original).expect("encode UserProfileV2 failed");
    let (decoded, _): (UserProfileV2, usize) =
        decode_from_slice(&bytes).expect("decode UserProfileV2 failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.id, 2002);
    assert_eq!(
        decoded.bio,
        Some(String::from("Rustacean and coffee enthusiast"))
    );
    assert_eq!(decoded.created_at, 1_700_000_000);
}

// ── Test 3: UserProfileV3 basic roundtrip ───────────────────────────────────

#[test]
fn test_user_profile_v3_basic_roundtrip() {
    let original = sample_v3_full();
    let bytes = encode_to_vec(&original).expect("encode UserProfileV3 failed");
    let (decoded, _): (UserProfileV3, usize) =
        decode_from_slice(&bytes).expect("decode UserProfileV3 failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.follower_count, 512);
    assert_eq!(
        decoded.avatar_url,
        Some(String::from("https://cdn.example.com/dan.png"))
    );
}

// ── Test 4: encode_versioned_value for V1 — verify version marker bytes ─────

#[test]
fn test_versioned_v1_bytes_start_with_magic() {
    let val = sample_v1();
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&val, version).expect("encode_versioned_value V1 failed");
    // The versioned header must start with the OXIV magic bytes
    assert!(
        encoded.len() > VERSIONED_MAGIC.len(),
        "encoded buffer too short to contain magic header"
    );
    assert_eq!(
        &encoded[..4],
        &VERSIONED_MAGIC,
        "first 4 bytes must be OXIV magic"
    );
    // Confirm the version is recoverable
    let (decoded, ver, _consumed): (UserProfileV1, Version, usize) =
        decode_versioned_value::<UserProfileV1>(&encoded)
            .expect("decode_versioned_value V1 failed");
    assert_eq!(decoded, val);
    assert_eq!(ver, version);
}

// ── Test 5: encode_versioned_value for V2 — verify version marker bytes ─────

#[test]
fn test_versioned_v2_bytes_start_with_magic() {
    let val = sample_v2_with_bio();
    let version = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&val, version).expect("encode_versioned_value V2 failed");
    assert_eq!(
        &encoded[..4],
        &VERSIONED_MAGIC,
        "first 4 bytes must be OXIV magic"
    );
    let (decoded, ver, _consumed): (UserProfileV2, Version, usize) =
        decode_versioned_value::<UserProfileV2>(&encoded)
            .expect("decode_versioned_value V2 failed");
    assert_eq!(decoded, val);
    assert_eq!(ver, version);
}

// ── Test 6: encode_versioned_value for V3 — verify version marker bytes ─────

#[test]
fn test_versioned_v3_bytes_start_with_magic() {
    let val = sample_v3_full();
    let version = Version::new(3, 0, 0);
    let encoded = encode_versioned_value(&val, version).expect("encode_versioned_value V3 failed");
    assert_eq!(
        &encoded[..4],
        &VERSIONED_MAGIC,
        "first 4 bytes must be OXIV magic"
    );
    let (decoded, ver, _consumed): (UserProfileV3, Version, usize) =
        decode_versioned_value::<UserProfileV3>(&encoded)
            .expect("decode_versioned_value V3 failed");
    assert_eq!(decoded, val);
    assert_eq!(ver, version);
}

// ── Test 7: V1 and V2 encode to different byte sequences ────────────────────

#[test]
fn test_v1_and_v2_produce_different_bytes() {
    // Use the same id/username/email so any difference comes from the schema.
    let v1 = UserProfileV1 {
        id: 9999,
        username: String::from("tester"),
        email: String::from("tester@example.com"),
    };
    let v2 = UserProfileV2 {
        id: 9999,
        username: String::from("tester"),
        email: String::from("tester@example.com"),
        bio: None,
        created_at: 0,
    };
    let bytes_v1 = encode_to_vec(&v1).expect("encode V1 failed");
    let bytes_v2 = encode_to_vec(&v2).expect("encode V2 failed");
    // V2 has two extra fields (bio: None, created_at: 0) so it must be longer
    assert_ne!(
        bytes_v1, bytes_v2,
        "V1 and V2 must encode to different byte sequences"
    );
    assert!(
        bytes_v2.len() > bytes_v1.len(),
        "V2 bytes must be longer than V1 bytes due to extra fields"
    );
}

// ── Test 8: V2 and V3 encode to different byte sequences ────────────────────

#[test]
fn test_v2_and_v3_produce_different_bytes() {
    let v2 = UserProfileV2 {
        id: 8888,
        username: String::from("user8"),
        email: String::from("u8@example.com"),
        bio: None,
        created_at: 0,
    };
    let v3 = UserProfileV3 {
        id: 8888,
        username: String::from("user8"),
        email: String::from("u8@example.com"),
        bio: None,
        created_at: 0,
        avatar_url: None,
        follower_count: 0,
    };
    let bytes_v2 = encode_to_vec(&v2).expect("encode V2 failed");
    let bytes_v3 = encode_to_vec(&v3).expect("encode V3 failed");
    assert_ne!(
        bytes_v2, bytes_v3,
        "V2 and V3 must encode to different byte sequences"
    );
    assert!(
        bytes_v3.len() > bytes_v2.len(),
        "V3 bytes must be longer than V2 bytes due to extra fields"
    );
}

// ── Test 9: Vec<UserProfileV1> roundtrip ────────────────────────────────────

#[test]
fn test_vec_user_profile_v1_roundtrip() {
    let profiles = vec![
        UserProfileV1 {
            id: 1,
            username: String::from("alpha"),
            email: String::from("alpha@example.com"),
        },
        UserProfileV1 {
            id: 2,
            username: String::from("beta"),
            email: String::from("beta@example.com"),
        },
        UserProfileV1 {
            id: 3,
            username: String::from("gamma"),
            email: String::from("gamma@example.com"),
        },
    ];
    let bytes = encode_to_vec(&profiles).expect("encode Vec<UserProfileV1> failed");
    let (decoded, _): (Vec<UserProfileV1>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<UserProfileV1> failed");
    assert_eq!(profiles, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[1].username, "beta");
}

// ── Test 10: Vec<UserProfileV2> roundtrip ───────────────────────────────────

#[test]
fn test_vec_user_profile_v2_roundtrip() {
    let profiles = vec![sample_v2_no_bio(), sample_v2_with_bio()];
    let bytes = encode_to_vec(&profiles).expect("encode Vec<UserProfileV2> failed");
    let (decoded, _): (Vec<UserProfileV2>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<UserProfileV2> failed");
    assert_eq!(profiles, decoded);
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded[0].bio, None);
    assert!(decoded[1].bio.is_some());
}

// ── Test 11: Option<UserProfileV1> Some roundtrip ───────────────────────────

#[test]
fn test_option_user_profile_v1_some_roundtrip() {
    let opt: Option<UserProfileV1> = Some(sample_v1());
    let bytes = encode_to_vec(&opt).expect("encode Option<UserProfileV1> Some failed");
    let (decoded, _): (Option<UserProfileV1>, usize) =
        decode_from_slice(&bytes).expect("decode Option<UserProfileV1> Some failed");
    assert!(decoded.is_some());
    let inner = decoded.expect("inner value must be Some");
    assert_eq!(inner.id, 1001);
    assert_eq!(inner.username, "alice");
}

// ── Test 12: Option<UserProfileV2> None roundtrip ───────────────────────────

#[test]
fn test_option_user_profile_v2_none_roundtrip() {
    let opt: Option<UserProfileV2> = None;
    let bytes = encode_to_vec(&opt).expect("encode Option<UserProfileV2> None failed");
    let (decoded, _): (Option<UserProfileV2>, usize) =
        decode_from_slice(&bytes).expect("decode Option<UserProfileV2> None failed");
    assert!(decoded.is_none(), "decoded Option must be None");
}

// ── Test 13: UserProfileV2 with None bio ────────────────────────────────────

#[test]
fn test_user_profile_v2_none_bio() {
    let profile = sample_v2_no_bio();
    let bytes = encode_to_vec(&profile).expect("encode V2 no bio failed");
    let (decoded, _): (UserProfileV2, usize) =
        decode_from_slice(&bytes).expect("decode V2 no bio failed");
    assert_eq!(decoded.bio, None, "bio must round-trip as None");
    assert_eq!(decoded.id, 3003);
    assert_eq!(decoded.created_at, 1_710_000_000);
}

// ── Test 14: UserProfileV2 with Some bio ────────────────────────────────────

#[test]
fn test_user_profile_v2_some_bio() {
    let profile = sample_v2_with_bio();
    let bytes = encode_to_vec(&profile).expect("encode V2 with bio failed");
    let (decoded, _): (UserProfileV2, usize) =
        decode_from_slice(&bytes).expect("decode V2 with bio failed");
    assert_eq!(
        decoded.bio,
        Some(String::from("Rustacean and coffee enthusiast")),
        "bio must round-trip as Some"
    );
    assert_eq!(decoded.id, 2002);
}

// ── Test 15: UserProfileV3 with all fields populated ────────────────────────

#[test]
fn test_user_profile_v3_all_fields_populated() {
    let profile = sample_v3_full();
    let bytes = encode_to_vec(&profile).expect("encode V3 full failed");
    let (decoded, _): (UserProfileV3, usize) =
        decode_from_slice(&bytes).expect("decode V3 full failed");
    assert_eq!(decoded.id, 4004);
    assert_eq!(decoded.bio, Some(String::from("Open-source contributor")));
    assert_eq!(
        decoded.avatar_url,
        Some(String::from("https://cdn.example.com/dan.png"))
    );
    assert_eq!(decoded.follower_count, 512);
    assert_eq!(decoded.created_at, 1_720_000_000);
}

// ── Test 16: UserProfileV3 with both Option fields None ─────────────────────

#[test]
fn test_user_profile_v3_both_options_none() {
    let profile = sample_v3_minimal();
    let bytes = encode_to_vec(&profile).expect("encode V3 minimal failed");
    let (decoded, _): (UserProfileV3, usize) =
        decode_from_slice(&bytes).expect("decode V3 minimal failed");
    assert_eq!(decoded.bio, None, "bio must be None");
    assert_eq!(decoded.avatar_url, None, "avatar_url must be None");
    assert_eq!(decoded.follower_count, 0);
    assert_eq!(decoded.id, 5005);
}

// ── Test 17: AccountStatus::Active roundtrip ────────────────────────────────

#[test]
fn test_account_status_active_roundtrip() {
    let status = AccountStatus::Active;
    let bytes = encode_to_vec(&status).expect("encode AccountStatus::Active failed");
    let (decoded, _): (AccountStatus, usize) =
        decode_from_slice(&bytes).expect("decode AccountStatus::Active failed");
    assert_eq!(decoded, AccountStatus::Active);
}

// ── Test 18: AccountStatus::Suspended roundtrip ─────────────────────────────

#[test]
fn test_account_status_suspended_roundtrip() {
    let status = AccountStatus::Suspended {
        reason: String::from("Terms of service violation"),
    };
    let bytes = encode_to_vec(&status).expect("encode AccountStatus::Suspended failed");
    let (decoded, _): (AccountStatus, usize) =
        decode_from_slice(&bytes).expect("decode AccountStatus::Suspended failed");
    assert_eq!(
        decoded,
        AccountStatus::Suspended {
            reason: String::from("Terms of service violation")
        }
    );
    if let AccountStatus::Suspended { reason } = decoded {
        assert_eq!(reason, "Terms of service violation");
    } else {
        panic!("expected AccountStatus::Suspended");
    }
}

// ── Test 19: AccountStatus::Deleted roundtrip ───────────────────────────────

#[test]
fn test_account_status_deleted_roundtrip() {
    let status = AccountStatus::Deleted;
    let bytes = encode_to_vec(&status).expect("encode AccountStatus::Deleted failed");
    let (decoded, _): (AccountStatus, usize) =
        decode_from_slice(&bytes).expect("decode AccountStatus::Deleted failed");
    assert_eq!(decoded, AccountStatus::Deleted);
}

// ── Test 20: Consumed bytes == encoded length for V3 ────────────────────────

#[test]
fn test_consumed_bytes_equals_encoded_length_v3() {
    let profile = sample_v3_full();
    let bytes = encode_to_vec(&profile).expect("encode V3 for consumed test failed");
    let encoded_len = bytes.len();
    let (_decoded, consumed): (UserProfileV3, usize) =
        decode_from_slice(&bytes).expect("decode V3 for consumed test failed");
    assert_eq!(
        consumed, encoded_len,
        "bytes consumed must equal total encoded length"
    );
}

// ── Test 21: Big-endian config roundtrip for V2 ─────────────────────────────

#[test]
fn test_big_endian_config_roundtrip_v2() {
    let profile = sample_v2_with_bio();
    let be_config = config::standard().with_big_endian();
    let bytes =
        encode_to_vec_with_config(&profile, be_config).expect("big-endian encode V2 failed");
    let (decoded, _): (UserProfileV2, usize) =
        decode_from_slice_with_config(&bytes, be_config).expect("big-endian decode V2 failed");
    assert_eq!(profile, decoded);
    // Confirm the big-endian bytes differ from the little-endian (standard) bytes
    let le_bytes = encode_to_vec(&profile).expect("little-endian encode V2 failed");
    // Big-endian and little-endian multi-byte integers encode differently;
    // the strings/bool bytes are the same, but numeric fields differ.
    assert_ne!(
        bytes, le_bytes,
        "big-endian and little-endian encodings of V2 must differ"
    );
}

// ── Test 22: Fixed-int config roundtrip for V1 ──────────────────────────────

#[test]
fn test_fixed_int_config_roundtrip_v1() {
    let profile = sample_v1();
    let fixed_config = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&profile, fixed_config).expect("fixed-int encode V1 failed");
    let (decoded, _): (UserProfileV1, usize) =
        decode_from_slice_with_config(&bytes, fixed_config).expect("fixed-int decode V1 failed");
    assert_eq!(profile, decoded);
    // Fixed-int uses 8 bytes for u64 vs varint — bytes should be longer
    let varint_bytes = encode_to_vec(&profile).expect("varint encode V1 failed");
    assert!(
        bytes.len() >= varint_bytes.len(),
        "fixed-int encoding must be at least as long as varint for V1 (u64 id is 8 bytes vs varint)"
    );
}
