//! Async streaming tests (39th set) for OxiCode — neural network training / deep learning domain.
//!
//! All 22 tests are top-level `#[test]` functions (no module wrapper, no async fn).
//! Each test drives a `tokio::runtime::Runtime` via `block_on`.
//! Gated by the `async-tokio` feature at the file level.

#![cfg(feature = "async-tokio")]
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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};
use oxicode::streaming::StreamingConfig;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LayerType {
    Dense,
    Conv2D,
    Lstm,
    Gru,
    Attention,
    BatchNorm,
    Dropout,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Optimizer {
    Sgd,
    Adam,
    RmsProp,
    AdaGrad,
    AdamW,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainingMetrics {
    epoch: u32,
    loss_micro: u64,
    accuracy_micro: u32,
    lr_nano: u64,
    batch_size: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LayerConfig {
    layer_id: u16,
    layer_type: LayerType,
    input_dim: u32,
    output_dim: u32,
    param_count: u64,
}

// ---------------------------------------------------------------------------
// Test 1: Single TrainingMetrics duplex roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_single_training_metrics_duplex_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let original = TrainingMetrics {
            epoch: 1,
            loss_micro: 2_500_000,
            accuracy_micro: 750_000,
            lr_nano: 1_000_000,
            batch_size: 32,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write_item failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<TrainingMetrics> = decoder.read_item().await.expect("read_item failed");
        assert_eq!(
            result,
            Some(original),
            "single TrainingMetrics roundtrip mismatch"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 2: LayerType::Dense async duplex roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_layer_type_dense_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&LayerType::Dense)
            .await
            .expect("write Dense failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerType> = decoder.read_item().await.expect("read Dense failed");
        assert_eq!(result, Some(LayerType::Dense), "Dense roundtrip mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 3: LayerType::Conv2D async duplex roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_layer_type_conv2d_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&LayerType::Conv2D)
            .await
            .expect("write Conv2D failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerType> = decoder.read_item().await.expect("read Conv2D failed");
        assert_eq!(result, Some(LayerType::Conv2D), "Conv2D roundtrip mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 4: LayerType::Lstm async duplex roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_layer_type_lstm_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&LayerType::Lstm)
            .await
            .expect("write Lstm failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerType> = decoder.read_item().await.expect("read Lstm failed");
        assert_eq!(result, Some(LayerType::Lstm), "Lstm roundtrip mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 5: Optimizer::Adam async duplex roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_optimizer_adam_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&Optimizer::Adam)
            .await
            .expect("write Adam failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<Optimizer> = decoder.read_item().await.expect("read Adam failed");
        assert_eq!(result, Some(Optimizer::Adam), "Adam roundtrip mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 6: Optimizer::AdamW async duplex roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_optimizer_adamw_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&Optimizer::AdamW)
            .await
            .expect("write AdamW failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<Optimizer> = decoder.read_item().await.expect("read AdamW failed");
        assert_eq!(result, Some(Optimizer::AdamW), "AdamW roundtrip mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 7: LayerConfig roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_layer_config_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let original = LayerConfig {
            layer_id: 3,
            layer_type: LayerType::Attention,
            input_dim: 512,
            output_dim: 512,
            param_count: 1_048_576,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write LayerConfig failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerConfig> =
            decoder.read_item().await.expect("read LayerConfig failed");
        assert_eq!(result, Some(original), "LayerConfig roundtrip mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 8: Batch of 10 TrainingMetrics write_all / read_all
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_batch_10_metrics_write_all_read_all() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let metrics: Vec<TrainingMetrics> = (0..10u32)
            .map(|i| TrainingMetrics {
                epoch: i,
                loss_micro: 3_000_000 - (i as u64 * 100_000),
                accuracy_micro: 600_000 + (i * 20_000),
                lr_nano: 1_000_000,
                batch_size: 64,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_all(metrics.clone().into_iter())
            .await
            .expect("write_all failed");
        encoder.finish().await.expect("finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<TrainingMetrics> = decoder.read_all().await.expect("read_all failed");

        assert_eq!(decoded.len(), 10, "batch count mismatch");
        for (i, (expected, actual)) in metrics.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(actual, expected, "metrics[{}] mismatch", i);
        }
    });
}

// ---------------------------------------------------------------------------
// Test 9: Empty stream returns None immediately
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_empty_stream_returns_none() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.finish().await.expect("finish empty stream failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<TrainingMetrics> = decoder
            .read_item()
            .await
            .expect("read from empty stream failed");
        assert_eq!(result, None, "empty stream must return None");
        assert!(
            decoder.is_finished(),
            "decoder must be finished after empty stream"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 10: Large batch — 50 epochs
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_large_batch_50_epochs() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let metrics: Vec<TrainingMetrics> = (1..=50u32)
            .map(|epoch| TrainingMetrics {
                epoch,
                loss_micro: 5_000_000 / epoch as u64,
                accuracy_micro: 500_000 + epoch * 8_000,
                lr_nano: 1_000_000 / (1 + epoch as u64 / 10),
                batch_size: 128,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for m in &metrics {
            encoder
                .write_item(m)
                .await
                .expect("write epoch metrics failed");
        }
        encoder.finish().await.expect("finish large batch failed");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<TrainingMetrics> = decoder
            .read_all()
            .await
            .expect("read_all large batch failed");

        assert_eq!(decoded.len(), 50, "must decode exactly 50 epoch metrics");
        assert_eq!(decoded[0].epoch, 1, "first epoch must be 1");
        assert_eq!(decoded[49].epoch, 50, "last epoch must be 50");
    });
}

// ---------------------------------------------------------------------------
// Test 11: Progress tracking — items_processed and bytes_processed increase
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_progress_tracking() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let metrics: Vec<TrainingMetrics> = (0..5u32)
            .map(|i| TrainingMetrics {
                epoch: i,
                loss_micro: 1_000_000,
                accuracy_micro: 800_000,
                lr_nano: 500_000,
                batch_size: 32,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_all(metrics.clone().into_iter())
            .await
            .expect("write_all progress test failed");
        encoder.finish().await.expect("finish progress test failed");

        let mut decoder = AsyncDecoder::new(reader);
        let _: Vec<TrainingMetrics> = decoder
            .read_all()
            .await
            .expect("read_all progress test failed");

        assert_eq!(
            decoder.progress().items_processed,
            5,
            "items_processed must be 5"
        );
        assert!(
            decoder.progress().bytes_processed > 0,
            "bytes_processed must be positive"
        );
        assert!(
            decoder.progress().chunks_processed >= 1,
            "at least one chunk must have been processed"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 12: All LayerType variants in one batch
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_all_layer_types_in_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let layers = vec![
            LayerType::Dense,
            LayerType::Conv2D,
            LayerType::Lstm,
            LayerType::Gru,
            LayerType::Attention,
            LayerType::BatchNorm,
            LayerType::Dropout,
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for layer in &layers {
            encoder
                .write_item(layer)
                .await
                .expect("write layer type failed");
        }
        encoder
            .finish()
            .await
            .expect("finish all layer types failed");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<LayerType> = decoder
            .read_all()
            .await
            .expect("read_all all layer types failed");

        assert_eq!(decoded.len(), 7, "must decode 7 layer types");
        assert_eq!(decoded[0], LayerType::Dense);
        assert_eq!(decoded[4], LayerType::Attention);
        assert_eq!(decoded[6], LayerType::Dropout);
    });
}

// ---------------------------------------------------------------------------
// Test 13: All Optimizer variants in one batch
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_all_optimizers_in_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let optimizers = vec![
            Optimizer::Sgd,
            Optimizer::Adam,
            Optimizer::RmsProp,
            Optimizer::AdaGrad,
            Optimizer::AdamW,
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for opt in &optimizers {
            encoder
                .write_item(opt)
                .await
                .expect("write optimizer failed");
        }
        encoder
            .finish()
            .await
            .expect("finish all optimizers failed");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<Optimizer> = decoder
            .read_all()
            .await
            .expect("read_all all optimizers failed");

        assert_eq!(decoded.len(), 5, "must decode 5 optimizers");
        assert_eq!(decoded[1], Optimizer::Adam);
        assert_eq!(decoded[4], Optimizer::AdamW);
    });
}

// ---------------------------------------------------------------------------
// Test 14: Concurrent write/read via duplex stream (producer writes, consumer reads)
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_concurrent_write_read() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);

        // Spawn writer task
        let write_handle = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
            for epoch in 1..=20u32 {
                let m = TrainingMetrics {
                    epoch,
                    loss_micro: 2_000_000 - (epoch as u64 * 50_000),
                    accuracy_micro: 700_000 + epoch * 10_000,
                    lr_nano: 900_000,
                    batch_size: 256,
                };
                encoder
                    .write_item(&m)
                    .await
                    .expect("concurrent write failed");
            }
            encoder.finish().await.expect("concurrent finish failed");
        });

        // Spawn reader task
        let read_handle = tokio::spawn(async move {
            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<TrainingMetrics> = decoder
                .read_all()
                .await
                .expect("concurrent read_all failed");
            decoded
        });

        write_handle.await.expect("writer task panicked");
        let decoded = read_handle.await.expect("reader task panicked");

        assert_eq!(decoded.len(), 20, "concurrent: must decode 20 metrics");
        assert_eq!(decoded[0].epoch, 1);
        assert_eq!(decoded[19].epoch, 20);
    });
}

// ---------------------------------------------------------------------------
// Test 15: Max parameter count (u64::MAX) in LayerConfig
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_max_parameter_count() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let original = LayerConfig {
            layer_id: 0,
            layer_type: LayerType::Dense,
            input_dim: u32::MAX,
            output_dim: u32::MAX,
            param_count: u64::MAX,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write max param count failed");
        encoder
            .finish()
            .await
            .expect("finish max param count failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerConfig> = decoder
            .read_item()
            .await
            .expect("read max param count failed");

        let decoded = result.expect("must decode max param count LayerConfig");
        assert_eq!(
            decoded.param_count,
            u64::MAX,
            "param_count must be u64::MAX"
        );
        assert_eq!(decoded.input_dim, u32::MAX, "input_dim must be u32::MAX");
    });
}

// ---------------------------------------------------------------------------
// Test 16: Near-zero loss metric
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_near_zero_loss_metric() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let original = TrainingMetrics {
            epoch: 999,
            loss_micro: 1,
            accuracy_micro: 999_999,
            lr_nano: 100,
            batch_size: 512,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write near-zero loss failed");
        encoder
            .finish()
            .await
            .expect("finish near-zero loss failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<TrainingMetrics> = decoder
            .read_item()
            .await
            .expect("read near-zero loss failed");

        let decoded = result.expect("must decode near-zero loss metrics");
        assert_eq!(decoded.loss_micro, 1, "loss_micro must be 1 (near zero)");
        assert_eq!(decoded.epoch, 999, "epoch must be 999");
    });
}

// ---------------------------------------------------------------------------
// Test 17: 100% accuracy metric
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_100_percent_accuracy() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let original = TrainingMetrics {
            epoch: 500,
            loss_micro: 0,
            accuracy_micro: 1_000_000,
            lr_nano: 0,
            batch_size: 1,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write 100% accuracy failed");
        encoder.finish().await.expect("finish 100% accuracy failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<TrainingMetrics> = decoder
            .read_item()
            .await
            .expect("read 100% accuracy failed");

        let decoded = result.expect("must decode 100% accuracy metrics");
        assert_eq!(
            decoded.accuracy_micro, 1_000_000,
            "accuracy_micro must be 1_000_000 (100%)"
        );
        assert_eq!(decoded.loss_micro, 0, "loss_micro must be 0");
    });
}

// ---------------------------------------------------------------------------
// Test 18: Learning rate decay sequence (5 epochs)
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_learning_rate_decay_sequence() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        // Each epoch halves the learning rate (in nano units)
        let lr_schedule: Vec<u64> = vec![1_000_000, 500_000, 250_000, 125_000, 62_500];
        let metrics: Vec<TrainingMetrics> = lr_schedule
            .iter()
            .enumerate()
            .map(|(i, &lr_nano)| TrainingMetrics {
                epoch: i as u32 + 1,
                loss_micro: 2_000_000 >> i,
                accuracy_micro: 700_000 + i as u32 * 50_000,
                lr_nano,
                batch_size: 64,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_all(metrics.clone().into_iter())
            .await
            .expect("write lr decay sequence failed");
        encoder
            .finish()
            .await
            .expect("finish lr decay sequence failed");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<TrainingMetrics> = decoder
            .read_all()
            .await
            .expect("read_all lr decay sequence failed");

        assert_eq!(decoded.len(), 5, "must decode 5 lr decay epochs");
        assert_eq!(decoded[0].lr_nano, 1_000_000, "epoch 1 lr mismatch");
        assert_eq!(decoded[4].lr_nano, 62_500, "epoch 5 lr mismatch");
        // Verify strictly decreasing
        for i in 1..5 {
            assert!(
                decoded[i].lr_nano < decoded[i - 1].lr_nano,
                "lr must decrease: epoch {} >= epoch {}",
                i + 1,
                i
            );
        }
    });
}

// ---------------------------------------------------------------------------
// Test 19: Attention layer with large dimensions
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_attention_layer_large_dims() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let original = LayerConfig {
            layer_id: 7,
            layer_type: LayerType::Attention,
            input_dim: 4096,
            output_dim: 4096,
            // 4096 * 4096 * 4 matrices = 67,108,864 params
            param_count: 67_108_864,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write attention layer failed");
        encoder
            .finish()
            .await
            .expect("finish attention layer failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerConfig> = decoder
            .read_item()
            .await
            .expect("read attention layer failed");

        let decoded = result.expect("must decode Attention LayerConfig");
        assert_eq!(
            decoded.layer_type,
            LayerType::Attention,
            "layer type mismatch"
        );
        assert_eq!(decoded.input_dim, 4096, "attention input_dim mismatch");
        assert_eq!(
            decoded.param_count, 67_108_864,
            "attention param_count mismatch"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 20: LSTM for time series
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_lstm_for_time_series() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        // LSTM: input=128, hidden=256; params = 4 * (128*256 + 256*256 + 256) = 394,240
        let original = LayerConfig {
            layer_id: 1,
            layer_type: LayerType::Lstm,
            input_dim: 128,
            output_dim: 256,
            param_count: 394_240,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write LSTM layer failed");
        encoder.finish().await.expect("finish LSTM layer failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerConfig> =
            decoder.read_item().await.expect("read LSTM layer failed");

        let decoded = result.expect("must decode LSTM LayerConfig");
        assert_eq!(decoded.layer_type, LayerType::Lstm, "must be Lstm");
        assert_eq!(decoded.input_dim, 128, "LSTM input_dim mismatch");
        assert_eq!(decoded.output_dim, 256, "LSTM output_dim mismatch");
        assert_eq!(decoded.param_count, 394_240, "LSTM param_count mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 21: BatchNorm layer config
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_batch_norm_layer() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        // BatchNorm: 2 * num_features parameters (gamma + beta)
        let original = LayerConfig {
            layer_id: 5,
            layer_type: LayerType::BatchNorm,
            input_dim: 256,
            output_dim: 256,
            param_count: 512,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("write BatchNorm layer failed");
        encoder
            .finish()
            .await
            .expect("finish BatchNorm layer failed");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LayerConfig> = decoder
            .read_item()
            .await
            .expect("read BatchNorm layer failed");

        let decoded = result.expect("must decode BatchNorm LayerConfig");
        assert_eq!(
            decoded.layer_type,
            LayerType::BatchNorm,
            "must be BatchNorm"
        );
        assert_eq!(decoded.param_count, 512, "BatchNorm param_count mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 22: Sync vs async consistency — TrainingMetrics encodes identically
//          via both sync encode_to_vec/decode_from_slice and async streaming.
// ---------------------------------------------------------------------------

#[test]
fn test_nn39_sync_vs_async_consistency() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime creation failed");
    rt.block_on(async {
        let original = TrainingMetrics {
            epoch: 42,
            loss_micro: 1_234_567,
            accuracy_micro: 876_543,
            lr_nano: 999_000,
            batch_size: 256,
        };

        // Sync encode/decode
        let sync_bytes = encode_to_vec(&original).expect("sync encode_to_vec failed");
        let (sync_decoded, _): (TrainingMetrics, _) =
            decode_from_slice(&sync_bytes).expect("sync decode_from_slice failed");
        assert_eq!(sync_decoded, original, "sync roundtrip mismatch");

        // Async streaming encode/decode via duplex
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_item(&original)
            .await
            .expect("async write_item failed");
        encoder.finish().await.expect("async finish failed");

        let mut decoder = AsyncDecoder::new(reader);
        let async_result: Option<TrainingMetrics> =
            decoder.read_item().await.expect("async read_item failed");
        let async_decoded = async_result.expect("must decode from async stream");

        // Both paths must produce identical results
        assert_eq!(async_decoded, original, "async vs original mismatch");
        assert_eq!(
            sync_decoded, async_decoded,
            "sync and async decoded values must be identical"
        );
    });
}

// ---------------------------------------------------------------------------
// Re-count verification:
//  1  test_nn39_single_training_metrics_duplex_roundtrip
//  2  test_nn39_layer_type_dense_roundtrip
//  3  test_nn39_layer_type_conv2d_roundtrip
//  4  test_nn39_layer_type_lstm_roundtrip
//  5  test_nn39_optimizer_adam_roundtrip
//  6  test_nn39_optimizer_adamw_roundtrip
//  7  test_nn39_layer_config_roundtrip
//  8  test_nn39_batch_10_metrics_write_all_read_all
//  9  test_nn39_empty_stream_returns_none
// 10  test_nn39_large_batch_50_epochs
// 11  test_nn39_progress_tracking
// 12  test_nn39_all_layer_types_in_one_batch
// 13  test_nn39_all_optimizers_in_one_batch
// 14  test_nn39_concurrent_write_read
// 15  test_nn39_max_parameter_count
// 16  test_nn39_near_zero_loss_metric
// 17  test_nn39_100_percent_accuracy
// 18  test_nn39_learning_rate_decay_sequence
// 19  test_nn39_attention_layer_large_dims
// 20  test_nn39_lstm_for_time_series
// 21  test_nn39_batch_norm_layer
// 22  test_nn39_sync_vs_async_consistency
//
// Total: 22 #[test] functions — spec satisfied.
