//! Advanced Zstd compression tests for OxiCode — Social Media Analytics domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world social media analytics and content moderation data:
//! engagement metrics, influencer profiles, moderation decisions, sentiment
//! analysis, hashtag trends, ad campaigns, audience demographics, content
//! recommendations, user behavior sessions, A/B test cohorts, brand safety,
//! viral spread cascades, creator monetization, community health scores, and
//! misinformation detection flags.

#![cfg(feature = "compression-zstd")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ModerationDecision {
    Approve,
    Flag,
    Remove,
    Escalate,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContentType {
    Text,
    Image,
    Video,
    Story,
    Reel,
    LiveStream,
    Poll,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SentimentLabel {
    VeryNegative,
    Negative,
    Neutral,
    Positive,
    VeryPositive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AgeGroup {
    Under18,
    Age18To24,
    Age25To34,
    Age35To44,
    Age45To54,
    Age55Plus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BrandSafetyLevel {
    Safe,
    LowRisk,
    MediumRisk,
    HighRisk,
    Blocked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MisinfoCategory {
    None,
    HealthMisinfo,
    PoliticalMisinfo,
    FinancialScam,
    Deepfake,
    ManipulatedMedia,
    OutOfContext,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MonetizationTier {
    Unmonetized,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CohortGroup {
    Control,
    VariantA,
    VariantB,
    VariantC,
}

/// Engagement metrics for a single post.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EngagementMetrics {
    post_id: u64,
    likes: u32,
    shares: u32,
    comments: u32,
    saves: u32,
    impressions: u64,
    reach: u64,
    content_type: ContentType,
    timestamp_epoch_ms: u64,
}

/// Influencer profile analytics snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InfluencerProfile {
    user_id: u64,
    handle: String,
    follower_count: u64,
    following_count: u32,
    avg_engagement_rate_bps: u32,
    total_posts: u32,
    verified: bool,
    tier: MonetizationTier,
    top_hashtags: Vec<String>,
    audience_age_distribution: Vec<(AgeGroup, u16)>,
}

/// Content moderation decision record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModerationRecord {
    record_id: u64,
    post_id: u64,
    decision: ModerationDecision,
    confidence_pct: u8,
    flagged_categories: Vec<String>,
    reviewer_id: Option<u64>,
    timestamp_epoch_ms: u64,
}

/// Sentiment analysis result for a piece of content.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SentimentResult {
    content_id: u64,
    label: SentimentLabel,
    score_millionths: i32,
    keywords: Vec<String>,
    language_code: String,
}

/// Hashtag trending data point.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HashtagTrend {
    hashtag: String,
    hour_bucket: u32,
    post_count: u64,
    unique_authors: u32,
    avg_engagement: u32,
    velocity_per_minute: u32,
    region_codes: Vec<String>,
}

/// Ad campaign performance snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdCampaign {
    campaign_id: u64,
    name: String,
    impressions: u64,
    clicks: u64,
    conversions: u32,
    spend_cents: u64,
    ctr_bps: u32,
    cpm_cents: u32,
    target_age_groups: Vec<AgeGroup>,
    brand_safety: BrandSafetyLevel,
}

/// Audience demographics breakdown.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudienceDemographics {
    segment_id: u32,
    region: String,
    age_group: AgeGroup,
    user_count: u64,
    avg_daily_active_minutes: u16,
    device_mobile_pct: u8,
    device_desktop_pct: u8,
    device_tablet_pct: u8,
}

/// Content recommendation score.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RecommendationScore {
    user_id: u64,
    content_id: u64,
    relevance_score_millionths: u32,
    freshness_score_millionths: u32,
    diversity_score_millionths: u32,
    final_rank: u32,
    content_type: ContentType,
}

/// User behavior session.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UserSession {
    session_id: u64,
    user_id: u64,
    start_epoch_ms: u64,
    duration_ms: u32,
    pages_viewed: u16,
    posts_liked: u16,
    posts_shared: u16,
    comments_written: u16,
    search_queries: Vec<String>,
}

