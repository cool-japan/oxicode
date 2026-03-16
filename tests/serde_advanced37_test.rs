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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// ── Domain types: e-commerce analytics / recommendation engine ────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    SmartTv,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum UserSegment {
    NewVisitor,
    Returning,
    Loyal { purchase_count: u32 },
    HighValue { lifetime_value_cents: u64 },
    AtRisk,
    Churned,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ConversionEvent {
    PageView { page_path: String },
    ProductView { product_id: u64 },
    AddToCart { product_id: u64, quantity: u32 },
    RemoveFromCart { product_id: u64 },
    CheckoutStarted,
    CheckoutCompleted { order_id: u64, total_cents: u64 },
    SearchQuery { query: String },
    ReviewSubmitted { product_id: u64, rating: u8 },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AbTestVariant {
    Control,
    TreatmentA,
    TreatmentB,
    TreatmentC,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProductRecommendation {
    product_id: u64,
    score: u32,
    rank: u32,
    category: String,
    price_cents: u64,
    in_stock: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CartItem {
    product_id: u64,
    quantity: u32,
    unit_price_cents: u64,
    discount_cents: Option<u64>,
    added_at_epoch_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AbandonedCart {
    session_id: String,
    user_id: Option<u64>,
    items: Vec<CartItem>,
    total_cents: u64,
    abandoned_at_epoch_ms: u64,
    recovery_email_sent: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ClickThroughRecord {
    impression_id: u64,
    product_id: u64,
    user_id: Option<u64>,
    position: u32,
    clicked: bool,
    dwell_time_ms: Option<u32>,
    converted: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UserSession {
    session_id: String,
    user_id: Option<u64>,
    device: DeviceType,
    segment: UserSegment,
    started_at_epoch_ms: u64,
    ended_at_epoch_ms: Option<u64>,
    page_views: u32,
    events: Vec<ConversionEvent>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AbTestAssignment {
    experiment_id: String,
    user_id: u64,
    variant: AbTestVariant,
    assigned_at_epoch_ms: u64,
    converted: Option<bool>,
    conversion_value_cents: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RecommendationBatch {
    user_id: u64,
    algorithm: String,
    generated_at_epoch_ms: u64,
    recommendations: Vec<ProductRecommendation>,
    context_product_id: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PurchaseFunnelSnapshot {
    snapshot_id: String,
    period_start_epoch_ms: u64,
    period_end_epoch_ms: u64,
    visitors: u64,
    product_viewers: u64,
    cart_adders: u64,
    checkout_starters: u64,
    purchasers: u64,
    revenue_cents: u64,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_cart_item(product_id: u64, qty: u32, price_cents: u64, discount: Option<u64>) -> CartItem {
    CartItem {
        product_id,
        quantity: qty,
        unit_price_cents: price_cents,
        discount_cents: discount,
        added_at_epoch_ms: 1_741_000_000_000 + product_id,
    }
}

fn make_recommendation(product_id: u64, rank: u32, score: u32) -> ProductRecommendation {
    ProductRecommendation {
        product_id,
        score,
        rank,
        category: format!("category_{}", rank),
        price_cents: 999 + product_id * 100,
        in_stock: rank % 3 != 0,
    }
}

fn make_session(session_id: &str, user_id: Option<u64>, device: DeviceType) -> UserSession {
    UserSession {
        session_id: session_id.to_string(),
        user_id,
        device,
        segment: UserSegment::Returning,
        started_at_epoch_ms: 1_741_100_000_000,
        ended_at_epoch_ms: Some(1_741_100_060_000),
        page_views: 5,
        events: vec![],
    }
}

// ── 1. DeviceType: all variants roundtrip ─────────────────────────────────────
#[test]
fn test_device_type_all_variants() {
    let cfg = config::standard();
    let variants = [
        DeviceType::Desktop,
        DeviceType::Mobile,
        DeviceType::Tablet,
        DeviceType::SmartTv,
        DeviceType::Unknown,
    ];
    for variant in &variants {
        let bytes = encode_to_vec(variant, cfg).expect("encode DeviceType variant");
        let (decoded, _): (DeviceType, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode DeviceType variant");
        assert_eq!(variant, &decoded);
    }
}

// ── 2. UserSegment: all variants including data-carrying ones ─────────────────
#[test]
fn test_user_segment_all_variants() {
    let cfg = config::standard();
    let variants = vec![
        UserSegment::NewVisitor,
        UserSegment::Returning,
        UserSegment::Loyal { purchase_count: 42 },
        UserSegment::HighValue {
            lifetime_value_cents: 1_250_000,
        },
        UserSegment::AtRisk,
        UserSegment::Churned,
    ];
    for variant in &variants {
        let bytes = encode_to_vec(variant, cfg).expect("encode UserSegment variant");
        let (decoded, _): (UserSegment, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode UserSegment variant");
        assert_eq!(variant, &decoded);
    }
}

// ── 3. ConversionEvent: all variants roundtrip ───────────────────────────────
#[test]
fn test_conversion_event_all_variants() {
    let cfg = config::standard();
    let events = vec![
        ConversionEvent::PageView {
            page_path: "/home".to_string(),
        },
        ConversionEvent::ProductView { product_id: 1001 },
        ConversionEvent::AddToCart {
            product_id: 2002,
            quantity: 3,
        },
        ConversionEvent::RemoveFromCart { product_id: 3003 },
        ConversionEvent::CheckoutStarted,
        ConversionEvent::CheckoutCompleted {
            order_id: 500_001,
            total_cents: 12_499,
        },
        ConversionEvent::SearchQuery {
            query: "wireless headphones".to_string(),
        },
        ConversionEvent::ReviewSubmitted {
            product_id: 4004,
            rating: 5,
        },
    ];
    for event in &events {
        let bytes = encode_to_vec(event, cfg).expect("encode ConversionEvent variant");
        let (decoded, _): (ConversionEvent, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode ConversionEvent variant");
        assert_eq!(event, &decoded);
    }
}

// ── 4. AbTestVariant: all variants roundtrip ─────────────────────────────────
#[test]
fn test_ab_test_variant_all_variants() {
    let cfg = config::standard();
    let variants = [
        AbTestVariant::Control,
        AbTestVariant::TreatmentA,
        AbTestVariant::TreatmentB,
        AbTestVariant::TreatmentC,
    ];
    for variant in &variants {
        let bytes = encode_to_vec(variant, cfg).expect("encode AbTestVariant");
        let (decoded, _): (AbTestVariant, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode AbTestVariant");
        assert_eq!(variant, &decoded);
    }
}

// ── 5. ProductRecommendation basic roundtrip ──────────────────────────────────
#[test]
fn test_product_recommendation_basic_roundtrip() {
    let cfg = config::standard();
    let rec = make_recommendation(10_001, 1, 9850);
    let bytes = encode_to_vec(&rec, cfg).expect("encode ProductRecommendation");
    let (decoded, _): (ProductRecommendation, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ProductRecommendation");
    assert_eq!(rec, decoded);
    assert_eq!(decoded.rank, 1);
    assert_eq!(decoded.product_id, 10_001);
}

// ── 6. CartItem with discount = Some ─────────────────────────────────────────
#[test]
fn test_cart_item_with_discount() {
    let cfg = config::standard();
    let item = make_cart_item(20_002, 2, 3_499, Some(500));
    let bytes = encode_to_vec(&item, cfg).expect("encode CartItem with discount");
    let (decoded, _): (CartItem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CartItem with discount");
    assert_eq!(item, decoded);
    assert_eq!(decoded.discount_cents, Some(500));
}

// ── 7. CartItem with discount = None ─────────────────────────────────────────
#[test]
fn test_cart_item_no_discount() {
    let cfg = config::standard();
    let item = make_cart_item(30_003, 1, 7_999, None);
    let bytes = encode_to_vec(&item, cfg).expect("encode CartItem no discount");
    let (decoded, _): (CartItem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CartItem no discount");
    assert_eq!(item, decoded);
    assert!(decoded.discount_cents.is_none());
}

// ── 8. AbandonedCart with anonymous user (user_id = None) ────────────────────
#[test]
fn test_abandoned_cart_anonymous_user() {
    let cfg = config::standard();
    let cart = AbandonedCart {
        session_id: "anon_sess_abc123".to_string(),
        user_id: None,
        items: vec![
            make_cart_item(50_001, 1, 12_900, None),
            make_cart_item(50_002, 3, 4_500, Some(300)),
        ],
        total_cents: 25_200,
        abandoned_at_epoch_ms: 1_741_200_000_000,
        recovery_email_sent: false,
    };
    let bytes = encode_to_vec(&cart, cfg).expect("encode AbandonedCart anonymous");
    let (decoded, _): (AbandonedCart, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AbandonedCart anonymous");
    assert_eq!(cart, decoded);
    assert!(decoded.user_id.is_none());
    assert_eq!(decoded.items.len(), 2);
}

// ── 9. AbandonedCart with identified user and recovery email ──────────────────
#[test]
fn test_abandoned_cart_identified_user_recovery_sent() {
    let cfg = config::standard();
    let cart = AbandonedCart {
        session_id: "auth_sess_xyz789".to_string(),
        user_id: Some(9_000_001),
        items: vec![
            make_cart_item(60_001, 2, 8_990, Some(899)),
            make_cart_item(60_002, 1, 24_999, None),
            make_cart_item(60_003, 4, 1_299, None),
        ],
        total_cents: 42_179,
        abandoned_at_epoch_ms: 1_741_300_000_000,
        recovery_email_sent: true,
    };
    let bytes = encode_to_vec(&cart, cfg).expect("encode AbandonedCart identified");
    let (decoded, _): (AbandonedCart, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AbandonedCart identified");
    assert_eq!(cart, decoded);
    assert_eq!(decoded.user_id, Some(9_000_001));
    assert!(decoded.recovery_email_sent);
    assert_eq!(decoded.items.len(), 3);
}

// ── 10. Vec<ProductRecommendation> roundtrip ─────────────────────────────────
#[test]
fn test_vec_product_recommendations_roundtrip() {
    let cfg = config::standard();
    let recs: Vec<ProductRecommendation> = (1u64..=10)
        .map(|i| make_recommendation(70_000 + i, i as u32, 10_000 - (i as u32 * 200)))
        .collect();
    let bytes = encode_to_vec(&recs, cfg).expect("encode Vec<ProductRecommendation>");
    let (decoded, _): (Vec<ProductRecommendation>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ProductRecommendation>");
    assert_eq!(recs, decoded);
    assert_eq!(decoded.len(), 10);
    assert_eq!(decoded[0].rank, 1);
}

// ── 11. UserSession with full event stream ────────────────────────────────────
#[test]
fn test_user_session_full_event_stream() {
    let cfg = config::standard();
    let mut session = make_session("sess_event_stream_001", Some(7_654_321), DeviceType::Mobile);
    session.segment = UserSegment::Loyal { purchase_count: 17 };
    session.page_views = 12;
    session.events = vec![
        ConversionEvent::PageView {
            page_path: "/".to_string(),
        },
        ConversionEvent::SearchQuery {
            query: "running shoes".to_string(),
        },
        ConversionEvent::ProductView { product_id: 80_001 },
        ConversionEvent::ProductView { product_id: 80_002 },
        ConversionEvent::AddToCart {
            product_id: 80_001,
            quantity: 1,
        },
        ConversionEvent::CheckoutStarted,
        ConversionEvent::CheckoutCompleted {
            order_id: 600_001,
            total_cents: 9_990,
        },
    ];
    let bytes = encode_to_vec(&session, cfg).expect("encode UserSession full events");
    let (decoded, _): (UserSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode UserSession full events");
    assert_eq!(session, decoded);
    assert_eq!(decoded.events.len(), 7);
}

// ── 12. UserSession anonymous visitor with tablet device ──────────────────────
#[test]
fn test_user_session_anonymous_tablet() {
    let cfg = config::standard();
    let mut session = make_session("anon_tablet_sess_042", None, DeviceType::Tablet);
    session.segment = UserSegment::NewVisitor;
    session.ended_at_epoch_ms = None;
    session.events = vec![
        ConversionEvent::PageView {
            page_path: "/sale".to_string(),
        },
        ConversionEvent::ProductView { product_id: 90_005 },
    ];
    let bytes = encode_to_vec(&session, cfg).expect("encode anonymous tablet session");
    let (decoded, _): (UserSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode anonymous tablet session");
    assert_eq!(session, decoded);
    assert!(decoded.user_id.is_none());
    assert!(decoded.ended_at_epoch_ms.is_none());
    assert_eq!(decoded.device, DeviceType::Tablet);
}

// ── 13. RecommendationBatch with context product ──────────────────────────────
#[test]
fn test_recommendation_batch_with_context() {
    let cfg = config::standard();
    let batch = RecommendationBatch {
        user_id: 1_234_567,
        algorithm: "collaborative_filtering_v3".to_string(),
        generated_at_epoch_ms: 1_741_400_000_000,
        recommendations: vec![
            make_recommendation(100_001, 1, 9500),
            make_recommendation(100_002, 2, 9200),
            make_recommendation(100_003, 3, 8900),
            make_recommendation(100_004, 4, 8600),
            make_recommendation(100_005, 5, 8300),
        ],
        context_product_id: Some(95_000),
    };
    let bytes = encode_to_vec(&batch, cfg).expect("encode RecommendationBatch with context");
    let (decoded, _): (RecommendationBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RecommendationBatch with context");
    assert_eq!(batch, decoded);
    assert_eq!(decoded.recommendations.len(), 5);
    assert_eq!(decoded.context_product_id, Some(95_000));
}

// ── 14. RecommendationBatch without context (homepage recommendations) ─────────
#[test]
fn test_recommendation_batch_no_context() {
    let cfg = config::standard();
    let batch = RecommendationBatch {
        user_id: 2_345_678,
        algorithm: "trending_v2".to_string(),
        generated_at_epoch_ms: 1_741_500_000_000,
        recommendations: vec![
            make_recommendation(110_001, 1, 9800),
            make_recommendation(110_002, 2, 9700),
        ],
        context_product_id: None,
    };
    let bytes = encode_to_vec(&batch, cfg).expect("encode RecommendationBatch no context");
    let (decoded, _): (RecommendationBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RecommendationBatch no context");
    assert_eq!(batch, decoded);
    assert!(decoded.context_product_id.is_none());
}

// ── 15. AbTestAssignment with conversion ─────────────────────────────────────
#[test]
fn test_ab_test_assignment_with_conversion() {
    let cfg = config::standard();
    let assignment = AbTestAssignment {
        experiment_id: "checkout_cta_v4".to_string(),
        user_id: 3_456_789,
        variant: AbTestVariant::TreatmentB,
        assigned_at_epoch_ms: 1_741_600_000_000,
        converted: Some(true),
        conversion_value_cents: Some(5_499),
    };
    let bytes = encode_to_vec(&assignment, cfg).expect("encode AbTestAssignment converted");
    let (decoded, _): (AbTestAssignment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AbTestAssignment converted");
    assert_eq!(assignment, decoded);
    assert_eq!(decoded.converted, Some(true));
    assert_eq!(decoded.conversion_value_cents, Some(5_499));
}

// ── 16. AbTestAssignment pending (no conversion outcome yet) ──────────────────
#[test]
fn test_ab_test_assignment_pending() {
    let cfg = config::standard();
    let assignment = AbTestAssignment {
        experiment_id: "recommendation_algo_v7".to_string(),
        user_id: 4_567_890,
        variant: AbTestVariant::Control,
        assigned_at_epoch_ms: 1_741_700_000_000,
        converted: None,
        conversion_value_cents: None,
    };
    let bytes = encode_to_vec(&assignment, cfg).expect("encode AbTestAssignment pending");
    let (decoded, _): (AbTestAssignment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AbTestAssignment pending");
    assert_eq!(assignment, decoded);
    assert!(decoded.converted.is_none());
    assert!(decoded.conversion_value_cents.is_none());
}

// ── 17. PurchaseFunnelSnapshot basic roundtrip ────────────────────────────────
#[test]
fn test_purchase_funnel_snapshot_roundtrip() {
    let cfg = config::standard();
    let funnel = PurchaseFunnelSnapshot {
        snapshot_id: "funnel_2026_03_15_daily".to_string(),
        period_start_epoch_ms: 1_741_824_000_000,
        period_end_epoch_ms: 1_741_910_400_000,
        visitors: 152_340,
        product_viewers: 89_221,
        cart_adders: 23_450,
        checkout_starters: 11_220,
        purchasers: 8_910,
        revenue_cents: 1_285_340_00,
    };
    let bytes = encode_to_vec(&funnel, cfg).expect("encode PurchaseFunnelSnapshot");
    let (decoded, _): (PurchaseFunnelSnapshot, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PurchaseFunnelSnapshot");
    assert_eq!(funnel, decoded);
    assert_eq!(decoded.purchasers, 8_910);
    assert!(decoded.purchasers < decoded.cart_adders);
}

// ── 18. Big-endian config: AbandonedCart with items ───────────────────────────
#[test]
fn test_abandoned_cart_big_endian() {
    let cfg = config::standard().with_big_endian();
    let cart = AbandonedCart {
        session_id: "be_sess_bigendian_001".to_string(),
        user_id: Some(5_678_901),
        items: vec![
            make_cart_item(200_001, 1, 14_990, Some(1_000)),
            make_cart_item(200_002, 2, 3_990, None),
            make_cart_item(200_003, 1, 55_000, Some(5_500)),
        ],
        total_cents: 72_470,
        abandoned_at_epoch_ms: 1_741_800_000_000,
        recovery_email_sent: false,
    };
    let bytes = encode_to_vec(&cart, cfg).expect("encode AbandonedCart big_endian");
    let (decoded, _): (AbandonedCart, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AbandonedCart big_endian");
    assert_eq!(cart, decoded);
    assert_eq!(decoded.items.len(), 3);
    assert_eq!(decoded.items[0].discount_cents, Some(1_000));
}

// ── 19. Fixed-int config: RecommendationBatch ────────────────────────────────
#[test]
fn test_recommendation_batch_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let batch = RecommendationBatch {
        user_id: 6_789_012,
        algorithm: "matrix_factorization_v2".to_string(),
        generated_at_epoch_ms: 1_741_900_000_000,
        recommendations: vec![
            make_recommendation(300_001, 1, 9900),
            make_recommendation(300_002, 2, 9750),
            make_recommendation(300_003, 3, 9600),
            make_recommendation(300_004, 4, 9450),
            make_recommendation(300_005, 5, 9300),
            make_recommendation(300_006, 6, 9150),
        ],
        context_product_id: Some(280_000),
    };
    let bytes = encode_to_vec(&batch, cfg).expect("encode RecommendationBatch fixed_int");
    let (decoded, _): (RecommendationBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RecommendationBatch fixed_int");
    assert_eq!(batch, decoded);
    assert_eq!(decoded.recommendations.len(), 6);
    assert_eq!(decoded.algorithm, "matrix_factorization_v2");
}

// ── 20. Large collection: Vec<ClickThroughRecord> (1000 items) ────────────────
#[test]
fn test_large_click_through_collection() {
    let cfg = config::standard();
    let records: Vec<ClickThroughRecord> = (0u64..1000)
        .map(|i| ClickThroughRecord {
            impression_id: 10_000_000 + i,
            product_id: 400_000 + (i % 200),
            user_id: if i % 5 == 0 {
                None
            } else {
                Some(7_000_000 + i)
            },
            position: (i % 20) as u32 + 1,
            clicked: i % 4 != 3,
            dwell_time_ms: if i % 4 != 3 {
                Some((i as u32 % 30_000) + 500)
            } else {
                None
            },
            converted: i % 10 == 0,
        })
        .collect();
    let bytes = encode_to_vec(&records, cfg).expect("encode Vec<ClickThroughRecord> 1000");
    let (decoded, _): (Vec<ClickThroughRecord>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ClickThroughRecord> 1000");
    assert_eq!(records, decoded);
    assert_eq!(decoded.len(), 1000);
    assert!(decoded[0].user_id.is_none());
    assert_eq!(decoded[1].user_id, Some(7_000_001));
}

// ── 21. Consumed bytes: UserSession complex ───────────────────────────────────
#[test]
fn test_consumed_bytes_user_session_complex() {
    let cfg = config::standard();
    let mut session = make_session(
        "consumed_check_sess_007",
        Some(8_901_234),
        DeviceType::Desktop,
    );
    session.segment = UserSegment::HighValue {
        lifetime_value_cents: 3_750_000,
    };
    session.page_views = 25;
    session.events = vec![
        ConversionEvent::PageView {
            page_path: "/".to_string(),
        },
        ConversionEvent::SearchQuery {
            query: "laptop".to_string(),
        },
        ConversionEvent::ProductView {
            product_id: 500_001,
        },
        ConversionEvent::ProductView {
            product_id: 500_002,
        },
        ConversionEvent::ProductView {
            product_id: 500_003,
        },
        ConversionEvent::AddToCart {
            product_id: 500_001,
            quantity: 1,
        },
        ConversionEvent::AddToCart {
            product_id: 500_003,
            quantity: 2,
        },
        ConversionEvent::RemoveFromCart {
            product_id: 500_003,
        },
        ConversionEvent::CheckoutStarted,
        ConversionEvent::ReviewSubmitted {
            product_id: 500_001,
            rating: 4,
        },
        ConversionEvent::CheckoutCompleted {
            order_id: 700_001,
            total_cents: 119_900,
        },
    ];
    let bytes = encode_to_vec(&session, cfg).expect("encode UserSession for consumed bytes check");
    let (decoded, consumed): (UserSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode UserSession for consumed bytes check");
    assert_eq!(session, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
    assert_eq!(decoded.events.len(), 11);
}

// ── 22. Combined config (big-endian + fixed-int): PurchaseFunnelSnapshot ──────
#[test]
fn test_purchase_funnel_combined_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let funnel = PurchaseFunnelSnapshot {
        snapshot_id: "combined_cfg_funnel_weekly".to_string(),
        period_start_epoch_ms: 1_741_219_200_000,
        period_end_epoch_ms: 1_741_824_000_000,
        visitors: 1_072_440,
        product_viewers: 623_580,
        cart_adders: 164_150,
        checkout_starters: 78_540,
        purchasers: 62_370,
        revenue_cents: 89_937_415_00,
    };
    let bytes = encode_to_vec(&funnel, cfg).expect("encode PurchaseFunnelSnapshot combined config");
    let (decoded, consumed): (PurchaseFunnelSnapshot, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode PurchaseFunnelSnapshot combined config");
    assert_eq!(funnel, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must match with combined config"
    );
    assert_eq!(decoded.snapshot_id, "combined_cfg_funnel_weekly");
    assert!(decoded.purchasers < decoded.cart_adders);
}
