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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ProductCategory {
    Tops,
    Bottoms,
    Dresses,
    Outerwear,
    Footwear,
    Accessories,
    Sportswear,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SizeSystem {
    Xs,
    S,
    M,
    L,
    Xl,
    Xxl,
    Numeric(u8),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FabricType {
    Cotton,
    Polyester,
    Wool,
    Silk,
    Linen,
    Denim,
    Synthetic,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ColorOption {
    hex_rgb: u32,
    name: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProductVariant {
    sku: String,
    size: SizeSystem,
    color: ColorOption,
    stock_count: u32,
    price_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FashionProduct {
    product_id: u64,
    name: String,
    category: ProductCategory,
    fabric: FabricType,
    variants: Vec<ProductVariant>,
    season: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Collection {
    collection_id: u64,
    name: String,
    products: Vec<FashionProduct>,
    release_date: u64,
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn default_color() -> ColorOption {
    ColorOption {
        hex_rgb: 0x000000,
        name: "Black".to_string(),
    }
}

fn make_variant(sku: &str, size: SizeSystem, stock: u32, price: u64) -> ProductVariant {
    ProductVariant {
        sku: sku.to_string(),
        size,
        color: default_color(),
        stock_count: stock,
        price_cents: price,
    }
}

fn make_product(
    id: u64,
    name: &str,
    cat: ProductCategory,
    fabric: FabricType,
    season: &str,
) -> FashionProduct {
    FashionProduct {
        product_id: id,
        name: name.to_string(),
        category: cat,
        fabric,
        variants: vec![],
        season: season.to_string(),
    }
}

// ── 1. ProductCategory: each variant (Tops / Bottoms / Dresses / Outerwear / Footwear / Accessories / Sportswear) ──
#[test]
fn test_product_category_all_variants() {
    let cfg = config::standard();
    let categories = [
        ProductCategory::Tops,
        ProductCategory::Bottoms,
        ProductCategory::Dresses,
        ProductCategory::Outerwear,
        ProductCategory::Footwear,
        ProductCategory::Accessories,
        ProductCategory::Sportswear,
    ];
    for cat in &categories {
        let bytes = encode_to_vec(cat, cfg).expect("encode ProductCategory variant");
        let (decoded, _): (ProductCategory, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode ProductCategory variant");
        assert_eq!(cat, &decoded);
    }
}

// ── 2. FabricType: each variant ───────────────────────────────────────────────
#[test]
fn test_fabric_type_all_variants() {
    let cfg = config::standard();
    let fabrics = [
        FabricType::Cotton,
        FabricType::Polyester,
        FabricType::Wool,
        FabricType::Silk,
        FabricType::Linen,
        FabricType::Denim,
        FabricType::Synthetic,
    ];
    for fabric in &fabrics {
        let bytes = encode_to_vec(fabric, cfg).expect("encode FabricType variant");
        let (decoded, _): (FabricType, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode FabricType variant");
        assert_eq!(fabric, &decoded);
    }
}

// ── 3. SizeSystem: simple variants (Xs through Xxl) ──────────────────────────
#[test]
fn test_size_system_simple_variants() {
    let cfg = config::standard();
    let sizes = [
        SizeSystem::Xs,
        SizeSystem::S,
        SizeSystem::M,
        SizeSystem::L,
        SizeSystem::Xl,
        SizeSystem::Xxl,
    ];
    for size in &sizes {
        let bytes = encode_to_vec(size, cfg).expect("encode SizeSystem simple variant");
        let (decoded, _): (SizeSystem, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode SizeSystem simple variant");
        assert_eq!(size, &decoded);
    }
}

// ── 4. SizeSystem: Numeric variant ────────────────────────────────────────────
#[test]
fn test_size_system_numeric_variant() {
    let cfg = config::standard();
    let size = SizeSystem::Numeric(42);
    let bytes = encode_to_vec(&size, cfg).expect("encode SizeSystem::Numeric");
    let (decoded, _): (SizeSystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SizeSystem::Numeric");
    assert_eq!(size, decoded);
}

// ── 5. ColorOption roundtrip ──────────────────────────────────────────────────
#[test]
fn test_color_option_roundtrip() {
    let cfg = config::standard();
    let color = ColorOption {
        hex_rgb: 0xFF5733,
        name: "Burnt Orange".to_string(),
    };
    let bytes = encode_to_vec(&color, cfg).expect("encode ColorOption");
    let (decoded, _): (ColorOption, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ColorOption");
    assert_eq!(color, decoded);
}

// ── 6. ProductVariant roundtrip ───────────────────────────────────────────────
#[test]
fn test_product_variant_roundtrip() {
    let cfg = config::standard();
    let variant = ProductVariant {
        sku: "SKU-001-M-BLK".to_string(),
        size: SizeSystem::M,
        color: ColorOption {
            hex_rgb: 0x1A1A1A,
            name: "Charcoal".to_string(),
        },
        stock_count: 50,
        price_cents: 4999,
    };
    let bytes = encode_to_vec(&variant, cfg).expect("encode ProductVariant");
    let (decoded, _): (ProductVariant, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ProductVariant");
    assert_eq!(variant, decoded);
}

// ── 7. FashionProduct with empty variants ─────────────────────────────────────
#[test]
fn test_fashion_product_empty_variants() {
    let cfg = config::standard();
    let product = make_product(
        1001,
        "Classic Crew Neck Tee",
        ProductCategory::Tops,
        FabricType::Cotton,
        "SS2026",
    );
    let bytes = encode_to_vec(&product, cfg).expect("encode FashionProduct (empty variants)");
    let (decoded, _): (FashionProduct, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FashionProduct (empty variants)");
    assert_eq!(product, decoded);
    assert!(decoded.variants.is_empty());
}

// ── 8. FashionProduct with 5 variants ────────────────────────────────────────
#[test]
fn test_fashion_product_five_variants() {
    let cfg = config::standard();
    let mut product = make_product(
        1002,
        "Slim Fit Chinos",
        ProductCategory::Bottoms,
        FabricType::Cotton,
        "AW2026",
    );
    product.variants = vec![
        make_variant("CHN-XS-KHK", SizeSystem::Xs, 10, 5999),
        make_variant("CHN-S-KHK", SizeSystem::S, 25, 5999),
        make_variant("CHN-M-KHK", SizeSystem::M, 40, 5999),
        make_variant("CHN-L-KHK", SizeSystem::L, 35, 5999),
        make_variant("CHN-XL-KHK", SizeSystem::Xl, 20, 5999),
    ];
    let bytes = encode_to_vec(&product, cfg).expect("encode FashionProduct (5 variants)");
    let (decoded, _): (FashionProduct, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FashionProduct (5 variants)");
    assert_eq!(product, decoded);
    assert_eq!(decoded.variants.len(), 5);
}

// ── 9. Collection with 3 products ─────────────────────────────────────────────
#[test]
fn test_collection_with_three_products() {
    let cfg = config::standard();
    let collection = Collection {
        collection_id: 500,
        name: "Spring Essentials 2026".to_string(),
        products: vec![
            make_product(
                2001,
                "Linen Shirt",
                ProductCategory::Tops,
                FabricType::Linen,
                "SS2026",
            ),
            make_product(
                2002,
                "Floral Dress",
                ProductCategory::Dresses,
                FabricType::Polyester,
                "SS2026",
            ),
            make_product(
                2003,
                "Canvas Sneaker",
                ProductCategory::Footwear,
                FabricType::Cotton,
                "SS2026",
            ),
        ],
        release_date: 1_740_000_000,
    };
    let bytes = encode_to_vec(&collection, cfg).expect("encode Collection (3 products)");
    let (decoded, _): (Collection, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Collection (3 products)");
    assert_eq!(collection, decoded);
    assert_eq!(decoded.products.len(), 3);
}

// ── 10. Vec<ProductVariant> roundtrip ─────────────────────────────────────────
#[test]
fn test_vec_product_variant_roundtrip() {
    let cfg = config::standard();
    let variants: Vec<ProductVariant> = vec![
        make_variant("VAR-001", SizeSystem::S, 15, 2999),
        make_variant("VAR-002", SizeSystem::M, 30, 2999),
        make_variant("VAR-003", SizeSystem::L, 20, 2999),
    ];
    let bytes = encode_to_vec(&variants, cfg).expect("encode Vec<ProductVariant>");
    let (decoded, _): (Vec<ProductVariant>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ProductVariant>");
    assert_eq!(variants, decoded);
}

// ── 11. Big-endian config ─────────────────────────────────────────────────────
#[test]
fn test_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let product = make_product(
        9999,
        "Trench Coat",
        ProductCategory::Outerwear,
        FabricType::Wool,
        "AW2026",
    );
    let bytes = encode_to_vec(&product, cfg).expect("encode with big_endian");
    let (decoded, _): (FashionProduct, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode with big_endian");
    assert_eq!(product, decoded);
}

// ── 12. Fixed-int config ──────────────────────────────────────────────────────
#[test]
fn test_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let variant = ProductVariant {
        sku: "FIX-SKU-001".to_string(),
        size: SizeSystem::Numeric(38),
        color: ColorOption {
            hex_rgb: 0xFFFFFF,
            name: "White".to_string(),
        },
        stock_count: 100,
        price_cents: 9999,
    };
    let bytes = encode_to_vec(&variant, cfg).expect("encode with fixed_int");
    let (decoded, _): (ProductVariant, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode with fixed_int");
    assert_eq!(variant, decoded);
}

// ── 13. Combined config: big-endian + fixed-int ───────────────────────────────
#[test]
fn test_combined_config_big_endian_fixed_int() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let collection = Collection {
        collection_id: 777,
        name: "Power Suit Collection".to_string(),
        products: vec![make_product(
            3001,
            "Tailored Blazer",
            ProductCategory::Tops,
            FabricType::Wool,
            "AW2026",
        )],
        release_date: 1_750_000_000,
    };
    let bytes = encode_to_vec(&collection, cfg).expect("encode combined config");
    let (decoded, _): (Collection, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode combined config");
    assert_eq!(collection, decoded);
}

// ── 14. Consumed-bytes check ──────────────────────────────────────────────────
#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let cfg = config::standard();
    let product = FashionProduct {
        product_id: 4242,
        name: "Heritage Denim Jacket".to_string(),
        category: ProductCategory::Outerwear,
        fabric: FabricType::Denim,
        variants: vec![
            make_variant("HRJ-M-IND", SizeSystem::M, 8, 12999),
            make_variant("HRJ-L-IND", SizeSystem::L, 5, 12999),
        ],
        season: "AW2026".to_string(),
    };
    let bytes = encode_to_vec(&product, cfg).expect("encode for consumed-bytes check");
    let (decoded, consumed): (FashionProduct, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode for consumed-bytes check");
    assert_eq!(product, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}

// ── 15. Luxury silk dress collection ─────────────────────────────────────────
#[test]
fn test_luxury_silk_dress_collection() {
    let cfg = config::standard();
    let silk_colors = vec![
        ColorOption {
            hex_rgb: 0xFFF0E6,
            name: "Ivory".to_string(),
        },
        ColorOption {
            hex_rgb: 0x800020,
            name: "Burgundy".to_string(),
        },
        ColorOption {
            hex_rgb: 0x191970,
            name: "Midnight Blue".to_string(),
        },
    ];
    let sizes = [
        SizeSystem::Xs,
        SizeSystem::S,
        SizeSystem::M,
        SizeSystem::L,
        SizeSystem::Xl,
    ];
    let variants: Vec<ProductVariant> = silk_colors
        .iter()
        .enumerate()
        .flat_map(|(ci, color)| {
            sizes.iter().map(move |size| ProductVariant {
                sku: format!("SILK-DRESS-{}-{:?}", ci, size),
                size: size.clone(),
                color: color.clone(),
                stock_count: 3,
                price_cents: 59900,
            })
        })
        .collect();
    let product = FashionProduct {
        product_id: 8001,
        name: "Ethereal Silk Evening Gown".to_string(),
        category: ProductCategory::Dresses,
        fabric: FabricType::Silk,
        variants,
        season: "FW2026".to_string(),
    };
    let collection = Collection {
        collection_id: 101,
        name: "Atelier Luxe FW2026".to_string(),
        products: vec![product],
        release_date: 1_745_000_000,
    };
    let bytes = encode_to_vec(&collection, cfg).expect("encode luxury silk collection");
    let (decoded, _): (Collection, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode luxury silk collection");
    assert_eq!(collection, decoded);
    assert_eq!(decoded.products[0].fabric, FabricType::Silk);
    assert_eq!(decoded.products[0].variants.len(), 15);
}

// ── 16. Denim collection with 5 sizes ────────────────────────────────────────
#[test]
fn test_denim_collection_five_sizes() {
    let cfg = config::standard();
    let sizes = [
        SizeSystem::Xs,
        SizeSystem::S,
        SizeSystem::M,
        SizeSystem::L,
        SizeSystem::Xl,
    ];
    let variants: Vec<ProductVariant> = sizes
        .iter()
        .map(|size| make_variant(&format!("DNM-{:?}", size), size.clone(), 20, 7999))
        .collect();
    let mut product = make_product(
        8002,
        "Straight-Leg Denim",
        ProductCategory::Bottoms,
        FabricType::Denim,
        "SS2026",
    );
    product.variants = variants;
    let collection = Collection {
        collection_id: 102,
        name: "Denim Revival 2026".to_string(),
        products: vec![product],
        release_date: 1_746_000_000,
    };
    let bytes = encode_to_vec(&collection, cfg).expect("encode denim collection");
    let (decoded, _): (Collection, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode denim collection");
    assert_eq!(collection, decoded);
    assert_eq!(decoded.products[0].variants.len(), 5);
    assert_eq!(decoded.products[0].fabric, FabricType::Denim);
}

// ── 17. Sportswear with all SizeSystem variants including Numeric ─────────────
#[test]
fn test_sportswear_all_sizes() {
    let cfg = config::standard();
    let sizes = vec![
        SizeSystem::Xs,
        SizeSystem::S,
        SizeSystem::M,
        SizeSystem::L,
        SizeSystem::Xl,
        SizeSystem::Xxl,
        SizeSystem::Numeric(28),
        SizeSystem::Numeric(30),
        SizeSystem::Numeric(32),
    ];
    let variants: Vec<ProductVariant> = sizes
        .into_iter()
        .enumerate()
        .map(|(i, size)| ProductVariant {
            sku: format!("SPT-RUN-{:03}", i),
            size,
            color: ColorOption {
                hex_rgb: 0x00BFFF,
                name: "Electric Blue".to_string(),
            },
            stock_count: 50,
            price_cents: 8999,
        })
        .collect();
    let mut product = make_product(
        8003,
        "Performance Running Tight",
        ProductCategory::Sportswear,
        FabricType::Synthetic,
        "SS2026",
    );
    product.variants = variants;
    let bytes = encode_to_vec(&product, cfg).expect("encode sportswear all sizes");
    let (decoded, _): (FashionProduct, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode sportswear all sizes");
    assert_eq!(product, decoded);
    assert_eq!(decoded.variants.len(), 9);
    assert_eq!(decoded.category, ProductCategory::Sportswear);
}

// ── 18. Seasonal collection: summer and winter products ───────────────────────
#[test]
fn test_seasonal_collection_summer_and_winter() {
    let cfg = config::standard();
    let summer_product = FashionProduct {
        product_id: 9001,
        name: "Lightweight Linen Shirt".to_string(),
        category: ProductCategory::Tops,
        fabric: FabricType::Linen,
        variants: vec![make_variant("LIN-S-WHT", SizeSystem::S, 30, 4499)],
        season: "SS2026".to_string(),
    };
    let winter_product = FashionProduct {
        product_id: 9002,
        name: "Merino Wool Overcoat".to_string(),
        category: ProductCategory::Outerwear,
        fabric: FabricType::Wool,
        variants: vec![make_variant("WOL-M-GRY", SizeSystem::M, 12, 24999)],
        season: "FW2026".to_string(),
    };
    let collection = Collection {
        collection_id: 200,
        name: "Year-Round Wardrobe Essentials".to_string(),
        products: vec![summer_product, winter_product],
        release_date: 1_741_000_000,
    };
    let bytes = encode_to_vec(&collection, cfg).expect("encode seasonal collection");
    let (decoded, _): (Collection, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode seasonal collection");
    assert_eq!(collection, decoded);
    assert_eq!(decoded.products[0].season, "SS2026");
    assert_eq!(decoded.products[1].season, "FW2026");
}

// ── 19. Out-of-stock variant (stock_count = 0) ────────────────────────────────
#[test]
fn test_out_of_stock_variant() {
    let cfg = config::standard();
    let variant = ProductVariant {
        sku: "OOS-XL-RED".to_string(),
        size: SizeSystem::Xl,
        color: ColorOption {
            hex_rgb: 0xFF0000,
            name: "Scarlet".to_string(),
        },
        stock_count: 0,
        price_cents: 3999,
    };
    let bytes = encode_to_vec(&variant, cfg).expect("encode out-of-stock variant");
    let (decoded, _): (ProductVariant, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode out-of-stock variant");
    assert_eq!(variant, decoded);
    assert_eq!(decoded.stock_count, 0);
}

// ── 20. Premium pricing (price_cents = u64::MAX) ──────────────────────────────
#[test]
fn test_premium_pricing_u64_max() {
    let cfg = config::standard();
    let variant = ProductVariant {
        sku: "COUTURE-001-XXL".to_string(),
        size: SizeSystem::Xxl,
        color: ColorOption {
            hex_rgb: 0xD4AF37,
            name: "Gold Leaf".to_string(),
        },
        stock_count: 1,
        price_cents: u64::MAX,
    };
    let bytes = encode_to_vec(&variant, cfg).expect("encode premium pricing u64::MAX");
    let (decoded, _): (ProductVariant, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode premium pricing u64::MAX");
    assert_eq!(variant, decoded);
    assert_eq!(decoded.price_cents, u64::MAX);
}

// ── 21. Unicode product names ─────────────────────────────────────────────────
#[test]
fn test_unicode_product_names() {
    let cfg = config::standard();
    let products = vec![
        make_product(
            7001,
            "シルクブラウス (Silk Blouse)",
            ProductCategory::Tops,
            FabricType::Silk,
            "SS2026",
        ),
        make_product(
            7002,
            "Robe en Soie Élégante",
            ProductCategory::Dresses,
            FabricType::Silk,
            "SS2026",
        ),
        make_product(
            7003,
            "Пальто из шерсти мериносы",
            ProductCategory::Outerwear,
            FabricType::Wool,
            "FW2026",
        ),
        make_product(
            7004,
            "한복 스타일 재킷",
            ProductCategory::Tops,
            FabricType::Synthetic,
            "AW2026",
        ),
        make_product(
            7005,
            "Džínová bunda – limitovaná edice",
            ProductCategory::Outerwear,
            FabricType::Denim,
            "SS2026",
        ),
    ];
    for product in &products {
        let bytes = encode_to_vec(product, cfg).expect("encode unicode product name");
        let (decoded, _): (FashionProduct, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode unicode product name");
        assert_eq!(product, &decoded);
    }
}

// ── 22. Multi-season catalog ──────────────────────────────────────────────────
#[test]
fn test_multi_season_catalog() {
    let cfg = config::standard();
    let seasons = ["SS2024", "AW2024", "SS2025", "AW2025", "SS2026", "AW2026"];
    let products: Vec<FashionProduct> = seasons
        .iter()
        .enumerate()
        .map(|(i, season)| {
            let mut p = make_product(
                (6000 + i) as u64,
                &format!("Signature Piece {}", i + 1),
                ProductCategory::Accessories,
                FabricType::Synthetic,
                season,
            );
            p.variants = vec![make_variant(
                &format!("ACC-{:04}-M", i),
                SizeSystem::M,
                10,
                1999 + (i as u64 * 500),
            )];
            p
        })
        .collect();
    let collection = Collection {
        collection_id: 999,
        name: "Brand Archive 2024–2026".to_string(),
        products,
        release_date: 1_735_000_000,
    };
    let bytes = encode_to_vec(&collection, cfg).expect("encode multi-season catalog");
    let (decoded, _): (Collection, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode multi-season catalog");
    assert_eq!(collection, decoded);
    assert_eq!(decoded.products.len(), 6);
    assert_eq!(decoded.products[0].season, "SS2024");
    assert_eq!(decoded.products[5].season, "AW2026");
}
