//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Theme: Nuclear fusion reactor diagnostics and plasma control systems.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced33_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Shared domain types — Tokamak / Stellarator fusion diagnostics
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlasmaParameters {
    electron_temperature_kev: f64,
    ion_temperature_kev: f64,
    electron_density_per_m3: f64,
    energy_confinement_time_s: f64,
    triple_product: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MagneticFieldCoil {
    coil_id: u32,
    current_ka: f64,
    field_strength_tesla: f64,
    temperature_kelvin: f64,
    is_superconducting: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NeutralBeamInjector {
    injector_id: u16,
    beam_energy_kev: f64,
    power_mw: f64,
    species: String,
    pulse_duration_s: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DivertorHeatFlux {
    tile_row: u32,
    tile_column: u32,
    heat_flux_mw_per_m2: f64,
    surface_temperature_c: f64,
    erosion_rate_nm_per_s: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PlasmaInstability {
    Stable,
    EdgeLocalizedMode {
        frequency_hz: f64,
        amplitude: f64,
    },
    NeoTearingMode {
        island_width_cm: f64,
        mode_numbers: (u32, u32),
    },
    VerticalDisplacementEvent {
        growth_rate_per_s: f64,
    },
    Disruption {
        thermal_quench_ms: f64,
        current_quench_ms: f64,
    },
    Sawtooth {
        period_ms: f64,
        inversion_radius_m: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TritiumBreeding {
    blanket_module_id: u32,
    tritium_breeding_ratio: f64,
    lithium6_enrichment_pct: f64,
    neutron_multiplier: String,
    annual_production_grams: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DisruptionPrediction {
    timestamp_ms: u64,
    locked_mode_amplitude_gauss: f64,
    radiated_power_fraction: f64,
    density_limit_fraction: f64,
    disruption_probability: f64,
    recommended_action: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FirstWallPanel {
    panel_id: u32,
    material: String,
    thickness_mm: f64,
    neutron_fluence_dpa: f64,
    helium_concentration_appm: f64,
    remaining_lifetime_years: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CryogenicCoolantState {
    SupercriticalHelium {
        temperature_k: f64,
        pressure_bar: f64,
    },
    LiquidNitrogen {
        level_pct: f64,
        flow_rate_liters_per_s: f64,
    },
    LiquidHelium {
        temperature_k: f64,
        bath_level_pct: f64,
    },
    WarmUp {
        current_temp_k: f64,
        target_temp_k: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StellaratorCoilGeometry {
    coil_family: String,
    num_periods: u32,
    major_radius_m: f64,
    minor_radius_m: f64,
    rotational_transform: f64,
    fourier_coefficients: Vec<f64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FusionPowerGain {
    fusion_power_mw: f64,
    auxiliary_heating_mw: f64,
    ohmic_heating_mw: f64,
    q_factor: f64,
    is_ignition: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PelletInjection {
    pellet_id: u64,
    diameter_mm: f64,
    velocity_m_per_s: f64,
    composition: String,
    injection_angle_deg: f64,
    fueling_efficiency_pct: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RadioFrequencyHeating {
    system_name: String,
    frequency_ghz: f64,
    power_mw: f64,
    harmonic_number: u32,
    absorption_efficiency_pct: f64,
    current_drive_ka: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VacuumVesselStatus {
    sector_id: u32,
    base_pressure_pa: f64,
    leak_rate_pa_m3_per_s: f64,
    bakeout_temperature_c: f64,
    is_under_vacuum: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlasmaShapeDescriptor {
    major_radius_m: f64,
    minor_radius_m: f64,
    elongation: f64,
    triangularity_upper: f64,
    triangularity_lower: f64,
    plasma_current_ma: f64,
    beta_normalized: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FuelCyclePhase {
    GasInjection {
        flow_rate_torr_l_per_s: f64,
    },
    PelletFueling {
        rate_hz: f64,
    },
    NeutralBeamFueling,
    Exhaust {
        pumping_speed_l_per_s: f64,
        tritium_recovery_pct: f64,
    },
    Idle,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlasmaCurrentProfile {
    total_current_ma: f64,
    bootstrap_fraction: f64,
    ohmic_fraction: f64,
    beam_driven_fraction: f64,
    rf_driven_fraction: f64,
    safety_factor_q95: f64,
    safety_factor_q0: f64,
}

// ---------------------------------------------------------------------------
// Test 1: Tokamak plasma parameters with Lawson criterion values
// ---------------------------------------------------------------------------
#[test]
fn test_plasma_parameters_lawson_criterion() {
    let plasma = PlasmaParameters {
        electron_temperature_kev: 25.0,
        ion_temperature_kev: 22.0,
        electron_density_per_m3: 1.0e20,
        energy_confinement_time_s: 3.7,
        triple_product: 6.6e21,
    };
    let bytes = encode_with_checksum(&plasma).expect("encode plasma parameters");
    let (decoded, _): (PlasmaParameters, _) =
        decode_with_checksum(&bytes).expect("decode plasma parameters");
    assert_eq!(decoded, plasma, "plasma parameters must roundtrip exactly");
}

// ---------------------------------------------------------------------------
// Test 2: Superconducting toroidal field coil
// ---------------------------------------------------------------------------
#[test]
fn test_toroidal_field_coil() {
    let coil = MagneticFieldCoil {
        coil_id: 7,
        current_ka: 68.0,
        field_strength_tesla: 11.8,
        temperature_kelvin: 4.5,
        is_superconducting: true,
    };
    let bytes = encode_with_checksum(&coil).expect("encode TF coil");
    let (decoded, consumed): (MagneticFieldCoil, _) =
        decode_with_checksum(&bytes).expect("decode TF coil");
    assert_eq!(decoded, coil);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 3: Poloidal field coil set (Vec)
// ---------------------------------------------------------------------------
#[test]
fn test_poloidal_field_coil_set() {
    let coils: Vec<MagneticFieldCoil> = (1..=6)
        .map(|i| MagneticFieldCoil {
            coil_id: i,
            current_ka: 15.0 + (i as f64) * 2.5,
            field_strength_tesla: 4.0 + (i as f64) * 0.3,
            temperature_kelvin: 4.2,
            is_superconducting: true,
        })
        .collect();
    let bytes = encode_with_checksum(&coils).expect("encode PF coil set");
    let (decoded, _): (Vec<MagneticFieldCoil>, _) =
        decode_with_checksum(&bytes).expect("decode PF coil set");
    assert_eq!(decoded.len(), 6);
    assert_eq!(decoded, coils);
}

// ---------------------------------------------------------------------------
// Test 4: Neutral beam injector — deuterium
// ---------------------------------------------------------------------------
#[test]
fn test_neutral_beam_injector_deuterium() {
    let nbi = NeutralBeamInjector {
        injector_id: 1,
        beam_energy_kev: 1000.0,
        power_mw: 16.5,
        species: "Deuterium".to_string(),
        pulse_duration_s: 400.0,
    };
    let bytes = encode_with_checksum(&nbi).expect("encode NBI");
    let (decoded, _): (NeutralBeamInjector, _) = decode_with_checksum(&bytes).expect("decode NBI");
    assert_eq!(decoded, nbi);
}

// ---------------------------------------------------------------------------
// Test 5: Divertor heat flux map row
// ---------------------------------------------------------------------------
#[test]
fn test_divertor_heat_flux_tiles() {
    let tiles: Vec<DivertorHeatFlux> = (0..8)
        .map(|col| DivertorHeatFlux {
            tile_row: 3,
            tile_column: col,
            heat_flux_mw_per_m2: 10.0 + (col as f64) * 1.2,
            surface_temperature_c: 800.0 + (col as f64) * 50.0,
            erosion_rate_nm_per_s: 0.05 + (col as f64) * 0.01,
        })
        .collect();
    let bytes = encode_with_checksum(&tiles).expect("encode divertor tiles");
    let (decoded, _): (Vec<DivertorHeatFlux>, _) =
        decode_with_checksum(&bytes).expect("decode divertor tiles");
    assert_eq!(decoded.len(), 8);
    assert_eq!(decoded, tiles);
}

// ---------------------------------------------------------------------------
// Test 6: Plasma instability — stable state
// ---------------------------------------------------------------------------
#[test]
fn test_plasma_instability_stable() {
    let state = PlasmaInstability::Stable;
    let bytes = encode_with_checksum(&state).expect("encode stable instability");
    let (decoded, _): (PlasmaInstability, _) =
        decode_with_checksum(&bytes).expect("decode stable instability");
    assert_eq!(decoded, state);
}

// ---------------------------------------------------------------------------
// Test 7: Plasma instability — ELM classification
// ---------------------------------------------------------------------------
#[test]
fn test_plasma_instability_elm() {
    let elm = PlasmaInstability::EdgeLocalizedMode {
        frequency_hz: 40.0,
        amplitude: 0.15,
    };
    let bytes = encode_with_checksum(&elm).expect("encode ELM instability");
    let (decoded, _): (PlasmaInstability, _) =
        decode_with_checksum(&bytes).expect("decode ELM instability");
    assert_eq!(decoded, elm);
}

// ---------------------------------------------------------------------------
// Test 8: Plasma instability — neoclassical tearing mode
// ---------------------------------------------------------------------------
#[test]
fn test_plasma_instability_ntm() {
    let ntm = PlasmaInstability::NeoTearingMode {
        island_width_cm: 8.5,
        mode_numbers: (3, 2),
    };
    let bytes = encode_with_checksum(&ntm).expect("encode NTM instability");
    let (decoded, _): (PlasmaInstability, _) =
        decode_with_checksum(&bytes).expect("decode NTM instability");
    assert_eq!(decoded, ntm);
}

// ---------------------------------------------------------------------------
// Test 9: Disruption prediction telemetry
// ---------------------------------------------------------------------------
#[test]
fn test_disruption_prediction_high_risk() {
    let prediction = DisruptionPrediction {
        timestamp_ms: 1_750_000_000_000,
        locked_mode_amplitude_gauss: 45.0,
        radiated_power_fraction: 0.72,
        density_limit_fraction: 0.91,
        disruption_probability: 0.87,
        recommended_action: "Initiate controlled rampdown".to_string(),
    };
    let bytes = encode_with_checksum(&prediction).expect("encode disruption prediction");
    let (decoded, _): (DisruptionPrediction, _) =
        decode_with_checksum(&bytes).expect("decode disruption prediction");
    assert_eq!(decoded, prediction);
}

// ---------------------------------------------------------------------------
// Test 10: Tritium breeding blanket module
// ---------------------------------------------------------------------------
#[test]
fn test_tritium_breeding_blanket() {
    let blanket = TritiumBreeding {
        blanket_module_id: 42,
        tritium_breeding_ratio: 1.15,
        lithium6_enrichment_pct: 60.0,
        neutron_multiplier: "Beryllium".to_string(),
        annual_production_grams: 55.8,
    };
    let bytes = encode_with_checksum(&blanket).expect("encode tritium breeding");
    let (decoded, _): (TritiumBreeding, _) =
        decode_with_checksum(&bytes).expect("decode tritium breeding");
    assert_eq!(decoded, blanket);
}

// ---------------------------------------------------------------------------
// Test 11: First wall tungsten panel under high fluence
// ---------------------------------------------------------------------------
#[test]
fn test_first_wall_tungsten_panel() {
    let panel = FirstWallPanel {
        panel_id: 129,
        material: "Tungsten W".to_string(),
        thickness_mm: 10.0,
        neutron_fluence_dpa: 14.5,
        helium_concentration_appm: 600.0,
        remaining_lifetime_years: 2.3,
    };
    let bytes = encode_with_checksum(&panel).expect("encode first wall panel");
    let (decoded, _): (FirstWallPanel, _) =
        decode_with_checksum(&bytes).expect("decode first wall panel");
    assert_eq!(decoded, panel);
}

// ---------------------------------------------------------------------------
// Test 12: Cryogenic coolant — supercritical helium loop
// ---------------------------------------------------------------------------
#[test]
fn test_cryogenic_supercritical_helium() {
    let state = CryogenicCoolantState::SupercriticalHelium {
        temperature_k: 4.5,
        pressure_bar: 3.0,
    };
    let bytes = encode_with_checksum(&state).expect("encode cryo SHe");
    let (decoded, _): (CryogenicCoolantState, _) =
        decode_with_checksum(&bytes).expect("decode cryo SHe");
    assert_eq!(decoded, state);
}

// ---------------------------------------------------------------------------
// Test 13: Cryogenic coolant — liquid nitrogen pre-cool
// ---------------------------------------------------------------------------
#[test]
fn test_cryogenic_liquid_nitrogen() {
    let state = CryogenicCoolantState::LiquidNitrogen {
        level_pct: 85.3,
        flow_rate_liters_per_s: 12.0,
    };
    let bytes = encode_with_checksum(&state).expect("encode cryo LN2");
    let (decoded, _): (CryogenicCoolantState, _) =
        decode_with_checksum(&bytes).expect("decode cryo LN2");
    assert_eq!(decoded, state);
}

// ---------------------------------------------------------------------------
// Test 14: Stellarator coil geometry with Fourier harmonics
// ---------------------------------------------------------------------------
#[test]
fn test_stellarator_coil_geometry() {
    let coil = StellaratorCoilGeometry {
        coil_family: "Modular non-planar".to_string(),
        num_periods: 5,
        major_radius_m: 5.5,
        minor_radius_m: 0.53,
        rotational_transform: 0.87,
        fourier_coefficients: vec![1.0, 0.042, -0.018, 0.0073, -0.0031, 0.0015, -0.00068],
    };
    let bytes = encode_with_checksum(&coil).expect("encode stellarator coil");
    let (decoded, _): (StellaratorCoilGeometry, _) =
        decode_with_checksum(&bytes).expect("decode stellarator coil");
    assert_eq!(decoded, coil);
}

// ---------------------------------------------------------------------------
// Test 15: Fusion power gain — Q > 10 burning plasma
// ---------------------------------------------------------------------------
#[test]
fn test_fusion_power_gain_burning_plasma() {
    let gain = FusionPowerGain {
        fusion_power_mw: 500.0,
        auxiliary_heating_mw: 40.0,
        ohmic_heating_mw: 1.2,
        q_factor: 12.14,
        is_ignition: false,
    };
    let bytes = encode_with_checksum(&gain).expect("encode Q factor");
    let (decoded, _): (FusionPowerGain, _) = decode_with_checksum(&bytes).expect("decode Q factor");
    assert_eq!(decoded, gain);
    assert!(!decoded.is_ignition);
}

// ---------------------------------------------------------------------------
// Test 16: Fusion power gain — ignition scenario
// ---------------------------------------------------------------------------
#[test]
fn test_fusion_power_gain_ignition() {
    let gain = FusionPowerGain {
        fusion_power_mw: 2000.0,
        auxiliary_heating_mw: 0.0,
        ohmic_heating_mw: 0.5,
        q_factor: f64::INFINITY,
        is_ignition: true,
    };
    let bytes = encode_with_checksum(&gain).expect("encode ignition Q");
    let (decoded, _): (FusionPowerGain, _) =
        decode_with_checksum(&bytes).expect("decode ignition Q");
    assert_eq!(decoded, gain);
    assert!(decoded.is_ignition);
    assert!(decoded.q_factor.is_infinite());
}

// ---------------------------------------------------------------------------
// Test 17: Pellet injection — high-field-side launch
// ---------------------------------------------------------------------------
#[test]
fn test_pellet_injection_hfs() {
    let pellet = PelletInjection {
        pellet_id: 9_500_001,
        diameter_mm: 3.0,
        velocity_m_per_s: 300.0,
        composition: "DT ice 50:50".to_string(),
        injection_angle_deg: 45.0,
        fueling_efficiency_pct: 92.0,
    };
    let bytes = encode_with_checksum(&pellet).expect("encode pellet injection");
    let (decoded, _): (PelletInjection, _) =
        decode_with_checksum(&bytes).expect("decode pellet injection");
    assert_eq!(decoded, pellet);
}

// ---------------------------------------------------------------------------
// Test 18: Radio-frequency ECRH heating system
// ---------------------------------------------------------------------------
#[test]
fn test_rf_heating_ecrh() {
    let ecrh = RadioFrequencyHeating {
        system_name: "ECRH upper launcher".to_string(),
        frequency_ghz: 170.0,
        power_mw: 20.0,
        harmonic_number: 2,
        absorption_efficiency_pct: 98.5,
        current_drive_ka: 120.0,
    };
    let bytes = encode_with_checksum(&ecrh).expect("encode ECRH");
    let (decoded, _): (RadioFrequencyHeating, _) =
        decode_with_checksum(&bytes).expect("decode ECRH");
    assert_eq!(decoded, ecrh);
}

// ---------------------------------------------------------------------------
// Test 19: Vacuum vessel integrity check
// ---------------------------------------------------------------------------
#[test]
fn test_vacuum_vessel_status() {
    let vessel = VacuumVesselStatus {
        sector_id: 5,
        base_pressure_pa: 1.0e-6,
        leak_rate_pa_m3_per_s: 1.0e-9,
        bakeout_temperature_c: 240.0,
        is_under_vacuum: true,
    };
    let bytes = encode_with_checksum(&vessel).expect("encode vacuum vessel");
    let (decoded, _): (VacuumVesselStatus, _) =
        decode_with_checksum(&bytes).expect("decode vacuum vessel");
    assert_eq!(decoded, vessel);
    assert!(decoded.is_under_vacuum);
}

// ---------------------------------------------------------------------------
// Test 20: Plasma shape — ITER-like D-shaped cross-section
// ---------------------------------------------------------------------------
#[test]
fn test_plasma_shape_iter_like() {
    let shape = PlasmaShapeDescriptor {
        major_radius_m: 6.2,
        minor_radius_m: 2.0,
        elongation: 1.85,
        triangularity_upper: 0.33,
        triangularity_lower: 0.49,
        plasma_current_ma: 15.0,
        beta_normalized: 1.8,
    };
    let bytes = encode_with_checksum(&shape).expect("encode plasma shape");
    let (decoded, _): (PlasmaShapeDescriptor, _) =
        decode_with_checksum(&bytes).expect("decode plasma shape");
    assert_eq!(decoded, shape);
}

// ---------------------------------------------------------------------------
// Test 21: Fuel cycle — tritium exhaust recovery
// ---------------------------------------------------------------------------
#[test]
fn test_fuel_cycle_exhaust_recovery() {
    let phase = FuelCyclePhase::Exhaust {
        pumping_speed_l_per_s: 50_000.0,
        tritium_recovery_pct: 99.5,
    };
    let bytes = encode_with_checksum(&phase).expect("encode fuel cycle exhaust");
    let (decoded, _): (FuelCyclePhase, _) =
        decode_with_checksum(&bytes).expect("decode fuel cycle exhaust");
    assert_eq!(decoded, phase);
}

// ---------------------------------------------------------------------------
// Test 22: Plasma current profile with bootstrap current
// ---------------------------------------------------------------------------
#[test]
fn test_plasma_current_profile() {
    let profile = PlasmaCurrentProfile {
        total_current_ma: 15.0,
        bootstrap_fraction: 0.35,
        ohmic_fraction: 0.10,
        beam_driven_fraction: 0.40,
        rf_driven_fraction: 0.15,
        safety_factor_q95: 3.0,
        safety_factor_q0: 1.05,
    };
    let bytes = encode_with_checksum(&profile).expect("encode current profile");
    let (decoded, _): (PlasmaCurrentProfile, _) =
        decode_with_checksum(&bytes).expect("decode current profile");
    assert_eq!(decoded, profile);

    // Verify fractions sum close to 1.0
    let fraction_sum = decoded.bootstrap_fraction
        + decoded.ohmic_fraction
        + decoded.beam_driven_fraction
        + decoded.rf_driven_fraction;
    assert!(
        (fraction_sum - 1.0).abs() < 1.0e-10,
        "current drive fractions must sum to 1.0, got {}",
        fraction_sum
    );
}
