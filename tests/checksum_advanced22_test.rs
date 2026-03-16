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
use oxicode::checksum::{unwrap_with_checksum, wrap_with_checksum, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types: toxicology / pharmacokinetics
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum AdministrationRoute {
    Oral,
    Intravenous,
    Subcutaneous,
    Inhalation,
    Topical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ToxicityClass {
    Benign,
    Mild,
    Moderate,
    Severe,
    Fatal,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CompoundDose {
    compound_id: u32,
    name: String,
    dose_ug: u64,
    route: AdministrationRoute,
    administered_at: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MetaboliteReading {
    metabolite_id: u32,
    plasma_conc_pmol: u64,
    half_life_s: u32,
    peak_time_s: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ToxicologyReport {
    report_id: u64,
    patient_id: u64,
    doses: Vec<CompoundDose>,
    metabolites: Vec<MetaboliteReading>,
    toxicity: ToxicityClass,
}

// ---------------------------------------------------------------------------
// Test 1: CompoundDose wrapped and verified
// ---------------------------------------------------------------------------
#[test]
fn test_compound_dose_wrapped_and_verified() {
    let dose = CompoundDose {
        compound_id: 1001,
        name: "Acetaminophen".to_string(),
        dose_ug: 500_000,
        route: AdministrationRoute::Oral,
        administered_at: 1_700_000_000,
    };
    let payload = encode_to_vec(&dose).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (CompoundDose, _) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(dose, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: MetaboliteReading integrity
// ---------------------------------------------------------------------------
#[test]
fn test_metabolite_reading_integrity() {
    let reading = MetaboliteReading {
        metabolite_id: 2002,
        plasma_conc_pmol: 780_000,
        half_life_s: 14400,
        peak_time_s: 3600,
    };
    let payload = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (MetaboliteReading, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(reading, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: ToxicologyReport integrity
// ---------------------------------------------------------------------------
#[test]
fn test_toxicology_report_integrity() {
    let report = ToxicologyReport {
        report_id: 9001,
        patient_id: 42,
        doses: vec![CompoundDose {
            compound_id: 5,
            name: "Ibuprofen".to_string(),
            dose_ug: 200_000,
            route: AdministrationRoute::Oral,
            administered_at: 1_600_000_000,
        }],
        metabolites: vec![MetaboliteReading {
            metabolite_id: 10,
            plasma_conc_pmol: 150_000,
            half_life_s: 7200,
            peak_time_s: 1800,
        }],
        toxicity: ToxicityClass::Mild,
    };
    let payload = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (ToxicologyReport, _) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: AdministrationRoute::Oral variant roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_route_oral_via_checksum() {
    let route = AdministrationRoute::Oral;
    let payload = encode_to_vec(&route).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (AdministrationRoute, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(route, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: AdministrationRoute::Intravenous variant roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_route_intravenous_via_checksum() {
    let route = AdministrationRoute::Intravenous;
    let payload = encode_to_vec(&route).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (AdministrationRoute, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(route, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: AdministrationRoute::Subcutaneous variant roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_route_subcutaneous_via_checksum() {
    let route = AdministrationRoute::Subcutaneous;
    let payload = encode_to_vec(&route).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (AdministrationRoute, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(route, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: AdministrationRoute::Inhalation variant roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_route_inhalation_via_checksum() {
    let route = AdministrationRoute::Inhalation;
    let payload = encode_to_vec(&route).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (AdministrationRoute, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(route, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: AdministrationRoute::Topical variant roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_route_topical_via_checksum() {
    let route = AdministrationRoute::Topical;
    let payload = encode_to_vec(&route).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (AdministrationRoute, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(route, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: each ToxicityClass variant roundtrips via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_all_toxicity_class_variants_via_checksum() {
    let variants = [
        ToxicityClass::Benign,
        ToxicityClass::Mild,
        ToxicityClass::Moderate,
        ToxicityClass::Severe,
        ToxicityClass::Fatal,
    ];
    for variant in variants {
        let payload = encode_to_vec(&variant).expect("encode failed");
        let wrapped = wrap_with_checksum(&payload);
        let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
        let (decoded, _): (ToxicityClass, _) =
            decode_from_slice(&recovered).expect("decode failed");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 10: HEADER_SIZE constant is exactly 16
// ---------------------------------------------------------------------------
#[test]
fn test_header_size_is_16() {
    assert_eq!(HEADER_SIZE, 16, "HEADER_SIZE must be 16 bytes");
}

// ---------------------------------------------------------------------------
// Test 11: wrap increases length by exactly HEADER_SIZE
// ---------------------------------------------------------------------------
#[test]
fn test_wrap_increases_length_by_header_size() {
    let dose = CompoundDose {
        compound_id: 7,
        name: "Warfarin".to_string(),
        dose_ug: 5_000,
        route: AdministrationRoute::Oral,
        administered_at: 1_650_000_000,
    };
    let payload = encode_to_vec(&dose).expect("encode failed");
    let payload_len = payload.len();
    let wrapped = wrap_with_checksum(&payload);
    assert_eq!(
        wrapped.len(),
        payload_len + HEADER_SIZE,
        "wrap must add exactly HEADER_SIZE bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 12: empty dose list wrapped and recovers
// ---------------------------------------------------------------------------
#[test]
fn test_empty_dose_list_wrapped() {
    let report = ToxicologyReport {
        report_id: 0,
        patient_id: 99,
        doses: vec![],
        metabolites: vec![],
        toxicity: ToxicityClass::Benign,
    };
    let payload = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (ToxicologyReport, _) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(report, decoded);
    assert!(decoded.doses.is_empty(), "doses should be empty");
}

// ---------------------------------------------------------------------------
// Test 13: large report (10 doses, 20 metabolites) integrity
// ---------------------------------------------------------------------------
#[test]
fn test_large_report_integrity() {
    let doses: Vec<CompoundDose> = (0..10)
        .map(|i| CompoundDose {
            compound_id: i as u32,
            name: format!("Compound_{}", i),
            dose_ug: (i as u64 + 1) * 10_000,
            route: AdministrationRoute::Intravenous,
            administered_at: 1_700_000_000 + i as u64 * 3600,
        })
        .collect();

    let metabolites: Vec<MetaboliteReading> = (0..20)
        .map(|i| MetaboliteReading {
            metabolite_id: i as u32,
            plasma_conc_pmol: (i as u64 + 1) * 50_000,
            half_life_s: 3600 * (i as u32 + 1),
            peak_time_s: 900 * (i as u32 + 1),
        })
        .collect();

    let report = ToxicologyReport {
        report_id: 88888,
        patient_id: 12345,
        doses,
        metabolites,
        toxicity: ToxicityClass::Moderate,
    };

    let payload = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (ToxicologyReport, _) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(report, decoded);
    assert_eq!(decoded.doses.len(), 10);
    assert_eq!(decoded.metabolites.len(), 20);
}

// ---------------------------------------------------------------------------
// Test 14: corruption detection for CompoundDose
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detection_compound_dose() {
    let dose = CompoundDose {
        compound_id: 300,
        name: "Digoxin".to_string(),
        dose_ug: 250,
        route: AdministrationRoute::Oral,
        administered_at: 1_680_000_000,
    };
    let payload = encode_to_vec(&dose).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption in CompoundDose payload must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 15: corruption detection for MetaboliteReading
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detection_metabolite_reading() {
    let reading = MetaboliteReading {
        metabolite_id: 555,
        plasma_conc_pmol: 990_000,
        half_life_s: 21600,
        peak_time_s: 5400,
    };
    let payload = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let mut corrupted = wrapped.clone();
    // Corrupt only the payload region (after the full header) to avoid integer
    // overflow inside the length-field validation path.
    for b in corrupted[HEADER_SIZE..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption in MetaboliteReading payload must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 16: corruption detection for ToxicologyReport
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detection_toxicology_report() {
    let report = ToxicologyReport {
        report_id: 7777,
        patient_id: 8888,
        doses: vec![CompoundDose {
            compound_id: 1,
            name: "Lithium".to_string(),
            dose_ug: 300_000,
            route: AdministrationRoute::Oral,
            administered_at: 1_690_000_000,
        }],
        metabolites: vec![],
        toxicity: ToxicityClass::Severe,
    };
    let payload = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption in ToxicologyReport payload must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 17: IV vs Oral produce distinct wrapped byte sequences
// ---------------------------------------------------------------------------
#[test]
fn test_iv_vs_oral_distinct_wrapped_bytes() {
    let dose_iv = CompoundDose {
        compound_id: 50,
        name: "Morphine".to_string(),
        dose_ug: 10_000,
        route: AdministrationRoute::Intravenous,
        administered_at: 1_710_000_000,
    };
    let dose_oral = CompoundDose {
        compound_id: 50,
        name: "Morphine".to_string(),
        dose_ug: 10_000,
        route: AdministrationRoute::Oral,
        administered_at: 1_710_000_000,
    };
    let payload_iv = encode_to_vec(&dose_iv).expect("encode iv failed");
    let payload_oral = encode_to_vec(&dose_oral).expect("encode oral failed");
    let wrapped_iv = wrap_with_checksum(&payload_iv);
    let wrapped_oral = wrap_with_checksum(&payload_oral);
    assert_ne!(
        wrapped_iv, wrapped_oral,
        "IV and Oral route encodings must differ"
    );
}

// ---------------------------------------------------------------------------
// Test 18: peak time accuracy preserved through checksum wrap/unwrap
// ---------------------------------------------------------------------------
#[test]
fn test_peak_time_accuracy() {
    let expected_peak = 7_200u32;
    let reading = MetaboliteReading {
        metabolite_id: 99,
        plasma_conc_pmol: 42_000,
        half_life_s: 28_800,
        peak_time_s: expected_peak,
    };
    let payload = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (MetaboliteReading, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(
        decoded.peak_time_s, expected_peak,
        "peak time must survive wrap/unwrap unchanged"
    );
}

// ---------------------------------------------------------------------------
// Test 19: plasma concentration accuracy preserved
// ---------------------------------------------------------------------------
#[test]
fn test_plasma_concentration_accuracy() {
    let expected_conc = 123_456_789_012u64;
    let reading = MetaboliteReading {
        metabolite_id: 77,
        plasma_conc_pmol: expected_conc,
        half_life_s: 3600,
        peak_time_s: 900,
    };
    let payload = encode_to_vec(&reading).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (MetaboliteReading, _) =
        decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(
        decoded.plasma_conc_pmol, expected_conc,
        "plasma concentration must survive wrap/unwrap unchanged"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Fatal toxicity class integrity
// ---------------------------------------------------------------------------
#[test]
fn test_fatal_toxicity_class_integrity() {
    let report = ToxicologyReport {
        report_id: 666,
        patient_id: 1,
        doses: vec![CompoundDose {
            compound_id: 99,
            name: "Unknown_Agent".to_string(),
            dose_ug: 999_999_999,
            route: AdministrationRoute::Inhalation,
            administered_at: 1_720_000_000,
        }],
        metabolites: vec![MetaboliteReading {
            metabolite_id: 1,
            plasma_conc_pmol: u64::MAX / 2,
            half_life_s: 600,
            peak_time_s: 120,
        }],
        toxicity: ToxicityClass::Fatal,
    };
    let payload = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (ToxicologyReport, _) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.toxicity, ToxicityClass::Fatal);
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: benign report roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_benign_report_roundtrip() {
    let report = ToxicologyReport {
        report_id: 1,
        patient_id: 2,
        doses: vec![CompoundDose {
            compound_id: 10,
            name: "Vitamin_C".to_string(),
            dose_ug: 1_000_000,
            route: AdministrationRoute::Oral,
            administered_at: 1_600_000_001,
        }],
        metabolites: vec![MetaboliteReading {
            metabolite_id: 5,
            plasma_conc_pmol: 200,
            half_life_s: 3600,
            peak_time_s: 1200,
        }],
        toxicity: ToxicityClass::Benign,
    };
    let payload = encode_to_vec(&report).expect("encode failed");
    let wrapped = wrap_with_checksum(&payload);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (ToxicologyReport, _) = decode_from_slice(&recovered).expect("decode failed");
    assert_eq!(decoded.toxicity, ToxicityClass::Benign);
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: wrap/unwrap idempotency — double-wrap then double-unwrap
// ---------------------------------------------------------------------------
#[test]
fn test_wrap_unwrap_idempotency() {
    let dose = CompoundDose {
        compound_id: 200,
        name: "Metformin".to_string(),
        dose_ug: 1_000_000,
        route: AdministrationRoute::Oral,
        administered_at: 1_705_000_000,
    };
    let payload = encode_to_vec(&dose).expect("encode failed");

    // First wrap/unwrap cycle
    let wrapped_once = wrap_with_checksum(&payload);
    let recovered_once = unwrap_with_checksum(&wrapped_once).expect("first unwrap failed");

    // Second wrap/unwrap cycle on the recovered payload
    let wrapped_twice = wrap_with_checksum(&recovered_once);
    let recovered_twice = unwrap_with_checksum(&wrapped_twice).expect("second unwrap failed");

    assert_eq!(
        payload, recovered_twice,
        "double wrap/unwrap must yield the original payload"
    );

    let (decoded, _): (CompoundDose, _) =
        decode_from_slice(&recovered_twice).expect("final decode failed");
    assert_eq!(dose, decoded);
}
