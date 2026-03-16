//! Tests for combined feature usage

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
use oxicode::{Decode, Encode};

// ── versioning + checksum ─────────────────────────────────────────────────────

#[cfg(all(feature = "checksum", feature = "derive"))]
mod versioning_checksum {
    use super::*;
    use oxicode::versioning::Version;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DataV1 {
        name: String,
        value: u32,
    }

    #[test]
    fn test_versioned_then_checksummed() {
        let data = DataV1 {
            name: "test".into(),
            value: 42,
        };
        let version = Version::new(1, 0, 0);

        // Encode with version metadata, then wrap with checksum
        let versioned =
            oxicode::encode_versioned_value(&data, version).expect("versioned encode failed");
        let checksummed = oxicode::checksum::wrap_with_checksum(&versioned);

        // Verify checksum, then decode versioned payload
        let verified =
            oxicode::checksum::verify_checksum(&checksummed).expect("checksum verify failed");
        let (decoded, decoded_version, _): (DataV1, Version, usize) =
            oxicode::decode_versioned_value(verified).expect("versioned decode failed");

        assert_eq!(data, decoded);
        assert_eq!(version, decoded_version);
    }

    #[test]
    fn test_checksum_catches_corruption_of_versioned_data() {
        let data = DataV1 {
            name: "important".into(),
            value: 999,
        };
        let versioned =
            oxicode::encode_versioned_value(&data, Version::new(1, 0, 0)).expect("encode failed");
        let mut checksummed = oxicode::checksum::wrap_with_checksum(&versioned);

        // Corrupt a byte in the payload region (after the 16-byte checksum header)
        let mid = oxicode::checksum::HEADER_SIZE
            + checksummed[oxicode::checksum::HEADER_SIZE..].len() / 2;
        checksummed[mid] ^= 0xFF;

        // Checksum verification must fail
        assert!(
            oxicode::checksum::verify_checksum(&checksummed).is_err(),
            "verify_checksum should reject corrupted data"
        );
    }

    #[test]
    fn test_version_survives_checksum_roundtrip() {
        let version = Version::new(3, 7, 11);
        let value: Vec<u64> = (0..50).collect();

        let versioned =
            oxicode::encode_versioned_value(&value, version).expect("versioned encode failed");
        let checksummed = oxicode::checksum::wrap_with_checksum(&versioned);
        let verified =
            oxicode::checksum::verify_checksum(&checksummed).expect("checksum verify failed");
        let (decoded, decoded_version, _): (Vec<u64>, Version, usize) =
            oxicode::decode_versioned_value(verified).expect("versioned decode failed");

        assert_eq!(value, decoded);
        assert_eq!(version, decoded_version);
    }
}

// ── compression + checksum ────────────────────────────────────────────────────

#[cfg(all(feature = "compression-lz4", feature = "checksum"))]
mod compression_checksum {
    use oxicode::compression::{compress, decompress, Compression};

