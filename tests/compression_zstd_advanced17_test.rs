//! Advanced Zstd compression tests for OxiCode — Global Supply Chain & Logistics domain.
//!
//! Covers encode -> compress -> decompress -> decode round-trips for types that
//! model real-world supply chain operations: container shipping manifests, port
//! terminal operations, customs declarations, bill of lading, freight forwarding
//! quotes, warehouse management, cross-docking, last-mile delivery tracking,
//! cold chain temperature logs, reverse logistics, intermodal transport, demand
//! forecasting, inventory reorder points, supplier scorecards, and carbon
//! footprint per shipment.

#![cfg(feature = "compression-zstd")]
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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContainerSize {
    TwentyFoot,
    FortyFoot,
    FortyFootHighCube,
    FortyFiveFoot,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContainerType {
    DryStorage,
    Refrigerated,
    OpenTop,
    FlatRack,
    Tank,
    Ventilated,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HazmatClass {
    Explosives,
    Gases,
    FlammableLiquids,
    FlammableSolids,
    OxidizingSubstances,
    ToxicSubstances,
    RadioactiveMaterial,
    Corrosives,
    MiscellaneousDangerous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransportMode {
    OceanFreight,
    AirFreight,
    RailFreight,
    TruckLoad,
    LessThanTruckLoad,
    Intermodal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IncotermsRule {
    ExWorks,
    FreeCarrier,
    FreeOnBoard,
    CostInsuranceFreight,
    DeliveredAtPlace,
    DeliveredDutyPaid,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CustomsStatus {
    Pending,
    DocumentsSubmitted,
    UnderInspection,
    Cleared,
    HeldForExamination,
    Rejected,
    DutyPaid,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeliveryStatus {
    OrderReceived,
    PickedUp,
    InTransitToHub,
    AtSortingFacility,
    OutForDelivery,
    Delivered,
    DeliveryAttemptFailed,
    ReturnedToSender,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TemperatureZone {
    Frozen,
    Chilled,
    CoolRoom,
    Ambient,
    Controlled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReturnReason {
    Damaged,
    WrongItem,
    DefectiveProduct,
    CustomerChanged,
    ExpiredShelfLife,
    RecallNotice,
    QualityFail,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SupplierTier {
    Strategic,
    Preferred,
    Approved,
    Conditional,
    PendingReview,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ForecastMethod {
    MovingAverage,
    ExponentialSmoothing,
    Arima,
    SeasonalDecomposition,
    MachineLearning,
    JudgmentalOverride,
}

// ---------------------------------------------------------------------------
// Composite domain structs
// ---------------------------------------------------------------------------

/// A single line item within a shipping manifest.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ManifestLineItem {
    commodity_code: String,
    description: String,
    quantity: u32,
    weight_grams: u64,
    volume_cm3: u64,
    declared_value_cents: u64,
    hazmat: Option<HazmatClass>,
}

/// Container shipping manifest.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContainerManifest {
    manifest_id: String,
    container_number: String,
    container_size: ContainerSize,
    container_type: ContainerType,
    seal_number: String,
    shipper_name: String,
    consignee_name: String,
    origin_port_code: String,
    destination_port_code: String,
    gross_weight_kg: u32,
    line_items: Vec<ManifestLineItem>,
}

/// Port terminal operation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PortTerminalOperation {
    operation_id: u64,
    terminal_code: String,
    berth_number: u16,
    vessel_name: String,
    vessel_imo: String,
    crane_id: String,
    container_number: String,
    move_type: String,
    timestamp_epoch_ms: u64,
    bay_row_tier: (u16, u16, u16),
    dwell_time_hours: u32,
}

/// Customs declaration form.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CustomsDeclaration {
    declaration_id: String,
    customs_office_code: String,
    declarant_name: String,
    importer_of_record: String,
    status: CustomsStatus,
    hs_tariff_codes: Vec<String>,
    total_declared_value_cents: u64,
    currency_code: String,
    duty_amount_cents: u64,
    vat_amount_cents: u64,
    inspection_notes: Vec<String>,
}

/// Bill of lading.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BillOfLading {
    bol_number: String,
    booking_reference: String,
    carrier_name: String,
    carrier_scac: String,
    shipper: String,
    consignee: String,
    notify_party: String,
    port_of_loading: String,
    port_of_discharge: String,
    place_of_delivery: String,
    incoterms: IncotermsRule,
    container_numbers: Vec<String>,
    total_packages: u32,
    total_weight_kg: u32,
    freight_charges_cents: u64,
    is_negotiable: bool,
}

/// Freight forwarding quote.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FreightQuote {
    quote_id: String,
    forwarder_name: String,
    transport_mode: TransportMode,
    origin_city: String,
    destination_city: String,
    transit_days_min: u16,
    transit_days_max: u16,
    ocean_freight_cents: u64,
    inland_haulage_cents: u64,
    customs_brokerage_cents: u64,
    insurance_cents: u64,
    total_cents: u64,
    valid_until_epoch: u64,
    notes: Vec<String>,
}

/// Warehouse bin location.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BinLocation {
    warehouse_id: String,
    zone: String,
    aisle: u16,
    rack: u16,
    level: u8,
    position: u8,
    sku: String,
    on_hand_qty: u32,
    allocated_qty: u32,
    available_qty: u32,
    max_capacity: u32,
}

/// Pick list for warehouse order fulfillment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PickListEntry {
    order_id: String,
    sku: String,
    bin_location_code: String,
    pick_quantity: u32,
    picked_quantity: u32,
    is_complete: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PickList {
    pick_list_id: String,
    warehouse_id: String,
    wave_number: u32,
    picker_employee_id: String,
    entries: Vec<PickListEntry>,
    started_epoch_ms: u64,
    completed_epoch_ms: Option<u64>,
}

/// Cross-docking operation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrossDockOperation {
    operation_id: String,
    facility_id: String,
    inbound_trailer: String,
    inbound_carrier: String,
    outbound_trailer: String,
    outbound_carrier: String,
    pallet_count: u32,
    carton_count: u32,
    sort_lane: String,
    received_epoch_ms: u64,
    shipped_epoch_ms: Option<u64>,
    handling_time_seconds: u32,
}

/// Last-mile delivery tracking event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LastMileEvent {
    tracking_number: String,
    status: DeliveryStatus,
    timestamp_epoch_ms: u64,
    driver_id: String,
    latitude_microdeg: i64,
    longitude_microdeg: i64,
    photo_proof_hash: Option<String>,
    recipient_name: Option<String>,
    delivery_notes: String,
}

/// Cold chain temperature log entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ColdChainLog {
    shipment_id: String,
    sensor_id: String,
    zone: TemperatureZone,
    target_temp_milli_c: i32,
    actual_temp_milli_c: i32,
    humidity_percent_x10: u16,
    timestamp_epoch_ms: u64,
    is_excursion: bool,
    excursion_duration_sec: Option<u32>,
}

