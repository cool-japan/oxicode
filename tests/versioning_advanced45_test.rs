#![cfg(feature = "versioning")]
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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types: Podcast & Audio Streaming Platform ────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SubscriptionTier {
    Free,
    Basic,
    Premium,
    Creator,
    Enterprise,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AdPlacement {
    PreRoll,
    MidRoll,
    PostRoll,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ModerationFlag {
    Clean,
    Explicit,
    UnderReview,
    Blocked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DemographicSegment {
    GenZ,
    Millennial,
    GenX,
    Boomer,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RssNamespace {
    Itunes,
    GooglePlay,
    Spotify,
    PodcastIndex,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PodcastRssFeed {
    feed_id: u64,
    title: String,
    author: String,
    description: String,
    language: String,
    category: String,
    explicit: bool,
    episode_count: u32,
    last_build_timestamp: u64,
    image_url: String,
    namespaces: Vec<RssNamespace>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EpisodeTranscript {
    episode_id: u64,
    podcast_id: u64,
    language: String,
    segment_count: u32,
    word_count: u64,
    speaker_labels: Vec<String>,
    timestamps_ms: Vec<u64>,
    text_hash: String,
    auto_generated: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ListenerAnalytics {
    analytics_id: u64,
    episode_id: u64,
    total_plays: u64,
    unique_listeners: u64,
    completion_rate_bps: u16,
    average_listen_duration_sec: u32,
    skip_points_ms: Vec<u64>,
    peak_concurrent_listeners: u32,
    drop_off_rate_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdInsertionMarker {
    marker_id: u64,
    episode_id: u64,
    placement: AdPlacement,
    offset_ms: u64,
    duration_ms: u32,
    campaign_id: u64,
    advertiser: String,
    cpm_micros: u64,
    impressions: u64,
    clicks: u32,
    fill_rate_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlaylistCuration {
    playlist_id: u64,
    curator: String,
    title: String,
    description: String,
    episode_ids: Vec<u64>,
    follower_count: u64,
    is_editorial: bool,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RecommendationScore {
    user_id: u64,
    podcast_id: u64,
    relevance_score_x1e6: u64,
    collaborative_score_x1e6: u64,
    content_score_x1e6: u64,
    recency_boost_x1e6: u64,
    final_rank: u32,
    model_version: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContentModerationRecord {
    record_id: u64,
    episode_id: u64,
    flag: ModerationFlag,
    reason: String,
    reviewer_id: u64,
    reviewed_at: u64,
    auto_flagged: bool,
    confidence_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CreatorMonetization {
    creator_id: u64,
    podcast_id: u64,
    total_tips_micros: u128,
    subscriber_count: u64,
    tier: SubscriptionTier,
    monthly_revenue_micros: u128,
    payout_threshold_micros: u64,
    last_payout_timestamp: u64,
    lifetime_earnings_micros: u128,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioFingerprint {
    fingerprint_id: u64,
    episode_id: u64,
    hash_algorithm: String,
    hash_value: String,
    duration_ms: u64,
    sample_rate_hz: u32,
    bit_depth: u16,
    channel_count: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrossPromotionCampaign {
    campaign_id: u64,
    source_podcast_id: u64,
    target_podcast_id: u64,
    promo_clip_duration_ms: u32,
    impressions_target: u64,
    impressions_delivered: u64,
    conversion_count: u32,
    start_timestamp: u64,
    end_timestamp: u64,
    budget_micros: u128,
    spent_micros: u128,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DynamicAudioAd {
    ad_id: u64,
    advertiser: String,
    audio_url: String,
    duration_ms: u32,
    target_segments: Vec<DemographicSegment>,
    frequency_cap_per_user: u16,
    bid_cpm_micros: u64,
    creative_hash: String,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ListenerDemographic {
    segment_id: u64,
    segment: DemographicSegment,
    listener_count: u64,
    avg_session_duration_sec: u32,
    top_genre: String,
    device_mobile_pct_bps: u16,
    device_desktop_pct_bps: u16,
    device_smart_speaker_pct_bps: u16,
    region: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HostingBandwidthQuota {
    account_id: u64,
    tier: SubscriptionTier,
    monthly_bandwidth_bytes: u64,
    used_bandwidth_bytes: u64,
    storage_limit_bytes: u64,
    used_storage_bytes: u64,
    overage_rate_micros_per_gb: u64,
    billing_cycle_start: u64,
    billing_cycle_end: u64,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_podcast_rss_feed_roundtrip() {
    let feed = PodcastRssFeed {
        feed_id: 10001,
        title: "The Rustacean Station".to_string(),
        author: "Rust Community".to_string(),
        description: "Weekly podcast about Rust programming language".to_string(),
        language: "en-US".to_string(),
        category: "Technology".to_string(),
        explicit: false,
        episode_count: 312,
        last_build_timestamp: 1_710_500_000,
        image_url: "https://cdn.example.com/rustacean.png".to_string(),
        namespaces: vec![
            RssNamespace::Itunes,
            RssNamespace::Spotify,
            RssNamespace::PodcastIndex,
        ],
    };
    let bytes = encode_to_vec(&feed).expect("encode PodcastRssFeed failed");
    let (decoded, consumed) =
        decode_from_slice::<PodcastRssFeed>(&bytes).expect("decode PodcastRssFeed failed");
    assert_eq!(feed, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_podcast_rss_feed_versioned_v1_0_0() {
    let feed = PodcastRssFeed {
        feed_id: 10002,
        title: "Oxide and Friends".to_string(),
        author: "Oxide Computer".to_string(),
        description: "Hardware, software, and the future of computing".to_string(),
        language: "en".to_string(),
        category: "Technology".to_string(),
        explicit: false,
        episode_count: 87,
        last_build_timestamp: 1_712_000_000,
        image_url: "https://cdn.example.com/oxide.jpg".to_string(),
        namespaces: vec![
            RssNamespace::Itunes,
            RssNamespace::GooglePlay,
            RssNamespace::Custom("podcast:value".to_string()),
        ],
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&feed, version)
        .expect("encode versioned PodcastRssFeed v1.0.0 failed");
    let (decoded, ver, _consumed): (PodcastRssFeed, Version, usize) =
        decode_versioned_value::<PodcastRssFeed>(&bytes)
            .expect("decode versioned PodcastRssFeed v1.0.0 failed");
    assert_eq!(feed, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_episode_transcript_roundtrip() {
    let transcript = EpisodeTranscript {
        episode_id: 20001,
        podcast_id: 10001,
        language: "en-US".to_string(),
        segment_count: 1420,
        word_count: 8500,
        speaker_labels: vec![
            "Host".to_string(),
            "Guest A".to_string(),
            "Guest B".to_string(),
        ],
        timestamps_ms: vec![0, 15000, 32500, 61000, 120000],
        text_hash: "sha256:a1b2c3d4e5f6".to_string(),
        auto_generated: true,
    };
    let bytes = encode_to_vec(&transcript).expect("encode EpisodeTranscript failed");
    let (decoded, consumed) =
        decode_from_slice::<EpisodeTranscript>(&bytes).expect("decode EpisodeTranscript failed");
    assert_eq!(transcript, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_episode_transcript_versioned_v2_1_0() {
    let transcript = EpisodeTranscript {
        episode_id: 20002,
        podcast_id: 10002,
        language: "ja".to_string(),
        segment_count: 890,
        word_count: 4200,
        speaker_labels: vec!["Narrator".to_string()],
        timestamps_ms: vec![0, 5000, 10000, 20000],
        text_hash: "sha256:deadbeefcafe".to_string(),
        auto_generated: false,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&transcript, version)
        .expect("encode versioned EpisodeTranscript v2.1.0 failed");
    let (decoded, ver, consumed): (EpisodeTranscript, Version, usize) =
        decode_versioned_value::<EpisodeTranscript>(&bytes)
            .expect("decode versioned EpisodeTranscript v2.1.0 failed");
    assert_eq!(transcript, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_listener_analytics_high_completion_roundtrip() {
    let analytics = ListenerAnalytics {
        analytics_id: 30001,
        episode_id: 20001,
        total_plays: 145_000,
        unique_listeners: 98_000,
        completion_rate_bps: 8750,
        average_listen_duration_sec: 2640,
        skip_points_ms: vec![60_000, 180_000, 900_000],
        peak_concurrent_listeners: 3200,
        drop_off_rate_bps: 1250,
    };
    let bytes = encode_to_vec(&analytics).expect("encode ListenerAnalytics failed");
    let (decoded, consumed) =
        decode_from_slice::<ListenerAnalytics>(&bytes).expect("decode ListenerAnalytics failed");
    assert_eq!(analytics, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_listener_analytics_versioned_v1_5_0() {
    let analytics = ListenerAnalytics {
        analytics_id: 30002,
        episode_id: 20002,
        total_plays: 520,
        unique_listeners: 410,
        completion_rate_bps: 4200,
        average_listen_duration_sec: 780,
        skip_points_ms: vec![30_000, 120_000, 240_000, 600_000, 720_000],
        peak_concurrent_listeners: 45,
        drop_off_rate_bps: 5800,
    };
    let version = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&analytics, version)
        .expect("encode versioned ListenerAnalytics v1.5.0 failed");
    let (decoded, ver, _consumed): (ListenerAnalytics, Version, usize) =
        decode_versioned_value::<ListenerAnalytics>(&bytes)
            .expect("decode versioned ListenerAnalytics v1.5.0 failed");
    assert_eq!(analytics, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_ad_insertion_marker_preroll_roundtrip() {
    let marker = AdInsertionMarker {
        marker_id: 40001,
        episode_id: 20001,
        placement: AdPlacement::PreRoll,
        offset_ms: 0,
        duration_ms: 30_000,
        campaign_id: 50001,
        advertiser: "CloudProvider Inc".to_string(),
        cpm_micros: 25_000_000,
        impressions: 145_000,
        clicks: 2900,
        fill_rate_bps: 9500,
    };
    let bytes = encode_to_vec(&marker).expect("encode AdInsertionMarker PreRoll failed");
    let (decoded, consumed) = decode_from_slice::<AdInsertionMarker>(&bytes)
        .expect("decode AdInsertionMarker PreRoll failed");
    assert_eq!(marker, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_ad_insertion_marker_midroll_versioned_v3_0_0() {
    let marker = AdInsertionMarker {
        marker_id: 40002,
        episode_id: 20002,
        placement: AdPlacement::MidRoll,
        offset_ms: 900_000,
        duration_ms: 60_000,
        campaign_id: 50002,
        advertiser: "DevToolsCo".to_string(),
        cpm_micros: 35_000_000,
        impressions: 520,
        clicks: 15,
        fill_rate_bps: 8200,
    };
    let version = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&marker, version)
        .expect("encode versioned AdInsertionMarker MidRoll v3.0.0 failed");
    let (decoded, ver, consumed): (AdInsertionMarker, Version, usize) =
        decode_versioned_value::<AdInsertionMarker>(&bytes)
            .expect("decode versioned AdInsertionMarker MidRoll v3.0.0 failed");
    assert_eq!(marker, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_playlist_curation_editorial_roundtrip() {
    let playlist = PlaylistCuration {
        playlist_id: 60001,
        curator: "editorial_team".to_string(),
        title: "Best of Rust Podcasts 2025".to_string(),
        description: "Curated collection of top Rust programming episodes".to_string(),
        episode_ids: vec![20001, 20002, 20003, 20004, 20005],
        follower_count: 15_400,
        is_editorial: true,
        created_at: 1_700_000_000,
        updated_at: 1_712_500_000,
    };
    let bytes = encode_to_vec(&playlist).expect("encode PlaylistCuration editorial failed");
    let (decoded, consumed) = decode_from_slice::<PlaylistCuration>(&bytes)
        .expect("decode PlaylistCuration editorial failed");
    assert_eq!(playlist, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_playlist_curation_user_versioned_v1_2_3() {
    let playlist = PlaylistCuration {
        playlist_id: 60002,
        curator: "user_42".to_string(),
        title: "My Morning Commute Mix".to_string(),
        description: "Episodes for the daily drive".to_string(),
        episode_ids: vec![20010, 20020, 20030],
        follower_count: 7,
        is_editorial: false,
        created_at: 1_711_000_000,
        updated_at: 1_712_000_000,
    };
    let version = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&playlist, version)
        .expect("encode versioned PlaylistCuration user v1.2.3 failed");
    let (decoded, ver, consumed): (PlaylistCuration, Version, usize) =
        decode_versioned_value::<PlaylistCuration>(&bytes)
            .expect("decode versioned PlaylistCuration user v1.2.3 failed");
    assert_eq!(playlist, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_recommendation_score_high_relevance_roundtrip() {
    let score = RecommendationScore {
        user_id: 70001,
        podcast_id: 10001,
        relevance_score_x1e6: 950_000,
        collaborative_score_x1e6: 870_000,
        content_score_x1e6: 920_000,
        recency_boost_x1e6: 100_000,
        final_rank: 1,
        model_version: "rec-v3.7.1".to_string(),
    };
    let bytes = encode_to_vec(&score).expect("encode RecommendationScore failed");
    let (decoded, consumed) = decode_from_slice::<RecommendationScore>(&bytes)
        .expect("decode RecommendationScore failed");
    assert_eq!(score, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_recommendation_score_versioned_v2_0_0() {
    let score = RecommendationScore {
        user_id: 70002,
        podcast_id: 10002,
        relevance_score_x1e6: 450_000,
        collaborative_score_x1e6: 320_000,
        content_score_x1e6: 610_000,
        recency_boost_x1e6: 50_000,
        final_rank: 42,
        model_version: "rec-v4.0.0-beta".to_string(),
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&score, version)
        .expect("encode versioned RecommendationScore v2.0.0 failed");
    let (decoded, ver, _consumed): (RecommendationScore, Version, usize) =
        decode_versioned_value::<RecommendationScore>(&bytes)
            .expect("decode versioned RecommendationScore v2.0.0 failed");
    assert_eq!(score, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_content_moderation_explicit_roundtrip() {
    let record = ContentModerationRecord {
        record_id: 80001,
        episode_id: 20001,
        flag: ModerationFlag::Explicit,
        reason: "Contains strong language in interview segment".to_string(),
        reviewer_id: 99001,
        reviewed_at: 1_711_500_000,
        auto_flagged: true,
        confidence_bps: 9200,
    };
    let bytes = encode_to_vec(&record).expect("encode ContentModerationRecord Explicit failed");
    let (decoded, consumed) = decode_from_slice::<ContentModerationRecord>(&bytes)
        .expect("decode ContentModerationRecord Explicit failed");
    assert_eq!(record, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_content_moderation_blocked_versioned_v1_0_0() {
    let record = ContentModerationRecord {
        record_id: 80002,
        episode_id: 20099,
        flag: ModerationFlag::Blocked,
        reason: "Violates platform content policy section 4.2".to_string(),
        reviewer_id: 99002,
        reviewed_at: 1_712_100_000,
        auto_flagged: false,
        confidence_bps: 10000,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&record, version)
        .expect("encode versioned ContentModerationRecord Blocked v1.0.0 failed");
    let (decoded, ver, consumed): (ContentModerationRecord, Version, usize) =
        decode_versioned_value::<ContentModerationRecord>(&bytes)
            .expect("decode versioned ContentModerationRecord Blocked v1.0.0 failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_creator_monetization_premium_roundtrip() {
    let monetization = CreatorMonetization {
        creator_id: 90001,
        podcast_id: 10001,
        total_tips_micros: 125_000_000_000u128,
        subscriber_count: 4200,
        tier: SubscriptionTier::Premium,
        monthly_revenue_micros: 8_500_000_000u128,
        payout_threshold_micros: 50_000_000,
        last_payout_timestamp: 1_711_800_000,
        lifetime_earnings_micros: 450_000_000_000u128,
    };
    let bytes = encode_to_vec(&monetization).expect("encode CreatorMonetization Premium failed");
    let (decoded, consumed) = decode_from_slice::<CreatorMonetization>(&bytes)
        .expect("decode CreatorMonetization Premium failed");
    assert_eq!(monetization, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_creator_monetization_free_versioned_v1_0_0() {
    let monetization = CreatorMonetization {
        creator_id: 90002,
        podcast_id: 10002,
        total_tips_micros: 0u128,
        subscriber_count: 0,
        tier: SubscriptionTier::Free,
        monthly_revenue_micros: 0u128,
        payout_threshold_micros: 50_000_000,
        last_payout_timestamp: 0,
        lifetime_earnings_micros: 0u128,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&monetization, version)
        .expect("encode versioned CreatorMonetization Free v1.0.0 failed");
    let (decoded, ver, _consumed): (CreatorMonetization, Version, usize) =
        decode_versioned_value::<CreatorMonetization>(&bytes)
            .expect("decode versioned CreatorMonetization Free v1.0.0 failed");
    assert_eq!(monetization, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_audio_fingerprint_roundtrip() {
    let fingerprint = AudioFingerprint {
        fingerprint_id: 100001,
        episode_id: 20001,
        hash_algorithm: "chromaprint-v2".to_string(),
        hash_value: "AQADtNIyRYiYkBL-IP8hHceP48eRo8cRH0cehThy_MiP-EeSI-".to_string(),
        duration_ms: 3_600_000,
        sample_rate_hz: 44100,
        bit_depth: 16,
        channel_count: 2,
    };
    let bytes = encode_to_vec(&fingerprint).expect("encode AudioFingerprint failed");
    let (decoded, consumed) =
        decode_from_slice::<AudioFingerprint>(&bytes).expect("decode AudioFingerprint failed");
    assert_eq!(fingerprint, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_cross_promotion_campaign_versioned_v2_5_0() {
    let campaign = CrossPromotionCampaign {
        campaign_id: 110001,
        source_podcast_id: 10001,
        target_podcast_id: 10002,
        promo_clip_duration_ms: 45_000,
        impressions_target: 100_000,
        impressions_delivered: 67_500,
        conversion_count: 1350,
        start_timestamp: 1_710_000_000,
        end_timestamp: 1_712_500_000,
        budget_micros: 5_000_000_000u128,
        spent_micros: 3_375_000_000u128,
    };
    let version = Version::new(2, 5, 0);
    let bytes = encode_versioned_value(&campaign, version)
        .expect("encode versioned CrossPromotionCampaign v2.5.0 failed");
    let (decoded, ver, consumed): (CrossPromotionCampaign, Version, usize) =
        decode_versioned_value::<CrossPromotionCampaign>(&bytes)
            .expect("decode versioned CrossPromotionCampaign v2.5.0 failed");
    assert_eq!(campaign, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_dynamic_audio_ad_multi_segment_roundtrip() {
    let ad = DynamicAudioAd {
        ad_id: 120001,
        advertiser: "TechStartup XYZ".to_string(),
        audio_url: "https://ads.example.com/creatives/120001.opus".to_string(),
        duration_ms: 30_000,
        target_segments: vec![DemographicSegment::GenZ, DemographicSegment::Millennial],
        frequency_cap_per_user: 3,
        bid_cpm_micros: 18_000_000,
        creative_hash: "blake3:9f86d081884c7d659a2feaa0c55ad015".to_string(),
        active: true,
    };
    let bytes = encode_to_vec(&ad).expect("encode DynamicAudioAd failed");
    let (decoded, consumed) =
        decode_from_slice::<DynamicAudioAd>(&bytes).expect("decode DynamicAudioAd failed");
    assert_eq!(ad, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_listener_demographic_versioned_v1_0_0() {
    let demographic = ListenerDemographic {
        segment_id: 130001,
        segment: DemographicSegment::Millennial,
        listener_count: 2_500_000,
        avg_session_duration_sec: 1800,
        top_genre: "Technology".to_string(),
        device_mobile_pct_bps: 6500,
        device_desktop_pct_bps: 2000,
        device_smart_speaker_pct_bps: 1500,
        region: "North America".to_string(),
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&demographic, version)
        .expect("encode versioned ListenerDemographic v1.0.0 failed");
    let (decoded, ver, consumed): (ListenerDemographic, Version, usize) =
        decode_versioned_value::<ListenerDemographic>(&bytes)
            .expect("decode versioned ListenerDemographic v1.0.0 failed");
    assert_eq!(demographic, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_hosting_bandwidth_quota_enterprise_versioned_v3_1_0() {
    let quota = HostingBandwidthQuota {
        account_id: 140001,
        tier: SubscriptionTier::Enterprise,
        monthly_bandwidth_bytes: 10_000_000_000_000,
        used_bandwidth_bytes: 7_250_000_000_000,
        storage_limit_bytes: 5_000_000_000_000,
        used_storage_bytes: 3_100_000_000_000,
        overage_rate_micros_per_gb: 50_000,
        billing_cycle_start: 1_711_929_600,
        billing_cycle_end: 1_714_521_600,
    };
    let version = Version::new(3, 1, 0);
    let bytes = encode_versioned_value(&quota, version)
        .expect("encode versioned HostingBandwidthQuota Enterprise v3.1.0 failed");
    let (decoded, ver, _consumed): (HostingBandwidthQuota, Version, usize) =
        decode_versioned_value::<HostingBandwidthQuota>(&bytes)
            .expect("decode versioned HostingBandwidthQuota Enterprise v3.1.0 failed");
    assert_eq!(quota, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_ad_insertion_marker_postroll_version_upgrade_v1_to_v2() {
    let marker = AdInsertionMarker {
        marker_id: 40003,
        episode_id: 20001,
        placement: AdPlacement::PostRoll,
        offset_ms: 3_540_000,
        duration_ms: 15_000,
        campaign_id: 50003,
        advertiser: "PodHosting Pro".to_string(),
        cpm_micros: 12_000_000,
        impressions: 88_000,
        clicks: 440,
        fill_rate_bps: 7800,
    };
    let v1 = Version::new(1, 0, 0);
    let bytes_v1 = encode_versioned_value(&marker, v1)
        .expect("encode versioned AdInsertionMarker PostRoll v1.0.0 failed");
    let (decoded_v1, ver_v1, _consumed_v1): (AdInsertionMarker, Version, usize) =
        decode_versioned_value::<AdInsertionMarker>(&bytes_v1)
            .expect("decode versioned AdInsertionMarker PostRoll v1.0.0 failed");
    assert_eq!(marker, decoded_v1);
    assert_eq!(ver_v1.major, 1);

    let v2 = Version::new(2, 0, 0);
    let bytes_v2 = encode_versioned_value(&decoded_v1, v2)
        .expect("re-encode versioned AdInsertionMarker PostRoll v2.0.0 failed");
    let (decoded_v2, ver_v2, consumed_v2): (AdInsertionMarker, Version, usize) =
        decode_versioned_value::<AdInsertionMarker>(&bytes_v2)
            .expect("decode versioned AdInsertionMarker PostRoll v2.0.0 failed");
    assert_eq!(marker, decoded_v2);
    assert_eq!(ver_v2.major, 2);
    assert_eq!(ver_v2.minor, 0);
    assert!(consumed_v2 > 0);
}
