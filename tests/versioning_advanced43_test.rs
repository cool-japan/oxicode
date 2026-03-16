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
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types: Electric Vehicles & Charging Infrastructure ────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CellChemistry {
    Nmc811,
    Lfp,
    Nca,
    LithiumTitanate,
    SolidState,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChargerType {
    Level1Ac,
    Level2Ac,
    DcFastCcs,
    DcFastChademo,
    V2GBidirectional,
    MegachargerHpc,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChargingSessionStatus {
    Initiated,
    Authorized,
    Charging,
    SuspendedEv,
    SuspendedEvse,
    Finishing,
    Completed,
    Faulted,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThermalMode {
    Idle,
    Heating,
    Cooling,
    Preconditioning,
    EmergencyCooldown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OtaUpdateState {
    Available,
    Downloading,
    ReadyToInstall,
    Installing,
    Verifying,
    Applied,
    RolledBack,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IsolationStatus {
    Normal,
    MonitoringDegraded,
    ContactorOpen,
    EmergencyDisconnect,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryManagementSystem {
    pack_id: u64,
    cell_count: u16,
    cell_voltage_min_mv: u16,
    cell_voltage_max_mv: u16,
    cell_voltage_avg_mv: u16,
    pack_voltage_mv: u32,
    pack_current_ma: i32,
    soc_permille: u16,
    soh_permille: u16,
    temperature_min_decideg: i16,
    temperature_max_decideg: i16,
    chemistry: CellChemistry,
    cycle_count: u32,
    capacity_mah: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChargingSessionRecord {
    session_id: u64,
    connector_id: u32,
    vehicle_id: String,
    charger_type: ChargerType,
    status: ChargingSessionStatus,
    energy_delivered_wh: u64,
    max_power_w: u32,
    start_timestamp: u64,
    stop_timestamp: u64,
    meter_start_wh: u64,
    meter_stop_wh: u64,
    transaction_id: String,
    tariff_cents_per_kwh: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VehicleTelematics {
    vin: String,
    odometer_m: u64,
    estimated_range_m: u32,
    energy_consumption_wh_per_km: u16,
    speed_kmh: u16,
    ambient_temp_decideg: i16,
    cabin_temp_decideg: i16,
    latitude_microdeg: i64,
    longitude_microdeg: i64,
    heading_decideg: u16,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MotorControllerParams {
    controller_id: u32,
    max_torque_nm_x10: u32,
    max_rpm: u32,
    phase_current_limit_ma: u32,
    dc_bus_voltage_mv: u32,
    pwm_frequency_hz: u32,
    inverter_temp_decideg: i16,
    motor_temp_decideg: i16,
    efficiency_permille: u16,
    regen_torque_limit_nm_x10: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RegenBrakingProfile {
    profile_id: u32,
    vehicle_model: String,
    max_regen_power_w: u32,
    min_speed_for_regen_kmh: u8,
    coast_decel_mg: u16,
    low_regen_decel_mg: u16,
    high_regen_decel_mg: u16,
    blended_brake_threshold_mg: u16,
    soc_cutoff_permille: u16,
    temp_derate_start_decideg: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThermalManagementSystem {
    system_id: u32,
    mode: ThermalMode,
    coolant_inlet_temp_decideg: i16,
    coolant_outlet_temp_decideg: i16,
    coolant_flow_ml_per_min: u32,
    compressor_power_w: u16,
    ptc_heater_power_w: u16,
    battery_target_temp_decideg: i16,
    cabin_target_temp_decideg: i16,
    heat_pump_cop_x100: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChargingStationConfig {
    station_id: u64,
    operator: String,
    location_name: String,
    latitude_microdeg: i64,
    longitude_microdeg: i64,
    connector_count: u8,
    charger_type: ChargerType,
    max_power_kw: u32,
    grid_connection_kva: u32,
    has_battery_buffer: bool,
    v2g_capable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FleetVehicleStatus {
    fleet_id: u32,
    vehicle_id: String,
    soc_permille: u16,
    range_remaining_m: u32,
    is_charging: bool,
    is_in_service: bool,
    daily_energy_wh: u64,
    daily_distance_m: u64,
    driver_id: Option<String>,
    next_maintenance_km: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RoutePlanChargingStop {
    stop_index: u8,
    station_id: u64,
    station_name: String,
    arrival_soc_permille: u16,
    departure_soc_permille: u16,
    charge_duration_s: u32,
    distance_from_prev_m: u32,
    energy_needed_wh: u32,
    charger_type: ChargerType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryDegradationModel {
    model_id: u32,
    chemistry: CellChemistry,
    calendar_aging_rate_ppm_per_day: u32,
    cycle_aging_rate_ppm_per_cycle: u32,
    temp_acceleration_factor_x100: u16,
    depth_of_discharge_factor_x100: u16,
    soh_at_eol_permille: u16,
    expected_cycle_life: u32,
    warranty_years: u8,
    warranty_km: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerElectronicsEfficiency {
    component_id: u32,
    component_name: String,
    dc_dc_efficiency_permille: u16,
    inverter_efficiency_permille: u16,
    onboard_charger_efficiency_permille: u16,
    total_drivetrain_efficiency_permille: u16,
    standby_power_w: u16,
    peak_efficiency_load_percent: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OtaUpdateManifest {
    manifest_id: u64,
    vehicle_model: String,
    firmware_version: String,
    target_ecu: String,
    state: OtaUpdateState,
    payload_size_bytes: u64,
    sha256_hex: String,
    release_timestamp: u64,
    rollback_version: String,
    requires_standstill: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrashSafetyBatteryIsolation {
    event_id: u64,
    vehicle_id: String,
    isolation_status: IsolationStatus,
    impact_g_x10: u16,
    contactor_open_latency_us: u32,
    pack_voltage_after_mv: u32,
    insulation_resistance_kohm: u32,
    coolant_leak_detected: bool,
    thermal_runaway_risk: bool,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridIntegrationSchedule {
    schedule_id: u64,
    station_id: u64,
    start_hour: u8,
    end_hour: u8,
    max_import_kw: u32,
    max_export_kw: u32,
    energy_price_cents_per_kwh: u32,
    demand_response_enrolled: bool,
    frequency_regulation: bool,
    target_soc_permille: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WarrantyClaimRecord {
    claim_id: u64,
    vin: String,
    component: String,
    failure_description: String,
    odometer_at_failure_km: u32,
    soh_at_failure_permille: u16,
    cycle_count_at_failure: u32,
    claim_amount_cents: u64,
    approved: bool,
    replacement_part_id: Option<String>,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_bms_nmc_pack_roundtrip() {
    let bms = BatteryManagementSystem {
        pack_id: 100_001,
        cell_count: 96,
        cell_voltage_min_mv: 3620,
        cell_voltage_max_mv: 3685,
        cell_voltage_avg_mv: 3652,
        pack_voltage_mv: 350_592,
        pack_current_ma: -45_000,
        soc_permille: 723,
        soh_permille: 961,
        temperature_min_decideg: 278,
        temperature_max_decideg: 315,
        chemistry: CellChemistry::Nmc811,
        cycle_count: 412,
        capacity_mah: 78_000,
    };
    let bytes = encode_to_vec(&bms).expect("encode BMS NMC pack");
    let (decoded, consumed): (BatteryManagementSystem, usize) =
        decode_from_slice(&bytes).expect("decode BMS NMC pack");
    assert_eq!(bms, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_bms_versioned_v1_0_0() {
    let bms = BatteryManagementSystem {
        pack_id: 200_002,
        cell_count: 108,
        cell_voltage_min_mv: 3280,
        cell_voltage_max_mv: 3320,
        cell_voltage_avg_mv: 3300,
        pack_voltage_mv: 356_400,
        pack_current_ma: 120_000,
        soc_permille: 340,
        soh_permille: 985,
        temperature_min_decideg: 252,
        temperature_max_decideg: 290,
        chemistry: CellChemistry::Lfp,
        cycle_count: 1_850,
        capacity_mah: 105_000,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&bms, version).expect("encode versioned BMS v1.0.0");
    let (decoded, ver, _consumed): (BatteryManagementSystem, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned BMS v1.0.0");
    assert_eq!(bms, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_charging_session_dc_fast_roundtrip() {
    let session = ChargingSessionRecord {
        session_id: 5_000_001,
        connector_id: 3,
        vehicle_id: "WBA12345678901234".to_string(),
        charger_type: ChargerType::DcFastCcs,
        status: ChargingSessionStatus::Completed,
        energy_delivered_wh: 52_340,
        max_power_w: 150_000,
        start_timestamp: 1_710_500_000,
        stop_timestamp: 1_710_501_800,
        meter_start_wh: 1_234_000,
        meter_stop_wh: 1_286_340,
        transaction_id: "TX-2026-0315-0001".to_string(),
        tariff_cents_per_kwh: 35,
    };
    let bytes = encode_to_vec(&session).expect("encode DC fast session");
    let (decoded, consumed): (ChargingSessionRecord, usize) =
        decode_from_slice(&bytes).expect("decode DC fast session");
    assert_eq!(session, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_charging_session_versioned_v2_1_0() {
    let session = ChargingSessionRecord {
        session_id: 5_000_002,
        connector_id: 1,
        vehicle_id: "5YJ3E1EA7PF000042".to_string(),
        charger_type: ChargerType::Level2Ac,
        status: ChargingSessionStatus::Charging,
        energy_delivered_wh: 11_200,
        max_power_w: 11_520,
        start_timestamp: 1_710_600_000,
        stop_timestamp: 0,
        meter_start_wh: 987_000,
        meter_stop_wh: 987_000,
        transaction_id: "TX-2026-0315-0002".to_string(),
        tariff_cents_per_kwh: 18,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&session, version).expect("encode versioned session v2.1.0");
    let (decoded, ver, consumed): (ChargingSessionRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned session v2.1.0");
    assert_eq!(session, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_vehicle_telematics_highway_roundtrip() {
    let telemetry = VehicleTelematics {
        vin: "WVWZZZ3CZWE000001".to_string(),
        odometer_m: 45_320_000,
        estimated_range_m: 287_000,
        energy_consumption_wh_per_km: 178,
        speed_kmh: 118,
        ambient_temp_decideg: 225,
        cabin_temp_decideg: 210,
        latitude_microdeg: 48_856_614,
        longitude_microdeg: 2_352_222,
        heading_decideg: 2700,
        timestamp: 1_710_700_000,
    };
    let bytes = encode_to_vec(&telemetry).expect("encode telematics highway");
    let (decoded, consumed): (VehicleTelematics, usize) =
        decode_from_slice(&bytes).expect("decode telematics highway");
    assert_eq!(telemetry, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_vehicle_telematics_versioned_v1_3_0() {
    let telemetry = VehicleTelematics {
        vin: "KNAB3512ALT000099".to_string(),
        odometer_m: 12_100_000,
        estimated_range_m: 410_000,
        energy_consumption_wh_per_km: 142,
        speed_kmh: 0,
        ambient_temp_decideg: -50,
        cabin_temp_decideg: 200,
        latitude_microdeg: 37_566_535,
        longitude_microdeg: 126_977_969,
        heading_decideg: 0,
        timestamp: 1_710_800_000,
    };
    let version = Version::new(1, 3, 0);
    let bytes =
        encode_versioned_value(&telemetry, version).expect("encode versioned telematics v1.3.0");
    let (decoded, ver, _consumed): (VehicleTelematics, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned telematics v1.3.0");
    assert_eq!(telemetry, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_motor_controller_high_torque_roundtrip() {
    let mc = MotorControllerParams {
        controller_id: 7001,
        max_torque_nm_x10: 6_600,
        max_rpm: 17_000,
        phase_current_limit_ma: 800_000,
        dc_bus_voltage_mv: 800_000,
        pwm_frequency_hz: 10_000,
        inverter_temp_decideg: 452,
        motor_temp_decideg: 1050,
        efficiency_permille: 965,
        regen_torque_limit_nm_x10: 3_500,
    };
    let bytes = encode_to_vec(&mc).expect("encode motor controller");
    let (decoded, consumed): (MotorControllerParams, usize) =
        decode_from_slice(&bytes).expect("decode motor controller");
    assert_eq!(mc, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_regen_braking_profile_versioned_v3_0_0() {
    let profile = RegenBrakingProfile {
        profile_id: 42,
        vehicle_model: "EV-Sedan-2026".to_string(),
        max_regen_power_w: 70_000,
        min_speed_for_regen_kmh: 5,
        coast_decel_mg: 30,
        low_regen_decel_mg: 80,
        high_regen_decel_mg: 250,
        blended_brake_threshold_mg: 400,
        soc_cutoff_permille: 950,
        temp_derate_start_decideg: -100,
    };
    let version = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&profile, version).expect("encode versioned regen v3.0.0");
    let (decoded, ver, consumed): (RegenBrakingProfile, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned regen v3.0.0");
    assert_eq!(profile, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_thermal_management_cooling_roundtrip() {
    let tms = ThermalManagementSystem {
        system_id: 801,
        mode: ThermalMode::Cooling,
        coolant_inlet_temp_decideg: 380,
        coolant_outlet_temp_decideg: 345,
        coolant_flow_ml_per_min: 12_000,
        compressor_power_w: 2_500,
        ptc_heater_power_w: 0,
        battery_target_temp_decideg: 300,
        cabin_target_temp_decideg: 220,
        heat_pump_cop_x100: 320,
    };
    let bytes = encode_to_vec(&tms).expect("encode TMS cooling");
    let (decoded, consumed): (ThermalManagementSystem, usize) =
        decode_from_slice(&bytes).expect("decode TMS cooling");
    assert_eq!(tms, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_thermal_management_preconditioning_versioned_v1_2_0() {
    let tms = ThermalManagementSystem {
        system_id: 802,
        mode: ThermalMode::Preconditioning,
        coolant_inlet_temp_decideg: -120,
        coolant_outlet_temp_decideg: -80,
        coolant_flow_ml_per_min: 8_000,
        compressor_power_w: 0,
        ptc_heater_power_w: 5_000,
        battery_target_temp_decideg: 250,
        cabin_target_temp_decideg: 200,
        heat_pump_cop_x100: 280,
    };
    let version = Version::new(1, 2, 0);
    let bytes = encode_versioned_value(&tms, version).expect("encode versioned TMS v1.2.0");
    let (decoded, ver, _consumed): (ThermalManagementSystem, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned TMS v1.2.0");
    assert_eq!(tms, decoded);
    assert_eq!(ver.minor, 2);
}

#[test]
fn test_charging_station_v2g_config_roundtrip() {
    let station = ChargingStationConfig {
        station_id: 900_100,
        operator: "GridCharge Inc.".to_string(),
        location_name: "Shibuya Hikarie Parking B2".to_string(),
        latitude_microdeg: 35_659_097,
        longitude_microdeg: 139_703_610,
        connector_count: 4,
        charger_type: ChargerType::V2GBidirectional,
        max_power_kw: 22,
        grid_connection_kva: 200,
        has_battery_buffer: true,
        v2g_capable: true,
    };
    let bytes = encode_to_vec(&station).expect("encode V2G station config");
    let (decoded, consumed): (ChargingStationConfig, usize) =
        decode_from_slice(&bytes).expect("decode V2G station config");
    assert_eq!(station, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_charging_station_megacharger_versioned_v4_0_0() {
    let station = ChargingStationConfig {
        station_id: 900_200,
        operator: "HyperCharge EU".to_string(),
        location_name: "Autobahn A9 Rastplatz Nord".to_string(),
        latitude_microdeg: 48_774_900,
        longitude_microdeg: 11_431_230,
        connector_count: 12,
        charger_type: ChargerType::MegachargerHpc,
        max_power_kw: 350,
        grid_connection_kva: 5_000,
        has_battery_buffer: true,
        v2g_capable: false,
    };
    let version = Version::new(4, 0, 0);
    let bytes =
        encode_versioned_value(&station, version).expect("encode versioned megacharger v4.0.0");
    let (decoded, ver, consumed): (ChargingStationConfig, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned megacharger v4.0.0");
    assert_eq!(station, decoded);
    assert_eq!(ver.major, 4);
    assert!(consumed > 0);
}

#[test]
fn test_fleet_vehicle_status_active_driver_roundtrip() {
    let status = FleetVehicleStatus {
        fleet_id: 55,
        vehicle_id: "FLEET-JP-0042".to_string(),
        soc_permille: 650,
        range_remaining_m: 195_000,
        is_charging: false,
        is_in_service: true,
        daily_energy_wh: 34_200,
        daily_distance_m: 187_000,
        driver_id: Some("DRV-1001".to_string()),
        next_maintenance_km: 8_500,
    };
    let bytes = encode_to_vec(&status).expect("encode fleet status active");
    let (decoded, consumed): (FleetVehicleStatus, usize) =
        decode_from_slice(&bytes).expect("decode fleet status active");
    assert_eq!(status, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_fleet_vehicle_no_driver_versioned_v2_0_1() {
    let status = FleetVehicleStatus {
        fleet_id: 55,
        vehicle_id: "FLEET-JP-0099".to_string(),
        soc_permille: 980,
        range_remaining_m: 320_000,
        is_charging: true,
        is_in_service: false,
        daily_energy_wh: 0,
        daily_distance_m: 0,
        driver_id: None,
        next_maintenance_km: 22_000,
    };
    let version = Version::new(2, 0, 1);
    let bytes = encode_versioned_value(&status, version).expect("encode versioned fleet v2.0.1");
    let (decoded, ver, _consumed): (FleetVehicleStatus, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned fleet v2.0.1");
    assert_eq!(status, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.patch, 1);
}

#[test]
fn test_route_plan_charging_stop_roundtrip() {
    let stop = RoutePlanChargingStop {
        stop_index: 2,
        station_id: 900_100,
        station_name: "Hamamatsu SA (Tomei)".to_string(),
        arrival_soc_permille: 150,
        departure_soc_permille: 800,
        charge_duration_s: 1_800,
        distance_from_prev_m: 210_000,
        energy_needed_wh: 38_500,
        charger_type: ChargerType::DcFastCcs,
    };
    let bytes = encode_to_vec(&stop).expect("encode route plan stop");
    let (decoded, consumed): (RoutePlanChargingStop, usize) =
        decode_from_slice(&bytes).expect("decode route plan stop");
    assert_eq!(stop, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_battery_degradation_model_lfp_versioned_v1_1_0() {
    let model = BatteryDegradationModel {
        model_id: 301,
        chemistry: CellChemistry::Lfp,
        calendar_aging_rate_ppm_per_day: 8,
        cycle_aging_rate_ppm_per_cycle: 35,
        temp_acceleration_factor_x100: 120,
        depth_of_discharge_factor_x100: 105,
        soh_at_eol_permille: 700,
        expected_cycle_life: 6_000,
        warranty_years: 10,
        warranty_km: 200_000,
    };
    let version = Version::new(1, 1, 0);
    let bytes =
        encode_versioned_value(&model, version).expect("encode versioned degradation v1.1.0");
    let (decoded, ver, consumed): (BatteryDegradationModel, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned degradation v1.1.0");
    assert_eq!(model, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_power_electronics_efficiency_roundtrip() {
    let pe = PowerElectronicsEfficiency {
        component_id: 4001,
        component_name: "Gen3-SiC-Drivetrain".to_string(),
        dc_dc_efficiency_permille: 972,
        inverter_efficiency_permille: 985,
        onboard_charger_efficiency_permille: 940,
        total_drivetrain_efficiency_permille: 910,
        standby_power_w: 45,
        peak_efficiency_load_percent: 65,
    };
    let bytes = encode_to_vec(&pe).expect("encode power electronics");
    let (decoded, consumed): (PowerElectronicsEfficiency, usize) =
        decode_from_slice(&bytes).expect("decode power electronics");
    assert_eq!(pe, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_ota_update_manifest_versioned_v5_2_0() {
    let manifest = OtaUpdateManifest {
        manifest_id: 60_001,
        vehicle_model: "EV-Crossover-2026".to_string(),
        firmware_version: "2026.12.1-rc3".to_string(),
        target_ecu: "BMS-Main".to_string(),
        state: OtaUpdateState::ReadyToInstall,
        payload_size_bytes: 67_108_864,
        sha256_hex: "a3f5b7c9e1d2f4a6b8c0d2e4f6a8b0c2d4e6f8a0b2c4d6e8f0a2b4c6d8e0f2".to_string(),
        release_timestamp: 1_710_900_000,
        rollback_version: "2026.11.2".to_string(),
        requires_standstill: true,
    };
    let version = Version::new(5, 2, 0);
    let bytes = encode_versioned_value(&manifest, version).expect("encode versioned OTA v5.2.0");
    let (decoded, ver, consumed): (OtaUpdateManifest, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned OTA v5.2.0");
    assert_eq!(manifest, decoded);
    assert_eq!(ver.major, 5);
    assert_eq!(ver.minor, 2);
    assert!(consumed > 0);
}

#[test]
fn test_crash_safety_isolation_emergency_roundtrip() {
    let event = CrashSafetyBatteryIsolation {
        event_id: 77_001,
        vehicle_id: "CRASH-TEST-VIN-001".to_string(),
        isolation_status: IsolationStatus::EmergencyDisconnect,
        impact_g_x10: 350,
        contactor_open_latency_us: 850,
        pack_voltage_after_mv: 12,
        insulation_resistance_kohm: 45,
        coolant_leak_detected: true,
        thermal_runaway_risk: true,
        timestamp: 1_711_000_000,
    };
    let bytes = encode_to_vec(&event).expect("encode crash isolation");
    let (decoded, consumed): (CrashSafetyBatteryIsolation, usize) =
        decode_from_slice(&bytes).expect("decode crash isolation");
    assert_eq!(event, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_grid_integration_off_peak_versioned_v1_0_2() {
    let schedule = GridIntegrationSchedule {
        schedule_id: 88_001,
        station_id: 900_100,
        start_hour: 23,
        end_hour: 6,
        max_import_kw: 50,
        max_export_kw: 30,
        energy_price_cents_per_kwh: 8,
        demand_response_enrolled: true,
        frequency_regulation: true,
        target_soc_permille: 800,
    };
    let version = Version::new(1, 0, 2);
    let bytes = encode_versioned_value(&schedule, version).expect("encode versioned grid v1.0.2");
    let (decoded, ver, consumed): (GridIntegrationSchedule, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned grid v1.0.2");
    assert_eq!(schedule, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.patch, 2);
    assert!(consumed > 0);
}

#[test]
fn test_warranty_claim_approved_with_replacement_roundtrip() {
    let claim = WarrantyClaimRecord {
        claim_id: 99_001,
        vin: "JN1TANT31Z0000042".to_string(),
        component: "Battery Module #7".to_string(),
        failure_description: "Cell group 7B capacity below 65% SoH at 78k km within 8yr warranty"
            .to_string(),
        odometer_at_failure_km: 78_000,
        soh_at_failure_permille: 648,
        cycle_count_at_failure: 1_920,
        claim_amount_cents: 850_000,
        approved: true,
        replacement_part_id: Some("MOD-NMC811-96S-R2".to_string()),
    };
    let bytes = encode_to_vec(&claim).expect("encode warranty claim approved");
    let (decoded, consumed): (WarrantyClaimRecord, usize) =
        decode_from_slice(&bytes).expect("decode warranty claim approved");
    assert_eq!(claim, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_warranty_claim_denied_versioned_v3_1_0() {
    let claim = WarrantyClaimRecord {
        claim_id: 99_002,
        vin: "WDBRF61J21F000007".to_string(),
        component: "Onboard Charger".to_string(),
        failure_description: "Intermittent L2 charging failure after aftermarket EVSE usage"
            .to_string(),
        odometer_at_failure_km: 125_000,
        soh_at_failure_permille: 892,
        cycle_count_at_failure: 980,
        claim_amount_cents: 320_000,
        approved: false,
        replacement_part_id: None,
    };
    let version = Version::new(3, 1, 0);
    let bytes = encode_versioned_value(&claim, version).expect("encode versioned warranty v3.1.0");
    let (decoded, ver, _consumed): (WarrantyClaimRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned warranty v3.1.0");
    assert_eq!(claim, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 1);
}
