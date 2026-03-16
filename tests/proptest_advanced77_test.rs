//! Advanced property-based tests (set 77) — Railway Signaling and Train Control Systems domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers track circuit occupancy, signal aspects, interlocking route tables,
//! ETCS movement authorities, balise telegrams, timetable scheduling, rolling
//! stock characteristics, platform dwell times, points/switch positions, axle
//! counter readings, train detection sections, ATP interventions, level crossing
//! barriers, speed restriction orders, and energy consumption profiles.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// ── Domain types ──────────────────────────────────────────────────────────────

/// Track circuit occupancy state for a block section.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrackCircuitOccupancy {
    /// Track circuit identifier.
    circuit_id: u32,
    /// Whether the track circuit is occupied.
    occupied: bool,
    /// Timestamp of last state change (Unix seconds).
    last_change_s: u64,
    /// Rail impedance reading in ohms.
    impedance_ohms: f32,
    /// Signal strength in dBm.
    signal_strength_dbm: f32,
}

/// Railway signal aspect (colour/state).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SignalAspect {
    /// Red — stop.
    Red,
    /// Single yellow — caution, next signal at danger.
    Yellow,
    /// Double yellow — preliminary caution.
    DoubleYellow,
    /// Green — proceed.
    Green,
    /// Flashing yellow — caution with diverging route.
    FlashingYellow { flash_rate_hz: f32 },
    /// Flashing green — high-speed proceed.
    FlashingGreen { flash_rate_hz: f32 },
}

/// Interlocking route table entry mapping signal to permitted routes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InterlockingRoute {
    /// Route identifier.
    route_id: u32,
    /// Entry signal identifier.
    entry_signal_id: u32,
    /// Exit signal identifier.
    exit_signal_id: u32,
    /// Required points positions (point_id, normal=true/reverse=false).
    points_required: Vec<(u32, bool)>,
    /// Track circuits that must be clear.
    clear_circuits: Vec<u32>,
    /// Route locked flag.
    locked: bool,
}

/// ETCS Level 2 movement authority.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MovementAuthority {
    /// MA identifier.
    ma_id: u64,
    /// Target distance in metres from current position.
    target_distance_m: f32,
    /// Permitted speed in km/h.
    permitted_speed_kmh: f32,
    /// End-of-authority speed in km/h (usually 0).
    eoa_speed_kmh: f32,
    /// Time-out value in seconds.
    timeout_s: u32,
    /// Danger point distance beyond EOA in metres.
    danger_point_m: f32,
    /// Overlap distance in metres.
    overlap_m: f32,
}

/// Eurobalise telegram data packet.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BaliseTelegram {
    /// Balise group identifier.
    group_id: u32,
    /// National balise identifier.
    nid_bg: u32,
    /// Direction of validity (true = nominal, false = reverse).
    direction_nominal: bool,
    /// Linked balise group flag.
    linked: bool,
    /// Distance to next balise group in metres.
    distance_to_next_m: f32,
    /// Gradient profile: (distance_m, gradient_permille) pairs.
    gradient_profile: Vec<(f32, f32)>,
}

/// Timetable schedule entry for a station stop.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimetableEntry {
    /// Train service number.
    service_number: u32,
    /// Station identifier.
    station_id: u16,
    /// Scheduled arrival time (seconds since midnight).
    arrival_s: u32,
    /// Scheduled departure time (seconds since midnight).
    departure_s: u32,
    /// Platform number.
    platform_number: u8,
    /// Whether the stop is conditional (request stop).
    request_stop: bool,
}

/// Rolling stock characteristics for braking and traction.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RollingStockCharacteristics {
    /// Vehicle identifier.
    vehicle_id: u64,
    /// Total train length in metres.
    length_m: f32,
    /// Total mass in tonnes.
    mass_tonnes: f32,
    /// Maximum service braking rate in m/s^2.
    max_service_brake_ms2: f32,
    /// Maximum emergency braking rate in m/s^2.
    max_emergency_brake_ms2: f32,
    /// Maximum speed in km/h.
    max_speed_kmh: f32,
    /// Rotary mass factor (typically 1.04-1.10).
    rotary_mass_factor: f32,
}

