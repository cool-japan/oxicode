//! Comprehensive roundtrip tests for complex real-world data structures.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::collections::{BTreeMap, HashMap};
use std::f64::consts::{E, PI};
use std::net::IpAddr;

mod complex_roundtrip_tests {
    use super::*;

    // ===== Test 1: UserProfile =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct UserProfile {
        id: u64,
        name: String,
        email: String,
        interests: Vec<String>,
    }

    #[test]
    fn test_user_profile_roundtrip() {
        let profile = UserProfile {
            id: 42,
            name: "Alice Wonderland".to_string(),
            email: "alice@example.com".to_string(),
            interests: vec![
                "Rust".to_string(),
                "serialization".to_string(),
                "distributed systems".to_string(),
            ],
        };
        let enc = encode_to_vec(&profile).expect("encode UserProfile");
        let (dec, _): (UserProfile, _) = decode_from_slice(&enc).expect("decode UserProfile");
        assert_eq!(profile, dec);
    }

    // ===== Test 2: DatabaseRecord =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DatabaseRecord {
        id: u64,
        timestamp: u64,
        data: Vec<u8>,
        metadata: HashMap<String, String>,
    }

    #[test]
    fn test_database_record_roundtrip() {
        let mut metadata = HashMap::new();
        metadata.insert("table".to_string(), "users".to_string());
        metadata.insert("version".to_string(), "3".to_string());
        metadata.insert("region".to_string(), "eu-west-1".to_string());

        let record = DatabaseRecord {
            id: 9_999_999,
            timestamp: 1_700_000_000,
            data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
            metadata,
        };
        let enc = encode_to_vec(&record).expect("encode DatabaseRecord");
        let (dec, _): (DatabaseRecord, _) = decode_from_slice(&enc).expect("decode DatabaseRecord");
        assert_eq!(record, dec);
    }

    // ===== Test 3: FlatTree (binary tree simulated with flat structure) =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TreeNode {
        value: i64,
        left_index: Option<u32>,
        right_index: Option<u32>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FlatTree {
        nodes: Vec<TreeNode>,
        root_index: u32,
    }

    #[test]
    fn test_flat_tree_roundtrip() {
        let tree = FlatTree {
            nodes: vec![
                TreeNode {
                    value: 10,
                    left_index: Some(1),
                    right_index: Some(2),
                },
                TreeNode {
                    value: 5,
                    left_index: Some(3),
                    right_index: Some(4),
                },
                TreeNode {
                    value: 15,
                    left_index: None,
                    right_index: None,
                },
                TreeNode {
                    value: 3,
                    left_index: None,
                    right_index: None,
                },
                TreeNode {
                    value: 7,
                    left_index: None,
                    right_index: None,
                },
            ],
            root_index: 0,
        };
        let enc = encode_to_vec(&tree).expect("encode FlatTree");
        let (dec, _): (FlatTree, _) = decode_from_slice(&enc).expect("decode FlatTree");
        assert_eq!(tree, dec);
    }

    // ===== Test 4: GraphEdge with PI-based weight =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct GraphEdge {
        source: u32,
        target: u32,
        weight: f64,
    }

    #[test]
    fn test_graph_edge_roundtrip() {
        let edges = vec![
            GraphEdge {
                source: 0,
                target: 1,
                weight: PI,
            },
            GraphEdge {
                source: 1,
                target: 2,
                weight: E,
            },
            GraphEdge {
                source: 2,
                target: 0,
                weight: PI / E,
            },
            GraphEdge {
                source: 3,
                target: 1,
                weight: 2.0 * PI,
            },
        ];
        let enc = encode_to_vec(&edges).expect("encode GraphEdge vec");
        let (dec, _): (Vec<GraphEdge>, _) = decode_from_slice(&enc).expect("decode GraphEdge vec");
        assert_eq!(edges, dec);
    }

    // ===== Test 5: Matrix2x2 =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Matrix2x2 {
        data: [[f64; 2]; 2],
    }

    #[test]
    fn test_matrix2x2_roundtrip() {
        let mat = Matrix2x2 {
            data: [[1.0, PI], [E, 2.0 * PI]],
        };
        let enc = encode_to_vec(&mat).expect("encode Matrix2x2");
        let (dec, _): (Matrix2x2, _) = decode_from_slice(&enc).expect("decode Matrix2x2");
        assert_eq!(mat, dec);
    }

    // ===== Test 6: Config with BTreeMap<String, Vec<String>> =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Config {
        name: String,
        settings: BTreeMap<String, Vec<String>>,
    }

    #[test]
    fn test_config_btreemap_roundtrip() {
        let mut settings = BTreeMap::new();
        settings.insert(
            "allowed_origins".to_string(),
            vec![
                "https://example.com".to_string(),
                "https://api.example.com".to_string(),
            ],
        );
        settings.insert(
            "log_levels".to_string(),
            vec!["error".to_string(), "warn".to_string(), "info".to_string()],
        );
        settings.insert(
            "features".to_string(),
            vec!["compression".to_string(), "checksum".to_string()],
        );

        let config = Config {
            name: "production".to_string(),
            settings,
        };
        let enc = encode_to_vec(&config).expect("encode Config");
        let (dec, _): (Config, _) = decode_from_slice(&enc).expect("decode Config");
        assert_eq!(config, dec);
    }

    // ===== Test 7: LogEntry with level enum =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum LogLevel {
        Trace,
        Debug,
        Info,
        Warn,
        Error,
        Fatal,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LogEntry {
        timestamp: u64,
        level: LogLevel,
        message: String,
        tags: Vec<String>,
    }

    #[test]
    fn test_log_entry_roundtrip() {
        let entry = LogEntry {
            timestamp: 1_700_123_456,
            level: LogLevel::Warn,
            message: "Connection pool exhausted, retrying...".to_string(),
            tags: vec![
                "database".to_string(),
                "pool".to_string(),
                "retry".to_string(),
            ],
        };
        let enc = encode_to_vec(&entry).expect("encode LogEntry");
        let (dec, _): (LogEntry, _) = decode_from_slice(&enc).expect("decode LogEntry");
        assert_eq!(entry, dec);
    }

    // ===== Test 8: ApiResponse generic =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ApiResponse<T: Encode + Decode> {
        status_code: u16,
        data: Option<T>,
        errors: Vec<String>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct UserData {
        user_id: u64,
        username: String,
        roles: Vec<String>,
    }

    #[test]
    fn test_api_response_generic_roundtrip() {
        let response = ApiResponse {
            status_code: 200,
            data: Some(UserData {
                user_id: 1001,
                username: "bob".to_string(),
                roles: vec!["admin".to_string(), "editor".to_string()],
            }),
            errors: vec![],
        };
        let enc = encode_to_vec(&response).expect("encode ApiResponse<UserData>");
        let (dec, _): (ApiResponse<UserData>, _) =
            decode_from_slice(&enc).expect("decode ApiResponse<UserData>");
        assert_eq!(response, dec);

        // Also test error response
        let err_response: ApiResponse<UserData> = ApiResponse {
            status_code: 422,
            data: None,
            errors: vec![
                "email: is invalid".to_string(),
                "password: too short".to_string(),
            ],
        };
        let enc2 = encode_to_vec(&err_response).expect("encode error ApiResponse");
        let (dec2, _): (ApiResponse<UserData>, _) =
            decode_from_slice(&enc2).expect("decode error ApiResponse");
        assert_eq!(err_response, dec2);
    }

    // ===== Test 9: Invoice with items Vec<(String, u32, f64)> =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Invoice {
        invoice_number: String,
        items: Vec<(String, u32, f64)>,
        total: f64,
    }

    #[test]
    fn test_invoice_roundtrip() {
        let invoice = Invoice {
            invoice_number: "INV-2026-001".to_string(),
            items: vec![
                ("Widget A".to_string(), 10, 4.99),
                ("Widget B".to_string(), 3, 19.99),
                ("Service Fee".to_string(), 1, PI * 10.0),
            ],
            total: 10.0 * 4.99 + 3.0 * 19.99 + PI * 10.0,
        };
        let enc = encode_to_vec(&invoice).expect("encode Invoice");
        let (dec, _): (Invoice, _) = decode_from_slice(&enc).expect("decode Invoice");
        assert_eq!(invoice, dec);
    }

    // ===== Test 10: Large Vec<UserProfile> (100 items) =====

    #[test]
    fn test_large_user_profile_vec_roundtrip() {
        let profiles: Vec<UserProfile> = (0..100)
            .map(|i| UserProfile {
                id: i as u64,
                name: format!("User_{:03}", i),
                email: format!("user{}@example.com", i),
                interests: vec![format!("interest_{}", i % 10), format!("hobby_{}", i % 5)],
            })
            .collect();
        let enc = encode_to_vec(&profiles).expect("encode 100 UserProfiles");
        let (dec, _): (Vec<UserProfile>, _) =
            decode_from_slice(&enc).expect("decode 100 UserProfiles");
        assert_eq!(profiles, dec);
    }

    // ===== Test 11: Document with sections =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Section {
        title: String,
        paragraphs: Vec<String>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Document {
        title: String,
        author: String,
        sections: Vec<Section>,
    }

    #[test]
    fn test_document_roundtrip() {
        let doc = Document {
            title: "OxiCode: Modern Binary Serialization".to_string(),
            author: "COOLJAPAN OU".to_string(),
            sections: vec![
                Section {
                    title: "Introduction".to_string(),
                    paragraphs: vec![
                        "OxiCode is a high-performance binary serialization library.".to_string(),
                        "It is the successor to bincode with improved ergonomics.".to_string(),
                    ],
                },
                Section {
                    title: "Getting Started".to_string(),
                    paragraphs: vec![
                        "Add oxicode to your Cargo.toml dependencies.".to_string(),
                        "Derive Encode and Decode on your types.".to_string(),
                        "Call encode_to_vec and decode_from_slice.".to_string(),
                    ],
                },
                Section {
                    title: "Advanced Features".to_string(),
                    paragraphs: vec![
                        "Compression via LZ4 or Zstd.".to_string(),
                        "CRC32 checksum integrity verification.".to_string(),
                        "Async streaming with Tokio.".to_string(),
                    ],
                },
            ],
        };
        let enc = encode_to_vec(&doc).expect("encode Document");
        let (dec, _): (Document, _) = decode_from_slice(&enc).expect("decode Document");
        assert_eq!(doc, dec);
    }

    // ===== Test 12: Nested 5-level deep struct =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level5 {
        value: f64,
        label: String,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level4 {
        inner: Level5,
        id: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level3 {
        inner: Level4,
        tag: String,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level2 {
        inner: Level3,
        active: bool,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level1 {
        inner: Level2,
        name: String,
    }

    #[test]
    fn test_nested_5_levels_roundtrip() {
        let nested = Level1 {
            name: "root".to_string(),
            inner: Level2 {
                active: true,
                inner: Level3 {
                    tag: "mid".to_string(),
                    inner: Level4 {
                        id: 7,
                        inner: Level5 {
                            value: PI * E,
                            label: "leaf".to_string(),
                        },
                    },
                },
            },
        };
        let enc = encode_to_vec(&nested).expect("encode Level1 (5-deep)");
        let (dec, _): (Level1, _) = decode_from_slice(&enc).expect("decode Level1 (5-deep)");
        assert_eq!(nested, dec);
    }

    // ===== Test 13: EventStream with 1000 events =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum Event {
        Click { x: i16, y: i16 },
        KeyPress(u8),
        Scroll { delta: i8 },
        Resize { width: u16, height: u16 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct EventStream {
        session_id: u64,
        events: Vec<Event>,
    }

    #[test]
    fn test_event_stream_1000_events_roundtrip() {
        let events: Vec<Event> = (0..1000)
            .map(|i| match i % 4 {
                0 => Event::Click {
                    x: (i % 1920) as i16,
                    y: (i % 1080) as i16,
                },
                1 => Event::KeyPress((i % 128) as u8),
                2 => Event::Scroll {
                    delta: if i % 2 == 0 { 3 } else { -3 },
                },
                _ => Event::Resize {
                    width: 1920,
                    height: 1080,
                },
            })
            .collect();
        let stream = EventStream {
            session_id: 0xDEAD_BEEF_CAFE_1234,
            events,
        };
        let enc = encode_to_vec(&stream).expect("encode EventStream 1000");
        let (dec, _): (EventStream, _) = decode_from_slice(&enc).expect("decode EventStream 1000");
        assert_eq!(stream, dec);
    }

    // ===== Test 14: Protocol with header + payload + checksum =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Protocol {
        header: [u8; 4],
        version: u8,
        payload: Vec<u8>,
        checksum: u32,
    }

    #[test]
    fn test_protocol_roundtrip() {
        let payload: Vec<u8> = (0u8..=127).collect();
        let checksum: u32 = payload.iter().map(|&b| b as u32).sum();
        let protocol = Protocol {
            header: [0x4F, 0x58, 0x49, 0x43], // "OXIC"
            version: 2,
            payload,
            checksum,
        };
        let enc = encode_to_vec(&protocol).expect("encode Protocol");
        let (dec, _): (Protocol, _) = decode_from_slice(&enc).expect("decode Protocol");
        assert_eq!(protocol, dec);
    }

    // ===== Test 15: JSON-like Value enum (recursive via Box) =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum Value {
        Null,
        Bool(bool),
        Int(i64),
        Float(f64),
        Str(String),
        Array(Vec<Value>),
        Object(BTreeMap<String, Box<Value>>),
    }

    #[test]
    fn test_json_like_value_roundtrip() {
        let mut obj = BTreeMap::new();
        obj.insert(
            "name".to_string(),
            Box::new(Value::Str("oxicode".to_string())),
        );
        obj.insert("version".to_string(), Box::new(Value::Int(2)));
        obj.insert("pi".to_string(), Box::new(Value::Float(PI)));
        obj.insert("active".to_string(), Box::new(Value::Bool(true)));
        obj.insert(
            "tags".to_string(),
            Box::new(Value::Array(vec![
                Value::Str("rust".to_string()),
                Value::Str("serialization".to_string()),
                Value::Null,
            ])),
        );

        let value = Value::Object(obj);
        let enc = encode_to_vec(&value).expect("encode Value");
        let (dec, _): (Value, _) = decode_from_slice(&enc).expect("decode Value");
        assert_eq!(value, dec);
    }

    // ===== Test 16: Key-value store BTreeMap<String, Vec<u8>> with 200 entries =====

    #[test]
    fn test_kv_store_200_entries_roundtrip() {
        let store: BTreeMap<String, Vec<u8>> = (0..200)
            .map(|i| {
                let key = format!("key:{:04}", i);
                let val: Vec<u8> = (0u8..(i as u8 % 32 + 4)).map(|b| b ^ (i as u8)).collect();
                (key, val)
            })
            .collect();
        let enc = encode_to_vec(&store).expect("encode kv store 200");
        let (dec, _): (BTreeMap<String, Vec<u8>>, _) =
            decode_from_slice(&enc).expect("decode kv store 200");
        assert_eq!(store, dec);
    }

    // ===== Test 17: TimeSeries =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TimeSeries {
        metric: String,
        timestamps: Vec<u64>,
        values: Vec<f64>,
    }

    #[test]
    fn test_time_series_roundtrip() {
        let n = 500usize;
        let timestamps: Vec<u64> = (0..n).map(|i| 1_700_000_000 + i as u64 * 60).collect();
        let values: Vec<f64> = (0..n)
            .map(|i| (i as f64 * PI / 100.0).sin() * E.powi(2))
            .collect();
        let ts = TimeSeries {
            metric: "cpu.usage".to_string(),
            timestamps,
            values,
        };
        let enc = encode_to_vec(&ts).expect("encode TimeSeries");
        let (dec, _): (TimeSeries, _) = decode_from_slice(&enc).expect("decode TimeSeries");
        assert_eq!(ts, dec);
    }

    // ===== Test 18: Struct with all Rust primitive types =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct AllPrimitives {
        b: bool,
        u8_val: u8,
        u16_val: u16,
        u32_val: u32,
        u64_val: u64,
        u128_val: u128,
        usize_val: usize,
        i8_val: i8,
        i16_val: i16,
        i32_val: i32,
        i64_val: i64,
        i128_val: i128,
        isize_val: isize,
        f32_val: f32,
        f64_val: f64,
        char_val: char,
    }

    #[test]
    fn test_all_primitives_roundtrip() {
        let prim = AllPrimitives {
            b: true,
            u8_val: u8::MAX,
            u16_val: u16::MAX,
            u32_val: u32::MAX,
            u64_val: u64::MAX,
            u128_val: u128::MAX,
            usize_val: usize::MAX,
            i8_val: i8::MIN,
            i16_val: i16::MIN,
            i32_val: i32::MIN,
            i64_val: i64::MIN,
            i128_val: i128::MIN,
            isize_val: isize::MIN,
            f32_val: std::f32::consts::PI,
            f64_val: PI,
            char_val: '★',
        };
        let enc = encode_to_vec(&prim).expect("encode AllPrimitives");
        let (dec, _): (AllPrimitives, _) = decode_from_slice(&enc).expect("decode AllPrimitives");
        assert_eq!(prim, dec);
    }

    // ===== Test 19: NetworkPacket with IpAddr =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct NetworkPacket {
        source: IpAddr,
        destination: IpAddr,
        payload: Vec<u8>,
        timestamp: u64,
        ttl: u8,
    }

    #[test]
    fn test_network_packet_roundtrip() {
        let ipv4_src: IpAddr = "192.168.1.100".parse().expect("parse src IPv4");
        let ipv6_dst: IpAddr = "2001:db8::1".parse().expect("parse dst IPv6");
        let packet = NetworkPacket {
            source: ipv4_src,
            destination: ipv6_dst,
            payload: b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n".to_vec(),
            timestamp: 1_700_000_000_000,
            ttl: 64,
        };
        let enc = encode_to_vec(&packet).expect("encode NetworkPacket");
        let (dec, _): (NetworkPacket, _) = decode_from_slice(&enc).expect("decode NetworkPacket");
        assert_eq!(packet, dec);
    }

    // ===== Test 20: Certificate =====

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Certificate {
        issuer: String,
        subject: String,
        serial: [u8; 16],
        valid_from: u64,
        valid_until: u64,
        public_key_bytes: Vec<u8>,
    }

    #[test]
    fn test_certificate_roundtrip() {
        let cert = Certificate {
            issuer: "CN=OxiCode Root CA, O=COOLJAPAN OU, C=JP".to_string(),
            subject: "CN=api.example.com, O=Example Corp, C=US".to_string(),
            serial: [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
                0x32, 0x10,
            ],
            valid_from: 1_700_000_000,
            valid_until: 1_731_536_000,
            public_key_bytes: (0u8..64).collect(),
        };
        let enc = encode_to_vec(&cert).expect("encode Certificate");
        let (dec, _): (Certificate, _) = decode_from_slice(&enc).expect("decode Certificate");
        assert_eq!(cert, dec);
    }

    // ===== Test 21: Deeply nested Option =====

    #[test]
    fn test_deeply_nested_option_roundtrip() {
        type DeepOption = Option<Option<Option<Vec<String>>>>;

        let some_val: DeepOption = Some(Some(Some(vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
        ])));
        let enc = encode_to_vec(&some_val).expect("encode deep Some");
        let (dec, _): (DeepOption, _) = decode_from_slice(&enc).expect("decode deep Some");
        assert_eq!(some_val, dec);

        let none_outer: DeepOption = None;
        let enc2 = encode_to_vec(&none_outer).expect("encode outer None");
        let (dec2, _): (DeepOption, _) = decode_from_slice(&enc2).expect("decode outer None");
        assert_eq!(none_outer, dec2);

        let none_inner: DeepOption = Some(Some(None));
        let enc3 = encode_to_vec(&none_inner).expect("encode inner None");
        let (dec3, _): (DeepOption, _) = decode_from_slice(&enc3).expect("decode inner None");
        assert_eq!(none_inner, dec3);
    }

    // ===== Test 22: Mixed collection BTreeMap<String, Vec<HashMap<u32, String>>> =====

    #[test]
    fn test_mixed_collection_roundtrip() {
        let mut top: BTreeMap<String, Vec<HashMap<u32, String>>> = BTreeMap::new();

        let mut map_a: HashMap<u32, String> = HashMap::new();
        map_a.insert(1, "one".to_string());
        map_a.insert(2, "two".to_string());
        map_a.insert(3, "three".to_string());

        let mut map_b: HashMap<u32, String> = HashMap::new();
        map_b.insert(100, "hundred".to_string());
        map_b.insert(200, "two-hundred".to_string());

        top.insert("group_alpha".to_string(), vec![map_a]);
        top.insert("group_beta".to_string(), vec![map_b]);

        let mut map_c: HashMap<u32, String> = HashMap::new();
        for i in 0u32..10 {
            map_c.insert(i, format!("item_{}", i));
        }
        top.insert("group_gamma".to_string(), vec![map_c]);

        let enc = encode_to_vec(&top).expect("encode mixed collection");
        let (dec, _): (BTreeMap<String, Vec<HashMap<u32, String>>>, _) =
            decode_from_slice(&enc).expect("decode mixed collection");
        assert_eq!(top, dec);
    }
}
