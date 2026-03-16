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
use oxicode::config;
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PrecisionFormat {
    Fp32,
    Fp16,
    Bf16,
    Int8,
    Int4,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ActivationFn {
    Relu,
    Gelu,
    Silu,
    Sigmoid,
    Tanh,
    HardSwish,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeploymentTarget {
    Cpu,
    Gpu,
    Npu,
    Dsp,
    Fpga,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OptimizationPass {
    ConstFolding,
    DeadCodeElim,
    OpFusion,
    LayoutOpt,
    KernelAutoTune,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelWeights {
    layer_id: u32,
    weight_count: u64,
    precision: PrecisionFormat,
    checksum: u32,
    compressed_size_bytes: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InferenceParams {
    batch_size: u32,
    max_sequence_len: u32,
    temperature_x1000: u32,
    top_k: u32,
    top_p_x1000: u32,
    beam_width: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuantizationConfig {
    precision: PrecisionFormat,
    symmetric: bool,
    per_channel: bool,
    calibration_samples: u32,
    dynamic: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LayerArchitecture {
    layer_id: u32,
    layer_name: String,
    input_channels: u32,
    output_channels: u32,
    kernel_size: u32,
    stride: u32,
    padding: u32,
    activation: ActivationFn,
    use_bias: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatchNormStats {
    layer_id: u32,
    running_mean_count: u32,
    running_var_count: u32,
    epsilon_x1e9: u64,
    momentum_x1e6: u32,
    affine: bool,
    track_running_stats: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeatureExtractor {
    extractor_id: u32,
    name: String,
    input_dim: u32,
    output_dim: u32,
    num_layers: u32,
    precision: PrecisionFormat,
    target: DeploymentTarget,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelMetadata {
    model_id: u64,
    name: String,
    version_tag: String,
    architecture: String,
    param_count: u64,
    created_epoch_secs: u64,
    framework: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeploymentSpec {
    spec_id: u64,
    model_id: u64,
    target: DeploymentTarget,
    precision: PrecisionFormat,
    max_latency_ms: u32,
    min_throughput_qps: u32,
    memory_budget_mb: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OptimizationPlan {
    model_id: u64,
    passes: Vec<OptimizationPass>,
    target: DeploymentTarget,
    expected_speedup_x100: u32,
    preserve_accuracy_pct_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelRegistryEntry {
    entry_id: u64,
    model_id: u64,
    name: String,
    precision: PrecisionFormat,
    target: DeploymentTarget,
    size_bytes: u64,
    active: bool,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_model_weights_roundtrip() {
    let weights = ModelWeights {
        layer_id: 7,
        weight_count: 4_194_304,
        precision: PrecisionFormat::Fp16,
        checksum: 0xDEAD_BEEF,
        compressed_size_bytes: 8_388_608,
    };
    let encoded = encode_to_vec(&weights).expect("encode ModelWeights failed");
    let (decoded, consumed): (ModelWeights, _) =
        decode_from_slice(&encoded).expect("decode ModelWeights failed");
    assert_eq!(weights, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_model_weights_versioned() {
    let weights = ModelWeights {
        layer_id: 0,
        weight_count: 786_432,
        precision: PrecisionFormat::Int8,
        checksum: 0xCAFE_BABE,
        compressed_size_bytes: 786_432,
    };
    let ver = Version::new(3, 1, 0);
    let encoded =
        encode_versioned_value(&weights, ver).expect("encode_versioned_value ModelWeights failed");
    let (decoded, decoded_ver, _): (ModelWeights, _, _) =
        decode_versioned_value::<ModelWeights>(&encoded)
            .expect("decode_versioned_value ModelWeights failed");
    assert_eq!(weights, decoded);
    assert_eq!(decoded_ver.major, 3);
    assert_eq!(decoded_ver.minor, 1);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_inference_params_roundtrip() {
    let params = InferenceParams {
        batch_size: 8,
        max_sequence_len: 512,
        temperature_x1000: 700,
        top_k: 50,
        top_p_x1000: 900,
        beam_width: 4,
    };
    let encoded = encode_to_vec(&params).expect("encode InferenceParams failed");
    let (decoded, consumed): (InferenceParams, _) =
        decode_from_slice(&encoded).expect("decode InferenceParams failed");
    assert_eq!(params, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_inference_params_versioned() {
    let params = InferenceParams {
        batch_size: 1,
        max_sequence_len: 2048,
        temperature_x1000: 1000,
        top_k: 0,
        top_p_x1000: 950,
        beam_width: 1,
    };
    let ver = Version::new(1, 4, 2);
    let encoded = encode_versioned_value(&params, ver)
        .expect("encode_versioned_value InferenceParams failed");
    let (decoded, decoded_ver, _): (InferenceParams, _, _) =
        decode_versioned_value::<InferenceParams>(&encoded)
            .expect("decode_versioned_value InferenceParams failed");
    assert_eq!(params, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 4);
    assert_eq!(decoded_ver.patch, 2);
}

#[test]
fn test_quantization_config_roundtrip() {
    let qcfg = QuantizationConfig {
        precision: PrecisionFormat::Int8,
        symmetric: true,
        per_channel: false,
        calibration_samples: 512,
        dynamic: false,
    };
    let encoded = encode_to_vec(&qcfg).expect("encode QuantizationConfig failed");
    let (decoded, _): (QuantizationConfig, _) =
        decode_from_slice(&encoded).expect("decode QuantizationConfig failed");
    assert_eq!(qcfg, decoded);
}

#[test]
fn test_quantization_config_with_fixed_int_encoding() {
    let qcfg = QuantizationConfig {
        precision: PrecisionFormat::Int4,
        symmetric: false,
        per_channel: true,
        calibration_samples: 1024,
        dynamic: true,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = oxicode::encode_to_vec_with_config(&qcfg, cfg)
        .expect("encode QuantizationConfig fixed int failed");
    let (decoded, _): (QuantizationConfig, _) =
        oxicode::decode_from_slice_with_config(&encoded, cfg)
            .expect("decode QuantizationConfig fixed int failed");
    assert_eq!(qcfg, decoded);
}

#[test]
fn test_layer_architecture_roundtrip() {
    let layer = LayerArchitecture {
        layer_id: 12,
        layer_name: String::from("conv2d_depthwise"),
        input_channels: 128,
        output_channels: 128,
        kernel_size: 3,
        stride: 1,
        padding: 1,
        activation: ActivationFn::Relu,
        use_bias: false,
    };
    let encoded = encode_to_vec(&layer).expect("encode LayerArchitecture failed");
    let (decoded, consumed): (LayerArchitecture, _) =
        decode_from_slice(&encoded).expect("decode LayerArchitecture failed");
    assert_eq!(layer, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_layer_architecture_versioned_minor_bump() {
    let layer = LayerArchitecture {
        layer_id: 3,
        layer_name: String::from("pointwise_expand"),
        input_channels: 64,
        output_channels: 256,
        kernel_size: 1,
        stride: 1,
        padding: 0,
        activation: ActivationFn::Gelu,
        use_bias: true,
    };
    let ver_old = Version::new(2, 0, 0);
    let ver_new = Version::new(2, 1, 0);
    let encoded_old =
        encode_versioned_value(&layer, ver_old).expect("encode LayerArchitecture v2.0 failed");
    let encoded_new =
        encode_versioned_value(&layer, ver_new).expect("encode LayerArchitecture v2.1 failed");
    let (decoded_old, dv_old, _): (LayerArchitecture, Version, usize) =
        decode_versioned_value::<LayerArchitecture>(&encoded_old)
            .expect("decode LayerArchitecture v2.0 failed");
    let (decoded_new, dv_new, _): (LayerArchitecture, Version, usize) =
        decode_versioned_value::<LayerArchitecture>(&encoded_new)
            .expect("decode LayerArchitecture v2.1 failed");
    assert_eq!(layer, decoded_old);
    assert_eq!(layer, decoded_new);
    assert!(dv_old < dv_new);
    assert!(dv_new.is_minor_update_from(&dv_old));
}

#[test]
fn test_batch_norm_stats_roundtrip() {
    let bn = BatchNormStats {
        layer_id: 5,
        running_mean_count: 256,
        running_var_count: 256,
        epsilon_x1e9: 1_000,
        momentum_x1e6: 100_000,
        affine: true,
        track_running_stats: true,
    };
    let encoded = encode_to_vec(&bn).expect("encode BatchNormStats failed");
    let (decoded, consumed): (BatchNormStats, _) =
        decode_from_slice(&encoded).expect("decode BatchNormStats failed");
    assert_eq!(bn, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_batch_norm_stats_versioned_patch() {
    let bn = BatchNormStats {
        layer_id: 99,
        running_mean_count: 512,
        running_var_count: 512,
        epsilon_x1e9: 500,
        momentum_x1e6: 10_000,
        affine: false,
        track_running_stats: false,
    };
    let ver = Version::new(1, 0, 3);
    let encoded =
        encode_versioned_value(&bn, ver).expect("encode_versioned_value BatchNormStats failed");
    let (decoded, decoded_ver, _): (BatchNormStats, _, _) =
        decode_versioned_value::<BatchNormStats>(&encoded)
            .expect("decode_versioned_value BatchNormStats failed");
    assert_eq!(bn, decoded);
    assert_eq!(decoded_ver.patch, 3);
    assert!(decoded_ver.is_patch_update_from(&Version::new(1, 0, 2)));
}

#[test]
fn test_feature_extractor_roundtrip() {
    let fx = FeatureExtractor {
        extractor_id: 42,
        name: String::from("mobilenet_v3_features"),
        input_dim: 224,
        output_dim: 960,
        num_layers: 18,
        precision: PrecisionFormat::Fp32,
        target: DeploymentTarget::Npu,
    };
    let encoded = encode_to_vec(&fx).expect("encode FeatureExtractor failed");
    let (decoded, _): (FeatureExtractor, _) =
        decode_from_slice(&encoded).expect("decode FeatureExtractor failed");
    assert_eq!(fx, decoded);
}

#[test]
fn test_feature_extractor_versioned_with_big_endian() {
    let fx = FeatureExtractor {
        extractor_id: 7,
        name: String::from("efficientnet_b4"),
        input_dim: 380,
        output_dim: 1792,
        num_layers: 32,
        precision: PrecisionFormat::Bf16,
        target: DeploymentTarget::Gpu,
    };
    let ver = Version::new(4, 2, 1);
    let encoded =
        encode_versioned_value(&fx, ver).expect("encode_versioned_value FeatureExtractor failed");
    let (decoded, decoded_ver, _): (FeatureExtractor, _, _) =
        decode_versioned_value::<FeatureExtractor>(&encoded)
            .expect("decode_versioned_value FeatureExtractor failed");
    assert_eq!(fx, decoded);
    assert_eq!(decoded_ver.major, 4);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 1);
}

#[test]
fn test_model_metadata_roundtrip() {
    let meta = ModelMetadata {
        model_id: 100_001,
        name: String::from("llama-3-8b"),
        version_tag: String::from("v3.0.0"),
        architecture: String::from("transformer-decoder"),
        param_count: 8_000_000_000,
        created_epoch_secs: 1_700_000_000,
        framework: String::from("pytorch"),
    };
    let encoded = encode_to_vec(&meta).expect("encode ModelMetadata failed");
    let (decoded, consumed): (ModelMetadata, _) =
        decode_from_slice(&encoded).expect("decode ModelMetadata failed");
    assert_eq!(meta, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_model_metadata_versioned_zero_patch() {
    let meta = ModelMetadata {
        model_id: 200_002,
        name: String::from("phi-3-mini"),
        version_tag: String::from("v1.0.0"),
        architecture: String::from("transformer"),
        param_count: 3_800_000_000,
        created_epoch_secs: 1_710_000_000,
        framework: String::from("onnx"),
    };
    let ver = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&meta, ver).expect("encode_versioned_value ModelMetadata failed");
    let (decoded, decoded_ver, _): (ModelMetadata, _, _) =
        decode_versioned_value::<ModelMetadata>(&encoded)
            .expect("decode_versioned_value ModelMetadata failed");
    assert_eq!(meta, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert!(!decoded_ver.is_breaking_change_from(&Version::new(1, 0, 0)));
}

#[test]
fn test_deployment_spec_roundtrip() {
    let spec = DeploymentSpec {
        spec_id: 9_001,
        model_id: 100_001,
        target: DeploymentTarget::Npu,
        precision: PrecisionFormat::Int8,
        max_latency_ms: 50,
        min_throughput_qps: 200,
        memory_budget_mb: 256,
    };
    let encoded = encode_to_vec(&spec).expect("encode DeploymentSpec failed");
    let (decoded, _): (DeploymentSpec, _) =
        decode_from_slice(&encoded).expect("decode DeploymentSpec failed");
    assert_eq!(spec, decoded);
}

#[test]
fn test_deployment_spec_versioned_cpu_target() {
    let spec = DeploymentSpec {
        spec_id: 1,
        model_id: 5,
        target: DeploymentTarget::Cpu,
        precision: PrecisionFormat::Fp32,
        max_latency_ms: 500,
        min_throughput_qps: 10,
        memory_budget_mb: 4096,
    };
    let ver = Version::new(2, 3, 7);
    let encoded =
        encode_versioned_value(&spec, ver).expect("encode_versioned_value DeploymentSpec failed");
    let (decoded, decoded_ver, bytes_consumed): (DeploymentSpec, _, _) =
        decode_versioned_value::<DeploymentSpec>(&encoded)
            .expect("decode_versioned_value DeploymentSpec failed");
    assert_eq!(spec, decoded);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 3);
    assert_eq!(decoded_ver.patch, 7);
    assert!(bytes_consumed > 0);
}

#[test]
fn test_optimization_plan_roundtrip() {
    let plan = OptimizationPlan {
        model_id: 77,
        passes: vec![
            OptimizationPass::ConstFolding,
            OptimizationPass::OpFusion,
            OptimizationPass::KernelAutoTune,
        ],
        target: DeploymentTarget::Gpu,
        expected_speedup_x100: 340,
        preserve_accuracy_pct_x100: 9980,
    };
    let encoded = encode_to_vec(&plan).expect("encode OptimizationPlan failed");
    let (decoded, consumed): (OptimizationPlan, _) =
        decode_from_slice(&encoded).expect("decode OptimizationPlan failed");
    assert_eq!(plan, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_optimization_plan_versioned_all_passes() {
    let plan = OptimizationPlan {
        model_id: 88,
        passes: vec![
            OptimizationPass::ConstFolding,
            OptimizationPass::DeadCodeElim,
            OptimizationPass::OpFusion,
            OptimizationPass::LayoutOpt,
            OptimizationPass::KernelAutoTune,
        ],
        target: DeploymentTarget::Npu,
        expected_speedup_x100: 500,
        preserve_accuracy_pct_x100: 9995,
    };
    let ver = Version::new(5, 0, 0);
    let encoded =
        encode_versioned_value(&plan, ver).expect("encode_versioned_value OptimizationPlan failed");
    let (decoded, decoded_ver, _): (OptimizationPlan, _, _) =
        decode_versioned_value::<OptimizationPlan>(&encoded)
            .expect("decode_versioned_value OptimizationPlan failed");
    assert_eq!(plan, decoded);
    assert_eq!(decoded_ver.major, 5);
    assert!(decoded_ver.is_breaking_change_from(&Version::new(4, 9, 9)));
}

#[test]
fn test_model_registry_entry_roundtrip() {
    let entry = ModelRegistryEntry {
        entry_id: 1_000,
        model_id: 500,
        name: String::from("resnet50_quant_npu"),
        precision: PrecisionFormat::Int8,
        target: DeploymentTarget::Npu,
        size_bytes: 25_165_824,
        active: true,
    };
    let encoded = encode_to_vec(&entry).expect("encode ModelRegistryEntry failed");
    let (decoded, _): (ModelRegistryEntry, _) =
        decode_from_slice(&encoded).expect("decode ModelRegistryEntry failed");
    assert_eq!(entry, decoded);
}

#[test]
fn test_model_registry_entry_versioned_inactive() {
    let entry = ModelRegistryEntry {
        entry_id: 9_999,
        model_id: 333,
        name: String::from("deprecated_bert_v1"),
        precision: PrecisionFormat::Fp32,
        target: DeploymentTarget::Cpu,
        size_bytes: 440_000_000,
        active: false,
    };
    let ver = Version::new(0, 9, 1);
    let encoded = encode_versioned_value(&entry, ver)
        .expect("encode_versioned_value ModelRegistryEntry failed");
    let (decoded, decoded_ver, _): (ModelRegistryEntry, Version, usize) =
        decode_versioned_value::<ModelRegistryEntry>(&encoded)
            .expect("decode_versioned_value ModelRegistryEntry failed");
    assert_eq!(entry, decoded);
    // Pre-1.0: same minor must match for compatibility
    assert!(decoded_ver.is_compatible_with(&Version::new(0, 9, 0)));
    assert!(!decoded_ver.is_compatible_with(&Version::new(0, 8, 5)));
}

#[test]
fn test_version_ordering_across_deployment_schemas() {
    let schema_v1 = Version::new(1, 0, 0);
    let schema_v2 = Version::new(1, 5, 3);
    let schema_v3 = Version::new(2, 0, 0);

    let spec = DeploymentSpec {
        spec_id: 42,
        model_id: 10,
        target: DeploymentTarget::Fpga,
        precision: PrecisionFormat::Int8,
        max_latency_ms: 20,
        min_throughput_qps: 500,
        memory_budget_mb: 128,
    };

    let enc_v1 =
        encode_versioned_value(&spec, schema_v1).expect("encode DeploymentSpec schema_v1 failed");
    let enc_v3 =
        encode_versioned_value(&spec, schema_v3).expect("encode DeploymentSpec schema_v3 failed");

    let (_, ver1, _): (DeploymentSpec, Version, usize) =
        decode_versioned_value::<DeploymentSpec>(&enc_v1).expect("decode schema_v1 failed");
    let (_, ver3, _): (DeploymentSpec, Version, usize) =
        decode_versioned_value::<DeploymentSpec>(&enc_v3).expect("decode schema_v3 failed");

    assert!(ver1 < schema_v2);
    assert!(schema_v2 < ver3);
    assert!(ver1.is_compatible_with(&schema_v2));
    assert!(!ver1.is_compatible_with(&ver3));
}

#[test]
fn test_vec_of_layer_architectures_roundtrip() {
    let layers: Vec<LayerArchitecture> = vec![
        LayerArchitecture {
            layer_id: 0,
            layer_name: String::from("stem_conv"),
            input_channels: 3,
            output_channels: 32,
            kernel_size: 3,
            stride: 2,
            padding: 1,
            activation: ActivationFn::HardSwish,
            use_bias: false,
        },
        LayerArchitecture {
            layer_id: 1,
            layer_name: String::from("mbconv_expand"),
            input_channels: 32,
            output_channels: 16,
            kernel_size: 1,
            stride: 1,
            padding: 0,
            activation: ActivationFn::Silu,
            use_bias: true,
        },
    ];
    let encoded = encode_to_vec(&layers).expect("encode Vec<LayerArchitecture> failed");
    let (decoded, consumed): (Vec<LayerArchitecture>, _) =
        decode_from_slice(&encoded).expect("decode Vec<LayerArchitecture> failed");
    assert_eq!(layers, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_versioned_weight_schema_satisfies_minimum() {
    let weight = ModelWeights {
        layer_id: 15,
        weight_count: 2_097_152,
        precision: PrecisionFormat::Fp16,
        checksum: 0xABCD_1234,
        compressed_size_bytes: 4_194_304,
    };
    let ver = Version::new(3, 4, 5);
    let minimum_required = Version::new(3, 0, 0);

    let encoded =
        encode_versioned_value(&weight, ver).expect("encode_versioned_value weights failed");
    let (decoded, decoded_ver, _): (ModelWeights, Version, usize) =
        decode_versioned_value::<ModelWeights>(&encoded)
            .expect("decode_versioned_value weights failed");

    assert_eq!(weight, decoded);
    assert!(decoded_ver.satisfies(&minimum_required));
    assert_eq!(decoded_ver.tuple(), (3, 4, 5));
}
