#![cfg(feature = "std")]
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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ── Domain types: Particle Physics Detector Systems ─────────────────────────

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
enum SubDetector {
    InnerTracker,
    OuterTracker,
    ElectromagneticCalorimeter,
    HadronicCalorimeter,
    MuonSpectrometer,
    ForwardDetector,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct TrackerHit {
    layer: u16,
    phi: f64,
    eta: f64,
    r: f64,
    z: f64,
    charge_deposited: f32,
    timestamp_ns: u64,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct CalorimeterCell {
    detector: SubDetector,
    tower_eta: i16,
    tower_phi: i16,
    sampling_layer: u8,
    energy_gev: f64,
    time_ns: f32,
    quality_flag: u8,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct MuonChamberHit {
    station: u8,
    chamber_type: String,
    local_x: f64,
    local_y: f64,
    drift_radius: f32,
    is_on_track: bool,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
enum ParticleId {
    Electron,
    Muon,
    Pion,
    Kaon,
    Proton,
    Photon,
    Neutron,
    Unknown(u32),
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct FourMomentum {
    px: f64,
    py: f64,
    pz: f64,
    energy: f64,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct ReconstructedTrack {
    track_id: u32,
    n_hits: u16,
    chi2: f64,
    ndf: u16,
    d0: f64,
    z0: f64,
    phi0: f64,
    theta: f64,
    q_over_p: f64,
    momentum: FourMomentum,
    particle_hypothesis: ParticleId,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct Jet {
    jet_id: u16,
    algorithm: String,
    radius_parameter: f32,
    momentum: FourMomentum,
    n_constituents: u32,
    em_fraction: f32,
    btag_weight: f64,
    is_b_tagged: bool,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct MissingTransverseEnergy {
    met_x: f64,
    met_y: f64,
    sum_et: f64,
    significance: f64,
    source: String,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
enum TriggerLevel {
    L1Hardware,
    L1Topological,
    HltStep1,
    HltStep2,
    HltFinal,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct TriggerDecision {
    chain_name: String,
    level: TriggerLevel,
    passed: bool,
    prescale: f32,
    run_time_us: u32,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct LuminosityBlock {
    run_number: u32,
    lb_number: u32,
    inst_luminosity: f64,
    integrated_luminosity_pb_inv: f64,
    pileup_mu: f32,
    n_vertices: u16,
    duration_s: f32,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct BeamParameters {
    beam_energy_gev: f64,
    beta_star_m: f64,
    crossing_angle_urad: f64,
    emittance_x_um: f64,
    emittance_y_um: f64,
    bunch_spacing_ns: u32,
    n_bunches: u16,
    beam_spot_x: f64,
    beam_spot_y: f64,
    beam_spot_z: f64,
    beam_spot_sigma_z: f64,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct MonteCarloTruthParticle {
    barcode: i64,
    pdg_id: i32,
    status: u16,
    momentum: FourMomentum,
    production_vertex_x: f64,
    production_vertex_y: f64,
    production_vertex_z: f64,
    parent_barcodes: Vec<i64>,
    child_barcodes: Vec<i64>,
    is_stable: bool,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct CrossSectionMeasurement {
    process_name: String,
    cross_section_pb: f64,
    stat_error_up: f64,
    stat_error_down: f64,
    syst_error_up: f64,
    syst_error_down: f64,
    luminosity_pb_inv: f64,
    n_signal: f64,
    n_background: f64,
    sqrt_s_gev: f64,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct Vertex {
    vertex_id: u16,
    x: f64,
    y: f64,
    z: f64,
    chi2: f64,
    ndf: u16,
    n_tracks: u16,
    vertex_type: u8,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct EventHeader {
    run_number: u32,
    event_number: u64,
    timestamp: u64,
    bunch_crossing_id: u16,
    lb_number: u32,
    detector_mask: u64,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct FullEvent {
    header: EventHeader,
    tracker_hits: Vec<TrackerHit>,
    calo_cells: Vec<CalorimeterCell>,
    muon_hits: Vec<MuonChamberHit>,
    tracks: Vec<ReconstructedTrack>,
    vertices: Vec<Vertex>,
    jets: Vec<Jet>,
    met: MissingTransverseEnergy,
    triggers: Vec<TriggerDecision>,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct DetectorAlignment {
    module_id: u32,
    sub_detector: SubDetector,
    dx: f64,
    dy: f64,
    dz: f64,
    rot_alpha: f64,
    rot_beta: f64,
    rot_gamma: f64,
    valid_from_run: u32,
    valid_to_run: u32,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct PileupEvent {
    n_interactions: u16,
    vertices: Vec<Vertex>,
    sum_pt_squared: Vec<f64>,
    is_hard_scatter: Vec<bool>,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
enum CalibrationStatus {
    NotCalibrated,
    Preliminary,
    Final,
    Superseded,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct CalibrationRecord {
    channel_id: u32,
    gain: f64,
    pedestal: f64,
    noise_rms: f64,
    status: CalibrationStatus,
    iov_start: u64,
    iov_end: u64,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct SystematicsVariation {
    name: String,
    up_weight: f64,
    down_weight: f64,
    is_correlated: bool,
    affected_processes: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct HistogramBin {
    low_edge: f64,
    high_edge: f64,
    content: f64,
    error: f64,
    entries: u64,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct Histogram {
    name: String,
    title: String,
    bins: Vec<HistogramBin>,
    underflow: f64,
    overflow: f64,
    total_entries: u64,
}

// ── Test 1: Tracker hit roundtrip via file ──────────────────────────────────

#[test]
fn test_tracker_hit_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_tracker_hit_{}.bin",
        std::process::id()
    ));
    let hit = TrackerHit {
        layer: 3,
        phi: 1.5707963,
        eta: -0.42,
        r: 88.5,
        z: 312.7,
        charge_deposited: 4.2,
        timestamp_ns: 1_000_000_042,
    };
    encode_to_file(&hit, &path).expect("encode tracker hit to file");
    let decoded: TrackerHit = decode_from_file(&path).expect("decode tracker hit from file");
    assert_eq!(hit, decoded);
    std::fs::remove_file(&path).expect("cleanup tracker hit file");
}

// ── Test 2: Calorimeter cell collection via vec ─────────────────────────────

#[test]
fn test_calorimeter_cells_vec_roundtrip() {
    let cells: Vec<CalorimeterCell> = vec![
        CalorimeterCell {
            detector: SubDetector::ElectromagneticCalorimeter,
            tower_eta: 14,
            tower_phi: 23,
            sampling_layer: 2,
            energy_gev: 35.6,
            time_ns: 1.2,
            quality_flag: 0,
        },
        CalorimeterCell {
            detector: SubDetector::HadronicCalorimeter,
            tower_eta: -7,
            tower_phi: 50,
            sampling_layer: 0,
            energy_gev: 112.3,
            time_ns: 3.1,
            quality_flag: 1,
        },
    ];
    let encoded = encode_to_vec(&cells).expect("encode calorimeter cells");
    let (decoded, _bytes_read): (Vec<CalorimeterCell>, usize) =
        decode_from_slice(&encoded).expect("decode calorimeter cells");
    assert_eq!(cells, decoded);
}

// ── Test 3: Muon chamber hits via file ──────────────────────────────────────

#[test]
fn test_muon_chamber_hits_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_muon_hits_{}.bin",
        std::process::id()
    ));
    let hits = vec![
        MuonChamberHit {
            station: 1,
            chamber_type: "MDT".to_string(),
            local_x: 15.32,
            local_y: -4.18,
            drift_radius: 7.25,
            is_on_track: true,
        },
        MuonChamberHit {
            station: 3,
            chamber_type: "CSC".to_string(),
            local_x: -22.1,
            local_y: 8.77,
            drift_radius: 0.0,
            is_on_track: false,
        },
    ];
    encode_to_file(&hits, &path).expect("encode muon hits to file");
    let decoded: Vec<MuonChamberHit> = decode_from_file(&path).expect("decode muon hits from file");
    assert_eq!(hits, decoded);
    std::fs::remove_file(&path).expect("cleanup muon hits file");
}

// ── Test 4: Particle identification enum variants ───────────────────────────

#[test]
fn test_particle_id_all_variants_vec() {
    let particles = vec![
        ParticleId::Electron,
        ParticleId::Muon,
        ParticleId::Pion,
        ParticleId::Kaon,
        ParticleId::Proton,
        ParticleId::Photon,
        ParticleId::Neutron,
        ParticleId::Unknown(999),
    ];
    let encoded = encode_to_vec(&particles).expect("encode particle IDs");
    let (decoded, _): (Vec<ParticleId>, usize) =
        decode_from_slice(&encoded).expect("decode particle IDs");
    assert_eq!(particles, decoded);
}

// ── Test 5: Reconstructed track with full kinematics ────────────────────────

#[test]
fn test_reconstructed_track_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_reco_track_{}.bin",
        std::process::id()
    ));
    let track = ReconstructedTrack {
        track_id: 42,
        n_hits: 36,
        chi2: 28.5,
        ndf: 30,
        d0: 0.015,
        z0: -1.23,
        phi0: 2.14159,
        theta: 1.2,
        q_over_p: 0.00015,
        momentum: FourMomentum {
            px: 45.2,
            py: -12.8,
            pz: 120.5,
            energy: 130.1,
        },
        particle_hypothesis: ParticleId::Muon,
    };
    encode_to_file(&track, &path).expect("encode reco track");
    let decoded: ReconstructedTrack = decode_from_file(&path).expect("decode reco track");
    assert_eq!(track, decoded);
    std::fs::remove_file(&path).expect("cleanup reco track file");
}

// ── Test 6: Jet clustering results via vec ──────────────────────────────────

#[test]
fn test_jet_clustering_vec() {
    let jets = vec![
        Jet {
            jet_id: 0,
            algorithm: "anti-kt".to_string(),
            radius_parameter: 0.4,
            momentum: FourMomentum {
                px: 200.0,
                py: -50.0,
                pz: 300.0,
                energy: 380.0,
            },
            n_constituents: 42,
            em_fraction: 0.65,
            btag_weight: 0.92,
            is_b_tagged: true,
        },
        Jet {
            jet_id: 1,
            algorithm: "anti-kt".to_string(),
            radius_parameter: 0.4,
            momentum: FourMomentum {
                px: -80.0,
                py: 120.0,
                pz: -60.0,
                energy: 155.0,
            },
            n_constituents: 28,
            em_fraction: 0.30,
            btag_weight: 0.05,
            is_b_tagged: false,
        },
    ];
    let encoded = encode_to_vec(&jets).expect("encode jets");
    let (decoded, _): (Vec<Jet>, usize) = decode_from_slice(&encoded).expect("decode jets");
    assert_eq!(jets, decoded);
}

// ── Test 7: Missing transverse energy via file ──────────────────────────────

#[test]
fn test_missing_et_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_met_{}.bin",
        std::process::id()
    ));
    let met = MissingTransverseEnergy {
        met_x: -45.3,
        met_y: 67.8,
        sum_et: 1250.0,
        significance: 8.5,
        source: "TST".to_string(),
    };
    encode_to_file(&met, &path).expect("encode MET");
    let decoded: MissingTransverseEnergy = decode_from_file(&path).expect("decode MET");
    assert_eq!(met, decoded);
    std::fs::remove_file(&path).expect("cleanup MET file");
}

// ── Test 8: Trigger decision chain via vec ──────────────────────────────────

#[test]
fn test_trigger_chain_vec() {
    let triggers = vec![
        TriggerDecision {
            chain_name: "L1_EM22VHI".to_string(),
            level: TriggerLevel::L1Hardware,
            passed: true,
            prescale: 1.0,
            run_time_us: 2,
        },
        TriggerDecision {
            chain_name: "HLT_e26_lhtight_nod0".to_string(),
            level: TriggerLevel::HltStep1,
            passed: true,
            prescale: 1.0,
            run_time_us: 150,
        },
        TriggerDecision {
            chain_name: "HLT_mu26_ivarmedium".to_string(),
            level: TriggerLevel::HltFinal,
            passed: false,
            prescale: 1.0,
            run_time_us: 200,
        },
    ];
    let encoded = encode_to_vec(&triggers).expect("encode triggers");
    let (decoded, _): (Vec<TriggerDecision>, usize) =
        decode_from_slice(&encoded).expect("decode triggers");
    assert_eq!(triggers, decoded);
}

// ── Test 9: Luminosity block via file ───────────────────────────────────────

#[test]
fn test_luminosity_block_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_lumi_{}.bin",
        std::process::id()
    ));
    let lb = LuminosityBlock {
        run_number: 364_292,
        lb_number: 512,
        inst_luminosity: 2.0e34,
        integrated_luminosity_pb_inv: 139.0,
        pileup_mu: 38.5,
        n_vertices: 42,
        duration_s: 60.0,
    };
    encode_to_file(&lb, &path).expect("encode luminosity block");
    let decoded: LuminosityBlock = decode_from_file(&path).expect("decode luminosity block");
    assert_eq!(lb, decoded);
    std::fs::remove_file(&path).expect("cleanup luminosity file");
}

// ── Test 10: Beam parameters via vec ────────────────────────────────────────

#[test]
fn test_beam_parameters_vec() {
    let beam = BeamParameters {
        beam_energy_gev: 6800.0,
        beta_star_m: 0.30,
        crossing_angle_urad: 160.0,
        emittance_x_um: 2.5,
        emittance_y_um: 2.5,
        bunch_spacing_ns: 25,
        n_bunches: 2748,
        beam_spot_x: 0.068,
        beam_spot_y: -0.012,
        beam_spot_z: 1.5,
        beam_spot_sigma_z: 42.0,
    };
    let encoded = encode_to_vec(&beam).expect("encode beam parameters");
    let (decoded, _): (BeamParameters, usize) =
        decode_from_slice(&encoded).expect("decode beam parameters");
    assert_eq!(beam, decoded);
}

// ── Test 11: Monte Carlo truth record via file ──────────────────────────────

#[test]
fn test_mc_truth_particle_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_mc_truth_{}.bin",
        std::process::id()
    ));
    let truth = MonteCarloTruthParticle {
        barcode: 1001,
        pdg_id: 6,  // top quark
        status: 22, // intermediate
        momentum: FourMomentum {
            px: 85.3,
            py: -42.1,
            pz: 210.0,
            energy: 235.0,
        },
        production_vertex_x: 0.0,
        production_vertex_y: 0.0,
        production_vertex_z: 0.0,
        parent_barcodes: vec![3, 4],
        child_barcodes: vec![1002, 1003, 1004],
        is_stable: false,
    };
    encode_to_file(&truth, &path).expect("encode MC truth particle");
    let decoded: MonteCarloTruthParticle =
        decode_from_file(&path).expect("decode MC truth particle");
    assert_eq!(truth, decoded);
    std::fs::remove_file(&path).expect("cleanup MC truth file");
}

// ── Test 12: Cross-section measurement via vec ──────────────────────────────

#[test]
fn test_cross_section_measurement_vec() {
    let xsec = CrossSectionMeasurement {
        process_name: "pp -> ttbar -> l+jets".to_string(),
        cross_section_pb: 831.76,
        stat_error_up: 2.3,
        stat_error_down: 2.3,
        syst_error_up: 35.0,
        syst_error_down: 38.0,
        luminosity_pb_inv: 139_000.0,
        n_signal: 25400.0,
        n_background: 8200.0,
        sqrt_s_gev: 13000.0,
    };
    let encoded = encode_to_vec(&xsec).expect("encode cross section");
    let (decoded, _): (CrossSectionMeasurement, usize) =
        decode_from_slice(&encoded).expect("decode cross section");
    assert_eq!(xsec, decoded);
}

// ── Test 13: Full event reconstruction via file ─────────────────────────────

#[test]
fn test_full_event_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_full_event_{}.bin",
        std::process::id()
    ));
    let event = FullEvent {
        header: EventHeader {
            run_number: 410_000,
            event_number: 123_456_789,
            timestamp: 1_700_000_000_000,
            bunch_crossing_id: 1234,
            lb_number: 100,
            detector_mask: 0xFFFF_FFFF_FFFF_0000,
        },
        tracker_hits: vec![TrackerHit {
            layer: 0,
            phi: 0.5,
            eta: 1.0,
            r: 33.0,
            z: 50.0,
            charge_deposited: 3.8,
            timestamp_ns: 100,
        }],
        calo_cells: vec![CalorimeterCell {
            detector: SubDetector::ElectromagneticCalorimeter,
            tower_eta: 10,
            tower_phi: 20,
            sampling_layer: 1,
            energy_gev: 55.0,
            time_ns: 0.5,
            quality_flag: 0,
        }],
        muon_hits: vec![],
        tracks: vec![ReconstructedTrack {
            track_id: 1,
            n_hits: 12,
            chi2: 10.5,
            ndf: 10,
            d0: 0.002,
            z0: 0.5,
            phi0: 1.0,
            theta: 1.5,
            q_over_p: 0.001,
            momentum: FourMomentum {
                px: 30.0,
                py: 40.0,
                pz: 50.0,
                energy: 70.7,
            },
            particle_hypothesis: ParticleId::Electron,
        }],
        vertices: vec![Vertex {
            vertex_id: 0,
            x: 0.01,
            y: -0.005,
            z: 1.2,
            chi2: 5.0,
            ndf: 8,
            n_tracks: 5,
            vertex_type: 1,
        }],
        jets: vec![],
        met: MissingTransverseEnergy {
            met_x: 20.0,
            met_y: -15.0,
            sum_et: 500.0,
            significance: 3.2,
            source: "PFlow".to_string(),
        },
        triggers: vec![TriggerDecision {
            chain_name: "HLT_e60_lhmedium".to_string(),
            level: TriggerLevel::HltFinal,
            passed: true,
            prescale: 1.0,
            run_time_us: 120,
        }],
    };
    encode_to_file(&event, &path).expect("encode full event");
    let decoded: FullEvent = decode_from_file(&path).expect("decode full event");
    assert_eq!(event, decoded);
    std::fs::remove_file(&path).expect("cleanup full event file");
}

// ── Test 14: Detector alignment constants via file ──────────────────────────

#[test]
fn test_detector_alignment_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_alignment_{}.bin",
        std::process::id()
    ));
    let alignments: Vec<DetectorAlignment> = (0..5)
        .map(|i| DetectorAlignment {
            module_id: 1000 + i,
            sub_detector: SubDetector::InnerTracker,
            dx: 0.001 * (i as f64),
            dy: -0.002 * (i as f64),
            dz: 0.0005 * (i as f64),
            rot_alpha: 1e-5 * (i as f64),
            rot_beta: -2e-5 * (i as f64),
            rot_gamma: 0.5e-5 * (i as f64),
            valid_from_run: 400_000,
            valid_to_run: 410_000,
        })
        .collect();
    encode_to_file(&alignments, &path).expect("encode alignment constants");
    let decoded: Vec<DetectorAlignment> =
        decode_from_file(&path).expect("decode alignment constants");
    assert_eq!(alignments, decoded);
    std::fs::remove_file(&path).expect("cleanup alignment file");
}

