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

// =============================================================================
// Industrial Robotics & Factory Automation Domain Types
// =============================================================================

/// 6-DOF joint angle configuration for an industrial robot arm (radians).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JointAngles {
    j1: f64,
    j2: f64,
    j3: f64,
    j4: f64,
    j5: f64,
    j6: f64,
}

/// End effector pose in Cartesian space with orientation (Euler angles).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EndEffectorPose {
    x: f64,
    y: f64,
    z: f64,
    roll: f64,
    pitch: f64,
    yaw: f64,
}

/// PLC program state machine.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PlcState {
    Idle,
    Running { step_index: u32, cycle_count: u64 },
    Faulted { error_code: u16, message: String },
    EStop,
    Paused { resume_step: u32 },
}

/// Conveyor belt configuration and runtime data.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConveyorBelt {
    belt_id: u32,
    speed_mps: f64,
    length_m: f64,
    running: bool,
    items_on_belt: u16,
    motor_current_amps: f32,
}

/// Pick-and-place operation cycle record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PickAndPlaceCycle {
    cycle_id: u64,
    pick_position: EndEffectorPose,
    place_position: EndEffectorPose,
    cycle_time_ms: u32,
    success: bool,
    grip_force_n: f32,
}

/// Welding seam parameters for arc welding.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeldingSeam {
    seam_id: u32,
    voltage_v: f32,
    current_a: f32,
    wire_feed_mps: f32,
    travel_speed_mps: f32,
    gas_flow_lpm: f32,
    seam_length_mm: f64,
    weave_pattern: WeavePattern,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeavePattern {
    Linear,
    Zigzag {
        amplitude_mm: f32,
        frequency_hz: f32,
    },
    Circular {
        radius_mm: f32,
    },
    Figure8 {
        width_mm: f32,
        height_mm: f32,
    },
}

/// Vision system object detection result.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DetectionResult {
    object_class: String,
    confidence: f32,
    bbox_x: f32,
    bbox_y: f32,
    bbox_w: f32,
    bbox_h: f32,
    depth_mm: f64,
}

/// Torque sensor reading from a joint.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TorqueSensorReading {
    joint_index: u8,
    torque_nm: f64,
    temperature_c: f32,
    timestamp_us: u64,
    overload_flag: bool,
}

/// Safety zone configuration (axis-aligned bounding box).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SafetyZone {
    zone_id: u16,
    zone_type: SafetyZoneType,
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
    speed_limit_mps: f64,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SafetyZoneType {
    Warning,
    Reduced,
    Prohibited,
    Collaborative,
}

/// OPC-UA data point for SCADA integration.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpcUaDataPoint {
    node_id: String,
    value: OpcUaValue,
    timestamp_ns: u64,
    quality: u8,
    source_timestamp_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OpcUaValue {
    Boolean(bool),
    Int32(i32),
    Float(f32),
    Double(f64),
    StringVal(String),
    ByteArray(Vec<u8>),
}

/// PID controller tuning parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PidParameters {
    kp: f64,
    ki: f64,
    kd: f64,
    setpoint: f64,
    output_min: f64,
    output_max: f64,
    integral_windup_limit: f64,
    deadband: f64,
}

/// Motion profile for coordinated robot movement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MotionProfile {
    profile_id: u32,
    max_velocity_mps: f64,
    max_acceleration_mps2: f64,
    max_jerk_mps3: f64,
    blend_radius_mm: f64,
    waypoints: Vec<EndEffectorPose>,
}

/// IO module state for digital/analog signals.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IoModuleState {
    module_id: u16,
    digital_inputs: u32,
    digital_outputs: u32,
    analog_inputs: Vec<f32>,
    analog_outputs: Vec<f32>,
}

/// Tool center point calibration data.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TcpCalibration {
    tool_id: u8,
    offset_x: f64,
    offset_y: f64,
    offset_z: f64,
    rotation_rx: f64,
    rotation_ry: f64,
    rotation_rz: f64,
    mass_kg: f32,
    center_of_gravity: (f64, f64, f64),
}

/// Production batch record for traceability.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionBatch {
    batch_id: String,
    part_count: u32,
    pass_count: u32,
    fail_count: u32,
    start_timestamp_ms: u64,
    end_timestamp_ms: u64,
    cycle_times_ms: Vec<u32>,
}

/// Fieldbus diagnostic snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FieldbusDiagnostics {
    bus_type: FieldbusType,
    nodes_online: u16,
    nodes_total: u16,
    error_count: u64,
    last_cycle_us: u32,
    jitter_us: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FieldbusType {
    EtherCat,
    ProfiNet,
    EthernetIp,
    Modbus,
    CanOpen,
}

/// Gripper state for pneumatic/electric grippers.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GripperState {
    gripper_id: u8,
    grip_type: GripType,
    position_pct: f32,
    force_n: f32,
    object_detected: bool,
    fault: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GripType {
    Pneumatic { pressure_bar: f32 },
    Electric { current_ma: f32 },
    Vacuum { vacuum_kpa: f32 },
    Magnetic { field_strength: f32 },
}

