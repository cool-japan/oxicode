//! Checksum tests for OxiCode — edge AI inference pipelines theme.
//!
//! Exactly 22 `#[test]` functions exercising CRC32 checksum roundtrips with
//! domain types drawn from ML model metadata, inference requests/results,
//! deployment configs, hardware accelerator specs, and more.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced24_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — Edge AI inference pipeline
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QuantizationLevel {
    Float32,
    Float16,
    Int8,
    Int4,
    Binary,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelMetadata {
    name: String,
    version: String,
    layer_count: u32,
    parameter_count: u64,
    quantization: QuantizationLevel,
    input_shapes: Vec<Vec<u32>>,
    output_shapes: Vec<Vec<u32>>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InferenceRequest {
    request_id: u64,
    model_name: String,
    batch_size: u32,
    input_tensor_shape: Vec<u32>,
    input_data: Vec<f32>,
    priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoundingBox {
    x_min: f32,
    y_min: f32,
    x_max: f32,
    y_max: f32,
    class_id: u32,
    confidence: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClassProbability {
    class_name: String,
    probability: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InferenceResult {
    request_id: u64,
    class_probabilities: Vec<ClassProbability>,
    bounding_boxes: Vec<BoundingBox>,
    latency_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RuntimeFormat {
    Onnx,
    TfLite,
    TensorRt,
    CoreMl,
    OpenVino,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeploymentConfig {
    model_name: String,
    runtime: RuntimeFormat,
    max_batch_size: u32,
    timeout_ms: u64,
    enable_dynamic_batching: bool,
    num_replicas: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AcceleratorType {
    Gpu,
    Tpu,
    Npu,
    Fpga,
    Cpu,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HardwareAccelerator {
    name: String,
    accelerator_type: AcceleratorType,
    memory_mb: u64,
    compute_units: u32,
    clock_mhz: u32,
    power_watts: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LatencyMeasurement {
    model_name: String,
    batch_size: u32,
    preprocess_us: u64,
    inference_us: u64,
    postprocess_us: u64,
    total_us: u64,
    percentile_p50_us: u64,
    percentile_p99_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThroughputMetrics {
    model_name: String,
    requests_per_second: f64,
    tokens_per_second: f64,
    images_per_second: f64,
    gpu_utilization_pct: f32,
    memory_utilization_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelVersion {
    model_id: String,
    major: u32,
    minor: u32,
    patch: u32,
    commit_hash: String,
    trained_epochs: u32,
    validation_accuracy: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AbTestAssignment {
    experiment_id: String,
    user_segment: String,
    model_variant_a: String,
    model_variant_b: String,
    traffic_split_pct: f32,
    assigned_variant: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FeatureType {
    Embedding(Vec<f32>),
    Scalar(f64),
    Categorical(String),
    Histogram(Vec<u32>),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeatureExtractionPipeline {
    pipeline_name: String,
    stages: Vec<String>,
    extracted_features: Vec<FeatureType>,
    total_extraction_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnomalyDetectionScore {
    sensor_id: String,
    timestamp_epoch_ms: u64,
    raw_value: f64,
    anomaly_score: f64,
    is_anomalous: bool,
    contributing_features: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FederatedLearningRound {
    round_number: u32,
    participant_count: u32,
    aggregated_loss: f64,
    global_accuracy: f64,
    model_delta_size_bytes: u64,
    convergence_metric: f64,
    dropped_participants: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdgeDeviceProfile {
    device_id: String,
    accelerator: HardwareAccelerator,
    available_memory_mb: u64,
    battery_pct: Option<f32>,
    network_bandwidth_kbps: u32,
    deployed_models: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelOptimizationRecord {
    original_size_bytes: u64,
    optimized_size_bytes: u64,
    quantization: QuantizationLevel,
    pruning_ratio: f32,
    distillation_teacher: Option<String>,
    accuracy_delta: f64,
    speedup_factor: f64,
}

// ---------------------------------------------------------------------------
// Test 1: Model metadata checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_model_metadata_checksum_roundtrip() {
    let meta = ModelMetadata {
        name: "mobilenet_v3_small".into(),
        version: "2.1.0".into(),
        layer_count: 62,
        parameter_count: 2_537_000,
        quantization: QuantizationLevel::Int8,
        input_shapes: vec![vec![1, 3, 224, 224]],
        output_shapes: vec![vec![1, 1000]],
    };
    let encoded = encode_with_checksum(&meta).expect("encode model metadata failed");
    let (decoded, consumed): (ModelMetadata, _) =
        decode_with_checksum(&encoded).expect("decode model metadata failed");
    assert_eq!(decoded, meta);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Inference request checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_inference_request_checksum_roundtrip() {
    let req = InferenceRequest {
        request_id: 9988776655,
        model_name: "yolov8n".into(),
        batch_size: 4,
        input_tensor_shape: vec![4, 3, 640, 640],
        input_data: vec![0.5, 0.3, 0.7, 0.1, 0.9, 0.2, 0.8, 0.4],
        priority: 2,
    };
    let encoded = encode_with_checksum(&req).expect("encode inference request failed");
    let (decoded, consumed): (InferenceRequest, _) =
        decode_with_checksum(&encoded).expect("decode inference request failed");
    assert_eq!(decoded, req);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: Inference result with bounding boxes
// ---------------------------------------------------------------------------
#[test]
fn test_inference_result_bounding_boxes_checksum() {
    let result = InferenceResult {
        request_id: 42,
        class_probabilities: vec![
            ClassProbability {
                class_name: "cat".into(),
                probability: 0.92,
            },
            ClassProbability {
                class_name: "dog".into(),
                probability: 0.05,
            },
            ClassProbability {
                class_name: "bird".into(),
                probability: 0.03,
            },
        ],
        bounding_boxes: vec![BoundingBox {
            x_min: 10.5,
            y_min: 20.3,
            x_max: 150.7,
            y_max: 200.1,
            class_id: 0,
            confidence: 0.92,
        }],
        latency_ms: 3.14,
    };
    let encoded = encode_with_checksum(&result).expect("encode inference result failed");
    let (decoded, consumed): (InferenceResult, _) =
        decode_with_checksum(&encoded).expect("decode inference result failed");
    assert_eq!(decoded, result);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Deployment config for multiple runtimes
// ---------------------------------------------------------------------------
#[test]
fn test_deployment_config_checksum_roundtrip() {
    let configs = vec![
        DeploymentConfig {
            model_name: "resnet50".into(),
            runtime: RuntimeFormat::TensorRt,
            max_batch_size: 32,
            timeout_ms: 5000,
            enable_dynamic_batching: true,
            num_replicas: 3,
        },
        DeploymentConfig {
            model_name: "bert_tiny".into(),
            runtime: RuntimeFormat::Onnx,
            max_batch_size: 8,
            timeout_ms: 2000,
            enable_dynamic_batching: false,
            num_replicas: 1,
        },
    ];
    let encoded = encode_with_checksum(&configs).expect("encode deployment configs failed");
    let (decoded, consumed): (Vec<DeploymentConfig>, _) =
        decode_with_checksum(&encoded).expect("decode deployment configs failed");
    assert_eq!(decoded, configs);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Hardware accelerator specs
// ---------------------------------------------------------------------------
#[test]
fn test_hardware_accelerator_checksum_roundtrip() {
    let hw = HardwareAccelerator {
        name: "NVIDIA Jetson Orin NX".into(),
        accelerator_type: AcceleratorType::Gpu,
        memory_mb: 16384,
        compute_units: 1024,
        clock_mhz: 918,
        power_watts: 25.0,
    };
    let encoded = encode_with_checksum(&hw).expect("encode hardware accelerator failed");
    let (decoded, consumed): (HardwareAccelerator, _) =
        decode_with_checksum(&encoded).expect("decode hardware accelerator failed");
    assert_eq!(decoded, hw);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Latency measurements with percentiles
// ---------------------------------------------------------------------------
#[test]
fn test_latency_measurement_checksum_roundtrip() {
    let latency = LatencyMeasurement {
        model_name: "efficientnet_b0".into(),
        batch_size: 1,
        preprocess_us: 450,
        inference_us: 2800,
        postprocess_us: 120,
        total_us: 3370,
        percentile_p50_us: 2750,
        percentile_p99_us: 4100,
    };
    let encoded = encode_with_checksum(&latency).expect("encode latency failed");
    let (decoded, consumed): (LatencyMeasurement, _) =
        decode_with_checksum(&encoded).expect("decode latency failed");
    assert_eq!(decoded, latency);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Throughput metrics
// ---------------------------------------------------------------------------
#[test]
fn test_throughput_metrics_checksum_roundtrip() {
    let metrics = ThroughputMetrics {
        model_name: "whisper_tiny".into(),
        requests_per_second: 145.7,
        tokens_per_second: 3200.5,
        images_per_second: 0.0,
        gpu_utilization_pct: 87.3,
        memory_utilization_pct: 62.1,
    };
    let encoded = encode_with_checksum(&metrics).expect("encode throughput metrics failed");
    let (decoded, consumed): (ThroughputMetrics, _) =
        decode_with_checksum(&encoded).expect("decode throughput metrics failed");
    assert_eq!(decoded, metrics);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Model versioning
// ---------------------------------------------------------------------------
#[test]
fn test_model_version_checksum_roundtrip() {
    let ver = ModelVersion {
        model_id: "img-cls-prod-v3".into(),
        major: 3,
        minor: 2,
        patch: 1,
        commit_hash: "a1b2c3d4e5f6".into(),
        trained_epochs: 120,
        validation_accuracy: 0.9534,
    };
    let encoded = encode_with_checksum(&ver).expect("encode model version failed");
    let (decoded, consumed): (ModelVersion, _) =
        decode_with_checksum(&encoded).expect("decode model version failed");
    assert_eq!(decoded, ver);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: A/B test assignment
// ---------------------------------------------------------------------------
#[test]
fn test_ab_test_assignment_checksum_roundtrip() {
    let assignment = AbTestAssignment {
        experiment_id: "exp-2026-q1-latency".into(),
        user_segment: "premium_tier".into(),
        model_variant_a: "resnet50_fp16".into(),
        model_variant_b: "mobilenet_v3_int8".into(),
        traffic_split_pct: 70.0,
        assigned_variant: 0,
    };
    let encoded = encode_with_checksum(&assignment).expect("encode ab test failed");
    let (decoded, consumed): (AbTestAssignment, _) =
        decode_with_checksum(&encoded).expect("decode ab test failed");
    assert_eq!(decoded, assignment);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Feature extraction pipeline with mixed feature types
// ---------------------------------------------------------------------------
#[test]
fn test_feature_extraction_pipeline_checksum_roundtrip() {
    let pipeline = FeatureExtractionPipeline {
        pipeline_name: "image_embedding_v2".into(),
        stages: vec![
            "resize_256".into(),
            "center_crop_224".into(),
            "normalize".into(),
            "backbone_forward".into(),
            "pooling".into(),
        ],
        extracted_features: vec![
            FeatureType::Embedding(vec![0.1, -0.3, 0.7, 0.05, -0.92]),
            FeatureType::Scalar(0.87),
            FeatureType::Categorical("outdoor_scene".into()),
            FeatureType::Histogram(vec![12, 45, 78, 23, 5]),
        ],
        total_extraction_ms: 8.42,
    };
    let encoded = encode_with_checksum(&pipeline).expect("encode feature pipeline failed");
    let (decoded, consumed): (FeatureExtractionPipeline, _) =
        decode_with_checksum(&encoded).expect("decode feature pipeline failed");
    assert_eq!(decoded, pipeline);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Anomaly detection scores
// ---------------------------------------------------------------------------
#[test]
fn test_anomaly_detection_score_checksum_roundtrip() {
    let score = AnomalyDetectionScore {
        sensor_id: "vibration-sensor-42".into(),
        timestamp_epoch_ms: 1_710_000_000_000,
        raw_value: 3.78,
        anomaly_score: 0.94,
        is_anomalous: true,
        contributing_features: vec![
            "frequency_peak_shift".into(),
            "amplitude_deviation".into(),
            "harmonic_ratio".into(),
        ],
    };
    let encoded = encode_with_checksum(&score).expect("encode anomaly score failed");
    let (decoded, consumed): (AnomalyDetectionScore, _) =
        decode_with_checksum(&encoded).expect("decode anomaly score failed");
    assert_eq!(decoded, score);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Federated learning round
// ---------------------------------------------------------------------------
#[test]
fn test_federated_learning_round_checksum_roundtrip() {
    let round = FederatedLearningRound {
        round_number: 57,
        participant_count: 1200,
        aggregated_loss: 0.0342,
        global_accuracy: 0.9187,
        model_delta_size_bytes: 4_500_000,
        convergence_metric: 0.0015,
        dropped_participants: 3,
    };
    let encoded = encode_with_checksum(&round).expect("encode federated round failed");
    let (decoded, consumed): (FederatedLearningRound, _) =
        decode_with_checksum(&encoded).expect("decode federated round failed");
    assert_eq!(decoded, round);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Edge device profile (nested structs)
// ---------------------------------------------------------------------------
#[test]
fn test_edge_device_profile_checksum_roundtrip() {
    let device = EdgeDeviceProfile {
        device_id: "edge-node-asia-017".into(),
        accelerator: HardwareAccelerator {
            name: "Google Coral TPU".into(),
            accelerator_type: AcceleratorType::Tpu,
            memory_mb: 2048,
            compute_units: 4,
            clock_mhz: 500,
            power_watts: 2.0,
        },
        available_memory_mb: 4096,
        battery_pct: Some(73.5),
        network_bandwidth_kbps: 15000,
        deployed_models: vec![
            "person_detection_v2".into(),
            "gesture_recognition_v1".into(),
        ],
    };
    let encoded = encode_with_checksum(&device).expect("encode edge device failed");
    let (decoded, consumed): (EdgeDeviceProfile, _) =
        decode_with_checksum(&encoded).expect("decode edge device failed");
    assert_eq!(decoded, device);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Model optimization record
// ---------------------------------------------------------------------------
#[test]
fn test_model_optimization_record_checksum_roundtrip() {
    let record = ModelOptimizationRecord {
        original_size_bytes: 250_000_000,
        optimized_size_bytes: 12_500_000,
        quantization: QuantizationLevel::Int4,
        pruning_ratio: 0.65,
        distillation_teacher: Some("resnet152_pretrained".into()),
        accuracy_delta: -0.012,
        speedup_factor: 8.7,
    };
    let encoded = encode_with_checksum(&record).expect("encode optimization record failed");
    let (decoded, consumed): (ModelOptimizationRecord, _) =
        decode_with_checksum(&encoded).expect("decode optimization record failed");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Plain encode/decode for quantization level enum
// ---------------------------------------------------------------------------
#[test]
fn test_quantization_levels_plain_roundtrip() {
    let levels = vec![
        QuantizationLevel::Float32,
        QuantizationLevel::Float16,
        QuantizationLevel::Int8,
        QuantizationLevel::Int4,
        QuantizationLevel::Binary,
    ];
    let encoded = encode_to_vec(&levels).expect("encode quantization levels failed");
    let (decoded, consumed): (Vec<QuantizationLevel>, _) =
        decode_from_slice(&encoded).expect("decode quantization levels failed");
    assert_eq!(decoded, levels);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Plain encode/decode for runtime format variants
// ---------------------------------------------------------------------------
#[test]
fn test_runtime_format_plain_roundtrip() {
    let runtimes = vec![
        RuntimeFormat::Onnx,
        RuntimeFormat::TfLite,
        RuntimeFormat::TensorRt,
        RuntimeFormat::CoreMl,
        RuntimeFormat::OpenVino,
    ];
    let encoded = encode_to_vec(&runtimes).expect("encode runtimes failed");
    let (decoded, consumed): (Vec<RuntimeFormat>, _) =
        decode_from_slice(&encoded).expect("decode runtimes failed");
    assert_eq!(decoded, runtimes);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Corrupted checksum detection on inference result
// ---------------------------------------------------------------------------
#[test]
fn test_corrupted_checksum_inference_result() {
    let result = InferenceResult {
        request_id: 100,
        class_probabilities: vec![ClassProbability {
            class_name: "person".into(),
            probability: 0.88,
        }],
        bounding_boxes: vec![BoundingBox {
            x_min: 50.0,
            y_min: 30.0,
            x_max: 200.0,
            y_max: 400.0,
            class_id: 0,
            confidence: 0.88,
        }],
        latency_ms: 5.67,
    };
    let mut encoded = encode_with_checksum(&result).expect("encode inference result failed");
    // Corrupt a payload byte
    let payload_offset = oxicode::checksum::HEADER_SIZE + 2;
    if payload_offset < encoded.len() {
        encoded[payload_offset] ^= 0xFF;
    }
    let decode_result: oxicode::Result<(InferenceResult, usize)> = decode_with_checksum(&encoded);
    assert!(
        decode_result.is_err(),
        "corrupted checksum must fail to decode"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Multiple anomaly scores in a batch
// ---------------------------------------------------------------------------
#[test]
fn test_batch_anomaly_scores_checksum_roundtrip() {
    let scores: Vec<AnomalyDetectionScore> = (0..10)
        .map(|i| AnomalyDetectionScore {
            sensor_id: format!("sensor-{:03}", i),
            timestamp_epoch_ms: 1_710_000_000_000 + i as u64 * 1000,
            raw_value: 1.0 + i as f64 * 0.5,
            anomaly_score: if i % 3 == 0 { 0.95 } else { 0.12 },
            is_anomalous: i % 3 == 0,
            contributing_features: vec![format!("feature_{}", i)],
        })
        .collect();
    let encoded = encode_with_checksum(&scores).expect("encode batch anomaly scores failed");
    let (decoded, consumed): (Vec<AnomalyDetectionScore>, _) =
        decode_with_checksum(&encoded).expect("decode batch anomaly scores failed");
    assert_eq!(decoded, scores);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: Plain encode/decode for hardware accelerator types
// ---------------------------------------------------------------------------
#[test]
fn test_accelerator_types_plain_roundtrip() {
    let types = vec![
        AcceleratorType::Gpu,
        AcceleratorType::Tpu,
        AcceleratorType::Npu,
        AcceleratorType::Fpga,
        AcceleratorType::Cpu,
    ];
    let encoded = encode_to_vec(&types).expect("encode accelerator types failed");
    let (decoded, consumed): (Vec<AcceleratorType>, _) =
        decode_from_slice(&encoded).expect("decode accelerator types failed");
    assert_eq!(decoded, types);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: Federated learning convergence series
// ---------------------------------------------------------------------------
#[test]
fn test_federated_learning_convergence_series_checksum() {
    let rounds: Vec<FederatedLearningRound> = (1..=5)
        .map(|r| FederatedLearningRound {
            round_number: r,
            participant_count: 500 + r * 10,
            aggregated_loss: 1.0 / (r as f64 + 1.0),
            global_accuracy: 0.7 + r as f64 * 0.04,
            model_delta_size_bytes: 10_000_000 - r as u64 * 500_000,
            convergence_metric: 0.1 / r as f64,
            dropped_participants: if r > 3 { 0 } else { 5 - r },
        })
        .collect();
    let encoded = encode_with_checksum(&rounds).expect("encode federated rounds failed");
    let (decoded, consumed): (Vec<FederatedLearningRound>, _) =
        decode_with_checksum(&encoded).expect("decode federated rounds failed");
    assert_eq!(decoded, rounds);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 21: Nested pipeline with optimization and deployment
// ---------------------------------------------------------------------------
#[test]
fn test_full_pipeline_config_checksum_roundtrip() {
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct FullPipelineConfig {
        model_meta: ModelMetadata,
        optimization: ModelOptimizationRecord,
        deployment: DeploymentConfig,
        target_device: EdgeDeviceProfile,
    }

    let config = FullPipelineConfig {
        model_meta: ModelMetadata {
            name: "detr_resnet50".into(),
            version: "1.0.0".into(),
            layer_count: 101,
            parameter_count: 41_300_000,
            quantization: QuantizationLevel::Float16,
            input_shapes: vec![vec![1, 3, 800, 800]],
            output_shapes: vec![vec![1, 100, 4], vec![1, 100, 91]],
        },
        optimization: ModelOptimizationRecord {
            original_size_bytes: 166_000_000,
            optimized_size_bytes: 83_000_000,
            quantization: QuantizationLevel::Float16,
            pruning_ratio: 0.0,
            distillation_teacher: None,
            accuracy_delta: -0.002,
            speedup_factor: 1.8,
        },
        deployment: DeploymentConfig {
            model_name: "detr_resnet50".into(),
            runtime: RuntimeFormat::TensorRt,
            max_batch_size: 4,
            timeout_ms: 10000,
            enable_dynamic_batching: true,
            num_replicas: 2,
        },
        target_device: EdgeDeviceProfile {
            device_id: "factory-floor-cam-09".into(),
            accelerator: HardwareAccelerator {
                name: "NVIDIA Jetson AGX Xavier".into(),
                accelerator_type: AcceleratorType::Gpu,
                memory_mb: 32768,
                compute_units: 512,
                clock_mhz: 1377,
                power_watts: 30.0,
            },
            available_memory_mb: 24576,
            battery_pct: None,
            network_bandwidth_kbps: 100000,
            deployed_models: vec!["detr_resnet50".into(), "ocr_crnn_v2".into()],
        },
    };
    let encoded = encode_with_checksum(&config).expect("encode full pipeline config failed");
    let (decoded, consumed): (FullPipelineConfig, _) =
        decode_with_checksum(&encoded).expect("decode full pipeline config failed");
    assert_eq!(decoded, config);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: Plain encode/decode for feature types enum variants
// ---------------------------------------------------------------------------
#[test]
fn test_feature_types_plain_roundtrip() {
    let features = vec![
        FeatureType::Embedding(vec![-1.0, 0.0, 1.0, 0.5, -0.5]),
        FeatureType::Scalar(42.195),
        FeatureType::Categorical("industrial_machinery".into()),
        FeatureType::Histogram(vec![0, 5, 23, 67, 120, 89, 34, 8, 1]),
    ];
    let encoded = encode_to_vec(&features).expect("encode feature types failed");
    let (decoded, consumed): (Vec<FeatureType>, _) =
        decode_from_slice(&encoded).expect("decode feature types failed");
    assert_eq!(decoded, features);
    assert_eq!(consumed, encoded.len());
}
