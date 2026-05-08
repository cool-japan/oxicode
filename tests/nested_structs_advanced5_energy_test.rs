//! Energy/quality/digital-thread-focused tests for nested_structs_advanced5 (split from nested_structs_advanced5_test.rs).

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
// Energy consumption profiles
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerMeterReading {
    meter_id: String,
    active_kw: f64,
    reactive_kvar: f64,
    power_factor: f64,
    voltage_v: f64,
    current_a: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyZone {
    zone_name: String,
    meters: Vec<PowerMeterReading>,
    sub_zones: Vec<EnergyZone>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyProfile {
    facility_id: String,
    period: String,
    zones: Vec<EnergyZone>,
    total_kwh: f64,
    peak_demand_kw: f64,
}

// ---------------------------------------------------------------------------
// Quality inspection results
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeasurementResult {
    parameter: String,
    nominal: f64,
    measured: f64,
    tolerance: f64,
    pass: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InspectionStation {
    station_name: String,
    inspector_id: String,
    measurements: Vec<MeasurementResult>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityInspection {
    batch_id: String,
    product_code: String,
    stations: Vec<InspectionStation>,
    overall_pass: bool,
    timestamp_ms: u64,
}

// ---------------------------------------------------------------------------
// Supply chain digital thread
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SupplierInfo {
    supplier_id: String,
    name: String,
    lead_time_days: u16,
    quality_rating: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaterialLot {
    lot_number: String,
    supplier: SupplierInfo,
    quantity: f64,
    unit: String,
    received_date: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DigitalThread {
    work_order: String,
    material_lots: Vec<MaterialLot>,
    production_steps: Vec<String>,
    final_inspection: Option<QualityInspection>,
}

// ===========================================================================
// Helper constructors
// ===========================================================================

fn roundtrip<T: Encode + Decode + std::fmt::Debug + PartialEq>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {ctx}"));
    let (decoded, _): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {ctx}"));
    assert_eq!(val, &decoded, "roundtrip failed for {ctx}");
}

// ===========================================================================
// Tests
// ===========================================================================

// Test 8: Energy consumption profile with nested zones and sub-zones
#[test]
fn test_energy_profile_roundtrip() {
    let profile = EnergyProfile {
        facility_id: "FAC-001".to_string(),
        period: "2026-03".to_string(),
        zones: vec![EnergyZone {
            zone_name: "Building-A".to_string(),
            meters: vec![PowerMeterReading {
                meter_id: "EM-001".to_string(),
                active_kw: 150.0,
                reactive_kvar: 45.0,
                power_factor: 0.96,
                voltage_v: 480.0,
                current_a: 195.0,
            }],
            sub_zones: vec![
                EnergyZone {
                    zone_name: "HVAC".to_string(),
                    meters: vec![PowerMeterReading {
                        meter_id: "EM-002".to_string(),
                        active_kw: 80.0,
                        reactive_kvar: 30.0,
                        power_factor: 0.94,
                        voltage_v: 480.0,
                        current_a: 105.0,
                    }],
                    sub_zones: vec![EnergyZone {
                        zone_name: "Chiller-1".to_string(),
                        meters: vec![PowerMeterReading {
                            meter_id: "EM-003".to_string(),
                            active_kw: 55.0,
                            reactive_kvar: 20.0,
                            power_factor: 0.94,
                            voltage_v: 480.0,
                            current_a: 72.0,
                        }],
                        sub_zones: vec![],
                    }],
                },
                EnergyZone {
                    zone_name: "Lighting".to_string(),
                    meters: vec![PowerMeterReading {
                        meter_id: "EM-004".to_string(),
                        active_kw: 12.0,
                        reactive_kvar: 1.0,
                        power_factor: 0.99,
                        voltage_v: 277.0,
                        current_a: 43.5,
                    }],
                    sub_zones: vec![],
                },
            ],
        }],
        total_kwh: 108000.0,
        peak_demand_kw: 220.0,
    };
    roundtrip(&profile, "energy profile");
}

// Test 9: Quality inspection with multiple stations and measurements
#[test]
fn test_quality_inspection_roundtrip() {
    let inspection = QualityInspection {
        batch_id: "B-20260315-001".to_string(),
        product_code: "WIDGET-A".to_string(),
        stations: vec![
            InspectionStation {
                station_name: "Dimensional".to_string(),
                inspector_id: "INS-042".to_string(),
                measurements: vec![
                    MeasurementResult {
                        parameter: "length_mm".to_string(),
                        nominal: 100.0,
                        measured: 100.02,
                        tolerance: 0.05,
                        pass: true,
                    },
                    MeasurementResult {
                        parameter: "width_mm".to_string(),
                        nominal: 50.0,
                        measured: 49.98,
                        tolerance: 0.05,
                        pass: true,
                    },
                ],
            },
            InspectionStation {
                station_name: "Surface".to_string(),
                inspector_id: "INS-043".to_string(),
                measurements: vec![MeasurementResult {
                    parameter: "roughness_ra".to_string(),
                    nominal: 0.8,
                    measured: 0.75,
                    tolerance: 0.2,
                    pass: true,
                }],
            },
        ],
        overall_pass: true,
        timestamp_ms: 1710500000000,
    };
    roundtrip(&inspection, "quality inspection");
}

// Test 10: Supply chain digital thread linking materials to production
#[test]
fn test_digital_thread_roundtrip() {
    let thread = DigitalThread {
        work_order: "WO-2026-0315".to_string(),
        material_lots: vec![
            MaterialLot {
                lot_number: "LOT-A-001".to_string(),
                supplier: SupplierInfo {
                    supplier_id: "SUP-10".to_string(),
                    name: "SteelCo".to_string(),
                    lead_time_days: 14,
                    quality_rating: 4.5,
                },
                quantity: 500.0,
                unit: "kg".to_string(),
                received_date: "2026-03-01".to_string(),
            },
            MaterialLot {
                lot_number: "LOT-B-002".to_string(),
                supplier: SupplierInfo {
                    supplier_id: "SUP-20".to_string(),
                    name: "PolymerInc".to_string(),
                    lead_time_days: 7,
                    quality_rating: 4.8,
                },
                quantity: 200.0,
                unit: "liters".to_string(),
                received_date: "2026-03-05".to_string(),
            },
        ],
        production_steps: vec![
            "Casting".to_string(),
            "Machining".to_string(),
            "Coating".to_string(),
            "Assembly".to_string(),
        ],
        final_inspection: None,
    };
    roundtrip(&thread, "digital thread");
}

// Test 16: Digital thread with final inspection present
#[test]
fn test_digital_thread_with_inspection_roundtrip() {
    let thread = DigitalThread {
        work_order: "WO-2026-0400".to_string(),
        material_lots: vec![MaterialLot {
            lot_number: "LOT-X-100".to_string(),
            supplier: SupplierInfo {
                supplier_id: "SUP-99".to_string(),
                name: "AluminaCorp".to_string(),
                lead_time_days: 21,
                quality_rating: 4.2,
            },
            quantity: 1000.0,
            unit: "kg".to_string(),
            received_date: "2026-02-20".to_string(),
        }],
        production_steps: vec!["Extrusion".to_string(), "Anodizing".to_string()],
        final_inspection: Some(QualityInspection {
            batch_id: "B-FINAL-001".to_string(),
            product_code: "EXTRUSION-PRO".to_string(),
            stations: vec![InspectionStation {
                station_name: "FinalCheck".to_string(),
                inspector_id: "INS-100".to_string(),
                measurements: vec![MeasurementResult {
                    parameter: "hardness_hrc".to_string(),
                    nominal: 60.0,
                    measured: 59.8,
                    tolerance: 2.0,
                    pass: true,
                }],
            }],
            overall_pass: true,
            timestamp_ms: 1710600000000,
        }),
    };
    roundtrip(&thread, "digital thread with inspection");
}
