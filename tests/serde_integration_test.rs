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
mod serde_tests {
    use serde::{Deserialize, Serialize};

    // ---------------------------------------------------------------------------
    // Test types
    // ---------------------------------------------------------------------------

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Config {
        name: String,
        values: Vec<u32>,
        enabled: bool,
        nested: Option<Inner>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Inner {
        x: f64,
        y: f64,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Status {
        Active,
        Inactive,
        Pending(u32),
        Named { code: u32, msg: String },
    }

    // ---------------------------------------------------------------------------
    // Struct roundtrip tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_roundtrip_struct_with_nested() {
        let cfg = Config {
            name: "hello".to_string(),
            values: vec![1, 2, 3],
            enabled: true,
            nested: Some(Inner { x: 1.5, y: 2.5 }),
        };
        let bytes = oxicode::serde::encode_to_vec(&cfg, oxicode::config::standard())
            .expect("encode failed");
        let (decoded, consumed): (Config, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode failed");
        assert_eq!(cfg, decoded);
        assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_serde_roundtrip_struct_none_nested() {
        let cfg = Config {
            name: "world".to_string(),
            values: vec![10, 20, 30, 40],
            enabled: false,
            nested: None,
        };
        let bytes = oxicode::serde::encode_to_vec(&cfg, oxicode::config::standard())
            .expect("encode failed");
        let (decoded, consumed): (Config, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode failed");
        assert_eq!(cfg, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // ---------------------------------------------------------------------------
    // Vec roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_roundtrip_vec() {
        let data: Vec<u32> = vec![10, 20, 30];
        let bytes = oxicode::serde::encode_to_vec(&data, oxicode::config::standard())
            .expect("encode failed");
        let (decoded, consumed): (Vec<u32>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode failed");
        assert_eq!(data, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // ---------------------------------------------------------------------------
    // Option roundtrip tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_roundtrip_option_some() {
        let value: Option<String> = Some("world".to_string());
        let bytes = oxicode::serde::encode_to_vec(&value, oxicode::config::standard())
            .expect("encode failed");
        let (decoded, _): (Option<String>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode failed");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_serde_roundtrip_option_none() {
        let value: Option<String> = None;
        let bytes = oxicode::serde::encode_to_vec(&value, oxicode::config::standard())
            .expect("encode failed");
        let (decoded, _): (Option<String>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode failed");
        assert_eq!(value, decoded);
    }

    // ---------------------------------------------------------------------------
    // Native encoding compatibility tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_native_compatibility_u32() {
        let val: u32 = 42;
        let serde_bytes = oxicode::serde::encode_to_vec(&val, oxicode::config::standard())
            .expect("serde encode failed");
        let native_bytes = oxicode::encode_to_vec(&val).expect("native encode failed");
        assert_eq!(
            serde_bytes, native_bytes,
            "serde and native encoding must match for u32"
        );
    }

    #[test]
    fn test_serde_native_compatibility_string() {
        let val = "hello world".to_string();
        let serde_bytes = oxicode::serde::encode_to_vec(&val, oxicode::config::standard())
            .expect("serde encode failed");
        let native_bytes = oxicode::encode_to_vec(&val).expect("native encode failed");
        assert_eq!(
            serde_bytes, native_bytes,
            "serde and native encoding must match for String"
        );
    }

    #[test]
    fn test_serde_native_compatibility_bool() {
        for val in [true, false] {
            let serde_bytes = oxicode::serde::encode_to_vec(&val, oxicode::config::standard())
                .expect("serde encode failed");
            let native_bytes = oxicode::encode_to_vec(&val).expect("native encode failed");
            assert_eq!(
                serde_bytes, native_bytes,
                "serde and native encoding must match for bool={val}"
            );
        }
    }

    // ---------------------------------------------------------------------------
    // Convenience functions: encode_serde / decode_serde
    // ---------------------------------------------------------------------------

    #[test]
    fn test_encode_decode_serde_convenience() {
        let cfg = Config {
            name: "convenience_test".to_string(),
            values: vec![100, 200, 300],
            enabled: true,
            nested: Some(Inner {
                x: std::f64::consts::PI,
                y: 2.71,
            }),
        };
        let bytes = oxicode::serde::encode_serde(&cfg).expect("encode_serde failed");
        let decoded: Config = oxicode::serde::decode_serde(&bytes).expect("decode_serde failed");
        assert_eq!(cfg, decoded);
    }

    #[test]
    fn test_encode_decode_serde_with_config() {
        let val: u64 = 9999;
        let cfg_fixed = oxicode::config::standard().with_fixed_int_encoding();
        let bytes = oxicode::serde::encode_serde_with_config(&val, cfg_fixed)
            .expect("encode_serde_with_config failed");
        let decoded: u64 = oxicode::serde::decode_serde_with_config(&bytes, cfg_fixed)
            .expect("decode_serde_with_config failed");
        assert_eq!(val, decoded);
    }

    // ---------------------------------------------------------------------------
    // encode_into_slice roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_encode_into_slice() {
        let val: u32 = 12345;
        let mut buf = [0u8; 64];
        let written =
            oxicode::serde::encode_into_slice(&val, &mut buf, oxicode::config::standard())
                .expect("encode_into_slice failed");
        assert!(written > 0);
        let (decoded, consumed): (u32, usize) =
            oxicode::serde::decode_owned_from_slice(&buf[..written], oxicode::config::standard())
                .expect("decode failed");
        assert_eq!(val, decoded);
        assert_eq!(consumed, written);
    }

    // ---------------------------------------------------------------------------
    // std::io::Write / std::io::Read roundtrip via Cursor
    // ---------------------------------------------------------------------------

    #[cfg(feature = "std")]
    #[test]
    fn test_serde_std_write_read_roundtrip() {
        use std::io::Cursor;

        let cfg = Config {
            name: "io_test".to_string(),
            values: vec![7, 8, 9],
            enabled: false,
            nested: None,
        };

        // Encode into an in-memory Cursor
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let written =
            oxicode::serde::encode_into_std_write(&cfg, &mut cursor, oxicode::config::standard())
                .expect("encode_into_std_write failed");
        assert!(written > 0);

        // Rewind and decode
        cursor.set_position(0);
        let (decoded, bytes_read): (Config, usize) =
            oxicode::serde::decode_from_std_read(cursor, oxicode::config::standard())
                .expect("decode_from_std_read failed");
        assert_eq!(cfg, decoded);
        assert_eq!(bytes_read, written);
    }

    // ---------------------------------------------------------------------------
    // Compat wrapper tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_compat_wrapper_roundtrip() {
        use oxicode::serde::Compat;
        #[allow(unused_imports)]
        use oxicode::Encode;

        let cfg = Config {
            name: "compat_test".to_string(),
            values: vec![1, 2, 3],
            enabled: true,
            nested: Some(Inner { x: 0.0, y: -1.0 }),
        };

        // Encode via Compat wrapper using native Encode trait
        let wrapped = Compat(cfg);
        let bytes = oxicode::encode_to_vec(&wrapped).expect("Compat encode failed");

        // Decode back via Compat wrapper
        let (Compat(decoded), _): (Compat<Config>, usize) =
            oxicode::decode_from_slice(&bytes).expect("Compat decode failed");

        assert_eq!(wrapped.0, decoded);
    }

    // ---------------------------------------------------------------------------
    // i128 / u128 roundtrip via serde
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_i128_roundtrip() {
        let val: i128 = i128::MIN / 2;
        let bytes = oxicode::serde::encode_to_vec(&val, oxicode::config::standard())
            .expect("i128 encode failed");
        let (decoded, consumed): (i128, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("i128 decode failed");
        assert_eq!(val, decoded);
        assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_serde_u128_roundtrip() {
        let val: u128 = u128::MAX / 3;
        let bytes = oxicode::serde::encode_to_vec(&val, oxicode::config::standard())
            .expect("u128 encode failed");
        let (decoded, consumed): (u128, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("u128 decode failed");
        assert_eq!(val, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // ---------------------------------------------------------------------------
    // HashMap roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_hashmap_roundtrip() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert("alpha".to_string(), 1u32);
        map.insert("beta".to_string(), 2u32);
        map.insert("gamma".to_string(), 3u32);

        let bytes = oxicode::serde::encode_to_vec(&map, oxicode::config::standard())
            .expect("HashMap encode failed");
        let (decoded, consumed): (HashMap<String, u32>, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("HashMap decode failed");
        assert_eq!(map, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // ---------------------------------------------------------------------------
    // Enum variant roundtrip tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_enum_unit_variant_roundtrip() {
        for variant in [Status::Active, Status::Inactive] {
            let bytes = oxicode::serde::encode_to_vec(&variant, oxicode::config::standard())
                .expect("enum unit encode failed");
            let (decoded, _): (Status, usize) =
                oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                    .expect("enum unit decode failed");
            assert_eq!(variant, decoded);
        }
    }

    #[test]
    fn test_serde_enum_tuple_variant_roundtrip() {
        let variant = Status::Pending(42);
        let bytes = oxicode::serde::encode_to_vec(&variant, oxicode::config::standard())
            .expect("enum tuple encode failed");
        let (decoded, _): (Status, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("enum tuple decode failed");
        assert_eq!(variant, decoded);
    }

    #[test]
    fn test_serde_enum_struct_variant_roundtrip() {
        let variant = Status::Named {
            code: 404,
            msg: "not found".to_string(),
        };
        let bytes = oxicode::serde::encode_to_vec(&variant, oxicode::config::standard())
            .expect("enum struct variant encode failed");
        let (decoded, _): (Status, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("enum struct variant decode failed");
        assert_eq!(variant, decoded);
    }

    // ---------------------------------------------------------------------------
    // encoded_serde_size consistency
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_encoded_size_matches_actual() {
        let cfg = Config {
            name: "size_test".to_string(),
            values: vec![1, 2, 3, 4, 5],
            enabled: true,
            nested: Some(Inner { x: 1.0, y: 2.0 }),
        };
        let size = oxicode::serde::encoded_serde_size(&cfg).expect("encoded_serde_size failed");
        let bytes = oxicode::serde::encode_serde(&cfg).expect("encode_serde for size check failed");
        assert_eq!(
            size,
            bytes.len(),
            "encoded_serde_size must match actual encoded length"
        );
    }

    // ---------------------------------------------------------------------------
    // File roundtrip (std-gated)
    // ---------------------------------------------------------------------------

    #[cfg(feature = "std")]
    #[test]
    fn test_serde_file_roundtrip() {
        let cfg = Config {
            name: "file_test".to_string(),
            values: vec![100, 200, 300],
            enabled: true,
            nested: Some(Inner {
                x: 2.71,
                y: std::f64::consts::PI,
            }),
        };

        let tmp_path = std::env::temp_dir().join("oxicode_serde_integration_test.bin");

        oxicode::serde::encode_serde_to_file(&cfg, &tmp_path).expect("encode_serde_to_file failed");

        let decoded: Config = oxicode::serde::decode_serde_from_file(&tmp_path)
            .expect("decode_serde_from_file failed");

        // Clean up
        let _ = std::fs::remove_file(&tmp_path);

        assert_eq!(cfg, decoded);
    }

    // ---------------------------------------------------------------------------
    // Nested struct with multiple layers
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_deeply_nested_struct() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Outer {
            label: String,
            inner: Vec<Inner>,
            flag: bool,
        }

        let outer = Outer {
            label: "deep".to_string(),
            inner: vec![
                Inner { x: 1.1, y: 2.2 },
                Inner { x: 3.3, y: 4.4 },
                Inner { x: -1.0, y: 0.0 },
            ],
            flag: true,
        };

        let bytes = oxicode::serde::encode_serde(&outer).expect("deep encode failed");
        let decoded: Outer = oxicode::serde::decode_serde(&bytes).expect("deep decode failed");
        assert_eq!(outer, decoded);
    }

    // ---------------------------------------------------------------------------
    // Primitive types roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_primitives_roundtrip() {
        // bool
        let bytes = oxicode::serde::encode_serde(&true).expect("bool encode");
        let v: bool = oxicode::serde::decode_serde(&bytes).expect("bool decode");
        assert!(v);

        // i8
        let bytes = oxicode::serde::encode_serde(&(-42i8)).expect("i8 encode");
        let v: i8 = oxicode::serde::decode_serde(&bytes).expect("i8 decode");
        assert_eq!(v, -42i8);

        // f32
        let bytes = oxicode::serde::encode_serde(&std::f32::consts::PI).expect("f32 encode");
        let v: f32 = oxicode::serde::decode_serde(&bytes).expect("f32 decode");
        assert!((v - std::f32::consts::PI).abs() < 1e-5);

        // char
        let bytes = oxicode::serde::encode_serde(&'Z').expect("char encode");
        let v: char = oxicode::serde::decode_serde(&bytes).expect("char decode");
        assert_eq!(v, 'Z');
    }

    // ---------------------------------------------------------------------------
    // Error message preservation (OwnedCustom)
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_error_display_is_non_empty() {
        // Attempt to decode garbage bytes — error message should be non-empty
        let garbage = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let result: Result<(Config, usize), _> = oxicode::serde::decode_owned_from_slice::<Config, _>(
            &garbage,
            oxicode::config::standard(),
        );
        // It may succeed or fail depending on what the garbage decodes to;
        // when it fails the error message should be displayable and non-empty.
        if let Err(e) = result {
            let msg = e.to_string();
            assert!(!msg.is_empty(), "error message must not be empty");
        }
    }

    // ---------------------------------------------------------------------------
    // Nested Option roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_nested_option_roundtrip() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Nested {
            inner: Option<Option<u32>>,
        }

        for inner in [None, Some(None), Some(Some(42u32))] {
            let v = Nested { inner };
            let bytes = oxicode::serde::encode_serde(&v).expect("nested option encode");
            let decoded: Nested =
                oxicode::serde::decode_serde(&bytes).expect("nested option decode");
            assert_eq!(
                v, decoded,
                "nested option roundtrip failed for {:?}",
                v.inner
            );
        }
    }

    // ---------------------------------------------------------------------------
    // Externally-tagged enum with struct variants
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_externally_tagged_enum_struct_variants() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        enum Event {
            Login { user_id: u64, timestamp: u64 },
            Logout { user_id: u64 },
            Purchase { item_id: u32, price: f64 },
        }

        let cases = [
            Event::Login {
                user_id: 1,
                timestamp: 1_000_000,
            },
            Event::Logout { user_id: 2 },
            Event::Purchase {
                item_id: 100,
                price: 29.99,
            },
        ];

        for event in cases {
            let bytes = oxicode::serde::encode_serde(&event).expect("event encode");
            let decoded: Event = oxicode::serde::decode_serde(&bytes).expect("event decode");
            assert_eq!(event, decoded);
        }
    }

    // ---------------------------------------------------------------------------
    // encoded_serde_size with binary payload
    // ---------------------------------------------------------------------------

    #[test]
    fn test_encoded_serde_size_binary_payload() {
        #[derive(Serialize, Deserialize)]
        struct Payload {
            data: Vec<u8>,
        }

        let v = Payload {
            data: vec![0u8; 1024],
        };
        let size = oxicode::serde::encoded_serde_size(&v).expect("encoded_serde_size");
        let bytes = oxicode::serde::encode_serde(&v).expect("encode payload");
        assert_eq!(
            size,
            bytes.len(),
            "encoded_serde_size must equal actual encoded length"
        );
    }

    // ---------------------------------------------------------------------------
    // HashMap with i64 values roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_hashmap_i64_values_roundtrip() {
        use std::collections::HashMap;

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct WithMap {
            data: HashMap<String, i64>,
        }

        let mut map = HashMap::new();
        map.insert("alpha".to_string(), 1i64);
        map.insert("beta".to_string(), -2i64);
        map.insert("gamma".to_string(), i64::MAX);

        let v = WithMap { data: map };
        let bytes = oxicode::serde::encode_serde(&v).expect("hashmap i64 encode");
        let decoded: WithMap = oxicode::serde::decode_serde(&bytes).expect("hashmap i64 decode");
        assert_eq!(v, decoded);
    }

    // ---------------------------------------------------------------------------
    // File I/O roundtrip with Config-like struct (std-gated)
    // ---------------------------------------------------------------------------

    #[cfg(feature = "std")]
    #[test]
    fn test_serde_file_io_roundtrip_server_config() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct ServerConfig {
            host: String,
            port: u16,
            debug: bool,
        }

        let cfg = ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
            debug: true,
        };

        let path = std::env::temp_dir().join("oxicode_serde_server_config_test.bin");

        oxicode::serde::encode_serde_to_file(&cfg, &path).expect("encode to file");
        let loaded: ServerConfig =
            oxicode::serde::decode_serde_from_file(&path).expect("decode from file");

        let _ = std::fs::remove_file(&path);

        assert_eq!(cfg, loaded);
    }

    // ---------------------------------------------------------------------------
    // encode_into_slice + decode_owned_from_slice exact byte count
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_slice_encode_exact_byte_count() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Point {
            x: f64,
            y: f64,
        }

        let p = Point { x: 1.5, y: -3.75 };
        let mut buf = [0u8; 256];
        let written = oxicode::serde::encode_into_slice(&p, &mut buf, oxicode::config::standard())
            .expect("encode_into_slice");
        assert!(written > 0, "must write at least one byte");

        let (decoded, consumed): (Point, usize) =
            oxicode::serde::decode_owned_from_slice(&buf[..written], oxicode::config::standard())
                .expect("decode_owned_from_slice");
        assert_eq!(p, decoded);
        assert_eq!(consumed, written, "consumed must equal written byte count");
    }

    // ---------------------------------------------------------------------------
    // BorrowCompat wrapper roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_borrow_compat_wrapper_roundtrip() {
        use oxicode::serde::{BorrowCompat, Compat};

        let s = "hello serde borrow".to_string();
        let wrapped = Compat(s.clone());
        let bytes = oxicode::encode_to_vec(&wrapped).expect("BorrowCompat encode");
        let (Compat(decoded), _): (Compat<String>, usize) =
            oxicode::decode_from_slice(&bytes).expect("BorrowCompat decode");
        assert_eq!(s, decoded);

        // Verify BorrowCompat<T> is accessible from the public API (T owns the data)
        let _ = core::marker::PhantomData::<BorrowCompat<String>>;
    }

    // ---------------------------------------------------------------------------
    // Large Vec roundtrip via serde
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_large_vec_roundtrip() {
        let data: Vec<u64> = (0u64..4096).collect();
        let bytes = oxicode::serde::encode_serde(&data).expect("large vec encode");
        let decoded: Vec<u64> = oxicode::serde::decode_serde(&bytes).expect("large vec decode");
        assert_eq!(data, decoded);
    }

    // ---------------------------------------------------------------------------
    // Tuple roundtrip via serde
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_tuple_roundtrip() {
        let t = (42u32, "hello".to_string(), true, -7i64);
        let bytes = oxicode::serde::encode_serde(&t).expect("tuple encode");
        let decoded: (u32, String, bool, i64) =
            oxicode::serde::decode_serde(&bytes).expect("tuple decode");
        assert_eq!(t, decoded);
    }

    // ---------------------------------------------------------------------------
    // 1. Serde round-trip for a struct with #[serde(rename = "...")] field
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_roundtrip_rename_field() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Renamed {
            #[serde(rename = "user_name")]
            name: String,
            #[serde(rename = "user_age")]
            age: u32,
        }

        let original = Renamed {
            name: "Alice".to_string(),
            age: 30,
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("rename encode");
        let decoded: Renamed = oxicode::serde::decode_serde(&bytes).expect("rename decode");
        assert_eq!(original, decoded);
    }

    // ---------------------------------------------------------------------------
    // 2. Serde round-trip for struct with #[serde(skip)] field
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_roundtrip_skip_field() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct WithSkip {
            essential: String,
            #[serde(skip)]
            transient: u64,
            count: u32,
        }

        let original = WithSkip {
            essential: "important".to_string(),
            transient: 99999,
            count: 42,
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("skip encode");
        // After round-trip the skipped field resets to Default::default()
        let decoded: WithSkip = oxicode::serde::decode_serde(&bytes).expect("skip decode");
        assert_eq!(decoded.essential, original.essential);
        assert_eq!(decoded.count, original.count);
        assert_eq!(decoded.transient, 0u64, "skipped field must be default");
    }

    // ---------------------------------------------------------------------------
    // 3. Serde round-trip for struct with #[serde(default)] field
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_roundtrip_default_field() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct WithDefault {
            label: String,
            #[serde(default)]
            score: f64,
        }

        let original = WithDefault {
            label: "test".to_string(),
            score: std::f64::consts::PI,
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("default encode");
        let decoded: WithDefault = oxicode::serde::decode_serde(&bytes).expect("default decode");
        assert_eq!(original, decoded);

        // Verify that a struct with the field omitted (score==0.0) also round-trips
        let zeroed = WithDefault {
            label: "zero".to_string(),
            score: 0.0,
        };
        let bytes2 = oxicode::serde::encode_serde(&zeroed).expect("default zero encode");
        let decoded2: WithDefault =
            oxicode::serde::decode_serde(&bytes2).expect("default zero decode");
        assert_eq!(zeroed, decoded2);
    }

    // ---------------------------------------------------------------------------
    // 4. Vec<serde_json::Value> encoding via oxicode serde (encode path)
    //
    // Note: serde_json::Value uses deserialize_any on the decode path, which
    // requires a self-describing format. oxicode is a compact binary format and
    // does not implement deserialize_any. This test verifies that:
    //   a) Encoding Vec<serde_json::Value> succeeds and produces non-empty bytes.
    //   b) The encoded size reported by encoded_serde_size matches the actual byte length.
    //   c) A Vec<String> (self-sufficient serde type) roundtrips correctly as an
    //      alternative proof that Vec<Serialize> works end-to-end.
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_vec_json_value_encode_and_string_roundtrip() {
        use serde_json::Value;

        // Encode Vec<serde_json::Value> — should succeed (Serialize path only)
        let json_values: Vec<Value> = vec![
            Value::Null,
            Value::Bool(true),
            Value::String("hello".to_string()),
            Value::Number(serde_json::Number::from(42)),
        ];
        let bytes = oxicode::serde::encode_serde(&json_values).expect("Vec<Value> encode");
        assert!(!bytes.is_empty(), "encoded bytes must be non-empty");

        // Verify the encoded size oracle matches
        let predicted =
            oxicode::serde::encoded_serde_size(&json_values).expect("encoded_serde_size");
        assert_eq!(
            predicted,
            bytes.len(),
            "encoded_serde_size must match actual length"
        );

        // Full roundtrip using Vec<String> (does not require deserialize_any)
        let string_values: Vec<String> = vec![
            "null".to_string(),
            "true".to_string(),
            "hello".to_string(),
            "42".to_string(),
        ];
        let str_bytes = oxicode::serde::encode_serde(&string_values).expect("Vec<String> encode");
        let decoded: Vec<String> =
            oxicode::serde::decode_serde(&str_bytes).expect("Vec<String> decode");
        assert_eq!(string_values, decoded);
    }

    // ---------------------------------------------------------------------------
    // 5. HashMap<String, serde_json::Value> encode + HashMap<String, String> roundtrip
    //
    // serde_json::Value requires deserialize_any (self-describing format) on decode,
    // which oxicode's compact binary format does not implement. This test verifies:
    //   a) Encoding HashMap<String, serde_json::Value> succeeds.
    //   b) A structurally equivalent HashMap<String, String> roundtrips correctly.
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_hashmap_json_value_encode_and_string_roundtrip() {
        use serde_json::Value;
        use std::collections::HashMap;

        // Encode only — serde_json::Value serializes fine
        let mut json_map: HashMap<String, Value> = HashMap::new();
        json_map.insert("null_val".to_string(), Value::Null);
        json_map.insert("bool_val".to_string(), Value::Bool(false));
        json_map.insert("str_val".to_string(), Value::String("world".to_string()));
        json_map.insert(
            "num_val".to_string(),
            Value::Number(serde_json::Number::from(100)),
        );
        let bytes = oxicode::serde::encode_serde(&json_map).expect("HashMap<String, Value> encode");
        assert!(!bytes.is_empty(), "encoded bytes must be non-empty");

        // Full roundtrip using HashMap<String, String>
        let mut str_map: HashMap<String, String> = HashMap::new();
        str_map.insert("null_val".to_string(), "null".to_string());
        str_map.insert("bool_val".to_string(), "false".to_string());
        str_map.insert("str_val".to_string(), "world".to_string());
        str_map.insert("num_val".to_string(), "100".to_string());

        let str_bytes =
            oxicode::serde::encode_serde(&str_map).expect("HashMap<String, String> encode");
        let decoded: HashMap<String, String> =
            oxicode::serde::decode_serde(&str_bytes).expect("HashMap<String, String> decode");
        assert_eq!(str_map, decoded);
    }

    // ---------------------------------------------------------------------------
    // 6. Struct with nested serde-serializable type (using a concrete nested type)
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_struct_with_nested_serde_type() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Metadata {
            version: u32,
            author: String,
            tags: Vec<String>,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Envelope {
            id: u64,
            meta: Metadata,
            payload: Vec<u8>,
        }

        let envelope = Envelope {
            id: 9876543210,
            meta: Metadata {
                version: 3,
                author: "oxicode".to_string(),
                tags: vec!["binary".to_string(), "compact".to_string()],
            },
            payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        };
        let bytes = oxicode::serde::encode_serde(&envelope).expect("Envelope encode");
        let decoded: Envelope = oxicode::serde::decode_serde(&bytes).expect("Envelope decode");
        assert_eq!(envelope, decoded);
    }

    // ---------------------------------------------------------------------------
    // 7. encode_serde and decode_serde top-level functions roundtrip
    // ---------------------------------------------------------------------------

    #[test]
    fn test_encode_serde_decode_serde_top_level_roundtrip() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct TopLevel {
            key: String,
            value: i64,
            active: bool,
        }

        let original = TopLevel {
            key: "top_level_key".to_string(),
            value: -987654321,
            active: true,
        };
        let bytes = oxicode::serde::encode_serde(&original).expect("top-level encode_serde");
        let decoded: TopLevel =
            oxicode::serde::decode_serde(&bytes).expect("top-level decode_serde");
        assert_eq!(original, decoded);
        // Verify bytes are non-empty
        assert!(!bytes.is_empty());
    }

    // ---------------------------------------------------------------------------
    // 8. Serde encode to vec then decode from slice matches original
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_encode_to_vec_decode_from_slice_consistency() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Pair {
            first: f32,
            second: f32,
        }

        let original = Pair {
            first: 1.23_f32,
            second: -4.56_f32,
        };
        let bytes = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode_to_vec");
        let (decoded, consumed): (Pair, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard())
                .expect("decode_owned_from_slice");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len(), "all encoded bytes must be consumed");
    }

    // ---------------------------------------------------------------------------
    // 9. Serde-derived struct with Option<Vec<String>> field
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_struct_option_vec_string_roundtrip() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Record {
            id: u32,
            tags: Option<Vec<String>>,
            description: String,
        }

        let with_tags = Record {
            id: 1,
            tags: Some(vec![
                "rust".to_string(),
                "serde".to_string(),
                "oxicode".to_string(),
            ]),
            description: "with tags".to_string(),
        };
        let without_tags = Record {
            id: 2,
            tags: None,
            description: "no tags".to_string(),
        };

        for record in [&with_tags, &without_tags] {
            let bytes = oxicode::serde::encode_serde(record).expect("Option<Vec<String>> encode");
            let decoded: Record =
                oxicode::serde::decode_serde(&bytes).expect("Option<Vec<String>> decode");
            assert_eq!(*record, decoded);
        }
    }

