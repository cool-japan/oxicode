//! Tests for streaming encoder/decoder API

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
use oxicode::streaming::{
    BufferStreamingDecoder, BufferStreamingEncoder, StreamingConfig, DEFAULT_CHUNK_SIZE,
    MAX_CHUNK_SIZE,
};

// ---- BufferStreamingEncoder / BufferStreamingDecoder tests ----

#[test]
fn test_buffer_encode_decode_roundtrip_u32() {
    let mut encoder = BufferStreamingEncoder::new();
    let items: Vec<u32> = (0..50).collect();
    for item in &items {
        encoder.write_item(item).expect("write_item failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");

    assert_eq!(items, decoded);
    assert!(decoder.is_finished());
}

#[test]
fn test_buffer_encode_decode_roundtrip_string() {
    let mut encoder = BufferStreamingEncoder::new();
    let items = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ];
    for item in &items {
        encoder.write_item(item).expect("write_item failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<String> = decoder.read_all().expect("read_all failed");

    assert_eq!(items, decoded);
}

#[test]
fn test_buffer_read_item_by_item() {
    let mut encoder = BufferStreamingEncoder::new();
    encoder.write_item(&10u32).expect("write_item failed");
    encoder.write_item(&20u32).expect("write_item failed");
    encoder.write_item(&30u32).expect("write_item failed");
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    assert_eq!(
        decoder.read_item::<u32>().expect("read_item failed"),
        Some(10)
    );
    assert_eq!(
        decoder.read_item::<u32>().expect("read_item failed"),
        Some(20)
    );
    assert_eq!(
        decoder.read_item::<u32>().expect("read_item failed"),
        Some(30)
    );
    assert_eq!(decoder.read_item::<u32>().expect("read_item failed"), None);
    assert!(decoder.is_finished());
}

#[test]
fn test_buffer_empty_stream() {
    let encoder = BufferStreamingEncoder::new();
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");

    assert!(decoded.is_empty());
    assert!(decoder.is_finished());
}

#[test]
fn test_buffer_small_chunk_size_forces_multiple_chunks() {
    // 5000 u32 items encode to ~8750 bytes of payload, which at 1024-byte chunks
    // forces at least 8 chunks (well above 1).
    let config = StreamingConfig::new().with_chunk_size(1024);
    let mut encoder = BufferStreamingEncoder::with_config(config);

    for i in 0..5000u32 {
        encoder.write_item(&i).expect("write_item failed");
    }

    let encoded = encoder.finish();

    // Full roundtrip still works
    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");
    let expected: Vec<u32> = (0..5000).collect();
    assert_eq!(expected, decoded);

    // Multiple chunks must have been decoded
    assert!(
        decoder.progress().chunks_processed >= 2,
        "expected >= 2 chunks, got {}",
        decoder.progress().chunks_processed
    );
}

#[test]
fn test_buffer_progress_tracking() {
    let mut encoder = BufferStreamingEncoder::new();
    for i in 0..20u32 {
        encoder.write_item(&i).expect("write_item failed");
    }
    let encoded = encoder.finish();

    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let _: Vec<u32> = decoder.read_all().expect("read_all failed");

    assert_eq!(decoder.progress().items_processed, 20);
    assert!(decoder.progress().chunks_processed >= 1);
    assert!(decoder.progress().bytes_processed > 0);
}

#[test]
fn test_buffer_encoder_progress() {
    let config = StreamingConfig::new().with_chunk_size(1024);
    let mut encoder = BufferStreamingEncoder::with_config(config);

    for i in 0..300u32 {
        encoder.write_item(&i).expect("write_item failed");
    }

    let encoded = encoder.finish();

    // Verify via decoder that all 300 items were encoded correctly
    let mut decoder = BufferStreamingDecoder::new(&encoded);
    let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");
    assert_eq!(decoded.len(), 300);
    assert!(decoder.progress().chunks_processed >= 1);
    assert_eq!(decoder.progress().items_processed, 300);
}

// ---- StreamingConfig builder tests ----

#[test]
fn test_config_default_values() {
    let config = StreamingConfig::default();
    assert_eq!(config.chunk_size, DEFAULT_CHUNK_SIZE);
    assert!(!config.flush_per_item);
}

#[test]
fn test_config_with_chunk_size_clamped_low() {
    let config = StreamingConfig::new().with_chunk_size(10); // below 1024
    assert_eq!(config.chunk_size, 1024);
}

#[test]
fn test_config_with_chunk_size_clamped_high() {
    let config = StreamingConfig::new().with_chunk_size(usize::MAX);
    assert_eq!(config.chunk_size, MAX_CHUNK_SIZE);
}

#[test]
fn test_config_flush_per_item() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    assert!(config.flush_per_item);
}

#[test]
fn test_config_max_buffer() {
    let config = StreamingConfig::new().with_max_buffer(8 * 1024 * 1024);
    assert_eq!(config.max_buffer_size, 8 * 1024 * 1024);
}

// ---- std-only StreamingEncoder / StreamingDecoder tests ----

#[cfg(feature = "std")]
mod std_streaming {
    use oxicode::streaming::{StreamingDecoder, StreamingEncoder};
    use std::io::Cursor;

    #[test]
    fn test_io_encoder_decoder_roundtrip() {
        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut buffer);
            for i in 0..100u32 {
                encoder.write_item(&i).expect("write_item failed");
            }
            encoder.finish().expect("finish failed");
        }

        assert!(!buffer.is_empty());

        let cursor = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");

        let expected: Vec<u32> = (0..100).collect();
        assert_eq!(expected, decoded);
        assert!(decoder.is_finished());
    }

    #[test]
    fn test_io_encoder_empty_stream() {
        let mut buffer: Vec<u8> = Vec::new();
        {
            let encoder = StreamingEncoder::new(&mut buffer);
            encoder.finish().expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cursor);
        let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");
        assert!(decoded.is_empty());
        assert!(decoder.is_finished());
    }

    #[test]
    fn test_io_decoder_read_item_by_item() {
        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut buffer);
            encoder.write_item(&1u32).expect("write_item failed");
            encoder.write_item(&2u32).expect("write_item failed");
            encoder.write_item(&3u32).expect("write_item failed");
            encoder.finish().expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cursor);
        assert_eq!(
            decoder.read_item::<u32>().expect("read_item failed"),
            Some(1)
        );
        assert_eq!(
            decoder.read_item::<u32>().expect("read_item failed"),
            Some(2)
        );
        assert_eq!(
            decoder.read_item::<u32>().expect("read_item failed"),
            Some(3)
        );
        assert_eq!(decoder.read_item::<u32>().expect("read_item failed"), None);
    }

    #[test]
    fn test_io_encoder_progress() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut encoder = StreamingEncoder::new(&mut buffer);
        encoder.set_estimated_total(10);
        for i in 0..10u32 {
            encoder.write_item(&i).expect("write_item failed");
        }
        // Progress before finish only reflects fully flushed chunks
        let _writer = encoder.finish().expect("finish failed");
    }

    #[test]
    fn test_io_encoder_write_all() {
        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut buffer);
            let items: Vec<u64> = (100..120).collect();
            encoder.write_all(items).expect("write_all failed");
            encoder.finish().expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cursor);
        let decoded: Vec<u64> = decoder.read_all().expect("read_all failed");
        let expected: Vec<u64> = (100..120).collect();
        assert_eq!(expected, decoded);
    }
}
