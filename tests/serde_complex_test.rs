//! Complex serde integration tests for OxiCode.
//!
//! All 22 tests are individually gated on `#[cfg(feature = "serde")]` and cover
//! advanced serde attribute combinations, deeply nested types, config variants,
//! and complex composite structures — distinct from existing serde test coverage.

// ---------------------------------------------------------------------------
// Test 1: Simple struct with serde derive roundtrip
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
fn test_cx_01_simple_struct_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Point {
        x: f64,
        y: f64,
    }

    let original = Point { x: 1.23, y: -4.56 };
    let bytes = serde_encode(&original, oxicode::config::standard()).expect("encode Point");
    let (decoded, consumed): (Point, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode Point");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 2: Enum with serde derive roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_02_simple_enum_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    for dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        let bytes = serde_encode(&dir, oxicode::config::standard()).expect("encode Direction");
        let (decoded, consumed): (Direction, usize) =
            serde_decode(&bytes, oxicode::config::standard()).expect("decode Direction");
        assert_eq!(dir, decoded);
        assert_eq!(consumed, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// Test 3: Struct with rename attribute: #[serde(rename = "field_name")]
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_03_struct_rename_attribute_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct UserRecord {
        #[serde(rename = "user_id")]
        id: u64,
        #[serde(rename = "display_name")]
        name: String,
        #[serde(rename = "account_active")]
        active: bool,
    }

    let original = UserRecord {
        id: 42,
        name: "alice".to_string(),
        active: true,
    };
    let bytes =
        serde_encode(&original, oxicode::config::standard()).expect("encode renamed struct");
    let (decoded, consumed): (UserRecord, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode renamed struct");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 4: Struct with skip field: #[serde(skip)]
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_04_skip_field_defaults_on_decode() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Payload {
        data: Vec<u8>,
        checksum: u32,
        #[serde(skip)]
        internal_tag: u64, // not serialized; reverts to Default (0) on decode
    }

    let original = Payload {
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
        checksum: 0xCAFE_BABE,
        internal_tag: 0xFFFF_FFFF_FFFF_FFFF, // skipped — must not appear in bytes
    };
    let bytes = serde_encode(&original, oxicode::config::standard()).expect("encode skip struct");
    let (decoded, consumed): (Payload, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode skip struct");

    assert_eq!(decoded.data, original.data);
    assert_eq!(decoded.checksum, original.checksum);
    assert_eq!(
        decoded.internal_tag, 0u64,
        "skipped field must default to 0"
    );
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 5: Nested structs with serde
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_05_nested_structs_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
        zip: u32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
        address: Address,
    }

    let original = Person {
        name: "Bob".to_string(),
        age: 30,
        address: Address {
            street: "123 Main St".to_string(),
            city: "Springfield".to_string(),
            zip: 12345,
        },
    };
    let bytes = serde_encode(&original, oxicode::config::standard()).expect("encode nested");
    let (decoded, consumed): (Person, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode nested");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 6: Vec<SerdeStruct> roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_06_vec_of_serde_structs_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Score {
        player: String,
        points: u64,
    }

    let original = vec![
        Score {
            player: "Alice".to_string(),
            points: 9001,
        },
        Score {
            player: "Bob".to_string(),
            points: 42,
        },
        Score {
            player: "Charlie".to_string(),
            points: 1337,
        },
    ];
    let bytes = serde_encode(&original, oxicode::config::standard()).expect("encode Vec<Score>");
    let (decoded, consumed): (Vec<Score>, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode Vec<Score>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 7: HashMap<String, SerdeStruct> roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_07_hashmap_string_to_struct_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap; // deterministic order for assertions

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Config {
        value: i32,
        enabled: bool,
    }

    let mut original: BTreeMap<String, Config> = BTreeMap::new();
    original.insert(
        "alpha".to_string(),
        Config {
            value: 1,
            enabled: true,
        },
    );
    original.insert(
        "beta".to_string(),
        Config {
            value: -99,
            enabled: false,
        },
    );

    let bytes = serde_encode(&original, oxicode::config::standard())
        .expect("encode BTreeMap<String,Config>");
    let (decoded, consumed): (BTreeMap<String, Config>, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode BTreeMap<String,Config>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 8: Option<SerdeStruct> Some/None roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_08_option_struct_some_and_none_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Tag {
        label: String,
        priority: u8,
    }

    // --- Some variant ---
    let some_val: Option<Tag> = Some(Tag {
        label: "urgent".to_string(),
        priority: 1,
    });
    let bytes_some =
        serde_encode(&some_val, oxicode::config::standard()).expect("encode Option::Some");
    let (decoded_some, _): (Option<Tag>, usize) =
        serde_decode(&bytes_some, oxicode::config::standard()).expect("decode Option::Some");
    assert_eq!(some_val, decoded_some);

    // --- None variant ---
    let none_val: Option<Tag> = None;
    let bytes_none =
        serde_encode(&none_val, oxicode::config::standard()).expect("encode Option::None");
    let (decoded_none, _): (Option<Tag>, usize) =
        serde_decode(&bytes_none, oxicode::config::standard()).expect("decode Option::None");
    assert_eq!(none_val, decoded_none);
}

// ---------------------------------------------------------------------------
// Test 9: Serde struct with i128 field
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_09_struct_with_i128_field() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct BigNum {
        label: String,
        value: i128,
    }

    let original = BigNum {
        label: "max".to_string(),
        value: i128::MAX,
    };
    let bytes = serde_encode(&original, oxicode::config::standard()).expect("encode i128 struct");
    let (decoded, consumed): (BigNum, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode i128 struct");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    let original_min = BigNum {
        label: "min".to_string(),
        value: i128::MIN,
    };
    let bytes_min =
        serde_encode(&original_min, oxicode::config::standard()).expect("encode i128 min");
    let (decoded_min, _): (BigNum, usize) =
        serde_decode(&bytes_min, oxicode::config::standard()).expect("decode i128 min");
    assert_eq!(original_min, decoded_min);
}

// ---------------------------------------------------------------------------
// Test 10: Serde struct with Vec<String> field
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_10_struct_with_vec_string_field() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Tags {
        name: String,
        tags: Vec<String>,
    }

    let original = Tags {
        name: "post".to_string(),
        tags: vec![
            "rust".to_string(),
            "serialization".to_string(),
            "binary".to_string(),
            "oxicode".to_string(),
        ],
    };
    let bytes =
        serde_encode(&original, oxicode::config::standard()).expect("encode Vec<String> struct");
    let (decoded, consumed): (Tags, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode Vec<String> struct");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 11: Serde enum with tuple variants
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_11_enum_tuple_variants_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Event {
        Click(u32, u32), // (x, y)
        KeyPress(char),
        Scroll(f32),
        Resize(u16, u16), // (width, height)
    }

    let cases = [
        Event::Click(100, 200),
        Event::KeyPress('z'),
        Event::Scroll(-1.5),
        Event::Resize(1920, 1080),
    ];
    for ev in &cases {
        let bytes = serde_encode(ev, oxicode::config::standard()).expect("encode tuple variant");
        let (decoded, consumed): (Event, usize) =
            serde_decode(&bytes, oxicode::config::standard()).expect("decode tuple variant");
        assert_eq!(ev, &decoded);
        assert_eq!(consumed, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// Test 12: Serde enum with struct variants
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_12_enum_struct_variants_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Notification {
        Email { recipient: String, subject: String },
        Sms { phone: String, body: String },
        Push { device_id: u64, message: String },
    }

    let cases = [
        Notification::Email {
            recipient: "user@example.com".to_string(),
            subject: "Hello".to_string(),
        },
        Notification::Sms {
            phone: "+1-555-0100".to_string(),
            body: "Test message".to_string(),
        },
        Notification::Push {
            device_id: 0xDEAD_BEEF_CAFE_1234,
            message: "Alert!".to_string(),
        },
    ];
    for notif in &cases {
        let bytes =
            serde_encode(notif, oxicode::config::standard()).expect("encode struct variant");
        let (decoded, consumed): (Notification, usize) =
            serde_decode(&bytes, oxicode::config::standard()).expect("decode struct variant");
        assert_eq!(notif, &decoded);
        assert_eq!(consumed, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// Test 13: Serde flatten via composition (binary-safe alternative)
//          #[serde(flatten)] is not supported by the binary format; instead
//          we verify that a struct containing a nested "flattened" sub-struct
//          (using an explicit field) round-trips correctly.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_13_flatten_via_nested_composition() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Base {
        created_at: u64,
        updated_at: u64,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Record {
        id: u32,
        name: String,
        timestamps: Base, // explicit nesting — binary-format-safe composition
    }

    let original = Record {
        id: 1,
        name: "record-one".to_string(),
        timestamps: Base {
            created_at: 1_700_000_000,
            updated_at: 1_700_001_000,
        },
    };
    let bytes =
        serde_encode(&original, oxicode::config::standard()).expect("encode composed record");
    let (decoded, consumed): (Record, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode composed record");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 14: Serde with default: #[serde(default)] field
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_14_serde_default_field_explicit_value() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    fn default_max_retries() -> u32 {
        3
    }

    fn default_label() -> String {
        "unnamed".to_string()
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Job {
        id: u64,
        #[serde(default = "default_max_retries")]
        max_retries: u32,
        #[serde(default = "default_label")]
        label: String,
    }

    // Explicit non-default values must survive the roundtrip.
    let original = Job {
        id: 99,
        max_retries: 10,
        label: "critical-job".to_string(),
    };
    let bytes =
        serde_encode(&original, oxicode::config::standard()).expect("encode default field struct");
    let (decoded, consumed): (Job, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode default field struct");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.max_retries, 10);
    assert_eq!(decoded.label, "critical-job");
}

// ---------------------------------------------------------------------------
// Test 15: Large serde struct (10 fields)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_15_large_struct_ten_fields() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TenFields {
        f1: u8,
        f2: u16,
        f3: u32,
        f4: u64,
        f5: i8,
        f6: i16,
        f7: i32,
        f8: i64,
        f9: f32,
        f10: f64,
    }

    let original = TenFields {
        f1: 255,
        f2: 65535,
        f3: 4_294_967_295,
        f4: 18_446_744_073_709_551_615,
        f5: -128,
        f6: -32768,
        f7: -2_147_483_648,
        f8: -9_223_372_036_854_775_808,
        f9: core::f32::consts::PI,
        f10: core::f64::consts::E,
    };
    let bytes =
        serde_encode(&original, oxicode::config::standard()).expect("encode 10-field struct");
    let (decoded, consumed): (TenFields, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode 10-field struct");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 16: Serde struct with bool, u32, String, Vec<u8> fields
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_16_mixed_primitive_fields() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Packet {
        valid: bool,
        sequence: u32,
        source: String,
        payload: Vec<u8>,
    }

    let original = Packet {
        valid: true,
        sequence: 1_234_567,
        source: "192.168.1.1".to_string(),
        payload: (0u8..=255u8).collect(),
    };
    let bytes = serde_encode(&original, oxicode::config::standard()).expect("encode mixed packet");
    let (decoded, consumed): (Packet, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode mixed packet");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 17: Nested enum with serde
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_17_nested_enum_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Inner {
        A,
        B(u32),
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Outer {
        Leaf(String),
        Node { left: Inner, right: Inner },
    }

    let cases = [
        Outer::Leaf("hello".to_string()),
        Outer::Node {
            left: Inner::A,
            right: Inner::B(42),
        },
        Outer::Node {
            left: Inner::B(999),
            right: Inner::A,
        },
    ];
    for case in &cases {
        let bytes = serde_encode(case, oxicode::config::standard()).expect("encode nested enum");
        let (decoded, consumed): (Outer, usize) =
            serde_decode(&bytes, oxicode::config::standard()).expect("decode nested enum");
        assert_eq!(case, &decoded);
        assert_eq!(consumed, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// Test 18: Serde struct encode size matches expectations
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_18_encode_size_matches_expectation() {
    use oxicode::serde::{encode_to_vec as serde_encode, encoded_serde_size};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Sized {
        a: u8,
        b: u8,
    }

    let val = Sized { a: 10, b: 20 };
    let bytes = serde_encode(&val, oxicode::config::standard()).expect("encode Sized");
    let predicted = encoded_serde_size(&val).expect("predicted size");

    // The size reported by encode_to_vec and encoded_serde_size must agree.
    assert_eq!(
        bytes.len(),
        predicted,
        "encoded_serde_size must match actual encoded length"
    );
    // Each u8 in variable-int encoding takes exactly 1 byte → total = 2.
    assert_eq!(bytes.len(), 2, "two u8 fields should each occupy 1 byte");
}

// ---------------------------------------------------------------------------
// Test 19: Fixed int config with serde
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_19_fixed_int_config_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Counter {
        count: u32,
        total: u64,
    }

    let cfg = oxicode::config::standard().with_fixed_int_encoding();

    let original = Counter {
        count: 42,
        total: 1_000_000,
    };
    let bytes = serde_encode(&original, cfg).expect("encode fixed-int Counter");
    let (decoded, consumed): (Counter, usize) =
        serde_decode(&bytes, cfg).expect("decode fixed-int Counter");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // With fixed-int encoding, u32 occupies exactly 4 bytes and u64 exactly 8 bytes.
    assert_eq!(
        bytes.len(),
        4 + 8,
        "fixed-int: 4 bytes for u32 + 8 bytes for u64"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Big endian config with serde
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_20_big_endian_config_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Header {
        magic: u32,
        version: u16,
        flags: u8,
    }

    let cfg = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let original = Header {
        magic: 0x0C1C0DE3,
        version: 2,
        flags: 0xFF,
    };
    let bytes = serde_encode(&original, cfg).expect("encode big-endian Header");
    let (decoded, consumed): (Header, usize) =
        serde_decode(&bytes, cfg).expect("decode big-endian Header");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // In big-endian + fixed-int, the first byte of the u32 is the most-significant byte.
    let expected_magic_be = original.magic.to_be_bytes();
    assert_eq!(
        &bytes[..4],
        &expected_magic_be,
        "big-endian encoding of magic must match to_be_bytes()"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec of serde enums roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_21_vec_of_enums_roundtrip() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Op {
        Add(i64, i64),
        Mul(i64, i64),
        Neg(i64),
        Zero,
    }

    let original = vec![
        Op::Add(3, 4),
        Op::Mul(-2, 10),
        Op::Neg(99),
        Op::Zero,
        Op::Add(i64::MAX, -1),
    ];
    let bytes = serde_encode(&original, oxicode::config::standard()).expect("encode Vec<Op>");
    let (decoded, consumed): (Vec<Op>, usize) =
        serde_decode(&bytes, oxicode::config::standard()).expect("decode Vec<Op>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 22: Complex nested: serde struct containing Vec<SerdEnum> and
//          Option<SerdeStruct>
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_cx_22_complex_nested_struct_with_vec_enum_and_option() {
    use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Status {
        Pending,
        Running { progress: u8 },
        Done(String),
        Failed { code: i32, reason: String },
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Metadata {
        author: String,
        revision: u32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Pipeline {
        name: String,
        stages: Vec<Status>,
        meta: Option<Metadata>,
    }

    // With metadata
    let original_with_meta = Pipeline {
        name: "ci-build".to_string(),
        stages: vec![
            Status::Done("init".to_string()),
            Status::Running { progress: 50 },
            Status::Failed {
                code: -1,
                reason: "OOM".to_string(),
            },
        ],
        meta: Some(Metadata {
            author: "kitasan".to_string(),
            revision: 7,
        }),
    };
    let bytes_with = serde_encode(&original_with_meta, oxicode::config::standard())
        .expect("encode Pipeline with meta");
    let (decoded_with, consumed_with): (Pipeline, usize) =
        serde_decode(&bytes_with, oxicode::config::standard()).expect("decode Pipeline with meta");
    assert_eq!(original_with_meta, decoded_with);
    assert_eq!(consumed_with, bytes_with.len());

    // Without metadata
    let original_no_meta = Pipeline {
        name: "nightly".to_string(),
        stages: vec![Status::Pending, Status::Pending],
        meta: None,
    };
    let bytes_none = serde_encode(&original_no_meta, oxicode::config::standard())
        .expect("encode Pipeline no meta");
    let (decoded_none, consumed_none): (Pipeline, usize) =
        serde_decode(&bytes_none, oxicode::config::standard()).expect("decode Pipeline no meta");
    assert_eq!(original_no_meta, decoded_none);
    assert_eq!(consumed_none, bytes_none.len());
}