/// A/B test cohort result.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AbTestResult {
    experiment_id: u32,
    cohort: CohortGroup,
    sample_size: u32,
    conversion_rate_bps: u32,
    avg_session_duration_ms: u32,
    engagement_lift_bps: i32,
    statistical_significance_bps: u32,
}

/// Brand safety classification for content.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BrandSafetyReport {
    content_id: u64,
    level: BrandSafetyLevel,
    flagged_topics: Vec<String>,
    advertiser_suitability_pct: u8,
    human_reviewed: bool,
    override_reason: Option<String>,
}

/// Viral spread cascade node.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CascadeNode {
    node_user_id: u64,
    parent_user_id: Option<u64>,
    depth: u16,
    share_epoch_ms: u64,
    follower_count_at_share: u32,
    downstream_shares: u32,
}

/// Viral spread cascade graph.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ViralCascade {
    original_post_id: u64,
    total_reach: u64,
    max_depth: u16,
    nodes: Vec<CascadeNode>,
}

/// Creator monetization metrics.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CreatorMonetization {
    creator_id: u64,
    tier: MonetizationTier,
    revenue_cents: u64,
    ad_revenue_cents: u64,
    sponsorship_revenue_cents: u64,
    tip_revenue_cents: u64,
    subscriber_count: u32,
    payout_pending_cents: u64,
    eligible_for_bonus: bool,
}

/// Community health score snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommunityHealth {
    community_id: u64,
    member_count: u64,
    active_member_pct: u8,
    toxicity_score_bps: u32,
    helpfulness_score_bps: u32,
    reports_per_thousand: u32,
    mod_actions_last_day: u32,
    sentiment_distribution: Vec<(SentimentLabel, u16)>,
}

/// Misinformation detection flag.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MisinfoFlag {
    content_id: u64,
    category: MisinfoCategory,
    confidence_pct: u8,
    source_credibility_pct: u8,
    fact_check_url: Option<String>,
    times_reported: u32,
    auto_detected: bool,
    action_taken: ModerationDecision,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_engagement(seed: u64) -> EngagementMetrics {
    EngagementMetrics {
        post_id: seed * 1000 + 1,
        likes: (seed as u32).wrapping_mul(37) + 100,
        shares: (seed as u32).wrapping_mul(13) + 10,
        comments: (seed as u32).wrapping_mul(7) + 5,
        saves: (seed as u32).wrapping_mul(3) + 2,
        impressions: seed * 5000 + 10_000,
        reach: seed * 3000 + 5000,
        content_type: match seed % 7 {
            0 => ContentType::Text,
            1 => ContentType::Image,
            2 => ContentType::Video,
            3 => ContentType::Story,
            4 => ContentType::Reel,
            5 => ContentType::LiveStream,
            _ => ContentType::Poll,
        },
        timestamp_epoch_ms: 1_700_000_000_000 + seed * 3_600_000,
    }
}

fn make_influencer(seed: u64) -> InfluencerProfile {
    InfluencerProfile {
        user_id: seed * 100 + 42,
        handle: format!("@influencer_{seed}"),
        follower_count: seed * 50_000 + 1000,
        following_count: (seed as u32) * 100 + 200,
        avg_engagement_rate_bps: (seed as u32) * 50 + 200,
        total_posts: (seed as u32) * 30 + 100,
        verified: seed % 3 == 0,
        tier: match seed % 6 {
            0 => MonetizationTier::Unmonetized,
            1 => MonetizationTier::Bronze,
            2 => MonetizationTier::Silver,
            3 => MonetizationTier::Gold,
            4 => MonetizationTier::Platinum,
            _ => MonetizationTier::Diamond,
        },
        top_hashtags: (0..3).map(|i| format!("#topic_{}_{i}", seed)).collect(),
        audience_age_distribution: vec![
            (AgeGroup::Under18, 500),
            (AgeGroup::Age18To24, 3000),
            (AgeGroup::Age25To34, 3500),
            (AgeGroup::Age35To44, 2000),
            (AgeGroup::Age45To54, 700),
            (AgeGroup::Age55Plus, 300),
        ],
    }
}

