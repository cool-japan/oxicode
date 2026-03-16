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
enum ActivationFn {
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
    GELU,
    SiLU,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LayerType {
    Dense,
    Conv2D,
    LSTM,
    Attention,
    Dropout,
    BatchNorm,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OptimizerType {
    SGD,
    Adam,
    AdamW,
    RMSProp,
    Adagrad,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TaskType {
    Classification,
    Regression,
    Generation,
    Detection,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LayerConfig {
    layer_id: u32,
    layer_type: LayerType,
    input_dim: u32,
    output_dim: u32,
    activation: ActivationFn,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModelCheckpoint {
    checkpoint_id: u64,
    model_name: String,
    epoch: u32,
    loss_x1e6: u32,
    accuracy_x1e6: u32,
    layers: Vec<LayerConfig>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainingHyperparams {
    lr_x1e8: u32,
    batch_size: u32,
    optimizer: OptimizerType,
    weight_decay_x1e8: u32,
    task_type: TaskType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InferenceResult {
    request_id: u64,
    model_name: String,
    predicted_class: u32,
    confidence_x1e6: u32,
    latency_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DatasetSplit {
    dataset_id: u32,
    train_samples: u64,
    val_samples: u64,
    test_samples: u64,
    num_classes: u32,
}

#[test]
fn test_activation_fn_relu_versioned_v1() {
    let activation = ActivationFn::ReLU;
    let version = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&activation, version).expect("encode ActivationFn::ReLU v1.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<ActivationFn>(&encoded).expect("decode ActivationFn::ReLU v1.0.0");
    assert_eq!(decoded, ActivationFn::ReLU);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_activation_fn_gelu_versioned_v2() {
    let activation = ActivationFn::GELU;
    let version = Version::new(2, 0, 0);
    let encoded =
        encode_versioned_value(&activation, version).expect("encode ActivationFn::GELU v2.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<ActivationFn>(&encoded).expect("decode ActivationFn::GELU v2.0.0");
    assert_eq!(decoded, ActivationFn::GELU);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_layer_type_attention_versioned_v1_3() {
    let layer_type = LayerType::Attention;
    let version = Version::new(1, 3, 0);
    let encoded =
        encode_versioned_value(&layer_type, version).expect("encode LayerType::Attention v1.3.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<LayerType>(&encoded).expect("decode LayerType::Attention v1.3.0");
    assert_eq!(decoded, LayerType::Attention);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_optimizer_type_adamw_versioned_v1() {
    let optimizer = OptimizerType::AdamW;
    let version = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&optimizer, version).expect("encode OptimizerType::AdamW v1.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<OptimizerType>(&encoded)
        .expect("decode OptimizerType::AdamW v1.0.0");
    assert_eq!(decoded, OptimizerType::AdamW);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_task_type_generation_versioned_v2() {
    let task = TaskType::Generation;
    let version = Version::new(2, 0, 0);
    let encoded =
        encode_versioned_value(&task, version).expect("encode TaskType::Generation v2.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<TaskType>(&encoded).expect("decode TaskType::Generation v2.0.0");
    assert_eq!(decoded, TaskType::Generation);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_layer_config_dense_versioned_v1() {
    let layer = LayerConfig {
        layer_id: 0,
        layer_type: LayerType::Dense,
        input_dim: 784,
        output_dim: 256,
        activation: ActivationFn::ReLU,
    };
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&layer, version).expect("encode LayerConfig Dense v1.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<LayerConfig>(&encoded).expect("decode LayerConfig Dense v1.0.0");
    assert_eq!(decoded, layer);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_layer_config_conv2d_versioned_v1_3() {
    let layer = LayerConfig {
        layer_id: 1,
        layer_type: LayerType::Conv2D,
        input_dim: 3,
        output_dim: 64,
        activation: ActivationFn::GELU,
    };
    let version = Version::new(1, 3, 0);
    let encoded =
        encode_versioned_value(&layer, version).expect("encode LayerConfig Conv2D v1.3.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<LayerConfig>(&encoded).expect("decode LayerConfig Conv2D v1.3.0");
    assert_eq!(decoded, layer);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_layer_config_lstm_versioned_v2() {
    let layer = LayerConfig {
        layer_id: 2,
        layer_type: LayerType::LSTM,
        input_dim: 128,
        output_dim: 256,
        activation: ActivationFn::Tanh,
    };
    let version = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&layer, version).expect("encode LayerConfig LSTM v2.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<LayerConfig>(&encoded).expect("decode LayerConfig LSTM v2.0.0");
    assert_eq!(decoded, layer);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_model_checkpoint_small_versioned_v1() {
    let checkpoint = ModelCheckpoint {
        checkpoint_id: 1001,
        model_name: "ResNet18".to_string(),
        epoch: 10,
        loss_x1e6: 234567,
        accuracy_x1e6: 921000,
        layers: vec![
            LayerConfig {
                layer_id: 0,
                layer_type: LayerType::Conv2D,
                input_dim: 3,
                output_dim: 64,
                activation: ActivationFn::ReLU,
            },
            LayerConfig {
                layer_id: 1,
                layer_type: LayerType::BatchNorm,
                input_dim: 64,
                output_dim: 64,
                activation: ActivationFn::ReLU,
            },
        ],
    };
    let version = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&checkpoint, version).expect("encode ModelCheckpoint v1.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<ModelCheckpoint>(&encoded).expect("decode ModelCheckpoint v1.0.0");
    assert_eq!(decoded, checkpoint);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_model_checkpoint_transformer_versioned_v2() {
    let checkpoint = ModelCheckpoint {
        checkpoint_id: 2048,
        model_name: "GPT-mini".to_string(),
        epoch: 50,
        loss_x1e6: 98765,
        accuracy_x1e6: 987654,
        layers: vec![
            LayerConfig {
                layer_id: 0,
                layer_type: LayerType::Attention,
                input_dim: 512,
                output_dim: 512,
                activation: ActivationFn::SiLU,
            },
            LayerConfig {
                layer_id: 1,
                layer_type: LayerType::Dense,
                input_dim: 512,
                output_dim: 2048,
                activation: ActivationFn::GELU,
            },
            LayerConfig {
                layer_id: 2,
                layer_type: LayerType::Dense,
                input_dim: 2048,
                output_dim: 512,
                activation: ActivationFn::GELU,
            },
        ],
    };
    let version = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&checkpoint, version)
        .expect("encode ModelCheckpoint transformer v2.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<ModelCheckpoint>(&encoded)
        .expect("decode ModelCheckpoint transformer v2.0.0");
    assert_eq!(decoded, checkpoint);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_training_hyperparams_adam_versioned_v1() {
    let hyperparams = TrainingHyperparams {
        lr_x1e8: 100000,
        batch_size: 32,
        optimizer: OptimizerType::Adam,
        weight_decay_x1e8: 10000,
        task_type: TaskType::Classification,
    };
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&hyperparams, version)
        .expect("encode TrainingHyperparams Adam v1.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<TrainingHyperparams>(&encoded)
        .expect("decode TrainingHyperparams Adam v1.0.0");
    assert_eq!(decoded, hyperparams);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_training_hyperparams_sgd_versioned_v1_3() {
    let hyperparams = TrainingHyperparams {
        lr_x1e8: 1000000,
        batch_size: 128,
        optimizer: OptimizerType::SGD,
        weight_decay_x1e8: 500,
        task_type: TaskType::Regression,
    };
    let version = Version::new(1, 3, 0);
    let encoded = encode_versioned_value(&hyperparams, version)
        .expect("encode TrainingHyperparams SGD v1.3.0");
    let (decoded, ver, consumed) = decode_versioned_value::<TrainingHyperparams>(&encoded)
        .expect("decode TrainingHyperparams SGD v1.3.0");
    assert_eq!(decoded, hyperparams);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_inference_result_versioned_v1() {
    let result = InferenceResult {
        request_id: 9999,
        model_name: "MobileNetV3".to_string(),
        predicted_class: 42,
        confidence_x1e6: 973210,
        latency_ms: 15,
    };
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&result, version).expect("encode InferenceResult v1.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<InferenceResult>(&encoded).expect("decode InferenceResult v1.0.0");
    assert_eq!(decoded, result);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_inference_result_versioned_v2() {
    let result = InferenceResult {
        request_id: 12345678,
        model_name: "EfficientNetB7".to_string(),
        predicted_class: 7,
        confidence_x1e6: 998001,
        latency_ms: 8,
    };
    let version = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&result, version).expect("encode InferenceResult v2.0.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<InferenceResult>(&encoded).expect("decode InferenceResult v2.0.0");
    assert_eq!(decoded, result);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_dataset_split_imagenet_versioned_v1() {
    let split = DatasetSplit {
        dataset_id: 1,
        train_samples: 1281167,
        val_samples: 50000,
        test_samples: 100000,
        num_classes: 1000,
    };
    let version = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&split, version).expect("encode DatasetSplit ImageNet v1.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<DatasetSplit>(&encoded)
        .expect("decode DatasetSplit ImageNet v1.0.0");
    assert_eq!(decoded, split);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_dataset_split_versioned_v1_3() {
    let split = DatasetSplit {
        dataset_id: 5,
        train_samples: 60000,
        val_samples: 5000,
        test_samples: 10000,
        num_classes: 10,
    };
    let version = Version::new(1, 3, 0);
    let encoded = encode_versioned_value(&split, version).expect("encode DatasetSplit v1.3.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<DatasetSplit>(&encoded).expect("decode DatasetSplit v1.3.0");
    assert_eq!(decoded, split);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_vec_layer_configs_versioned_v1() {
    let layers: Vec<LayerConfig> = vec![
        LayerConfig {
            layer_id: 0,
            layer_type: LayerType::Dense,
            input_dim: 784,
            output_dim: 512,
            activation: ActivationFn::ReLU,
        },
        LayerConfig {
            layer_id: 1,
            layer_type: LayerType::Dropout,
            input_dim: 512,
            output_dim: 512,
            activation: ActivationFn::ReLU,
        },
        LayerConfig {
            layer_id: 2,
            layer_type: LayerType::Dense,
            input_dim: 512,
            output_dim: 10,
            activation: ActivationFn::Softmax,
        },
    ];
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&layers, version).expect("encode Vec<LayerConfig> v1.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<Vec<LayerConfig>>(&encoded)
        .expect("decode Vec<LayerConfig> v1.0.0");
    assert_eq!(decoded, layers);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_vec_inference_results_versioned_v2() {
    let results: Vec<InferenceResult> = vec![
        InferenceResult {
            request_id: 1,
            model_name: "BERT-base".to_string(),
            predicted_class: 0,
            confidence_x1e6: 910000,
            latency_ms: 22,
        },
        InferenceResult {
            request_id: 2,
            model_name: "BERT-base".to_string(),
            predicted_class: 1,
            confidence_x1e6: 880000,
            latency_ms: 19,
        },
    ];
    let version = Version::new(2, 0, 0);
    let encoded =
        encode_versioned_value(&results, version).expect("encode Vec<InferenceResult> v2.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<Vec<InferenceResult>>(&encoded)
        .expect("decode Vec<InferenceResult> v2.0.0");
    assert_eq!(decoded, results);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_model_checkpoint_no_layers_versioned_v1() {
    let checkpoint = ModelCheckpoint {
        checkpoint_id: 0,
        model_name: "EmptyModel".to_string(),
        epoch: 0,
        loss_x1e6: 1000000,
        accuracy_x1e6: 0,
        layers: vec![],
    };
    let version = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&checkpoint, version).expect("encode empty ModelCheckpoint v1.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<ModelCheckpoint>(&encoded)
        .expect("decode empty ModelCheckpoint v1.0.0");
    assert_eq!(decoded, checkpoint);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_layer_config_roundtrip_plain_encode_decode() {
    let layer = LayerConfig {
        layer_id: 99,
        layer_type: LayerType::BatchNorm,
        input_dim: 256,
        output_dim: 256,
        activation: ActivationFn::Sigmoid,
    };
    let encoded = encode_to_vec(&layer).expect("encode_to_vec LayerConfig");
    let (decoded, consumed) =
        decode_from_slice::<LayerConfig>(&encoded).expect("decode_from_slice LayerConfig");
    assert_eq!(decoded, layer);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_training_hyperparams_rmsprop_versioned_v2() {
    let hyperparams = TrainingHyperparams {
        lr_x1e8: 50000,
        batch_size: 64,
        optimizer: OptimizerType::RMSProp,
        weight_decay_x1e8: 0,
        task_type: TaskType::Detection,
    };
    let version = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&hyperparams, version)
        .expect("encode TrainingHyperparams RMSProp v2.0.0");
    let (decoded, ver, consumed) = decode_versioned_value::<TrainingHyperparams>(&encoded)
        .expect("decode TrainingHyperparams RMSProp v2.0.0");
    assert_eq!(decoded, hyperparams);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_consumed_bytes_matches_encoded_length_for_checkpoint() {
    let checkpoint = ModelCheckpoint {
        checkpoint_id: 777,
        model_name: "ViT-L-16".to_string(),
        epoch: 100,
        loss_x1e6: 12000,
        accuracy_x1e6: 995000,
        layers: vec![LayerConfig {
            layer_id: 0,
            layer_type: LayerType::Attention,
            input_dim: 1024,
            output_dim: 1024,
            activation: ActivationFn::GELU,
        }],
    };
    let version = Version::new(1, 3, 0);
    let encoded =
        encode_versioned_value(&checkpoint, version).expect("encode ViT checkpoint v1.3.0");
    let (decoded, ver, consumed) =
        decode_versioned_value::<ModelCheckpoint>(&encoded).expect("decode ViT checkpoint v1.3.0");
    assert_eq!(decoded, checkpoint);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, encoded.len());
}
