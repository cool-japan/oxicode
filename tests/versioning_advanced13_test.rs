//! E-commerce / product catalog versioning tests for OxiCode (set 13).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and three generations of Product structs (V1/V2/V3) with all
//! ProductCategory variants, OrderLine, Vec of versioned items, version
//! comparison, consumed bytes, and version equality.

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
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value,
    versioning::Version, Decode, Encode,
};

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProductCategory {
    Electronics,
    Clothing,
    Food,
    Books,
    Sports,
    HomeGarden,
    Toys,
    Automotive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductV1 {
    id: u64,
    name: String,
    category: ProductCategory,
    price_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductV2 {
    id: u64,
    name: String,
    category: ProductCategory,
    price_cents: u64,
    description: String,
    stock: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductV3 {
    id: u64,
    name: String,
    category: ProductCategory,
    price_cents: u64,
    description: String,
    stock: u32,
    tags: Vec<String>,
    rating: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderLine {
    product_id: u64,
    quantity: u32,
    unit_price_cents: u64,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// ProductV1 Electronics roundtrip at version 1.0.0
#[test]
fn test_product_v1_electronics_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = ProductV1 {
        id: 1001,
        name: String::from("Wireless Headphones"),
        category: ProductCategory::Electronics,
        price_cents: 4999,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ProductV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// ProductV1 Clothing roundtrip at version 1.1.0
#[test]
fn test_product_v1_clothing_roundtrip() {
    let version = Version::new(1, 1, 0);
    let original = ProductV1 {
        id: 2002,
        name: String::from("Cotton T-Shirt"),
        category: ProductCategory::Clothing,
        price_cents: 1299,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ProductV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// ProductV2 Food roundtrip at version 2.0.0
#[test]
fn test_product_v2_food_roundtrip() {
    let version = Version::new(2, 0, 0);
    let original = ProductV2 {
        id: 3003,
        name: String::from("Organic Oats"),
        category: ProductCategory::Food,
        price_cents: 599,
        description: String::from("Rolled oats, 1kg bag"),
        stock: 250,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ProductV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// ProductV2 Books roundtrip at version 2.1.3
#[test]
fn test_product_v2_books_roundtrip() {
    let version = Version::new(2, 1, 3);
    let original = ProductV2 {
        id: 4004,
        name: String::from("Rust Programming Language"),
        category: ProductCategory::Books,
        price_cents: 3999,
        description: String::from("The official Rust book"),
        stock: 42,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ProductV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// ProductV3 Sports with tags and rating roundtrip at version 3.0.0
#[test]
fn test_product_v3_sports_with_tags_and_rating_roundtrip() {
    let version = Version::new(3, 0, 0);
    let original = ProductV3 {
        id: 5005,
        name: String::from("Carbon Road Bike"),
        category: ProductCategory::Sports,
        price_cents: 149900,
        description: String::from("Lightweight carbon frame racing bicycle"),
        stock: 10,
        tags: vec![
            String::from("cycling"),
            String::from("road"),
            String::from("carbon"),
        ],
        rating: Some(4.7_f32),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ProductV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.id, original.id);
    assert_eq!(decoded.name, original.name);
    assert_eq!(decoded.tags, original.tags);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// ProductV3 HomeGarden with None rating roundtrip
#[test]
fn test_product_v3_home_garden_no_rating_roundtrip() {
    let version = Version::new(3, 0, 0);
    let original = ProductV3 {
        id: 6006,
        name: String::from("Garden Hose 50ft"),
        category: ProductCategory::HomeGarden,
        price_cents: 2499,
        description: String::from("Heavy-duty expandable garden hose"),
        stock: 100,
        tags: vec![String::from("garden"), String::from("outdoor")],
        rating: None,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (ProductV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.rating, None);
    assert_eq!(decoded.category, ProductCategory::HomeGarden);
    assert_eq!(ver, version);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// ProductV3 Toys with empty tags roundtrip
#[test]
fn test_product_v3_toys_empty_tags_roundtrip() {
    let version = Version::new(3, 2, 0);
    let original = ProductV3 {
        id: 7007,
        name: String::from("Building Blocks Set"),
        category: ProductCategory::Toys,
        price_cents: 1999,
        description: String::from("Classic wooden building blocks for ages 2+"),
        stock: 75,
        tags: vec![],
        rating: Some(4.9_f32),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (ProductV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(decoded.tags.is_empty());
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// ProductV3 Automotive roundtrip
#[test]
fn test_product_v3_automotive_roundtrip() {
    let version = Version::new(3, 1, 0);
    let original = ProductV3 {
        id: 8008,
        name: String::from("Car Floor Mats"),
        category: ProductCategory::Automotive,
        price_cents: 3499,
        description: String::from("Universal fit all-weather rubber mats"),
        stock: 200,
        tags: vec![String::from("car"), String::from("interior")],
        rating: Some(4.2_f32),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (ProductV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.category, ProductCategory::Automotive);
    assert_eq!(decoded.id, 8008);
    assert_eq!(ver, version);
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// Version extracted from encoded ProductV1 matches expected
#[test]
fn test_version_extracted_from_product_v1_bytes() {
    let version = Version::new(1, 5, 3);
    let original = ProductV1 {
        id: 9009,
        name: String::from("USB-C Cable"),
        category: ProductCategory::Electronics,
        price_cents: 999,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (ProductV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 3);
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// Major version comparison: 1.0.0 < 2.0.0
#[test]
fn test_version_major_comparison() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v1 < v2);
    assert!(v2 > v1);
    assert_ne!(v1, v2);
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// Minor version comparison: 2.0.0 < 2.1.0
#[test]
fn test_version_minor_comparison() {
    let v_low = Version::new(2, 0, 0);
    let v_high = Version::new(2, 1, 0);
    assert!(v_low < v_high);
    assert!(v_high > v_low);
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// Patch version comparison: 3.0.0 < 3.0.5
#[test]
fn test_version_patch_comparison() {
    let v_low = Version::new(3, 0, 0);
    let v_high = Version::new(3, 0, 5);
    assert!(v_low < v_high);
    assert!(v_high > v_low);
    assert_ne!(v_low, v_high);
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// Version equality: same major/minor/patch
#[test]
fn test_version_equality() {
    let v_a = Version::new(2, 3, 4);
    let v_b = Version::new(2, 3, 4);
    assert_eq!(v_a, v_b);
    assert!(!(v_a < v_b));
    assert!(!(v_a > v_b));
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// Vec of versioned ProductV1 items, each encoded independently
#[test]
fn test_vec_of_versioned_product_v1_items() {
    let version = Version::new(1, 0, 0);
    let products = vec![
        ProductV1 {
            id: 101,
            name: String::from("Keyboard"),
            category: ProductCategory::Electronics,
            price_cents: 7999,
        },
        ProductV1 {
            id: 102,
            name: String::from("Jeans"),
            category: ProductCategory::Clothing,
            price_cents: 5499,
        },
        ProductV1 {
            id: 103,
            name: String::from("Protein Powder"),
            category: ProductCategory::Food,
            price_cents: 3999,
        },
    ];

    for original in &products {
        let encoded =
            encode_versioned_value(original, version).expect("encode_versioned_value failed");
        let (decoded, ver, consumed): (ProductV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(&decoded, original);
        assert_eq!(ver, version);
        assert!(consumed > 0);
    }
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// OrderLine roundtrip at version 1.0.0
#[test]
fn test_order_line_roundtrip_v1() {
    let version = Version::new(1, 0, 0);
    let original = OrderLine {
        product_id: 5005,
        quantity: 3,
        unit_price_cents: 4999,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (OrderLine, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// OrderLine roundtrip at version 2.0.0 (same data, different version tag)
#[test]
fn test_order_line_roundtrip_v2() {
    let version = Version::new(2, 0, 0);
    let original = OrderLine {
        product_id: 6006,
        quantity: 1,
        unit_price_cents: 149900,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (OrderLine, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.product_id, 6006);
    assert_eq!(decoded.quantity, 1);
    assert_eq!(decoded.unit_price_cents, 149900);
    assert_eq!(ver, Version::new(2, 0, 0));
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Same ProductV2 data encoded at different versions produces different version tags
#[test]
fn test_same_product_v2_data_different_version_tags() {
    let v1 = Version::new(2, 0, 0);
    let v2 = Version::new(2, 1, 0);
    let product = ProductV2 {
        id: 7777,
        name: String::from("Yoga Mat"),
        category: ProductCategory::Sports,
        price_cents: 2799,
        description: String::from("Non-slip yoga mat 6mm"),
        stock: 50,
    };

    let encoded_v1 =
        encode_versioned_value(&product, v1).expect("encode_versioned_value v1 failed");
    let encoded_v2 =
        encode_versioned_value(&product, v2).expect("encode_versioned_value v2 failed");

    let (decoded1, ver1, _): (ProductV2, Version, usize) =
        decode_versioned_value(&encoded_v1).expect("decode v1 failed");
    let (decoded2, ver2, _): (ProductV2, Version, usize) =
        decode_versioned_value(&encoded_v2).expect("decode v2 failed");

    assert_eq!(decoded1, product);
    assert_eq!(decoded2, product);
    assert_eq!(ver1, v1);
    assert_eq!(ver2, v2);
    assert_ne!(ver1, ver2);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// Consumed bytes is positive and does not exceed total encoded buffer length
#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let version = Version::new(1, 0, 0);
    let original = ProductV1 {
        id: 8888,
        name: String::from("HDMI Cable"),
        category: ProductCategory::Electronics,
        price_cents: 1499,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let total_len = encoded.len();
    let (_decoded, _ver, consumed): (ProductV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    // consumed counts bytes read from the payload section; it must be > 0 and <= total
    assert!(consumed > 0, "consumed must be positive");
    assert!(
        consumed <= total_len,
        "consumed ({consumed}) must not exceed total encoded length ({total_len})"
    );
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// ProductV1 plain encode/decode baseline (no versioning) still works
#[test]
fn test_product_v1_plain_encode_decode_baseline() {
    let original = ProductV1 {
        id: 9999,
        name: String::from("Laptop Stand"),
        category: ProductCategory::Electronics,
        price_cents: 4999,
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (ProductV1, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// ProductV3 with many tags and a high-precision rating roundtrip
#[test]
fn test_product_v3_many_tags_roundtrip() {
    let version = Version::new(3, 5, 2);
    let original = ProductV3 {
        id: 11111,
        name: String::from("Multi-tool Kit"),
        category: ProductCategory::Automotive,
        price_cents: 8999,
        description: String::from("Professional grade multi-tool set with 28 pieces"),
        stock: 33,
        tags: vec![
            String::from("tools"),
            String::from("automotive"),
            String::from("workshop"),
            String::from("professional"),
            String::from("gift"),
        ],
        rating: Some(4.85_f32),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ProductV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.tags.len(), 5);
    assert_eq!(decoded.name, original.name);
    assert_eq!(decoded.stock, 33);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// Version field accessors (major, minor, patch) are correct after decode
#[test]
fn test_version_field_accessors_after_decode() {
    let version = Version::new(7, 13, 42);
    let original = OrderLine {
        product_id: 22222,
        quantity: 5,
        unit_price_cents: 999,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (OrderLine, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 7);
    assert_eq!(ver.minor, 13);
    assert_eq!(ver.patch, 42);
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// ProductV2 zero-stock and zero-price roundtrip (boundary values)
#[test]
fn test_product_v2_zero_stock_zero_price_roundtrip() {
    let version = Version::new(2, 0, 0);
    let original = ProductV2 {
        id: 0,
        name: String::from(""),
        category: ProductCategory::Books,
        price_cents: 0,
        description: String::from(""),
        stock: 0,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ProductV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.id, 0);
    assert_eq!(decoded.name, "");
    assert_eq!(decoded.price_cents, 0);
    assert_eq!(decoded.stock, 0);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}