/// Platform dwell time record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlatformDwellTime {
    /// Station identifier.
    station_id: u16,
    /// Platform number.
    platform: u8,
    /// Planned dwell time in seconds.
    planned_dwell_s: u16,
    /// Actual dwell time in seconds.
    actual_dwell_s: u16,
    /// Number of passengers boarding.
    passengers_boarding: u16,
    /// Number of passengers alighting.
    passengers_alighting: u16,
    /// Door open duration in seconds.
    door_open_s: u16,
}

/// Points (switch/turnout) position status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PointsPosition {
    /// Normal position (straight through).
    Normal { point_id: u32 },
    /// Reverse position (diverging route).
    Reverse { point_id: u32 },
    /// Points in transition between positions.
    InTransition { point_id: u32, elapsed_ms: u32 },
    /// Points failed or not detected.
    Failed { point_id: u32, fault_code: u8 },
}

/// Axle counter reading for a train detection section.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AxleCounterReading {
    /// Counter identifier.
    counter_id: u32,
    /// Section identifier.
    section_id: u32,
    /// Count-in axles (entry direction).
    count_in: u16,
    /// Count-out axles (exit direction).
    count_out: u16,
    /// Section occupied (count_in != count_out).
    section_occupied: bool,
    /// Last reset timestamp (Unix seconds).
    last_reset_s: u64,
}

/// Train detection section status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainDetectionSection {
    /// Section identifier.
    section_id: u32,
    /// Section length in metres.
    length_m: f32,
    /// Occupied flag.
    occupied: bool,
    /// Axle counter readings for this section.
    axle_counts: Vec<AxleCounterReading>,
    /// Adjacent section identifiers.
    adjacent_sections: Vec<u32>,
}

/// Automatic Train Protection (ATP) intervention record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AtpIntervention {
    /// Warning issued to driver.
    Warning {
        train_id: u64,
        speed_kmh: f32,
        permitted_kmh: f32,
    },
    /// Service brake applied.
    ServiceBrake {
        train_id: u64,
        speed_kmh: f32,
        permitted_kmh: f32,
        braking_distance_m: f32,
    },
    /// Emergency brake applied.
    EmergencyBrake {
        train_id: u64,
        speed_kmh: f32,
        reason_code: u8,
    },
}

/// Level crossing barrier status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LevelCrossingStatus {
    /// Crossing identifier.
    crossing_id: u32,
    /// Barrier lowered flag.
    barriers_lowered: bool,
    /// Warning lights active flag.
    lights_active: bool,
    /// Audible warning active flag.
    audible_warning: bool,
    /// Time since activation in seconds.
    activation_time_s: u16,
    /// Number of road vehicles detected.
    vehicles_detected: u8,
    /// Obstacle detection alarm active.
    obstacle_alarm: bool,
}

/// Temporary speed restriction order.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpeedRestrictionOrder {
    /// Order identifier.
    order_id: u64,
    /// Start location (km post).
    start_km: f32,
    /// End location (km post).
    end_km: f32,
    /// Restricted speed in km/h.
    speed_kmh: f32,
    /// Reason code.
    reason_code: u8,
    /// Valid from (Unix seconds).
    valid_from_s: u64,
    /// Valid until (Unix seconds).
    valid_until_s: u64,
}

/// Energy consumption profile snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyConsumptionProfile {
    /// Train identifier.
    train_id: u64,
    /// Current traction power draw in kW.
    traction_kw: f32,
    /// Auxiliary power draw in kW (HVAC, lighting).
    auxiliary_kw: f32,
    /// Regenerative braking power return in kW.
    regen_kw: f32,
    /// Pantograph voltage in V.
    pantograph_v: f32,
    /// Line current in amperes.
    line_current_a: f32,
    /// Cumulative energy consumed in kWh.
    cumulative_kwh: f32,
}

// ── prop_compose! strategies ──────────────────────────────────────────────────

prop_compose! {
    fn arb_track_circuit()(
        circuit_id: u32,
        occupied: bool,
        last_change_s: u64,
        impedance_ohms in 0.1f32..100.0f32,
        signal_strength_dbm in (-120.0f32)..0.0f32,
    ) -> TrackCircuitOccupancy {
        TrackCircuitOccupancy {
            circuit_id, occupied, last_change_s, impedance_ohms, signal_strength_dbm,
        }
    }
}

