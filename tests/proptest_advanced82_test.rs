//! Advanced property-based tests (set 82) — Autonomous Warehouse Robotics domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers AGV navigation, bin storage locations, pick wave assignments,
//! goods-to-person robot commands, conveyor sorter divert decisions, packing
//! station configs, inventory cycle counts, receiving dock appointments, putaway
//! optimization, zone pick routing, replenishment triggers, robot battery swap
//! scheduling, collision avoidance grids, throughput monitoring, and warehouse
//! slotting optimization.

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

/// 2-D position on the warehouse floor grid.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridPosition {
    /// Column index (0-based).
    col: u16,
    /// Row index (0-based).
    row: u16,
}

/// Heading of an AGV expressed as a cardinal/intercardinal direction.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AgvHeading {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Full AGV navigation state snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AgvNavState {
    agv_id: u32,
    position: GridPosition,
    heading: AgvHeading,
    speed_mm_per_s: u32,
    battery_pct: u8,
    payload_kg: u16,
    estop_active: bool,
}

/// Physical bin storage location inside the warehouse.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BinLocation {
    aisle: u16,
    bay: u16,
    level: u8,
    slot: u8,
    zone_id: u8,
}

/// A single order line within a pick wave.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PickLine {
    sku_id: u64,
    quantity: u16,
    bin: BinLocation,
    priority: u8,
}

/// An entire pick wave assignment dispatched to a worker or robot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PickWaveAssignment {
    wave_id: u32,
    assignee_id: u32,
    lines: Vec<PickLine>,
    deadline_epoch_s: u64,
    is_express: bool,
}

/// Command sent to a goods-to-person (GTP) robot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GtpRobotCommand {
    FetchPod {
        pod_id: u64,
        destination: GridPosition,
    },
    ReturnPod {
        pod_id: u64,
        home: GridPosition,
    },
    Charge {
        station_id: u16,
    },
    Halt,
    Resume {
        target: GridPosition,
    },
}

/// Decision a conveyor sorter takes at a divert point.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DivertDecision {
    StraightThrough,
    DivertLeft { lane_id: u16 },
    DivertRight { lane_id: u16 },
    Recirculate,
    Reject { reason_code: u8 },
}

/// Full sorter divert event including parcel context.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SorterDivertEvent {
    scan_id: u64,
    parcel_weight_g: u32,
    decision: DivertDecision,
    divert_point_id: u16,
    timestamp_ms: u64,
}

/// Configuration for a packing station.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PackingStationConfig {
    station_id: u16,
    max_weight_g: u32,
    max_length_mm: u16,
    max_width_mm: u16,
    max_height_mm: u16,
    supported_carrier_ids: Vec<u16>,
    auto_tape_enabled: bool,
    scale_tare_g: u16,
}

/// Result of an inventory cycle count for a single bin.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CycleCountResult {
    bin: BinLocation,
    sku_id: u64,
    expected_qty: u32,
    counted_qty: u32,
    variance: i32,
    counter_id: u32,
    timestamp_epoch_s: u64,
}

/// A receiving dock appointment for an inbound trailer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DockAppointment {
    appointment_id: u64,
    dock_door: u8,
    carrier_code: u16,
    expected_pallet_count: u16,
    arrival_window_start_s: u64,
    arrival_window_end_s: u64,
    is_cross_dock: bool,
}

/// Putaway instruction telling where to store received goods.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PutawayInstruction {
    sku_id: u64,
    quantity: u32,
    target_bin: BinLocation,
    weight_per_unit_g: u32,
    fragile: bool,
    hazmat_class: u8,
}

/// Optimised putaway plan containing multiple instructions.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PutawayPlan {
    plan_id: u64,
    instructions: Vec<PutawayInstruction>,
    total_units: u32,
    estimated_minutes: u16,
}

/// A single step in a zone-pick routing plan.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ZonePickStep {
    step_index: u16,
    bin: BinLocation,
    sku_id: u64,
    pick_qty: u16,
    walk_distance_cm: u32,
}

/// Complete zone-pick route for a picker.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ZonePickRoute {
    route_id: u32,
    zone_id: u8,
    picker_id: u32,
    steps: Vec<ZonePickStep>,
    total_distance_cm: u64,
}

