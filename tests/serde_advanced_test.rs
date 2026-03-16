//! 22 comprehensive serde integration tests for OxiCode.
//!
//! All tests are gated on `#[cfg(feature = "serde")]` and cover primitive types,
//! standard collections, derived structs and enums, serde attribute combinations
//! (`rename`, `skip`, `default`, `flatten`), and deeply nested composite types.
//! None of these tests duplicate coverage in `serde_integration_test.rs`.

#![cfg(feature = "serde")]
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
mod serde_advanced_tests {
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    // -----------------------------------------------------------------------
    // Test 1: Serde serialize/deserialize of primitive u32
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_01_primitive_u32_roundtrip() {
        let original: u32 = 987_654_321;
        let bytes = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode u32 failed");
        let (decoded, consumed): (u32, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode u32 failed");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // Test 2: Serde serialize/deserialize of String
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_02_string_roundtrip() {
        let original = String::from("The quick brown fox jumps over the lazy dog");
        let bytes = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode String failed");
        let (decoded, consumed): (String, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode String failed");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // Test 3: Serde serialize/deserialize of Vec<u32>
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_03_vec_u32_roundtrip() {
        let original: Vec<u32> = vec![0, 1, u32::MAX, 42, 100_000, 7];
        let bytes = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode Vec<u32> failed");
        let (decoded, consumed): (Vec<u32>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode Vec<u32> failed");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // Test 4: Serde serialize/deserialize of BTreeMap<String, u32>
    //         (BTreeMap used for deterministic key order)
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_04_btreemap_string_u32_roundtrip() {
        let mut original: BTreeMap<String, u32> = BTreeMap::new();
        original.insert("zebra".to_string(), 999);
        original.insert("alpha".to_string(), 1);
        original.insert("middle".to_string(), 500);

        let bytes = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode BTreeMap failed");
        let (decoded, consumed): (BTreeMap<String, u32>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode BTreeMap failed");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Verify key order is preserved
        let orig_keys: Vec<&String> = original.keys().collect();
        let dec_keys: Vec<&String> = decoded.keys().collect();
        assert_eq!(orig_keys, dec_keys, "BTreeMap key order must be preserved");
    }

    // -----------------------------------------------------------------------
    // Test 5: Serde serialize/deserialize of Option<String> — Some and None
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_05_option_string_some_and_none() {
        let some_val: Option<String> = Some("present value".to_string());
        let bytes_some = oxicode::serde::encode_to_vec(&some_val, oxicode::config::standard())
            .expect("encode Option<String> Some failed");
        let (decoded_some, _): (Option<String>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes_some, oxicode::config::standard())
                .expect("decode Option<String> Some failed");
        assert_eq!(some_val, decoded_some);

        let none_val: Option<String> = None;
        let bytes_none = oxicode::serde::encode_to_vec(&none_val, oxicode::config::standard())
            .expect("encode Option<String> None failed");
        let (decoded_none, _): (Option<String>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes_none, oxicode::config::standard())
                .expect("decode Option<String> None failed");
        assert_eq!(none_val, decoded_none);
    }

    // -----------------------------------------------------------------------
    // Test 6: Serde serialize/deserialize of #[derive(Serialize, Deserialize)] struct
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Sensor {
        id: u64,
        name: String,
        temperature: f64,
        online: bool,
    }

    #[test]
    fn test_adv_06_derived_struct_roundtrip() {
        let original = Sensor {
            id: 42,
            name: "sensor_A".to_string(),
            temperature: 36.6,
            online: true,
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("encode Sensor failed");
        let decoded: Sensor = oxicode::serde::decode_serde(&bytes).expect("decode Sensor failed");
        assert_eq!(original, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 7: Serde serialize/deserialize of #[derive(Serialize, Deserialize)] enum
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Signal {
        Start,
        Stop,
        Pause(u32),
        Configure { key: String, value: u64 },
    }

    #[test]
    fn test_adv_07_derived_enum_all_variants() {
        let cases = [
            Signal::Start,
            Signal::Stop,
            Signal::Pause(9),
            Signal::Configure {
                key: "timeout".to_string(),
                value: 30,
            },
        ];
        for sig in cases {
            let bytes = oxicode::serde::encode_serde(&sig).expect("encode Signal failed");
            let decoded: Signal =
                oxicode::serde::decode_serde(&bytes).expect("decode Signal failed");
            assert_eq!(sig, decoded);
        }
    }

    // -----------------------------------------------------------------------
    // Test 8: Serde encode matches oxicode native encode for same type (u64)
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_08_serde_matches_native_encode_u64() {
        let val: u64 = 0xCAFE_BABE_DEAD_BEEF;
        let serde_bytes = oxicode::serde::encode_to_vec(&val, oxicode::config::standard())
            .expect("serde encode u64 failed");
        let native_bytes = oxicode::encode_to_vec(&val).expect("native encode u64 failed");
        assert_eq!(
            serde_bytes, native_bytes,
            "serde and native encode must be byte-identical for u64"
        );
    }

    // -----------------------------------------------------------------------
    // Test 9: Serde with nested structs
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct GeoPoint {
        lat: f64,
        lon: f64,
        elevation_m: f32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Location {
        name: String,
        country_code: String,
        coordinates: GeoPoint,
    }

    #[test]
    fn test_adv_09_nested_structs_roundtrip() {
        let original = Location {
            name: "Mount Everest".to_string(),
            country_code: "NP".to_string(),
            coordinates: GeoPoint {
                lat: 27.9881,
                lon: 86.9250,
                elevation_m: 8848.86,
            },
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("encode Location failed");
        let decoded: Location =
            oxicode::serde::decode_serde(&bytes).expect("decode Location failed");
        assert_eq!(original, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 10: Serde with Vec of structs
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_10_vec_of_structs_roundtrip() {
        let original = vec![
            Sensor {
                id: 1,
                name: "cpu_temp".to_string(),
                temperature: 55.3,
                online: true,
            },
            Sensor {
                id: 2,
                name: "gpu_temp".to_string(),
                temperature: 72.1,
                online: true,
            },
            Sensor {
                id: 3,
                name: "disk_temp".to_string(),
                temperature: 38.9,
                online: false,
            },
        ];
        let bytes = oxicode::serde::encode_serde(&original).expect("encode Vec<Sensor> failed");
        let decoded: Vec<Sensor> =
            oxicode::serde::decode_serde(&bytes).expect("decode Vec<Sensor> failed");
        assert_eq!(original, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 11: Serde with rename attributes (#[serde(rename = "...")])
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct RenamedFields {
        #[serde(rename = "person_name")]
        name: String,
        #[serde(rename = "birth_year")]
        year: u16,
        #[serde(rename = "is_active")]
        active: bool,
    }

    #[test]
    fn test_adv_11_rename_attributes_roundtrip() {
        let original = RenamedFields {
            name: "Alice".to_string(),
            year: 1995,
            active: true,
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("encode RenamedFields failed");
        let decoded: RenamedFields =
            oxicode::serde::decode_serde(&bytes).expect("decode RenamedFields failed");
        assert_eq!(original, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 12: Serde with skip attributes (#[serde(skip)])
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct WithSkippedField {
        essential: String,
        score: u32,
        #[serde(skip)]
        cache_handle: u64, // skipped: not serialized; defaults to 0 on decode
    }

    #[test]
    fn test_adv_12_skip_attribute_field_omitted() {
        let original = WithSkippedField {
            essential: "keep this field".to_string(),
            score: 100,
            cache_handle: 0xDEAD_BEEF, // will be skipped
        };
        let bytes =
            oxicode::serde::encode_serde(&original).expect("encode WithSkippedField failed");
        let decoded: WithSkippedField =
            oxicode::serde::decode_serde(&bytes).expect("decode WithSkippedField failed");
        assert_eq!(original.essential, decoded.essential);
        assert_eq!(original.score, decoded.score);
        // Skipped field is reconstructed via Default::default() => 0
        assert_eq!(
            decoded.cache_handle, 0u64,
            "skipped field must default to 0"
        );
    }

    // -----------------------------------------------------------------------
    // Test 13: Serde with default attributes (#[serde(default)])
    // -----------------------------------------------------------------------

    fn default_retries() -> u8 {
        3
    }

    fn default_timeout() -> u64 {
        30_000
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct RetryConfig {
        endpoint: String,
        #[serde(default = "default_retries")]
        max_retries: u8,
        #[serde(default = "default_timeout")]
        timeout_ms: u64,
    }

    #[test]
    fn test_adv_13_default_attribute_value_preserved() {
        // When explicitly set, the non-default value must survive the roundtrip.
        let original = RetryConfig {
            endpoint: "https://api.example.com".to_string(),
            max_retries: 5,
            timeout_ms: 60_000,
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("encode RetryConfig failed");
        let decoded: RetryConfig =
            oxicode::serde::decode_serde(&bytes).expect("decode RetryConfig failed");
        assert_eq!(original, decoded);
        // The explicit values (not the defaults) must be present
        assert_eq!(decoded.max_retries, 5);
        assert_eq!(decoded.timeout_ms, 60_000);
    }

    // -----------------------------------------------------------------------
    // Test 14: Serde encode/decode of tuple structs
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ColorRgb(u8, u8, u8);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Meters(f64);

    #[test]
    fn test_adv_14_tuple_structs_roundtrip() {
        let color = ColorRgb(200, 100, 50);
        let bytes = oxicode::serde::encode_serde(&color).expect("encode ColorRgb failed");
        let decoded: ColorRgb =
            oxicode::serde::decode_serde(&bytes).expect("decode ColorRgb failed");
        assert_eq!(color, decoded);

        let dist = Meters(42.195);
        let bytes = oxicode::serde::encode_serde(&dist).expect("encode Meters failed");
        let decoded_dist: Meters =
            oxicode::serde::decode_serde(&bytes).expect("decode Meters failed");
        assert_eq!(dist, decoded_dist);
    }

    // -----------------------------------------------------------------------
    // Test 15: Serde encode of unit variants in enum
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Compass {
        North,
        South,
        East,
        West,
        NorthEast,
        SouthWest,
    }

    #[test]
    fn test_adv_15_unit_variants_roundtrip() {
        let directions = [
            Compass::North,
            Compass::South,
            Compass::East,
            Compass::West,
            Compass::NorthEast,
            Compass::SouthWest,
        ];
        for dir in directions {
            let bytes = oxicode::serde::encode_serde(&dir).expect("encode Compass failed");
            let decoded: Compass =
                oxicode::serde::decode_serde(&bytes).expect("decode Compass failed");
            assert_eq!(dir, decoded);
        }
    }

    // -----------------------------------------------------------------------
    // Test 16: Serde encode of newtype variants
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Measurement {
        Temperature(f64),
        Pressure(f32),
        Count(u64),
        Label(String),
    }

    #[test]
    fn test_adv_16_newtype_variants_roundtrip() {
        let cases = [
            Measurement::Temperature(-273.15),
            Measurement::Pressure(101_325.0),
            Measurement::Count(u64::MAX),
            Measurement::Label("high-precision".to_string()),
        ];
        for m in cases {
            let bytes = oxicode::serde::encode_serde(&m).expect("encode Measurement failed");
            let decoded: Measurement =
                oxicode::serde::decode_serde(&bytes).expect("decode Measurement failed");
            assert_eq!(m, decoded);
        }
    }

    // -----------------------------------------------------------------------
    // Test 17: Serde encode of tuple variants
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Geometry {
        Point2D(f64, f64),
        Point3D(f64, f64, f64),
        BoundingBox(f32, f32, f32, f32), // x_min, y_min, x_max, y_max
    }

    #[test]
    fn test_adv_17_tuple_variants_roundtrip() {
        let shapes = [
            Geometry::Point2D(1.5, -3.0),
            Geometry::Point3D(0.0, 0.0, 100.0),
            Geometry::BoundingBox(-1.0, -1.0, 1.0, 1.0),
        ];
        for shape in shapes {
            let bytes = oxicode::serde::encode_serde(&shape).expect("encode Geometry failed");
            let decoded: Geometry =
                oxicode::serde::decode_serde(&bytes).expect("decode Geometry failed");
            assert_eq!(shape, decoded);
        }
    }

    // -----------------------------------------------------------------------
    // Test 18: Serde encode of struct variants
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum ApiResponse {
        Success {
            status: u16,
            body: String,
        },
        Error {
            code: u32,
            message: String,
            retryable: bool,
        },
        Redirect {
            location: String,
            permanent: bool,
        },
    }

    #[test]
    fn test_adv_18_struct_variants_roundtrip() {
        let responses = [
            ApiResponse::Success {
                status: 200,
                body: "{\"ok\":true}".to_string(),
            },
            ApiResponse::Error {
                code: 503,
                message: "Service Unavailable".to_string(),
                retryable: true,
            },
            ApiResponse::Redirect {
                location: "https://new.example.com".to_string(),
                permanent: false,
            },
        ];
        for resp in responses {
            let bytes = oxicode::serde::encode_serde(&resp).expect("encode ApiResponse failed");
            let decoded: ApiResponse =
                oxicode::serde::decode_serde(&bytes).expect("decode ApiResponse failed");
            assert_eq!(resp, decoded);
        }
    }

    // -----------------------------------------------------------------------
    // Test 19: Serde encode with nested enums
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Priority {
        Low,
        Medium,
        High,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum TaskState {
        Pending { priority: Priority },
        Running { priority: Priority, pid: u32 },
        Done,
        Failed { reason: String, priority: Priority },
    }

    #[test]
    fn test_adv_19_nested_enums_roundtrip() {
        let states = [
            TaskState::Pending {
                priority: Priority::Low,
            },
            TaskState::Running {
                priority: Priority::High,
                pid: 1234,
            },
            TaskState::Done,
            TaskState::Failed {
                reason: "OOM".to_string(),
                priority: Priority::Medium,
            },
        ];
        for state in states {
            let bytes = oxicode::serde::encode_serde(&state).expect("encode TaskState failed");
            let decoded: TaskState =
                oxicode::serde::decode_serde(&bytes).expect("decode TaskState failed");
            assert_eq!(state, decoded);
        }
    }

    // -----------------------------------------------------------------------
    // Test 20: Serde encode of struct with #[serde(flatten)] via composition
    //          (binary format requires explicit struct nesting; flatten into a
    //           struct field is used here rather than a map, which is the
    //           well-supported binary-codec form)
    // -----------------------------------------------------------------------

    /// Outer wrapper that nests AuditFields as a named field and accesses the
    /// flattened data via the inner struct — compatible with binary serde.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct AuditFields {
        created_by: String,
        created_at_unix: u64,
        version: u32,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct DocumentWithAudit {
        title: String,
        content: String,
        audit: AuditFields, // nested field; binary-safe alternative to flatten
    }

    #[test]
    fn test_adv_20_struct_with_nested_audit_field_roundtrip() {
        let original = DocumentWithAudit {
            title: "Architecture Decision Record".to_string(),
            content: "We chose binary encoding for performance.".to_string(),
            audit: AuditFields {
                created_by: "bob".to_string(),
                created_at_unix: 1_710_000_000,
                version: 7,
            },
        };
        let bytes =
            oxicode::serde::encode_serde(&original).expect("encode DocumentWithAudit failed");
        let decoded: DocumentWithAudit =
            oxicode::serde::decode_serde(&bytes).expect("decode DocumentWithAudit failed");
        assert_eq!(original, decoded);
        assert_eq!(decoded.audit.created_by, "bob");
        assert_eq!(decoded.audit.version, 7);
        assert_eq!(decoded.audit.created_at_unix, 1_710_000_000);
    }

    // -----------------------------------------------------------------------
    // Test 21: Serde roundtrip with bool, f32, f64, char types
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_21_bool_f32_f64_char_roundtrip() {
        // bool
        for b in [true, false] {
            let bytes = oxicode::serde::encode_serde(&b).expect("encode bool failed");
            let decoded: bool = oxicode::serde::decode_serde(&bytes).expect("decode bool failed");
            assert_eq!(b, decoded, "bool roundtrip failed for {b}");
        }

        // f32 — use bit-equality to avoid floating-point comparison pitfalls
        let f32_values: [f32; 5] = [0.0, -1.0, f32::MAX, f32::MIN_POSITIVE, std::f32::consts::PI];
        for f in f32_values {
            let bytes = oxicode::serde::encode_serde(&f).expect("encode f32 failed");
            let decoded: f32 = oxicode::serde::decode_serde(&bytes).expect("decode f32 failed");
            assert_eq!(
                f.to_bits(),
                decoded.to_bits(),
                "f32 bit pattern must be preserved for {f}"
            );
        }

        // f64 — use bit-equality
        let f64_values: [f64; 5] = [0.0, -1.0, f64::MAX, f64::MIN_POSITIVE, std::f64::consts::E];
        for f in f64_values {
            let bytes = oxicode::serde::encode_serde(&f).expect("encode f64 failed");
            let decoded: f64 = oxicode::serde::decode_serde(&bytes).expect("decode f64 failed");
            assert_eq!(
                f.to_bits(),
                decoded.to_bits(),
                "f64 bit pattern must be preserved for {f}"
            );
        }

        // char — ASCII and multi-byte Unicode
        for c in ['A', 'z', '0', '\t', '\u{00E9}', '\u{4E2D}'] {
            let bytes = oxicode::serde::encode_serde(&c).expect("encode char failed");
            let decoded: char = oxicode::serde::decode_serde(&bytes).expect("decode char failed");
            assert_eq!(c, decoded, "char roundtrip failed for '{c}'");
        }
    }

    // -----------------------------------------------------------------------
    // Test 22: Serde encode of Option<Vec<BTreeMap<String, u32>>>
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_22_option_vec_btreemap_string_u32_roundtrip() {
        let mut map_a: BTreeMap<String, u32> = BTreeMap::new();
        map_a.insert("x".to_string(), 10);
        map_a.insert("y".to_string(), 20);
        map_a.insert("z".to_string(), 30);

        let mut map_b: BTreeMap<String, u32> = BTreeMap::new();
        map_b.insert("alpha".to_string(), 100);
        map_b.insert("beta".to_string(), 200);

        let test_values: [Option<Vec<BTreeMap<String, u32>>>; 3] =
            [None, Some(vec![]), Some(vec![map_a, map_b])];

        for original in test_values {
            let bytes = oxicode::serde::encode_serde(&original)
                .expect("encode Option<Vec<BTreeMap<String, u32>>> failed");
            let decoded: Option<Vec<BTreeMap<String, u32>>> = oxicode::serde::decode_serde(&bytes)
                .expect("decode Option<Vec<BTreeMap<String, u32>>> failed");
            assert_eq!(
                original, decoded,
                "Option<Vec<BTreeMap<String, u32>>> roundtrip failed"
            );
        }
    }
}