// ── Test 15: Pileup event with multiple vertices via vec ────────────────────

#[test]
fn test_pileup_event_vec() {
    let pileup = PileupEvent {
        n_interactions: 40,
        vertices: (0..40)
            .map(|i| Vertex {
                vertex_id: i,
                x: 0.01 * (i as f64),
                y: -0.005 * (i as f64),
                z: -200.0 + 10.0 * (i as f64),
                chi2: 2.0 + 0.5 * (i as f64),
                ndf: 4,
                n_tracks: 3 + i,
                vertex_type: if i == 0 { 1 } else { 3 },
            })
            .collect(),
        sum_pt_squared: (0..40).map(|i| 100.0 + 50.0 * (i as f64)).collect(),
        is_hard_scatter: {
            let mut v = vec![false; 40];
            v[0] = true;
            v
        },
    };
    let encoded = encode_to_vec(&pileup).expect("encode pileup event");
    let (decoded, _): (PileupEvent, usize) =
        decode_from_slice(&encoded).expect("decode pileup event");
    assert_eq!(pileup, decoded);
}

// ── Test 16: Calibration records via file ───────────────────────────────────

#[test]
fn test_calibration_records_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_calib_{}.bin",
        std::process::id()
    ));
    let records: Vec<CalibrationRecord> = vec![
        CalibrationRecord {
            channel_id: 0,
            gain: 1.0,
            pedestal: 200.0,
            noise_rms: 1.5,
            status: CalibrationStatus::Final,
            iov_start: 1_000_000,
            iov_end: 2_000_000,
        },
        CalibrationRecord {
            channel_id: 1,
            gain: 0.98,
            pedestal: 198.5,
            noise_rms: 1.7,
            status: CalibrationStatus::Preliminary,
            iov_start: 1_000_000,
            iov_end: 1_500_000,
        },
        CalibrationRecord {
            channel_id: 2,
            gain: 0.0,
            pedestal: 0.0,
            noise_rms: 999.0,
            status: CalibrationStatus::NotCalibrated,
            iov_start: 0,
            iov_end: 0,
        },
    ];
    encode_to_file(&records, &path).expect("encode calibration records");
    let decoded: Vec<CalibrationRecord> =
        decode_from_file(&path).expect("decode calibration records");
    assert_eq!(records, decoded);
    std::fs::remove_file(&path).expect("cleanup calibration file");
}

