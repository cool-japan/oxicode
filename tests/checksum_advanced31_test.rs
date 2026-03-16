//! Advanced checksum tests for OxiCode — data center operations & infrastructure domain.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced31_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types: data center operations and infrastructure
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RackUnit {
    Empty,
    Server { asset_tag: String, height_u: u8 },
    NetworkSwitch { model: String, port_count: u16 },
    PatchPanel { cable_count: u16 },
    StorageArray { capacity_tb: u64 },
    BlankPanel { height_u: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ServerRackLayout {
    rack_id: String,
    location_row: String,
    location_col: u32,
    total_u: u8,
    max_power_kw: u32,
    current_draw_watts: u32,
    weight_kg: u32,
    units: Vec<(u8, RackUnit)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AssetLifecyclePhase {
    Procurement {
        po_number: String,
        vendor: String,
        cost_cents: u64,
    },
    Staging {
        warehouse_location: String,
    },
    Deployed {
        rack_id: String,
        u_position: u8,
    },
    Maintenance {
        ticket_id: String,
        reason: String,
    },
    Decommissioned {
        disposal_method: String,
        date_epoch_s: u64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HardwareLifecycleRecord {
    asset_id: String,
    serial_number: String,
    model: String,
    manufacturer: String,
    purchase_date_epoch_s: u64,
    warranty_expiry_epoch_s: u64,
    phase: AssetLifecyclePhase,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoolingUnitType {
    Crah {
        fan_speed_rpm: u32,
        coil_temp_c_x100: i32,
    },
    Crac {
        compressor_on: bool,
        refrigerant_pressure_psi_x10: u32,
    },
    InRowCooler {
        row_id: String,
        capacity_kw: u32,
    },
    RearDoorHeatExchanger {
        rack_id: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoolingSystemMetrics {
    unit_id: String,
    unit_type: CoolingUnitType,
    hot_aisle_temp_c_x100: i32,
    cold_aisle_temp_c_x100: i32,
    supply_temp_c_x100: i32,
    return_temp_c_x100: i32,
    humidity_pct_x100: u32,
    airflow_cfm: u32,
    timestamp_epoch_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BatteryChemistry {
    LeadAcid,
    LithiumIon,
    LithiumIronPhosphate,
    NickelCadmium,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UpsBatteryHealth {
    ups_id: String,
    battery_string_id: u32,
    chemistry: BatteryChemistry,
    voltage_mv: u32,
    current_ma: i32,
    temperature_c_x100: i32,
    state_of_charge_pct_x10: u16,
    cycles_completed: u32,
    estimated_runtime_seconds: u32,
    last_test_epoch_s: u64,
    replace_by_epoch_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PduOutletState {
    Off,
    On { current_ma: u32, voltage_mv: u32 },
    Locked,
    Fault { error_code: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PduPowerDistribution {
    pdu_id: String,
    feed_label: String,
    total_capacity_amps: u32,
    total_current_draw_ma: u32,
    voltage_mv: u32,
    phase_count: u8,
    outlets: Vec<(u8, PduOutletState)>,
    breaker_tripped: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PortSpeed {
    Speed1G,
    Speed10G,
    Speed25G,
    Speed40G,
    Speed100G,
    Speed400G,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PortStatus {
    Up { speed: PortSpeed, vlan_id: u16 },
    Down,
    AdminDown,
    ErrDisabled { reason: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkSwitchPortConfig {
    switch_id: String,
    port_index: u32,
    label: String,
    status: PortStatus,
    mac_address: Vec<u8>,
    tx_bytes: u64,
    rx_bytes: u64,
    tx_errors: u64,
    rx_errors: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CableType {
    Cat6,
    Cat6a,
    Om3Fiber,
    Om4Fiber,
    SingleModeFiber,
    DacCopper,
    PowerC13,
    PowerC19,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CableManagementRecord {
    cable_id: String,
    cable_type: CableType,
    length_cm: u32,
    source_rack: String,
    source_port: String,
    dest_rack: String,
    dest_port: String,
    installed_epoch_s: u64,
    label_color: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PueCalculation {
    facility_id: String,
    measurement_epoch_s: u64,
    total_facility_power_watts: u64,
    it_equipment_power_watts: u64,
    cooling_power_watts: u64,
    lighting_power_watts: u64,
    pue_x1000: u32,
    target_pue_x1000: u32,
    measurement_interval_seconds: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CapacityPlanningForecast {
    forecast_id: String,
    data_hall_id: String,
    current_rack_count: u32,
    max_rack_count: u32,
    current_power_kw: u32,
    max_power_kw: u32,
    current_cooling_kw: u32,
    max_cooling_kw: u32,
    projected_exhaustion_epoch_s: u64,
    monthly_growth_rate_pct_x100: u32,
    planned_expansions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SuppressionAgent {
    Fm200,
    Novec1230,
    InertGasIg541,
    InertGasIg55,
    WaterMist,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FireSuppressionStatus {
    zone_id: String,
    agent: SuppressionAgent,
    cylinder_pressure_psi_x10: u32,
    smoke_detectors_ok: u32,
    smoke_detectors_fault: u32,
    vesda_air_sampling_active: bool,
    last_inspection_epoch_s: u64,
    system_armed: bool,
    discharge_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AccessEventKind {
    BadgeIn,
    BadgeOut,
    TailgateDetected,
    DoorHeldOpen { duration_seconds: u32 },
    ForcedEntry,
    BiometricVerified,
    VisitorEscorted { visitor_name: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PhysicalSecurityAccessLog {
    event_id: u64,
    timestamp_epoch_s: u64,
    person_id: String,
    door_id: String,
    zone: String,
    event_kind: AccessEventKind,
    granted: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnvironmentalSensorReading {
    sensor_id: String,
    location: String,
    temperature_c_x100: i32,
    humidity_pct_x100: u32,
    differential_pressure_pa_x10: i32,
    water_leak_detected: bool,
    particulate_count: u32,
    timestamp_epoch_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlaComplianceMetric {
    service_id: String,
    period_start_epoch_s: u64,
    period_end_epoch_s: u64,
    uptime_pct_x10000: u32,
    target_uptime_pct_x10000: u32,
    incidents_count: u32,
    mttr_seconds: u32,
    mtbf_seconds: u64,
    penalty_credits_cents: u64,
    compliant: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FailoverResult {
    Success { switchover_ms: u64 },
    PartialFailure { components_failed: Vec<String> },
    TotalFailure { root_cause: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DisasterRecoveryFailoverTest {
    test_id: String,
    primary_site: String,
    dr_site: String,
    test_epoch_s: u64,
    rpo_seconds: u64,
    rto_seconds: u64,
    actual_recovery_seconds: u64,
    data_loss_bytes: u64,
    result: FailoverResult,
    services_tested: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DcimRecord {
    facility_name: String,
    region: String,
    tier_level: u8,
    total_sqft: u64,
    raised_floor_height_cm: u32,
    data_halls: Vec<String>,
    power_feeds: Vec<String>,
    commissioning_epoch_s: u64,
    certification_expiry_epoch_s: u64,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Test 1: server rack layout with mixed equipment
#[test]
fn test_server_rack_layout_roundtrip() {
    let rack = ServerRackLayout {
        rack_id: "DC1-R01-A05".into(),
        location_row: "A".into(),
        location_col: 5,
        total_u: 42,
        max_power_kw: 20,
        current_draw_watts: 14_500,
        weight_kg: 820,
        units: vec![
            (1, RackUnit::PatchPanel { cable_count: 48 }),
            (
                2,
                RackUnit::NetworkSwitch {
                    model: "Arista-7050X3".into(),
                    port_count: 48,
                },
            ),
            (
                4,
                RackUnit::Server {
                    asset_tag: "SRV-00421".into(),
                    height_u: 2,
                },
            ),
            (
                6,
                RackUnit::Server {
                    asset_tag: "SRV-00422".into(),
                    height_u: 2,
                },
            ),
            (8, RackUnit::StorageArray { capacity_tb: 960 }),
            (12, RackUnit::BlankPanel { height_u: 1 }),
        ],
    };
    let encoded = encode_with_checksum(&rack).expect("encode rack layout");
    let (decoded, _): (ServerRackLayout, _) =
        decode_with_checksum(&encoded).expect("decode rack layout");
    assert_eq!(rack, decoded);
}

/// Test 2: DCIM facility record
#[test]
fn test_dcim_record_roundtrip() {
    let dcim = DcimRecord {
        facility_name: "EU-WEST-1 Primary".into(),
        region: "eu-west-1".into(),
        tier_level: 3,
        total_sqft: 120_000,
        raised_floor_height_cm: 90,
        data_halls: vec!["DH-A".into(), "DH-B".into(), "DH-C".into()],
        power_feeds: vec!["UTIL-A 10MVA".into(), "UTIL-B 10MVA".into()],
        commissioning_epoch_s: 1_577_836_800,
        certification_expiry_epoch_s: 1_735_689_600,
    };
    let encoded = encode_with_checksum(&dcim).expect("encode DCIM record");
    let (decoded, _): (DcimRecord, _) = decode_with_checksum(&encoded).expect("decode DCIM record");
    assert_eq!(dcim, decoded);
}

/// Test 3: cooling system metrics — CRAH unit
#[test]
fn test_cooling_crah_metrics_roundtrip() {
    let metrics = CoolingSystemMetrics {
        unit_id: "CRAH-DH-A-01".into(),
        unit_type: CoolingUnitType::Crah {
            fan_speed_rpm: 1_800,
            coil_temp_c_x100: 750,
        },
        hot_aisle_temp_c_x100: 3520,
        cold_aisle_temp_c_x100: 2100,
        supply_temp_c_x100: 1800,
        return_temp_c_x100: 3200,
        humidity_pct_x100: 4500,
        airflow_cfm: 12_000,
        timestamp_epoch_s: 1_710_000_000,
    };
    let encoded = encode_with_checksum(&metrics).expect("encode CRAH metrics");
    let (decoded, _): (CoolingSystemMetrics, _) =
        decode_with_checksum(&encoded).expect("decode CRAH metrics");
    assert_eq!(metrics, decoded);
}

/// Test 4: cooling system metrics — CRAC unit
#[test]
fn test_cooling_crac_metrics_roundtrip() {
    let metrics = CoolingSystemMetrics {
        unit_id: "CRAC-DH-B-03".into(),
        unit_type: CoolingUnitType::Crac {
            compressor_on: true,
            refrigerant_pressure_psi_x10: 2_450,
        },
        hot_aisle_temp_c_x100: 3680,
        cold_aisle_temp_c_x100: 2250,
        supply_temp_c_x100: 1650,
        return_temp_c_x100: 3400,
        humidity_pct_x100: 4200,
        airflow_cfm: 8_500,
        timestamp_epoch_s: 1_710_000_300,
    };
    let encoded = encode_with_checksum(&metrics).expect("encode CRAC metrics");
    let (decoded, _): (CoolingSystemMetrics, _) =
        decode_with_checksum(&encoded).expect("decode CRAC metrics");
    assert_eq!(metrics, decoded);
}

/// Test 5: UPS battery health — lithium-ion
#[test]
fn test_ups_battery_health_roundtrip() {
    let battery = UpsBatteryHealth {
        ups_id: "UPS-A-01".into(),
        battery_string_id: 1,
        chemistry: BatteryChemistry::LithiumIon,
        voltage_mv: 432_000,
        current_ma: -15_000,
        temperature_c_x100: 2350,
        state_of_charge_pct_x10: 945,
        cycles_completed: 120,
        estimated_runtime_seconds: 1_800,
        last_test_epoch_s: 1_709_000_000,
        replace_by_epoch_s: 1_798_000_000,
    };
    let encoded = encode_with_checksum(&battery).expect("encode UPS battery health");
    let (decoded, _): (UpsBatteryHealth, _) =
        decode_with_checksum(&encoded).expect("decode UPS battery health");
    assert_eq!(battery, decoded);
}

/// Test 6: PDU power distribution with multiple outlet states
#[test]
fn test_pdu_power_distribution_roundtrip() {
    let pdu = PduPowerDistribution {
        pdu_id: "PDU-A05-L".into(),
        feed_label: "Feed-A".into(),
        total_capacity_amps: 60,
        total_current_draw_ma: 32_400,
        voltage_mv: 208_000,
        phase_count: 3,
        outlets: vec![
            (
                1,
                PduOutletState::On {
                    current_ma: 5_200,
                    voltage_mv: 208_000,
                },
            ),
            (
                2,
                PduOutletState::On {
                    current_ma: 4_800,
                    voltage_mv: 208_000,
                },
            ),
            (3, PduOutletState::Off),
            (4, PduOutletState::Fault { error_code: 0x0012 }),
            (5, PduOutletState::Locked),
            (
                6,
                PduOutletState::On {
                    current_ma: 7_100,
                    voltage_mv: 208_000,
                },
            ),
        ],
        breaker_tripped: false,
    };
    let encoded = encode_with_checksum(&pdu).expect("encode PDU distribution");
    let (decoded, _): (PduPowerDistribution, _) =
        decode_with_checksum(&encoded).expect("decode PDU distribution");
    assert_eq!(pdu, decoded);
}

/// Test 7: network switch port config — active port
#[test]
fn test_network_switch_port_config_roundtrip() {
    let port = NetworkSwitchPortConfig {
        switch_id: "TOR-A05-01".into(),
        port_index: 17,
        label: "Eth1/17".into(),
        status: PortStatus::Up {
            speed: PortSpeed::Speed25G,
            vlan_id: 100,
        },
        mac_address: vec![0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E],
        tx_bytes: 8_421_000_000_000,
        rx_bytes: 7_998_000_000_000,
        tx_errors: 3,
        rx_errors: 12,
    };
    let encoded = encode_with_checksum(&port).expect("encode switch port config");
    let (decoded, _): (NetworkSwitchPortConfig, _) =
        decode_with_checksum(&encoded).expect("decode switch port config");
    assert_eq!(port, decoded);
}

/// Test 8: cable management record — fiber optic
#[test]
fn test_cable_management_record_roundtrip() {
    let cable = CableManagementRecord {
        cable_id: "FIB-2024-00847".into(),
        cable_type: CableType::Om4Fiber,
        length_cm: 1_500,
        source_rack: "DC1-R01-A05".into(),
        source_port: "TOR-01/Eth1/49".into(),
        dest_rack: "DC1-R03-A05".into(),
        dest_port: "SPINE-01/Eth3/1".into(),
        installed_epoch_s: 1_700_000_000,
        label_color: "yellow".into(),
    };
    let encoded = encode_with_checksum(&cable).expect("encode cable record");
    let (decoded, _): (CableManagementRecord, _) =
        decode_with_checksum(&encoded).expect("decode cable record");
    assert_eq!(cable, decoded);
}

/// Test 9: PUE calculation
#[test]
fn test_pue_calculation_roundtrip() {
    let pue = PueCalculation {
        facility_id: "EU-WEST-1".into(),
        measurement_epoch_s: 1_710_000_000,
        total_facility_power_watts: 8_500_000,
        it_equipment_power_watts: 6_200_000,
        cooling_power_watts: 1_800_000,
        lighting_power_watts: 50_000,
        pue_x1000: 1_371,
        target_pue_x1000: 1_300,
        measurement_interval_seconds: 900,
    };
    let encoded = encode_with_checksum(&pue).expect("encode PUE calculation");
    let (decoded, _): (PueCalculation, _) =
        decode_with_checksum(&encoded).expect("decode PUE calculation");
    assert_eq!(pue, decoded);
}

/// Test 10: capacity planning forecast
#[test]
fn test_capacity_planning_forecast_roundtrip() {
    let forecast = CapacityPlanningForecast {
        forecast_id: "FY26-Q1-DH-A".into(),
        data_hall_id: "DH-A".into(),
        current_rack_count: 280,
        max_rack_count: 400,
        current_power_kw: 2_800,
        max_power_kw: 4_000,
        current_cooling_kw: 3_200,
        max_cooling_kw: 4_500,
        projected_exhaustion_epoch_s: 1_798_000_000,
        monthly_growth_rate_pct_x100: 250,
        planned_expansions: vec!["DH-A Phase 2 (Q3 FY26)".into(), "DH-D Build (FY27)".into()],
    };
    let encoded = encode_with_checksum(&forecast).expect("encode capacity forecast");
    let (decoded, _): (CapacityPlanningForecast, _) =
        decode_with_checksum(&encoded).expect("decode capacity forecast");
    assert_eq!(forecast, decoded);
}

/// Test 11: hardware lifecycle — procurement phase
#[test]
fn test_hardware_lifecycle_procurement_roundtrip() {
    let record = HardwareLifecycleRecord {
        asset_id: "HW-2025-09821".into(),
        serial_number: "SN-X42-K9801".into(),
        model: "PowerEdge R760".into(),
        manufacturer: "Dell".into(),
        purchase_date_epoch_s: 1_706_000_000,
        warranty_expiry_epoch_s: 1_800_000_000,
        phase: AssetLifecyclePhase::Procurement {
            po_number: "PO-2025-4421".into(),
            vendor: "Dell Technologies".into(),
            cost_cents: 1_250_000,
        },
    };
    let encoded = encode_with_checksum(&record).expect("encode lifecycle procurement");
    let (decoded, _): (HardwareLifecycleRecord, _) =
        decode_with_checksum(&encoded).expect("decode lifecycle procurement");
    assert_eq!(record, decoded);
}

/// Test 12: hardware lifecycle — decommissioned phase
#[test]
fn test_hardware_lifecycle_decommission_roundtrip() {
    let record = HardwareLifecycleRecord {
        asset_id: "HW-2019-01234".into(),
        serial_number: "SN-OLD-5671".into(),
        model: "ProLiant DL380 Gen9".into(),
        manufacturer: "HPE".into(),
        purchase_date_epoch_s: 1_546_300_000,
        warranty_expiry_epoch_s: 1_640_000_000,
        phase: AssetLifecyclePhase::Decommissioned {
            disposal_method: "ITAD certified destruction".into(),
            date_epoch_s: 1_709_500_000,
        },
    };
    let encoded = encode_with_checksum(&record).expect("encode lifecycle decommission");
    let (decoded, _): (HardwareLifecycleRecord, _) =
        decode_with_checksum(&encoded).expect("decode lifecycle decommission");
    assert_eq!(record, decoded);
}

/// Test 13: fire suppression system status
#[test]
fn test_fire_suppression_status_roundtrip() {
    let status = FireSuppressionStatus {
        zone_id: "DH-A-ZONE-1".into(),
        agent: SuppressionAgent::Novec1230,
        cylinder_pressure_psi_x10: 3_600,
        smoke_detectors_ok: 24,
        smoke_detectors_fault: 0,
        vesda_air_sampling_active: true,
        last_inspection_epoch_s: 1_707_000_000,
        system_armed: true,
        discharge_count: 0,
    };
    let encoded = encode_with_checksum(&status).expect("encode fire suppression");
    let (decoded, _): (FireSuppressionStatus, _) =
        decode_with_checksum(&encoded).expect("decode fire suppression");
    assert_eq!(status, decoded);
}

/// Test 14: physical security access log — badge in
#[test]
fn test_physical_security_access_log_roundtrip() {
    let log = PhysicalSecurityAccessLog {
        event_id: 9_000_001,
        timestamp_epoch_s: 1_710_050_000,
        person_id: "EMP-44210".into(),
        door_id: "DH-A-MAIN-ENTRY".into(),
        zone: "Data Hall A".into(),
        event_kind: AccessEventKind::BiometricVerified,
        granted: true,
    };
    let encoded = encode_with_checksum(&log).expect("encode access log");
    let (decoded, _): (PhysicalSecurityAccessLog, _) =
        decode_with_checksum(&encoded).expect("decode access log");
    assert_eq!(log, decoded);
}

/// Test 15: physical security — visitor escorted entry
#[test]
fn test_security_visitor_escort_roundtrip() {
    let log = PhysicalSecurityAccessLog {
        event_id: 9_000_042,
        timestamp_epoch_s: 1_710_060_000,
        person_id: "EMP-11005".into(),
        door_id: "DH-B-CAGE-3".into(),
        zone: "Cage 3".into(),
        event_kind: AccessEventKind::VisitorEscorted {
            visitor_name: "Jane Auditor".into(),
        },
        granted: true,
    };
    let encoded = encode_with_checksum(&log).expect("encode visitor escort log");
    let (decoded, _): (PhysicalSecurityAccessLog, _) =
        decode_with_checksum(&encoded).expect("decode visitor escort log");
    assert_eq!(log, decoded);
}

/// Test 16: environmental sensor reading
#[test]
fn test_environmental_sensor_reading_roundtrip() {
    let reading = EnvironmentalSensorReading {
        sensor_id: "ENV-DH-A-ROW3-TOP".into(),
        location: "Row 3, Top of Rack".into(),
        temperature_c_x100: 3100,
        humidity_pct_x100: 4800,
        differential_pressure_pa_x10: 25,
        water_leak_detected: false,
        particulate_count: 1_200,
        timestamp_epoch_s: 1_710_070_000,
    };
    let encoded = encode_with_checksum(&reading).expect("encode env sensor");
    let (decoded, _): (EnvironmentalSensorReading, _) =
        decode_with_checksum(&encoded).expect("decode env sensor");
    assert_eq!(reading, decoded);
}

/// Test 17: SLA compliance metric — compliant
#[test]
fn test_sla_compliance_metric_roundtrip() {
    let sla = SlaComplianceMetric {
        service_id: "SVC-COLO-PREMIUM-001".into(),
        period_start_epoch_s: 1_706_745_600,
        period_end_epoch_s: 1_709_424_000,
        uptime_pct_x10000: 9_999,
        target_uptime_pct_x10000: 9_995,
        incidents_count: 1,
        mttr_seconds: 240,
        mtbf_seconds: 2_592_000,
        penalty_credits_cents: 0,
        compliant: true,
    };
    let encoded = encode_with_checksum(&sla).expect("encode SLA metric");
    let (decoded, _): (SlaComplianceMetric, _) =
        decode_with_checksum(&encoded).expect("decode SLA metric");
    assert_eq!(sla, decoded);
}

/// Test 18: disaster recovery failover test — successful
#[test]
fn test_dr_failover_success_roundtrip() {
    let dr_test = DisasterRecoveryFailoverTest {
        test_id: "DR-2025-Q1-001".into(),
        primary_site: "EU-WEST-1".into(),
        dr_site: "EU-WEST-2".into(),
        test_epoch_s: 1_709_800_000,
        rpo_seconds: 300,
        rto_seconds: 3_600,
        actual_recovery_seconds: 2_140,
        data_loss_bytes: 0,
        result: FailoverResult::Success {
            switchover_ms: 2_140_000,
        },
        services_tested: vec![
            "DNS".into(),
            "LoadBalancer".into(),
            "AppTier".into(),
            "Database".into(),
            "ObjectStorage".into(),
        ],
    };
    let encoded = encode_with_checksum(&dr_test).expect("encode DR failover test");
    let (decoded, _): (DisasterRecoveryFailoverTest, _) =
        decode_with_checksum(&encoded).expect("decode DR failover test");
    assert_eq!(dr_test, decoded);
}

/// Test 19: DR failover — partial failure
#[test]
fn test_dr_failover_partial_failure_roundtrip() {
    let dr_test = DisasterRecoveryFailoverTest {
        test_id: "DR-2025-Q1-002".into(),
        primary_site: "US-EAST-1".into(),
        dr_site: "US-EAST-2".into(),
        test_epoch_s: 1_709_900_000,
        rpo_seconds: 60,
        rto_seconds: 1_800,
        actual_recovery_seconds: 4_200,
        data_loss_bytes: 1_048_576,
        result: FailoverResult::PartialFailure {
            components_failed: vec![
                "Redis cluster replication lag".into(),
                "Kafka topic partition reassignment timeout".into(),
            ],
        },
        services_tested: vec!["CacheLayer".into(), "MessageQueue".into(), "AppTier".into()],
    };
    let encoded = encode_with_checksum(&dr_test).expect("encode DR partial failure");
    let (decoded, _): (DisasterRecoveryFailoverTest, _) =
        decode_with_checksum(&encoded).expect("decode DR partial failure");
    assert_eq!(dr_test, decoded);
}

/// Test 20: corruption detection — flip byte in rack layout payload
#[test]
fn test_corruption_detection_rack_layout() {
    let rack = ServerRackLayout {
        rack_id: "DC2-R10-B07".into(),
        location_row: "B".into(),
        location_col: 7,
        total_u: 48,
        max_power_kw: 25,
        current_draw_watts: 18_000,
        weight_kg: 950,
        units: vec![
            (
                1,
                RackUnit::Server {
                    asset_tag: "SRV-99001".into(),
                    height_u: 4,
                },
            ),
            (
                5,
                RackUnit::Server {
                    asset_tag: "SRV-99002".into(),
                    height_u: 4,
                },
            ),
        ],
    };
    let mut encoded = encode_with_checksum(&rack).expect("encode for corruption test");
    let mid = encoded.len() / 2;
    encoded[mid] ^= 0xFF;
    let result: Result<(ServerRackLayout, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "corrupted rack layout must fail checksum validation"
    );
}

/// Test 21: corruption detection — truncated UPS battery record
#[test]
fn test_corruption_detection_truncated_ups_battery() {
    let battery = UpsBatteryHealth {
        ups_id: "UPS-B-03".into(),
        battery_string_id: 2,
        chemistry: BatteryChemistry::LithiumIronPhosphate,
        voltage_mv: 768_000,
        current_ma: -8_000,
        temperature_c_x100: 2800,
        state_of_charge_pct_x10: 870,
        cycles_completed: 55,
        estimated_runtime_seconds: 2_400,
        last_test_epoch_s: 1_708_500_000,
        replace_by_epoch_s: 1_830_000_000,
    };
    let encoded = encode_with_checksum(&battery).expect("encode for truncation test");
    let truncated = &encoded[..encoded.len().saturating_sub(6)];
    let result: Result<(UpsBatteryHealth, usize), _> = decode_with_checksum(truncated);
    assert!(
        result.is_err(),
        "truncated UPS battery record must fail checksum validation"
    );
}

/// Test 22: corruption detection — zeroed checksum bytes in PUE record
#[test]
fn test_corruption_detection_zeroed_checksum_pue() {
    let pue = PueCalculation {
        facility_id: "US-EAST-1".into(),
        measurement_epoch_s: 1_710_100_000,
        total_facility_power_watts: 12_000_000,
        it_equipment_power_watts: 9_000_000,
        cooling_power_watts: 2_400_000,
        lighting_power_watts: 80_000,
        pue_x1000: 1_333,
        target_pue_x1000: 1_250,
        measurement_interval_seconds: 300,
    };
    let mut encoded = encode_with_checksum(&pue).expect("encode for zeroed checksum test");
    // Zero out the CRC32 bytes (offset 12..16 in the header)
    encoded[12] = 0x00;
    encoded[13] = 0x00;
    encoded[14] = 0x00;
    encoded[15] = 0x00;
    let result: Result<(PueCalculation, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "zeroed checksum in PUE record must fail validation"
    );
}