fn make_moderation_record(seed: u64) -> ModerationRecord {
    ModerationRecord {
        record_id: seed * 7 + 1,
        post_id: seed * 1000 + 5,
        decision: match seed % 4 {
            0 => ModerationDecision::Approve,
            1 => ModerationDecision::Flag,
            2 => ModerationDecision::Remove,
            _ => ModerationDecision::Escalate,
        },
        confidence_pct: ((seed * 17) % 100) as u8,
        flagged_categories: vec![
            format!("category_{}", seed % 5),
            format!("policy_{}", seed % 3),
        ],
        reviewer_id: if seed % 2 == 0 {
            Some(seed * 10 + 99)
        } else {
            None
        },
        timestamp_epoch_ms: 1_700_000_000_000 + seed * 60_000,
    }
}

fn make_sentiment(seed: u64) -> SentimentResult {
    SentimentResult {
        content_id: seed * 11 + 3,
        label: match seed % 5 {
            0 => SentimentLabel::VeryNegative,
            1 => SentimentLabel::Negative,
            2 => SentimentLabel::Neutral,
            3 => SentimentLabel::Positive,
            _ => SentimentLabel::VeryPositive,
        },
        score_millionths: ((seed as i32) * 200_000) - 500_000,
        keywords: (0..4).map(|i| format!("keyword_{seed}_{i}")).collect(),
        language_code: if seed % 2 == 0 {
            "en".to_string()
        } else {
            "ja".to_string()
        },
    }
}

fn make_hashtag_trend(seed: u32) -> HashtagTrend {
    HashtagTrend {
        hashtag: format!("#trending_{seed}"),
        hour_bucket: seed * 3 + 1,
        post_count: (seed as u64) * 800 + 100,
        unique_authors: seed * 200 + 50,
        avg_engagement: seed * 40 + 20,
        velocity_per_minute: seed * 10 + 5,
        region_codes: vec!["US".to_string(), "JP".to_string(), "EU".to_string()],
    }
}

fn make_ad_campaign(seed: u64) -> AdCampaign {
    AdCampaign {
        campaign_id: seed * 999 + 1,
        name: format!("Campaign_Spring_{seed}"),
        impressions: seed * 1_000_000 + 500_000,
        clicks: seed * 5000 + 2000,
        conversions: (seed as u32) * 100 + 50,
        spend_cents: seed * 100_000 + 50_000,
        ctr_bps: (seed as u32) * 10 + 50,
        cpm_cents: (seed as u32) * 100 + 200,
        target_age_groups: vec![AgeGroup::Age18To24, AgeGroup::Age25To34],
        brand_safety: match seed % 5 {
            0 => BrandSafetyLevel::Safe,
            1 => BrandSafetyLevel::LowRisk,
            2 => BrandSafetyLevel::MediumRisk,
            3 => BrandSafetyLevel::HighRisk,
            _ => BrandSafetyLevel::Blocked,
        },
    }
}

fn make_recommendation(user_seed: u64, content_seed: u64) -> RecommendationScore {
    RecommendationScore {
        user_id: user_seed * 10 + 1,
        content_id: content_seed * 10 + 2,
        relevance_score_millionths: ((user_seed + content_seed) as u32) * 100_000 + 200_000,
        freshness_score_millionths: (content_seed as u32) * 50_000 + 300_000,
        diversity_score_millionths: (user_seed as u32) * 30_000 + 100_000,
        final_rank: (user_seed as u32) * 5 + (content_seed as u32) + 1,
        content_type: match content_seed % 4 {
            0 => ContentType::Image,
            1 => ContentType::Video,
            2 => ContentType::Reel,
            _ => ContentType::Text,
        },
    }
}

fn make_session(seed: u64) -> UserSession {
    UserSession {
        session_id: seed * 13 + 7,
        user_id: seed * 100 + 42,
        start_epoch_ms: 1_700_000_000_000 + seed * 86_400_000,
        duration_ms: (seed as u32) * 60_000 + 30_000,
        pages_viewed: (seed as u16) * 5 + 3,
        posts_liked: (seed as u16) * 2 + 1,
        posts_shared: (seed as u16) + 1,
        comments_written: (seed as u16) % 5,
        search_queries: (0..2).map(|i| format!("query_{seed}_{i}")).collect(),
    }
}