/// Reverse logistics return processing record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReturnProcessing {
    rma_number: String,
    original_order_id: String,
    sku: String,
    quantity: u32,
    reason: ReturnReason,
    condition_grade: u8,
    refund_amount_cents: u64,
    restocking_fee_cents: u64,
    disposition: String,
    inspected_by: String,
    processed_epoch_ms: u64,
}

/// Intermodal transport leg.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IntermodalLeg {
    leg_id: u32,
    mode: TransportMode,
    carrier: String,
    origin_terminal: String,
    destination_terminal: String,
    departure_epoch_ms: u64,
    arrival_epoch_ms: u64,
    equipment_number: String,
    distance_meters: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IntermodalShipment {
    shipment_id: String,
    booking_ref: String,
    legs: Vec<IntermodalLeg>,
    total_transit_hours: u32,
    total_distance_meters: u64,
    carbon_kg_x100: u64,
}

/// Demand forecasting snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DemandForecast {
    forecast_id: String,
    sku: String,
    location_id: String,
    method: ForecastMethod,
    period_start_epoch: u64,
    period_days: u16,
    predicted_demand: u32,
    lower_bound: u32,
    upper_bound: u32,
    confidence_pct_x100: u16,
    actual_demand: Option<u32>,
    forecast_error_pct_x100: Option<i32>,
}

/// Inventory reorder point record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReorderPoint {
    sku: String,
    warehouse_id: String,
    on_hand: u32,
    safety_stock: u32,
    reorder_level: u32,
    economic_order_quantity: u32,
    lead_time_days: u16,
    avg_daily_demand: u32,
    needs_reorder: bool,
    last_review_epoch: u64,
}

/// Supplier scorecard.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SupplierScorecard {
    supplier_id: String,
    supplier_name: String,
    tier: SupplierTier,
    on_time_delivery_pct_x100: u16,
    quality_reject_rate_ppm: u32,
    lead_time_days_avg: u16,
    lead_time_days_stddev: u16,
    cost_competitiveness_score: u8,
    responsiveness_score: u8,
    sustainability_score: u8,
    overall_score: u16,
    evaluation_period_start: u64,
    evaluation_period_end: u64,
}

/// Carbon footprint per shipment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarbonFootprint {
    shipment_id: String,
    transport_mode: TransportMode,
    distance_km: u64,
    weight_kg: u32,
    co2_grams: u64,
    nox_milligrams: u64,
    sox_milligrams: u64,
    pm25_micrograms: u64,
    fuel_liters_x100: u64,
    offset_credits_purchased: bool,
    emission_factor_source: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_container_shipping_manifest_roundtrip() {
    let manifest = ContainerManifest {
        manifest_id: "MAN-2026-00451".into(),
        container_number: "MSCU7654321".into(),
        container_size: ContainerSize::FortyFootHighCube,
        container_type: ContainerType::DryStorage,
        seal_number: "SEAL-88210".into(),
        shipper_name: "Yokohama Electronics Co.".into(),
        consignee_name: "Rotterdam Distribution BV".into(),
        origin_port_code: "JPYOK".into(),
        destination_port_code: "NLRTM".into(),
        gross_weight_kg: 24_500,
        line_items: vec![
            ManifestLineItem {
                commodity_code: "8542.31".into(),
                description: "Integrated circuits, processors".into(),
                quantity: 12_000,
                weight_grams: 6_000_000,
                volume_cm3: 2_400_000,
                declared_value_cents: 45_000_000,
                hazmat: None,
            },
            ManifestLineItem {
                commodity_code: "8507.60".into(),
                description: "Lithium-ion batteries".into(),
                quantity: 5_000,
                weight_grams: 15_000_000,
                volume_cm3: 8_000_000,
                declared_value_cents: 22_500_000,
                hazmat: Some(HazmatClass::MiscellaneousDangerous),
            },
        ],
    };

    let encoded = encode_to_vec(&manifest).expect("encode manifest");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress manifest");
    let decompressed = decompress(&compressed).expect("decompress manifest");
    let (decoded, _): (ContainerManifest, _) =
        decode_from_slice(&decompressed).expect("decode manifest");
    assert_eq!(manifest, decoded);
}

