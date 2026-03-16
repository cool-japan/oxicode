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

// --- Domain types ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProductCatalogItem {
    sku: String,
    name: String,
    brand: String,
    base_price_cents: u64,
    category: String,
    is_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SizeVariant {
    sku: String,
    size_label: String,
    size_numeric: Option<f32>,
    stock_qty: u32,
    fit_type: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ColorPalette {
    palette_id: u32,
    season: String,
    colors: Vec<String>,
    hex_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SeasonalCollection {
    collection_id: u64,
    name: String,
    season: String,
    year: u16,
    item_count: u32,
    featured: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InventoryLevel {
    warehouse_id: u32,
    sku: String,
    quantity_on_hand: u32,
    quantity_reserved: u32,
    reorder_threshold: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SkuManifest {
    sku: String,
    product_id: u64,
    color: String,
    size: String,
    barcode: String,
    weight_grams: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TrendForecast {
    trend_id: u32,
    label: String,
    confidence_pct: f32,
    predicted_units: u64,
    target_demographic: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InfluencerCollaboration {
    collab_id: u64,
    influencer_handle: String,
    platform: String,
    commission_bps: u32,
    active: bool,
    promo_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VirtualTryOnSession {
    session_id: String,
    user_id: u64,
    product_sku: String,
    duration_ms: u32,
    converted: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LoyaltyProgramPoints {
    member_id: u64,
    tier: String,
    points_balance: u32,
    lifetime_points: u64,
    expiry_year: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ReturnReason {
    WrongSize,
    DefectiveItem,
    ChangedMind,
    NotAsDescribed,
    LateDelivery,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReturnRefundRecord {
    order_id: u64,
    sku: String,
    reason: ReturnReason,
    refund_amount_cents: u64,
    restocked: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SustainableFashionScore {
    product_id: u64,
    material_score: f32,
    supply_chain_score: f32,
    carbon_footprint_kg: f32,
    certifications: Vec<String>,
    overall_grade: String,
}

// --- Tests ---

#[test]
fn test_product_catalog_item_roundtrip() {
    let cfg = config::standard();
    let item = ProductCatalogItem {
        sku: "SKU-001-BLK-M".to_string(),
        name: "Classic Slim-Fit Chino".to_string(),
        brand: "UrbanThread".to_string(),
        base_price_cents: 8999,
        category: "bottoms".to_string(),
        is_active: true,
    };
    let bytes = encode_to_vec(&item, cfg).expect("encode ProductCatalogItem");
    let (decoded, _): (ProductCatalogItem, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ProductCatalogItem");
    assert_eq!(item, decoded);
}

#[test]
fn test_size_variant_with_none_numeric_roundtrip() {
    let cfg = config::standard();
    let variant = SizeVariant {
        sku: "SKU-002-WHT-OS".to_string(),
        size_label: "One Size".to_string(),
        size_numeric: None,
        stock_qty: 120,
        fit_type: "relaxed".to_string(),
    };
    let bytes = encode_to_vec(&variant, cfg).expect("encode SizeVariant None");
    let (decoded, _): (SizeVariant, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SizeVariant None");
    assert_eq!(variant, decoded);
}

#[test]
fn test_size_variant_with_numeric_roundtrip() {
    let cfg = config::standard();
    let variant = SizeVariant {
        sku: "SKU-003-NAV-32".to_string(),
        size_label: "32".to_string(),
        size_numeric: Some(32.0),
        stock_qty: 45,
        fit_type: "slim".to_string(),
    };
    let bytes = encode_to_vec(&variant, cfg).expect("encode SizeVariant Some");
    let (decoded, _): (SizeVariant, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SizeVariant Some");
    assert_eq!(variant, decoded);
}

#[test]
fn test_color_palette_roundtrip() {
    let cfg = config::standard();
    let palette = ColorPalette {
        palette_id: 77,
        season: "Spring/Summer 2026".to_string(),
        colors: vec![
            "Sage Green".to_string(),
            "Dusty Rose".to_string(),
            "Ivory".to_string(),
        ],
        hex_codes: vec![
            "#8FBC8F".to_string(),
            "#DCAE96".to_string(),
            "#FFFFF0".to_string(),
        ],
    };
    let bytes = encode_to_vec(&palette, cfg).expect("encode ColorPalette");
    let (decoded, _): (ColorPalette, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ColorPalette");
    assert_eq!(palette, decoded);
}

#[test]
fn test_seasonal_collection_featured_roundtrip() {
    let cfg = config::standard();
    let collection = SeasonalCollection {
        collection_id: 20260101,
        name: "Riviera Escape".to_string(),
        season: "SS".to_string(),
        year: 2026,
        item_count: 84,
        featured: true,
    };
    let bytes = encode_to_vec(&collection, cfg).expect("encode SeasonalCollection");
    let (decoded, _): (SeasonalCollection, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SeasonalCollection");
    assert_eq!(collection, decoded);
}

#[test]
fn test_inventory_level_roundtrip() {
    let cfg = config::standard();
    let inv = InventoryLevel {
        warehouse_id: 3,
        sku: "SKU-004-GRY-L".to_string(),
        quantity_on_hand: 500,
        quantity_reserved: 37,
        reorder_threshold: 50,
    };
    let bytes = encode_to_vec(&inv, cfg).expect("encode InventoryLevel");
    let (decoded, _): (InventoryLevel, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InventoryLevel");
    assert_eq!(inv, decoded);
}

#[test]
fn test_sku_manifest_roundtrip_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let manifest = SkuManifest {
        sku: "SKU-005-RED-S".to_string(),
        product_id: 98765,
        color: "Crimson Red".to_string(),
        size: "S".to_string(),
        barcode: "036000291452".to_string(),
        weight_grams: 312,
    };
    let bytes = encode_to_vec(&manifest, cfg).expect("encode SkuManifest fixed int");
    let (decoded, _): (SkuManifest, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SkuManifest fixed int");
    assert_eq!(manifest, decoded);
}

#[test]
fn test_trend_forecast_roundtrip() {
    let cfg = config::standard();
    let forecast = TrendForecast {
        trend_id: 42,
        label: "Cottagecore Revival".to_string(),
        confidence_pct: 87.5,
        predicted_units: 15_000,
        target_demographic: "18-34 female".to_string(),
    };
    let bytes = encode_to_vec(&forecast, cfg).expect("encode TrendForecast");
    let (decoded, _): (TrendForecast, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TrendForecast");
    assert_eq!(forecast, decoded);
}

#[test]
fn test_influencer_collaboration_roundtrip() {
    let cfg = config::standard();
    let collab = InfluencerCollaboration {
        collab_id: 9001,
        influencer_handle: "@mia_styles".to_string(),
        platform: "instagram".to_string(),
        commission_bps: 800,
        active: true,
        promo_codes: vec!["MIA10".to_string(), "MIAFALL".to_string()],
    };
    let bytes = encode_to_vec(&collab, cfg).expect("encode InfluencerCollaboration");
    let (decoded, _): (InfluencerCollaboration, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InfluencerCollaboration");
    assert_eq!(collab, decoded);
}

#[test]
fn test_virtual_try_on_session_converted_roundtrip() {
    let cfg = config::standard();
    let session = VirtualTryOnSession {
        session_id: "sess-uuid-aabbccdd".to_string(),
        user_id: 55_001,
        product_sku: "SKU-006-BLU-M".to_string(),
        duration_ms: 4_200,
        converted: true,
    };
    let bytes = encode_to_vec(&session, cfg).expect("encode VirtualTryOnSession converted");
    let (decoded, _): (VirtualTryOnSession, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VirtualTryOnSession converted");
    assert_eq!(session, decoded);
}

#[test]
fn test_virtual_try_on_session_not_converted_roundtrip() {
    let cfg = config::standard();
    let session = VirtualTryOnSession {
        session_id: "sess-uuid-eeff0011".to_string(),
        user_id: 55_002,
        product_sku: "SKU-007-PNK-XS".to_string(),
        duration_ms: 800,
        converted: false,
    };
    let bytes = encode_to_vec(&session, cfg).expect("encode VirtualTryOnSession not converted");
    let (decoded, _): (VirtualTryOnSession, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VirtualTryOnSession not converted");
    assert_eq!(session, decoded);
}

#[test]
fn test_loyalty_program_points_gold_tier_roundtrip() {
    let cfg = config::standard();
    let loyalty = LoyaltyProgramPoints {
        member_id: 1_000_001,
        tier: "Gold".to_string(),
        points_balance: 25_400,
        lifetime_points: 182_000,
        expiry_year: 2027,
    };
    let bytes = encode_to_vec(&loyalty, cfg).expect("encode LoyaltyProgramPoints gold");
    let (decoded, _): (LoyaltyProgramPoints, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode LoyaltyProgramPoints gold");
    assert_eq!(loyalty, decoded);
}

#[test]
fn test_return_refund_wrong_size_roundtrip() {
    let cfg = config::standard();
    let record = ReturnRefundRecord {
        order_id: 7_654_321,
        sku: "SKU-008-GRN-XL".to_string(),
        reason: ReturnReason::WrongSize,
        refund_amount_cents: 5999,
        restocked: true,
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode ReturnRefundRecord WrongSize");
    let (decoded, _): (ReturnRefundRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ReturnRefundRecord WrongSize");
    assert_eq!(record, decoded);
}

#[test]
fn test_return_refund_defective_item_roundtrip() {
    let cfg = config::standard();
    let record = ReturnRefundRecord {
        order_id: 7_654_322,
        sku: "SKU-009-BLK-M".to_string(),
        reason: ReturnReason::DefectiveItem,
        refund_amount_cents: 12_499,
        restocked: false,
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode ReturnRefundRecord DefectiveItem");
    let (decoded, _): (ReturnRefundRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ReturnRefundRecord DefectiveItem");
    assert_eq!(record, decoded);
}

#[test]
fn test_sustainable_fashion_score_roundtrip() {
    let cfg = config::standard();
    let score = SustainableFashionScore {
        product_id: 88_001,
        material_score: 9.1,
        supply_chain_score: 7.8,
        carbon_footprint_kg: 2.35,
        certifications: vec![
            "GOTS".to_string(),
            "Fair Trade".to_string(),
            "OEKO-TEX".to_string(),
        ],
        overall_grade: "A".to_string(),
    };
    let bytes = encode_to_vec(&score, cfg).expect("encode SustainableFashionScore");
    let (decoded, _): (SustainableFashionScore, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SustainableFashionScore");
    assert_eq!(score, decoded);
}

#[test]
fn test_vec_of_catalog_items_roundtrip() {
    let cfg = config::standard();
    let items = vec![
        ProductCatalogItem {
            sku: "SKU-010-WHT-S".to_string(),
            name: "Linen Wrap Dress".to_string(),
            brand: "SoftStitch".to_string(),
            base_price_cents: 11_999,
            category: "dresses".to_string(),
            is_active: true,
        },
        ProductCatalogItem {
            sku: "SKU-011-TAN-M".to_string(),
            name: "Cargo Utility Jacket".to_string(),
            brand: "FieldCraft".to_string(),
            base_price_cents: 18_499,
            category: "outerwear".to_string(),
            is_active: false,
        },
    ];
    let bytes = encode_to_vec(&items, cfg).expect("encode Vec<ProductCatalogItem>");
    let (decoded, _): (Vec<ProductCatalogItem>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ProductCatalogItem>");
    assert_eq!(items, decoded);
}

#[test]
fn test_empty_promo_codes_collab_roundtrip() {
    let cfg = config::standard();
    let collab = InfluencerCollaboration {
        collab_id: 9002,
        influencer_handle: "@taro_mode".to_string(),
        platform: "tiktok".to_string(),
        commission_bps: 500,
        active: false,
        promo_codes: vec![],
    };
    let bytes = encode_to_vec(&collab, cfg).expect("encode InfluencerCollaboration empty codes");
    let (decoded, _): (InfluencerCollaboration, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InfluencerCollaboration empty codes");
    assert_eq!(collab, decoded);
}

#[test]
fn test_trend_forecast_big_endian_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let forecast = TrendForecast {
        trend_id: 99,
        label: "Y2K Aesthetic".to_string(),
        confidence_pct: 72.3,
        predicted_units: 42_000,
        target_demographic: "16-28 all genders".to_string(),
    };
    let bytes = encode_to_vec(&forecast, cfg).expect("encode TrendForecast big endian");
    let (decoded, _): (TrendForecast, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TrendForecast big endian");
    assert_eq!(forecast, decoded);
}

#[test]
fn test_inventory_level_zero_reserved_roundtrip() {
    let cfg = config::standard();
    let inv = InventoryLevel {
        warehouse_id: 1,
        sku: "SKU-012-BLK-XXL".to_string(),
        quantity_on_hand: 0,
        quantity_reserved: 0,
        reorder_threshold: 10,
    };
    let bytes = encode_to_vec(&inv, cfg).expect("encode InventoryLevel zero");
    let (decoded, _): (InventoryLevel, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InventoryLevel zero");
    assert_eq!(inv, decoded);
}

#[test]
fn test_sustainable_score_no_certifications_roundtrip() {
    let cfg = config::standard();
    let score = SustainableFashionScore {
        product_id: 88_002,
        material_score: 3.2,
        supply_chain_score: 4.0,
        carbon_footprint_kg: 8.9,
        certifications: vec![],
        overall_grade: "D".to_string(),
    };
    let bytes = encode_to_vec(&score, cfg).expect("encode SustainableFashionScore no certs");
    let (decoded, _): (SustainableFashionScore, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SustainableFashionScore no certs");
    assert_eq!(score, decoded);
}

#[test]
fn test_loyalty_points_consumed_bytes_equals_len() {
    let cfg = config::standard();
    let loyalty = LoyaltyProgramPoints {
        member_id: 2_000_001,
        tier: "Silver".to_string(),
        points_balance: 8_300,
        lifetime_points: 56_700,
        expiry_year: 2026,
    };
    let bytes = encode_to_vec(&loyalty, cfg).expect("encode LoyaltyProgramPoints for size check");
    let (_decoded, consumed): (LoyaltyProgramPoints, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode LoyaltyProgramPoints for size check");
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_return_reason_all_variants_roundtrip() {
    let cfg = config::standard();
    let reasons = vec![
        ReturnReason::WrongSize,
        ReturnReason::DefectiveItem,
        ReturnReason::ChangedMind,
        ReturnReason::NotAsDescribed,
        ReturnReason::LateDelivery,
    ];
    let bytes = encode_to_vec(&reasons, cfg).expect("encode Vec<ReturnReason>");
    let (decoded, _): (Vec<ReturnReason>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ReturnReason>");
    assert_eq!(reasons, decoded);
}