/// Laser tracker measurement for calibration.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LaserMeasurement {
    point_id: u32,
    x_mm: f64,
    y_mm: f64,
    z_mm: f64,
    uncertainty_um: f32,
    reflector_type: u8,
    temperature_c: f32,
}

/// Multi-robot coordination message.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RobotCoordinationMsg {
    sender_id: u8,
    sequence: u64,
    msg_type: CoordinationMsgType,
    timestamp_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoordinationMsgType {
    Heartbeat,
    ZoneClaim { zone_id: u16 },
    ZoneRelease { zone_id: u16 },
    SyncRequest { barrier_id: u32 },
    SyncAck { barrier_id: u32 },
    Abort { reason: String },
}

// =============================================================================
// Strategy generators
// =============================================================================

fn arb_joint_angles() -> impl Strategy<Value = JointAngles> {
    (
        -std::f64::consts::PI..=std::f64::consts::PI,
        -std::f64::consts::PI..=std::f64::consts::PI,
        -std::f64::consts::PI..=std::f64::consts::PI,
        -std::f64::consts::PI..=std::f64::consts::PI,
        -std::f64::consts::PI..=std::f64::consts::PI,
        -std::f64::consts::PI..=std::f64::consts::PI,
    )
        .prop_map(|(j1, j2, j3, j4, j5, j6)| JointAngles {
            j1,
            j2,
            j3,
            j4,
            j5,
            j6,
        })
}

fn arb_end_effector_pose() -> impl Strategy<Value = EndEffectorPose> {
    (
        -2000.0f64..2000.0,
        -2000.0f64..2000.0,
        0.0f64..3000.0,
        -std::f64::consts::PI..=std::f64::consts::PI,
        -std::f64::consts::FRAC_PI_2..=std::f64::consts::FRAC_PI_2,
        -std::f64::consts::PI..=std::f64::consts::PI,
    )
        .prop_map(|(x, y, z, roll, pitch, yaw)| EndEffectorPose {
            x,
            y,
            z,
            roll,
            pitch,
            yaw,
        })
}

fn arb_plc_state() -> impl Strategy<Value = PlcState> {
    prop_oneof![
        Just(PlcState::Idle),
        (any::<u32>(), any::<u64>()).prop_map(|(step_index, cycle_count)| PlcState::Running {
            step_index,
            cycle_count,
        }),
        (any::<u16>(), "[a-zA-Z0-9 ]{1,50}").prop_map(|(error_code, message)| PlcState::Faulted {
            error_code,
            message,
        }),
        Just(PlcState::EStop),
        any::<u32>().prop_map(|resume_step| PlcState::Paused { resume_step }),
    ]
}

fn arb_conveyor_belt() -> impl Strategy<Value = ConveyorBelt> {
    (
        any::<u32>(),
        0.01f64..5.0,
        1.0f64..100.0,
        any::<bool>(),
        0u16..500,
        0.1f32..50.0,
    )
        .prop_map(
            |(belt_id, speed_mps, length_m, running, items_on_belt, motor_current_amps)| {
                ConveyorBelt {
                    belt_id,
                    speed_mps,
                    length_m,
                    running,
                    items_on_belt,
                    motor_current_amps,
                }
            },
        )
}

fn arb_weave_pattern() -> impl Strategy<Value = WeavePattern> {
    prop_oneof![
        Just(WeavePattern::Linear),
        (0.5f32..10.0, 0.5f32..20.0).prop_map(|(amplitude_mm, frequency_hz)| {
            WeavePattern::Zigzag {
                amplitude_mm,
                frequency_hz,
            }
        }),
        (0.5f32..10.0).prop_map(|radius_mm| WeavePattern::Circular { radius_mm }),
        (1.0f32..15.0, 1.0f32..15.0).prop_map(|(width_mm, height_mm)| WeavePattern::Figure8 {
            width_mm,
            height_mm,
        }),
    ]
}

fn arb_welding_seam() -> impl Strategy<Value = WeldingSeam> {
    (
        any::<u32>(),
        10.0f32..40.0,
        50.0f32..400.0,
        0.01f32..0.5,
        0.001f32..0.05,
        5.0f32..30.0,
        10.0f64..5000.0,
        arb_weave_pattern(),
    )
        .prop_map(
            |(
                seam_id,
                voltage_v,
                current_a,
                wire_feed_mps,
                travel_speed_mps,
                gas_flow_lpm,
                seam_length_mm,
                weave_pattern,
            )| {
                WeldingSeam {
                    seam_id,
                    voltage_v,
                    current_a,
                    wire_feed_mps,
                    travel_speed_mps,
                    gas_flow_lpm,
                    seam_length_mm,
                    weave_pattern,
                }
            },
        )
}

fn arb_detection_result() -> impl Strategy<Value = DetectionResult> {
    (
        prop_oneof![
            Just("bolt".to_string()),
            Just("nut".to_string()),
            Just("washer".to_string()),
            Just("pcb".to_string()),
            Just("connector".to_string()),
        ],
        0.0f32..1.0,
        0.0f32..1920.0,
        0.0f32..1080.0,
        10.0f32..500.0,
        10.0f32..500.0,
        100.0f64..5000.0,
    )
        .prop_map(
            |(object_class, confidence, bbox_x, bbox_y, bbox_w, bbox_h, depth_mm)| {
                DetectionResult {
                    object_class,
                    confidence,
                    bbox_x,
                    bbox_y,
                    bbox_w,
                    bbox_h,
                    depth_mm,
                }
            },
        )
}