// ── Test 17: Systematics variations via vec ─────────────────────────────────

#[test]
fn test_systematics_variations_vec() {
    let systs = vec![
        SystematicsVariation {
            name: "JES_NP1".to_string(),
            up_weight: 1.02,
            down_weight: 0.98,
            is_correlated: true,
            affected_processes: vec!["ttbar".to_string(), "W+jets".to_string()],
        },
        SystematicsVariation {
            name: "MUON_SCALE".to_string(),
            up_weight: 1.005,
            down_weight: 0.995,
            is_correlated: true,
            affected_processes: vec![
                "ttbar".to_string(),
                "Z+jets".to_string(),
                "Diboson".to_string(),
            ],
        },
        SystematicsVariation {
            name: "BTAG_LIGHT_0".to_string(),
            up_weight: 1.10,
            down_weight: 0.90,
            is_correlated: false,
            affected_processes: vec!["ttbar".to_string()],
        },
    ];
    let encoded = encode_to_vec(&systs).expect("encode systematics");
    let (decoded, _): (Vec<SystematicsVariation>, usize) =
        decode_from_slice(&encoded).expect("decode systematics");
    assert_eq!(systs, decoded);
}

// ── Test 18: Histogram via file ─────────────────────────────────────────────

#[test]
fn test_histogram_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_histogram_{}.bin",
        std::process::id()
    ));
    let hist = Histogram {
        name: "h_mll".to_string(),
        title: "Dilepton invariant mass;m_{ll} [GeV];Events / 2 GeV".to_string(),
        bins: (0..50)
            .map(|i| {
                let low = 60.0 + 2.0 * (i as f64);
                let high = low + 2.0;
                let center = (low + high) / 2.0;
                let content =
                    1000.0 * (-(center - 91.2_f64).powi(2) / (2.0 * 2.5_f64.powi(2))).exp();
                HistogramBin {
                    low_edge: low,
                    high_edge: high,
                    content,
                    error: content.sqrt().max(1.0),
                    entries: content as u64,
                }
            })
            .collect(),
        underflow: 12.0,
        overflow: 8.0,
        total_entries: 45_000,
    };
    encode_to_file(&hist, &path).expect("encode histogram");
    let decoded: Histogram = decode_from_file(&path).expect("decode histogram");
    assert_eq!(hist, decoded);
    std::fs::remove_file(&path).expect("cleanup histogram file");
}

