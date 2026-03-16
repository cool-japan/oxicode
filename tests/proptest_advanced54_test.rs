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
enum Platform {
    Twitter,
    Instagram,
    Facebook,
    Tiktok,
    Youtube,
    LinkedIn,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContentType {
    Text,
    Image,
    Video,
    Story,
    Reel,
    Live,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EngagementMetrics {
    likes: u64,
    shares: u64,
    comments: u64,
    views: u64,
    saves: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SocialPost {
    post_id: u64,
    platform: Platform,
    content_type: ContentType,
    metrics: EngagementMetrics,
    created_at: u64,
}

fn platform_from_index(idx: u8) -> Platform {
    match idx % 6 {
        0 => Platform::Twitter,
        1 => Platform::Instagram,
        2 => Platform::Facebook,
        3 => Platform::Tiktok,
        4 => Platform::Youtube,
        _ => Platform::LinkedIn,
    }
}

fn content_type_from_index(idx: u8) -> ContentType {
    match idx % 6 {
        0 => ContentType::Text,
        1 => ContentType::Image,
        2 => ContentType::Video,
        3 => ContentType::Story,
        4 => ContentType::Reel,
        _ => ContentType::Live,
    }
}

fn arb_platform() -> impl Strategy<Value = Platform> {
    (0u8..6u8).prop_map(platform_from_index)
}

fn arb_content_type() -> impl Strategy<Value = ContentType> {
    (0u8..6u8).prop_map(content_type_from_index)
}

fn arb_engagement_metrics() -> impl Strategy<Value = EngagementMetrics> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<u32>(),
    )
        .prop_map(
            |(likes, shares, comments, views, saves)| EngagementMetrics {
                likes,
                shares,
                comments,
                views,
                saves,
            },
        )
}

fn arb_social_post() -> impl Strategy<Value = SocialPost> {
    (
        any::<u64>(),
        arb_platform(),
        arb_content_type(),
        arb_engagement_metrics(),
        any::<u64>(),
    )
        .prop_map(
            |(post_id, platform, content_type, metrics, created_at)| SocialPost {
                post_id,
                platform,
                content_type,
                metrics,
                created_at,
            },
        )
}

proptest! {
    #[test]
    fn test_engagement_metrics_roundtrip(metrics in arb_engagement_metrics()) {
        let encoded = encode_to_vec(&metrics).expect("encode EngagementMetrics failed");
        let (decoded, _): (EngagementMetrics, usize) =
            decode_from_slice(&encoded).expect("decode EngagementMetrics failed");
        prop_assert_eq!(metrics, decoded);
    }

    #[test]
    fn test_social_post_roundtrip(post in arb_social_post()) {
        let encoded = encode_to_vec(&post).expect("encode SocialPost failed");
        let (decoded, _): (SocialPost, usize) =
            decode_from_slice(&encoded).expect("decode SocialPost failed");
        prop_assert_eq!(post, decoded);
    }

    #[test]
    fn test_engagement_metrics_consumed_bytes_equals_encoded_length(
        metrics in arb_engagement_metrics()
    ) {
        let encoded = encode_to_vec(&metrics).expect("encode EngagementMetrics failed");
        let (_, consumed): (EngagementMetrics, usize) =
            decode_from_slice(&encoded).expect("decode EngagementMetrics failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_social_post_consumed_bytes_equals_encoded_length(post in arb_social_post()) {
        let encoded = encode_to_vec(&post).expect("encode SocialPost failed");
        let (_, consumed): (SocialPost, usize) =
            decode_from_slice(&encoded).expect("decode SocialPost failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_engagement_metrics_encode_deterministic(metrics in arb_engagement_metrics()) {
        let encoded1 = encode_to_vec(&metrics).expect("first encode EngagementMetrics failed");
        let encoded2 = encode_to_vec(&metrics).expect("second encode EngagementMetrics failed");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_social_post_encode_deterministic(post in arb_social_post()) {
        let encoded1 = encode_to_vec(&post).expect("first encode SocialPost failed");
        let encoded2 = encode_to_vec(&post).expect("second encode SocialPost failed");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_vec_social_post_roundtrip(posts in proptest::collection::vec(arb_social_post(), 0..8)) {
        let encoded = encode_to_vec(&posts).expect("encode Vec<SocialPost> failed");
        let (decoded, _): (Vec<SocialPost>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<SocialPost> failed");
        prop_assert_eq!(posts, decoded);
    }

    #[test]
    fn test_option_social_post_roundtrip(opt in proptest::option::of(arb_social_post())) {
        let encoded = encode_to_vec(&opt).expect("encode Option<SocialPost> failed");
        let (decoded, _): (Option<SocialPost>, usize) =
            decode_from_slice(&encoded).expect("decode Option<SocialPost> failed");
        prop_assert_eq!(opt, decoded);
    }

    #[test]
    fn test_platform_variant_roundtrip(idx in 0u8..6u8) {
        let platform = platform_from_index(idx);
        let encoded = encode_to_vec(&platform).expect("encode Platform failed");
        let (decoded, _): (Platform, usize) =
            decode_from_slice(&encoded).expect("decode Platform failed");
        prop_assert_eq!(platform, decoded);
    }

    #[test]
    fn test_content_type_variant_roundtrip(idx in 0u8..6u8) {
        let content_type = content_type_from_index(idx);
        let encoded = encode_to_vec(&content_type).expect("encode ContentType failed");
        let (decoded, _): (ContentType, usize) =
            decode_from_slice(&encoded).expect("decode ContentType failed");
        prop_assert_eq!(content_type, decoded);
    }

    #[test]
    fn test_u8_basic_roundtrip(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("encode u8 failed");
        let (decoded, _): (u8, usize) = decode_from_slice(&encoded).expect("decode u8 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i32_basic_roundtrip(val in any::<i32>()) {
        let encoded = encode_to_vec(&val).expect("encode i32 failed");
        let (decoded, _): (i32, usize) = decode_from_slice(&encoded).expect("decode i32 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64_basic_roundtrip(val in any::<u64>()) {
        let encoded = encode_to_vec(&val).expect("encode u64 failed");
        let (decoded, _): (u64, usize) = decode_from_slice(&encoded).expect("decode u64 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i64_basic_roundtrip(val in any::<i64>()) {
        let encoded = encode_to_vec(&val).expect("encode i64 failed");
        let (decoded, _): (i64, usize) = decode_from_slice(&encoded).expect("decode i64 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool_basic_roundtrip(val in any::<bool>()) {
        let encoded = encode_to_vec(&val).expect("encode bool failed");
        let (decoded, _): (bool, usize) =
            decode_from_slice(&encoded).expect("decode bool failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_string_basic_roundtrip(val in ".*") {
        let encoded = encode_to_vec(&val).expect("encode String failed");
        let (decoded, _): (String, usize) =
            decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_f32_basic_roundtrip(val in proptest::num::f32::ANY) {
        let encoded = encode_to_vec(&val).expect("encode f32 failed");
        let (decoded, _): (f32, usize) = decode_from_slice(&encoded).expect("decode f32 failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_f64_basic_roundtrip(val in proptest::num::f64::ANY) {
        let encoded = encode_to_vec(&val).expect("encode f64 failed");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("decode f64 failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_vec_u8_roundtrip(val in proptest::collection::vec(any::<u8>(), 0..64)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<u8> failed");
        let (decoded, _): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<u8> failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_string_roundtrip(val in proptest::collection::vec(".*", 0..8)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<String> failed");
        let (decoded, _): (Vec<String>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<String> failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_option_u64_roundtrip(val in proptest::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&val).expect("encode Option<u64> failed");
        let (decoded, _): (Option<u64>, usize) =
            decode_from_slice(&encoded).expect("decode Option<u64> failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_distinct_social_posts_encode_distinctly_or_equally(
        post_a in arb_social_post(),
        post_b in arb_social_post(),
    ) {
        let encoded_a = encode_to_vec(&post_a).expect("encode post_a failed");
        let encoded_b = encode_to_vec(&post_b).expect("encode post_b failed");
        if post_a == post_b {
            prop_assert_eq!(&encoded_a, &encoded_b);
        } else {
            prop_assert_ne!(&encoded_a, &encoded_b);
        }
    }

    #[test]
    fn test_engagement_metrics_zero_values_roundtrip(
        _dummy in Just(())
    ) {
        let metrics = EngagementMetrics {
            likes: 0,
            shares: 0,
            comments: 0,
            views: 0,
            saves: 0,
        };
        let encoded = encode_to_vec(&metrics).expect("encode zero EngagementMetrics failed");
        let (decoded, consumed): (EngagementMetrics, usize) =
            decode_from_slice(&encoded).expect("decode zero EngagementMetrics failed");
        prop_assert_eq!(metrics, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_engagement_metrics_max_values_roundtrip(
        _dummy in Just(())
    ) {
        let metrics = EngagementMetrics {
            likes: u64::MAX,
            shares: u64::MAX,
            comments: u64::MAX,
            views: u64::MAX,
            saves: u32::MAX,
        };
        let encoded = encode_to_vec(&metrics).expect("encode max EngagementMetrics failed");
        let (decoded, consumed): (EngagementMetrics, usize) =
            decode_from_slice(&encoded).expect("decode max EngagementMetrics failed");
        prop_assert_eq!(metrics, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}