fn arb_torque_reading() -> impl Strategy<Value = TorqueSensorReading> {
    (
        0u8..6,
        -500.0f64..500.0,
        20.0f32..80.0,
        any::<u64>(),
        any::<bool>(),
    )
        .prop_map(
            |(joint_index, torque_nm, temperature_c, timestamp_us, overload_flag)| {
                TorqueSensorReading {
                    joint_index,
                    torque_nm,
                    temperature_c,
                    timestamp_us,
                    overload_flag,
                }
            },
        )
}

fn arb_safety_zone_type() -> impl Strategy<Value = SafetyZoneType> {
    prop_oneof![
        Just(SafetyZoneType::Warning),
        Just(SafetyZoneType::Reduced),
        Just(SafetyZoneType::Prohibited),
        Just(SafetyZoneType::Collaborative),
    ]
}

fn arb_safety_zone() -> impl Strategy<Value = SafetyZone> {
    (
        any::<u16>(),
        arb_safety_zone_type(),
        (-5000.0f64..0.0, -5000.0f64..0.0, 0.0f64..100.0),
        (0.0f64..5000.0, 0.0f64..5000.0, 100.0f64..3000.0),
        0.01f64..2.0,
        any::<bool>(),
    )
        .prop_map(
            |(
                zone_id,
                zone_type,
                (min_x, min_y, min_z),
                (max_x, max_y, max_z),
                speed_limit_mps,
                active,
            )| {
                SafetyZone {
                    zone_id,
                    zone_type,
                    min_x,
                    min_y,
                    min_z,
                    max_x,
                    max_y,
                    max_z,
                    speed_limit_mps,
                    active,
                }
            },
        )
}

fn arb_opcua_value() -> impl Strategy<Value = OpcUaValue> {
    prop_oneof![
        any::<bool>().prop_map(OpcUaValue::Boolean),
        any::<i32>().prop_map(OpcUaValue::Int32),
        (0.0f32..1000.0).prop_map(OpcUaValue::Float),
        (-1e6f64..1e6).prop_map(OpcUaValue::Double),
        "[a-zA-Z0-9]{1,30}".prop_map(OpcUaValue::StringVal),
        prop::collection::vec(any::<u8>(), 0..32).prop_map(OpcUaValue::ByteArray),
    ]
}

fn arb_opcua_data_point() -> impl Strategy<Value = OpcUaDataPoint> {
    (
        "ns=[0-9];s=[a-zA-Z]{1,20}",
        arb_opcua_value(),
        any::<u64>(),
        0u8..4,
        any::<u64>(),
    )
        .prop_map(
            |(node_id, value, timestamp_ns, quality, source_timestamp_ns)| OpcUaDataPoint {
                node_id,
                value,
                timestamp_ns,
                quality,
                source_timestamp_ns,
            },
        )
}

fn arb_pid_parameters() -> impl Strategy<Value = PidParameters> {
    (
        0.0f64..100.0,
        0.0f64..50.0,
        0.0f64..20.0,
        -1000.0f64..1000.0,
        -1000.0f64..0.0,
        0.0f64..1000.0,
        0.0f64..500.0,
        0.0f64..10.0,
    )
        .prop_map(
            |(kp, ki, kd, setpoint, output_min, output_max, integral_windup_limit, deadband)| {
                PidParameters {
                    kp,
                    ki,
                    kd,
                    setpoint,
                    output_min,
                    output_max,
                    integral_windup_limit,
                    deadband,
                }
            },
        )
}

fn arb_motion_profile() -> impl Strategy<Value = MotionProfile> {
    (
        any::<u32>(),
        0.01f64..5.0,
        0.01f64..20.0,
        0.01f64..100.0,
        0.0f64..50.0,
        prop::collection::vec(arb_end_effector_pose(), 1..6),
    )
        .prop_map(
            |(
                profile_id,
                max_velocity_mps,
                max_acceleration_mps2,
                max_jerk_mps3,
                blend_radius_mm,
                waypoints,
            )| {
                MotionProfile {
                    profile_id,
                    max_velocity_mps,
                    max_acceleration_mps2,
                    max_jerk_mps3,
                    blend_radius_mm,
                    waypoints,
                }
            },
        )
}

fn arb_io_module_state() -> impl Strategy<Value = IoModuleState> {
    (
        any::<u16>(),
        any::<u32>(),
        any::<u32>(),
        prop::collection::vec(0.0f32..10.0, 0..8),
        prop::collection::vec(0.0f32..10.0, 0..4),
    )
        .prop_map(
            |(module_id, digital_inputs, digital_outputs, analog_inputs, analog_outputs)| {
                IoModuleState {
                    module_id,
                    digital_inputs,
                    digital_outputs,
                    analog_inputs,
                    analog_outputs,
                }
            },
        )
}