/// Trigger condition for an automatic replenishment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReplenishmentTrigger {
    MinQtyThreshold {
        sku_id: u64,
        current_qty: u32,
        min_qty: u32,
    },
    ScheduledInterval {
        sku_id: u64,
        interval_hours: u16,
    },
    DemandForecast {
        sku_id: u64,
        forecast_units: u32,
        confidence_pct: u8,
    },
    ManualOverride {
        sku_id: u64,
        requested_by: u32,
    },
}

/// Schedule entry for a robot battery swap.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatterySwapSchedule {
    robot_id: u32,
    current_charge_pct: u8,
    swap_station_id: u16,
    scheduled_epoch_s: u64,
    estimated_swap_duration_s: u16,
    is_urgent: bool,
}

/// A single cell in a collision-avoidance grid.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GridCellState {
    Free,
    OccupiedByAgv { agv_id: u32 },
    StaticObstacle,
    TemporaryBlock { expires_epoch_s: u64 },
    Reserved { agv_id: u32, until_epoch_s: u64 },
}

/// Snapshot of a rectangular region of the collision-avoidance grid.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollisionGridSnapshot {
    origin: GridPosition,
    width: u16,
    height: u16,
    cells: Vec<GridCellState>,
    snapshot_epoch_ms: u64,
}

/// Throughput measurement for a conveyor segment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThroughputMeasurement {
    segment_id: u16,
    parcels_per_hour: u32,
    avg_weight_g: u32,
    peak_rate_per_min: u16,
    jam_events: u16,
    uptime_pct: u8,
    measurement_window_s: u32,
}

/// Slotting recommendation for optimising pick face allocation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlottingRecommendation {
    sku_id: u64,
    current_bin: BinLocation,
    proposed_bin: BinLocation,
    velocity_rank: u32,
    ergonomic_score: u8,
    pick_frequency_per_day: u16,
    estimated_time_saved_s: u32,
}

// ── Strategies ───────────────────────────────────────────────────────────────

fn arb_grid_position() -> impl Strategy<Value = GridPosition> {
    (0..500u16, 0..500u16).prop_map(|(col, row)| GridPosition { col, row })
}

fn arb_heading() -> impl Strategy<Value = AgvHeading> {
    prop_oneof![
        Just(AgvHeading::North),
        Just(AgvHeading::NorthEast),
        Just(AgvHeading::East),
        Just(AgvHeading::SouthEast),
        Just(AgvHeading::South),
        Just(AgvHeading::SouthWest),
        Just(AgvHeading::West),
        Just(AgvHeading::NorthWest),
    ]
}

fn arb_agv_nav_state() -> impl Strategy<Value = AgvNavState> {
    (
        any::<u32>(),
        arb_grid_position(),
        arb_heading(),
        0..5000u32,
        0..=100u8,
        0..2000u16,
        any::<bool>(),
    )
        .prop_map(
            |(agv_id, position, heading, speed, bat, payload, estop)| AgvNavState {
                agv_id,
                position,
                heading,
                speed_mm_per_s: speed,
                battery_pct: bat,
                payload_kg: payload,
                estop_active: estop,
            },
        )
}

fn arb_bin_location() -> impl Strategy<Value = BinLocation> {
    (0..200u16, 0..100u16, 0..8u8, 0..4u8, 0..12u8).prop_map(
        |(aisle, bay, level, slot, zone_id)| BinLocation {
            aisle,
            bay,
            level,
            slot,
            zone_id,
        },
    )
}

fn arb_pick_line() -> impl Strategy<Value = PickLine> {
    (any::<u64>(), 1..200u16, arb_bin_location(), 0..5u8).prop_map(
        |(sku_id, quantity, bin, priority)| PickLine {
            sku_id,
            quantity,
            bin,
            priority,
        },
    )
}

prop_compose! {
    fn arb_pick_wave_assignment()(
        wave_id in any::<u32>(),
        assignee_id in any::<u32>(),
        lines in prop::collection::vec(arb_pick_line(), 0..8),
        deadline_epoch_s in any::<u64>(),
        is_express in any::<bool>(),
    ) -> PickWaveAssignment {
        PickWaveAssignment { wave_id, assignee_id, lines, deadline_epoch_s, is_express }
    }
}

