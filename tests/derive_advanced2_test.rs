//! Advanced derive macro feature tests for OxiCode (set 2)

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

// Struct with multiple fields
#[derive(Debug, PartialEq, Encode, Decode)]
struct Sensor {
    id: u32,
    name: String,
    value: f64,
    unit: String,
    active: bool,
}

// Enum with various variant types
#[derive(Debug, PartialEq, Encode, Decode)]
enum Signal {
    None,
    Integer(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
    Pair(u32, String),
    Named { key: String, val: Vec<u8> },
}

// Generic struct
#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<A, B> {
    first: A,
    second: B,
}

// Nested struct
#[derive(Debug, PartialEq, Encode, Decode)]
struct Reading {
    sensor: Sensor,
    signal: Signal,
    timestamp: u64,
}

// --- Test 1: Sensor roundtrip ---
#[test]
fn test_sensor_roundtrip() {
    let sensor = Sensor {
        id: 42,
        name: String::from("TempSensor"),
        value: 36.6,
        unit: String::from("Celsius"),
        active: true,
    };
    let encoded = encode_to_vec(&sensor).expect("Failed to encode Sensor");
    let (decoded, _): (Sensor, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Sensor");
    assert_eq!(sensor, decoded);
}

// --- Test 2: Signal::None roundtrip ---
#[test]
fn test_signal_none_roundtrip() {
    let signal = Signal::None;
    let encoded = encode_to_vec(&signal).expect("Failed to encode Signal::None");
    let (decoded, _): (Signal, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Signal::None");
    assert_eq!(signal, decoded);
}

// --- Test 3: Signal::Integer roundtrip ---
#[test]
fn test_signal_integer_roundtrip() {
    let signal = Signal::Integer(-987654321_i64);
    let encoded = encode_to_vec(&signal).expect("Failed to encode Signal::Integer");
    let (decoded, _): (Signal, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Signal::Integer");
    assert_eq!(signal, decoded);
}

// --- Test 4: Signal::Float roundtrip ---
#[test]
fn test_signal_float_roundtrip() {
    let signal = Signal::Float(3.141592653589793);
    let encoded = encode_to_vec(&signal).expect("Failed to encode Signal::Float");
    let (decoded, _): (Signal, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Signal::Float");
    assert_eq!(signal, decoded);
}

// --- Test 5: Signal::Text roundtrip ---
#[test]
fn test_signal_text_roundtrip() {
    let signal = Signal::Text(String::from("Hello, OxiCode!"));
    let encoded = encode_to_vec(&signal).expect("Failed to encode Signal::Text");
    let (decoded, _): (Signal, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Signal::Text");
    assert_eq!(signal, decoded);
}

// --- Test 6: Signal::Bytes roundtrip ---
#[test]
fn test_signal_bytes_roundtrip() {
    let signal = Signal::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF]);
    let encoded = encode_to_vec(&signal).expect("Failed to encode Signal::Bytes");
    let (decoded, _): (Signal, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Signal::Bytes");
    assert_eq!(signal, decoded);
}

// --- Test 7: Signal::Pair roundtrip ---
#[test]
fn test_signal_pair_roundtrip() {
    let signal = Signal::Pair(100, String::from("pair-value"));
    let encoded = encode_to_vec(&signal).expect("Failed to encode Signal::Pair");
    let (decoded, _): (Signal, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Signal::Pair");
    assert_eq!(signal, decoded);
}

// --- Test 8: Signal::Named roundtrip ---
#[test]
fn test_signal_named_roundtrip() {
    let signal = Signal::Named {
        key: String::from("config-key"),
        val: vec![1, 2, 3, 4, 5],
    };
    let encoded = encode_to_vec(&signal).expect("Failed to encode Signal::Named");
    let (decoded, _): (Signal, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Signal::Named");
    assert_eq!(signal, decoded);
}

// --- Test 9: Reading nested struct roundtrip ---
#[test]
fn test_reading_nested_roundtrip() {
    let reading = Reading {
        sensor: Sensor {
            id: 7,
            name: String::from("PressureSensor"),
            value: 101.325,
            unit: String::from("kPa"),
            active: false,
        },
        signal: Signal::Float(101.325),
        timestamp: 1_700_000_000_u64,
    };
    let encoded = encode_to_vec(&reading).expect("Failed to encode Reading");
    let (decoded, _): (Reading, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Reading");
    assert_eq!(reading, decoded);
}

// --- Test 10: Pair<u32, String> roundtrip ---
#[test]
fn test_pair_u32_string_roundtrip() {
    let pair = Pair {
        first: 9999_u32,
        second: String::from("rust-pair"),
    };
    let encoded = encode_to_vec(&pair).expect("Failed to encode Pair<u32, String>");
    let (decoded, _): (Pair<u32, String>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Pair<u32, String>");
    assert_eq!(pair, decoded);
}

// --- Test 11: Pair<String, Vec<u8>> roundtrip ---
#[test]
fn test_pair_string_vec_u8_roundtrip() {
    let pair = Pair {
        first: String::from("data-key"),
        second: vec![10_u8, 20, 30, 40, 50],
    };
    let encoded = encode_to_vec(&pair).expect("Failed to encode Pair<String, Vec<u8>>");
    let (decoded, _): (Pair<String, Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Pair<String, Vec<u8>>");
    assert_eq!(pair, decoded);
}

// --- Test 12: Pair<bool, f64> roundtrip ---
#[test]
fn test_pair_bool_f64_roundtrip() {
    let pair = Pair {
        first: true,
        second: 2.718281828459045_f64,
    };
    let encoded = encode_to_vec(&pair).expect("Failed to encode Pair<bool, f64>");
    let (decoded, _): (Pair<bool, f64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Pair<bool, f64>");
    assert_eq!(pair, decoded);
}

// --- Test 13: Vec<Sensor> roundtrip (3 items) ---
#[test]
fn test_vec_sensor_roundtrip() {
    let sensors = vec![
        Sensor {
            id: 1,
            name: String::from("Alpha"),
            value: 1.1,
            unit: String::from("m"),
            active: true,
        },
        Sensor {
            id: 2,
            name: String::from("Beta"),
            value: 2.2,
            unit: String::from("kg"),
            active: false,
        },
        Sensor {
            id: 3,
            name: String::from("Gamma"),
            value: 3.3,
            unit: String::from("s"),
            active: true,
        },
    ];
    let encoded = encode_to_vec(&sensors).expect("Failed to encode Vec<Sensor>");
    let (decoded, _): (Vec<Sensor>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Sensor>");
    assert_eq!(sensors, decoded);
}

// --- Test 14: Vec<Signal> all variants roundtrip (7 items) ---
#[test]
fn test_vec_signal_all_variants_roundtrip() {
    let signals = vec![
        Signal::None,
        Signal::Integer(42),
        Signal::Float(1.5),
        Signal::Text(String::from("text")),
        Signal::Bytes(vec![0xAB, 0xCD]),
        Signal::Pair(7, String::from("p")),
        Signal::Named {
            key: String::from("k"),
            val: vec![0xFF],
        },
    ];
    let encoded = encode_to_vec(&signals).expect("Failed to encode Vec<Signal>");
    let (decoded, _): (Vec<Signal>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Signal>");
    assert_eq!(signals, decoded);
}

// --- Test 15: Vec<Reading> roundtrip (2 items) ---
#[test]
fn test_vec_reading_roundtrip() {
    let readings = vec![
        Reading {
            sensor: Sensor {
                id: 10,
                name: String::from("S1"),
                value: 0.0,
                unit: String::from("V"),
                active: true,
            },
            signal: Signal::Integer(0),
            timestamp: 1000,
        },
        Reading {
            sensor: Sensor {
                id: 20,
                name: String::from("S2"),
                value: 5.0,
                unit: String::from("A"),
                active: false,
            },
            signal: Signal::Float(5.0),
            timestamp: 2000,
        },
    ];
    let encoded = encode_to_vec(&readings).expect("Failed to encode Vec<Reading>");
    let (decoded, _): (Vec<Reading>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Reading>");
    assert_eq!(readings, decoded);
}

// --- Test 16: Option<Sensor> Some roundtrip ---
#[test]
fn test_option_sensor_some_roundtrip() {
    let opt_sensor: Option<Sensor> = Some(Sensor {
        id: 99,
        name: String::from("OptSensor"),
        value: -273.15,
        unit: String::from("K"),
        active: false,
    });
    let encoded = encode_to_vec(&opt_sensor).expect("Failed to encode Option<Sensor> Some");
    let (decoded, _): (Option<Sensor>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<Sensor> Some");
    assert_eq!(opt_sensor, decoded);
}

// --- Test 17: Option<Signal> None roundtrip ---
#[test]
fn test_option_signal_none_roundtrip() {
    let opt_signal: Option<Signal> = Option::None;
    let encoded = encode_to_vec(&opt_signal).expect("Failed to encode Option<Signal> None");
    let (decoded, _): (Option<Signal>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<Signal> None");
    assert_eq!(opt_signal, decoded);
}

// --- Test 18: Sensor with fixed-int config ---
#[test]
fn test_sensor_with_fixed_int_config() {
    let sensor = Sensor {
        id: 255,
        name: String::from("FixedSensor"),
        value: 100.0,
        unit: String::from("psi"),
        active: true,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&sensor, cfg)
        .expect("Failed to encode Sensor with fixed-int config");
    let (decoded, _): (Sensor, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Sensor with fixed-int config");
    assert_eq!(sensor, decoded);
}

// --- Test 19: Consumed bytes equals encoded length for Reading ---
#[test]
fn test_reading_consumed_bytes_equals_encoded_length() {
    let reading = Reading {
        sensor: Sensor {
            id: 5,
            name: String::from("Len"),
            value: 9.9,
            unit: String::from("Hz"),
            active: true,
        },
        signal: Signal::Text(String::from("length-check")),
        timestamp: 999_999_u64,
    };
    let encoded = encode_to_vec(&reading).expect("Failed to encode Reading for length check");
    let (_decoded, consumed): (Reading, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Reading for length check");
    assert_eq!(
        consumed,
        encoded.len(),
        "Consumed bytes must equal encoded slice length"
    );
}

// --- Test 20: Two equal Sensors produce same bytes ---
#[test]
fn test_equal_sensors_produce_same_bytes() {
    let sensor_a = Sensor {
        id: 1,
        name: String::from("Eq"),
        value: 1.0,
        unit: String::from("N"),
        active: true,
    };
    let sensor_b = Sensor {
        id: 1,
        name: String::from("Eq"),
        value: 1.0,
        unit: String::from("N"),
        active: true,
    };
    let bytes_a = encode_to_vec(&sensor_a).expect("Failed to encode sensor_a");
    let bytes_b = encode_to_vec(&sensor_b).expect("Failed to encode sensor_b");
    assert_eq!(
        bytes_a, bytes_b,
        "Equal Sensors must produce identical encoded bytes"
    );
}

// --- Test 21: Different Sensors produce different bytes ---
#[test]
fn test_different_sensors_produce_different_bytes() {
    let sensor_x = Sensor {
        id: 1,
        name: String::from("X"),
        value: 1.0,
        unit: String::from("m"),
        active: true,
    };
    let sensor_y = Sensor {
        id: 2,
        name: String::from("Y"),
        value: 2.0,
        unit: String::from("s"),
        active: false,
    };
    let bytes_x = encode_to_vec(&sensor_x).expect("Failed to encode sensor_x");
    let bytes_y = encode_to_vec(&sensor_y).expect("Failed to encode sensor_y");
    assert_ne!(
        bytes_x, bytes_y,
        "Different Sensors must produce distinct encoded bytes"
    );
}

// --- Test 22: Vec<Pair<u32, u32>> roundtrip (5 items) ---
#[test]
fn test_vec_pair_u32_u32_roundtrip() {
    let pairs: Vec<Pair<u32, u32>> = vec![
        Pair {
            first: 0,
            second: 0,
        },
        Pair {
            first: 1,
            second: 100,
        },
        Pair {
            first: 42,
            second: 42,
        },
        Pair {
            first: 999,
            second: 1,
        },
        Pair {
            first: u32::MAX,
            second: u32::MIN,
        },
    ];
    let encoded = encode_to_vec(&pairs).expect("Failed to encode Vec<Pair<u32, u32>>");
    let (decoded, _): (Vec<Pair<u32, u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Pair<u32, u32>>");
    assert_eq!(pairs, decoded);
}