#[test]
fn test_port_terminal_operations_batch() {
    let operations: Vec<PortTerminalOperation> = (0..50)
        .map(|i| PortTerminalOperation {
            operation_id: 100_000 + i,
            terminal_code: "SGSIN-T3".into(),
            berth_number: (i % 8 + 1) as u16,
            vessel_name: format!("MV OCEAN VOYAGER {}", i / 10),
            vessel_imo: format!("IMO{}", 9_800_000 + i),
            crane_id: format!("QC-{:02}", i % 6 + 1),
            container_number: format!("MAEU{:07}", 1_000_000 + i),
            move_type: if i % 3 == 0 {
                "DISCHARGE".into()
            } else {
                "LOAD".into()
            },
            timestamp_epoch_ms: 1_740_000_000_000 + i * 45_000,
            bay_row_tier: ((i % 20 + 1) as u16, (i % 10 + 1) as u16, (i % 8 + 1) as u16),
            dwell_time_hours: (i * 3 + 12) as u32,
        })
        .collect();

    let encoded = encode_to_vec(&operations).expect("encode port ops");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress port ops");
    let decompressed = decompress(&compressed).expect("decompress port ops");
    let (decoded, _): (Vec<PortTerminalOperation>, _) =
        decode_from_slice(&decompressed).expect("decode port ops");
    assert_eq!(operations, decoded);
}

#[test]
fn test_customs_declaration_with_inspection() {
    let declaration = CustomsDeclaration {
        declaration_id: "CUS-DE-2026-19283".into(),
        customs_office_code: "DE005300".into(),
        declarant_name: "European Freight Brokers GmbH".into(),
        importer_of_record: "Berlin Auto Parts AG".into(),
        status: CustomsStatus::HeldForExamination,
        hs_tariff_codes: vec![
            "8708.29".into(),
            "8708.30".into(),
            "8708.40".into(),
            "4016.93".into(),
        ],
        total_declared_value_cents: 1_250_000_00,
        currency_code: "EUR".into(),
        duty_amount_cents: 43_750_00,
        vat_amount_cents: 237_500_00,
        inspection_notes: vec![
            "Random selection for physical inspection".into(),
            "Verify HS classification for rubber gaskets".into(),
            "Request certificate of origin from shipper".into(),
        ],
    };

    let encoded = encode_to_vec(&declaration).expect("encode customs");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress customs");
    let decompressed = decompress(&compressed).expect("decompress customs");
    let (decoded, _): (CustomsDeclaration, _) =
        decode_from_slice(&decompressed).expect("decode customs");
    assert_eq!(declaration, decoded);
}

#[test]
fn test_bill_of_lading_negotiable() {
    let bol = BillOfLading {
        bol_number: "HLCU-BOL-2026-77431".into(),
        booking_reference: "BKG-2026-55120".into(),
        carrier_name: "Hapag-Lloyd AG".into(),
        carrier_scac: "HLCU".into(),
        shipper: "Shanghai Heavy Machinery Export Corp".into(),
        consignee: "TO ORDER OF ISSUING BANK".into(),
        notify_party: "Midwest Industrial Equipment LLC".into(),
        port_of_loading: "CNSHA".into(),
        port_of_discharge: "USLGB".into(),
        place_of_delivery: "Chicago, IL".into(),
        incoterms: IncotermsRule::CostInsuranceFreight,
        container_numbers: vec![
            "HLCU1234567".into(),
            "HLCU2345678".into(),
            "HLCU3456789".into(),
        ],
        total_packages: 84,
        total_weight_kg: 67_200,
        freight_charges_cents: 12_450_00,
        is_negotiable: true,
    };

    let encoded = encode_to_vec(&bol).expect("encode bol");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress bol");
    let decompressed = decompress(&compressed).expect("decompress bol");
    let (decoded, _): (BillOfLading, _) = decode_from_slice(&decompressed).expect("decode bol");
    assert_eq!(bol, decoded);
}

#[test]
fn test_freight_quotes_comparison() {
    let quotes = vec![
        FreightQuote {
            quote_id: "FQ-2026-A001".into(),
            forwarder_name: "Kuehne+Nagel".into(),
            transport_mode: TransportMode::OceanFreight,
            origin_city: "Shenzhen".into(),
            destination_city: "Hamburg".into(),
            transit_days_min: 28,
            transit_days_max: 35,
            ocean_freight_cents: 3_200_00,
            inland_haulage_cents: 450_00,
            customs_brokerage_cents: 180_00,
            insurance_cents: 95_00,
            total_cents: 3_925_00,
            valid_until_epoch: 1_742_000_000,
            notes: vec!["Subject to GRI effective April 1".into()],
        },
        FreightQuote {
            quote_id: "FQ-2026-A002".into(),
            forwarder_name: "DHL Global Forwarding".into(),
            transport_mode: TransportMode::AirFreight,
            origin_city: "Shenzhen".into(),
            destination_city: "Hamburg".into(),
            transit_days_min: 3,
            transit_days_max: 5,
            ocean_freight_cents: 0,
            inland_haulage_cents: 300_00,
            customs_brokerage_cents: 180_00,
            insurance_cents: 120_00,
            total_cents: 18_600_00,
            valid_until_epoch: 1_741_500_000,
            notes: vec![
                "Rate per kg: $4.50".into(),
                "Minimum chargeable weight: 100kg".into(),
            ],
        },
    ];

    let encoded = encode_to_vec(&quotes).expect("encode quotes");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress quotes");
    let decompressed = decompress(&compressed).expect("decompress quotes");
    let (decoded, _): (Vec<FreightQuote>, _) =
        decode_from_slice(&decompressed).expect("decode quotes");
    assert_eq!(quotes, decoded);
}

#[test]
fn test_warehouse_bin_locations_large_batch() {
    let bins: Vec<BinLocation> = (0..200)
        .map(|i| BinLocation {
            warehouse_id: "WH-EAST-07".into(),
            zone: format!("ZONE-{}", (i % 4) + 1),
            aisle: (i / 40 + 1) as u16,
            rack: (i % 40 / 4 + 1) as u16,
            level: (i % 4 + 1) as u8,
            position: (i % 2 + 1) as u8,
            sku: format!("SKU-{:06}", 100_000 + i),
            on_hand_qty: (50 + i * 3) as u32,
            allocated_qty: (i * 2) as u32,
            available_qty: (50 + i) as u32,
            max_capacity: 500,
        })
        .collect();

    let encoded = encode_to_vec(&bins).expect("encode bins");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress bins");

    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );

    let decompressed = decompress(&compressed).expect("decompress bins");
    let (decoded, _): (Vec<BinLocation>, _) =
        decode_from_slice(&decompressed).expect("decode bins");
    assert_eq!(bins, decoded);
}