    #[test]
    fn test_compress_then_checksum() {
        let data: Vec<u32> = (0..1000).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");

        // Compress, then wrap with checksum
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let wrapped = oxicode::checksum::wrap_with_checksum(&compressed);

        // Verify checksum, decompress, decode
        let verified = oxicode::checksum::verify_checksum(&wrapped).expect("verify failed");
        let decompressed = decompress(verified).expect("decompress failed");
        let (decoded, _): (Vec<u32>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_checksum_catches_compressed_corruption() {
        let data: Vec<u8> = vec![0xAB; 200];
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let mut wrapped = oxicode::checksum::wrap_with_checksum(&compressed);

        // Corrupt a byte in the compressed payload section
        let corrupt_idx = oxicode::checksum::HEADER_SIZE + 1;
        wrapped[corrupt_idx] ^= 0xFF;

        assert!(
            oxicode::checksum::verify_checksum(&wrapped).is_err(),
            "corrupted compressed+checksummed data must fail verification"
        );
    }

    #[test]
    fn test_compress_checksum_empty_vec() {
        let data: Vec<u8> = vec![];
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let wrapped = oxicode::checksum::wrap_with_checksum(&compressed);
        let verified = oxicode::checksum::verify_checksum(&wrapped).expect("verify failed");
        let decompressed = decompress(verified).expect("decompress failed");
        let (decoded, _): (Vec<u8>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(data, decoded);
    }
}

// ── streaming-style sequential decode + derive ────────────────────────────────

#[cfg(all(feature = "std", feature = "derive"))]
mod streaming_derive {
    use super::*;

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u8")]
    enum StreamEvent {
        Start { id: u64 },
        Data { bytes: Vec<u8> },
        End,
    }

    #[test]
    fn test_stream_events_roundtrip() {
        let events = vec![
            StreamEvent::Start { id: 1 },
            StreamEvent::Data {
                bytes: vec![1, 2, 3, 4, 5],
            },
            StreamEvent::Data {
                bytes: vec![6, 7, 8],
            },
            StreamEvent::End,
        ];

        // Encode all events into a single contiguous buffer
        let mut buf = Vec::new();
        for event in &events {
            let enc = oxicode::encode_to_vec(event).expect("encode failed");
            buf.extend_from_slice(&enc);
        }

        // Sequentially decode each event back from the buffer
        let mut decoded_events: Vec<StreamEvent> = Vec::new();
        let mut offset = 0;
        while offset < buf.len() {
            let (event, consumed): (StreamEvent, usize) =
                oxicode::decode_from_slice(&buf[offset..]).expect("decode failed");
            decoded_events.push(event);
            offset += consumed;
        }

        assert_eq!(events, decoded_events);
    }

    #[test]
    fn test_stream_single_event_roundtrip() {
        let event = StreamEvent::Data {
            bytes: (0u8..=127).collect(),
        };
        let encoded = oxicode::encode_to_vec(&event).expect("encode failed");
        let (decoded, consumed): (StreamEvent, usize) =
            oxicode::decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(event, decoded);
        assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_mixed_derive_types_sequential() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Header {
            version: u16,
            flags: u8,
        }

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Payload {
            data: Vec<u32>,
        }

        let header = Header {
            version: 2,
            flags: 0b101,
        };
        let payload = Payload {
            data: vec![10, 20, 30],
        };

        let mut buf = Vec::new();
        buf.extend_from_slice(&oxicode::encode_to_vec(&header).expect("encode header"));
        buf.extend_from_slice(&oxicode::encode_to_vec(&payload).expect("encode payload"));

        let (decoded_header, h_consumed): (Header, usize) =
            oxicode::decode_from_slice(&buf).expect("decode header");
        let (decoded_payload, _): (Payload, usize) =
            oxicode::decode_from_slice(&buf[h_consumed..]).expect("decode payload");

        assert_eq!(header, decoded_header);
        assert_eq!(payload, decoded_payload);
    }
}

// ── serde + checksum ──────────────────────────────────────────────────────────

#[cfg(all(feature = "serde", feature = "checksum"))]
mod serde_checksum {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SerdeData {
        key: String,
        value: Vec<u8>,
    }

    #[test]
    fn test_serde_encoded_with_checksum() {
        let data = SerdeData {
            key: "test".into(),
            value: vec![1, 2, 3],
        };

        let encoded = oxicode::serde::encode_serde(&data).expect("serde encode failed");
        let wrapped = oxicode::checksum::wrap_with_checksum(&encoded);
        let verified =
            oxicode::checksum::verify_checksum(&wrapped).expect("checksum verify failed");
        let decoded: SerdeData =
            oxicode::serde::decode_serde(verified).expect("serde decode failed");

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_serde_checksum_detects_corruption() {
        let data = SerdeData {
            key: "integrity".into(),
            value: (0u8..32).collect(),
        };
        let encoded = oxicode::serde::encode_serde(&data).expect("serde encode failed");
        let mut wrapped = oxicode::checksum::wrap_with_checksum(&encoded);

        // Corrupt a byte after the checksum header
        let corrupt_idx = oxicode::checksum::HEADER_SIZE;
        wrapped[corrupt_idx] ^= 0xFF;

        assert!(
            oxicode::checksum::verify_checksum(&wrapped).is_err(),
            "corrupted serde+checksummed data must fail verification"
        );
    }

    #[test]
    fn test_serde_checksum_empty_string_field() {
        let data = SerdeData {
            key: String::new(),
            value: vec![],
        };
        let encoded = oxicode::serde::encode_serde(&data).expect("serde encode failed");
        let wrapped = oxicode::checksum::wrap_with_checksum(&encoded);
        let verified = oxicode::checksum::verify_checksum(&wrapped).expect("verify failed");
        let decoded: SerdeData = oxicode::serde::decode_serde(verified).expect("decode failed");
        assert_eq!(data, decoded);
    }
}

// ── versioning + compression ──────────────────────────────────────────────────

#[cfg(all(feature = "compression-lz4", feature = "derive"))]
mod versioning_compression {
    use super::*;
    use oxicode::compression::{compress, decompress, Compression};
    use oxicode::versioning::Version;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Record {
        id: u64,
        tags: Vec<String>,
    }

    #[test]
    fn test_versioned_then_compressed_roundtrip() {
        let record = Record {
            id: 42,
            tags: vec!["alpha".into(), "beta".into(), "gamma".into()],
        };
        let version = Version::new(2, 0, 0);

        let versioned =
            oxicode::encode_versioned_value(&record, version).expect("versioned encode failed");
        let compressed = compress(&versioned, Compression::Lz4).expect("compress failed");

        // Reverse: decompress, then decode versioned
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, decoded_version, _): (Record, Version, usize) =
            oxicode::decode_versioned_value(&decompressed).expect("versioned decode failed");

        assert_eq!(record, decoded);
        assert_eq!(version, decoded_version);
    }
}
