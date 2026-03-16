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

// ── Domain Types: Robotic Surgery & Surgical Navigation ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JointAngles {
    shoulder_yaw: f64,
    shoulder_pitch: f64,
    elbow: f64,
    wrist_roll: f64,
    wrist_pitch: f64,
    wrist_yaw: f64,
    gripper_aperture: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EndEffectorPose {
    position: Vec3,
    orientation: Quaternion,
    timestamp_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HapticFeedback {
    force_x: f32,
    force_y: f32,
    force_z: f32,
    torque_x: f32,
    torque_y: f32,
    torque_z: f32,
    vibration_amplitude: f32,
    vibration_freq_hz: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CameraParams {
    focal_length_mm: f64,
    sensor_width_px: u32,
    sensor_height_px: u32,
    fov_horizontal_deg: f32,
    fov_vertical_deg: f32,
    white_balance_k: u16,
    exposure_us: u32,
    gain_db: f32,
    is_infrared: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TissueDeformation {
    node_id: u32,
    displacement: Vec3,
    strain_xx: f64,
    strain_yy: f64,
    strain_zz: f64,
    strain_xy: f64,
    young_modulus_kpa: f64,
    poisson_ratio: f64,
    is_punctured: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SuturePoint {
    entry_point: Vec3,
    exit_point: Vec3,
    tension_newtons: f32,
    thread_diameter_mm: f32,
    stitch_index: u16,
    is_knotted: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SutureTrack {
    suture_id: u32,
    points: Vec<SuturePoint>,
    total_length_mm: f64,
    material_code: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrocarPort {
    port_id: u8,
    insertion_site: Vec3,
    angle_deg: f32,
    diameter_mm: f32,
    depth_mm: f32,
    instrument_present: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsufflationReading {
    pressure_mmhg: f32,
    flow_rate_lpm: f32,
    volume_liters: f32,
    co2_concentration_pct: f32,
    timestamp_ms: u64,
    alarm_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ElectrosurgeryMode {
    Cut {
        power_watts: f32,
    },
    Coagulate {
        power_watts: f32,
        duty_cycle_pct: u8,
    },
    Blend {
        cut_power: f32,
        coag_power: f32,
        blend_ratio: u8,
    },
    Bipolar {
        power_watts: f32,
        impedance_ohms: f32,
    },
    Standby,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ElectrosurgerySettings {
    mode: ElectrosurgeryMode,
    activation_duration_ms: u32,
    tissue_contact: bool,
    temperature_c: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NavigationCoordinate {
    patient_space: Vec3,
    image_space: Vec3,
    confidence: f32,
    landmark_id: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RegistrationTransform {
    rotation: [f64; 9],
    translation: Vec3,
    fiducial_error_mm: f64,
    num_landmarks: u16,
    is_valid: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ToolTipDynamics {
    velocity: Vec3,
    acceleration: Vec3,
    speed_mm_per_sec: f64,
    jerk_magnitude: f64,
    timestamp_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollisionVolume {
    center: Vec3,
    half_extents: Vec3,
    safety_margin_mm: f32,
    priority: u8,
    is_active: bool,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProcedurePhase {
    Preparation,
    Incision,
    Dissection { depth_mm: f32 },
    Resection { margin_mm: f32 },
    Hemostasis { blood_loss_ml: f32 },
    Anastomosis { stitch_count: u16 },
    Closure,
    Completed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BloodLossEstimate {
    cumulative_ml: f64,
    rate_ml_per_min: f32,
    sponge_count: u16,
    suction_volume_ml: f64,
    hematocrit_pct: f32,
    confidence_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PostOpScore {
    operative_time_min: u32,
    estimated_blood_loss_ml: f64,
    complication_grade: u8,
    pain_score: u8,
    mobility_score: u8,
    wound_healing_grade: u8,
    readmission_risk_pct: f32,
    notes_length: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InstrumentKinematics {
    arm_id: u8,
    joints: JointAngles,
    pose: EndEffectorPose,
    dynamics: ToolTipDynamics,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SurgicalSnapshot {
    frame_id: u64,
    phase: ProcedurePhase,
    blood_loss: BloodLossEstimate,
    insufflation: InsufflationReading,
    electrosurgery: ElectrosurgerySettings,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProcedureRecord {
    procedure_id: u64,
    patient_hash: u64,
    post_op: PostOpScore,
    trocar_ports: Vec<TrocarPort>,
    registration: RegistrationTransform,
}

// ── Strategies ──

prop_compose! {
    fn arb_vec3()(x in any::<f64>(), y in any::<f64>(), z in any::<f64>()) -> Vec3 {
        Vec3 { x, y, z }
    }
}

prop_compose! {
    fn arb_quaternion()(w in any::<f64>(), x in any::<f64>(), y in any::<f64>(), z in any::<f64>()) -> Quaternion {
        Quaternion { w, x, y, z }
    }
}

prop_compose! {
    fn arb_joint_angles()(
        shoulder_yaw in any::<f64>(),
        shoulder_pitch in any::<f64>(),
        elbow in any::<f64>(),
        wrist_roll in any::<f64>(),
        wrist_pitch in any::<f64>(),
        wrist_yaw in any::<f64>(),
        gripper_aperture in any::<f64>()
    ) -> JointAngles {
        JointAngles {
            shoulder_yaw, shoulder_pitch, elbow,
            wrist_roll, wrist_pitch, wrist_yaw, gripper_aperture,
        }
    }
}

prop_compose! {
    fn arb_end_effector_pose()(
        position in arb_vec3(),
        orientation in arb_quaternion(),
        timestamp_us in any::<u64>()
    ) -> EndEffectorPose {
        EndEffectorPose { position, orientation, timestamp_us }
    }
}

prop_compose! {
    fn arb_haptic_feedback()(
        force_x in any::<f32>(),
        force_y in any::<f32>(),
        force_z in any::<f32>(),
        torque_x in any::<f32>(),
        torque_y in any::<f32>(),
        torque_z in any::<f32>(),
        vibration_amplitude in any::<f32>(),
        vibration_freq_hz in any::<u16>()
    ) -> HapticFeedback {
        HapticFeedback {
            force_x, force_y, force_z,
            torque_x, torque_y, torque_z,
            vibration_amplitude, vibration_freq_hz,
        }
    }
}

prop_compose! {
    fn arb_camera_params()(
        focal_length_mm in any::<f64>(),
        sensor_width_px in any::<u32>(),
        sensor_height_px in any::<u32>(),
        fov_horizontal_deg in any::<f32>(),
        fov_vertical_deg in any::<f32>(),
        white_balance_k in any::<u16>(),
        exposure_us in any::<u32>(),
        gain_db in any::<f32>(),
        is_infrared in any::<bool>()
    ) -> CameraParams {
        CameraParams {
            focal_length_mm, sensor_width_px, sensor_height_px,
            fov_horizontal_deg, fov_vertical_deg,
            white_balance_k, exposure_us, gain_db, is_infrared,
        }
    }
}

prop_compose! {
    fn arb_tissue_deformation()(
        node_id in any::<u32>(),
        displacement in arb_vec3(),
        strain_xx in any::<f64>(),
        strain_yy in any::<f64>(),
        strain_zz in any::<f64>(),
        strain_xy in any::<f64>(),
        young_modulus_kpa in any::<f64>(),
        poisson_ratio in any::<f64>(),
        is_punctured in any::<bool>()
    ) -> TissueDeformation {
        TissueDeformation {
            node_id, displacement,
            strain_xx, strain_yy, strain_zz, strain_xy,
            young_modulus_kpa, poisson_ratio, is_punctured,
        }
    }
}

prop_compose! {
    fn arb_suture_point()(
        entry_point in arb_vec3(),
        exit_point in arb_vec3(),
        tension_newtons in any::<f32>(),
        thread_diameter_mm in any::<f32>(),
        stitch_index in any::<u16>(),
        is_knotted in any::<bool>()
    ) -> SuturePoint {
        SuturePoint {
            entry_point, exit_point,
            tension_newtons, thread_diameter_mm,
            stitch_index, is_knotted,
        }
    }
}

prop_compose! {
    fn arb_suture_track()(
        suture_id in any::<u32>(),
        points in prop::collection::vec(arb_suture_point(), 0..4),
        total_length_mm in any::<f64>(),
        material_code in any::<u8>()
    ) -> SutureTrack {
        SutureTrack { suture_id, points, total_length_mm, material_code }
    }
}

prop_compose! {
    fn arb_trocar_port()(
        port_id in any::<u8>(),
        insertion_site in arb_vec3(),
        angle_deg in any::<f32>(),
        diameter_mm in any::<f32>(),
        depth_mm in any::<f32>(),
        instrument_present in any::<bool>()
    ) -> TrocarPort {
        TrocarPort {
            port_id, insertion_site,
            angle_deg, diameter_mm, depth_mm, instrument_present,
        }
    }
}

prop_compose! {
    fn arb_insufflation()(
        pressure_mmhg in any::<f32>(),
        flow_rate_lpm in any::<f32>(),
        volume_liters in any::<f32>(),
        co2_concentration_pct in any::<f32>(),
        timestamp_ms in any::<u64>(),
        alarm_active in any::<bool>()
    ) -> InsufflationReading {
        InsufflationReading {
            pressure_mmhg, flow_rate_lpm, volume_liters,
            co2_concentration_pct, timestamp_ms, alarm_active,
        }
    }
}

fn arb_electrosurgery_mode() -> impl Strategy<Value = ElectrosurgeryMode> {
    prop_oneof![
        any::<f32>().prop_map(|p| ElectrosurgeryMode::Cut { power_watts: p }),
        (any::<f32>(), any::<u8>()).prop_map(|(p, d)| ElectrosurgeryMode::Coagulate {
            power_watts: p,
            duty_cycle_pct: d
        }),
        (any::<f32>(), any::<f32>(), any::<u8>()).prop_map(|(c, g, b)| ElectrosurgeryMode::Blend {
            cut_power: c,
            coag_power: g,
            blend_ratio: b
        }),
        (any::<f32>(), any::<f32>()).prop_map(|(p, i)| ElectrosurgeryMode::Bipolar {
            power_watts: p,
            impedance_ohms: i
        }),
        Just(ElectrosurgeryMode::Standby),
    ]
}

prop_compose! {
    fn arb_electrosurgery_settings()(
        mode in arb_electrosurgery_mode(),
        activation_duration_ms in any::<u32>(),
        tissue_contact in any::<bool>(),
        temperature_c in any::<f32>()
    ) -> ElectrosurgerySettings {
        ElectrosurgerySettings {
            mode, activation_duration_ms, tissue_contact, temperature_c,
        }
    }
}

prop_compose! {
    fn arb_navigation_coordinate()(
        patient_space in arb_vec3(),
        image_space in arb_vec3(),
        confidence in any::<f32>(),
        landmark_id in any::<u16>()
    ) -> NavigationCoordinate {
        NavigationCoordinate { patient_space, image_space, confidence, landmark_id }
    }
}

prop_compose! {
    fn arb_registration_transform()(
        rotation in prop::array::uniform9(any::<f64>()),
        translation in arb_vec3(),
        fiducial_error_mm in any::<f64>(),
        num_landmarks in any::<u16>(),
        is_valid in any::<bool>()
    ) -> RegistrationTransform {
        RegistrationTransform {
            rotation, translation, fiducial_error_mm, num_landmarks, is_valid,
        }
    }
}

prop_compose! {
    fn arb_tool_tip_dynamics()(
        velocity in arb_vec3(),
        acceleration in arb_vec3(),
        speed_mm_per_sec in any::<f64>(),
        jerk_magnitude in any::<f64>(),
        timestamp_us in any::<u64>()
    ) -> ToolTipDynamics {
        ToolTipDynamics {
            velocity, acceleration, speed_mm_per_sec, jerk_magnitude, timestamp_us,
        }
    }
}

prop_compose! {
    fn arb_collision_volume()(
        center in arb_vec3(),
        half_extents in arb_vec3(),
        safety_margin_mm in any::<f32>(),
        priority in any::<u8>(),
        is_active in any::<bool>(),
        label in "[a-zA-Z0-9_]{0,16}"
    ) -> CollisionVolume {
        CollisionVolume {
            center, half_extents, safety_margin_mm, priority, is_active, label,
        }
    }
}

fn arb_procedure_phase() -> impl Strategy<Value = ProcedurePhase> {
    prop_oneof![
        Just(ProcedurePhase::Preparation),
        Just(ProcedurePhase::Incision),
        any::<f32>().prop_map(|d| ProcedurePhase::Dissection { depth_mm: d }),
        any::<f32>().prop_map(|m| ProcedurePhase::Resection { margin_mm: m }),
        any::<f32>().prop_map(|b| ProcedurePhase::Hemostasis { blood_loss_ml: b }),
        any::<u16>().prop_map(|s| ProcedurePhase::Anastomosis { stitch_count: s }),
        Just(ProcedurePhase::Closure),
        Just(ProcedurePhase::Completed),
    ]
}

prop_compose! {
    fn arb_blood_loss()(
        cumulative_ml in any::<f64>(),
        rate_ml_per_min in any::<f32>(),
        sponge_count in any::<u16>(),
        suction_volume_ml in any::<f64>(),
        hematocrit_pct in any::<f32>(),
        confidence_pct in any::<f32>()
    ) -> BloodLossEstimate {
        BloodLossEstimate {
            cumulative_ml, rate_ml_per_min, sponge_count,
            suction_volume_ml, hematocrit_pct, confidence_pct,
        }
    }
}

prop_compose! {
    fn arb_post_op_score()(
        operative_time_min in any::<u32>(),
        estimated_blood_loss_ml in any::<f64>(),
        complication_grade in any::<u8>(),
        pain_score in any::<u8>(),
        mobility_score in any::<u8>(),
        wound_healing_grade in any::<u8>(),
        readmission_risk_pct in any::<f32>(),
        notes_length in any::<u32>()
    ) -> PostOpScore {
        PostOpScore {
            operative_time_min, estimated_blood_loss_ml,
            complication_grade, pain_score, mobility_score,
            wound_healing_grade, readmission_risk_pct, notes_length,
        }
    }
}

prop_compose! {
    fn arb_instrument_kinematics()(
        arm_id in any::<u8>(),
        joints in arb_joint_angles(),
        pose in arb_end_effector_pose(),
        dynamics in arb_tool_tip_dynamics()
    ) -> InstrumentKinematics {
        InstrumentKinematics { arm_id, joints, pose, dynamics }
    }
}

prop_compose! {
    fn arb_surgical_snapshot()(
        frame_id in any::<u64>(),
        phase in arb_procedure_phase(),
        blood_loss in arb_blood_loss(),
        insufflation in arb_insufflation(),
        electrosurgery in arb_electrosurgery_settings()
    ) -> SurgicalSnapshot {
        SurgicalSnapshot { frame_id, phase, blood_loss, insufflation, electrosurgery }
    }
}

prop_compose! {
    fn arb_procedure_record()(
        procedure_id in any::<u64>(),
        patient_hash in any::<u64>(),
        post_op in arb_post_op_score(),
        trocar_ports in prop::collection::vec(arb_trocar_port(), 0..5),
        registration in arb_registration_transform()
    ) -> ProcedureRecord {
        ProcedureRecord {
            procedure_id, patient_hash, post_op, trocar_ports, registration,
        }
    }
}

// ── Tests ──

#[test]
fn test_joint_angles_roundtrip() {
    proptest!(|(val in arb_joint_angles())| {
        let encoded = encode_to_vec(&val).expect("encode joint angles failed");
        let (decoded, _) = decode_from_slice::<JointAngles>(&encoded)
            .expect("decode joint angles failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_end_effector_pose_roundtrip() {
    proptest!(|(val in arb_end_effector_pose())| {
        let encoded = encode_to_vec(&val).expect("encode end effector pose failed");
        let (decoded, _) = decode_from_slice::<EndEffectorPose>(&encoded)
            .expect("decode end effector pose failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_haptic_feedback_roundtrip() {
    proptest!(|(val in arb_haptic_feedback())| {
        let encoded = encode_to_vec(&val).expect("encode haptic feedback failed");
        let (decoded, _) = decode_from_slice::<HapticFeedback>(&encoded)
            .expect("decode haptic feedback failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_camera_params_roundtrip() {
    proptest!(|(val in arb_camera_params())| {
        let encoded = encode_to_vec(&val).expect("encode camera params failed");
        let (decoded, _) = decode_from_slice::<CameraParams>(&encoded)
            .expect("decode camera params failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_tissue_deformation_roundtrip() {
    proptest!(|(val in arb_tissue_deformation())| {
        let encoded = encode_to_vec(&val).expect("encode tissue deformation failed");
        let (decoded, _) = decode_from_slice::<TissueDeformation>(&encoded)
            .expect("decode tissue deformation failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_suture_point_roundtrip() {
    proptest!(|(val in arb_suture_point())| {
        let encoded = encode_to_vec(&val).expect("encode suture point failed");
        let (decoded, _) = decode_from_slice::<SuturePoint>(&encoded)
            .expect("decode suture point failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_suture_track_roundtrip() {
    proptest!(|(val in arb_suture_track())| {
        let encoded = encode_to_vec(&val).expect("encode suture track failed");
        let (decoded, _) = decode_from_slice::<SutureTrack>(&encoded)
            .expect("decode suture track failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_trocar_port_roundtrip() {
    proptest!(|(val in arb_trocar_port())| {
        let encoded = encode_to_vec(&val).expect("encode trocar port failed");
        let (decoded, _) = decode_from_slice::<TrocarPort>(&encoded)
            .expect("decode trocar port failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_insufflation_reading_roundtrip() {
    proptest!(|(val in arb_insufflation())| {
        let encoded = encode_to_vec(&val).expect("encode insufflation reading failed");
        let (decoded, _) = decode_from_slice::<InsufflationReading>(&encoded)
            .expect("decode insufflation reading failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_electrosurgery_settings_roundtrip() {
    proptest!(|(val in arb_electrosurgery_settings())| {
        let encoded = encode_to_vec(&val).expect("encode electrosurgery settings failed");
        let (decoded, _) = decode_from_slice::<ElectrosurgerySettings>(&encoded)
            .expect("decode electrosurgery settings failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_navigation_coordinate_roundtrip() {
    proptest!(|(val in arb_navigation_coordinate())| {
        let encoded = encode_to_vec(&val).expect("encode navigation coordinate failed");
        let (decoded, _) = decode_from_slice::<NavigationCoordinate>(&encoded)
            .expect("decode navigation coordinate failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_registration_transform_roundtrip() {
    proptest!(|(val in arb_registration_transform())| {
        let encoded = encode_to_vec(&val).expect("encode registration transform failed");
        let (decoded, _) = decode_from_slice::<RegistrationTransform>(&encoded)
            .expect("decode registration transform failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_tool_tip_dynamics_roundtrip() {
    proptest!(|(val in arb_tool_tip_dynamics())| {
        let encoded = encode_to_vec(&val).expect("encode tool tip dynamics failed");
        let (decoded, _) = decode_from_slice::<ToolTipDynamics>(&encoded)
            .expect("decode tool tip dynamics failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_collision_volume_roundtrip() {
    proptest!(|(val in arb_collision_volume())| {
        let encoded = encode_to_vec(&val).expect("encode collision volume failed");
        let (decoded, _) = decode_from_slice::<CollisionVolume>(&encoded)
            .expect("decode collision volume failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_procedure_phase_roundtrip() {
    proptest!(|(val in arb_procedure_phase())| {
        let encoded = encode_to_vec(&val).expect("encode procedure phase failed");
        let (decoded, _) = decode_from_slice::<ProcedurePhase>(&encoded)
            .expect("decode procedure phase failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_blood_loss_estimate_roundtrip() {
    proptest!(|(val in arb_blood_loss())| {
        let encoded = encode_to_vec(&val).expect("encode blood loss estimate failed");
        let (decoded, _) = decode_from_slice::<BloodLossEstimate>(&encoded)
            .expect("decode blood loss estimate failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_post_op_score_roundtrip() {
    proptest!(|(val in arb_post_op_score())| {
        let encoded = encode_to_vec(&val).expect("encode post op score failed");
        let (decoded, _) = decode_from_slice::<PostOpScore>(&encoded)
            .expect("decode post op score failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_instrument_kinematics_roundtrip() {
    proptest!(|(val in arb_instrument_kinematics())| {
        let encoded = encode_to_vec(&val).expect("encode instrument kinematics failed");
        let (decoded, _) = decode_from_slice::<InstrumentKinematics>(&encoded)
            .expect("decode instrument kinematics failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_surgical_snapshot_roundtrip() {
    proptest!(|(val in arb_surgical_snapshot())| {
        let encoded = encode_to_vec(&val).expect("encode surgical snapshot failed");
        let (decoded, _) = decode_from_slice::<SurgicalSnapshot>(&encoded)
            .expect("decode surgical snapshot failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_procedure_record_roundtrip() {
    proptest!(|(val in arb_procedure_record())| {
        let encoded = encode_to_vec(&val).expect("encode procedure record failed");
        let (decoded, _) = decode_from_slice::<ProcedureRecord>(&encoded)
            .expect("decode procedure record failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_vec_collision_volumes_roundtrip() {
    proptest!(|(val in prop::collection::vec(arb_collision_volume(), 0..6))| {
        let encoded = encode_to_vec(&val).expect("encode collision volume vec failed");
        let (decoded, _) = decode_from_slice::<Vec<CollisionVolume>>(&encoded)
            .expect("decode collision volume vec failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_multi_arm_kinematics_roundtrip() {
    proptest!(|(arms in prop::collection::vec(arb_instrument_kinematics(), 1..4),
                camera in arb_camera_params(),
                nav_points in prop::collection::vec(arb_navigation_coordinate(), 0..5))| {
        let combined: (Vec<InstrumentKinematics>, CameraParams, Vec<NavigationCoordinate>) =
            (arms, camera, nav_points);
        let encoded = encode_to_vec(&combined).expect("encode multi-arm scene failed");
        let (decoded, _) = decode_from_slice::<(
            Vec<InstrumentKinematics>,
            CameraParams,
            Vec<NavigationCoordinate>,
        )>(&encoded)
            .expect("decode multi-arm scene failed");
        prop_assert_eq!(combined, decoded);
    });
}