#[test]
fn test_pick_list_wave_processing() {
    let pick_list = PickList {
        pick_list_id: "PL-2026-03-15-0042".into(),
        warehouse_id: "WH-WEST-12".into(),
        wave_number: 3,
        picker_employee_id: "EMP-7891".into(),
        entries: vec![
            PickListEntry {
                order_id: "ORD-882110".into(),
                sku: "SKU-443201".into(),
                bin_location_code: "A-05-03-B".into(),
                pick_quantity: 4,
                picked_quantity: 4,
                is_complete: true,
            },
            PickListEntry {
                order_id: "ORD-882110".into(),
                sku: "SKU-221087".into(),
                bin_location_code: "C-12-01-A".into(),
                pick_quantity: 1,
                picked_quantity: 1,
                is_complete: true,
            },
            PickListEntry {
                order_id: "ORD-882113".into(),
                sku: "SKU-990321".into(),
                bin_location_code: "B-08-04-C".into(),
                pick_quantity: 12,
                picked_quantity: 10,
                is_complete: false,
            },
        ],
        started_epoch_ms: 1_742_050_000_000,
        completed_epoch_ms: None,
    };

    let encoded = encode_to_vec(&pick_list).expect("encode pick list");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress pick list");
    let decompressed = decompress(&compressed).expect("decompress pick list");
    let (decoded, _): (PickList, _) = decode_from_slice(&decompressed).expect("decode pick list");
    assert_eq!(pick_list, decoded);
}

#[test]
fn test_cross_docking_operations() {
    let ops: Vec<CrossDockOperation> = (0..15)
        .map(|i| CrossDockOperation {
            operation_id: format!("XD-2026-{:04}", 1000 + i),
            facility_id: "XD-MEM-01".into(),
            inbound_trailer: format!("TR-IN-{:04}", 2000 + i),
            inbound_carrier: if i % 2 == 0 {
                "FedEx Freight".into()
            } else {
                "XPO Logistics".into()
            },
            outbound_trailer: format!("TR-OUT-{:04}", 3000 + i),
            outbound_carrier: format!("Regional Carrier {}", i % 5 + 1),
            pallet_count: (8 + i % 12) as u32,
            carton_count: (40 + i * 7) as u32,
            sort_lane: format!("LANE-{}", (i % 8) + 1),
            received_epoch_ms: 1_742_000_000_000 + i as u64 * 600_000,
            shipped_epoch_ms: if i < 12 {
                Some(1_742_000_000_000 + i as u64 * 600_000 + 1_800_000)
            } else {
                None
            },
            handling_time_seconds: (1800 + i * 120) as u32,
        })
        .collect();

    let encoded = encode_to_vec(&ops).expect("encode cross-dock");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress cross-dock");
    let decompressed = decompress(&compressed).expect("decompress cross-dock");
    let (decoded, _): (Vec<CrossDockOperation>, _) =
        decode_from_slice(&decompressed).expect("decode cross-dock");
    assert_eq!(ops, decoded);
}

#[test]
fn test_last_mile_delivery_tracking_events() {
    let events = vec![
        LastMileEvent {
            tracking_number: "1Z999AA10123456784".into(),
            status: DeliveryStatus::PickedUp,
            timestamp_epoch_ms: 1_742_060_000_000,
            driver_id: "DRV-4421".into(),
            latitude_microdeg: 35_689_500,
            longitude_microdeg: 139_691_700,
            photo_proof_hash: None,
            recipient_name: None,
            delivery_notes: "Picked up from warehouse WH-TYO-03".into(),
        },
        LastMileEvent {
            tracking_number: "1Z999AA10123456784".into(),
            status: DeliveryStatus::InTransitToHub,
            timestamp_epoch_ms: 1_742_063_600_000,
            driver_id: "DRV-4421".into(),
            latitude_microdeg: 35_672_100,
            longitude_microdeg: 139_710_300,
            photo_proof_hash: None,
            recipient_name: None,
            delivery_notes: "En route to Shinagawa sorting hub".into(),
        },
        LastMileEvent {
            tracking_number: "1Z999AA10123456784".into(),
            status: DeliveryStatus::Delivered,
            timestamp_epoch_ms: 1_742_078_400_000,
            driver_id: "DRV-5512".into(),
            latitude_microdeg: 35_658_300,
            longitude_microdeg: 139_745_400,
            photo_proof_hash: Some("sha256:a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0".into()),
            recipient_name: Some("Tanaka Ichiro".into()),
            delivery_notes: "Left at front door, signed by recipient".into(),
        },
    ];

    let encoded = encode_to_vec(&events).expect("encode delivery events");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress delivery events");
    let decompressed = decompress(&compressed).expect("decompress delivery events");
    let (decoded, _): (Vec<LastMileEvent>, _) =
        decode_from_slice(&decompressed).expect("decode delivery events");
    assert_eq!(events, decoded);
}

