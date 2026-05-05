//! Etest/packaging/ESD-focused tests for nested_structs_advanced6 (split from nested_structs_advanced6_test.rs).

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
// Domain types: Die coordinate (shared with wafer and metrology files)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DieCoordinate {
    row: u16,
    col: u16,
    site: u8,
}

// ---------------------------------------------------------------------------
// Electrical test results
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransistorParams {
    device_type: String,
    channel_length_nm: f64,
    channel_width_nm: f64,
    vt_mv: f64,
    idsat_ua_per_um: f64,
    ioff_pa_per_um: f64,
    subthreshold_swing_mv_per_dec: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LeakageCurrent {
    junction_name: String,
    leakage_na: f64,
    voltage_v: f64,
    temperature_c: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtestSite {
    site_id: u16,
    die: DieCoordinate,
    transistors: Vec<TransistorParams>,
    leakage: Vec<LeakageCurrent>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EtestWafer {
    wafer_id: String,
    sites: Vec<EtestSite>,
    pass: bool,
    median_vt_mv: f64,
}

// ---------------------------------------------------------------------------
// Packaging
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WireBondSpec {
    pad_name: String,
    wire_material: String,
    wire_diameter_um: f64,
    loop_height_um: f64,
    ball_size_um: f64,
    bond_force_gf: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlipChipBump {
    bump_id: String,
    material: String,
    diameter_um: f64,
    pitch_um: f64,
    height_um: f64,
    underfill: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PackageSubstrate {
    substrate_type: String,
    layer_count: u8,
    thickness_mm: f64,
    material: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PackageDesign {
    package_type: String,
    body_size_mm_x: f64,
    body_size_mm_y: f64,
    substrate: PackageSubstrate,
    wire_bonds: Vec<WireBondSpec>,
    flip_chip_bumps: Vec<FlipChipBump>,
    lead_count: u16,
}

// ---------------------------------------------------------------------------
// ESD protection
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EsdTestResult {
    model: String,
    voltage_kv: f64,
    pin_name: String,
    passed: bool,
    leakage_after_na: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EsdProtection {
    clamp_type: String,
    trigger_voltage_v: f64,
    holding_voltage_v: f64,
    on_resistance_ohm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EsdSpec {
    device_name: String,
    protection_cells: Vec<EsdProtection>,
    test_results: Vec<EsdTestResult>,
    classification: String,
}

// ===========================================================================
// Tests
// ===========================================================================

#[test]
fn test_electrical_test_transistor_params() {
    let etest = EtestWafer {
        wafer_id: "W07-N7-ETEST".into(),
        sites: vec![
            EtestSite {
                site_id: 1,
                die: DieCoordinate {
                    row: 12,
                    col: 15,
                    site: 1,
                },
                transistors: vec![
                    TransistorParams {
                        device_type: "NFET".into(),
                        channel_length_nm: 7.0,
                        channel_width_nm: 100.0,
                        vt_mv: 280.0,
                        idsat_ua_per_um: 1050.0,
                        ioff_pa_per_um: 5.0,
                        subthreshold_swing_mv_per_dec: 68.0,
                    },
                    TransistorParams {
                        device_type: "PFET".into(),
                        channel_length_nm: 7.0,
                        channel_width_nm: 100.0,
                        vt_mv: -310.0,
                        idsat_ua_per_um: 780.0,
                        ioff_pa_per_um: 3.0,
                        subthreshold_swing_mv_per_dec: 70.0,
                    },
                ],
                leakage: vec![LeakageCurrent {
                    junction_name: "N+/P-Well".into(),
                    leakage_na: 0.12,
                    voltage_v: -1.0,
                    temperature_c: 25.0,
                }],
            },
            EtestSite {
                site_id: 2,
                die: DieCoordinate {
                    row: 12,
                    col: 16,
                    site: 1,
                },
                transistors: vec![TransistorParams {
                    device_type: "NFET".into(),
                    channel_length_nm: 7.0,
                    channel_width_nm: 100.0,
                    vt_mv: 275.0,
                    idsat_ua_per_um: 1060.0,
                    ioff_pa_per_um: 4.8,
                    subthreshold_swing_mv_per_dec: 67.5,
                }],
                leakage: vec![],
            },
        ],
        pass: true,
        median_vt_mv: 278.0,
    };
    let encoded = encode_to_vec(&etest).expect("encode etest wafer");
    let (decoded, _): (EtestWafer, _) = decode_from_slice(&encoded).expect("decode etest wafer");
    assert_eq!(etest, decoded);
}

#[test]
fn test_wire_bond_packaging() {
    let pkg = PackageDesign {
        package_type: "QFN-48".into(),
        body_size_mm_x: 7.0,
        body_size_mm_y: 7.0,
        substrate: PackageSubstrate {
            substrate_type: "Laminate".into(),
            layer_count: 4,
            thickness_mm: 0.4,
            material: "BT-Resin".into(),
        },
        wire_bonds: vec![
            WireBondSpec {
                pad_name: "VDD".into(),
                wire_material: "Au".into(),
                wire_diameter_um: 25.0,
                loop_height_um: 200.0,
                ball_size_um: 55.0,
                bond_force_gf: 30.0,
            },
            WireBondSpec {
                pad_name: "IO_0".into(),
                wire_material: "Au".into(),
                wire_diameter_um: 20.0,
                loop_height_um: 180.0,
                ball_size_um: 48.0,
                bond_force_gf: 25.0,
            },
        ],
        flip_chip_bumps: vec![],
        lead_count: 48,
    };
    let encoded = encode_to_vec(&pkg).expect("encode wire bond package");
    let (decoded, _): (PackageDesign, _) =
        decode_from_slice(&encoded).expect("decode wire bond package");
    assert_eq!(pkg, decoded);
}

#[test]
fn test_flip_chip_packaging() {
    let pkg = PackageDesign {
        package_type: "FCBGA-1024".into(),
        body_size_mm_x: 35.0,
        body_size_mm_y: 35.0,
        substrate: PackageSubstrate {
            substrate_type: "ABF-Buildup".into(),
            layer_count: 12,
            thickness_mm: 1.2,
            material: "ABF-GX92".into(),
        },
        wire_bonds: vec![],
        flip_chip_bumps: vec![
            FlipChipBump {
                bump_id: "C4-A1".into(),
                material: "SnAg".into(),
                diameter_um: 80.0,
                pitch_um: 150.0,
                height_um: 50.0,
                underfill: true,
            },
            FlipChipBump {
                bump_id: "C4-A2".into(),
                material: "SnAg".into(),
                diameter_um: 80.0,
                pitch_um: 150.0,
                height_um: 50.0,
                underfill: true,
            },
            FlipChipBump {
                bump_id: "uBump-B1".into(),
                material: "Cu-Pillar".into(),
                diameter_um: 25.0,
                pitch_um: 40.0,
                height_um: 30.0,
                underfill: true,
            },
        ],
        lead_count: 1024,
    };
    let encoded = encode_to_vec(&pkg).expect("encode flip chip package");
    let (decoded, _): (PackageDesign, _) =
        decode_from_slice(&encoded).expect("decode flip chip package");
    assert_eq!(pkg, decoded);
}

#[test]
fn test_esd_protection_spec() {
    let esd = EsdSpec {
        device_name: "APEX7-IO-PAD".into(),
        protection_cells: vec![
            EsdProtection {
                clamp_type: "GGNMOS".into(),
                trigger_voltage_v: 6.5,
                holding_voltage_v: 3.5,
                on_resistance_ohm: 2.5,
            },
            EsdProtection {
                clamp_type: "RC-Triggered-Clamp".into(),
                trigger_voltage_v: 5.0,
                holding_voltage_v: 1.8,
                on_resistance_ohm: 1.0,
            },
        ],
        test_results: vec![
            EsdTestResult {
                model: "HBM".into(),
                voltage_kv: 2.0,
                pin_name: "IO_5".into(),
                passed: true,
                leakage_after_na: 0.8,
            },
            EsdTestResult {
                model: "CDM".into(),
                voltage_kv: 0.5,
                pin_name: "IO_5".into(),
                passed: true,
                leakage_after_na: 1.2,
            },
            EsdTestResult {
                model: "HBM".into(),
                voltage_kv: 2.0,
                pin_name: "VDD".into(),
                passed: true,
                leakage_after_na: 0.5,
            },
        ],
        classification: "Class-2".into(),
    };
    let encoded = encode_to_vec(&esd).expect("encode esd spec");
    let (decoded, _): (EsdSpec, _) = decode_from_slice(&encoded).expect("decode esd spec");
    assert_eq!(esd, decoded);
}
