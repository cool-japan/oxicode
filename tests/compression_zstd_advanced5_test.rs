#![cfg(feature = "compression-zstd")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

fn compress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Zstd).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FileRecord {
    path: String,
    size_bytes: u64,
    content_type: String,
    metadata: Vec<(String, String)>,
    is_directory: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FileEvent {
    Created { path: String },
    Modified { path: String, new_size: u64 },
    Deleted(String),
    Moved { from: String, to: String },
    Renamed { old_name: String, new_name: String },
}

#[test]
fn test_file_record_roundtrip_zstd() {
    let record = FileRecord {
        path: "/home/user/documents/report.pdf".to_string(),
        size_bytes: 1_048_576,
        content_type: "application/pdf".to_string(),
        metadata: vec![
            ("author".to_string(), "Alice".to_string()),
            ("created".to_string(), "2026-01-01".to_string()),
        ],
        is_directory: false,
    };
    let encoded = encode_to_vec(&record).expect("encode FileRecord");
    let compressed = compress_zstd(&encoded).expect("compress FileRecord");
    let decompressed = decompress_zstd(&compressed).expect("decompress FileRecord");
    let (decoded, _): (FileRecord, usize) =
        decode_from_slice(&decompressed).expect("decode FileRecord");
    assert_eq!(record, decoded);
}

#[test]
fn test_file_event_created_roundtrip_zstd() {
    let event = FileEvent::Created {
        path: "/var/log/app.log".to_string(),
    };
    let encoded = encode_to_vec(&event).expect("encode FileEvent::Created");
    let compressed = compress_zstd(&encoded).expect("compress FileEvent::Created");
    let decompressed = decompress_zstd(&compressed).expect("decompress FileEvent::Created");
    let (decoded, _): (FileEvent, usize) =
        decode_from_slice(&decompressed).expect("decode FileEvent::Created");
    assert_eq!(event, decoded);
}

#[test]
fn test_file_event_modified_roundtrip_zstd() {
    let event = FileEvent::Modified {
        path: "/etc/config.toml".to_string(),
        new_size: 2048,
    };
    let encoded = encode_to_vec(&event).expect("encode FileEvent::Modified");
    let compressed = compress_zstd(&encoded).expect("compress FileEvent::Modified");
    let decompressed = decompress_zstd(&compressed).expect("decompress FileEvent::Modified");
    let (decoded, _): (FileEvent, usize) =
        decode_from_slice(&decompressed).expect("decode FileEvent::Modified");
    assert_eq!(event, decoded);
}

#[test]
fn test_file_event_deleted_roundtrip_zstd() {
    let event = FileEvent::Deleted("/tmp/stale_lock.pid".to_string());
    let encoded = encode_to_vec(&event).expect("encode FileEvent::Deleted");
    let compressed = compress_zstd(&encoded).expect("compress FileEvent::Deleted");
    let decompressed = decompress_zstd(&compressed).expect("decompress FileEvent::Deleted");
    let (decoded, _): (FileEvent, usize) =
        decode_from_slice(&decompressed).expect("decode FileEvent::Deleted");
    assert_eq!(event, decoded);
}

#[test]
fn test_file_event_moved_roundtrip_zstd() {
    let event = FileEvent::Moved {
        from: "/home/user/old_dir/file.txt".to_string(),
        to: "/home/user/new_dir/file.txt".to_string(),
    };
    let encoded = encode_to_vec(&event).expect("encode FileEvent::Moved");
    let compressed = compress_zstd(&encoded).expect("compress FileEvent::Moved");
    let decompressed = decompress_zstd(&compressed).expect("decompress FileEvent::Moved");
    let (decoded, _): (FileEvent, usize) =
        decode_from_slice(&decompressed).expect("decode FileEvent::Moved");
    assert_eq!(event, decoded);
}

#[test]
fn test_file_event_renamed_roundtrip_zstd() {
    let event = FileEvent::Renamed {
        old_name: "report_draft.docx".to_string(),
        new_name: "report_final.docx".to_string(),
    };
    let encoded = encode_to_vec(&event).expect("encode FileEvent::Renamed");
    let compressed = compress_zstd(&encoded).expect("compress FileEvent::Renamed");
    let decompressed = decompress_zstd(&compressed).expect("decompress FileEvent::Renamed");
    let (decoded, _): (FileEvent, usize) =
        decode_from_slice(&decompressed).expect("decode FileEvent::Renamed");
    assert_eq!(event, decoded);
}

#[test]
fn test_vec_of_file_records_roundtrip_zstd() {
    let records: Vec<FileRecord> = (0..10)
        .map(|i| FileRecord {
            path: format!("/data/file_{}.bin", i),
            size_bytes: i as u64 * 1024,
            content_type: "application/octet-stream".to_string(),
            metadata: vec![("index".to_string(), i.to_string())],
            is_directory: false,
        })
        .collect();
    let encoded = encode_to_vec(&records).expect("encode Vec<FileRecord>");
    let compressed = compress_zstd(&encoded).expect("compress Vec<FileRecord>");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<FileRecord>");
    let (decoded, _): (Vec<FileRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<FileRecord>");
    assert_eq!(records, decoded);
}

#[test]
fn test_vec_of_file_events_roundtrip_zstd() {
    let events: Vec<FileEvent> = vec![
        FileEvent::Created {
            path: "/srv/uploads/img.png".to_string(),
        },
        FileEvent::Modified {
            path: "/srv/uploads/img.png".to_string(),
            new_size: 204_800,
        },
        FileEvent::Renamed {
            old_name: "img.png".to_string(),
            new_name: "image_final.png".to_string(),
        },
        FileEvent::Moved {
            from: "/srv/uploads/image_final.png".to_string(),
            to: "/srv/archive/image_final.png".to_string(),
        },
        FileEvent::Deleted("/srv/archive/image_final.png".to_string()),
    ];
    let encoded = encode_to_vec(&events).expect("encode Vec<FileEvent>");
    let compressed = compress_zstd(&encoded).expect("compress Vec<FileEvent>");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<FileEvent>");
    let (decoded, _): (Vec<FileEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<FileEvent>");
    assert_eq!(events, decoded);
}

#[test]
fn test_option_file_record_some_zstd() {
    let record: Option<FileRecord> = Some(FileRecord {
        path: "/opt/app/config.json".to_string(),
        size_bytes: 512,
        content_type: "application/json".to_string(),
        metadata: vec![],
        is_directory: false,
    });
    let encoded = encode_to_vec(&record).expect("encode Option<FileRecord> Some");
    let compressed = compress_zstd(&encoded).expect("compress Option<FileRecord> Some");
    let decompressed = decompress_zstd(&compressed).expect("decompress Option<FileRecord> Some");
    let (decoded, _): (Option<FileRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<FileRecord> Some");
    assert_eq!(record, decoded);
}

#[test]
fn test_option_file_record_none_zstd() {
    let record: Option<FileRecord> = None;
    let encoded = encode_to_vec(&record).expect("encode Option<FileRecord> None");
    let compressed = compress_zstd(&encoded).expect("compress Option<FileRecord> None");
    let decompressed = decompress_zstd(&compressed).expect("decompress Option<FileRecord> None");
    let (decoded, _): (Option<FileRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<FileRecord> None");
    assert_eq!(record, decoded);
}

#[test]
fn test_option_file_event_some_zstd() {
    let event: Option<FileEvent> = Some(FileEvent::Moved {
        from: "/a/b/c".to_string(),
        to: "/x/y/z".to_string(),
    });
    let encoded = encode_to_vec(&event).expect("encode Option<FileEvent> Some");
    let compressed = compress_zstd(&encoded).expect("compress Option<FileEvent> Some");
    let decompressed = decompress_zstd(&compressed).expect("decompress Option<FileEvent> Some");
    let (decoded, _): (Option<FileEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<FileEvent> Some");
    assert_eq!(event, decoded);
}

#[test]
fn test_compression_ratio_repetitive_file_records_zstd() {
    let record = FileRecord {
        path: "/data/repetitive_file.txt".to_string(),
        size_bytes: 0,
        content_type: "text/plain".to_string(),
        metadata: vec![
            ("key".to_string(), "value".to_string()),
            ("category".to_string(), "test".to_string()),
        ],
        is_directory: false,
    };
    let records: Vec<FileRecord> = std::iter::repeat_with(|| FileRecord {
        path: record.path.clone(),
        size_bytes: record.size_bytes,
        content_type: record.content_type.clone(),
        metadata: record.metadata.clone(),
        is_directory: record.is_directory,
    })
    .take(200)
    .collect();
    let encoded = encode_to_vec(&records).expect("encode repetitive FileRecords");
    let compressed = compress_zstd(&encoded).expect("compress repetitive FileRecords");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd should compress repetitive data: original={} compressed={}",
        encoded.len(),
        compressed.len()
    );
}

#[test]
fn test_compression_ratio_repetitive_file_events_zstd() {
    let events: Vec<FileEvent> = (0..150)
        .map(|i| FileEvent::Created {
            path: format!("/logs/event_{:05}.log", i % 10),
        })
        .collect();
    let encoded = encode_to_vec(&events).expect("encode repetitive FileEvents");
    let compressed = compress_zstd(&encoded).expect("compress repetitive FileEvents");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd should compress repetitive events: original={} compressed={}",
        encoded.len(),
        compressed.len()
    );
}

#[test]
fn test_idempotence_compress_decompress_file_record_zstd() {
    let record = FileRecord {
        path: "/usr/share/doc/readme.md".to_string(),
        size_bytes: 4096,
        content_type: "text/markdown".to_string(),
        metadata: vec![("version".to_string(), "1.0".to_string())],
        is_directory: false,
    };
    let encoded = encode_to_vec(&record).expect("encode for idempotence test");
    let compressed1 = compress_zstd(&encoded).expect("first compress");
    let decompressed1 = decompress_zstd(&compressed1).expect("first decompress");
    let compressed2 = compress_zstd(&decompressed1).expect("second compress");
    let decompressed2 = decompress_zstd(&compressed2).expect("second decompress");
    assert_eq!(
        encoded, decompressed1,
        "first decompress must equal original"
    );
    assert_eq!(
        encoded, decompressed2,
        "second decompress must equal original"
    );
}

#[test]
fn test_consumed_bytes_file_record_zstd() {
    let record = FileRecord {
        path: "/mnt/storage/archive.tar".to_string(),
        size_bytes: 10_000_000,
        content_type: "application/x-tar".to_string(),
        metadata: vec![
            ("compression".to_string(), "none".to_string()),
            ("owner".to_string(), "root".to_string()),
        ],
        is_directory: false,
    };
    let encoded = encode_to_vec(&record).expect("encode for consumed bytes test");
    let compressed = compress_zstd(&encoded).expect("compress for consumed bytes test");
    let decompressed = decompress_zstd(&compressed).expect("decompress for consumed bytes test");
    let (decoded, consumed): (FileRecord, usize) =
        decode_from_slice(&decompressed).expect("decode for consumed bytes test");
    assert_eq!(record, decoded);
    assert_eq!(
        consumed,
        decompressed.len(),
        "consumed bytes should equal decompressed length"
    );
}

#[test]
fn test_consumed_bytes_file_event_zstd() {
    let event = FileEvent::Renamed {
        old_name: "legacy_module.rs".to_string(),
        new_name: "modern_module.rs".to_string(),
    };
    let encoded = encode_to_vec(&event).expect("encode FileEvent for consumed bytes");
    let compressed = compress_zstd(&encoded).expect("compress FileEvent for consumed bytes");
    let decompressed =
        decompress_zstd(&compressed).expect("decompress FileEvent for consumed bytes");
    let (decoded, consumed): (FileEvent, usize) =
        decode_from_slice(&decompressed).expect("decode FileEvent for consumed bytes");
    assert_eq!(event, decoded);
    assert_eq!(consumed, decompressed.len());
}

#[test]
fn test_error_on_corrupt_zstd_data() {
    let record = FileRecord {
        path: "/etc/hosts".to_string(),
        size_bytes: 256,
        content_type: "text/plain".to_string(),
        metadata: vec![],
        is_directory: false,
    };
    let encoded = encode_to_vec(&record).expect("encode for corruption test");
    let mut compressed = compress_zstd(&encoded).expect("compress for corruption test");
    // Corrupt heavily — overwrite most of the data with garbage
    let len = compressed.len();
    if len > 4 {
        for byte in compressed[4..].iter_mut() {
            *byte ^= 0xFF;
        }
    }
    let result = decompress_zstd(&compressed);
    assert!(
        result.is_err(),
        "decompressing corrupt data should return an error"
    );
}

#[test]
fn test_error_on_empty_input_zstd() {
    let result = decompress_zstd(&[]);
    assert!(
        result.is_err(),
        "decompressing empty input should return an error"
    );
}

#[test]
fn test_error_on_raw_bytes_as_zstd() {
    let garbage: Vec<u8> = (0u8..=127).cycle().take(256).collect();
    let result = decompress_zstd(&garbage);
    assert!(
        result.is_err(),
        "decompressing non-zstd bytes should return an error"
    );
}

#[test]
fn test_directory_file_record_roundtrip_zstd() {
    let record = FileRecord {
        path: "/var/lib/data/".to_string(),
        size_bytes: 0,
        content_type: "inode/directory".to_string(),
        metadata: vec![
            ("permissions".to_string(), "755".to_string()),
            ("owner".to_string(), "www-data".to_string()),
        ],
        is_directory: true,
    };
    let encoded = encode_to_vec(&record).expect("encode directory FileRecord");
    let compressed = compress_zstd(&encoded).expect("compress directory FileRecord");
    let decompressed = decompress_zstd(&compressed).expect("decompress directory FileRecord");
    let (decoded, _): (FileRecord, usize) =
        decode_from_slice(&decompressed).expect("decode directory FileRecord");
    assert_eq!(record, decoded);
    assert!(decoded.is_directory);
    assert_eq!(decoded.size_bytes, 0);
}

#[test]
fn test_large_metadata_file_record_zstd() {
    let metadata: Vec<(String, String)> = (0..100)
        .map(|i| (format!("meta_key_{:03}", i), format!("meta_value_{:03}", i)))
        .collect();
    let record = FileRecord {
        path: "/srv/large_meta/file.dat".to_string(),
        size_bytes: 999_999_999,
        content_type: "application/octet-stream".to_string(),
        metadata,
        is_directory: false,
    };
    let encoded = encode_to_vec(&record).expect("encode large-metadata FileRecord");
    let compressed = compress_zstd(&encoded).expect("compress large-metadata FileRecord");
    let decompressed = decompress_zstd(&compressed).expect("decompress large-metadata FileRecord");
    let (decoded, _): (FileRecord, usize) =
        decode_from_slice(&decompressed).expect("decode large-metadata FileRecord");
    assert_eq!(record, decoded);
    assert_eq!(decoded.metadata.len(), 100);
}

#[test]
fn test_unicode_paths_file_event_zstd() {
    let events: Vec<FileEvent> = vec![
        FileEvent::Created {
            path: "/データ/ファイル.txt".to_string(),
        },
        FileEvent::Moved {
            from: "/datos/archivo_viejo.bin".to_string(),
            to: "/données/fichier_nouveau.bin".to_string(),
        },
        FileEvent::Renamed {
            old_name: "旧ファイル名.rs".to_string(),
            new_name: "新ファイル名.rs".to_string(),
        },
    ];
    let encoded = encode_to_vec(&events).expect("encode unicode FileEvents");
    let compressed = compress_zstd(&encoded).expect("compress unicode FileEvents");
    let decompressed = decompress_zstd(&compressed).expect("decompress unicode FileEvents");
    let (decoded, _): (Vec<FileEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode unicode FileEvents");
    assert_eq!(events, decoded);
}

#[test]
fn test_zero_size_file_record_zstd() {
    let record = FileRecord {
        path: "/dev/null".to_string(),
        size_bytes: 0,
        content_type: "application/x-empty".to_string(),
        metadata: vec![],
        is_directory: false,
    };
    let encoded = encode_to_vec(&record).expect("encode zero-size FileRecord");
    let compressed = compress_zstd(&encoded).expect("compress zero-size FileRecord");
    let decompressed = decompress_zstd(&compressed).expect("decompress zero-size FileRecord");
    let (decoded, _): (FileRecord, usize) =
        decode_from_slice(&decompressed).expect("decode zero-size FileRecord");
    assert_eq!(record, decoded);
    assert_eq!(decoded.size_bytes, 0);
    assert!(!decoded.is_directory);
}
