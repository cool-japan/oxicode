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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ParticleType {
    Electron,
    Muon,
    Tau,
    Photon,
    Proton,
    Neutron,
    Pion,
    Kaon,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DetectorSubsystem {
    InnerTracker,
    ElectromagnCalorimeter,
    HadronCalorimeter,
    MuonSpectrometer,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DetectorHit {
    hit_id: u32,
    subsystem: DetectorSubsystem,
    layer: u8,
    x_um: i32,
    y_um: i32,
    z_um: i32,
    energy_mev: u32,
    time_ps: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ParticleTrack {
    track_id: u32,
    particle_type: ParticleType,
    charge: i8,
    pt_mev: u32,
    eta_micro: i32,
    phi_micro: i32,
    hits: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CollisionEvent {
    event_id: u64,
    run_number: u32,
    lumi_block: u32,
    tracks: Vec<ParticleTrack>,
    hits: Vec<DetectorHit>,
    beam_energy_gev: u32,
}

// --- ParticleType variant roundtrips ---

#[test]
fn test_particle_type_electron() {
    let cfg = config::standard();
    let val = ParticleType::Electron;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Electron");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Electron");
    assert_eq!(val, decoded);
}

#[test]
fn test_particle_type_muon() {
    let cfg = config::standard();
    let val = ParticleType::Muon;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Muon");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Muon");
    assert_eq!(val, decoded);
}

#[test]
fn test_particle_type_tau() {
    let cfg = config::standard();
    let val = ParticleType::Tau;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Tau");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Tau");
    assert_eq!(val, decoded);
}

#[test]
fn test_particle_type_photon() {
    let cfg = config::standard();
    let val = ParticleType::Photon;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Photon");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Photon");
    assert_eq!(val, decoded);
}

#[test]
fn test_particle_type_proton() {
    let cfg = config::standard();
    let val = ParticleType::Proton;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Proton");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Proton");
    assert_eq!(val, decoded);
}

#[test]
fn test_particle_type_neutron() {
    let cfg = config::standard();
    let val = ParticleType::Neutron;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Neutron");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Neutron");
    assert_eq!(val, decoded);
}

#[test]
fn test_particle_type_pion() {
    let cfg = config::standard();
    let val = ParticleType::Pion;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Pion");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Pion");
    assert_eq!(val, decoded);
}

