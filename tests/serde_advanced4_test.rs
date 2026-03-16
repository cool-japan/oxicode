//! Advanced serde integration tests (set 4) for OxiCode.
//!
//! All 22 tests are individually gated on `#[cfg(feature = "serde")]`.
//! No `#[cfg(test)]` module wrapper — all tests are top-level.
//! No `unwrap()` — every fallible call uses `.expect("…")`.
//!
//! Coverage angles are distinct from `serde_integration_test.rs`,
//! `serde_complex_test.rs`, `serde_advanced_test.rs`, `serde_advanced2_test.rs`,
//! and `serde_owned_test.rs`.

// ---------------------------------------------------------------------------
// Test 1: Serde encode/decode of enum with #[serde(rename_all = "snake_case")]
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
fn test_adv4_01_enum_rename_all_snake_case() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum NetworkEvent {
        ConnectionOpened,
        DataReceived,
        ConnectionClosed,
        ErrorOccurred,
    }

    let variants = [
        NetworkEvent::ConnectionOpened,
        NetworkEvent::DataReceived,
        NetworkEvent::ConnectionClosed,
        NetworkEvent::ErrorOccurred,
    ];
    for variant in &variants {
        let bytes = encode_serde(variant).expect("encode NetworkEvent variant");
        let decoded: NetworkEvent = decode_serde(&bytes).expect("decode NetworkEvent variant");
        assert_eq!(variant, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 2: Serde encode/decode of struct with multiple #[serde(rename)] fields
//         Uses a distinct struct from existing rename tests.
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_02_struct_multiple_rename_fields() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct DatabaseRecord {
        #[serde(rename = "record_id")]
        id: u64,
        #[serde(rename = "table_name")]
        table: String,
        #[serde(rename = "row_count")]
        rows: u32,
        #[serde(rename = "is_indexed")]
        indexed: bool,
    }

    let original = DatabaseRecord {
        id: 9_876_543,
        table: "orders".to_string(),
        rows: 150_000,
        indexed: true,
    };
    let bytes = encode_serde(&original).expect("encode DatabaseRecord with renames");
    let decoded: DatabaseRecord = decode_serde(&bytes).expect("decode DatabaseRecord with renames");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Serde encode/decode of deeply nested structs (four-level nesting)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_03_four_level_nested_structs() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Pixel {
        r: u8,
        g: u8,
        b: u8,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Tile {
        x: u32,
        y: u32,
        color: Pixel,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Layer {
        name: String,
        tiles: Vec<Tile>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Canvas {
        width: u32,
        height: u32,
        layer: Layer,
    }

    let original = Canvas {
        width: 256,
        height: 256,
        layer: Layer {
            name: "background".to_string(),
            tiles: vec![
                Tile {
                    x: 0,
                    y: 0,
                    color: Pixel { r: 255, g: 0, b: 0 },
                },
                Tile {
                    x: 1,
                    y: 0,
                    color: Pixel { r: 0, g: 255, b: 0 },
                },
            ],
        },
    };
    let bytes = encode_serde(&original).expect("encode Canvas 4-level nested");
    let decoded: Canvas = decode_serde(&bytes).expect("decode Canvas 4-level nested");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Serde encode/decode of Vec<struct> (standalone Vec, not a field)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_04_vec_of_struct_standalone() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Product {
        sku: String,
        price_cents: u64,
        in_stock: bool,
    }

    let original: Vec<Product> = vec![
        Product {
            sku: "WIDGET-001".to_string(),
            price_cents: 999,
            in_stock: true,
        },
        Product {
            sku: "GADGET-042".to_string(),
            price_cents: 4_999,
            in_stock: false,
        },
        Product {
            sku: "DEVICE-777".to_string(),
            price_cents: 29_999,
            in_stock: true,
        },
    ];
    let bytes = encode_serde(&original).expect("encode Vec<Product>");
    let decoded: Vec<Product> = decode_serde(&bytes).expect("decode Vec<Product>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 5: Serde encode/decode of struct with multiple Option fields
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_05_struct_multiple_option_fields() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct UserProfile {
        username: String,
        email: Option<String>,
        age: Option<u8>,
        bio: Option<String>,
        website: Option<String>,
    }

    // All Some
    let all_some = UserProfile {
        username: "alice".to_string(),
        email: Some("alice@example.com".to_string()),
        age: Some(30),
        bio: Some("Software engineer".to_string()),
        website: Some("https://alice.dev".to_string()),
    };
    let bytes = encode_serde(&all_some).expect("encode UserProfile all-Some");
    let decoded: UserProfile = decode_serde(&bytes).expect("decode UserProfile all-Some");
    assert_eq!(all_some, decoded);

    // All None
    let all_none = UserProfile {
        username: "ghost".to_string(),
        email: None,
        age: None,
        bio: None,
        website: None,
    };
    let bytes = encode_serde(&all_none).expect("encode UserProfile all-None");
    let decoded: UserProfile = decode_serde(&bytes).expect("decode UserProfile all-None");
    assert_eq!(all_none, decoded);

    // Mixed
    let mixed = UserProfile {
        username: "bob".to_string(),
        email: Some("bob@example.com".to_string()),
        age: None,
        bio: None,
        website: Some("https://bob.io".to_string()),
    };
    let bytes = encode_serde(&mixed).expect("encode UserProfile mixed");
    let decoded: UserProfile = decode_serde(&bytes).expect("decode UserProfile mixed");
    assert_eq!(mixed, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Serde roundtrip of HashMap<String, Vec<u32>>
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_06_hashmap_string_to_vec_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct GroupData {
        label: String,
        groups: HashMap<String, Vec<u32>>,
    }

    let mut groups = HashMap::new();
    groups.insert("evens".to_string(), vec![2, 4, 6, 8, 10]);
    groups.insert("odds".to_string(), vec![1, 3, 5, 7, 9]);
    groups.insert("primes".to_string(), vec![2, 3, 5, 7, 11, 13]);

    let original = GroupData {
        label: "number_groups".to_string(),
        groups,
    };
    let bytes = encode_serde(&original).expect("encode GroupData with HashMap<String, Vec<u32>>");
    let decoded: GroupData =
        decode_serde(&bytes).expect("decode GroupData with HashMap<String, Vec<u32>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Serde encode/decode with big-endian config (struct with mixed types)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_07_encode_decode_with_big_endian_config() {
    use oxicode::serde::{decode_serde_with_config, encode_serde_with_config};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct NetworkPacket {
        version: u8,
        source_port: u16,
        dest_port: u16,
        sequence_number: u32,
        payload_length: u64,
    }

    let cfg = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let original = NetworkPacket {
        version: 4,
        source_port: 8080,
        dest_port: 443,
        sequence_number: 0x0102_0304,
        payload_length: 1024,
    };
    let bytes =
        encode_serde_with_config(&original, cfg).expect("encode NetworkPacket big-endian fixed");
    let decoded: NetworkPacket =
        decode_serde_with_config(&bytes, cfg).expect("decode NetworkPacket big-endian fixed");
    assert_eq!(original, decoded);

    // 1 (u8) + 2 (u16) + 2 (u16) + 4 (u32) + 8 (u64) = 17 bytes
    assert_eq!(
        bytes.len(),
        17,
        "fixed big-endian NetworkPacket must be 17 bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Cross-format: serde-serialized data decoded with decode_from_slice
//         (serde::decode_from_slice variant that returns (T, usize))
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_08_cross_format_encode_serde_decode_from_slice() {
    use oxicode::serde::{decode_from_slice as serde_decode_slice, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Coordinate {
        longitude: f64,
        latitude: f64,
        altitude_m: f32,
    }

    let original = Coordinate {
        longitude: 139.6917,
        latitude: 35.6895,
        altitude_m: 44.0,
    };
    let bytes = encode_serde(&original).expect("encode_serde Coordinate");
    let (decoded, consumed): (Coordinate, usize) =
        serde_decode_slice(&bytes, oxicode::config::standard())
            .expect("serde decode_from_slice Coordinate");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 9: Serde roundtrip of BTreeMap<u32, String>
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_09_btreemap_u32_string_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Registry {
        version: u32,
        entries: BTreeMap<u32, String>,
    }

    let mut entries = BTreeMap::new();
    entries.insert(1, "alpha".to_string());
    entries.insert(2, "beta".to_string());
    entries.insert(100, "gamma".to_string());
    entries.insert(999, "delta".to_string());

    let original = Registry {
        version: 2,
        entries,
    };
    let bytes = encode_serde(&original).expect("encode Registry BTreeMap");
    let decoded: Registry = decode_serde(&bytes).expect("decode Registry BTreeMap");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Serde roundtrip of Vec<Option<f64>>
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_10_vec_of_option_f64_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};

    let original: Vec<Option<f64>> = vec![
        Some(1.0),
        None,
        Some(core::f64::consts::PI),
        None,
        Some(-core::f64::consts::E),
        Some(f64::INFINITY),
        None,
    ];
    let bytes = encode_serde(&original).expect("encode Vec<Option<f64>>");
    let decoded: Vec<Option<f64>> = decode_serde(&bytes).expect("decode Vec<Option<f64>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Serde roundtrip of tuple (mixed types)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_11_tuple_mixed_types_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};

    let original: (u64, String, bool, f32) = (
        42_000_000,
        "oxicode".to_string(),
        true,
        core::f32::consts::PI,
    );
    let bytes = encode_serde(&original).expect("encode (u64, String, bool, f32)");
    let decoded: (u64, String, bool, f32) =
        decode_serde(&bytes).expect("decode (u64, String, bool, f32)");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Serde roundtrip of nested Option<Vec<String>>
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_12_option_vec_string_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Config {
        name: String,
        tags: Option<Vec<String>>,
    }

    // Some branch with non-empty Vec
    let with_tags = Config {
        name: "service-a".to_string(),
        tags: Some(vec!["production".to_string(), "v2".to_string()]),
    };
    let bytes = encode_serde(&with_tags).expect("encode Config with tags");
    let decoded: Config = decode_serde(&bytes).expect("decode Config with tags");
    assert_eq!(with_tags, decoded);

    // Some branch with empty Vec
    let empty_tags = Config {
        name: "service-b".to_string(),
        tags: Some(vec![]),
    };
    let bytes = encode_serde(&empty_tags).expect("encode Config with empty tags");
    let decoded: Config = decode_serde(&bytes).expect("decode Config with empty tags");
    assert_eq!(empty_tags, decoded);

    // None branch
    let no_tags = Config {
        name: "service-c".to_string(),
        tags: None,
    };
    let bytes = encode_serde(&no_tags).expect("encode Config with no tags");
    let decoded: Config = decode_serde(&bytes).expect("decode Config with no tags");
    assert_eq!(no_tags, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Serde roundtrip with encode_into_slice (serde module)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_13_encode_into_slice_serde_roundtrip() {
    use oxicode::serde::{
        decode_from_slice as serde_decode_slice, encode_into_slice as serde_encode_slice,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Header {
        magic: u32,
        version: u8,
        flags: u16,
    }

    let original = Header {
        magic: 0xCAFE_BABE,
        version: 3,
        flags: 0b0000_1111_1010_0101,
    };

    let mut buf = [0u8; 64];
    let written = serde_encode_slice(&original, &mut buf, oxicode::config::standard())
        .expect("encode_into_slice Header");
    assert!(written > 0, "must write at least 1 byte");

    let (decoded, consumed): (Header, usize) =
        serde_decode_slice(&buf[..written], oxicode::config::standard())
            .expect("decode_from_slice Header");
    assert_eq!(original, decoded);
    assert_eq!(consumed, written);
}

// ---------------------------------------------------------------------------
// Test 14: Serde roundtrip of HashMap<String, HashMap<String, u32>> (nested maps)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_14_nested_hashmap_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use std::collections::HashMap;

    let mut inner1 = HashMap::new();
    inner1.insert("reads".to_string(), 1_000u32);
    inner1.insert("writes".to_string(), 250u32);

    let mut inner2 = HashMap::new();
    inner2.insert("requests".to_string(), 5_000u32);
    inner2.insert("errors".to_string(), 12u32);

    let mut original: HashMap<String, HashMap<String, u32>> = HashMap::new();
    original.insert("database".to_string(), inner1);
    original.insert("http".to_string(), inner2);

    let bytes = encode_serde(&original).expect("encode nested HashMap");
    let decoded: HashMap<String, HashMap<String, u32>> =
        decode_serde(&bytes).expect("decode nested HashMap");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Serde roundtrip of struct with Vec<(String, u64)> (Vec of tuples)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_15_vec_of_tuples_field() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Leaderboard {
        game: String,
        scores: Vec<(String, u64)>,
    }

    let original = Leaderboard {
        game: "oxicode-challenge".to_string(),
        scores: vec![
            ("Alice".to_string(), 98_500),
            ("Bob".to_string(), 87_200),
            ("Charlie".to_string(), 75_000),
        ],
    };
    let bytes = encode_serde(&original).expect("encode Leaderboard with Vec<(String, u64)>");
    let decoded: Leaderboard =
        decode_serde(&bytes).expect("decode Leaderboard with Vec<(String, u64)>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Serde roundtrip of enum with struct variants (adjacently distinct
//          from test 4 which uses Command; this uses a Shape enum)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_16_enum_struct_variants_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
        Point,
    }

    let shapes = [
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 10.0,
            height: 4.5,
        },
        Shape::Triangle {
            base: 6.0,
            height: 3.0,
        },
        Shape::Point,
    ];
    for shape in &shapes {
        let bytes = encode_serde(shape).expect("encode Shape variant");
        let decoded: Shape = decode_serde(&bytes).expect("decode Shape variant");
        assert_eq!(shape, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 17: Serde roundtrip with encode_into_std_write / decode_from_std_read
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_17_encode_into_std_write_decode_from_std_read() {
    use oxicode::serde::{decode_from_std_read, encode_into_std_write};
    use serde::{Deserialize, Serialize};
    use std::io::Cursor;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Bookmark {
        title: String,
        url: String,
        visit_count: u32,
    }

    let original = Bookmark {
        title: "OxiCode Documentation".to_string(),
        url: "https://docs.oxicode.rs".to_string(),
        visit_count: 42,
    };

    let mut buf: Vec<u8> = Vec::new();
    let written = encode_into_std_write(&original, &mut buf, oxicode::config::standard())
        .expect("encode_into_std_write Bookmark");
    assert!(written > 0, "must write some bytes");

    let cursor = Cursor::new(&buf[..]);
    let (decoded, consumed): (Bookmark, usize) =
        decode_from_std_read(cursor, oxicode::config::standard())
            .expect("decode_from_std_read Bookmark");
    assert_eq!(original, decoded);
    assert_eq!(consumed, written);
}

// ---------------------------------------------------------------------------
// Test 18: Serde roundtrip with file I/O (encode_serde_to_file /
//           decode_serde_from_file) using a temp file
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_18_encode_serde_to_file_decode_serde_from_file() {
    use oxicode::serde::{decode_serde_from_file, encode_serde_to_file};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct AppState {
        session_id: u64,
        logged_in: bool,
        username: String,
        permissions: Vec<String>,
    }

    let original = AppState {
        session_id: 0xDEAD_BEEF_0000_0001,
        logged_in: true,
        username: "admin".to_string(),
        permissions: vec![
            "read".to_string(),
            "write".to_string(),
            "delete".to_string(),
        ],
    };

    let mut path = std::env::temp_dir();
    path.push("oxicode_serde_adv4_test18.bin");

    encode_serde_to_file(&original, &path).expect("encode_serde_to_file AppState");
    let decoded: AppState = decode_serde_from_file(&path).expect("decode_serde_from_file AppState");
    assert_eq!(original, decoded);

    // Clean up
    let _ = std::fs::remove_file(&path);
}

// ---------------------------------------------------------------------------
// Test 19: Serde roundtrip of struct with many string fields
//          (large struct with diverse string/numeric content)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_19_struct_many_string_fields() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Article {
        id: u64,
        title: String,
        content: String,
        author: String,
        category: String,
        created_at: u64,
        updated_at: u64,
        published: bool,
        view_count: u64,
    }

    let original = Article {
        id: 12_345,
        title: "Binary Serialization with OxiCode".to_string(),
        content: "OxiCode provides efficient binary encoding...".to_string(),
        author: "team_kitasan".to_string(),
        category: "engineering".to_string(),
        created_at: 1_700_000_000,
        updated_at: 1_700_100_000,
        published: true,
        view_count: 9_999,
    };
    let bytes = encode_serde(&original).expect("encode Article with many fields");
    let decoded: Article = decode_serde(&bytes).expect("decode Article with many fields");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Serde roundtrip of Vec<Vec<u8>> (nested Vecs)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_20_vec_of_vec_u8_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};

    let original: Vec<Vec<u8>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![],
        vec![255, 254, 253],
        vec![0; 10],
        (0u8..=127).collect(),
    ];
    let bytes = encode_serde(&original).expect("encode Vec<Vec<u8>>");
    let decoded: Vec<Vec<u8>> = decode_serde(&bytes).expect("decode Vec<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Serde roundtrip of struct with Vec<String> field — empty, single,
//          and multi-element cases
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_21_vec_string_field_empty_single_multi() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Event {
        name: String,
        tags: Vec<String>,
        timestamp: u64,
    }

    // Empty tags
    let empty = Event {
        name: "boot".to_string(),
        tags: vec![],
        timestamp: 1_700_000_000,
    };
    let bytes = encode_serde(&empty).expect("encode Event with empty tags");
    let decoded: Event = decode_serde(&bytes).expect("decode Event with empty tags");
    assert_eq!(empty, decoded);
    assert!(decoded.tags.is_empty(), "tags must be empty");

    // Single tag
    let single = Event {
        name: "login".to_string(),
        tags: vec!["auth".to_string()],
        timestamp: 1_700_100_000,
    };
    let bytes = encode_serde(&single).expect("encode Event with single tag");
    let decoded: Event = decode_serde(&bytes).expect("decode Event with single tag");
    assert_eq!(single, decoded);

    // Multiple tags
    let multi = Event {
        name: "purchase".to_string(),
        tags: vec![
            "billing".to_string(),
            "checkout".to_string(),
            "cart".to_string(),
        ],
        timestamp: 1_700_200_000,
    };
    let bytes = encode_serde(&multi).expect("encode Event with multiple tags");
    let decoded: Event = decode_serde(&bytes).expect("decode Event with multiple tags");
    assert_eq!(multi, decoded);
    assert_eq!(decoded.tags.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 22: Serde roundtrip of a generic enum with associated data
//          (Result<T, E> — serializable via serde derive on a custom type)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_adv4_22_generic_result_like_enum_roundtrip() {
    use oxicode::serde::{decode_serde, encode_serde};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum ApiResult<T, E> {
        Success(T),
        Failure { code: u32, error: E },
        Pending,
    }

    // Success branch with a struct payload
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ResponseBody {
        items: Vec<String>,
        total: u64,
    }

    let ok: ApiResult<ResponseBody, String> = ApiResult::Success(ResponseBody {
        items: vec!["item1".to_string(), "item2".to_string()],
        total: 2,
    });
    let bytes = encode_serde(&ok).expect("encode ApiResult::Success");
    let decoded: ApiResult<ResponseBody, String> =
        decode_serde(&bytes).expect("decode ApiResult::Success");
    assert_eq!(ok, decoded);

    // Failure branch
    let err: ApiResult<ResponseBody, String> = ApiResult::Failure {
        code: 404,
        error: "resource not found".to_string(),
    };
    let bytes = encode_serde(&err).expect("encode ApiResult::Failure");
    let decoded: ApiResult<ResponseBody, String> =
        decode_serde(&bytes).expect("decode ApiResult::Failure");
    assert_eq!(err, decoded);

    // Pending branch
    let pending: ApiResult<ResponseBody, String> = ApiResult::Pending;
    let bytes = encode_serde(&pending).expect("encode ApiResult::Pending");
    let decoded: ApiResult<ResponseBody, String> =
        decode_serde(&bytes).expect("decode ApiResult::Pending");
    assert_eq!(pending, decoded);
}
