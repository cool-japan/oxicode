#![cfg(feature = "compression-lz4")]
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

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SensorReading {
    id: u32,
    temperature: f32,
    humidity: f32,
    pressure: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Ok,
    Warning(String),
    Error { code: u32, msg: String },
}

#[test]
fn test_lz4_f32_roundtrip() {
    let value: f32 = 3.14159_f32;
    let encoded = encode_to_vec(&value).expect("encode f32 failed");
    let compressed = compress_lz4(&encoded).expect("compress f32 failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress f32 failed");
    let (decoded, _): (f32, usize) = decode_from_slice(&decompressed).expect("decode f32 failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_f64_roundtrip() {
    let value: f64 = 2.718281828459045_f64;
    let encoded = encode_to_vec(&value).expect("encode f64 failed");
    let compressed = compress_lz4(&encoded).expect("compress f64 failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress f64 failed");
    let (decoded, _): (f64, usize) = decode_from_slice(&decompressed).expect("decode f64 failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_i32_negative_roundtrip() {
    let value: i32 = -42_i32;
    let encoded = encode_to_vec(&value).expect("encode i32 failed");
    let compressed = compress_lz4(&encoded).expect("compress i32 failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress i32 failed");
    let (decoded, _): (i32, usize) = decode_from_slice(&decompressed).expect("decode i32 failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_i64_min_roundtrip() {
    let value: i64 = i64::MIN;
    let encoded = encode_to_vec(&value).expect("encode i64::MIN failed");
    let compressed = compress_lz4(&encoded).expect("compress i64::MIN failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress i64::MIN failed");
    let (decoded, _): (i64, usize) =
        decode_from_slice(&decompressed).expect("decode i64::MIN failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_sensor_reading_struct_roundtrip() {
    let reading = SensorReading {
        id: 1001,
        temperature: 23.5_f32,
        humidity: 68.2_f32,
        pressure: 1013.25_f64,
    };
    let encoded = encode_to_vec(&reading).expect("encode SensorReading failed");
    let compressed = compress_lz4(&encoded).expect("compress SensorReading failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress SensorReading failed");
    let (decoded, _): (SensorReading, usize) =
        decode_from_slice(&decompressed).expect("decode SensorReading failed");
    assert_eq!(reading, decoded);
}

#[test]
fn test_lz4_status_ok_roundtrip() {
    let status = Status::Ok;
    let encoded = encode_to_vec(&status).expect("encode Status::Ok failed");
    let compressed = compress_lz4(&encoded).expect("compress Status::Ok failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Status::Ok failed");
    let (decoded, _): (Status, usize) =
        decode_from_slice(&decompressed).expect("decode Status::Ok failed");
    assert_eq!(status, decoded);
}

#[test]
fn test_lz4_status_warning_roundtrip() {
    let status = Status::Warning("voltage high".to_string());
    let encoded = encode_to_vec(&status).expect("encode Status::Warning failed");
    let compressed = compress_lz4(&encoded).expect("compress Status::Warning failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Status::Warning failed");
    let (decoded, _): (Status, usize) =
        decode_from_slice(&decompressed).expect("decode Status::Warning failed");
    assert_eq!(status, decoded);
}

#[test]
fn test_lz4_status_error_roundtrip() {
    let status = Status::Error {
        code: 500,
        msg: "internal sensor fault".to_string(),
    };
    let encoded = encode_to_vec(&status).expect("encode Status::Error failed");
    let compressed = compress_lz4(&encoded).expect("compress Status::Error failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Status::Error failed");
    let (decoded, _): (Status, usize) =
        decode_from_slice(&decompressed).expect("decode Status::Error failed");
    assert_eq!(status, decoded);
}

#[test]
fn test_lz4_vec_sensor_readings_10_roundtrip() {
    let readings: Vec<SensorReading> = (0..10)
        .map(|i| SensorReading {
            id: i as u32,
            temperature: 20.0_f32 + i as f32 * 0.5_f32,
            humidity: 50.0_f32 + i as f32 * 1.0_f32,
            pressure: 1000.0_f64 + i as f64 * 2.5_f64,
        })
        .collect();
    let encoded = encode_to_vec(&readings).expect("encode Vec<SensorReading> failed");
    let compressed = compress_lz4(&encoded).expect("compress Vec<SensorReading> failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<SensorReading> failed");
    let (decoded, _): (Vec<SensorReading>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<SensorReading> failed");
    assert_eq!(readings, decoded);
}

#[test]
fn test_lz4_vec_status_mixed_roundtrip() {
    let statuses: Vec<Status> = vec![
        Status::Ok,
        Status::Warning("low battery".to_string()),
        Status::Error {
            code: 404,
            msg: "not found".to_string(),
        },
        Status::Ok,
        Status::Warning("overheat".to_string()),
    ];
    let encoded = encode_to_vec(&statuses).expect("encode Vec<Status> failed");
    let compressed = compress_lz4(&encoded).expect("compress Vec<Status> failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<Status> failed");
    let (decoded, _): (Vec<Status>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<Status> failed");
    assert_eq!(statuses, decoded);
}

#[test]
fn test_lz4_large_vec_f32_1000_roundtrip() {
    let data: Vec<f32> = (0..1000).map(|i| i as f32 * 0.001_f32).collect();
    let encoded = encode_to_vec(&data).expect("encode large Vec<f32> failed");
    let compressed = compress_lz4(&encoded).expect("compress large Vec<f32> failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress large Vec<f32> failed");
    let (decoded, _): (Vec<f32>, usize) =
        decode_from_slice(&decompressed).expect("decode large Vec<f32> failed");
    assert_eq!(data, decoded);
}

#[test]
fn test_lz4_repetitive_i32_compresses_smaller() {
    let data: Vec<i32> = vec![0xDEAD_i32; 500];
    let encoded = encode_to_vec(&data).expect("encode repetitive Vec<i32> failed");
    let compressed = compress_lz4(&encoded).expect("compress repetitive Vec<i32> failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes) for repetitive data",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress repetitive Vec<i32> failed");
    let (decoded, _): (Vec<i32>, usize) =
        decode_from_slice(&decompressed).expect("decode repetitive Vec<i32> failed");
    assert_eq!(data, decoded);
}

#[test]
fn test_lz4_nested_vec_vec_u8_roundtrip() {
    let data: Vec<Vec<u8>> = (0u8..8)
        .map(|i| (0u8..16).map(|j| i.wrapping_mul(j)).collect())
        .collect();
    let encoded = encode_to_vec(&data).expect("encode Vec<Vec<u8>> failed");
    let compressed = compress_lz4(&encoded).expect("compress Vec<Vec<u8>> failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<Vec<u8>> failed");
    let (decoded, _): (Vec<Vec<u8>>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<Vec<u8>> failed");
    assert_eq!(data, decoded);
}

#[test]
fn test_lz4_option_some_sensor_reading_roundtrip() {
    let value: Option<SensorReading> = Some(SensorReading {
        id: 77,
        temperature: -10.5_f32,
        humidity: 90.1_f32,
        pressure: 950.0_f64,
    });
    let encoded = encode_to_vec(&value).expect("encode Option<SensorReading> Some failed");
    let compressed = compress_lz4(&encoded).expect("compress Option<SensorReading> Some failed");
    let decompressed =
        decompress_lz4(&compressed).expect("decompress Option<SensorReading> Some failed");
    let (decoded, _): (Option<SensorReading>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<SensorReading> Some failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_option_none_sensor_reading_roundtrip() {
    let value: Option<SensorReading> = None;
    let encoded = encode_to_vec(&value).expect("encode Option<SensorReading> None failed");
    let compressed = compress_lz4(&encoded).expect("compress Option<SensorReading> None failed");
    let decompressed =
        decompress_lz4(&compressed).expect("decompress Option<SensorReading> None failed");
    let (decoded, _): (Option<SensorReading>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<SensorReading> None failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_compressed_output_nonempty_for_sensor_reading() {
    let reading = SensorReading {
        id: 42,
        temperature: 25.0_f32,
        humidity: 55.0_f32,
        pressure: 1012.0_f64,
    };
    let encoded = encode_to_vec(&reading).expect("encode SensorReading failed");
    let compressed = compress_lz4(&encoded).expect("compress SensorReading failed");
    assert!(
        !compressed.is_empty(),
        "compressed output must be non-empty"
    );
}

#[test]
fn test_lz4_decompressed_matches_original_encoded_bytes() {
    let reading = SensorReading {
        id: 99,
        temperature: 37.0_f32,
        humidity: 44.4_f32,
        pressure: 1005.5_f64,
    };
    let encoded = encode_to_vec(&reading).expect("encode SensorReading failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    assert_eq!(
        encoded, decompressed,
        "decompressed bytes must exactly match original encoded bytes"
    );
}

#[test]
fn test_lz4_large_data_5000_bytes_roundtrip() {
    let data: Vec<u8> = (0u8..=255).cycle().take(5000).collect();
    let encoded = encode_to_vec(&data).expect("encode 5000-byte Vec<u8> failed");
    let compressed = compress_lz4(&encoded).expect("compress 5000-byte data failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress 5000-byte data failed");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode 5000-byte data failed");
    assert_eq!(data, decoded);
}

#[test]
fn test_lz4_single_byte_u8_roundtrip() {
    let value: u8 = 0xAB_u8;
    let encoded = encode_to_vec(&value).expect("encode u8 failed");
    let compressed = compress_lz4(&encoded).expect("compress u8 failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress u8 failed");
    let (decoded, _): (u8, usize) = decode_from_slice(&decompressed).expect("decode u8 failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_u128_roundtrip() {
    let value: u128 = u128::MAX / 3;
    let encoded = encode_to_vec(&value).expect("encode u128 failed");
    let compressed = compress_lz4(&encoded).expect("compress u128 failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress u128 failed");
    let (decoded, _): (u128, usize) = decode_from_slice(&decompressed).expect("decode u128 failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_lz4_vec_string_50_roundtrip() {
    let strings: Vec<String> = (0..50)
        .map(|i| format!("sensor_node_{:04}_reading", i))
        .collect();
    let encoded = encode_to_vec(&strings).expect("encode Vec<String> failed");
    let compressed = compress_lz4(&encoded).expect("compress Vec<String> failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<String> failed");
    let (decoded, _): (Vec<String>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<String> failed");
    assert_eq!(strings, decoded);
}

#[test]
fn test_lz4_compress_twice_is_deterministic() {
    let reading = SensorReading {
        id: 7,
        temperature: 18.3_f32,
        humidity: 62.0_f32,
        pressure: 1008.0_f64,
    };
    let encoded = encode_to_vec(&reading).expect("encode SensorReading failed");
    let compressed_a = compress_lz4(&encoded).expect("compress first time failed");
    let compressed_b = compress_lz4(&encoded).expect("compress second time failed");
    assert_eq!(
        compressed_a, compressed_b,
        "lz4 compression must be deterministic across two calls"
    );
}
