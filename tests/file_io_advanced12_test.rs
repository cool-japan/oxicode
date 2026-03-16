#![cfg(feature = "std")]
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
    config, decode_from_file, decode_from_std_read, encode_into_std_write, encode_to_file, Decode,
    Encode,
};
use std::fs::File;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Category {
    Electronics,
    Clothing,
    Food,
    Books,
    Sports,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InventoryItem {
    sku: String,
    name: String,
    category: Category,
    quantity: u32,
    price_cents: u64,
    weight_g: Option<u32>,
}

fn make_path(suffix: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "oxicode_inv_{}_{}_{}.bin",
        std::process::id(),
        suffix,
        // extra uniqueness via suffix itself avoids collisions between tests
        suffix.len()
    ))
}

// Test 1: InventoryItem encode_to_file / decode_from_file roundtrip
#[test]
fn test_inventory_item_encode_decode_file_roundtrip() {
    let item = InventoryItem {
        sku: "SKU-001".to_string(),
        name: "Wireless Headphones".to_string(),
        category: Category::Electronics,
        quantity: 50,
        price_cents: 9999,
        weight_g: Some(320),
    };
    let path = make_path("t01_roundtrip");
    encode_to_file(&item, &path).expect("encode_to_file failed");
    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(item, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 2: Category::Electronics roundtrip via file
#[test]
fn test_category_electronics_roundtrip_via_file() {
    let cat = Category::Electronics;
    let path = make_path("t02_cat_electronics");
    encode_to_file(&cat, &path).expect("encode_to_file failed");
    let decoded: Category = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(cat, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 3: Category::Other(String) roundtrip via file
#[test]
fn test_category_other_string_roundtrip_via_file() {
    let cat = Category::Other("Seasonal Clearance".to_string());
    let path = make_path("t03_cat_other");
    encode_to_file(&cat, &path).expect("encode_to_file failed");
    let decoded: Category = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(cat, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 4: Vec<InventoryItem> to file / from file
#[test]
fn test_vec_inventory_items_file_roundtrip() {
    let items = vec![
        InventoryItem {
            sku: "SKU-100".to_string(),
            name: "T-Shirt".to_string(),
            category: Category::Clothing,
            quantity: 200,
            price_cents: 1999,
            weight_g: Some(150),
        },
        InventoryItem {
            sku: "SKU-200".to_string(),
            name: "Programming Book".to_string(),
            category: Category::Books,
            quantity: 30,
            price_cents: 4999,
            weight_g: Some(450),
        },
        InventoryItem {
            sku: "SKU-300".to_string(),
            name: "Energy Bar".to_string(),
            category: Category::Food,
            quantity: 500,
            price_cents: 299,
            weight_g: Some(60),
        },
    ];
    let path = make_path("t04_vec_items");
    encode_to_file(&items, &path).expect("encode_to_file failed");
    let decoded: Vec<InventoryItem> = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(items, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 5: encode_into_std_write + decode_from_std_read roundtrip
#[test]
fn test_encode_into_std_write_decode_from_std_read_roundtrip() {
    let item = InventoryItem {
        sku: "SKU-500".to_string(),
        name: "Running Shoes".to_string(),
        category: Category::Sports,
        quantity: 75,
        price_cents: 8999,
        weight_g: Some(700),
    };
    let path = make_path("t05_std_write_read");
    {
        let file = File::create(&path).expect("create file failed");
        encode_into_std_write(item.clone(), file, config::standard())
            .expect("encode_into_std_write failed");
    }
    let file = File::open(&path).expect("open file failed");
    let decoded: InventoryItem =
        decode_from_std_read(file, config::standard()).expect("decode_from_std_read failed");
    assert_eq!(item, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 6: File size equals encode_to_vec length
#[test]
fn test_file_size_equals_encode_to_vec_length() {
    let item = InventoryItem {
        sku: "SKU-042".to_string(),
        name: "Bluetooth Speaker".to_string(),
        category: Category::Electronics,
        quantity: 15,
        price_cents: 5499,
        weight_g: Some(410),
    };
    let path = make_path("t06_file_size");
    encode_to_file(&item, &path).expect("encode_to_file failed");
    let file_bytes = std::fs::read(&path).expect("read failed");
    let vec_bytes = oxicode::encode_to_vec(&item).expect("encode_to_vec failed");
    assert_eq!(file_bytes.len(), vec_bytes.len());
    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).ok();
}

// Test 7: Overwrite existing file with new value
#[test]
fn test_overwrite_existing_file_with_new_value() {
    let path = make_path("t07_overwrite");
    let original = InventoryItem {
        sku: "SKU-OLD".to_string(),
        name: "Old Product".to_string(),
        category: Category::Other("Discontinued".to_string()),
        quantity: 0,
        price_cents: 0,
        weight_g: None,
    };
    encode_to_file(&original, &path).expect("first encode_to_file failed");

    let replacement = InventoryItem {
        sku: "SKU-NEW".to_string(),
        name: "Replacement Product".to_string(),
        category: Category::Electronics,
        quantity: 100,
        price_cents: 7999,
        weight_g: Some(250),
    };
    encode_to_file(&replacement, &path).expect("second encode_to_file failed");

    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(replacement, decoded);
    assert_ne!(original.sku, decoded.sku);
    std::fs::remove_file(&path).ok();
}

// Test 8: Multiple sequential writes to same file (overwrite each time)
#[test]
fn test_multiple_sequential_writes_same_file() {
    let path = make_path("t08_seq_writes");
    let skus = ["SKU-A", "SKU-B", "SKU-C", "SKU-D"];
    let mut last_item = None;
    for (i, sku) in skus.iter().enumerate() {
        let item = InventoryItem {
            sku: sku.to_string(),
            name: format!("Product {i}"),
            category: Category::Books,
            quantity: i as u32 * 10,
            price_cents: i as u64 * 1000,
            weight_g: Some(i as u32 * 100 + 50),
        };
        encode_to_file(&item, &path).expect("encode_to_file failed in sequential write");
        last_item = Some(item);
    }
    let expected = last_item.expect("no items written");
    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(expected, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 9: InventoryItem with None weight_g
#[test]
fn test_inventory_item_none_weight_roundtrip() {
    let item = InventoryItem {
        sku: "SKU-NONE".to_string(),
        name: "Digital Download".to_string(),
        category: Category::Books,
        quantity: 9999,
        price_cents: 999,
        weight_g: None,
    };
    let path = make_path("t09_none_weight");
    encode_to_file(&item, &path).expect("encode_to_file failed");
    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(item, decoded);
    assert!(decoded.weight_g.is_none());
    std::fs::remove_file(&path).ok();
}

// Test 10: InventoryItem with Some weight_g
#[test]
fn test_inventory_item_some_weight_roundtrip() {
    let item = InventoryItem {
        sku: "SKU-HEAVY".to_string(),
        name: "Dumbbell Set".to_string(),
        category: Category::Sports,
        quantity: 10,
        price_cents: 15999,
        weight_g: Some(20000),
    };
    let path = make_path("t10_some_weight");
    encode_to_file(&item, &path).expect("encode_to_file failed");
    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(item, decoded);
    assert_eq!(decoded.weight_g, Some(20000));
    std::fs::remove_file(&path).ok();
}

// Test 11: decode_from_file on non-existent path returns Err
#[test]
fn test_decode_from_file_nonexistent_returns_err() {
    let path = make_path("t11_nonexistent_ZZZZ999");
    // ensure it does not exist
    std::fs::remove_file(&path).ok();
    let result = decode_from_file::<InventoryItem>(&path);
    assert!(
        result.is_err(),
        "expected Err for non-existent file, got Ok"
    );
}

// Test 12: Empty Vec<InventoryItem> to file
#[test]
fn test_empty_vec_inventory_items_file_roundtrip() {
    let items: Vec<InventoryItem> = vec![];
    let path = make_path("t12_empty_vec");
    encode_to_file(&items, &path).expect("encode_to_file failed");
    let decoded: Vec<InventoryItem> = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(items, decoded);
    assert!(decoded.is_empty());
    std::fs::remove_file(&path).ok();
}

// Test 13: Large InventoryItem (long strings) roundtrip
#[test]
fn test_large_inventory_item_long_strings_roundtrip() {
    let item = InventoryItem {
        sku: "X".repeat(1000),
        name: "Y".repeat(5000),
        category: Category::Other("Z".repeat(2000)),
        quantity: u32::MAX,
        price_cents: 999_999_999,
        weight_g: Some(u32::MAX / 2),
    };
    let path = make_path("t13_large_strings");
    encode_to_file(&item, &path).expect("encode_to_file failed");
    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(item, decoded);
    assert_eq!(decoded.sku.len(), 1000);
    assert_eq!(decoded.name.len(), 5000);
    std::fs::remove_file(&path).ok();
}

// Test 14: All Category variants roundtrip via file
#[test]
fn test_all_category_variants_roundtrip_via_file() {
    let variants = vec![
        Category::Electronics,
        Category::Clothing,
        Category::Food,
        Category::Books,
        Category::Sports,
        Category::Other("Garden & Outdoor".to_string()),
    ];
    let path = make_path("t14_all_categories");
    encode_to_file(&variants, &path).expect("encode_to_file failed");
    let decoded: Vec<Category> = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(variants, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 15: Big-endian config encode_to_file roundtrip
#[test]
fn test_big_endian_config_encode_to_file_roundtrip() {
    let item = InventoryItem {
        sku: "SKU-BE".to_string(),
        name: "Big Endian Widget".to_string(),
        category: Category::Electronics,
        quantity: 42,
        price_cents: 1234,
        weight_g: Some(500),
    };
    let path = make_path("t15_big_endian");
    let cfg = config::standard().with_big_endian();
    oxicode::encode_to_file_with_config(&item, &path, cfg)
        .expect("encode_to_file_with_config failed");
    let decoded: InventoryItem = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode_from_file_with_config failed");
    assert_eq!(item, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 16: Fixed-int config encode_to_file roundtrip
#[test]
fn test_fixed_int_config_encode_to_file_roundtrip() {
    let item = InventoryItem {
        sku: "SKU-FI".to_string(),
        name: "Fixed Int Product".to_string(),
        category: Category::Clothing,
        quantity: 128,
        price_cents: 2599,
        weight_g: Some(300),
    };
    let path = make_path("t16_fixed_int");
    let cfg = config::standard().with_fixed_int_encoding();
    oxicode::encode_to_file_with_config(&item, &path, cfg)
        .expect("encode_to_file_with_config failed");
    let decoded: InventoryItem = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode_from_file_with_config failed");
    assert_eq!(item, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 17: encode_into_std_write to cursor, verify bytes match encode_to_vec
#[test]
fn test_encode_into_std_write_cursor_bytes_match_encode_to_vec() {
    let item = InventoryItem {
        sku: "SKU-CUR".to_string(),
        name: "Cursor Test Item".to_string(),
        category: Category::Food,
        quantity: 88,
        price_cents: 349,
        weight_g: None,
    };
    let mut cursor = std::io::Cursor::new(Vec::new());
    let written = encode_into_std_write(item.clone(), &mut cursor, config::standard())
        .expect("encode_into_std_write to cursor failed");
    let cursor_bytes = cursor.into_inner();
    let vec_bytes = oxicode::encode_to_vec(&item).expect("encode_to_vec failed");
    assert_eq!(written, vec_bytes.len());
    assert_eq!(cursor_bytes, vec_bytes);
}

// Test 18: Unicode SKU and name roundtrip via file
#[test]
fn test_unicode_sku_and_name_roundtrip_via_file() {
    let item = InventoryItem {
        sku: "商品-東京-001".to_string(),
        name: "東京タワー記念品 🗼".to_string(),
        category: Category::Other("みやげ物・観光".to_string()),
        quantity: 25,
        price_cents: 1500,
        weight_g: Some(80),
    };
    let path = make_path("t18_unicode");
    encode_to_file(&item, &path).expect("encode_to_file failed");
    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(item, decoded);
    assert_eq!(decoded.sku, "商品-東京-001");
    std::fs::remove_file(&path).ok();
}

// Test 19: Option<InventoryItem> Some via file
#[test]
fn test_option_inventory_item_some_via_file() {
    let item = InventoryItem {
        sku: "SKU-OPT-SOME".to_string(),
        name: "Optional Item Present".to_string(),
        category: Category::Sports,
        quantity: 5,
        price_cents: 3299,
        weight_g: Some(950),
    };
    let opt_item: Option<InventoryItem> = Some(item);
    let path = make_path("t19_option_some");
    encode_to_file(&opt_item, &path).expect("encode_to_file failed");
    let decoded: Option<InventoryItem> = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(opt_item, decoded);
    assert!(decoded.is_some());
    std::fs::remove_file(&path).ok();
}

// Test 20: Option<InventoryItem> None via file
#[test]
fn test_option_inventory_item_none_via_file() {
    let opt_item: Option<InventoryItem> = None;
    let path = make_path("t20_option_none");
    encode_to_file(&opt_item, &path).expect("encode_to_file failed");
    let decoded: Option<InventoryItem> = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(opt_item, decoded);
    assert!(decoded.is_none());
    std::fs::remove_file(&path).ok();
}

// Test 21: u64 price_cents = u64::MAX via file
#[test]
fn test_price_cents_u64_max_via_file() {
    let item = InventoryItem {
        sku: "SKU-MAXPRICE".to_string(),
        name: "Ultra Premium Edition".to_string(),
        category: Category::Electronics,
        quantity: 1,
        price_cents: u64::MAX,
        weight_g: Some(100),
    };
    let path = make_path("t21_u64_max");
    encode_to_file(&item, &path).expect("encode_to_file failed");
    let decoded: InventoryItem = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(item, decoded);
    assert_eq!(decoded.price_cents, u64::MAX);
    std::fs::remove_file(&path).ok();
}

// Test 22: Vec<Category> with all variant types via file
#[test]
fn test_vec_category_all_variants_via_file() {
    let categories: Vec<Category> = vec![
        Category::Electronics,
        Category::Clothing,
        Category::Food,
        Category::Books,
        Category::Sports,
        Category::Other("Vintage Collectibles".to_string()),
        Category::Other("".to_string()),
        Category::Electronics,
        Category::Food,
        Category::Other("Limited Edition".to_string()),
    ];
    let path = make_path("t22_vec_all_cats");
    encode_to_file(&categories, &path).expect("encode_to_file failed");
    let decoded: Vec<Category> = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(categories, decoded);
    assert_eq!(decoded.len(), 10);
    // verify the empty Other string round-tripped correctly
    assert_eq!(decoded[6], Category::Other("".to_string()));
    std::fs::remove_file(&path).ok();
}