fn arb_tcp_calibration() -> impl Strategy<Value = TcpCalibration> {
    (
        any::<u8>(),
        (-500.0f64..500.0, -500.0f64..500.0, -500.0f64..500.0),
        (
            -std::f64::consts::PI..=std::f64::consts::PI,
            -std::f64::consts::PI..=std::f64::consts::PI,
            -std::f64::consts::PI..=std::f64::consts::PI,
        ),
        0.01f32..50.0,
        (-100.0f64..100.0, -100.0f64..100.0, -100.0f64..100.0),
    )
        .prop_map(
            |(
                tool_id,
                (offset_x, offset_y, offset_z),
                (rotation_rx, rotation_ry, rotation_rz),
                mass_kg,
                center_of_gravity,
            )| {
                TcpCalibration {
                    tool_id,
                    offset_x,
                    offset_y,
                    offset_z,
                    rotation_rx,
                    rotation_ry,
                    rotation_rz,
                    mass_kg,
                    center_of_gravity,
                }
            },
        )
}

fn arb_production_batch() -> impl Strategy<Value = ProductionBatch> {
    (
        "[A-Z]{2}[0-9]{6}",
        1u32..10000,
        any::<u64>(),
        any::<u64>(),
        prop::collection::vec(100u32..60000, 0..20),
    )
        .prop_map(
            |(batch_id, part_count, start_timestamp_ms, end_timestamp_ms, cycle_times_ms)| {
                let fail_count = part_count / 10;
                let pass_count = part_count - fail_count;
                ProductionBatch {
                    batch_id,
                    part_count,
                    pass_count,
                    fail_count,
                    start_timestamp_ms,
                    end_timestamp_ms,
                    cycle_times_ms,
                }
            },
        )
}

fn arb_fieldbus_type() -> impl Strategy<Value = FieldbusType> {
    prop_oneof![
        Just(FieldbusType::EtherCat),
        Just(FieldbusType::ProfiNet),
        Just(FieldbusType::EthernetIp),
        Just(FieldbusType::Modbus),
        Just(FieldbusType::CanOpen),
    ]
}

fn arb_fieldbus_diagnostics() -> impl Strategy<Value = FieldbusDiagnostics> {
    (
        arb_fieldbus_type(),
        1u16..128,
        any::<u64>(),
        100u32..10000,
        0u32..500,
    )
        .prop_map(
            |(bus_type, nodes_online, error_count, last_cycle_us, jitter_us)| {
                let nodes_total = nodes_online + (nodes_online / 5);
                FieldbusDiagnostics {
                    bus_type,
                    nodes_online,
                    nodes_total,
                    error_count,
                    last_cycle_us,
                    jitter_us,
                }
            },
        )
}

fn arb_grip_type() -> impl Strategy<Value = GripType> {
    prop_oneof![
        (1.0f32..8.0).prop_map(|pressure_bar| GripType::Pneumatic { pressure_bar }),
        (100.0f32..5000.0).prop_map(|current_ma| GripType::Electric { current_ma }),
        (-100.0f32..-10.0).prop_map(|vacuum_kpa| GripType::Vacuum { vacuum_kpa }),
        (0.1f32..10.0).prop_map(|field_strength| GripType::Magnetic { field_strength }),
    ]
}

fn arb_gripper_state() -> impl Strategy<Value = GripperState> {
    (
        any::<u8>(),
        arb_grip_type(),
        0.0f32..100.0,
        0.0f32..200.0,
        any::<bool>(),
        any::<bool>(),
    )
        .prop_map(
            |(gripper_id, grip_type, position_pct, force_n, object_detected, fault)| GripperState {
                gripper_id,
                grip_type,
                position_pct,
                force_n,
                object_detected,
                fault,
            },
        )
}

fn arb_laser_measurement() -> impl Strategy<Value = LaserMeasurement> {
    (
        any::<u32>(),
        -10000.0f64..10000.0,
        -10000.0f64..10000.0,
        -10000.0f64..10000.0,
        0.1f32..100.0,
        0u8..4,
        15.0f32..30.0,
    )
        .prop_map(
            |(point_id, x_mm, y_mm, z_mm, uncertainty_um, reflector_type, temperature_c)| {
                LaserMeasurement {
                    point_id,
                    x_mm,
                    y_mm,
                    z_mm,
                    uncertainty_um,
                    reflector_type,
                    temperature_c,
                }
            },
        )
}

fn arb_coordination_msg_type() -> impl Strategy<Value = CoordinationMsgType> {
    prop_oneof![
        Just(CoordinationMsgType::Heartbeat),
        any::<u16>().prop_map(|zone_id| CoordinationMsgType::ZoneClaim { zone_id }),
        any::<u16>().prop_map(|zone_id| CoordinationMsgType::ZoneRelease { zone_id }),
        any::<u32>().prop_map(|barrier_id| CoordinationMsgType::SyncRequest { barrier_id }),
        any::<u32>().prop_map(|barrier_id| CoordinationMsgType::SyncAck { barrier_id }),
        "[a-z ]{1,30}".prop_map(|reason| CoordinationMsgType::Abort { reason }),
    ]
}

