//! Advanced file I/O tests for satellite telemetry / space mission domain

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TelemetryFrame {
    frame_id: u64,
    satellite_id: u32,
    timestamp: u64,
    battery_pct: u8,
    signal_dbm: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrbitData {
    epoch: u64,
    semi_major_axis: f64,
    eccentricity: f64,
    inclination: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommandPacket {
    sequence: u64,
    satellite_id: u32,
    command: String,
    params: Vec<u8>,
}

#[test]
fn test_telemetry_frame_basic_roundtrip() {
    let path = temp_dir().join("oxicode_sat_test_001.bin");
    let frame = TelemetryFrame {
        frame_id: 1001,
        satellite_id: 42,
        timestamp: 1_700_000_000,
        battery_pct: 87,
        signal_dbm: -45,
    };
    encode_to_file(&frame, &path).expect("encode TelemetryFrame failed");
    let decoded: TelemetryFrame = decode_from_file(&path).expect("decode TelemetryFrame failed");
    assert_eq!(frame, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_orbit_data_basic_roundtrip() {
    let path = temp_dir().join("oxicode_sat_test_002.bin");
    let orbit = OrbitData {
        epoch: 1_700_100_000,
        semi_major_axis: 6_778_137.0,
        eccentricity: 0.000_123,
        inclination: 51.6435,
    };
    encode_to_file(&orbit, &path).expect("encode OrbitData failed");
    let decoded: OrbitData = decode_from_file(&path).expect("decode OrbitData failed");
    assert_eq!(orbit, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_command_packet_basic_roundtrip() {
    let path = temp_dir().join("oxicode_sat_test_003.bin");
    let pkt = CommandPacket {
        sequence: 5001,
        satellite_id: 7,
        command: "SAFE_MODE".to_string(),
        params: vec![0x01, 0x02, 0x03],
    };
    encode_to_file(&pkt, &path).expect("encode CommandPacket failed");
    let decoded: CommandPacket = decode_from_file(&path).expect("decode CommandPacket failed");
    assert_eq!(pkt, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_telemetry_frame_negative_signal_dbm() {
    let path = temp_dir().join("oxicode_sat_test_004.bin");
    let frame = TelemetryFrame {
        frame_id: 9999,
        satellite_id: 1,
        timestamp: 1_700_200_000,
        battery_pct: 12,
        signal_dbm: -120,
    };
    encode_to_file(&frame, &path).expect("encode negative signal_dbm failed");
    let decoded: TelemetryFrame =
        decode_from_file(&path).expect("decode negative signal_dbm failed");
    assert_eq!(frame.signal_dbm, decoded.signal_dbm);
    assert_eq!(frame, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_telemetry_frame_max_signal_dbm() {
    let path = temp_dir().join("oxicode_sat_test_005.bin");
    let frame = TelemetryFrame {
        frame_id: 2,
        satellite_id: 255,
        timestamp: u64::MAX,
        battery_pct: 100,
        signal_dbm: i16::MAX,
    };
    encode_to_file(&frame, &path).expect("encode max signal_dbm failed");
    let decoded: TelemetryFrame = decode_from_file(&path).expect("decode max signal_dbm failed");
    assert_eq!(frame, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_telemetry_frame_min_signal_dbm() {
    let path = temp_dir().join("oxicode_sat_test_006.bin");
    let frame = TelemetryFrame {
        frame_id: 3,
        satellite_id: 0,
        timestamp: 0,
        battery_pct: 0,
        signal_dbm: i16::MIN,
    };
    encode_to_file(&frame, &path).expect("encode min signal_dbm failed");
    let decoded: TelemetryFrame = decode_from_file(&path).expect("decode min signal_dbm failed");
    assert_eq!(frame.signal_dbm, i16::MIN);
    assert_eq!(frame, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_orbit_data_high_precision_f64() {
    let path = temp_dir().join("oxicode_sat_test_007.bin");
    let orbit = OrbitData {
        epoch: 1_700_300_000,
        semi_major_axis: 42_164_000.123_456_789,
        eccentricity: 0.000_000_001,
        inclination: 0.0,
    };
    encode_to_file(&orbit, &path).expect("encode high-precision OrbitData failed");
    let decoded: OrbitData =
        decode_from_file(&path).expect("decode high-precision OrbitData failed");
    assert_eq!(
        orbit.semi_major_axis.to_bits(),
        decoded.semi_major_axis.to_bits()
    );
    assert_eq!(orbit.eccentricity.to_bits(), decoded.eccentricity.to_bits());
    assert_eq!(orbit, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_command_packet_empty_params() {
    let path = temp_dir().join("oxicode_sat_test_008.bin");
    let pkt = CommandPacket {
        sequence: 1,
        satellite_id: 10,
        command: "PING".to_string(),
        params: vec![],
    };
    encode_to_file(&pkt, &path).expect("encode CommandPacket empty params failed");
    let decoded: CommandPacket =
        decode_from_file(&path).expect("decode CommandPacket empty params failed");
    assert!(decoded.params.is_empty());
    assert_eq!(pkt, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_command_packet_large_params_vec() {
    let path = temp_dir().join("oxicode_sat_test_009.bin");
    let pkt = CommandPacket {
        sequence: 8888,
        satellite_id: 33,
        command: "BULK_UPLOAD".to_string(),
        params: (0u8..=255).cycle().take(10_000).collect(),
    };
    encode_to_file(&pkt, &path).expect("encode large CommandPacket params failed");
    let decoded: CommandPacket =
        decode_from_file(&path).expect("decode large CommandPacket params failed");
    assert_eq!(pkt.params.len(), decoded.params.len());
    assert_eq!(pkt, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_multiple_telemetry_files() {
    let frames = vec![
        TelemetryFrame {
            frame_id: 100,
            satellite_id: 1,
            timestamp: 1_700_400_000,
            battery_pct: 90,
            signal_dbm: -30,
        },
        TelemetryFrame {
            frame_id: 101,
            satellite_id: 2,
            timestamp: 1_700_400_001,
            battery_pct: 75,
            signal_dbm: -55,
        },
        TelemetryFrame {
            frame_id: 102,
            satellite_id: 3,
            timestamp: 1_700_400_002,
            battery_pct: 50,
            signal_dbm: -80,
        },
    ];
    let paths: Vec<_> = (10..13)
        .map(|n| temp_dir().join(format!("oxicode_sat_test_010_{n}.bin")))
        .collect();

    for (frame, path) in frames.iter().zip(paths.iter()) {
        encode_to_file(frame, path).expect("encode multi-file TelemetryFrame failed");
    }
    for (frame, path) in frames.iter().zip(paths.iter()) {
        let decoded: TelemetryFrame =
            decode_from_file(path).expect("decode multi-file TelemetryFrame failed");
        assert_eq!(*frame, decoded);
        if path.exists() {
            std::fs::remove_file(path).ok();
        }
    }
}

#[test]
fn test_overwrite_telemetry_frame() {
    let path = temp_dir().join("oxicode_sat_test_011.bin");
    let first = TelemetryFrame {
        frame_id: 1,
        satellite_id: 1,
        timestamp: 1_000,
        battery_pct: 50,
        signal_dbm: -10,
    };
    let second = TelemetryFrame {
        frame_id: 999,
        satellite_id: 88,
        timestamp: 9_999_999,
        battery_pct: 5,
        signal_dbm: -99,
    };
    encode_to_file(&first, &path).expect("first encode overwrite failed");
    encode_to_file(&second, &path).expect("second encode overwrite failed");
    let decoded: TelemetryFrame = decode_from_file(&path).expect("decode overwrite failed");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_file_size_matches_encode_to_vec_telemetry() {
    let path = temp_dir().join("oxicode_sat_test_012.bin");
    let frame = TelemetryFrame {
        frame_id: 777,
        satellite_id: 21,
        timestamp: 1_700_500_000,
        battery_pct: 65,
        signal_dbm: -42,
    };
    encode_to_file(&frame, &path).expect("encode for size check failed");
    let file_bytes = std::fs::read(&path).expect("read file for size check failed");
    let vec_bytes = encode_to_vec(&frame).expect("encode_to_vec for size check failed");
    assert_eq!(file_bytes.len(), vec_bytes.len());
    assert_eq!(file_bytes, vec_bytes);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_file_size_matches_encode_to_vec_orbit() {
    let path = temp_dir().join("oxicode_sat_test_013.bin");
    let orbit = OrbitData {
        epoch: 1_700_600_000,
        semi_major_axis: 7_000_000.0,
        eccentricity: 0.001,
        inclination: 97.4,
    };
    encode_to_file(&orbit, &path).expect("encode orbit for size check failed");
    let file_bytes = std::fs::read(&path).expect("read orbit file for size check failed");
    let vec_bytes = encode_to_vec(&orbit).expect("encode_to_vec orbit for size check failed");
    assert_eq!(file_bytes.len(), vec_bytes.len());
    assert_eq!(file_bytes, vec_bytes);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_file_size_matches_encode_to_vec_command() {
    let path = temp_dir().join("oxicode_sat_test_014.bin");
    let pkt = CommandPacket {
        sequence: 300,
        satellite_id: 15,
        command: "REBOOT_SUBSYSTEM".to_string(),
        params: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    encode_to_file(&pkt, &path).expect("encode command for size check failed");
    let file_bytes = std::fs::read(&path).expect("read command file for size check failed");
    let vec_bytes = encode_to_vec(&pkt).expect("encode_to_vec command for size check failed");
    assert_eq!(file_bytes.len(), vec_bytes.len());
    assert_eq!(file_bytes, vec_bytes);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_decode_from_slice_matches_file_telemetry() {
    let path = temp_dir().join("oxicode_sat_test_015.bin");
    let frame = TelemetryFrame {
        frame_id: 512,
        satellite_id: 7,
        timestamp: 1_700_700_000,
        battery_pct: 99,
        signal_dbm: 0,
    };
    encode_to_file(&frame, &path).expect("encode for slice compare failed");
    let file_bytes = std::fs::read(&path).expect("read for slice compare failed");
    let (slice_decoded, _): (TelemetryFrame, _) =
        decode_from_slice(&file_bytes).expect("decode_from_slice failed");
    let file_decoded: TelemetryFrame = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(slice_decoded, file_decoded);
    assert_eq!(frame, slice_decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_orbit_data_zero_eccentricity_circular() {
    let path = temp_dir().join("oxicode_sat_test_016.bin");
    let orbit = OrbitData {
        epoch: 0,
        semi_major_axis: 6_371_000.0,
        eccentricity: 0.0,
        inclination: 0.0,
    };
    encode_to_file(&orbit, &path).expect("encode circular orbit failed");
    let decoded: OrbitData = decode_from_file(&path).expect("decode circular orbit failed");
    assert_eq!(decoded.eccentricity, 0.0);
    assert_eq!(orbit, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_telemetry_frame_zero_battery() {
    let path = temp_dir().join("oxicode_sat_test_017.bin");
    let frame = TelemetryFrame {
        frame_id: 404,
        satellite_id: 99,
        timestamp: 1_700_800_000,
        battery_pct: 0,
        signal_dbm: -110,
    };
    encode_to_file(&frame, &path).expect("encode zero-battery frame failed");
    let decoded: TelemetryFrame =
        decode_from_file(&path).expect("decode zero-battery frame failed");
    assert_eq!(decoded.battery_pct, 0);
    assert_eq!(frame, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_command_packet_long_command_string() {
    let path = temp_dir().join("oxicode_sat_test_018.bin");
    let pkt = CommandPacket {
        sequence: 7777,
        satellite_id: 5,
        command: "EXECUTE_ATTITUDE_CONTROL_MANEUVER_DELTA_V_SEQUENCE_ALPHA".to_string(),
        params: (0u8..128).collect(),
    };
    encode_to_file(&pkt, &path).expect("encode long command string failed");
    let decoded: CommandPacket =
        decode_from_file(&path).expect("decode long command string failed");
    assert_eq!(pkt.command.len(), decoded.command.len());
    assert_eq!(pkt, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_sequential_orbit_file_overwrites() {
    let path = temp_dir().join("oxicode_sat_test_019.bin");
    let orbits = [
        OrbitData {
            epoch: 1_000,
            semi_major_axis: 6_500_000.0,
            eccentricity: 0.01,
            inclination: 28.5,
        },
        OrbitData {
            epoch: 2_000,
            semi_major_axis: 7_000_000.0,
            eccentricity: 0.02,
            inclination: 45.0,
        },
        OrbitData {
            epoch: 3_000,
            semi_major_axis: 8_000_000.0,
            eccentricity: 0.05,
            inclination: 90.0,
        },
    ];
    for orbit in &orbits {
        encode_to_file(orbit, &path).expect("encode sequential orbit overwrite failed");
    }
    let decoded: OrbitData =
        decode_from_file(&path).expect("decode sequential orbit overwrite failed");
    assert_eq!(orbits[2], decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_telemetry_positive_and_negative_signal_dbm_alternating() {
    let frames = vec![
        TelemetryFrame {
            frame_id: 200,
            satellite_id: 50,
            timestamp: 1_700_900_000,
            battery_pct: 80,
            signal_dbm: 10,
        },
        TelemetryFrame {
            frame_id: 201,
            satellite_id: 51,
            timestamp: 1_700_900_001,
            battery_pct: 60,
            signal_dbm: -10,
        },
        TelemetryFrame {
            frame_id: 202,
            satellite_id: 52,
            timestamp: 1_700_900_002,
            battery_pct: 40,
            signal_dbm: 5,
        },
        TelemetryFrame {
            frame_id: 203,
            satellite_id: 53,
            timestamp: 1_700_900_003,
            battery_pct: 20,
            signal_dbm: -5,
        },
    ];
    for (i, frame) in frames.iter().enumerate() {
        let path = temp_dir().join(format!("oxicode_sat_test_020_{i}.bin"));
        encode_to_file(frame, &path).expect("encode alternating signal_dbm failed");
        let decoded: TelemetryFrame =
            decode_from_file(&path).expect("decode alternating signal_dbm failed");
        assert_eq!(*frame, decoded);
        if path.exists() {
            std::fs::remove_file(&path).ok();
        }
    }
}

#[test]
fn test_command_packet_params_all_byte_values() {
    let path = temp_dir().join("oxicode_sat_test_021.bin");
    let pkt = CommandPacket {
        sequence: 65535,
        satellite_id: 128,
        command: "FULL_BYTE_RANGE_TEST".to_string(),
        params: (0u8..=255).collect(),
    };
    encode_to_file(&pkt, &path).expect("encode full-byte-range params failed");
    let decoded: CommandPacket =
        decode_from_file(&path).expect("decode full-byte-range params failed");
    assert_eq!(decoded.params.len(), 256);
    for (expected, actual) in (0u8..=255).zip(decoded.params.iter()) {
        assert_eq!(expected, *actual);
    }
    assert_eq!(pkt, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

#[test]
fn test_large_telemetry_batch_file_and_vec_consistency() {
    let path = temp_dir().join("oxicode_sat_test_022.bin");
    // Encode a Vec of TelemetryFrames representing a large telemetry batch
    let frames: Vec<TelemetryFrame> = (0u64..500)
        .map(|i| TelemetryFrame {
            frame_id: i,
            satellite_id: (i % 16) as u32,
            timestamp: 1_700_000_000 + i * 10,
            battery_pct: (100 - (i % 100)) as u8,
            signal_dbm: -50 - (i % 70) as i16,
        })
        .collect();
    encode_to_file(&frames, &path).expect("encode large telemetry batch to file failed");
    let file_bytes = std::fs::read(&path).expect("read large telemetry batch file failed");
    let vec_bytes = encode_to_vec(&frames).expect("encode_to_vec large telemetry batch failed");
    assert_eq!(
        file_bytes, vec_bytes,
        "file and vec encodings must be identical"
    );
    let decoded: Vec<TelemetryFrame> =
        decode_from_file(&path).expect("decode large telemetry batch from file failed");
    assert_eq!(frames.len(), decoded.len());
    assert_eq!(frames, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}
