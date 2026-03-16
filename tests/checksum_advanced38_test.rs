//! Advanced checksum tests for OxiCode — exactly 22 top-level #[test] functions.
//! Theme: Elevator and vertical transportation systems.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced38_test

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
// Shared domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum TravelDirection {
    Up,
    Down,
    Idle,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DoorState {
    FullyClosed,
    Opening,
    FullyOpen,
    Closing,
    FaultedOpen,
    FaultedClosed,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SafetyCircuitState {
    Healthy,
    GovTripped,
    BufferTripped,
    DoorLockBroken,
    OvertravelTripped,
    EmergencyStop,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum EmergencyMode {
    Normal,
    FireService,
    SeismicOperation,
    PowerFailureRecall,
    IndependentService,
    InspectionMode,
    MedicalEmergency,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DispatchAlgorithm {
    Collective,
    DestinationDispatch,
    EtaBasedAllocation,
    ZonedDispatch,
    AiPredictive,
}

// ---------------------------------------------------------------------------
// Test 1: Car position and speed tracking
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct CarPositionSpeed {
    car_id: u16,
    floor_position_mm: i64,
    speed_mm_per_sec: i32,
    direction: TravelDirection,
    encoder_pulses: u64,
    deceleration_point_mm: i64,
    rated_speed_mm_per_sec: i32,
    is_releveling: bool,
}

#[test]
fn test_car_position_speed_roundtrip() {
    let value = CarPositionSpeed {
        car_id: 3,
        floor_position_mm: 45_720,
        speed_mm_per_sec: 2540,
        direction: TravelDirection::Up,
        encoder_pulses: 1_284_901,
        deceleration_point_mm: 48_000,
        rated_speed_mm_per_sec: 3050,
        is_releveling: false,
    };
    let encoded = encode_with_checksum(&value).expect("encode car position/speed failed");
    let (decoded, consumed): (CarPositionSpeed, _) =
        decode_with_checksum(&encoded).expect("decode car position/speed failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Door state tracking
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct DoorController {
    car_id: u16,
    front_door: DoorState,
    rear_door: DoorState,
    open_dwell_ms: u32,
    nudging_active: bool,
    reopen_count: u8,
    photo_eye_blocked: bool,
    door_motor_current_ma: u16,
    door_zone_sensor: bool,
}

#[test]
fn test_door_controller_roundtrip() {
    let value = DoorController {
        car_id: 1,
        front_door: DoorState::Opening,
        rear_door: DoorState::FullyClosed,
        open_dwell_ms: 3500,
        nudging_active: false,
        reopen_count: 2,
        photo_eye_blocked: true,
        door_motor_current_ma: 870,
        door_zone_sensor: true,
    };
    let encoded = encode_with_checksum(&value).expect("encode door controller failed");
    let (decoded, consumed): (DoorController, _) =
        decode_with_checksum(&encoded).expect("decode door controller failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: Hall call registration
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct HallCall {
    floor: u8,
    direction: TravelDirection,
    timestamp_epoch_ms: u64,
    vip_priority: bool,
    source_panel_id: u16,
}

#[test]
fn test_hall_call_roundtrip() {
    let calls = vec![
        HallCall {
            floor: 1,
            direction: TravelDirection::Up,
            timestamp_epoch_ms: 1_700_000_000_000,
            vip_priority: false,
            source_panel_id: 101,
        },
        HallCall {
            floor: 15,
            direction: TravelDirection::Down,
            timestamp_epoch_ms: 1_700_000_001_500,
            vip_priority: true,
            source_panel_id: 215,
        },
    ];
    let encoded = encode_with_checksum(&calls).expect("encode hall calls failed");
    let (decoded, consumed): (Vec<HallCall>, _) =
        decode_with_checksum(&encoded).expect("decode hall calls failed");
    assert_eq!(decoded, calls);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Car call registration
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct CarCall {
    car_id: u16,
    destination_floor: u8,
    timestamp_epoch_ms: u64,
    is_handicap: bool,
    is_firefighter: bool,
}

#[test]
fn test_car_call_roundtrip() {
    let calls = vec![
        CarCall {
            car_id: 2,
            destination_floor: 10,
            timestamp_epoch_ms: 1_700_000_005_000,
            is_handicap: true,
            is_firefighter: false,
        },
        CarCall {
            car_id: 2,
            destination_floor: 22,
            timestamp_epoch_ms: 1_700_000_005_200,
            is_handicap: false,
            is_firefighter: false,
        },
    ];
    let encoded = encode_with_checksum(&calls).expect("encode car calls failed");
    let (decoded, consumed): (Vec<CarCall>, _) =
        decode_with_checksum(&encoded).expect("decode car calls failed");
    assert_eq!(decoded, calls);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Dispatch algorithm configuration
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct DispatchConfig {
    algorithm: DispatchAlgorithm,
    group_size: u8,
    lobby_floor: u8,
    top_floor: u8,
    express_zone_start: u8,
    express_zone_end: u8,
    eta_weight: u16,
    energy_weight: u16,
    wait_time_limit_sec: u16,
}

#[test]
fn test_dispatch_config_roundtrip() {
    let value = DispatchConfig {
        algorithm: DispatchAlgorithm::DestinationDispatch,
        group_size: 6,
        lobby_floor: 1,
        top_floor: 52,
        express_zone_start: 30,
        express_zone_end: 52,
        eta_weight: 700,
        energy_weight: 300,
        wait_time_limit_sec: 45,
    };
    let encoded = encode_with_checksum(&value).expect("encode dispatch config failed");
    let (decoded, consumed): (DispatchConfig, _) =
        decode_with_checksum(&encoded).expect("decode dispatch config failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Load cell readings
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct LoadCellReading {
    car_id: u16,
    weight_kg_x100: i32,
    rated_capacity_kg: u16,
    overload_threshold_pct: u8,
    is_overloaded: bool,
    passenger_estimate: u8,
    tare_weight_kg_x100: i32,
    adc_raw_value: u32,
}

#[test]
fn test_load_cell_reading_roundtrip() {
    let value = LoadCellReading {
        car_id: 4,
        weight_kg_x100: 135_000,
        rated_capacity_kg: 1800,
        overload_threshold_pct: 80,
        is_overloaded: false,
        passenger_estimate: 12,
        tare_weight_kg_x100: 280_000,
        adc_raw_value: 0x00FF_A3B2,
    };
    let encoded = encode_with_checksum(&value).expect("encode load cell reading failed");
    let (decoded, consumed): (LoadCellReading, _) =
        decode_with_checksum(&encoded).expect("decode load cell reading failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Safety circuit states
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SafetyCircuitReport {
    car_id: u16,
    overall_state: SafetyCircuitState,
    governor_ok: bool,
    buffer_ok: bool,
    door_lock_chain_ok: bool,
    pit_switch_ok: bool,
    car_top_stop_ok: bool,
    overtravel_up_ok: bool,
    overtravel_down_ok: bool,
    safeties_set_at_epoch_ms: u64,
}

#[test]
fn test_safety_circuit_report_roundtrip() {
    let value = SafetyCircuitReport {
        car_id: 1,
        overall_state: SafetyCircuitState::Healthy,
        governor_ok: true,
        buffer_ok: true,
        door_lock_chain_ok: true,
        pit_switch_ok: true,
        car_top_stop_ok: true,
        overtravel_up_ok: true,
        overtravel_down_ok: true,
        safeties_set_at_epoch_ms: 1_700_000_010_000,
    };
    let encoded = encode_with_checksum(&value).expect("encode safety circuit report failed");
    let (decoded, consumed): (SafetyCircuitReport, _) =
        decode_with_checksum(&encoded).expect("decode safety circuit report failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Rope/belt tension measurements
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RopeTension {
    rope_index: u8,
    tension_newtons_x10: u32,
    nominal_tension_newtons_x10: u32,
    deviation_pct_x100: i16,
    rope_diameter_mm_x10: u16,
    lay_length_mm: u16,
    is_within_tolerance: bool,
}

#[test]
fn test_rope_tension_roundtrip() {
    let ropes = vec![
        RopeTension {
            rope_index: 0,
            tension_newtons_x10: 48_500,
            nominal_tension_newtons_x10: 50_000,
            deviation_pct_x100: -300,
            rope_diameter_mm_x10: 125,
            lay_length_mm: 200,
            is_within_tolerance: true,
        },
        RopeTension {
            rope_index: 1,
            tension_newtons_x10: 50_200,
            nominal_tension_newtons_x10: 50_000,
            deviation_pct_x100: 40,
            rope_diameter_mm_x10: 125,
            lay_length_mm: 200,
            is_within_tolerance: true,
        },
        RopeTension {
            rope_index: 2,
            tension_newtons_x10: 42_000,
            nominal_tension_newtons_x10: 50_000,
            deviation_pct_x100: -1600,
            rope_diameter_mm_x10: 124,
            lay_length_mm: 198,
            is_within_tolerance: false,
        },
    ];
    let encoded = encode_with_checksum(&ropes).expect("encode rope tensions failed");
    let (decoded, consumed): (Vec<RopeTension>, _) =
        decode_with_checksum(&encoded).expect("decode rope tensions failed");
    assert_eq!(decoded, ropes);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Drive controller parameters
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct DriveController {
    car_id: u16,
    motor_type: String,
    rated_power_kw_x10: u32,
    bus_voltage_v_x10: u32,
    motor_current_a_x100: i32,
    pwm_frequency_hz: u16,
    encoder_resolution_ppr: u32,
    regen_braking_enabled: bool,
    thermal_protection_celsius_x10: u16,
    fault_code: u16,
}

#[test]
fn test_drive_controller_roundtrip() {
    let value = DriveController {
        car_id: 2,
        motor_type: String::from("PMSM-Gearless"),
        rated_power_kw_x10: 220,
        bus_voltage_v_x10: 6_000,
        motor_current_a_x100: 15_400,
        pwm_frequency_hz: 8000,
        encoder_resolution_ppr: 4096,
        regen_braking_enabled: true,
        thermal_protection_celsius_x10: 850,
        fault_code: 0,
    };
    let encoded = encode_with_checksum(&value).expect("encode drive controller failed");
    let (decoded, consumed): (DriveController, _) =
        decode_with_checksum(&encoded).expect("decode drive controller failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Maintenance inspection record
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum InspectionResult {
    Pass,
    ConditionalPass(String),
    Fail(String),
    NotApplicable,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct InspectionItem {
    code: String,
    description: String,
    result: InspectionResult,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MaintenanceInspection {
    car_id: u16,
    inspector_id: String,
    date_yyyymmdd: u32,
    items: Vec<InspectionItem>,
    next_inspection_days: u16,
}

#[test]
fn test_maintenance_inspection_roundtrip() {
    let value = MaintenanceInspection {
        car_id: 5,
        inspector_id: String::from("TECH-4492"),
        date_yyyymmdd: 20260315,
        items: vec![
            InspectionItem {
                code: String::from("GOV-01"),
                description: String::from("Governor rope tension check"),
                result: InspectionResult::Pass,
            },
            InspectionItem {
                code: String::from("BUF-02"),
                description: String::from("Buffer oil level inspection"),
                result: InspectionResult::ConditionalPass(String::from(
                    "Oil slightly below nominal, refill recommended",
                )),
            },
            InspectionItem {
                code: String::from("DOR-05"),
                description: String::from("Door restrictor device test"),
                result: InspectionResult::Fail(String::from(
                    "Restrictor cable frayed; replace before next cycle",
                )),
            },
        ],
        next_inspection_days: 90,
    };
    let encoded = encode_with_checksum(&value).expect("encode maintenance inspection failed");
    let (decoded, consumed): (MaintenanceInspection, _) =
        decode_with_checksum(&encoded).expect("decode maintenance inspection failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Modernization project tracking
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum ModernizationPhase {
    Assessment,
    Engineering,
    Procurement,
    Installation,
    Testing,
    Handover,
    Complete,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ModernizationProject {
    project_id: String,
    building_name: String,
    car_ids: Vec<u16>,
    phase: ModernizationPhase,
    original_install_year: u16,
    scope_controller: bool,
    scope_door_operator: bool,
    scope_fixtures: bool,
    scope_ropes: bool,
    budget_cents: u64,
    spent_cents: u64,
}

#[test]
fn test_modernization_project_roundtrip() {
    let value = ModernizationProject {
        project_id: String::from("MOD-2026-0173"),
        building_name: String::from("Shibuya Sky Tower"),
        car_ids: vec![1, 2, 3, 4],
        phase: ModernizationPhase::Installation,
        original_install_year: 1998,
        scope_controller: true,
        scope_door_operator: true,
        scope_fixtures: true,
        scope_ropes: false,
        budget_cents: 2_400_000_00,
        spent_cents: 1_850_000_00,
    };
    let encoded = encode_with_checksum(&value).expect("encode modernization project failed");
    let (decoded, consumed): (ModernizationProject, _) =
        decode_with_checksum(&encoded).expect("decode modernization project failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Destination dispatch configuration
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct FloorZone {
    zone_name: String,
    floor_start: u8,
    floor_end: u8,
    assigned_car_ids: Vec<u16>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DestinationDispatchConfig {
    building_id: u32,
    zones: Vec<FloorZone>,
    lobby_kiosk_count: u8,
    max_wait_sec: u16,
    max_travel_sec: u16,
    allow_cross_zone: bool,
}

#[test]
fn test_destination_dispatch_config_roundtrip() {
    let value = DestinationDispatchConfig {
        building_id: 900_001,
        zones: vec![
            FloorZone {
                zone_name: String::from("Low-Rise"),
                floor_start: 2,
                floor_end: 15,
                assigned_car_ids: vec![1, 2],
            },
            FloorZone {
                zone_name: String::from("Mid-Rise"),
                floor_start: 16,
                floor_end: 30,
                assigned_car_ids: vec![3, 4],
            },
            FloorZone {
                zone_name: String::from("High-Rise"),
                floor_start: 31,
                floor_end: 52,
                assigned_car_ids: vec![5, 6],
            },
        ],
        lobby_kiosk_count: 8,
        max_wait_sec: 30,
        max_travel_sec: 90,
        allow_cross_zone: false,
    };
    let encoded = encode_with_checksum(&value).expect("encode destination dispatch config failed");
    let (decoded, consumed): (DestinationDispatchConfig, _) =
        decode_with_checksum(&encoded).expect("decode destination dispatch config failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Traffic pattern analysis
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum TrafficPattern {
    MorningUpPeak,
    EveningDownPeak,
    LunchTimeTwoWay,
    Interfloor,
    LowTraffic,
    SpecialEvent,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TrafficAnalysisSlot {
    hour: u8,
    minute: u8,
    pattern: TrafficPattern,
    total_trips: u32,
    avg_wait_sec_x10: u16,
    avg_travel_sec_x10: u16,
    peak_passengers_per_five_min: u16,
}

#[test]
fn test_traffic_analysis_roundtrip() {
    let slots = vec![
        TrafficAnalysisSlot {
            hour: 8,
            minute: 0,
            pattern: TrafficPattern::MorningUpPeak,
            total_trips: 342,
            avg_wait_sec_x10: 225,
            avg_travel_sec_x10: 480,
            peak_passengers_per_five_min: 47,
        },
        TrafficAnalysisSlot {
            hour: 12,
            minute: 0,
            pattern: TrafficPattern::LunchTimeTwoWay,
            total_trips: 218,
            avg_wait_sec_x10: 310,
            avg_travel_sec_x10: 550,
            peak_passengers_per_five_min: 33,
        },
        TrafficAnalysisSlot {
            hour: 17,
            minute: 30,
            pattern: TrafficPattern::EveningDownPeak,
            total_trips: 389,
            avg_wait_sec_x10: 195,
            avg_travel_sec_x10: 420,
            peak_passengers_per_five_min: 52,
        },
    ];
    let encoded = encode_with_checksum(&slots).expect("encode traffic analysis failed");
    let (decoded, consumed): (Vec<TrafficAnalysisSlot>, _) =
        decode_with_checksum(&encoded).expect("decode traffic analysis failed");
    assert_eq!(decoded, slots);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Emergency mode configuration
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct EmergencyConfig {
    car_id: u16,
    mode: EmergencyMode,
    recall_floor: u8,
    alternate_recall_floor: u8,
    fire_phase1_active: bool,
    fire_phase2_active: bool,
    seismic_sensor_threshold_gal: u16,
    backup_power_available: bool,
    fireman_key_inserted: bool,
    announcement_message_id: u16,
}

#[test]
fn test_emergency_config_roundtrip() {
    let value = EmergencyConfig {
        car_id: 3,
        mode: EmergencyMode::FireService,
        recall_floor: 1,
        alternate_recall_floor: 2,
        fire_phase1_active: true,
        fire_phase2_active: false,
        seismic_sensor_threshold_gal: 80,
        backup_power_available: true,
        fireman_key_inserted: true,
        announcement_message_id: 5001,
    };
    let encoded = encode_with_checksum(&value).expect("encode emergency config failed");
    let (decoded, consumed): (EmergencyConfig, _) =
        decode_with_checksum(&encoded).expect("decode emergency config failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Elevator group status snapshot
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ElevatorGroupStatus {
    group_id: u16,
    car_count: u8,
    cars_in_service: u8,
    cars_on_independent: u8,
    cars_on_inspection: u8,
    cars_out_of_service: u8,
    active_hall_calls: u16,
    active_car_calls: u16,
    current_algorithm: DispatchAlgorithm,
    energy_consumption_wh: u64,
}

#[test]
fn test_elevator_group_status_roundtrip() {
    let value = ElevatorGroupStatus {
        group_id: 1,
        car_count: 6,
        cars_in_service: 5,
        cars_on_independent: 0,
        cars_on_inspection: 1,
        cars_out_of_service: 0,
        active_hall_calls: 12,
        active_car_calls: 8,
        current_algorithm: DispatchAlgorithm::EtaBasedAllocation,
        energy_consumption_wh: 45_320,
    };
    let encoded = encode_with_checksum(&value).expect("encode group status failed");
    let (decoded, consumed): (ElevatorGroupStatus, _) =
        decode_with_checksum(&encoded).expect("decode group status failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Counterweight and balance factor
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct CounterweightSpec {
    car_id: u16,
    car_weight_kg: u32,
    counterweight_kg: u32,
    balance_factor_pct_x10: u16,
    rated_load_kg: u16,
    compensation_chain_kg_per_m_x100: u16,
    travel_height_mm: u64,
    sheave_diameter_mm: u16,
}

#[test]
fn test_counterweight_spec_roundtrip() {
    let value = CounterweightSpec {
        car_id: 2,
        car_weight_kg: 3200,
        counterweight_kg: 4520,
        balance_factor_pct_x10: 420,
        rated_load_kg: 2500,
        compensation_chain_kg_per_m_x100: 345,
        travel_height_mm: 156_000,
        sheave_diameter_mm: 640,
    };
    let encoded = encode_with_checksum(&value).expect("encode counterweight spec failed");
    let (decoded, consumed): (CounterweightSpec, _) =
        decode_with_checksum(&encoded).expect("decode counterweight spec failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Pit and overhead clearances
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ShaftClearances {
    car_id: u16,
    pit_depth_mm: u32,
    overhead_clearance_mm: u32,
    counterweight_runby_mm: u32,
    car_top_refuge_space_mm: u32,
    buffer_stroke_mm: u16,
    buffer_type: String,
    code_compliant: bool,
}

#[test]
fn test_shaft_clearances_roundtrip() {
    let value = ShaftClearances {
        car_id: 1,
        pit_depth_mm: 1800,
        overhead_clearance_mm: 4200,
        counterweight_runby_mm: 300,
        car_top_refuge_space_mm: 1100,
        buffer_stroke_mm: 225,
        buffer_type: String::from("Polyurethane-Spring"),
        code_compliant: true,
    };
    let encoded = encode_with_checksum(&value).expect("encode shaft clearances failed");
    let (decoded, consumed): (ShaftClearances, _) =
        decode_with_checksum(&encoded).expect("decode shaft clearances failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Ride quality measurement
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RideQualityMeasurement {
    car_id: u16,
    measurement_date_yyyymmdd: u32,
    peak_accel_mg: i16,
    peak_jerk_mg_per_sec: i16,
    vibration_lateral_mg: i16,
    vibration_longitudinal_mg: i16,
    noise_level_dba_x10: u16,
    start_comfort_score_x10: u16,
    stop_comfort_score_x10: u16,
    overall_score_x10: u16,
}

#[test]
fn test_ride_quality_measurement_roundtrip() {
    let value = RideQualityMeasurement {
        car_id: 4,
        measurement_date_yyyymmdd: 20260310,
        peak_accel_mg: 12,
        peak_jerk_mg_per_sec: 8,
        vibration_lateral_mg: 5,
        vibration_longitudinal_mg: 3,
        noise_level_dba_x10: 420,
        start_comfort_score_x10: 88,
        stop_comfort_score_x10: 92,
        overall_score_x10: 90,
    };
    let encoded = encode_with_checksum(&value).expect("encode ride quality failed");
    let (decoded, consumed): (RideQualityMeasurement, _) =
        decode_with_checksum(&encoded).expect("decode ride quality failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: Intercom and communication system
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum IntercomState {
    Idle,
    Ringing,
    Connected,
    OnHold,
    Disconnected,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IntercomSystem {
    car_id: u16,
    state: IntercomState,
    auto_dial_number: String,
    last_test_epoch_sec: u64,
    last_test_passed: bool,
    battery_backup_pct: u8,
    call_duration_sec: u32,
    microphone_ok: bool,
    speaker_ok: bool,
}

#[test]
fn test_intercom_system_roundtrip() {
    let value = IntercomSystem {
        car_id: 6,
        state: IntercomState::Idle,
        auto_dial_number: String::from("+81-3-5555-0199"),
        last_test_epoch_sec: 1_710_000_000,
        last_test_passed: true,
        battery_backup_pct: 95,
        call_duration_sec: 0,
        microphone_ok: true,
        speaker_ok: true,
    };
    let encoded = encode_with_checksum(&value).expect("encode intercom system failed");
    let (decoded, consumed): (IntercomSystem, _) =
        decode_with_checksum(&encoded).expect("decode intercom system failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: Landing door assembly
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct LandingDoor {
    floor: u8,
    interlock_healthy: bool,
    hanger_roller_wear_pct: u8,
    sill_gap_mm_x10: u16,
    closer_spring_tension_n_x10: u16,
    gibs_clearance_mm_x10: u16,
    fire_rating_minutes: u16,
    vision_panel_installed: bool,
}

#[test]
fn test_landing_door_assembly_roundtrip() {
    let doors: Vec<LandingDoor> = (1..=5)
        .map(|floor| LandingDoor {
            floor,
            interlock_healthy: floor != 3,
            hanger_roller_wear_pct: 10 + floor * 5,
            sill_gap_mm_x10: 60 + u16::from(floor) * 2,
            closer_spring_tension_n_x10: 450,
            gibs_clearance_mm_x10: 30,
            fire_rating_minutes: 120,
            vision_panel_installed: floor == 1,
        })
        .collect();
    let encoded = encode_with_checksum(&doors).expect("encode landing doors failed");
    let (decoded, consumed): (Vec<LandingDoor>, _) =
        decode_with_checksum(&encoded).expect("decode landing doors failed");
    assert_eq!(decoded, doors);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 21: Energy consumption report
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct EnergyReport {
    car_id: u16,
    report_date_yyyymmdd: u32,
    total_trips: u32,
    motor_energy_wh: u64,
    regen_energy_wh: u64,
    lighting_energy_wh: u32,
    ventilation_energy_wh: u32,
    standby_energy_wh: u32,
    doors_energy_wh: u32,
    controller_energy_wh: u32,
    net_energy_wh: u64,
}

#[test]
fn test_energy_report_roundtrip() {
    let value = EnergyReport {
        car_id: 1,
        report_date_yyyymmdd: 20260314,
        total_trips: 487,
        motor_energy_wh: 128_400,
        regen_energy_wh: 34_200,
        lighting_energy_wh: 8_600,
        ventilation_energy_wh: 4_300,
        standby_energy_wh: 12_100,
        doors_energy_wh: 2_700,
        controller_energy_wh: 5_900,
        net_energy_wh: 127_800,
    };
    let encoded = encode_with_checksum(&value).expect("encode energy report failed");
    let (decoded, consumed): (EnergyReport, _) =
        decode_with_checksum(&encoded).expect("decode energy report failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: Comprehensive elevator configuration bundle
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ElevatorConfigBundle {
    building_code: String,
    car_id: u16,
    manufacturer: String,
    model: String,
    serial_number: String,
    install_year: u16,
    floors_served: Vec<u8>,
    rated_speed_mm_per_sec: u32,
    rated_capacity_kg: u16,
    car_dimensions_mm: (u32, u32, u32),
    door_width_mm: u16,
    door_height_mm: u16,
    travel_mm: u64,
    drive_type: String,
    roping_ratio: u8,
    num_ropes: u8,
    has_rear_door: bool,
    has_destination_dispatch: bool,
    emergency_mode: EmergencyMode,
    firmware_version: String,
}

#[test]
fn test_elevator_config_bundle_roundtrip() {
    let value = ElevatorConfigBundle {
        building_code: String::from("TKY-MARU-2026"),
        car_id: 3,
        manufacturer: String::from("CoolJapan Elevator Co."),
        model: String::from("SkyRise-XG7"),
        serial_number: String::from("SR-XG7-00024819"),
        install_year: 2025,
        floors_served: vec![1, 2, 3, 5, 6, 7, 8, 9, 10, 15, 20, 25, 30, 35, 40, 45, 50],
        rated_speed_mm_per_sec: 7000,
        rated_capacity_kg: 1600,
        car_dimensions_mm: (1600, 1500, 2700),
        door_width_mm: 900,
        door_height_mm: 2100,
        travel_mm: 210_000,
        drive_type: String::from("PMSM-Gearless-Regen"),
        roping_ratio: 2,
        num_ropes: 8,
        has_rear_door: false,
        has_destination_dispatch: true,
        emergency_mode: EmergencyMode::Normal,
        firmware_version: String::from("4.7.2-rc1"),
    };
    let encoded = encode_with_checksum(&value).expect("encode elevator config bundle failed");
    let (decoded, consumed): (ElevatorConfigBundle, _) =
        decode_with_checksum(&encoded).expect("decode elevator config bundle failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}