fn arb_robot_coordination_msg() -> impl Strategy<Value = RobotCoordinationMsg> {
    (
        0u8..16,
        any::<u64>(),
        arb_coordination_msg_type(),
        any::<u64>(),
    )
        .prop_map(
            |(sender_id, sequence, msg_type, timestamp_us)| RobotCoordinationMsg {
                sender_id,
                sequence,
                msg_type,
                timestamp_us,
            },
        )
}

fn arb_pick_and_place_cycle() -> impl Strategy<Value = PickAndPlaceCycle> {
    (
        any::<u64>(),
        arb_end_effector_pose(),
        arb_end_effector_pose(),
        100u32..30000,
        any::<bool>(),
        0.5f32..100.0,
    )
        .prop_map(
            |(cycle_id, pick_position, place_position, cycle_time_ms, success, grip_force_n)| {
                PickAndPlaceCycle {
                    cycle_id,
                    pick_position,
                    place_position,
                    cycle_time_ms,
                    success,
                    grip_force_n,
                }
            },
        )
}

// =============================================================================
// Tests (22 total)
// =============================================================================

#[test]
fn test_joint_angles_roundtrip() {
    proptest!(|(joints in arb_joint_angles())| {
        let encoded = encode_to_vec(&joints).expect("encode JointAngles failed");
        let (decoded, _) = decode_from_slice::<JointAngles>(&encoded)
            .expect("decode JointAngles failed");
        prop_assert_eq!(joints.j1.to_bits(), decoded.j1.to_bits());
        prop_assert_eq!(joints.j2.to_bits(), decoded.j2.to_bits());
        prop_assert_eq!(joints.j3.to_bits(), decoded.j3.to_bits());
        prop_assert_eq!(joints.j4.to_bits(), decoded.j4.to_bits());
        prop_assert_eq!(joints.j5.to_bits(), decoded.j5.to_bits());
        prop_assert_eq!(joints.j6.to_bits(), decoded.j6.to_bits());
    });
}

#[test]
fn test_end_effector_pose_roundtrip() {
    proptest!(|(pose in arb_end_effector_pose())| {
        let encoded = encode_to_vec(&pose).expect("encode EndEffectorPose failed");
        let (decoded, _) = decode_from_slice::<EndEffectorPose>(&encoded)
            .expect("decode EndEffectorPose failed");
        prop_assert_eq!(pose.x.to_bits(), decoded.x.to_bits());
        prop_assert_eq!(pose.y.to_bits(), decoded.y.to_bits());
        prop_assert_eq!(pose.z.to_bits(), decoded.z.to_bits());
        prop_assert_eq!(pose.roll.to_bits(), decoded.roll.to_bits());
        prop_assert_eq!(pose.pitch.to_bits(), decoded.pitch.to_bits());
        prop_assert_eq!(pose.yaw.to_bits(), decoded.yaw.to_bits());
    });
}

#[test]
fn test_plc_state_roundtrip() {
    proptest!(|(state in arb_plc_state())| {
        let encoded = encode_to_vec(&state).expect("encode PlcState failed");
        let (decoded, _) = decode_from_slice::<PlcState>(&encoded)
            .expect("decode PlcState failed");
        prop_assert_eq!(state, decoded);
    });
}

#[test]
fn test_conveyor_belt_roundtrip() {
    proptest!(|(belt in arb_conveyor_belt())| {
        let encoded = encode_to_vec(&belt).expect("encode ConveyorBelt failed");
        let (decoded, _) = decode_from_slice::<ConveyorBelt>(&encoded)
            .expect("decode ConveyorBelt failed");
        prop_assert_eq!(belt.belt_id, decoded.belt_id);
        prop_assert_eq!(belt.speed_mps.to_bits(), decoded.speed_mps.to_bits());
        prop_assert_eq!(belt.length_m.to_bits(), decoded.length_m.to_bits());
        prop_assert_eq!(belt.running, decoded.running);
        prop_assert_eq!(belt.items_on_belt, decoded.items_on_belt);
        prop_assert_eq!(belt.motor_current_amps.to_bits(), decoded.motor_current_amps.to_bits());
    });
}

#[test]
fn test_pick_and_place_cycle_roundtrip() {
    proptest!(|(cycle in arb_pick_and_place_cycle())| {
        let encoded = encode_to_vec(&cycle).expect("encode PickAndPlaceCycle failed");
        let (decoded, _) = decode_from_slice::<PickAndPlaceCycle>(&encoded)
            .expect("decode PickAndPlaceCycle failed");
        prop_assert_eq!(cycle.cycle_id, decoded.cycle_id);
        prop_assert_eq!(cycle.cycle_time_ms, decoded.cycle_time_ms);
        prop_assert_eq!(cycle.success, decoded.success);
        prop_assert_eq!(cycle.grip_force_n.to_bits(), decoded.grip_force_n.to_bits());
        prop_assert_eq!(cycle.pick_position.x.to_bits(), decoded.pick_position.x.to_bits());
        prop_assert_eq!(cycle.place_position.x.to_bits(), decoded.place_position.x.to_bits());
    });
}

