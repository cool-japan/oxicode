//! Mega-snapshot-focused tests for nested_structs_advanced12 (split from nested_structs_advanced12_test.rs).

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
enum FrequencyBand {
    N1,
    N3,
    N7,
    N28,
    N41,
    N77,
    N78,
    N79,
    N257,
    N258,
    N260,
    N261,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SliceType {
    EmBB,
    URLLC,
    MIoT,
    V2X,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NfType {
    Amf,
    Smf,
    Upf,
    Nrf,
    Nssf,
    Ausf,
    Udm,
    Pcf,
    Nef,
    Af,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MecAppState {
    Deployed,
    Running,
    Suspended,
    Failed,
    Migrating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlarmSeverity {
    Critical,
    Major,
    Minor,
    Warning,
    Info,
}

// ---------------------------------------------------------------------------
// Domain structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
    altitude_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BeamPattern {
    beam_id: u16,
    azimuth_deg: f32,
    elevation_deg: f32,
    beamwidth_h_deg: f32,
    beamwidth_v_deg: f32,
    gain_dbi: f32,
    ssb_index: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AntennaArray {
    array_id: u32,
    rows: u16,
    columns: u16,
    polarization_count: u8,
    element_spacing_mm: f32,
    beam_patterns: Vec<BeamPattern>,
    tilt_deg: f32,
    frequency_band: FrequencyBand,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellSector {
    sector_id: u32,
    pci: u16,
    earfcn: u32,
    bandwidth_mhz: u16,
    tx_power_dbm: f32,
    antenna: AntennaArray,
    neighbor_pcis: Vec<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellTower {
    tower_id: u64,
    name: String,
    location: GeoLocation,
    sectors: Vec<CellSector>,
    backhaul_capacity_gbps: f32,
    power_supply_backup_hours: u16,
    operator_id: String,
}

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
struct NfInstance {
    instance_id: String,
    nf_type: NfType,
    ip_addresses: Vec<String>,
    capacity: u32,
    load_percent: u8,
    heartbeat_interval_s: u16,
    supported_slices: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NfServiceChain {
    chain_id: u64,
    name: String,
    functions: Vec<NfInstance>,
    latency_budget_ms: u32,
    redundancy_level: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoreNetworkDeployment {
    deployment_id: String,
    service_chains: Vec<NfServiceChain>,
    nrf_endpoint: String,
    total_nf_count: u32,
    region: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectrumBlock {
    start_freq_mhz: u32,
    end_freq_mhz: u32,
    bandwidth_mhz: u16,
    band: FrequencyBand,
    license_holder: String,
    license_expiry_year: u16,
    duplex_mode: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectrumAllocationTable {
    country_code: String,
    blocks: Vec<SpectrumBlock>,
    total_allocated_mhz: u32,
    last_auction_date: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KpiMetric {
    metric_name: String,
    value: f64,
    unit: String,
    threshold_warning: Option<f64>,
    threshold_critical: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KpiCategory {
    category_name: String,
    metrics: Vec<KpiMetric>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkKpiDashboard {
    dashboard_id: u64,
    region: String,
    timestamp: u64,
    categories: Vec<KpiCategory>,
    overall_health_score: f64,
    active_alarms: Vec<(AlarmSeverity, String)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MecApplication {
    app_id: String,
    name: String,
    state: MecAppState,
    cpu_cores: u16,
    memory_mb: u32,
    storage_gb: u32,
    latency_requirement_ms: u32,
    endpoints: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MecHost {
    host_id: String,
    location: GeoLocation,
    total_cpu_cores: u16,
    total_memory_mb: u32,
    applications: Vec<MecApplication>,
    connected_gnb_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MecDeployment {
    deployment_name: String,
    hosts: Vec<MecHost>,
    orchestrator_endpoint: String,
    total_app_count: u32,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_geo(lat: f64, lon: f64, alt: f32) -> GeoLocation {
    GeoLocation {
        latitude: lat,
        longitude: lon,
        altitude_m: alt,
    }
}

fn make_beam(id: u16, az: f32, el: f32) -> BeamPattern {
    BeamPattern {
        beam_id: id,
        azimuth_deg: az,
        elevation_deg: el,
        beamwidth_h_deg: 10.0,
        beamwidth_v_deg: 8.0,
        gain_dbi: 18.5,
        ssb_index: id as u8,
    }
}

fn make_antenna(id: u32, band: FrequencyBand, beams: Vec<BeamPattern>) -> AntennaArray {
    AntennaArray {
        array_id: id,
        rows: 8,
        columns: 8,
        polarization_count: 2,
        element_spacing_mm: 17.5,
        beam_patterns: beams,
        tilt_deg: 6.0,
        frequency_band: band,
    }
}

fn make_sector(sid: u32, pci: u16, band: FrequencyBand) -> CellSector {
    CellSector {
        sector_id: sid,
        pci,
        earfcn: 630000 + sid,
        bandwidth_mhz: 100,
        tx_power_dbm: 43.0,
        antenna: make_antenna(
            sid,
            band,
            vec![
                make_beam(0, 0.0, -6.0),
                make_beam(1, 30.0, -6.0),
                make_beam(2, -30.0, -6.0),
            ],
        ),
        neighbor_pcis: vec![pci.wrapping_add(1), pci.wrapping_add(2)],
    }
}

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

fn make_nf(id: &str, nf_type: NfType) -> NfInstance {
    NfInstance {
        instance_id: id.to_string(),
        nf_type,
        ip_addresses: vec!["10.0.1.1".to_string(), "10.0.1.2".to_string()],
        capacity: 10_000,
        load_percent: 45,
        heartbeat_interval_s: 30,
        supported_slices: vec![1, 2, 3],
    }
}

fn make_kpi_metric(name: &str, value: f64, unit: &str) -> KpiMetric {
    KpiMetric {
        metric_name: name.to_string(),
        value,
        unit: unit.to_string(),
        threshold_warning: Some(value * 0.8),
        threshold_critical: Some(value * 0.5),
    }
}

// ---------------------------------------------------------------------------
// Test 21: Full 5G deployment snapshot (deeply nested aggregate)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Full5gDeploymentSnapshot {
    operator_name: String,
    snapshot_timestamp: u64,
    towers: Vec<CellTower>,
    slices: Vec<NetworkSlice>,
    core: CoreNetworkDeployment,
    mec: MecDeployment,
    kpi: NetworkKpiDashboard,
    spectrum: SpectrumAllocationTable,
}

#[test]
fn test_full_5g_deployment_snapshot() {
    let snapshot = Full5gDeploymentSnapshot {
        operator_name: "5G-Japan-Corp".to_string(),
        snapshot_timestamp: 1_700_400_000,
        towers: vec![CellTower {
            tower_id: 1,
            name: "Central-Tower".to_string(),
            location: make_geo(35.6762, 139.6503, 50.0),
            sectors: vec![
                make_sector(1, 10, FrequencyBand::N78),
                make_sector(2, 11, FrequencyBand::N257),
            ],
            backhaul_capacity_gbps: 100.0,
            power_supply_backup_hours: 12,
            operator_id: "5GJC".to_string(),
        }],
        slices: vec![
            make_slice(1, SliceType::EmBB, "Consumer-Broadband"),
            make_slice(2, SliceType::URLLC, "Industrial-Control"),
        ],
        core: CoreNetworkDeployment {
            deployment_id: "CORE-MAIN".to_string(),
            service_chains: vec![NfServiceChain {
                chain_id: 1,
                name: "Primary-UP".to_string(),
                functions: vec![
                    make_nf("amf-main", NfType::Amf),
                    make_nf("smf-main", NfType::Smf),
                    make_nf("upf-main", NfType::Upf),
                ],
                latency_budget_ms: 15,
                redundancy_level: 3,
            }],
            nrf_endpoint: "https://nrf.main.5gjc.jp:8443".to_string(),
            total_nf_count: 3,
            region: "National".to_string(),
        },
        mec: MecDeployment {
            deployment_name: "MEC-National".to_string(),
            hosts: vec![MecHost {
                host_id: "mec-central".to_string(),
                location: make_geo(35.6762, 139.6503, 5.0),
                total_cpu_cores: 128,
                total_memory_mb: 262144,
                applications: vec![MecApplication {
                    app_id: "cdn-edge".to_string(),
                    name: "CDN Edge Cache".to_string(),
                    state: MecAppState::Running,
                    cpu_cores: 32,
                    memory_mb: 65536,
                    storage_gb: 2000,
                    latency_requirement_ms: 5,
                    endpoints: vec!["https://cdn.edge.5gjc.jp".to_string()],
                }],
                connected_gnb_ids: vec![1, 2, 3],
            }],
            orchestrator_endpoint: "https://mec-orch.5gjc.jp:9443".to_string(),
            total_app_count: 1,
        },
        kpi: NetworkKpiDashboard {
            dashboard_id: 1,
            region: "National".to_string(),
            timestamp: 1_700_400_000,
            categories: vec![KpiCategory {
                category_name: "Overall".to_string(),
                metrics: vec![make_kpi_metric("connected_ues", 1_500_000.0, "count")],
            }],
            overall_health_score: 95.0,
            active_alarms: vec![],
        },
        spectrum: SpectrumAllocationTable {
            country_code: "JP".to_string(),
            blocks: vec![SpectrumBlock {
                start_freq_mhz: 3700,
                end_freq_mhz: 3800,
                bandwidth_mhz: 100,
                band: FrequencyBand::N78,
                license_holder: "5G-Japan-Corp".to_string(),
                license_expiry_year: 2038,
                duplex_mode: "TDD".to_string(),
            }],
            total_allocated_mhz: 100,
            last_auction_date: "2023-03-01".to_string(),
        },
    };
    let bytes = encode_to_vec(&snapshot).expect("encode full snapshot");
    let (decoded, _): (Full5gDeploymentSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode full snapshot");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.towers.len(), 1);
    assert_eq!(decoded.slices.len(), 2);
    assert_eq!(decoded.core.service_chains[0].functions.len(), 3);
}
