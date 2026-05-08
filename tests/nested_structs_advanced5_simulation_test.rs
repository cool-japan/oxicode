//! Simulation/sensor-fusion/edge-gateway/trend-focused tests for nested_structs_advanced5 (split from nested_structs_advanced5_test.rs).

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
// Domain types: Factory floor layout (needed by SimulationScenario and PlantDigitalTwin)
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
// SCADA alarm tree (needed by PlantDigitalTwin)
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
// Production KPIs (needed by PlantDigitalTwin)
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
// PID process control loops (needed by PlantDigitalTwin)
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
// Energy consumption profiles (needed by PlantDigitalTwin)
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
