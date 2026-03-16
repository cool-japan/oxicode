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

// --- HVAC Building Automation Domain Types ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AhuOperatingMode {
    Heating,
    Cooling,
    Economizer,
    Dehumidification,
    Standby,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AirHandlingUnitConfig {
    ahu_id: u32,
    name: String,
    operating_mode: AhuOperatingMode,
    supply_air_temp_setpoint_f: f32,
    return_air_temp_f: f32,
    mixed_air_temp_f: f32,
    outdoor_air_damper_pct: f32,
    supply_fan_speed_pct: f32,
    return_fan_speed_pct: f32,
    filter_alarm_active: bool,
    total_cfm: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VavBoxPosition {
    vav_id: String,
    zone_name: String,
    damper_position_pct: f32,
    airflow_cfm: u32,
    reheat_valve_pct: f32,
    zone_temp_f: f32,
    zone_setpoint_f: f32,
    is_in_deadband: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ChillerState {
    Off,
    Starting,
    Running { load_pct: f32 },
    Stopping,
    Fault(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ChillerPlantOptimization {
    plant_id: u32,
    chiller_count: u8,
    active_chillers: Vec<u32>,
    chiller_states: Vec<ChillerState>,
    chilled_water_supply_temp_f: f32,
    chilled_water_return_temp_f: f32,
    condenser_water_temp_f: f32,
    plant_kw: f64,
    plant_tons: f64,
    kw_per_ton: f64,
    optimal_staging_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BoilerFireRate {
    Off,
    LowFire,
    MediumFire,
    HighFire,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BoilerCascadeControl {
    lead_boiler_id: u32,
    lag_boiler_ids: Vec<u32>,
    lead_fire_rate: BoilerFireRate,
    lag_fire_rates: Vec<BoilerFireRate>,
    hot_water_supply_temp_f: f32,
    hot_water_return_temp_f: f32,
    supply_setpoint_f: f32,
    outdoor_air_reset_active: bool,
    cascade_delay_seconds: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ScheduleEntry {
    day_of_week: u8,
    occupied_start_hour: u8,
    occupied_start_minute: u8,
    occupied_end_hour: u8,
    occupied_end_minute: u8,
    heating_setpoint_f: f32,
    cooling_setpoint_f: f32,
    unoccupied_heating_setpoint_f: f32,
    unoccupied_cooling_setpoint_f: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ThermostatScheduleConfig {
    thermostat_id: String,
    zone_name: String,
    schedule: Vec<ScheduleEntry>,
    holiday_override_active: bool,
    temp_override_expiry_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DuctPressureReading {
    sensor_id: String,
    duct_section: String,
    static_pressure_inwc: f32,
    velocity_pressure_inwc: f32,
    total_pressure_inwc: f32,
    alarm_high_limit_inwc: f32,
    alarm_low_limit_inwc: f32,
    alarm_state: bool,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RefrigerantType {
    R410A,
    R134A,
    R32,
    R454B,
    R1234yf,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RefrigerantChargeLevel {
    circuit_id: u32,
    refrigerant_type: RefrigerantType,
    charge_oz: f32,
    subcooling_f: f32,
    superheat_f: f32,
    suction_pressure_psig: f32,
    discharge_pressure_psig: f32,
    charge_status_ok: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BacnetObjectType {
    AnalogInput,
    AnalogOutput,
    AnalogValue,
    BinaryInput,
    BinaryOutput,
    BinaryValue,
    MultiStateInput,
    MultiStateOutput,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BacnetPointConfig {
    device_instance: u32,
    object_type: BacnetObjectType,
    object_instance: u32,
    object_name: String,
    description: String,
    engineering_units: String,
    cov_increment: Option<f32>,
    out_of_service: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EnergyRecoveryWheelState {
    wheel_id: u32,
    rotation_rpm: f32,
    supply_entering_temp_f: f32,
    supply_leaving_temp_f: f32,
    exhaust_entering_temp_f: f32,
    exhaust_leaving_temp_f: f32,
    sensible_effectiveness_pct: f32,
    latent_effectiveness_pct: f32,
    bypass_damper_open: bool,
    frost_protection_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FanStage {
    Off,
    Stage1,
    Stage2,
    Stage3,
    VariableSpeed(f32),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CoolingTowerFanStaging {
    tower_id: u32,
    cell_count: u8,
    fan_stages: Vec<FanStage>,
    basin_temp_f: f32,
    approach_temp_f: f32,
    wet_bulb_temp_f: f32,
    blowdown_active: bool,
    chemical_treatment_active: bool,
    vibration_alarm: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ZoneComfortMetrics {
    zone_id: String,
    predicted_mean_vote: f32,
    predicted_pct_dissatisfied: f32,
    operative_temp_f: f32,
    mean_radiant_temp_f: f32,
    relative_humidity_pct: f32,
    air_velocity_fpm: f32,
    clothing_insulation_clo: f32,
    metabolic_rate_met: f32,
    comfort_acceptable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FilterDifferentialPressure {
    filter_bank_id: String,
    ahu_id: u32,
    filter_type: String,
    merv_rating: u8,
    dp_inwc: f32,
    clean_dp_inwc: f32,
    dirty_limit_inwc: f32,
    pct_life_remaining: f32,
    replacement_due: bool,
    last_replaced_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ActuatorType {
    SpringReturn,
    NonSpringReturn,
    FloatingPoint,
    Proportional,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DamperActuatorPosition {
    actuator_id: String,
    damper_name: String,
    actuator_type: ActuatorType,
    commanded_pct: f32,
    feedback_pct: f32,
    travel_time_seconds: u16,
    stroke_count: u32,
    override_active: bool,
    fault_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PumpConfiguration {
    pump_id: u32,
    pump_name: String,
    is_lead: bool,
    speed_pct: f32,
    differential_pressure_psi: f32,
    flow_gpm: f32,
    motor_amps: f32,
    vfd_fault: bool,
    runtime_hours: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum HumidifierType {
    Steam,
    Ultrasonic,
    Evaporative,
    Atomizing,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HumidificationControl {
    humidifier_id: u32,
    humidifier_type: HumidifierType,
    zone_rh_pct: f32,
    setpoint_rh_pct: f32,
    output_pct: f32,
    water_conductivity_us: f32,
    drain_cycle_active: bool,
    steam_pressure_psig: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ExhaustFanInterlock {
    fan_id: String,
    area_served: String,
    cfm_rated: u32,
    cfm_actual: u32,
    interlock_zone_ids: Vec<String>,
    makeup_air_required: bool,
    current_status_on: bool,
    dp_across_fan_inwc: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HeatExchangerPerformance {
    hx_id: u32,
    hx_type: String,
    primary_inlet_temp_f: f32,
    primary_outlet_temp_f: f32,
    secondary_inlet_temp_f: f32,
    secondary_outlet_temp_f: f32,
    primary_flow_gpm: f32,
    secondary_flow_gpm: f32,
    fouling_factor: f64,
    effectiveness_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BuildingPressurization {
    building_id: u32,
    reference_pressure_inwc: f32,
    actual_pressure_inwc: f32,
    relief_damper_pct: f32,
    stairwell_pressurization_on: bool,
    smoke_mode_active: bool,
    wind_speed_mph: f32,
    wind_direction_deg: u16,
}

// --- Tests ---

#[test]
fn test_ahu_config_roundtrip() {
    let cfg = config::standard();
    let ahu = AirHandlingUnitConfig {
        ahu_id: 1,
        name: "AHU-1 North Wing".to_string(),
        operating_mode: AhuOperatingMode::Cooling,
        supply_air_temp_setpoint_f: 55.0,
        return_air_temp_f: 74.2,
        mixed_air_temp_f: 63.5,
        outdoor_air_damper_pct: 35.0,
        supply_fan_speed_pct: 78.5,
        return_fan_speed_pct: 72.0,
        filter_alarm_active: false,
        total_cfm: 12000,
    };
    let bytes = encode_to_vec(&ahu, cfg).expect("encode AirHandlingUnitConfig");
    let (decoded, _): (AirHandlingUnitConfig, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AirHandlingUnitConfig");
    assert_eq!(ahu, decoded);
}

#[test]
fn test_vav_box_position_roundtrip() {
    let cfg = config::standard();
    let vav = VavBoxPosition {
        vav_id: "VAV-3-201".to_string(),
        zone_name: "Conference Room 201".to_string(),
        damper_position_pct: 45.0,
        airflow_cfm: 320,
        reheat_valve_pct: 0.0,
        zone_temp_f: 73.1,
        zone_setpoint_f: 72.0,
        is_in_deadband: true,
    };
    let bytes = encode_to_vec(&vav, cfg).expect("encode VavBoxPosition");
    let (decoded, _): (VavBoxPosition, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VavBoxPosition");
    assert_eq!(vav, decoded);
}

#[test]
fn test_chiller_plant_optimization_roundtrip() {
    let cfg = config::standard();
    let plant = ChillerPlantOptimization {
        plant_id: 1,
        chiller_count: 3,
        active_chillers: vec![1, 3],
        chiller_states: vec![
            ChillerState::Running { load_pct: 82.5 },
            ChillerState::Off,
            ChillerState::Running { load_pct: 65.0 },
        ],
        chilled_water_supply_temp_f: 44.2,
        chilled_water_return_temp_f: 56.8,
        condenser_water_temp_f: 82.0,
        plant_kw: 485.3,
        plant_tons: 820.0,
        kw_per_ton: 0.592,
        optimal_staging_active: true,
    };
    let bytes = encode_to_vec(&plant, cfg).expect("encode ChillerPlantOptimization");
    let (decoded, _): (ChillerPlantOptimization, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ChillerPlantOptimization");
    assert_eq!(plant, decoded);
}

#[test]
fn test_chiller_fault_state_roundtrip() {
    let cfg = config::standard();
    let plant = ChillerPlantOptimization {
        plant_id: 2,
        chiller_count: 2,
        active_chillers: vec![2],
        chiller_states: vec![
            ChillerState::Fault("High discharge pressure".to_string()),
            ChillerState::Running { load_pct: 95.0 },
        ],
        chilled_water_supply_temp_f: 46.0,
        chilled_water_return_temp_f: 58.0,
        condenser_water_temp_f: 88.5,
        plant_kw: 310.0,
        plant_tons: 400.0,
        kw_per_ton: 0.775,
        optimal_staging_active: false,
    };
    let bytes = encode_to_vec(&plant, cfg).expect("encode chiller fault state");
    let (decoded, _): (ChillerPlantOptimization, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode chiller fault state");
    assert_eq!(plant, decoded);
}

#[test]
fn test_boiler_cascade_control_roundtrip() {
    let cfg = config::standard();
    let cascade = BoilerCascadeControl {
        lead_boiler_id: 1,
        lag_boiler_ids: vec![2, 3],
        lead_fire_rate: BoilerFireRate::HighFire,
        lag_fire_rates: vec![BoilerFireRate::MediumFire, BoilerFireRate::Off],
        hot_water_supply_temp_f: 178.5,
        hot_water_return_temp_f: 155.0,
        supply_setpoint_f: 180.0,
        outdoor_air_reset_active: true,
        cascade_delay_seconds: 300,
    };
    let bytes = encode_to_vec(&cascade, cfg).expect("encode BoilerCascadeControl");
    let (decoded, _): (BoilerCascadeControl, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BoilerCascadeControl");
    assert_eq!(cascade, decoded);
}

#[test]
fn test_thermostat_schedule_config_roundtrip() {
    let cfg = config::standard();
    let sched = ThermostatScheduleConfig {
        thermostat_id: "TSTAT-FL2-ZONE4".to_string(),
        zone_name: "Open Office Floor 2".to_string(),
        schedule: vec![
            ScheduleEntry {
                day_of_week: 1,
                occupied_start_hour: 7,
                occupied_start_minute: 0,
                occupied_end_hour: 18,
                occupied_end_minute: 0,
                heating_setpoint_f: 70.0,
                cooling_setpoint_f: 74.0,
                unoccupied_heating_setpoint_f: 60.0,
                unoccupied_cooling_setpoint_f: 85.0,
            },
            ScheduleEntry {
                day_of_week: 6,
                occupied_start_hour: 9,
                occupied_start_minute: 0,
                occupied_end_hour: 13,
                occupied_end_minute: 0,
                heating_setpoint_f: 68.0,
                cooling_setpoint_f: 76.0,
                unoccupied_heating_setpoint_f: 58.0,
                unoccupied_cooling_setpoint_f: 85.0,
            },
        ],
        holiday_override_active: false,
        temp_override_expiry_epoch: None,
    };
    let bytes = encode_to_vec(&sched, cfg).expect("encode ThermostatScheduleConfig");
    let (decoded, _): (ThermostatScheduleConfig, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ThermostatScheduleConfig");
    assert_eq!(sched, decoded);
}

#[test]
fn test_thermostat_with_override_roundtrip() {
    let cfg = config::standard();
    let sched = ThermostatScheduleConfig {
        thermostat_id: "TSTAT-FL5-EXEC".to_string(),
        zone_name: "Executive Suite".to_string(),
        schedule: vec![ScheduleEntry {
            day_of_week: 3,
            occupied_start_hour: 6,
            occupied_start_minute: 30,
            occupied_end_hour: 20,
            occupied_end_minute: 0,
            heating_setpoint_f: 71.0,
            cooling_setpoint_f: 73.0,
            unoccupied_heating_setpoint_f: 62.0,
            unoccupied_cooling_setpoint_f: 82.0,
        }],
        holiday_override_active: true,
        temp_override_expiry_epoch: Some(1_710_000_000),
    };
    let bytes = encode_to_vec(&sched, cfg).expect("encode thermostat override");
    let (decoded, _): (ThermostatScheduleConfig, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode thermostat override");
    assert_eq!(sched, decoded);
}

#[test]
fn test_duct_pressure_reading_roundtrip() {
    let cfg = config::standard();
    let reading = DuctPressureReading {
        sensor_id: "DP-AHU1-SUPPLY".to_string(),
        duct_section: "Main Supply Trunk".to_string(),
        static_pressure_inwc: 1.25,
        velocity_pressure_inwc: 0.18,
        total_pressure_inwc: 1.43,
        alarm_high_limit_inwc: 2.5,
        alarm_low_limit_inwc: 0.5,
        alarm_state: false,
        timestamp_epoch: 1_709_900_000,
    };
    let bytes = encode_to_vec(&reading, cfg).expect("encode DuctPressureReading");
    let (decoded, _): (DuctPressureReading, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DuctPressureReading");
    assert_eq!(reading, decoded);
}

#[test]
fn test_refrigerant_charge_level_roundtrip() {
    let cfg = config::standard();
    let charge = RefrigerantChargeLevel {
        circuit_id: 1,
        refrigerant_type: RefrigerantType::R410A,
        charge_oz: 288.0,
        subcooling_f: 12.5,
        superheat_f: 10.0,
        suction_pressure_psig: 118.0,
        discharge_pressure_psig: 380.0,
        charge_status_ok: true,
    };
    let bytes = encode_to_vec(&charge, cfg).expect("encode RefrigerantChargeLevel");
    let (decoded, _): (RefrigerantChargeLevel, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RefrigerantChargeLevel");
    assert_eq!(charge, decoded);
}

#[test]
fn test_bacnet_point_config_analog_roundtrip() {
    let cfg = config::standard();
    let point = BacnetPointConfig {
        device_instance: 100001,
        object_type: BacnetObjectType::AnalogInput,
        object_instance: 1,
        object_name: "AHU1-SAT".to_string(),
        description: "AHU-1 Supply Air Temperature".to_string(),
        engineering_units: "degF".to_string(),
        cov_increment: Some(0.5),
        out_of_service: false,
    };
    let bytes = encode_to_vec(&point, cfg).expect("encode BacnetPointConfig analog");
    let (decoded, _): (BacnetPointConfig, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BacnetPointConfig analog");
    assert_eq!(point, decoded);
}

#[test]
fn test_bacnet_point_config_binary_roundtrip() {
    let cfg = config::standard();
    let point = BacnetPointConfig {
        device_instance: 100002,
        object_type: BacnetObjectType::BinaryOutput,
        object_instance: 12,
        object_name: "AHU2-SF-CMD".to_string(),
        description: "AHU-2 Supply Fan Command".to_string(),
        engineering_units: "on/off".to_string(),
        cov_increment: None,
        out_of_service: false,
    };
    let bytes = encode_to_vec(&point, cfg).expect("encode BacnetPointConfig binary");
    let (decoded, _): (BacnetPointConfig, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BacnetPointConfig binary");
    assert_eq!(point, decoded);
}

#[test]
fn test_energy_recovery_wheel_state_roundtrip() {
    let cfg = config::standard();
    let wheel = EnergyRecoveryWheelState {
        wheel_id: 1,
        rotation_rpm: 12.5,
        supply_entering_temp_f: 95.0,
        supply_leaving_temp_f: 78.0,
        exhaust_entering_temp_f: 74.0,
        exhaust_leaving_temp_f: 88.0,
        sensible_effectiveness_pct: 76.2,
        latent_effectiveness_pct: 62.0,
        bypass_damper_open: false,
        frost_protection_active: false,
    };
    let bytes = encode_to_vec(&wheel, cfg).expect("encode EnergyRecoveryWheelState");
    let (decoded, _): (EnergyRecoveryWheelState, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EnergyRecoveryWheelState");
    assert_eq!(wheel, decoded);
}

#[test]
fn test_cooling_tower_fan_staging_roundtrip() {
    let cfg = config::standard();
    let tower = CoolingTowerFanStaging {
        tower_id: 1,
        cell_count: 3,
        fan_stages: vec![
            FanStage::Stage3,
            FanStage::VariableSpeed(85.0),
            FanStage::Stage1,
        ],
        basin_temp_f: 82.0,
        approach_temp_f: 7.0,
        wet_bulb_temp_f: 75.0,
        blowdown_active: true,
        chemical_treatment_active: true,
        vibration_alarm: false,
    };
    let bytes = encode_to_vec(&tower, cfg).expect("encode CoolingTowerFanStaging");
    let (decoded, _): (CoolingTowerFanStaging, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CoolingTowerFanStaging");
    assert_eq!(tower, decoded);
}

#[test]
fn test_zone_comfort_metrics_roundtrip() {
    let cfg = config::standard();
    let comfort = ZoneComfortMetrics {
        zone_id: "ZONE-FL3-SOUTH".to_string(),
        predicted_mean_vote: 0.3,
        predicted_pct_dissatisfied: 7.0,
        operative_temp_f: 73.5,
        mean_radiant_temp_f: 72.0,
        relative_humidity_pct: 45.0,
        air_velocity_fpm: 25.0,
        clothing_insulation_clo: 0.6,
        metabolic_rate_met: 1.1,
        comfort_acceptable: true,
    };
    let bytes = encode_to_vec(&comfort, cfg).expect("encode ZoneComfortMetrics");
    let (decoded, _): (ZoneComfortMetrics, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ZoneComfortMetrics");
    assert_eq!(comfort, decoded);
}

#[test]
fn test_filter_differential_pressure_roundtrip() {
    let cfg = config::standard();
    let filter = FilterDifferentialPressure {
        filter_bank_id: "FB-AHU3-PRE".to_string(),
        ahu_id: 3,
        filter_type: "Pleated Panel".to_string(),
        merv_rating: 13,
        dp_inwc: 0.85,
        clean_dp_inwc: 0.25,
        dirty_limit_inwc: 1.5,
        pct_life_remaining: 48.0,
        replacement_due: false,
        last_replaced_epoch: 1_700_000_000,
    };
    let bytes = encode_to_vec(&filter, cfg).expect("encode FilterDifferentialPressure");
    let (decoded, _): (FilterDifferentialPressure, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FilterDifferentialPressure");
    assert_eq!(filter, decoded);
}

#[test]
fn test_damper_actuator_position_roundtrip() {
    let cfg = config::standard();
    let damper = DamperActuatorPosition {
        actuator_id: "ACT-OA-AHU1".to_string(),
        damper_name: "Outdoor Air Damper AHU-1".to_string(),
        actuator_type: ActuatorType::Proportional,
        commanded_pct: 42.0,
        feedback_pct: 41.5,
        travel_time_seconds: 90,
        stroke_count: 185_000,
        override_active: false,
        fault_detected: false,
    };
    let bytes = encode_to_vec(&damper, cfg).expect("encode DamperActuatorPosition");
    let (decoded, _): (DamperActuatorPosition, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DamperActuatorPosition");
    assert_eq!(damper, decoded);
}

#[test]
fn test_pump_configuration_roundtrip() {
    let cfg = config::standard();
    let pump = PumpConfiguration {
        pump_id: 2,
        pump_name: "CHWP-2 Secondary".to_string(),
        is_lead: false,
        speed_pct: 68.0,
        differential_pressure_psi: 12.5,
        flow_gpm: 450.0,
        motor_amps: 28.3,
        vfd_fault: false,
        runtime_hours: 42_350,
    };
    let bytes = encode_to_vec(&pump, cfg).expect("encode PumpConfiguration");
    let (decoded, _): (PumpConfiguration, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PumpConfiguration");
    assert_eq!(pump, decoded);
}

#[test]
fn test_humidification_control_steam_roundtrip() {
    let cfg = config::standard();
    let humidifier = HumidificationControl {
        humidifier_id: 1,
        humidifier_type: HumidifierType::Steam,
        zone_rh_pct: 28.0,
        setpoint_rh_pct: 40.0,
        output_pct: 75.0,
        water_conductivity_us: 120.0,
        drain_cycle_active: false,
        steam_pressure_psig: Some(5.0),
    };
    let bytes = encode_to_vec(&humidifier, cfg).expect("encode HumidificationControl steam");
    let (decoded, _): (HumidificationControl, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HumidificationControl steam");
    assert_eq!(humidifier, decoded);
}

#[test]
fn test_humidification_control_ultrasonic_roundtrip() {
    let cfg = config::standard();
    let humidifier = HumidificationControl {
        humidifier_id: 4,
        humidifier_type: HumidifierType::Ultrasonic,
        zone_rh_pct: 52.0,
        setpoint_rh_pct: 50.0,
        output_pct: 10.0,
        water_conductivity_us: 85.0,
        drain_cycle_active: true,
        steam_pressure_psig: None,
    };
    let bytes = encode_to_vec(&humidifier, cfg).expect("encode HumidificationControl ultrasonic");
    let (decoded, _): (HumidificationControl, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HumidificationControl ultrasonic");
    assert_eq!(humidifier, decoded);
}

#[test]
fn test_exhaust_fan_interlock_roundtrip() {
    let cfg = config::standard();
    let fan = ExhaustFanInterlock {
        fan_id: "EF-LAB-301".to_string(),
        area_served: "Chemistry Lab 301".to_string(),
        cfm_rated: 2000,
        cfm_actual: 1850,
        interlock_zone_ids: vec![
            "ZONE-LAB-301".to_string(),
            "ZONE-LAB-302".to_string(),
            "ZONE-CORR-3W".to_string(),
        ],
        makeup_air_required: true,
        current_status_on: true,
        dp_across_fan_inwc: 0.75,
    };
    let bytes = encode_to_vec(&fan, cfg).expect("encode ExhaustFanInterlock");
    let (decoded, _): (ExhaustFanInterlock, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ExhaustFanInterlock");
    assert_eq!(fan, decoded);
}

#[test]
fn test_heat_exchanger_performance_roundtrip() {
    let cfg = config::standard();
    let hx = HeatExchangerPerformance {
        hx_id: 1,
        hx_type: "Plate-and-Frame".to_string(),
        primary_inlet_temp_f: 180.0,
        primary_outlet_temp_f: 150.0,
        secondary_inlet_temp_f: 120.0,
        secondary_outlet_temp_f: 145.0,
        primary_flow_gpm: 200.0,
        secondary_flow_gpm: 220.0,
        fouling_factor: 0.0005,
        effectiveness_pct: 83.3,
    };
    let bytes = encode_to_vec(&hx, cfg).expect("encode HeatExchangerPerformance");
    let (decoded, _): (HeatExchangerPerformance, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HeatExchangerPerformance");
    assert_eq!(hx, decoded);
}

#[test]
fn test_building_pressurization_roundtrip() {
    let cfg = config::standard();
    let bp = BuildingPressurization {
        building_id: 1,
        reference_pressure_inwc: 0.05,
        actual_pressure_inwc: 0.06,
        relief_damper_pct: 15.0,
        stairwell_pressurization_on: true,
        smoke_mode_active: false,
        wind_speed_mph: 12.0,
        wind_direction_deg: 225,
    };
    let bytes = encode_to_vec(&bp, cfg).expect("encode BuildingPressurization");
    let (decoded, _): (BuildingPressurization, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BuildingPressurization");
    assert_eq!(bp, decoded);
}