#[test]
fn test_cold_chain_temperature_log_excursion() {
    let logs: Vec<ColdChainLog> = (0..100)
        .map(|i| {
            let actual = -18_000 + (i as i32 * 50);
            let target = -18_000;
            let is_excursion = (actual - target).unsigned_abs() > 2_000;
            ColdChainLog {
                shipment_id: "COLD-2026-7812".into(),
                sensor_id: format!("SENSOR-{}", i / 25 + 1),
                zone: TemperatureZone::Frozen,
                target_temp_milli_c: target,
                actual_temp_milli_c: actual,
                humidity_percent_x10: 650 + (i as u16 % 50),
                timestamp_epoch_ms: 1_742_000_000_000 + i as u64 * 300_000,
                is_excursion,
                excursion_duration_sec: if is_excursion {
                    Some(300 * (i as u32 - 40).min(60))
                } else {
                    None
                },
            }
        })
        .collect();

    let encoded = encode_to_vec(&logs).expect("encode cold chain");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress cold chain");

    assert!(
        compressed.len() < encoded.len(),
        "cold chain logs should compress well, got {} vs {}",
        compressed.len(),
        encoded.len()
    );

    let decompressed = decompress(&compressed).expect("decompress cold chain");
    let (decoded, _): (Vec<ColdChainLog>, _) =
        decode_from_slice(&decompressed).expect("decode cold chain");
    assert_eq!(logs, decoded);
}

#[test]
fn test_reverse_logistics_returns_batch() {
    let returns = vec![
        ReturnProcessing {
            rma_number: "RMA-2026-00551".into(),
            original_order_id: "ORD-771203".into(),
            sku: "ELEC-TV-55-OLED".into(),
            quantity: 1,
            reason: ReturnReason::Damaged,
            condition_grade: 3,
            refund_amount_cents: 129_999,
            restocking_fee_cents: 0,
            disposition: "Salvage to repair center".into(),
            inspected_by: "QA-TEAM-B".into(),
            processed_epoch_ms: 1_742_100_000_000,
        },
        ReturnProcessing {
            rma_number: "RMA-2026-00552".into(),
            original_order_id: "ORD-771890".into(),
            sku: "CLOTH-JACKET-L-BLK".into(),
            quantity: 2,
            reason: ReturnReason::WrongItem,
            condition_grade: 9,
            refund_amount_cents: 18_998,
            restocking_fee_cents: 0,
            disposition: "Return to sellable inventory".into(),
            inspected_by: "QA-TEAM-A".into(),
            processed_epoch_ms: 1_742_100_300_000,
        },
        ReturnProcessing {
            rma_number: "RMA-2026-00553".into(),
            original_order_id: "ORD-770004".into(),
            sku: "FOOD-SUPPLEMENT-VD3".into(),
            quantity: 3,
            reason: ReturnReason::ExpiredShelfLife,
            condition_grade: 0,
            refund_amount_cents: 5_997,
            restocking_fee_cents: 0,
            disposition: "Destroy per regulation".into(),
            inspected_by: "QA-TEAM-C".into(),
            processed_epoch_ms: 1_742_100_600_000,
        },
    ];

    let encoded = encode_to_vec(&returns).expect("encode returns");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress returns");
    let decompressed = decompress(&compressed).expect("decompress returns");
    let (decoded, _): (Vec<ReturnProcessing>, _) =
        decode_from_slice(&decompressed).expect("decode returns");
    assert_eq!(returns, decoded);
}

#[test]
fn test_intermodal_shipment_multileg() {
    let shipment = IntermodalShipment {
        shipment_id: "IMS-2026-44210".into(),
        booking_ref: "BKG-INTERMODAL-8890".into(),
        legs: vec![
            IntermodalLeg {
                leg_id: 1,
                mode: TransportMode::TruckLoad,
                carrier: "Swift Transportation".into(),
                origin_terminal: "Factory Gate, Guangzhou".into(),
                destination_terminal: "Yantian Port Terminal".into(),
                departure_epoch_ms: 1_742_000_000_000,
                arrival_epoch_ms: 1_742_014_400_000,
                equipment_number: "CHASSIS-GZ-5512".into(),
                distance_meters: 85_000,
            },
            IntermodalLeg {
                leg_id: 2,
                mode: TransportMode::OceanFreight,
                carrier: "COSCO Shipping".into(),
                origin_terminal: "Yantian, CN".into(),
                destination_terminal: "Long Beach, US".into(),
                departure_epoch_ms: 1_742_100_000_000,
                arrival_epoch_ms: 1_743_400_000_000,
                equipment_number: "COSU1234567".into(),
                distance_meters: 18_500_000,
            },
            IntermodalLeg {
                leg_id: 3,
                mode: TransportMode::RailFreight,
                carrier: "BNSF Railway".into(),
                origin_terminal: "Long Beach Intermodal".into(),
                destination_terminal: "Chicago Logistics Park".into(),
                departure_epoch_ms: 1_743_500_000_000,
                arrival_epoch_ms: 1_743_900_000_000,
                equipment_number: "BNSF-WELL-88210".into(),
                distance_meters: 3_200_000,
            },
            IntermodalLeg {
                leg_id: 4,
                mode: TransportMode::LessThanTruckLoad,
                carrier: "Old Dominion Freight".into(),
                origin_terminal: "Chicago CY".into(),
                destination_terminal: "Customer DC, Indianapolis".into(),
                departure_epoch_ms: 1_743_950_000_000,
                arrival_epoch_ms: 1_744_000_000_000,
                equipment_number: "ODFL-TR-11023".into(),
                distance_meters: 290_000,
            },
        ],
        total_transit_hours: 556,
        total_distance_meters: 22_075_000,
        carbon_kg_x100: 385_000,
    };

    let encoded = encode_to_vec(&shipment).expect("encode intermodal");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress intermodal");
    let decompressed = decompress(&compressed).expect("decompress intermodal");
    let (decoded, _): (IntermodalShipment, _) =
        decode_from_slice(&decompressed).expect("decode intermodal");
    assert_eq!(shipment, decoded);
}

