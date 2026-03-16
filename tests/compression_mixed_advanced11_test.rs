#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RegisterType {
    Coil,
    DiscreteInput,
    InputRegister,
    HoldingRegister,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ModbusFunction {
    ReadCoils,
    ReadDiscreteInputs,
    ReadHoldingRegisters,
    WriteMultipleRegisters,
    WriteMultipleCoils,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlarmPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProcessState {
    Running,
    Stopped,
    Fault,
    Maintenance,
    Starting,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModbusFrame {
    device_id: u8,
    function: ModbusFunction,
    register_addr: u16,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProcessTag {
    tag_id: u32,
    name: String,
    register_type: RegisterType,
    value_raw: u32,
    quality_ok: bool,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlarmRecord {
    alarm_id: u64,
    tag_id: u32,
    priority: AlarmPriority,
    message: String,
    triggered_at: u64,
    acknowledged: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ControlLoop {
    loop_id: u32,
    name: String,
    state: ProcessState,
    setpoint_x1000: i32,
    pv_x1000: i32,
    output_pct_x100: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EventLog {
    event_id: u64,
    timestamp: u64,
    source: String,
    description: String,
    state: ProcessState,
}

// Test 1: ModbusFrame roundtrip with LZ4
#[test]
fn test_modbus_frame_roundtrip_lz4() {
    let frame = ModbusFrame {
        device_id: 1,
        function: ModbusFunction::ReadHoldingRegisters,
        register_addr: 0x0100,
        data: vec![0x00, 0x0A, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    };
    let encoded = encode_to_vec(&frame).expect("encode ModbusFrame");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress ModbusFrame");
    let decompressed = decompress(&compressed).expect("lz4 decompress ModbusFrame");
    let (decoded, _): (ModbusFrame, usize) =
        decode_from_slice(&decompressed).expect("decode ModbusFrame lz4");
    assert_eq!(frame, decoded);
}

// Test 2: ModbusFrame roundtrip with Zstd
#[test]
fn test_modbus_frame_roundtrip_zstd() {
    let frame = ModbusFrame {
        device_id: 15,
        function: ModbusFunction::WriteMultipleRegisters,
        register_addr: 0x0200,
        data: vec![0x01, 0x00, 0x00, 0x64, 0x00, 0x00, 0x01, 0xF4],
    };
    let encoded = encode_to_vec(&frame).expect("encode ModbusFrame");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress ModbusFrame");
    let decompressed = decompress(&compressed).expect("zstd decompress ModbusFrame");
    let (decoded, _): (ModbusFrame, usize) =
        decode_from_slice(&decompressed).expect("decode ModbusFrame zstd");
    assert_eq!(frame, decoded);
}

// Test 3: ProcessTag roundtrip with LZ4
#[test]
fn test_process_tag_roundtrip_lz4() {
    let tag = ProcessTag {
        tag_id: 42,
        name: "TIC-101.PV".to_string(),
        register_type: RegisterType::InputRegister,
        value_raw: 27350,
        quality_ok: true,
        timestamp: 1_700_000_001,
    };
    let encoded = encode_to_vec(&tag).expect("encode ProcessTag");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress ProcessTag");
    let decompressed = decompress(&compressed).expect("lz4 decompress ProcessTag");
    let (decoded, _): (ProcessTag, usize) =
        decode_from_slice(&decompressed).expect("decode ProcessTag lz4");
    assert_eq!(tag, decoded);
}

// Test 4: ProcessTag roundtrip with Zstd
#[test]
fn test_process_tag_roundtrip_zstd() {
    let tag = ProcessTag {
        tag_id: 99,
        name: "FIC-201.SP".to_string(),
        register_type: RegisterType::HoldingRegister,
        value_raw: 50000,
        quality_ok: false,
        timestamp: 1_700_000_500,
    };
    let encoded = encode_to_vec(&tag).expect("encode ProcessTag");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress ProcessTag");
    let decompressed = decompress(&compressed).expect("zstd decompress ProcessTag");
    let (decoded, _): (ProcessTag, usize) =
        decode_from_slice(&decompressed).expect("decode ProcessTag zstd");
    assert_eq!(tag, decoded);
}

// Test 5: AlarmRecord roundtrip with LZ4
#[test]
fn test_alarm_record_roundtrip_lz4() {
    let alarm = AlarmRecord {
        alarm_id: 1001,
        tag_id: 42,
        priority: AlarmPriority::High,
        message: "TIC-101 temperature exceeded high-high limit".to_string(),
        triggered_at: 1_700_001_000,
        acknowledged: false,
    };
    let encoded = encode_to_vec(&alarm).expect("encode AlarmRecord");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress AlarmRecord");
    let decompressed = decompress(&compressed).expect("lz4 decompress AlarmRecord");
    let (decoded, _): (AlarmRecord, usize) =
        decode_from_slice(&decompressed).expect("decode AlarmRecord lz4");
    assert_eq!(alarm, decoded);
}

// Test 6: AlarmRecord roundtrip with Zstd
#[test]
fn test_alarm_record_roundtrip_zstd() {
    let alarm = AlarmRecord {
        alarm_id: 2002,
        tag_id: 99,
        priority: AlarmPriority::Critical,
        message: "Emergency shutdown initiated: pressure exceeds 120 bar".to_string(),
        triggered_at: 1_700_002_000,
        acknowledged: true,
    };
    let encoded = encode_to_vec(&alarm).expect("encode AlarmRecord");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress AlarmRecord");
    let decompressed = decompress(&compressed).expect("zstd decompress AlarmRecord");
    let (decoded, _): (AlarmRecord, usize) =
        decode_from_slice(&decompressed).expect("decode AlarmRecord zstd");
    assert_eq!(alarm, decoded);
}

// Test 7: ControlLoop roundtrip with LZ4
#[test]
fn test_control_loop_roundtrip_lz4() {
    let ctrl = ControlLoop {
        loop_id: 301,
        name: "TEMP_CTRL_REACTOR_A".to_string(),
        state: ProcessState::Running,
        setpoint_x1000: 85_000,
        pv_x1000: 84_750,
        output_pct_x100: 6250,
    };
    let encoded = encode_to_vec(&ctrl).expect("encode ControlLoop");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress ControlLoop");
    let decompressed = decompress(&compressed).expect("lz4 decompress ControlLoop");
    let (decoded, _): (ControlLoop, usize) =
        decode_from_slice(&decompressed).expect("decode ControlLoop lz4");
    assert_eq!(ctrl, decoded);
}

// Test 8: ControlLoop roundtrip with Zstd
#[test]
fn test_control_loop_roundtrip_zstd() {
    let ctrl = ControlLoop {
        loop_id: 402,
        name: "FLOW_CTRL_LINE_B".to_string(),
        state: ProcessState::Maintenance,
        setpoint_x1000: 120_000,
        pv_x1000: 0,
        output_pct_x100: 0,
    };
    let encoded = encode_to_vec(&ctrl).expect("encode ControlLoop");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress ControlLoop");
    let decompressed = decompress(&compressed).expect("zstd decompress ControlLoop");
    let (decoded, _): (ControlLoop, usize) =
        decode_from_slice(&decompressed).expect("decode ControlLoop zstd");
    assert_eq!(ctrl, decoded);
}

// Test 9: EventLog roundtrip with LZ4
#[test]
fn test_event_log_roundtrip_lz4() {
    let event = EventLog {
        event_id: 500001,
        timestamp: 1_700_003_000,
        source: "PLC-UNIT-03".to_string(),
        description: "Reactor A startup sequence initiated by operator".to_string(),
        state: ProcessState::Starting,
    };
    let encoded = encode_to_vec(&event).expect("encode EventLog");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress EventLog");
    let decompressed = decompress(&compressed).expect("lz4 decompress EventLog");
    let (decoded, _): (EventLog, usize) =
        decode_from_slice(&decompressed).expect("decode EventLog lz4");
    assert_eq!(event, decoded);
}

// Test 10: EventLog roundtrip with Zstd
#[test]
fn test_event_log_roundtrip_zstd() {
    let event = EventLog {
        event_id: 600002,
        timestamp: 1_700_004_000,
        source: "SCADA-SERVER-01".to_string(),
        description: "Fault detected on pump P-201: motor overload protection tripped".to_string(),
        state: ProcessState::Fault,
    };
    let encoded = encode_to_vec(&event).expect("encode EventLog");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress EventLog");
    let decompressed = decompress(&compressed).expect("zstd decompress EventLog");
    let (decoded, _): (EventLog, usize) =
        decode_from_slice(&decompressed).expect("decode EventLog zstd");
    assert_eq!(event, decoded);
}

// Test 11: Large tag scan — LZ4 compression ratio (1000+ repetitive ProcessTag entries)
#[test]
fn test_large_tag_scan_lz4_compression_ratio() {
    let tags: Vec<ProcessTag> = (0..1000)
        .map(|i| ProcessTag {
            tag_id: i as u32,
            name: format!("TAG_{:04}", i % 20),
            register_type: RegisterType::InputRegister,
            value_raw: 25000 + (i % 50) as u32,
            quality_ok: true,
            timestamp: 1_700_000_000 + i as u64,
        })
        .collect();

    let encoded = encode_to_vec(&tags).expect("encode large tag scan");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large tag scan");

    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive tag scan: {} -> {}",
        encoded.len(),
        compressed.len()
    );

    let decompressed = decompress(&compressed).expect("lz4 decompress large tag scan");
    let (decoded, _): (Vec<ProcessTag>, usize) =
        decode_from_slice(&decompressed).expect("decode large tag scan lz4");
    assert_eq!(tags, decoded);
}

// Test 12: Large tag scan — Zstd compression ratio (1000+ repetitive ProcessTag entries)
#[test]
fn test_large_tag_scan_zstd_compression_ratio() {
    let tags: Vec<ProcessTag> = (0..1000)
        .map(|i| ProcessTag {
            tag_id: i as u32,
            name: format!("TAG_{:04}", i % 20),
            register_type: RegisterType::HoldingRegister,
            value_raw: 30000 + (i % 100) as u32,
            quality_ok: true,
            timestamp: 1_700_010_000 + i as u64,
        })
        .collect();

    let encoded = encode_to_vec(&tags).expect("encode large tag scan zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large tag scan");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd should compress repetitive tag scan: {} -> {}",
        encoded.len(),
        compressed.len()
    );

    let decompressed = decompress(&compressed).expect("zstd decompress large tag scan");
    let (decoded, _): (Vec<ProcessTag>, usize) =
        decode_from_slice(&decompressed).expect("decode large tag scan zstd");
    assert_eq!(tags, decoded);
}

// Test 13: Large alarm log — LZ4 compression ratio (1000+ alarms)
#[test]
fn test_large_alarm_log_lz4_compression_ratio() {
    let alarms: Vec<AlarmRecord> = (0..1000)
        .map(|i| AlarmRecord {
            alarm_id: 10000 + i as u64,
            tag_id: (i % 50) as u32,
            priority: match i % 4 {
                0 => AlarmPriority::Low,
                1 => AlarmPriority::Medium,
                2 => AlarmPriority::High,
                _ => AlarmPriority::Critical,
            },
            message: format!("Process deviation on TAG_{:04}: value out of range", i % 50),
            triggered_at: 1_700_020_000 + i as u64,
            acknowledged: i % 3 == 0,
        })
        .collect();

    let encoded = encode_to_vec(&alarms).expect("encode large alarm log");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large alarm log");

    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive alarm log: {} -> {}",
        encoded.len(),
        compressed.len()
    );

    let decompressed = decompress(&compressed).expect("lz4 decompress large alarm log");
    let (decoded, _): (Vec<AlarmRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode large alarm log lz4");
    assert_eq!(alarms, decoded);
}

// Test 14: Large alarm log — Zstd compression ratio (1000+ alarms)
#[test]
fn test_large_alarm_log_zstd_compression_ratio() {
    let alarms: Vec<AlarmRecord> = (0..1000)
        .map(|i| AlarmRecord {
            alarm_id: 20000 + i as u64,
            tag_id: (i % 50) as u32,
            priority: match i % 4 {
                0 => AlarmPriority::Low,
                1 => AlarmPriority::Medium,
                2 => AlarmPriority::High,
                _ => AlarmPriority::Critical,
            },
            message: format!(
                "Alarm condition active on TAG_{:04}: threshold exceeded",
                i % 50
            ),
            triggered_at: 1_700_030_000 + i as u64,
            acknowledged: i % 5 == 0,
        })
        .collect();

    let encoded = encode_to_vec(&alarms).expect("encode large alarm log zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large alarm log");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd should compress repetitive alarm log: {} -> {}",
        encoded.len(),
        compressed.len()
    );

    let decompressed = decompress(&compressed).expect("zstd decompress large alarm log");
    let (decoded, _): (Vec<AlarmRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode large alarm log zstd");
    assert_eq!(alarms, decoded);
}

// Test 15: LZ4 vs Zstd produce different bytes but same decoded result — ControlLoop
#[test]
fn test_lz4_vs_zstd_different_bytes_same_result_control_loop() {
    let ctrl = ControlLoop {
        loop_id: 701,
        name: "PRESSURE_CTRL_VESSEL_C".to_string(),
        state: ProcessState::Running,
        setpoint_x1000: 50_000,
        pv_x1000: 49_800,
        output_pct_x100: 4800,
    };

    let encoded = encode_to_vec(&ctrl).expect("encode ControlLoop for comparison");

    let compressed_lz4 =
        compress(&encoded, Compression::Lz4).expect("lz4 compress ControlLoop comparison");
    let compressed_zstd =
        compress(&encoded, Compression::Zstd).expect("zstd compress ControlLoop comparison");

    assert_ne!(
        compressed_lz4, compressed_zstd,
        "LZ4 and Zstd compressed bytes must differ"
    );

    let decompressed_lz4 =
        decompress(&compressed_lz4).expect("lz4 decompress ControlLoop comparison");
    let decompressed_zstd =
        decompress(&compressed_zstd).expect("zstd decompress ControlLoop comparison");

    let (decoded_lz4, _): (ControlLoop, usize) =
        decode_from_slice(&decompressed_lz4).expect("decode ControlLoop from lz4");
    let (decoded_zstd, _): (ControlLoop, usize) =
        decode_from_slice(&decompressed_zstd).expect("decode ControlLoop from zstd");

    assert_eq!(
        decoded_lz4, decoded_zstd,
        "Decoded results must be identical regardless of codec"
    );
    assert_eq!(ctrl, decoded_lz4);
}

// Test 16: LZ4 vs Zstd produce different bytes but same decoded result — EventLog
#[test]
fn test_lz4_vs_zstd_different_bytes_same_result_event_log() {
    let event = EventLog {
        event_id: 999999,
        timestamp: 1_700_050_000,
        source: "RTU-FIELDBUS-07".to_string(),
        description: "Valve V-305 position feedback mismatch detected during self-test".to_string(),
        state: ProcessState::Fault,
    };

    let encoded = encode_to_vec(&event).expect("encode EventLog for comparison");

    let compressed_lz4 =
        compress(&encoded, Compression::Lz4).expect("lz4 compress EventLog comparison");
    let compressed_zstd =
        compress(&encoded, Compression::Zstd).expect("zstd compress EventLog comparison");

    assert_ne!(
        compressed_lz4, compressed_zstd,
        "LZ4 and Zstd compressed bytes must differ for EventLog"
    );

    let decompressed_lz4 = decompress(&compressed_lz4).expect("lz4 decompress EventLog comparison");
    let decompressed_zstd =
        decompress(&compressed_zstd).expect("zstd decompress EventLog comparison");

    let (decoded_lz4, _): (EventLog, usize) =
        decode_from_slice(&decompressed_lz4).expect("decode EventLog from lz4");
    let (decoded_zstd, _): (EventLog, usize) =
        decode_from_slice(&decompressed_zstd).expect("decode EventLog from zstd");

    assert_eq!(decoded_lz4, decoded_zstd);
    assert_eq!(event, decoded_lz4);
}

// Test 17: Error on corrupted LZ4 compressed data
#[test]
fn test_corrupted_lz4_data_returns_error() {
    let tag = ProcessTag {
        tag_id: 77,
        name: "AI-501.PV".to_string(),
        register_type: RegisterType::InputRegister,
        value_raw: 15000,
        quality_ok: true,
        timestamp: 1_700_060_000,
    };
    let encoded = encode_to_vec(&tag).expect("encode ProcessTag for corruption test");
    let corrupted = compress(&encoded, Compression::Lz4).expect("lz4 compress for corruption test");

    // Truncate to 4 bytes (guarantees decompression failure)
    let truncated = &corrupted[..4.min(corrupted.len())];

    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "Decompress of corrupted LZ4 data should return an error"
    );
}

// Test 18: Error on corrupted Zstd compressed data
#[test]
fn test_corrupted_zstd_data_returns_error() {
    let alarm = AlarmRecord {
        alarm_id: 3333,
        tag_id: 11,
        priority: AlarmPriority::Medium,
        message: "Conveyor speed below minimum threshold".to_string(),
        triggered_at: 1_700_070_000,
        acknowledged: false,
    };
    let encoded = encode_to_vec(&alarm).expect("encode AlarmRecord for corruption test");
    let corrupted =
        compress(&encoded, Compression::Zstd).expect("zstd compress for corruption test");

    // Truncate to 4 bytes (guarantees decompression failure)
    let truncated = &corrupted[..4.min(corrupted.len())];

    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "Decompress of corrupted Zstd data should return an error"
    );
}

// Test 19: Empty data vec in ModbusFrame — LZ4 edge case
#[test]
fn test_empty_data_vec_modbus_frame_lz4() {
    let frame = ModbusFrame {
        device_id: 0,
        function: ModbusFunction::ReadCoils,
        register_addr: 0x0000,
        data: vec![],
    };
    let encoded = encode_to_vec(&frame).expect("encode ModbusFrame empty data");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress ModbusFrame empty data");
    let decompressed = decompress(&compressed).expect("lz4 decompress ModbusFrame empty data");
    let (decoded, _): (ModbusFrame, usize) =
        decode_from_slice(&decompressed).expect("decode ModbusFrame empty data lz4");
    assert_eq!(frame, decoded);
    assert!(decoded.data.is_empty(), "Decoded data vec must be empty");
}

// Test 20: Empty data vec in ModbusFrame — Zstd edge case
#[test]
fn test_empty_data_vec_modbus_frame_zstd() {
    let frame = ModbusFrame {
        device_id: 0,
        function: ModbusFunction::ReadDiscreteInputs,
        register_addr: 0x0000,
        data: vec![],
    };
    let encoded = encode_to_vec(&frame).expect("encode ModbusFrame empty data zstd");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress ModbusFrame empty data");
    let decompressed = decompress(&compressed).expect("zstd decompress ModbusFrame empty data");
    let (decoded, _): (ModbusFrame, usize) =
        decode_from_slice(&decompressed).expect("decode ModbusFrame empty data zstd");
    assert_eq!(frame, decoded);
    assert!(decoded.data.is_empty(), "Decoded data vec must be empty");
}

// Test 21: Multiple control loops roundtrip with LZ4 — mixed process states
#[test]
fn test_multiple_control_loops_lz4() {
    let loops = vec![
        ControlLoop {
            loop_id: 1,
            name: "TEMP_CTRL_UNIT_01".to_string(),
            state: ProcessState::Running,
            setpoint_x1000: 75_000,
            pv_x1000: 74_900,
            output_pct_x100: 5200,
        },
        ControlLoop {
            loop_id: 2,
            name: "FLOW_CTRL_UNIT_02".to_string(),
            state: ProcessState::Stopped,
            setpoint_x1000: 0,
            pv_x1000: 0,
            output_pct_x100: 0,
        },
        ControlLoop {
            loop_id: 3,
            name: "LEVEL_CTRL_TANK_A".to_string(),
            state: ProcessState::Fault,
            setpoint_x1000: 60_000,
            pv_x1000: 80_000,
            output_pct_x100: 10000,
        },
        ControlLoop {
            loop_id: 4,
            name: "PRESSURE_CTRL_LINE_C".to_string(),
            state: ProcessState::Starting,
            setpoint_x1000: 40_000,
            pv_x1000: 5_000,
            output_pct_x100: 9900,
        },
    ];

    let encoded = encode_to_vec(&loops).expect("encode multiple control loops");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress multiple control loops");
    let decompressed = decompress(&compressed).expect("lz4 decompress multiple control loops");
    let (decoded, _): (Vec<ControlLoop>, usize) =
        decode_from_slice(&decompressed).expect("decode multiple control loops lz4");
    assert_eq!(loops, decoded);
    assert_eq!(decoded.len(), 4);
}

// Test 22: Modbus broadcast scan with all function codes — Zstd roundtrip
#[test]
fn test_modbus_all_function_codes_zstd() {
    let frames = vec![
        ModbusFrame {
            device_id: 1,
            function: ModbusFunction::ReadCoils,
            register_addr: 0x0000,
            data: vec![0x00, 0x08],
        },
        ModbusFrame {
            device_id: 2,
            function: ModbusFunction::ReadDiscreteInputs,
            register_addr: 0x0010,
            data: vec![0x00, 0x10],
        },
        ModbusFrame {
            device_id: 3,
            function: ModbusFunction::ReadHoldingRegisters,
            register_addr: 0x0100,
            data: vec![0x00, 0x04],
        },
        ModbusFrame {
            device_id: 4,
            function: ModbusFunction::WriteMultipleRegisters,
            register_addr: 0x0200,
            data: vec![0x01, 0x00, 0x00, 0x64, 0x00, 0x00, 0x01, 0xF4],
        },
        ModbusFrame {
            device_id: 5,
            function: ModbusFunction::WriteMultipleCoils,
            register_addr: 0x0300,
            data: vec![0x00, 0x08, 0x01, 0xAB],
        },
    ];

    let encoded = encode_to_vec(&frames).expect("encode all function code frames");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress all function code frames");
    let decompressed = decompress(&compressed).expect("zstd decompress all function code frames");
    let (decoded, _): (Vec<ModbusFrame>, usize) =
        decode_from_slice(&decompressed).expect("decode all function code frames zstd");
    assert_eq!(frames, decoded);
    assert_eq!(decoded.len(), 5);

    // Verify each function code survived the roundtrip
    assert_eq!(decoded[0].function, ModbusFunction::ReadCoils);
    assert_eq!(decoded[1].function, ModbusFunction::ReadDiscreteInputs);
    assert_eq!(decoded[2].function, ModbusFunction::ReadHoldingRegisters);
    assert_eq!(decoded[3].function, ModbusFunction::WriteMultipleRegisters);
    assert_eq!(decoded[4].function, ModbusFunction::WriteMultipleCoils);
}
