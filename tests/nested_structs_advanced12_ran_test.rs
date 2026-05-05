//! RAN / cell / spectrum-focused tests for nested_structs_advanced12 (split from nested_structs_advanced12_test.rs).

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
enum ModulationScheme {
    Qpsk,
    Qam16,
    Qam64,
    Qam256,
    Qam1024,
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
struct GnbNode {
    gnb_id: u64,
    name: String,
    location: GeoLocation,
    cells: Vec<CellSector>,
    connected_amf_ids: Vec<u64>,
    max_ue_capacity: u32,
    f1_interface_ip: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CuDuSplit {
    cu_id: u64,
    du_nodes: Vec<GnbNode>,
    midhaul_latency_us: u32,
    midhaul_bandwidth_gbps: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RanTopology {
    region_name: String,
    cu_du_splits: Vec<CuDuSplit>,
    total_gnb_count: u32,
    total_cell_count: u32,
    inter_gnb_x2_links: Vec<(u64, u64)>,
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
struct MimoLayer {
    layer_id: u8,
    precoding_matrix_index: u16,
    modulation: ModulationScheme,
    code_rate: f32,
    cw_index: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MimoConfig {
    config_id: u64,
    antenna_ports: u16,
    layers: Vec<MimoLayer>,
    max_rank: u8,
    codebook_type: String,
    srs_resources: u8,
    csi_rs_resources: u16,
    digital_beamforming: bool,
    analog_beamforming: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MassiveMimoDeployment {
    site_id: u64,
    configs: Vec<MimoConfig>,
    total_antenna_elements: u32,
    supported_bands: Vec<FrequencyBand>,
    calibration_timestamp: u64,
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

fn make_mimo_layer(lid: u8, mod_scheme: ModulationScheme) -> MimoLayer {
    MimoLayer {
        layer_id: lid,
        precoding_matrix_index: 42,
        modulation: mod_scheme,
        code_rate: 0.65,
        cw_index: lid % 2,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Cell tower with multi-sector antenna arrays
// ---------------------------------------------------------------------------

#[test]
fn test_cell_tower_multisector() {
    let tower = CellTower {
        tower_id: 1001,
        name: "Tokyo-Shibuya-01".to_string(),
        location: make_geo(35.6595, 139.7004, 45.0),
        sectors: vec![
            make_sector(1, 100, FrequencyBand::N78),
            make_sector(2, 101, FrequencyBand::N78),
            make_sector(3, 102, FrequencyBand::N257),
        ],
        backhaul_capacity_gbps: 25.0,
        power_supply_backup_hours: 8,
        operator_id: "JP-OPER-01".to_string(),
    };
    let bytes = encode_to_vec(&tower).expect("encode cell tower");
    let (decoded, _): (CellTower, usize) = decode_from_slice(&bytes).expect("decode cell tower");
    assert_eq!(tower, decoded);
    assert_eq!(decoded.sectors.len(), 3);
    assert_eq!(decoded.sectors[0].antenna.beam_patterns.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 4: RAN topology with CU-DU split
// ---------------------------------------------------------------------------

#[test]
fn test_ran_topology_cu_du() {
    let gnb1 = GnbNode {
        gnb_id: 5001,
        name: "gNB-Osaka-01".to_string(),
        location: make_geo(34.6937, 135.5023, 30.0),
        cells: vec![make_sector(10, 200, FrequencyBand::N77)],
        connected_amf_ids: vec![1, 2],
        max_ue_capacity: 5000,
        f1_interface_ip: "10.100.0.1".to_string(),
    };
    let gnb2 = GnbNode {
        gnb_id: 5002,
        name: "gNB-Osaka-02".to_string(),
        location: make_geo(34.6950, 135.5050, 28.0),
        cells: vec![
            make_sector(11, 201, FrequencyBand::N77),
            make_sector(12, 202, FrequencyBand::N258),
        ],
        connected_amf_ids: vec![1],
        max_ue_capacity: 8000,
        f1_interface_ip: "10.100.0.2".to_string(),
    };
    let topology = RanTopology {
        region_name: "Kansai".to_string(),
        cu_du_splits: vec![CuDuSplit {
            cu_id: 9001,
            du_nodes: vec![gnb1, gnb2],
            midhaul_latency_us: 250,
            midhaul_bandwidth_gbps: 25.0,
        }],
        total_gnb_count: 2,
        total_cell_count: 3,
        inter_gnb_x2_links: vec![(5001, 5002)],
    };
    let bytes = encode_to_vec(&topology).expect("encode RAN topology");
    let (decoded, _): (RanTopology, usize) =
        decode_from_slice(&bytes).expect("decode RAN topology");
    assert_eq!(topology, decoded);
    assert_eq!(decoded.cu_du_splits[0].du_nodes.len(), 2);
}

// ---------------------------------------------------------------------------
// Test 6: Spectrum allocation table
// ---------------------------------------------------------------------------

#[test]
fn test_spectrum_allocation() {
    let table = SpectrumAllocationTable {
        country_code: "JP".to_string(),
        blocks: vec![
            SpectrumBlock {
                start_freq_mhz: 3600,
                end_freq_mhz: 3700,
                bandwidth_mhz: 100,
                band: FrequencyBand::N78,
                license_holder: "OperatorA".to_string(),
                license_expiry_year: 2035,
                duplex_mode: "TDD".to_string(),
            },
            SpectrumBlock {
                start_freq_mhz: 27500,
                end_freq_mhz: 27900,
                bandwidth_mhz: 400,
                band: FrequencyBand::N257,
                license_holder: "OperatorB".to_string(),
                license_expiry_year: 2033,
                duplex_mode: "TDD".to_string(),
            },
            SpectrumBlock {
                start_freq_mhz: 700,
                end_freq_mhz: 730,
                bandwidth_mhz: 30,
                band: FrequencyBand::N28,
                license_holder: "OperatorC".to_string(),
                license_expiry_year: 2040,
                duplex_mode: "FDD".to_string(),
            },
        ],
        total_allocated_mhz: 530,
        last_auction_date: "2024-06-15".to_string(),
    };
    let bytes = encode_to_vec(&table).expect("encode spectrum table");
    let (decoded, _): (SpectrumAllocationTable, usize) =
        decode_from_slice(&bytes).expect("decode spectrum table");
    assert_eq!(table, decoded);
    assert_eq!(decoded.blocks.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 8: Massive MIMO deployment
// ---------------------------------------------------------------------------

#[test]
fn test_massive_mimo_deployment() {
    let deployment = MassiveMimoDeployment {
        site_id: 2001,
        configs: vec![
            MimoConfig {
                config_id: 1,
                antenna_ports: 64,
                layers: vec![
                    make_mimo_layer(0, ModulationScheme::Qam256),
                    make_mimo_layer(1, ModulationScheme::Qam256),
                    make_mimo_layer(2, ModulationScheme::Qam64),
                    make_mimo_layer(3, ModulationScheme::Qam64),
                ],
                max_rank: 4,
                codebook_type: "TypeII".to_string(),
                srs_resources: 4,
                csi_rs_resources: 32,
                digital_beamforming: true,
                analog_beamforming: true,
            },
            MimoConfig {
                config_id: 2,
                antenna_ports: 32,
                layers: vec![
                    make_mimo_layer(0, ModulationScheme::Qam1024),
                    make_mimo_layer(1, ModulationScheme::Qam1024),
                ],
                max_rank: 2,
                codebook_type: "TypeI".to_string(),
                srs_resources: 2,
                csi_rs_resources: 16,
                digital_beamforming: true,
                analog_beamforming: false,
            },
        ],
        total_antenna_elements: 256,
        supported_bands: vec![FrequencyBand::N78, FrequencyBand::N257],
        calibration_timestamp: 1_700_100_000,
    };
    let bytes = encode_to_vec(&deployment).expect("encode MIMO deployment");
    let (decoded, _): (MassiveMimoDeployment, usize) =
        decode_from_slice(&bytes).expect("decode MIMO deployment");
    assert_eq!(deployment, decoded);
    assert_eq!(decoded.configs[0].layers.len(), 4);
}

// ---------------------------------------------------------------------------
// Test 14: Complex RAN topology with multiple CU-DU splits
// ---------------------------------------------------------------------------

#[test]
fn test_ran_topology_multi_cu() {
    let make_gnb = |id: u64, name: &str, lat: f64, lon: f64| -> GnbNode {
        GnbNode {
            gnb_id: id,
            name: name.to_string(),
            location: make_geo(lat, lon, 25.0),
            cells: vec![
                make_sector(id as u32 * 10, (id * 10) as u16, FrequencyBand::N78),
                make_sector(
                    id as u32 * 10 + 1,
                    (id * 10 + 1) as u16,
                    FrequencyBand::N258,
                ),
            ],
            connected_amf_ids: vec![1, 2],
            max_ue_capacity: 6000,
            f1_interface_ip: format!("10.200.{}.1", id),
        }
    };

    let topology = RanTopology {
        region_name: "Chubu".to_string(),
        cu_du_splits: vec![
            CuDuSplit {
                cu_id: 8001,
                du_nodes: vec![
                    make_gnb(7001, "gNB-Nagoya-01", 35.1815, 136.9066),
                    make_gnb(7002, "gNB-Nagoya-02", 35.1700, 136.8900),
                ],
                midhaul_latency_us: 200,
                midhaul_bandwidth_gbps: 50.0,
            },
            CuDuSplit {
                cu_id: 8002,
                du_nodes: vec![make_gnb(7003, "gNB-Shizuoka-01", 34.9756, 138.3828)],
                midhaul_latency_us: 350,
                midhaul_bandwidth_gbps: 25.0,
            },
        ],
        total_gnb_count: 3,
        total_cell_count: 6,
        inter_gnb_x2_links: vec![(7001, 7002), (7001, 7003)],
    };
    let bytes = encode_to_vec(&topology).expect("encode multi-CU topology");
    let (decoded, _): (RanTopology, usize) =
        decode_from_slice(&bytes).expect("decode multi-CU topology");
    assert_eq!(topology, decoded);
    assert_eq!(decoded.cu_du_splits.len(), 2);
    assert_eq!(decoded.inter_gnb_x2_links.len(), 2);
}