#[test]
fn test_demand_forecast_snapshots() {
    let forecasts: Vec<DemandForecast> = (0..30)
        .map(|i| {
            let predicted = 500 + i * 20;
            let actual = if i < 20 { Some(480 + i * 21) } else { None };
            let error =
                actual.map(|a| ((a as i64 - predicted as i64) * 10_000 / predicted as i64) as i32);
            DemandForecast {
                forecast_id: format!("FC-2026-{:04}", 3000 + i),
                sku: format!("SKU-{:06}", 200_000 + i % 10),
                location_id: format!("LOC-{:03}", i % 5 + 1),
                method: match i % 5 {
                    0 => ForecastMethod::MovingAverage,
                    1 => ForecastMethod::ExponentialSmoothing,
                    2 => ForecastMethod::Arima,
                    3 => ForecastMethod::SeasonalDecomposition,
                    _ => ForecastMethod::MachineLearning,
                },
                period_start_epoch: 1_740_000_000 + i as u64 * 86_400 * 7,
                period_days: 7,
                predicted_demand: predicted as u32,
                lower_bound: (predicted as u32).saturating_sub(80),
                upper_bound: predicted as u32 + 120,
                confidence_pct_x100: 9500,
                actual_demand: actual.map(|a| a as u32),
                forecast_error_pct_x100: error,
            }
        })
        .collect();

    let encoded = encode_to_vec(&forecasts).expect("encode forecasts");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress forecasts");
    let decompressed = decompress(&compressed).expect("decompress forecasts");
    let (decoded, _): (Vec<DemandForecast>, _) =
        decode_from_slice(&decompressed).expect("decode forecasts");
    assert_eq!(forecasts, decoded);
}

#[test]
fn test_inventory_reorder_points() {
    let reorder_points = vec![
        ReorderPoint {
            sku: "WIDGET-A-100".into(),
            warehouse_id: "WH-ATL-02".into(),
            on_hand: 45,
            safety_stock: 100,
            reorder_level: 200,
            economic_order_quantity: 500,
            lead_time_days: 14,
            avg_daily_demand: 15,
            needs_reorder: true,
            last_review_epoch: 1_742_000_000,
        },
        ReorderPoint {
            sku: "GADGET-B-200".into(),
            warehouse_id: "WH-ATL-02".into(),
            on_hand: 1_200,
            safety_stock: 300,
            reorder_level: 600,
            economic_order_quantity: 1_000,
            lead_time_days: 21,
            avg_daily_demand: 25,
            needs_reorder: false,
            last_review_epoch: 1_742_000_000,
        },
        ReorderPoint {
            sku: "SPROCKET-C-50".into(),
            warehouse_id: "WH-DFW-01".into(),
            on_hand: 180,
            safety_stock: 200,
            reorder_level: 400,
            economic_order_quantity: 750,
            lead_time_days: 30,
            avg_daily_demand: 10,
            needs_reorder: true,
            last_review_epoch: 1_741_900_000,
        },
    ];

    let encoded = encode_to_vec(&reorder_points).expect("encode reorder points");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress reorder points");
    let decompressed = decompress(&compressed).expect("decompress reorder points");
    let (decoded, _): (Vec<ReorderPoint>, _) =
        decode_from_slice(&decompressed).expect("decode reorder points");
    assert_eq!(reorder_points, decoded);
}

#[test]
fn test_supplier_scorecards_evaluation() {
    let scorecards = vec![
        SupplierScorecard {
            supplier_id: "SUP-CN-0042".into(),
            supplier_name: "Precision Components Shenzhen".into(),
            tier: SupplierTier::Strategic,
            on_time_delivery_pct_x100: 9820,
            quality_reject_rate_ppm: 120,
            lead_time_days_avg: 18,
            lead_time_days_stddev: 3,
            cost_competitiveness_score: 88,
            responsiveness_score: 92,
            sustainability_score: 75,
            overall_score: 8900,
            evaluation_period_start: 1_735_689_600,
            evaluation_period_end: 1_743_465_600,
        },
        SupplierScorecard {
            supplier_id: "SUP-DE-0105".into(),
            supplier_name: "Bavarian Precision Machining".into(),
            tier: SupplierTier::Preferred,
            on_time_delivery_pct_x100: 9950,
            quality_reject_rate_ppm: 35,
            lead_time_days_avg: 28,
            lead_time_days_stddev: 2,
            cost_competitiveness_score: 62,
            responsiveness_score: 85,
            sustainability_score: 91,
            overall_score: 8700,
            evaluation_period_start: 1_735_689_600,
            evaluation_period_end: 1_743_465_600,
        },
        SupplierScorecard {
            supplier_id: "SUP-IN-0231".into(),
            supplier_name: "Mumbai Textile Exports".into(),
            tier: SupplierTier::Conditional,
            on_time_delivery_pct_x100: 8200,
            quality_reject_rate_ppm: 890,
            lead_time_days_avg: 35,
            lead_time_days_stddev: 12,
            cost_competitiveness_score: 95,
            responsiveness_score: 60,
            sustainability_score: 45,
            overall_score: 6100,
            evaluation_period_start: 1_735_689_600,
            evaluation_period_end: 1_743_465_600,
        },
    ];

    let encoded = encode_to_vec(&scorecards).expect("encode scorecards");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress scorecards");
    let decompressed = decompress(&compressed).expect("decompress scorecards");
    let (decoded, _): (Vec<SupplierScorecard>, _) =
        decode_from_slice(&decompressed).expect("decode scorecards");
    assert_eq!(scorecards, decoded);
}

