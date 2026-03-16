//! Advanced checksum tests for OxiCode -- Formula 1 racing telemetry theme.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced32_test

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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — F1 racing telemetry
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SectorSplit {
    sector: u8,
    time_ms: u32,
    speed_trap_kph: f64,
    is_personal_best: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LapTiming {
    lap_number: u16,
    sectors: Vec<SectorSplit>,
    total_time_ms: u32,
    gap_to_leader_ms: i32,
    interval_ms: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TireCompound {
    Soft,
    Medium,
    Hard,
    Intermediate,
    FullWet,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TireTelemetry {
    compound: TireCompound,
    surface_temp_c: f64,
    inner_temp_c: f64,
    pressure_psi: f64,
    degradation_pct: f64,
    laps_on_tire: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TireSet {
    front_left: TireTelemetry,
    front_right: TireTelemetry,
    rear_left: TireTelemetry,
    rear_right: TireTelemetry,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EngineParameters {
    rpm: u16,
    mgu_k_harvesting_kw: f64,
    mgu_h_harvesting_kw: f64,
    ers_store_kj: f64,
    oil_temp_c: f64,
    water_temp_c: f64,
    fuel_flow_kg_per_hour: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrsZone {
    zone_id: u8,
    detection_point_m: f64,
    activation_point_m: f64,
    is_open: bool,
    speed_delta_kph: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuelStrategy {
    current_load_kg: f64,
    target_finish_kg: f64,
    consumption_per_lap_kg: f64,
    laps_remaining: u16,
    fuel_delta_kg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BrakeTemperatures {
    front_left_c: f64,
    front_right_c: f64,
    rear_left_c: f64,
    rear_right_c: f64,
    bias_pct_front: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SuspensionTravel {
    front_left_mm: f64,
    front_right_mm: f64,
    rear_left_mm: f64,
    rear_right_mm: f64,
    ride_height_front_mm: f64,
    ride_height_rear_mm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DriverInputs {
    steering_angle_deg: f64,
    throttle_pct: f64,
    brake_pct: f64,
    gear: i8,
    drs_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ErsDeployment {
    mode: u8,
    deploy_kj_per_lap: f64,
    harvest_kj_per_lap: f64,
    battery_charge_pct: f64,
    laps_until_target: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PitStopBreakdown {
    total_time_s: f64,
    jack_time_s: f64,
    front_left_wheel_s: f64,
    front_right_wheel_s: f64,
    rear_left_wheel_s: f64,
    rear_right_wheel_s: f64,
    fuel_time_s: f64,
    front_wing_adjust_s: f64,
    release_time_s: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeatherCondition {
    Dry,
    LightRain,
    HeavyRain,
    Overcast,
    Changeable,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherRadar {
    condition: WeatherCondition,
    rain_probability_pct: f64,
    track_temp_c: f64,
    air_temp_c: f64,
    humidity_pct: f64,
    wind_speed_kph: f64,
    wind_direction_deg: f64,
    forecast_change_in_laps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WindTunnelCorrelation {
    downforce_n: f64,
    drag_n: f64,
    side_force_n: f64,
    cl: f64,
    cd: f64,
    efficiency_ratio: f64,
    yaw_angle_deg: f64,
    tunnel_speed_kph: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CfdValidation {
    mesh_cells: u64,
    convergence_residual: f64,
    pressure_delta_pa: f64,
    predicted_downforce_n: f64,
    measured_downforce_n: f64,
    correlation_pct: f64,
    simulation_tag: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DriverBiometrics {
    heart_rate_bpm: u16,
    g_lateral: f64,
    g_longitudinal: f64,
    g_vertical: f64,
    core_temp_c: f64,
    hydration_ml_remaining: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TelemetryFrame {
    timestamp_ms: u64,
    car_number: u8,
    speed_kph: f64,
    driver_inputs: DriverInputs,
    engine: EngineParameters,
}

// ---------------------------------------------------------------------------
// Test 1: Lap timing with sector splits
// ---------------------------------------------------------------------------
#[test]
fn test_lap_timing_sector_splits() {
    let timing = LapTiming {
        lap_number: 42,
        sectors: vec![
            SectorSplit {
                sector: 1,
                time_ms: 28_345,
                speed_trap_kph: 312.7,
                is_personal_best: true,
            },
            SectorSplit {
                sector: 2,
                time_ms: 34_112,
                speed_trap_kph: 287.3,
                is_personal_best: false,
            },
            SectorSplit {
                sector: 3,
                time_ms: 26_891,
                speed_trap_kph: 298.1,
                is_personal_best: true,
            },
        ],
        total_time_ms: 89_348,
        gap_to_leader_ms: 2_341,
        interval_ms: 456,
    };
    let encoded = encode_with_checksum(&timing).expect("encode lap timing");
    let (decoded, consumed): (LapTiming, _) =
        decode_with_checksum(&encoded).expect("decode lap timing");
    assert_eq!(decoded, timing);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Full tire set telemetry
// ---------------------------------------------------------------------------
#[test]
fn test_tire_set_telemetry() {
    let make_tire = |compound, surf, inner, psi, deg, laps| TireTelemetry {
        compound,
        surface_temp_c: surf,
        inner_temp_c: inner,
        pressure_psi: psi,
        degradation_pct: deg,
        laps_on_tire: laps,
    };
    let tire_set = TireSet {
        front_left: make_tire(TireCompound::Soft, 105.3, 98.1, 22.5, 14.2, 12),
        front_right: make_tire(TireCompound::Soft, 107.8, 99.5, 22.7, 15.1, 12),
        rear_left: make_tire(TireCompound::Soft, 112.1, 103.4, 21.8, 18.3, 12),
        rear_right: make_tire(TireCompound::Soft, 113.6, 104.9, 22.0, 19.7, 12),
    };
    let encoded = encode_with_checksum(&tire_set).expect("encode tire set");
    let (decoded, consumed): (TireSet, _) =
        decode_with_checksum(&encoded).expect("decode tire set");
    assert_eq!(decoded, tire_set);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: Engine parameters with MGU-K / MGU-H harvesting
// ---------------------------------------------------------------------------
#[test]
fn test_engine_parameters_mgu() {
    let engine = EngineParameters {
        rpm: 12_450,
        mgu_k_harvesting_kw: 120.5,
        mgu_h_harvesting_kw: 85.3,
        ers_store_kj: 3_200.0,
        oil_temp_c: 128.4,
        water_temp_c: 105.7,
        fuel_flow_kg_per_hour: 97.3,
    };
    let encoded = encode_with_checksum(&engine).expect("encode engine params");
    let (decoded, consumed): (EngineParameters, _) =
        decode_with_checksum(&encoded).expect("decode engine params");
    assert_eq!(decoded, engine);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: DRS activation zones
// ---------------------------------------------------------------------------
#[test]
fn test_drs_activation_zones() {
    let zones = vec![
        DrsZone {
            zone_id: 1,
            detection_point_m: 850.0,
            activation_point_m: 920.0,
            is_open: true,
            speed_delta_kph: 12.4,
        },
        DrsZone {
            zone_id: 2,
            detection_point_m: 3_200.0,
            activation_point_m: 3_340.0,
            is_open: false,
            speed_delta_kph: 15.8,
        },
        DrsZone {
            zone_id: 3,
            detection_point_m: 4_610.0,
            activation_point_m: 4_700.0,
            is_open: true,
            speed_delta_kph: 10.2,
        },
    ];
    let encoded = encode_with_checksum(&zones).expect("encode DRS zones");
    let (decoded, consumed): (Vec<DrsZone>, _) =
        decode_with_checksum(&encoded).expect("decode DRS zones");
    assert_eq!(decoded, zones);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Fuel load calculations
// ---------------------------------------------------------------------------
#[test]
fn test_fuel_strategy_calculation() {
    let strategy = FuelStrategy {
        current_load_kg: 78.5,
        target_finish_kg: 1.5,
        consumption_per_lap_kg: 1.42,
        laps_remaining: 52,
        fuel_delta_kg: -0.34,
    };
    let encoded = encode_with_checksum(&strategy).expect("encode fuel strategy");
    let (decoded, consumed): (FuelStrategy, _) =
        decode_with_checksum(&encoded).expect("decode fuel strategy");
    assert_eq!(decoded, strategy);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Brake temperatures across all corners
// ---------------------------------------------------------------------------
#[test]
fn test_brake_temperatures() {
    let brakes = BrakeTemperatures {
        front_left_c: 850.2,
        front_right_c: 870.6,
        rear_left_c: 720.1,
        rear_right_c: 735.3,
        bias_pct_front: 57.5,
    };
    let encoded = encode_with_checksum(&brakes).expect("encode brake temps");
    let (decoded, consumed): (BrakeTemperatures, _) =
        decode_with_checksum(&encoded).expect("decode brake temps");
    assert_eq!(decoded, brakes);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Suspension travel and ride height
// ---------------------------------------------------------------------------
#[test]
fn test_suspension_travel() {
    let suspension = SuspensionTravel {
        front_left_mm: 32.4,
        front_right_mm: 31.8,
        rear_left_mm: 28.6,
        rear_right_mm: 29.1,
        ride_height_front_mm: 25.0,
        ride_height_rear_mm: 55.0,
    };
    let encoded = encode_with_checksum(&suspension).expect("encode suspension");
    let (decoded, consumed): (SuspensionTravel, _) =
        decode_with_checksum(&encoded).expect("decode suspension");
    assert_eq!(decoded, suspension);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Driver inputs — steering, throttle, brake, gear
// ---------------------------------------------------------------------------
#[test]
fn test_driver_inputs() {
    let inputs = DriverInputs {
        steering_angle_deg: -42.7,
        throttle_pct: 0.0,
        brake_pct: 87.3,
        gear: 3,
        drs_active: false,
    };
    let encoded = encode_with_checksum(&inputs).expect("encode driver inputs");
    let (decoded, consumed): (DriverInputs, _) =
        decode_with_checksum(&encoded).expect("decode driver inputs");
    assert_eq!(decoded, inputs);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: ERS deployment strategy
// ---------------------------------------------------------------------------
#[test]
fn test_ers_deployment_strategy() {
    let ers = ErsDeployment {
        mode: 3,
        deploy_kj_per_lap: 4_000.0,
        harvest_kj_per_lap: 3_100.0,
        battery_charge_pct: 62.5,
        laps_until_target: 8,
    };
    let encoded = encode_with_checksum(&ers).expect("encode ERS deployment");
    let (decoded, consumed): (ErsDeployment, _) =
        decode_with_checksum(&encoded).expect("decode ERS deployment");
    assert_eq!(decoded, ers);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Pit stop timing breakdown
// ---------------------------------------------------------------------------
#[test]
fn test_pit_stop_breakdown() {
    let pit = PitStopBreakdown {
        total_time_s: 2.34,
        jack_time_s: 0.15,
        front_left_wheel_s: 1.92,
        front_right_wheel_s: 1.88,
        rear_left_wheel_s: 1.95,
        rear_right_wheel_s: 1.90,
        fuel_time_s: 0.0,
        front_wing_adjust_s: 0.0,
        release_time_s: 0.21,
    };
    let encoded = encode_with_checksum(&pit).expect("encode pit stop");
    let (decoded, consumed): (PitStopBreakdown, _) =
        decode_with_checksum(&encoded).expect("decode pit stop");
    assert_eq!(decoded, pit);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Weather radar for race strategy
// ---------------------------------------------------------------------------
#[test]
fn test_weather_radar() {
    let weather = WeatherRadar {
        condition: WeatherCondition::Changeable,
        rain_probability_pct: 65.0,
        track_temp_c: 34.2,
        air_temp_c: 26.8,
        humidity_pct: 78.4,
        wind_speed_kph: 14.3,
        wind_direction_deg: 225.0,
        forecast_change_in_laps: 7,
    };
    let encoded = encode_with_checksum(&weather).expect("encode weather");
    let (decoded, consumed): (WeatherRadar, _) =
        decode_with_checksum(&encoded).expect("decode weather");
    assert_eq!(decoded, weather);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Wind tunnel correlation data
// ---------------------------------------------------------------------------
#[test]
fn test_wind_tunnel_correlation() {
    let tunnel = WindTunnelCorrelation {
        downforce_n: 18_500.0,
        drag_n: 6_200.0,
        side_force_n: 120.5,
        cl: 3.15,
        cd: 1.05,
        efficiency_ratio: 3.0,
        yaw_angle_deg: 2.5,
        tunnel_speed_kph: 180.0,
    };
    let encoded = encode_with_checksum(&tunnel).expect("encode wind tunnel");
    let (decoded, consumed): (WindTunnelCorrelation, _) =
        decode_with_checksum(&encoded).expect("decode wind tunnel");
    assert_eq!(decoded, tunnel);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: CFD simulation validation
// ---------------------------------------------------------------------------
#[test]
fn test_cfd_simulation_validation() {
    let cfd = CfdValidation {
        mesh_cells: 120_000_000,
        convergence_residual: 1e-6,
        pressure_delta_pa: 245.8,
        predicted_downforce_n: 18_750.0,
        measured_downforce_n: 18_500.0,
        correlation_pct: 98.7,
        simulation_tag: String::from("FW-Rev7-Monza-2026-Low-DF"),
    };
    let encoded = encode_with_checksum(&cfd).expect("encode CFD validation");
    let (decoded, consumed): (CfdValidation, _) =
        decode_with_checksum(&encoded).expect("decode CFD validation");
    assert_eq!(decoded, cfd);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Driver biometrics — heart rate and G-forces
// ---------------------------------------------------------------------------
#[test]
fn test_driver_biometrics() {
    let biometrics = DriverBiometrics {
        heart_rate_bpm: 165,
        g_lateral: 4.8,
        g_longitudinal: -5.2,
        g_vertical: 1.1,
        core_temp_c: 38.6,
        hydration_ml_remaining: 450.0,
    };
    let encoded = encode_with_checksum(&biometrics).expect("encode biometrics");
    let (decoded, consumed): (DriverBiometrics, _) =
        decode_with_checksum(&encoded).expect("decode biometrics");
    assert_eq!(decoded, biometrics);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Full telemetry frame combining driver inputs and engine
// ---------------------------------------------------------------------------
#[test]
fn test_telemetry_frame() {
    let frame = TelemetryFrame {
        timestamp_ms: 1_710_453_600_000,
        car_number: 44,
        speed_kph: 315.8,
        driver_inputs: DriverInputs {
            steering_angle_deg: 1.2,
            throttle_pct: 100.0,
            brake_pct: 0.0,
            gear: 8,
            drs_active: true,
        },
        engine: EngineParameters {
            rpm: 12_800,
            mgu_k_harvesting_kw: 0.0,
            mgu_h_harvesting_kw: 45.0,
            ers_store_kj: 1_800.0,
            oil_temp_c: 135.2,
            water_temp_c: 110.3,
            fuel_flow_kg_per_hour: 100.0,
        },
    };
    let encoded = encode_with_checksum(&frame).expect("encode telemetry frame");
    let (decoded, consumed): (TelemetryFrame, _) =
        decode_with_checksum(&encoded).expect("decode telemetry frame");
    assert_eq!(decoded, frame);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Vector of lap timings (race stint)
// ---------------------------------------------------------------------------
#[test]
fn test_race_stint_lap_timings() {
    let stint: Vec<LapTiming> = (1..=5)
        .map(|lap| LapTiming {
            lap_number: lap,
            sectors: vec![
                SectorSplit {
                    sector: 1,
                    time_ms: 28_000 + u32::from(lap) * 15,
                    speed_trap_kph: 310.0 - f64::from(lap) * 0.3,
                    is_personal_best: lap == 1,
                },
                SectorSplit {
                    sector: 2,
                    time_ms: 33_500 + u32::from(lap) * 20,
                    speed_trap_kph: 285.0 - f64::from(lap) * 0.2,
                    is_personal_best: false,
                },
                SectorSplit {
                    sector: 3,
                    time_ms: 27_000 + u32::from(lap) * 10,
                    speed_trap_kph: 295.0 - f64::from(lap) * 0.4,
                    is_personal_best: false,
                },
            ],
            total_time_ms: 88_500 + u32::from(lap) * 45,
            gap_to_leader_ms: i32::from(lap) * 800,
            interval_ms: i32::from(lap) * 120,
        })
        .collect();
    let encoded = encode_with_checksum(&stint).expect("encode race stint");
    let (decoded, consumed): (Vec<LapTiming>, _) =
        decode_with_checksum(&encoded).expect("decode race stint");
    assert_eq!(decoded, stint);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Wet weather tire swap scenario
// ---------------------------------------------------------------------------
#[test]
fn test_wet_weather_tire_swap() {
    let wet_tire = |compound| TireTelemetry {
        compound,
        surface_temp_c: 65.0,
        inner_temp_c: 58.0,
        pressure_psi: 20.5,
        degradation_pct: 0.0,
        laps_on_tire: 0,
    };
    let set = TireSet {
        front_left: wet_tire(TireCompound::Intermediate),
        front_right: wet_tire(TireCompound::Intermediate),
        rear_left: wet_tire(TireCompound::Intermediate),
        rear_right: wet_tire(TireCompound::Intermediate),
    };
    let encoded = encode_with_checksum(&set).expect("encode wet tires");
    let (decoded, consumed): (TireSet, _) =
        decode_with_checksum(&encoded).expect("decode wet tires");
    assert_eq!(decoded, set);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Nested pit + weather combined strategy
// ---------------------------------------------------------------------------
#[test]
fn test_pit_weather_combined_strategy() {
    let data: (PitStopBreakdown, WeatherRadar) = (
        PitStopBreakdown {
            total_time_s: 3.12,
            jack_time_s: 0.18,
            front_left_wheel_s: 2.10,
            front_right_wheel_s: 2.05,
            rear_left_wheel_s: 2.15,
            rear_right_wheel_s: 2.08,
            fuel_time_s: 0.0,
            front_wing_adjust_s: 0.45,
            release_time_s: 0.25,
        },
        WeatherRadar {
            condition: WeatherCondition::LightRain,
            rain_probability_pct: 85.0,
            track_temp_c: 22.1,
            air_temp_c: 18.5,
            humidity_pct: 92.0,
            wind_speed_kph: 22.7,
            wind_direction_deg: 310.0,
            forecast_change_in_laps: 3,
        },
    );
    let encoded = encode_with_checksum(&data).expect("encode pit+weather");
    let (decoded, consumed): ((PitStopBreakdown, WeatherRadar), _) =
        decode_with_checksum(&encoded).expect("decode pit+weather");
    assert_eq!(decoded, data);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: Corruption detection — flip bit in payload
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_payload_bit_flip() {
    let frame = TelemetryFrame {
        timestamp_ms: 1_000_000,
        car_number: 1,
        speed_kph: 280.0,
        driver_inputs: DriverInputs {
            steering_angle_deg: 0.0,
            throttle_pct: 50.0,
            brake_pct: 0.0,
            gear: 6,
            drs_active: false,
        },
        engine: EngineParameters {
            rpm: 11_000,
            mgu_k_harvesting_kw: 80.0,
            mgu_h_harvesting_kw: 60.0,
            ers_store_kj: 2_500.0,
            oil_temp_c: 125.0,
            water_temp_c: 100.0,
            fuel_flow_kg_per_hour: 90.0,
        },
    };
    let mut encoded = encode_with_checksum(&frame).expect("encode frame for corruption");
    assert!(encoded.len() > HEADER_SIZE + 2);
    // Flip a bit in the payload region
    encoded[HEADER_SIZE + 1] ^= 0xFF;
    let result: Result<(TelemetryFrame, usize), _> = decode_with_checksum(&encoded);
    assert!(result.is_err(), "corrupted payload must be detected");
}

// ---------------------------------------------------------------------------
// Test 20: Corruption detection — truncated data
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_truncated_data() {
    let biometrics = DriverBiometrics {
        heart_rate_bpm: 180,
        g_lateral: 5.5,
        g_longitudinal: -4.0,
        g_vertical: 1.0,
        core_temp_c: 39.1,
        hydration_ml_remaining: 200.0,
    };
    let encoded = encode_with_checksum(&biometrics).expect("encode biometrics for truncation");
    let truncated = &encoded[..encoded.len() - 4];
    let result: Result<(DriverBiometrics, usize), _> = decode_with_checksum(truncated);
    assert!(result.is_err(), "truncated data must be rejected");
}

// ---------------------------------------------------------------------------
// Test 21: Corruption detection — modified checksum bytes
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_modified_checksum() {
    let fuel = FuelStrategy {
        current_load_kg: 55.0,
        target_finish_kg: 1.0,
        consumption_per_lap_kg: 1.5,
        laps_remaining: 35,
        fuel_delta_kg: 0.12,
    };
    let mut encoded = encode_with_checksum(&fuel).expect("encode fuel for checksum tampering");
    // CRC32 is at bytes 12..16 in the header
    encoded[12] ^= 0xAA;
    encoded[13] ^= 0x55;
    let result: Result<(FuelStrategy, usize), _> = decode_with_checksum(&encoded);
    assert!(result.is_err(), "modified checksum must be detected");
}

// ---------------------------------------------------------------------------
// Test 22: Corruption detection — zero-length payload after valid header
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_header_only() {
    let brakes = BrakeTemperatures {
        front_left_c: 900.0,
        front_right_c: 910.0,
        rear_left_c: 780.0,
        rear_right_c: 790.0,
        bias_pct_front: 56.0,
    };
    let encoded = encode_with_checksum(&brakes).expect("encode brakes for header-only test");
    // Keep only the header, discard the payload
    let header_only = &encoded[..HEADER_SIZE];
    let result: Result<(BrakeTemperatures, usize), _> = decode_with_checksum(header_only);
    assert!(result.is_err(), "header-only data must fail decoding");
}
