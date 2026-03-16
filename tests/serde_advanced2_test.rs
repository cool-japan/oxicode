//! Advanced serde integration tests (set 2) for OxiCode.
//!
//! All 22 tests are individually gated on `#[cfg(feature = "serde")]`.
//! No `#[cfg(test)]` module wrapper — all tests are top-level.
//! No `unwrap()` — every fallible call uses `.expect("…")`.
//!
//! Coverage angles are distinct from `serde_integration_test.rs`,
//! `serde_complex_test.rs`, `serde_advanced_test.rs`, and
//! `serde_owned_test.rs`.

// ---------------------------------------------------------------------------
// Test 1: Serde-derived struct with #[serde(rename)] roundtrip
//         (top-level per-field rename — different struct than serde_complex
//          test 3 which uses "user_id"/"display_name"/"account_active")
// ---------------------------------------------------------------------------

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
#[cfg(feature = "serde")]
#[test]
fn test_adv2_01_rename_field_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Sensor {
        #[serde(rename = "sensor_id")]
        id: u32,
        #[serde(rename = "celsius_value")]
        temperature: f64,
        #[serde(rename = "is_online")]
        online: bool,
    }

    let original = Sensor {
        id: 7,
        temperature: 36.6,
        online: true,
    };
    let bytes = encode_serde(&original).expect("encode renamed Sensor");
    let decoded: Sensor = decode_serde(&bytes).expect("decode renamed Sensor");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Serde-derived struct with #[serde(skip)] field roundtrip
//         Verifies that the skipped field reverts to Default, not the
//         encoded value. Uses a distinct struct from serde_complex test 4.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_02_skip_field_reverts_to_default() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Transaction {
        amount: u64,
        currency: String,
        #[serde(skip)]
        session_token: String, // must not appear in bytes; defaults to ""
    }

    let original = Transaction {
        amount: 1_000_000,
        currency: "USD".to_string(),
        session_token: "secret-xyz".to_string(),
    };
    let bytes = encode_serde(&original).expect("encode Transaction with skip");
    let decoded: Transaction = decode_serde(&bytes).expect("decode Transaction with skip");

    assert_eq!(decoded.amount, original.amount);
    assert_eq!(decoded.currency, original.currency);
    assert_eq!(
        decoded.session_token, "",
        "skipped field must default to empty string"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Serde-derived struct with #[serde(default)] roundtrip
//         Uses a custom default function; explicit values must survive.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_03_default_field_explicit_value_survives() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    fn default_port() -> u16 {
        8080
    }

    fn default_host() -> String {
        "localhost".to_string()
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ServerConfig {
        name: String,
        #[serde(default = "default_port")]
        port: u16,
        #[serde(default = "default_host")]
        host: String,
    }

    let original = ServerConfig {
        name: "api".to_string(),
        port: 443,
        host: "example.com".to_string(),
    };
    let bytes = encode_serde(&original).expect("encode ServerConfig with default");
    let decoded: ServerConfig = decode_serde(&bytes).expect("decode ServerConfig with default");
    assert_eq!(original, decoded);
    assert_eq!(decoded.port, 443, "explicit port must override default");
    assert_eq!(
        decoded.host, "example.com",
        "explicit host must override default"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Serde-derived enum roundtrip covering unit, tuple, and struct
//         variants in a single type distinct from existing enum tests.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_04_enum_all_variant_kinds_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Command {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    let variants = [
        Command::Quit,
        Command::Move { x: -10, y: 20 },
        Command::Write("hello oxicode".to_string()),
        Command::ChangeColor(255, 128, 0),
    ];
    for variant in &variants {
        let bytes = encode_serde(variant).expect("encode Command variant");
        let decoded: Command = decode_serde(&bytes).expect("decode Command variant");
        assert_eq!(variant, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 5: Serde-derived newtype struct roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_05_newtype_struct_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Meters(f64);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct UserId(u64);

    let dist = Meters(99.5);
    let bytes = encode_serde(&dist).expect("encode Meters newtype");
    let decoded: Meters = decode_serde(&bytes).expect("decode Meters newtype");
    assert_eq!(dist, decoded);

    let uid = UserId(u64::MAX / 2);
    let bytes = encode_serde(&uid).expect("encode UserId newtype");
    let decoded_uid: UserId = decode_serde(&bytes).expect("decode UserId newtype");
    assert_eq!(uid, decoded_uid);
}

// ---------------------------------------------------------------------------
// Test 6: Serde-derived tuple struct roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_06_tuple_struct_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct RgbColor(u8, u8, u8);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Point3D(f32, f32, f32);

    let color = RgbColor(10, 200, 30);
    let bytes = encode_serde(&color).expect("encode RgbColor tuple struct");
    let decoded: RgbColor = decode_serde(&bytes).expect("decode RgbColor tuple struct");
    assert_eq!(color, decoded);

    let point = Point3D(-1.5, 0.0, 42.0);
    let bytes = encode_serde(&point).expect("encode Point3D tuple struct");
    let decoded_point: Point3D = decode_serde(&bytes).expect("decode Point3D tuple struct");
    assert_eq!(point, decoded_point);
}

// ---------------------------------------------------------------------------
// Test 7: Serde-derived unit struct roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_07_unit_struct_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Marker;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Empty;

    let marker = Marker;
    let bytes = encode_serde(&marker).expect("encode unit struct Marker");
    let decoded: Marker = decode_serde(&bytes).expect("decode unit struct Marker");
    assert_eq!(marker, decoded);

    let empty = Empty;
    let bytes = encode_serde(&empty).expect("encode unit struct Empty");
    let decoded_empty: Empty = decode_serde(&bytes).expect("decode unit struct Empty");
    assert_eq!(empty, decoded_empty);
}