#[test]
fn test_welding_seam_roundtrip() {
    proptest!(|(seam in arb_welding_seam())| {
        let encoded = encode_to_vec(&seam).expect("encode WeldingSeam failed");
        let (decoded, _) = decode_from_slice::<WeldingSeam>(&encoded)
            .expect("decode WeldingSeam failed");
        prop_assert_eq!(seam.seam_id, decoded.seam_id);
        prop_assert_eq!(seam.voltage_v.to_bits(), decoded.voltage_v.to_bits());
        prop_assert_eq!(seam.current_a.to_bits(), decoded.current_a.to_bits());
        prop_assert_eq!(seam.weave_pattern, decoded.weave_pattern);
    });
}

#[test]
fn test_detection_result_roundtrip() {
    proptest!(|(det in arb_detection_result())| {
        let encoded = encode_to_vec(&det).expect("encode DetectionResult failed");
        let (decoded, _) = decode_from_slice::<DetectionResult>(&encoded)
            .expect("decode DetectionResult failed");
        prop_assert_eq!(&det.object_class, &decoded.object_class);
        prop_assert_eq!(det.confidence.to_bits(), decoded.confidence.to_bits());
        prop_assert_eq!(det.bbox_x.to_bits(), decoded.bbox_x.to_bits());
        prop_assert_eq!(det.depth_mm.to_bits(), decoded.depth_mm.to_bits());
    });
}

#[test]
fn test_torque_sensor_reading_roundtrip() {
    proptest!(|(reading in arb_torque_reading())| {
        let encoded = encode_to_vec(&reading).expect("encode TorqueSensorReading failed");
        let (decoded, _) = decode_from_slice::<TorqueSensorReading>(&encoded)
            .expect("decode TorqueSensorReading failed");
        prop_assert_eq!(reading.joint_index, decoded.joint_index);
        prop_assert_eq!(reading.torque_nm.to_bits(), decoded.torque_nm.to_bits());
        prop_assert_eq!(reading.timestamp_us, decoded.timestamp_us);
        prop_assert_eq!(reading.overload_flag, decoded.overload_flag);
    });
}

#[test]
fn test_safety_zone_roundtrip() {
    proptest!(|(zone in arb_safety_zone())| {
        let encoded = encode_to_vec(&zone).expect("encode SafetyZone failed");
        let (decoded, _) = decode_from_slice::<SafetyZone>(&encoded)
            .expect("decode SafetyZone failed");
        prop_assert_eq!(zone.zone_id, decoded.zone_id);
        prop_assert_eq!(zone.zone_type, decoded.zone_type);
        prop_assert_eq!(zone.min_x.to_bits(), decoded.min_x.to_bits());
        prop_assert_eq!(zone.max_z.to_bits(), decoded.max_z.to_bits());
        prop_assert_eq!(zone.active, decoded.active);
    });
}

#[test]
fn test_opcua_data_point_roundtrip() {
    proptest!(|(dp in arb_opcua_data_point())| {
        let encoded = encode_to_vec(&dp).expect("encode OpcUaDataPoint failed");
        let (decoded, _) = decode_from_slice::<OpcUaDataPoint>(&encoded)
            .expect("decode OpcUaDataPoint failed");
        prop_assert_eq!(&dp.node_id, &decoded.node_id);
        prop_assert_eq!(dp.quality, decoded.quality);
        prop_assert_eq!(dp.timestamp_ns, decoded.timestamp_ns);
    });
}

#[test]
fn test_pid_parameters_roundtrip() {
    proptest!(|(pid in arb_pid_parameters())| {
        let encoded = encode_to_vec(&pid).expect("encode PidParameters failed");
        let (decoded, _) = decode_from_slice::<PidParameters>(&encoded)
            .expect("decode PidParameters failed");
        prop_assert_eq!(pid.kp.to_bits(), decoded.kp.to_bits());
        prop_assert_eq!(pid.ki.to_bits(), decoded.ki.to_bits());
        prop_assert_eq!(pid.kd.to_bits(), decoded.kd.to_bits());
        prop_assert_eq!(pid.setpoint.to_bits(), decoded.setpoint.to_bits());
        prop_assert_eq!(pid.output_min.to_bits(), decoded.output_min.to_bits());
        prop_assert_eq!(pid.output_max.to_bits(), decoded.output_max.to_bits());
        prop_assert_eq!(pid.integral_windup_limit.to_bits(), decoded.integral_windup_limit.to_bits());
        prop_assert_eq!(pid.deadband.to_bits(), decoded.deadband.to_bits());
    });
}

#[test]
fn test_motion_profile_roundtrip() {
    proptest!(|(profile in arb_motion_profile())| {
        let encoded = encode_to_vec(&profile).expect("encode MotionProfile failed");
        let (decoded, _) = decode_from_slice::<MotionProfile>(&encoded)
            .expect("decode MotionProfile failed");
        prop_assert_eq!(profile.profile_id, decoded.profile_id);
        prop_assert_eq!(profile.waypoints.len(), decoded.waypoints.len());
        prop_assert_eq!(profile.max_velocity_mps.to_bits(), decoded.max_velocity_mps.to_bits());
        prop_assert_eq!(profile.blend_radius_mm.to_bits(), decoded.blend_radius_mm.to_bits());
        for (wp_orig, wp_dec) in profile.waypoints.iter().zip(decoded.waypoints.iter()) {
            prop_assert_eq!(wp_orig.x.to_bits(), wp_dec.x.to_bits());
            prop_assert_eq!(wp_orig.y.to_bits(), wp_dec.y.to_bits());
            prop_assert_eq!(wp_orig.z.to_bits(), wp_dec.z.to_bits());
        }
    });
}

