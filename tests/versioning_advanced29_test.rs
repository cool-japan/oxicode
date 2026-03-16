#![cfg(feature = "versioning")]
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
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BaseStation {
    station_id: u32,
    lat_x1e6: i32,
    lon_x1e6: i32,
    freq_band_mhz: u32,
    max_users: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QosClass {
    EmBB,
    URLLC,
    MMTC,
    Custom(u8),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkSlice {
    slice_id: u32,
    qos_class: QosClass,
    bandwidth_mbps: u32,
    latency_ms: u16,
    priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HandoverReason {
    SignalStrength,
    LoadBalancing,
    Mobility,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HandoverRecord {
    ue_id: u64,
    source_cell: u32,
    target_cell: u32,
    timestamp_ms: u64,
    reason: HandoverReason,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PacketHeader {
    src_addr: u32,
    dst_addr: u32,
    protocol: u8,
    length: u32,
    seq_num: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkKpi {
    cell_id: u32,
    timestamp: u64,
    throughput_mbps: f32,
    latency_ms: f32,
    packet_loss_pct: f32,
}

// --- Test 1: BaseStation basic encode/decode roundtrip ---
#[test]
fn test_base_station_roundtrip() {
    let station = BaseStation {
        station_id: 1001,
        lat_x1e6: 48_856_613,
        lon_x1e6: 2_352_222,
        freq_band_mhz: 3500,
        max_users: 512,
    };
    let encoded = encode_to_vec(&station).expect("encode BaseStation");
    let (decoded, consumed): (BaseStation, usize) =
        decode_from_slice(&encoded).expect("decode BaseStation");
    assert_eq!(station, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 2: BaseStation versioned with v1.0.0 ---
#[test]
fn test_base_station_versioned_v1() {
    let station = BaseStation {
        station_id: 2002,
        lat_x1e6: 35_689_487,
        lon_x1e6: 139_691_706,
        freq_band_mhz: 700,
        max_users: 1024,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&station, ver).expect("encode versioned BaseStation v1");
    let (decoded, version, consumed): (BaseStation, Version, usize) =
        decode_versioned_value::<BaseStation>(&encoded).expect("decode versioned BaseStation v1");
    assert_eq!(station, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 3: BaseStation versioned with v2.0.0 ---
#[test]
fn test_base_station_versioned_v2() {
    let station = BaseStation {
        station_id: 3003,
        lat_x1e6: 51_507_351,
        lon_x1e6: -122_333,
        freq_band_mhz: 2600,
        max_users: 256,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&station, ver).expect("encode versioned BaseStation v2");
    let (decoded, version, consumed): (BaseStation, Version, usize) =
        decode_versioned_value::<BaseStation>(&encoded).expect("decode versioned BaseStation v2");
    assert_eq!(station, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert!(consumed > 0);
}

// --- Test 4: NetworkSlice with EmBB QoS class ---
#[test]
fn test_network_slice_embb_roundtrip() {
    let slice = NetworkSlice {
        slice_id: 101,
        qos_class: QosClass::EmBB,
        bandwidth_mbps: 1000,
        latency_ms: 10,
        priority: 5,
    };
    let encoded = encode_to_vec(&slice).expect("encode NetworkSlice EmBB");
    let (decoded, consumed): (NetworkSlice, usize) =
        decode_from_slice(&encoded).expect("decode NetworkSlice EmBB");
    assert_eq!(slice, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 5: NetworkSlice with URLLC QoS class versioned v1.0.0 ---
#[test]
fn test_network_slice_urllc_versioned_v1() {
    let slice = NetworkSlice {
        slice_id: 202,
        qos_class: QosClass::URLLC,
        bandwidth_mbps: 100,
        latency_ms: 1,
        priority: 10,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&slice, ver).expect("encode versioned NetworkSlice URLLC");
    let (decoded, version, consumed): (NetworkSlice, Version, usize) =
        decode_versioned_value::<NetworkSlice>(&encoded)
            .expect("decode versioned NetworkSlice URLLC");
    assert_eq!(slice, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 6: NetworkSlice with MMTC QoS class versioned v3.1.0 ---
#[test]
fn test_network_slice_mmtc_versioned_v3_1() {
    let slice = NetworkSlice {
        slice_id: 303,
        qos_class: QosClass::MMTC,
        bandwidth_mbps: 10,
        latency_ms: 100,
        priority: 1,
    };
    let ver = Version::new(3, 1, 0);
    let encoded =
        encode_versioned_value(&slice, ver).expect("encode versioned NetworkSlice MMTC v3.1");
    let (decoded, version, consumed): (NetworkSlice, Version, usize) =
        decode_versioned_value::<NetworkSlice>(&encoded)
            .expect("decode versioned NetworkSlice MMTC v3.1");
    assert_eq!(slice, decoded);
    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 7: NetworkSlice with Custom QoS class ---
#[test]
fn test_network_slice_custom_qos_roundtrip() {
    let slice = NetworkSlice {
        slice_id: 404,
        qos_class: QosClass::Custom(42),
        bandwidth_mbps: 500,
        latency_ms: 20,
        priority: 7,
    };
    let encoded = encode_to_vec(&slice).expect("encode NetworkSlice Custom QoS");
    let (decoded, consumed): (NetworkSlice, usize) =
        decode_from_slice(&encoded).expect("decode NetworkSlice Custom QoS");
    assert_eq!(slice, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 8: HandoverRecord with SignalStrength reason ---
#[test]
fn test_handover_signal_strength_roundtrip() {
    let record = HandoverRecord {
        ue_id: 9_000_000_001,
        source_cell: 111,
        target_cell: 222,
        timestamp_ms: 1_700_000_000_000,
        reason: HandoverReason::SignalStrength,
    };
    let encoded = encode_to_vec(&record).expect("encode HandoverRecord SignalStrength");
    let (decoded, consumed): (HandoverRecord, usize) =
        decode_from_slice(&encoded).expect("decode HandoverRecord SignalStrength");
    assert_eq!(record, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 9: HandoverRecord versioned v1.0.0 with LoadBalancing ---
#[test]
fn test_handover_load_balancing_versioned_v1() {
    let record = HandoverRecord {
        ue_id: 9_000_000_002,
        source_cell: 333,
        target_cell: 444,
        timestamp_ms: 1_700_000_001_000,
        reason: HandoverReason::LoadBalancing,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&record, ver)
        .expect("encode versioned HandoverRecord LoadBalancing");
    let (decoded, version, consumed): (HandoverRecord, Version, usize) =
        decode_versioned_value::<HandoverRecord>(&encoded)
            .expect("decode versioned HandoverRecord LoadBalancing");
    assert_eq!(record, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 10: HandoverRecord versioned v2.0.0 with Mobility ---
#[test]
fn test_handover_mobility_versioned_v2() {
    let record = HandoverRecord {
        ue_id: 9_000_000_003,
        source_cell: 555,
        target_cell: 666,
        timestamp_ms: 1_700_000_002_000,
        reason: HandoverReason::Mobility,
    };
    let ver = Version::new(2, 0, 0);
    let encoded =
        encode_versioned_value(&record, ver).expect("encode versioned HandoverRecord Mobility v2");
    let (decoded, version, consumed): (HandoverRecord, Version, usize) =
        decode_versioned_value::<HandoverRecord>(&encoded)
            .expect("decode versioned HandoverRecord Mobility v2");
    assert_eq!(record, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 11: HandoverRecord with Emergency reason ---
#[test]
fn test_handover_emergency_roundtrip() {
    let record = HandoverRecord {
        ue_id: 9_000_000_004,
        source_cell: 777,
        target_cell: 888,
        timestamp_ms: 1_700_000_003_000,
        reason: HandoverReason::Emergency,
    };
    let encoded = encode_to_vec(&record).expect("encode HandoverRecord Emergency");
    let (decoded, consumed): (HandoverRecord, usize) =
        decode_from_slice(&encoded).expect("decode HandoverRecord Emergency");
    assert_eq!(record, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 12: PacketHeader basic roundtrip ---
#[test]
fn test_packet_header_roundtrip() {
    let header = PacketHeader {
        src_addr: 0xC0A80001,
        dst_addr: 0xC0A80002,
        protocol: 17,
        length: 1500,
        seq_num: 1_000_000,
    };
    let encoded = encode_to_vec(&header).expect("encode PacketHeader");
    let (decoded, consumed): (PacketHeader, usize) =
        decode_from_slice(&encoded).expect("decode PacketHeader");
    assert_eq!(header, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 13: PacketHeader versioned v1.0.0 ---
#[test]
fn test_packet_header_versioned_v1() {
    let header = PacketHeader {
        src_addr: 0x0A000001,
        dst_addr: 0x0A000002,
        protocol: 6,
        length: 64,
        seq_num: 42,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&header, ver).expect("encode versioned PacketHeader v1");
    let (decoded, version, consumed): (PacketHeader, Version, usize) =
        decode_versioned_value::<PacketHeader>(&encoded).expect("decode versioned PacketHeader v1");
    assert_eq!(header, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 14: PacketHeader versioned v3.1.0 ---
#[test]
fn test_packet_header_versioned_v3_1() {
    let header = PacketHeader {
        src_addr: 0xAC100001,
        dst_addr: 0xAC100002,
        protocol: 132,
        length: 9000,
        seq_num: 999_999_999,
    };
    let ver = Version::new(3, 1, 0);
    let encoded = encode_versioned_value(&header, ver).expect("encode versioned PacketHeader v3.1");
    let (decoded, version, consumed): (PacketHeader, Version, usize) =
        decode_versioned_value::<PacketHeader>(&encoded)
            .expect("decode versioned PacketHeader v3.1");
    assert_eq!(header, decoded);
    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 0);
    assert!(consumed > 0);
}

// --- Test 15: NetworkKpi basic roundtrip ---
#[test]
fn test_network_kpi_roundtrip() {
    let kpi = NetworkKpi {
        cell_id: 5001,
        timestamp: 1_700_100_000_000,
        throughput_mbps: 850.5,
        latency_ms: 2.3,
        packet_loss_pct: 0.01,
    };
    let encoded = encode_to_vec(&kpi).expect("encode NetworkKpi");
    let (decoded, consumed): (NetworkKpi, usize) =
        decode_from_slice(&encoded).expect("decode NetworkKpi");
    assert_eq!(kpi, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 16: NetworkKpi versioned v2.0.0 ---
#[test]
fn test_network_kpi_versioned_v2() {
    let kpi = NetworkKpi {
        cell_id: 5002,
        timestamp: 1_700_200_000_000,
        throughput_mbps: 1200.0,
        latency_ms: 1.5,
        packet_loss_pct: 0.001,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&kpi, ver).expect("encode versioned NetworkKpi v2");
    let (decoded, version, consumed): (NetworkKpi, Version, usize) =
        decode_versioned_value::<NetworkKpi>(&encoded).expect("decode versioned NetworkKpi v2");
    assert_eq!(kpi, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 17: NetworkKpi versioned v3.1.0 ---
#[test]
fn test_network_kpi_versioned_v3_1() {
    let kpi = NetworkKpi {
        cell_id: 5003,
        timestamp: 1_700_300_000_000,
        throughput_mbps: 400.75,
        latency_ms: 5.0,
        packet_loss_pct: 0.5,
    };
    let ver = Version::new(3, 1, 0);
    let encoded = encode_versioned_value(&kpi, ver).expect("encode versioned NetworkKpi v3.1");
    let (decoded, version, consumed): (NetworkKpi, Version, usize) =
        decode_versioned_value::<NetworkKpi>(&encoded).expect("decode versioned NetworkKpi v3.1");
    assert_eq!(kpi, decoded);
    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 18: Vec of BaseStations versioned v1.0.0 ---
#[test]
fn test_vec_base_stations_versioned_v1() {
    let stations: Vec<BaseStation> = vec![
        BaseStation {
            station_id: 1,
            lat_x1e6: 10_000_000,
            lon_x1e6: 20_000_000,
            freq_band_mhz: 800,
            max_users: 128,
        },
        BaseStation {
            station_id: 2,
            lat_x1e6: 11_000_000,
            lon_x1e6: 21_000_000,
            freq_band_mhz: 1800,
            max_users: 256,
        },
        BaseStation {
            station_id: 3,
            lat_x1e6: 12_000_000,
            lon_x1e6: 22_000_000,
            freq_band_mhz: 2100,
            max_users: 512,
        },
    ];
    let ver = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&stations, ver).expect("encode versioned Vec<BaseStation> v1");
    let (decoded, version, consumed): (Vec<BaseStation>, Version, usize) =
        decode_versioned_value::<Vec<BaseStation>>(&encoded)
            .expect("decode versioned Vec<BaseStation> v1");
    assert_eq!(stations, decoded);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 19: Vec of HandoverRecords versioned v2.0.0 ---
#[test]
fn test_vec_handover_records_versioned_v2() {
    let records: Vec<HandoverRecord> = vec![
        HandoverRecord {
            ue_id: 1_001,
            source_cell: 10,
            target_cell: 11,
            timestamp_ms: 1_000_000,
            reason: HandoverReason::SignalStrength,
        },
        HandoverRecord {
            ue_id: 1_002,
            source_cell: 20,
            target_cell: 21,
            timestamp_ms: 1_001_000,
            reason: HandoverReason::LoadBalancing,
        },
        HandoverRecord {
            ue_id: 1_003,
            source_cell: 30,
            target_cell: 31,
            timestamp_ms: 1_002_000,
            reason: HandoverReason::Emergency,
        },
    ];
    let ver = Version::new(2, 0, 0);
    let encoded =
        encode_versioned_value(&records, ver).expect("encode versioned Vec<HandoverRecord> v2");
    let (decoded, version, consumed): (Vec<HandoverRecord>, Version, usize) =
        decode_versioned_value::<Vec<HandoverRecord>>(&encoded)
            .expect("decode versioned Vec<HandoverRecord> v2");
    assert_eq!(records, decoded);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 20: Vec of NetworkKpis versioned v3.1.0 ---
#[test]
fn test_vec_network_kpis_versioned_v3_1() {
    let kpis: Vec<NetworkKpi> = vec![
        NetworkKpi {
            cell_id: 100,
            timestamp: 1_700_000_000,
            throughput_mbps: 300.0,
            latency_ms: 3.0,
            packet_loss_pct: 0.1,
        },
        NetworkKpi {
            cell_id: 101,
            timestamp: 1_700_001_000,
            throughput_mbps: 600.0,
            latency_ms: 2.0,
            packet_loss_pct: 0.05,
        },
        NetworkKpi {
            cell_id: 102,
            timestamp: 1_700_002_000,
            throughput_mbps: 900.0,
            latency_ms: 1.0,
            packet_loss_pct: 0.01,
        },
    ];
    let ver = Version::new(3, 1, 0);
    let encoded =
        encode_versioned_value(&kpis, ver).expect("encode versioned Vec<NetworkKpi> v3.1");
    let (decoded, version, consumed): (Vec<NetworkKpi>, Version, usize) =
        decode_versioned_value::<Vec<NetworkKpi>>(&encoded)
            .expect("decode versioned Vec<NetworkKpi> v3.1");
    assert_eq!(kpis, decoded);
    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, encoded.len());
}

// --- Test 21: Version fields comparison across multiple versions ---
#[test]
fn test_version_fields_comparison() {
    let station = BaseStation {
        station_id: 7777,
        lat_x1e6: -33_868_820,
        lon_x1e6: 151_209_290,
        freq_band_mhz: 28_000,
        max_users: 2048,
    };

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 1, 0);

    let encoded_v1 = encode_versioned_value(&station, v1).expect("encode v1");
    let encoded_v2 = encode_versioned_value(&station, v2).expect("encode v2");
    let encoded_v3 = encode_versioned_value(&station, v3).expect("encode v3");

    let (_, ver1, _): (BaseStation, Version, usize) =
        decode_versioned_value::<BaseStation>(&encoded_v1).expect("decode v1");
    let (_, ver2, _): (BaseStation, Version, usize) =
        decode_versioned_value::<BaseStation>(&encoded_v2).expect("decode v2");
    let (_, ver3, _): (BaseStation, Version, usize) =
        decode_versioned_value::<BaseStation>(&encoded_v3).expect("decode v3");

    assert_eq!(ver1.major, 1);
    assert_eq!(ver1.minor, 0);
    assert_eq!(ver1.patch, 0);

    assert_eq!(ver2.major, 2);
    assert_eq!(ver2.minor, 0);
    assert_eq!(ver2.patch, 0);

    assert_eq!(ver3.major, 3);
    assert_eq!(ver3.minor, 1);
    assert_eq!(ver3.patch, 0);

    assert!(ver2.major > ver1.major);
    assert!(ver3.major > ver2.major);
    assert!(ver3.minor > ver2.minor);
}

// --- Test 22: Consumed bytes increase for larger payloads ---
#[test]
fn test_consumed_bytes_scale_with_payload() {
    let small_slice = NetworkSlice {
        slice_id: 1,
        qos_class: QosClass::EmBB,
        bandwidth_mbps: 100,
        latency_ms: 5,
        priority: 1,
    };
    let large_slices: Vec<NetworkSlice> = (0u32..16)
        .map(|i| NetworkSlice {
            slice_id: i,
            qos_class: QosClass::Custom((i % 255) as u8),
            bandwidth_mbps: i * 100,
            latency_ms: (i as u16) + 1,
            priority: (i % 10) as u8,
        })
        .collect();

    let ver = Version::new(1, 0, 0);

    let encoded_small = encode_versioned_value(&small_slice, ver).expect("encode small slice");
    let encoded_large = encode_versioned_value(&large_slices, ver).expect("encode large slices");

    let (_, _, consumed_small): (NetworkSlice, Version, usize) =
        decode_versioned_value::<NetworkSlice>(&encoded_small).expect("decode small slice");
    let (decoded_large, version_large, consumed_large): (Vec<NetworkSlice>, Version, usize) =
        decode_versioned_value::<Vec<NetworkSlice>>(&encoded_large).expect("decode large slices");

    assert_eq!(consumed_small, encoded_small.len());
    assert_eq!(consumed_large, encoded_large.len());
    assert!(consumed_large > consumed_small);
    assert_eq!(decoded_large.len(), 16);
    assert_eq!(version_large.major, 1);
    assert_eq!(version_large.minor, 0);
    assert_eq!(version_large.patch, 0);
}
