//! Machine learning / model inference versioning tests for OxiCode (set 17).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and ML domain structs (ModelV1, ModelV2, InferenceBatch) with the
//! ModelType enum across all its variants, various version tags, field verification,
//! version comparison, consumed bytes accounting, Vec of models, version field
//! preservation, and plain encode/decode baseline.

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
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ModelType {
    LinearRegression,
    NeuralNetwork,
    RandomForest,
    GradientBoosting,
    SVM,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelV1 {
    model_id: u64,
    model_type: ModelType,
    accuracy: f32,
    features: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelV2 {
    model_id: u64,
    model_type: ModelType,
    accuracy: f32,
    features: u32,
    training_samples: u64,
    version_tag: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InferenceBatch {
    batch_id: u64,
    model_id: u64,
    inputs: Vec<f32>,
    outputs: Vec<f32>,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// ModelV1 with ModelType::LinearRegression at version 1.0.0
#[test]
fn test_model_v1_linear_regression_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = ModelV1 {
        model_id: 1001,
        model_type: ModelType::LinearRegression,
        accuracy: 0.87,
        features: 15,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.model_type, ModelType::LinearRegression);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// ModelV1 with ModelType::NeuralNetwork at version 1.0.0
#[test]
fn test_model_v1_neural_network_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = ModelV1 {
        model_id: 1002,
        model_type: ModelType::NeuralNetwork,
        accuracy: 0.95,
        features: 128,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.model_type, ModelType::NeuralNetwork);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// ModelV1 with ModelType::RandomForest at version 1.0.0
#[test]
fn test_model_v1_random_forest_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = ModelV1 {
        model_id: 1003,
        model_type: ModelType::RandomForest,
        accuracy: 0.91,
        features: 42,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.model_type, ModelType::RandomForest);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// ModelV1 with ModelType::GradientBoosting at version 1.0.0
#[test]
fn test_model_v1_gradient_boosting_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = ModelV1 {
        model_id: 1004,
        model_type: ModelType::GradientBoosting,
        accuracy: 0.93,
        features: 64,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.model_type, ModelType::GradientBoosting);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// ModelV1 with ModelType::SVM at version 1.0.0
#[test]
fn test_model_v1_svm_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = ModelV1 {
        model_id: 1005,
        model_type: ModelType::SVM,
        accuracy: 0.88,
        features: 20,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.model_type, ModelType::SVM);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// ModelV2 roundtrip at version 2.0.0 with NeuralNetwork and large training set
#[test]
fn test_model_v2_neural_network_large_training_set_v2_0_0() {
    let version = Version::new(2, 0, 0);
    let original = ModelV2 {
        model_id: 2001,
        model_type: ModelType::NeuralNetwork,
        accuracy: 0.97,
        features: 256,
        training_samples: 1_000_000,
        version_tag: String::from("v2.0.0-release"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(ver.major, 2);
    assert!((decoded.accuracy - 0.97_f32).abs() < 1e-5);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// ModelV2 roundtrip at version 2.1.0 with GradientBoosting and version_tag
#[test]
fn test_model_v2_gradient_boosting_with_tag_v2_1_0() {
    let version = Version::new(2, 1, 0);
    let original = ModelV2 {
        model_id: 2002,
        model_type: ModelType::GradientBoosting,
        accuracy: 0.94,
        features: 80,
        training_samples: 50_000,
        version_tag: String::from("xgb-prod-2.1"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver.minor, 1);
    assert!(consumed > 0);
    assert_eq!(decoded.version_tag, "xgb-prod-2.1");
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// ModelV2 roundtrip at version 3.0.0 with LinearRegression and minimal features
#[test]
fn test_model_v2_linear_regression_minimal_features_v3_0_0() {
    let version = Version::new(3, 0, 0);
    let original = ModelV2 {
        model_id: 2003,
        model_type: ModelType::LinearRegression,
        accuracy: 0.72,
        features: 3,
        training_samples: 500,
        version_tag: String::from("lr-baseline"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.features, 3);
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// InferenceBatch with non-empty inputs and outputs at version 2.0.0
#[test]
fn test_inference_batch_with_inputs_and_outputs_v2_0_0() {
    let version = Version::new(2, 0, 0);
    let original = InferenceBatch {
        batch_id: 3001,
        model_id: 2001,
        inputs: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        outputs: vec![0.85, 0.15],
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (InferenceBatch, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.inputs.len(), 5);
    assert_eq!(decoded.outputs.len(), 2);
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// InferenceBatch with empty inputs and outputs (edge case) at version 2.0.0
#[test]
fn test_inference_batch_empty_vecs_v2_0_0() {
    let version = Version::new(2, 0, 0);
    let original = InferenceBatch {
        batch_id: 3002,
        model_id: 1001,
        inputs: vec![],
        outputs: vec![],
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (InferenceBatch, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert!(decoded.inputs.is_empty());
    assert!(decoded.outputs.is_empty());
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// Version ordering: v1 < v2 < v3 representing model schema generations
#[test]
fn test_version_ordering_model_generations_v1_lt_v2_lt_v3() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);
    assert!(v1 < v2, "ModelV1 schema version must be less than ModelV2");
    assert!(v2 < v3, "ModelV2 schema version must be less than ModelV3");
    assert!(v1 < v3, "ModelV1 schema version must be less than ModelV3");
    assert_ne!(v1, v2);
    assert_ne!(v2, v3);
    assert_ne!(v1, v3);
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// Version comparison: minor ordering within the same major version
#[test]
fn test_version_minor_ordering_within_major() {
    let v2_0 = Version::new(2, 0, 0);
    let v2_1 = Version::new(2, 1, 0);
    let v2_9 = Version::new(2, 9, 0);
    assert!(v2_0 < v2_1);
    assert!(v2_1 < v2_9);
    assert!(v2_0 < v2_9);
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// Version comparison: patch ordering within the same major.minor
#[test]
fn test_version_patch_ordering_within_minor() {
    let v1_0_0 = Version::new(1, 0, 0);
    let v1_0_1 = Version::new(1, 0, 1);
    let v1_0_5 = Version::new(1, 0, 5);
    assert!(v1_0_0 < v1_0_1);
    assert!(v1_0_1 < v1_0_5);
    assert_ne!(v1_0_0, v1_0_5);
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// Version field preservation: major, minor, patch survive encode/decode roundtrip
#[test]
fn test_version_field_preservation_after_decode() {
    let version = Version::new(4, 11, 77);
    let original = ModelV1 {
        model_id: 9999,
        model_type: ModelType::SVM,
        accuracy: 0.80,
        features: 32,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (ModelV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 11);
    assert_eq!(ver.patch, 77);
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// Consumed bytes: positive and within total encoded buffer length
#[test]
fn test_consumed_bytes_within_encoded_buffer_bounds() {
    let version = Version::new(2, 0, 0);
    let original = ModelV2 {
        model_id: 4001,
        model_type: ModelType::RandomForest,
        accuracy: 0.89,
        features: 50,
        training_samples: 10_000,
        version_tag: String::from("rf-v2-stable"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let total_len = encoded.len();
    let (_decoded, _ver, consumed): (ModelV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert!(consumed > 0, "consumed bytes must be positive");
    assert!(
        consumed <= total_len,
        "consumed ({consumed}) must not exceed total encoded length ({total_len})"
    );
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// Vec of ModelV1 models, each versioned independently and decoded correctly
#[test]
fn test_vec_of_models_versioned_independently() {
    let version = Version::new(1, 0, 0);
    let catalogue = vec![
        ModelV1 {
            model_id: 5001,
            model_type: ModelType::LinearRegression,
            accuracy: 0.70,
            features: 10,
        },
        ModelV1 {
            model_id: 5002,
            model_type: ModelType::NeuralNetwork,
            accuracy: 0.96,
            features: 512,
        },
        ModelV1 {
            model_id: 5003,
            model_type: ModelType::RandomForest,
            accuracy: 0.90,
            features: 30,
        },
        ModelV1 {
            model_id: 5004,
            model_type: ModelType::GradientBoosting,
            accuracy: 0.92,
            features: 60,
        },
    ];
    for original in &catalogue {
        let encoded =
            encode_versioned_value(original, version).expect("encode_versioned_value failed");
        let (decoded, ver, consumed): (ModelV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(&decoded, original);
        assert_eq!(ver, version);
        assert!(consumed > 0);
    }
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Same ModelV2 data tagged at two different version tags: decoded data is identical
// but version fields differ
#[test]
fn test_model_v2_same_data_different_version_tags() {
    let v_old = Version::new(2, 0, 0);
    let v_new = Version::new(2, 4, 3);
    let model = ModelV2 {
        model_id: 6001,
        model_type: ModelType::NeuralNetwork,
        accuracy: 0.98,
        features: 200,
        training_samples: 500_000,
        version_tag: String::from("nn-experimental"),
    };

    let enc_old = encode_versioned_value(&model, v_old).expect("encode v_old failed");
    let enc_new = encode_versioned_value(&model, v_new).expect("encode v_new failed");

    let (decoded_old, ver_old, _): (ModelV2, Version, usize) =
        decode_versioned_value(&enc_old).expect("decode v_old failed");
    let (decoded_new, ver_new, _): (ModelV2, Version, usize) =
        decode_versioned_value(&enc_new).expect("decode v_new failed");

    assert_eq!(decoded_old, model);
    assert_eq!(decoded_new, model);
    assert_eq!(ver_old, v_old);
    assert_eq!(ver_new, v_new);
    assert_ne!(ver_old, ver_new);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// ModelV2 with long version_tag string and large training_samples count
#[test]
fn test_model_v2_long_version_tag_large_training_samples() {
    let version = Version::new(3, 1, 0);
    let original = ModelV2 {
        model_id: 7001,
        model_type: ModelType::GradientBoosting,
        accuracy: 0.995,
        features: 1024,
        training_samples: 100_000_000,
        version_tag: String::from(
            "gradient-boosting-production-model-region-us-west-2-cohort-alpha-v3.1.0",
        ),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ModelV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.training_samples, 100_000_000);
    assert!(decoded.version_tag.contains("gradient-boosting"));
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// InferenceBatch with large input and output vectors at version 3.0.0
#[test]
fn test_inference_batch_large_vectors_v3_0_0() {
    let version = Version::new(3, 0, 0);
    let inputs: Vec<f32> = (0..64).map(|i| i as f32 * 0.01).collect();
    let outputs: Vec<f32> = (0..10).map(|i| i as f32 * 0.1).collect();
    let original = InferenceBatch {
        batch_id: 7002,
        model_id: 2001,
        inputs: inputs.clone(),
        outputs: outputs.clone(),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (InferenceBatch, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.inputs.len(), 64);
    assert_eq!(decoded.outputs.len(), 10);
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// Version equality: two identical Version values compare equal
#[test]
fn test_version_equality_identical_values() {
    let va = Version::new(3, 8, 15);
    let vb = Version::new(3, 8, 15);
    assert_eq!(va, vb);
    assert!(!(va < vb));
    assert!(!(va > vb));
    assert!(va <= vb);
    assert!(va >= vb);
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// Plain encode/decode baseline for ModelV1 (no versioning wrapper)
#[test]
fn test_model_v1_plain_encode_decode_baseline() {
    let original = ModelV1 {
        model_id: 8001,
        model_type: ModelType::RandomForest,
        accuracy: 0.91,
        features: 25,
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (ModelV1, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.features, 25);
    assert_eq!(decoded.model_type, ModelType::RandomForest);
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// Plain encode/decode baseline for InferenceBatch (no versioning wrapper),
// confirming the encoding is independent of version metadata
#[test]
fn test_inference_batch_plain_encode_decode_baseline() {
    let original = InferenceBatch {
        batch_id: 8002,
        model_id: 1003,
        inputs: vec![1.0, 2.0, 3.0],
        outputs: vec![0.99],
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (InferenceBatch, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.inputs, vec![1.0_f32, 2.0_f32, 3.0_f32]);
    assert_eq!(decoded.outputs, vec![0.99_f32]);
    assert_eq!(decoded.batch_id, 8002);
}
