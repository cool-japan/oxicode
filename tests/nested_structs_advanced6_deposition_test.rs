//! Deposition/diffusion/implant-focused tests for nested_structs_advanced6 (split from nested_structs_advanced6_test.rs).

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
// Domain types: Lot info (shared with wafer file)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LotInfo {
    lot_id: String,
    product_code: String,
    technology_node_nm: u16,
    wafer_count: u8,
    priority: u8,
}

// ---------------------------------------------------------------------------
// CMP settings
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlurryConfig {
    slurry_type: String,
    flow_rate_ml_per_min: f64,
    ph_value: f64,
    abrasive_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PadConditioner {
    conditioner_type: String,
    sweep_speed_rpm: f64,
    down_force_lbs: f64,
    in_situ: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CmpProfile {
    platen_speed_rpm: f64,
    carrier_speed_rpm: f64,
    down_force_psi: f64,
    polish_time_sec: f64,
    slurry: SlurryConfig,
    conditioner: PadConditioner,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CmpProcess {
    process_id: String,
    layer_target: String,
    profiles: Vec<CmpProfile>,
    post_clean_recipe: String,
}

// ---------------------------------------------------------------------------
// Ion implantation
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantSpecies {
    element: String,
    isotope_mass: u16,
    charge_state: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantDose {
    dose_atoms_per_cm2: f64,
    energy_kev: f64,
    tilt_deg: f64,
    twist_deg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantProfile {
    profile_name: String,
    species: ImplantSpecies,
    dose_params: ImplantDose,
    beam_current_ua: f64,
    scan_mode: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IonImplantRecipe {
    recipe_id: String,
    tool_name: String,
    profiles: Vec<ImplantProfile>,
    anneal_required: bool,
    anneal_temp_c: Option<f64>,
}

// ---------------------------------------------------------------------------
// Diffusion furnace profiles
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FurnaceRamp {
    segment_name: String,
    start_temp_c: f64,
    end_temp_c: f64,
    ramp_rate_c_per_min: f64,
    hold_time_min: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FurnaceGas {
    gas_name: String,
    flow_slm: f64,
    start_time_min: f64,
    stop_time_min: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FurnaceRecipe {
    recipe_name: String,
    tube_id: String,
    ramp_segments: Vec<FurnaceRamp>,
    gas_profile: Vec<FurnaceGas>,
    boat_rotation_rpm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiffusionRun {
    run_id: String,
    lot: LotInfo,
    recipe: FurnaceRecipe,
    oxide_target_nm: f64,
    oxide_measured_nm: Option<f64>,
}

// ---------------------------------------------------------------------------
// CVD/PVD deposition
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrecursorGas {
    chemical_name: String,
    flow_sccm: f64,
    bubbler_temp_c: Option<f64>,
    carrier_gas: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DepositionConditions {
    temperature_c: f64,
    pressure_torr: f64,
    rf_power_watts: Option<f64>,
    dc_power_kw: Option<f64>,
    substrate_bias_v: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DepositionLayer {
    layer_name: String,
    method: String,
    precursors: Vec<PrecursorGas>,
    conditions: DepositionConditions,
    target_thickness_nm: f64,
    deposition_rate_nm_per_min: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DepositionProcess {
    process_id: String,
    tool_id: String,
    layers: Vec<DepositionLayer>,
    total_time_min: f64,
}

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn test_cmp_polish_process() {
    let cmp = CmpProcess {
        process_id: "CMP-CU-DUAL-V2".into(),
        layer_target: "COPPER_M2".into(),
        profiles: vec![
            CmpProfile {
                platen_speed_rpm: 93.0,
                carrier_speed_rpm: 87.0,
                down_force_psi: 2.5,
                polish_time_sec: 60.0,
                slurry: SlurryConfig {
                    slurry_type: "Barrier-CMP".into(),
                    flow_rate_ml_per_min: 200.0,
                    ph_value: 9.5,
                    abrasive_pct: 3.0,
                },
                conditioner: PadConditioner {
                    conditioner_type: "Diamond-Disk".into(),
                    sweep_speed_rpm: 20.0,
                    down_force_lbs: 5.0,
                    in_situ: true,
                },
            },
            CmpProfile {
                platen_speed_rpm: 60.0,
                carrier_speed_rpm: 55.0,
                down_force_psi: 1.2,
                polish_time_sec: 30.0,
                slurry: SlurryConfig {
                    slurry_type: "Buff-Clean".into(),
                    flow_rate_ml_per_min: 300.0,
                    ph_value: 7.0,
                    abrasive_pct: 0.5,
                },
                conditioner: PadConditioner {
                    conditioner_type: "Brush".into(),
                    sweep_speed_rpm: 10.0,
                    down_force_lbs: 2.0,
                    in_situ: false,
                },
            },
        ],
        post_clean_recipe: "MEGASONIC-DIW-V3".into(),
    };
    let encoded = encode_to_vec(&cmp).expect("encode cmp process");
    let (decoded, _): (CmpProcess, _) = decode_from_slice(&encoded).expect("decode cmp process");
    assert_eq!(cmp, decoded);
}

#[test]
fn test_ion_implant_recipe() {
    let recipe = IonImplantRecipe {
        recipe_id: "IMPL-NWELL-V5".into(),
        tool_name: "AMAT-VIISta-900".into(),
        profiles: vec![
            ImplantProfile {
                profile_name: "N-Well Deep".into(),
                species: ImplantSpecies {
                    element: "P".into(),
                    isotope_mass: 31,
                    charge_state: 1,
                },
                dose_params: ImplantDose {
                    dose_atoms_per_cm2: 1.5e13,
                    energy_kev: 500.0,
                    tilt_deg: 7.0,
                    twist_deg: 22.0,
                },
                beam_current_ua: 800.0,
                scan_mode: "hybrid".into(),
            },
            ImplantProfile {
                profile_name: "N-Well Retro".into(),
                species: ImplantSpecies {
                    element: "P".into(),
                    isotope_mass: 31,
                    charge_state: 2,
                },
                dose_params: ImplantDose {
                    dose_atoms_per_cm2: 5.0e12,
                    energy_kev: 180.0,
                    tilt_deg: 7.0,
                    twist_deg: 22.0,
                },
                beam_current_ua: 500.0,
                scan_mode: "parallel".into(),
            },
        ],
        anneal_required: true,
        anneal_temp_c: Some(1050.0),
    };
    let encoded = encode_to_vec(&recipe).expect("encode implant recipe");
    let (decoded, _): (IonImplantRecipe, _) =
        decode_from_slice(&encoded).expect("decode implant recipe");
    assert_eq!(recipe, decoded);
}

#[test]
fn test_diffusion_furnace_recipe() {
    let run = DiffusionRun {
        run_id: "DIFF-20260315-001".into(),
        lot: LotInfo {
            lot_id: "LOT-GATE-OX".into(),
            product_code: "N5-HPC".into(),
            technology_node_nm: 5,
            wafer_count: 25,
            priority: 2,
        },
        recipe: FurnaceRecipe {
            recipe_name: "GATE-OX-12A".into(),
            tube_id: "TUBE-04".into(),
            ramp_segments: vec![
                FurnaceRamp {
                    segment_name: "Ramp-Up".into(),
                    start_temp_c: 600.0,
                    end_temp_c: 850.0,
                    ramp_rate_c_per_min: 5.0,
                    hold_time_min: 0.0,
                },
                FurnaceRamp {
                    segment_name: "Oxidation".into(),
                    start_temp_c: 850.0,
                    end_temp_c: 850.0,
                    ramp_rate_c_per_min: 0.0,
                    hold_time_min: 12.0,
                },
                FurnaceRamp {
                    segment_name: "Anneal".into(),
                    start_temp_c: 850.0,
                    end_temp_c: 1000.0,
                    ramp_rate_c_per_min: 3.0,
                    hold_time_min: 5.0,
                },
                FurnaceRamp {
                    segment_name: "Cool-Down".into(),
                    start_temp_c: 1000.0,
                    end_temp_c: 600.0,
                    ramp_rate_c_per_min: -2.0,
                    hold_time_min: 0.0,
                },
            ],
            gas_profile: vec![
                FurnaceGas {
                    gas_name: "N2".into(),
                    flow_slm: 10.0,
                    start_time_min: 0.0,
                    stop_time_min: 50.0,
                },
                FurnaceGas {
                    gas_name: "O2-DRY".into(),
                    flow_slm: 5.0,
                    start_time_min: 50.0,
                    stop_time_min: 62.0,
                },
                FurnaceGas {
                    gas_name: "N2O".into(),
                    flow_slm: 2.0,
                    start_time_min: 62.0,
                    stop_time_min: 67.0,
                },
            ],
            boat_rotation_rpm: 1.5,
        },
        oxide_target_nm: 1.2,
        oxide_measured_nm: Some(1.18),
    };
    let encoded = encode_to_vec(&run).expect("encode diffusion run");
    let (decoded, _): (DiffusionRun, _) =
        decode_from_slice(&encoded).expect("decode diffusion run");
    assert_eq!(run, decoded);
}

#[test]
fn test_cvd_deposition_process() {
    let dep = DepositionProcess {
        process_id: "CVD-ILD-V6".into(),
        tool_id: "AMAT-PRODUCER-07".into(),
        layers: vec![
            DepositionLayer {
                layer_name: "USG-Liner".into(),
                method: "PECVD".into(),
                precursors: vec![
                    PrecursorGas {
                        chemical_name: "TEOS".into(),
                        flow_sccm: 1200.0,
                        bubbler_temp_c: Some(35.0),
                        carrier_gas: Some("He".into()),
                    },
                    PrecursorGas {
                        chemical_name: "O2".into(),
                        flow_sccm: 600.0,
                        bubbler_temp_c: None,
                        carrier_gas: None,
                    },
                ],
                conditions: DepositionConditions {
                    temperature_c: 400.0,
                    pressure_torr: 8.0,
                    rf_power_watts: Some(700.0),
                    dc_power_kw: None,
                    substrate_bias_v: None,
                },
                target_thickness_nm: 50.0,
                deposition_rate_nm_per_min: 250.0,
            },
            DepositionLayer {
                layer_name: "Low-k-ILD".into(),
                method: "PECVD".into(),
                precursors: vec![PrecursorGas {
                    chemical_name: "DEMS".into(),
                    flow_sccm: 800.0,
                    bubbler_temp_c: Some(40.0),
                    carrier_gas: Some("He".into()),
                }],
                conditions: DepositionConditions {
                    temperature_c: 350.0,
                    pressure_torr: 6.0,
                    rf_power_watts: Some(500.0),
                    dc_power_kw: None,
                    substrate_bias_v: Some(-50.0),
                },
                target_thickness_nm: 200.0,
                deposition_rate_nm_per_min: 150.0,
            },
        ],
        total_time_min: 3.5,
    };
    let encoded = encode_to_vec(&dep).expect("encode deposition process");
    let (decoded, _): (DepositionProcess, _) =
        decode_from_slice(&encoded).expect("decode deposition process");
    assert_eq!(dep, decoded);
}

#[test]
fn test_pvd_sputtering_layer() {
    let dep = DepositionProcess {
        process_id: "PVD-BARRIER-TaN".into(),
        tool_id: "AMAT-ENDURA-03".into(),
        layers: vec![DepositionLayer {
            layer_name: "TaN-Barrier".into(),
            method: "PVD-Reactive".into(),
            precursors: vec![PrecursorGas {
                chemical_name: "N2".into(),
                flow_sccm: 30.0,
                bubbler_temp_c: None,
                carrier_gas: None,
            }],
            conditions: DepositionConditions {
                temperature_c: 250.0,
                pressure_torr: 0.003,
                rf_power_watts: None,
                dc_power_kw: Some(12.0),
                substrate_bias_v: Some(-100.0),
            },
            target_thickness_nm: 3.0,
            deposition_rate_nm_per_min: 15.0,
        }],
        total_time_min: 0.2,
    };
    let encoded = encode_to_vec(&dep).expect("encode pvd process");
    let (decoded, _): (DepositionProcess, _) =
        decode_from_slice(&encoded).expect("decode pvd process");
    assert_eq!(dep, decoded);
}

#[test]
fn test_implant_without_anneal() {
    let recipe = IonImplantRecipe {
        recipe_id: "IMPL-HALO-V2".into(),
        tool_name: "AXCELIS-PURION-H".into(),
        profiles: vec![ImplantProfile {
            profile_name: "Halo-NMOS".into(),
            species: ImplantSpecies {
                element: "In".into(),
                isotope_mass: 115,
                charge_state: 1,
            },
            dose_params: ImplantDose {
                dose_atoms_per_cm2: 3.0e13,
                energy_kev: 60.0,
                tilt_deg: 28.0,
                twist_deg: 0.0,
            },
            beam_current_ua: 200.0,
            scan_mode: "quad".into(),
        }],
        anneal_required: false,
        anneal_temp_c: None,
    };
    let encoded = encode_to_vec(&recipe).expect("encode halo implant");
    let (decoded, _): (IonImplantRecipe, _) =
        decode_from_slice(&encoded).expect("decode halo implant");
    assert_eq!(recipe, decoded);
}

#[test]
fn test_diffusion_no_measurement() {
    let run = DiffusionRun {
        run_id: "DIFF-20260315-005".into(),
        lot: LotInfo {
            lot_id: "LOT-LINER-OX".into(),
            product_code: "N3-HVM".into(),
            technology_node_nm: 3,
            wafer_count: 13,
            priority: 3,
        },
        recipe: FurnaceRecipe {
            recipe_name: "LINER-OX-5A".into(),
            tube_id: "TUBE-09".into(),
            ramp_segments: vec![FurnaceRamp {
                segment_name: "Rapid-Ox".into(),
                start_temp_c: 700.0,
                end_temp_c: 700.0,
                ramp_rate_c_per_min: 0.0,
                hold_time_min: 2.0,
            }],
            gas_profile: vec![FurnaceGas {
                gas_name: "O2-DRY".into(),
                flow_slm: 3.0,
                start_time_min: 0.0,
                stop_time_min: 2.0,
            }],
            boat_rotation_rpm: 2.0,
        },
        oxide_target_nm: 0.5,
        oxide_measured_nm: None,
    };
    let encoded = encode_to_vec(&run).expect("encode liner ox run");
    let (decoded, _): (DiffusionRun, _) = decode_from_slice(&encoded).expect("decode liner ox run");
    assert_eq!(run, decoded);
}