// ── Test 19: MC truth decay chain (B meson) via file ────────────────────────

#[test]
fn test_mc_truth_decay_chain_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_decay_chain_{}.bin",
        std::process::id()
    ));
    let b_meson = MonteCarloTruthParticle {
        barcode: 500,
        pdg_id: 521, // B+
        status: 2,
        momentum: FourMomentum {
            px: 15.0,
            py: -8.0,
            pz: 40.0,
            energy: 44.5,
        },
        production_vertex_x: 0.0,
        production_vertex_y: 0.0,
        production_vertex_z: 0.0,
        parent_barcodes: vec![100],
        child_barcodes: vec![501, 502, 503],
        is_stable: false,
    };
    let jpsi = MonteCarloTruthParticle {
        barcode: 501,
        pdg_id: 443, // J/psi
        status: 2,
        momentum: FourMomentum {
            px: 10.0,
            py: -5.0,
            pz: 25.0,
            energy: 27.8,
        },
        production_vertex_x: 0.5,
        production_vertex_y: -0.1,
        production_vertex_z: 2.0,
        parent_barcodes: vec![500],
        child_barcodes: vec![504, 505],
        is_stable: false,
    };
    let kaon = MonteCarloTruthParticle {
        barcode: 502,
        pdg_id: 321, // K+
        status: 1,
        momentum: FourMomentum {
            px: 5.0,
            py: -3.0,
            pz: 15.0,
            energy: 16.2,
        },
        production_vertex_x: 0.5,
        production_vertex_y: -0.1,
        production_vertex_z: 2.0,
        parent_barcodes: vec![500],
        child_barcodes: vec![],
        is_stable: true,
    };
    let chain = vec![b_meson, jpsi, kaon];
    encode_to_file(&chain, &path).expect("encode decay chain");
    let decoded: Vec<MonteCarloTruthParticle> =
        decode_from_file(&path).expect("decode decay chain");
    assert_eq!(chain, decoded);
    std::fs::remove_file(&path).expect("cleanup decay chain file");
}