fn arb_gtp_command() -> impl Strategy<Value = GtpRobotCommand> {
    prop_oneof![
        (any::<u64>(), arb_grid_position()).prop_map(|(pod_id, dest)| GtpRobotCommand::FetchPod {
            pod_id,
            destination: dest,
        }),
        (any::<u64>(), arb_grid_position())
            .prop_map(|(pod_id, home)| GtpRobotCommand::ReturnPod { pod_id, home }),
        (0..100u16).prop_map(|sid| GtpRobotCommand::Charge { station_id: sid }),
        Just(GtpRobotCommand::Halt),
        arb_grid_position().prop_map(|t| GtpRobotCommand::Resume { target: t }),
    ]
}

fn arb_divert_decision() -> impl Strategy<Value = DivertDecision> {
    prop_oneof![
        Just(DivertDecision::StraightThrough),
        (0..64u16).prop_map(|lid| DivertDecision::DivertLeft { lane_id: lid }),
        (0..64u16).prop_map(|lid| DivertDecision::DivertRight { lane_id: lid }),
        Just(DivertDecision::Recirculate),
        any::<u8>().prop_map(|rc| DivertDecision::Reject { reason_code: rc }),
    ]
}

prop_compose! {
    fn arb_sorter_divert_event()(
        scan_id in any::<u64>(),
        parcel_weight_g in 1..50_000u32,
        decision in arb_divert_decision(),
        divert_point_id in any::<u16>(),
        timestamp_ms in any::<u64>(),
    ) -> SorterDivertEvent {
        SorterDivertEvent { scan_id, parcel_weight_g, decision, divert_point_id, timestamp_ms }
    }
}

prop_compose! {
    fn arb_packing_station_config()(
        station_id in any::<u16>(),
        max_weight_g in 1..100_000u32,
        max_length_mm in 100..2000u16,
        max_width_mm in 100..1500u16,
        max_height_mm in 50..1000u16,
        supported_carrier_ids in prop::collection::vec(any::<u16>(), 0..6),
        auto_tape_enabled in any::<bool>(),
        scale_tare_g in 0..500u16,
    ) -> PackingStationConfig {
        PackingStationConfig {
            station_id, max_weight_g, max_length_mm, max_width_mm,
            max_height_mm, supported_carrier_ids, auto_tape_enabled, scale_tare_g,
        }
    }
}

prop_compose! {
    fn arb_cycle_count_result()(
        bin in arb_bin_location(),
        sku_id in any::<u64>(),
        expected_qty in 0..10_000u32,
        counted_qty in 0..10_000u32,
        counter_id in any::<u32>(),
        timestamp_epoch_s in any::<u64>(),
    ) -> CycleCountResult {
        let variance = counted_qty as i32 - expected_qty as i32;
        CycleCountResult { bin, sku_id, expected_qty, counted_qty, variance, counter_id, timestamp_epoch_s }
    }
}

prop_compose! {
    fn arb_dock_appointment()(
        appointment_id in any::<u64>(),
        dock_door in 0..50u8,
        carrier_code in any::<u16>(),
        expected_pallet_count in 1..60u16,
        arrival_window_start_s in any::<u64>(),
        window_len in 1800..14400u64,
        is_cross_dock in any::<bool>(),
    ) -> DockAppointment {
        DockAppointment {
            appointment_id, dock_door, carrier_code, expected_pallet_count,
            arrival_window_start_s,
            arrival_window_end_s: arrival_window_start_s.wrapping_add(window_len),
            is_cross_dock,
        }
    }
}

prop_compose! {
    fn arb_putaway_instruction()(
        sku_id in any::<u64>(),
        quantity in 1..5_000u32,
        target_bin in arb_bin_location(),
        weight_per_unit_g in 1..50_000u32,
        fragile in any::<bool>(),
        hazmat_class in 0..10u8,
    ) -> PutawayInstruction {
        PutawayInstruction { sku_id, quantity, target_bin, weight_per_unit_g, fragile, hazmat_class }
    }
}

prop_compose! {
    fn arb_putaway_plan()(
        plan_id in any::<u64>(),
        instructions in prop::collection::vec(arb_putaway_instruction(), 0..6),
        estimated_minutes in 1..480u16,
    ) -> PutawayPlan {
        let total_units = instructions.iter().map(|i| i.quantity).sum();
        PutawayPlan { plan_id, instructions, total_units, estimated_minutes }
    }
}

