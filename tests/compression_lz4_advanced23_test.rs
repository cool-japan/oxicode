#![cfg(feature = "compression-lz4")]
//! Advanced LZ4 compression tests #23 — Fashion Retail & E-Commerce domain.
//!
//! Covers product catalogs (SKU, color, size, material), inventory by warehouse/store,
//! shopping cart snapshots, order fulfillment pipelines, return/exchange workflows,
//! size recommendation data, style compatibility matrices, seasonal collection metadata,
//! price markdowns, loyalty program tiers, customer segmentation profiles,
//! A/B test experiment configs, visual merchandising planograms, supply chain lead times,
//! and fabric composition specs.

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
// Domain enums
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GarmentSize {
    XXS,
    XS,
    S,
    M,
    L,
    XL,
    XXL,
    Custom {
        chest_cm: u16,
        waist_cm: u16,
        length_cm: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Color {
    Named(String),
    Hex(u32),
    Pantone { code: String, season: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProductCategory {
    Tops,
    Bottoms,
    Outerwear,
    Footwear,
    Accessories,
    Swimwear,
    Activewear,
    Formalwear,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrderStatus {
    Pending,
    PaymentConfirmed,
    Picking,
    Packing,
    Shipped { tracking_id: String },
    Delivered,
    Cancelled { reason: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReturnReason {
    WrongSize,
    DefectiveItem,
    NotAsDescribed,
    ChangedMind,
    LateDelivery,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReturnResolution {
    Refund {
        amount_cents: u64,
    },
    Exchange {
        new_sku: String,
    },
    StoreCredit {
        credit_cents: u64,
        expiry_epoch: u64,
    },
    Rejected {
        explanation: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LoyaltyTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
    Resort,
    PreFall,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExperimentVariant {
    Control,
    VariantA,
    VariantB,
    VariantC,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FixtureType {
    WallDisplay,
    FloorRack,
    Table,
    Mannequin,
    EndCap,
    Window,
}

// ---------------------------------------------------------------------------
// Domain structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FabricComposition {
    fiber_name: String,
    percentage_x10: u16, // 555 = 55.5%
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductCatalogEntry {
    sku: String,
    name: String,
    category: ProductCategory,
    colors: Vec<Color>,
    sizes: Vec<GarmentSize>,
    materials: Vec<FabricComposition>,
    price_cents: u64,
    weight_grams: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WarehouseStock {
    warehouse_id: u32,
    warehouse_name: String,
    region: String,
    sku: String,
    quantity_on_hand: u32,
    quantity_reserved: u32,
    reorder_point: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StoreInventory {
    store_id: u32,
    store_name: String,
    city: String,
    items: Vec<StoreInventoryLine>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StoreInventoryLine {
    sku: String,
    size: GarmentSize,
    color: Color,
    quantity: u32,
    display_quantity: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CartItem {
    sku: String,
    name: String,
    size: GarmentSize,
    color: Color,
    quantity: u32,
    unit_price_cents: u64,
    discount_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ShoppingCartSnapshot {
    cart_id: String,
    customer_id: u64,
    items: Vec<CartItem>,
    coupon_code: Option<String>,
    created_epoch: u64,
    last_modified_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FulfillmentStep {
    step_name: String,
    started_epoch: Option<u64>,
    completed_epoch: Option<u64>,
    assignee: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderFulfillment {
    order_id: u64,
    status: OrderStatus,
    steps: Vec<FulfillmentStep>,
    shipping_carrier: String,
    estimated_delivery_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReturnRequest {
    return_id: u64,
    order_id: u64,
    sku: String,
    reason: ReturnReason,
    resolution: ReturnResolution,
    initiated_epoch: u64,
    resolved_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SizeRecommendation {
    customer_id: u64,
    category: ProductCategory,
    recommended_size: GarmentSize,
    confidence_pct_x10: u16, // 875 = 87.5%
    body_measurements_cm: Vec<u16>,
    past_purchase_skus: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StyleCompatibilityEntry {
    sku_a: String,
    sku_b: String,
    compatibility_score_x100: u16, // 0–10000
    pairing_category: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeasonalCollection {
    collection_id: u32,
    name: String,
    season: Season,
    year: u16,
    designer: String,
    skus: Vec<String>,
    launch_epoch: u64,
    end_epoch: u64,
    theme_keywords: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PriceMarkdown {
    sku: String,
    original_price_cents: u64,
    markdown_price_cents: u64,
    markdown_pct_x10: u16,
    start_epoch: u64,
    end_epoch: u64,
    reason: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoyaltyMember {
    member_id: u64,
    tier: LoyaltyTier,
    points_balance: u64,
    lifetime_spend_cents: u64,
    tier_expiry_epoch: u64,
    preferred_categories: Vec<ProductCategory>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CustomerSegment {
    segment_id: u32,
    name: String,
    description: String,
    member_count: u64,
    avg_order_value_cents: u64,
    avg_orders_per_year_x10: u16,
    top_categories: Vec<ProductCategory>,
    age_range: (u8, u8),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ABTestExperiment {
    experiment_id: u64,
    name: String,
    hypothesis: String,
    variants: Vec<ExperimentVariant>,
    traffic_pct_x10: u16,
    metric_name: String,
    start_epoch: u64,
    end_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlanogramSlot {
    fixture: FixtureType,
    position_x: u16,
    position_y: u16,
    sku: String,
    face_count: u8,
    priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VisualMerchandisingPlanogram {
    planogram_id: u32,
    store_id: u32,
    zone_name: String,
    slots: Vec<PlanogramSlot>,
    effective_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SupplyChainLeadTime {
    supplier_id: u32,
    supplier_name: String,
    sku: String,
    production_days: u16,
    shipping_days: u16,
    customs_days: u16,
    total_days: u16,
    origin_country: String,
}

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn roundtrip_lz4<T: Encode + Decode<()> + PartialEq + std::fmt::Debug>(
    value: &T,
    label: &str,
) -> Vec<u8> {
    let encoded = encode_to_vec(value).unwrap_or_else(|_| panic!("encode {label}"));
    let compressed =
        compress(&encoded, Compression::Lz4).unwrap_or_else(|_| panic!("compress {label}"));
    let decompressed = decompress(&compressed).unwrap_or_else(|_| panic!("decompress {label}"));
    let (decoded, _): (T, usize) =
        decode_from_slice(&decompressed).unwrap_or_else(|_| panic!("decode {label}"));
    assert_eq!(*value, decoded, "roundtrip mismatch for {label}");
    compressed
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// --- Test 1: Single product catalog entry ---
#[test]
fn test_product_catalog_single_entry() {
    let entry = ProductCatalogEntry {
        sku: "WMN-DRS-FL-0042".to_string(),
        name: "Floral Wrap Dress".to_string(),
        category: ProductCategory::Tops,
        colors: vec![
            Color::Named("Coral".to_string()),
            Color::Pantone {
                code: "16-1546".to_string(),
                season: "SS26".to_string(),
            },
        ],
        sizes: vec![
            GarmentSize::XS,
            GarmentSize::S,
            GarmentSize::M,
            GarmentSize::L,
        ],
        materials: vec![
            FabricComposition {
                fiber_name: "Viscose".to_string(),
                percentage_x10: 700,
            },
            FabricComposition {
                fiber_name: "Polyester".to_string(),
                percentage_x10: 300,
            },
        ],
        price_cents: 8999,
        weight_grams: 280,
    };
    roundtrip_lz4(&entry, "product catalog single");
}

// --- Test 2: Large product catalog with compression ratio ---
#[test]
fn test_product_catalog_bulk_compression_ratio() {
    let catalog: Vec<ProductCatalogEntry> = (0..200)
        .map(|i| ProductCatalogEntry {
            sku: format!("CAT-BULK-{i:05}"),
            name: format!("Generic Garment #{i}"),
            category: if i % 3 == 0 {
                ProductCategory::Tops
            } else if i % 3 == 1 {
                ProductCategory::Bottoms
            } else {
                ProductCategory::Outerwear
            },
            colors: vec![Color::Hex(0x334455 + i as u32)],
            sizes: vec![GarmentSize::M],
            materials: vec![FabricComposition {
                fiber_name: "Cotton".to_string(),
                percentage_x10: 1000,
            }],
            price_cents: 3000 + (i as u64 * 100),
            weight_grams: 200 + i * 5,
        })
        .collect();

    let encoded = encode_to_vec(&catalog).expect("encode bulk catalog");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress bulk catalog");
    let decompressed = decompress(&compressed).expect("decompress bulk catalog");
    let (decoded, _): (Vec<ProductCatalogEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode bulk catalog");

    assert_eq!(catalog, decoded);
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
}

// --- Test 3: Warehouse stock across multiple locations ---
#[test]
fn test_warehouse_stock_multi_location() {
    let stocks: Vec<WarehouseStock> = vec![
        WarehouseStock {
            warehouse_id: 1,
            warehouse_name: "East Coast DC".to_string(),
            region: "US-EAST".to_string(),
            sku: "MEN-JKT-LTH-0010".to_string(),
            quantity_on_hand: 450,
            quantity_reserved: 32,
            reorder_point: 100,
        },
        WarehouseStock {
            warehouse_id: 2,
            warehouse_name: "West Coast DC".to_string(),
            region: "US-WEST".to_string(),
            sku: "MEN-JKT-LTH-0010".to_string(),
            quantity_on_hand: 280,
            quantity_reserved: 15,
            reorder_point: 75,
        },
        WarehouseStock {
            warehouse_id: 3,
            warehouse_name: "EU Central".to_string(),
            region: "EU-CENTRAL".to_string(),
            sku: "MEN-JKT-LTH-0010".to_string(),
            quantity_on_hand: 620,
            quantity_reserved: 88,
            reorder_point: 150,
        },
    ];
    roundtrip_lz4(&stocks, "warehouse stock multi-location");
}

// --- Test 4: Store inventory with mixed sizes and colors ---
#[test]
fn test_store_inventory_mixed() {
    let store = StoreInventory {
        store_id: 501,
        store_name: "Flagship Tokyo Ginza".to_string(),
        city: "Tokyo".to_string(),
        items: vec![
            StoreInventoryLine {
                sku: "WMN-BLS-SLK-0077".to_string(),
                size: GarmentSize::S,
                color: Color::Named("Ivory".to_string()),
                quantity: 12,
                display_quantity: 3,
            },
            StoreInventoryLine {
                sku: "WMN-BLS-SLK-0077".to_string(),
                size: GarmentSize::M,
                color: Color::Hex(0xFFF5EE),
                quantity: 8,
                display_quantity: 2,
            },
            StoreInventoryLine {
                sku: "UNI-SNK-RUN-0200".to_string(),
                size: GarmentSize::Custom {
                    chest_cm: 0,
                    waist_cm: 0,
                    length_cm: 27,
                },
                color: Color::Named("Obsidian".to_string()),
                quantity: 24,
                display_quantity: 6,
            },
        ],
    };
    roundtrip_lz4(&store, "store inventory mixed");
}

// --- Test 5: Shopping cart with coupon ---
#[test]
fn test_shopping_cart_with_coupon() {
    let cart = ShoppingCartSnapshot {
        cart_id: "CART-9a3f-e7b2".to_string(),
        customer_id: 4420019,
        items: vec![
            CartItem {
                sku: "WMN-DRS-MXI-0101".to_string(),
                name: "Maxi Dress Sunset".to_string(),
                size: GarmentSize::L,
                color: Color::Named("Burnt Orange".to_string()),
                quantity: 1,
                unit_price_cents: 12500,
                discount_cents: 0,
            },
            CartItem {
                sku: "ACC-SCF-CSH-0033".to_string(),
                name: "Cashmere Scarf".to_string(),
                size: GarmentSize::M,
                color: Color::Pantone {
                    code: "19-4052".to_string(),
                    season: "AW26".to_string(),
                },
                quantity: 1,
                unit_price_cents: 7800,
                discount_cents: 1560,
            },
        ],
        coupon_code: Some("WINTER20".to_string()),
        created_epoch: 1_740_000_000,
        last_modified_epoch: 1_740_000_120,
    };
    roundtrip_lz4(&cart, "cart with coupon");
}

// --- Test 6: Shopping cart snapshot without coupon ---
#[test]
fn test_shopping_cart_no_coupon() {
    let cart = ShoppingCartSnapshot {
        cart_id: "CART-bb01-1111".to_string(),
        customer_id: 8800055,
        items: vec![CartItem {
            sku: "MEN-PLO-CTN-0005".to_string(),
            name: "Classic Polo Cotton".to_string(),
            size: GarmentSize::XL,
            color: Color::Hex(0x1A1A2E),
            quantity: 3,
            unit_price_cents: 4500,
            discount_cents: 0,
        }],
        coupon_code: None,
        created_epoch: 1_740_100_000,
        last_modified_epoch: 1_740_100_000,
    };
    roundtrip_lz4(&cart, "cart no coupon");
}

// --- Test 7: Order fulfillment pipeline ---
#[test]
fn test_order_fulfillment_pipeline() {
    let order = OrderFulfillment {
        order_id: 77700001,
        status: OrderStatus::Shipped {
            tracking_id: "LZ4TRACK99887766".to_string(),
        },
        steps: vec![
            FulfillmentStep {
                step_name: "Payment Verified".to_string(),
                started_epoch: Some(1_740_200_000),
                completed_epoch: Some(1_740_200_005),
                assignee: "payment-svc".to_string(),
            },
            FulfillmentStep {
                step_name: "Pick".to_string(),
                started_epoch: Some(1_740_200_060),
                completed_epoch: Some(1_740_200_300),
                assignee: "picker-12".to_string(),
            },
            FulfillmentStep {
                step_name: "Pack".to_string(),
                started_epoch: Some(1_740_200_310),
                completed_epoch: Some(1_740_200_500),
                assignee: "packer-07".to_string(),
            },
            FulfillmentStep {
                step_name: "Ship".to_string(),
                started_epoch: Some(1_740_200_600),
                completed_epoch: Some(1_740_200_700),
                assignee: "dock-03".to_string(),
            },
        ],
        shipping_carrier: "DHL Express".to_string(),
        estimated_delivery_epoch: 1_740_400_000,
    };
    roundtrip_lz4(&order, "order fulfillment pipeline");
}

// --- Test 8: Cancelled order fulfillment ---
#[test]
fn test_order_fulfillment_cancelled() {
    let order = OrderFulfillment {
        order_id: 77700002,
        status: OrderStatus::Cancelled {
            reason: "Customer requested cancellation before picking".to_string(),
        },
        steps: vec![FulfillmentStep {
            step_name: "Payment Verified".to_string(),
            started_epoch: Some(1_740_300_000),
            completed_epoch: Some(1_740_300_003),
            assignee: "payment-svc".to_string(),
        }],
        shipping_carrier: String::new(),
        estimated_delivery_epoch: 0,
    };
    roundtrip_lz4(&order, "order cancelled");
}

// --- Test 9: Return/exchange workflow — refund ---
#[test]
fn test_return_refund_workflow() {
    let ret = ReturnRequest {
        return_id: 330001,
        order_id: 77700001,
        sku: "WMN-DRS-MXI-0101".to_string(),
        reason: ReturnReason::WrongSize,
        resolution: ReturnResolution::Refund {
            amount_cents: 12500,
        },
        initiated_epoch: 1_740_500_000,
        resolved_epoch: Some(1_740_600_000),
    };
    roundtrip_lz4(&ret, "return refund");
}

// --- Test 10: Return/exchange workflow — exchange ---
#[test]
fn test_return_exchange_workflow() {
    let ret = ReturnRequest {
        return_id: 330002,
        order_id: 77700001,
        sku: "ACC-SCF-CSH-0033".to_string(),
        reason: ReturnReason::NotAsDescribed,
        resolution: ReturnResolution::Exchange {
            new_sku: "ACC-SCF-CSH-0034".to_string(),
        },
        initiated_epoch: 1_740_500_500,
        resolved_epoch: None,
    };
    roundtrip_lz4(&ret, "return exchange");
}

// --- Test 11: Return with store credit resolution ---
#[test]
fn test_return_store_credit() {
    let ret = ReturnRequest {
        return_id: 330003,
        order_id: 77700010,
        sku: "MEN-PLO-CTN-0005".to_string(),
        reason: ReturnReason::ChangedMind,
        resolution: ReturnResolution::StoreCredit {
            credit_cents: 4500,
            expiry_epoch: 1_772_000_000,
        },
        initiated_epoch: 1_740_700_000,
        resolved_epoch: Some(1_740_710_000),
    };
    roundtrip_lz4(&ret, "return store credit");
}

// --- Test 12: Size recommendation data ---
#[test]
fn test_size_recommendation() {
    let rec = SizeRecommendation {
        customer_id: 4420019,
        category: ProductCategory::Tops,
        recommended_size: GarmentSize::M,
        confidence_pct_x10: 923,
        body_measurements_cm: vec![88, 72, 165],
        past_purchase_skus: vec![
            "WMN-BLS-SLK-0077".to_string(),
            "WMN-DRS-FL-0042".to_string(),
            "WMN-DRS-MXI-0101".to_string(),
        ],
    };
    roundtrip_lz4(&rec, "size recommendation");
}

// --- Test 13: Style compatibility matrix ---
#[test]
fn test_style_compatibility_matrix() {
    let entries: Vec<StyleCompatibilityEntry> = vec![
        StyleCompatibilityEntry {
            sku_a: "WMN-DRS-FL-0042".to_string(),
            sku_b: "ACC-SCF-CSH-0033".to_string(),
            compatibility_score_x100: 8750,
            pairing_category: "dress+accessory".to_string(),
        },
        StyleCompatibilityEntry {
            sku_a: "MEN-PLO-CTN-0005".to_string(),
            sku_b: "MEN-CHN-STR-0015".to_string(),
            compatibility_score_x100: 9200,
            pairing_category: "smart-casual-set".to_string(),
        },
        StyleCompatibilityEntry {
            sku_a: "UNI-SNK-RUN-0200".to_string(),
            sku_b: "UNI-JGR-FLC-0088".to_string(),
            compatibility_score_x100: 9500,
            pairing_category: "athleisure-set".to_string(),
        },
    ];
    roundtrip_lz4(&entries, "style compatibility matrix");
}

// --- Test 14: Seasonal collection metadata ---
#[test]
fn test_seasonal_collection_metadata() {
    let collection = SeasonalCollection {
        collection_id: 2026_01,
        name: "Aurora Borealis SS26".to_string(),
        season: Season::Spring,
        year: 2026,
        designer: "Isabelle Monet".to_string(),
        skus: vec![
            "WMN-DRS-FL-0042".to_string(),
            "WMN-BLS-SLK-0077".to_string(),
            "WMN-SKT-PLT-0060".to_string(),
            "ACC-BAG-TTE-0011".to_string(),
        ],
        launch_epoch: 1_740_000_000,
        end_epoch: 1_755_000_000,
        theme_keywords: vec![
            "pastel".to_string(),
            "ethereal".to_string(),
            "nature-inspired".to_string(),
            "flowing-silhouettes".to_string(),
        ],
    };
    roundtrip_lz4(&collection, "seasonal collection");
}

// --- Test 15: Price markdowns batch with compression ratio ---
#[test]
fn test_price_markdowns_compression_ratio() {
    let markdowns: Vec<PriceMarkdown> = (0..150)
        .map(|i| PriceMarkdown {
            sku: format!("MD-ITEM-{i:04}"),
            original_price_cents: 10000 + i as u64 * 50,
            markdown_price_cents: 7000 + i as u64 * 30,
            markdown_pct_x10: 300,
            start_epoch: 1_741_000_000,
            end_epoch: 1_742_000_000,
            reason: "End of season clearance".to_string(),
        })
        .collect();

    let encoded = encode_to_vec(&markdowns).expect("encode markdowns");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress markdowns");
    let decompressed = decompress(&compressed).expect("decompress markdowns");
    let (decoded, _): (Vec<PriceMarkdown>, usize) =
        decode_from_slice(&decompressed).expect("decode markdowns");

    assert_eq!(markdowns, decoded);
    assert!(
        compressed.len() < encoded.len(),
        "markdowns compressed ({}) should be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );
}

// --- Test 16: Loyalty program tiers ---
#[test]
fn test_loyalty_program_members() {
    let members = vec![
        LoyaltyMember {
            member_id: 1001,
            tier: LoyaltyTier::Bronze,
            points_balance: 1200,
            lifetime_spend_cents: 45_000,
            tier_expiry_epoch: 1_772_000_000,
            preferred_categories: vec![ProductCategory::Tops, ProductCategory::Accessories],
        },
        LoyaltyMember {
            member_id: 1002,
            tier: LoyaltyTier::Gold,
            points_balance: 48_500,
            lifetime_spend_cents: 890_000,
            tier_expiry_epoch: 1_772_000_000,
            preferred_categories: vec![
                ProductCategory::Formalwear,
                ProductCategory::Outerwear,
                ProductCategory::Footwear,
            ],
        },
        LoyaltyMember {
            member_id: 1003,
            tier: LoyaltyTier::Diamond,
            points_balance: 312_000,
            lifetime_spend_cents: 5_600_000,
            tier_expiry_epoch: 1_780_000_000,
            preferred_categories: vec![ProductCategory::Formalwear, ProductCategory::Accessories],
        },
    ];
    roundtrip_lz4(&members, "loyalty members");
}

// --- Test 17: Customer segmentation profiles ---
#[test]
fn test_customer_segmentation_profiles() {
    let segments = vec![
        CustomerSegment {
            segment_id: 1,
            name: "Trend-Forward Millennials".to_string(),
            description: "Fashion-conscious 28-38 year-olds with high purchase frequency"
                .to_string(),
            member_count: 185_000,
            avg_order_value_cents: 15_200,
            avg_orders_per_year_x10: 82,
            top_categories: vec![
                ProductCategory::Tops,
                ProductCategory::Activewear,
                ProductCategory::Accessories,
            ],
            age_range: (28, 38),
        },
        CustomerSegment {
            segment_id: 2,
            name: "Premium Essentials Shoppers".to_string(),
            description: "Quality-focused buyers who prefer timeless pieces".to_string(),
            member_count: 92_000,
            avg_order_value_cents: 28_500,
            avg_orders_per_year_x10: 35,
            top_categories: vec![ProductCategory::Outerwear, ProductCategory::Formalwear],
            age_range: (35, 55),
        },
    ];
    roundtrip_lz4(&segments, "customer segments");
}

// --- Test 18: A/B test experiment config ---
#[test]
fn test_ab_experiment_config() {
    let experiment = ABTestExperiment {
        experiment_id: 50001,
        name: "PDP Image Carousel vs Grid".to_string(),
        hypothesis: "Grid layout increases add-to-cart rate by 5%".to_string(),
        variants: vec![
            ExperimentVariant::Control,
            ExperimentVariant::VariantA,
            ExperimentVariant::VariantB,
        ],
        traffic_pct_x10: 500,
        metric_name: "add_to_cart_rate".to_string(),
        start_epoch: 1_741_500_000,
        end_epoch: Some(1_742_500_000),
    };
    roundtrip_lz4(&experiment, "AB experiment");
}

// --- Test 19: Visual merchandising planogram ---
#[test]
fn test_visual_merchandising_planogram() {
    let planogram = VisualMerchandisingPlanogram {
        planogram_id: 8001,
        store_id: 501,
        zone_name: "Main Entrance - Spring Feature".to_string(),
        slots: vec![
            PlanogramSlot {
                fixture: FixtureType::Window,
                position_x: 0,
                position_y: 0,
                sku: "WMN-DRS-FL-0042".to_string(),
                face_count: 2,
                priority: 1,
            },
            PlanogramSlot {
                fixture: FixtureType::Mannequin,
                position_x: 1,
                position_y: 0,
                sku: "WMN-BLS-SLK-0077".to_string(),
                face_count: 1,
                priority: 2,
            },
            PlanogramSlot {
                fixture: FixtureType::Table,
                position_x: 2,
                position_y: 0,
                sku: "ACC-SCF-CSH-0033".to_string(),
                face_count: 4,
                priority: 3,
            },
            PlanogramSlot {
                fixture: FixtureType::FloorRack,
                position_x: 3,
                position_y: 0,
                sku: "UNI-SNK-RUN-0200".to_string(),
                face_count: 6,
                priority: 4,
            },
            PlanogramSlot {
                fixture: FixtureType::EndCap,
                position_x: 4,
                position_y: 1,
                sku: "ACC-BAG-TTE-0011".to_string(),
                face_count: 3,
                priority: 5,
            },
        ],
        effective_epoch: 1_741_000_000,
    };
    roundtrip_lz4(&planogram, "planogram");
}

// --- Test 20: Supply chain lead times ---
#[test]
fn test_supply_chain_lead_times() {
    let lead_times = vec![
        SupplyChainLeadTime {
            supplier_id: 201,
            supplier_name: "Tessuti Milano Srl".to_string(),
            sku: "FAB-SLK-IT-001".to_string(),
            production_days: 21,
            shipping_days: 14,
            customs_days: 3,
            total_days: 38,
            origin_country: "Italy".to_string(),
        },
        SupplyChainLeadTime {
            supplier_id: 202,
            supplier_name: "Guangzhou Textiles Co".to_string(),
            sku: "FAB-CTN-CN-045".to_string(),
            production_days: 14,
            shipping_days: 28,
            customs_days: 5,
            total_days: 47,
            origin_country: "China".to_string(),
        },
        SupplyChainLeadTime {
            supplier_id: 203,
            supplier_name: "Istanbul Denim Works".to_string(),
            sku: "FAB-DNM-TR-012".to_string(),
            production_days: 18,
            shipping_days: 10,
            customs_days: 2,
            total_days: 30,
            origin_country: "Turkey".to_string(),
        },
    ];
    roundtrip_lz4(&lead_times, "supply chain lead times");
}

// --- Test 21: Fabric composition specs ---
#[test]
fn test_fabric_composition_specs() {
    let fabrics: Vec<Vec<FabricComposition>> = vec![
        vec![
            FabricComposition {
                fiber_name: "Organic Cotton".to_string(),
                percentage_x10: 600,
            },
            FabricComposition {
                fiber_name: "Recycled Polyester".to_string(),
                percentage_x10: 350,
            },
            FabricComposition {
                fiber_name: "Elastane".to_string(),
                percentage_x10: 50,
            },
        ],
        vec![FabricComposition {
            fiber_name: "Mulberry Silk".to_string(),
            percentage_x10: 1000,
        }],
        vec![
            FabricComposition {
                fiber_name: "Merino Wool".to_string(),
                percentage_x10: 800,
            },
            FabricComposition {
                fiber_name: "Cashmere".to_string(),
                percentage_x10: 200,
            },
        ],
    ];
    roundtrip_lz4(&fabrics, "fabric compositions");
}

// --- Test 22: End-to-end fashion retail scenario with compression ratio ---
#[test]
fn test_end_to_end_fashion_retail_scenario() {
    // Build a rich combined payload representing a day's operations snapshot
    let catalog_entries: Vec<ProductCatalogEntry> = (0..50)
        .map(|i| ProductCatalogEntry {
            sku: format!("E2E-SKU-{i:04}"),
            name: format!("E2E Product {i}"),
            category: match i % 5 {
                0 => ProductCategory::Tops,
                1 => ProductCategory::Bottoms,
                2 => ProductCategory::Footwear,
                3 => ProductCategory::Accessories,
                _ => ProductCategory::Activewear,
            },
            colors: vec![Color::Hex(0xAA0000 + i as u32 * 0x111)],
            sizes: vec![GarmentSize::S, GarmentSize::M, GarmentSize::L],
            materials: vec![FabricComposition {
                fiber_name: "Cotton Blend".to_string(),
                percentage_x10: 1000,
            }],
            price_cents: 5000 + i as u64 * 200,
            weight_grams: 150 + i * 10,
        })
        .collect();

    let carts: Vec<ShoppingCartSnapshot> = (0..20)
        .map(|i| ShoppingCartSnapshot {
            cart_id: format!("E2E-CART-{i:04}"),
            customer_id: 9_000_000 + i as u64,
            items: vec![CartItem {
                sku: format!("E2E-SKU-{:04}", i % 50),
                name: format!("E2E Product {}", i % 50),
                size: GarmentSize::M,
                color: Color::Hex(0xBBCCDD),
                quantity: 1 + (i % 3),
                unit_price_cents: 5000 + (i as u64 % 50) * 200,
                discount_cents: 0,
            }],
            coupon_code: if i % 4 == 0 {
                Some("SAVE10".to_string())
            } else {
                None
            },
            created_epoch: 1_742_000_000 + i as u64 * 60,
            last_modified_epoch: 1_742_000_000 + i as u64 * 60,
        })
        .collect();

    let returns: Vec<ReturnRequest> = (0..10)
        .map(|i| ReturnRequest {
            return_id: 900_000 + i as u64,
            order_id: 77_700_000 + i as u64,
            sku: format!("E2E-SKU-{:04}", i),
            reason: if i % 2 == 0 {
                ReturnReason::WrongSize
            } else {
                ReturnReason::ChangedMind
            },
            resolution: ReturnResolution::Refund {
                amount_cents: 5000 + i as u64 * 200,
            },
            initiated_epoch: 1_742_100_000 + i as u64 * 3600,
            resolved_epoch: Some(1_742_200_000 + i as u64 * 3600),
        })
        .collect();

    // Combine into a single tuple payload
    let payload = (catalog_entries.clone(), carts.clone(), returns.clone());

    let encoded = encode_to_vec(&payload).expect("encode e2e scenario");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress e2e scenario");
    let decompressed = decompress(&compressed).expect("decompress e2e scenario");
    let (decoded, _): (
        (
            Vec<ProductCatalogEntry>,
            Vec<ShoppingCartSnapshot>,
            Vec<ReturnRequest>,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode e2e scenario");

    assert_eq!(payload, decoded);
    assert!(
        compressed.len() < encoded.len(),
        "e2e compressed ({}) should be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );
}
