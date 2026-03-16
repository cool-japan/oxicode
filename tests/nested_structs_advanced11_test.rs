//! Advanced nested struct encoding tests for OxiCode (set 11)
//! Theme: Fashion retail and apparel supply chain management

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
