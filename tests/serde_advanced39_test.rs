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
enum TransportMode {
    Air,
    Sea,
    Rail,
    Road,
    Multimodal,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ItemStatus {
    InWarehouse,
    InTransit,
    Delivered,
    Returned,
    Damaged,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TemperatureZone {
    Ambient,
    Chilled,
    Frozen,
    Controlled,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum HazmatClass {
    None,
    Flammable,
    Corrosive,
    Toxic,
    Explosive,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Sku {
    sku_id: u64,
    name: String,
    weight_g: u32,
    volume_cm3: u32,
    temperature_zone: TemperatureZone,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Warehouse {
    warehouse_id: u32,
    name: String,
    location: String,
    capacity_m3: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ShipmentItem {
    sku_id: u64,
    quantity: u32,
    status: ItemStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Shipment {
    shipment_id: u64,
    origin_id: u32,
    destination_id: u32,
    transport_mode: TransportMode,
    items: Vec<ShipmentItem>,
    created_at: u64,
    eta: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InventoryRecord {
    warehouse_id: u32,
    sku_id: u64,
    quantity: u32,
    last_updated: u64,
    hazmat: HazmatClass,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SupplyChainEvent {
    event_id: u64,
    shipment_id: u64,
    timestamp: u64,
    location: String,
    event_type: String,
}

// Test 1: Basic Sku roundtrip with standard config
#[test]
fn test_sku_standard_roundtrip() {
    let sku = Sku {
        sku_id: 100001,
        name: "Frozen Salmon Fillet".to_string(),
        weight_g: 500,
        volume_cm3: 600,
        temperature_zone: TemperatureZone::Frozen,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&sku, cfg).expect("encode Sku failed");
    let (decoded, _): (Sku, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Sku failed");
    assert_eq!(sku, decoded);
}

// Test 2: Sku with Ambient temperature zone
#[test]
fn test_sku_ambient_zone() {
    let sku = Sku {
        sku_id: 200002,
        name: "Dry Pasta Box".to_string(),
        weight_g: 1000,
        volume_cm3: 1200,
        temperature_zone: TemperatureZone::Ambient,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&sku, cfg).expect("encode Sku ambient failed");
    let (decoded, _): (Sku, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Sku ambient failed");
    assert_eq!(sku, decoded);
}

// Test 3: Warehouse roundtrip with big endian config
#[test]
fn test_warehouse_big_endian_roundtrip() {
    let warehouse = Warehouse {
        warehouse_id: 42,
        name: "Hamburg Distribution Center".to_string(),
        location: "Hamburg, Germany".to_string(),
        capacity_m3: 50000.0,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&warehouse, cfg).expect("encode Warehouse big_endian failed");
    let (decoded, _): (Warehouse, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Warehouse big_endian failed");
    assert_eq!(warehouse, decoded);
}

// Test 4: Shipment with Air transport and ETA present
#[test]
fn test_shipment_air_with_eta() {
    let shipment = Shipment {
        shipment_id: 9900001,
        origin_id: 1,
        destination_id: 7,
        transport_mode: TransportMode::Air,
        items: vec![
            ShipmentItem {
                sku_id: 100001,
                quantity: 20,
                status: ItemStatus::InTransit,
            },
            ShipmentItem {
                sku_id: 100002,
                quantity: 5,
                status: ItemStatus::InTransit,
            },
        ],
        created_at: 1700000000,
        eta: Some(1700086400),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment air failed");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment air failed");
    assert_eq!(shipment, decoded);
}

// Test 5: Shipment with Sea transport and no ETA (None)
#[test]
fn test_shipment_sea_no_eta() {
    let shipment = Shipment {
        shipment_id: 9900002,
        origin_id: 3,
        destination_id: 15,
        transport_mode: TransportMode::Sea,
        items: vec![ShipmentItem {
            sku_id: 200001,
            quantity: 500,
            status: ItemStatus::InWarehouse,
        }],
        created_at: 1700050000,
        eta: None,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment sea no eta failed");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment sea no eta failed");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.eta, None);
}

// Test 6: Shipment with empty items vec
#[test]
fn test_shipment_empty_items() {
    let shipment = Shipment {
        shipment_id: 9900003,
        origin_id: 5,
        destination_id: 9,
        transport_mode: TransportMode::Rail,
        items: vec![],
        created_at: 1700100000,
        eta: Some(1700200000),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment empty items failed");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment empty items failed");
    assert_eq!(shipment, decoded);
    assert!(decoded.items.is_empty());
}

// Test 7: InventoryRecord with hazmat Flammable, fixed int encoding
#[test]
fn test_inventory_hazmat_flammable_fixed_int() {
    let record = InventoryRecord {
        warehouse_id: 10,
        sku_id: 300001,
        quantity: 75,
        last_updated: 1700150000,
        hazmat: HazmatClass::Flammable,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec(&record, cfg).expect("encode InventoryRecord flammable fixed_int failed");
    let (decoded, _): (InventoryRecord, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode InventoryRecord flammable fixed_int failed");
    assert_eq!(record, decoded);
}

// Test 8: InventoryRecord with hazmat None
#[test]
fn test_inventory_hazmat_none() {
    let record = InventoryRecord {
        warehouse_id: 20,
        sku_id: 400002,
        quantity: 1500,
        last_updated: 1700200000,
        hazmat: HazmatClass::None,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&record, cfg).expect("encode InventoryRecord hazmat none failed");
    let (decoded, _): (InventoryRecord, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode InventoryRecord hazmat none failed");
    assert_eq!(record, decoded);
}

// Test 9: InventoryRecord with hazmat Explosive, big endian
#[test]
fn test_inventory_hazmat_explosive_big_endian() {
    let record = InventoryRecord {
        warehouse_id: 99,
        sku_id: 500003,
        quantity: 10,
        last_updated: 1700250000,
        hazmat: HazmatClass::Explosive,
    };
    let cfg = config::standard().with_big_endian();
    let bytes =
        encode_to_vec(&record, cfg).expect("encode InventoryRecord explosive big_endian failed");
    let (decoded, _): (InventoryRecord, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode InventoryRecord explosive big_endian failed");
    assert_eq!(record, decoded);
}

// Test 10: SupplyChainEvent roundtrip
#[test]
fn test_supply_chain_event_roundtrip() {
    let event = SupplyChainEvent {
        event_id: 1,
        shipment_id: 9900001,
        timestamp: 1700060000,
        location: "Frankfurt Airport, Germany".to_string(),
        event_type: "CUSTOMS_CLEARANCE".to_string(),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&event, cfg).expect("encode SupplyChainEvent failed");
    let (decoded, _): (SupplyChainEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SupplyChainEvent failed");
    assert_eq!(event, decoded);
}

// Test 11: SupplyChainEvent with fixed int encoding
#[test]
fn test_supply_chain_event_fixed_int() {
    let event = SupplyChainEvent {
        event_id: 55,
        shipment_id: 9900010,
        timestamp: 1700070000,
        location: "Rotterdam Port, Netherlands".to_string(),
        event_type: "VESSEL_DEPARTURE".to_string(),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&event, cfg).expect("encode SupplyChainEvent fixed_int failed");
    let (decoded, _): (SupplyChainEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SupplyChainEvent fixed_int failed");
    assert_eq!(event, decoded);
}

// Test 12: Vec of multiple Shipments roundtrip
#[test]
fn test_vec_of_shipments_roundtrip() {
    let shipments = vec![
        Shipment {
            shipment_id: 1001,
            origin_id: 1,
            destination_id: 2,
            transport_mode: TransportMode::Road,
            items: vec![ShipmentItem {
                sku_id: 10,
                quantity: 100,
                status: ItemStatus::Delivered,
            }],
            created_at: 1700000000,
            eta: Some(1700010000),
        },
        Shipment {
            shipment_id: 1002,
            origin_id: 3,
            destination_id: 4,
            transport_mode: TransportMode::Multimodal,
            items: vec![
                ShipmentItem {
                    sku_id: 20,
                    quantity: 50,
                    status: ItemStatus::InTransit,
                },
                ShipmentItem {
                    sku_id: 21,
                    quantity: 30,
                    status: ItemStatus::InTransit,
                },
            ],
            created_at: 1700020000,
            eta: None,
        },
        Shipment {
            shipment_id: 1003,
            origin_id: 5,
            destination_id: 6,
            transport_mode: TransportMode::Air,
            items: vec![ShipmentItem {
                sku_id: 30,
                quantity: 5,
                status: ItemStatus::Returned,
            }],
            created_at: 1700030000,
            eta: Some(1700040000),
        },
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&shipments, cfg).expect("encode Vec<Shipment> failed");
    let (decoded, _): (Vec<Shipment>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Shipment> failed");
    assert_eq!(shipments, decoded);
    assert_eq!(decoded.len(), 3);
}

// Test 13: All TransportMode variants roundtrip
#[test]
fn test_all_transport_modes() {
    let modes = vec![
        TransportMode::Air,
        TransportMode::Sea,
        TransportMode::Rail,
        TransportMode::Road,
        TransportMode::Multimodal,
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&modes, cfg).expect("encode TransportMode variants failed");
    let (decoded, _): (Vec<TransportMode>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransportMode variants failed");
    assert_eq!(modes, decoded);
}

// Test 14: All ItemStatus variants roundtrip
#[test]
fn test_all_item_statuses() {
    let statuses = vec![
        ItemStatus::InWarehouse,
        ItemStatus::InTransit,
        ItemStatus::Delivered,
        ItemStatus::Returned,
        ItemStatus::Damaged,
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&statuses, cfg).expect("encode ItemStatus variants failed");
    let (decoded, _): (Vec<ItemStatus>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ItemStatus variants failed");
    assert_eq!(statuses, decoded);
}

// Test 15: All TemperatureZone variants roundtrip
#[test]
fn test_all_temperature_zones() {
    let zones = vec![
        TemperatureZone::Ambient,
        TemperatureZone::Chilled,
        TemperatureZone::Frozen,
        TemperatureZone::Controlled,
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&zones, cfg).expect("encode TemperatureZone variants failed");
    let (decoded, _): (Vec<TemperatureZone>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TemperatureZone variants failed");
    assert_eq!(zones, decoded);
}

// Test 16: All HazmatClass variants roundtrip
#[test]
fn test_all_hazmat_classes() {
    let classes = vec![
        HazmatClass::None,
        HazmatClass::Flammable,
        HazmatClass::Corrosive,
        HazmatClass::Toxic,
        HazmatClass::Explosive,
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&classes, cfg).expect("encode HazmatClass variants failed");
    let (decoded, _): (Vec<HazmatClass>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HazmatClass variants failed");
    assert_eq!(classes, decoded);
}

// Test 17: Large event log with 100+ SupplyChainEvents
#[test]
fn test_large_event_log_roundtrip() {
    let events: Vec<SupplyChainEvent> = (0u64..120)
        .map(|i| SupplyChainEvent {
            event_id: i + 1,
            shipment_id: 9900000 + (i % 10),
            timestamp: 1700000000 + i * 3600,
            location: format!("Checkpoint-{}", i % 25),
            event_type: match i % 5 {
                0 => "DEPARTURE".to_string(),
                1 => "ARRIVAL".to_string(),
                2 => "CUSTOMS_CLEARANCE".to_string(),
                3 => "DELAY_REPORTED".to_string(),
                _ => "SCAN".to_string(),
            },
        })
        .collect();
    assert_eq!(events.len(), 120);
    let cfg = config::standard();
    let bytes = encode_to_vec(&events, cfg).expect("encode large event log failed");
    let (decoded, _): (Vec<SupplyChainEvent>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large event log failed");
    assert_eq!(events.len(), decoded.len());
    assert_eq!(events, decoded);
}

// Test 18: Shipment with Damaged item status and Multimodal transport
#[test]
fn test_shipment_damaged_items_multimodal() {
    let shipment = Shipment {
        shipment_id: 8800001,
        origin_id: 11,
        destination_id: 22,
        transport_mode: TransportMode::Multimodal,
        items: vec![
            ShipmentItem {
                sku_id: 600001,
                quantity: 3,
                status: ItemStatus::Damaged,
            },
            ShipmentItem {
                sku_id: 600002,
                quantity: 10,
                status: ItemStatus::Delivered,
            },
        ],
        created_at: 1700300000,
        eta: Some(1700400000),
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment damaged multimodal failed");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment damaged multimodal failed");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.items[0].status, ItemStatus::Damaged);
}

// Test 19: Multiple InventoryRecords with mixed hazmat, big endian
#[test]
fn test_vec_inventory_records_big_endian() {
    let records = vec![
        InventoryRecord {
            warehouse_id: 1,
            sku_id: 700001,
            quantity: 200,
            last_updated: 1700310000,
            hazmat: HazmatClass::None,
        },
        InventoryRecord {
            warehouse_id: 1,
            sku_id: 700002,
            quantity: 50,
            last_updated: 1700320000,
            hazmat: HazmatClass::Corrosive,
        },
        InventoryRecord {
            warehouse_id: 2,
            sku_id: 700003,
            quantity: 800,
            last_updated: 1700330000,
            hazmat: HazmatClass::Toxic,
        },
        InventoryRecord {
            warehouse_id: 3,
            sku_id: 700004,
            quantity: 15,
            last_updated: 1700340000,
            hazmat: HazmatClass::Flammable,
        },
    ];
    let cfg = config::standard().with_big_endian();
    let bytes =
        encode_to_vec(&records, cfg).expect("encode Vec<InventoryRecord> big_endian failed");
    let (decoded, _): (Vec<InventoryRecord>, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode Vec<InventoryRecord> big_endian failed");
    assert_eq!(records, decoded);
    assert_eq!(decoded[1].hazmat, HazmatClass::Corrosive);
    assert_eq!(decoded[2].hazmat, HazmatClass::Toxic);
}

// Test 20: Sku with Controlled temperature zone and fixed int config
#[test]
fn test_sku_controlled_zone_fixed_int() {
    let sku = Sku {
        sku_id: 800001,
        name: "Pharmaceutical Compound X".to_string(),
        weight_g: 250,
        volume_cm3: 300,
        temperature_zone: TemperatureZone::Controlled,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&sku, cfg).expect("encode Sku controlled zone fixed_int failed");
    let (decoded, _): (Sku, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Sku controlled zone fixed_int failed");
    assert_eq!(sku, decoded);
    assert_eq!(decoded.temperature_zone, TemperatureZone::Controlled);
}

// Test 21: Shipment eta Some vs None comparison
#[test]
fn test_shipment_eta_some_vs_none_encoding_differs() {
    let shipment_with_eta = Shipment {
        shipment_id: 7700001,
        origin_id: 30,
        destination_id: 31,
        transport_mode: TransportMode::Road,
        items: vec![ShipmentItem {
            sku_id: 900001,
            quantity: 1,
            status: ItemStatus::InTransit,
        }],
        created_at: 1700500000,
        eta: Some(1700600000),
    };
    let shipment_no_eta = Shipment {
        shipment_id: 7700002,
        origin_id: 30,
        destination_id: 31,
        transport_mode: TransportMode::Road,
        items: vec![ShipmentItem {
            sku_id: 900001,
            quantity: 1,
            status: ItemStatus::InTransit,
        }],
        created_at: 1700500000,
        eta: None,
    };
    let cfg = config::standard();
    let bytes_with_eta =
        encode_to_vec(&shipment_with_eta, cfg).expect("encode Shipment with eta failed");
    let bytes_no_eta = encode_to_vec(&shipment_no_eta, cfg).expect("encode Shipment no eta failed");
    assert_ne!(
        bytes_with_eta, bytes_no_eta,
        "Encodings of Some(eta) and None should differ"
    );
    let (decoded_with_eta, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes_with_eta, cfg).expect("decode Shipment with eta failed");
    let (decoded_no_eta, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes_no_eta, cfg).expect("decode Shipment no eta failed");
    assert_eq!(decoded_with_eta.eta, Some(1700600000));
    assert_eq!(decoded_no_eta.eta, None);
}

// Test 22: Full supply chain scenario — Warehouse, Sku, Shipment, Events
#[test]
fn test_full_supply_chain_scenario() {
    let warehouse = Warehouse {
        warehouse_id: 1,
        name: "Tokyo Logistics Hub".to_string(),
        location: "Tokyo, Japan".to_string(),
        capacity_m3: 120000.0,
    };
    let sku = Sku {
        sku_id: 1000001,
        name: "Electronic Component PCB-7".to_string(),
        weight_g: 150,
        volume_cm3: 200,
        temperature_zone: TemperatureZone::Ambient,
    };
    let shipment = Shipment {
        shipment_id: 6600001,
        origin_id: 1,
        destination_id: 2,
        transport_mode: TransportMode::Air,
        items: vec![ShipmentItem {
            sku_id: 1000001,
            quantity: 500,
            status: ItemStatus::InTransit,
        }],
        created_at: 1700700000,
        eta: Some(1700786400),
    };
    let events: Vec<SupplyChainEvent> = vec![
        SupplyChainEvent {
            event_id: 1,
            shipment_id: 6600001,
            timestamp: 1700700000,
            location: "Tokyo Logistics Hub".to_string(),
            event_type: "SHIPMENT_CREATED".to_string(),
        },
        SupplyChainEvent {
            event_id: 2,
            shipment_id: 6600001,
            timestamp: 1700710000,
            location: "Narita International Airport".to_string(),
            event_type: "DEPARTURE".to_string(),
        },
        SupplyChainEvent {
            event_id: 3,
            shipment_id: 6600001,
            timestamp: 1700760000,
            location: "Los Angeles International Airport".to_string(),
            event_type: "ARRIVAL".to_string(),
        },
    ];
    let cfg = config::standard();
    let wh_bytes = encode_to_vec(&warehouse, cfg).expect("encode Warehouse scenario failed");
    let sku_bytes = encode_to_vec(&sku, cfg).expect("encode Sku scenario failed");
    let ship_bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment scenario failed");
    let ev_bytes = encode_to_vec(&events, cfg).expect("encode events scenario failed");
    let (dec_wh, _): (Warehouse, usize) =
        decode_owned_from_slice(&wh_bytes, cfg).expect("decode Warehouse scenario failed");
    let (dec_sku, _): (Sku, usize) =
        decode_owned_from_slice(&sku_bytes, cfg).expect("decode Sku scenario failed");
    let (dec_ship, _): (Shipment, usize) =
        decode_owned_from_slice(&ship_bytes, cfg).expect("decode Shipment scenario failed");
    let (dec_ev, _): (Vec<SupplyChainEvent>, usize) =
        decode_owned_from_slice(&ev_bytes, cfg).expect("decode events scenario failed");
    assert_eq!(warehouse, dec_wh);
    assert_eq!(sku, dec_sku);
    assert_eq!(shipment, dec_ship);
    assert_eq!(events, dec_ev);
    assert_eq!(dec_ship.transport_mode, TransportMode::Air);
    assert_eq!(dec_ship.items[0].status, ItemStatus::InTransit);
    assert_eq!(dec_wh.capacity_m3, 120000.0);
    assert_eq!(dec_ev.len(), 3);
}