// ── Test 20: Multiple trigger levels via file ───────────────────────────────

#[test]
fn test_trigger_levels_all_variants_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_trigger_levels_{}.bin",
        std::process::id()
    ));
    let decisions = vec![
        TriggerDecision {
            chain_name: "L1_J120".to_string(),
            level: TriggerLevel::L1Hardware,
            passed: true,
            prescale: 1.0,
            run_time_us: 1,
        },
        TriggerDecision {
            chain_name: "L1_TOPO_2MU4".to_string(),
            level: TriggerLevel::L1Topological,
            passed: true,
            prescale: 1.0,
            run_time_us: 3,
        },
        TriggerDecision {
            chain_name: "HLT_j420_a10t".to_string(),
            level: TriggerLevel::HltStep1,
            passed: false,
            prescale: 1.0,
            run_time_us: 80,
        },
        TriggerDecision {
            chain_name: "HLT_j420_a10t_lcw".to_string(),
            level: TriggerLevel::HltStep2,
            passed: false,
            prescale: 1.0,
            run_time_us: 250,
        },
        TriggerDecision {
            chain_name: "HLT_2mu14".to_string(),
            level: TriggerLevel::HltFinal,
            passed: true,
            prescale: 1.0,
            run_time_us: 300,
        },
    ];
    encode_to_file(&decisions, &path).expect("encode trigger levels");
    let decoded: Vec<TriggerDecision> = decode_from_file(&path).expect("decode trigger levels");
    assert_eq!(decisions, decoded);
    std::fs::remove_file(&path).expect("cleanup trigger levels file");
}

