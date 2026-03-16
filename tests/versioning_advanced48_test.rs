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
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ── Domain types: Edge AI and On-Device Inference ────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QuantizationMode {
    Int8Symmetric,
    Int8Asymmetric,
    Fp16,
    MixedPrecision,
    Int4Grouped,
    BFloat16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ActivationFunction {
    Relu,
    Relu6,
    Sigmoid,
    Tanh,
    Swish,
    Gelu,
    HardSwish,
    LeakyRelu,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AcceleratorType {
    NpuGeneric,
    AppleNeuralEngine,
    QualcommHexagonDsp,
    GoogleEdgeTpu,
    IntelMovidius,
    CpuOnly,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PruningStrategy {
    Unstructured,
    StructuredChannelwise,
    StructuredFilterwise,
    BlockSparse,
    MovementPruning,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OnnxOpsetDomain {
    DefaultAi,
    MicrosoftNnapi,
    MicrosoftDml,
    CustomEdge,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FederatedAggregation {
    FedAvg,
    FedProx,
    FedSgd,
    SecureAggregation,
    DifferentialPrivacy,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuantizationConfig {
    model_name: String,
    mode: QuantizationMode,
    calibration_samples: u32,
    per_channel: bool,
    scale_factor_x1e6: u64,
    zero_point: i32,
    min_range_x1e6: i64,
    max_range_x1e6: i64,
    bit_width: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InferenceLatencyMeasurement {
    session_id: u64,
    model_name: String,
    device_name: String,
    preprocess_us: u64,
    inference_us: u64,
    postprocess_us: u64,
    total_us: u64,
    batch_size: u32,
    warmup_runs: u16,
    measured_runs: u16,
    p99_latency_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NpuConfiguration {
    device_id: u32,
    accelerator: AcceleratorType,
    firmware_version: String,
    max_compute_units: u16,
    clock_freq_mhz: u32,
    sram_kb: u32,
    supported_ops: Vec<String>,
    power_budget_mw: u32,
    thermal_limit_celsius: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelPruningMetadata {
    model_id: String,
    strategy: PruningStrategy,
    original_params: u64,
    pruned_params: u64,
    sparsity_ratio_bps: u16,
    accuracy_drop_bps: u16,
    pruning_epochs: u32,
    threshold_x1e6: u64,
    fine_tune_epochs: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KnowledgeDistillationResult {
    teacher_model: String,
    student_model: String,
    teacher_params: u64,
    student_params: u64,
    teacher_accuracy_bps: u16,
    student_accuracy_bps: u16,
    temperature_x100: u32,
    alpha_x1000: u32,
    distillation_epochs: u32,
    loss_x1e6: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OnDeviceTrainingState {
    training_id: u64,
    model_name: String,
    current_epoch: u32,
    total_epochs: u32,
    learning_rate_x1e8: u64,
    train_loss_x1e6: u64,
    val_loss_x1e6: u64,
    samples_processed: u64,
    gradient_norm_x1e6: u64,
    memory_used_kb: u64,
    battery_pct_start: u8,
    battery_pct_current: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FederatedLearningRound {
    round_id: u32,
    global_model_version: u64,
    aggregation: FederatedAggregation,
    num_participants: u32,
    min_participants_required: u32,
    global_loss_x1e6: u64,
    global_accuracy_bps: u16,
    communication_bytes: u64,
    round_duration_secs: u32,
    privacy_budget_x1e4: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TinyMlDeployConfig {
    target_mcu: String,
    flash_kb: u32,
    ram_kb: u32,
    model_size_bytes: u64,
    arena_size_bytes: u32,
    ops_resolver_entries: u16,
    input_tensor_bytes: u32,
    output_tensor_bytes: u32,
    interpreter_stack_bytes: u32,
    uses_cmsis_nn: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OnnxOperatorSet {
    domain: OnnxOpsetDomain,
    opset_version: u16,
    model_ir_version: u16,
    num_nodes: u32,
    num_initializers: u32,
    custom_ops: Vec<String>,
    producer_name: String,
    producer_version: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TensorShapeSpec {
    name: String,
    dimensions: Vec<u64>,
    element_size_bytes: u8,
    is_dynamic: bool,
    total_elements: u64,
    memory_layout_row_major: bool,
    alignment_bytes: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatchNormStatistics {
    layer_name: String,
    num_features: u32,
    running_mean_x1e6: Vec<i64>,
    running_var_x1e6: Vec<u64>,
    momentum_x1e6: u32,
    epsilon_x1e10: u64,
    affine: bool,
    track_running_stats: bool,
    num_batches_tracked: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ActivationLayerConfig {
    layer_index: u32,
    function: ActivationFunction,
    in_place: bool,
    negative_slope_x1e6: u32,
    input_channels: u32,
    output_channels: u32,
    fused_with_conv: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelCompressionSummary {
    model_name: String,
    original_size_bytes: u64,
    compressed_size_bytes: u64,
    quantization_mode: QuantizationMode,
    pruning_strategy: Option<PruningStrategy>,
    sparsity_bps: u16,
    accuracy_retained_bps: u16,
    latency_speedup_x100: u32,
    target_accelerator: AcceleratorType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdgeInferenceSession {
    session_id: u64,
    model_name: String,
    accelerator: AcceleratorType,
    input_shapes: Vec<Vec<u64>>,
    output_shapes: Vec<Vec<u64>>,
    num_threads: u8,
    enable_fp16: bool,
    enable_int8: bool,
    profiling_enabled: bool,
    max_memory_mb: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NeuralArchSearchResult {
    search_id: u64,
    architecture_hash: String,
    num_layers: u16,
    total_flops: u64,
    total_params: u64,
    accuracy_bps: u16,
    latency_us: u64,
    energy_uj: u64,
    pareto_optimal: bool,
    search_iterations: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeightSharingConfig {
    group_id: u32,
    shared_layers: Vec<u32>,
    codebook_size: u32,
    bits_per_weight: u8,
    reconstruction_error_x1e6: u64,
    compression_ratio_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OnDeviceCacheEntry {
    cache_key: String,
    model_version: u64,
    tensor_name: String,
    offset_bytes: u64,
    size_bytes: u64,
    last_accessed_epoch: u64,
    hit_count: u32,
    eviction_priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SplitInferencePartition {
    partition_id: u32,
    total_partitions: u32,
    start_layer: u32,
    end_layer: u32,
    device_target: AcceleratorType,
    intermediate_tensor_bytes: u64,
    estimated_latency_us: u64,
    communication_overhead_us: u64,
    requires_sync: bool,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_quantization_config_int8_symmetric() {
    let val = QuantizationConfig {
        model_name: "mobilenet_v3_small".to_string(),
        mode: QuantizationMode::Int8Symmetric,
        calibration_samples: 500,
        per_channel: true,
        scale_factor_x1e6: 3_921,
        zero_point: 0,
        min_range_x1e6: -128_000_000,
        max_range_x1e6: 127_000_000,
        bit_width: 8,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode QuantizationConfig");
    let (decoded, decoded_version, _size): (QuantizationConfig, Version, usize) =
        decode_versioned_value(&bytes).expect("decode QuantizationConfig");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_quantization_config_mixed_precision() {
    let val = QuantizationConfig {
        model_name: "efficientnet_lite4".to_string(),
        mode: QuantizationMode::MixedPrecision,
        calibration_samples: 1000,
        per_channel: false,
        scale_factor_x1e6: 7_843,
        zero_point: 128,
        min_range_x1e6: 0,
        max_range_x1e6: 255_000_000,
        bit_width: 8,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode mixed precision config");
    let (decoded, decoded_version, _size): (QuantizationConfig, Version, usize) =
        decode_versioned_value(&bytes).expect("decode mixed precision config");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_inference_latency_measurement() {
    let val = InferenceLatencyMeasurement {
        session_id: 99001,
        model_name: "yolov8n".to_string(),
        device_name: "Pixel 8 Tensor G3".to_string(),
        preprocess_us: 1_200,
        inference_us: 8_500,
        postprocess_us: 450,
        total_us: 10_150,
        batch_size: 1,
        warmup_runs: 10,
        measured_runs: 100,
        p99_latency_us: 12_300,
    };
    let version = Version::new(1, 2, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode InferenceLatencyMeasurement");
    let (decoded, decoded_version, _size): (InferenceLatencyMeasurement, Version, usize) =
        decode_versioned_value(&bytes).expect("decode InferenceLatencyMeasurement");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_npu_configuration_apple_neural_engine() {
    let val = NpuConfiguration {
        device_id: 1,
        accelerator: AcceleratorType::AppleNeuralEngine,
        firmware_version: "17.4.1".to_string(),
        max_compute_units: 16,
        clock_freq_mhz: 1000,
        sram_kb: 32768,
        supported_ops: vec![
            "Conv2D".to_string(),
            "DepthwiseConv2D".to_string(),
            "FullyConnected".to_string(),
            "Softmax".to_string(),
            "BatchNorm".to_string(),
        ],
        power_budget_mw: 5000,
        thermal_limit_celsius: 95,
    };
    let version = Version::new(1, 0, 3);
    let bytes = encode_versioned_value(&val, version).expect("encode NpuConfiguration ANE");
    let (decoded, decoded_version, _size): (NpuConfiguration, Version, usize) =
        decode_versioned_value(&bytes).expect("decode NpuConfiguration ANE");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_npu_configuration_edge_tpu() {
    let val = NpuConfiguration {
        device_id: 42,
        accelerator: AcceleratorType::GoogleEdgeTpu,
        firmware_version: "14.1".to_string(),
        max_compute_units: 8,
        clock_freq_mhz: 500,
        sram_kb: 8192,
        supported_ops: vec![
            "Conv2D".to_string(),
            "MaxPool2D".to_string(),
            "Quantize".to_string(),
        ],
        power_budget_mw: 2000,
        thermal_limit_celsius: 85,
    };
    let version = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode NpuConfiguration EdgeTPU");
    let (decoded, decoded_version, _size): (NpuConfiguration, Version, usize) =
        decode_versioned_value(&bytes).expect("decode NpuConfiguration EdgeTPU");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_model_pruning_metadata_structured() {
    let val = ModelPruningMetadata {
        model_id: "resnet50_pruned_v3".to_string(),
        strategy: PruningStrategy::StructuredChannelwise,
        original_params: 25_557_032,
        pruned_params: 12_778_516,
        sparsity_ratio_bps: 5000,
        accuracy_drop_bps: 85,
        pruning_epochs: 30,
        threshold_x1e6: 10_000,
        fine_tune_epochs: 15,
    };
    let version = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode ModelPruningMetadata");
    let (decoded, decoded_version, _size): (ModelPruningMetadata, Version, usize) =
        decode_versioned_value(&bytes).expect("decode ModelPruningMetadata");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_knowledge_distillation_result() {
    let val = KnowledgeDistillationResult {
        teacher_model: "bert_large_uncased".to_string(),
        student_model: "distilbert_base".to_string(),
        teacher_params: 340_000_000,
        student_params: 66_000_000,
        teacher_accuracy_bps: 9350,
        student_accuracy_bps: 9180,
        temperature_x100: 400,
        alpha_x1000: 700,
        distillation_epochs: 50,
        loss_x1e6: 234_567,
    };
    let version = Version::new(1, 3, 2);
    let bytes = encode_versioned_value(&val, version).expect("encode KnowledgeDistillationResult");
    let (decoded, decoded_version, _size): (KnowledgeDistillationResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode KnowledgeDistillationResult");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_on_device_training_state() {
    let val = OnDeviceTrainingState {
        training_id: 7700,
        model_name: "personalization_head_v2".to_string(),
        current_epoch: 5,
        total_epochs: 10,
        learning_rate_x1e8: 100,
        train_loss_x1e6: 345_000,
        val_loss_x1e6: 412_000,
        samples_processed: 12_800,
        gradient_norm_x1e6: 1_500_000,
        memory_used_kb: 524_288,
        battery_pct_start: 87,
        battery_pct_current: 72,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode OnDeviceTrainingState");
    let (decoded, decoded_version, _size): (OnDeviceTrainingState, Version, usize) =
        decode_versioned_value(&bytes).expect("decode OnDeviceTrainingState");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_federated_learning_round_fedavg() {
    let val = FederatedLearningRound {
        round_id: 142,
        global_model_version: 1420,
        aggregation: FederatedAggregation::FedAvg,
        num_participants: 500,
        min_participants_required: 100,
        global_loss_x1e6: 187_432,
        global_accuracy_bps: 8920,
        communication_bytes: 4_194_304,
        round_duration_secs: 3600,
        privacy_budget_x1e4: 10_000,
    };
    let version = Version::new(2, 0, 1);
    let bytes =
        encode_versioned_value(&val, version).expect("encode FederatedLearningRound FedAvg");
    let (decoded, decoded_version, _size): (FederatedLearningRound, Version, usize) =
        decode_versioned_value(&bytes).expect("decode FederatedLearningRound FedAvg");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_federated_learning_round_differential_privacy() {
    let val = FederatedLearningRound {
        round_id: 88,
        global_model_version: 880,
        aggregation: FederatedAggregation::DifferentialPrivacy,
        num_participants: 10_000,
        min_participants_required: 2_000,
        global_loss_x1e6: 298_111,
        global_accuracy_bps: 8640,
        communication_bytes: 16_777_216,
        round_duration_secs: 7200,
        privacy_budget_x1e4: 5_000,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode FederatedLearningRound DP");
    let (decoded, decoded_version, _size): (FederatedLearningRound, Version, usize) =
        decode_versioned_value(&bytes).expect("decode FederatedLearningRound DP");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_tinyml_deploy_config_cortex_m4() {
    let val = TinyMlDeployConfig {
        target_mcu: "STM32F411RE_Cortex-M4".to_string(),
        flash_kb: 512,
        ram_kb: 128,
        model_size_bytes: 98_304,
        arena_size_bytes: 65_536,
        ops_resolver_entries: 12,
        input_tensor_bytes: 9_408,
        output_tensor_bytes: 40,
        interpreter_stack_bytes: 4096,
        uses_cmsis_nn: true,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode TinyMlDeployConfig Cortex-M4");
    let (decoded, decoded_version, _size): (TinyMlDeployConfig, Version, usize) =
        decode_versioned_value(&bytes).expect("decode TinyMlDeployConfig Cortex-M4");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_onnx_operator_set_with_custom_ops() {
    let val = OnnxOperatorSet {
        domain: OnnxOpsetDomain::DefaultAi,
        opset_version: 18,
        model_ir_version: 9,
        num_nodes: 347,
        num_initializers: 162,
        custom_ops: vec![
            "QuantizedConv".to_string(),
            "FusedBatchNormRelu".to_string(),
            "DepthToSpaceNCHW".to_string(),
        ],
        producer_name: "pytorch".to_string(),
        producer_version: "2.3.0".to_string(),
    };
    let version = Version::new(1, 4, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode OnnxOperatorSet");
    let (decoded, decoded_version, _size): (OnnxOperatorSet, Version, usize) =
        decode_versioned_value(&bytes).expect("decode OnnxOperatorSet");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_tensor_shape_spec_image_input() {
    let val = TensorShapeSpec {
        name: "input_image".to_string(),
        dimensions: vec![1, 3, 224, 224],
        element_size_bytes: 4,
        is_dynamic: false,
        total_elements: 150_528,
        memory_layout_row_major: true,
        alignment_bytes: 64,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode TensorShapeSpec image");
    let (decoded, decoded_version, _size): (TensorShapeSpec, Version, usize) =
        decode_versioned_value(&bytes).expect("decode TensorShapeSpec image");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_tensor_shape_spec_dynamic_sequence() {
    let val = TensorShapeSpec {
        name: "token_embeddings".to_string(),
        dimensions: vec![1, 512, 768],
        element_size_bytes: 2,
        is_dynamic: true,
        total_elements: 393_216,
        memory_layout_row_major: true,
        alignment_bytes: 16,
    };
    let version = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode TensorShapeSpec dynamic seq");
    let (decoded, decoded_version, _size): (TensorShapeSpec, Version, usize) =
        decode_versioned_value(&bytes).expect("decode TensorShapeSpec dynamic seq");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_batch_norm_statistics() {
    let val = BatchNormStatistics {
        layer_name: "layer3.bottleneck2.bn1".to_string(),
        num_features: 4,
        running_mean_x1e6: vec![-12_345, 67_890, 1_234, -98_765],
        running_var_x1e6: vec![1_000_000, 980_000, 1_020_000, 995_000],
        momentum_x1e6: 100_000,
        epsilon_x1e10: 100_000,
        affine: true,
        track_running_stats: true,
        num_batches_tracked: 45_000,
    };
    let version = Version::new(1, 0, 2);
    let bytes = encode_versioned_value(&val, version).expect("encode BatchNormStatistics");
    let (decoded, decoded_version, _size): (BatchNormStatistics, Version, usize) =
        decode_versioned_value(&bytes).expect("decode BatchNormStatistics");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_activation_layer_config_hard_swish_fused() {
    let val = ActivationLayerConfig {
        layer_index: 14,
        function: ActivationFunction::HardSwish,
        in_place: true,
        negative_slope_x1e6: 0,
        input_channels: 672,
        output_channels: 672,
        fused_with_conv: true,
    };
    let version = Version::new(2, 0, 0);
    let bytes =
        encode_versioned_value(&val, version).expect("encode ActivationLayerConfig HardSwish");
    let (decoded, decoded_version, _size): (ActivationLayerConfig, Version, usize) =
        decode_versioned_value(&bytes).expect("decode ActivationLayerConfig HardSwish");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_model_compression_summary_full_pipeline() {
    let val = ModelCompressionSummary {
        model_name: "deeplabv3_mobilenet_v3".to_string(),
        original_size_bytes: 23_500_000,
        compressed_size_bytes: 5_800_000,
        quantization_mode: QuantizationMode::Int8Asymmetric,
        pruning_strategy: Some(PruningStrategy::StructuredFilterwise),
        sparsity_bps: 4200,
        accuracy_retained_bps: 9720,
        latency_speedup_x100: 380,
        target_accelerator: AcceleratorType::QualcommHexagonDsp,
    };
    let version = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode ModelCompressionSummary");
    let (decoded, decoded_version, _size): (ModelCompressionSummary, Version, usize) =
        decode_versioned_value(&bytes).expect("decode ModelCompressionSummary");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_edge_inference_session_multi_io() {
    let val = EdgeInferenceSession {
        session_id: 550_001,
        model_name: "mediapipe_face_mesh".to_string(),
        accelerator: AcceleratorType::NpuGeneric,
        input_shapes: vec![vec![1, 192, 192, 3]],
        output_shapes: vec![vec![1, 468, 3], vec![1, 1]],
        num_threads: 4,
        enable_fp16: true,
        enable_int8: false,
        profiling_enabled: false,
        max_memory_mb: 256,
    };
    let version = Version::new(1, 2, 1);
    let bytes = encode_versioned_value(&val, version).expect("encode EdgeInferenceSession");
    let (decoded, decoded_version, _size): (EdgeInferenceSession, Version, usize) =
        decode_versioned_value(&bytes).expect("decode EdgeInferenceSession");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_neural_arch_search_result_pareto() {
    let val = NeuralArchSearchResult {
        search_id: 30042,
        architecture_hash: "a3f7b2c1d9e8".to_string(),
        num_layers: 53,
        total_flops: 320_000_000,
        total_params: 3_400_000,
        accuracy_bps: 9150,
        latency_us: 5_200,
        energy_uj: 18_000,
        pareto_optimal: true,
        search_iterations: 500,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode NeuralArchSearchResult");
    let (decoded, decoded_version, _size): (NeuralArchSearchResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode NeuralArchSearchResult");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_weight_sharing_config() {
    let val = WeightSharingConfig {
        group_id: 7,
        shared_layers: vec![3, 4, 5, 6, 7, 8],
        codebook_size: 256,
        bits_per_weight: 4,
        reconstruction_error_x1e6: 4_321,
        compression_ratio_x100: 800,
    };
    let version = Version::new(1, 1, 1);
    let bytes = encode_versioned_value(&val, version).expect("encode WeightSharingConfig");
    let (decoded, decoded_version, _size): (WeightSharingConfig, Version, usize) =
        decode_versioned_value(&bytes).expect("decode WeightSharingConfig");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_on_device_cache_entry() {
    let val = OnDeviceCacheEntry {
        cache_key: "conv2d_layer12_weights_int8".to_string(),
        model_version: 42,
        tensor_name: "model.features.12.conv.weight".to_string(),
        offset_bytes: 1_048_576,
        size_bytes: 262_144,
        last_accessed_epoch: 1_710_000_000,
        hit_count: 8_432,
        eviction_priority: 2,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode OnDeviceCacheEntry");
    let (decoded, decoded_version, _size): (OnDeviceCacheEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode OnDeviceCacheEntry");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_split_inference_partition() {
    let val = SplitInferencePartition {
        partition_id: 1,
        total_partitions: 3,
        start_layer: 0,
        end_layer: 22,
        device_target: AcceleratorType::AppleNeuralEngine,
        intermediate_tensor_bytes: 802_816,
        estimated_latency_us: 3_100,
        communication_overhead_us: 150,
        requires_sync: true,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&val, version).expect("encode SplitInferencePartition");
    let (decoded, decoded_version, _size): (SplitInferencePartition, Version, usize) =
        decode_versioned_value(&bytes).expect("decode SplitInferencePartition");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}
