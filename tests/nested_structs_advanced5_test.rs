//! Advanced nested struct encoding tests for OxiCode (set 5)
//! Theme: Digital twins and industrial IoT

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
// Predictive maintenance models
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VibrationReading {
    axis: String,
    rms_velocity: f64,
    peak_acceleration: f64,
    dominant_freq_hz: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemperatureReading {
    sensor_id: String,
    value_celsius: f64,
    location_on_asset: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AcousticReading {
    db_level: f64,
    frequency_spectrum: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceModel {
    asset_id: u64,
    model_version: String,
    vibration: Vec<VibrationReading>,
    temperatures: Vec<TemperatureReading>,
    acoustic: AcousticReading,
    remaining_useful_life_hours: f64,
    confidence: f64,
}

// ---------------------------------------------------------------------------
// OPC UA node hierarchy
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpcUaVariable {
    browse_name: String,
    node_id: String,
    data_type: String,
    value_as_f64: f64,
    writable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpcUaObject {
    browse_name: String,
    node_id: String,
    variables: Vec<OpcUaVariable>,
    children: Vec<OpcUaObject>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpcUaNamespace {
    uri: String,
    index: u16,
    root_objects: Vec<OpcUaObject>,
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

// ---------------------------------------------------------------------------
// Energy consumption profiles
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerMeterReading {
    meter_id: String,
    active_kw: f64,
    reactive_kvar: f64,
    power_factor: f64,
    voltage_v: f64,
    current_a: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyZone {
    zone_name: String,
    meters: Vec<PowerMeterReading>,
    sub_zones: Vec<EnergyZone>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyProfile {
    facility_id: String,
    period: String,
    zones: Vec<EnergyZone>,
    total_kwh: f64,
    peak_demand_kw: f64,
}

// ---------------------------------------------------------------------------
// Quality inspection results
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeasurementResult {
    parameter: String,
    nominal: f64,
    measured: f64,
    tolerance: f64,
    pass: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InspectionStation {
    station_name: String,
    inspector_id: String,
    measurements: Vec<MeasurementResult>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityInspection {
    batch_id: String,
    product_code: String,
    stations: Vec<InspectionStation>,
    overall_pass: bool,
    timestamp_ms: u64,
}

// ---------------------------------------------------------------------------
// Supply chain digital thread
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SupplierInfo {
    supplier_id: String,
    name: String,
    lead_time_days: u16,
    quality_rating: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaterialLot {
    lot_number: String,
    supplier: SupplierInfo,
    quantity: f64,
    unit: String,
    received_date: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DigitalThread {
    work_order: String,
    material_lots: Vec<MaterialLot>,
    production_steps: Vec<String>,
    final_inspection: Option<QualityInspection>,
}

// ---------------------------------------------------------------------------
// Asset lifecycle states
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssetEvent {
    event_type: String,
    timestamp_ms: u64,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssetLifecycle {
    asset_id: u64,
    asset_name: String,
    current_state: String,
    install_date: String,
    history: Vec<AssetEvent>,
    maintenance_model: Option<MaintenanceModel>,
}

// ---------------------------------------------------------------------------
// Simulation scenario configs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimulationParameter {
    name: String,
    value: f64,
    min_bound: f64,
    max_bound: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimulationScenario {
    scenario_id: String,
    description: String,
    parameters: Vec<SimulationParameter>,
    factory_snapshot: FactoryFloor,
    duration_seconds: u64,
}

// ---------------------------------------------------------------------------
// Sensor fusion pipeline
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorConfig {
    sensor_id: String,
    sensor_type: String,
    sample_rate_hz: u32,
    calibration_offset: f64,
    calibration_gain: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FusionStage {
    stage_name: String,
    algorithm: String,
    input_sensors: Vec<String>,
    parameters: Vec<(String, f64)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorFusionPipeline {
    pipeline_id: String,
    sensors: Vec<SensorConfig>,
    stages: Vec<FusionStage>,
    output_tag: String,
}

// ---------------------------------------------------------------------------
// Edge gateway config
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProtocolAdapter {
    protocol: String,
    port: u16,
    poll_interval_ms: u32,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DataRoute {
    source_tag: String,
    destination: String,
    transform: Option<String>,
    buffer_size: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdgeGatewayConfig {
    gateway_id: String,
    firmware: String,
    adapters: Vec<ProtocolAdapter>,
    routes: Vec<DataRoute>,
    local_storage_mb: u32,
}

// ---------------------------------------------------------------------------
// Historical trend archive
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrendSample {
    timestamp_ms: u64,
    value: f64,
    quality: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrendTag {
    tag_name: String,
    engineering_unit: String,
    samples: Vec<TrendSample>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrendArchive {
    archive_id: String,
    start_ms: u64,
    end_ms: u64,
    tags: Vec<TrendTag>,
    compression_ratio: f64,
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

// Test 3: Predictive maintenance model with vibration, temperature, and acoustic data
#[test]
fn test_predictive_maintenance_model_roundtrip() {
    let model = MaintenanceModel {
        asset_id: 42,
        model_version: "v3.1.0".to_string(),
        vibration: vec![
            VibrationReading {
                axis: "X".to_string(),
                rms_velocity: 2.4,
                peak_acceleration: 9.8,
                dominant_freq_hz: 120.0,
            },
            VibrationReading {
                axis: "Y".to_string(),
                rms_velocity: 1.8,
                peak_acceleration: 7.2,
                dominant_freq_hz: 60.0,
            },
            VibrationReading {
                axis: "Z".to_string(),
                rms_velocity: 3.1,
                peak_acceleration: 12.5,
                dominant_freq_hz: 240.0,
            },
        ],
        temperatures: vec![
            TemperatureReading {
                sensor_id: "T-001".to_string(),
                value_celsius: 72.5,
                location_on_asset: "bearing_drive_end".to_string(),
            },
            TemperatureReading {
                sensor_id: "T-002".to_string(),
                value_celsius: 68.3,
                location_on_asset: "bearing_non_drive_end".to_string(),
            },
        ],
        acoustic: AcousticReading {
            db_level: 85.2,
            frequency_spectrum: vec![0.1, 0.4, 0.9, 1.2, 0.7, 0.3],
        },
        remaining_useful_life_hours: 1200.5,
        confidence: 0.87,
    };
    roundtrip(&model, "predictive maintenance model");
}

// Test 4: OPC UA namespace with nested object hierarchy (3 levels deep)
#[test]
fn test_opcua_namespace_hierarchy_roundtrip() {
    let ns = OpcUaNamespace {
        uri: "urn:factory:opcua:plant-a".to_string(),
        index: 2,
        root_objects: vec![OpcUaObject {
            browse_name: "ProductionLine1".to_string(),
            node_id: "ns=2;s=Line1".to_string(),
            variables: vec![OpcUaVariable {
                browse_name: "LineStatus".to_string(),
                node_id: "ns=2;s=Line1.Status".to_string(),
                data_type: "Int32".to_string(),
                value_as_f64: 1.0,
                writable: false,
            }],
            children: vec![OpcUaObject {
                browse_name: "Station1".to_string(),
                node_id: "ns=2;s=Line1.St1".to_string(),
                variables: vec![OpcUaVariable {
                    browse_name: "CycleTime".to_string(),
                    node_id: "ns=2;s=Line1.St1.CT".to_string(),
                    data_type: "Double".to_string(),
                    value_as_f64: 45.3,
                    writable: false,
                }],
                children: vec![OpcUaObject {
                    browse_name: "Motor1".to_string(),
                    node_id: "ns=2;s=Line1.St1.M1".to_string(),
                    variables: vec![
                        OpcUaVariable {
                            browse_name: "Speed".to_string(),
                            node_id: "ns=2;s=Line1.St1.M1.Speed".to_string(),
                            data_type: "Double".to_string(),
                            value_as_f64: 1750.0,
                            writable: true,
                        },
                        OpcUaVariable {
                            browse_name: "Current".to_string(),
                            node_id: "ns=2;s=Line1.St1.M1.Current".to_string(),
                            data_type: "Float".to_string(),
                            value_as_f64: 14.2,
                            writable: false,
                        },
                    ],
                    children: vec![],
                }],
            }],
        }],
    };
    roundtrip(&ns, "OPC UA namespace");
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

// Test 8: Energy consumption profile with nested zones and sub-zones
#[test]
fn test_energy_profile_roundtrip() {
    let profile = EnergyProfile {
        facility_id: "FAC-001".to_string(),
        period: "2026-03".to_string(),
        zones: vec![EnergyZone {
            zone_name: "Building-A".to_string(),
            meters: vec![PowerMeterReading {
                meter_id: "EM-001".to_string(),
                active_kw: 150.0,
                reactive_kvar: 45.0,
                power_factor: 0.96,
                voltage_v: 480.0,
                current_a: 195.0,
            }],
            sub_zones: vec![
                EnergyZone {
                    zone_name: "HVAC".to_string(),
                    meters: vec![PowerMeterReading {
                        meter_id: "EM-002".to_string(),
                        active_kw: 80.0,
                        reactive_kvar: 30.0,
                        power_factor: 0.94,
                        voltage_v: 480.0,
                        current_a: 105.0,
                    }],
                    sub_zones: vec![EnergyZone {
                        zone_name: "Chiller-1".to_string(),
                        meters: vec![PowerMeterReading {
                            meter_id: "EM-003".to_string(),
                            active_kw: 55.0,
                            reactive_kvar: 20.0,
                            power_factor: 0.94,
                            voltage_v: 480.0,
                            current_a: 72.0,
                        }],
                        sub_zones: vec![],
                    }],
                },
                EnergyZone {
                    zone_name: "Lighting".to_string(),
                    meters: vec![PowerMeterReading {
                        meter_id: "EM-004".to_string(),
                        active_kw: 12.0,
                        reactive_kvar: 1.0,
                        power_factor: 0.99,
                        voltage_v: 277.0,
                        current_a: 43.5,
                    }],
                    sub_zones: vec![],
                },
            ],
        }],
        total_kwh: 108000.0,
        peak_demand_kw: 220.0,
    };
    roundtrip(&profile, "energy profile");
}

// Test 9: Quality inspection with multiple stations and measurements
#[test]
fn test_quality_inspection_roundtrip() {
    let inspection = QualityInspection {
        batch_id: "B-20260315-001".to_string(),
        product_code: "WIDGET-A".to_string(),
        stations: vec![
            InspectionStation {
                station_name: "Dimensional".to_string(),
                inspector_id: "INS-042".to_string(),
                measurements: vec![
                    MeasurementResult {
                        parameter: "length_mm".to_string(),
                        nominal: 100.0,
                        measured: 100.02,
                        tolerance: 0.05,
                        pass: true,
                    },
                    MeasurementResult {
                        parameter: "width_mm".to_string(),
                        nominal: 50.0,
                        measured: 49.98,
                        tolerance: 0.05,
                        pass: true,
                    },
                ],
            },
            InspectionStation {
                station_name: "Surface".to_string(),
                inspector_id: "INS-043".to_string(),
                measurements: vec![MeasurementResult {
                    parameter: "roughness_ra".to_string(),
                    nominal: 0.8,
                    measured: 0.75,
                    tolerance: 0.2,
                    pass: true,
                }],
            },
        ],
        overall_pass: true,
        timestamp_ms: 1710500000000,
    };
    roundtrip(&inspection, "quality inspection");
}

// Test 10: Supply chain digital thread linking materials to production
#[test]
fn test_digital_thread_roundtrip() {
    let thread = DigitalThread {
        work_order: "WO-2026-0315".to_string(),
        material_lots: vec![
            MaterialLot {
                lot_number: "LOT-A-001".to_string(),
                supplier: SupplierInfo {
                    supplier_id: "SUP-10".to_string(),
                    name: "SteelCo".to_string(),
                    lead_time_days: 14,
                    quality_rating: 4.5,
                },
                quantity: 500.0,
                unit: "kg".to_string(),
                received_date: "2026-03-01".to_string(),
            },
            MaterialLot {
                lot_number: "LOT-B-002".to_string(),
                supplier: SupplierInfo {
                    supplier_id: "SUP-20".to_string(),
                    name: "PolymerInc".to_string(),
                    lead_time_days: 7,
                    quality_rating: 4.8,
                },
                quantity: 200.0,
                unit: "liters".to_string(),
                received_date: "2026-03-05".to_string(),
            },
        ],
        production_steps: vec![
            "Casting".to_string(),
            "Machining".to_string(),
            "Coating".to_string(),
            "Assembly".to_string(),
        ],
        final_inspection: None,
    };
    roundtrip(&thread, "digital thread");
}

// Test 11: Asset lifecycle with embedded maintenance model
#[test]
fn test_asset_lifecycle_roundtrip() {
    let lifecycle = AssetLifecycle {
        asset_id: 1001,
        asset_name: "CNC-Mill-07".to_string(),
        current_state: "operational".to_string(),
        install_date: "2022-06-15".to_string(),
        history: vec![
            AssetEvent {
                event_type: "installed".to_string(),
                timestamp_ms: 1655308800000,
                description: "Initial installation and commissioning".to_string(),
            },
            AssetEvent {
                event_type: "maintenance".to_string(),
                timestamp_ms: 1670000000000,
                description: "Spindle bearing replacement".to_string(),
            },
            AssetEvent {
                event_type: "upgrade".to_string(),
                timestamp_ms: 1700000000000,
                description: "Controller firmware update to v5.2".to_string(),
            },
        ],
        maintenance_model: Some(MaintenanceModel {
            asset_id: 1001,
            model_version: "v2.0".to_string(),
            vibration: vec![VibrationReading {
                axis: "Z".to_string(),
                rms_velocity: 1.9,
                peak_acceleration: 6.5,
                dominant_freq_hz: 180.0,
            }],
            temperatures: vec![TemperatureReading {
                sensor_id: "T-SPINDLE".to_string(),
                value_celsius: 55.0,
                location_on_asset: "spindle_housing".to_string(),
            }],
            acoustic: AcousticReading {
                db_level: 78.0,
                frequency_spectrum: vec![0.2, 0.5, 0.8, 0.6, 0.3],
            },
            remaining_useful_life_hours: 3500.0,
            confidence: 0.92,
        }),
    };
    roundtrip(&lifecycle, "asset lifecycle");
}

// Test 12: Simulation scenario with embedded factory snapshot
#[test]
fn test_simulation_scenario_roundtrip() {
    let scenario = SimulationScenario {
        scenario_id: "SIM-001".to_string(),
        description: "Throughput optimization under peak demand".to_string(),
        parameters: vec![
            SimulationParameter {
                name: "conveyor_speed".to_string(),
                value: 0.75,
                min_bound: 0.1,
                max_bound: 2.0,
            },
            SimulationParameter {
                name: "buffer_capacity".to_string(),
                value: 50.0,
                min_bound: 10.0,
                max_bound: 200.0,
            },
        ],
        factory_snapshot: make_factory_floor(),
        duration_seconds: 86400,
    };
    roundtrip(&scenario, "simulation scenario");
}

// Test 13: Sensor fusion pipeline configuration
#[test]
fn test_sensor_fusion_pipeline_roundtrip() {
    let pipeline = SensorFusionPipeline {
        pipeline_id: "FUSE-VIB-01".to_string(),
        sensors: vec![
            SensorConfig {
                sensor_id: "ACC-X".to_string(),
                sensor_type: "accelerometer".to_string(),
                sample_rate_hz: 10000,
                calibration_offset: -0.02,
                calibration_gain: 1.003,
            },
            SensorConfig {
                sensor_id: "ACC-Y".to_string(),
                sensor_type: "accelerometer".to_string(),
                sample_rate_hz: 10000,
                calibration_offset: 0.01,
                calibration_gain: 0.998,
            },
            SensorConfig {
                sensor_id: "PROX-1".to_string(),
                sensor_type: "proximity".to_string(),
                sample_rate_hz: 1000,
                calibration_offset: 0.0,
                calibration_gain: 1.0,
            },
        ],
        stages: vec![
            FusionStage {
                stage_name: "Resample".to_string(),
                algorithm: "linear_interpolation".to_string(),
                input_sensors: vec![
                    "ACC-X".to_string(),
                    "ACC-Y".to_string(),
                    "PROX-1".to_string(),
                ],
                parameters: vec![("target_rate".to_string(), 10000.0)],
            },
            FusionStage {
                stage_name: "Kalman".to_string(),
                algorithm: "extended_kalman_filter".to_string(),
                input_sensors: vec!["ACC-X".to_string(), "ACC-Y".to_string()],
                parameters: vec![
                    ("process_noise".to_string(), 0.01),
                    ("measurement_noise".to_string(), 0.05),
                ],
            },
        ],
        output_tag: "FUSED_POSITION".to_string(),
    };
    roundtrip(&pipeline, "sensor fusion pipeline");
}

// Test 14: Edge gateway configuration with protocol adapters and data routes
#[test]
fn test_edge_gateway_config_roundtrip() {
    let gw = EdgeGatewayConfig {
        gateway_id: "GW-PLANT-A-01".to_string(),
        firmware: "EdgeOS 3.4.1".to_string(),
        adapters: vec![
            ProtocolAdapter {
                protocol: "Modbus-TCP".to_string(),
                port: 502,
                poll_interval_ms: 100,
                tags: vec!["PLC1_REG0".to_string(), "PLC1_REG1".to_string()],
            },
            ProtocolAdapter {
                protocol: "OPC-UA".to_string(),
                port: 4840,
                poll_interval_ms: 500,
                tags: vec!["Line1.Speed".to_string(), "Line1.Temp".to_string()],
            },
            ProtocolAdapter {
                protocol: "MQTT".to_string(),
                port: 1883,
                poll_interval_ms: 1000,
                tags: vec!["env/temperature".to_string(), "env/humidity".to_string()],
            },
        ],
        routes: vec![
            DataRoute {
                source_tag: "PLC1_REG0".to_string(),
                destination: "cloud/timeseries".to_string(),
                transform: Some("scale(0.01)".to_string()),
                buffer_size: 1024,
            },
            DataRoute {
                source_tag: "Line1.Speed".to_string(),
                destination: "local/historian".to_string(),
                transform: None,
                buffer_size: 512,
            },
        ],
        local_storage_mb: 4096,
    };
    roundtrip(&gw, "edge gateway config");
}

// Test 15: Historical trend archive with multiple tags and samples
#[test]
fn test_trend_archive_roundtrip() {
    let archive = TrendArchive {
        archive_id: "ARCH-2026-03".to_string(),
        start_ms: 1709251200000,
        end_ms: 1711929600000,
        tags: vec![
            TrendTag {
                tag_name: "TT-101".to_string(),
                engineering_unit: "degC".to_string(),
                samples: vec![
                    TrendSample {
                        timestamp_ms: 1709251200000,
                        value: 72.1,
                        quality: 192,
                    },
                    TrendSample {
                        timestamp_ms: 1709251260000,
                        value: 72.3,
                        quality: 192,
                    },
                    TrendSample {
                        timestamp_ms: 1709251320000,
                        value: 71.9,
                        quality: 192,
                    },
                ],
            },
            TrendTag {
                tag_name: "PT-201".to_string(),
                engineering_unit: "bar".to_string(),
                samples: vec![
                    TrendSample {
                        timestamp_ms: 1709251200000,
                        value: 3.45,
                        quality: 192,
                    },
                    TrendSample {
                        timestamp_ms: 1709251260000,
                        value: 3.47,
                        quality: 192,
                    },
                ],
            },
        ],
        compression_ratio: 4.2,
    };
    roundtrip(&archive, "trend archive");
}

// Test 16: Digital thread with final inspection present
#[test]
fn test_digital_thread_with_inspection_roundtrip() {
    let thread = DigitalThread {
        work_order: "WO-2026-0400".to_string(),
        material_lots: vec![MaterialLot {
            lot_number: "LOT-X-100".to_string(),
            supplier: SupplierInfo {
                supplier_id: "SUP-99".to_string(),
                name: "AluminaCorp".to_string(),
                lead_time_days: 21,
                quality_rating: 4.2,
            },
            quantity: 1000.0,
            unit: "kg".to_string(),
            received_date: "2026-02-20".to_string(),
        }],
        production_steps: vec!["Extrusion".to_string(), "Anodizing".to_string()],
        final_inspection: Some(QualityInspection {
            batch_id: "B-FINAL-001".to_string(),
            product_code: "EXTRUSION-PRO".to_string(),
            stations: vec![InspectionStation {
                station_name: "FinalCheck".to_string(),
                inspector_id: "INS-100".to_string(),
                measurements: vec![MeasurementResult {
                    parameter: "hardness_hrc".to_string(),
                    nominal: 60.0,
                    measured: 59.8,
                    tolerance: 2.0,
                    pass: true,
                }],
            }],
            overall_pass: true,
            timestamp_ms: 1710600000000,
        }),
    };
    roundtrip(&thread, "digital thread with inspection");
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

// Test 20: Asset lifecycle with no maintenance model
#[test]
fn test_asset_lifecycle_no_model_roundtrip() {
    let lifecycle = AssetLifecycle {
        asset_id: 2002,
        asset_name: "Conveyor-Belt-12".to_string(),
        current_state: "decommissioned".to_string(),
        install_date: "2018-01-10".to_string(),
        history: vec![
            AssetEvent {
                event_type: "installed".to_string(),
                timestamp_ms: 1515542400000,
                description: "Installed on Line-3".to_string(),
            },
            AssetEvent {
                event_type: "decommissioned".to_string(),
                timestamp_ms: 1700000000000,
                description: "Replaced by higher-capacity belt".to_string(),
            },
        ],
        maintenance_model: None,
    };
    roundtrip(&lifecycle, "asset lifecycle no model");
}

// Test 21: Combined edge gateway + sensor fusion (cross-domain nesting)
#[test]
fn test_cross_domain_edge_fusion_roundtrip() {
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct EdgeFusionDeployment {
        gateway: EdgeGatewayConfig,
        pipelines: Vec<SensorFusionPipeline>,
        deployment_name: String,
    }

    let deployment = EdgeFusionDeployment {
        gateway: EdgeGatewayConfig {
            gateway_id: "GW-EDGE-FUSION".to_string(),
            firmware: "EdgeOS 4.0.0".to_string(),
            adapters: vec![ProtocolAdapter {
                protocol: "EtherNet/IP".to_string(),
                port: 44818,
                poll_interval_ms: 50,
                tags: vec!["DriveSpeed".to_string(), "DriveTorque".to_string()],
            }],
            routes: vec![DataRoute {
                source_tag: "DriveSpeed".to_string(),
                destination: "fusion/input".to_string(),
                transform: None,
                buffer_size: 2048,
            }],
            local_storage_mb: 8192,
        },
        pipelines: vec![SensorFusionPipeline {
            pipeline_id: "FUSE-DRIVE-01".to_string(),
            sensors: vec![SensorConfig {
                sensor_id: "DRIVE-SPD".to_string(),
                sensor_type: "encoder".to_string(),
                sample_rate_hz: 5000,
                calibration_offset: 0.0,
                calibration_gain: 1.0,
            }],
            stages: vec![FusionStage {
                stage_name: "LowPass".to_string(),
                algorithm: "butterworth_2nd".to_string(),
                input_sensors: vec!["DRIVE-SPD".to_string()],
                parameters: vec![("cutoff_hz".to_string(), 500.0)],
            }],
            output_tag: "DRIVE_FILTERED".to_string(),
        }],
        deployment_name: "Drive Monitor v1".to_string(),
    };
    roundtrip(&deployment, "edge fusion deployment");
}

// Test 22: Full plant digital twin combining multiple subsystems
#[test]
fn test_full_plant_digital_twin_roundtrip() {
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct PlantDigitalTwin {
        plant_name: String,
        floor: FactoryFloor,
        kpi_report: ProductionKpiReport,
        alarm_tree: ScadaAlarmTree,
        control: ProcessControlUnit,
        energy: EnergyProfile,
        trend: TrendArchive,
    }

    let twin = PlantDigitalTwin {
        plant_name: "SmartFactory-Alpha".to_string(),
        floor: FactoryFloor {
            floor_id: "F-TWIN".to_string(),
            building: "Main".to_string(),
            stations: vec![make_workstation(1, 1)],
            area_sqm: 3000.0,
        },
        kpi_report: ProductionKpiReport {
            line_id: "TWIN-LINE".to_string(),
            report_date: "2026-03-15".to_string(),
            shifts: vec![ShiftKpi {
                shift_id: "Day".to_string(),
                oee: OeeMetrics {
                    availability: 0.96,
                    performance: 0.91,
                    quality: 0.995,
                    overall: 0.96 * 0.91 * 0.995,
                },
                reliability: ReliabilityKpis {
                    mtbf_hours: 1440.0,
                    mttr_hours: 0.75,
                    failure_count: 0,
                },
                units_produced: 8200,
                scrap_count: 5,
            }],
        },
        alarm_tree: ScadaAlarmTree {
            plant_section: "Twin-Section".to_string(),
            root_groups: vec![AlarmGroup {
                group_name: "Temperature".to_string(),
                conditions: vec![AlarmCondition {
                    tag: "TT-TWIN".to_string(),
                    threshold: 90.0,
                    comparison: "GT".to_string(),
                    severity: 4,
                }],
                sub_groups: vec![],
                enabled: true,
            }],
            total_alarm_count: 1,
        },
        control: ProcessControlUnit {
            unit_name: "Twin-Reactor".to_string(),
            loops: vec![ControlLoop {
                tag: "TIC-TWIN".to_string(),
                setpoint: 200.0,
                process_variable: 199.5,
                output_percent: 58.0,
                tuning: PidTuning {
                    kp: 1.5,
                    ki: 0.08,
                    kd: 0.2,
                },
                in_auto: true,
            }],
            cascade_pairs: vec![],
        },
        energy: EnergyProfile {
            facility_id: "FAC-TWIN".to_string(),
            period: "2026-03".to_string(),
            zones: vec![EnergyZone {
                zone_name: "Main-Hall".to_string(),
                meters: vec![PowerMeterReading {
                    meter_id: "EM-TWIN".to_string(),
                    active_kw: 300.0,
                    reactive_kvar: 80.0,
                    power_factor: 0.97,
                    voltage_v: 480.0,
                    current_a: 390.0,
                }],
                sub_zones: vec![],
            }],
            total_kwh: 216000.0,
            peak_demand_kw: 420.0,
        },
        trend: TrendArchive {
            archive_id: "ARCH-TWIN".to_string(),
            start_ms: 1710500000000,
            end_ms: 1710586400000,
            tags: vec![TrendTag {
                tag_name: "TT-TWIN".to_string(),
                engineering_unit: "degC".to_string(),
                samples: vec![
                    TrendSample {
                        timestamp_ms: 1710500000000,
                        value: 199.5,
                        quality: 192,
                    },
                    TrendSample {
                        timestamp_ms: 1710500060000,
                        value: 200.1,
                        quality: 192,
                    },
                ],
            }],
            compression_ratio: 3.8,
        },
    };
    roundtrip(&twin, "full plant digital twin");
}
