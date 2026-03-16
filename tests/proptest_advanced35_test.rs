//! Advanced property-based roundtrip tests (set 35) using proptest.
//!
//! Theme: Supply chain / inventory management.
//! Types: ItemCategory, InventoryItem, WarehouseLocation, StockMovement.
//! Each proptest! block contains exactly one #[test] function.
//! Tests verify that encode → decode is a perfect roundtrip for all tested types.

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
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ItemCategory {
    Electronics,
    Clothing,
    Food,
    Tools,
    Books,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InventoryItem {
    sku: u64,
    category: ItemCategory,
    quantity: u32,
    unit_price_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WarehouseLocation {
    zone: u8,
    row: u8,
    shelf: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StockMovement {
    item_sku: u64,
    from_location: Option<WarehouseLocation>,
    to_location: Option<WarehouseLocation>,
    quantity: u32,
}

fn item_category_from_u8(n: u8) -> ItemCategory {
    match n % 5 {
        0 => ItemCategory::Electronics,
        1 => ItemCategory::Clothing,
        2 => ItemCategory::Food,
        3 => ItemCategory::Tools,
        _ => ItemCategory::Books,
    }
}

fn inventory_item_strategy() -> impl Strategy<Value = InventoryItem> {
    (any::<u64>(), 0u8..5u8, any::<u32>(), any::<u64>()).prop_map(
        |(sku, cat_idx, quantity, unit_price_cents)| InventoryItem {
            sku,
            category: item_category_from_u8(cat_idx),
            quantity,
            unit_price_cents,
        },
    )
}

fn warehouse_location_strategy() -> impl Strategy<Value = WarehouseLocation> {
    (any::<u8>(), any::<u8>(), any::<u8>()).prop_map(|(zone, row, shelf)| WarehouseLocation {
        zone,
        row,
        shelf,
    })
}

fn stock_movement_strategy() -> impl Strategy<Value = StockMovement> {
    (
        any::<u64>(),
        proptest::option::of(warehouse_location_strategy()),
        proptest::option::of(warehouse_location_strategy()),
        any::<u32>(),
    )
        .prop_map(
            |(item_sku, from_location, to_location, quantity)| StockMovement {
                item_sku,
                from_location,
                to_location,
                quantity,
            },
        )
}

// Test 1: InventoryItem full roundtrip
proptest! {
    #[test]
    fn prop_inventory_item_roundtrip(item in inventory_item_strategy()) {
        let encoded = encode_to_vec(&item).expect("encode failed");
        let (decoded, _): (InventoryItem, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(item, decoded);
    }
}

// Test 2: ItemCategory variant roundtrip via index
proptest! {
    #[test]
    fn prop_item_category_variant_roundtrip(n in 0u8..5u8) {
        let cat = item_category_from_u8(n);
        let encoded = encode_to_vec(&cat).expect("encode failed");
        let (decoded, _): (ItemCategory, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(cat, decoded);
    }
}

// Test 3: WarehouseLocation roundtrip
proptest! {
    #[test]
    fn prop_warehouse_location_roundtrip(loc in warehouse_location_strategy()) {
        let encoded = encode_to_vec(&loc).expect("encode failed");
        let (decoded, _): (WarehouseLocation, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(loc, decoded);
    }
}

// Test 4: StockMovement full roundtrip (includes nested Option<WarehouseLocation>)
proptest! {
    #[test]
    fn prop_stock_movement_roundtrip(movement in stock_movement_strategy()) {
        let encoded = encode_to_vec(&movement).expect("encode failed");
        let (decoded, _): (StockMovement, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(movement, decoded);
    }
}

// Test 5: Vec<InventoryItem> roundtrip (0..8 items)
proptest! {
    #[test]
    fn prop_vec_inventory_item_roundtrip(
        items in proptest::collection::vec(inventory_item_strategy(), 0..8)
    ) {
        let encoded = encode_to_vec(&items).expect("encode failed");
        let (decoded, _): (Vec<InventoryItem>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(items, decoded);
    }
}

// Test 6: Vec<StockMovement> roundtrip (0..5 items)
proptest! {
    #[test]
    fn prop_vec_stock_movement_roundtrip(
        movements in proptest::collection::vec(stock_movement_strategy(), 0..5)
    ) {
        let encoded = encode_to_vec(&movements).expect("encode failed");
        let (decoded, _): (Vec<StockMovement>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(movements, decoded);
    }
}

// Test 7: Option<InventoryItem> roundtrip
proptest! {
    #[test]
    fn prop_option_inventory_item_roundtrip(
        opt in proptest::option::of(inventory_item_strategy())
    ) {
        let encoded = encode_to_vec(&opt).expect("encode failed");
        let (decoded, _): (Option<InventoryItem>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(opt, decoded);
    }
}

// Test 8: Option<WarehouseLocation> — None encodes differently from Some
proptest! {
    #[test]
    fn prop_option_warehouse_location_none_vs_some_differ(loc in warehouse_location_strategy()) {
        let none_val: Option<WarehouseLocation> = None;
        let some_val: Option<WarehouseLocation> = Some(loc);
        let encoded_none = encode_to_vec(&none_val).expect("encode failed");
        let encoded_some = encode_to_vec(&some_val).expect("encode failed");
        prop_assert_ne!(encoded_none, encoded_some);
    }
}

// Test 9: StockMovement with both locations None roundtrip
proptest! {
    #[test]
    fn prop_stock_movement_both_locations_none_roundtrip(
        item_sku: u64,
        quantity: u32,
    ) {
        let movement = StockMovement {
            item_sku,
            from_location: None,
            to_location: None,
            quantity,
        };
        let encoded = encode_to_vec(&movement).expect("encode failed");
        let (decoded, consumed): (StockMovement, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(movement, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 10: StockMovement with both locations Some roundtrip
proptest! {
    #[test]
    fn prop_stock_movement_both_locations_some_roundtrip(
        item_sku: u64,
        from in warehouse_location_strategy(),
        to in warehouse_location_strategy(),
        quantity: u32,
    ) {
        let movement = StockMovement {
            item_sku,
            from_location: Some(from),
            to_location: Some(to),
            quantity,
        };
        let encoded = encode_to_vec(&movement).expect("encode failed");
        let (decoded, consumed): (StockMovement, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(movement, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 11: Deterministic encoding for InventoryItem
proptest! {
    #[test]
    fn prop_inventory_item_deterministic_encoding(item in inventory_item_strategy()) {
        let encoded_a = encode_to_vec(&item).expect("encode failed");
        let encoded_b = encode_to_vec(&item).expect("encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// Test 12: Deterministic encoding for StockMovement
proptest! {
    #[test]
    fn prop_stock_movement_deterministic_encoding(movement in stock_movement_strategy()) {
        let encoded_a = encode_to_vec(&movement).expect("encode failed");
        let encoded_b = encode_to_vec(&movement).expect("encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// Test 13: Consumed bytes == encoded length for InventoryItem
proptest! {
    #[test]
    fn prop_inventory_item_consumed_eq_len(item in inventory_item_strategy()) {
        let encoded = encode_to_vec(&item).expect("encode failed");
        let (_, consumed): (InventoryItem, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 14: Consumed bytes == encoded length for StockMovement
proptest! {
    #[test]
    fn prop_stock_movement_consumed_eq_len(movement in stock_movement_strategy()) {
        let encoded = encode_to_vec(&movement).expect("encode failed");
        let (_, consumed): (StockMovement, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 15: Distinct InventoryItems with different SKUs encode differently
proptest! {
    #[test]
    fn prop_inventory_items_different_sku_encode_differently(
        sku_a: u64,
        sku_b: u64,
        cat_idx in 0u8..5u8,
        quantity: u32,
        unit_price_cents: u64,
    ) {
        prop_assume!(sku_a != sku_b);
        let item_a = InventoryItem {
            sku: sku_a,
            category: item_category_from_u8(cat_idx),
            quantity,
            unit_price_cents,
        };
        let item_b = InventoryItem {
            sku: sku_b,
            category: item_category_from_u8(cat_idx),
            quantity,
            unit_price_cents,
        };
        let encoded_a = encode_to_vec(&item_a).expect("encode failed");
        let encoded_b = encode_to_vec(&item_b).expect("encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// Test 16: Distinct WarehouseLocations with different zones encode differently
proptest! {
    #[test]
    fn prop_warehouse_locations_different_zone_encode_differently(
        zone_a: u8,
        zone_b: u8,
        row: u8,
        shelf: u8,
    ) {
        prop_assume!(zone_a != zone_b);
        let loc_a = WarehouseLocation { zone: zone_a, row, shelf };
        let loc_b = WarehouseLocation { zone: zone_b, row, shelf };
        let encoded_a = encode_to_vec(&loc_a).expect("encode failed");
        let encoded_b = encode_to_vec(&loc_b).expect("encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// Test 17: u64 SKU full range roundtrip
proptest! {
    #[test]
    fn prop_sku_u64_full_range_roundtrip(sku: u64) {
        let encoded = encode_to_vec(&sku).expect("encode failed");
        let (decoded, consumed): (u64, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(decoded, sku);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 18: u32 quantity full range roundtrip
proptest! {
    #[test]
    fn prop_quantity_u32_full_range_roundtrip(quantity: u32) {
        let encoded = encode_to_vec(&quantity).expect("encode failed");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(decoded, quantity);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 19: InventoryItem with u64::MAX SKU and price roundtrip
proptest! {
    #[test]
    fn prop_inventory_item_max_values_roundtrip(
        cat_idx in 0u8..5u8,
        quantity: u32,
    ) {
        let item = InventoryItem {
            sku: u64::MAX,
            category: item_category_from_u8(cat_idx),
            quantity,
            unit_price_cents: u64::MAX,
        };
        let encoded = encode_to_vec(&item).expect("encode failed");
        let (decoded, consumed): (InventoryItem, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(item, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 20: Double encode/decode identity for InventoryItem
proptest! {
    #[test]
    fn prop_inventory_item_double_encode_decode_identity(item in inventory_item_strategy()) {
        let encoded_once = encode_to_vec(&item).expect("encode failed");
        let (decoded_once, _): (InventoryItem, usize) =
            decode_from_slice(&encoded_once).expect("decode failed");
        let encoded_twice = encode_to_vec(&decoded_once).expect("encode failed");
        let (decoded_twice, consumed): (InventoryItem, usize) =
            decode_from_slice(&encoded_twice).expect("decode failed");
        prop_assert_eq!(item, decoded_twice);
        prop_assert_eq!(consumed, encoded_twice.len());
    }
}

// Test 21: Double encode/decode identity for StockMovement
proptest! {
    #[test]
    fn prop_stock_movement_double_encode_decode_identity(movement in stock_movement_strategy()) {
        let encoded_once = encode_to_vec(&movement).expect("encode failed");
        let (decoded_once, _): (StockMovement, usize) =
            decode_from_slice(&encoded_once).expect("decode failed");
        let encoded_twice = encode_to_vec(&decoded_once).expect("encode failed");
        let (decoded_twice, consumed): (StockMovement, usize) =
            decode_from_slice(&encoded_twice).expect("decode failed");
        prop_assert_eq!(movement, decoded_twice);
        prop_assert_eq!(consumed, encoded_twice.len());
    }
}

// Test 22: Encoded bytes are non-empty for all four types
proptest! {
    #[test]
    fn prop_all_types_encoded_bytes_non_empty(
        item in inventory_item_strategy(),
        loc in warehouse_location_strategy(),
        movement in stock_movement_strategy(),
        cat_idx in 0u8..5u8,
    ) {
        let item_bytes = encode_to_vec(&item).expect("encode failed");
        let loc_bytes = encode_to_vec(&loc).expect("encode failed");
        let movement_bytes = encode_to_vec(&movement).expect("encode failed");
        let cat_bytes = encode_to_vec(&item_category_from_u8(cat_idx)).expect("encode failed");
        prop_assert!(!item_bytes.is_empty(), "InventoryItem encoded bytes must be non-empty");
        prop_assert!(!loc_bytes.is_empty(), "WarehouseLocation encoded bytes must be non-empty");
        prop_assert!(!movement_bytes.is_empty(), "StockMovement encoded bytes must be non-empty");
        prop_assert!(!cat_bytes.is_empty(), "ItemCategory encoded bytes must be non-empty");
    }
}