fn arb_zone_pick_step() -> impl Strategy<Value = ZonePickStep> {
    (
        0..500u16,
        arb_bin_location(),
        any::<u64>(),
        1..200u16,
        0..100_000u32,
    )
        .prop_map(
            |(step_index, bin, sku_id, pick_qty, walk_distance_cm)| ZonePickStep {
                step_index,
                bin,
                sku_id,
                pick_qty,
                walk_distance_cm,
            },
        )
}

prop_compose! {
    fn arb_zone_pick_route()(
        route_id in any::<u32>(),
        zone_id in 0..12u8,
        picker_id in any::<u32>(),
        steps in prop::collection::vec(arb_zone_pick_step(), 0..8),
    ) -> ZonePickRoute {
        let total_distance_cm = steps.iter().map(|s| s.walk_distance_cm as u64).sum();
        ZonePickRoute { route_id, zone_id, picker_id, steps, total_distance_cm }
    }
}

fn arb_replenishment_trigger() -> impl Strategy<Value = ReplenishmentTrigger> {
    prop_oneof![
        (any::<u64>(), 0..1000u32, 1..500u32).prop_map(|(sku, cur, min)| {
            ReplenishmentTrigger::MinQtyThreshold {
                sku_id: sku,
                current_qty: cur,
                min_qty: min,
            }
        }),
        (any::<u64>(), 1..720u16).prop_map(|(sku, hrs)| {
            ReplenishmentTrigger::ScheduledInterval {
                sku_id: sku,
                interval_hours: hrs,
            }
        }),
        (any::<u64>(), 1..50_000u32, 1..100u8).prop_map(|(sku, fc, conf)| {
            ReplenishmentTrigger::DemandForecast {
                sku_id: sku,
                forecast_units: fc,
                confidence_pct: conf,
            }
        }),
        (any::<u64>(), any::<u32>()).prop_map(|(sku, by)| {
            ReplenishmentTrigger::ManualOverride {
                sku_id: sku,
                requested_by: by,
            }
        }),
    ]
}

prop_compose! {
    fn arb_battery_swap_schedule()(
        robot_id in any::<u32>(),
        current_charge_pct in 0..=100u8,
        swap_station_id in 0..50u16,
        scheduled_epoch_s in any::<u64>(),
        estimated_swap_duration_s in 30..600u16,
        is_urgent in any::<bool>(),
    ) -> BatterySwapSchedule {
        BatterySwapSchedule {
            robot_id, current_charge_pct, swap_station_id,
            scheduled_epoch_s, estimated_swap_duration_s, is_urgent,
        }
    }
}

fn arb_grid_cell_state() -> impl Strategy<Value = GridCellState> {
    prop_oneof![
        Just(GridCellState::Free),
        any::<u32>().prop_map(|id| GridCellState::OccupiedByAgv { agv_id: id }),
        Just(GridCellState::StaticObstacle),
        any::<u64>().prop_map(|exp| GridCellState::TemporaryBlock {
            expires_epoch_s: exp,
        }),
        (any::<u32>(), any::<u64>()).prop_map(|(id, until)| GridCellState::Reserved {
            agv_id: id,
            until_epoch_s: until,
        }),
    ]
}

prop_compose! {
    fn arb_collision_grid_snapshot()(
        origin in arb_grid_position(),
        w in 1..6u16,
        h in 1..6u16,
    )(
        origin in Just(origin),
        width in Just(w),
        height in Just(h),
        cells in prop::collection::vec(arb_grid_cell_state(), (w as usize) * (h as usize)),
        snapshot_epoch_ms in any::<u64>(),
    ) -> CollisionGridSnapshot {
        CollisionGridSnapshot { origin, width, height, cells, snapshot_epoch_ms }
    }
}

prop_compose! {
    fn arb_throughput_measurement()(
        segment_id in any::<u16>(),
        parcels_per_hour in 0..20_000u32,
        avg_weight_g in 1..50_000u32,
        peak_rate_per_min in 0..500u16,
        jam_events in 0..200u16,
        uptime_pct in 0..=100u8,
        measurement_window_s in 60..86_400u32,
    ) -> ThroughputMeasurement {
        ThroughputMeasurement {
            segment_id, parcels_per_hour, avg_weight_g,
            peak_rate_per_min, jam_events, uptime_pct, measurement_window_s,
        }
    }
}

