//! Advanced nested struct encoding tests for OxiCode (set 11, part B)
//! Theme: Fashion retail and apparel supply chain management — tests 12–22

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

// ---------------------------------------------------------------------------
// Test 12: Care label and compliance data
// ---------------------------------------------------------------------------
#[test]
fn test_care_label_compliance() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CareInstruction {
        symbol_code: u16,
        text: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ComplianceCert {
        cert_name: String,
        cert_number: String,
        issued_epoch: u64,
        expires_epoch: u64,
        valid: bool,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CareLabel {
        garment_sku: String,
        care_instructions: Vec<CareInstruction>,
        certifications: Vec<ComplianceCert>,
        country_of_origin: String,
        translated_labels: Vec<String>,
    }

    let label = CareLabel {
        garment_sku: "JK-WOOL-2024".to_string(),
        care_instructions: vec![
            CareInstruction {
                symbol_code: 100,
                text: "Dry clean only".to_string(),
            },
            CareInstruction {
                symbol_code: 200,
                text: "Do not bleach".to_string(),
            },
            CareInstruction {
                symbol_code: 310,
                text: "Iron low heat".to_string(),
            },
            CareInstruction {
                symbol_code: 400,
                text: "Do not tumble dry".to_string(),
            },
        ],
        certifications: vec![
            ComplianceCert {
                cert_name: "OEKO-TEX Standard 100".to_string(),
                cert_number: "OT-2024-88321".to_string(),
                issued_epoch: 1696118400,
                expires_epoch: 1727740800,
                valid: true,
            },
            ComplianceCert {
                cert_name: "GOTS Organic".to_string(),
                cert_number: "GOTS-44210".to_string(),
                issued_epoch: 1696118400,
                expires_epoch: 1727740800,
                valid: true,
            },
        ],
        country_of_origin: "IT".to_string(),
        translated_labels: vec![
            "EN".to_string(),
            "FR".to_string(),
            "DE".to_string(),
            "IT".to_string(),
            "JA".to_string(),
        ],
    };
    let bytes = encode_to_vec(&label).expect("encode care label");
    let (decoded, _): (CareLabel, usize) = decode_from_slice(&bytes).expect("decode care label");
    assert_eq!(label, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Visual merchandising planogram
// ---------------------------------------------------------------------------
#[test]
fn test_visual_merchandising_planogram() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DisplayFixture {
        fixture_type: String,
        width_cm: u16,
        height_cm: u16,
        assigned_skus: Vec<String>,
        max_pieces: u16,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ZoneLayout {
        zone_name: String,
        floor_level: u8,
        fixtures: Vec<DisplayFixture>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Planogram {
        store_id: u32,
        season: String,
        effective_date_epoch: u64,
        zones: Vec<ZoneLayout>,
    }

    let plano = Planogram {
        store_id: 201,
        season: "SS25".to_string(),
        effective_date_epoch: 1708300000,
        zones: vec![
            ZoneLayout {
                zone_name: "Window Display".to_string(),
                floor_level: 0,
                fixtures: vec![DisplayFixture {
                    fixture_type: "mannequin_group".to_string(),
                    width_cm: 300,
                    height_cm: 180,
                    assigned_skus: vec!["SS25-DWN-001".to_string(), "SS25-ACC-010".to_string()],
                    max_pieces: 3,
                }],
            },
            ZoneLayout {
                zone_name: "Main Floor - Women".to_string(),
                floor_level: 1,
                fixtures: vec![
                    DisplayFixture {
                        fixture_type: "hanging_rail".to_string(),
                        width_cm: 120,
                        height_cm: 170,
                        assigned_skus: vec!["SS25-DWN-001".to_string(), "SS25-DWN-002".to_string()],
                        max_pieces: 24,
                    },
                    DisplayFixture {
                        fixture_type: "folding_table".to_string(),
                        width_cm: 90,
                        height_cm: 85,
                        assigned_skus: vec!["SS25-KNT-010".to_string()],
                        max_pieces: 36,
                    },
                ],
            },
        ],
    };
    let bytes = encode_to_vec(&plano).expect("encode planogram");
    let (decoded, _): (Planogram, usize) = decode_from_slice(&bytes).expect("decode planogram");
    assert_eq!(plano, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Sustainability scorecard
// ---------------------------------------------------------------------------
#[test]
fn test_sustainability_scorecard() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct EmissionEntry {
        scope: u8,
        category: String,
        co2_grams: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WaterUsage {
        process: String,
        litres: u64,
        recycled_litres: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SustainabilityMetrics {
        emissions: Vec<EmissionEntry>,
        water: Vec<WaterUsage>,
        renewable_energy_pct: u8,
        waste_diverted_pct: u8,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ProductScorecard {
        sku: String,
        lifecycle_stage: String,
        metrics: SustainabilityMetrics,
        overall_grade: String,
    }

    let card = ProductScorecard {
        sku: "JK-WOOL-2024".to_string(),
        lifecycle_stage: "cradle-to-gate".to_string(),
        metrics: SustainabilityMetrics {
            emissions: vec![
                EmissionEntry {
                    scope: 1,
                    category: "raw_material".to_string(),
                    co2_grams: 4200,
                },
                EmissionEntry {
                    scope: 2,
                    category: "manufacturing".to_string(),
                    co2_grams: 1800,
                },
                EmissionEntry {
                    scope: 3,
                    category: "transport".to_string(),
                    co2_grams: 900,
                },
            ],
            water: vec![
                WaterUsage {
                    process: "dyeing".to_string(),
                    litres: 120,
                    recycled_litres: 45,
                },
                WaterUsage {
                    process: "washing".to_string(),
                    litres: 80,
                    recycled_litres: 60,
                },
            ],
            renewable_energy_pct: 72,
            waste_diverted_pct: 88,
        },
        overall_grade: "B+".to_string(),
    };
    let bytes = encode_to_vec(&card).expect("encode sustainability scorecard");
    let (decoded, _): (ProductScorecard, usize) =
        decode_from_slice(&bytes).expect("decode sustainability scorecard");
    assert_eq!(card, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Wholesale buyer order with payment terms
// ---------------------------------------------------------------------------
#[test]
fn test_wholesale_buyer_order() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct BuyerContact {
        name: String,
        email: String,
        phone: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PaymentTerms {
        net_days: u16,
        discount_bps: u16,
        discount_days: u16,
        currency: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WholesaleLine {
        sku: String,
        size_run: Vec<String>,
        packs: u32,
        units_per_pack: u16,
        wholesale_price_cents: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WholesaleOrder {
        po_number: String,
        buyer: BuyerContact,
        terms: PaymentTerms,
        lines: Vec<WholesaleLine>,
        delivery_window_start: u64,
        delivery_window_end: u64,
    }

    let order = WholesaleOrder {
        po_number: "PO-2025-NM-0042".to_string(),
        buyer: BuyerContact {
            name: "Nordstrom Menswear Dept".to_string(),
            email: "buying@nordstrom.example".to_string(),
            phone: "+1-206-555-0199".to_string(),
        },
        terms: PaymentTerms {
            net_days: 60,
            discount_bps: 200,
            discount_days: 10,
            currency: "USD".to_string(),
        },
        lines: vec![
            WholesaleLine {
                sku: "AW25-OC-BLZ".to_string(),
                size_run: vec![
                    "S".to_string(),
                    "M".to_string(),
                    "L".to_string(),
                    "XL".to_string(),
                ],
                packs: 50,
                units_per_pack: 4,
                wholesale_price_cents: 7800,
            },
            WholesaleLine {
                sku: "AW25-OC-TRS".to_string(),
                size_run: vec![
                    "28".to_string(),
                    "30".to_string(),
                    "32".to_string(),
                    "34".to_string(),
                    "36".to_string(),
                ],
                packs: 40,
                units_per_pack: 5,
                wholesale_price_cents: 5200,
            },
        ],
        delivery_window_start: 1722470400,
        delivery_window_end: 1724976000,
    };
    let bytes = encode_to_vec(&order).expect("encode wholesale order");
    let (decoded, _): (WholesaleOrder, usize) =
        decode_from_slice(&bytes).expect("decode wholesale order");
    assert_eq!(order, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Influencer collaboration campaign
// ---------------------------------------------------------------------------
#[test]
fn test_influencer_collaboration_campaign() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SocialPlatform {
        platform_name: String,
        handle: String,
        follower_count: u64,
        engagement_rate_bps: u16,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Influencer {
        influencer_id: u32,
        name: String,
        platforms: Vec<SocialPlatform>,
        niche: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Deliverable {
        content_type: String,
        quantity: u8,
        deadline_epoch: u64,
        approved: bool,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Campaign {
        campaign_id: u64,
        name: String,
        influencers: Vec<Influencer>,
        deliverables: Vec<Deliverable>,
        budget_cents: u64,
        collection_tag: String,
    }

    let campaign = Campaign {
        campaign_id: 9900,
        name: "SS25 Dawn Launch".to_string(),
        influencers: vec![
            Influencer {
                influencer_id: 301,
                name: "Chiara Bianchi".to_string(),
                platforms: vec![
                    SocialPlatform {
                        platform_name: "Instagram".to_string(),
                        handle: "@chiarab_style".to_string(),
                        follower_count: 2_800_000,
                        engagement_rate_bps: 320,
                    },
                    SocialPlatform {
                        platform_name: "TikTok".to_string(),
                        handle: "@chiarab".to_string(),
                        follower_count: 4_100_000,
                        engagement_rate_bps: 510,
                    },
                ],
                niche: "high_fashion".to_string(),
            },
            Influencer {
                influencer_id: 302,
                name: "Yuki Sato".to_string(),
                platforms: vec![SocialPlatform {
                    platform_name: "YouTube".to_string(),
                    handle: "YukiStyleJP".to_string(),
                    follower_count: 950_000,
                    engagement_rate_bps: 480,
                }],
                niche: "minimalist_fashion".to_string(),
            },
        ],
        deliverables: vec![
            Deliverable {
                content_type: "instagram_reel".to_string(),
                quantity: 3,
                deadline_epoch: 1710000000,
                approved: false,
            },
            Deliverable {
                content_type: "tiktok_video".to_string(),
                quantity: 2,
                deadline_epoch: 1710000000,
                approved: false,
            },
            Deliverable {
                content_type: "youtube_haul".to_string(),
                quantity: 1,
                deadline_epoch: 1711000000,
                approved: false,
            },
        ],
        budget_cents: 25_000_00,
        collection_tag: "SS25-Dawn".to_string(),
    };
    let bytes = encode_to_vec(&campaign).expect("encode campaign");
    let (decoded, _): (Campaign, usize) = decode_from_slice(&bytes).expect("decode campaign");
    assert_eq!(campaign, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Garment alteration tracking
// ---------------------------------------------------------------------------
#[test]
fn test_garment_alteration_tracking() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct AlterationDetail {
        alteration_type: String,
        area: String,
        adjustment_mm: i32,
        tailor_notes: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct AlterationOrder {
        order_id: u64,
        customer_id: u64,
        garment_sku: String,
        original_size: String,
        alterations: Vec<AlterationDetail>,
        rush_order: bool,
        estimated_minutes: u32,
        price_cents: u64,
        completed: bool,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TailorWorkload {
        tailor_id: u32,
        name: String,
        specializations: Vec<String>,
        active_orders: Vec<AlterationOrder>,
    }

    let workload = TailorWorkload {
        tailor_id: 15,
        name: "Giovanni Marchetti".to_string(),
        specializations: vec![
            "suits".to_string(),
            "evening_wear".to_string(),
            "leather".to_string(),
        ],
        active_orders: vec![
            AlterationOrder {
                order_id: 60001,
                customer_id: 440021,
                garment_sku: "AW25-OC-BLZ".to_string(),
                original_size: "L".to_string(),
                alterations: vec![
                    AlterationDetail {
                        alteration_type: "shorten".to_string(),
                        area: "sleeve".to_string(),
                        adjustment_mm: -15,
                        tailor_notes: "maintain button placement".to_string(),
                    },
                    AlterationDetail {
                        alteration_type: "take_in".to_string(),
                        area: "waist".to_string(),
                        adjustment_mm: -10,
                        tailor_notes: "both sides evenly".to_string(),
                    },
                ],
                rush_order: false,
                estimated_minutes: 90,
                price_cents: 4500,
                completed: false,
            },
            AlterationOrder {
                order_id: 60002,
                customer_id: 440099,
                garment_sku: "AW25-EVE-GWN".to_string(),
                original_size: "S".to_string(),
                alterations: vec![AlterationDetail {
                    alteration_type: "hem".to_string(),
                    area: "skirt".to_string(),
                    adjustment_mm: -30,
                    tailor_notes: "invisible hem stitch".to_string(),
                }],
                rush_order: true,
                estimated_minutes: 45,
                price_cents: 6000,
                completed: false,
            },
        ],
    };
    let bytes = encode_to_vec(&workload).expect("encode tailor workload");
    let (decoded, _): (TailorWorkload, usize) =
        decode_from_slice(&bytes).expect("decode tailor workload");
    assert_eq!(workload, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Fashion show event lineup
// ---------------------------------------------------------------------------
#[test]
fn test_fashion_show_event_lineup() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ModelProfile {
        model_id: u32,
        name: String,
        height_cm: u16,
        agency: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct RunwayLook {
        look_number: u8,
        model: ModelProfile,
        garment_skus: Vec<String>,
        music_cue_seconds: u32,
        walk_duration_seconds: u16,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ShowSegment {
        segment_name: String,
        looks: Vec<RunwayLook>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FashionShow {
        show_name: String,
        venue: String,
        date_epoch: u64,
        segments: Vec<ShowSegment>,
        total_looks: u16,
        press_accredited: u32,
    }

    let show = FashionShow {
        show_name: "Maison Lumiere SS25 Presentation".to_string(),
        venue: "Palais de Tokyo, Paris".to_string(),
        date_epoch: 1709200000,
        segments: vec![
            ShowSegment {
                segment_name: "Dawn".to_string(),
                looks: vec![
                    RunwayLook {
                        look_number: 1,
                        model: ModelProfile {
                            model_id: 501,
                            name: "Adut Akech".to_string(),
                            height_cm: 178,
                            agency: "Elite".to_string(),
                        },
                        garment_skus: vec!["SS25-DWN-001".to_string(), "SS25-ACC-010".to_string()],
                        music_cue_seconds: 0,
                        walk_duration_seconds: 42,
                    },
                    RunwayLook {
                        look_number: 2,
                        model: ModelProfile {
                            model_id: 502,
                            name: "Liu Wen".to_string(),
                            height_cm: 178,
                            agency: "IMG".to_string(),
                        },
                        garment_skus: vec!["SS25-DWN-002".to_string()],
                        music_cue_seconds: 45,
                        walk_duration_seconds: 40,
                    },
                ],
            },
            ShowSegment {
                segment_name: "Dusk".to_string(),
                looks: vec![RunwayLook {
                    look_number: 3,
                    model: ModelProfile {
                        model_id: 503,
                        name: "Vittoria Ceretti".to_string(),
                        height_cm: 176,
                        agency: "Next".to_string(),
                    },
                    garment_skus: vec![
                        "SS25-DSK-001".to_string(),
                        "SS25-ACC-030".to_string(),
                        "SS25-SHO-015".to_string(),
                    ],
                    music_cue_seconds: 180,
                    walk_duration_seconds: 50,
                }],
            },
        ],
        total_looks: 3,
        press_accredited: 240,
    };
    let bytes = encode_to_vec(&show).expect("encode fashion show");
    let (decoded, _): (FashionShow, usize) =
        decode_from_slice(&bytes).expect("decode fashion show");
    assert_eq!(show, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: E-commerce product review aggregation
// ---------------------------------------------------------------------------
#[test]
fn test_product_review_aggregation() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReviewPhoto {
        photo_url: String,
        width_px: u16,
        height_px: u16,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FitFeedback {
        runs_small: bool,
        runs_large: bool,
        true_to_size: bool,
        reviewer_height_cm: Option<u16>,
        reviewer_weight_kg: Option<u16>,
        size_purchased: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Review {
        review_id: u64,
        rating: u8,
        title: String,
        body: String,
        fit: FitFeedback,
        photos: Vec<ReviewPhoto>,
        verified_purchase: bool,
        helpful_votes: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReviewSummary {
        sku: String,
        total_reviews: u32,
        average_rating_tenths: u16,
        true_to_size_pct: u8,
        reviews: Vec<Review>,
    }

    let summary = ReviewSummary {
        sku: "SS25-DWN-001".to_string(),
        total_reviews: 2,
        average_rating_tenths: 45,
        true_to_size_pct: 68,
        reviews: vec![
            Review {
                review_id: 110001,
                rating: 5,
                title: "Absolutely stunning".to_string(),
                body: "The silk quality is incredible. Bought for a wedding and received many compliments.".to_string(),
                fit: FitFeedback {
                    runs_small: false,
                    runs_large: false,
                    true_to_size: true,
                    reviewer_height_cm: Some(165),
                    reviewer_weight_kg: Some(55),
                    size_purchased: "S".to_string(),
                },
                photos: vec![
                    ReviewPhoto { photo_url: "https://img.example.com/r/110001_1.jpg".to_string(), width_px: 1200, height_px: 1600 },
                    ReviewPhoto { photo_url: "https://img.example.com/r/110001_2.jpg".to_string(), width_px: 1200, height_px: 1600 },
                ],
                verified_purchase: true,
                helpful_votes: 42,
            },
            Review {
                review_id: 110002,
                rating: 4,
                title: "Beautiful but runs small".to_string(),
                body: "Love the fabric but had to exchange for a size up.".to_string(),
                fit: FitFeedback {
                    runs_small: true,
                    runs_large: false,
                    true_to_size: false,
                    reviewer_height_cm: Some(172),
                    reviewer_weight_kg: Some(64),
                    size_purchased: "M".to_string(),
                },
                photos: vec![],
                verified_purchase: true,
                helpful_votes: 18,
            },
        ],
    };
    let bytes = encode_to_vec(&summary).expect("encode review summary");
    let (decoded, _): (ReviewSummary, usize) =
        decode_from_slice(&bytes).expect("decode review summary");
    assert_eq!(summary, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Loyalty programme tiers
// ---------------------------------------------------------------------------
#[test]
fn test_loyalty_programme_tiers() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TierBenefit {
        benefit_name: String,
        description: String,
        value_cents: Option<u64>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LoyaltyTier {
        tier_name: String,
        min_spend_cents: u64,
        point_multiplier_bps: u16,
        benefits: Vec<TierBenefit>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MemberActivity {
        order_id: u64,
        spent_cents: u64,
        points_earned: u32,
        epoch: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LoyaltyMember {
        member_id: u64,
        name: String,
        current_tier: LoyaltyTier,
        lifetime_spend_cents: u64,
        points_balance: u64,
        recent_activity: Vec<MemberActivity>,
    }

    let member = LoyaltyMember {
        member_id: 440021,
        name: "Elena Rossi".to_string(),
        current_tier: LoyaltyTier {
            tier_name: "Gold".to_string(),
            min_spend_cents: 500_000,
            point_multiplier_bps: 15000,
            benefits: vec![
                TierBenefit {
                    benefit_name: "free_shipping".to_string(),
                    description: "Free express shipping on all orders".to_string(),
                    value_cents: None,
                },
                TierBenefit {
                    benefit_name: "birthday_voucher".to_string(),
                    description: "Birthday gift voucher".to_string(),
                    value_cents: Some(5000),
                },
                TierBenefit {
                    benefit_name: "early_access".to_string(),
                    description: "48h early access to new collections".to_string(),
                    value_cents: None,
                },
            ],
        },
        lifetime_spend_cents: 782_500,
        points_balance: 11_430,
        recent_activity: vec![
            MemberActivity {
                order_id: 1_000_042,
                spent_cents: 15997,
                points_earned: 240,
                epoch: 1700200000,
            },
            MemberActivity {
                order_id: 1_000_038,
                spent_cents: 32500,
                points_earned: 488,
                epoch: 1699800000,
            },
        ],
    };
    let bytes = encode_to_vec(&member).expect("encode loyalty member");
    let (decoded, _): (LoyaltyMember, usize) =
        decode_from_slice(&bytes).expect("decode loyalty member");
    assert_eq!(member, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Textile dyeing batch records
// ---------------------------------------------------------------------------
#[test]
fn test_textile_dyeing_batch() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DyeChemical {
        chemical_name: String,
        cas_number: String,
        dosage_grams_per_litre: u32,
        eco_rating: u8,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DyeRecipe {
        recipe_id: String,
        target_color: String,
        target_hex: u32,
        chemicals: Vec<DyeChemical>,
        temperature_celsius: u16,
        duration_minutes: u16,
        ph_target_tenths: u16,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ColorfastnessTest {
        test_type: String,
        grade: u8,
        passed: bool,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DyeBatch {
        batch_id: u64,
        fabric_ref: String,
        length_metres: u32,
        recipe: DyeRecipe,
        tests: Vec<ColorfastnessTest>,
        approved: bool,
        operator_id: u32,
    }

    let batch = DyeBatch {
        batch_id: 330055,
        fabric_ref: "FAB-001".to_string(),
        length_metres: 800,
        recipe: DyeRecipe {
            recipe_id: "DYE-NVY-042".to_string(),
            target_color: "Navy".to_string(),
            target_hex: 0x000080,
            chemicals: vec![
                DyeChemical {
                    chemical_name: "Reactive Blue 19".to_string(),
                    cas_number: "2580-78-1".to_string(),
                    dosage_grams_per_litre: 25,
                    eco_rating: 7,
                },
                DyeChemical {
                    chemical_name: "Sodium Carbonate".to_string(),
                    cas_number: "497-19-8".to_string(),
                    dosage_grams_per_litre: 15,
                    eco_rating: 9,
                },
                DyeChemical {
                    chemical_name: "Sodium Sulphate".to_string(),
                    cas_number: "7757-82-6".to_string(),
                    dosage_grams_per_litre: 40,
                    eco_rating: 8,
                },
            ],
            temperature_celsius: 60,
            duration_minutes: 90,
            ph_target_tenths: 110,
        },
        tests: vec![
            ColorfastnessTest {
                test_type: "washing_40C".to_string(),
                grade: 4,
                passed: true,
            },
            ColorfastnessTest {
                test_type: "rubbing_dry".to_string(),
                grade: 5,
                passed: true,
            },
            ColorfastnessTest {
                test_type: "rubbing_wet".to_string(),
                grade: 3,
                passed: true,
            },
            ColorfastnessTest {
                test_type: "light_exposure".to_string(),
                grade: 4,
                passed: true,
            },
        ],
        approved: true,
        operator_id: 88,
    };
    let bytes = encode_to_vec(&batch).expect("encode dye batch");
    let (decoded, _): (DyeBatch, usize) = decode_from_slice(&bytes).expect("decode dye batch");
    assert_eq!(batch, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Multi-warehouse replenishment plan
// ---------------------------------------------------------------------------
#[test]
fn test_multi_warehouse_replenishment_plan() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DemandForecast {
        sku: String,
        size: String,
        weekly_demand: Vec<u32>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct StockLevel {
        sku: String,
        size: String,
        on_hand: u32,
        in_transit: u32,
        reorder_point: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReplenishmentLine {
        sku: String,
        size: String,
        order_qty: u32,
        source_warehouse_id: u32,
        estimated_arrival_epoch: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WarehouseNode {
        warehouse_id: u32,
        name: String,
        stock_levels: Vec<StockLevel>,
        forecasts: Vec<DemandForecast>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReplenishmentPlan {
        plan_id: u64,
        generated_epoch: u64,
        warehouses: Vec<WarehouseNode>,
        transfers: Vec<ReplenishmentLine>,
        total_units_to_move: u32,
    }

    let plan = ReplenishmentPlan {
        plan_id: 42000,
        generated_epoch: 1700700000,
        warehouses: vec![
            WarehouseNode {
                warehouse_id: 55,
                name: "Rotterdam Hub".to_string(),
                stock_levels: vec![
                    StockLevel {
                        sku: "SS25-DWN-001".to_string(),
                        size: "S".to_string(),
                        on_hand: 300,
                        in_transit: 0,
                        reorder_point: 100,
                    },
                    StockLevel {
                        sku: "SS25-DWN-001".to_string(),
                        size: "M".to_string(),
                        on_hand: 50,
                        in_transit: 200,
                        reorder_point: 150,
                    },
                ],
                forecasts: vec![
                    DemandForecast {
                        sku: "SS25-DWN-001".to_string(),
                        size: "S".to_string(),
                        weekly_demand: vec![40, 45, 50, 55],
                    },
                    DemandForecast {
                        sku: "SS25-DWN-001".to_string(),
                        size: "M".to_string(),
                        weekly_demand: vec![60, 65, 70, 80],
                    },
                ],
            },
            WarehouseNode {
                warehouse_id: 60,
                name: "Milan DC".to_string(),
                stock_levels: vec![
                    StockLevel {
                        sku: "SS25-DWN-001".to_string(),
                        size: "S".to_string(),
                        on_hand: 80,
                        in_transit: 100,
                        reorder_point: 60,
                    },
                    StockLevel {
                        sku: "SS25-DWN-001".to_string(),
                        size: "M".to_string(),
                        on_hand: 400,
                        in_transit: 0,
                        reorder_point: 100,
                    },
                ],
                forecasts: vec![
                    DemandForecast {
                        sku: "SS25-DWN-001".to_string(),
                        size: "S".to_string(),
                        weekly_demand: vec![20, 25, 30, 25],
                    },
                    DemandForecast {
                        sku: "SS25-DWN-001".to_string(),
                        size: "M".to_string(),
                        weekly_demand: vec![30, 35, 40, 45],
                    },
                ],
            },
        ],
        transfers: vec![
            ReplenishmentLine {
                sku: "SS25-DWN-001".to_string(),
                size: "M".to_string(),
                order_qty: 150,
                source_warehouse_id: 60,
                estimated_arrival_epoch: 1701100000,
            },
            ReplenishmentLine {
                sku: "SS25-DWN-001".to_string(),
                size: "S".to_string(),
                order_qty: 80,
                source_warehouse_id: 55,
                estimated_arrival_epoch: 1701200000,
            },
        ],
        total_units_to_move: 230,
    };
    let bytes = encode_to_vec(&plan).expect("encode replenishment plan");
    let (decoded, _): (ReplenishmentPlan, usize) =
        decode_from_slice(&bytes).expect("decode replenishment plan");
    assert_eq!(plan, decoded);
}
