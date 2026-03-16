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

// --- Auction House & Collectibles Marketplace Domain Types ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LotCatalogEntry {
    lot_number: u32,
    title: String,
    artist_or_maker: String,
    medium: String,
    dimensions_cm: (f64, f64, f64),
    year_created: Option<u16>,
    estimate_low_cents: u64,
    estimate_high_cents: u64,
    category: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BidderRegistration {
    bidder_id: u64,
    paddle_number: u32,
    full_name: String,
    email: String,
    phone: String,
    deposit_cents: u64,
    credit_approved: bool,
    registration_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BidIncrementRule {
    range_floor_cents: u64,
    range_ceiling_cents: u64,
    increment_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BidIncrementTable {
    table_id: u32,
    auction_house: String,
    rules: Vec<BidIncrementRule>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HammerPriceRecord {
    lot_number: u32,
    hammer_price_cents: u64,
    winning_paddle: u32,
    auctioneer_id: u16,
    sale_timestamp: u64,
    currency_code: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BuyersPremiumTier {
    threshold_cents: u64,
    rate_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BuyersPremiumSchedule {
    schedule_id: u32,
    effective_date: String,
    tiers: Vec<BuyersPremiumTier>,
    online_surcharge_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ConsignmentAgreement {
    agreement_id: u64,
    consignor_name: String,
    consignor_id: u64,
    lot_numbers: Vec<u32>,
    commission_rate_bps: u32,
    insurance_coverage_cents: u64,
    minimum_sale_price_cents: Option<u64>,
    signed_date: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AuthenticationCertificate {
    certificate_id: u64,
    lot_number: u32,
    expert_name: String,
    expert_credentials: String,
    authentication_date: String,
    methodology: String,
    conclusion: AuthenticationConclusion,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AuthenticationConclusion {
    Authentic,
    LikelyAuthentic { confidence_pct: u8 },
    Inconclusive { reason: String },
    NotAuthentic,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ConditionReport {
    report_id: u64,
    lot_number: u32,
    overall_grade: ConditionGrade,
    surface_condition: String,
    structural_integrity: String,
    restoration_history: Vec<String>,
    examiner_name: String,
    report_date: String,
    uv_examined: bool,
    infrared_examined: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ConditionGrade {
    Excellent,
    VeryGood,
    Good,
    Fair,
    Poor,
    AsFound,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProvenanceEntry {
    owner_name: String,
    acquisition_method: String,
    from_year: Option<u16>,
    to_year: Option<u16>,
    location: String,
    documentation_ref: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProvenanceRecord {
    lot_number: u32,
    chain: Vec<ProvenanceEntry>,
    exhibition_history: Vec<String>,
    literature_references: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ShippingLogistics {
    shipment_id: u64,
    lot_number: u32,
    origin_address: String,
    destination_address: String,
    carrier: String,
    tracking_number: Option<String>,
    crate_dimensions_cm: (f64, f64, f64),
    weight_kg: f64,
    insurance_value_cents: u64,
    climate_controlled: bool,
    white_glove_service: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PostSaleSettlement {
    settlement_id: u64,
    sale_id: u64,
    buyer_id: u64,
    hammer_price_cents: u64,
    buyers_premium_cents: u64,
    taxes_cents: u64,
    shipping_cents: u64,
    total_due_cents: u64,
    payment_status: PaymentStatus,
    due_date: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PaymentStatus {
    Pending,
    PartiallyPaid { amount_paid_cents: u64 },
    PaidInFull,
    Overdue { days_overdue: u32 },
    Defaulted,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReservePriceConfig {
    lot_number: u32,
    reserve_cents: u64,
    disclosed: bool,
    auto_reduce_if_unsold: bool,
    reduced_reserve_cents: Option<u64>,
    consignor_approved: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BidType {
    InRoom {
        paddle_number: u32,
    },
    Absentee {
        max_bid_cents: u64,
        submitted_date: String,
    },
    Telephone {
        phone_line: u8,
        staff_bidder: String,
    },
    Online {
        platform_user_id: String,
        ip_address: String,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BidRecord {
    bid_id: u64,
    lot_number: u32,
    bidder_id: u64,
    bid_type: BidType,
    amount_cents: u64,
    timestamp: u64,
    accepted: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AuctionCalendarEvent {
    event_id: u64,
    title: String,
    sale_number: String,
    venue: String,
    start_date: String,
    end_date: String,
    lot_count: u32,
    preview_dates: Vec<String>,
    is_live_streamed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CollectibleGrading {
    grading_id: u64,
    item_description: String,
    grading_service: String,
    grade_label: String,
    numeric_grade: Option<f32>,
    encapsulated: bool,
    serial_number: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ConsignorPayout {
    payout_id: u64,
    consignor_id: u64,
    agreement_id: u64,
    gross_proceeds_cents: u64,
    commission_deducted_cents: u64,
    insurance_deducted_cents: u64,
    photography_fee_cents: u64,
    net_payout_cents: u64,
    payout_method: String,
    payout_date: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CustomsDeclararion {
    declaration_id: u64,
    shipment_id: u64,
    hs_code: String,
    declared_value_cents: u64,
    country_of_origin: String,
    requires_export_license: bool,
    cites_listed: bool,
    description_for_customs: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PhotographyOrder {
    order_id: u64,
    lot_numbers: Vec<u32>,
    photographer: String,
    shoot_date: String,
    image_count: u32,
    high_res_delivered: bool,
    retouching_requested: bool,
    total_cost_cents: u64,
}

// --- Tests ---

#[test]
fn test_lot_catalog_entry_roundtrip() {
    let val = LotCatalogEntry {
        lot_number: 142,
        title: "Nocturne in Blue and Gold".to_string(),
        artist_or_maker: "James McNeill Whistler".to_string(),
        medium: "Oil on canvas".to_string(),
        dimensions_cm: (50.8, 68.6, 3.2),
        year_created: Some(1872),
        estimate_low_cents: 800_000_00,
        estimate_high_cents: 1_200_000_00,
        category: "19th Century European Paintings".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode lot catalog entry");
    let (decoded, _): (LotCatalogEntry, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode lot catalog entry");
    assert_eq!(val, decoded);
}

#[test]
fn test_bidder_registration_roundtrip() {
    let val = BidderRegistration {
        bidder_id: 88_001,
        paddle_number: 312,
        full_name: "Elena Vasquez-Chen".to_string(),
        email: "elena.vc@example.com".to_string(),
        phone: "+1-212-555-0147".to_string(),
        deposit_cents: 50_000_00,
        credit_approved: true,
        registration_timestamp: 1_700_000_000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode bidder registration");
    let (decoded, _): (BidderRegistration, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode bidder registration");
    assert_eq!(val, decoded);
}

#[test]
fn test_bid_increment_table_roundtrip() {
    let val = BidIncrementTable {
        table_id: 7,
        auction_house: "Sterling & Associates".to_string(),
        rules: vec![
            BidIncrementRule {
                range_floor_cents: 0,
                range_ceiling_cents: 100_000,
                increment_cents: 5_000,
            },
            BidIncrementRule {
                range_floor_cents: 100_000,
                range_ceiling_cents: 500_000,
                increment_cents: 10_000,
            },
            BidIncrementRule {
                range_floor_cents: 500_000,
                range_ceiling_cents: 1_000_000,
                increment_cents: 25_000,
            },
            BidIncrementRule {
                range_floor_cents: 1_000_000,
                range_ceiling_cents: 5_000_000,
                increment_cents: 50_000,
            },
            BidIncrementRule {
                range_floor_cents: 5_000_000,
                range_ceiling_cents: u64::MAX,
                increment_cents: 100_000,
            },
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode bid increment table");
    let (decoded, _): (BidIncrementTable, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode bid increment table");
    assert_eq!(val, decoded);
}

#[test]
fn test_hammer_price_record_roundtrip() {
    let val = HammerPriceRecord {
        lot_number: 56,
        hammer_price_cents: 3_400_000_00,
        winning_paddle: 178,
        auctioneer_id: 3,
        sale_timestamp: 1_700_100_000,
        currency_code: "USD".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode hammer price record");
    let (decoded, _): (HammerPriceRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode hammer price record");
    assert_eq!(val, decoded);
}

#[test]
fn test_buyers_premium_schedule_roundtrip() {
    let val = BuyersPremiumSchedule {
        schedule_id: 12,
        effective_date: "2025-09-01".to_string(),
        tiers: vec![
            BuyersPremiumTier {
                threshold_cents: 1_000_000_00,
                rate_bps: 2600,
            },
            BuyersPremiumTier {
                threshold_cents: 6_000_000_00,
                rate_bps: 2000,
            },
            BuyersPremiumTier {
                threshold_cents: u64::MAX,
                rate_bps: 1450,
            },
        ],
        online_surcharge_bps: 100,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode buyers premium schedule");
    let (decoded, _): (BuyersPremiumSchedule, _) =
        decode_owned_from_slice(&bytes, config::standard())
            .expect("decode buyers premium schedule");
    assert_eq!(val, decoded);
}

#[test]
fn test_consignment_agreement_roundtrip() {
    let val = ConsignmentAgreement {
        agreement_id: 4_500_001,
        consignor_name: "The Pemberton Estate".to_string(),
        consignor_id: 77_042,
        lot_numbers: vec![201, 202, 203, 204, 205],
        commission_rate_bps: 1000,
        insurance_coverage_cents: 15_000_000_00,
        minimum_sale_price_cents: Some(2_000_000_00),
        signed_date: "2025-07-15".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode consignment agreement");
    let (decoded, _): (ConsignmentAgreement, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode consignment agreement");
    assert_eq!(val, decoded);
}

#[test]
fn test_authentication_certificate_authentic_roundtrip() {
    let val = AuthenticationCertificate {
        certificate_id: 90_120,
        lot_number: 77,
        expert_name: "Dr. Margaux Fontaine".to_string(),
        expert_credentials: "PhD Art History, Sorbonne; 25 years authentication experience"
            .to_string(),
        authentication_date: "2025-08-20".to_string(),
        methodology: "Provenance review, pigment analysis, X-ray fluorescence, canvas thread count"
            .to_string(),
        conclusion: AuthenticationConclusion::Authentic,
        notes: "Consistent with documented works from the artist's Blue Period".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode authentication certificate");
    let (decoded, _): (AuthenticationCertificate, _) =
        decode_owned_from_slice(&bytes, config::standard())
            .expect("decode authentication certificate");
    assert_eq!(val, decoded);
}

#[test]
fn test_authentication_certificate_inconclusive_roundtrip() {
    let val = AuthenticationCertificate {
        certificate_id: 90_121,
        lot_number: 78,
        expert_name: "Prof. Haruto Nakamura".to_string(),
        expert_credentials: "Senior curator, National Museum of Modern Art".to_string(),
        authentication_date: "2025-08-22".to_string(),
        methodology: "Infrared reflectography, stylistic comparison".to_string(),
        conclusion: AuthenticationConclusion::Inconclusive {
            reason: "Underdrawing pattern atypical but not conclusively disqualifying".to_string(),
        },
        notes: "Recommend additional dendrochronology testing on panel support".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode inconclusive auth cert");
    let (decoded, _): (AuthenticationCertificate, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode inconclusive auth cert");
    assert_eq!(val, decoded);
}

#[test]
fn test_condition_report_roundtrip() {
    let val = ConditionReport {
        report_id: 55_300,
        lot_number: 142,
        overall_grade: ConditionGrade::VeryGood,
        surface_condition: "Minor craquelure in upper right quadrant; no flaking".to_string(),
        structural_integrity: "Canvas taut, stretcher bars sound, no warping".to_string(),
        restoration_history: vec![
            "Relined 1965, Atelier Rentoilage Paris".to_string(),
            "Varnish removal and re-application 1998".to_string(),
        ],
        examiner_name: "Sophie Greenwald".to_string(),
        report_date: "2025-09-01".to_string(),
        uv_examined: true,
        infrared_examined: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode condition report");
    let (decoded, _): (ConditionReport, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode condition report");
    assert_eq!(val, decoded);
}

#[test]
fn test_provenance_record_roundtrip() {
    let val = ProvenanceRecord {
        lot_number: 142,
        chain: vec![
            ProvenanceEntry {
                owner_name: "The Artist's Studio".to_string(),
                acquisition_method: "Created".to_string(),
                from_year: Some(1872),
                to_year: Some(1874),
                location: "London, England".to_string(),
                documentation_ref: None,
            },
            ProvenanceEntry {
                owner_name: "W. C. Alexander".to_string(),
                acquisition_method: "Purchased directly from artist".to_string(),
                from_year: Some(1874),
                to_year: Some(1916),
                location: "London, England".to_string(),
                documentation_ref: Some("Letter dated March 1874, Alexander Papers".to_string()),
            },
            ProvenanceEntry {
                owner_name: "Private European Collection".to_string(),
                acquisition_method: "Inherited".to_string(),
                from_year: Some(1916),
                to_year: None,
                location: "Geneva, Switzerland".to_string(),
                documentation_ref: Some("Estate inventory ref. 1916-A-447".to_string()),
            },
        ],
        exhibition_history: vec![
            "Royal Academy, London, 1873".to_string(),
            "Retrospective, Tate Gallery, 1934".to_string(),
        ],
        literature_references: vec![
            "Young, A. M., et al., The Paintings of James McNeill Whistler, 1980, cat. no. 171"
                .to_string(),
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode provenance record");
    let (decoded, _): (ProvenanceRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode provenance record");
    assert_eq!(val, decoded);
}

#[test]
fn test_shipping_logistics_roundtrip() {
    let val = ShippingLogistics {
        shipment_id: 670_001,
        lot_number: 142,
        origin_address: "Sterling & Associates, 1230 Avenue of the Americas, New York, NY 10020"
            .to_string(),
        destination_address: "14 Rue du Faubourg Saint-Honore, 75008 Paris, France".to_string(),
        carrier: "Cadogan Tate Fine Art".to_string(),
        tracking_number: Some("CT-FA-2025-90001".to_string()),
        crate_dimensions_cm: (85.0, 100.0, 22.0),
        weight_kg: 28.5,
        insurance_value_cents: 4_000_000_00,
        climate_controlled: true,
        white_glove_service: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode shipping logistics");
    let (decoded, _): (ShippingLogistics, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode shipping logistics");
    assert_eq!(val, decoded);
}

#[test]
fn test_post_sale_settlement_paid_roundtrip() {
    let val = PostSaleSettlement {
        settlement_id: 120_500,
        sale_id: 8_001,
        buyer_id: 88_001,
        hammer_price_cents: 3_400_000_00,
        buyers_premium_cents: 765_500_00,
        taxes_cents: 345_856_00,
        shipping_cents: 12_500_00,
        total_due_cents: 4_523_856_00,
        payment_status: PaymentStatus::PaidInFull,
        due_date: "2025-10-15".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode post sale settlement");
    let (decoded, _): (PostSaleSettlement, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode post sale settlement");
    assert_eq!(val, decoded);
}

#[test]
fn test_reserve_price_config_roundtrip() {
    let val = ReservePriceConfig {
        lot_number: 142,
        reserve_cents: 600_000_00,
        disclosed: false,
        auto_reduce_if_unsold: true,
        reduced_reserve_cents: Some(450_000_00),
        consignor_approved: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode reserve price config");
    let (decoded, _): (ReservePriceConfig, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode reserve price config");
    assert_eq!(val, decoded);
}

#[test]
fn test_bid_record_in_room_roundtrip() {
    let val = BidRecord {
        bid_id: 990_001,
        lot_number: 56,
        bidder_id: 88_001,
        bid_type: BidType::InRoom { paddle_number: 312 },
        amount_cents: 3_400_000_00,
        timestamp: 1_700_100_000,
        accepted: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode in-room bid record");
    let (decoded, _): (BidRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode in-room bid record");
    assert_eq!(val, decoded);
}

#[test]
fn test_bid_record_absentee_roundtrip() {
    let val = BidRecord {
        bid_id: 990_010,
        lot_number: 73,
        bidder_id: 88_020,
        bid_type: BidType::Absentee {
            max_bid_cents: 750_000_00,
            submitted_date: "2025-09-10".to_string(),
        },
        amount_cents: 600_000_00,
        timestamp: 1_700_050_000,
        accepted: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode absentee bid record");
    let (decoded, _): (BidRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode absentee bid record");
    assert_eq!(val, decoded);
}

#[test]
fn test_bid_record_telephone_roundtrip() {
    let val = BidRecord {
        bid_id: 990_015,
        lot_number: 56,
        bidder_id: 88_033,
        bid_type: BidType::Telephone {
            phone_line: 4,
            staff_bidder: "Annalise Moreau".to_string(),
        },
        amount_cents: 3_200_000_00,
        timestamp: 1_700_099_500,
        accepted: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode telephone bid record");
    let (decoded, _): (BidRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode telephone bid record");
    assert_eq!(val, decoded);
}

#[test]
fn test_bid_record_online_roundtrip() {
    let val = BidRecord {
        bid_id: 990_020,
        lot_number: 112,
        bidder_id: 88_099,
        bid_type: BidType::Online {
            platform_user_id: "collector_hk_2025".to_string(),
            ip_address: "203.0.113.42".to_string(),
        },
        amount_cents: 45_000_00,
        timestamp: 1_700_080_000,
        accepted: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode online bid record");
    let (decoded, _): (BidRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode online bid record");
    assert_eq!(val, decoded);
}

#[test]
fn test_auction_calendar_event_roundtrip() {
    let val = AuctionCalendarEvent {
        event_id: 3_001,
        title: "Important Old Master Paintings".to_string(),
        sale_number: "SA-2025-OM-044".to_string(),
        venue: "Sterling & Associates, Main Saleroom".to_string(),
        start_date: "2025-11-12".to_string(),
        end_date: "2025-11-12".to_string(),
        lot_count: 87,
        preview_dates: vec![
            "2025-11-08".to_string(),
            "2025-11-09".to_string(),
            "2025-11-10".to_string(),
            "2025-11-11".to_string(),
        ],
        is_live_streamed: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode auction calendar event");
    let (decoded, _): (AuctionCalendarEvent, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode auction calendar event");
    assert_eq!(val, decoded);
}

#[test]
fn test_collectible_grading_roundtrip() {
    let val = CollectibleGrading {
        grading_id: 440_001,
        item_description: "1952 Topps #311 Mickey Mantle Rookie Card".to_string(),
        grading_service: "PSA".to_string(),
        grade_label: "NM-MT".to_string(),
        numeric_grade: Some(8.0),
        encapsulated: true,
        serial_number: "PSA-84770012".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode collectible grading");
    let (decoded, _): (CollectibleGrading, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode collectible grading");
    assert_eq!(val, decoded);
}

#[test]
fn test_consignor_payout_roundtrip() {
    let val = ConsignorPayout {
        payout_id: 60_200,
        consignor_id: 77_042,
        agreement_id: 4_500_001,
        gross_proceeds_cents: 8_750_000_00,
        commission_deducted_cents: 875_000_00,
        insurance_deducted_cents: 43_750_00,
        photography_fee_cents: 2_500_00,
        net_payout_cents: 7_828_750_00,
        payout_method: "Wire transfer".to_string(),
        payout_date: "2025-11-28".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode consignor payout");
    let (decoded, _): (ConsignorPayout, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode consignor payout");
    assert_eq!(val, decoded);
}

#[test]
fn test_customs_declaration_roundtrip() {
    let val = CustomsDeclararion {
        declaration_id: 770_050,
        shipment_id: 670_001,
        hs_code: "9701.10.0000".to_string(),
        declared_value_cents: 3_400_000_00,
        country_of_origin: "United Kingdom".to_string(),
        requires_export_license: true,
        cites_listed: false,
        description_for_customs: "Original oil painting on canvas, 19th century, by J.M. Whistler"
            .to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode customs declaration");
    let (decoded, _): (CustomsDeclararion, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode customs declaration");
    assert_eq!(val, decoded);
}

#[test]
fn test_photography_order_roundtrip() {
    let val = PhotographyOrder {
        order_id: 33_100,
        lot_numbers: vec![201, 202, 203, 204, 205, 206, 207, 208],
        photographer: "Atelier Lumiere".to_string(),
        shoot_date: "2025-08-05".to_string(),
        image_count: 48,
        high_res_delivered: true,
        retouching_requested: true,
        total_cost_cents: 4_800_00,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode photography order");
    let (decoded, _): (PhotographyOrder, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode photography order");
    assert_eq!(val, decoded);
}
