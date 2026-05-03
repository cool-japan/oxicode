//! Advanced nested struct encoding tests for OxiCode (set 11, part A)
//! Theme: Fashion retail and apparel supply chain management — tests 1–11

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
// Test 1: Fashion product catalog with color/size variants
// ---------------------------------------------------------------------------
#[test]
fn test_fashion_product_catalog() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SizeVariant {
        size_code: String,
        chest_cm: u16,
        waist_cm: u16,
        stock_count: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ColorVariant {
        color_name: String,
        hex_code: u32,
        sizes: Vec<SizeVariant>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Product {
        sku: String,
        name: String,
        price_cents: u64,
        colors: Vec<ColorVariant>,
    }

    let product = Product {
        sku: "AP-2024-001".to_string(),
        name: "Silk Blouse".to_string(),
        price_cents: 8999,
        colors: vec![
            ColorVariant {
                color_name: "Ivory".to_string(),
                hex_code: 0xFFFFF0,
                sizes: vec![
                    SizeVariant {
                        size_code: "S".to_string(),
                        chest_cm: 86,
                        waist_cm: 68,
                        stock_count: 15,
                    },
                    SizeVariant {
                        size_code: "M".to_string(),
                        chest_cm: 92,
                        waist_cm: 74,
                        stock_count: 22,
                    },
                    SizeVariant {
                        size_code: "L".to_string(),
                        chest_cm: 98,
                        waist_cm: 80,
                        stock_count: 8,
                    },
                ],
            },
            ColorVariant {
                color_name: "Navy".to_string(),
                hex_code: 0x000080,
                sizes: vec![
                    SizeVariant {
                        size_code: "XS".to_string(),
                        chest_cm: 80,
                        waist_cm: 62,
                        stock_count: 5,
                    },
                    SizeVariant {
                        size_code: "S".to_string(),
                        chest_cm: 86,
                        waist_cm: 68,
                        stock_count: 18,
                    },
                ],
            },
        ],
    };
    let bytes = encode_to_vec(&product).expect("encode product catalog");
    let (decoded, _): (Product, usize) = decode_from_slice(&bytes).expect("decode product catalog");
    assert_eq!(product, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Supply chain tracking (factory → warehouse → store)
// ---------------------------------------------------------------------------
#[test]
fn test_supply_chain_tracking() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct GeoCoord {
        lat: f64,
        lon: f64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Factory {
        factory_id: u32,
        name: String,
        country: String,
        coord: GeoCoord,
        capacity_units_per_month: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Warehouse {
        warehouse_id: u32,
        name: String,
        coord: GeoCoord,
        max_pallets: u32,
        current_pallets: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct RetailStore {
        store_id: u32,
        address: String,
        coord: GeoCoord,
        floor_area_sqm: u16,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Shipment {
        shipment_id: u64,
        origin_factory: Factory,
        transit_warehouse: Warehouse,
        destination_store: RetailStore,
        unit_count: u32,
        shipped_epoch: u64,
        delivered: bool,
    }

    let shipment = Shipment {
        shipment_id: 900123,
        origin_factory: Factory {
            factory_id: 10,
            name: "Shenzhen Textiles".to_string(),
            country: "CN".to_string(),
            coord: GeoCoord {
                lat: 22.5431,
                lon: 114.0579,
            },
            capacity_units_per_month: 50000,
        },
        transit_warehouse: Warehouse {
            warehouse_id: 55,
            name: "Rotterdam Hub".to_string(),
            coord: GeoCoord {
                lat: 51.9244,
                lon: 4.4777,
            },
            max_pallets: 12000,
            current_pallets: 8430,
        },
        destination_store: RetailStore {
            store_id: 201,
            address: "123 Oxford Street, London".to_string(),
            coord: GeoCoord {
                lat: 51.5154,
                lon: -0.1410,
            },
            floor_area_sqm: 340,
        },
        unit_count: 2400,
        shipped_epoch: 1700000000,
        delivered: false,
    };
    let bytes = encode_to_vec(&shipment).expect("encode shipment");
    let (decoded, _): (Shipment, usize) = decode_from_slice(&bytes).expect("decode shipment");
    assert_eq!(shipment, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Fabric composition with material percentages
// ---------------------------------------------------------------------------
#[test]
fn test_fabric_composition() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MaterialComponent {
        material_name: String,
        percentage_tenths: u16, // 0-1000 representing 0.0%-100.0%
        origin_country: String,
        certified_organic: bool,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FabricSpec {
        fabric_id: String,
        weight_gsm: u16,
        width_cm: u16,
        components: Vec<MaterialComponent>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct GarmentFabrics {
        garment_sku: String,
        outer_fabric: FabricSpec,
        lining: Option<FabricSpec>,
        interlining: Option<FabricSpec>,
    }

    let garment = GarmentFabrics {
        garment_sku: "JK-WOOL-2024".to_string(),
        outer_fabric: FabricSpec {
            fabric_id: "FAB-001".to_string(),
            weight_gsm: 280,
            width_cm: 150,
            components: vec![
                MaterialComponent {
                    material_name: "Merino Wool".to_string(),
                    percentage_tenths: 700,
                    origin_country: "AU".to_string(),
                    certified_organic: true,
                },
                MaterialComponent {
                    material_name: "Cashmere".to_string(),
                    percentage_tenths: 200,
                    origin_country: "MN".to_string(),
                    certified_organic: false,
                },
                MaterialComponent {
                    material_name: "Elastane".to_string(),
                    percentage_tenths: 100,
                    origin_country: "DE".to_string(),
                    certified_organic: false,
                },
            ],
        },
        lining: Some(FabricSpec {
            fabric_id: "FAB-002".to_string(),
            weight_gsm: 80,
            width_cm: 140,
            components: vec![MaterialComponent {
                material_name: "Cupro".to_string(),
                percentage_tenths: 1000,
                origin_country: "JP".to_string(),
                certified_organic: false,
            }],
        }),
        interlining: None,
    };
    let bytes = encode_to_vec(&garment).expect("encode garment fabrics");
    let (decoded, _): (GarmentFabrics, usize) =
        decode_from_slice(&bytes).expect("decode garment fabrics");
    assert_eq!(garment, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Seasonal collection hierarchy
// ---------------------------------------------------------------------------
#[test]
fn test_seasonal_collection_hierarchy() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Lookbook {
        look_number: u8,
        description: String,
        hero_sku: String,
        accessory_skus: Vec<String>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Capsule {
        capsule_name: String,
        theme: String,
        looks: Vec<Lookbook>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SeasonalCollection {
        season: String,
        year: u16,
        designer: String,
        capsules: Vec<Capsule>,
        total_sku_count: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct BrandPortfolio {
        brand_name: String,
        collections: Vec<SeasonalCollection>,
    }

    let portfolio = BrandPortfolio {
        brand_name: "Maison Lumiere".to_string(),
        collections: vec![
            SeasonalCollection {
                season: "SS25".to_string(),
                year: 2025,
                designer: "Akiko Tanaka".to_string(),
                capsules: vec![
                    Capsule {
                        capsule_name: "Dawn".to_string(),
                        theme: "Sunrise pastels".to_string(),
                        looks: vec![
                            Lookbook {
                                look_number: 1,
                                description: "Flowing silk dress with layered obi belt".to_string(),
                                hero_sku: "SS25-DWN-001".to_string(),
                                accessory_skus: vec![
                                    "SS25-ACC-010".to_string(),
                                    "SS25-ACC-011".to_string(),
                                ],
                            },
                            Lookbook {
                                look_number: 2,
                                description: "Structured linen blazer with wide-leg trouser"
                                    .to_string(),
                                hero_sku: "SS25-DWN-002".to_string(),
                                accessory_skus: vec!["SS25-ACC-020".to_string()],
                            },
                        ],
                    },
                    Capsule {
                        capsule_name: "Dusk".to_string(),
                        theme: "Evening metallics".to_string(),
                        looks: vec![Lookbook {
                            look_number: 1,
                            description: "Sequined column gown".to_string(),
                            hero_sku: "SS25-DSK-001".to_string(),
                            accessory_skus: vec![],
                        }],
                    },
                ],
                total_sku_count: 87,
            },
            SeasonalCollection {
                season: "AW25".to_string(),
                year: 2025,
                designer: "Akiko Tanaka".to_string(),
                capsules: vec![],
                total_sku_count: 0,
            },
        ],
    };
    let bytes = encode_to_vec(&portfolio).expect("encode brand portfolio");
    let (decoded, _): (BrandPortfolio, usize) =
        decode_from_slice(&bytes).expect("decode brand portfolio");
    assert_eq!(portfolio, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Customer size profile with body measurements
// ---------------------------------------------------------------------------
#[test]
fn test_customer_size_profile() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct BodyMeasurement {
        label: String,
        value_mm: u32,
        measured_epoch: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SizePreference {
        category: String,
        preferred_size: String,
        fit_preference: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CustomerProfile {
        customer_id: u64,
        name: String,
        measurements: Vec<BodyMeasurement>,
        preferences: Vec<SizePreference>,
        height_cm: u16,
        weight_kg: u16,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FitRecommendation {
        profile: CustomerProfile,
        recommended_sku: String,
        recommended_size: String,
        confidence_pct: u8,
    }

    let rec = FitRecommendation {
        profile: CustomerProfile {
            customer_id: 440021,
            name: "Elena Rossi".to_string(),
            measurements: vec![
                BodyMeasurement {
                    label: "bust".to_string(),
                    value_mm: 880,
                    measured_epoch: 1700100000,
                },
                BodyMeasurement {
                    label: "waist".to_string(),
                    value_mm: 680,
                    measured_epoch: 1700100000,
                },
                BodyMeasurement {
                    label: "hip".to_string(),
                    value_mm: 960,
                    measured_epoch: 1700100000,
                },
                BodyMeasurement {
                    label: "shoulder_width".to_string(),
                    value_mm: 390,
                    measured_epoch: 1700100000,
                },
                BodyMeasurement {
                    label: "inseam".to_string(),
                    value_mm: 780,
                    measured_epoch: 1700100000,
                },
            ],
            preferences: vec![
                SizePreference {
                    category: "tops".to_string(),
                    preferred_size: "M".to_string(),
                    fit_preference: "relaxed".to_string(),
                },
                SizePreference {
                    category: "bottoms".to_string(),
                    preferred_size: "28".to_string(),
                    fit_preference: "slim".to_string(),
                },
            ],
            height_cm: 168,
            weight_kg: 58,
        },
        recommended_sku: "SS25-DWN-001".to_string(),
        recommended_size: "M".to_string(),
        confidence_pct: 92,
    };
    let bytes = encode_to_vec(&rec).expect("encode fit recommendation");
    let (decoded, _): (FitRecommendation, usize) =
        decode_from_slice(&bytes).expect("decode fit recommendation");
    assert_eq!(rec, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Order fulfillment pipeline
// ---------------------------------------------------------------------------
#[test]
fn test_order_fulfillment_pipeline() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct OrderLineItem {
        sku: String,
        size: String,
        color: String,
        quantity: u16,
        unit_price_cents: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ShippingAddress {
        street: String,
        city: String,
        postal_code: String,
        country: String,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FulfillmentStep {
        step_name: String,
        completed: bool,
        timestamp_epoch: Option<u64>,
        warehouse_id: Option<u32>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Order {
        order_id: u64,
        customer_id: u64,
        items: Vec<OrderLineItem>,
        address: ShippingAddress,
        steps: Vec<FulfillmentStep>,
        total_cents: u64,
    }

    let order = Order {
        order_id: 1_000_042,
        customer_id: 440021,
        items: vec![
            OrderLineItem {
                sku: "SS25-DWN-001".to_string(),
                size: "M".to_string(),
                color: "Ivory".to_string(),
                quantity: 1,
                unit_price_cents: 8999,
            },
            OrderLineItem {
                sku: "SS25-ACC-010".to_string(),
                size: "ONE".to_string(),
                color: "Gold".to_string(),
                quantity: 2,
                unit_price_cents: 3499,
            },
        ],
        address: ShippingAddress {
            street: "Via Roma 42".to_string(),
            city: "Milano".to_string(),
            postal_code: "20121".to_string(),
            country: "IT".to_string(),
        },
        steps: vec![
            FulfillmentStep {
                step_name: "order_placed".to_string(),
                completed: true,
                timestamp_epoch: Some(1700200000),
                warehouse_id: None,
            },
            FulfillmentStep {
                step_name: "payment_confirmed".to_string(),
                completed: true,
                timestamp_epoch: Some(1700200060),
                warehouse_id: None,
            },
            FulfillmentStep {
                step_name: "picking".to_string(),
                completed: true,
                timestamp_epoch: Some(1700210000),
                warehouse_id: Some(55),
            },
            FulfillmentStep {
                step_name: "packing".to_string(),
                completed: false,
                timestamp_epoch: None,
                warehouse_id: Some(55),
            },
            FulfillmentStep {
                step_name: "shipped".to_string(),
                completed: false,
                timestamp_epoch: None,
                warehouse_id: None,
            },
        ],
        total_cents: 15997,
    };
    let bytes = encode_to_vec(&order).expect("encode order");
    let (decoded, _): (Order, usize) = decode_from_slice(&bytes).expect("decode order");
    assert_eq!(order, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Return / exchange workflow
// ---------------------------------------------------------------------------
#[test]
fn test_return_exchange_workflow() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReturnItem {
        sku: String,
        size: String,
        reason_code: u8,
        reason_text: String,
        condition_score: u8,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ExchangeRequest {
        new_sku: String,
        new_size: String,
        price_diff_cents: i64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct QualityInspection {
        inspector_id: u32,
        passed: bool,
        notes: String,
        inspected_epoch: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReturnCase {
        case_id: u64,
        order_id: u64,
        items: Vec<ReturnItem>,
        exchange: Option<ExchangeRequest>,
        inspection: Option<QualityInspection>,
        refund_cents: Option<u64>,
        status: String,
    }

    let case = ReturnCase {
        case_id: 80001,
        order_id: 1_000_042,
        items: vec![ReturnItem {
            sku: "SS25-DWN-001".to_string(),
            size: "M".to_string(),
            reason_code: 2,
            reason_text: "Too tight in shoulders".to_string(),
            condition_score: 10,
        }],
        exchange: Some(ExchangeRequest {
            new_sku: "SS25-DWN-001".to_string(),
            new_size: "L".to_string(),
            price_diff_cents: 0,
        }),
        inspection: Some(QualityInspection {
            inspector_id: 77,
            passed: true,
            notes: "No damage, tags attached".to_string(),
            inspected_epoch: 1700400000,
        }),
        refund_cents: None,
        status: "exchange_approved".to_string(),
    };
    let bytes = encode_to_vec(&case).expect("encode return case");
    let (decoded, _): (ReturnCase, usize) = decode_from_slice(&bytes).expect("decode return case");
    assert_eq!(case, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Trend forecasting with regional breakdowns
// ---------------------------------------------------------------------------
#[test]
fn test_trend_forecasting_regional() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct RegionalDemand {
        region: String,
        projected_units: u32,
        confidence_pct: u8,
        top_colors: Vec<String>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TrendItem {
        trend_name: String,
        category: String,
        season: String,
        regional_data: Vec<RegionalDemand>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ForecastReport {
        report_id: u64,
        generated_epoch: u64,
        analyst: String,
        trends: Vec<TrendItem>,
    }

    let report = ForecastReport {
        report_id: 5500,
        generated_epoch: 1700500000,
        analyst: "Marie Dupont".to_string(),
        trends: vec![
            TrendItem {
                trend_name: "Oversized Tailoring".to_string(),
                category: "outerwear".to_string(),
                season: "AW25".to_string(),
                regional_data: vec![
                    RegionalDemand {
                        region: "EU-West".to_string(),
                        projected_units: 45000,
                        confidence_pct: 78,
                        top_colors: vec![
                            "Charcoal".to_string(),
                            "Camel".to_string(),
                            "Olive".to_string(),
                        ],
                    },
                    RegionalDemand {
                        region: "NA-East".to_string(),
                        projected_units: 32000,
                        confidence_pct: 65,
                        top_colors: vec!["Black".to_string(), "Navy".to_string()],
                    },
                ],
            },
            TrendItem {
                trend_name: "Sheer Layering".to_string(),
                category: "tops".to_string(),
                season: "SS26".to_string(),
                regional_data: vec![RegionalDemand {
                    region: "APAC".to_string(),
                    projected_units: 68000,
                    confidence_pct: 82,
                    top_colors: vec![
                        "Blush".to_string(),
                        "Lavender".to_string(),
                        "Sky".to_string(),
                        "Mint".to_string(),
                    ],
                }],
            },
        ],
    };
    let bytes = encode_to_vec(&report).expect("encode forecast report");
    let (decoded, _): (ForecastReport, usize) =
        decode_from_slice(&bytes).expect("decode forecast report");
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Pricing tiers and promotional rules
// ---------------------------------------------------------------------------
#[test]
fn test_pricing_tiers_promotions() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PriceTier {
        tier_name: String,
        min_qty: u32,
        discount_bps: u16, // basis points
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PromotionalRule {
        promo_code: String,
        description: String,
        tiers: Vec<PriceTier>,
        stackable: bool,
        valid_from_epoch: u64,
        valid_to_epoch: u64,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PricingStrategy {
        strategy_id: u32,
        base_price_cents: u64,
        currency: String,
        promotions: Vec<PromotionalRule>,
        loyalty_multiplier_bps: u16,
    }

    let strategy = PricingStrategy {
        strategy_id: 301,
        base_price_cents: 15900,
        currency: "EUR".to_string(),
        promotions: vec![
            PromotionalRule {
                promo_code: "SUMMER20".to_string(),
                description: "Summer flash sale".to_string(),
                tiers: vec![
                    PriceTier {
                        tier_name: "standard".to_string(),
                        min_qty: 1,
                        discount_bps: 2000,
                    },
                    PriceTier {
                        tier_name: "bulk".to_string(),
                        min_qty: 5,
                        discount_bps: 2500,
                    },
                ],
                stackable: false,
                valid_from_epoch: 1717200000,
                valid_to_epoch: 1719792000,
            },
            PromotionalRule {
                promo_code: "LOYALTY10".to_string(),
                description: "Loyalty programme discount".to_string(),
                tiers: vec![PriceTier {
                    tier_name: "all".to_string(),
                    min_qty: 1,
                    discount_bps: 1000,
                }],
                stackable: true,
                valid_from_epoch: 0,
                valid_to_epoch: u64::MAX,
            },
        ],
        loyalty_multiplier_bps: 150,
    };
    let bytes = encode_to_vec(&strategy).expect("encode pricing strategy");
    let (decoded, _): (PricingStrategy, usize) =
        decode_from_slice(&bytes).expect("decode pricing strategy");
    assert_eq!(strategy, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Garment production work order
// ---------------------------------------------------------------------------
#[test]
fn test_garment_production_work_order() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CutPiece {
        piece_name: String,
        fabric_ref: String,
        quantity: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SewingOperation {
        op_number: u16,
        description: String,
        machine_type: String,
        time_seconds: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct QualityCheckpoint {
        checkpoint_name: String,
        defect_tolerance_pct: u8,
        is_final: bool,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WorkOrder {
        wo_id: u64,
        style_number: String,
        target_qty: u32,
        cut_pieces: Vec<CutPiece>,
        operations: Vec<SewingOperation>,
        checkpoints: Vec<QualityCheckpoint>,
        deadline_epoch: u64,
    }

    let wo = WorkOrder {
        wo_id: 770001,
        style_number: "ST-2025-BLZ-04".to_string(),
        target_qty: 1200,
        cut_pieces: vec![
            CutPiece {
                piece_name: "front_panel".to_string(),
                fabric_ref: "FAB-001".to_string(),
                quantity: 2400,
            },
            CutPiece {
                piece_name: "back_panel".to_string(),
                fabric_ref: "FAB-001".to_string(),
                quantity: 1200,
            },
            CutPiece {
                piece_name: "sleeve".to_string(),
                fabric_ref: "FAB-001".to_string(),
                quantity: 2400,
            },
            CutPiece {
                piece_name: "collar".to_string(),
                fabric_ref: "FAB-003".to_string(),
                quantity: 1200,
            },
        ],
        operations: vec![
            SewingOperation {
                op_number: 10,
                description: "Attach front darts".to_string(),
                machine_type: "lockstitch".to_string(),
                time_seconds: 45,
            },
            SewingOperation {
                op_number: 20,
                description: "Join shoulder seams".to_string(),
                machine_type: "overlock".to_string(),
                time_seconds: 30,
            },
            SewingOperation {
                op_number: 30,
                description: "Set sleeves".to_string(),
                machine_type: "lockstitch".to_string(),
                time_seconds: 60,
            },
            SewingOperation {
                op_number: 40,
                description: "Attach collar".to_string(),
                machine_type: "lockstitch".to_string(),
                time_seconds: 55,
            },
            SewingOperation {
                op_number: 50,
                description: "Hem".to_string(),
                machine_type: "coverstitch".to_string(),
                time_seconds: 25,
            },
        ],
        checkpoints: vec![
            QualityCheckpoint {
                checkpoint_name: "post-cut".to_string(),
                defect_tolerance_pct: 3,
                is_final: false,
            },
            QualityCheckpoint {
                checkpoint_name: "mid-sew".to_string(),
                defect_tolerance_pct: 2,
                is_final: false,
            },
            QualityCheckpoint {
                checkpoint_name: "final-inspection".to_string(),
                defect_tolerance_pct: 1,
                is_final: true,
            },
        ],
        deadline_epoch: 1703000000,
    };
    let bytes = encode_to_vec(&wo).expect("encode work order");
    let (decoded, _): (WorkOrder, usize) = decode_from_slice(&bytes).expect("decode work order");
    assert_eq!(wo, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Inventory allocation across channels
// ---------------------------------------------------------------------------
#[test]
fn test_inventory_allocation_channels() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ChannelAllocation {
        channel: String,
        reserved_units: u32,
        sold_units: u32,
        buffer_units: u32,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SkuInventory {
        sku: String,
        size: String,
        color: String,
        total_units: u32,
        allocations: Vec<ChannelAllocation>,
    }
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct InventorySnapshot {
        snapshot_epoch: u64,
        warehouse_id: u32,
        items: Vec<SkuInventory>,
    }

    let snapshot = InventorySnapshot {
        snapshot_epoch: 1700600000,
        warehouse_id: 55,
        items: vec![
            SkuInventory {
                sku: "SS25-DWN-001".to_string(),
                size: "M".to_string(),
                color: "Ivory".to_string(),
                total_units: 500,
                allocations: vec![
                    ChannelAllocation {
                        channel: "ecommerce".to_string(),
                        reserved_units: 200,
                        sold_units: 120,
                        buffer_units: 20,
                    },
                    ChannelAllocation {
                        channel: "wholesale".to_string(),
                        reserved_units: 150,
                        sold_units: 80,
                        buffer_units: 0,
                    },
                    ChannelAllocation {
                        channel: "retail_stores".to_string(),
                        reserved_units: 130,
                        sold_units: 95,
                        buffer_units: 10,
                    },
                ],
            },
            SkuInventory {
                sku: "SS25-DWN-001".to_string(),
                size: "S".to_string(),
                color: "Ivory".to_string(),
                total_units: 300,
                allocations: vec![
                    ChannelAllocation {
                        channel: "ecommerce".to_string(),
                        reserved_units: 180,
                        sold_units: 140,
                        buffer_units: 10,
                    },
                    ChannelAllocation {
                        channel: "retail_stores".to_string(),
                        reserved_units: 110,
                        sold_units: 60,
                        buffer_units: 5,
                    },
                ],
            },
        ],
    };
    let bytes = encode_to_vec(&snapshot).expect("encode inventory snapshot");
    let (decoded, _): (InventorySnapshot, usize) =
        decode_from_slice(&bytes).expect("decode inventory snapshot");
    assert_eq!(snapshot, decoded);
}