#[test]
fn test_io_module_state_roundtrip() {
    proptest!(|(io_mod in arb_io_module_state())| {
        let encoded = encode_to_vec(&io_mod).expect("encode IoModuleState failed");
        let (decoded, _) = decode_from_slice::<IoModuleState>(&encoded)
            .expect("decode IoModuleState failed");
        prop_assert_eq!(io_mod.module_id, decoded.module_id);
        prop_assert_eq!(io_mod.digital_inputs, decoded.digital_inputs);
        prop_assert_eq!(io_mod.digital_outputs, decoded.digital_outputs);
        prop_assert_eq!(io_mod.analog_inputs.len(), decoded.analog_inputs.len());
        prop_assert_eq!(io_mod.analog_outputs.len(), decoded.analog_outputs.len());
        for (a, b) in io_mod.analog_inputs.iter().zip(decoded.analog_inputs.iter()) {
            prop_assert_eq!(a.to_bits(), b.to_bits());
        }
    });
}

#[test]
fn test_tcp_calibration_roundtrip() {
    proptest!(|(tcp in arb_tcp_calibration())| {
        let encoded = encode_to_vec(&tcp).expect("encode TcpCalibration failed");
        let (decoded, _) = decode_from_slice::<TcpCalibration>(&encoded)
            .expect("decode TcpCalibration failed");
        prop_assert_eq!(tcp.tool_id, decoded.tool_id);
        prop_assert_eq!(tcp.offset_x.to_bits(), decoded.offset_x.to_bits());
        prop_assert_eq!(tcp.offset_y.to_bits(), decoded.offset_y.to_bits());
        prop_assert_eq!(tcp.offset_z.to_bits(), decoded.offset_z.to_bits());
        prop_assert_eq!(tcp.mass_kg.to_bits(), decoded.mass_kg.to_bits());
        prop_assert_eq!(tcp.center_of_gravity.0.to_bits(), decoded.center_of_gravity.0.to_bits());
        prop_assert_eq!(tcp.center_of_gravity.1.to_bits(), decoded.center_of_gravity.1.to_bits());
        prop_assert_eq!(tcp.center_of_gravity.2.to_bits(), decoded.center_of_gravity.2.to_bits());
    });
}

#[test]
fn test_production_batch_roundtrip() {
    proptest!(|(batch in arb_production_batch())| {
        let encoded = encode_to_vec(&batch).expect("encode ProductionBatch failed");
        let (decoded, _) = decode_from_slice::<ProductionBatch>(&encoded)
            .expect("decode ProductionBatch failed");
        prop_assert_eq!(&batch.batch_id, &decoded.batch_id);
        prop_assert_eq!(batch.part_count, decoded.part_count);
        prop_assert_eq!(batch.pass_count, decoded.pass_count);
        prop_assert_eq!(batch.fail_count, decoded.fail_count);
        prop_assert_eq!(batch.cycle_times_ms, decoded.cycle_times_ms);
    });
}

#[test]
fn test_fieldbus_diagnostics_roundtrip() {
    proptest!(|(diag in arb_fieldbus_diagnostics())| {
        let encoded = encode_to_vec(&diag).expect("encode FieldbusDiagnostics failed");
        let (decoded, _) = decode_from_slice::<FieldbusDiagnostics>(&encoded)
            .expect("decode FieldbusDiagnostics failed");
        prop_assert_eq!(diag.bus_type, decoded.bus_type);
        prop_assert_eq!(diag.nodes_online, decoded.nodes_online);
        prop_assert_eq!(diag.nodes_total, decoded.nodes_total);
        prop_assert_eq!(diag.error_count, decoded.error_count);
        prop_assert_eq!(diag.last_cycle_us, decoded.last_cycle_us);
        prop_assert_eq!(diag.jitter_us, decoded.jitter_us);
    });
}

#[test]
fn test_gripper_state_roundtrip() {
    proptest!(|(grip in arb_gripper_state())| {
        let encoded = encode_to_vec(&grip).expect("encode GripperState failed");
        let (decoded, _) = decode_from_slice::<GripperState>(&encoded)
            .expect("decode GripperState failed");
        prop_assert_eq!(grip.gripper_id, decoded.gripper_id);
        prop_assert_eq!(grip.grip_type, decoded.grip_type);
        prop_assert_eq!(grip.position_pct.to_bits(), decoded.position_pct.to_bits());
        prop_assert_eq!(grip.force_n.to_bits(), decoded.force_n.to_bits());
        prop_assert_eq!(grip.object_detected, decoded.object_detected);
        prop_assert_eq!(grip.fault, decoded.fault);
    });
}