fn make_ab_test(seed: u32) -> AbTestResult {
    AbTestResult {
        experiment_id: seed * 3 + 1,
        cohort: match seed % 4 {
            0 => CohortGroup::Control,
            1 => CohortGroup::VariantA,
            2 => CohortGroup::VariantB,
            _ => CohortGroup::VariantC,
        },
        sample_size: seed * 1000 + 5000,
        conversion_rate_bps: seed * 50 + 300,
        avg_session_duration_ms: seed * 10_000 + 120_000,
        engagement_lift_bps: (seed as i32) * 25 - 50,
        statistical_significance_bps: seed * 100 + 9000,
    }
}

fn make_cascade(seed: u64, node_count: usize) -> ViralCascade {
    let nodes: Vec<CascadeNode> = (0..node_count)
        .map(|i| {
            let idx = i as u64;
            CascadeNode {
                node_user_id: seed * 1000 + idx * 10 + 1,
                parent_user_id: if i == 0 {
                    None
                } else {
                    Some(seed * 1000 + (idx - 1) * 10 + 1)
                },
                depth: i as u16,
                share_epoch_ms: 1_700_000_000_000 + idx * 300_000,
                follower_count_at_share: (idx as u32) * 500 + 100,
                downstream_shares: (node_count - i - 1) as u32,
            }
        })
        .collect();
    ViralCascade {
        original_post_id: seed * 9999 + 1,
        total_reach: (node_count as u64) * 2000 + 500,
        max_depth: (node_count - 1) as u16,
        nodes,
    }
}

fn make_creator_monetization(seed: u64) -> CreatorMonetization {
    CreatorMonetization {
        creator_id: seed * 77 + 1,
        tier: match seed % 6 {
            0 => MonetizationTier::Unmonetized,
            1 => MonetizationTier::Bronze,
            2 => MonetizationTier::Silver,
            3 => MonetizationTier::Gold,
            4 => MonetizationTier::Platinum,
            _ => MonetizationTier::Diamond,
        },
        revenue_cents: seed * 150_000 + 10_000,
        ad_revenue_cents: seed * 80_000 + 5_000,
        sponsorship_revenue_cents: seed * 50_000 + 3_000,
        tip_revenue_cents: seed * 20_000 + 2_000,
        subscriber_count: (seed as u32) * 500 + 100,
        payout_pending_cents: seed * 30_000 + 1_000,
        eligible_for_bonus: seed % 3 == 0,
    }
}

fn make_community_health(seed: u64) -> CommunityHealth {
    CommunityHealth {
        community_id: seed * 31 + 1,
        member_count: seed * 10_000 + 500,
        active_member_pct: ((seed * 7) % 100) as u8,
        toxicity_score_bps: (seed as u32) * 100 + 200,
        helpfulness_score_bps: (seed as u32) * 200 + 5000,
        reports_per_thousand: (seed as u32) * 5 + 10,
        mod_actions_last_day: (seed as u32) * 3 + 2,
        sentiment_distribution: vec![
            (SentimentLabel::VeryNegative, 200),
            (SentimentLabel::Negative, 1000),
            (SentimentLabel::Neutral, 4000),
            (SentimentLabel::Positive, 3500),
            (SentimentLabel::VeryPositive, 1300),
        ],
    }
}

fn make_misinfo_flag(seed: u64) -> MisinfoFlag {
    MisinfoFlag {
        content_id: seed * 41 + 7,
        category: match seed % 7 {
            0 => MisinfoCategory::None,
            1 => MisinfoCategory::HealthMisinfo,
            2 => MisinfoCategory::PoliticalMisinfo,
            3 => MisinfoCategory::FinancialScam,
            4 => MisinfoCategory::Deepfake,
            5 => MisinfoCategory::ManipulatedMedia,
            _ => MisinfoCategory::OutOfContext,
        },
        confidence_pct: ((seed * 13) % 100) as u8,
        source_credibility_pct: ((seed * 19) % 100) as u8,
        fact_check_url: if seed % 2 == 0 {
            Some(format!("https://factcheck.example.com/article/{seed}"))
        } else {
            None
        },
        times_reported: (seed as u32) * 7 + 1,
        auto_detected: seed % 3 != 0,
        action_taken: match seed % 4 {
            0 => ModerationDecision::Approve,
            1 => ModerationDecision::Flag,
            2 => ModerationDecision::Remove,
            _ => ModerationDecision::Escalate,
        },
    }
}