prop_compose! {
    fn arb_movement_authority()(
        ma_id: u64,
        target_distance_m in 100.0f32..50_000.0f32,
        permitted_speed_kmh in 10.0f32..350.0f32,
        eoa_speed_kmh in 0.0f32..10.0f32,
        timeout_s in 10u32..600u32,
        danger_point_m in 10.0f32..200.0f32,
        overlap_m in 10.0f32..300.0f32,
    ) -> MovementAuthority {
        MovementAuthority {
            ma_id, target_distance_m, permitted_speed_kmh, eoa_speed_kmh,
            timeout_s, danger_point_m, overlap_m,
        }
    }
}

prop_compose! {
    fn arb_rolling_stock()(
        vehicle_id: u64,
        length_m in 20.0f32..400.0f32,
        mass_tonnes in 50.0f32..1200.0f32,
        max_service_brake_ms2 in 0.5f32..1.5f32,
        max_emergency_brake_ms2 in 1.0f32..3.0f32,
        max_speed_kmh in 80.0f32..350.0f32,
        rotary_mass_factor in 1.04f32..1.10f32,
    ) -> RollingStockCharacteristics {
        RollingStockCharacteristics {
            vehicle_id, length_m, mass_tonnes, max_service_brake_ms2,
            max_emergency_brake_ms2, max_speed_kmh, rotary_mass_factor,
        }
    }
}

prop_compose! {
    fn arb_energy_profile()(
        train_id: u64,
        traction_kw in 0.0f32..6000.0f32,
        auxiliary_kw in 0.0f32..500.0f32,
        regen_kw in 0.0f32..3000.0f32,
        pantograph_v in 600.0f32..25_000.0f32,
        line_current_a in 0.0f32..2000.0f32,
        cumulative_kwh in 0.0f32..100_000.0f32,
    ) -> EnergyConsumptionProfile {
        EnergyConsumptionProfile {
            train_id, traction_kw, auxiliary_kw, regen_kw,
            pantograph_v, line_current_a, cumulative_kwh,
        }
    }
}

prop_compose! {
    fn arb_timetable_entry()(
        service_number in 1u32..99_999u32,
        station_id: u16,
        arrival_s in 0u32..86_400u32,
        dwell in 30u32..600u32,
        platform_number in 1u8..30u8,
        request_stop: bool,
    ) -> TimetableEntry {
        TimetableEntry {
            service_number,
            station_id,
            arrival_s,
            departure_s: arrival_s.saturating_add(dwell),
            platform_number,
            request_stop,
        }
    }
}