#[test]
fn test_carbon_footprint_per_shipment() {
    let footprints = vec![
        CarbonFootprint {
            shipment_id: "SHP-2026-99001".into(),
            transport_mode: TransportMode::OceanFreight,
            distance_km: 18_500,
            weight_kg: 24_000,
            co2_grams: 1_332_000,
            nox_milligrams: 25_900_000,
            sox_milligrams: 8_400_000,
            pm25_micrograms: 1_200_000_000,
            fuel_liters_x100: 48_000,
            offset_credits_purchased: false,
            emission_factor_source: "IMO GHG Study 2024".into(),
        },
        CarbonFootprint {
            shipment_id: "SHP-2026-99002".into(),
            transport_mode: TransportMode::AirFreight,
            distance_km: 9_200,
            weight_kg: 850,
            co2_grams: 4_692_000,
            nox_milligrams: 42_000_000,
            sox_milligrams: 1_100_000,
            pm25_micrograms: 350_000_000,
            fuel_liters_x100: 152_000,
            offset_credits_purchased: true,
            emission_factor_source: "ICAO Carbon Emissions Calculator v8".into(),
        },
    ];

    let encoded = encode_to_vec(&footprints).expect("encode carbon footprint");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress carbon footprint");
    let decompressed = decompress(&compressed).expect("decompress carbon footprint");
    let (decoded, _): (Vec<CarbonFootprint>, _) =
        decode_from_slice(&decompressed).expect("decode carbon footprint");
    assert_eq!(footprints, decoded);
}

#[test]
fn test_manifest_with_all_hazmat_classes() {
    let items: Vec<ManifestLineItem> = vec![
        HazmatClass::Explosives,
        HazmatClass::Gases,
        HazmatClass::FlammableLiquids,
        HazmatClass::FlammableSolids,
        HazmatClass::OxidizingSubstances,
        HazmatClass::ToxicSubstances,
        HazmatClass::RadioactiveMaterial,
        HazmatClass::Corrosives,
        HazmatClass::MiscellaneousDangerous,
    ]
    .into_iter()
    .enumerate()
    .map(|(i, haz)| ManifestLineItem {
        commodity_code: format!("HAZ-{:04}", i + 1),
        description: format!("Hazmat sample class {}", i + 1),
        quantity: 10,
        weight_grams: 5_000,
        volume_cm3: 3_000,
        declared_value_cents: 100_00,
        hazmat: Some(haz),
    })
    .collect();

    let manifest = ContainerManifest {
        manifest_id: "MAN-HAZ-2026-001".into(),
        container_number: "TRIU9988776".into(),
        container_size: ContainerSize::TwentyFoot,
        container_type: ContainerType::Tank,
        seal_number: "SEAL-HAZ-001".into(),
        shipper_name: "Chemical Logistics International".into(),
        consignee_name: "Industrial Chemicals EMEA BV".into(),
        origin_port_code: "USHOU".into(),
        destination_port_code: "BEANR".into(),
        gross_weight_kg: 18_000,
        line_items: items,
    };

    let encoded = encode_to_vec(&manifest).expect("encode hazmat manifest");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress hazmat manifest");
    let decompressed = decompress(&compressed).expect("decompress hazmat manifest");
    let (decoded, _): (ContainerManifest, _) =
        decode_from_slice(&decompressed).expect("decode hazmat manifest");
    assert_eq!(manifest, decoded);
}

#[test]
fn test_cold_chain_multiple_temperature_zones() {
    let zones = vec![
        (TemperatureZone::Frozen, -25_000, "FROZ-COMP-1"),
        (TemperatureZone::Chilled, 2_000, "CHILL-COMP-2"),
        (TemperatureZone::CoolRoom, 8_000, "COOL-SEC-3"),
        (TemperatureZone::Ambient, 20_000, "AMB-HOLD-4"),
        (TemperatureZone::Controlled, 15_000, "CTRL-BAY-5"),
    ];

    let logs: Vec<ColdChainLog> = zones
        .into_iter()
        .enumerate()
        .flat_map(|(zi, (zone, target, sensor))| {
            (0..10).map(move |i| ColdChainLog {
                shipment_id: format!("MULTI-ZONE-{}", zi + 1),
                sensor_id: sensor.to_string(),
                zone: zone.clone(),
                target_temp_milli_c: target,
                actual_temp_milli_c: target + (i as i32 - 5) * 100,
                humidity_percent_x10: 550 + (i * 10) as u16,
                timestamp_epoch_ms: 1_742_000_000_000 + (zi * 10 + i) as u64 * 60_000,
                is_excursion: false,
                excursion_duration_sec: None,
            })
        })
        .collect();

    let encoded = encode_to_vec(&logs).expect("encode multi-zone cold chain");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress multi-zone");
    let decompressed = decompress(&compressed).expect("decompress multi-zone");
    let (decoded, _): (Vec<ColdChainLog>, _) =
        decode_from_slice(&decompressed).expect("decode multi-zone");
    assert_eq!(logs, decoded);
}

#[test]
fn test_all_customs_statuses() {
    let statuses = vec![
        CustomsStatus::Pending,
        CustomsStatus::DocumentsSubmitted,
        CustomsStatus::UnderInspection,
        CustomsStatus::Cleared,
        CustomsStatus::HeldForExamination,
        CustomsStatus::Rejected,
        CustomsStatus::DutyPaid,
    ];

    let declarations: Vec<CustomsDeclaration> = statuses
        .into_iter()
        .enumerate()
        .map(|(i, status)| CustomsDeclaration {
            declaration_id: format!("CUS-TEST-{:04}", i),
            customs_office_code: format!("XX{:06}", i),
            declarant_name: format!("Test Declarant {}", i),
            importer_of_record: format!("Test Importer {}", i),
            status,
            hs_tariff_codes: vec![format!("{:04}.{:02}", 8400 + i, i * 10)],
            total_declared_value_cents: (i as u64 + 1) * 100_000,
            currency_code: "USD".into(),
            duty_amount_cents: (i as u64 + 1) * 5_000,
            vat_amount_cents: (i as u64 + 1) * 19_000,
            inspection_notes: vec![],
        })
        .collect();

    let encoded = encode_to_vec(&declarations).expect("encode all customs statuses");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress customs statuses");
    let decompressed = decompress(&compressed).expect("decompress customs statuses");
    let (decoded, _): (Vec<CustomsDeclaration>, _) =
        decode_from_slice(&decompressed).expect("decode customs statuses");
    assert_eq!(declarations, decoded);
}