#[test]
fn test_particle_type_kaon() {
    let cfg = config::standard();
    let val = ParticleType::Kaon;
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleType::Kaon");
    let (decoded, _): (ParticleType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleType::Kaon");
    assert_eq!(val, decoded);
}

// --- DetectorSubsystem variant roundtrips ---

#[test]
fn test_detector_subsystem_inner_tracker() {
    let cfg = config::standard();
    let val = DetectorSubsystem::InnerTracker;
    let bytes = encode_to_vec(&val, cfg).expect("encode DetectorSubsystem::InnerTracker");
    let (decoded, _): (DetectorSubsystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DetectorSubsystem::InnerTracker");
    assert_eq!(val, decoded);
}

#[test]
fn test_detector_subsystem_electromagn_calorimeter() {
    let cfg = config::standard();
    let val = DetectorSubsystem::ElectromagnCalorimeter;
    let bytes = encode_to_vec(&val, cfg).expect("encode DetectorSubsystem::ElectromagnCalorimeter");
    let (decoded, _): (DetectorSubsystem, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode DetectorSubsystem::ElectromagnCalorimeter");
    assert_eq!(val, decoded);
}

#[test]
fn test_detector_subsystem_hadron_calorimeter() {
    let cfg = config::standard();
    let val = DetectorSubsystem::HadronCalorimeter;
    let bytes = encode_to_vec(&val, cfg).expect("encode DetectorSubsystem::HadronCalorimeter");
    let (decoded, _): (DetectorSubsystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DetectorSubsystem::HadronCalorimeter");
    assert_eq!(val, decoded);
}

#[test]
fn test_detector_subsystem_muon_spectrometer() {
    let cfg = config::standard();
    let val = DetectorSubsystem::MuonSpectrometer;
    let bytes = encode_to_vec(&val, cfg).expect("encode DetectorSubsystem::MuonSpectrometer");
    let (decoded, _): (DetectorSubsystem, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DetectorSubsystem::MuonSpectrometer");
    assert_eq!(val, decoded);
}

// --- DetectorHit standard roundtrip ---

#[test]
fn test_detector_hit_roundtrip_standard() {
    let cfg = config::standard();
    let val = DetectorHit {
        hit_id: 42,
        subsystem: DetectorSubsystem::InnerTracker,
        layer: 3,
        x_um: 1500,
        y_um: -800,
        z_um: 25000,
        energy_mev: 150,
        time_ps: 1_234_567_890,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DetectorHit standard");
    let (decoded, _): (DetectorHit, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DetectorHit standard");
    assert_eq!(val, decoded);
}

// --- ParticleTrack roundtrip ---

#[test]
fn test_particle_track_roundtrip() {
    let cfg = config::standard();
    let val = ParticleTrack {
        track_id: 101,
        particle_type: ParticleType::Muon,
        charge: -1,
        pt_mev: 45_000,
        eta_micro: 1_250_000,
        phi_micro: 785_398,
        hits: vec![1, 2, 3, 4, 5, 6],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleTrack");
    let (decoded, _): (ParticleTrack, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleTrack");
    assert_eq!(val, decoded);
}

// --- CollisionEvent with empty tracks/hits ---

#[test]
fn test_collision_event_empty_tracks_hits() {
    let cfg = config::standard();
    let val = CollisionEvent {
        event_id: 9_999_999,
        run_number: 300_000,
        lumi_block: 1,
        tracks: vec![],
        hits: vec![],
        beam_energy_gev: 6500,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CollisionEvent empty");
    let (decoded, _): (CollisionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CollisionEvent empty");
    assert_eq!(val, decoded);
}

// --- CollisionEvent with 5 tracks ---

#[test]
fn test_collision_event_five_tracks() {
    let cfg = config::standard();
    let make_track = |id: u32, ptype: ParticleType, charge: i8, pt: u32| ParticleTrack {
        track_id: id,
        particle_type: ptype,
        charge,
        pt_mev: pt,
        eta_micro: 500_000,
        phi_micro: 1_000_000,
        hits: vec![id * 10, id * 10 + 1, id * 10 + 2],
    };
    let val = CollisionEvent {
        event_id: 10_000,
        run_number: 350_001,
        lumi_block: 12,
        tracks: vec![
            make_track(1, ParticleType::Electron, -1, 30_000),
            make_track(2, ParticleType::Muon, -1, 42_000),
            make_track(3, ParticleType::Pion, 1, 8_000),
            make_track(4, ParticleType::Kaon, 1, 5_500),
            make_track(5, ParticleType::Proton, 1, 60_000),
        ],
        hits: vec![],
        beam_energy_gev: 6500,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CollisionEvent 5 tracks");
    let (decoded, _): (CollisionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CollisionEvent 5 tracks");
    assert_eq!(val, decoded);
}

// --- CollisionEvent with 10 hits ---

#[test]
fn test_collision_event_ten_hits() {
    let cfg = config::standard();
    let make_hit = |id: u32, sub: DetectorSubsystem, layer: u8| DetectorHit {
        hit_id: id,
        subsystem: sub,
        layer,
        x_um: (id as i32) * 100,
        y_um: -((id as i32) * 50),
        z_um: (id as i32) * 200,
        energy_mev: id * 10,
        time_ps: (id as u64) * 1000,
    };
    let val = CollisionEvent {
        event_id: 20_000,
        run_number: 350_002,
        lumi_block: 25,
        tracks: vec![],
        hits: vec![
            make_hit(1, DetectorSubsystem::InnerTracker, 1),
            make_hit(2, DetectorSubsystem::InnerTracker, 2),
            make_hit(3, DetectorSubsystem::ElectromagnCalorimeter, 1),
            make_hit(4, DetectorSubsystem::ElectromagnCalorimeter, 2),
            make_hit(5, DetectorSubsystem::ElectromagnCalorimeter, 3),
            make_hit(6, DetectorSubsystem::HadronCalorimeter, 1),
            make_hit(7, DetectorSubsystem::HadronCalorimeter, 2),
            make_hit(8, DetectorSubsystem::HadronCalorimeter, 3),
            make_hit(9, DetectorSubsystem::MuonSpectrometer, 1),
            make_hit(10, DetectorSubsystem::MuonSpectrometer, 2),
        ],
        beam_energy_gev: 6500,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CollisionEvent 10 hits");
    let (decoded, _): (CollisionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CollisionEvent 10 hits");
    assert_eq!(val, decoded);
}

// --- Vec<DetectorHit> roundtrip ---

#[test]
fn test_vec_detector_hit_roundtrip() {
    let cfg = config::standard();
    let val: Vec<DetectorHit> = vec![
        DetectorHit {
            hit_id: 1,
            subsystem: DetectorSubsystem::InnerTracker,
            layer: 1,
            x_um: 200,
            y_um: -300,
            z_um: 5000,
            energy_mev: 50,
            time_ps: 100_000,
        },
        DetectorHit {
            hit_id: 2,
            subsystem: DetectorSubsystem::ElectromagnCalorimeter,
            layer: 2,
            x_um: -1000,
            y_um: 800,
            z_um: 15000,
            energy_mev: 2500,
            time_ps: 250_000,
        },
        DetectorHit {
            hit_id: 3,
            subsystem: DetectorSubsystem::MuonSpectrometer,
            layer: 4,
            x_um: 5000,
            y_um: 5000,
            z_um: 80000,
            energy_mev: 300,
            time_ps: 5_000_000,
        },
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<DetectorHit>");
    let (decoded, _): (Vec<DetectorHit>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<DetectorHit>");
    assert_eq!(val, decoded);
}

// --- Vec<ParticleTrack> roundtrip ---

#[test]
fn test_vec_particle_track_roundtrip() {
    let cfg = config::standard();
    let val: Vec<ParticleTrack> = vec![
        ParticleTrack {
            track_id: 1,
            particle_type: ParticleType::Electron,
            charge: -1,
            pt_mev: 25_000,
            eta_micro: -2_100_000,
            phi_micro: 314_159,
            hits: vec![10, 11, 12],
        },
        ParticleTrack {
            track_id: 2,
            particle_type: ParticleType::Photon,
            charge: 0,
            pt_mev: 80_000,
            eta_micro: 0,
            phi_micro: 1_570_796,
            hits: vec![20, 21],
        },
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<ParticleTrack>");
    let (decoded, _): (Vec<ParticleTrack>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ParticleTrack>");
    assert_eq!(val, decoded);
}

// --- big_endian config ---

#[test]
fn test_big_endian_config_detector_hit() {
    let cfg = config::standard().with_big_endian();
    let val = DetectorHit {
        hit_id: 777,
        subsystem: DetectorSubsystem::HadronCalorimeter,
        layer: 5,
        x_um: -4200,
        y_um: 3100,
        z_um: 70000,
        energy_mev: 8_500,
        time_ps: 9_876_543_210,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DetectorHit big_endian");
    let (decoded, _): (DetectorHit, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DetectorHit big_endian");
    assert_eq!(val, decoded);
}

// --- fixed_int config ---

#[test]
fn test_fixed_int_config_particle_track() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = ParticleTrack {
        track_id: 555,
        particle_type: ParticleType::Tau,
        charge: 1,
        pt_mev: 120_000,
        eta_micro: -500_000,
        phi_micro: 2_356_194,
        hits: vec![100, 101, 102, 103],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleTrack fixed_int");
    let (decoded, _): (ParticleTrack, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleTrack fixed_int");
    assert_eq!(val, decoded);
}

// --- big_endian + fixed_int combined config ---

#[test]
fn test_big_endian_fixed_int_collision_event() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val = CollisionEvent {
        event_id: 123_456_789,
        run_number: 400_000,
        lumi_block: 88,
        tracks: vec![ParticleTrack {
            track_id: 1,
            particle_type: ParticleType::Muon,
            charge: -1,
            pt_mev: 55_000,
            eta_micro: 1_800_000,
            phi_micro: 942_477,
            hits: vec![50, 51, 52, 53, 54],
        }],
        hits: vec![DetectorHit {
            hit_id: 50,
            subsystem: DetectorSubsystem::MuonSpectrometer,
            layer: 3,
            x_um: 10_000,
            y_um: 10_000,
            z_um: 100_000,
            energy_mev: 400,
            time_ps: 8_000_000,
        }],
        beam_energy_gev: 6500,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode CollisionEvent big_endian+fixed_int");
    let (decoded, _): (CollisionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CollisionEvent big_endian+fixed_int");
    assert_eq!(val, decoded);
}

// --- consumed bytes check ---

#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let cfg = config::standard();
    let val = ParticleTrack {
        track_id: 999,
        particle_type: ParticleType::Kaon,
        charge: 1,
        pt_mev: 3_500,
        eta_micro: 2_400_000,
        phi_micro: 4_712_388,
        hits: vec![200, 201, 202],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ParticleTrack consumed check");
    let (_decoded, consumed): (ParticleTrack, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ParticleTrack consumed check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// --- high-energy collision (1000 GeV) ---

#[test]
fn test_high_energy_collision_1000_gev() {
    let cfg = config::standard();
    let val = CollisionEvent {
        event_id: 1,
        run_number: 1,
        lumi_block: 1,
        tracks: vec![
            ParticleTrack {
                track_id: 1,
                particle_type: ParticleType::Proton,
                charge: 1,
                pt_mev: 500_000,
                eta_micro: 0,
                phi_micro: 0,
                hits: vec![1, 2, 3],
            },
            ParticleTrack {
                track_id: 2,
                particle_type: ParticleType::Proton,
                charge: 1,
                pt_mev: 500_000,
                eta_micro: 0,
                phi_micro: 3_141_592,
                hits: vec![4, 5, 6],
            },
        ],
        hits: vec![],
        beam_energy_gev: 1000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode high-energy 1000 GeV collision");
    let (decoded, _): (CollisionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode high-energy 1000 GeV collision");
    assert_eq!(val, decoded);
    assert_eq!(decoded.beam_energy_gev, 1000);
}

// --- muon spectrometer hits ---

#[test]
fn test_muon_spectrometer_hits() {
    let cfg = config::standard();
    let muon_hits: Vec<DetectorHit> = (0..8)
        .map(|i| DetectorHit {
            hit_id: 1000 + i,
            subsystem: DetectorSubsystem::MuonSpectrometer,
            layer: (i % 4 + 1) as u8,
            x_um: (i as i32) * 3000,
            y_um: (i as i32) * 3000,
            z_um: 200_000 + (i as i32) * 50_000,
            energy_mev: 200 + i * 5,
            time_ps: 10_000_000 + (i as u64) * 500_000,
        })
        .collect();
    let bytes = encode_to_vec(&muon_hits, cfg).expect("encode muon spectrometer hits");
    let (decoded, _): (Vec<DetectorHit>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode muon spectrometer hits");
    assert_eq!(muon_hits, decoded);
    assert!(decoded
        .iter()
        .all(|h| h.subsystem == DetectorSubsystem::MuonSpectrometer));
}

// --- electromagnetic shower (50 ECAL hits) ---

#[test]
fn test_electromagnetic_shower_fifty_ecal_hits() {
    let cfg = config::standard();
    let ecal_hits: Vec<DetectorHit> = (0..50)
        .map(|i| DetectorHit {
            hit_id: 2000 + i,
            subsystem: DetectorSubsystem::ElectromagnCalorimeter,
            layer: (i % 5 + 1) as u8,
            x_um: ((i as i32) - 25) * 500,
            y_um: ((i as i32) - 25) * 500,
            z_um: 30_000 + (i as i32) * 100,
            energy_mev: 500 + i * 20,
            time_ps: 3_000_000 + (i as u64) * 10_000,
        })
        .collect();
    let bytes = encode_to_vec(&ecal_hits, cfg).expect("encode EM shower 50 ECAL hits");
    let (decoded, _): (Vec<DetectorHit>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EM shower 50 ECAL hits");
    assert_eq!(ecal_hits, decoded);
    assert_eq!(decoded.len(), 50);
    let total_energy: u32 = decoded.iter().map(|h| h.energy_mev).sum();
    assert!(total_energy > 0, "EM shower must deposit energy");
}

// --- negative charge particle ---

#[test]
fn test_negative_charge_particle() {
    let cfg = config::standard();
    let val = ParticleTrack {
        track_id: 300,
        particle_type: ParticleType::Electron,
        charge: -1,
        pt_mev: 15_000,
        eta_micro: -1_200_000,
        phi_micro: 6_000_000,
        hits: vec![300, 301, 302, 303],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode negative charge particle");
    let (decoded, _): (ParticleTrack, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode negative charge particle");
    assert_eq!(val, decoded);
    assert_eq!(decoded.charge, -1);
}

// --- zero hits track (ghost track) ---

#[test]
fn test_ghost_track_zero_hits() {
    let cfg = config::standard();
    let val = ParticleTrack {
        track_id: 9999,
        particle_type: ParticleType::Pion,
        charge: 1,
        pt_mev: 1_200,
        eta_micro: 3_000_000,
        phi_micro: 628_318,
        hits: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ghost track");
    let (decoded, _): (ParticleTrack, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ghost track");
    assert_eq!(val, decoded);
    assert!(
        decoded.hits.is_empty(),
        "ghost track must have zero associated hits"
    );
}

// --- large event (20 tracks, 100 hits) ---

#[test]
fn test_large_event_twenty_tracks_hundred_hits() {
    let cfg = config::standard();
    let particle_types = [
        ParticleType::Electron,
        ParticleType::Muon,
        ParticleType::Pion,
        ParticleType::Kaon,
        ParticleType::Proton,
    ];
    let subsystems = [
        DetectorSubsystem::InnerTracker,
        DetectorSubsystem::ElectromagnCalorimeter,
        DetectorSubsystem::HadronCalorimeter,
        DetectorSubsystem::MuonSpectrometer,
    ];
    let tracks: Vec<ParticleTrack> = (0..20)
        .map(|i| ParticleTrack {
            track_id: i,
            particle_type: particle_types[(i as usize) % particle_types.len()].clone(),
            charge: if i % 2 == 0 { 1 } else { -1 },
            pt_mev: 5_000 + i * 3_000,
            eta_micro: ((i as i32) - 10) * 200_000,
            phi_micro: (i * 300_000) as i32,
            hits: ((i * 5)..(i * 5 + 5)).collect(),
        })
        .collect();
    let hits: Vec<DetectorHit> = (0..100)
        .map(|i| DetectorHit {
            hit_id: i,
            subsystem: subsystems[(i as usize) % subsystems.len()].clone(),
            layer: (i % 6 + 1) as u8,
            x_um: ((i as i32) - 50) * 400,
            y_um: ((i as i32) - 50) * 400,
            z_um: (i as i32) * 1000,
            energy_mev: 100 + i * 15,
            time_ps: (i as u64) * 50_000,
        })
        .collect();
    let val = CollisionEvent {
        event_id: 999_999,
        run_number: 500_000,
        lumi_block: 200,
        tracks,
        hits,
        beam_energy_gev: 6500,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode large event 20 tracks 100 hits");
    let (decoded, _): (CollisionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large event 20 tracks 100 hits");
    assert_eq!(val, decoded);
    assert_eq!(decoded.tracks.len(), 20);
    assert_eq!(decoded.hits.len(), 100);
}

// --- inner tracker precision hits ---

#[test]
fn test_inner_tracker_precision_hits() {
    let cfg = config::standard();
    let precision_hits: Vec<DetectorHit> = vec![
        DetectorHit {
            hit_id: 5000,
            subsystem: DetectorSubsystem::InnerTracker,
            layer: 1,
            x_um: 123,
            y_um: -456,
            z_um: 7890,
            energy_mev: 30,
            time_ps: 50_000,
        },
        DetectorHit {
            hit_id: 5001,
            subsystem: DetectorSubsystem::InnerTracker,
            layer: 2,
            x_um: 124,
            y_um: -457,
            z_um: 8012,
            energy_mev: 28,
            time_ps: 50_100,
        },
        DetectorHit {
            hit_id: 5002,
            subsystem: DetectorSubsystem::InnerTracker,
            layer: 3,
            x_um: 126,
            y_um: -459,
            z_um: 8200,
            energy_mev: 31,
            time_ps: 50_250,
        },
        DetectorHit {
            hit_id: 5003,
            subsystem: DetectorSubsystem::InnerTracker,
            layer: 4,
            x_um: 129,
            y_um: -462,
            z_um: 8450,
            energy_mev: 29,
            time_ps: 50_400,
        },
    ];
    let bytes = encode_to_vec(&precision_hits, cfg).expect("encode inner tracker precision hits");
    let (decoded, consumed): (Vec<DetectorHit>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode inner tracker precision hits");
    assert_eq!(precision_hits, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(decoded
        .iter()
        .all(|h| h.subsystem == DetectorSubsystem::InnerTracker));
}

// --- hadronic jet reconstruction ---

#[test]
fn test_hadronic_jet_reconstruction() {
    let cfg = config::standard();
    let jet_tracks: Vec<ParticleTrack> = vec![
        ParticleTrack {
            track_id: 701,
            particle_type: ParticleType::Pion,
            charge: 1,
            pt_mev: 12_000,
            eta_micro: 800_000,
            phi_micro: 1_000_000,
            hits: vec![710, 711, 712, 713],
        },
        ParticleTrack {
            track_id: 702,
            particle_type: ParticleType::Kaon,
            charge: -1,
            pt_mev: 9_500,
            eta_micro: 820_000,
            phi_micro: 1_010_000,
            hits: vec![720, 721, 722],
        },
        ParticleTrack {
            track_id: 703,
            particle_type: ParticleType::Proton,
            charge: 1,
            pt_mev: 18_000,
            eta_micro: 790_000,
            phi_micro: 990_000,
            hits: vec![730, 731, 732, 733, 734],
        },
        ParticleTrack {
            track_id: 704,
            particle_type: ParticleType::Neutron,
            charge: 0,
            pt_mev: 22_000,
            eta_micro: 810_000,
            phi_micro: 1_005_000,
            hits: vec![],
        },
    ];
    let hcal_hits: Vec<DetectorHit> = (0..6)
        .map(|i| DetectorHit {
            hit_id: 800 + i,
            subsystem: DetectorSubsystem::HadronCalorimeter,
            layer: (i + 1) as u8,
            x_um: 3000 + (i as i32) * 200,
            y_um: 3000 + (i as i32) * 200,
            z_um: 45_000 + (i as i32) * 2000,
            energy_mev: 3_000 + i * 500,
            time_ps: 4_000_000 + (i as u64) * 200_000,
        })
        .collect();
    let val = CollisionEvent {
        event_id: 77_777,
        run_number: 450_000,
        lumi_block: 55,
        tracks: jet_tracks,
        hits: hcal_hits,
        beam_energy_gev: 6500,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode hadronic jet event");
    let (decoded, consumed): (CollisionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode hadronic jet event");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
    let total_pt: u32 = decoded.tracks.iter().map(|t| t.pt_mev).sum();
    assert!(total_pt > 50_000, "jet total pT must exceed 50 GeV");
}
