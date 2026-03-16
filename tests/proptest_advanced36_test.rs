//! Advanced property-based roundtrip tests (set 36) using proptest.
//!
//! Theme: Social media / user content.
//! Types: ContentType, Post, UserProfile, Reaction.
//! Each proptest! block contains exactly one #[test] function.
//! Tests verify that encode → decode is a perfect roundtrip for all tested types.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContentType {
    Text,
    Image,
    Video,
    Audio,
    Link,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Post {
    id: u64,
    user_id: u64,
    content_type: ContentType,
    text: String,
    likes: u32,
    is_public: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UserProfile {
    user_id: u64,
    username: String,
    follower_count: u64,
    is_verified: bool,
    bio: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Reaction {
    post_id: u64,
    user_id: u64,
    reaction_type: u8,
}

fn content_type_from_u8(n: u8) -> ContentType {
    match n % 5 {
        0 => ContentType::Text,
        1 => ContentType::Image,
        2 => ContentType::Video,
        3 => ContentType::Audio,
        _ => ContentType::Link,
    }
}

fn post_strategy() -> impl Strategy<Value = Post> {
    (
        any::<u64>(),
        any::<u64>(),
        0u8..5u8,
        "[a-z ]{0,64}",
        any::<u32>(),
        any::<bool>(),
    )
        .prop_map(|(id, user_id, ct_idx, text, likes, is_public)| Post {
            id,
            user_id,
            content_type: content_type_from_u8(ct_idx),
            text,
            likes,
            is_public,
        })
}

fn user_profile_strategy() -> impl Strategy<Value = UserProfile> {
    (
        any::<u64>(),
        "[a-z]{1,32}",
        any::<u64>(),
        any::<bool>(),
        proptest::option::of("[a-z ]{0,128}"),
    )
        .prop_map(
            |(user_id, username, follower_count, is_verified, bio)| UserProfile {
                user_id,
                username,
                follower_count,
                is_verified,
                bio,
            },
        )
}

fn reaction_strategy() -> impl Strategy<Value = Reaction> {
    (any::<u64>(), any::<u64>(), any::<u8>()).prop_map(|(post_id, user_id, reaction_type)| {
        Reaction {
            post_id,
            user_id,
            reaction_type,
        }
    })
}

// Test 1: Post full roundtrip
proptest! {
    #[test]
    fn prop_post_roundtrip(post in post_strategy()) {
        let encoded = encode_to_vec(&post).expect("encode failed");
        let (decoded, _): (Post, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(post, decoded);
    }
}

// Test 2: ContentType variant roundtrip via index
proptest! {
    #[test]
    fn prop_content_type_variant_roundtrip(n in 0u8..5u8) {
        let ct = content_type_from_u8(n);
        let encoded = encode_to_vec(&ct).expect("encode failed");
        let (decoded, _): (ContentType, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(ct, decoded);
    }
}

// Test 3: UserProfile full roundtrip
proptest! {
    #[test]
    fn prop_user_profile_roundtrip(profile in user_profile_strategy()) {
        let encoded = encode_to_vec(&profile).expect("encode failed");
        let (decoded, _): (UserProfile, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(profile, decoded);
    }
}

// Test 4: Reaction roundtrip
proptest! {
    #[test]
    fn prop_reaction_roundtrip(reaction in reaction_strategy()) {
        let encoded = encode_to_vec(&reaction).expect("encode failed");
        let (decoded, _): (Reaction, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(reaction, decoded);
    }
}

// Test 5: Vec<Post> roundtrip (0..8 items)
proptest! {
    #[test]
    fn prop_vec_post_roundtrip(
        posts in proptest::collection::vec(post_strategy(), 0..8)
    ) {
        let encoded = encode_to_vec(&posts).expect("encode failed");
        let (decoded, _): (Vec<Post>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(posts, decoded);
    }
}

// Test 6: Vec<UserProfile> roundtrip (0..5 items)
proptest! {
    #[test]
    fn prop_vec_user_profile_roundtrip(
        profiles in proptest::collection::vec(user_profile_strategy(), 0..5)
    ) {
        let encoded = encode_to_vec(&profiles).expect("encode failed");
        let (decoded, _): (Vec<UserProfile>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(profiles, decoded);
    }
}

// Test 7: Vec<Reaction> roundtrip (0..10 items)
proptest! {
    #[test]
    fn prop_vec_reaction_roundtrip(
        reactions in proptest::collection::vec(reaction_strategy(), 0..10)
    ) {
        let encoded = encode_to_vec(&reactions).expect("encode failed");
        let (decoded, _): (Vec<Reaction>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(reactions, decoded);
    }
}

// Test 8: Option<bio> — None encodes differently from Some
proptest! {
    #[test]
    fn prop_option_bio_none_vs_some_differ(bio in "[a-z ]{1,64}") {
        let none_val: Option<String> = None;
        let some_val: Option<String> = Some(bio);
        let encoded_none = encode_to_vec(&none_val).expect("encode failed");
        let encoded_some = encode_to_vec(&some_val).expect("encode failed");
        prop_assert_ne!(encoded_none, encoded_some);
    }
}

// Test 9: UserProfile with bio=None roundtrip and consumed bytes check
proptest! {
    #[test]
    fn prop_user_profile_bio_none_consumed_eq_len(
        user_id: u64,
        follower_count: u64,
        is_verified: bool,
    ) {
        let profile = UserProfile {
            user_id,
            username: "testuser".to_string(),
            follower_count,
            is_verified,
            bio: None,
        };
        let encoded = encode_to_vec(&profile).expect("encode failed");
        let (decoded, consumed): (UserProfile, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(profile, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 10: UserProfile with bio=Some roundtrip and consumed bytes check
proptest! {
    #[test]
    fn prop_user_profile_bio_some_consumed_eq_len(
        user_id: u64,
        bio in "[a-z ]{1,64}",
    ) {
        let profile = UserProfile {
            user_id,
            username: "biouser".to_string(),
            follower_count: 42,
            is_verified: false,
            bio: Some(bio),
        };
        let encoded = encode_to_vec(&profile).expect("encode failed");
        let (decoded, consumed): (UserProfile, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(profile, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 11: Deterministic encoding for Post
proptest! {
    #[test]
    fn prop_post_deterministic_encoding(post in post_strategy()) {
        let encoded_a = encode_to_vec(&post).expect("encode failed");
        let encoded_b = encode_to_vec(&post).expect("encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// Test 12: Deterministic encoding for UserProfile
proptest! {
    #[test]
    fn prop_user_profile_deterministic_encoding(profile in user_profile_strategy()) {
        let encoded_a = encode_to_vec(&profile).expect("encode failed");
        let encoded_b = encode_to_vec(&profile).expect("encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// Test 13: Consumed bytes == encoded length for Post
proptest! {
    #[test]
    fn prop_post_consumed_eq_len(post in post_strategy()) {
        let encoded = encode_to_vec(&post).expect("encode failed");
        let (_, consumed): (Post, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 14: Consumed bytes == encoded length for Reaction
proptest! {
    #[test]
    fn prop_reaction_consumed_eq_len(reaction in reaction_strategy()) {
        let encoded = encode_to_vec(&reaction).expect("encode failed");
        let (_, consumed): (Reaction, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 15: Distinct Posts with different IDs encode differently
proptest! {
    #[test]
    fn prop_posts_different_id_encode_differently(
        id_a: u64,
        id_b: u64,
        user_id: u64,
        ct_idx in 0u8..5u8,
        likes: u32,
        is_public: bool,
    ) {
        prop_assume!(id_a != id_b);
        let post_a = Post {
            id: id_a,
            user_id,
            content_type: content_type_from_u8(ct_idx),
            text: "hello".to_string(),
            likes,
            is_public,
        };
        let post_b = Post {
            id: id_b,
            user_id,
            content_type: content_type_from_u8(ct_idx),
            text: "hello".to_string(),
            likes,
            is_public,
        };
        let encoded_a = encode_to_vec(&post_a).expect("encode failed");
        let encoded_b = encode_to_vec(&post_b).expect("encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// Test 16: Distinct UserProfiles with different user_ids encode differently
proptest! {
    #[test]
    fn prop_user_profiles_different_id_encode_differently(
        user_id_a: u64,
        user_id_b: u64,
        follower_count: u64,
        is_verified: bool,
    ) {
        prop_assume!(user_id_a != user_id_b);
        let profile_a = UserProfile {
            user_id: user_id_a,
            username: "user".to_string(),
            follower_count,
            is_verified,
            bio: None,
        };
        let profile_b = UserProfile {
            user_id: user_id_b,
            username: "user".to_string(),
            follower_count,
            is_verified,
            bio: None,
        };
        let encoded_a = encode_to_vec(&profile_a).expect("encode failed");
        let encoded_b = encode_to_vec(&profile_b).expect("encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// Test 17: All five ContentType variants encode to distinct bytes
proptest! {
    #[test]
    fn prop_all_content_type_variants_distinct(_dummy: u8) {
        let variants = [
            ContentType::Text,
            ContentType::Image,
            ContentType::Video,
            ContentType::Audio,
            ContentType::Link,
        ];
        let encoded: Vec<Vec<u8>> = variants
            .iter()
            .map(|v| encode_to_vec(v).expect("encode failed"))
            .collect();
        for i in 0..encoded.len() {
            for j in (i + 1)..encoded.len() {
                prop_assert_ne!(
                    &encoded[i],
                    &encoded[j],
                    "ContentType variants at indices {} and {} should differ",
                    i,
                    j
                );
            }
        }
    }
}

// Test 18: Post with is_public=true vs is_public=false encode differently
proptest! {
    #[test]
    fn prop_post_public_flag_encodes_differently(
        id: u64,
        user_id: u64,
        ct_idx in 0u8..5u8,
        likes: u32,
    ) {
        let post_public = Post {
            id,
            user_id,
            content_type: content_type_from_u8(ct_idx),
            text: "content".to_string(),
            likes,
            is_public: true,
        };
        let post_private = Post {
            id,
            user_id,
            content_type: content_type_from_u8(ct_idx),
            text: "content".to_string(),
            likes,
            is_public: false,
        };
        let encoded_public = encode_to_vec(&post_public).expect("encode failed");
        let encoded_private = encode_to_vec(&post_private).expect("encode failed");
        prop_assert_ne!(encoded_public, encoded_private);
    }
}

// Test 19: Reaction with different reaction_type values encode differently
proptest! {
    #[test]
    fn prop_reactions_different_type_encode_differently(
        post_id: u64,
        user_id: u64,
        rt_a: u8,
        rt_b: u8,
    ) {
        prop_assume!(rt_a != rt_b);
        let reaction_a = Reaction { post_id, user_id, reaction_type: rt_a };
        let reaction_b = Reaction { post_id, user_id, reaction_type: rt_b };
        let encoded_a = encode_to_vec(&reaction_a).expect("encode failed");
        let encoded_b = encode_to_vec(&reaction_b).expect("encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// Test 20: Post with u64::MAX id roundtrip
proptest! {
    #[test]
    fn prop_post_max_id_roundtrip(
        user_id: u64,
        ct_idx in 0u8..5u8,
        likes: u32,
        is_public: bool,
    ) {
        let post = Post {
            id: u64::MAX,
            user_id,
            content_type: content_type_from_u8(ct_idx),
            text: "max id post".to_string(),
            likes,
            is_public,
        };
        let encoded = encode_to_vec(&post).expect("encode failed");
        let (decoded, consumed): (Post, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(post, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 21: Double encode/decode identity for Post
proptest! {
    #[test]
    fn prop_post_double_encode_decode_identity(post in post_strategy()) {
        let encoded_once = encode_to_vec(&post).expect("encode failed");
        let (decoded_once, _): (Post, usize) =
            decode_from_slice(&encoded_once).expect("decode failed");
        let encoded_twice = encode_to_vec(&decoded_once).expect("encode failed");
        let (decoded_twice, consumed): (Post, usize) =
            decode_from_slice(&encoded_twice).expect("decode failed");
        prop_assert_eq!(post, decoded_twice);
        prop_assert_eq!(consumed, encoded_twice.len());
    }
}

// Test 22: Encoded bytes are non-empty for all four types
proptest! {
    #[test]
    fn prop_all_types_encoded_bytes_non_empty(
        post in post_strategy(),
        profile in user_profile_strategy(),
        reaction in reaction_strategy(),
        ct_idx in 0u8..5u8,
    ) {
        let post_bytes = encode_to_vec(&post).expect("encode failed");
        let profile_bytes = encode_to_vec(&profile).expect("encode failed");
        let reaction_bytes = encode_to_vec(&reaction).expect("encode failed");
        let ct_bytes = encode_to_vec(&content_type_from_u8(ct_idx)).expect("encode failed");
        prop_assert!(!post_bytes.is_empty(), "Post encoded bytes must be non-empty");
        prop_assert!(!profile_bytes.is_empty(), "UserProfile encoded bytes must be non-empty");
        prop_assert!(!reaction_bytes.is_empty(), "Reaction encoded bytes must be non-empty");
        prop_assert!(!ct_bytes.is_empty(), "ContentType encoded bytes must be non-empty");
    }
}