#[test]
fn test_delivery_status_full_lifecycle() {
    let lifecycle_statuses = vec![
        DeliveryStatus::OrderReceived,
        DeliveryStatus::PickedUp,
        DeliveryStatus::InTransitToHub,
        DeliveryStatus::AtSortingFacility,
        DeliveryStatus::OutForDelivery,
        DeliveryStatus::DeliveryAttemptFailed,
        DeliveryStatus::OutForDelivery,
        DeliveryStatus::Delivered,
    ];

    let events: Vec<LastMileEvent> = lifecycle_statuses
        .into_iter()
        .enumerate()
        .map(|(i, status)| {
            let is_delivered = matches!(status, DeliveryStatus::Delivered);
            LastMileEvent {
                tracking_number: "LIFECYCLE-TEST-001".into(),
                status,
                timestamp_epoch_ms: 1_742_000_000_000 + i as u64 * 3_600_000,
                driver_id: format!("DRV-{:04}", 1000 + i),
                latitude_microdeg: 40_712_800 + i as i64 * 1_000,
                longitude_microdeg: -74_006_000 + i as i64 * 500,
                photo_proof_hash: if is_delivered {
                    Some("sha256:deadbeef01234567".into())
                } else {
                    None
                },
                recipient_name: if is_delivered {
                    Some("John Smith".into())
                } else {
                    None
                },
                delivery_notes: format!("Event #{} in lifecycle", i + 1),
            }
        })
        .collect();

    let encoded = encode_to_vec(&events).expect("encode lifecycle");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress lifecycle");
    let decompressed = decompress(&compressed).expect("decompress lifecycle");
    let (decoded, _): (Vec<LastMileEvent>, _) =
        decode_from_slice(&decompressed).expect("decode lifecycle");
    assert_eq!(events, decoded);
}

#[test]
fn test_large_intermodal_network_compression_ratio() {
    let shipments: Vec<IntermodalShipment> = (0..25)
        .map(|s| IntermodalShipment {
            shipment_id: format!("NET-IMS-{:05}", s),
            booking_ref: format!("NET-BKG-{:05}", s),
            legs: (0..4)
                .map(|l| IntermodalLeg {
                    leg_id: l + 1,
                    mode: match l {
                        0 => TransportMode::TruckLoad,
                        1 => TransportMode::OceanFreight,
                        2 => TransportMode::RailFreight,
                        _ => TransportMode::LessThanTruckLoad,
                    },
                    carrier: format!("Carrier-{}-{}", s, l),
                    origin_terminal: format!("Terminal-{}-{}-O", s, l),
                    destination_terminal: format!("Terminal-{}-{}-D", s, l),
                    departure_epoch_ms: 1_742_000_000_000 + s as u64 * 86_400_000,
                    arrival_epoch_ms: 1_742_000_000_000 + s as u64 * 86_400_000 + 43_200_000,
                    equipment_number: format!("EQ-{:05}-{}", s, l),
                    distance_meters: 500_000 + l as u64 * 2_000_000,
                })
                .collect(),
            total_transit_hours: 480 + s * 12,
            total_distance_meters: 6_500_000 + s as u64 * 100_000,
            carbon_kg_x100: 250_000 + s as u64 * 5_000,
        })
        .collect();

    let encoded = encode_to_vec(&shipments).expect("encode network");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress network");

    let ratio_pct = (compressed.len() as f64 / encoded.len() as f64) * 100.0;
    assert!(
        ratio_pct < 85.0,
        "expected compression ratio < 85%, got {:.1}% ({} -> {} bytes)",
        ratio_pct,
        encoded.len(),
        compressed.len()
    );

    let decompressed = decompress(&compressed).expect("decompress network");
    let (decoded, _): (Vec<IntermodalShipment>, _) =
        decode_from_slice(&decompressed).expect("decode network");
    assert_eq!(shipments, decoded);
}

#[test]
fn test_all_return_reasons_roundtrip() {
    let reasons = vec![
        ReturnReason::Damaged,
        ReturnReason::WrongItem,
        ReturnReason::DefectiveProduct,
        ReturnReason::CustomerChanged,
        ReturnReason::ExpiredShelfLife,
        ReturnReason::RecallNotice,
        ReturnReason::QualityFail,
    ];

    let returns: Vec<ReturnProcessing> = reasons
        .into_iter()
        .enumerate()
        .map(|(i, reason)| ReturnProcessing {
            rma_number: format!("RMA-REASON-{:03}", i),
            original_order_id: format!("ORD-REASON-{:03}", i),
            sku: format!("SKU-REASON-{:03}", i),
            quantity: (i as u32 + 1) * 2,
            reason,
            condition_grade: (9 - i) as u8,
            refund_amount_cents: (i as u64 + 1) * 2500,
            restocking_fee_cents: if i > 3 { 500 } else { 0 },
            disposition: format!("Disposition path {}", i),
            inspected_by: format!("INSPECTOR-{}", i % 3),
            processed_epoch_ms: 1_742_200_000_000 + i as u64 * 60_000,
        })
        .collect();

    let encoded = encode_to_vec(&returns).expect("encode all reasons");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress all reasons");
    let decompressed = decompress(&compressed).expect("decompress all reasons");
    let (decoded, _): (Vec<ReturnProcessing>, _) =
        decode_from_slice(&decompressed).expect("decode all reasons");
    assert_eq!(returns, decoded);
}