prop_compose! {
    fn arb_level_crossing()(
        crossing_id: u32,
        barriers_lowered: bool,
        lights_active: bool,
        audible_warning: bool,
        activation_time_s in 0u16..300u16,
        vehicles_detected in 0u8..50u8,
        obstacle_alarm: bool,
    ) -> LevelCrossingStatus {
        LevelCrossingStatus {
            crossing_id, barriers_lowered, lights_active, audible_warning,
            activation_time_s, vehicles_detected, obstacle_alarm,
        }
    }
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. TrackCircuitOccupancy roundtrip ────────────────────────────────────────

#[test]
fn test_track_circuit_occupancy_roundtrip() {
    proptest!(|(val in arb_track_circuit())| {
        let enc = encode_to_vec(&val).expect("encode TrackCircuitOccupancy failed");
        let (dec, consumed): (TrackCircuitOccupancy, usize) =
            decode_from_slice(&enc).expect("decode TrackCircuitOccupancy failed");
        prop_assert_eq!(&val, &dec, "TrackCircuitOccupancy roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 2. Vec<TrackCircuitOccupancy> roundtrip ───────────────────────────────────

#[test]
fn test_vec_track_circuit_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(arb_track_circuit(), 0..10usize),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<TrackCircuitOccupancy> failed");
        let (dec, consumed): (Vec<TrackCircuitOccupancy>, usize) =
            decode_from_slice(&enc).expect("decode Vec<TrackCircuitOccupancy> failed");
        prop_assert_eq!(&items, &dec, "Vec<TrackCircuitOccupancy> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 3. SignalAspect roundtrip (all variants) ──────────────────────────────────

#[test]
fn test_signal_aspect_roundtrip() {
    let arb_aspect = prop_oneof![
        Just(SignalAspect::Red),
        Just(SignalAspect::Yellow),
        Just(SignalAspect::DoubleYellow),
        Just(SignalAspect::Green),
        (0.5f32..3.0f32).prop_map(|r| SignalAspect::FlashingYellow { flash_rate_hz: r }),
        (0.5f32..3.0f32).prop_map(|r| SignalAspect::FlashingGreen { flash_rate_hz: r }),
    ];
    proptest!(|(val in arb_aspect)| {
        let enc = encode_to_vec(&val).expect("encode SignalAspect failed");
        let (dec, consumed): (SignalAspect, usize) =
            decode_from_slice(&enc).expect("decode SignalAspect failed");
        prop_assert_eq!(&val, &dec, "SignalAspect roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 4. InterlockingRoute roundtrip ────────────────────────────────────────────

#[test]
fn test_interlocking_route_roundtrip() {
    proptest!(|(
        route_id: u32,
        entry_signal_id: u32,
        exit_signal_id: u32,
        points_required in prop::collection::vec(
            (any::<u32>(), any::<bool>()), 0..6usize
        ),
        clear_circuits in prop::collection::vec(any::<u32>(), 0..8usize),
        locked: bool,
    )| {
        let val = InterlockingRoute {
            route_id, entry_signal_id, exit_signal_id,
            points_required, clear_circuits, locked,
        };
        let enc = encode_to_vec(&val).expect("encode InterlockingRoute failed");
        let (dec, consumed): (InterlockingRoute, usize) =
            decode_from_slice(&enc).expect("decode InterlockingRoute failed");
        prop_assert_eq!(&val, &dec, "InterlockingRoute roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 5. MovementAuthority roundtrip ────────────────────────────────────────────

#[test]
fn test_movement_authority_roundtrip() {
    proptest!(|(val in arb_movement_authority())| {
        let enc = encode_to_vec(&val).expect("encode MovementAuthority failed");
        let (dec, consumed): (MovementAuthority, usize) =
            decode_from_slice(&enc).expect("decode MovementAuthority failed");
        prop_assert_eq!(&val, &dec, "MovementAuthority roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 6. MovementAuthority re-encode determinism ────────────────────────────────

#[test]
fn test_movement_authority_determinism() {
    proptest!(|(val in arb_movement_authority())| {
        let enc1 = encode_to_vec(&val).expect("first encode MovementAuthority failed");
        let enc2 = encode_to_vec(&val).expect("second encode MovementAuthority failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 7. BaliseTelegram roundtrip ───────────────────────────────────────────────

#[test]
fn test_balise_telegram_roundtrip() {
    proptest!(|(
        group_id: u32,
        nid_bg: u32,
        direction_nominal: bool,
        linked: bool,
        distance_to_next_m in 50.0f32..10_000.0f32,
        gradient_profile in prop::collection::vec(
            (0.0f32..50_000.0f32, (-30.0f32)..30.0f32), 0..8usize
        ),
    )| {
        let val = BaliseTelegram {
            group_id, nid_bg, direction_nominal, linked,
            distance_to_next_m, gradient_profile,
        };
        let enc = encode_to_vec(&val).expect("encode BaliseTelegram failed");
        let (dec, consumed): (BaliseTelegram, usize) =
            decode_from_slice(&enc).expect("decode BaliseTelegram failed");
        prop_assert_eq!(&val, &dec, "BaliseTelegram roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 8. TimetableEntry roundtrip ───────────────────────────────────────────────

#[test]
fn test_timetable_entry_roundtrip() {
    proptest!(|(val in arb_timetable_entry())| {
        let enc = encode_to_vec(&val).expect("encode TimetableEntry failed");
        let (dec, consumed): (TimetableEntry, usize) =
            decode_from_slice(&enc).expect("decode TimetableEntry failed");
        prop_assert_eq!(&val, &dec, "TimetableEntry roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 9. Vec<TimetableEntry> roundtrip ──────────────────────────────────────────

#[test]
fn test_vec_timetable_entry_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(arb_timetable_entry(), 0..12usize),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<TimetableEntry> failed");
        let (dec, consumed): (Vec<TimetableEntry>, usize) =
            decode_from_slice(&enc).expect("decode Vec<TimetableEntry> failed");
        prop_assert_eq!(&items, &dec, "Vec<TimetableEntry> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 10. RollingStockCharacteristics roundtrip ─────────────────────────────────

#[test]
fn test_rolling_stock_roundtrip() {
    proptest!(|(val in arb_rolling_stock())| {
        let enc = encode_to_vec(&val).expect("encode RollingStockCharacteristics failed");
        let (dec, consumed): (RollingStockCharacteristics, usize) =
            decode_from_slice(&enc).expect("decode RollingStockCharacteristics failed");
        prop_assert_eq!(&val, &dec, "RollingStockCharacteristics roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 11. PlatformDwellTime roundtrip ───────────────────────────────────────────

#[test]
fn test_platform_dwell_time_roundtrip() {
    proptest!(|(
        station_id: u16,
        platform in 1u8..30u8,
        planned_dwell_s in 30u16..600u16,
        actual_dwell_s in 20u16..900u16,
        passengers_boarding in 0u16..500u16,
        passengers_alighting in 0u16..500u16,
        door_open_s in 10u16..300u16,
    )| {
        let val = PlatformDwellTime {
            station_id, platform, planned_dwell_s, actual_dwell_s,
            passengers_boarding, passengers_alighting, door_open_s,
        };
        let enc = encode_to_vec(&val).expect("encode PlatformDwellTime failed");
        let (dec, consumed): (PlatformDwellTime, usize) =
            decode_from_slice(&enc).expect("decode PlatformDwellTime failed");
        prop_assert_eq!(&val, &dec, "PlatformDwellTime roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 12. PointsPosition roundtrip (all variants) ──────────────────────────────

#[test]
fn test_points_position_roundtrip() {
    let arb_points = prop_oneof![
        any::<u32>().prop_map(|id| PointsPosition::Normal { point_id: id }),
        any::<u32>().prop_map(|id| PointsPosition::Reverse { point_id: id }),
        (any::<u32>(), 0u32..5000u32).prop_map(|(id, ms)| PointsPosition::InTransition {
            point_id: id,
            elapsed_ms: ms
        }),
        (any::<u32>(), any::<u8>()).prop_map(|(id, fc)| PointsPosition::Failed {
            point_id: id,
            fault_code: fc
        }),
    ];
    proptest!(|(val in arb_points)| {
        let enc = encode_to_vec(&val).expect("encode PointsPosition failed");
        let (dec, consumed): (PointsPosition, usize) =
            decode_from_slice(&enc).expect("decode PointsPosition failed");
        prop_assert_eq!(&val, &dec, "PointsPosition roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 13. AxleCounterReading roundtrip ──────────────────────────────────────────

#[test]
fn test_axle_counter_reading_roundtrip() {
    proptest!(|(
        counter_id: u32,
        section_id: u32,
        count_in: u16,
        count_out: u16,
        last_reset_s: u64,
    )| {
        let section_occupied = count_in != count_out;
        let val = AxleCounterReading {
            counter_id, section_id, count_in, count_out,
            section_occupied, last_reset_s,
        };
        let enc = encode_to_vec(&val).expect("encode AxleCounterReading failed");
        let (dec, consumed): (AxleCounterReading, usize) =
            decode_from_slice(&enc).expect("decode AxleCounterReading failed");
        prop_assert_eq!(&val, &dec, "AxleCounterReading roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 14. TrainDetectionSection roundtrip ───────────────────────────────────────

#[test]
fn test_train_detection_section_roundtrip() {
    proptest!(|(
        section_id: u32,
        length_m in 50.0f32..5000.0f32,
        occupied: bool,
        axle_counts in prop::collection::vec(
            (any::<u32>(), any::<u32>(), 0u16..200u16, 0u16..200u16, any::<u64>())
                .prop_map(|(cid, sid, ci, co, lr)| AxleCounterReading {
                    counter_id: cid, section_id: sid, count_in: ci, count_out: co,
                    section_occupied: ci != co, last_reset_s: lr,
                }),
            0..4usize,
        ),
        adjacent_sections in prop::collection::vec(any::<u32>(), 0..4usize),
    )| {
        let val = TrainDetectionSection {
            section_id, length_m, occupied, axle_counts, adjacent_sections,
        };
        let enc = encode_to_vec(&val).expect("encode TrainDetectionSection failed");
        let (dec, consumed): (TrainDetectionSection, usize) =
            decode_from_slice(&enc).expect("decode TrainDetectionSection failed");
        prop_assert_eq!(&val, &dec, "TrainDetectionSection roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 15. AtpIntervention::Warning roundtrip ────────────────────────────────────

#[test]
fn test_atp_warning_roundtrip() {
    proptest!(|(
        train_id: u64,
        speed_kmh in 0.0f32..350.0f32,
        permitted_kmh in 0.0f32..350.0f32,
    )| {
        let val = AtpIntervention::Warning { train_id, speed_kmh, permitted_kmh };
        let enc = encode_to_vec(&val).expect("encode AtpIntervention::Warning failed");
        let (dec, consumed): (AtpIntervention, usize) =
            decode_from_slice(&enc).expect("decode AtpIntervention::Warning failed");
        prop_assert_eq!(&val, &dec, "AtpIntervention::Warning roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 16. AtpIntervention::ServiceBrake roundtrip ───────────────────────────────

#[test]
fn test_atp_service_brake_roundtrip() {
    proptest!(|(
        train_id: u64,
        speed_kmh in 0.0f32..350.0f32,
        permitted_kmh in 0.0f32..350.0f32,
        braking_distance_m in 0.0f32..5000.0f32,
    )| {
        let val = AtpIntervention::ServiceBrake {
            train_id, speed_kmh, permitted_kmh, braking_distance_m,
        };
        let enc = encode_to_vec(&val).expect("encode AtpIntervention::ServiceBrake failed");
        let (dec, consumed): (AtpIntervention, usize) =
            decode_from_slice(&enc).expect("decode AtpIntervention::ServiceBrake failed");
        prop_assert_eq!(&val, &dec, "AtpIntervention::ServiceBrake roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 17. AtpIntervention::EmergencyBrake roundtrip ─────────────────────────────

#[test]
fn test_atp_emergency_brake_roundtrip() {
    proptest!(|(
        train_id: u64,
        speed_kmh in 0.0f32..350.0f32,
        reason_code: u8,
    )| {
        let val = AtpIntervention::EmergencyBrake { train_id, speed_kmh, reason_code };
        let enc = encode_to_vec(&val).expect("encode AtpIntervention::EmergencyBrake failed");
        let (dec, consumed): (AtpIntervention, usize) =
            decode_from_slice(&enc).expect("decode AtpIntervention::EmergencyBrake failed");
        prop_assert_eq!(&val, &dec, "AtpIntervention::EmergencyBrake roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 18. LevelCrossingStatus roundtrip ─────────────────────────────────────────

#[test]
fn test_level_crossing_status_roundtrip() {
    proptest!(|(val in arb_level_crossing())| {
        let enc = encode_to_vec(&val).expect("encode LevelCrossingStatus failed");
        let (dec, consumed): (LevelCrossingStatus, usize) =
            decode_from_slice(&enc).expect("decode LevelCrossingStatus failed");
        prop_assert_eq!(&val, &dec, "LevelCrossingStatus roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 19. SpeedRestrictionOrder roundtrip ───────────────────────────────────────

#[test]
fn test_speed_restriction_order_roundtrip() {
    proptest!(|(
        order_id: u64,
        start_km in 0.0f32..1000.0f32,
        extent in 0.1f32..50.0f32,
        speed_kmh in 5.0f32..160.0f32,
        reason_code: u8,
        valid_from_s: u64,
        duration in 3600u64..604_800u64,
    )| {
        let val = SpeedRestrictionOrder {
            order_id,
            start_km,
            end_km: start_km + extent,
            speed_kmh,
            reason_code,
            valid_from_s,
            valid_until_s: valid_from_s.saturating_add(duration),
        };
        let enc = encode_to_vec(&val).expect("encode SpeedRestrictionOrder failed");
        let (dec, consumed): (SpeedRestrictionOrder, usize) =
            decode_from_slice(&enc).expect("decode SpeedRestrictionOrder failed");
        prop_assert_eq!(&val, &dec, "SpeedRestrictionOrder roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 20. EnergyConsumptionProfile roundtrip ────────────────────────────────────

#[test]
fn test_energy_consumption_profile_roundtrip() {
    proptest!(|(val in arb_energy_profile())| {
        let enc = encode_to_vec(&val).expect("encode EnergyConsumptionProfile failed");
        let (dec, consumed): (EnergyConsumptionProfile, usize) =
            decode_from_slice(&enc).expect("decode EnergyConsumptionProfile failed");
        prop_assert_eq!(&val, &dec, "EnergyConsumptionProfile roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 21. EnergyConsumptionProfile re-encode idempotency ────────────────────────

#[test]
fn test_energy_profile_reencode_idempotent() {
    proptest!(|(val in arb_energy_profile())| {
        let enc1 = encode_to_vec(&val).expect("first encode EnergyConsumptionProfile failed");
        let (decoded, _): (EnergyConsumptionProfile, usize) =
            decode_from_slice(&enc1).expect("decode EnergyConsumptionProfile failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode EnergyConsumptionProfile failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 22. Mixed railway snapshot — consumed bytes check ─────────────────────────

#[test]
fn test_mixed_railway_snapshot_consumed_bytes() {
    proptest!(|(
        circuit_id: u32,
        occupied: bool,
        ma_id: u64,
        target_distance_m in 100.0f32..50_000.0f32,
        permitted_speed_kmh in 10.0f32..350.0f32,
        train_id: u64,
        traction_kw in 0.0f32..6000.0f32,
        crossing_id: u32,
        barriers_lowered: bool,
    )| {
        let track = TrackCircuitOccupancy {
            circuit_id, occupied, last_change_s: 1_700_000_000,
            impedance_ohms: 5.0, signal_strength_dbm: -60.0,
        };
        let ma = MovementAuthority {
            ma_id, target_distance_m, permitted_speed_kmh,
            eoa_speed_kmh: 0.0, timeout_s: 120, danger_point_m: 50.0, overlap_m: 100.0,
        };
        let energy = EnergyConsumptionProfile {
            train_id, traction_kw, auxiliary_kw: 80.0, regen_kw: 0.0,
            pantograph_v: 25_000.0, line_current_a: 200.0, cumulative_kwh: 500.0,
        };
        let crossing = LevelCrossingStatus {
            crossing_id, barriers_lowered, lights_active: barriers_lowered,
            audible_warning: barriers_lowered, activation_time_s: 30,
            vehicles_detected: 0, obstacle_alarm: false,
        };

        let enc_track = encode_to_vec(&track).expect("encode track circuit snapshot failed");
        let enc_ma = encode_to_vec(&ma).expect("encode movement authority snapshot failed");
        let enc_energy = encode_to_vec(&energy).expect("encode energy profile snapshot failed");
        let enc_crossing = encode_to_vec(&crossing).expect("encode level crossing snapshot failed");

        let (_, c_track): (TrackCircuitOccupancy, usize) =
            decode_from_slice(&enc_track).expect("decode track circuit snapshot failed");
        let (_, c_ma): (MovementAuthority, usize) =
            decode_from_slice(&enc_ma).expect("decode movement authority snapshot failed");
        let (_, c_energy): (EnergyConsumptionProfile, usize) =
            decode_from_slice(&enc_energy).expect("decode energy profile snapshot failed");
        let (_, c_crossing): (LevelCrossingStatus, usize) =
            decode_from_slice(&enc_crossing).expect("decode level crossing snapshot failed");

        prop_assert_eq!(c_track, enc_track.len(), "track consumed bytes mismatch");
        prop_assert_eq!(c_ma, enc_ma.len(), "MA consumed bytes mismatch");
        prop_assert_eq!(c_energy, enc_energy.len(), "energy consumed bytes mismatch");
        prop_assert_eq!(c_crossing, enc_crossing.len(), "crossing consumed bytes mismatch");
    });
}
