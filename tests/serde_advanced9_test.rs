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
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared test types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    name: String,
    value: u32,
    enabled: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Measurement {
    sensor_id: u32,
    timestamp: u64,
    readings: Vec<f64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    status_code: u16,
    response_data: String,
    is_success: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Event {
    Start,
    Stop,
    Data(Vec<u8>),
    Error { code: u32, message: String },
}

// ---------------------------------------------------------------------------
// Test 1: Config serde roundtrip via oxicode encode/decode
// ---------------------------------------------------------------------------

#[test]
fn test_config_serde_roundtrip() {
    let cfg = Config {
        name: "test-config".to_string(),
        value: 42,
        enabled: true,
    };
    let enc = oxicode::serde::encode_to_vec(&cfg, oxicode::config::standard())
        .expect("encode Config failed");
    let (val, _): (Config, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Config failed");
    assert_eq!(cfg, val);
}

// ---------------------------------------------------------------------------
// Test 2: Measurement serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_measurement_serde_roundtrip() {
    let m = Measurement {
        sensor_id: 7,
        timestamp: 1_700_000_000,
        readings: vec![1.1, 2.2, 3.3],
    };
    let enc = oxicode::serde::encode_to_vec(&m, oxicode::config::standard())
        .expect("encode Measurement failed");
    let (val, _): (Measurement, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Measurement failed");
    assert_eq!(m, val);
}

// ---------------------------------------------------------------------------
// Test 3: ApiResponse with camelCase serde attrs roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_api_response_camelcase_roundtrip() {
    let resp = ApiResponse {
        status_code: 200,
        response_data: "OK".to_string(),
        is_success: true,
    };
    let enc = oxicode::serde::encode_to_vec(&resp, oxicode::config::standard())
        .expect("encode ApiResponse failed");
    let (val, _): (ApiResponse, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode ApiResponse failed");
    assert_eq!(resp, val);
}

// ---------------------------------------------------------------------------
// Test 4: Event::Start roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_start_roundtrip() {
    let event = Event::Start;
    let enc = oxicode::serde::encode_to_vec(&event, oxicode::config::standard())
        .expect("encode Event::Start failed");
    let (val, _): (Event, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Event::Start failed");
    assert_eq!(event, val);
}

// ---------------------------------------------------------------------------
// Test 5: Event::Stop roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_stop_roundtrip() {
    let event = Event::Stop;
    let enc = oxicode::serde::encode_to_vec(&event, oxicode::config::standard())
        .expect("encode Event::Stop failed");
    let (val, _): (Event, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Event::Stop failed");
    assert_eq!(event, val);
}

// ---------------------------------------------------------------------------
// Test 6: Event::Data(vec![1,2,3]) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_data_roundtrip() {
    let event = Event::Data(vec![1, 2, 3]);
    let enc = oxicode::serde::encode_to_vec(&event, oxicode::config::standard())
        .expect("encode Event::Data failed");
    let (val, _): (Event, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Event::Data failed");
    assert_eq!(event, val);
}

// ---------------------------------------------------------------------------
// Test 7: Event::Error roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_error_roundtrip() {
    let event = Event::Error {
        code: 404,
        message: "not found".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&event, oxicode::config::standard())
        .expect("encode Event::Error failed");
    let (val, _): (Event, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Event::Error failed");
    assert_eq!(event, val);
}

// ---------------------------------------------------------------------------
// Test 8: Vec<Config> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_config_roundtrip() {
    let configs = vec![
        Config {
            name: "a".to_string(),
            value: 1,
            enabled: true,
        },
        Config {
            name: "b".to_string(),
            value: 2,
            enabled: false,
        },
        Config {
            name: "c".to_string(),
            value: 3,
            enabled: true,
        },
    ];
    let enc = oxicode::serde::encode_to_vec(&configs, oxicode::config::standard())
        .expect("encode Vec<Config> failed");
    let (val, _): (Vec<Config>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<Config> failed");
    assert_eq!(configs, val);
}

// ---------------------------------------------------------------------------
// Test 9: Option<Config> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_config_some_roundtrip() {
    let opt: Option<Config> = Some(Config {
        name: "option-test".to_string(),
        value: 99,
        enabled: false,
    });
    let enc = oxicode::serde::encode_to_vec(&opt, oxicode::config::standard())
        .expect("encode Option<Config> Some failed");
    let (val, _): (Option<Config>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Option<Config> Some failed");
    assert_eq!(opt, val);
}

// ---------------------------------------------------------------------------
// Test 10: Option<Config> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_config_none_roundtrip() {
    let opt: Option<Config> = None;
    let enc = oxicode::serde::encode_to_vec(&opt, oxicode::config::standard())
        .expect("encode Option<Config> None failed");
    let (val, _): (Option<Config>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Option<Config> None failed");
    assert_eq!(opt, val);
}

// ---------------------------------------------------------------------------
// Test 11: Config using fixed-int config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_config_fixed_int_encoding_roundtrip() {
    let cfg = Config {
        name: "fixed-int".to_string(),
        value: 1234,
        enabled: true,
    };
    let fixed_cfg = oxicode::config::standard().with_fixed_int_encoding();
    let enc = oxicode::serde::encode_to_vec(&cfg, fixed_cfg)
        .expect("encode Config with fixed-int config failed");
    let (val, _): (Config, usize) = oxicode::serde::decode_owned_from_slice(&enc, fixed_cfg)
        .expect("decode Config with fixed-int config failed");
    assert_eq!(cfg, val);
}

// ---------------------------------------------------------------------------
// Test 12: Two Configs with same data encode identical bytes
// ---------------------------------------------------------------------------

#[test]
fn test_same_configs_encode_identical_bytes() {
    let cfg_a = Config {
        name: "same".to_string(),
        value: 77,
        enabled: true,
    };
    let cfg_b = Config {
        name: "same".to_string(),
        value: 77,
        enabled: true,
    };
    let enc_a = oxicode::serde::encode_to_vec(&cfg_a, oxicode::config::standard())
        .expect("encode cfg_a failed");
    let enc_b = oxicode::serde::encode_to_vec(&cfg_b, oxicode::config::standard())
        .expect("encode cfg_b failed");
    assert_eq!(
        enc_a, enc_b,
        "identical configs must encode to identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Two Configs with different data encode different bytes
// ---------------------------------------------------------------------------

#[test]
fn test_different_configs_encode_different_bytes() {
    let cfg_a = Config {
        name: "alpha".to_string(),
        value: 10,
        enabled: true,
    };
    let cfg_b = Config {
        name: "beta".to_string(),
        value: 20,
        enabled: false,
    };
    let enc_a = oxicode::serde::encode_to_vec(&cfg_a, oxicode::config::standard())
        .expect("encode cfg_a failed");
    let enc_b = oxicode::serde::encode_to_vec(&cfg_b, oxicode::config::standard())
        .expect("encode cfg_b failed");
    assert_ne!(
        enc_a, enc_b,
        "different configs must encode to different bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Serde struct bytes same as manually encoding equivalent oxicode struct
// ---------------------------------------------------------------------------

#[test]
fn test_serde_bytes_match_native_encode() {
    // A type that implements both serde and oxicode's native traits
    #[derive(Debug, PartialEq, Serialize, Deserialize, oxicode::Encode, oxicode::Decode)]
    struct SimplePoint {
        x: u32,
        y: u32,
    }

    let point = SimplePoint { x: 10, y: 20 };

    // Encode via serde path
    let serde_bytes = oxicode::serde::encode_to_vec(&point, oxicode::config::standard())
        .expect("serde encode SimplePoint failed");

    // Encode via native oxicode path
    let native_bytes = oxicode::encode_to_vec_with_config(&point, oxicode::config::standard())
        .expect("native encode SimplePoint failed");

    assert_eq!(
        serde_bytes, native_bytes,
        "serde and native encode paths must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 15: HashMap<String, Config> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_hashmap_string_config_roundtrip() {
    let mut map: HashMap<String, Config> = HashMap::new();
    map.insert(
        "first".to_string(),
        Config {
            name: "first-cfg".to_string(),
            value: 1,
            enabled: true,
        },
    );
    map.insert(
        "second".to_string(),
        Config {
            name: "second-cfg".to_string(),
            value: 2,
            enabled: false,
        },
    );
    let enc = oxicode::serde::encode_to_vec(&map, oxicode::config::standard())
        .expect("encode HashMap<String, Config> failed");
    let (val, _): (HashMap<String, Config>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode HashMap<String, Config> failed");
    assert_eq!(map, val);
}

// ---------------------------------------------------------------------------
// Test 16: Vec<Event> with all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_event_all_variants_roundtrip() {
    let events = vec![
        Event::Start,
        Event::Data(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        Event::Error {
            code: 500,
            message: "internal error".to_string(),
        },
        Event::Stop,
    ];
    let enc = oxicode::serde::encode_to_vec(&events, oxicode::config::standard())
        .expect("encode Vec<Event> failed");
    let (val, _): (Vec<Event>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<Event> failed");
    assert_eq!(events, val);
}

// ---------------------------------------------------------------------------
// Test 17: Struct with #[serde(rename = "n")] on field — roundtrip still works
// ---------------------------------------------------------------------------

#[test]
fn test_serde_rename_field_roundtrip() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Renamed {
        #[serde(rename = "n")]
        name: String,
        score: u64,
    }

    let item = Renamed {
        name: "player_one".to_string(),
        score: 9999,
    };
    let enc = oxicode::serde::encode_to_vec(&item, oxicode::config::standard())
        .expect("encode Renamed failed");
    let (val, _): (Renamed, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Renamed failed");
    assert_eq!(item, val);
}

// ---------------------------------------------------------------------------
// Test 18: Measurement with 100 readings roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_measurement_100_readings_roundtrip() {
    let readings: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
    let m = Measurement {
        sensor_id: 42,
        timestamp: 9_999_999_999,
        readings,
    };
    let enc = oxicode::serde::encode_to_vec(&m, oxicode::config::standard())
        .expect("encode Measurement 100 readings failed");
    let (val, _): (Measurement, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Measurement 100 readings failed");
    assert_eq!(m, val);
}

// ---------------------------------------------------------------------------
// Test 19: ApiResponse with unicode data roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_api_response_unicode_roundtrip() {
    let resp = ApiResponse {
        status_code: 201,
        response_data: "こんにちは世界 🌍 Привет мир".to_string(),
        is_success: true,
    };
    let enc = oxicode::serde::encode_to_vec(&resp, oxicode::config::standard())
        .expect("encode ApiResponse unicode failed");
    let (val, _): (ApiResponse, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode ApiResponse unicode failed");
    assert_eq!(resp, val);
}

// ---------------------------------------------------------------------------
// Test 20: Nested: Config inside another serde struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nested_config_in_outer_struct_roundtrip() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Container {
        id: u64,
        inner: Config,
        label: String,
    }

    let container = Container {
        id: 12345,
        inner: Config {
            name: "nested".to_string(),
            value: 255,
            enabled: false,
        },
        label: "outer-label".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&container, oxicode::config::standard())
        .expect("encode Container failed");
    let (val, _): (Container, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Container failed");
    assert_eq!(container, val);
}

// ---------------------------------------------------------------------------
// Test 21: Empty Vec<Event> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_empty_vec_event_roundtrip() {
    let events: Vec<Event> = vec![];
    let enc = oxicode::serde::encode_to_vec(&events, oxicode::config::standard())
        .expect("encode empty Vec<Event> failed");
    let (val, _): (Vec<Event>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode empty Vec<Event> failed");
    assert_eq!(events, val);
}

// ---------------------------------------------------------------------------
// Test 22: BTreeMap<u32, Event> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_btreemap_u32_event_roundtrip() {
    let mut map: BTreeMap<u32, Event> = BTreeMap::new();
    map.insert(0, Event::Start);
    map.insert(1, Event::Data(vec![10, 20, 30]));
    map.insert(
        2,
        Event::Error {
            code: 42,
            message: "something went wrong".to_string(),
        },
    );
    map.insert(3, Event::Stop);
    let enc = oxicode::serde::encode_to_vec(&map, oxicode::config::standard())
        .expect("encode BTreeMap<u32, Event> failed");
    let (val, _): (BTreeMap<u32, Event>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode BTreeMap<u32, Event> failed");
    assert_eq!(map, val);
}
