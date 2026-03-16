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

// ── Domain types: Elevator & Vertical Transportation Systems ────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DoorStatus {
    Open,
    Closing,
    Closed,
    Opening,
    Blocked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TravelDirection {
    Up,
    Down,
    Idle,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DispatchAlgorithm {
    CollectiveSelective,
    DestinationDispatch,
    NearestCar,
    ZoneBased,
    AiPredictive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrafficPattern {
    UpPeak,
    DownPeak,
    Interfloor,
    LunchRush,
    LowTraffic,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SafetyComponent {
    Governor,
    Buffer,
    SafetyGear,
    DoorInterlock,
    OverspeedDetector,
    SlackRopeSwitch,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OperatingMode {
    Normal,
    FirefighterService,
    EarthquakeMode,
    IndependentService,
    InspectionMode,
    OutOfService,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EscalatorState {
    Running,
    Stopped,
    SlowDown,
    Reversing,
    Fault,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ElevatorCarPosition {
    car_id: u32,
    shaft_id: u16,
    floor_number: i16,
    position_mm: i64,
    direction: TravelDirection,
    door_status: DoorStatus,
    speed_mm_per_sec: u32,
    acceleration_mm_per_sec2: i32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MotorDriveParameters {
    drive_id: u32,
    car_id: u32,
    vfd_frequency_hz_x100: u32,
    torque_nm_x100: i32,
    current_amps_x100: u32,
    voltage_v_x10: u32,
    power_kw_x100: i32,
    motor_temp_c_x10: i16,
    brake_engaged: bool,
    regenerating: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoadCellReading {
    car_id: u32,
    total_weight_kg_x10: u32,
    estimated_passengers: u8,
    capacity_pct: u8,
    overload_warning: bool,
    balance_offset_kg_x10: i16,
    reading_timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SafetyDiagnostic {
    car_id: u32,
    component: SafetyComponent,
    healthy: bool,
    last_test_timestamp: u64,
    trip_count: u32,
    next_inspection_epoch: u64,
    fault_code: u16,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceSchedule {
    schedule_id: u64,
    car_id: u32,
    task_name: String,
    interval_days: u16,
    last_performed_epoch: u64,
    next_due_epoch: u64,
    technician_id: u32,
    rope_wear_pct_x10: u16,
    lubrication_level_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrafficAnalysis {
    building_id: u32,
    analysis_window_sec: u32,
    pattern: TrafficPattern,
    total_trips: u32,
    avg_wait_time_sec_x10: u32,
    avg_travel_time_sec_x10: u32,
    peak_floor: i16,
    hall_calls_up: u32,
    hall_calls_down: u32,
    car_calls: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DestinationDispatchGroup {
    group_id: u32,
    building_id: u32,
    algorithm: DispatchAlgorithm,
    car_ids: Vec<u32>,
    pending_requests: u16,
    avg_response_time_sec_x10: u32,
    zone_low_floor: i16,
    zone_high_floor: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EscalatorStepChainMonitor {
    escalator_id: u32,
    state: EscalatorState,
    chain_tension_n_x10: u32,
    step_speed_mm_per_sec: u32,
    handrail_speed_mm_per_sec: u32,
    missing_steps: u8,
    comb_plate_ok: bool,
    running_hours: u64,
    vibration_level_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModernizationProject {
    project_id: u64,
    building_id: u32,
    original_install_year: u16,
    target_completion_epoch: u64,
    controller_upgraded: bool,
    door_operator_upgraded: bool,
    fixtures_upgraded: bool,
    machine_replaced: bool,
    budget_cents: u64,
    spent_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyRegeneration {
    car_id: u32,
    period_start_epoch: u64,
    period_end_epoch: u64,
    consumed_kwh_x100: u64,
    regenerated_kwh_x100: u64,
    net_kwh_x100: i64,
    trip_count: u32,
    regen_efficiency_pct_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EarthquakeModeActivation {
    event_id: u64,
    building_id: u32,
    seismic_intensity_x100: u32,
    p_wave_detected_epoch: u64,
    s_wave_detected_epoch: u64,
    cars_halted: Vec<u32>,
    nearest_floor_stops: Vec<i16>,
    all_doors_opened: bool,
    reset_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FirefighterServiceRecord {
    record_id: u64,
    car_id: u32,
    mode: OperatingMode,
    activation_epoch: u64,
    recall_floor: i16,
    phase: u8,
    key_switch_on: bool,
    deactivation_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IoTSensorTelemetry {
    sensor_id: u64,
    car_id: u32,
    temperature_c_x10: i16,
    humidity_pct_x10: u16,
    co2_ppm: u16,
    noise_db_x10: u16,
    light_lux: u32,
    door_cycle_count: u64,
    vibration_x_x1000: i32,
    vibration_y_x1000: i32,
    vibration_z_x1000: i32,
    timestamp_ms: u64,
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_elevator_car_position_ascending_roundtrip() {
    let pos = ElevatorCarPosition {
        car_id: 1,
        shaft_id: 1,
        floor_number: 12,
        position_mm: 42_000,
        direction: TravelDirection::Up,
        door_status: DoorStatus::Closed,
        speed_mm_per_sec: 3500,
        acceleration_mm_per_sec2: 1200,
        timestamp_ms: 1_710_000_000_000,
    };
    let bytes = encode_to_vec(&pos).expect("encode ElevatorCarPosition ascending failed");
    let (decoded, consumed) = decode_from_slice::<ElevatorCarPosition>(&bytes)
        .expect("decode ElevatorCarPosition ascending failed");
    assert_eq!(pos, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_elevator_car_position_versioned_v1_0_0() {
    let pos = ElevatorCarPosition {
        car_id: 2,
        shaft_id: 3,
        floor_number: -2,
        position_mm: -7_000,
        direction: TravelDirection::Down,
        door_status: DoorStatus::Closed,
        speed_mm_per_sec: 2500,
        acceleration_mm_per_sec2: -800,
        timestamp_ms: 1_710_000_500_000,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&pos, version)
        .expect("encode versioned ElevatorCarPosition v1.0.0 failed");
    let (decoded, ver, _consumed): (ElevatorCarPosition, Version, usize) =
        decode_versioned_value::<ElevatorCarPosition>(&bytes)
            .expect("decode versioned ElevatorCarPosition v1.0.0 failed");
    assert_eq!(pos, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_door_status_blocked_with_opening_transition() {
    let pos = ElevatorCarPosition {
        car_id: 4,
        shaft_id: 2,
        floor_number: 7,
        position_mm: 24_500,
        direction: TravelDirection::Idle,
        door_status: DoorStatus::Blocked,
        speed_mm_per_sec: 0,
        acceleration_mm_per_sec2: 0,
        timestamp_ms: 1_710_001_000_000,
    };
    let bytes = encode_to_vec(&pos).expect("encode DoorStatus Blocked failed");
    let (decoded, consumed) =
        decode_from_slice::<ElevatorCarPosition>(&bytes).expect("decode DoorStatus Blocked failed");
    assert_eq!(pos, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_motor_drive_parameters_regenerating_roundtrip() {
    let drive = MotorDriveParameters {
        drive_id: 101,
        car_id: 1,
        vfd_frequency_hz_x100: 5000,
        torque_nm_x100: -15000,
        current_amps_x100: 8500,
        voltage_v_x10: 3800,
        power_kw_x100: -12500,
        motor_temp_c_x10: 650,
        brake_engaged: false,
        regenerating: true,
    };
    let bytes = encode_to_vec(&drive).expect("encode MotorDriveParameters regen failed");
    let (decoded, consumed) = decode_from_slice::<MotorDriveParameters>(&bytes)
        .expect("decode MotorDriveParameters regen failed");
    assert_eq!(drive, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_motor_drive_versioned_v2_1_0() {
    let drive = MotorDriveParameters {
        drive_id: 202,
        car_id: 3,
        vfd_frequency_hz_x100: 6000,
        torque_nm_x100: 22000,
        current_amps_x100: 12000,
        voltage_v_x10: 4000,
        power_kw_x100: 45000,
        motor_temp_c_x10: 780,
        brake_engaged: false,
        regenerating: false,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&drive, version)
        .expect("encode versioned MotorDriveParameters v2.1.0 failed");
    let (decoded, ver, consumed): (MotorDriveParameters, Version, usize) =
        decode_versioned_value::<MotorDriveParameters>(&bytes)
            .expect("decode versioned MotorDriveParameters v2.1.0 failed");
    assert_eq!(drive, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_load_cell_overload_warning_roundtrip() {
    let reading = LoadCellReading {
        car_id: 1,
        total_weight_kg_x10: 16_500,
        estimated_passengers: 21,
        capacity_pct: 110,
        overload_warning: true,
        balance_offset_kg_x10: 350,
        reading_timestamp_ms: 1_710_002_000_000,
    };
    let bytes = encode_to_vec(&reading).expect("encode LoadCellReading overload failed");
    let (decoded, consumed) = decode_from_slice::<LoadCellReading>(&bytes)
        .expect("decode LoadCellReading overload failed");
    assert_eq!(reading, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_safety_diagnostic_governor_versioned_v1_5_0() {
    let diag = SafetyDiagnostic {
        car_id: 2,
        component: SafetyComponent::Governor,
        healthy: true,
        last_test_timestamp: 1_709_000_000,
        trip_count: 0,
        next_inspection_epoch: 1_725_000_000,
        fault_code: 0,
        description: "Governor rope tension within spec, sheave groove wear nominal".to_string(),
    };
    let version = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&diag, version)
        .expect("encode versioned SafetyDiagnostic Governor v1.5.0 failed");
    let (decoded, ver, _consumed): (SafetyDiagnostic, Version, usize) =
        decode_versioned_value::<SafetyDiagnostic>(&bytes)
            .expect("decode versioned SafetyDiagnostic Governor v1.5.0 failed");
    assert_eq!(diag, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_safety_diagnostic_buffer_fault_roundtrip() {
    let diag = SafetyDiagnostic {
        car_id: 5,
        component: SafetyComponent::Buffer,
        healthy: false,
        last_test_timestamp: 1_708_500_000,
        trip_count: 2,
        next_inspection_epoch: 1_710_000_000,
        fault_code: 4012,
        description: "Oil buffer low fluid level detected, hydraulic seal degradation".to_string(),
    };
    let bytes = encode_to_vec(&diag).expect("encode SafetyDiagnostic Buffer fault failed");
    let (decoded, consumed) = decode_from_slice::<SafetyDiagnostic>(&bytes)
        .expect("decode SafetyDiagnostic Buffer fault failed");
    assert_eq!(diag, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_maintenance_schedule_rope_inspection_roundtrip() {
    let schedule = MaintenanceSchedule {
        schedule_id: 50001,
        car_id: 3,
        task_name: "Wire rope inspection and measurement".to_string(),
        interval_days: 180,
        last_performed_epoch: 1_700_000_000,
        next_due_epoch: 1_715_552_000,
        technician_id: 7042,
        rope_wear_pct_x10: 185,
        lubrication_level_pct: 72,
    };
    let bytes = encode_to_vec(&schedule).expect("encode MaintenanceSchedule rope failed");
    let (decoded, consumed) = decode_from_slice::<MaintenanceSchedule>(&bytes)
        .expect("decode MaintenanceSchedule rope failed");
    assert_eq!(schedule, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_maintenance_schedule_versioned_v3_0_0() {
    let schedule = MaintenanceSchedule {
        schedule_id: 50002,
        car_id: 1,
        task_name: "Guide shoe lubrication and rail alignment".to_string(),
        interval_days: 90,
        last_performed_epoch: 1_705_000_000,
        next_due_epoch: 1_712_776_000,
        technician_id: 3021,
        rope_wear_pct_x10: 45,
        lubrication_level_pct: 95,
    };
    let version = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&schedule, version)
        .expect("encode versioned MaintenanceSchedule v3.0.0 failed");
    let (decoded, ver, consumed): (MaintenanceSchedule, Version, usize) =
        decode_versioned_value::<MaintenanceSchedule>(&bytes)
            .expect("decode versioned MaintenanceSchedule v3.0.0 failed");
    assert_eq!(schedule, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_traffic_analysis_up_peak_roundtrip() {
    let analysis = TrafficAnalysis {
        building_id: 100,
        analysis_window_sec: 300,
        pattern: TrafficPattern::UpPeak,
        total_trips: 87,
        avg_wait_time_sec_x10: 245,
        avg_travel_time_sec_x10: 380,
        peak_floor: 1,
        hall_calls_up: 142,
        hall_calls_down: 11,
        car_calls: 310,
    };
    let bytes = encode_to_vec(&analysis).expect("encode TrafficAnalysis UpPeak failed");
    let (decoded, consumed) =
        decode_from_slice::<TrafficAnalysis>(&bytes).expect("decode TrafficAnalysis UpPeak failed");
    assert_eq!(analysis, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_destination_dispatch_group_ai_predictive_versioned_v2_0_0() {
    let group = DestinationDispatchGroup {
        group_id: 1,
        building_id: 100,
        algorithm: DispatchAlgorithm::AiPredictive,
        car_ids: vec![1, 2, 3, 4, 5, 6],
        pending_requests: 14,
        avg_response_time_sec_x10: 187,
        zone_low_floor: 1,
        zone_high_floor: 50,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&group, version)
        .expect("encode versioned DestinationDispatchGroup v2.0.0 failed");
    let (decoded, ver, _consumed): (DestinationDispatchGroup, Version, usize) =
        decode_versioned_value::<DestinationDispatchGroup>(&bytes)
            .expect("decode versioned DestinationDispatchGroup v2.0.0 failed");
    assert_eq!(group, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
}

#[test]
fn test_destination_dispatch_zone_based_roundtrip() {
    let group = DestinationDispatchGroup {
        group_id: 2,
        building_id: 200,
        algorithm: DispatchAlgorithm::ZoneBased,
        car_ids: vec![10, 11, 12],
        pending_requests: 3,
        avg_response_time_sec_x10: 320,
        zone_low_floor: 20,
        zone_high_floor: 40,
    };
    let bytes = encode_to_vec(&group).expect("encode DestinationDispatchGroup ZoneBased failed");
    let (decoded, consumed) = decode_from_slice::<DestinationDispatchGroup>(&bytes)
        .expect("decode DestinationDispatchGroup ZoneBased failed");
    assert_eq!(group, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_escalator_step_chain_fault_roundtrip() {
    let monitor = EscalatorStepChainMonitor {
        escalator_id: 301,
        state: EscalatorState::Fault,
        chain_tension_n_x10: 98_500,
        step_speed_mm_per_sec: 0,
        handrail_speed_mm_per_sec: 0,
        missing_steps: 1,
        comb_plate_ok: false,
        running_hours: 45_000,
        vibration_level_x100: 8500,
    };
    let bytes = encode_to_vec(&monitor).expect("encode EscalatorStepChainMonitor Fault failed");
    let (decoded, consumed) = decode_from_slice::<EscalatorStepChainMonitor>(&bytes)
        .expect("decode EscalatorStepChainMonitor Fault failed");
    assert_eq!(monitor, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_escalator_step_chain_versioned_v1_2_3() {
    let monitor = EscalatorStepChainMonitor {
        escalator_id: 302,
        state: EscalatorState::Running,
        chain_tension_n_x10: 75_000,
        step_speed_mm_per_sec: 500,
        handrail_speed_mm_per_sec: 510,
        missing_steps: 0,
        comb_plate_ok: true,
        running_hours: 12_300,
        vibration_level_x100: 1200,
    };
    let version = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&monitor, version)
        .expect("encode versioned EscalatorStepChainMonitor v1.2.3 failed");
    let (decoded, ver, consumed): (EscalatorStepChainMonitor, Version, usize) =
        decode_versioned_value::<EscalatorStepChainMonitor>(&bytes)
            .expect("decode versioned EscalatorStepChainMonitor v1.2.3 failed");
    assert_eq!(monitor, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_modernization_project_partial_upgrade_roundtrip() {
    let project = ModernizationProject {
        project_id: 90001,
        building_id: 100,
        original_install_year: 1985,
        target_completion_epoch: 1_720_000_000,
        controller_upgraded: true,
        door_operator_upgraded: true,
        fixtures_upgraded: false,
        machine_replaced: false,
        budget_cents: 450_000_00,
        spent_cents: 280_000_00,
    };
    let bytes = encode_to_vec(&project).expect("encode ModernizationProject partial failed");
    let (decoded, consumed) = decode_from_slice::<ModernizationProject>(&bytes)
        .expect("decode ModernizationProject partial failed");
    assert_eq!(project, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_energy_regeneration_versioned_v4_0_1() {
    let regen = EnergyRegeneration {
        car_id: 2,
        period_start_epoch: 1_709_000_000,
        period_end_epoch: 1_709_086_400,
        consumed_kwh_x100: 125_000,
        regenerated_kwh_x100: 37_500,
        net_kwh_x100: 87_500,
        trip_count: 340,
        regen_efficiency_pct_x10: 300,
    };
    let version = Version::new(4, 0, 1);
    let bytes = encode_versioned_value(&regen, version)
        .expect("encode versioned EnergyRegeneration v4.0.1 failed");
    let (decoded, ver, _consumed): (EnergyRegeneration, Version, usize) =
        decode_versioned_value::<EnergyRegeneration>(&bytes)
            .expect("decode versioned EnergyRegeneration v4.0.1 failed");
    assert_eq!(regen, decoded);
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 1);
}

#[test]
fn test_earthquake_mode_activation_with_car_list_roundtrip() {
    let activation = EarthquakeModeActivation {
        event_id: 77001,
        building_id: 100,
        seismic_intensity_x100: 450,
        p_wave_detected_epoch: 1_710_500_000,
        s_wave_detected_epoch: 1_710_500_008,
        cars_halted: vec![1, 2, 3, 4, 5, 6],
        nearest_floor_stops: vec![5, 12, 8, 1, 20, 15],
        all_doors_opened: true,
        reset_epoch: None,
    };
    let bytes = encode_to_vec(&activation).expect("encode EarthquakeModeActivation failed");
    let (decoded, consumed) = decode_from_slice::<EarthquakeModeActivation>(&bytes)
        .expect("decode EarthquakeModeActivation failed");
    assert_eq!(activation, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_earthquake_mode_versioned_v1_0_0_with_reset() {
    let activation = EarthquakeModeActivation {
        event_id: 77002,
        building_id: 200,
        seismic_intensity_x100: 320,
        p_wave_detected_epoch: 1_711_000_000,
        s_wave_detected_epoch: 1_711_000_005,
        cars_halted: vec![10, 11],
        nearest_floor_stops: vec![3, 7],
        all_doors_opened: true,
        reset_epoch: Some(1_711_003_600),
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&activation, version)
        .expect("encode versioned EarthquakeModeActivation v1.0.0 failed");
    let (decoded, ver, consumed): (EarthquakeModeActivation, Version, usize) =
        decode_versioned_value::<EarthquakeModeActivation>(&bytes)
            .expect("decode versioned EarthquakeModeActivation v1.0.0 failed");
    assert_eq!(activation, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert!(consumed > 0);
}

#[test]
fn test_firefighter_service_phase2_versioned_v2_3_0() {
    let record = FirefighterServiceRecord {
        record_id: 88001,
        car_id: 1,
        mode: OperatingMode::FirefighterService,
        activation_epoch: 1_710_100_000,
        recall_floor: 1,
        phase: 2,
        key_switch_on: true,
        deactivation_epoch: None,
    };
    let version = Version::new(2, 3, 0);
    let bytes = encode_versioned_value(&record, version)
        .expect("encode versioned FirefighterServiceRecord v2.3.0 failed");
    let (decoded, ver, _consumed): (FirefighterServiceRecord, Version, usize) =
        decode_versioned_value::<FirefighterServiceRecord>(&bytes)
            .expect("decode versioned FirefighterServiceRecord v2.3.0 failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_iot_sensor_telemetry_full_payload_roundtrip() {
    let telemetry = IoTSensorTelemetry {
        sensor_id: 600_001,
        car_id: 4,
        temperature_c_x10: 238,
        humidity_pct_x10: 455,
        co2_ppm: 820,
        noise_db_x10: 625,
        light_lux: 350,
        door_cycle_count: 1_234_567,
        vibration_x_x1000: 45,
        vibration_y_x1000: -12,
        vibration_z_x1000: 9810,
        timestamp_ms: 1_710_003_000_000,
    };
    let bytes = encode_to_vec(&telemetry).expect("encode IoTSensorTelemetry failed");
    let (decoded, consumed) =
        decode_from_slice::<IoTSensorTelemetry>(&bytes).expect("decode IoTSensorTelemetry failed");
    assert_eq!(telemetry, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_iot_sensor_telemetry_versioned_v5_2_0_version_upgrade() {
    let telemetry = IoTSensorTelemetry {
        sensor_id: 600_002,
        car_id: 6,
        temperature_c_x10: 310,
        humidity_pct_x10: 680,
        co2_ppm: 1100,
        noise_db_x10: 720,
        light_lux: 200,
        door_cycle_count: 2_500_000,
        vibration_x_x1000: -85,
        vibration_y_x1000: 130,
        vibration_z_x1000: 9795,
        timestamp_ms: 1_710_004_000_000,
    };
    let v1 = Version::new(5, 2, 0);
    let bytes_v1 = encode_versioned_value(&telemetry, v1)
        .expect("encode versioned IoTSensorTelemetry v5.2.0 failed");
    let (decoded_v1, ver_v1, _consumed_v1): (IoTSensorTelemetry, Version, usize) =
        decode_versioned_value::<IoTSensorTelemetry>(&bytes_v1)
            .expect("decode versioned IoTSensorTelemetry v5.2.0 failed");
    assert_eq!(telemetry, decoded_v1);
    assert_eq!(ver_v1.major, 5);
    assert_eq!(ver_v1.minor, 2);

    let v2 = Version::new(6, 0, 0);
    let bytes_v2 = encode_versioned_value(&decoded_v1, v2)
        .expect("re-encode IoTSensorTelemetry at v6.0.0 failed");
    let (decoded_v2, ver_v2, consumed_v2): (IoTSensorTelemetry, Version, usize) =
        decode_versioned_value::<IoTSensorTelemetry>(&bytes_v2)
            .expect("decode IoTSensorTelemetry v6.0.0 upgrade failed");
    assert_eq!(telemetry, decoded_v2);
    assert_eq!(ver_v2.major, 6);
    assert_eq!(ver_v2.minor, 0);
    assert!(consumed_v2 > 0);
}
