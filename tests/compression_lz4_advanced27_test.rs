#![cfg(feature = "compression-lz4")]
//! Advanced LZ4 compression tests themed around particle physics and high-energy experiments.
//!
//! Covers particle collision events (transverse momentum, pseudorapidity, phi),
//! calorimeter cell energies, tracker hit coordinates, muon chamber readings,
//! trigger decision levels, beam luminosity measurements, Monte Carlo truth records,
//! jet reconstruction parameters, missing transverse energy, vertex fitting results,
//! particle identification (PID) likelihoods, cross-section measurements,
//! detector alignment constants, data acquisition rates, and accelerator magnet settings.

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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FourMomentum {
    px: f64,
    py: f64,
    pz: f64,
    energy: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KinematicObservables {
    transverse_momentum_gev: f64,
    pseudorapidity: f64,
    phi_rad: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ParticleSpecies {
    Electron,
    Muon,
    Tau,
    Photon,
    Jet,
    MissingEt,
    Proton,
    Neutron,
    Pion { charge: i8 },
    Kaon { charge: i8 },
    BHadron { pdg_id: i32 },
    Unknown { raw_code: i32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReconstructedParticle {
    species: ParticleSpecies,
    four_momentum: FourMomentum,
    kinematics: KinematicObservables,
    charge: i8,
    isolation: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollisionEvent {
    run_number: u64,
    event_number: u64,
    luminosity_block: u32,
    bunch_crossing_id: u16,
    center_of_mass_energy_gev: f64,
    particles: Vec<ReconstructedParticle>,
    primary_vertex: VertexFit,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CalorimeterCell {
    layer: u8,
    eta_index: i16,
    phi_index: u16,
    energy_gev: f64,
    time_ns: f64,
    quality_flag: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CalorimeterCluster {
    cluster_id: u32,
    cells: Vec<CalorimeterCell>,
    total_energy_gev: f64,
    barycenter_eta: f64,
    barycenter_phi: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrackerHit {
    detector_layer: u8,
    module_id: u32,
    local_x_mm: f64,
    local_y_mm: f64,
    global_x_mm: f64,
    global_y_mm: f64,
    global_z_mm: f64,
    charge_adc: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Track {
    track_id: u32,
    hits: Vec<TrackerHit>,
    chi2_per_ndf: f64,
    d0_mm: f64,
    z0_mm: f64,
    pt_gev: f64,
    eta: f64,
    phi: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MuonChamberReading {
    station: u8,
    sector: u8,
    wire_number: u16,
    drift_time_ns: f64,
    adc_count: u16,
    hit_position_mm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MuonSegment {
    segment_id: u32,
    readings: Vec<MuonChamberReading>,
    local_direction_theta: f64,
    local_direction_phi: f64,
    chi2: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TriggerLevel {
    L1Hardware {
        prescale: u32,
    },
    L2Software {
        algorithm: String,
        threshold_gev: f64,
    },
    EventFilter {
        stream_tag: String,
        accepted: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TriggerDecision {
    trigger_name: String,
    level: TriggerLevel,
    passed: bool,
    bit_position: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BeamLuminosity {
    fill_number: u32,
    luminosity_block: u32,
    inst_luminosity_per_ub: f64,
    integrated_luminosity_per_fb: f64,
    pileup_mu: f64,
    beam_energy_gev: f64,
    beta_star_m: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MonteCarloTruth {
    pdg_id: i32,
    status_code: u8,
    barcode: u64,
    production_vertex_x_mm: f64,
    production_vertex_y_mm: f64,
    production_vertex_z_mm: f64,
    four_momentum: FourMomentum,
    parent_barcodes: Vec<u64>,
    child_barcodes: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JetParameters {
    jet_id: u32,
    algorithm: String,
    radius_parameter: f64,
    pt_gev: f64,
    eta: f64,
    phi: f64,
    mass_gev: f64,
    num_constituents: u32,
    btag_weight: f64,
    jvt_score: f64,
    emf: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MissingTransverseEnergy {
    met_x_gev: f64,
    met_y_gev: f64,
    met_magnitude_gev: f64,
    met_phi: f64,
    sum_et_gev: f64,
    significance: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VertexFit {
    vertex_id: u32,
    x_mm: f64,
    y_mm: f64,
    z_mm: f64,
    chi2: f64,
    ndf: u16,
    num_tracks: u32,
    cov_xx: f64,
    cov_yy: f64,
    cov_zz: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PidLikelihood {
    electron: f64,
    muon: f64,
    pion: f64,
    kaon: f64,
    proton: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrossSectionMeasurement {
    process_name: String,
    cross_section_pb: f64,
    stat_error_pb: f64,
    syst_error_up_pb: f64,
    syst_error_down_pb: f64,
    luminosity_per_fb: f64,
    sqrt_s_gev: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DetectorAlignment {
    module_id: u32,
    subsystem: String,
    dx_um: f64,
    dy_um: f64,
    dz_um: f64,
    rotation_alpha_mrad: f64,
    rotation_beta_mrad: f64,
    rotation_gamma_mrad: f64,
    valid_from_run: u64,
    valid_to_run: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DaqRate {
    partition: String,
    l1_rate_khz: f64,
    hlt_rate_khz: f64,
    recording_rate_khz: f64,
    data_bandwidth_gbps: f64,
    dead_time_fraction: f64,
    busy_fraction: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MagnetSetting {
    magnet_name: String,
    current_amps: f64,
    field_tesla: f64,
    polarity: i8,
    temperature_kelvin: f64,
    ramp_rate_amps_per_sec: f64,
    quench_protection_active: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// --- Test 1: Collision event with electron pair ---
#[test]
fn test_collision_event_electron_pair_roundtrip() {
    let event = CollisionEvent {
        run_number: 364_292,
        event_number: 1_578_230_912,
        luminosity_block: 437,
        bunch_crossing_id: 1832,
        center_of_mass_energy_gev: 13_600.0,
        particles: vec![
            ReconstructedParticle {
                species: ParticleSpecies::Electron,
                four_momentum: FourMomentum {
                    px: 32.1,
                    py: -18.7,
                    pz: 105.3,
                    energy: 113.5,
                },
                kinematics: KinematicObservables {
                    transverse_momentum_gev: 37.15,
                    pseudorapidity: 1.82,
                    phi_rad: -0.528,
                },
                charge: -1,
                isolation: 0.03,
            },
            ReconstructedParticle {
                species: ParticleSpecies::Electron,
                four_momentum: FourMomentum {
                    px: -28.4,
                    py: 22.9,
                    pz: -78.6,
                    energy: 88.2,
                },
                kinematics: KinematicObservables {
                    transverse_momentum_gev: 36.5,
                    pseudorapidity: -1.45,
                    phi_rad: 2.467,
                },
                charge: 1,
                isolation: 0.02,
            },
        ],
        primary_vertex: VertexFit {
            vertex_id: 0,
            x_mm: 0.015,
            y_mm: -0.008,
            z_mm: 42.3,
            chi2: 12.7,
            ndf: 18,
            num_tracks: 21,
            cov_xx: 0.001,
            cov_yy: 0.001,
            cov_zz: 0.04,
        },
    };

    let encoded = encode_to_vec(&event).expect("encode collision event");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress collision event");
    let decompressed = decompress(&compressed).expect("decompress collision event");
    let (decoded, _): (CollisionEvent, usize) =
        decode_from_slice(&decompressed).expect("decode collision event");

    assert_eq!(event, decoded);
}

// --- Test 2: Calorimeter cluster roundtrip ---
#[test]
fn test_calorimeter_cluster_roundtrip() {
    let cluster = CalorimeterCluster {
        cluster_id: 4521,
        cells: vec![
            CalorimeterCell {
                layer: 1,
                eta_index: 42,
                phi_index: 128,
                energy_gev: 3.21,
                time_ns: 0.5,
                quality_flag: 0,
            },
            CalorimeterCell {
                layer: 2,
                eta_index: 42,
                phi_index: 128,
                energy_gev: 12.87,
                time_ns: 0.3,
                quality_flag: 0,
            },
            CalorimeterCell {
                layer: 2,
                eta_index: 43,
                phi_index: 128,
                energy_gev: 8.54,
                time_ns: 0.4,
                quality_flag: 0,
            },
            CalorimeterCell {
                layer: 3,
                eta_index: 42,
                phi_index: 128,
                energy_gev: 1.03,
                time_ns: 0.8,
                quality_flag: 1,
            },
        ],
        total_energy_gev: 25.65,
        barycenter_eta: 0.654,
        barycenter_phi: 1.995,
    };

    let encoded = encode_to_vec(&cluster).expect("encode calorimeter cluster");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress calorimeter cluster");
    let decompressed = decompress(&compressed).expect("decompress calorimeter cluster");
    let (decoded, _): (CalorimeterCluster, usize) =
        decode_from_slice(&decompressed).expect("decode calorimeter cluster");

    assert_eq!(cluster, decoded);
}

// --- Test 3: Tracker hit and track reconstruction ---
#[test]
fn test_track_reconstruction_roundtrip() {
    let track = Track {
        track_id: 9001,
        hits: vec![
            TrackerHit {
                detector_layer: 0,
                module_id: 1200,
                local_x_mm: 3.45,
                local_y_mm: -0.12,
                global_x_mm: 33.5,
                global_y_mm: 12.1,
                global_z_mm: -250.0,
                charge_adc: 1850,
            },
            TrackerHit {
                detector_layer: 1,
                module_id: 1340,
                local_x_mm: 5.78,
                local_y_mm: 0.34,
                global_x_mm: 55.2,
                global_y_mm: 20.3,
                global_z_mm: -248.1,
                charge_adc: 2100,
            },
            TrackerHit {
                detector_layer: 2,
                module_id: 1510,
                local_x_mm: 8.91,
                local_y_mm: 0.67,
                global_x_mm: 88.7,
                global_y_mm: 32.4,
                global_z_mm: -245.5,
                charge_adc: 1920,
            },
            TrackerHit {
                detector_layer: 3,
                module_id: 1720,
                local_x_mm: 12.34,
                local_y_mm: 1.01,
                global_x_mm: 122.0,
                global_y_mm: 44.8,
                global_z_mm: -242.2,
                charge_adc: 2050,
            },
        ],
        chi2_per_ndf: 0.87,
        d0_mm: 0.012,
        z0_mm: 0.45,
        pt_gev: 45.3,
        eta: 0.32,
        phi: -1.21,
    };

    let encoded = encode_to_vec(&track).expect("encode track");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress track");
    let decompressed = decompress(&compressed).expect("decompress track");
    let (decoded, _): (Track, usize) = decode_from_slice(&decompressed).expect("decode track");

    assert_eq!(track, decoded);
}

// --- Test 4: Muon segment with chamber readings ---
#[test]
fn test_muon_segment_roundtrip() {
    let segment = MuonSegment {
        segment_id: 7788,
        readings: vec![
            MuonChamberReading {
                station: 1,
                sector: 5,
                wire_number: 120,
                drift_time_ns: 234.5,
                adc_count: 890,
                hit_position_mm: 15.67,
            },
            MuonChamberReading {
                station: 1,
                sector: 5,
                wire_number: 121,
                drift_time_ns: 189.2,
                adc_count: 910,
                hit_position_mm: 16.12,
            },
            MuonChamberReading {
                station: 2,
                sector: 5,
                wire_number: 85,
                drift_time_ns: 312.8,
                adc_count: 780,
                hit_position_mm: 22.34,
            },
            MuonChamberReading {
                station: 3,
                sector: 5,
                wire_number: 63,
                drift_time_ns: 401.1,
                adc_count: 650,
                hit_position_mm: 30.91,
            },
        ],
        local_direction_theta: 0.125,
        local_direction_phi: -2.31,
        chi2: 3.45,
    };

    let encoded = encode_to_vec(&segment).expect("encode muon segment");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress muon segment");
    let decompressed = decompress(&compressed).expect("decompress muon segment");
    let (decoded, _): (MuonSegment, usize) =
        decode_from_slice(&decompressed).expect("decode muon segment");

    assert_eq!(segment, decoded);
}

// --- Test 5: Trigger decision chain ---
#[test]
fn test_trigger_decision_chain_roundtrip() {
    let decisions = vec![
        TriggerDecision {
            trigger_name: "L1_EM22VHI".to_string(),
            level: TriggerLevel::L1Hardware { prescale: 1 },
            passed: true,
            bit_position: 42,
        },
        TriggerDecision {
            trigger_name: "HLT_e26_lhtight_nod0".to_string(),
            level: TriggerLevel::L2Software {
                algorithm: "TrigEgammaFastElectron".to_string(),
                threshold_gev: 26.0,
            },
            passed: true,
            bit_position: 310,
        },
        TriggerDecision {
            trigger_name: "HLT_e26_lhtight_nod0_ivarloose".to_string(),
            level: TriggerLevel::EventFilter {
                stream_tag: "Main".to_string(),
                accepted: true,
            },
            passed: true,
            bit_position: 512,
        },
    ];

    let encoded = encode_to_vec(&decisions).expect("encode trigger decisions");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress trigger decisions");
    let decompressed = decompress(&compressed).expect("decompress trigger decisions");
    let (decoded, _): (Vec<TriggerDecision>, usize) =
        decode_from_slice(&decompressed).expect("decode trigger decisions");

    assert_eq!(decisions, decoded);
}

// --- Test 6: Beam luminosity measurement ---
#[test]
fn test_beam_luminosity_roundtrip() {
    let lumi = BeamLuminosity {
        fill_number: 8924,
        luminosity_block: 512,
        inst_luminosity_per_ub: 2.05e34,
        integrated_luminosity_per_fb: 140.1,
        pileup_mu: 52.3,
        beam_energy_gev: 6800.0,
        beta_star_m: 0.30,
    };

    let encoded = encode_to_vec(&lumi).expect("encode beam luminosity");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress beam luminosity");
    let decompressed = decompress(&compressed).expect("decompress beam luminosity");
    let (decoded, _): (BeamLuminosity, usize) =
        decode_from_slice(&decompressed).expect("decode beam luminosity");

    assert_eq!(lumi, decoded);
}

// --- Test 7: Monte Carlo truth record with decay chain ---
#[test]
fn test_monte_carlo_truth_decay_chain_roundtrip() {
    let truth_particles = vec![
        MonteCarloTruth {
            pdg_id: 25,
            status_code: 62,
            barcode: 1000,
            production_vertex_x_mm: 0.0,
            production_vertex_y_mm: 0.0,
            production_vertex_z_mm: 0.0,
            four_momentum: FourMomentum {
                px: 30.2,
                py: -15.8,
                pz: 180.5,
                energy: 185.0,
            },
            parent_barcodes: vec![],
            child_barcodes: vec![1001, 1002],
        },
        MonteCarloTruth {
            pdg_id: 5,
            status_code: 23,
            barcode: 1001,
            production_vertex_x_mm: 0.0,
            production_vertex_y_mm: 0.0,
            production_vertex_z_mm: 0.0,
            four_momentum: FourMomentum {
                px: 22.1,
                py: -8.4,
                pz: 95.7,
                energy: 99.3,
            },
            parent_barcodes: vec![1000],
            child_barcodes: vec![1003, 1004, 1005],
        },
        MonteCarloTruth {
            pdg_id: -5,
            status_code: 23,
            barcode: 1002,
            production_vertex_x_mm: 0.0,
            production_vertex_y_mm: 0.0,
            production_vertex_z_mm: 0.0,
            four_momentum: FourMomentum {
                px: 8.1,
                py: -7.4,
                pz: 84.8,
                energy: 85.7,
            },
            parent_barcodes: vec![1000],
            child_barcodes: vec![1006, 1007],
        },
    ];

    let encoded = encode_to_vec(&truth_particles).expect("encode MC truth");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress MC truth");
    let decompressed = decompress(&compressed).expect("decompress MC truth");
    let (decoded, _): (Vec<MonteCarloTruth>, usize) =
        decode_from_slice(&decompressed).expect("decode MC truth");

    assert_eq!(truth_particles, decoded);
    assert_eq!(decoded[0].pdg_id, 25); // Higgs boson
    assert_eq!(decoded[0].child_barcodes.len(), 2);
}

// --- Test 8: Jet reconstruction parameters ---
#[test]
fn test_jet_reconstruction_roundtrip() {
    let jets = vec![
        JetParameters {
            jet_id: 0,
            algorithm: "AntiKt4EMTopo".to_string(),
            radius_parameter: 0.4,
            pt_gev: 125.8,
            eta: -0.42,
            phi: 2.87,
            mass_gev: 12.3,
            num_constituents: 38,
            btag_weight: 0.92,
            jvt_score: 0.97,
            emf: 0.35,
        },
        JetParameters {
            jet_id: 1,
            algorithm: "AntiKt4EMTopo".to_string(),
            radius_parameter: 0.4,
            pt_gev: 67.4,
            eta: 1.15,
            phi: -0.78,
            mass_gev: 8.1,
            num_constituents: 22,
            btag_weight: 0.15,
            jvt_score: 0.95,
            emf: 0.61,
        },
    ];

    let encoded = encode_to_vec(&jets).expect("encode jets");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress jets");
    let decompressed = decompress(&compressed).expect("decompress jets");
    let (decoded, _): (Vec<JetParameters>, usize) =
        decode_from_slice(&decompressed).expect("decode jets");

    assert_eq!(jets, decoded);
}

// --- Test 9: Missing transverse energy ---
#[test]
fn test_missing_transverse_energy_roundtrip() {
    let met = MissingTransverseEnergy {
        met_x_gev: -45.2,
        met_y_gev: 32.8,
        met_magnitude_gev: 55.85,
        met_phi: 2.515,
        sum_et_gev: 892.3,
        significance: 8.7,
    };

    let encoded = encode_to_vec(&met).expect("encode MET");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress MET");
    let decompressed = decompress(&compressed).expect("decompress MET");
    let (decoded, _): (MissingTransverseEnergy, usize) =
        decode_from_slice(&decompressed).expect("decode MET");

    assert_eq!(met, decoded);
}

// --- Test 10: Vertex fitting results ---
#[test]
fn test_vertex_fit_collection_roundtrip() {
    let vertices = vec![
        VertexFit {
            vertex_id: 0,
            x_mm: 0.012,
            y_mm: -0.005,
            z_mm: -23.4,
            chi2: 8.9,
            ndf: 14,
            num_tracks: 18,
            cov_xx: 0.0008,
            cov_yy: 0.0009,
            cov_zz: 0.032,
        },
        VertexFit {
            vertex_id: 1,
            x_mm: 0.310,
            y_mm: 0.125,
            z_mm: -21.8,
            chi2: 3.2,
            ndf: 4,
            num_tracks: 5,
            cov_xx: 0.005,
            cov_yy: 0.006,
            cov_zz: 0.15,
        },
        VertexFit {
            vertex_id: 2,
            x_mm: -0.050,
            y_mm: 0.200,
            z_mm: 15.6,
            chi2: 1.8,
            ndf: 2,
            num_tracks: 3,
            cov_xx: 0.012,
            cov_yy: 0.011,
            cov_zz: 0.30,
        },
    ];

    let encoded = encode_to_vec(&vertices).expect("encode vertices");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress vertices");
    let decompressed = decompress(&compressed).expect("decompress vertices");
    let (decoded, _): (Vec<VertexFit>, usize) =
        decode_from_slice(&decompressed).expect("decode vertices");

    assert_eq!(vertices, decoded);
}

// --- Test 11: PID likelihoods ---
#[test]
fn test_pid_likelihood_roundtrip() {
    let pid = PidLikelihood {
        electron: 0.95,
        muon: 0.02,
        pion: 0.01,
        kaon: 0.005,
        proton: 0.015,
    };

    let encoded = encode_to_vec(&pid).expect("encode PID likelihood");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress PID likelihood");
    let decompressed = decompress(&compressed).expect("decompress PID likelihood");
    let (decoded, _): (PidLikelihood, usize) =
        decode_from_slice(&decompressed).expect("decode PID likelihood");

    assert_eq!(pid, decoded);
}

// --- Test 12: Cross-section measurement ---
#[test]
fn test_cross_section_measurement_roundtrip() {
    let xsec = CrossSectionMeasurement {
        process_name: "ttbar_inclusive_13p6TeV".to_string(),
        cross_section_pb: 924.5,
        stat_error_pb: 1.8,
        syst_error_up_pb: 32.1,
        syst_error_down_pb: 28.7,
        luminosity_per_fb: 140.1,
        sqrt_s_gev: 13_600.0,
    };

    let encoded = encode_to_vec(&xsec).expect("encode cross-section");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress cross-section");
    let decompressed = decompress(&compressed).expect("decompress cross-section");
    let (decoded, _): (CrossSectionMeasurement, usize) =
        decode_from_slice(&decompressed).expect("decode cross-section");

    assert_eq!(xsec, decoded);
}

// --- Test 13: Detector alignment constants ---
#[test]
fn test_detector_alignment_roundtrip() {
    let alignment = DetectorAlignment {
        module_id: 43210,
        subsystem: "InnerPixelBarrel".to_string(),
        dx_um: 2.34,
        dy_um: -1.05,
        dz_um: 0.87,
        rotation_alpha_mrad: 0.0012,
        rotation_beta_mrad: -0.0008,
        rotation_gamma_mrad: 0.0003,
        valid_from_run: 364_000,
        valid_to_run: 365_000,
    };

    let encoded = encode_to_vec(&alignment).expect("encode alignment");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress alignment");
    let decompressed = decompress(&compressed).expect("decompress alignment");
    let (decoded, _): (DetectorAlignment, usize) =
        decode_from_slice(&decompressed).expect("decode alignment");

    assert_eq!(alignment, decoded);
}

// --- Test 14: DAQ rate snapshot ---
#[test]
fn test_daq_rate_roundtrip() {
    let daq = DaqRate {
        partition: "ATLAS".to_string(),
        l1_rate_khz: 100.0,
        hlt_rate_khz: 10.0,
        recording_rate_khz: 1.2,
        data_bandwidth_gbps: 6.4,
        dead_time_fraction: 0.015,
        busy_fraction: 0.03,
    };

    let encoded = encode_to_vec(&daq).expect("encode DAQ rate");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress DAQ rate");
    let decompressed = decompress(&compressed).expect("decompress DAQ rate");
    let (decoded, _): (DaqRate, usize) = decode_from_slice(&decompressed).expect("decode DAQ rate");

    assert_eq!(daq, decoded);
}

// --- Test 15: Accelerator magnet settings ---
#[test]
fn test_magnet_settings_roundtrip() {
    let magnets = vec![
        MagnetSetting {
            magnet_name: "CMS_Solenoid".to_string(),
            current_amps: 18_164.0,
            field_tesla: 3.8,
            polarity: 1,
            temperature_kelvin: 4.5,
            ramp_rate_amps_per_sec: 10.0,
            quench_protection_active: true,
        },
        MagnetSetting {
            magnet_name: "LHC_Dipole_Sector12".to_string(),
            current_amps: 11_850.0,
            field_tesla: 8.33,
            polarity: 1,
            temperature_kelvin: 1.9,
            ramp_rate_amps_per_sec: 5.0,
            quench_protection_active: true,
        },
    ];

    let encoded = encode_to_vec(&magnets).expect("encode magnet settings");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress magnet settings");
    let decompressed = decompress(&compressed).expect("decompress magnet settings");
    let (decoded, _): (Vec<MagnetSetting>, usize) =
        decode_from_slice(&decompressed).expect("decode magnet settings");

    assert_eq!(magnets, decoded);
}

// --- Test 16: Particle species enum variant roundtrips ---
#[test]
fn test_particle_species_variants_roundtrip() {
    let species_list = vec![
        ParticleSpecies::Electron,
        ParticleSpecies::Muon,
        ParticleSpecies::Tau,
        ParticleSpecies::Photon,
        ParticleSpecies::Jet,
        ParticleSpecies::MissingEt,
        ParticleSpecies::Pion { charge: 1 },
        ParticleSpecies::Pion { charge: -1 },
        ParticleSpecies::Kaon { charge: 1 },
        ParticleSpecies::BHadron { pdg_id: 521 },
        ParticleSpecies::Unknown { raw_code: 99999 },
    ];

    let encoded = encode_to_vec(&species_list).expect("encode particle species list");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress particle species list");
    let decompressed = decompress(&compressed).expect("decompress particle species list");
    let (decoded, _): (Vec<ParticleSpecies>, usize) =
        decode_from_slice(&decompressed).expect("decode particle species list");

    assert_eq!(species_list, decoded);
}

// --- Test 17: Large calorimeter readout compression ratio ---
#[test]
fn test_large_calorimeter_readout_compression_ratio() {
    let mut cells: Vec<CalorimeterCell> = Vec::with_capacity(2000);
    for i in 0..2000_u16 {
        cells.push(CalorimeterCell {
            layer: (i % 7) as u8,
            eta_index: (i as i16) - 1000,
            phi_index: i,
            energy_gev: 0.5 + (i as f64) * 0.01,
            time_ns: 0.2,
            quality_flag: 0,
        });
    }

    let encoded = encode_to_vec(&cells).expect("encode large calorimeter readout");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress large calorimeter readout");
    let decompressed = decompress(&compressed).expect("decompress large calorimeter readout");
    let (decoded, _): (Vec<CalorimeterCell>, usize) =
        decode_from_slice(&decompressed).expect("decode large calorimeter readout");

    assert_eq!(cells.len(), decoded.len());
    assert_eq!(cells[0], decoded[0]);
    assert_eq!(cells[1999], decoded[1999]);

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "Compression ratio {} should be < 1.0 for 2000 calorimeter cells",
        ratio
    );
}

// --- Test 18: Large tracker hit collection compression ratio ---
#[test]
fn test_large_tracker_hits_compression_ratio() {
    let mut hits: Vec<TrackerHit> = Vec::with_capacity(1500);
    for i in 0..1500_u32 {
        hits.push(TrackerHit {
            detector_layer: (i % 6) as u8,
            module_id: 1000 + i,
            local_x_mm: 5.0 + (i as f64) * 0.1,
            local_y_mm: -0.5 + (i as f64) * 0.01,
            global_x_mm: 50.0 + (i as f64) * 0.3,
            global_y_mm: 20.0 + (i as f64) * 0.15,
            global_z_mm: -300.0 + (i as f64) * 0.4,
            charge_adc: 1800 + (i % 500) as u16,
        });
    }

    let encoded = encode_to_vec(&hits).expect("encode large tracker hits");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large tracker hits");
    let decompressed = decompress(&compressed).expect("decompress large tracker hits");
    let (decoded, _): (Vec<TrackerHit>, usize) =
        decode_from_slice(&decompressed).expect("decode large tracker hits");

    assert_eq!(hits.len(), decoded.len());
    assert_eq!(hits[0], decoded[0]);
    assert_eq!(hits[1499], decoded[1499]);

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "Compression ratio {} should be < 1.0 for 1500 tracker hits",
        ratio
    );
}

// --- Test 19: Full event with jets, MET, and triggers ---
#[test]
fn test_full_event_record_roundtrip() {
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct FullEventRecord {
        event: CollisionEvent,
        jets: Vec<JetParameters>,
        met: MissingTransverseEnergy,
        triggers: Vec<TriggerDecision>,
        luminosity: BeamLuminosity,
    }

    let record = FullEventRecord {
        event: CollisionEvent {
            run_number: 364_300,
            event_number: 2_000_000_001,
            luminosity_block: 100,
            bunch_crossing_id: 2400,
            center_of_mass_energy_gev: 13_600.0,
            particles: vec![ReconstructedParticle {
                species: ParticleSpecies::Muon,
                four_momentum: FourMomentum {
                    px: 18.5,
                    py: 25.3,
                    pz: 60.1,
                    energy: 68.2,
                },
                kinematics: KinematicObservables {
                    transverse_momentum_gev: 31.3,
                    pseudorapidity: 1.12,
                    phi_rad: 0.94,
                },
                charge: -1,
                isolation: 0.04,
            }],
            primary_vertex: VertexFit {
                vertex_id: 0,
                x_mm: 0.01,
                y_mm: -0.003,
                z_mm: 10.2,
                chi2: 6.5,
                ndf: 10,
                num_tracks: 12,
                cov_xx: 0.0009,
                cov_yy: 0.001,
                cov_zz: 0.035,
            },
        },
        jets: vec![JetParameters {
            jet_id: 0,
            algorithm: "AntiKt4EMPFlow".to_string(),
            radius_parameter: 0.4,
            pt_gev: 210.5,
            eta: -0.15,
            phi: 1.23,
            mass_gev: 18.7,
            num_constituents: 55,
            btag_weight: 0.98,
            jvt_score: 0.99,
            emf: 0.28,
        }],
        met: MissingTransverseEnergy {
            met_x_gev: -78.3,
            met_y_gev: 12.5,
            met_magnitude_gev: 79.29,
            met_phi: 2.98,
            sum_et_gev: 1350.0,
            significance: 12.1,
        },
        triggers: vec![TriggerDecision {
            trigger_name: "L1_MU20".to_string(),
            level: TriggerLevel::L1Hardware { prescale: 1 },
            passed: true,
            bit_position: 15,
        }],
        luminosity: BeamLuminosity {
            fill_number: 9001,
            luminosity_block: 100,
            inst_luminosity_per_ub: 1.8e34,
            integrated_luminosity_per_fb: 142.0,
            pileup_mu: 48.0,
            beam_energy_gev: 6800.0,
            beta_star_m: 0.30,
        },
    };

    let encoded = encode_to_vec(&record).expect("encode full event record");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress full event record");
    let decompressed = decompress(&compressed).expect("decompress full event record");
    let (decoded, _): (FullEventRecord, usize) =
        decode_from_slice(&decompressed).expect("decode full event record");

    assert_eq!(record, decoded);
}

// --- Test 20: Multiple cross-section measurements compression ratio ---
#[test]
fn test_cross_section_collection_compression_ratio() {
    let processes = [
        ("ttbar_dileptonic", 87.3, 0.4, 3.1, 2.9),
        ("ttbar_semileptonic", 365.2, 1.2, 12.5, 11.8),
        ("ttbar_allhad", 471.9, 2.1, 18.3, 17.0),
        ("Wplus_enu", 11_340.0, 5.0, 210.0, 195.0),
        ("Wminus_enu", 8_520.0, 4.0, 160.0, 148.0),
        ("Z_ee", 1_981.0, 1.5, 42.0, 39.0),
        ("WW_inclusive", 118.7, 0.9, 5.2, 4.8),
        ("WZ_inclusive", 51.1, 0.5, 2.3, 2.1),
        ("ZZ_inclusive", 16.5, 0.2, 0.8, 0.7),
        ("single_top_tchan", 217.0, 1.0, 8.5, 7.9),
    ];

    let measurements: Vec<CrossSectionMeasurement> = processes
        .iter()
        .map(
            |(name, xsec, stat, syst_up, syst_dn)| CrossSectionMeasurement {
                process_name: name.to_string(),
                cross_section_pb: *xsec,
                stat_error_pb: *stat,
                syst_error_up_pb: *syst_up,
                syst_error_down_pb: *syst_dn,
                luminosity_per_fb: 140.1,
                sqrt_s_gev: 13_600.0,
            },
        )
        .collect();

    let encoded = encode_to_vec(&measurements).expect("encode cross-section collection");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress cross-section collection");
    let decompressed = decompress(&compressed).expect("decompress cross-section collection");
    let (decoded, _): (Vec<CrossSectionMeasurement>, usize) =
        decode_from_slice(&decompressed).expect("decode cross-section collection");

    assert_eq!(measurements, decoded);

    // Repetitive floating-point fields should compress well
    assert!(
        compressed.len() < encoded.len(),
        "Compressed size {} should be less than uncompressed {}",
        compressed.len(),
        encoded.len()
    );
}

// --- Test 21: Detector alignment constants batch compression ratio ---
#[test]
fn test_detector_alignment_batch_compression_ratio() {
    let mut alignments: Vec<DetectorAlignment> = Vec::with_capacity(500);
    for i in 0..500_u32 {
        alignments.push(DetectorAlignment {
            module_id: 10_000 + i,
            subsystem: "PixelEndcapA".to_string(),
            dx_um: 1.0 + (i as f64) * 0.001,
            dy_um: -0.5 + (i as f64) * 0.0005,
            dz_um: 0.3 + (i as f64) * 0.0002,
            rotation_alpha_mrad: 0.001,
            rotation_beta_mrad: -0.0005,
            rotation_gamma_mrad: 0.0002,
            valid_from_run: 364_000,
            valid_to_run: 365_000,
        });
    }

    let encoded = encode_to_vec(&alignments).expect("encode alignment batch");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress alignment batch");
    let decompressed = decompress(&compressed).expect("decompress alignment batch");
    let (decoded, _): (Vec<DetectorAlignment>, usize) =
        decode_from_slice(&decompressed).expect("decode alignment batch");

    assert_eq!(alignments.len(), decoded.len());
    assert_eq!(alignments[0], decoded[0]);
    assert_eq!(alignments[499], decoded[499]);

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "Compression ratio {} should be < 1.0 for 500 alignment constants with repetitive subsystem strings",
        ratio
    );
}

// --- Test 22: Combined MC truth + PID + jets (complex nested event) ---
#[test]
fn test_complex_nested_mc_event_roundtrip() {
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct SimulatedEvent {
        truth_particles: Vec<MonteCarloTruth>,
        reco_jets: Vec<JetParameters>,
        pid_per_track: Vec<PidLikelihood>,
        met: MissingTransverseEnergy,
        vertices: Vec<VertexFit>,
    }

    let event = SimulatedEvent {
        truth_particles: vec![
            MonteCarloTruth {
                pdg_id: 6,
                status_code: 62,
                barcode: 500,
                production_vertex_x_mm: 0.0,
                production_vertex_y_mm: 0.0,
                production_vertex_z_mm: 0.0,
                four_momentum: FourMomentum {
                    px: 80.1,
                    py: -42.3,
                    pz: 250.7,
                    energy: 271.0,
                },
                parent_barcodes: vec![],
                child_barcodes: vec![501, 502],
            },
            MonteCarloTruth {
                pdg_id: -6,
                status_code: 62,
                barcode: 600,
                production_vertex_x_mm: 0.0,
                production_vertex_y_mm: 0.0,
                production_vertex_z_mm: 0.0,
                four_momentum: FourMomentum {
                    px: -72.5,
                    py: 55.1,
                    pz: -195.3,
                    energy: 220.8,
                },
                parent_barcodes: vec![],
                child_barcodes: vec![601, 602],
            },
            MonteCarloTruth {
                pdg_id: 24,
                status_code: 22,
                barcode: 501,
                production_vertex_x_mm: 0.0,
                production_vertex_y_mm: 0.0,
                production_vertex_z_mm: 0.0,
                four_momentum: FourMomentum {
                    px: 45.0,
                    py: -28.0,
                    pz: 110.0,
                    energy: 128.5,
                },
                parent_barcodes: vec![500],
                child_barcodes: vec![503, 504],
            },
        ],
        reco_jets: vec![
            JetParameters {
                jet_id: 0,
                algorithm: "AntiKt4EMPFlow".to_string(),
                radius_parameter: 0.4,
                pt_gev: 180.2,
                eta: 0.78,
                phi: -1.56,
                mass_gev: 15.3,
                num_constituents: 42,
                btag_weight: 0.95,
                jvt_score: 0.98,
                emf: 0.30,
            },
            JetParameters {
                jet_id: 1,
                algorithm: "AntiKt4EMPFlow".to_string(),
                radius_parameter: 0.4,
                pt_gev: 95.7,
                eta: -1.22,
                phi: 2.10,
                mass_gev: 10.1,
                num_constituents: 28,
                btag_weight: 0.88,
                jvt_score: 0.96,
                emf: 0.42,
            },
            JetParameters {
                jet_id: 2,
                algorithm: "AntiKt4EMPFlow".to_string(),
                radius_parameter: 0.4,
                pt_gev: 55.3,
                eta: 2.01,
                phi: 0.35,
                mass_gev: 5.8,
                num_constituents: 18,
                btag_weight: 0.05,
                jvt_score: 0.91,
                emf: 0.65,
            },
        ],
        pid_per_track: vec![
            PidLikelihood {
                electron: 0.01,
                muon: 0.97,
                pion: 0.01,
                kaon: 0.005,
                proton: 0.005,
            },
            PidLikelihood {
                electron: 0.02,
                muon: 0.03,
                pion: 0.85,
                kaon: 0.08,
                proton: 0.02,
            },
            PidLikelihood {
                electron: 0.01,
                muon: 0.01,
                pion: 0.10,
                kaon: 0.82,
                proton: 0.06,
            },
        ],
        met: MissingTransverseEnergy {
            met_x_gev: -55.0,
            met_y_gev: 28.3,
            met_magnitude_gev: 61.86,
            met_phi: 2.67,
            sum_et_gev: 1100.0,
            significance: 9.5,
        },
        vertices: vec![
            VertexFit {
                vertex_id: 0,
                x_mm: 0.008,
                y_mm: -0.004,
                z_mm: 5.6,
                chi2: 15.2,
                ndf: 22,
                num_tracks: 25,
                cov_xx: 0.0007,
                cov_yy: 0.0008,
                cov_zz: 0.028,
            },
            VertexFit {
                vertex_id: 1,
                x_mm: 0.5,
                y_mm: 0.3,
                z_mm: 7.2,
                chi2: 2.1,
                ndf: 2,
                num_tracks: 3,
                cov_xx: 0.008,
                cov_yy: 0.009,
                cov_zz: 0.20,
            },
        ],
    };

    let encoded = encode_to_vec(&event).expect("encode simulated event");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress simulated event");
    let decompressed = decompress(&compressed).expect("decompress simulated event");
    let (decoded, _): (SimulatedEvent, usize) =
        decode_from_slice(&decompressed).expect("decode simulated event");

    assert_eq!(event, decoded);
    assert_eq!(decoded.truth_particles.len(), 3);
    assert_eq!(decoded.reco_jets.len(), 3);
    assert_eq!(decoded.pid_per_track.len(), 3);
    assert_eq!(decoded.vertices.len(), 2);
}
