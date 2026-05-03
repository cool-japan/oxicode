//! Compile-time harness: verifies that the corrected code from MIGRATION.md
//! actually compiles and produces correct round-trip results against the real API.
//!
//! Run with:
//!   cargo test -p oxicode --test migration_guide_compiles

use oxicode::{Decode, Encode};

/// Test 1: Basic encode_to_vec_with_config + decode_from_slice_with_config round-trip.
///
/// Mirrors MIGRATION.md "Step 3: Update Function Calls" section.
#[test]
fn test_basic_encode_decode() {
    #[derive(Encode, Decode, PartialEq, Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    let original = Point { x: 1.5, y: 2.5 };

    let encoded = oxicode::encode_to_vec_with_config(&original, oxicode::config::standard())
        .expect("encode failed");

    let (decoded, _len): (Point, usize) =
        oxicode::decode_from_slice_with_config(&encoded, oxicode::config::standard())
            .expect("decode failed");

    assert_eq!(original, decoded);
}

/// Test 2: Serde encode_to_vec + decode_from_slice round-trip.
///
/// Mirrors MIGRATION.md "Using Serde Integration" section.
/// Guarded by the `serde` feature flag.
#[cfg(feature = "serde")]
#[test]
fn test_serde_encode_decode() {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let original = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let encoded = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("serde encode failed");

    let (decoded, _len): (Person, usize) =
        oxicode::serde::decode_from_slice(&encoded, oxicode::config::standard())
            .expect("serde decode failed");

    assert_eq!(original, decoded);
}

/// Test 3: Legacy config round-trip demonstrating bincode 1.x wire-format compatibility.
///
/// Mirrors MIGRATION.md "Legacy/Bincode-Compatible Configuration" section.
/// Uses `oxicode::config::legacy()` which produces little-endian + fixed-int encoding —
/// the bincode 1.x default wire format (equivalent to bincode 2.0's `config::legacy()` preset).
#[test]
fn test_legacy_compat_encode_decode() {
    #[derive(Encode, Decode, PartialEq, Debug)]
    struct Record {
        id: u64,
        value: i32,
    }

    let original = Record { id: 42, value: -7 };

    let config = oxicode::config::legacy();

    let encoded =
        oxicode::encode_to_vec_with_config(&original, config).expect("legacy encode failed");

    let (decoded, _len): (Record, usize) =
        oxicode::decode_from_slice_with_config(&encoded, config).expect("legacy decode failed");

    assert_eq!(original, decoded);

    // Verify that legacy uses fixed-int encoding: a u64 should be exactly 8 bytes.
    let id_bytes = oxicode::encode_to_vec_with_config(&42u64, config).expect("encode u64 failed");
    assert_eq!(
        id_bytes.len(),
        8,
        "legacy config must use fixed-int encoding (8 bytes for u64)"
    );
}

/// Test 4: encode_to_fixed_array returns a tuple (arr, n); caller must unpack both.
///
/// Mirrors README.md line 203 fix: `let (arr, n): ([u8; 32], usize) = ...`
#[test]
fn test_encode_to_fixed_array_tuple_unpack() {
    #[derive(Encode, Decode, PartialEq, Debug)]
    struct Tag {
        id: u32,
        flag: bool,
    }
    let original = Tag { id: 99, flag: true };

    let (arr, n): ([u8; 32], usize) =
        oxicode::encode_to_fixed_array::<32, Tag>(&original).expect("encode failed");
    let (decoded, _): (Tag, usize) = oxicode::decode_from_slice(&arr[..n]).expect("decode failed");

    assert_eq!(original, decoded);
}

/// Test 5: encode_seq_to_vec accepts an iterator; decode_iter_from_slice needs turbofish.
///
/// Mirrors README.md lines 206-207 fix: `.into_iter()` and `::<T>` turbofish.
#[test]
fn test_encode_seq_iter_round_trip() {
    let items_in: [u8; 3] = [10, 20, 30];
    let bytes = oxicode::encode_seq_to_vec(items_in.into_iter()).expect("encode seq failed");
    let items_out: Vec<u8> = oxicode::decode_iter_from_slice::<u8>(&bytes)
        .expect("iter init failed")
        .collect::<Result<Vec<u8>, _>>()
        .expect("iter collect failed");
    assert_eq!(items_out, vec![10u8, 20, 30]);
}

/// Test 6: the `_with_config` named forms work for the 2-arg encode/decode path.
///
/// Mirrors README.md lines 418-419 fix: `encode_to_vec_with_config` / `decode_from_slice_with_config`.
#[test]
fn test_compat_two_arg_with_config_form() {
    #[derive(Encode, Decode, PartialEq, Debug)]
    struct Frame {
        seq: u32,
        payload: u64,
    }
    let original = Frame {
        seq: 7,
        payload: 0xDEAD_BEEF,
    };

    let bytes = oxicode::encode_to_vec_with_config(&original, oxicode::config::standard())
        .expect("encode_with_config failed");
    let (decoded, _): (Frame, usize) =
        oxicode::decode_from_slice_with_config(&bytes, oxicode::config::standard())
            .expect("decode_with_config failed");
    assert_eq!(original, decoded);
}
