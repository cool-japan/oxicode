//! Slice / session / bearer-focused tests for nested_structs_advanced12 (split from nested_structs_advanced12_test.rs).

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

// ---------------------------------------------------------------------------
// Domain enums
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SliceType {
    EmBB,
    URLLC,
    MIoT,
    V2X,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BearerType {
    DefaultBearer,
    DedicatedBearer,
    EmergencyBearer,
}

// ---------------------------------------------------------------------------
// Domain structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QosProfile {
    qci: u8,
    five_qi: u16,
    priority_level: u8,
    packet_delay_budget_ms: u32,
    packet_error_rate_exp: i8,
    max_data_burst_volume_bytes: u32,
    guaranteed_bitrate_kbps: Option<u64>,
    max_bitrate_kbps: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkSlice {
    snssai_sst: u8,
    snssai_sd: Option<u32>,
    slice_type: SliceType,
    name: String,
    qos_profiles: Vec<QosProfile>,
    max_subscribers: u32,
    isolation_level: u8,
    allowed_tai_list: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BearerContext {
    bearer_id: u32,
    bearer_type: BearerType,
    qos: QosProfile,
    tft_filters: Vec<String>,
    gtp_teid: u32,
    uplink_bytes: u64,
    downlink_bytes: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubscriberSession {
    imsi: String,
    msisdn: String,
    session_id: u64,
    apn: String,
    ip_address: String,
    bearer_contexts: Vec<BearerContext>,
    serving_cell_pci: u16,
    slice: Option<NetworkSlice>,
    registration_timestamp: u64,
    idle: bool,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_qos(qci: u8, fqi: u16, delay_ms: u32) -> QosProfile {
    QosProfile {
        qci,
        five_qi: fqi,
        priority_level: qci,
        packet_delay_budget_ms: delay_ms,
        packet_error_rate_exp: -6,
        max_data_burst_volume_bytes: 1500,
        guaranteed_bitrate_kbps: Some(10_000),
        max_bitrate_kbps: Some(100_000),
    }
}

fn make_slice(sst: u8, st: SliceType, name: &str) -> NetworkSlice {
    NetworkSlice {
        snssai_sst: sst,
        snssai_sd: Some(0x010203),
        slice_type: st,
        name: name.to_string(),
        qos_profiles: vec![make_qos(1, 1, 100), make_qos(5, 5, 300)],
        max_subscribers: 50_000,
        isolation_level: 2,
        allowed_tai_list: vec![100, 200, 300],
    }
}

fn make_bearer(id: u32, bt: BearerType) -> BearerContext {
    BearerContext {
        bearer_id: id,
        bearer_type: bt,
        qos: make_qos(1, 1, 100),
        tft_filters: vec!["permit ip any any".to_string()],
        gtp_teid: 0xABCD_0000 + id,
        uplink_bytes: 1_000_000,
        downlink_bytes: 5_000_000,
    }
}

// ---------------------------------------------------------------------------
// Test 2: Network slice with QoS profiles
// ---------------------------------------------------------------------------

#[test]
fn test_network_slice_qos() {
    let slice = NetworkSlice {
        snssai_sst: 1,
        snssai_sd: Some(0xABCDEF),
        slice_type: SliceType::EmBB,
        name: "Enhanced Mobile Broadband".to_string(),
        qos_profiles: vec![
            make_qos(1, 1, 100),
            make_qos(5, 5, 300),
            make_qos(9, 9, 500),
            QosProfile {
                qci: 65,
                five_qi: 65,
                priority_level: 1,
                packet_delay_budget_ms: 75,
                packet_error_rate_exp: -2,
                max_data_burst_volume_bytes: 4096,
                guaranteed_bitrate_kbps: Some(500_000),
                max_bitrate_kbps: None,
            },
        ],
        max_subscribers: 200_000,
        isolation_level: 3,
        allowed_tai_list: vec![10001, 10002, 10003, 10004],
    };
    let bytes = encode_to_vec(&slice).expect("encode network slice");
    let (decoded, _): (NetworkSlice, usize) =
        decode_from_slice(&bytes).expect("decode network slice");
    assert_eq!(slice, decoded);
    assert_eq!(decoded.qos_profiles.len(), 4);
    assert_eq!(decoded.qos_profiles[3].max_bitrate_kbps, None);
}

// ---------------------------------------------------------------------------
// Test 3: Subscriber session with bearer contexts
// ---------------------------------------------------------------------------

#[test]
fn test_subscriber_session_bearers() {
    let session = SubscriberSession {
        imsi: "440101234567890".to_string(),
        msisdn: "+81901234567".to_string(),
        session_id: 9999001,
        apn: "internet.5g.jp".to_string(),
        ip_address: "100.64.10.55".to_string(),
        bearer_contexts: vec![
            make_bearer(5, BearerType::DefaultBearer),
            make_bearer(6, BearerType::DedicatedBearer),
            make_bearer(7, BearerType::DedicatedBearer),
        ],
        serving_cell_pci: 100,
        slice: Some(make_slice(1, SliceType::EmBB, "eMBB-01")),
        registration_timestamp: 1_700_000_000,
        idle: false,
    };
    let bytes = encode_to_vec(&session).expect("encode subscriber session");
    let (decoded, _): (SubscriberSession, usize) =
        decode_from_slice(&bytes).expect("decode subscriber session");
    assert_eq!(session, decoded);
    assert_eq!(decoded.bearer_contexts.len(), 3);
    assert!(decoded.slice.is_some());
}

// ---------------------------------------------------------------------------
// Test 12: URLLC slice with strict QoS
// ---------------------------------------------------------------------------

#[test]
fn test_urllc_slice_strict_qos() {
    let slice = NetworkSlice {
        snssai_sst: 2,
        snssai_sd: Some(0x000001),
        slice_type: SliceType::URLLC,
        name: "Ultra-Reliable Factory Automation".to_string(),
        qos_profiles: vec![
            QosProfile {
                qci: 82,
                five_qi: 82,
                priority_level: 1,
                packet_delay_budget_ms: 1,
                packet_error_rate_exp: -5,
                max_data_burst_volume_bytes: 256,
                guaranteed_bitrate_kbps: Some(1_000),
                max_bitrate_kbps: Some(5_000),
            },
            QosProfile {
                qci: 83,
                five_qi: 83,
                priority_level: 2,
                packet_delay_budget_ms: 5,
                packet_error_rate_exp: -5,
                max_data_burst_volume_bytes: 512,
                guaranteed_bitrate_kbps: Some(2_000),
                max_bitrate_kbps: Some(10_000),
            },
        ],
        max_subscribers: 1_000,
        isolation_level: 5,
        allowed_tai_list: vec![50001],
    };
    let bytes = encode_to_vec(&slice).expect("encode URLLC slice");
    let (decoded, _): (NetworkSlice, usize) =
        decode_from_slice(&bytes).expect("decode URLLC slice");
    assert_eq!(slice, decoded);
    assert_eq!(decoded.qos_profiles[0].packet_delay_budget_ms, 1);
}

// ---------------------------------------------------------------------------
// Test 13: Subscriber session without slice (no slice assigned)
// ---------------------------------------------------------------------------

#[test]
fn test_subscriber_session_no_slice() {
    let session = SubscriberSession {
        imsi: "440105556667778".to_string(),
        msisdn: "+81905556677".to_string(),
        session_id: 8888001,
        apn: "legacy.lte.jp".to_string(),
        ip_address: "100.64.20.99".to_string(),
        bearer_contexts: vec![make_bearer(5, BearerType::DefaultBearer)],
        serving_cell_pci: 400,
        slice: None,
        registration_timestamp: 1_700_050_000,
        idle: true,
    };
    let bytes = encode_to_vec(&session).expect("encode session no slice");
    let (decoded, _): (SubscriberSession, usize) =
        decode_from_slice(&bytes).expect("decode session no slice");
    assert_eq!(session, decoded);
    assert!(decoded.slice.is_none());
    assert!(decoded.idle);
}

// ---------------------------------------------------------------------------
// Test 22: V2X custom slice with emergency bearer session
// ---------------------------------------------------------------------------

#[test]
fn test_v2x_slice_emergency_bearer() {
    let v2x_slice = NetworkSlice {
        snssai_sst: 4,
        snssai_sd: Some(0x040000),
        slice_type: SliceType::V2X,
        name: "Vehicle-to-Everything".to_string(),
        qos_profiles: vec![
            QosProfile {
                qci: 75,
                five_qi: 75,
                priority_level: 1,
                packet_delay_budget_ms: 3,
                packet_error_rate_exp: -5,
                max_data_burst_volume_bytes: 1200,
                guaranteed_bitrate_kbps: Some(50_000),
                max_bitrate_kbps: Some(200_000),
            },
            QosProfile {
                qci: 79,
                five_qi: 79,
                priority_level: 3,
                packet_delay_budget_ms: 50,
                packet_error_rate_exp: -3,
                max_data_burst_volume_bytes: 8000,
                guaranteed_bitrate_kbps: None,
                max_bitrate_kbps: Some(500_000),
            },
        ],
        max_subscribers: 10_000,
        isolation_level: 4,
        allowed_tai_list: vec![60001, 60002, 60003],
    };

    let session = SubscriberSession {
        imsi: "440107777888999".to_string(),
        msisdn: "+81907778899".to_string(),
        session_id: 7777001,
        apn: "v2x.connected.jp".to_string(),
        ip_address: "100.64.50.1".to_string(),
        bearer_contexts: vec![
            make_bearer(5, BearerType::DefaultBearer),
            BearerContext {
                bearer_id: 6,
                bearer_type: BearerType::EmergencyBearer,
                qos: QosProfile {
                    qci: 69,
                    five_qi: 69,
                    priority_level: 0,
                    packet_delay_budget_ms: 1,
                    packet_error_rate_exp: -6,
                    max_data_burst_volume_bytes: 256,
                    guaranteed_bitrate_kbps: Some(256),
                    max_bitrate_kbps: Some(512),
                },
                tft_filters: vec![
                    "permit ip 10.0.0.0/8 any".to_string(),
                    "permit ip any 224.0.0.0/4".to_string(),
                ],
                gtp_teid: 0xE000_0006,
                uplink_bytes: 500,
                downlink_bytes: 1200,
            },
        ],
        serving_cell_pci: 600,
        slice: Some(v2x_slice),
        registration_timestamp: 1_700_500_000,
        idle: false,
    };
    let bytes = encode_to_vec(&session).expect("encode V2X session");
    let (decoded, _): (SubscriberSession, usize) =
        decode_from_slice(&bytes).expect("decode V2X session");
    assert_eq!(session, decoded);
    assert_eq!(decoded.bearer_contexts.len(), 2);
    let emergency = &decoded.bearer_contexts[1];
    assert_eq!(emergency.bearer_type, BearerType::EmergencyBearer);
    assert_eq!(emergency.qos.priority_level, 0);
    let decoded_slice = decoded.slice.expect("V2X slice should be present");
    assert_eq!(decoded_slice.slice_type, SliceType::V2X);
}
