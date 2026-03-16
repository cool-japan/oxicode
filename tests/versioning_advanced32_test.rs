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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TwinState {
    Synchronized,
    Diverged,
    Offline,
    Calibrating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SimulationStatus {
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorFusionMethod {
    Kalman,
    ParticleFilter,
    Complementary,
    Bayesian,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PhysicsModel {
    Rigid,
    Flexible,
    FluidDynamic,
    Thermal,
    Electromagnetic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssetDigitalTwin {
    twin_id: u64,
    asset_id: u64,
    name: String,
    state: TwinState,
    last_sync: u64,
    accuracy_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimulationRun {
    run_id: u64,
    twin_id: u64,
    status: SimulationStatus,
    model: PhysicsModel,
    start_time: u64,
    elapsed_ms: u64,
    step_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorFusion {
    fusion_id: u64,
    twin_id: u64,
    method: SensorFusionMethod,
    sensor_count: u8,
    confidence_x1000: u32,
    last_updated: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnomalyDetection {
    anomaly_id: u64,
    twin_id: u64,
    timestamp: u64,
    severity: u8,
    description: String,
    auto_corrected: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TwinParameter {
    twin_id: u64,
    param_name: String,
    value_x1e6: i64,
    uncertainty_x1e6: u64,
    updated_at: u64,
}

// --- Test 1: AssetDigitalTwin roundtrip basic encode/decode ---
#[test]
fn test_asset_digital_twin_basic_roundtrip() {
    let twin = AssetDigitalTwin {
        twin_id: 1001,
        asset_id: 5001,
        name: "PressureVessel-A1".to_string(),
        state: TwinState::Synchronized,
        last_sync: 1700000000,
        accuracy_pct: 97,
    };
    let bytes = encode_to_vec(&twin).expect("encode AssetDigitalTwin");
    let (decoded, _size): (AssetDigitalTwin, usize) =
        decode_from_slice(&bytes).expect("decode AssetDigitalTwin");
    assert_eq!(twin, decoded);
}

// --- Test 2: AssetDigitalTwin versioned encoding v1.0.0 ---
#[test]
fn test_asset_digital_twin_versioned_v1() {
    let twin = AssetDigitalTwin {
        twin_id: 1002,
        asset_id: 5002,
        name: "TurbineRotor-B3".to_string(),
        state: TwinState::Calibrating,
        last_sync: 1700001000,
        accuracy_pct: 88,
    };
    let ver = Version::new(1, 0, 0);
    let bytes =
        encode_versioned_value(&twin, ver).expect("encode versioned AssetDigitalTwin v1.0.0");
    let (decoded, version, _size): (AssetDigitalTwin, Version, usize) =
        decode_versioned_value::<AssetDigitalTwin>(&bytes)
            .expect("decode versioned AssetDigitalTwin v1.0.0");
    assert_eq!(twin, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
}

// --- Test 3: AssetDigitalTwin versioned encoding v2.0.0 ---
#[test]
fn test_asset_digital_twin_versioned_v2() {
    let twin = AssetDigitalTwin {
        twin_id: 1003,
        asset_id: 5003,
        name: "HeatExchanger-C7".to_string(),
        state: TwinState::Diverged,
        last_sync: 1700002000,
        accuracy_pct: 72,
    };
    let ver = Version::new(2, 0, 0);
    let bytes =
        encode_versioned_value(&twin, ver).expect("encode versioned AssetDigitalTwin v2.0.0");
    let (decoded, version, _size): (AssetDigitalTwin, Version, usize) =
        decode_versioned_value::<AssetDigitalTwin>(&bytes)
            .expect("decode versioned AssetDigitalTwin v2.0.0");
    assert_eq!(twin, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
}

// --- Test 4: TwinState::Offline variant ---
#[test]
fn test_twin_state_offline_roundtrip() {
    let twin = AssetDigitalTwin {
        twin_id: 1004,
        asset_id: 5004,
        name: "CoolingTower-D2".to_string(),
        state: TwinState::Offline,
        last_sync: 1700003000,
        accuracy_pct: 0,
    };
    let bytes = encode_to_vec(&twin).expect("encode TwinState::Offline");
    let (decoded, _): (AssetDigitalTwin, usize) =
        decode_from_slice(&bytes).expect("decode TwinState::Offline");
    assert_eq!(decoded.state, TwinState::Offline);
    assert_eq!(decoded.accuracy_pct, 0);
}

// --- Test 5: SimulationRun with Kalman physics model and Running status ---
#[test]
fn test_simulation_run_running_rigid_roundtrip() {
    let run = SimulationRun {
        run_id: 2001,
        twin_id: 1001,
        status: SimulationStatus::Running,
        model: PhysicsModel::Rigid,
        start_time: 1700010000,
        elapsed_ms: 5000,
        step_count: 500,
    };
    let bytes = encode_to_vec(&run).expect("encode SimulationRun running/rigid");
    let (decoded, _): (SimulationRun, usize) =
        decode_from_slice(&bytes).expect("decode SimulationRun running/rigid");
    assert_eq!(run, decoded);
    assert_eq!(decoded.status, SimulationStatus::Running);
    assert_eq!(decoded.model, PhysicsModel::Rigid);
}

// --- Test 6: SimulationRun versioned v2.1.0 with FluidDynamic model ---
#[test]
fn test_simulation_run_versioned_v2_1_fluid_dynamic() {
    let run = SimulationRun {
        run_id: 2002,
        twin_id: 1002,
        status: SimulationStatus::Completed,
        model: PhysicsModel::FluidDynamic,
        start_time: 1700020000,
        elapsed_ms: 120000,
        step_count: 12000,
    };
    let ver = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&run, ver).expect("encode versioned SimulationRun v2.1.0");
    let (decoded, version, _size): (SimulationRun, Version, usize) =
        decode_versioned_value::<SimulationRun>(&bytes)
            .expect("decode versioned SimulationRun v2.1.0");
    assert_eq!(run, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 0);
    assert_eq!(decoded.model, PhysicsModel::FluidDynamic);
}

// --- Test 7: SimulationRun Failed status with Electromagnetic physics ---
#[test]
fn test_simulation_run_failed_electromagnetic() {
    let run = SimulationRun {
        run_id: 2003,
        twin_id: 1003,
        status: SimulationStatus::Failed,
        model: PhysicsModel::Electromagnetic,
        start_time: 1700030000,
        elapsed_ms: 800,
        step_count: 80,
    };
    let bytes = encode_to_vec(&run).expect("encode SimulationRun failed/electromagnetic");
    let (decoded, _): (SimulationRun, usize) =
        decode_from_slice(&bytes).expect("decode SimulationRun failed/electromagnetic");
    assert_eq!(decoded.status, SimulationStatus::Failed);
    assert_eq!(decoded.model, PhysicsModel::Electromagnetic);
    assert_eq!(decoded.step_count, 80);
}

// --- Test 8: SensorFusion Kalman method roundtrip ---
#[test]
fn test_sensor_fusion_kalman_roundtrip() {
    let fusion = SensorFusion {
        fusion_id: 3001,
        twin_id: 1001,
        method: SensorFusionMethod::Kalman,
        sensor_count: 12,
        confidence_x1000: 987,
        last_updated: 1700040000,
    };
    let bytes = encode_to_vec(&fusion).expect("encode SensorFusion Kalman");
    let (decoded, _): (SensorFusion, usize) =
        decode_from_slice(&bytes).expect("decode SensorFusion Kalman");
    assert_eq!(fusion, decoded);
    assert_eq!(decoded.method, SensorFusionMethod::Kalman);
    assert_eq!(decoded.confidence_x1000, 987);
}

// --- Test 9: SensorFusion Bayesian versioned v1.0.0 ---
#[test]
fn test_sensor_fusion_bayesian_versioned_v1() {
    let fusion = SensorFusion {
        fusion_id: 3002,
        twin_id: 1002,
        method: SensorFusionMethod::Bayesian,
        sensor_count: 8,
        confidence_x1000: 850,
        last_updated: 1700041000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&fusion, ver)
        .expect("encode versioned SensorFusion Bayesian v1.0.0");
    let (decoded, version, _): (SensorFusion, Version, usize) =
        decode_versioned_value::<SensorFusion>(&bytes)
            .expect("decode versioned SensorFusion Bayesian v1.0.0");
    assert_eq!(fusion, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(decoded.method, SensorFusionMethod::Bayesian);
}

// --- Test 10: SensorFusion ParticleFilter and Complementary roundtrips ---
#[test]
fn test_sensor_fusion_particle_filter_and_complementary() {
    let particle = SensorFusion {
        fusion_id: 3003,
        twin_id: 1003,
        method: SensorFusionMethod::ParticleFilter,
        sensor_count: 32,
        confidence_x1000: 920,
        last_updated: 1700042000,
    };
    let complementary = SensorFusion {
        fusion_id: 3004,
        twin_id: 1004,
        method: SensorFusionMethod::Complementary,
        sensor_count: 4,
        confidence_x1000: 760,
        last_updated: 1700043000,
    };
    let bytes_p = encode_to_vec(&particle).expect("encode ParticleFilter fusion");
    let bytes_c = encode_to_vec(&complementary).expect("encode Complementary fusion");
    let (dec_p, _): (SensorFusion, usize) =
        decode_from_slice(&bytes_p).expect("decode ParticleFilter fusion");
    let (dec_c, _): (SensorFusion, usize) =
        decode_from_slice(&bytes_c).expect("decode Complementary fusion");
    assert_eq!(dec_p.method, SensorFusionMethod::ParticleFilter);
    assert_eq!(dec_c.method, SensorFusionMethod::Complementary);
}

// --- Test 11: AnomalyDetection auto-corrected roundtrip ---
#[test]
fn test_anomaly_detection_auto_corrected_roundtrip() {
    let anomaly = AnomalyDetection {
        anomaly_id: 4001,
        twin_id: 1001,
        timestamp: 1700050000,
        severity: 7,
        description: "Pressure spike detected on inlet valve".to_string(),
        auto_corrected: true,
    };
    let bytes = encode_to_vec(&anomaly).expect("encode AnomalyDetection auto-corrected");
    let (decoded, _): (AnomalyDetection, usize) =
        decode_from_slice(&bytes).expect("decode AnomalyDetection auto-corrected");
    assert_eq!(anomaly, decoded);
    assert!(decoded.auto_corrected);
    assert_eq!(decoded.severity, 7);
}

// --- Test 12: AnomalyDetection versioned v2.0.0 critical severity ---
#[test]
fn test_anomaly_detection_versioned_v2_critical() {
    let anomaly = AnomalyDetection {
        anomaly_id: 4002,
        twin_id: 1002,
        timestamp: 1700051000,
        severity: 10,
        description: "Overheat in thermal zone 3, emergency shutdown initiated".to_string(),
        auto_corrected: false,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&anomaly, ver)
        .expect("encode versioned AnomalyDetection v2.0.0 critical");
    let (decoded, version, _): (AnomalyDetection, Version, usize) =
        decode_versioned_value::<AnomalyDetection>(&bytes)
            .expect("decode versioned AnomalyDetection v2.0.0 critical");
    assert_eq!(anomaly, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.patch, 0);
    assert_eq!(decoded.severity, 10);
    assert!(!decoded.auto_corrected);
}

// --- Test 13: TwinParameter positive value versioned v2.1.0 ---
#[test]
fn test_twin_parameter_positive_versioned_v2_1() {
    let param = TwinParameter {
        twin_id: 1001,
        param_name: "thermal_conductivity".to_string(),
        value_x1e6: 205_000_000,
        uncertainty_x1e6: 1_000_000,
        updated_at: 1700060000,
    };
    let ver = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&param, ver).expect("encode versioned TwinParameter v2.1.0");
    let (decoded, version, _): (TwinParameter, Version, usize) =
        decode_versioned_value::<TwinParameter>(&bytes)
            .expect("decode versioned TwinParameter v2.1.0");
    assert_eq!(param, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 0);
    assert_eq!(decoded.value_x1e6, 205_000_000);
}

// --- Test 14: TwinParameter negative value (damping coefficient) ---
#[test]
fn test_twin_parameter_negative_value_roundtrip() {
    let param = TwinParameter {
        twin_id: 1003,
        param_name: "damping_ratio".to_string(),
        value_x1e6: -50_000,
        uncertainty_x1e6: 500,
        updated_at: 1700061000,
    };
    let bytes = encode_to_vec(&param).expect("encode TwinParameter negative value");
    let (decoded, _): (TwinParameter, usize) =
        decode_from_slice(&bytes).expect("decode TwinParameter negative value");
    assert_eq!(param, decoded);
    assert_eq!(decoded.value_x1e6, -50_000);
}

// --- Test 15: Vec<AssetDigitalTwin> versioned v1.0.0 ---
#[test]
fn test_vec_asset_digital_twin_versioned_v1() {
    let twins = vec![
        AssetDigitalTwin {
            twin_id: 1010,
            asset_id: 5010,
            name: "Compressor-E1".to_string(),
            state: TwinState::Synchronized,
            last_sync: 1700070000,
            accuracy_pct: 99,
        },
        AssetDigitalTwin {
            twin_id: 1011,
            asset_id: 5011,
            name: "Compressor-E2".to_string(),
            state: TwinState::Calibrating,
            last_sync: 1700070500,
            accuracy_pct: 91,
        },
        AssetDigitalTwin {
            twin_id: 1012,
            asset_id: 5012,
            name: "Compressor-E3".to_string(),
            state: TwinState::Diverged,
            last_sync: 1700071000,
            accuracy_pct: 63,
        },
    ];
    let ver = Version::new(1, 0, 0);
    let bytes =
        encode_versioned_value(&twins, ver).expect("encode versioned Vec<AssetDigitalTwin> v1.0.0");
    let (decoded, version, _): (Vec<AssetDigitalTwin>, Version, usize) =
        decode_versioned_value::<Vec<AssetDigitalTwin>>(&bytes)
            .expect("decode versioned Vec<AssetDigitalTwin> v1.0.0");
    assert_eq!(twins, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(decoded.len(), 3);
}

// --- Test 16: Vec<SimulationRun> versioned v2.0.0 ---
#[test]
fn test_vec_simulation_run_versioned_v2() {
    let runs = vec![
        SimulationRun {
            run_id: 2010,
            twin_id: 1001,
            status: SimulationStatus::Completed,
            model: PhysicsModel::Thermal,
            start_time: 1700080000,
            elapsed_ms: 60000,
            step_count: 6000,
        },
        SimulationRun {
            run_id: 2011,
            twin_id: 1001,
            status: SimulationStatus::Paused,
            model: PhysicsModel::Flexible,
            start_time: 1700081000,
            elapsed_ms: 30000,
            step_count: 3000,
        },
    ];
    let ver = Version::new(2, 0, 0);
    let bytes =
        encode_versioned_value(&runs, ver).expect("encode versioned Vec<SimulationRun> v2.0.0");
    let (decoded, version, _): (Vec<SimulationRun>, Version, usize) =
        decode_versioned_value::<Vec<SimulationRun>>(&bytes)
            .expect("decode versioned Vec<SimulationRun> v2.0.0");
    assert_eq!(runs, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(decoded[0].model, PhysicsModel::Thermal);
    assert_eq!(decoded[1].status, SimulationStatus::Paused);
}

// --- Test 17: Vec<SensorFusion> versioned v2.1.0 ---
#[test]
fn test_vec_sensor_fusion_versioned_v2_1() {
    let fusions = vec![
        SensorFusion {
            fusion_id: 3010,
            twin_id: 1005,
            method: SensorFusionMethod::Kalman,
            sensor_count: 16,
            confidence_x1000: 975,
            last_updated: 1700090000,
        },
        SensorFusion {
            fusion_id: 3011,
            twin_id: 1005,
            method: SensorFusionMethod::Bayesian,
            sensor_count: 16,
            confidence_x1000: 940,
            last_updated: 1700090500,
        },
        SensorFusion {
            fusion_id: 3012,
            twin_id: 1006,
            method: SensorFusionMethod::ParticleFilter,
            sensor_count: 64,
            confidence_x1000: 910,
            last_updated: 1700091000,
        },
    ];
    let ver = Version::new(2, 1, 0);
    let bytes =
        encode_versioned_value(&fusions, ver).expect("encode versioned Vec<SensorFusion> v2.1.0");
    let (decoded, version, _): (Vec<SensorFusion>, Version, usize) =
        decode_versioned_value::<Vec<SensorFusion>>(&bytes)
            .expect("decode versioned Vec<SensorFusion> v2.1.0");
    assert_eq!(fusions, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 1);
    assert_eq!(decoded.len(), 3);
}

// --- Test 18: Vec<AnomalyDetection> mixed severities versioned v1.0.0 ---
#[test]
fn test_vec_anomaly_detection_mixed_versioned_v1() {
    let anomalies = vec![
        AnomalyDetection {
            anomaly_id: 4010,
            twin_id: 1007,
            timestamp: 1700100000,
            severity: 2,
            description: "Minor vibration drift in bearing assembly".to_string(),
            auto_corrected: true,
        },
        AnomalyDetection {
            anomaly_id: 4011,
            twin_id: 1007,
            timestamp: 1700100300,
            severity: 9,
            description: "Critical misalignment in drive shaft coupling".to_string(),
            auto_corrected: false,
        },
    ];
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&anomalies, ver)
        .expect("encode versioned Vec<AnomalyDetection> v1.0.0");
    let (decoded, version, _): (Vec<AnomalyDetection>, Version, usize) =
        decode_versioned_value::<Vec<AnomalyDetection>>(&bytes)
            .expect("decode versioned Vec<AnomalyDetection> v1.0.0");
    assert_eq!(anomalies, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(decoded[0].auto_corrected, true);
    assert_eq!(decoded[1].severity, 9);
}

// --- Test 19: Vec<TwinParameter> multi-physics parameters versioned v2.0.0 ---
#[test]
fn test_vec_twin_parameter_multi_physics_versioned_v2() {
    let params = vec![
        TwinParameter {
            twin_id: 1008,
            param_name: "youngs_modulus_pa".to_string(),
            value_x1e6: 210_000_000_000_i64,
            uncertainty_x1e6: 500_000_000,
            updated_at: 1700110000,
        },
        TwinParameter {
            twin_id: 1008,
            param_name: "poisson_ratio".to_string(),
            value_x1e6: 300_000,
            uncertainty_x1e6: 5_000,
            updated_at: 1700110001,
        },
        TwinParameter {
            twin_id: 1008,
            param_name: "density_kg_m3".to_string(),
            value_x1e6: 7_850_000_000_i64,
            uncertainty_x1e6: 10_000_000,
            updated_at: 1700110002,
        },
    ];
    let ver = Version::new(2, 0, 0);
    let bytes =
        encode_versioned_value(&params, ver).expect("encode versioned Vec<TwinParameter> v2.0.0");
    let (decoded, version, _): (Vec<TwinParameter>, Version, usize) =
        decode_versioned_value::<Vec<TwinParameter>>(&bytes)
            .expect("decode versioned Vec<TwinParameter> v2.0.0");
    assert_eq!(params, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(decoded[1].param_name, "poisson_ratio");
}

// --- Test 20: PhysicsModel all variants encode/decode ---
#[test]
fn test_physics_model_all_variants() {
    let models = vec![
        PhysicsModel::Rigid,
        PhysicsModel::Flexible,
        PhysicsModel::FluidDynamic,
        PhysicsModel::Thermal,
        PhysicsModel::Electromagnetic,
    ];
    for model in &models {
        let bytes = encode_to_vec(model).expect("encode PhysicsModel variant");
        let (decoded, _): (PhysicsModel, usize) =
            decode_from_slice(&bytes).expect("decode PhysicsModel variant");
        assert_eq!(model, &decoded);
    }
}

// --- Test 21: Version field access and version ordering for industrial protocol ---
#[test]
fn test_version_fields_industrial_protocol_upgrade() {
    let twin = AssetDigitalTwin {
        twin_id: 1020,
        asset_id: 5020,
        name: "ReactorCore-F9".to_string(),
        state: TwinState::Synchronized,
        last_sync: 1700120000,
        accuracy_pct: 100,
    };

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v2_1 = Version::new(2, 1, 0);

    let bytes_v1 = encode_versioned_value(&twin, v1).expect("encode v1.0.0 twin");
    let bytes_v2 = encode_versioned_value(&twin, v2).expect("encode v2.0.0 twin");
    let bytes_v2_1 = encode_versioned_value(&twin, v2_1).expect("encode v2.1.0 twin");

    let (_, ver1, _): (AssetDigitalTwin, Version, usize) =
        decode_versioned_value::<AssetDigitalTwin>(&bytes_v1).expect("decode v1.0.0 twin");
    let (_, ver2, _): (AssetDigitalTwin, Version, usize) =
        decode_versioned_value::<AssetDigitalTwin>(&bytes_v2).expect("decode v2.0.0 twin");
    let (_, ver2_1, _): (AssetDigitalTwin, Version, usize) =
        decode_versioned_value::<AssetDigitalTwin>(&bytes_v2_1).expect("decode v2.1.0 twin");

    assert_eq!(ver1.major, 1);
    assert_eq!(ver1.minor, 0);
    assert_eq!(ver1.patch, 0);

    assert_eq!(ver2.major, 2);
    assert_eq!(ver2.minor, 0);
    assert_eq!(ver2.patch, 0);

    assert_eq!(ver2_1.major, 2);
    assert_eq!(ver2_1.minor, 1);
    assert_eq!(ver2_1.patch, 0);

    // Protocol upgrade: v2 is newer than v1 on major
    assert!(ver2.major > ver1.major);
    // Protocol patch: v2.1 adds minor feature over v2.0
    assert!(ver2_1.minor > ver2.minor);
}

// --- Test 22: End-to-end digital twin simulation cycle with all types ---
#[test]
fn test_end_to_end_digital_twin_simulation_cycle() {
    let twin = AssetDigitalTwin {
        twin_id: 9999,
        asset_id: 8888,
        name: "NuclearCoolantPump-G1".to_string(),
        state: TwinState::Synchronized,
        last_sync: 1700200000,
        accuracy_pct: 98,
    };

    let run = SimulationRun {
        run_id: 7777,
        twin_id: 9999,
        status: SimulationStatus::Running,
        model: PhysicsModel::FluidDynamic,
        start_time: 1700200100,
        elapsed_ms: 0,
        step_count: 0,
    };

    let fusion = SensorFusion {
        fusion_id: 6666,
        twin_id: 9999,
        method: SensorFusionMethod::Kalman,
        sensor_count: 48,
        confidence_x1000: 993,
        last_updated: 1700200050,
    };

    let param = TwinParameter {
        twin_id: 9999,
        param_name: "flow_rate_m3_per_s".to_string(),
        value_x1e6: 1_500_000,
        uncertainty_x1e6: 10_000,
        updated_at: 1700200050,
    };

    let anomaly = AnomalyDetection {
        anomaly_id: 5555,
        twin_id: 9999,
        timestamp: 1700200200,
        severity: 3,
        description: "Slight cavitation detected in impeller zone".to_string(),
        auto_corrected: true,
    };

    let ver = Version::new(2, 1, 0);

    let bytes_twin = encode_versioned_value(&twin, ver).expect("encode cycle twin");
    let bytes_run = encode_versioned_value(&run, ver).expect("encode cycle run");
    let bytes_fusion = encode_versioned_value(&fusion, ver).expect("encode cycle fusion");
    let bytes_param = encode_versioned_value(&param, ver).expect("encode cycle param");
    let bytes_anomaly = encode_versioned_value(&anomaly, ver).expect("encode cycle anomaly");

    let (dec_twin, ver_twin, _): (AssetDigitalTwin, Version, usize) =
        decode_versioned_value::<AssetDigitalTwin>(&bytes_twin).expect("decode cycle twin");
    let (dec_run, ver_run, _): (SimulationRun, Version, usize) =
        decode_versioned_value::<SimulationRun>(&bytes_run).expect("decode cycle run");
    let (dec_fusion, ver_fusion, _): (SensorFusion, Version, usize) =
        decode_versioned_value::<SensorFusion>(&bytes_fusion).expect("decode cycle fusion");
    let (dec_param, ver_param, _): (TwinParameter, Version, usize) =
        decode_versioned_value::<TwinParameter>(&bytes_param).expect("decode cycle param");
    let (dec_anomaly, ver_anomaly, _): (AnomalyDetection, Version, usize) =
        decode_versioned_value::<AnomalyDetection>(&bytes_anomaly).expect("decode cycle anomaly");

    assert_eq!(twin, dec_twin);
    assert_eq!(run, dec_run);
    assert_eq!(fusion, dec_fusion);
    assert_eq!(param, dec_param);
    assert_eq!(anomaly, dec_anomaly);

    for v in [ver_twin, ver_run, ver_fusion, ver_param, ver_anomaly] {
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 1);
        assert_eq!(v.patch, 0);
    }

    assert_eq!(dec_twin.state, TwinState::Synchronized);
    assert_eq!(dec_run.model, PhysicsModel::FluidDynamic);
    assert_eq!(dec_fusion.method, SensorFusionMethod::Kalman);
    assert_eq!(dec_param.param_name, "flow_rate_m3_per_s");
    assert!(dec_anomaly.auto_corrected);
}
