//! Factory/PLC/SCADA/production-focused tests for nested_structs_advanced5 (split from nested_structs_advanced5_test.rs).

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

// ---------------------------------------------------------------------------
// Domain types: Factory floor layout
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Position3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MachineSpec {
    manufacturer: String,
    model_number: String,
    rated_power_kw: f64,
    max_rpm: u32,
    voltage: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Machine {
    id: u64,
    name: String,
    position: Position3D,
    dimensions: Dimensions,
    spec: MachineSpec,
    operational: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConveyorSegment {
    segment_id: u32,
    start: Position3D,
    end: Position3D,
    speed_mps: f64,
    belt_width_mm: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Conveyor {
    id: u64,
    name: String,
    segments: Vec<ConveyorSegment>,
    reversible: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Workstation {
    id: u32,
    label: String,
    operator_count: u8,
    machines: Vec<Machine>,
    input_conveyor: Option<Conveyor>,
    output_conveyor: Option<Conveyor>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FactoryFloor {
    floor_id: String,
    building: String,
    stations: Vec<Workstation>,
    area_sqm: f64,
}

// ---------------------------------------------------------------------------
// PLC register snapshots
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlcRegister {
    address: u16,
    value: u32,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlcModule {
    slot: u8,
    module_type: String,
    registers: Vec<PlcRegister>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlcSnapshot {
    plc_id: String,
    firmware_version: String,
    scan_cycle_us: u32,
    modules: Vec<PlcModule>,
    timestamp_ms: u64,
}

// ---------------------------------------------------------------------------
// SCADA alarm tree
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlarmCondition {
    tag: String,
    threshold: f64,
    comparison: String,
    severity: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlarmGroup {
    group_name: String,
    conditions: Vec<AlarmCondition>,
    sub_groups: Vec<AlarmGroup>,
    enabled: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScadaAlarmTree {
    plant_section: String,
    root_groups: Vec<AlarmGroup>,
    total_alarm_count: u32,
}

// ---------------------------------------------------------------------------
// Production KPIs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OeeMetrics {
    availability: f64,
    performance: f64,
    quality: f64,
    overall: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReliabilityKpis {
    mtbf_hours: f64,
    mttr_hours: f64,
    failure_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ShiftKpi {
    shift_id: String,
    oee: OeeMetrics,
    reliability: ReliabilityKpis,
    units_produced: u64,
    scrap_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionKpiReport {
    line_id: String,
    report_date: String,
    shifts: Vec<ShiftKpi>,
}

// ---------------------------------------------------------------------------
// PID process control loops
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PidTuning {
    kp: f64,
    ki: f64,
    kd: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ControlLoop {
    tag: String,
    setpoint: f64,
    process_variable: f64,
    output_percent: f64,
    tuning: PidTuning,
    in_auto: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProcessControlUnit {
    unit_name: String,
    loops: Vec<ControlLoop>,
    cascade_pairs: Vec<(String, String)>,
}

// ===========================================================================
// Helper constructors
// ===========================================================================

fn pos(x: f64, y: f64, z: f64) -> Position3D {
    Position3D { x, y, z }
}

fn dims(w: f64, h: f64, d: f64) -> Dimensions {
    Dimensions {
        width: w,
        height: h,
        depth: d,
    }
}

fn make_machine(id: u64, name: &str, operational: bool) -> Machine {
    Machine {
        id,
        name: name.to_string(),
        position: pos(id as f64 * 2.0, 0.0, 0.0),
        dimensions: dims(2.0, 3.0, 1.5),
        spec: MachineSpec {
            manufacturer: "Siemens".to_string(),
            model_number: format!("SM-{id}"),
            rated_power_kw: 22.0,
            max_rpm: 3600,
            voltage: 480,
        },
        operational,
    }
}

fn make_conveyor(id: u64, name: &str, seg_count: u32) -> Conveyor {
    let segments = (0..seg_count)
        .map(|i| ConveyorSegment {
            segment_id: i,
            start: pos(i as f64, 0.0, 1.0),
            end: pos(i as f64 + 1.0, 0.0, 1.0),
            speed_mps: 0.5,
            belt_width_mm: 600,
        })
        .collect();
    Conveyor {
        id,
        name: name.to_string(),
        segments,
        reversible: false,
    }
}

fn make_workstation(id: u32, machine_count: usize) -> Workstation {
    let machines = (0..machine_count)
        .map(|i| make_machine(id as u64 * 100 + i as u64, &format!("M-{id}-{i}"), true))
        .collect();
    Workstation {
        id,
        label: format!("WS-{id}"),
        operator_count: 2,
        machines,
        input_conveyor: Some(make_conveyor(id as u64 * 10, "InConv", 3)),
        output_conveyor: Some(make_conveyor(id as u64 * 10 + 1, "OutConv", 2)),
    }
}

fn make_factory_floor() -> FactoryFloor {
    FactoryFloor {
        floor_id: "F-001".to_string(),
        building: "Plant-A".to_string(),
        stations: vec![make_workstation(1, 2), make_workstation(2, 3)],
        area_sqm: 5000.0,
    }
}

fn roundtrip<T: Encode + Decode + std::fmt::Debug + PartialEq>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {ctx}"));
    let (decoded, _): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {ctx}"));
    assert_eq!(val, &decoded, "roundtrip failed for {ctx}");
}

// ===========================================================================
// Tests
// ===========================================================================

// Test 1: Factory floor layout with nested machines, conveyors, and workstations
#[test]
fn test_factory_floor_layout_roundtrip() {
    let floor = make_factory_floor();
    roundtrip(&floor, "factory floor layout");
}

// Test 2: PLC register snapshot with multiple modules
#[test]
fn test_plc_snapshot_roundtrip() {
    let snapshot = PlcSnapshot {
        plc_id: "PLC-01".to_string(),
        firmware_version: "4.2.1".to_string(),
        scan_cycle_us: 500,
        modules: vec![
            PlcModule {
                slot: 0,
                module_type: "DI-16".to_string(),
                registers: vec![
                    PlcRegister {
                        address: 0x0000,
                        value: 0xFFFF,
                        label: "DI_Word0".to_string(),
                    },
                    PlcRegister {
                        address: 0x0001,
                        value: 0x00FF,
                        label: "DI_Word1".to_string(),
                    },
                ],
            },
            PlcModule {
                slot: 1,
                module_type: "AO-8".to_string(),
                registers: vec![
                    PlcRegister {
                        address: 0x0100,
                        value: 16384,
                        label: "AO_Ch0".to_string(),
                    },
                    PlcRegister {
                        address: 0x0101,
                        value: 8192,
                        label: "AO_Ch1".to_string(),
                    },
                    PlcRegister {
                        address: 0x0102,
                        value: 32000,
                        label: "AO_Ch2".to_string(),
                    },
                ],
            },
        ],
        timestamp_ms: 1710500000000,
    };
    roundtrip(&snapshot, "PLC snapshot");
}

// Test 5: SCADA alarm tree with nested alarm groups
#[test]
fn test_scada_alarm_tree_roundtrip() {
    let tree = ScadaAlarmTree {
        plant_section: "Boiler House".to_string(),
        root_groups: vec![AlarmGroup {
            group_name: "Pressure".to_string(),
            conditions: vec![AlarmCondition {
                tag: "PT-101".to_string(),
                threshold: 150.0,
                comparison: "GT".to_string(),
                severity: 3,
            }],
            sub_groups: vec![
                AlarmGroup {
                    group_name: "High Pressure".to_string(),
                    conditions: vec![AlarmCondition {
                        tag: "PT-101".to_string(),
                        threshold: 200.0,
                        comparison: "GT".to_string(),
                        severity: 5,
                    }],
                    sub_groups: vec![AlarmGroup {
                        group_name: "Emergency Shutdown".to_string(),
                        conditions: vec![AlarmCondition {
                            tag: "PT-101".to_string(),
                            threshold: 250.0,
                            comparison: "GT".to_string(),
                            severity: 10,
                        }],
                        sub_groups: vec![],
                        enabled: true,
                    }],
                    enabled: true,
                },
                AlarmGroup {
                    group_name: "Low Pressure".to_string(),
                    conditions: vec![AlarmCondition {
                        tag: "PT-101".to_string(),
                        threshold: 50.0,
                        comparison: "LT".to_string(),
                        severity: 2,
                    }],
                    sub_groups: vec![],
                    enabled: true,
                },
            ],
            enabled: true,
        }],
        total_alarm_count: 4,
    };
    roundtrip(&tree, "SCADA alarm tree");
}

// Test 6: Production KPI report with OEE and reliability metrics per shift
#[test]
fn test_production_kpi_report_roundtrip() {
    let report = ProductionKpiReport {
        line_id: "LINE-A".to_string(),
        report_date: "2026-03-15".to_string(),
        shifts: vec![
            ShiftKpi {
                shift_id: "Morning".to_string(),
                oee: OeeMetrics {
                    availability: 0.95,
                    performance: 0.88,
                    quality: 0.99,
                    overall: 0.95 * 0.88 * 0.99,
                },
                reliability: ReliabilityKpis {
                    mtbf_hours: 720.0,
                    mttr_hours: 1.5,
                    failure_count: 1,
                },
                units_produced: 4500,
                scrap_count: 12,
            },
            ShiftKpi {
                shift_id: "Afternoon".to_string(),
                oee: OeeMetrics {
                    availability: 0.92,
                    performance: 0.85,
                    quality: 0.97,
                    overall: 0.92 * 0.85 * 0.97,
                },
                reliability: ReliabilityKpis {
                    mtbf_hours: 360.0,
                    mttr_hours: 2.0,
                    failure_count: 2,
                },
                units_produced: 4100,
                scrap_count: 25,
            },
        ],
    };
    roundtrip(&report, "production KPI report");
}

// Test 7: PID process control unit with multiple loops and cascade pairs
#[test]
fn test_process_control_unit_roundtrip() {
    let unit = ProcessControlUnit {
        unit_name: "Reactor-1".to_string(),
        loops: vec![
            ControlLoop {
                tag: "TIC-101".to_string(),
                setpoint: 180.0,
                process_variable: 179.4,
                output_percent: 62.3,
                tuning: PidTuning {
                    kp: 1.2,
                    ki: 0.05,
                    kd: 0.3,
                },
                in_auto: true,
            },
            ControlLoop {
                tag: "FIC-102".to_string(),
                setpoint: 500.0,
                process_variable: 498.7,
                output_percent: 55.0,
                tuning: PidTuning {
                    kp: 0.8,
                    ki: 0.1,
                    kd: 0.0,
                },
                in_auto: true,
            },
            ControlLoop {
                tag: "PIC-103".to_string(),
                setpoint: 3.5,
                process_variable: 3.48,
                output_percent: 40.0,
                tuning: PidTuning {
                    kp: 2.0,
                    ki: 0.02,
                    kd: 0.5,
                },
                in_auto: false,
            },
        ],
        cascade_pairs: vec![("TIC-101".to_string(), "FIC-102".to_string())],
    };
    roundtrip(&unit, "process control unit");
}

// Test 17: Factory floor with empty stations (boundary case)
#[test]
fn test_factory_floor_empty_stations_roundtrip() {
    let floor = FactoryFloor {
        floor_id: "F-EMPTY".to_string(),
        building: "Warehouse-C".to_string(),
        stations: vec![],
        area_sqm: 800.0,
    };
    roundtrip(&floor, "factory floor empty stations");
}

// Test 18: Workstation with no conveyors (standalone)
#[test]
fn test_workstation_no_conveyors_roundtrip() {
    let ws = Workstation {
        id: 99,
        label: "Standalone-WS".to_string(),
        operator_count: 1,
        machines: vec![make_machine(9901, "Lathe-01", true)],
        input_conveyor: None,
        output_conveyor: None,
    };
    roundtrip(&ws, "workstation without conveyors");
}

// Test 19: Multiple PLC snapshots in a vec (simulating historian batch)
#[test]
fn test_plc_snapshot_batch_roundtrip() {
    let batch: Vec<PlcSnapshot> = (0..5)
        .map(|i| PlcSnapshot {
            plc_id: format!("PLC-{:02}", i),
            firmware_version: "5.0.0".to_string(),
            scan_cycle_us: 250 + i * 50,
            modules: vec![PlcModule {
                slot: 0,
                module_type: "AI-4".to_string(),
                registers: vec![PlcRegister {
                    address: 0x0200 + i as u16,
                    value: 20000 + i * 100,
                    label: format!("AI_Ch{i}"),
                }],
            }],
            timestamp_ms: 1710500000000 + i as u64 * 1000,
        })
        .collect();
    roundtrip(&batch, "PLC snapshot batch");
}
