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
enum StorageClass {
    Ambient,
    Refrigerated,
    Frozen,
    Hazardous,
    Fragile,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum OrderStatus {
    Pending,
    Confirmed,
    Picking,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Product {
    sku: String,
    name: String,
    weight_g: u32,
    storage_class: StorageClass,
    unit_cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InventoryItem {
    product_sku: String,
    warehouse_id: u32,
    quantity: u32,
    reorder_point: u32,
    reserved: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PurchaseOrder {
    order_id: u64,
    supplier_id: u32,
    status: OrderStatus,
    items: Vec<(String, u32)>,
    total_cents: u64,
    created_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Warehouse {
    warehouse_id: u32,
    name: String,
    capacity_units: u32,
    inventory: Vec<InventoryItem>,
}

// --- StorageClass variant tests ---

#[test]
fn test_storage_class_ambient_roundtrip() {
    let cfg = config::standard();
    let val = StorageClass::Ambient;
    let bytes = encode_to_vec(&val, cfg).expect("encode StorageClass::Ambient");
    let (decoded, _): (StorageClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode StorageClass::Ambient");
    assert_eq!(val, decoded);
}

#[test]
fn test_storage_class_refrigerated_roundtrip() {
    let cfg = config::standard();
    let val = StorageClass::Refrigerated;
    let bytes = encode_to_vec(&val, cfg).expect("encode StorageClass::Refrigerated");
    let (decoded, _): (StorageClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode StorageClass::Refrigerated");
    assert_eq!(val, decoded);
}

#[test]
fn test_storage_class_frozen_roundtrip() {
    let cfg = config::standard();
    let val = StorageClass::Frozen;
    let bytes = encode_to_vec(&val, cfg).expect("encode StorageClass::Frozen");
    let (decoded, _): (StorageClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode StorageClass::Frozen");
    assert_eq!(val, decoded);
}

#[test]
fn test_storage_class_hazardous_roundtrip() {
    let cfg = config::standard();
    let val = StorageClass::Hazardous;
    let bytes = encode_to_vec(&val, cfg).expect("encode StorageClass::Hazardous");
    let (decoded, _): (StorageClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode StorageClass::Hazardous");
    assert_eq!(val, decoded);
}

#[test]
fn test_storage_class_fragile_roundtrip() {
    let cfg = config::standard();
    let val = StorageClass::Fragile;
    let bytes = encode_to_vec(&val, cfg).expect("encode StorageClass::Fragile");
    let (decoded, _): (StorageClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode StorageClass::Fragile");
    assert_eq!(val, decoded);
}

// --- OrderStatus variant tests ---

#[test]
fn test_order_status_pending_roundtrip() {
    let cfg = config::standard();
    let val = OrderStatus::Pending;
    let bytes = encode_to_vec(&val, cfg).expect("encode OrderStatus::Pending");
    let (decoded, _): (OrderStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrderStatus::Pending");
    assert_eq!(val, decoded);
}

#[test]
fn test_order_status_confirmed_roundtrip() {
    let cfg = config::standard();
    let val = OrderStatus::Confirmed;
    let bytes = encode_to_vec(&val, cfg).expect("encode OrderStatus::Confirmed");
    let (decoded, _): (OrderStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrderStatus::Confirmed");
    assert_eq!(val, decoded);
}

#[test]
fn test_order_status_picking_roundtrip() {
    let cfg = config::standard();
    let val = OrderStatus::Picking;
    let bytes = encode_to_vec(&val, cfg).expect("encode OrderStatus::Picking");
    let (decoded, _): (OrderStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrderStatus::Picking");
    assert_eq!(val, decoded);
}

#[test]
fn test_order_status_shipped_roundtrip() {
    let cfg = config::standard();
    let val = OrderStatus::Shipped;
    let bytes = encode_to_vec(&val, cfg).expect("encode OrderStatus::Shipped");
    let (decoded, _): (OrderStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrderStatus::Shipped");
    assert_eq!(val, decoded);
}

#[test]
fn test_order_status_delivered_roundtrip() {
    let cfg = config::standard();
    let val = OrderStatus::Delivered;
    let bytes = encode_to_vec(&val, cfg).expect("encode OrderStatus::Delivered");
    let (decoded, _): (OrderStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrderStatus::Delivered");
    assert_eq!(val, decoded);
}

#[test]
fn test_order_status_cancelled_roundtrip() {
    let cfg = config::standard();
    let val = OrderStatus::Cancelled;
    let bytes = encode_to_vec(&val, cfg).expect("encode OrderStatus::Cancelled");
    let (decoded, _): (OrderStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OrderStatus::Cancelled");
    assert_eq!(val, decoded);
}

// --- Struct roundtrip tests ---

#[test]
fn test_product_roundtrip_standard() {
    let cfg = config::standard();
    let val = Product {
        sku: "SKU-001".to_string(),
        name: "Widget Alpha".to_string(),
        weight_g: 250,
        storage_class: StorageClass::Ambient,
        unit_cost_cents: 1999,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Product");
    let (decoded, _): (Product, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Product");
    assert_eq!(val, decoded);
}

#[test]
fn test_inventory_item_roundtrip() {
    let cfg = config::standard();
    let val = InventoryItem {
        product_sku: "SKU-002".to_string(),
        warehouse_id: 7,
        quantity: 500,
        reorder_point: 50,
        reserved: 20,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode InventoryItem");
    let (decoded, _): (InventoryItem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InventoryItem");
    assert_eq!(val, decoded);
}

#[test]
fn test_purchase_order_empty_items() {
    let cfg = config::standard();
    let val = PurchaseOrder {
        order_id: 1001,
        supplier_id: 42,
        status: OrderStatus::Pending,
        items: vec![],
        total_cents: 0,
        created_at: 1_700_000_000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PurchaseOrder empty items");
    let (decoded, _): (PurchaseOrder, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PurchaseOrder empty items");
    assert_eq!(val, decoded);
}

#[test]
fn test_purchase_order_five_items() {
    let cfg = config::standard();
    let val = PurchaseOrder {
        order_id: 2002,
        supplier_id: 99,
        status: OrderStatus::Confirmed,
        items: vec![
            ("SKU-001".to_string(), 10),
            ("SKU-002".to_string(), 25),
            ("SKU-003".to_string(), 5),
            ("SKU-004".to_string(), 100),
            ("SKU-005".to_string(), 3),
        ],
        total_cents: 148_750,
        created_at: 1_710_000_000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PurchaseOrder 5 items");
    let (decoded, _): (PurchaseOrder, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PurchaseOrder 5 items");
    assert_eq!(val, decoded);
}

#[test]
fn test_warehouse_empty_inventory() {
    let cfg = config::standard();
    let val = Warehouse {
        warehouse_id: 1,
        name: "Central Hub".to_string(),
        capacity_units: 10_000,
        inventory: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Warehouse empty inventory");
    let (decoded, _): (Warehouse, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Warehouse empty inventory");
    assert_eq!(val, decoded);
}

#[test]
fn test_warehouse_ten_inventory_items() {
    let cfg = config::standard();
    let inventory: Vec<InventoryItem> = (0..10)
        .map(|i| InventoryItem {
            product_sku: format!("SKU-{:03}", i),
            warehouse_id: 2,
            quantity: (i + 1) * 100,
            reorder_point: 20,
            reserved: i * 5,
        })
        .collect();
    let val = Warehouse {
        warehouse_id: 2,
        name: "East Wing".to_string(),
        capacity_units: 5_000,
        inventory,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Warehouse 10 items");
    let (decoded, _): (Warehouse, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Warehouse 10 items");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_product_roundtrip() {
    let cfg = config::standard();
    let val: Vec<Product> = vec![
        Product {
            sku: "SKU-A".to_string(),
            name: "Frozen Fish".to_string(),
            weight_g: 800,
            storage_class: StorageClass::Frozen,
            unit_cost_cents: 3500,
        },
        Product {
            sku: "SKU-B".to_string(),
            name: "Battery Pack".to_string(),
            weight_g: 350,
            storage_class: StorageClass::Hazardous,
            unit_cost_cents: 12_000,
        },
        Product {
            sku: "SKU-C".to_string(),
            name: "Crystal Vase".to_string(),
            weight_g: 600,
            storage_class: StorageClass::Fragile,
            unit_cost_cents: 7_500,
        },
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<Product>");
    let (decoded, _): (Vec<Product>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Product>");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_purchase_order_roundtrip() {
    let cfg = config::standard();
    let val: Vec<PurchaseOrder> = vec![
        PurchaseOrder {
            order_id: 3001,
            supplier_id: 10,
            status: OrderStatus::Shipped,
            items: vec![("SKU-X".to_string(), 50)],
            total_cents: 25_000,
            created_at: 1_720_000_000,
        },
        PurchaseOrder {
            order_id: 3002,
            supplier_id: 10,
            status: OrderStatus::Delivered,
            items: vec![("SKU-Y".to_string(), 200), ("SKU-Z".to_string(), 75)],
            total_cents: 98_750,
            created_at: 1_720_100_000,
        },
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<PurchaseOrder>");
    let (decoded, _): (Vec<PurchaseOrder>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<PurchaseOrder>");
    assert_eq!(val, decoded);
}

// --- Config variant tests ---

#[test]
fn test_big_endian_config_product_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = Product {
        sku: "BE-SKU".to_string(),
        name: "Big Endian Product".to_string(),
        weight_g: 1024,
        storage_class: StorageClass::Refrigerated,
        unit_cost_cents: 59_99,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Product big_endian");
    let (decoded, _): (Product, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Product big_endian");
    assert_eq!(val, decoded);
}

#[test]
fn test_fixed_int_config_inventory_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = InventoryItem {
        product_sku: "FI-SKU".to_string(),
        warehouse_id: 99,
        quantity: 1_000_000,
        reorder_point: 100,
        reserved: 0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode InventoryItem fixed_int");
    let (decoded, _): (InventoryItem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InventoryItem fixed_int");
    assert_eq!(val, decoded);
}

#[test]
fn test_big_endian_fixed_int_combined_order_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val = PurchaseOrder {
        order_id: u64::MAX / 2,
        supplier_id: u32::MAX / 2,
        status: OrderStatus::Picking,
        items: vec![("COMBINED-SKU".to_string(), u32::MAX / 4)],
        total_cents: 999_999_999,
        created_at: 1_730_000_000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PurchaseOrder big_endian+fixed_int");
    let (decoded, _): (PurchaseOrder, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PurchaseOrder big_endian+fixed_int");
    assert_eq!(val, decoded);
}

#[test]
fn test_consumed_bytes_equals_buffer_length() {
    let cfg = config::standard();
    let val = Warehouse {
        warehouse_id: 55,
        name: "Bytes Check Warehouse".to_string(),
        capacity_units: 8_000,
        inventory: vec![InventoryItem {
            product_sku: "CHK-001".to_string(),
            warehouse_id: 55,
            quantity: 300,
            reorder_point: 30,
            reserved: 10,
        }],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Warehouse for consumed bytes check");
    let (_decoded, consumed): (Warehouse, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Warehouse for consumed bytes check");
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_large_warehouse_100_inventory_items() {
    let cfg = config::standard();
    let inventory: Vec<InventoryItem> = (0..100)
        .map(|i| InventoryItem {
            product_sku: format!("LARGE-SKU-{:04}", i),
            warehouse_id: 100,
            quantity: i * 10 + 1,
            reorder_point: 5,
            reserved: i % 3,
        })
        .collect();
    let val = Warehouse {
        warehouse_id: 100,
        name: "Mega Distribution Center".to_string(),
        capacity_units: 500_000,
        inventory,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode large Warehouse 100 items");
    let (decoded, consumed): (Warehouse, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large Warehouse 100 items");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.inventory.len(), 100);
}

#[test]
fn test_cancelled_order_roundtrip() {
    let cfg = config::standard();
    let val = PurchaseOrder {
        order_id: 9_001,
        supplier_id: 15,
        status: OrderStatus::Cancelled,
        items: vec![("CANCEL-A".to_string(), 50), ("CANCEL-B".to_string(), 30)],
        total_cents: 0,
        created_at: 1_705_000_000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode cancelled order");
    let (decoded, _): (PurchaseOrder, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode cancelled order");
    assert_eq!(val, decoded);
    assert_eq!(decoded.status, OrderStatus::Cancelled);
}

#[test]
fn test_fully_delivered_order_chain() {
    let cfg = config::standard();
    // Simulate a chain: same order serialized at each status transition, verify final state
    let statuses = [
        OrderStatus::Pending,
        OrderStatus::Confirmed,
        OrderStatus::Picking,
        OrderStatus::Shipped,
        OrderStatus::Delivered,
    ];
    for status in &statuses {
        let val = PurchaseOrder {
            order_id: 7_777,
            supplier_id: 33,
            status: status.clone(),
            items: vec![("CHAIN-SKU".to_string(), 1)],
            total_cents: 4_999,
            created_at: 1_715_000_000,
        };
        let bytes = encode_to_vec(&val, cfg).expect("encode order chain step");
        let (decoded, _): (PurchaseOrder, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode order chain step");
        assert_eq!(val, decoded);
    }
}

#[test]
fn test_product_with_zero_weight() {
    let cfg = config::standard();
    let val = Product {
        sku: "DIGITAL-001".to_string(),
        name: "Digital License Key".to_string(),
        weight_g: 0,
        storage_class: StorageClass::Ambient,
        unit_cost_cents: 4_999,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Product zero weight");
    let (decoded, _): (Product, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Product zero weight");
    assert_eq!(val, decoded);
    assert_eq!(decoded.weight_g, 0);
}

#[test]
fn test_reorder_point_boundary_at_quantity() {
    let cfg = config::standard();
    // reorder_point exactly equals quantity — boundary condition
    let val = InventoryItem {
        product_sku: "BOUNDARY-SKU".to_string(),
        warehouse_id: 3,
        quantity: 50,
        reorder_point: 50,
        reserved: 0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode InventoryItem reorder boundary");
    let (decoded, _): (InventoryItem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InventoryItem reorder boundary");
    assert_eq!(val, decoded);
    assert_eq!(decoded.quantity, decoded.reorder_point);
}

#[test]
fn test_reserved_exceeds_quantity_scenario() {
    let cfg = config::standard();
    // reserved > quantity is a valid data state (over-reservation edge case)
    let val = InventoryItem {
        product_sku: "OVERRES-SKU".to_string(),
        warehouse_id: 4,
        quantity: 10,
        reorder_point: 5,
        reserved: 15,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode InventoryItem reserved>quantity");
    let (decoded, _): (InventoryItem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InventoryItem reserved>quantity");
    assert_eq!(val, decoded);
    assert!(decoded.reserved > decoded.quantity);
}

#[test]
fn test_supplier_order_history_five_orders() {
    let cfg = config::standard();
    let supplier_id = 77_u32;
    let orders: Vec<PurchaseOrder> = (0..5)
        .map(|i| PurchaseOrder {
            order_id: 10_000 + i as u64,
            supplier_id,
            status: match i % 5 {
                0 => OrderStatus::Delivered,
                1 => OrderStatus::Delivered,
                2 => OrderStatus::Shipped,
                3 => OrderStatus::Confirmed,
                _ => OrderStatus::Pending,
            },
            items: (0..=(i as usize))
                .map(|j| {
                    (
                        format!("SUP-{}-SKU-{}", supplier_id, j),
                        (j as u32 + 1) * 10,
                    )
                })
                .collect(),
            total_cents: (i as u64 + 1) * 50_000,
            created_at: 1_700_000_000 + i as u64 * 86_400,
        })
        .collect();
    let bytes = encode_to_vec(&orders, cfg).expect("encode supplier order history");
    let (decoded, consumed): (Vec<PurchaseOrder>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode supplier order history");
    assert_eq!(orders, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 5);
    assert!(decoded.iter().all(|o| o.supplier_id == supplier_id));
}

#[test]
fn test_multi_warehouse_inventory() {
    let cfg = config::standard();
    let warehouses: Vec<Warehouse> = vec![
        Warehouse {
            warehouse_id: 1,
            name: "North Facility".to_string(),
            capacity_units: 20_000,
            inventory: vec![
                InventoryItem {
                    product_sku: "NORTH-001".to_string(),
                    warehouse_id: 1,
                    quantity: 1_500,
                    reorder_point: 200,
                    reserved: 100,
                },
                InventoryItem {
                    product_sku: "NORTH-002".to_string(),
                    warehouse_id: 1,
                    quantity: 750,
                    reorder_point: 100,
                    reserved: 50,
                },
            ],
        },
        Warehouse {
            warehouse_id: 2,
            name: "South Facility".to_string(),
            capacity_units: 15_000,
            inventory: vec![InventoryItem {
                product_sku: "SOUTH-001".to_string(),
                warehouse_id: 2,
                quantity: 2_000,
                reorder_point: 300,
                reserved: 0,
            }],
        },
        Warehouse {
            warehouse_id: 3,
            name: "Cold Storage".to_string(),
            capacity_units: 5_000,
            inventory: vec![
                InventoryItem {
                    product_sku: "COLD-001".to_string(),
                    warehouse_id: 3,
                    quantity: 400,
                    reorder_point: 50,
                    reserved: 10,
                },
                InventoryItem {
                    product_sku: "COLD-002".to_string(),
                    warehouse_id: 3,
                    quantity: 200,
                    reorder_point: 25,
                    reserved: 5,
                },
                InventoryItem {
                    product_sku: "COLD-003".to_string(),
                    warehouse_id: 3,
                    quantity: 600,
                    reorder_point: 75,
                    reserved: 30,
                },
            ],
        },
    ];
    let bytes = encode_to_vec(&warehouses, cfg).expect("encode multi-warehouse inventory");
    let (decoded, consumed): (Vec<Warehouse>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode multi-warehouse inventory");
    assert_eq!(warehouses, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 3);
    let total_items: usize = decoded.iter().map(|w| w.inventory.len()).sum();
    assert_eq!(total_items, 6);
}