// ---------------------------------------------------------------------------
// Test 8: Serde struct with nested serde structs (three-level nesting)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_08_three_level_nested_structs() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Coordinate {
        lat: f64,
        lon: f64,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Location {
        name: String,
        coord: Coordinate,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Route {
        id: u32,
        origin: Location,
        destination: Location,
    }

    let original = Route {
        id: 1,
        origin: Location {
            name: "Tokyo".to_string(),
            coord: Coordinate {
                lat: 35.6762,
                lon: 139.6503,
            },
        },
        destination: Location {
            name: "Osaka".to_string(),
            coord: Coordinate {
                lat: 34.6937,
                lon: 135.5023,
            },
        },
    };
    let bytes = encode_serde(&original).expect("encode Route with 3-level nesting");
    let decoded: Route = decode_serde(&bytes).expect("decode Route with 3-level nesting");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Serde struct with Vec<SerdeStruct> field
//         (Vec of a non-trivial struct — distinct from serde_complex test 6)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_09_vec_of_nested_structs_field() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Measurement {
        timestamp: u64,
        value: f64,
        unit: String,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TimeSeries {
        sensor_id: u32,
        readings: Vec<Measurement>,
    }

    let original = TimeSeries {
        sensor_id: 42,
        readings: vec![
            Measurement {
                timestamp: 1_700_000_000,
                value: 23.1,
                unit: "celsius".to_string(),
            },
            Measurement {
                timestamp: 1_700_000_060,
                value: 23.5,
                unit: "celsius".to_string(),
            },
            Measurement {
                timestamp: 1_700_000_120,
                value: 22.9,
                unit: "celsius".to_string(),
            },
        ],
    };
    let bytes = encode_serde(&original).expect("encode TimeSeries");
    let decoded: TimeSeries = decode_serde(&bytes).expect("decode TimeSeries");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Serde struct with HashMap<String, T>
//          (HashMap — not BTreeMap, using encode_serde convenience fn)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_10_hashmap_string_to_bool_field() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct FeatureFlags {
        version: u32,
        flags: HashMap<String, bool>,
    }

    let mut flags = HashMap::new();
    flags.insert("dark_mode".to_string(), true);
    flags.insert("beta_ui".to_string(), false);
    flags.insert("experimental".to_string(), true);

    let original = FeatureFlags { version: 3, flags };
    let bytes = encode_serde(&original).expect("encode FeatureFlags");
    let decoded: FeatureFlags = decode_serde(&bytes).expect("decode FeatureFlags");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Serde struct with Option<SerdeStruct> — Some and None branches
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_11_option_nested_struct_some_and_none() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Attachment {
        filename: String,
        size_bytes: u64,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Message {
        body: String,
        attachment: Option<Attachment>,
    }

    // Some branch
    let with_att = Message {
        body: "See attached".to_string(),
        attachment: Some(Attachment {
            filename: "report.pdf".to_string(),
            size_bytes: 204_800,
        }),
    };
    let bytes = encode_serde(&with_att).expect("encode Message with attachment");
    let decoded: Message = decode_serde(&bytes).expect("decode Message with attachment");
    assert_eq!(with_att, decoded);

    // None branch
    let without_att = Message {
        body: "No attachment here".to_string(),
        attachment: None,
    };
    let bytes = encode_serde(&without_att).expect("encode Message without attachment");
    let decoded: Message = decode_serde(&bytes).expect("decode Message without attachment");
    assert_eq!(without_att, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Serde roundtrip with big-endian + fixed-int config
//          Uses encode_serde_with_config / decode_serde_with_config.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_12_big_endian_fixed_int_config_roundtrip() {
    use oxicode::serde::{decode_serde_with_config, encode_serde_with_config};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Frame {
        opcode: u8,
        length: u32,
        checksum: u64,
    }

    let cfg = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let original = Frame {
        opcode: 0xAB,
        length: 1024,
        checksum: 0xDEAD_BEEF_CAFE_BABE,
    };
    let bytes =
        encode_serde_with_config(&original, cfg).expect("encode Frame big-endian fixed-int");
    let decoded: Frame =
        decode_serde_with_config(&bytes, cfg).expect("decode Frame big-endian fixed-int");
    assert_eq!(original, decoded);

    // 1 (u8) + 4 (u32 fixed) + 8 (u64 fixed) = 13 bytes
    assert_eq!(
        bytes.len(),
        13,
        "big-endian fixed-int Frame must encode to 13 bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Serde roundtrip with fixed-int config alone
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_13_fixed_int_only_config_roundtrip() {
    use oxicode::serde::{decode_serde_with_config, encode_serde_with_config};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Dimensions {
        width: u16,
        height: u16,
        depth: u32,
    }

    let cfg = oxicode::config::standard().with_fixed_int_encoding();

    let original = Dimensions {
        width: 1920,
        height: 1080,
        depth: 60,
    };
    let bytes = encode_serde_with_config(&original, cfg).expect("encode Dimensions with fixed-int");
    let decoded: Dimensions =
        decode_serde_with_config(&bytes, cfg).expect("decode Dimensions with fixed-int");
    assert_eq!(original, decoded);

    // 2 (u16) + 2 (u16) + 4 (u32) = 8 bytes
    assert_eq!(
        bytes.len(),
        8,
        "fixed-int Dimensions must encode to 8 bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 14: encode_serde then decode as a compatible type
//          A newtype wrapping u64 encodes identically to u64 in serde.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_14_compatible_type_cross_decode() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Millis(u64);

    // Encode a u64 value and decode it as Millis (transparent newtype).
    let raw: u64 = 9_876_543_210;
    let bytes = encode_serde(&raw).expect("encode u64");
    let decoded: Millis = decode_serde(&bytes).expect("decode as Millis newtype");
    assert_eq!(
        decoded.0, raw,
        "Millis newtype must decode the same bits as u64"
    );

    // Reverse: encode Millis, decode as u64.
    let m = Millis(42_000);
    let bytes2 = encode_serde(&m).expect("encode Millis");
    let decoded_u64: u64 = decode_serde(&bytes2).expect("decode Millis as u64");
    assert_eq!(decoded_u64, m.0);
}

// ---------------------------------------------------------------------------
// Test 15: Serde struct with all primitive field types
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_15_all_primitive_fields() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct AllPrimitives {
        b: bool,
        i8v: i8,
        i16v: i16,
        i32v: i32,
        i64v: i64,
        u8v: u8,
        u16v: u16,
        u32v: u32,
        u64v: u64,
        f32v: f32,
        f64v: f64,
        cv: char,
    }

    let original = AllPrimitives {
        b: true,
        i8v: i8::MIN,
        i16v: i16::MAX,
        i32v: -1_234_567,
        i64v: i64::MIN,
        u8v: u8::MAX,
        u16v: u16::MAX,
        u32v: u32::MAX,
        u64v: u64::MAX,
        f32v: core::f32::consts::PI,
        f64v: core::f64::consts::TAU,
        cv: '日',
    };
    let bytes = encode_serde(&original).expect("encode AllPrimitives");
    let decoded: AllPrimitives = decode_serde(&bytes).expect("decode AllPrimitives");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Serde enum with data variants (mixed unit / newtype / tuple /
//          struct variants in a single enum)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_16_enum_data_variants_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Expr {
        Num(f64),
        Add { lhs: Box<Expr>, rhs: Box<Expr> },
        Neg(Box<Expr>),
        Zero,
    }

    let ast = Expr::Add {
        lhs: Box::new(Expr::Num(3.0)),
        rhs: Box::new(Expr::Neg(Box::new(Expr::Num(1.0)))),
    };
    let bytes = encode_serde(&ast).expect("encode Expr AST");
    let decoded: Expr = decode_serde(&bytes).expect("decode Expr AST");
    assert_eq!(ast, decoded);

    let zero = Expr::Zero;
    let bytes_zero = encode_serde(&zero).expect("encode Expr::Zero");
    let decoded_zero: Expr = decode_serde(&bytes_zero).expect("decode Expr::Zero");
    assert_eq!(zero, decoded_zero);
}

// ---------------------------------------------------------------------------
// Test 17: encode_serde + decode_from_slice roundtrip
//          Mixes the convenience encode_serde with the lower-level
//          decode_from_slice (which supports borrowed lifetimes).
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_17_encode_serde_then_decode_from_slice() {
    use oxicode::serde::{decode_from_slice, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Sample {
        id: u32,
        label: String,
        scores: Vec<f32>,
    }

    let original = Sample {
        id: 99,
        label: "experiment-7".to_string(),
        scores: vec![0.1, 0.5, 0.9, -1.0],
    };
    let bytes = encode_serde(&original).expect("encode_serde Sample");
    let (decoded, consumed): (Sample, usize) =
        decode_from_slice(&bytes, oxicode::config::standard()).expect("decode_from_slice Sample");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18: encode_to_vec + decode_serde roundtrip
//          Mixes the lower-level encode_to_vec with the convenience
//          decode_serde (discards bytes-consumed count).
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_18_encode_to_vec_then_decode_serde() {
    use oxicode::serde::{decode_serde, encode_to_vec};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Metric {
        name: String,
        value: f64,
        tags: Vec<String>,
    }

    let original = Metric {
        name: "cpu.usage".to_string(),
        value: 87.3,
        tags: vec!["host:web01".to_string(), "env:prod".to_string()],
    };
    let bytes =
        encode_to_vec(&original, oxicode::config::standard()).expect("encode_to_vec Metric");
    let decoded: Metric = decode_serde(&bytes).expect("decode_serde Metric");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: encode_serde then verify byte count is non-zero and matches
//          encoded_serde_size prediction
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_19_encode_byte_count_matches_size_prediction() {
    use oxicode::serde::{encode_serde, encoded_serde_size};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Invoice {
        invoice_id: u64,
        line_items: Vec<u32>,
        total_cents: u64,
        paid: bool,
    }

    let original = Invoice {
        invoice_id: 100_001,
        line_items: vec![500, 1200, 350, 75],
        total_cents: 2125,
        paid: false,
    };
    let bytes = encode_serde(&original).expect("encode Invoice");
    let predicted = encoded_serde_size(&original).expect("encoded_serde_size Invoice");

    assert!(!bytes.is_empty(), "encoded bytes must not be empty");
    assert_eq!(
        bytes.len(),
        predicted,
        "encode_serde byte count must match encoded_serde_size"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Serde struct with large Vec field (1 000 elements)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_20_large_vec_field_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct LargePayload {
        id: u32,
        data: Vec<u64>,
    }

    let data: Vec<u64> = (0u64..1_000).map(|i| i * 1_000_003).collect();
    let original = LargePayload { id: 1, data };
    let bytes = encode_serde(&original).expect("encode LargePayload");
    let decoded: LargePayload = decode_serde(&bytes).expect("decode LargePayload");
    assert_eq!(original, decoded);
    assert_eq!(decoded.data.len(), 1_000);
}

// ---------------------------------------------------------------------------
// Test 21: encode_serde_with_config / decode_serde_with_config roundtrip
//          with a custom (little-endian + variable-int) configuration
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_21_encode_decode_serde_with_config_varint() {
    use oxicode::serde::{decode_serde_with_config, encode_serde_with_config};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SparseCounts {
        label: String,
        counts: Vec<u64>,
    }

    // Standard config is little-endian variable-int — exercise the
    // with_config variants explicitly.
    let cfg = oxicode::config::standard();

    let original = SparseCounts {
        label: "word_freq".to_string(),
        counts: vec![0, 1, 127, 128, 255, 16383, 16384, u64::MAX],
    };
    let bytes =
        encode_serde_with_config(&original, cfg).expect("encode_serde_with_config SparseCounts");
    let decoded: SparseCounts =
        decode_serde_with_config(&bytes, cfg).expect("decode_serde_with_config SparseCounts");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Serde derive macro on a generic struct
//          Generic structs with serde derive and concrete instantiations.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv2_22_serde_derive_on_generic_struct() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper<T> {
        value: T,
        tag: String,
    }

    // Instantiate with u32
    let w_u32: Wrapper<u32> = Wrapper {
        value: 42,
        tag: "uint".to_string(),
    };
    let bytes = encode_serde(&w_u32).expect("encode Wrapper<u32>");
    let decoded_u32: Wrapper<u32> = decode_serde(&bytes).expect("decode Wrapper<u32>");
    assert_eq!(w_u32, decoded_u32);

    // Instantiate with Vec<String>
    let w_vec: Wrapper<Vec<String>> = Wrapper {
        value: vec!["alpha".to_string(), "beta".to_string()],
        tag: "strings".to_string(),
    };
    let bytes2 = encode_serde(&w_vec).expect("encode Wrapper<Vec<String>>");
    let decoded_vec: Wrapper<Vec<String>> =
        decode_serde(&bytes2).expect("decode Wrapper<Vec<String>>");
    assert_eq!(w_vec, decoded_vec);

    // Instantiate with Option<f64>
    let w_opt: Wrapper<Option<f64>> = Wrapper {
        value: Some(core::f64::consts::E),
        tag: "optional_float".to_string(),
    };
    let bytes3 = encode_serde(&w_opt).expect("encode Wrapper<Option<f64>>");
    let decoded_opt: Wrapper<Option<f64>> =
        decode_serde(&bytes3).expect("decode Wrapper<Option<f64>>");
    assert_eq!(w_opt, decoded_opt);
}
