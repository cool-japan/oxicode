//! Advanced LZ4 compression tests for the logistics network optimization domain.
//!
//! Covers delivery routes, warehouse inventory, shipment tracking, vehicle fleet status,
//! package dimensions/weight, customs declarations, carrier rates, last-mile delivery,
//! cold chain logistics, reverse logistics, cross-docking operations, and freight forwarding.
#![cfg(feature = "compression-lz4")]

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct GeoCoordinate {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeliveryRoute {
    route_id: u64,
    origin: GeoCoordinate,
    destination: GeoCoordinate,
    waypoints: Vec<GeoCoordinate>,
    distance_km: f64,
    estimated_minutes: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WarehouseInventoryItem {
    sku: String,
    quantity: u32,
    bin_location: String,
    weight_kg: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WarehouseInventory {
    warehouse_id: u32,
    region: String,
    items: Vec<WarehouseInventoryItem>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ShipmentStatus {
    Pending,
    PickedUp,
    InTransit { hub: String, eta_hours: u32 },
    OutForDelivery,
    Delivered { proof_of_delivery: String },
    Failed { reason: String },
    Returned,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ShipmentTracking {
    tracking_number: String,
    status: ShipmentStatus,
    carrier: String,
    last_update_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum VehicleType {
    Bicycle,
    Motorcycle,
    Van,
    Truck { axles: u8 },
    RefrigeratedTruck { min_temp_celsius: i16 },
    DroneDelivery { max_payload_kg: f32 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FleetVehicle {
    vehicle_id: u64,
    vehicle_type: VehicleType,
    license_plate: String,
    current_location: GeoCoordinate,
    fuel_percent: u8,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PackageDimensions {
    length_cm: f32,
    width_cm: f32,
    height_cm: f32,
    weight_kg: f64,
    fragile: bool,
    hazmat: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CustomsDeclaration {
    declaration_id: String,
    origin_country: String,
    destination_country: String,
    declared_value_usd: f64,
    currency: String,
    goods_description: String,
    hs_code: String,
    duties_paid: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CarrierRate {
    carrier_id: u32,
    carrier_name: String,
    service_level: String,
    base_rate_usd: f64,
    per_kg_rate_usd: f64,
    fuel_surcharge_pct: f32,
    max_weight_kg: f64,
    transit_days: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LastMileDelivery {
    attempt_number: u8,
    driver_id: u64,
    vehicle_id: u64,
    package_ids: Vec<String>,
    route: DeliveryRoute,
    customer_contacted: bool,
    signature_required: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColdChainRecord {
    shipment_id: String,
    product: String,
    required_min_celsius: i16,
    required_max_celsius: i16,
    temperature_log: Vec<f32>,
    breach_detected: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ReturnReason {
    CustomerRefusal,
    DamagedGoods,
    WrongItem,
    AddressNotFound,
    RecipientUnavailable,
    Other(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReverseLogisticsEntry {
    return_id: String,
    original_shipment: String,
    reason: ReturnReason,
    refund_initiated: bool,
    restocking_eligible: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CrossDockOperation {
    dock_id: u32,
    inbound_carrier: String,
    outbound_carrier: String,
    transfer_window_minutes: u16,
    pallet_count: u32,
    items_transferred: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FreightForwarderBill {
    bill_id: String,
    shipper: String,
    consignee: String,
    origin_port: String,
    destination_port: String,
    total_weight_kg: f64,
    total_volume_m3: f64,
    freight_charges_usd: f64,
    containers: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 1 – basic delivery route round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_delivery_route_roundtrip() {
    let route = DeliveryRoute {
        route_id: 100_001,
        origin: GeoCoordinate {
            latitude: 59.4370,
            longitude: 24.7536,
        },
        destination: GeoCoordinate {
            latitude: 56.9496,
            longitude: 24.1052,
        },
        waypoints: vec![
            GeoCoordinate {
                latitude: 58.3780,
                longitude: 26.7290,
            },
            GeoCoordinate {
                latitude: 57.8132,
                longitude: 25.6812,
            },
        ],
        distance_km: 312.7,
        estimated_minutes: 210,
    };

    let encoded = encode_to_vec(&route).expect("encode DeliveryRoute failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress DeliveryRoute failed");
    let decompressed = decompress(&compressed).expect("decompress DeliveryRoute failed");
    let (decoded, _): (DeliveryRoute, usize) =
        decode_from_slice(&decompressed).expect("decode DeliveryRoute failed");

    assert_eq!(route, decoded);
}

// ---------------------------------------------------------------------------
// Test 2 – warehouse inventory round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_warehouse_inventory_roundtrip() {
    let inventory = WarehouseInventory {
        warehouse_id: 7,
        region: "Northern Europe".to_string(),
        items: vec![
            WarehouseInventoryItem {
                sku: "SKU-00123".to_string(),
                quantity: 500,
                bin_location: "A-03-12".to_string(),
                weight_kg: 1.5,
            },
            WarehouseInventoryItem {
                sku: "SKU-00456".to_string(),
                quantity: 200,
                bin_location: "B-07-04".to_string(),
                weight_kg: 0.25,
            },
        ],
    };

    let encoded = encode_to_vec(&inventory).expect("encode WarehouseInventory failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress WarehouseInventory failed");
    let decompressed = decompress(&compressed).expect("decompress WarehouseInventory failed");
    let (decoded, _): (WarehouseInventory, usize) =
        decode_from_slice(&decompressed).expect("decode WarehouseInventory failed");

    assert_eq!(inventory, decoded);
}

// ---------------------------------------------------------------------------
// Test 3 – shipment tracking with InTransit variant
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_shipment_tracking_in_transit_roundtrip() {
    let tracking = ShipmentTracking {
        tracking_number: "1Z999AA10123456784".to_string(),
        status: ShipmentStatus::InTransit {
            hub: "Warsaw Sorting Centre".to_string(),
            eta_hours: 18,
        },
        carrier: "ExpressFreight EU".to_string(),
        last_update_epoch: 1_750_000_000,
    };

    let encoded = encode_to_vec(&tracking).expect("encode ShipmentTracking failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress ShipmentTracking failed");
    let decompressed = decompress(&compressed).expect("decompress ShipmentTracking failed");
    let (decoded, _): (ShipmentTracking, usize) =
        decode_from_slice(&decompressed).expect("decode ShipmentTracking failed");

    assert_eq!(tracking, decoded);
}

// ---------------------------------------------------------------------------
// Test 4 – shipment tracking with Delivered variant
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_shipment_delivered_variant_roundtrip() {
    let tracking = ShipmentTracking {
        tracking_number: "TRK-2026-987654".to_string(),
        status: ShipmentStatus::Delivered {
            proof_of_delivery: "SIGNATURE:JohnDoe:2026-03-14T14:22:00Z".to_string(),
        },
        carrier: "LastMile Express".to_string(),
        last_update_epoch: 1_750_120_000,
    };

    let encoded = encode_to_vec(&tracking).expect("encode delivered tracking failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress delivered tracking failed");
    let decompressed = decompress(&compressed).expect("decompress delivered tracking failed");
    let (decoded, _): (ShipmentTracking, usize) =
        decode_from_slice(&decompressed).expect("decode delivered tracking failed");

    assert_eq!(tracking, decoded);
}

// ---------------------------------------------------------------------------
// Test 5 – fleet vehicle with RefrigeratedTruck variant
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_fleet_refrigerated_truck_roundtrip() {
    let vehicle = FleetVehicle {
        vehicle_id: 55_001,
        vehicle_type: VehicleType::RefrigeratedTruck {
            min_temp_celsius: -20,
        },
        license_plate: "723 KLM".to_string(),
        current_location: GeoCoordinate {
            latitude: 52.2297,
            longitude: 21.0122,
        },
        fuel_percent: 78,
        active: true,
    };

    let encoded = encode_to_vec(&vehicle).expect("encode FleetVehicle failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress FleetVehicle failed");
    let decompressed = decompress(&compressed).expect("decompress FleetVehicle failed");
    let (decoded, _): (FleetVehicle, usize) =
        decode_from_slice(&decompressed).expect("decode FleetVehicle failed");

    assert_eq!(vehicle, decoded);
}

// ---------------------------------------------------------------------------
// Test 6 – package dimensions round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_package_dimensions_roundtrip() {
    let pkg = PackageDimensions {
        length_cm: 60.0,
        width_cm: 40.0,
        height_cm: 30.0,
        weight_kg: 15.75,
        fragile: true,
        hazmat: false,
    };

    let encoded = encode_to_vec(&pkg).expect("encode PackageDimensions failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress PackageDimensions failed");
    let decompressed = decompress(&compressed).expect("decompress PackageDimensions failed");
    let (decoded, _): (PackageDimensions, usize) =
        decode_from_slice(&decompressed).expect("decode PackageDimensions failed");

    assert_eq!(pkg, decoded);
}

// ---------------------------------------------------------------------------
// Test 7 – customs declaration round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_customs_declaration_roundtrip() {
    let decl = CustomsDeclaration {
        declaration_id: "DECL-2026-0031588".to_string(),
        origin_country: "CN".to_string(),
        destination_country: "EE".to_string(),
        declared_value_usd: 349.99,
        currency: "USD".to_string(),
        goods_description: "Electronic components — PCBs and connectors".to_string(),
        hs_code: "8534.00.00".to_string(),
        duties_paid: true,
    };

    let encoded = encode_to_vec(&decl).expect("encode CustomsDeclaration failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress CustomsDeclaration failed");
    let decompressed = decompress(&compressed).expect("decompress CustomsDeclaration failed");
    let (decoded, _): (CustomsDeclaration, usize) =
        decode_from_slice(&decompressed).expect("decode CustomsDeclaration failed");

    assert_eq!(decl, decoded);
}

// ---------------------------------------------------------------------------
// Test 8 – carrier rates vec round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_carrier_rates_vec_roundtrip() {
    let rates: Vec<CarrierRate> = (0u32..50)
        .map(|i| CarrierRate {
            carrier_id: i,
            carrier_name: format!("Carrier-{i:04}"),
            service_level: if i % 3 == 0 {
                "express".to_string()
            } else {
                "standard".to_string()
            },
            base_rate_usd: 5.0 + (i as f64) * 0.5,
            per_kg_rate_usd: 0.10 + (i as f64) * 0.01,
            fuel_surcharge_pct: 4.5,
            max_weight_kg: 70.0,
            transit_days: (i % 7 + 1) as u8,
        })
        .collect();

    let encoded = encode_to_vec(&rates).expect("encode carrier rates failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress carrier rates failed");
    let decompressed = decompress(&compressed).expect("decompress carrier rates failed");
    let (decoded, _): (Vec<CarrierRate>, usize) =
        decode_from_slice(&decompressed).expect("decode carrier rates failed");

    assert_eq!(rates, decoded);
}

// ---------------------------------------------------------------------------
// Test 9 – last-mile delivery round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_last_mile_delivery_roundtrip() {
    let lmd = LastMileDelivery {
        attempt_number: 2,
        driver_id: 4001,
        vehicle_id: 55_002,
        package_ids: vec![
            "PKG-001".to_string(),
            "PKG-002".to_string(),
            "PKG-003".to_string(),
        ],
        route: DeliveryRoute {
            route_id: 200_010,
            origin: GeoCoordinate {
                latitude: 59.4370,
                longitude: 24.7536,
            },
            destination: GeoCoordinate {
                latitude: 59.4500,
                longitude: 24.8000,
            },
            waypoints: vec![],
            distance_km: 5.3,
            estimated_minutes: 15,
        },
        customer_contacted: true,
        signature_required: false,
    };

    let encoded = encode_to_vec(&lmd).expect("encode LastMileDelivery failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress LastMileDelivery failed");
    let decompressed = decompress(&compressed).expect("decompress LastMileDelivery failed");
    let (decoded, _): (LastMileDelivery, usize) =
        decode_from_slice(&decompressed).expect("decode LastMileDelivery failed");

    assert_eq!(lmd, decoded);
}

// ---------------------------------------------------------------------------
// Test 10 – cold chain temperature log round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cold_chain_record_roundtrip() {
    let record = ColdChainRecord {
        shipment_id: "CCL-2026-00778".to_string(),
        product: "Frozen Pharmaceuticals".to_string(),
        required_min_celsius: -25,
        required_max_celsius: -15,
        temperature_log: (-20..=20).map(|v| v as f32 * 0.5).collect(),
        breach_detected: false,
    };

    let encoded = encode_to_vec(&record).expect("encode ColdChainRecord failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress ColdChainRecord failed");
    let decompressed = decompress(&compressed).expect("decompress ColdChainRecord failed");
    let (decoded, _): (ColdChainRecord, usize) =
        decode_from_slice(&decompressed).expect("decode ColdChainRecord failed");

    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 11 – reverse logistics with DamagedGoods reason
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_reverse_logistics_damaged_roundtrip() {
    let entry = ReverseLogisticsEntry {
        return_id: "RET-2026-005512".to_string(),
        original_shipment: "1Z999AA10123456784".to_string(),
        reason: ReturnReason::DamagedGoods,
        refund_initiated: true,
        restocking_eligible: false,
    };

    let encoded = encode_to_vec(&entry).expect("encode ReverseLogisticsEntry failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress ReverseLogisticsEntry failed");
    let decompressed = decompress(&compressed).expect("decompress ReverseLogisticsEntry failed");
    let (decoded, _): (ReverseLogisticsEntry, usize) =
        decode_from_slice(&decompressed).expect("decode ReverseLogisticsEntry failed");

    assert_eq!(entry, decoded);
}

// ---------------------------------------------------------------------------
// Test 12 – reverse logistics with Other(String) reason
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_reverse_logistics_other_reason_roundtrip() {
    let entry = ReverseLogisticsEntry {
        return_id: "RET-2026-009900".to_string(),
        original_shipment: "TRK-2026-111222".to_string(),
        reason: ReturnReason::Other("Customs hold — re-export required".to_string()),
        refund_initiated: false,
        restocking_eligible: false,
    };

    let encoded = encode_to_vec(&entry).expect("encode Other reason failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Other reason failed");
    let decompressed = decompress(&compressed).expect("decompress Other reason failed");
    let (decoded, _): (ReverseLogisticsEntry, usize) =
        decode_from_slice(&decompressed).expect("decode Other reason failed");

    assert_eq!(entry, decoded);
}

// ---------------------------------------------------------------------------
// Test 13 – cross-dock operation round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cross_dock_operation_roundtrip() {
    let op = CrossDockOperation {
        dock_id: 42,
        inbound_carrier: "FreightLine BV".to_string(),
        outbound_carrier: "RapidShip GmbH".to_string(),
        transfer_window_minutes: 90,
        pallet_count: 24,
        items_transferred: (0u32..24).map(|i| format!("PLT-{i:05}")).collect(),
    };

    let encoded = encode_to_vec(&op).expect("encode CrossDockOperation failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress CrossDockOperation failed");
    let decompressed = decompress(&compressed).expect("decompress CrossDockOperation failed");
    let (decoded, _): (CrossDockOperation, usize) =
        decode_from_slice(&decompressed).expect("decode CrossDockOperation failed");

    assert_eq!(op, decoded);
}

// ---------------------------------------------------------------------------
// Test 14 – freight forwarder bill round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_freight_forwarder_bill_roundtrip() {
    let bill = FreightForwarderBill {
        bill_id: "AWB-2026-9984321".to_string(),
        shipper: "TechParts Shanghai Co.".to_string(),
        consignee: "EuroDistrib Tallinn OÜ".to_string(),
        origin_port: "SHA".to_string(),
        destination_port: "TLL".to_string(),
        total_weight_kg: 4_820.0,
        total_volume_m3: 18.6,
        freight_charges_usd: 12_350.00,
        containers: vec!["TEMU1234567".to_string(), "TEMU7654321".to_string()],
    };

    let encoded = encode_to_vec(&bill).expect("encode FreightForwarderBill failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress FreightForwarderBill failed");
    let decompressed = decompress(&compressed).expect("decompress FreightForwarderBill failed");
    let (decoded, _): (FreightForwarderBill, usize) =
        decode_from_slice(&decompressed).expect("decode FreightForwarderBill failed");

    assert_eq!(bill, decoded);
}

// ---------------------------------------------------------------------------
// Test 15 – compression ratio: 1000+ repetitive warehouse items
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_compression_ratio_warehouse_inventory() {
    // 1200 identical items — highly repetitive encoded bytes => LZ4 must compress below 1.0×
    let items: Vec<WarehouseInventoryItem> = (0u32..1_200)
        .map(|_| WarehouseInventoryItem {
            sku: "SKU-REPEAT-001".to_string(),
            quantity: 100,
            bin_location: "Z-99-99".to_string(),
            weight_kg: 2.0,
        })
        .collect();
    let inventory = WarehouseInventory {
        warehouse_id: 1,
        region: "Test Region".to_string(),
        items,
    };

    let encoded = encode_to_vec(&inventory).expect("encode large inventory failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large inventory failed");

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "compression ratio {ratio:.4} should be < 1.0 for 1200 repetitive inventory items \
         (encoded={} bytes, compressed={} bytes)",
        encoded.len(),
        compressed.len()
    );
}

// ---------------------------------------------------------------------------
// Test 16 – compression ratio: 1000+ identical tracking records
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_compression_ratio_shipment_tracking_bulk() {
    let records: Vec<ShipmentTracking> = (0u64..1_000)
        .map(|i| ShipmentTracking {
            tracking_number: format!("TRK-BULK-{i:010}"),
            status: ShipmentStatus::InTransit {
                hub: "Central Hub".to_string(),
                eta_hours: 24,
            },
            carrier: "BulkCarrier Express".to_string(),
            last_update_epoch: 1_750_000_000 + i,
        })
        .collect();

    let encoded = encode_to_vec(&records).expect("encode bulk tracking failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress bulk tracking failed");

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "compression ratio {ratio:.4} should be < 1.0 for 1000 tracking records \
         (encoded={} bytes, compressed={} bytes)",
        encoded.len(),
        compressed.len()
    );
}

// ---------------------------------------------------------------------------
// Test 17 – compression ratio: 1000+ identical cold chain readings
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_compression_ratio_cold_chain_bulk() {
    // Build one large record whose temperature_log has 5000 identical values
    let record = ColdChainRecord {
        shipment_id: "CCL-BULK-00001".to_string(),
        product: "Frozen Vaccine Batch".to_string(),
        required_min_celsius: -30,
        required_max_celsius: -20,
        temperature_log: vec![-25.0_f32; 5_000],
        breach_detected: false,
    };

    let encoded = encode_to_vec(&record).expect("encode cold chain bulk failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress cold chain bulk failed");

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "compression ratio {ratio:.4} should be < 1.0 for 5000-element temperature log \
         (encoded={} bytes, compressed={} bytes)",
        encoded.len(),
        compressed.len()
    );

    // Verify round-trip integrity
    let decompressed = decompress(&compressed).expect("decompress cold chain bulk failed");
    let (decoded, _): (ColdChainRecord, usize) =
        decode_from_slice(&decompressed).expect("decode cold chain bulk failed");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 18 – data integrity: all enum variants in a batch
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_all_shipment_status_variants_integrity() {
    let statuses: Vec<ShipmentStatus> = vec![
        ShipmentStatus::Pending,
        ShipmentStatus::PickedUp,
        ShipmentStatus::InTransit {
            hub: "Brussels Hub".to_string(),
            eta_hours: 6,
        },
        ShipmentStatus::OutForDelivery,
        ShipmentStatus::Delivered {
            proof_of_delivery: "SIG:Jane".to_string(),
        },
        ShipmentStatus::Failed {
            reason: "Door locked".to_string(),
        },
        ShipmentStatus::Returned,
    ];

    let encoded = encode_to_vec(&statuses).expect("encode statuses failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress statuses failed");
    let decompressed = decompress(&compressed).expect("decompress statuses failed");
    let (decoded, _): (Vec<ShipmentStatus>, usize) =
        decode_from_slice(&decompressed).expect("decode statuses failed");

    assert_eq!(statuses, decoded);
    assert_eq!(decoded.len(), 7);
}

// ---------------------------------------------------------------------------
// Test 19 – data integrity: all vehicle type variants
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_all_vehicle_type_variants_integrity() {
    let vehicles: Vec<FleetVehicle> = vec![
        FleetVehicle {
            vehicle_id: 1,
            vehicle_type: VehicleType::Bicycle,
            license_plate: "BIKE-001".to_string(),
            current_location: GeoCoordinate {
                latitude: 0.0,
                longitude: 0.0,
            },
            fuel_percent: 100,
            active: true,
        },
        FleetVehicle {
            vehicle_id: 2,
            vehicle_type: VehicleType::Motorcycle,
            license_plate: "MOTO-002".to_string(),
            current_location: GeoCoordinate {
                latitude: 1.0,
                longitude: 1.0,
            },
            fuel_percent: 80,
            active: true,
        },
        FleetVehicle {
            vehicle_id: 3,
            vehicle_type: VehicleType::Van,
            license_plate: "VAN-003".to_string(),
            current_location: GeoCoordinate {
                latitude: 2.0,
                longitude: 2.0,
            },
            fuel_percent: 60,
            active: true,
        },
        FleetVehicle {
            vehicle_id: 4,
            vehicle_type: VehicleType::Truck { axles: 5 },
            license_plate: "TRK-004".to_string(),
            current_location: GeoCoordinate {
                latitude: 3.0,
                longitude: 3.0,
            },
            fuel_percent: 45,
            active: false,
        },
        FleetVehicle {
            vehicle_id: 5,
            vehicle_type: VehicleType::RefrigeratedTruck {
                min_temp_celsius: -18,
            },
            license_plate: "REF-005".to_string(),
            current_location: GeoCoordinate {
                latitude: 4.0,
                longitude: 4.0,
            },
            fuel_percent: 90,
            active: true,
        },
        FleetVehicle {
            vehicle_id: 6,
            vehicle_type: VehicleType::DroneDelivery {
                max_payload_kg: 2.5,
            },
            license_plate: "DRN-006".to_string(),
            current_location: GeoCoordinate {
                latitude: 5.0,
                longitude: 5.0,
            },
            fuel_percent: 55,
            active: true,
        },
    ];

    let encoded = encode_to_vec(&vehicles).expect("encode fleet failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress fleet failed");
    let decompressed = decompress(&compressed).expect("decompress fleet failed");
    let (decoded, _): (Vec<FleetVehicle>, usize) =
        decode_from_slice(&decompressed).expect("decode fleet failed");

    assert_eq!(vehicles, decoded);
    assert_eq!(decoded.len(), 6);
}

// ---------------------------------------------------------------------------
// Test 20 – data integrity: all return reason variants
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_all_return_reason_variants_integrity() {
    let entries: Vec<ReverseLogisticsEntry> = vec![
        ReverseLogisticsEntry {
            return_id: "R1".to_string(),
            original_shipment: "S1".to_string(),
            reason: ReturnReason::CustomerRefusal,
            refund_initiated: true,
            restocking_eligible: true,
        },
        ReverseLogisticsEntry {
            return_id: "R2".to_string(),
            original_shipment: "S2".to_string(),
            reason: ReturnReason::DamagedGoods,
            refund_initiated: true,
            restocking_eligible: false,
        },
        ReverseLogisticsEntry {
            return_id: "R3".to_string(),
            original_shipment: "S3".to_string(),
            reason: ReturnReason::WrongItem,
            refund_initiated: false,
            restocking_eligible: true,
        },
        ReverseLogisticsEntry {
            return_id: "R4".to_string(),
            original_shipment: "S4".to_string(),
            reason: ReturnReason::AddressNotFound,
            refund_initiated: false,
            restocking_eligible: false,
        },
        ReverseLogisticsEntry {
            return_id: "R5".to_string(),
            original_shipment: "S5".to_string(),
            reason: ReturnReason::RecipientUnavailable,
            refund_initiated: false,
            restocking_eligible: true,
        },
        ReverseLogisticsEntry {
            return_id: "R6".to_string(),
            original_shipment: "S6".to_string(),
            reason: ReturnReason::Other("Act of God — flood damage".to_string()),
            refund_initiated: true,
            restocking_eligible: false,
        },
    ];

    let encoded = encode_to_vec(&entries).expect("encode return entries failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress return entries failed");
    let decompressed = decompress(&compressed).expect("decompress return entries failed");
    let (decoded, _): (Vec<ReverseLogisticsEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode return entries failed");

    assert_eq!(entries, decoded);
    assert_eq!(decoded.len(), 6);
}

// ---------------------------------------------------------------------------
// Test 21 – large cross-dock batch compression ratio
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_compression_ratio_cross_dock_bulk() {
    // 1000 identical cross-dock operations to produce highly repetitive data
    let ops: Vec<CrossDockOperation> = (0u32..1_000)
        .map(|_| CrossDockOperation {
            dock_id: 1,
            inbound_carrier: "StdCarrier A".to_string(),
            outbound_carrier: "StdCarrier B".to_string(),
            transfer_window_minutes: 60,
            pallet_count: 10,
            items_transferred: vec!["PLT-STANDARD".to_string(); 10],
        })
        .collect();

    let encoded = encode_to_vec(&ops).expect("encode cross-dock bulk failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress cross-dock bulk failed");

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "compression ratio {ratio:.4} should be < 1.0 for 1000 identical cross-dock ops \
         (encoded={} bytes, compressed={} bytes)",
        encoded.len(),
        compressed.len()
    );

    // Integrity check
    let decompressed = decompress(&compressed).expect("decompress cross-dock bulk failed");
    let (decoded, _): (Vec<CrossDockOperation>, usize) =
        decode_from_slice(&decompressed).expect("decode cross-dock bulk failed");
    assert_eq!(ops, decoded);
}

// ---------------------------------------------------------------------------
// Test 22 – freight forwarder bulk bills compression ratio + integrity
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_freight_forwarder_bills_bulk_ratio_and_integrity() {
    // 1000 freight bills with identical carrier/port data to stress LZ4
    let bills: Vec<FreightForwarderBill> = (0u32..1_000)
        .map(|i| FreightForwarderBill {
            bill_id: format!("AWB-BULK-{i:07}"),
            shipper: "GlobalShipper Corp".to_string(),
            consignee: "EuroDist OÜ".to_string(),
            origin_port: "SHA".to_string(),
            destination_port: "HEL".to_string(),
            total_weight_kg: 1_000.0,
            total_volume_m3: 5.0,
            freight_charges_usd: 8_000.0,
            containers: vec!["CONT-STD-001".to_string(), "CONT-STD-002".to_string()],
        })
        .collect();

    let encoded = encode_to_vec(&bills).expect("encode freight bills bulk failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress freight bills bulk failed");

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "compression ratio {ratio:.4} should be < 1.0 for 1000 freight bills \
         (encoded={} bytes, compressed={} bytes)",
        encoded.len(),
        compressed.len()
    );

    let decompressed = decompress(&compressed).expect("decompress freight bills bulk failed");
    let (decoded, _): (Vec<FreightForwarderBill>, usize) =
        decode_from_slice(&decompressed).expect("decode freight bills bulk failed");

    assert_eq!(bills.len(), decoded.len());
    // Spot-check first and last entries
    assert_eq!(bills[0], decoded[0]);
    assert_eq!(bills[999], decoded[999]);
}
