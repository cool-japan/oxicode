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
// Domain types: Renewable Energy Grid Management & Storage Systems
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SolarPanelArray {
    array_id: String,
    panel_count: u32,
    tilt_angle_deg: f64,
    azimuth_deg: f64,
    peak_capacity_kw: f64,
    tracking_mode: SolarTrackingMode,
    installation_year: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SolarTrackingMode {
    Fixed,
    SingleAxis,
    DualAxis,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WindTurbineScada {
    turbine_id: String,
    rotor_speed_rpm: f64,
    nacelle_orientation_deg: f64,
    blade_pitch_deg: f64,
    generator_power_kw: f64,
    vibration_mm_s: f64,
    oil_temp_celsius: f64,
    status: TurbineStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TurbineStatus {
    Generating,
    Idling,
    CutOutHighWind,
    Maintenance,
    Fault(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BatteryStorageState {
    unit_id: String,
    chemistry: BatteryChemistry,
    state_of_charge_pct: f64,
    state_of_health_pct: f64,
    voltage_v: f64,
    current_a: f64,
    temperature_celsius: f64,
    cycle_count: u32,
    capacity_kwh: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BatteryChemistry {
    LithiumIronPhosphate,
    LithiumNickelManganese,
    SodiumIon,
    VanadiumRedoxFlow,
    ZincBromine,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GridFrequencyRegulation {
    region_id: String,
    nominal_frequency_hz: f64,
    measured_frequency_hz: f64,
    deviation_mhz: f64,
    regulation_signal: f64,
    reserve_capacity_mw: f64,
    droop_pct: f64,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DemandResponseEvent {
    event_id: u64,
    utility_name: String,
    signal_level: DemandResponseLevel,
    start_epoch_secs: u64,
    duration_minutes: u32,
    target_reduction_kw: f64,
    enrolled_sites: u32,
    actual_reduction_kw: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DemandResponseLevel {
    Normal,
    Moderate,
    High,
    Critical,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SmartInverterSettings {
    inverter_id: String,
    max_power_kw: f64,
    power_factor: f64,
    reactive_power_var: f64,
    voltage_ride_through: bool,
    frequency_ride_through: bool,
    ramp_rate_pct_per_sec: f64,
    islanding_detection: bool,
    firmware_version: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MicrogridIslandingState {
    microgrid_id: String,
    mode: MicrogridMode,
    generation_kw: f64,
    load_kw: f64,
    storage_kw: f64,
    frequency_hz: f64,
    voltage_pu: f64,
    connected_generators: Vec<String>,
    black_start_capable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MicrogridMode {
    GridConnected,
    Islanded,
    Transitioning,
    BlackStart,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EvChargingStationLoad {
    station_id: String,
    connector_type: EvConnectorType,
    max_power_kw: f64,
    current_power_kw: f64,
    session_energy_kwh: f64,
    vehicle_soc_pct: Option<f64>,
    price_per_kwh_cents: u32,
    occupied: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum EvConnectorType {
    Ccs2,
    CHAdeMO,
    Type2AC,
    Nacs,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HydroelectricDamFlow {
    dam_id: String,
    reservoir_level_m: f64,
    inflow_m3_per_s: f64,
    outflow_m3_per_s: f64,
    spillway_open: bool,
    turbine_flows: Vec<f64>,
    generation_mw: f64,
    environmental_flow_m3_per_s: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GeothermalWellData {
    well_id: String,
    depth_m: f64,
    bottom_hole_temp_celsius: f64,
    wellhead_pressure_bar: f64,
    steam_fraction: f64,
    flow_rate_kg_per_s: f64,
    enthalpy_kj_per_kg: f64,
    scaling_risk: ScalingRisk,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ScalingRisk {
    Low,
    Moderate,
    High,
    Inhibited,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EnergyCertificateRecord {
    certificate_id: String,
    source: RenewableSource,
    generation_mwh: f64,
    vintage_year: u16,
    registry: String,
    owner_account: String,
    retired: bool,
    price_cents_per_mwh: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RenewableSource {
    Solar,
    WindOnshore,
    WindOffshore,
    Hydro,
    Geothermal,
    Biomass,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WeatherForecastIntegration {
    location_id: String,
    latitude: f64,
    longitude: f64,
    forecast_horizon_hours: u32,
    global_horizontal_irradiance_wm2: f64,
    wind_speed_10m_ms: f64,
    wind_direction_deg: f64,
    ambient_temp_celsius: f64,
    cloud_cover_pct: f64,
    precipitation_mm: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PowerPurchaseAgreement {
    ppa_id: String,
    buyer: String,
    seller: String,
    contracted_mw: f64,
    strike_price_cents_per_mwh: u64,
    escalation_pct_annual: f64,
    term_years: u16,
    curtailment_allowed: bool,
    balancing_responsible: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TransmissionConstraint {
    line_id: String,
    from_bus: String,
    to_bus: String,
    thermal_limit_mw: f64,
    current_flow_mw: f64,
    congestion_price_cents: u64,
    contingency_rating_mw: f64,
    overloaded: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EnergyStorageDispatch {
    unit_id: String,
    mode: StorageDispatchMode,
    power_kw: f64,
    target_soc_pct: f64,
    market_signal: f64,
    schedule_slots: Vec<ScheduleSlot>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StorageDispatchMode {
    Charging,
    Discharging,
    Standby,
    PeakShaving,
    FrequencyResponse,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ScheduleSlot {
    hour: u8,
    power_kw: f64,
    price_signal: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VirtualPowerPlant {
    vpp_id: String,
    name: String,
    aggregated_capacity_mw: f64,
    asset_count: u32,
    asset_types: Vec<String>,
    dispatch_priority: Vec<String>,
    min_response_time_secs: u32,
    contracted_capacity_mw: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CarbonIntensityReading {
    grid_region: String,
    timestamp_epoch: u64,
    grams_co2_per_kwh: f64,
    marginal_source: String,
    forecast_24h: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProtectionRelayEvent {
    relay_id: String,
    event_type: ProtectionEventType,
    fault_current_a: f64,
    trip_time_ms: f64,
    recloser_attempts: u8,
    zone: u8,
    lockout: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ProtectionEventType {
    Overcurrent,
    OverUnderVoltage,
    OverUnderFrequency,
    DirectionalEarthFault,
    ReverseePower,
    LossOfMains,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_solar_panel_array_dual_axis() {
    let cfg = config::standard();
    let val = SolarPanelArray {
        array_id: "SPV-SITE-042".to_string(),
        panel_count: 480,
        tilt_angle_deg: 25.0,
        azimuth_deg: 180.0,
        peak_capacity_kw: 192.0,
        tracking_mode: SolarTrackingMode::DualAxis,
        installation_year: 2024,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SolarPanelArray");
    let (decoded, _): (SolarPanelArray, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SolarPanelArray");
    assert_eq!(val, decoded);
}

#[test]
fn test_wind_turbine_scada_generating() {
    let cfg = config::standard();
    let val = WindTurbineScada {
        turbine_id: "WT-NORTH-014".to_string(),
        rotor_speed_rpm: 12.5,
        nacelle_orientation_deg: 225.0,
        blade_pitch_deg: 3.2,
        generator_power_kw: 2850.0,
        vibration_mm_s: 1.8,
        oil_temp_celsius: 62.0,
        status: TurbineStatus::Generating,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode WindTurbineScada generating");
    let (decoded, _): (WindTurbineScada, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WindTurbineScada generating");
    assert_eq!(val, decoded);
}

#[test]
fn test_wind_turbine_scada_fault() {
    let cfg = config::standard();
    let val = WindTurbineScada {
        turbine_id: "WT-SOUTH-003".to_string(),
        rotor_speed_rpm: 0.0,
        nacelle_orientation_deg: 90.0,
        blade_pitch_deg: 90.0,
        generator_power_kw: 0.0,
        vibration_mm_s: 8.5,
        oil_temp_celsius: 95.0,
        status: TurbineStatus::Fault("gearbox bearing overheat".to_string()),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode WindTurbineScada fault");
    let (decoded, _): (WindTurbineScada, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WindTurbineScada fault");
    assert_eq!(val, decoded);
}

#[test]
fn test_battery_storage_lfp_charging() {
    let cfg = config::standard();
    let val = BatteryStorageState {
        unit_id: "BESS-A-001".to_string(),
        chemistry: BatteryChemistry::LithiumIronPhosphate,
        state_of_charge_pct: 45.3,
        state_of_health_pct: 97.2,
        voltage_v: 812.0,
        current_a: -125.0,
        temperature_celsius: 28.5,
        cycle_count: 342,
        capacity_kwh: 2000.0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BatteryStorageState LFP");
    let (decoded, _): (BatteryStorageState, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BatteryStorageState LFP");
    assert_eq!(val, decoded);
}

#[test]
fn test_battery_storage_vanadium_flow() {
    let cfg = config::standard();
    let val = BatteryStorageState {
        unit_id: "VRFB-GRID-002".to_string(),
        chemistry: BatteryChemistry::VanadiumRedoxFlow,
        state_of_charge_pct: 78.0,
        state_of_health_pct: 99.5,
        voltage_v: 48.0,
        current_a: 200.0,
        temperature_celsius: 35.0,
        cycle_count: 12500,
        capacity_kwh: 8000.0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BatteryStorageState VRFB");
    let (decoded, _): (BatteryStorageState, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BatteryStorageState VRFB");
    assert_eq!(val, decoded);
}

#[test]
fn test_grid_frequency_regulation_deviation() {
    let cfg = config::standard();
    let val = GridFrequencyRegulation {
        region_id: "ERCOT-NORTH".to_string(),
        nominal_frequency_hz: 60.0,
        measured_frequency_hz: 59.965,
        deviation_mhz: -35.0,
        regulation_signal: 0.72,
        reserve_capacity_mw: 450.0,
        droop_pct: 4.0,
        active: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode GridFrequencyRegulation");
    let (decoded, _): (GridFrequencyRegulation, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GridFrequencyRegulation");
    assert_eq!(val, decoded);
}

#[test]
fn test_demand_response_event_critical() {
    let cfg = config::standard();
    let val = DemandResponseEvent {
        event_id: 99201,
        utility_name: "Pacific Grid Energy".to_string(),
        signal_level: DemandResponseLevel::Critical,
        start_epoch_secs: 1719500400,
        duration_minutes: 120,
        target_reduction_kw: 5000.0,
        enrolled_sites: 1420,
        actual_reduction_kw: Some(4875.5),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DemandResponseEvent critical");
    let (decoded, _): (DemandResponseEvent, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DemandResponseEvent critical");
    assert_eq!(val, decoded);
}

#[test]
fn test_demand_response_event_pending() {
    let cfg = config::standard();
    let val = DemandResponseEvent {
        event_id: 99205,
        utility_name: "Northeast Power Coop".to_string(),
        signal_level: DemandResponseLevel::Moderate,
        start_epoch_secs: 1719590000,
        duration_minutes: 60,
        target_reduction_kw: 1200.0,
        enrolled_sites: 380,
        actual_reduction_kw: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DemandResponseEvent pending");
    let (decoded, _): (DemandResponseEvent, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DemandResponseEvent pending");
    assert_eq!(val, decoded);
}

#[test]
fn test_smart_inverter_settings_full_features() {
    let cfg = config::standard();
    let val = SmartInverterSettings {
        inverter_id: "INV-SMA-50K-012".to_string(),
        max_power_kw: 50.0,
        power_factor: 0.95,
        reactive_power_var: 8500.0,
        voltage_ride_through: true,
        frequency_ride_through: true,
        ramp_rate_pct_per_sec: 10.0,
        islanding_detection: true,
        firmware_version: "3.14.2-rc1".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SmartInverterSettings");
    let (decoded, _): (SmartInverterSettings, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SmartInverterSettings");
    assert_eq!(val, decoded);
}

#[test]
fn test_microgrid_islanded_operation() {
    let cfg = config::standard();
    let val = MicrogridIslandingState {
        microgrid_id: "MG-CAMPUS-01".to_string(),
        mode: MicrogridMode::Islanded,
        generation_kw: 850.0,
        load_kw: 780.0,
        storage_kw: -70.0,
        frequency_hz: 60.02,
        voltage_pu: 1.01,
        connected_generators: vec![
            "SPV-BLDG-A".to_string(),
            "BESS-MG-01".to_string(),
            "GENSET-BACKUP".to_string(),
        ],
        black_start_capable: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode MicrogridIslandingState");
    let (decoded, _): (MicrogridIslandingState, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MicrogridIslandingState");
    assert_eq!(val, decoded);
}

#[test]
fn test_ev_charging_station_fast_dc() {
    let cfg = config::standard();
    let val = EvChargingStationLoad {
        station_id: "EVSE-HWY-101-04".to_string(),
        connector_type: EvConnectorType::Ccs2,
        max_power_kw: 350.0,
        current_power_kw: 285.0,
        session_energy_kwh: 42.5,
        vehicle_soc_pct: Some(68.0),
        price_per_kwh_cents: 45,
        occupied: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode EvChargingStationLoad DC");
    let (decoded, _): (EvChargingStationLoad, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EvChargingStationLoad DC");
    assert_eq!(val, decoded);
}

#[test]
fn test_ev_charging_station_idle() {
    let cfg = config::standard();
    let val = EvChargingStationLoad {
        station_id: "EVSE-PARK-009".to_string(),
        connector_type: EvConnectorType::Type2AC,
        max_power_kw: 22.0,
        current_power_kw: 0.0,
        session_energy_kwh: 0.0,
        vehicle_soc_pct: None,
        price_per_kwh_cents: 32,
        occupied: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode EvChargingStationLoad idle");
    let (decoded, _): (EvChargingStationLoad, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EvChargingStationLoad idle");
    assert_eq!(val, decoded);
}

#[test]
fn test_hydroelectric_dam_flow_multi_turbine() {
    let cfg = config::standard();
    let val = HydroelectricDamFlow {
        dam_id: "DAM-COLUMBIA-05".to_string(),
        reservoir_level_m: 342.8,
        inflow_m3_per_s: 1200.0,
        outflow_m3_per_s: 1150.0,
        spillway_open: false,
        turbine_flows: vec![280.0, 290.0, 285.0, 295.0],
        generation_mw: 480.0,
        environmental_flow_m3_per_s: 50.0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode HydroelectricDamFlow");
    let (decoded, _): (HydroelectricDamFlow, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HydroelectricDamFlow");
    assert_eq!(val, decoded);
}

#[test]
fn test_geothermal_well_high_enthalpy() {
    let cfg = config::standard();
    let val = GeothermalWellData {
        well_id: "GEO-ICE-KR-21".to_string(),
        depth_m: 2500.0,
        bottom_hole_temp_celsius: 320.0,
        wellhead_pressure_bar: 18.5,
        steam_fraction: 0.85,
        flow_rate_kg_per_s: 45.0,
        enthalpy_kj_per_kg: 2750.0,
        scaling_risk: ScalingRisk::Inhibited,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode GeothermalWellData");
    let (decoded, _): (GeothermalWellData, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GeothermalWellData");
    assert_eq!(val, decoded);
}

#[test]
fn test_energy_certificate_trading_retired() {
    let cfg = config::standard();
    let val = EnergyCertificateRecord {
        certificate_id: "REC-2025-US-0042187".to_string(),
        source: RenewableSource::WindOffshore,
        generation_mwh: 1.0,
        vintage_year: 2025,
        registry: "M-RETS".to_string(),
        owner_account: "CORP-ACME-ENERGY".to_string(),
        retired: true,
        price_cents_per_mwh: 3500,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode EnergyCertificateRecord retired");
    let (decoded, _): (EnergyCertificateRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EnergyCertificateRecord retired");
    assert_eq!(val, decoded);
}

#[test]
fn test_weather_forecast_integration_cloudy() {
    let cfg = config::standard();
    let val = WeatherForecastIntegration {
        location_id: "WX-SITE-042".to_string(),
        latitude: 34.0522,
        longitude: -118.2437,
        forecast_horizon_hours: 48,
        global_horizontal_irradiance_wm2: 120.0,
        wind_speed_10m_ms: 3.5,
        wind_direction_deg: 270.0,
        ambient_temp_celsius: 18.0,
        cloud_cover_pct: 75.0,
        precipitation_mm: 2.5,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode WeatherForecastIntegration");
    let (decoded, _): (WeatherForecastIntegration, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WeatherForecastIntegration");
    assert_eq!(val, decoded);
}

#[test]
fn test_power_purchase_agreement_long_term() {
    let cfg = config::standard();
    let val = PowerPurchaseAgreement {
        ppa_id: "PPA-2024-SOLAR-019".to_string(),
        buyer: "MegaCorp Industries".to_string(),
        seller: "SunField Renewables LLC".to_string(),
        contracted_mw: 150.0,
        strike_price_cents_per_mwh: 2800,
        escalation_pct_annual: 1.5,
        term_years: 20,
        curtailment_allowed: true,
        balancing_responsible: "SunField Renewables LLC".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PowerPurchaseAgreement");
    let (decoded, _): (PowerPurchaseAgreement, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PowerPurchaseAgreement");
    assert_eq!(val, decoded);
}

#[test]
fn test_transmission_constraint_congested() {
    let cfg = config::standard();
    let val = TransmissionConstraint {
        line_id: "TL-345KV-NE-042".to_string(),
        from_bus: "BUS-NORTH-1201".to_string(),
        to_bus: "BUS-SOUTH-1405".to_string(),
        thermal_limit_mw: 1200.0,
        current_flow_mw: 1185.0,
        congestion_price_cents: 15200,
        contingency_rating_mw: 1080.0,
        overloaded: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode TransmissionConstraint");
    let (decoded, _): (TransmissionConstraint, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransmissionConstraint");
    assert_eq!(val, decoded);
}

#[test]
fn test_energy_storage_dispatch_with_schedule() {
    let cfg = config::standard();
    let val = EnergyStorageDispatch {
        unit_id: "BESS-DISP-003".to_string(),
        mode: StorageDispatchMode::PeakShaving,
        power_kw: 500.0,
        target_soc_pct: 20.0,
        market_signal: 85.5,
        schedule_slots: vec![
            ScheduleSlot {
                hour: 14,
                power_kw: 500.0,
                price_signal: 92.0,
            },
            ScheduleSlot {
                hour: 15,
                power_kw: 750.0,
                price_signal: 110.0,
            },
            ScheduleSlot {
                hour: 16,
                power_kw: 1000.0,
                price_signal: 145.0,
            },
            ScheduleSlot {
                hour: 17,
                power_kw: 1000.0,
                price_signal: 160.0,
            },
            ScheduleSlot {
                hour: 18,
                power_kw: 750.0,
                price_signal: 130.0,
            },
            ScheduleSlot {
                hour: 19,
                power_kw: 250.0,
                price_signal: 75.0,
            },
        ],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode EnergyStorageDispatch");
    let (decoded, _): (EnergyStorageDispatch, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EnergyStorageDispatch");
    assert_eq!(val, decoded);
}

#[test]
fn test_virtual_power_plant_aggregation() {
    let cfg = config::standard();
    let val = VirtualPowerPlant {
        vpp_id: "VPP-REGION-NE-01".to_string(),
        name: "Northeast Distributed Fleet".to_string(),
        aggregated_capacity_mw: 85.0,
        asset_count: 342,
        asset_types: vec![
            "residential_solar".to_string(),
            "commercial_battery".to_string(),
            "ev_charger_v2g".to_string(),
            "smart_thermostat".to_string(),
        ],
        dispatch_priority: vec![
            "commercial_battery".to_string(),
            "ev_charger_v2g".to_string(),
            "residential_solar".to_string(),
            "smart_thermostat".to_string(),
        ],
        min_response_time_secs: 4,
        contracted_capacity_mw: 60.0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode VirtualPowerPlant");
    let (decoded, _): (VirtualPowerPlant, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VirtualPowerPlant");
    assert_eq!(val, decoded);
}

#[test]
fn test_carbon_intensity_with_forecast() {
    let cfg = config::standard();
    let val = CarbonIntensityReading {
        grid_region: "CAISO".to_string(),
        timestamp_epoch: 1719504000,
        grams_co2_per_kwh: 185.0,
        marginal_source: "natural_gas_ccgt".to_string(),
        forecast_24h: vec![
            210.0, 220.0, 200.0, 180.0, 150.0, 120.0, 95.0, 80.0, 75.0, 70.0, 65.0, 60.0, 65.0,
            80.0, 110.0, 150.0, 190.0, 230.0, 250.0, 240.0, 220.0, 200.0, 195.0, 190.0,
        ],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CarbonIntensityReading");
    let (decoded, _): (CarbonIntensityReading, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CarbonIntensityReading");
    assert_eq!(val, decoded);
}

#[test]
fn test_protection_relay_overcurrent_lockout() {
    let cfg = config::standard();
    let val = ProtectionRelayEvent {
        relay_id: "RELAY-SUB-NE-07-A".to_string(),
        event_type: ProtectionEventType::Overcurrent,
        fault_current_a: 12500.0,
        trip_time_ms: 45.0,
        recloser_attempts: 3,
        zone: 2,
        lockout: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ProtectionRelayEvent");
    let (decoded, _): (ProtectionRelayEvent, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ProtectionRelayEvent");
    assert_eq!(val, decoded);
}
