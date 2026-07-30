//! Cross-library compatibility tests for the SERDE bridge, as opposed to the
//! native `Encode`/`Decode` derive path already covered in `src/lib.rs`.
//!
//! The serde bridge (`oxicode::serde::{encode_to_vec, decode_from_slice, ...}`)
//! hand-rolls its own wire decisions for the serde data model (see
//! `src/features/serde/ser.rs` / `de.rs`) rather than delegating to oxicode's
//! native `Encode`/`Decode` impls, so it needs its own byte-for-byte parity
//! check against `bincode::serde::{encode_to_vec, decode_from_slice, ...}`.
//!
//! Covers: struct, enum (unit/tuple/struct variants), `BTreeMap`, `Option`,
//! and `i128`/`u128`, under both `standard()` and `legacy()` configs, plus the
//! `std::io` reader/writer entry points.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SerdePoint {
    x: i32,
    y: i32,
    label: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum SerdeShape {
    Unit,
    Tuple(f64, f64),
    Struct { w: f64, h: f64 },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SerdeCorpus {
    point: SerdePoint,
    shapes: Vec<SerdeShape>,
    scores: BTreeMap<String, i64>,
    maybe: Option<u64>,
    absent: Option<u64>,
    big_signed: i128,
    big_unsigned: u128,
}

fn sample() -> SerdeCorpus {
    let mut scores = BTreeMap::new();
    scores.insert("alice".to_string(), 100);
    scores.insert("bob".to_string(), -50);

    SerdeCorpus {
        point: SerdePoint {
            x: 10,
            y: -20,
            label: "origin".to_string(),
        },
        shapes: vec![
            SerdeShape::Unit,
            SerdeShape::Tuple(1.5, 2.5),
            SerdeShape::Struct { w: 3.0, h: 4.0 },
        ],
        scores,
        maybe: Some(999),
        absent: None,
        big_signed: i128::MIN,
        big_unsigned: u128::MAX,
    }
}

#[test]
fn serde_struct_enum_map_option_i128_standard_config_byte_identical() {
    let value = sample();

    let oxi_bytes = oxicode::serde::encode_to_vec(&value, oxicode::config::standard())
        .expect("oxicode serde encode failed");
    let bin_bytes = bincode::serde::encode_to_vec(&value, bincode::config::standard())
        .expect("bincode serde encode failed");

    assert_eq!(
        oxi_bytes, bin_bytes,
        "serde-bridge encoding should be byte-identical under standard() config"
    );

    let (oxi_decoded, _): (SerdeCorpus, _) =
        oxicode::serde::decode_from_slice(&bin_bytes, oxicode::config::standard())
            .expect("oxicode serde decode of bincode bytes failed");
    assert_eq!(value, oxi_decoded);

    let (bin_decoded, _): (SerdeCorpus, usize) =
        bincode::serde::decode_from_slice(&oxi_bytes, bincode::config::standard())
            .expect("bincode serde decode of oxicode bytes failed");
    assert_eq!(value, bin_decoded);
}

#[test]
fn serde_struct_enum_map_option_i128_legacy_config_byte_identical() {
    let value = sample();

    let oxi_bytes = oxicode::serde::encode_to_vec(&value, oxicode::config::legacy())
        .expect("oxicode serde encode failed");
    let bin_bytes = bincode::serde::encode_to_vec(&value, bincode::config::legacy())
        .expect("bincode serde encode failed");

    assert_eq!(
        oxi_bytes, bin_bytes,
        "serde-bridge encoding should be byte-identical under legacy() config"
    );
}

#[test]
fn serde_i128_u128_boundary_values_byte_identical() {
    let values: Vec<i128> = vec![
        0,
        1,
        -1,
        i128::MIN,
        i128::MAX,
        i64::MAX as i128 + 1,
        i64::MIN as i128 - 1,
    ];

    for value in values {
        let oxi_bytes = oxicode::serde::encode_to_vec(&value, oxicode::config::standard())
            .expect("oxicode serde encode failed");
        let bin_bytes = bincode::serde::encode_to_vec(value, bincode::config::standard())
            .expect("bincode serde encode failed");
        assert_eq!(
            oxi_bytes, bin_bytes,
            "serde i128 encoding for {value} should be identical"
        );
    }

    let u_values: Vec<u128> = vec![0, 1, u64::MAX as u128 + 1, u128::MAX];
    for value in u_values {
        let oxi_bytes = oxicode::serde::encode_to_vec(&value, oxicode::config::standard())
            .expect("oxicode serde encode failed");
        let bin_bytes = bincode::serde::encode_to_vec(value, bincode::config::standard())
            .expect("bincode serde encode failed");
        assert_eq!(
            oxi_bytes, bin_bytes,
            "serde u128 encoding for {value} should be identical"
        );
    }
}

#[test]
fn serde_option_none_and_some_byte_identical() {
    let none_val: Option<u64> = None;
    let some_val: Option<u64> = Some(u64::MAX);

    for value in [none_val, some_val] {
        let oxi_bytes = oxicode::serde::encode_to_vec(&value, oxicode::config::standard())
            .expect("oxicode serde encode failed");
        let bin_bytes = bincode::serde::encode_to_vec(value, bincode::config::standard())
            .expect("bincode serde encode failed");
        assert_eq!(oxi_bytes, bin_bytes);
    }
}

#[test]
fn serde_btreemap_byte_identical() {
    let mut map: BTreeMap<u32, String> = BTreeMap::new();
    map.insert(1, "a".to_string());
    map.insert(2, "b".to_string());
    map.insert(1000, "big-key".to_string());

    let oxi_bytes = oxicode::serde::encode_to_vec(&map, oxicode::config::standard())
        .expect("oxicode serde encode failed");
    let bin_bytes = bincode::serde::encode_to_vec(&map, bincode::config::standard())
        .expect("bincode serde encode failed");
    assert_eq!(oxi_bytes, bin_bytes);
}

#[test]
fn serde_std_io_reader_writer_round_trip_byte_identical() {
    let value = sample();
    let oxi_config = oxicode::config::standard();
    let bin_config = bincode::config::standard();

    let mut oxi_buf: Vec<u8> = Vec::new();
    let oxi_written = oxicode::serde::encode_into_std_write(&value, &mut oxi_buf, oxi_config)
        .expect("oxicode serde encode_into_std_write failed");
    assert_eq!(oxi_written, oxi_buf.len());

    let mut bin_buf: Vec<u8> = Vec::new();
    let bin_written = bincode::serde::encode_into_std_write(&value, &mut bin_buf, bin_config)
        .expect("bincode serde encode_into_std_write failed");
    assert_eq!(bin_written, bin_buf.len());

    assert_eq!(
        oxi_buf, bin_buf,
        "serde std::io writer path should be byte-identical"
    );

    let (oxi_decoded, _): (SerdeCorpus, usize) =
        oxicode::serde::decode_from_std_read(&mut bin_buf.as_slice(), oxi_config)
            .expect("oxicode serde decode_from_std_read of bincode bytes failed");
    assert_eq!(value, oxi_decoded);

    let bin_decoded: SerdeCorpus =
        bincode::serde::decode_from_std_read(&mut oxi_buf.as_slice(), bin_config)
            .expect("bincode serde decode_from_std_read of oxicode bytes failed");
    assert_eq!(value, bin_decoded);
}
