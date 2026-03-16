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

// ---------------------------------------------------------------------------
// Domain types – Logistics, Shipping & Global Supply Chain Management
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ShippingContainer {
    container_id: String,
    iso_size_type: String,
    tare_weight_kg: u32,
    max_payload_kg: u32,
    current_payload_kg: u32,
    seal_numbers: Vec<String>,
    is_refrigerated: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContainerManifestLine {
    line_no: u32,
    commodity_code: String,
    description: String,
    piece_count: u32,
    gross_weight_kg: f64,
    declared_value_usd: f64,
    origin_country: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BillOfLading {
    bol_number: String,
    shipper_name: String,
    consignee_name: String,
    notify_party: Option<String>,
    port_of_loading: String,
    port_of_discharge: String,
    vessel_name: String,
    voyage_number: String,
    container_ids: Vec<String>,
    freight_prepaid: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CustomsDeclarationType {
    Import,
    Export,
    Transit,
    TemporaryAdmission,
    Reexport,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CustomsDeclaration {
    declaration_id: String,
    declaration_type: CustomsDeclarationType,
    hs_codes: Vec<String>,
    total_value_usd: f64,
    duty_rate_bps: u32,
    vat_rate_bps: u32,
    country_of_origin: String,
    destination_country: String,
    approved: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WarehouseBinLocation {
    warehouse_code: String,
    zone: char,
    aisle: u16,
    rack: u16,
    shelf_level: u8,
    bin_id: String,
    sku: String,
    qty_on_hand: u32,
    max_capacity: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LastMileDeliveryStop {
    stop_seq: u16,
    address: String,
    postal_code: String,
    recipient_name: String,
    package_ids: Vec<String>,
    time_window_start_min: u16,
    time_window_end_min: u16,
    signature_required: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DeliveryRoute {
    route_id: String,
    driver_id: String,
    vehicle_plate: String,
    depot_code: String,
    stops: Vec<LastMileDeliveryStop>,
    total_distance_m: u32,
    estimated_duration_min: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FreightMode {
    FullContainerLoad,
    LessThanContainerLoad,
    BreakBulk,
    RoRo,
    AirFreight,
    RailIntermodal,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FreightRateQuote {
    quote_id: String,
    mode: FreightMode,
    origin_port: String,
    destination_port: String,
    base_rate_usd: f64,
    fuel_surcharge_usd: f64,
    war_risk_surcharge_usd: f64,
    peak_season_surcharge_usd: f64,
    currency: String,
    valid_until_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CargoInsurancePolicy {
    policy_number: String,
    insured_party: String,
    cargo_description: String,
    insured_value_usd: f64,
    premium_usd: f64,
    deductible_usd: f64,
    coverage_type: String,
    voyage_from: String,
    voyage_to: String,
    policy_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TerminalOperationType {
    Discharge,
    Loading,
    Transshipment,
    Restow,
    InspectionHold,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PortTerminalOperation {
    operation_id: String,
    terminal_code: String,
    berth_number: u8,
    vessel_name: String,
    operation_type: TerminalOperationType,
    container_count: u32,
    crane_id: String,
    start_epoch: u64,
    end_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CrossDockSchedule {
    schedule_id: String,
    dock_number: u16,
    inbound_carrier: String,
    outbound_carrier: String,
    arrival_epoch: u64,
    departure_epoch: u64,
    pallet_count: u32,
    priority: u8,
    is_hazmat: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ReverseLogisticsReason {
    CustomerReturn,
    Defective,
    Recall,
    EndOfLife,
    Overstock,
    WrongShipment,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReverseLogisticsOrder {
    rma_number: String,
    original_order_id: String,
    reason: ReverseLogisticsReason,
    item_skus: Vec<String>,
    pickup_address: String,
    return_warehouse: String,
    refund_amount_cents: u64,
    disposition: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ColdChainTemperatureLog {
    sensor_id: String,
    container_id: String,
    readings: Vec<i16>,
    min_allowed_celsius_x10: i16,
    max_allowed_celsius_x10: i16,
    breach_count: u32,
    log_interval_sec: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FleetGpsReading {
    vehicle_id: String,
    latitude_microdeg: i64,
    longitude_microdeg: i64,
    speed_kmh_x10: u16,
    heading_deg: u16,
    timestamp_epoch: u64,
    ignition_on: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DemurrageCharge {
    container_id: String,
    port_code: String,
    free_days: u8,
    days_used: u8,
    daily_rate_usd: f64,
    total_charge_usd: f64,
    currency: String,
    waiver_applied: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HazmatShipment {
    un_number: String,
    proper_shipping_name: String,
    hazard_class: String,
    packing_group: String,
    quantity_kg: f64,
    emergency_contact: String,
    placard_required: bool,
    marine_pollutant: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct IncotermsAgreement {
    agreement_id: String,
    incoterm: String,
    seller: String,
    buyer: String,
    named_place: String,
    risk_transfer_point: String,
    seller_obligations: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DropshipOrder {
    order_id: String,
    supplier_id: String,
    customer_address: String,
    line_items: Vec<DropshipLineItem>,
    shipping_method: String,
    total_cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DropshipLineItem {
    sku: String,
    qty: u32,
    unit_cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FreightForwarderBooking {
    booking_ref: String,
    forwarder_name: String,
    origin_city: String,
    destination_city: String,
    cargo_ready_epoch: u64,
    estimated_arrival_epoch: u64,
    total_volume_cbm: f64,
    total_weight_kg: f64,
    modes: Vec<FreightMode>,
    is_dg: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_shipping_container_roundtrip() {
    let cfg = config::standard();
    let val = ShippingContainer {
        container_id: "MSCU1234567".to_string(),
        iso_size_type: "40HC".to_string(),
        tare_weight_kg: 3800,
        max_payload_kg: 26680,
        current_payload_kg: 22150,
        seal_numbers: vec!["SL-90001".to_string(), "SL-90002".to_string()],
        is_refrigerated: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ShippingContainer");
    let (decoded, _): (ShippingContainer, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ShippingContainer");
    assert_eq!(val, decoded);
}

#[test]
fn test_container_manifest_line_roundtrip() {
    let cfg = config::standard();
    let val = ContainerManifestLine {
        line_no: 1,
        commodity_code: "8471.30".to_string(),
        description: "Portable digital computers weighing <10kg".to_string(),
        piece_count: 500,
        gross_weight_kg: 4200.5,
        declared_value_usd: 375000.0,
        origin_country: "CN".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ContainerManifestLine");
    let (decoded, _): (ContainerManifestLine, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ContainerManifestLine");
    assert_eq!(val, decoded);
}

#[test]
fn test_bill_of_lading_with_notify_party() {
    let cfg = config::standard();
    let val = BillOfLading {
        bol_number: "MAEU123456789".to_string(),
        shipper_name: "Shenzhen Electronics Co Ltd".to_string(),
        consignee_name: "Hamburg Logistics GmbH".to_string(),
        notify_party: Some("Deutsche Bank AG - Trade Finance".to_string()),
        port_of_loading: "CNSHA".to_string(),
        port_of_discharge: "DEHAM".to_string(),
        vessel_name: "Ever Given".to_string(),
        voyage_number: "V.052E".to_string(),
        container_ids: vec![
            "MSCU1234567".to_string(),
            "MSCU1234568".to_string(),
            "MSCU1234569".to_string(),
        ],
        freight_prepaid: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BillOfLading");
    let (decoded, _): (BillOfLading, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BillOfLading");
    assert_eq!(val, decoded);
}

#[test]
fn test_customs_declaration_import() {
    let cfg = config::standard();
    let val = CustomsDeclaration {
        declaration_id: "US-IMP-2026-0003142".to_string(),
        declaration_type: CustomsDeclarationType::Import,
        hs_codes: vec!["8471.30".to_string(), "8471.41".to_string()],
        total_value_usd: 512000.0,
        duty_rate_bps: 250,
        vat_rate_bps: 0,
        country_of_origin: "CN".to_string(),
        destination_country: "US".to_string(),
        approved: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CustomsDeclaration import");
    let (decoded, _): (CustomsDeclaration, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CustomsDeclaration import");
    assert_eq!(val, decoded);
}

#[test]
fn test_customs_declaration_transit() {
    let cfg = config::standard();
    let val = CustomsDeclaration {
        declaration_id: "EU-TRN-2026-0088741".to_string(),
        declaration_type: CustomsDeclarationType::Transit,
        hs_codes: vec!["2204.21".to_string()],
        total_value_usd: 48000.0,
        duty_rate_bps: 0,
        vat_rate_bps: 0,
        country_of_origin: "IT".to_string(),
        destination_country: "GB".to_string(),
        approved: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CustomsDeclaration transit");
    let (decoded, _): (CustomsDeclaration, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CustomsDeclaration transit");
    assert_eq!(val, decoded);
}

#[test]
fn test_warehouse_bin_location_roundtrip() {
    let cfg = config::standard();
    let val = WarehouseBinLocation {
        warehouse_code: "WH-LAX-02".to_string(),
        zone: 'C',
        aisle: 14,
        rack: 7,
        shelf_level: 3,
        bin_id: "C-14-07-03".to_string(),
        sku: "ELEC-LAPTOP-15PRO".to_string(),
        qty_on_hand: 42,
        max_capacity: 60,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode WarehouseBinLocation");
    let (decoded, _): (WarehouseBinLocation, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WarehouseBinLocation");
    assert_eq!(val, decoded);
}

#[test]
fn test_delivery_route_multiple_stops() {
    let cfg = config::standard();
    let val = DeliveryRoute {
        route_id: "RT-20260315-LAX-042".to_string(),
        driver_id: "DRV-8821".to_string(),
        vehicle_plate: "7ABC123".to_string(),
        depot_code: "DEP-LAX-SOUTH".to_string(),
        stops: vec![
            LastMileDeliveryStop {
                stop_seq: 1,
                address: "1234 Maple Drive, Torrance, CA 90501".to_string(),
                postal_code: "90501".to_string(),
                recipient_name: "Alice Johnson".to_string(),
                package_ids: vec!["PKG-00991".to_string()],
                time_window_start_min: 480,
                time_window_end_min: 600,
                signature_required: true,
            },
            LastMileDeliveryStop {
                stop_seq: 2,
                address: "5678 Oak Blvd, Gardena, CA 90248".to_string(),
                postal_code: "90248".to_string(),
                recipient_name: "Bob Smith".to_string(),
                package_ids: vec!["PKG-00992".to_string(), "PKG-00993".to_string()],
                time_window_start_min: 600,
                time_window_end_min: 720,
                signature_required: false,
            },
            LastMileDeliveryStop {
                stop_seq: 3,
                address: "910 Pine St, Carson, CA 90745".to_string(),
                postal_code: "90745".to_string(),
                recipient_name: "Carol Lee".to_string(),
                package_ids: vec!["PKG-00994".to_string()],
                time_window_start_min: 720,
                time_window_end_min: 840,
                signature_required: true,
            },
        ],
        total_distance_m: 47200,
        estimated_duration_min: 185,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DeliveryRoute");
    let (decoded, _): (DeliveryRoute, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DeliveryRoute");
    assert_eq!(val, decoded);
}

#[test]
fn test_freight_rate_quote_fcl() {
    let cfg = config::standard();
    let val = FreightRateQuote {
        quote_id: "FRQ-2026-SHARTM-001".to_string(),
        mode: FreightMode::FullContainerLoad,
        origin_port: "CNSHA".to_string(),
        destination_port: "NLRTM".to_string(),
        base_rate_usd: 2850.0,
        fuel_surcharge_usd: 425.0,
        war_risk_surcharge_usd: 35.0,
        peak_season_surcharge_usd: 600.0,
        currency: "USD".to_string(),
        valid_until_epoch: 1742169600,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode FreightRateQuote FCL");
    let (decoded, _): (FreightRateQuote, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FreightRateQuote FCL");
    assert_eq!(val, decoded);
}

#[test]
fn test_freight_rate_quote_air() {
    let cfg = config::standard();
    let val = FreightRateQuote {
        quote_id: "FRQ-2026-HKGLHR-AIR".to_string(),
        mode: FreightMode::AirFreight,
        origin_port: "HKHKG".to_string(),
        destination_port: "GBLHR".to_string(),
        base_rate_usd: 4.25,
        fuel_surcharge_usd: 1.10,
        war_risk_surcharge_usd: 0.15,
        peak_season_surcharge_usd: 0.0,
        currency: "USD".to_string(),
        valid_until_epoch: 1741910400,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode FreightRateQuote air");
    let (decoded, _): (FreightRateQuote, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FreightRateQuote air");
    assert_eq!(val, decoded);
}

#[test]
fn test_cargo_insurance_policy_roundtrip() {
    let cfg = config::standard();
    let val = CargoInsurancePolicy {
        policy_number: "MAR-2026-UK-004512".to_string(),
        insured_party: "Nordic Fish Exports AS".to_string(),
        cargo_description: "Frozen Atlantic salmon fillets".to_string(),
        insured_value_usd: 285000.0,
        premium_usd: 1425.0,
        deductible_usd: 5000.0,
        coverage_type: "All Risks - ICC(A)".to_string(),
        voyage_from: "NOBGO".to_string(),
        voyage_to: "JPYOK".to_string(),
        policy_active: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CargoInsurancePolicy");
    let (decoded, _): (CargoInsurancePolicy, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CargoInsurancePolicy");
    assert_eq!(val, decoded);
}

#[test]
fn test_port_terminal_operation_discharge() {
    let cfg = config::standard();
    let val = PortTerminalOperation {
        operation_id: "OPS-DEHAM-20260315-001".to_string(),
        terminal_code: "CTH".to_string(),
        berth_number: 4,
        vessel_name: "MSC Irina".to_string(),
        operation_type: TerminalOperationType::Discharge,
        container_count: 1850,
        crane_id: "STS-07".to_string(),
        start_epoch: 1742022000,
        end_epoch: Some(1742050800),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PortTerminalOperation");
    let (decoded, _): (PortTerminalOperation, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PortTerminalOperation");
    assert_eq!(val, decoded);
}

#[test]
fn test_port_terminal_operation_in_progress() {
    let cfg = config::standard();
    let val = PortTerminalOperation {
        operation_id: "OPS-SGSIN-20260315-014".to_string(),
        terminal_code: "PSA-T2".to_string(),
        berth_number: 11,
        vessel_name: "HMM Algeciras".to_string(),
        operation_type: TerminalOperationType::Transshipment,
        container_count: 620,
        crane_id: "QC-22".to_string(),
        start_epoch: 1742043600,
        end_epoch: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PortTerminalOp in-progress");
    let (decoded, _): (PortTerminalOperation, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PortTerminalOp in-progress");
    assert_eq!(val, decoded);
}

#[test]
fn test_cross_dock_schedule_hazmat() {
    let cfg = config::standard();
    let val = CrossDockSchedule {
        schedule_id: "XD-20260315-09".to_string(),
        dock_number: 14,
        inbound_carrier: "FedEx Freight".to_string(),
        outbound_carrier: "XPO Logistics".to_string(),
        arrival_epoch: 1742028000,
        departure_epoch: 1742035200,
        pallet_count: 24,
        priority: 1,
        is_hazmat: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CrossDockSchedule hazmat");
    let (decoded, _): (CrossDockSchedule, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CrossDockSchedule hazmat");
    assert_eq!(val, decoded);
}

#[test]
fn test_reverse_logistics_customer_return() {
    let cfg = config::standard();
    let val = ReverseLogisticsOrder {
        rma_number: "RMA-2026-0041523".to_string(),
        original_order_id: "ORD-2026-1098234".to_string(),
        reason: ReverseLogisticsReason::CustomerReturn,
        item_skus: vec![
            "SHOE-RUN-42-BLK".to_string(),
            "SOCK-SPORT-M-WHT".to_string(),
        ],
        pickup_address: "321 Elm Ct, Austin, TX 78701".to_string(),
        return_warehouse: "WH-DAL-01".to_string(),
        refund_amount_cents: 15499,
        disposition: "inspect_and_restock".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ReverseLogisticsOrder");
    let (decoded, _): (ReverseLogisticsOrder, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ReverseLogisticsOrder");
    assert_eq!(val, decoded);
}

#[test]
fn test_reverse_logistics_recall() {
    let cfg = config::standard();
    let val = ReverseLogisticsOrder {
        rma_number: "RMA-2026-RECALL-0078".to_string(),
        original_order_id: "ORD-2025-8812001".to_string(),
        reason: ReverseLogisticsReason::Recall,
        item_skus: vec!["BATT-LIION-5000".to_string()],
        pickup_address: "45 Industrial Park Rd, Memphis, TN 38118".to_string(),
        return_warehouse: "WH-MEM-HAZMAT".to_string(),
        refund_amount_cents: 4999,
        disposition: "destroy_certified".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ReverseLogistics recall");
    let (decoded, _): (ReverseLogisticsOrder, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ReverseLogistics recall");
    assert_eq!(val, decoded);
}

#[test]
fn test_cold_chain_temperature_log() {
    let cfg = config::standard();
    let val = ColdChainTemperatureLog {
        sensor_id: "SENS-RF-00412".to_string(),
        container_id: "MAEU7654321".to_string(),
        readings: vec![-185, -182, -188, -190, -179, -183, -200, -195, -187, -184],
        min_allowed_celsius_x10: -250,
        max_allowed_celsius_x10: -150,
        breach_count: 0,
        log_interval_sec: 300,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ColdChainTemperatureLog");
    let (decoded, _): (ColdChainTemperatureLog, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ColdChainTemperatureLog");
    assert_eq!(val, decoded);
}

#[test]
fn test_fleet_gps_reading_roundtrip() {
    let cfg = config::standard();
    let val = FleetGpsReading {
        vehicle_id: "TRK-EU-4471".to_string(),
        latitude_microdeg: 52_520_008,
        longitude_microdeg: 13_404_954,
        speed_kmh_x10: 875,
        heading_deg: 225,
        timestamp_epoch: 1742050000,
        ignition_on: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode FleetGpsReading");
    let (decoded, _): (FleetGpsReading, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FleetGpsReading");
    assert_eq!(val, decoded);
}

#[test]
fn test_demurrage_charge_with_waiver() {
    let cfg = config::standard();
    let val = DemurrageCharge {
        container_id: "OOLU8889123".to_string(),
        port_code: "USLAX".to_string(),
        free_days: 5,
        days_used: 12,
        daily_rate_usd: 175.0,
        total_charge_usd: 1225.0,
        currency: "USD".to_string(),
        waiver_applied: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DemurrageCharge waiver");
    let (decoded, _): (DemurrageCharge, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DemurrageCharge waiver");
    assert_eq!(val, decoded);
}

#[test]
fn test_hazmat_shipment_roundtrip() {
    let cfg = config::standard();
    let val = HazmatShipment {
        un_number: "UN1203".to_string(),
        proper_shipping_name: "Gasoline".to_string(),
        hazard_class: "3".to_string(),
        packing_group: "II".to_string(),
        quantity_kg: 18500.0,
        emergency_contact: "+1-800-424-9300 CHEMTREC".to_string(),
        placard_required: true,
        marine_pollutant: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode HazmatShipment");
    let (decoded, _): (HazmatShipment, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HazmatShipment");
    assert_eq!(val, decoded);
}

#[test]
fn test_incoterms_agreement_roundtrip() {
    let cfg = config::standard();
    let val = IncotermsAgreement {
        agreement_id: "INC-2026-CIF-00312".to_string(),
        incoterm: "CIF".to_string(),
        seller: "Yokohama Auto Parts KK".to_string(),
        buyer: "Detroit Assembly Corp".to_string(),
        named_place: "Port of Long Beach, CA".to_string(),
        risk_transfer_point: "Ship rail at port of loading".to_string(),
        seller_obligations: vec![
            "Arrange and pay for carriage".to_string(),
            "Obtain marine insurance ICC(C) minimum".to_string(),
            "Clear goods for export".to_string(),
            "Provide commercial invoice and transport document".to_string(),
        ],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode IncotermsAgreement");
    let (decoded, _): (IncotermsAgreement, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode IncotermsAgreement");
    assert_eq!(val, decoded);
}

#[test]
fn test_dropship_order_multiple_lines() {
    let cfg = config::standard();
    let val = DropshipOrder {
        order_id: "DS-2026-0098712".to_string(),
        supplier_id: "SUP-SHENZEN-044".to_string(),
        customer_address: "789 Broadway, New York, NY 10003".to_string(),
        line_items: vec![
            DropshipLineItem {
                sku: "USB-C-CABLE-2M".to_string(),
                qty: 3,
                unit_cost_cents: 450,
            },
            DropshipLineItem {
                sku: "PHONE-CASE-14PRO-CLR".to_string(),
                qty: 1,
                unit_cost_cents: 820,
            },
            DropshipLineItem {
                sku: "SCREEN-PROTECTOR-14".to_string(),
                qty: 2,
                unit_cost_cents: 310,
            },
        ],
        shipping_method: "ePacket".to_string(),
        total_cost_cents: 3340,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DropshipOrder");
    let (decoded, _): (DropshipOrder, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DropshipOrder");
    assert_eq!(val, decoded);
}

#[test]
fn test_freight_forwarder_booking_multimodal() {
    let cfg = config::standard();
    let val = FreightForwarderBooking {
        booking_ref: "FFB-KN-2026-EU-08841".to_string(),
        forwarder_name: "Kuehne+Nagel International AG".to_string(),
        origin_city: "Stuttgart".to_string(),
        destination_city: "Chicago".to_string(),
        cargo_ready_epoch: 1742169600,
        estimated_arrival_epoch: 1744761600,
        total_volume_cbm: 38.5,
        total_weight_kg: 12400.0,
        modes: vec![
            FreightMode::RailIntermodal,
            FreightMode::FullContainerLoad,
            FreightMode::RailIntermodal,
        ],
        is_dg: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode FreightForwarderBooking");
    let (decoded, _): (FreightForwarderBooking, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FreightForwarderBooking");
    assert_eq!(val, decoded);
}
