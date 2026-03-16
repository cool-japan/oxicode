//! Advanced file I/O tests for the nuclear energy / reactor monitoring domain.
//!
//! Covers reactor cores, fuel rods, neutron flux, coolant systems, control rods,
//! radiation monitoring, safety interlocks, power output, and temperature gradients.

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
use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ============================================================
// Domain types
// ============================================================

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReactorState {
    Shutdown,
    StartingUp,
    FullPower,
    ReducedPower,
    Scram,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CoolantPhase {
    Liquid,
    TwoPhase,
    Steam,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuelRod {
    rod_id: u32,
    enrichment_percent: f64,
    burnup_mwd_per_ton: f64,
    cladding_temperature_k: f64,
    is_failed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ControlRod {
    rod_id: u32,
    insertion_depth_percent: f64,
    worth_pcm: f64,
    is_stuck: bool,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NeutronFluxReading {
    detector_id: u16,
    flux_n_per_cm2_s: f64,
    fast_flux: f64,
    thermal_flux: f64,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoolantLoop {
    loop_id: u8,
    phase: CoolantPhase,
    inlet_temperature_k: f64,
    outlet_temperature_k: f64,
    flow_rate_kg_per_s: f64,
    pressure_mpa: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReactorCore {
    reactor_id: String,
    state: ReactorState,
    thermal_power_mw: f64,
    electrical_power_mw: f64,
    fuel_rods: Vec<FuelRod>,
    control_rods: Vec<ControlRod>,
    coolant_loops: Vec<CoolantLoop>,
    neutron_flux: Option<NeutronFluxReading>,
    safety_interlock_active: bool,
    uptime_hours: u64,
}

// ============================================================
// Helper constructors
// ============================================================

fn sample_fuel_rod(id: u32) -> FuelRod {
    FuelRod {
        rod_id: id,
        enrichment_percent: 3.5 + (id as f64) * 0.1,
        burnup_mwd_per_ton: 10_000.0 + (id as f64) * 500.0,
        cladding_temperature_k: 620.0 + (id as f64) * 2.0,
        is_failed: false,
    }
}

fn sample_control_rod(id: u32) -> ControlRod {
    ControlRod {
        rod_id: id,
        insertion_depth_percent: 20.0 + (id as f64) * 5.0,
        worth_pcm: 300.0 - (id as f64) * 10.0,
        is_stuck: false,
        label: format!("CR-{:03}", id),
    }
}

fn sample_coolant_loop(id: u8) -> CoolantLoop {
    CoolantLoop {
        loop_id: id,
        phase: CoolantPhase::Liquid,
        inlet_temperature_k: 565.0,
        outlet_temperature_k: 600.0,
        flow_rate_kg_per_s: 4_800.0,
        pressure_mpa: 15.5,
    }
}

fn sample_neutron_flux() -> NeutronFluxReading {
    NeutronFluxReading {
        detector_id: 7,
        flux_n_per_cm2_s: 3.2e13,
        fast_flux: 1.8e13,
        thermal_flux: 1.4e13,
        timestamp_ms: 1_700_000_000_000,
    }
}

fn sample_reactor_core() -> ReactorCore {
    ReactorCore {
        reactor_id: "UNIT-1".to_string(),
        state: ReactorState::FullPower,
        thermal_power_mw: 3_411.0,
        electrical_power_mw: 1_150.0,
        fuel_rods: (0..4).map(sample_fuel_rod).collect(),
        control_rods: (0..3).map(sample_control_rod).collect(),
        coolant_loops: (0..2).map(sample_coolant_loop).collect(),
        neutron_flux: Some(sample_neutron_flux()),
        safety_interlock_active: false,
        uptime_hours: 8_760,
    }
}

// ============================================================
// Tests
// ============================================================

#[test]
fn test_fuel_rod_basic_roundtrip() {
    let original = sample_fuel_rod(1);
    let encoded = encode_to_vec(&original).expect("encode FuelRod");
    let (decoded, _): (FuelRod, usize) = decode_from_slice(&encoded).expect("decode FuelRod");
    assert_eq!(original, decoded);
}

#[test]
fn test_control_rod_basic_roundtrip() {
    let original = sample_control_rod(5);
    let encoded = encode_to_vec(&original).expect("encode ControlRod");
    let (decoded, _): (ControlRod, usize) = decode_from_slice(&encoded).expect("decode ControlRod");
    assert_eq!(original, decoded);
}

#[test]
fn test_neutron_flux_basic_roundtrip() {
    let original = sample_neutron_flux();
    let encoded = encode_to_vec(&original).expect("encode NeutronFluxReading");
    let (decoded, _): (NeutronFluxReading, usize) =
        decode_from_slice(&encoded).expect("decode NeutronFluxReading");
    assert_eq!(original, decoded);
}

#[test]
fn test_coolant_loop_basic_roundtrip() {
    let original = sample_coolant_loop(0);
    let encoded = encode_to_vec(&original).expect("encode CoolantLoop");
    let (decoded, _): (CoolantLoop, usize) =
        decode_from_slice(&encoded).expect("decode CoolantLoop");
    assert_eq!(original, decoded);
}

#[test]
fn test_reactor_core_nested_roundtrip() {
    let original = sample_reactor_core();
    let encoded = encode_to_vec(&original).expect("encode ReactorCore");
    let (decoded, _): (ReactorCore, usize) =
        decode_from_slice(&encoded).expect("decode ReactorCore");
    assert_eq!(original, decoded);
}

#[test]
fn test_reactor_state_enum_all_variants() {
    let variants = [
        ReactorState::Shutdown,
        ReactorState::StartingUp,
        ReactorState::FullPower,
        ReactorState::ReducedPower,
        ReactorState::Scram,
    ];
    for state in &variants {
        let encoded = encode_to_vec(state).expect("encode ReactorState variant");
        let (decoded, _): (ReactorState, usize) =
            decode_from_slice(&encoded).expect("decode ReactorState variant");
        assert_eq!(state, &decoded);
    }
}

#[test]
fn test_coolant_phase_enum_roundtrip() {
    let phases = [
        CoolantPhase::Liquid,
        CoolantPhase::TwoPhase,
        CoolantPhase::Steam,
    ];
    for phase in &phases {
        let encoded = encode_to_vec(phase).expect("encode CoolantPhase");
        let (decoded, _): (CoolantPhase, usize) =
            decode_from_slice(&encoded).expect("decode CoolantPhase");
        assert_eq!(phase, &decoded);
    }
}

#[test]
fn test_reactor_core_file_io_roundtrip() {
    let original = sample_reactor_core();
    let path = temp_dir().join("oxicode_nuclear_reactor_core_26.bin");
    encode_to_file(&original, &path).expect("encode_to_file ReactorCore");
    let decoded: ReactorCore = decode_from_file(&path).expect("decode_from_file ReactorCore");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_neutron_flux_file_io_roundtrip() {
    let original = sample_neutron_flux();
    let path = temp_dir().join("oxicode_nuclear_neutron_flux_26.bin");
    encode_to_file(&original, &path).expect("encode_to_file NeutronFluxReading");
    let decoded: NeutronFluxReading =
        decode_from_file(&path).expect("decode_from_file NeutronFluxReading");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_file_io_matches_encode_to_vec() {
    let original = sample_reactor_core();
    let path = temp_dir().join("oxicode_nuclear_file_vs_vec_26.bin");
    encode_to_file(&original, &path).expect("encode_to_file");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = encode_to_vec(&original).expect("encode_to_vec");
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must match encode_to_vec output"
    );
    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_big_endian_config_fuel_rod_roundtrip() {
    let original = sample_fuel_rod(3);
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode FuelRod big-endian");
    let (decoded, _): (FuelRod, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode FuelRod big-endian");
    assert_eq!(original, decoded);
}

#[test]
fn test_fixed_int_config_control_rod_roundtrip() {
    let original = sample_control_rod(2);
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode ControlRod fixed-int");
    let (decoded, _): (ControlRod, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode ControlRod fixed-int");
    assert_eq!(original, decoded);
}

#[test]
fn test_big_endian_fixed_int_reactor_core_roundtrip() {
    let original = sample_reactor_core();
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode ReactorCore big-endian+fixed");
    let (decoded, _): (ReactorCore, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode ReactorCore big-endian+fixed");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_of_fuel_rods_roundtrip() {
    let original: Vec<FuelRod> = (0..10).map(sample_fuel_rod).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<FuelRod>");
    let (decoded, _): (Vec<FuelRod>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<FuelRod>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_of_coolant_loops_roundtrip() {
    let original: Vec<CoolantLoop> = (0..4).map(sample_coolant_loop).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<CoolantLoop>");
    let (decoded, _): (Vec<CoolantLoop>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<CoolantLoop>");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_neutron_flux_some_roundtrip() {
    let original: Option<NeutronFluxReading> = Some(sample_neutron_flux());
    let encoded = encode_to_vec(&original).expect("encode Some(NeutronFluxReading)");
    let (decoded, _): (Option<NeutronFluxReading>, usize) =
        decode_from_slice(&encoded).expect("decode Some(NeutronFluxReading)");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_neutron_flux_none_roundtrip() {
    let original: Option<NeutronFluxReading> = None;
    let encoded = encode_to_vec(&original).expect("encode None<NeutronFluxReading>");
    let (decoded, _): (Option<NeutronFluxReading>, usize) =
        decode_from_slice(&encoded).expect("decode None<NeutronFluxReading>");
    assert_eq!(original, decoded);
}

#[test]
fn test_reactor_core_with_no_neutron_flux_roundtrip() {
    let mut core = sample_reactor_core();
    core.neutron_flux = None;
    core.state = ReactorState::Shutdown;
    core.safety_interlock_active = true;
    let encoded = encode_to_vec(&core).expect("encode ReactorCore without flux");
    let (decoded, _): (ReactorCore, usize) =
        decode_from_slice(&encoded).expect("decode ReactorCore without flux");
    assert_eq!(core, decoded);
}

#[test]
fn test_bytes_consumed_matches_encoded_length() {
    let original = sample_reactor_core();
    let encoded = encode_to_vec(&original).expect("encode ReactorCore");
    let (_, consumed): (ReactorCore, usize) =
        decode_from_slice(&encoded).expect("decode ReactorCore");
    assert_eq!(
        consumed,
        encoded.len(),
        "bytes consumed must equal total encoded length"
    );
}

#[test]
fn test_large_collection_fuel_rods_roundtrip() {
    let original: Vec<FuelRod> = (0..500).map(sample_fuel_rod).collect();
    let encoded = encode_to_vec(&original).expect("encode large Vec<FuelRod>");
    let (decoded, bytes_consumed): (Vec<FuelRod>, usize) =
        decode_from_slice(&encoded).expect("decode large Vec<FuelRod>");
    assert_eq!(original.len(), decoded.len());
    assert_eq!(original, decoded);
    assert_eq!(bytes_consumed, encoded.len());
}

#[test]
fn test_large_collection_file_io_roundtrip() {
    let rods: Vec<FuelRod> = (0..200).map(sample_fuel_rod).collect();
    let path = temp_dir().join("oxicode_nuclear_large_rods_26.bin");
    encode_to_file(&rods, &path).expect("encode_to_file large Vec<FuelRod>");
    let decoded: Vec<FuelRod> =
        decode_from_file(&path).expect("decode_from_file large Vec<FuelRod>");
    assert_eq!(rods.len(), decoded.len());
    assert_eq!(rods, decoded);
    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_scram_state_safety_interlock_roundtrip() {
    let mut core = sample_reactor_core();
    core.state = ReactorState::Scram;
    core.safety_interlock_active = true;
    core.thermal_power_mw = 0.0;
    core.electrical_power_mw = 0.0;
    // Insert all control rods fully
    for cr in core.control_rods.iter_mut() {
        cr.insertion_depth_percent = 100.0;
    }
    let encoded = encode_to_vec(&core).expect("encode SCRAM ReactorCore");
    let (decoded, _): (ReactorCore, usize) =
        decode_from_slice(&encoded).expect("decode SCRAM ReactorCore");
    assert_eq!(core, decoded);
    assert_eq!(decoded.state, ReactorState::Scram);
    assert!(decoded.safety_interlock_active);
    for cr in &decoded.control_rods {
        assert_eq!(cr.insertion_depth_percent, 100.0);
    }
}

#[test]
fn test_two_phase_coolant_with_steam_transition_roundtrip() {
    let mut loop1 = sample_coolant_loop(1);
    loop1.phase = CoolantPhase::TwoPhase;
    loop1.outlet_temperature_k = 647.0; // near critical point
    loop1.pressure_mpa = 22.06; // critical pressure of water

    let mut loop2 = sample_coolant_loop(2);
    loop2.phase = CoolantPhase::Steam;
    loop2.outlet_temperature_k = 820.0;
    loop2.pressure_mpa = 6.0;

    let loops = vec![loop1, loop2];
    let encoded = encode_to_vec(&loops).expect("encode two-phase loops");
    let (decoded, bytes_consumed): (Vec<CoolantLoop>, usize) =
        decode_from_slice(&encoded).expect("decode two-phase loops");
    assert_eq!(loops, decoded);
    assert_eq!(bytes_consumed, encoded.len());
    assert_eq!(decoded[0].phase, CoolantPhase::TwoPhase);
    assert_eq!(decoded[1].phase, CoolantPhase::Steam);
}
