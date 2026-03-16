//! Advanced property-based tests (set 62) using proptest.
//!
//! 22 property-based tests covering traffic management / smart city transport domain.
//! All tests reside in a single proptest! block.
//! Domain: vehicle counts, signal phases, intersection control, congestion metrics,
//! incident detection, pedestrian crossings, toll collection, parking management.

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

// ── Domain structs and enums ──────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VehicleCount {
    intersection_id: u64,
    lane_id: u32,
    count: u32,
    timestamp_secs: u64,
    is_peak_hour: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SignalPhase {
    Green,
    Yellow,
    Red,
    FlashingRed,
    FlashingYellow,
    Off,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrafficSignal {
    signal_id: u64,
    intersection_id: u64,
    phase: SignalPhase,
    duration_ms: u32,
    pedestrian_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CongestionMetric {
    road_segment_id: u64,
    speed_kmh: f32,
    density_vehicles_per_km: f32,
    flow_rate: f64,
    congestion_index: f64,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IncidentSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IncidentReport {
    incident_id: u64,
    location_id: u32,
    severity: IncidentSeverity,
    lanes_blocked: u32,
    description: String,
    cleared: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PedestrianCrossing {
    crossing_id: u64,
    pedestrian_count: u32,
    wait_time_secs: u32,
    accessibility_enabled: bool,
    zone: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TollClass {
    Motorcycle,
    Car,
    LightTruck,
    HeavyTruck,
    Bus,
    EmergencyVehicle,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TollTransaction {
    transaction_id: u64,
    booth_id: u32,
    vehicle_class: TollClass,
    amount_cents: u32,
    timestamp_secs: u64,
    electronic: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ParkingStatus {
    Available,
    Occupied,
    Reserved,
    Disabled,
    Charging,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParkingSpace {
    space_id: u64,
    lot_id: u32,
    status: ParkingStatus,
    floor_level: i32,
    ev_charging: bool,
    plate_hint: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IntersectionControl {
    intersection_id: u64,
    name: String,
    signal_count: u32,
    active: bool,
    latitude: f64,
    longitude: f64,
    approaches: Vec<u32>,
}

// ── Single proptest! block with all 22 tests ──────────────────────────────────

proptest! {
    // ── 1. VehicleCount struct roundtrip ─────────────────────────────────────
    #[test]
    fn prop_vehicle_count_roundtrip(
        intersection_id: u64,
        lane_id in 0u32..32u32,
        count in 0u32..10_000u32,
        timestamp_secs: u64,
        is_peak_hour: bool,
    ) {
        let vc = VehicleCount { intersection_id, lane_id, count, timestamp_secs, is_peak_hour };
        let enc = encode_to_vec(&vc).expect("encode VehicleCount failed");
        let (decoded, _): (VehicleCount, usize) =
            decode_from_slice(&enc).expect("decode VehicleCount failed");
        prop_assert_eq!(vc, decoded);
    }

    // ── 2. SignalPhase enum roundtrip (all variants via index) ────────────────
    #[test]
    fn prop_signal_phase_enum_roundtrip(variant in 0usize..6usize) {
        let phase = match variant {
            0 => SignalPhase::Green,
            1 => SignalPhase::Yellow,
            2 => SignalPhase::Red,
            3 => SignalPhase::FlashingRed,
            4 => SignalPhase::FlashingYellow,
            _ => SignalPhase::Off,
        };
        let enc = encode_to_vec(&phase).expect("encode SignalPhase failed");
        let (decoded, _): (SignalPhase, usize) =
            decode_from_slice(&enc).expect("decode SignalPhase failed");
        prop_assert_eq!(phase, decoded);
    }

    // ── 3. TrafficSignal struct roundtrip ─────────────────────────────────────
    #[test]
    fn prop_traffic_signal_roundtrip(
        signal_id: u64,
        intersection_id: u64,
        phase_variant in 0usize..6usize,
        duration_ms in 1u32..180_000u32,
        pedestrian_active: bool,
    ) {
        let phase = match phase_variant {
            0 => SignalPhase::Green,
            1 => SignalPhase::Yellow,
            2 => SignalPhase::Red,
            3 => SignalPhase::FlashingRed,
            4 => SignalPhase::FlashingYellow,
            _ => SignalPhase::Off,
        };
        let sig = TrafficSignal { signal_id, intersection_id, phase, duration_ms, pedestrian_active };
        let enc = encode_to_vec(&sig).expect("encode TrafficSignal failed");
        let (decoded, _): (TrafficSignal, usize) =
            decode_from_slice(&enc).expect("decode TrafficSignal failed");
        prop_assert_eq!(sig, decoded);
    }

    // ── 4. Vec<VehicleCount> roundtrip ────────────────────────────────────────
    #[test]
    fn prop_vec_vehicle_count_roundtrip(
        items in prop::collection::vec(
            (any::<u64>(), 0u32..32u32, 0u32..10_000u32, any::<u64>(), any::<bool>()).prop_map(
                |(intersection_id, lane_id, count, timestamp_secs, is_peak_hour)| {
                    VehicleCount { intersection_id, lane_id, count, timestamp_secs, is_peak_hour }
                }
            ),
            0..8,
        ),
    ) {
        let enc = encode_to_vec(&items).expect("encode Vec<VehicleCount> failed");
        let (decoded, _): (Vec<VehicleCount>, usize) =
            decode_from_slice(&enc).expect("decode Vec<VehicleCount> failed");
        prop_assert_eq!(items, decoded);
    }

    // ── 5. CongestionMetric with f32/f64 fields roundtrip ────────────────────
    #[test]
    fn prop_congestion_metric_roundtrip(
        road_segment_id: u64,
        speed_kmh in 0.0f32..200.0f32,
        density_vehicles_per_km in 0.0f32..500.0f32,
        flow_rate in 0.0f64..10_000.0f64,
        congestion_index in 0.0f64..1.0f64,
        label in "[a-zA-Z0-9_]{1,20}",
    ) {
        let cm = CongestionMetric {
            road_segment_id,
            speed_kmh,
            density_vehicles_per_km,
            flow_rate,
            congestion_index,
            label,
        };
        let enc = encode_to_vec(&cm).expect("encode CongestionMetric failed");
        let (decoded, _): (CongestionMetric, usize) =
            decode_from_slice(&enc).expect("decode CongestionMetric failed");
        prop_assert_eq!(cm, decoded);
    }

    // ── 6. IncidentSeverity enum — all variants ───────────────────────────────
    #[test]
    fn prop_incident_severity_all_variants(variant in 0usize..4usize) {
        let sev = match variant {
            0 => IncidentSeverity::Minor,
            1 => IncidentSeverity::Moderate,
            2 => IncidentSeverity::Major,
            _ => IncidentSeverity::Critical,
        };
        let enc = encode_to_vec(&sev).expect("encode IncidentSeverity failed");
        let (decoded, _): (IncidentSeverity, usize) =
            decode_from_slice(&enc).expect("decode IncidentSeverity failed");
        prop_assert_eq!(sev, decoded);
    }

    // ── 7. IncidentReport nested struct roundtrip ─────────────────────────────
    #[test]
    fn prop_incident_report_roundtrip(
        incident_id: u64,
        location_id: u32,
        sev_variant in 0usize..4usize,
        lanes_blocked in 0u32..8u32,
        description in "[a-zA-Z0-9 ]{0,60}",
        cleared: bool,
    ) {
        let severity = match sev_variant {
            0 => IncidentSeverity::Minor,
            1 => IncidentSeverity::Moderate,
            2 => IncidentSeverity::Major,
            _ => IncidentSeverity::Critical,
        };
        let report = IncidentReport { incident_id, location_id, severity, lanes_blocked, description, cleared };
        let enc = encode_to_vec(&report).expect("encode IncidentReport failed");
        let (decoded, _): (IncidentReport, usize) =
            decode_from_slice(&enc).expect("decode IncidentReport failed");
        prop_assert_eq!(report, decoded);
    }

    // ── 8. PedestrianCrossing struct roundtrip ────────────────────────────────
    #[test]
    fn prop_pedestrian_crossing_roundtrip(
        crossing_id: u64,
        pedestrian_count in 0u32..500u32,
        wait_time_secs in 0u32..300u32,
        accessibility_enabled: bool,
        zone in "[A-Z]{1,5}",
    ) {
        let pc = PedestrianCrossing {
            crossing_id,
            pedestrian_count,
            wait_time_secs,
            accessibility_enabled,
            zone,
        };
        let enc = encode_to_vec(&pc).expect("encode PedestrianCrossing failed");
        let (decoded, _): (PedestrianCrossing, usize) =
            decode_from_slice(&enc).expect("decode PedestrianCrossing failed");
        prop_assert_eq!(pc, decoded);
    }

    // ── 9. TollClass enum — all 6 variants ────────────────────────────────────
    #[test]
    fn prop_toll_class_all_variants(variant in 0usize..6usize) {
        let tc = match variant {
            0 => TollClass::Motorcycle,
            1 => TollClass::Car,
            2 => TollClass::LightTruck,
            3 => TollClass::HeavyTruck,
            4 => TollClass::Bus,
            _ => TollClass::EmergencyVehicle,
        };
        let enc = encode_to_vec(&tc).expect("encode TollClass failed");
        let (decoded, _): (TollClass, usize) =
            decode_from_slice(&enc).expect("decode TollClass failed");
        prop_assert_eq!(tc, decoded);
    }

    // ── 10. TollTransaction struct roundtrip ──────────────────────────────────
    #[test]
    fn prop_toll_transaction_roundtrip(
        transaction_id: u64,
        booth_id in 0u32..1000u32,
        class_variant in 0usize..6usize,
        amount_cents in 0u32..100_000u32,
        timestamp_secs: u64,
        electronic: bool,
    ) {
        let vehicle_class = match class_variant {
            0 => TollClass::Motorcycle,
            1 => TollClass::Car,
            2 => TollClass::LightTruck,
            3 => TollClass::HeavyTruck,
            4 => TollClass::Bus,
            _ => TollClass::EmergencyVehicle,
        };
        let tx = TollTransaction { transaction_id, booth_id, vehicle_class, amount_cents, timestamp_secs, electronic };
        let enc = encode_to_vec(&tx).expect("encode TollTransaction failed");
        let (decoded, _): (TollTransaction, usize) =
            decode_from_slice(&enc).expect("decode TollTransaction failed");
        prop_assert_eq!(tx, decoded);
    }

    // ── 11. ParkingStatus enum — all 5 variants ───────────────────────────────
    #[test]
    fn prop_parking_status_all_variants(variant in 0usize..5usize) {
        let status = match variant {
            0 => ParkingStatus::Available,
            1 => ParkingStatus::Occupied,
            2 => ParkingStatus::Reserved,
            3 => ParkingStatus::Disabled,
            _ => ParkingStatus::Charging,
        };
        let enc = encode_to_vec(&status).expect("encode ParkingStatus failed");
        let (decoded, _): (ParkingStatus, usize) =
            decode_from_slice(&enc).expect("decode ParkingStatus failed");
        prop_assert_eq!(status, decoded);
    }

    // ── 12. ParkingSpace with Option<String> roundtrip ────────────────────────
    #[test]
    fn prop_parking_space_roundtrip(
        space_id: u64,
        lot_id: u32,
        status_variant in 0usize..5usize,
        floor_level in -5i32..20i32,
        ev_charging: bool,
        has_plate: bool,
        plate in "[A-Z0-9]{4,8}",
    ) {
        let status = match status_variant {
            0 => ParkingStatus::Available,
            1 => ParkingStatus::Occupied,
            2 => ParkingStatus::Reserved,
            3 => ParkingStatus::Disabled,
            _ => ParkingStatus::Charging,
        };
        let plate_hint = if has_plate { Some(plate) } else { None };
        let space = ParkingSpace { space_id, lot_id, status, floor_level, ev_charging, plate_hint };
        let enc = encode_to_vec(&space).expect("encode ParkingSpace failed");
        let (decoded, _): (ParkingSpace, usize) =
            decode_from_slice(&enc).expect("decode ParkingSpace failed");
        prop_assert_eq!(space, decoded);
    }

    // ── 13. IntersectionControl with Vec field roundtrip ─────────────────────
    #[test]
    fn prop_intersection_control_roundtrip(
        intersection_id: u64,
        name in "[a-zA-Z0-9_]{1,30}",
        signal_count in 1u32..16u32,
        active: bool,
        latitude in -90.0f64..90.0f64,
        longitude in -180.0f64..180.0f64,
        approaches in prop::collection::vec(0u32..8u32, 2..6usize),
    ) {
        let ic = IntersectionControl {
            intersection_id,
            name,
            signal_count,
            active,
            latitude,
            longitude,
            approaches,
        };
        let enc = encode_to_vec(&ic).expect("encode IntersectionControl failed");
        let (decoded, _): (IntersectionControl, usize) =
            decode_from_slice(&enc).expect("decode IntersectionControl failed");
        prop_assert_eq!(ic, decoded);
    }

    // ── 14. Option<TollTransaction> roundtrip ─────────────────────────────────
    #[test]
    fn prop_option_toll_transaction_roundtrip(
        present: bool,
        transaction_id: u64,
        booth_id in 0u32..1000u32,
        amount_cents in 0u32..50_000u32,
        timestamp_secs: u64,
        electronic: bool,
    ) {
        let opt = if present {
            Some(TollTransaction {
                transaction_id,
                booth_id,
                vehicle_class: TollClass::Car,
                amount_cents,
                timestamp_secs,
                electronic,
            })
        } else {
            None
        };
        let enc = encode_to_vec(&opt).expect("encode Option<TollTransaction> failed");
        let (decoded, _): (Option<TollTransaction>, usize) =
            decode_from_slice(&enc).expect("decode Option<TollTransaction> failed");
        prop_assert_eq!(opt, decoded);
    }

    // ── 15. Vec<ParkingSpace> roundtrip ──────────────────────────────────────
    #[test]
    fn prop_vec_parking_space_roundtrip(
        items in prop::collection::vec(
            (any::<u64>(), any::<u32>(), 0usize..5usize, -5i32..20i32, any::<bool>()).prop_map(
                |(space_id, lot_id, sv, floor_level, ev_charging)| {
                    let status = match sv {
                        0 => ParkingStatus::Available,
                        1 => ParkingStatus::Occupied,
                        2 => ParkingStatus::Reserved,
                        3 => ParkingStatus::Disabled,
                        _ => ParkingStatus::Charging,
                    };
                    ParkingSpace { space_id, lot_id, status, floor_level, ev_charging, plate_hint: None }
                }
            ),
            0..6,
        ),
    ) {
        let enc = encode_to_vec(&items).expect("encode Vec<ParkingSpace> failed");
        let (decoded, _): (Vec<ParkingSpace>, usize) =
            decode_from_slice(&enc).expect("decode Vec<ParkingSpace> failed");
        prop_assert_eq!(items, decoded);
    }

    // ── 16. Consumed bytes verification for VehicleCount ─────────────────────
    #[test]
    fn prop_vehicle_count_consumed_bytes(
        intersection_id: u64,
        lane_id in 0u32..32u32,
        count in 0u32..5000u32,
        timestamp_secs: u64,
        is_peak_hour: bool,
    ) {
        let vc = VehicleCount { intersection_id, lane_id, count, timestamp_secs, is_peak_hour };
        let enc = encode_to_vec(&vc).expect("encode VehicleCount failed");
        let (_, consumed): (VehicleCount, usize) =
            decode_from_slice(&enc).expect("decode VehicleCount failed");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal encoded length");
    }

    // ── 17. Deterministic encoding for TrafficSignal ──────────────────────────
    #[test]
    fn prop_traffic_signal_deterministic(
        signal_id: u64,
        intersection_id: u64,
        duration_ms in 1u32..60_000u32,
        pedestrian_active: bool,
    ) {
        let sig = TrafficSignal {
            signal_id,
            intersection_id,
            phase: SignalPhase::Green,
            duration_ms,
            pedestrian_active,
        };
        let enc1 = encode_to_vec(&sig).expect("first encode TrafficSignal failed");
        let enc2 = encode_to_vec(&sig).expect("second encode TrafficSignal failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    }

    // ── 18. Re-encoding decoded IncidentReport gives identical bytes ───────────
    #[test]
    fn prop_incident_report_reencode_idempotent(
        incident_id: u64,
        location_id: u32,
        lanes_blocked in 0u32..8u32,
        description in "[a-zA-Z0-9 ]{0,40}",
        cleared: bool,
    ) {
        let report = IncidentReport {
            incident_id,
            location_id,
            severity: IncidentSeverity::Moderate,
            lanes_blocked,
            description,
            cleared,
        };
        let enc1 = encode_to_vec(&report).expect("first encode IncidentReport failed");
        let (decoded, _): (IncidentReport, usize) =
            decode_from_slice(&enc1).expect("decode IncidentReport failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode IncidentReport failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    }

    // ── 19. i32 floor_level roundtrip (parking floor levels can be negative) ──
    #[test]
    fn prop_i32_floor_level_roundtrip(floor_level: i32) {
        let enc = encode_to_vec(&floor_level).expect("encode i32 floor level failed");
        let (decoded, _): (i32, usize) =
            decode_from_slice(&enc).expect("decode i32 floor level failed");
        prop_assert_eq!(floor_level, decoded);
    }

    // ── 20. u64 timestamp roundtrip ───────────────────────────────────────────
    #[test]
    fn prop_u64_timestamp_roundtrip(timestamp_secs: u64) {
        let enc = encode_to_vec(&timestamp_secs).expect("encode u64 timestamp failed");
        let (decoded, _): (u64, usize) =
            decode_from_slice(&enc).expect("decode u64 timestamp failed");
        prop_assert_eq!(timestamp_secs, decoded);
    }

    // ── 21. bool active flag roundtrip ────────────────────────────────────────
    #[test]
    fn prop_bool_active_flag_roundtrip(active: bool) {
        let enc = encode_to_vec(&active).expect("encode bool active flag failed");
        let (decoded, _): (bool, usize) =
            decode_from_slice(&enc).expect("decode bool active flag failed");
        prop_assert_eq!(active, decoded);
    }

    // ── 22. String road label roundtrip ───────────────────────────────────────
    #[test]
    fn prop_string_road_label_roundtrip(label in "[a-zA-Z0-9_\\- ]{0,64}") {
        let enc = encode_to_vec(&label).expect("encode String road label failed");
        let (decoded, _): (String, usize) =
            decode_from_slice(&enc).expect("decode String road label failed");
        prop_assert_eq!(label, decoded);
    }
}