// ── Test 21: Multi-histogram analysis output via file ───────────────────────

#[test]
fn test_multi_histogram_analysis_file() {
    let path = temp_dir().join(format!(
        "oxicode_test_physics_multi_hist_{}.bin",
        std::process::id()
    ));
    let make_flat_hist = |name: &str, title: &str, n_bins: usize, x_min: f64, x_max: f64| {
        let bin_width = (x_max - x_min) / (n_bins as f64);
        Histogram {
            name: name.to_string(),
            title: title.to_string(),
            bins: (0..n_bins)
                .map(|i| {
                    let low = x_min + bin_width * (i as f64);
                    HistogramBin {
                        low_edge: low,
                        high_edge: low + bin_width,
                        content: 100.0 + 10.0 * (i as f64),
                        error: 10.0,
                        entries: 100 + 10 * (i as u64),
                    }
                })
                .collect(),
            underflow: 5.0,
            overflow: 3.0,
            total_entries: 10_000,
        }
    };
    let histograms = vec![
        make_flat_hist(
            "h_pt_jet1",
            "Leading jet p_{T};p_{T} [GeV];Events",
            20,
            0.0,
            1000.0,
        ),
        make_flat_hist("h_eta_jet1", "Leading jet #eta;#eta;Events", 25, -2.5, 2.5),
        make_flat_hist(
            "h_met",
            "Missing E_{T};E_{T}^{miss} [GeV];Events",
            30,
            0.0,
            600.0,
        ),
    ];
    encode_to_file(&histograms, &path).expect("encode multi-histogram");
    let decoded: Vec<Histogram> = decode_from_file(&path).expect("decode multi-histogram");
    assert_eq!(histograms, decoded);
    std::fs::remove_file(&path).expect("cleanup multi-histogram file");
}