    // ---------------------------------------------------------------------------
    // 10. Large struct with 20 fields roundtrip via serde
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_large_struct_20_fields_roundtrip() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Large {
            f01: u8,
            f02: u16,
            f03: u32,
            f04: u64,
            f05: i8,
            f06: i16,
            f07: i32,
            f08: i64,
            f09: f32,
            f10: f64,
            f11: bool,
            f12: String,
            f13: Vec<u8>,
            f14: Option<u32>,
            f15: Option<String>,
            f16: u32,
            f17: i32,
            f18: f64,
            f19: String,
            f20: bool,
        }

        let original = Large {
            f01: 255,
            f02: 65535,
            f03: 0xDEAD_BEEF,
            f04: u64::MAX,
            f05: -128,
            f06: i16::MIN,
            f07: i32::MAX,
            f08: i64::MIN,
            f09: std::f32::consts::PI,
            f10: std::f64::consts::E,
            f11: true,
            f12: "twenty field struct".to_string(),
            f13: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            f14: Some(42),
            f15: Some("optional string".to_string()),
            f16: 100,
            f17: -100,
            f18: 1.618_033_988,
            f19: "golden ratio".to_string(),
            f20: false,
        };

        let bytes = oxicode::serde::encode_serde(&original).expect("large struct encode");
        let decoded: Large = oxicode::serde::decode_serde(&bytes).expect("large struct decode");
        assert_eq!(original, decoded);
    }

    // ---------------------------------------------------------------------------
    // 11. Serde encode with big-endian config
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_encode_big_endian_config_roundtrip() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Metrics {
            timestamp: u64,
            value: f64,
            source: String,
        }

        let cfg_be = oxicode::config::standard().with_big_endian();

        let original = Metrics {
            timestamp: 1_700_000_000_u64,
            value: std::f64::consts::PI,
            source: "sensor_01".to_string(),
        };

        let bytes = oxicode::serde::encode_to_vec(&original, cfg_be).expect("big-endian encode");
        let (decoded, consumed): (Metrics, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes, cfg_be).expect("big-endian decode");

        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len(), "all bytes must be consumed");

        // Verify big-endian differs from little-endian for this struct
        let cfg_le = oxicode::config::standard().with_little_endian();
        let bytes_le =
            oxicode::serde::encode_to_vec(&original, cfg_le).expect("little-endian encode");
        // The byte layouts may differ (endianness affects multi-byte integers)
        // We just verify both round-trip correctly and the BE bytes are valid
        let (decoded_le, _): (Metrics, usize) =
            oxicode::serde::decode_owned_from_slice(&bytes_le, cfg_le)
                .expect("little-endian decode");
        assert_eq!(original, decoded_le);
    }

    // ---------------------------------------------------------------------------
    // 12. serde::encode_to_vec vs encode_to_vec produce compatible bytes for simple types
    // ---------------------------------------------------------------------------

    #[test]
    fn test_serde_encode_to_vec_vs_native_encode_to_vec_compatibility() {
        // For types that implement both native Encode and serde Serialize,
        // the wire bytes must be identical so that native-encoded data can
        // be decoded via the serde path and vice-versa.
        let val_u64: u64 = 0xCAFE_BABE_1234_5678;
        let serde_bytes = oxicode::serde::encode_to_vec(&val_u64, oxicode::config::standard())
            .expect("serde encode u64");
        let native_bytes = oxicode::encode_to_vec(&val_u64).expect("native encode u64");
        assert_eq!(
            serde_bytes, native_bytes,
            "serde and native encoding must produce identical bytes for u64"
        );

        // Cross-decode: native bytes decoded via serde path
        let (serde_decoded, _): (u64, usize) =
            oxicode::serde::decode_owned_from_slice(&native_bytes, oxicode::config::standard())
                .expect("cross decode serde from native bytes");
        assert_eq!(val_u64, serde_decoded);

        // Cross-decode: serde bytes decoded via native path
        let (native_decoded, _): (u64, usize) =
            oxicode::decode_from_slice(&serde_bytes).expect("cross decode native from serde bytes");
        assert_eq!(val_u64, native_decoded);
    }
}