prop_compose! {
    fn arb_slotting_recommendation()(
        sku_id in any::<u64>(),
        current_bin in arb_bin_location(),
        proposed_bin in arb_bin_location(),
        velocity_rank in 1..100_000u32,
        ergonomic_score in 1..100u8,
        pick_frequency_per_day in 0..2000u16,
        estimated_time_saved_s in 0..3600u32,
    ) -> SlottingRecommendation {
        SlottingRecommendation {
            sku_id, current_bin, proposed_bin, velocity_rank,
            ergonomic_score, pick_frequency_per_day, estimated_time_saved_s,
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_agv_nav_state_roundtrip() {
    proptest!(|(val in arb_agv_nav_state())| {
        let encoded = encode_to_vec(&val).expect("encode AgvNavState failed");
        let (decoded, _) = decode_from_slice::<AgvNavState>(&encoded)
            .expect("decode AgvNavState failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_bin_location_roundtrip() {
    proptest!(|(val in arb_bin_location())| {
        let encoded = encode_to_vec(&val).expect("encode BinLocation failed");
        let (decoded, _) = decode_from_slice::<BinLocation>(&encoded)
            .expect("decode BinLocation failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_pick_wave_assignment_roundtrip() {
    proptest!(|(val in arb_pick_wave_assignment())| {
        let encoded = encode_to_vec(&val).expect("encode PickWaveAssignment failed");
        let (decoded, _) = decode_from_slice::<PickWaveAssignment>(&encoded)
            .expect("decode PickWaveAssignment failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_gtp_robot_command_roundtrip() {
    proptest!(|(val in arb_gtp_command())| {
        let encoded = encode_to_vec(&val).expect("encode GtpRobotCommand failed");
        let (decoded, _) = decode_from_slice::<GtpRobotCommand>(&encoded)
            .expect("decode GtpRobotCommand failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_sorter_divert_event_roundtrip() {
    proptest!(|(val in arb_sorter_divert_event())| {
        let encoded = encode_to_vec(&val).expect("encode SorterDivertEvent failed");
        let (decoded, _) = decode_from_slice::<SorterDivertEvent>(&encoded)
            .expect("decode SorterDivertEvent failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_packing_station_config_roundtrip() {
    proptest!(|(val in arb_packing_station_config())| {
        let encoded = encode_to_vec(&val).expect("encode PackingStationConfig failed");
        let (decoded, _) = decode_from_slice::<PackingStationConfig>(&encoded)
            .expect("decode PackingStationConfig failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_cycle_count_result_roundtrip() {
    proptest!(|(val in arb_cycle_count_result())| {
        let encoded = encode_to_vec(&val).expect("encode CycleCountResult failed");
        let (decoded, _) = decode_from_slice::<CycleCountResult>(&encoded)
            .expect("decode CycleCountResult failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_dock_appointment_roundtrip() {
    proptest!(|(val in arb_dock_appointment())| {
        let encoded = encode_to_vec(&val).expect("encode DockAppointment failed");
        let (decoded, _) = decode_from_slice::<DockAppointment>(&encoded)
            .expect("decode DockAppointment failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_putaway_plan_roundtrip() {
    proptest!(|(val in arb_putaway_plan())| {
        let encoded = encode_to_vec(&val).expect("encode PutawayPlan failed");
        let (decoded, _) = decode_from_slice::<PutawayPlan>(&encoded)
            .expect("decode PutawayPlan failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_zone_pick_route_roundtrip() {
    proptest!(|(val in arb_zone_pick_route())| {
        let encoded = encode_to_vec(&val).expect("encode ZonePickRoute failed");
        let (decoded, _) = decode_from_slice::<ZonePickRoute>(&encoded)
            .expect("decode ZonePickRoute failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_replenishment_trigger_roundtrip() {
    proptest!(|(val in arb_replenishment_trigger())| {
        let encoded = encode_to_vec(&val).expect("encode ReplenishmentTrigger failed");
        let (decoded, _) = decode_from_slice::<ReplenishmentTrigger>(&encoded)
            .expect("decode ReplenishmentTrigger failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_battery_swap_schedule_roundtrip() {
    proptest!(|(val in arb_battery_swap_schedule())| {
        let encoded = encode_to_vec(&val).expect("encode BatterySwapSchedule failed");
        let (decoded, _) = decode_from_slice::<BatterySwapSchedule>(&encoded)
            .expect("decode BatterySwapSchedule failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_collision_grid_snapshot_roundtrip() {
    proptest!(|(val in arb_collision_grid_snapshot())| {
        let encoded = encode_to_vec(&val).expect("encode CollisionGridSnapshot failed");
        let (decoded, _) = decode_from_slice::<CollisionGridSnapshot>(&encoded)
            .expect("decode CollisionGridSnapshot failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_throughput_measurement_roundtrip() {
    proptest!(|(val in arb_throughput_measurement())| {
        let encoded = encode_to_vec(&val).expect("encode ThroughputMeasurement failed");
        let (decoded, _) = decode_from_slice::<ThroughputMeasurement>(&encoded)
            .expect("decode ThroughputMeasurement failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_slotting_recommendation_roundtrip() {
    proptest!(|(val in arb_slotting_recommendation())| {
        let encoded = encode_to_vec(&val).expect("encode SlottingRecommendation failed");
        let (decoded, _) = decode_from_slice::<SlottingRecommendation>(&encoded)
            .expect("decode SlottingRecommendation failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_vec_of_agv_nav_states_roundtrip() {
    proptest!(|(val in prop::collection::vec(arb_agv_nav_state(), 0..10))| {
        let encoded = encode_to_vec(&val).expect("encode Vec<AgvNavState> failed");
        let (decoded, _) = decode_from_slice::<Vec<AgvNavState>>(&encoded)
            .expect("decode Vec<AgvNavState> failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_vec_of_gtp_commands_roundtrip() {
    proptest!(|(val in prop::collection::vec(arb_gtp_command(), 0..12))| {
        let encoded = encode_to_vec(&val).expect("encode Vec<GtpRobotCommand> failed");
        let (decoded, _) = decode_from_slice::<Vec<GtpRobotCommand>>(&encoded)
            .expect("decode Vec<GtpRobotCommand> failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_vec_of_replenishment_triggers_roundtrip() {
    proptest!(|(val in prop::collection::vec(arb_replenishment_trigger(), 0..10))| {
        let encoded = encode_to_vec(&val).expect("encode Vec<ReplenishmentTrigger> failed");
        let (decoded, _) = decode_from_slice::<Vec<ReplenishmentTrigger>>(&encoded)
            .expect("decode Vec<ReplenishmentTrigger> failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_option_dock_appointment_roundtrip() {
    proptest!(|(val in prop::option::of(arb_dock_appointment()))| {
        let encoded = encode_to_vec(&val).expect("encode Option<DockAppointment> failed");
        let (decoded, _) = decode_from_slice::<Option<DockAppointment>>(&encoded)
            .expect("decode Option<DockAppointment> failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_tuple_bin_location_and_cycle_count_roundtrip() {
    proptest!(|(bl in arb_bin_location(), cc in arb_cycle_count_result())| {
        let val = (bl, cc);
        let encoded = encode_to_vec(&val).expect("encode (BinLocation, CycleCountResult) failed");
        let (decoded, _) = decode_from_slice::<(BinLocation, CycleCountResult)>(&encoded)
            .expect("decode (BinLocation, CycleCountResult) failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_nested_option_vec_pick_wave_roundtrip() {
    proptest!(|(val in prop::option::of(prop::collection::vec(arb_pick_wave_assignment(), 0..5)))| {
        let encoded = encode_to_vec(&val)
            .expect("encode Option<Vec<PickWaveAssignment>> failed");
        let (decoded, _) = decode_from_slice::<Option<Vec<PickWaveAssignment>>>(&encoded)
            .expect("decode Option<Vec<PickWaveAssignment>> failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_vec_of_slotting_recommendations_roundtrip() {
    proptest!(|(val in prop::collection::vec(arb_slotting_recommendation(), 0..8))| {
        let encoded = encode_to_vec(&val)
            .expect("encode Vec<SlottingRecommendation> failed");
        let (decoded, _) = decode_from_slice::<Vec<SlottingRecommendation>>(&encoded)
            .expect("decode Vec<SlottingRecommendation> failed");
        prop_assert_eq!(val, decoded);
    });
}