fn make_brand_safety(seed: u64) -> BrandSafetyReport {
    BrandSafetyReport {
        content_id: seed * 23 + 5,
        level: match seed % 5 {
            0 => BrandSafetyLevel::Safe,
            1 => BrandSafetyLevel::LowRisk,
            2 => BrandSafetyLevel::MediumRisk,
            3 => BrandSafetyLevel::HighRisk,
            _ => BrandSafetyLevel::Blocked,
        },
        flagged_topics: (0..2).map(|i| format!("topic_{seed}_{i}")).collect(),
        advertiser_suitability_pct: ((seed * 11) % 100) as u8,
        human_reviewed: seed % 2 == 0,
        override_reason: if seed % 4 == 0 {
            Some(format!("Manual override by admin #{seed}"))
        } else {
            None
        },
    }
}

fn make_demographics(seed: u32) -> AudienceDemographics {
    AudienceDemographics {
        segment_id: seed * 5 + 1,
        region: match seed % 4 {
            0 => "NA".to_string(),
            1 => "EU".to_string(),
            2 => "APAC".to_string(),
            _ => "LATAM".to_string(),
        },
        age_group: match seed % 6 {
            0 => AgeGroup::Under18,
            1 => AgeGroup::Age18To24,
            2 => AgeGroup::Age25To34,
            3 => AgeGroup::Age35To44,
            4 => AgeGroup::Age45To54,
            _ => AgeGroup::Age55Plus,
        },
        user_count: (seed as u64) * 100_000 + 50_000,
        avg_daily_active_minutes: seed as u16 * 10 + 30,
        device_mobile_pct: 60,
        device_desktop_pct: 30,
        device_tablet_pct: 10,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Basic round-trip for a single EngagementMetrics record.
#[test]
fn test_zstd_engagement_metrics_roundtrip() {
    let metrics = make_engagement(42);
    let encoded = encode_to_vec(&metrics).expect("encode EngagementMetrics failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (EngagementMetrics, usize) =
        decode_from_slice(&decompressed).expect("decode EngagementMetrics failed");
    assert_eq!(metrics, decoded);
}

/// 2. Round-trip for an influencer profile with nested collections.
#[test]
fn test_zstd_influencer_profile_roundtrip() {
    let profile = make_influencer(7);
    let encoded = encode_to_vec(&profile).expect("encode InfluencerProfile failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (InfluencerProfile, usize) =
        decode_from_slice(&decompressed).expect("decode InfluencerProfile failed");
    assert_eq!(profile, decoded);
}

/// 3. Round-trip for a batch of moderation records.
#[test]
fn test_zstd_moderation_records_roundtrip() {
    let records: Vec<ModerationRecord> = (0..50).map(make_moderation_record).collect();
    let encoded = encode_to_vec(&records).expect("encode Vec<ModerationRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ModerationRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<ModerationRecord> failed");
    assert_eq!(records, decoded);
}

/// 4. Round-trip for sentiment analysis results.
#[test]
fn test_zstd_sentiment_results_roundtrip() {
    let results: Vec<SentimentResult> = (0..40).map(make_sentiment).collect();
    let encoded = encode_to_vec(&results).expect("encode Vec<SentimentResult> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<SentimentResult>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<SentimentResult> failed");
    assert_eq!(results, decoded);
}

/// 5. Round-trip for hashtag trending data.
#[test]
fn test_zstd_hashtag_trends_roundtrip() {
    let trends: Vec<HashtagTrend> = (0..30).map(make_hashtag_trend).collect();
    let encoded = encode_to_vec(&trends).expect("encode Vec<HashtagTrend> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<HashtagTrend>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<HashtagTrend> failed");
    assert_eq!(trends, decoded);
}

/// 6. Round-trip for ad campaign performance records.
#[test]
fn test_zstd_ad_campaigns_roundtrip() {
    let campaigns: Vec<AdCampaign> = (0..20).map(make_ad_campaign).collect();
    let encoded = encode_to_vec(&campaigns).expect("encode Vec<AdCampaign> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AdCampaign>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AdCampaign> failed");
    assert_eq!(campaigns, decoded);
}

/// 7. Round-trip for audience demographics segments.
#[test]
fn test_zstd_audience_demographics_roundtrip() {
    let segments: Vec<AudienceDemographics> = (0..24).map(make_demographics).collect();
    let encoded = encode_to_vec(&segments).expect("encode Vec<AudienceDemographics> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AudienceDemographics>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AudienceDemographics> failed");
    assert_eq!(segments, decoded);
}

/// 8. Round-trip for content recommendation scores.
#[test]
fn test_zstd_recommendation_scores_roundtrip() {
    let recs: Vec<RecommendationScore> = (0..10)
        .flat_map(|u| (0..5).map(move |c| make_recommendation(u, c)))
        .collect();
    let encoded = encode_to_vec(&recs).expect("encode Vec<RecommendationScore> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<RecommendationScore>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<RecommendationScore> failed");
    assert_eq!(recs, decoded);
}

/// 9. Round-trip for user behavior sessions.
#[test]
fn test_zstd_user_sessions_roundtrip() {
    let sessions: Vec<UserSession> = (0..35).map(make_session).collect();
    let encoded = encode_to_vec(&sessions).expect("encode Vec<UserSession> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<UserSession>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<UserSession> failed");
    assert_eq!(sessions, decoded);
}

/// 10. Round-trip for A/B test cohort results.
#[test]
fn test_zstd_ab_test_results_roundtrip() {
    let results: Vec<AbTestResult> = (0..16).map(make_ab_test).collect();
    let encoded = encode_to_vec(&results).expect("encode Vec<AbTestResult> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AbTestResult>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AbTestResult> failed");
    assert_eq!(results, decoded);
}

/// 11. Round-trip for brand safety reports.
#[test]
fn test_zstd_brand_safety_roundtrip() {
    let reports: Vec<BrandSafetyReport> = (0..25).map(make_brand_safety).collect();
    let encoded = encode_to_vec(&reports).expect("encode Vec<BrandSafetyReport> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<BrandSafetyReport>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<BrandSafetyReport> failed");
    assert_eq!(reports, decoded);
}

/// 12. Round-trip for a viral cascade graph.
#[test]
fn test_zstd_viral_cascade_roundtrip() {
    let cascade = make_cascade(99, 50);
    let encoded = encode_to_vec(&cascade).expect("encode ViralCascade failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ViralCascade, usize) =
        decode_from_slice(&decompressed).expect("decode ViralCascade failed");
    assert_eq!(cascade, decoded);
}

/// 13. Round-trip for creator monetization metrics.
#[test]
fn test_zstd_creator_monetization_roundtrip() {
    let creators: Vec<CreatorMonetization> = (0..30).map(make_creator_monetization).collect();
    let encoded = encode_to_vec(&creators).expect("encode Vec<CreatorMonetization> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CreatorMonetization>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<CreatorMonetization> failed");
    assert_eq!(creators, decoded);
}

/// 14. Round-trip for community health scores.
#[test]
fn test_zstd_community_health_roundtrip() {
    let health: Vec<CommunityHealth> = (0..20).map(make_community_health).collect();
    let encoded = encode_to_vec(&health).expect("encode Vec<CommunityHealth> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CommunityHealth>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<CommunityHealth> failed");
    assert_eq!(health, decoded);
}

/// 15. Round-trip for misinformation detection flags.
#[test]
fn test_zstd_misinfo_flags_roundtrip() {
    let flags: Vec<MisinfoFlag> = (0..40).map(make_misinfo_flag).collect();
    let encoded = encode_to_vec(&flags).expect("encode Vec<MisinfoFlag> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<MisinfoFlag>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<MisinfoFlag> failed");
    assert_eq!(flags, decoded);
}

/// 16. Compressed engagement batch is smaller than raw encoded data.
#[test]
fn test_zstd_engagement_batch_compression_ratio() {
    let batch: Vec<EngagementMetrics> = (0..200).map(make_engagement).collect();
    let encoded = encode_to_vec(&batch).expect("encode engagement batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<EngagementMetrics>, usize) =
        decode_from_slice(&decompressed).expect("decode engagement batch failed");
    assert_eq!(batch, decoded);
}

/// 17. Compressed influencer profiles are smaller than raw.
#[test]
fn test_zstd_influencer_batch_compression_ratio() {
    let profiles: Vec<InfluencerProfile> = (0..100).map(make_influencer).collect();
    let encoded = encode_to_vec(&profiles).expect("encode influencer batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<InfluencerProfile>, usize) =
        decode_from_slice(&decompressed).expect("decode influencer batch failed");
    assert_eq!(profiles, decoded);
}

/// 18. Large viral cascade compresses well.
#[test]
fn test_zstd_large_cascade_compression_ratio() {
    let cascade = make_cascade(1, 500);
    let encoded = encode_to_vec(&cascade).expect("encode large cascade failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ViralCascade, usize) =
        decode_from_slice(&decompressed).expect("decode large cascade failed");
    assert_eq!(cascade, decoded);
}

/// 19. Combined moderation + misinfo pipeline round-trip.
#[test]
fn test_zstd_moderation_misinfo_combined_roundtrip() {
    let combined: Vec<(ModerationRecord, MisinfoFlag)> = (0..30)
        .map(|i| (make_moderation_record(i), make_misinfo_flag(i)))
        .collect();
    let encoded = encode_to_vec(&combined).expect("encode combined mod+misinfo failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<(ModerationRecord, MisinfoFlag)>, usize) =
        decode_from_slice(&decompressed).expect("decode combined mod+misinfo failed");
    assert_eq!(combined, decoded);
}

/// 20. Full analytics dashboard snapshot (mixed types) round-trip.
#[test]
#[allow(clippy::type_complexity)]
fn test_zstd_analytics_dashboard_snapshot_roundtrip() {
    let snapshot = (
        (0..10).map(make_engagement).collect::<Vec<_>>(),
        (0..5).map(make_influencer).collect::<Vec<_>>(),
        (0..8).map(make_hashtag_trend).collect::<Vec<_>>(),
        (0..3).map(make_ad_campaign).collect::<Vec<_>>(),
    );
    let encoded = encode_to_vec(&snapshot).expect("encode dashboard snapshot failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            Vec<EngagementMetrics>,
            Vec<InfluencerProfile>,
            Vec<HashtagTrend>,
            Vec<AdCampaign>,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode dashboard snapshot failed");
    assert_eq!(snapshot, decoded);
}

/// 21. Session analytics with recommendation scores round-trip.
#[test]
fn test_zstd_session_recommendation_pipeline_roundtrip() {
    let pipeline: Vec<(UserSession, Vec<RecommendationScore>)> = (0..15)
        .map(|u| {
            let session = make_session(u);
            let recs = (0..5).map(|c| make_recommendation(u, c)).collect();
            (session, recs)
        })
        .collect();
    let encoded = encode_to_vec(&pipeline).expect("encode session+recs pipeline failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<(UserSession, Vec<RecommendationScore>)>, usize) =
        decode_from_slice(&decompressed).expect("decode session+recs pipeline failed");
    assert_eq!(pipeline, decoded);
}

/// 22. Community health + A/B test combined batch with size comparison.
#[test]
fn test_zstd_community_ab_test_combined_compression() {
    let combined: Vec<(CommunityHealth, Vec<AbTestResult>)> = (0..10)
        .map(|i| {
            let health = make_community_health(i);
            let ab_results = (0..4).map(|j| make_ab_test((i as u32) * 4 + j)).collect();
            (health, ab_results)
        })
        .collect();
    let encoded = encode_to_vec(&combined).expect("encode community+ab combined failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<(CommunityHealth, Vec<AbTestResult>)>, usize) =
        decode_from_slice(&decompressed).expect("decode community+ab combined failed");
    assert_eq!(combined, decoded);
}
