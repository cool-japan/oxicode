//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Theme: geothermal energy exploration and power generation.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced37_test

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
// Shared helper types — geothermal domain
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum WellType {
    Production,
    Reinjection,
    Observation,
    MakeUp,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FluidPhase {
    Liquid,
    Steam,
    TwoPhase { steam_fraction: u8 },
    Supercritical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ValveState {
    FullyOpen,
    PartiallyClosed(u8),
    FullyClosed,
    Throttling { target_pressure_kpa: u32 },
    EmergencyShutoff,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SeismicSeverity {
    Negligible,
    Minor,
    Moderate,
    Significant,
    Critical,
}

// ---------------------------------------------------------------------------
// Test 1: Well logging temperature gradient
// ---------------------------------------------------------------------------
#[test]
fn test_well_logging_temperature_gradient() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TemperatureGradient {
        well_id: String,
        depth_m: f64,
        temperature_c: f64,
        gradient_c_per_km: f64,
        measurement_epoch_s: u64,
    }

    let val = TemperatureGradient {
        well_id: "GEO-TG-001".to_string(),
        depth_m: 2450.0,
        temperature_c: 287.3,
        gradient_c_per_km: 82.5,
        measurement_epoch_s: 1_710_000_000,
    };
    let bytes = encode_with_checksum(&val).expect("encode temperature gradient");
    let (decoded, consumed): (TemperatureGradient, _) =
        decode_with_checksum(&bytes).expect("decode temperature gradient");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 2: Well logging pressure gradient
// ---------------------------------------------------------------------------
#[test]
fn test_well_logging_pressure_gradient() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PressureGradient {
        well_id: String,
        depth_m: f64,
        pressure_mpa: f64,
        fluid_density_kg_m3: f64,
        hydrostatic_head_mpa: f64,
        is_overpressured: bool,
    }

    let val = PressureGradient {
        well_id: "GEO-PG-114".to_string(),
        depth_m: 3100.0,
        pressure_mpa: 28.6,
        fluid_density_kg_m3: 830.0,
        hydrostatic_head_mpa: 25.2,
        is_overpressured: true,
    };
    let bytes = encode_with_checksum(&val).expect("encode pressure gradient");
    let (decoded, consumed): (PressureGradient, _) =
        decode_with_checksum(&bytes).expect("decode pressure gradient");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 3: Reservoir characterization
// ---------------------------------------------------------------------------
#[test]
fn test_reservoir_characterization() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReservoirCharacterization {
        field_name: String,
        permeability_md: f64,
        porosity_pct: f64,
        reservoir_temperature_c: f64,
        reservoir_pressure_mpa: f64,
        rock_type: String,
        estimated_volume_km3: f64,
        thermal_conductivity_w_mk: f64,
    }

    let val = ReservoirCharacterization {
        field_name: "Hellisheidi Sector B".to_string(),
        permeability_md: 150.0,
        porosity_pct: 12.8,
        reservoir_temperature_c: 310.0,
        reservoir_pressure_mpa: 34.0,
        rock_type: "Fractured Basalt".to_string(),
        estimated_volume_km3: 4.2,
        thermal_conductivity_w_mk: 2.1,
    };
    let bytes = encode_with_checksum(&val).expect("encode reservoir characterization");
    let (decoded, consumed): (ReservoirCharacterization, _) =
        decode_with_checksum(&bytes).expect("decode reservoir characterization");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 4: Steam turbine performance
// ---------------------------------------------------------------------------
#[test]
fn test_steam_turbine_performance() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SteamTurbinePerformance {
        unit_id: String,
        inlet_pressure_mpa: f64,
        inlet_temperature_c: f64,
        exhaust_pressure_kpa: f64,
        mass_flow_kg_s: f64,
        power_output_mw: f64,
        isentropic_efficiency_pct: f64,
        rotor_speed_rpm: u32,
        vibration_mm_s: f64,
    }

    let val = SteamTurbinePerformance {
        unit_id: "STU-Alpha-03".to_string(),
        inlet_pressure_mpa: 0.85,
        inlet_temperature_c: 172.0,
        exhaust_pressure_kpa: 10.0,
        mass_flow_kg_s: 48.5,
        power_output_mw: 25.3,
        isentropic_efficiency_pct: 81.2,
        rotor_speed_rpm: 3000,
        vibration_mm_s: 2.1,
    };
    let bytes = encode_with_checksum(&val).expect("encode steam turbine performance");
    let (decoded, consumed): (SteamTurbinePerformance, _) =
        decode_with_checksum(&bytes).expect("decode steam turbine performance");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 5: Binary cycle ORC parameters
// ---------------------------------------------------------------------------
#[test]
fn test_binary_cycle_orc_parameters() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct OrcParameters {
        plant_id: String,
        working_fluid: String,
        brine_inlet_temp_c: f64,
        brine_outlet_temp_c: f64,
        turbine_inlet_pressure_mpa: f64,
        condenser_pressure_kpa: f64,
        net_power_kw: u32,
        thermal_efficiency_pct: f64,
        pump_power_kw: u32,
        cooling_type: String,
    }

    let val = OrcParameters {
        plant_id: "ORC-BIN-007".to_string(),
        working_fluid: "n-Pentane".to_string(),
        brine_inlet_temp_c: 165.0,
        brine_outlet_temp_c: 72.0,
        turbine_inlet_pressure_mpa: 1.4,
        condenser_pressure_kpa: 120.0,
        net_power_kw: 5800,
        thermal_efficiency_pct: 13.5,
        pump_power_kw: 380,
        cooling_type: "Air-cooled".to_string(),
    };
    let bytes = encode_with_checksum(&val).expect("encode ORC parameters");
    let (decoded, consumed): (OrcParameters, _) =
        decode_with_checksum(&bytes).expect("decode ORC parameters");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 6: Wellhead valve states
// ---------------------------------------------------------------------------
#[test]
fn test_wellhead_valve_states() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WellheadValveConfig {
        well_id: String,
        master_valve: ValveState,
        wing_valve: ValveState,
        choke_valve: ValveState,
        bleed_valve: ValveState,
        wellhead_pressure_kpa: u32,
        wellhead_temperature_c: f64,
    }

    let val = WellheadValveConfig {
        well_id: "GEO-WH-042".to_string(),
        master_valve: ValveState::FullyOpen,
        wing_valve: ValveState::Throttling {
            target_pressure_kpa: 850,
        },
        choke_valve: ValveState::PartiallyClosed(35),
        bleed_valve: ValveState::FullyClosed,
        wellhead_pressure_kpa: 1100,
        wellhead_temperature_c: 195.0,
    };
    let bytes = encode_with_checksum(&val).expect("encode wellhead valve config");
    let (decoded, consumed): (WellheadValveConfig, _) =
        decode_with_checksum(&bytes).expect("decode wellhead valve config");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 7: Scaling and corrosion monitoring
// ---------------------------------------------------------------------------
#[test]
fn test_scaling_corrosion_monitoring() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ScalingCorrosionRecord {
        pipe_segment_id: String,
        inspection_epoch_s: u64,
        silica_scale_mm: f64,
        calcite_scale_mm: f64,
        corrosion_rate_mm_yr: f64,
        wall_thickness_remaining_mm: f64,
        ph_at_sample_point: f64,
        inhibitor_dosage_ppm: f64,
        action_required: bool,
    }

    let val = ScalingCorrosionRecord {
        pipe_segment_id: "PIPE-SEC-R2-08".to_string(),
        inspection_epoch_s: 1_709_800_000,
        silica_scale_mm: 3.2,
        calcite_scale_mm: 0.8,
        corrosion_rate_mm_yr: 0.12,
        wall_thickness_remaining_mm: 7.6,
        ph_at_sample_point: 6.2,
        inhibitor_dosage_ppm: 45.0,
        action_required: false,
    };
    let bytes = encode_with_checksum(&val).expect("encode scaling corrosion record");
    let (decoded, consumed): (ScalingCorrosionRecord, _) =
        decode_with_checksum(&bytes).expect("decode scaling corrosion record");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 8: Induced seismicity record
// ---------------------------------------------------------------------------
#[test]
fn test_induced_seismicity_record() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct InducedSeismicityEvent {
        event_id: u64,
        latitude_deg: f64,
        longitude_deg: f64,
        depth_km: f64,
        local_magnitude: f64,
        severity: SeismicSeverity,
        duration_ms: u32,
        nearest_well_id: String,
        distance_to_well_m: f64,
        injection_rate_at_time_kg_s: f64,
        required_shutdown: bool,
    }

    let val = InducedSeismicityEvent {
        event_id: 928_374_001,
        latitude_deg: 64.0386,
        longitude_deg: -21.4012,
        depth_km: 3.8,
        local_magnitude: 1.7,
        severity: SeismicSeverity::Minor,
        duration_ms: 450,
        nearest_well_id: "RJ-009".to_string(),
        distance_to_well_m: 620.0,
        injection_rate_at_time_kg_s: 35.0,
        required_shutdown: false,
    };
    let bytes = encode_with_checksum(&val).expect("encode seismicity event");
    let (decoded, consumed): (InducedSeismicityEvent, _) =
        decode_with_checksum(&bytes).expect("decode seismicity event");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 9: Heat exchanger fouling
// ---------------------------------------------------------------------------
#[test]
fn test_heat_exchanger_fouling() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct HeatExchangerFouling {
        exchanger_id: String,
        exchanger_type: String,
        design_u_value_w_m2k: f64,
        current_u_value_w_m2k: f64,
        fouling_resistance_m2k_w: f64,
        pressure_drop_increase_pct: f64,
        days_since_cleaning: u32,
        recommended_cleaning: bool,
    }

    let val = HeatExchangerFouling {
        exchanger_id: "HX-PHE-011".to_string(),
        exchanger_type: "Plate".to_string(),
        design_u_value_w_m2k: 3200.0,
        current_u_value_w_m2k: 2480.0,
        fouling_resistance_m2k_w: 0.000_091,
        pressure_drop_increase_pct: 18.5,
        days_since_cleaning: 247,
        recommended_cleaning: true,
    };
    let bytes = encode_with_checksum(&val).expect("encode heat exchanger fouling");
    let (decoded, consumed): (HeatExchangerFouling, _) =
        decode_with_checksum(&bytes).expect("decode heat exchanger fouling");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 10: Reinjection well data
// ---------------------------------------------------------------------------
#[test]
fn test_reinjection_well_data() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ReinjectionWellData {
        well_id: String,
        well_type: WellType,
        injection_rate_kg_s: f64,
        fluid_temperature_c: f64,
        injection_pressure_mpa: f64,
        cumulative_injected_tonnes: f64,
        injectivity_index_kg_s_mpa: f64,
        tracer_concentration_ppb: f64,
        fluid_phase: FluidPhase,
    }

    let val = ReinjectionWellData {
        well_id: "RJ-015".to_string(),
        well_type: WellType::Reinjection,
        injection_rate_kg_s: 80.0,
        fluid_temperature_c: 70.0,
        injection_pressure_mpa: 3.2,
        cumulative_injected_tonnes: 15_200_000.0,
        injectivity_index_kg_s_mpa: 25.0,
        tracer_concentration_ppb: 0.003,
        fluid_phase: FluidPhase::Liquid,
    };
    let bytes = encode_with_checksum(&val).expect("encode reinjection well data");
    let (decoded, consumed): (ReinjectionWellData, _) =
        decode_with_checksum(&bytes).expect("decode reinjection well data");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 11: Geochemical fluid analysis
// ---------------------------------------------------------------------------
#[test]
fn test_geochemical_fluid_analysis() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct GeochemicalAnalysis {
        sample_id: String,
        well_id: String,
        ph: f64,
        silica_mg_l: f64,
        chloride_mg_l: f64,
        sodium_mg_l: f64,
        potassium_mg_l: f64,
        calcium_mg_l: f64,
        bicarbonate_mg_l: f64,
        sulfate_mg_l: f64,
        h2s_mg_l: f64,
        co2_mg_l: f64,
        total_dissolved_solids_mg_l: f64,
        geothermometer_temp_c: f64,
    }

    let val = GeochemicalAnalysis {
        sample_id: "GCA-2024-0831".to_string(),
        well_id: "PRD-021".to_string(),
        ph: 7.4,
        silica_mg_l: 620.0,
        chloride_mg_l: 18_500.0,
        sodium_mg_l: 10_200.0,
        potassium_mg_l: 1_850.0,
        calcium_mg_l: 420.0,
        bicarbonate_mg_l: 95.0,
        sulfate_mg_l: 28.0,
        h2s_mg_l: 3.2,
        co2_mg_l: 1_250.0,
        total_dissolved_solids_mg_l: 32_000.0,
        geothermometer_temp_c: 295.0,
    };
    let bytes = encode_with_checksum(&val).expect("encode geochemical analysis");
    let (decoded, consumed): (GeochemicalAnalysis, _) =
        decode_with_checksum(&bytes).expect("decode geochemical analysis");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 12: Drill string telemetry
// ---------------------------------------------------------------------------
#[test]
fn test_drill_string_telemetry() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DrillStringTelemetry {
        run_id: String,
        bit_depth_m: f64,
        weight_on_bit_kn: f64,
        torque_knm: f64,
        rotary_speed_rpm: u32,
        rate_of_penetration_m_hr: f64,
        mud_flow_rate_l_min: f64,
        standpipe_pressure_mpa: f64,
        annular_pressure_mpa: f64,
        downhole_temperature_c: f64,
        inclination_deg: f64,
        azimuth_deg: f64,
    }

    let val = DrillStringTelemetry {
        run_id: "DRILL-RUN-088".to_string(),
        bit_depth_m: 2780.0,
        weight_on_bit_kn: 120.0,
        torque_knm: 18.5,
        rotary_speed_rpm: 90,
        rate_of_penetration_m_hr: 3.2,
        mud_flow_rate_l_min: 2400.0,
        standpipe_pressure_mpa: 14.2,
        annular_pressure_mpa: 12.8,
        downhole_temperature_c: 265.0,
        inclination_deg: 5.2,
        azimuth_deg: 142.0,
    };
    let bytes = encode_with_checksum(&val).expect("encode drill string telemetry");
    let (decoded, consumed): (DrillStringTelemetry, _) =
        decode_with_checksum(&bytes).expect("decode drill string telemetry");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 13: Flash separator stage
// ---------------------------------------------------------------------------
#[test]
fn test_flash_separator_stage() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FlashSeparatorStage {
        stage_number: u8,
        separator_id: String,
        inlet_enthalpy_kj_kg: f64,
        flash_pressure_kpa: f64,
        flash_temperature_c: f64,
        steam_fraction_pct: f64,
        steam_flow_kg_s: f64,
        brine_flow_kg_s: f64,
        noncondensable_gas_pct: f64,
    }

    let val = FlashSeparatorStage {
        stage_number: 1,
        separator_id: "SEP-HP-02".to_string(),
        inlet_enthalpy_kj_kg: 1100.0,
        flash_pressure_kpa: 600.0,
        flash_temperature_c: 158.8,
        steam_fraction_pct: 22.4,
        steam_flow_kg_s: 42.0,
        brine_flow_kg_s: 145.0,
        noncondensable_gas_pct: 1.8,
    };
    let bytes = encode_with_checksum(&val).expect("encode flash separator stage");
    let (decoded, consumed): (FlashSeparatorStage, _) =
        decode_with_checksum(&bytes).expect("decode flash separator stage");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 14: Condenser performance
// ---------------------------------------------------------------------------
#[test]
fn test_condenser_performance() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CondenserPerformance {
        condenser_id: String,
        condenser_type: String,
        vacuum_pressure_kpa: f64,
        condensate_temperature_c: f64,
        cooling_water_inlet_c: f64,
        cooling_water_outlet_c: f64,
        cooling_water_flow_kg_s: f64,
        heat_rejection_mw: f64,
        air_leakage_detected: bool,
    }

    let val = CondenserPerformance {
        condenser_id: "COND-DC-01".to_string(),
        condenser_type: "Direct Contact".to_string(),
        vacuum_pressure_kpa: 8.5,
        condensate_temperature_c: 42.5,
        cooling_water_inlet_c: 22.0,
        cooling_water_outlet_c: 38.0,
        cooling_water_flow_kg_s: 520.0,
        heat_rejection_mw: 35.0,
        air_leakage_detected: false,
    };
    let bytes = encode_with_checksum(&val).expect("encode condenser performance");
    let (decoded, consumed): (CondenserPerformance, _) =
        decode_with_checksum(&bytes).expect("decode condenser performance");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 15: Cooling tower operation
// ---------------------------------------------------------------------------
#[test]
fn test_cooling_tower_operation() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CoolingTowerOperation {
        tower_id: String,
        num_cells: u8,
        hot_water_temp_c: f64,
        cold_water_temp_c: f64,
        wet_bulb_temp_c: f64,
        approach_c: f64,
        range_c: f64,
        water_flow_m3_hr: f64,
        fan_power_kw: u32,
        drift_loss_pct: f64,
        makeup_water_m3_hr: f64,
    }

    let val = CoolingTowerOperation {
        tower_id: "CT-04".to_string(),
        num_cells: 6,
        hot_water_temp_c: 40.5,
        cold_water_temp_c: 25.0,
        wet_bulb_temp_c: 18.0,
        approach_c: 7.0,
        range_c: 15.5,
        water_flow_m3_hr: 8500.0,
        fan_power_kw: 450,
        drift_loss_pct: 0.002,
        makeup_water_m3_hr: 120.0,
    };
    let bytes = encode_with_checksum(&val).expect("encode cooling tower operation");
    let (decoded, consumed): (CoolingTowerOperation, _) =
        decode_with_checksum(&bytes).expect("decode cooling tower operation");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 16: Magnetotelluric survey point
// ---------------------------------------------------------------------------
#[test]
fn test_magnetotelluric_survey_point() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MtSurveyPoint {
        station_id: String,
        latitude_deg: f64,
        longitude_deg: f64,
        elevation_m: f64,
        apparent_resistivity_ohm_m: Vec<f64>,
        phase_deg: Vec<f64>,
        frequency_hz: Vec<f64>,
        clay_cap_depth_m: f64,
        interpreted_temperature_c: f64,
    }

    let val = MtSurveyPoint {
        station_id: "MT-LINE3-018".to_string(),
        latitude_deg: -38.6543,
        longitude_deg: 176.1054,
        elevation_m: 340.0,
        apparent_resistivity_ohm_m: vec![250.0, 85.0, 12.0, 5.0, 18.0, 95.0],
        phase_deg: vec![30.0, 45.0, 68.0, 72.0, 55.0, 35.0],
        frequency_hz: vec![100.0, 10.0, 1.0, 0.1, 0.01, 0.001],
        clay_cap_depth_m: 180.0,
        interpreted_temperature_c: 240.0,
    };
    let bytes = encode_with_checksum(&val).expect("encode MT survey point");
    let (decoded, consumed): (MtSurveyPoint, _) =
        decode_with_checksum(&bytes).expect("decode MT survey point");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 17: Well completion design
// ---------------------------------------------------------------------------
#[test]
fn test_well_completion_design() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CasingSection {
        outer_diameter_in: f64,
        weight_lb_ft: f64,
        grade: String,
        set_depth_m: f64,
        cement_top_m: f64,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WellCompletionDesign {
        well_id: String,
        total_depth_m: f64,
        well_type: WellType,
        casing_sections: Vec<CasingSection>,
        liner_top_m: f64,
        liner_bottom_m: f64,
        open_hole_interval_m: f64,
    }

    let val = WellCompletionDesign {
        well_id: "GEO-COMP-033".to_string(),
        total_depth_m: 2800.0,
        well_type: WellType::Production,
        casing_sections: vec![
            CasingSection {
                outer_diameter_in: 20.0,
                weight_lb_ft: 94.0,
                grade: "K55".to_string(),
                set_depth_m: 50.0,
                cement_top_m: 0.0,
            },
            CasingSection {
                outer_diameter_in: 13.375,
                weight_lb_ft: 68.0,
                grade: "L80".to_string(),
                set_depth_m: 800.0,
                cement_top_m: 0.0,
            },
            CasingSection {
                outer_diameter_in: 9.625,
                weight_lb_ft: 47.0,
                grade: "T95".to_string(),
                set_depth_m: 2000.0,
                cement_top_m: 600.0,
            },
        ],
        liner_top_m: 1900.0,
        liner_bottom_m: 2500.0,
        open_hole_interval_m: 300.0,
    };
    let bytes = encode_with_checksum(&val).expect("encode well completion design");
    let (decoded, consumed): (WellCompletionDesign, _) =
        decode_with_checksum(&bytes).expect("decode well completion design");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18: Gas extraction system
// ---------------------------------------------------------------------------
#[test]
fn test_gas_extraction_system() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct GasExtractionSystem {
        system_id: String,
        co2_fraction_pct: f64,
        h2s_fraction_pct: f64,
        nitrogen_fraction_pct: f64,
        hydrogen_fraction_pct: f64,
        methane_fraction_pct: f64,
        total_ncg_pct: f64,
        ejector_stages: u8,
        vacuum_pump_power_kw: u32,
        abatement_type: String,
        h2s_emission_g_kwh: f64,
    }

    let val = GasExtractionSystem {
        system_id: "NCG-EXT-02".to_string(),
        co2_fraction_pct: 85.0,
        h2s_fraction_pct: 8.5,
        nitrogen_fraction_pct: 3.0,
        hydrogen_fraction_pct: 2.0,
        methane_fraction_pct: 1.5,
        total_ncg_pct: 2.8,
        ejector_stages: 2,
        vacuum_pump_power_kw: 280,
        abatement_type: "Burner-Scrubber".to_string(),
        h2s_emission_g_kwh: 0.02,
    };
    let bytes = encode_with_checksum(&val).expect("encode gas extraction system");
    let (decoded, consumed): (GasExtractionSystem, _) =
        decode_with_checksum(&bytes).expect("decode gas extraction system");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 19: Downhole pump parameters
// ---------------------------------------------------------------------------
#[test]
fn test_downhole_pump_parameters() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DownholePumpParameters {
        pump_id: String,
        pump_type: String,
        set_depth_m: f64,
        motor_power_kw: u32,
        flow_rate_l_s: f64,
        head_m: f64,
        fluid_temperature_c: f64,
        motor_temperature_c: f64,
        current_draw_a: f64,
        vibration_level_g: f64,
        hours_runtime: u64,
    }

    let val = DownholePumpParameters {
        pump_id: "ESP-WELL-052".to_string(),
        pump_type: "Line Shaft Pump".to_string(),
        set_depth_m: 350.0,
        motor_power_kw: 220,
        flow_rate_l_s: 45.0,
        head_m: 280.0,
        fluid_temperature_c: 180.0,
        motor_temperature_c: 142.0,
        current_draw_a: 85.0,
        vibration_level_g: 0.35,
        hours_runtime: 24_500,
    };
    let bytes = encode_with_checksum(&val).expect("encode downhole pump parameters");
    let (decoded, consumed): (DownholePumpParameters, _) =
        decode_with_checksum(&bytes).expect("decode downhole pump parameters");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 20: Tracer test result
// ---------------------------------------------------------------------------
#[test]
fn test_tracer_test_result() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TracerTestResult {
        test_id: String,
        injection_well: String,
        monitoring_wells: Vec<String>,
        tracer_type: String,
        mass_injected_kg: f64,
        first_arrival_days: Vec<f64>,
        peak_concentration_ppb: Vec<f64>,
        recovery_pct: f64,
        mean_residence_time_days: f64,
        flow_path_identified: bool,
    }

    let val = TracerTestResult {
        test_id: "TT-2024-003".to_string(),
        injection_well: "RJ-009".to_string(),
        monitoring_wells: vec![
            "PRD-021".to_string(),
            "PRD-022".to_string(),
            "OBS-005".to_string(),
        ],
        tracer_type: "2,7-Naphthalenedisulfonate".to_string(),
        mass_injected_kg: 150.0,
        first_arrival_days: vec![12.0, 28.0, 45.0],
        peak_concentration_ppb: vec![85.0, 22.0, 5.0],
        recovery_pct: 38.5,
        mean_residence_time_days: 95.0,
        flow_path_identified: true,
    };
    let bytes = encode_with_checksum(&val).expect("encode tracer test result");
    let (decoded, consumed): (TracerTestResult, _) =
        decode_with_checksum(&bytes).expect("decode tracer test result");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 21: Plant environmental monitoring
// ---------------------------------------------------------------------------
#[test]
fn test_plant_environmental_monitoring() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct EnvironmentalMonitoring {
        station_id: String,
        timestamp_epoch_s: u64,
        h2s_ambient_ppm: f64,
        noise_level_dba: f64,
        thermal_discharge_temp_c: f64,
        subsidence_mm_yr: f64,
        groundwater_level_m: f64,
        co2_emission_tonnes_yr: f64,
        regulatory_compliant: bool,
        seismic_events_last_30d: u32,
        max_magnitude_last_30d: f64,
    }

    let val = EnvironmentalMonitoring {
        station_id: "ENV-MON-WEST-03".to_string(),
        timestamp_epoch_s: 1_710_100_000,
        h2s_ambient_ppm: 0.015,
        noise_level_dba: 52.0,
        thermal_discharge_temp_c: 28.0,
        subsidence_mm_yr: -2.5,
        groundwater_level_m: 45.0,
        co2_emission_tonnes_yr: 18_500.0,
        regulatory_compliant: true,
        seismic_events_last_30d: 8,
        max_magnitude_last_30d: 1.2,
    };
    let bytes = encode_with_checksum(&val).expect("encode environmental monitoring");
    let (decoded, consumed): (EnvironmentalMonitoring, _) =
        decode_with_checksum(&bytes).expect("decode environmental monitoring");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 22: Full geothermal field summary with nested enums
// ---------------------------------------------------------------------------
#[test]
fn test_geothermal_field_summary() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WellStatus {
        well_id: String,
        well_type: WellType,
        fluid_phase: FluidPhase,
        flow_rate_kg_s: f64,
        enthalpy_kj_kg: f64,
        wellhead_valve: ValveState,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct GeothermalFieldSummary {
        field_name: String,
        country: String,
        installed_capacity_mw: f64,
        current_output_mw: f64,
        capacity_factor_pct: f64,
        num_production_wells: u16,
        num_reinjection_wells: u16,
        wells: Vec<WellStatus>,
        reservoir_avg_temp_c: f64,
        reservoir_avg_pressure_mpa: f64,
        annual_energy_gwh: f64,
        seismic_severity_max: SeismicSeverity,
    }

    let val = GeothermalFieldSummary {
        field_name: "Wairakei-Tauhara".to_string(),
        country: "New Zealand".to_string(),
        installed_capacity_mw: 310.0,
        current_output_mw: 275.0,
        capacity_factor_pct: 92.0,
        num_production_wells: 42,
        num_reinjection_wells: 15,
        wells: vec![
            WellStatus {
                well_id: "WK-248".to_string(),
                well_type: WellType::Production,
                fluid_phase: FluidPhase::TwoPhase { steam_fraction: 28 },
                flow_rate_kg_s: 65.0,
                enthalpy_kj_kg: 1150.0,
                wellhead_valve: ValveState::FullyOpen,
            },
            WellStatus {
                well_id: "WK-301".to_string(),
                well_type: WellType::Production,
                fluid_phase: FluidPhase::Steam,
                flow_rate_kg_s: 18.0,
                enthalpy_kj_kg: 2750.0,
                wellhead_valve: ValveState::Throttling {
                    target_pressure_kpa: 950,
                },
            },
            WellStatus {
                well_id: "TH-RJ-07".to_string(),
                well_type: WellType::Reinjection,
                fluid_phase: FluidPhase::Liquid,
                flow_rate_kg_s: 90.0,
                enthalpy_kj_kg: 300.0,
                wellhead_valve: ValveState::PartiallyClosed(20),
            },
            WellStatus {
                well_id: "WK-OBS-11".to_string(),
                well_type: WellType::Observation,
                fluid_phase: FluidPhase::Liquid,
                flow_rate_kg_s: 0.0,
                enthalpy_kj_kg: 0.0,
                wellhead_valve: ValveState::FullyClosed,
            },
        ],
        reservoir_avg_temp_c: 260.0,
        reservoir_avg_pressure_mpa: 22.5,
        annual_energy_gwh: 2410.0,
        seismic_severity_max: SeismicSeverity::Negligible,
    };
    let bytes = encode_with_checksum(&val).expect("encode geothermal field summary");
    let (decoded, consumed): (GeothermalFieldSummary, _) =
        decode_with_checksum(&bytes).expect("decode geothermal field summary");
    assert_eq!(decoded, val);
    assert_eq!(consumed, bytes.len());
}
