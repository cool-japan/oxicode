#![cfg(feature = "serde")]
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
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum ModelType {
    Classification,
    Regression,
    Clustering,
    Generative,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum ActivationFn {
    Relu,
    Sigmoid,
    Tanh,
    Softmax,
    Linear,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct LayerConfig {
    layer_type: String,
    units: u32,
    activation: ActivationFn,
    dropout_rate: f32,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ModelMetadata {
    model_id: u64,
    name: String,
    model_type: ModelType,
    layers: Vec<LayerConfig>,
    input_shape: Vec<u32>,
    parameters: u64,
    accuracy: Option<f32>,
}

// --- Test 1: ModelType::Classification roundtrip ---
#[test]
fn test_model_type_classification_roundtrip() {
    let cfg = config::standard();
    let val = ModelType::Classification;
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelType::Classification");
    let (decoded, consumed): (ModelType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelType::Classification");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 2: ModelType::Regression roundtrip ---
#[test]
fn test_model_type_regression_roundtrip() {
    let cfg = config::standard();
    let val = ModelType::Regression;
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelType::Regression");
    let (decoded, consumed): (ModelType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelType::Regression");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 3: ModelType::Clustering roundtrip ---
#[test]
fn test_model_type_clustering_roundtrip() {
    let cfg = config::standard();
    let val = ModelType::Clustering;
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelType::Clustering");
    let (decoded, consumed): (ModelType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelType::Clustering");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 4: ModelType::Generative roundtrip ---
#[test]
fn test_model_type_generative_roundtrip() {
    let cfg = config::standard();
    let val = ModelType::Generative;
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelType::Generative");
    let (decoded, consumed): (ModelType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelType::Generative");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 5: ActivationFn::Relu roundtrip ---
#[test]
fn test_activation_fn_relu_roundtrip() {
    let cfg = config::standard();
    let val = ActivationFn::Relu;
    let bytes = encode_to_vec(&val, cfg).expect("encode ActivationFn::Relu");
    let (decoded, consumed): (ActivationFn, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ActivationFn::Relu");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 6: ActivationFn::Sigmoid roundtrip ---
#[test]
fn test_activation_fn_sigmoid_roundtrip() {
    let cfg = config::standard();
    let val = ActivationFn::Sigmoid;
    let bytes = encode_to_vec(&val, cfg).expect("encode ActivationFn::Sigmoid");
    let (decoded, consumed): (ActivationFn, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ActivationFn::Sigmoid");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 7: ActivationFn::Tanh roundtrip ---
#[test]
fn test_activation_fn_tanh_roundtrip() {
    let cfg = config::standard();
    let val = ActivationFn::Tanh;
    let bytes = encode_to_vec(&val, cfg).expect("encode ActivationFn::Tanh");
    let (decoded, consumed): (ActivationFn, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ActivationFn::Tanh");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 8: ActivationFn::Softmax roundtrip ---
#[test]
fn test_activation_fn_softmax_roundtrip() {
    let cfg = config::standard();
    let val = ActivationFn::Softmax;
    let bytes = encode_to_vec(&val, cfg).expect("encode ActivationFn::Softmax");
    let (decoded, consumed): (ActivationFn, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ActivationFn::Softmax");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 9: ActivationFn::Linear roundtrip ---
#[test]
fn test_activation_fn_linear_roundtrip() {
    let cfg = config::standard();
    let val = ActivationFn::Linear;
    let bytes = encode_to_vec(&val, cfg).expect("encode ActivationFn::Linear");
    let (decoded, consumed): (ActivationFn, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ActivationFn::Linear");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 10: LayerConfig roundtrip (dense + relu) ---
#[test]
fn test_layer_config_dense_relu_roundtrip() {
    let cfg = config::standard();
    let val = LayerConfig {
        layer_type: "Dense".to_string(),
        units: 128,
        activation: ActivationFn::Relu,
        dropout_rate: 0.2f32,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode LayerConfig dense/relu");
    let (decoded, consumed): (LayerConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode LayerConfig dense/relu");
    assert_eq!(val.layer_type, decoded.layer_type);
    assert_eq!(val.units, decoded.units);
    assert_eq!(val.activation, decoded.activation);
    assert!((val.dropout_rate - decoded.dropout_rate).abs() < 1e-6);
    assert_eq!(consumed, bytes.len());
}

// --- Test 11: LayerConfig roundtrip (conv + softmax, zero dropout) ---
#[test]
fn test_layer_config_conv_softmax_roundtrip() {
    let cfg = config::standard();
    let val = LayerConfig {
        layer_type: "Conv2D".to_string(),
        units: 64,
        activation: ActivationFn::Softmax,
        dropout_rate: 0.0f32,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode LayerConfig conv/softmax");
    let (decoded, consumed): (LayerConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode LayerConfig conv/softmax");
    assert_eq!(val.layer_type, decoded.layer_type);
    assert_eq!(val.units, decoded.units);
    assert_eq!(val.activation, decoded.activation);
    assert_eq!(val.dropout_rate.to_bits(), decoded.dropout_rate.to_bits());
    assert_eq!(consumed, bytes.len());
}

// --- Test 12: Vec<LayerConfig> roundtrip ---
#[test]
fn test_vec_layer_config_roundtrip() {
    let cfg = config::standard();
    let val = vec![
        LayerConfig {
            layer_type: "Dense".to_string(),
            units: 256,
            activation: ActivationFn::Relu,
            dropout_rate: 0.3f32,
        },
        LayerConfig {
            layer_type: "Dense".to_string(),
            units: 128,
            activation: ActivationFn::Sigmoid,
            dropout_rate: 0.1f32,
        },
        LayerConfig {
            layer_type: "Output".to_string(),
            units: 10,
            activation: ActivationFn::Softmax,
            dropout_rate: 0.0f32,
        },
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<LayerConfig>");
    let (decoded, consumed): (Vec<LayerConfig>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<LayerConfig>");
    assert_eq!(val.len(), decoded.len());
    for (orig, dec) in val.iter().zip(decoded.iter()) {
        assert_eq!(orig.layer_type, dec.layer_type);
        assert_eq!(orig.units, dec.units);
        assert_eq!(orig.activation, dec.activation);
        assert!((orig.dropout_rate - dec.dropout_rate).abs() < 1e-6);
    }
    assert_eq!(consumed, bytes.len());
}

// --- Test 13: Full ModelMetadata roundtrip (Classification, accuracy Some) ---
#[test]
fn test_model_metadata_classification_with_accuracy_roundtrip() {
    let cfg = config::standard();
    let val = ModelMetadata {
        model_id: 42,
        name: "ResNet50".to_string(),
        model_type: ModelType::Classification,
        layers: vec![
            LayerConfig {
                layer_type: "Conv2D".to_string(),
                units: 64,
                activation: ActivationFn::Relu,
                dropout_rate: 0.0f32,
            },
            LayerConfig {
                layer_type: "Dense".to_string(),
                units: 1000,
                activation: ActivationFn::Softmax,
                dropout_rate: 0.5f32,
            },
        ],
        input_shape: vec![224, 224, 3],
        parameters: 25_557_032,
        accuracy: Some(0.7612f32),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelMetadata classification");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelMetadata classification");
    assert_eq!(val.model_id, decoded.model_id);
    assert_eq!(val.name, decoded.name);
    assert_eq!(val.model_type, decoded.model_type);
    assert_eq!(val.input_shape, decoded.input_shape);
    assert_eq!(val.parameters, decoded.parameters);
    if let (Some(a), Some(b)) = (val.accuracy, decoded.accuracy) {
        assert!((a - b).abs() < 1e-6);
    } else {
        panic!("accuracy mismatch: expected Some");
    }
    assert_eq!(consumed, bytes.len());
}

// --- Test 14: ModelMetadata with accuracy None ---
#[test]
fn test_model_metadata_accuracy_none_roundtrip() {
    let cfg = config::standard();
    let val = ModelMetadata {
        model_id: 7,
        name: "UntrainedModel".to_string(),
        model_type: ModelType::Regression,
        layers: vec![LayerConfig {
            layer_type: "Dense".to_string(),
            units: 32,
            activation: ActivationFn::Linear,
            dropout_rate: 0.0f32,
        }],
        input_shape: vec![10],
        parameters: 330,
        accuracy: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelMetadata accuracy=None");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelMetadata accuracy=None");
    assert_eq!(val.model_id, decoded.model_id);
    assert_eq!(val.accuracy, decoded.accuracy);
    assert!(decoded.accuracy.is_none());
    assert_eq!(consumed, bytes.len());
}

// --- Test 15: ModelMetadata Generative type ---
#[test]
fn test_model_metadata_generative_roundtrip() {
    let cfg = config::standard();
    let val = ModelMetadata {
        model_id: 999,
        name: "GAN-v2".to_string(),
        model_type: ModelType::Generative,
        layers: vec![
            LayerConfig {
                layer_type: "Dense".to_string(),
                units: 512,
                activation: ActivationFn::Relu,
                dropout_rate: 0.2f32,
            },
            LayerConfig {
                layer_type: "Dense".to_string(),
                units: 784,
                activation: ActivationFn::Tanh,
                dropout_rate: 0.0f32,
            },
        ],
        input_shape: vec![100],
        parameters: 920_320,
        accuracy: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelMetadata Generative");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelMetadata Generative");
    assert_eq!(val.model_type, decoded.model_type);
    assert_eq!(val.name, decoded.name);
    assert_eq!(decoded.layers.len(), 2);
    assert_eq!(consumed, bytes.len());
}

// --- Test 16: ModelMetadata Clustering type ---
#[test]
fn test_model_metadata_clustering_roundtrip() {
    let cfg = config::standard();
    let val = ModelMetadata {
        model_id: 55,
        name: "KMeans-Encoder".to_string(),
        model_type: ModelType::Clustering,
        layers: vec![LayerConfig {
            layer_type: "Embedding".to_string(),
            units: 16,
            activation: ActivationFn::Linear,
            dropout_rate: 0.0f32,
        }],
        input_shape: vec![128],
        parameters: 2_048,
        accuracy: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelMetadata Clustering");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelMetadata Clustering");
    assert_eq!(val.model_type, decoded.model_type);
    assert_eq!(val.layers[0].layer_type, decoded.layers[0].layer_type);
    assert_eq!(consumed, bytes.len());
}

// --- Test 17: consumed bytes equals encoded length ---
#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let cfg = config::standard();
    let val = ModelMetadata {
        model_id: 1,
        name: "ConsumedBytesTest".to_string(),
        model_type: ModelType::Classification,
        layers: vec![LayerConfig {
            layer_type: "Dense".to_string(),
            units: 64,
            activation: ActivationFn::Relu,
            dropout_rate: 0.1f32,
        }],
        input_shape: vec![28, 28, 1],
        parameters: 50_890,
        accuracy: Some(0.9812f32),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode for consumed bytes check");
    let (_decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode for consumed bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}

// --- Test 18: config with big_endian variant ---
#[test]
fn test_model_metadata_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = ModelMetadata {
        model_id: 1234,
        name: "BigEndianModel".to_string(),
        model_type: ModelType::Regression,
        layers: vec![LayerConfig {
            layer_type: "LSTM".to_string(),
            units: 256,
            activation: ActivationFn::Tanh,
            dropout_rate: 0.25f32,
        }],
        input_shape: vec![50, 1],
        parameters: 264_192,
        accuracy: Some(0.8543f32),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelMetadata big_endian");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelMetadata big_endian");
    assert_eq!(val.model_id, decoded.model_id);
    assert_eq!(val.name, decoded.name);
    assert_eq!(val.model_type, decoded.model_type);
    assert_eq!(consumed, bytes.len());
}

// --- Test 19: config with fixed_int_encoding ---
#[test]
fn test_model_metadata_fixed_int_encoding_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = ModelMetadata {
        model_id: 9_000_000_000_u64,
        name: "FixedIntModel".to_string(),
        model_type: ModelType::Classification,
        layers: vec![
            LayerConfig {
                layer_type: "Conv1D".to_string(),
                units: 32,
                activation: ActivationFn::Relu,
                dropout_rate: 0.0f32,
            },
            LayerConfig {
                layer_type: "GlobalAvgPool".to_string(),
                units: 1,
                activation: ActivationFn::Linear,
                dropout_rate: 0.0f32,
            },
        ],
        input_shape: vec![1000, 1],
        parameters: 1_056,
        accuracy: Some(0.9201f32),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelMetadata fixed_int");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelMetadata fixed_int");
    assert_eq!(val.model_id, decoded.model_id);
    assert_eq!(val.parameters, decoded.parameters);
    assert_eq!(val.input_shape, decoded.input_shape);
    assert_eq!(consumed, bytes.len());
}

// --- Test 20: large model with many layers ---
#[test]
fn test_large_model_many_layers_roundtrip() {
    let cfg = config::standard();
    let layers: Vec<LayerConfig> = (0..50)
        .map(|i| {
            let activation = match i % 5 {
                0 => ActivationFn::Relu,
                1 => ActivationFn::Sigmoid,
                2 => ActivationFn::Tanh,
                3 => ActivationFn::Softmax,
                _ => ActivationFn::Linear,
            };
            LayerConfig {
                layer_type: format!("Dense_{}", i),
                units: 64 + i as u32 * 4,
                activation,
                dropout_rate: (i as f32) * 0.01f32,
            }
        })
        .collect();
    let val = ModelMetadata {
        model_id: 100_000,
        name: "LargeDeepNetwork".to_string(),
        model_type: ModelType::Classification,
        layers,
        input_shape: vec![512],
        parameters: 1_200_000,
        accuracy: Some(0.9501f32),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode large model");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large model");
    assert_eq!(val.model_id, decoded.model_id);
    assert_eq!(decoded.layers.len(), 50);
    for (i, (orig, dec)) in val.layers.iter().zip(decoded.layers.iter()).enumerate() {
        assert_eq!(orig.layer_type, dec.layer_type, "layer {} type mismatch", i);
        assert_eq!(orig.units, dec.units, "layer {} units mismatch", i);
        assert_eq!(
            orig.activation, dec.activation,
            "layer {} activation mismatch",
            i
        );
        assert!(
            (orig.dropout_rate - dec.dropout_rate).abs() < 1e-6,
            "layer {} dropout mismatch",
            i
        );
    }
    assert_eq!(consumed, bytes.len());
}

// --- Test 21: empty layers vec in ModelMetadata ---
#[test]
fn test_model_metadata_empty_layers_roundtrip() {
    let cfg = config::standard();
    let val = ModelMetadata {
        model_id: 0,
        name: "EmptyModel".to_string(),
        model_type: ModelType::Regression,
        layers: vec![],
        input_shape: vec![],
        parameters: 0,
        accuracy: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ModelMetadata empty layers");
    let (decoded, consumed): (ModelMetadata, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ModelMetadata empty layers");
    assert_eq!(val.model_id, decoded.model_id);
    assert_eq!(val.name, decoded.name);
    assert!(decoded.layers.is_empty());
    assert!(decoded.input_shape.is_empty());
    assert_eq!(decoded.parameters, 0);
    assert_eq!(consumed, bytes.len());
}

// --- Test 22: f32 dropout_rate bit-exact comparison ---
#[test]
fn test_layer_config_f32_bit_exact_roundtrip() {
    let cfg = config::standard();
    // Use values that are exactly representable in IEEE 754 f32
    let dropout_values: &[f32] = &[0.0f32, 0.25f32, 0.5f32, 0.125f32, 0.75f32];
    for &rate in dropout_values {
        let val = LayerConfig {
            layer_type: "Dense".to_string(),
            units: 100,
            activation: ActivationFn::Relu,
            dropout_rate: rate,
        };
        let bytes = encode_to_vec(&val, cfg).expect("encode LayerConfig f32 bit-exact");
        let (decoded, consumed): (LayerConfig, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode LayerConfig f32 bit-exact");
        assert_eq!(
            val.dropout_rate.to_bits(),
            decoded.dropout_rate.to_bits(),
            "f32 bit pattern mismatch for dropout_rate={}",
            rate
        );
        assert_eq!(consumed, bytes.len());
    }
}
