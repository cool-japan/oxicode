#![cfg(feature = "versioning")]

//! Railway signaling and train control systems (ERTMS/ETCS) versioning tests.
//!
//! 22 test functions covering track circuit states, signal aspects,
//! interlocking routes, movement authorities, balise telegrams, point machines,
//! axle counters, level crossings, timetable allocations, speed restrictions,
//! and more.

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

// ── Domain enums ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrackCircuitState {
    Clear,
    Occupied,
    Disturbed,
    Shunting,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SignalAspect {
    Red,
    Yellow,
    DoubleYellow,
    Green,
    FlashingYellow,
    FlashingGreen,
    Lunar,
    Extinguished,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PointPosition {
    Normal,
    Reverse,
    Moving,
    Failed,
    Undetected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BarrierState {
    Open,
    Closing,
    Closed,
    Opening,
    Blocked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EtcsLevel {
    Level0,
    Level1,
    Level2,
    Level3,
    Stm,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EtcsMode {
    FullSupervision,
    OnSight,
    StaffResponsible,
    Shunting,
    Unfitted,
    Trip,
    PostTrip,
    Standby,
    Isolation,
    SystemFailure,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RouteStatus {
    Free,
    Requested,
    Locked,
    Set,
    Occupied,
    Released,
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpeedRestrictionType {
    Permanent,
    Temporary,
    Emergency,
    Conditional,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrainCategory {
    HighSpeed,
    Intercity,
    Regional,
    Freight,
    Empty,
    Engineering,
}

// ── Domain structs ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrackCircuit {
    circuit_id: u32,
    section_name: String,
    state: TrackCircuitState,
    length_m: u32,
    max_speed_kmh: u16,
    last_update_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SignalHead {
    signal_id: u32,
    name: String,
    aspect: SignalAspect,
    milepost_m: u32,
    route_indicator: Option<String>,
    is_permissive: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InterlockingRoute {
    route_id: u64,
    entry_signal: String,
    exit_signal: String,
    status: RouteStatus,
    points: Vec<PointSetting>,
    conflicting_route_ids: Vec<u64>,
    overlap_length_m: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PointSetting {
    point_id: u32,
    required_position: PointPosition,
    detection_confirmed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MovementAuthority {
    ma_id: u64,
    train_id: String,
    start_milepost_m: u32,
    end_milepost_m: u32,
    end_of_authority_speed_kmh: u16,
    release_speed_kmh: u16,
    danger_point_m: u32,
    overlap_m: u32,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BaliseTelegram {
    balise_id: u64,
    nid_bg: u32,
    nid_c: u16,
    q_link: bool,
    packets: Vec<u8>,
    location_m: u32,
    orientation_nominal: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AxleCounter {
    counter_id: u32,
    section_name: String,
    count_in: u32,
    count_out: u32,
    is_balanced: bool,
    last_reset_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LevelCrossing {
    crossing_id: u32,
    name: String,
    barrier_state: BarrierState,
    approach_warning_active: bool,
    road_traffic_lights_active: bool,
    audible_warning_active: bool,
    activation_time_epoch: u64,
    expected_clear_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimetablePath {
    path_id: u64,
    train_uid: String,
    category: TrainCategory,
    origin_station: String,
    destination_station: String,
    departure_epoch: u64,
    arrival_epoch: u64,
    intermediate_stops: Vec<String>,
    platform_allocations: Vec<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpeedRestriction {
    restriction_id: u64,
    restriction_type: SpeedRestrictionType,
    start_milepost_m: u32,
    end_milepost_m: u32,
    speed_limit_kmh: u16,
    reason: String,
    valid_from_epoch: u64,
    valid_until_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtcsOnboardUnit {
    train_id: String,
    nid_engine: u32,
    level: EtcsLevel,
    mode: EtcsMode,
    current_speed_kmh: u16,
    permitted_speed_kmh: u16,
    target_speed_kmh: u16,
    target_distance_m: u32,
    brake_intervention_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainDetectionSection {
    section_id: u32,
    track_circuits: Vec<u32>,
    axle_counters: Vec<u32>,
    is_occupied: bool,
    last_train_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PointMachineStatus {
    point_id: u32,
    position: PointPosition,
    motor_current_ma: u32,
    throw_time_ms: u32,
    detection_left: bool,
    detection_right: bool,
    clamp_locked: bool,
    maintenance_due_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadioBlockCenter {
    rbc_id: u32,
    rbc_name: String,
    connected_trains: Vec<String>,
    active_ma_count: u32,
    handover_pending: Vec<u32>,
    last_heartbeat_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InterlockingSnapshot {
    interlocking_id: u32,
    name: String,
    routes: Vec<InterlockingRoute>,
    signals: Vec<SignalHead>,
    timestamp_epoch: u64,
}

// ── Test 1: Track circuit clear state roundtrip ─────────────────────────────

#[test]
fn test_track_circuit_clear_state_v1() {
    let tc = TrackCircuit {
        circuit_id: 101,
        section_name: "1T".to_string(),
        state: TrackCircuitState::Clear,
        length_m: 450,
        max_speed_kmh: 160,
        last_update_epoch: 1700000000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&tc, ver).expect("encode track circuit clear v1");
    let (decoded, version, consumed) =
        decode_versioned_value::<TrackCircuit>(&bytes).expect("decode track circuit clear v1");
    assert_eq!(decoded, tc);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert!(consumed > 0);
}

// ── Test 2: Track circuit occupied with disturbed neighbour ─────────────────

#[test]
fn test_track_circuit_occupied_v2() {
    let tc = TrackCircuit {
        circuit_id: 202,
        section_name: "2TAB".to_string(),
        state: TrackCircuitState::Occupied,
        length_m: 800,
        max_speed_kmh: 80,
        last_update_epoch: 1700001234,
    };
    let ver = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&tc, ver).expect("encode track circuit occupied v2.1");
    let (decoded, version, consumed) =
        decode_versioned_value::<TrackCircuit>(&bytes).expect("decode track circuit occupied v2.1");
    assert_eq!(decoded, tc);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

// ── Test 3: Signal aspect green with route indicator ────────────────────────

#[test]
fn test_signal_green_with_route_indicator_v1() {
    let sig = SignalHead {
        signal_id: 5001,
        name: "SIG-A1".to_string(),
        aspect: SignalAspect::Green,
        milepost_m: 12500,
        route_indicator: Some("P1".to_string()),
        is_permissive: false,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&sig, ver).expect("encode green signal v1");
    let (decoded, version, _consumed) =
        decode_versioned_value::<SignalHead>(&bytes).expect("decode green signal v1");
    assert_eq!(decoded, sig);
    assert_eq!(version.major, 1);
}

// ── Test 4: Signal aspect flashing yellow (approach control) ────────────────

#[test]
fn test_signal_flashing_yellow_permissive_v3() {
    let sig = SignalHead {
        signal_id: 5042,
        name: "SIG-B7".to_string(),
        aspect: SignalAspect::FlashingYellow,
        milepost_m: 34200,
        route_indicator: None,
        is_permissive: true,
    };
    let ver = Version::new(3, 0, 1);
    let bytes = encode_versioned_value(&sig, ver).expect("encode flashing yellow signal v3.0.1");
    let (decoded, version, consumed) =
        decode_versioned_value::<SignalHead>(&bytes).expect("decode flashing yellow signal v3.0.1");
    assert_eq!(decoded, sig);
    assert_eq!(version.major, 3);
    assert_eq!(version.patch, 1);
    assert!(consumed > 0);
}

// ── Test 5: Interlocking route with multiple point settings ─────────────────

#[test]
fn test_interlocking_route_with_points_v1() {
    let route = InterlockingRoute {
        route_id: 8001,
        entry_signal: "SIG-A1".to_string(),
        exit_signal: "SIG-A5".to_string(),
        status: RouteStatus::Set,
        points: vec![
            PointSetting {
                point_id: 101,
                required_position: PointPosition::Normal,
                detection_confirmed: true,
            },
            PointSetting {
                point_id: 102,
                required_position: PointPosition::Reverse,
                detection_confirmed: true,
            },
        ],
        conflicting_route_ids: vec![8002, 8003],
        overlap_length_m: 185,
    };
    let ver = Version::new(1, 2, 0);
    let bytes = encode_versioned_value(&route, ver).expect("encode interlocking route v1.2");
    let (decoded, version, consumed) = decode_versioned_value::<InterlockingRoute>(&bytes)
        .expect("decode interlocking route v1.2");
    assert_eq!(decoded, route);
    assert_eq!(version.minor, 2);
    assert!(consumed > 0);
}

// ── Test 6: Movement authority with overlap ─────────────────────────────────

#[test]
fn test_movement_authority_full_supervision_v1() {
    let ma = MovementAuthority {
        ma_id: 90001,
        train_id: "IC2048".to_string(),
        start_milepost_m: 5000,
        end_milepost_m: 25000,
        end_of_authority_speed_kmh: 0,
        release_speed_kmh: 15,
        danger_point_m: 25050,
        overlap_m: 200,
        timestamp_epoch: 1700100000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&ma, ver).expect("encode movement authority v1");
    let (decoded, version, _consumed) =
        decode_versioned_value::<MovementAuthority>(&bytes).expect("decode movement authority v1");
    assert_eq!(decoded, ma);
    assert_eq!(version.major, 1);
}

// ── Test 7: Balise telegram with packet data ────────────────────────────────

#[test]
fn test_balise_telegram_linked_group_v2() {
    let bt = BaliseTelegram {
        balise_id: 300001,
        nid_bg: 42,
        nid_c: 80,
        q_link: true,
        packets: vec![0x15, 0x45, 0x03, 0xFF, 0x27, 0x80, 0x00, 0xAB],
        location_m: 18750,
        orientation_nominal: true,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&bt, ver).expect("encode balise telegram linked v2");
    let (decoded, version, consumed) =
        decode_versioned_value::<BaliseTelegram>(&bytes).expect("decode balise telegram linked v2");
    assert_eq!(decoded, bt);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
}

// ── Test 8: Axle counter balanced section ───────────────────────────────────

#[test]
fn test_axle_counter_balanced_section_v1() {
    let ac = AxleCounter {
        counter_id: 401,
        section_name: "4TAC-North".to_string(),
        count_in: 128,
        count_out: 128,
        is_balanced: true,
        last_reset_epoch: 1699990000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&ac, ver).expect("encode axle counter balanced v1");
    let (decoded, version, consumed) =
        decode_versioned_value::<AxleCounter>(&bytes).expect("decode axle counter balanced v1");
    assert_eq!(decoded, ac);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

// ── Test 9: Axle counter unbalanced (ghost train) ───────────────────────────

#[test]
fn test_axle_counter_unbalanced_ghost_train_v1() {
    let ac = AxleCounter {
        counter_id: 402,
        section_name: "5TAC-South".to_string(),
        count_in: 64,
        count_out: 60,
        is_balanced: false,
        last_reset_epoch: 1699990500,
    };
    let bytes = encode_to_vec(&ac).expect("encode axle counter unbalanced");
    let (decoded, consumed) =
        decode_from_slice::<AxleCounter>(&bytes).expect("decode axle counter unbalanced");
    assert_eq!(decoded, ac);
    assert_eq!(consumed, bytes.len());
    assert!(!decoded.is_balanced);
    assert!(decoded.count_in > decoded.count_out);
}

// ── Test 10: Level crossing barrier closing sequence ────────────────────────

#[test]
fn test_level_crossing_closing_sequence_v2() {
    let lx = LevelCrossing {
        crossing_id: 550,
        name: "LX-Elm-Road".to_string(),
        barrier_state: BarrierState::Closing,
        approach_warning_active: true,
        road_traffic_lights_active: true,
        audible_warning_active: true,
        activation_time_epoch: 1700050000,
        expected_clear_epoch: 1700050120,
    };
    let ver = Version::new(2, 3, 0);
    let bytes = encode_versioned_value(&lx, ver).expect("encode level crossing closing v2.3");
    let (decoded, version, consumed) = decode_versioned_value::<LevelCrossing>(&bytes)
        .expect("decode level crossing closing v2.3");
    assert_eq!(decoded, lx);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 3);
    assert!(consumed > 0);
}

// ── Test 11: Level crossing blocked (failure mode) ──────────────────────────

#[test]
fn test_level_crossing_blocked_failure_v1() {
    let lx = LevelCrossing {
        crossing_id: 551,
        name: "LX-Oak-Lane".to_string(),
        barrier_state: BarrierState::Blocked,
        approach_warning_active: true,
        road_traffic_lights_active: true,
        audible_warning_active: false,
        activation_time_epoch: 1700060000,
        expected_clear_epoch: 0,
    };
    let ver = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&lx, ver).expect("encode level crossing blocked v1.1");
    let (decoded, version, _consumed) = decode_versioned_value::<LevelCrossing>(&bytes)
        .expect("decode level crossing blocked v1.1");
    assert_eq!(decoded, lx);
    assert_eq!(version.minor, 1);
    assert_eq!(decoded.barrier_state, BarrierState::Blocked);
}

// ── Test 12: Timetable path with intermediate stops ─────────────────────────

#[test]
fn test_timetable_path_intercity_with_stops_v1() {
    let path = TimetablePath {
        path_id: 77001,
        train_uid: "1A42".to_string(),
        category: TrainCategory::Intercity,
        origin_station: "London Euston".to_string(),
        destination_station: "Manchester Piccadilly".to_string(),
        departure_epoch: 1700070000,
        arrival_epoch: 1700077200,
        intermediate_stops: vec![
            "Milton Keynes Central".to_string(),
            "Stoke-on-Trent".to_string(),
            "Stockport".to_string(),
        ],
        platform_allocations: vec![8, 2, 1, 12],
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&path, ver).expect("encode timetable path intercity v1");
    let (decoded, version, consumed) = decode_versioned_value::<TimetablePath>(&bytes)
        .expect("decode timetable path intercity v1");
    assert_eq!(decoded, path);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

// ── Test 13: Timetable path freight (no intermediate stops) ─────────────────

#[test]
fn test_timetable_path_freight_direct_v2() {
    let path = TimetablePath {
        path_id: 88500,
        train_uid: "6Z99".to_string(),
        category: TrainCategory::Freight,
        origin_station: "Felixstowe North".to_string(),
        destination_station: "Trafford Park".to_string(),
        departure_epoch: 1700080000,
        arrival_epoch: 1700101600,
        intermediate_stops: vec![],
        platform_allocations: vec![],
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&path, ver).expect("encode timetable path freight v2");
    let (decoded, version, consumed) =
        decode_versioned_value::<TimetablePath>(&bytes).expect("decode timetable path freight v2");
    assert_eq!(decoded, path);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
    assert!(decoded.intermediate_stops.is_empty());
}

// ── Test 14: Speed restriction temporary ────────────────────────────────────

#[test]
fn test_speed_restriction_temporary_v1() {
    let sr = SpeedRestriction {
        restriction_id: 60001,
        restriction_type: SpeedRestrictionType::Temporary,
        start_milepost_m: 45000,
        end_milepost_m: 46200,
        speed_limit_kmh: 40,
        reason: "Track geometry defect near bridge 47B".to_string(),
        valid_from_epoch: 1700000000,
        valid_until_epoch: 1700604800,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&sr, ver).expect("encode temp speed restriction v1");
    let (decoded, version, consumed) = decode_versioned_value::<SpeedRestriction>(&bytes)
        .expect("decode temp speed restriction v1");
    assert_eq!(decoded, sr);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

// ── Test 15: Speed restriction emergency ────────────────────────────────────

#[test]
fn test_speed_restriction_emergency_v3() {
    let sr = SpeedRestriction {
        restriction_id: 60099,
        restriction_type: SpeedRestrictionType::Emergency,
        start_milepost_m: 78000,
        end_milepost_m: 78500,
        speed_limit_kmh: 5,
        reason: "Rail break detected by ultrasonic inspection".to_string(),
        valid_from_epoch: 1700200000,
        valid_until_epoch: 0,
    };
    let ver = Version::new(3, 1, 2);
    let bytes =
        encode_versioned_value(&sr, ver).expect("encode emergency speed restriction v3.1.2");
    let (decoded, version, consumed) = decode_versioned_value::<SpeedRestriction>(&bytes)
        .expect("decode emergency speed restriction v3.1.2");
    assert_eq!(decoded, sr);
    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 2);
    assert!(consumed > 0);
}

// ── Test 16: ETCS onboard unit full supervision ─────────────────────────────

#[test]
fn test_etcs_obu_full_supervision_level2_v1() {
    let obu = EtcsOnboardUnit {
        train_id: "IC2048".to_string(),
        nid_engine: 12345678,
        level: EtcsLevel::Level2,
        mode: EtcsMode::FullSupervision,
        current_speed_kmh: 280,
        permitted_speed_kmh: 300,
        target_speed_kmh: 300,
        target_distance_m: 15000,
        brake_intervention_active: false,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&obu, ver).expect("encode ETCS OBU FS L2 v1");
    let (decoded, version, consumed) =
        decode_versioned_value::<EtcsOnboardUnit>(&bytes).expect("decode ETCS OBU FS L2 v1");
    assert_eq!(decoded, obu);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

// ── Test 17: ETCS trip mode after signal passed at danger ───────────────────

#[test]
fn test_etcs_obu_trip_mode_spad_v2() {
    let obu = EtcsOnboardUnit {
        train_id: "RE7734".to_string(),
        nid_engine: 98765432,
        level: EtcsLevel::Level1,
        mode: EtcsMode::Trip,
        current_speed_kmh: 0,
        permitted_speed_kmh: 0,
        target_speed_kmh: 0,
        target_distance_m: 0,
        brake_intervention_active: true,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&obu, ver).expect("encode ETCS OBU trip mode v2");
    let (decoded, version, _consumed) =
        decode_versioned_value::<EtcsOnboardUnit>(&bytes).expect("decode ETCS OBU trip mode v2");
    assert_eq!(decoded, obu);
    assert_eq!(version.major, 2);
    assert!(decoded.brake_intervention_active);
    assert_eq!(decoded.mode, EtcsMode::Trip);
}

// ── Test 18: Train detection section with multiple sensors ──────────────────

#[test]
fn test_train_detection_section_occupied_v1() {
    let tds = TrainDetectionSection {
        section_id: 7001,
        track_circuits: vec![101, 102, 103],
        axle_counters: vec![401, 402],
        is_occupied: true,
        last_train_id: Some("FR8821".to_string()),
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&tds, ver).expect("encode train detection section v1");
    let (decoded, version, consumed) = decode_versioned_value::<TrainDetectionSection>(&bytes)
        .expect("decode train detection section v1");
    assert_eq!(decoded, tds);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

// ── Test 19: Point machine status with diagnostics ──────────────────────────

#[test]
fn test_point_machine_normal_position_diagnostics_v1() {
    let pm = PointMachineStatus {
        point_id: 201,
        position: PointPosition::Normal,
        motor_current_ma: 3200,
        throw_time_ms: 4500,
        detection_left: true,
        detection_right: false,
        clamp_locked: true,
        maintenance_due_epoch: 1702000000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&pm, ver).expect("encode point machine normal v1");
    let (decoded, version, consumed) = decode_versioned_value::<PointMachineStatus>(&bytes)
        .expect("decode point machine normal v1");
    assert_eq!(decoded, pm);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

// ── Test 20: Point machine failed state ─────────────────────────────────────

#[test]
fn test_point_machine_failed_no_detection_v2() {
    let pm = PointMachineStatus {
        point_id: 202,
        position: PointPosition::Failed,
        motor_current_ma: 8500,
        throw_time_ms: 12000,
        detection_left: false,
        detection_right: false,
        clamp_locked: false,
        maintenance_due_epoch: 1700500000,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&pm, ver).expect("encode point machine failed v2");
    let (decoded, version, consumed) = decode_versioned_value::<PointMachineStatus>(&bytes)
        .expect("decode point machine failed v2");
    assert_eq!(decoded, pm);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
    assert_eq!(decoded.position, PointPosition::Failed);
    assert!(!decoded.detection_left);
    assert!(!decoded.detection_right);
}

// ── Test 21: Radio block center with connected trains ───────────────────────

#[test]
fn test_rbc_multiple_connected_trains_v1() {
    let rbc = RadioBlockCenter {
        rbc_id: 9001,
        rbc_name: "RBC-Nord-3".to_string(),
        connected_trains: vec![
            "IC2048".to_string(),
            "TGV9244".to_string(),
            "RE7734".to_string(),
            "FR8821".to_string(),
        ],
        active_ma_count: 4,
        handover_pending: vec![9002],
        last_heartbeat_epoch: 1700300000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&rbc, ver).expect("encode RBC connected v1");
    let (decoded, version, consumed) =
        decode_versioned_value::<RadioBlockCenter>(&bytes).expect("decode RBC connected v1");
    assert_eq!(decoded, rbc);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
    assert_eq!(decoded.connected_trains.len(), 4);
}

// ── Test 22: Full interlocking snapshot (nested structs) ────────────────────

#[test]
fn test_interlocking_snapshot_full_v1() {
    let snapshot = InterlockingSnapshot {
        interlocking_id: 1,
        name: "Paddington Panel A".to_string(),
        routes: vec![
            InterlockingRoute {
                route_id: 8001,
                entry_signal: "SIG-P1".to_string(),
                exit_signal: "SIG-P5".to_string(),
                status: RouteStatus::Set,
                points: vec![PointSetting {
                    point_id: 301,
                    required_position: PointPosition::Normal,
                    detection_confirmed: true,
                }],
                conflicting_route_ids: vec![8002],
                overlap_length_m: 200,
            },
            InterlockingRoute {
                route_id: 8002,
                entry_signal: "SIG-P2".to_string(),
                exit_signal: "SIG-P6".to_string(),
                status: RouteStatus::Free,
                points: vec![
                    PointSetting {
                        point_id: 301,
                        required_position: PointPosition::Reverse,
                        detection_confirmed: false,
                    },
                    PointSetting {
                        point_id: 302,
                        required_position: PointPosition::Normal,
                        detection_confirmed: true,
                    },
                ],
                conflicting_route_ids: vec![8001, 8003],
                overlap_length_m: 185,
            },
        ],
        signals: vec![
            SignalHead {
                signal_id: 5001,
                name: "SIG-P1".to_string(),
                aspect: SignalAspect::Green,
                milepost_m: 100,
                route_indicator: Some("P1".to_string()),
                is_permissive: false,
            },
            SignalHead {
                signal_id: 5002,
                name: "SIG-P2".to_string(),
                aspect: SignalAspect::Red,
                milepost_m: 150,
                route_indicator: None,
                is_permissive: false,
            },
        ],
        timestamp_epoch: 1700400000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&snapshot, ver).expect("encode interlocking snapshot v1");
    let (decoded, version, consumed) = decode_versioned_value::<InterlockingSnapshot>(&bytes)
        .expect("decode interlocking snapshot v1");
    assert_eq!(decoded, snapshot);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
    assert_eq!(decoded.routes.len(), 2);
    assert_eq!(decoded.signals.len(), 2);
}