// ── Test 22: Cross-section with systematics summary via vec ─────────────────

#[test]
fn test_cross_section_with_systematics_vec() {
    let analysis = (
        CrossSectionMeasurement {
            process_name: "pp -> WW -> enuemunu".to_string(),
            cross_section_pb: 12.6,
            stat_error_up: 0.3,
            stat_error_down: 0.3,
            syst_error_up: 0.8,
            syst_error_down: 0.9,
            luminosity_pb_inv: 139_000.0,
            n_signal: 4500.0,
            n_background: 1200.0,
            sqrt_s_gev: 13000.0,
        },
        vec![
            SystematicsVariation {
                name: "ELECTRON_EFF_ID".to_string(),
                up_weight: 1.01,
                down_weight: 0.99,
                is_correlated: true,
                affected_processes: vec!["WW".to_string()],
            },
            SystematicsVariation {
                name: "MUON_EFF_TRIG".to_string(),
                up_weight: 1.005,
                down_weight: 0.995,
                is_correlated: true,
                affected_processes: vec!["WW".to_string(), "ttbar".to_string()],
            },
            SystematicsVariation {
                name: "PDF_NNPDF30".to_string(),
                up_weight: 1.015,
                down_weight: 0.985,
                is_correlated: true,
                affected_processes: vec![
                    "WW".to_string(),
                    "WZ".to_string(),
                    "ZZ".to_string(),
                    "ttbar".to_string(),
                ],
            },
        ],
        BeamParameters {
            beam_energy_gev: 6500.0,
            beta_star_m: 0.40,
            crossing_angle_urad: 150.0,
            emittance_x_um: 3.0,
            emittance_y_um: 3.0,
            bunch_spacing_ns: 25,
            n_bunches: 2556,
            beam_spot_x: 0.07,
            beam_spot_y: -0.01,
            beam_spot_z: 0.8,
            beam_spot_sigma_z: 44.0,
        },
    );
    let encoded = encode_to_vec(&analysis).expect("encode analysis summary");
    let (decoded, _): (
        (
            CrossSectionMeasurement,
            Vec<SystematicsVariation>,
            BeamParameters,
        ),
        usize,
    ) = decode_from_slice(&encoded).expect("decode analysis summary");
    assert_eq!(analysis, decoded);
}
