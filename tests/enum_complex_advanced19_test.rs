//! Industrial automation / PLC domain tests for oxicode encode/decode.
//! Covers PLCs, SCADA, sensor types, alarm states, and production lines.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ─── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum PlcSignalType {
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    AnalogOutput,
    Counter,
    Timer,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AlarmSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Emergency,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AlarmState {
    Active,
    Acknowledged,
    Cleared,
    Suppressed,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlcSignal {
    signal_id: u32,
    signal_type: PlcSignalType,
    address: u16,
    value: i32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlcAlarm {
    alarm_id: u32,
    severity: AlarmSeverity,
    state: AlarmState,
    message: String,
    triggered_at: u64,
    cleared_at: Option<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProductionLine {
    line_id: u32,
    name: String,
    signals: Vec<PlcSignal>,
    alarms: Vec<PlcAlarm>,
    production_count: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ScadaSnapshot {
    snapshot_id: u64,
    lines: Vec<ProductionLine>,
    total_alarms_active: u32,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn make_signal(
    id: u32,
    signal_type: PlcSignalType,
    address: u16,
    value: i32,
    ts: u64,
) -> PlcSignal {
    PlcSignal {
        signal_id: id,
        signal_type,
        address,
        value,
        timestamp_ms: ts,
    }
}

fn make_alarm(
    id: u32,
    severity: AlarmSeverity,
    state: AlarmState,
    msg: &str,
    triggered_at: u64,
    cleared_at: Option<u64>,
) -> PlcAlarm {
    PlcAlarm {
        alarm_id: id,
        severity,
        state,
        message: msg.to_string(),
        triggered_at,
        cleared_at,
    }
}

fn roundtrip_signal_type(val: PlcSignalType) -> PlcSignalType {
    let bytes = encode_to_vec(&val).expect("encode PlcSignalType");
    let (decoded, consumed): (PlcSignalType, usize) =
        decode_from_slice(&bytes).expect("decode PlcSignalType");
    assert_eq!(consumed, bytes.len());
    decoded
}

fn roundtrip_alarm_severity(val: AlarmSeverity) -> (AlarmSeverity, Vec<u8>) {
    let bytes = encode_to_vec(&val).expect("encode AlarmSeverity");
    let (decoded, _): (AlarmSeverity, usize) =
        decode_from_slice(&bytes).expect("decode AlarmSeverity");
    (decoded, bytes)
}

fn roundtrip_alarm_state(val: AlarmState) -> AlarmState {
    let bytes = encode_to_vec(&val).expect("encode AlarmState");
    let (decoded, consumed): (AlarmState, usize) =
        decode_from_slice(&bytes).expect("decode AlarmState");
    assert_eq!(consumed, bytes.len());
    decoded
}

// ─── Test 1: PlcSignalType digital variants (DigitalInput, DigitalOutput) ────

#[test]
fn test_plc_signal_type_digital_variants() {
    let di = roundtrip_signal_type(PlcSignalType::DigitalInput);
    assert_eq!(di, PlcSignalType::DigitalInput);

    let do_ = roundtrip_signal_type(PlcSignalType::DigitalOutput);
    assert_eq!(do_, PlcSignalType::DigitalOutput);

    // Verify distinct binary representations
    let di_bytes = encode_to_vec(&PlcSignalType::DigitalInput).expect("encode DI");
    let do_bytes = encode_to_vec(&PlcSignalType::DigitalOutput).expect("encode DO");
    assert_ne!(
        di_bytes, do_bytes,
        "DigitalInput and DigitalOutput must have distinct encodings"
    );
}

// ─── Test 2: PlcSignalType analog variants (AnalogInput, AnalogOutput) ───────

#[test]
fn test_plc_signal_type_analog_variants() {
    let ai = roundtrip_signal_type(PlcSignalType::AnalogInput);
    assert_eq!(ai, PlcSignalType::AnalogInput);

    let ao = roundtrip_signal_type(PlcSignalType::AnalogOutput);
    assert_eq!(ao, PlcSignalType::AnalogOutput);

    let ai_bytes = encode_to_vec(&PlcSignalType::AnalogInput).expect("encode AI");
    let ao_bytes = encode_to_vec(&PlcSignalType::AnalogOutput).expect("encode AO");
    assert_ne!(
        ai_bytes, ao_bytes,
        "AnalogInput and AnalogOutput must have distinct encodings"
    );
}

// ─── Test 3: PlcSignalType counter and timer variants ────────────────────────

#[test]
fn test_plc_signal_type_counter_and_timer() {
    let counter = roundtrip_signal_type(PlcSignalType::Counter);
    assert_eq!(counter, PlcSignalType::Counter);

    let timer = roundtrip_signal_type(PlcSignalType::Timer);
    assert_eq!(timer, PlcSignalType::Timer);

    let counter_bytes = encode_to_vec(&PlcSignalType::Counter).expect("encode Counter");
    let timer_bytes = encode_to_vec(&PlcSignalType::Timer).expect("encode Timer");
    assert_ne!(
        counter_bytes, timer_bytes,
        "Counter and Timer must have distinct encodings"
    );
}

// ─── Test 4: AlarmSeverity Info and Warning variants ─────────────────────────

#[test]
fn test_alarm_severity_info_and_warning() {
    let (info, info_bytes) = roundtrip_alarm_severity(AlarmSeverity::Info);
    assert_eq!(info, AlarmSeverity::Info);

    let (warning, warning_bytes) = roundtrip_alarm_severity(AlarmSeverity::Warning);
    assert_eq!(warning, AlarmSeverity::Warning);

    // Info and Warning must be distinct (ordering / discriminant check)
    assert_ne!(
        info_bytes, warning_bytes,
        "Info and Warning must differ in encoding"
    );
}

// ─── Test 5: AlarmSeverity Error and Critical variants ───────────────────────

#[test]
fn test_alarm_severity_error_and_critical() {
    let (error_sev, error_bytes) = roundtrip_alarm_severity(AlarmSeverity::Error);
    assert_eq!(error_sev, AlarmSeverity::Error);

    let (critical, critical_bytes) = roundtrip_alarm_severity(AlarmSeverity::Critical);
    assert_eq!(critical, AlarmSeverity::Critical);

    assert_ne!(
        error_bytes, critical_bytes,
        "Error and Critical must differ in encoding"
    );
}

// ─── Test 6: AlarmSeverity Emergency variant; also checks Info vs Emergency ──

#[test]
fn test_alarm_severity_emergency() {
    let (emergency, emergency_bytes) = roundtrip_alarm_severity(AlarmSeverity::Emergency);
    assert_eq!(emergency, AlarmSeverity::Emergency);

    // Distinct discriminants: Info (first) vs Emergency (last)
    let info_bytes =
        encode_to_vec(&AlarmSeverity::Info).expect("encode Info for discriminant check");
    assert_ne!(
        info_bytes, emergency_bytes,
        "Info and Emergency must have distinct discriminants"
    );

    // Verify all five severities produce distinct byte representations (ordering test)
    let all_severities: Vec<Vec<u8>> = [
        AlarmSeverity::Info,
        AlarmSeverity::Warning,
        AlarmSeverity::Error,
        AlarmSeverity::Critical,
        AlarmSeverity::Emergency,
    ]
    .iter()
    .map(|s| encode_to_vec(s).expect("encode severity for ordering check"))
    .collect();
    for i in 0..all_severities.len() {
        for j in (i + 1)..all_severities.len() {
            assert_ne!(
                all_severities[i], all_severities[j],
                "AlarmSeverity variants {i} and {j} must produce different encodings"
            );
        }
    }
}

// ─── Test 7: AlarmState Active and Acknowledged variants ─────────────────────

#[test]
fn test_alarm_state_active_and_acknowledged() {
    let active = roundtrip_alarm_state(AlarmState::Active);
    assert_eq!(active, AlarmState::Active);

    let acknowledged = roundtrip_alarm_state(AlarmState::Acknowledged);
    assert_eq!(acknowledged, AlarmState::Acknowledged);

    let active_bytes = encode_to_vec(&AlarmState::Active).expect("encode Active");
    let acknowledged_bytes = encode_to_vec(&AlarmState::Acknowledged).expect("encode Acknowledged");
    assert_ne!(
        active_bytes, acknowledged_bytes,
        "Active and Acknowledged must differ"
    );
}

// ─── Test 8: AlarmState Cleared and Suppressed variants ──────────────────────

#[test]
fn test_alarm_state_cleared_and_suppressed() {
    let cleared = roundtrip_alarm_state(AlarmState::Cleared);
    assert_eq!(cleared, AlarmState::Cleared);

    let suppressed = roundtrip_alarm_state(AlarmState::Suppressed);
    assert_eq!(suppressed, AlarmState::Suppressed);

    let cleared_bytes = encode_to_vec(&AlarmState::Cleared).expect("encode Cleared");
    let suppressed_bytes = encode_to_vec(&AlarmState::Suppressed).expect("encode Suppressed");
    assert_ne!(
        cleared_bytes, suppressed_bytes,
        "Cleared and Suppressed must differ"
    );
}

// ─── Test 9: PlcSignal full struct roundtrip (also checks address boundaries) ─

#[test]
fn test_plc_signal_roundtrip() {
    let signal = make_signal(
        101,
        PlcSignalType::AnalogInput,
        0x03E8,
        -512,
        1_700_000_000_000,
    );
    let bytes = encode_to_vec(&signal).expect("encode PlcSignal");
    let (decoded, consumed): (PlcSignal, usize) =
        decode_from_slice(&bytes).expect("decode PlcSignal");
    assert_eq!(signal, decoded);
    assert_eq!(consumed, bytes.len(), "all bytes must be consumed");

    // Boundary values for address (u16::MIN / u16::MAX) and value (i32::MIN / i32::MAX)
    let min_sig = make_signal(1000, PlcSignalType::DigitalInput, u16::MIN, i32::MIN, 0);
    let min_bytes = encode_to_vec(&min_sig).expect("encode min-boundary signal");
    let (decoded_min, _): (PlcSignal, usize) =
        decode_from_slice(&min_bytes).expect("decode min-boundary signal");
    assert_eq!(min_sig, decoded_min);
    assert_eq!(decoded_min.address, u16::MIN);
    assert_eq!(decoded_min.value, i32::MIN);

    let max_sig = make_signal(
        1001,
        PlcSignalType::DigitalOutput,
        u16::MAX,
        i32::MAX,
        u64::MAX / 2,
    );
    let max_bytes = encode_to_vec(&max_sig).expect("encode max-boundary signal");
    let (decoded_max, _): (PlcSignal, usize) =
        decode_from_slice(&max_bytes).expect("decode max-boundary signal");
    assert_eq!(max_sig, decoded_max);
    assert_eq!(decoded_max.address, u16::MAX);
    assert_eq!(decoded_max.value, i32::MAX);
}

// ─── Test 10: PlcAlarm with cleared_at = Some ────────────────────────────────

#[test]
fn test_plc_alarm_cleared_at_some() {
    let alarm = make_alarm(
        1,
        AlarmSeverity::Warning,
        AlarmState::Cleared,
        "Conveyor belt overspeed",
        1_700_000_001_000,
        Some(1_700_000_005_000),
    );
    let bytes = encode_to_vec(&alarm).expect("encode PlcAlarm cleared_at Some");
    let (decoded, consumed): (PlcAlarm, usize) =
        decode_from_slice(&bytes).expect("decode PlcAlarm cleared_at Some");
    assert_eq!(alarm, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(decoded.cleared_at.is_some(), "cleared_at must be Some");
    assert_eq!(decoded.cleared_at, Some(1_700_000_005_000));
}

// ─── Test 11: PlcAlarm with cleared_at = None ────────────────────────────────

#[test]
fn test_plc_alarm_cleared_at_none() {
    let alarm = make_alarm(
        2,
        AlarmSeverity::Critical,
        AlarmState::Active,
        "Motor temperature exceeded threshold",
        1_700_000_002_000,
        None,
    );
    let bytes = encode_to_vec(&alarm).expect("encode PlcAlarm cleared_at None");
    let (decoded, consumed): (PlcAlarm, usize) =
        decode_from_slice(&bytes).expect("decode PlcAlarm cleared_at None");
    assert_eq!(alarm, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(decoded.cleared_at.is_none(), "cleared_at must be None");
}

// ─── Test 12: ProductionLine empty (no signals, no alarms) ───────────────────

#[test]
fn test_production_line_empty() {
    let line = ProductionLine {
        line_id: 10,
        name: "Line-A (idle)".to_string(),
        signals: vec![],
        alarms: vec![],
        production_count: 0,
    };
    let bytes = encode_to_vec(&line).expect("encode empty ProductionLine");
    let (decoded, consumed): (ProductionLine, usize) =
        decode_from_slice(&bytes).expect("decode empty ProductionLine");
    assert_eq!(line, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(decoded.signals.is_empty());
    assert!(decoded.alarms.is_empty());
    assert_eq!(decoded.production_count, 0);
}

// ─── Test 13: ProductionLine with signals and alarms ─────────────────────────

#[test]
fn test_production_line_with_signals_and_alarms() {
    let signals = vec![
        make_signal(1, PlcSignalType::DigitalInput, 0x0001, 1, 1_700_000_000_000),
        make_signal(
            2,
            PlcSignalType::AnalogOutput,
            0x0100,
            4095,
            1_700_000_000_001,
        ),
        make_signal(3, PlcSignalType::Counter, 0x0200, 12345, 1_700_000_000_002),
    ];
    let alarms = vec![
        make_alarm(
            10,
            AlarmSeverity::Info,
            AlarmState::Cleared,
            "Startup complete",
            1_700_000_000_000,
            Some(1_700_000_000_500),
        ),
        make_alarm(
            11,
            AlarmSeverity::Warning,
            AlarmState::Active,
            "Pressure high",
            1_700_000_001_000,
            None,
        ),
    ];
    let line = ProductionLine {
        line_id: 20,
        name: "Assembly-Line-B".to_string(),
        signals,
        alarms,
        production_count: 98_765,
    };
    let bytes = encode_to_vec(&line).expect("encode ProductionLine with data");
    let (decoded, consumed): (ProductionLine, usize) =
        decode_from_slice(&bytes).expect("decode ProductionLine with data");
    assert_eq!(line, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.signals.len(), 3);
    assert_eq!(decoded.alarms.len(), 2);
    assert_eq!(decoded.production_count, 98_765);
}

// ─── Test 14: ScadaSnapshot full roundtrip ───────────────────────────────────

#[test]
fn test_scada_snapshot_roundtrip() {
    let line = ProductionLine {
        line_id: 1,
        name: "SCADA-Line-1".to_string(),
        signals: vec![make_signal(
            200,
            PlcSignalType::DigitalOutput,
            0x0010,
            0,
            1_700_100_000_000,
        )],
        alarms: vec![make_alarm(
            300,
            AlarmSeverity::Error,
            AlarmState::Acknowledged,
            "Valve actuator failure",
            1_700_100_001_000,
            None,
        )],
        production_count: 4_200,
    };
    let snapshot = ScadaSnapshot {
        snapshot_id: 9_001_001,
        lines: vec![line],
        total_alarms_active: 1,
    };
    let bytes = encode_to_vec(&snapshot).expect("encode ScadaSnapshot");
    let (decoded, consumed): (ScadaSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode ScadaSnapshot");
    assert_eq!(snapshot, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.total_alarms_active, 1);
    assert_eq!(decoded.lines.len(), 1);
}

// ─── Test 15: Big-endian config roundtrip ────────────────────────────────────

#[test]
fn test_big_endian_config() {
    let signal = make_signal(50, PlcSignalType::Timer, 0xFFFF, 99, 1_700_200_000_000);
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&signal, cfg).expect("encode big_endian");
    let (decoded, consumed): (PlcSignal, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big_endian");
    assert_eq!(signal, decoded);
    assert_eq!(consumed, bytes.len());

    // Big-endian should produce different bytes than default little-endian for the same multi-byte value
    let le_bytes = encode_to_vec(&signal).expect("encode little_endian for comparison");
    assert_ne!(
        bytes, le_bytes,
        "big-endian and little-endian must differ for non-trivial values"
    );
}

// ─── Test 16: Fixed-int config roundtrip ─────────────────────────────────────

#[test]
fn test_fixed_int_config() {
    let alarm = make_alarm(
        99,
        AlarmSeverity::Critical,
        AlarmState::Suppressed,
        "Emergency stop triggered",
        1_700_300_000_000,
        None,
    );
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&alarm, cfg).expect("encode fixed_int");
    let (decoded, consumed): (PlcAlarm, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed_int");
    assert_eq!(alarm, decoded);
    assert_eq!(consumed, bytes.len());
}

// ─── Test 17: Big-endian + fixed-int config roundtrip ────────────────────────

#[test]
fn test_big_endian_fixed_int_config() {
    let line = ProductionLine {
        line_id: 77,
        name: "BE-Fixed-Line".to_string(),
        signals: vec![make_signal(
            5,
            PlcSignalType::DigitalInput,
            0x00FF,
            1,
            1_700_400_000_000,
        )],
        alarms: vec![],
        production_count: u64::MAX, // also validates production_count at u64::MAX
    };
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&line, cfg).expect("encode BE+fixed");
    let (decoded, consumed): (ProductionLine, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode BE+fixed");
    assert_eq!(line, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(
        decoded.production_count,
        u64::MAX,
        "u64::MAX production count must survive BE+fixed config roundtrip"
    );
}

// ─── Test 18: Consumed bytes equals full encoded length ──────────────────────

#[test]
fn test_consumed_bytes_check() {
    let signal = make_signal(7, PlcSignalType::AnalogInput, 0x0002, 42, 999_999_999);
    let bytes = encode_to_vec(&signal).expect("encode for consumed bytes check");
    assert!(!bytes.is_empty(), "encoded bytes must not be empty");

    let (_, consumed): (PlcSignal, usize) =
        decode_from_slice(&bytes).expect("decode for consumed bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed ({consumed}) must equal total encoded length ({})",
        bytes.len()
    );

    // Also verify with legacy (fixed-int) config
    let cfg_legacy = config::legacy();
    let legacy_bytes =
        encode_to_vec_with_config(&signal, cfg_legacy).expect("encode legacy for consumed check");
    let (_, legacy_consumed): (PlcSignal, usize) =
        decode_from_slice_with_config(&legacy_bytes, cfg_legacy)
            .expect("decode legacy for consumed check");
    assert_eq!(
        legacy_consumed,
        legacy_bytes.len(),
        "legacy config consumed must equal encoded length"
    );
}

// ─── Test 19: Vec<PlcSignal> roundtrip ───────────────────────────────────────

#[test]
fn test_vec_plc_signal_roundtrip() {
    let signals: Vec<PlcSignal> = vec![
        make_signal(1, PlcSignalType::DigitalInput, 0x0001, 1, 1_000_000),
        make_signal(2, PlcSignalType::DigitalOutput, 0x0002, 0, 2_000_000),
        make_signal(3, PlcSignalType::AnalogInput, 0x0010, 2048, 3_000_000),
        make_signal(4, PlcSignalType::AnalogOutput, 0x0020, -100, 4_000_000),
        make_signal(5, PlcSignalType::Counter, 0x0030, 9999, 5_000_000),
        make_signal(6, PlcSignalType::Timer, 0x0040, 300, 6_000_000),
    ];
    let bytes = encode_to_vec(&signals).expect("encode Vec<PlcSignal>");
    let (decoded, consumed): (Vec<PlcSignal>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<PlcSignal>");
    assert_eq!(signals, decoded);
    assert_eq!(decoded.len(), 6);
    assert_eq!(consumed, bytes.len());
}

// ─── Test 20: Vec<PlcAlarm> roundtrip ────────────────────────────────────────

#[test]
fn test_vec_plc_alarm_roundtrip() {
    let alarms: Vec<PlcAlarm> = vec![
        make_alarm(
            1,
            AlarmSeverity::Info,
            AlarmState::Cleared,
            "Startup",
            1_000,
            Some(2_000),
        ),
        make_alarm(
            2,
            AlarmSeverity::Warning,
            AlarmState::Active,
            "Low pressure",
            3_000,
            None,
        ),
        make_alarm(
            3,
            AlarmSeverity::Error,
            AlarmState::Acknowledged,
            "Sensor fault",
            4_000,
            None,
        ),
        make_alarm(
            4,
            AlarmSeverity::Critical,
            AlarmState::Suppressed,
            "Overheat",
            5_000,
            None,
        ),
        make_alarm(
            5,
            AlarmSeverity::Emergency,
            AlarmState::Active,
            "Fire detected",
            6_000,
            None,
        ),
    ];
    let bytes = encode_to_vec(&alarms).expect("encode Vec<PlcAlarm>");
    let (decoded, consumed): (Vec<PlcAlarm>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<PlcAlarm>");
    assert_eq!(alarms, decoded);
    assert_eq!(decoded.len(), 5);
    assert_eq!(consumed, bytes.len());
}

// ─── Test 21: Large production line with 50 signals ──────────────────────────

#[test]
fn test_large_production_line_50_signals() {
    let signals: Vec<PlcSignal> = (0u32..50)
        .map(|i| {
            let signal_type = match i % 6 {
                0 => PlcSignalType::DigitalInput,
                1 => PlcSignalType::DigitalOutput,
                2 => PlcSignalType::AnalogInput,
                3 => PlcSignalType::AnalogOutput,
                4 => PlcSignalType::Counter,
                _ => PlcSignalType::Timer,
            };
            make_signal(
                i,
                signal_type,
                (i * 4) as u16,
                (i as i32) * 10 - 200,
                1_700_000_000_000 + i as u64,
            )
        })
        .collect();
    let line = ProductionLine {
        line_id: 99,
        name: "Large-PLC-Line-Industrial-Sector-3".to_string(),
        signals,
        alarms: vec![],
        production_count: 1_000_000,
    };
    let bytes = encode_to_vec(&line).expect("encode large ProductionLine");
    let (decoded, consumed): (ProductionLine, usize) =
        decode_from_slice(&bytes).expect("decode large ProductionLine");
    assert_eq!(line, decoded);
    assert_eq!(decoded.signals.len(), 50);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.production_count, 1_000_000);
}

// ─── Test 22: Emergency alarm roundtrip ──────────────────────────────────────

#[test]
fn test_emergency_alarm_roundtrip() {
    let alarm = make_alarm(
        9999,
        AlarmSeverity::Emergency,
        AlarmState::Active,
        "CRITICAL: Reactor coolant pump failure — SCRAM initiated",
        1_700_999_000_000,
        None,
    );
    let bytes = encode_to_vec(&alarm).expect("encode Emergency alarm");
    let (decoded, consumed): (PlcAlarm, usize) =
        decode_from_slice(&bytes).expect("decode Emergency alarm");
    assert_eq!(alarm, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.severity, AlarmSeverity::Emergency);
    assert_eq!(decoded.state, AlarmState::Active);
    assert!(
        decoded.cleared_at.is_none(),
        "active emergency alarm must not have cleared_at"
    );
    assert_eq!(decoded.alarm_id, 9999);

    // Verify Emergency encodes differently from Info (distinct discriminants)
    let info_bytes =
        encode_to_vec(&AlarmSeverity::Info).expect("encode Info for discriminant comparison");
    let emergency_bytes = encode_to_vec(&AlarmSeverity::Emergency)
        .expect("encode Emergency for discriminant comparison");
    assert_ne!(
        info_bytes, emergency_bytes,
        "Info and Emergency must have distinct discriminants"
    );
}