#[test]
fn test_laser_measurement_roundtrip() {
    proptest!(|(meas in arb_laser_measurement())| {
        let encoded = encode_to_vec(&meas).expect("encode LaserMeasurement failed");
        let (decoded, _) = decode_from_slice::<LaserMeasurement>(&encoded)
            .expect("decode LaserMeasurement failed");
        prop_assert_eq!(meas.point_id, decoded.point_id);
        prop_assert_eq!(meas.x_mm.to_bits(), decoded.x_mm.to_bits());
        prop_assert_eq!(meas.y_mm.to_bits(), decoded.y_mm.to_bits());
        prop_assert_eq!(meas.z_mm.to_bits(), decoded.z_mm.to_bits());
        prop_assert_eq!(meas.uncertainty_um.to_bits(), decoded.uncertainty_um.to_bits());
        prop_assert_eq!(meas.reflector_type, decoded.reflector_type);
    });
}

#[test]
fn test_robot_coordination_msg_roundtrip() {
    proptest!(|(msg in arb_robot_coordination_msg())| {
        let encoded = encode_to_vec(&msg).expect("encode RobotCoordinationMsg failed");
        let (decoded, _) = decode_from_slice::<RobotCoordinationMsg>(&encoded)
            .expect("decode RobotCoordinationMsg failed");
        prop_assert_eq!(msg.sender_id, decoded.sender_id);
        prop_assert_eq!(msg.sequence, decoded.sequence);
        prop_assert_eq!(msg.msg_type, decoded.msg_type);
        prop_assert_eq!(msg.timestamp_us, decoded.timestamp_us);
    });
}

#[test]
fn test_vec_of_torque_readings_roundtrip() {
    proptest!(|(readings in prop::collection::vec(arb_torque_reading(), 0..12))| {
        let encoded = encode_to_vec(&readings).expect("encode Vec<TorqueSensorReading> failed");
        let (decoded, _) = decode_from_slice::<Vec<TorqueSensorReading>>(&encoded)
            .expect("decode Vec<TorqueSensorReading> failed");
        prop_assert_eq!(readings.len(), decoded.len());
        for (orig, dec) in readings.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig.joint_index, dec.joint_index);
            prop_assert_eq!(orig.torque_nm.to_bits(), dec.torque_nm.to_bits());
            prop_assert_eq!(orig.timestamp_us, dec.timestamp_us);
        }
    });
}

#[test]
fn test_multiple_safety_zones_roundtrip() {
    proptest!(|(zones in prop::collection::vec(arb_safety_zone(), 1..8))| {
        let encoded = encode_to_vec(&zones).expect("encode Vec<SafetyZone> failed");
        let (decoded, _) = decode_from_slice::<Vec<SafetyZone>>(&encoded)
            .expect("decode Vec<SafetyZone> failed");
        prop_assert_eq!(zones.len(), decoded.len());
        for (orig, dec) in zones.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig.zone_id, dec.zone_id);
            prop_assert_eq!(&orig.zone_type, &dec.zone_type);
            prop_assert_eq!(orig.active, dec.active);
            prop_assert_eq!(orig.speed_limit_mps.to_bits(), dec.speed_limit_mps.to_bits());
        }
    });
}

#[test]
fn test_full_robot_cell_snapshot_roundtrip() {
    // Composite test: a complete snapshot of a robot cell state
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct RobotCellSnapshot {
        cell_id: u16,
        joints: JointAngles,
        tcp_pose: EndEffectorPose,
        plc: PlcState,
        conveyor: ConveyorBelt,
        gripper: GripperState,
        safety_zones: Vec<SafetyZone>,
        io_state: IoModuleState,
        fieldbus: FieldbusDiagnostics,
    }

    let strategy = (
        any::<u16>(),
        arb_joint_angles(),
        arb_end_effector_pose(),
        arb_plc_state(),
        arb_conveyor_belt(),
        arb_gripper_state(),
        prop::collection::vec(arb_safety_zone(), 1..4),
        arb_io_module_state(),
        arb_fieldbus_diagnostics(),
    )
        .prop_map(
            |(
                cell_id,
                joints,
                tcp_pose,
                plc,
                conveyor,
                gripper,
                safety_zones,
                io_state,
                fieldbus,
            )| {
                RobotCellSnapshot {
                    cell_id,
                    joints,
                    tcp_pose,
                    plc,
                    conveyor,
                    gripper,
                    safety_zones,
                    io_state,
                    fieldbus,
                }
            },
        );

    proptest!(|(snapshot in strategy)| {
        let encoded = encode_to_vec(&snapshot).expect("encode RobotCellSnapshot failed");
        let (decoded, consumed) = decode_from_slice::<RobotCellSnapshot>(&encoded)
            .expect("decode RobotCellSnapshot failed");
        prop_assert_eq!(consumed, encoded.len());
        prop_assert_eq!(snapshot.cell_id, decoded.cell_id);
        prop_assert_eq!(snapshot.plc, decoded.plc);
        prop_assert_eq!(snapshot.conveyor.belt_id, decoded.conveyor.belt_id);
        prop_assert_eq!(snapshot.gripper.gripper_id, decoded.gripper.gripper_id);
        prop_assert_eq!(snapshot.safety_zones.len(), decoded.safety_zones.len());
        prop_assert_eq!(snapshot.fieldbus.bus_type, decoded.fieldbus.bus_type);
    });
}
