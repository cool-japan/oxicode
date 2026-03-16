//! Tests for healthcare / medical-records enums and structs — advanced enum roundtrip coverage.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

#[derive(Debug, PartialEq, Encode, Decode)]
enum BloodType {
    APos,
    ANeg,
    BPos,
    BNeg,
    ABPos,
    ABNeg,
    OPos,
    ONeg,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Gender {
    Male,
    Female,
    NonBinary,
    PreferNotToSay,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DiagnosisCode {
    icd_code: String,
    description: String,
    severity: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MedicalRecord {
    patient_id: u64,
    blood_type: BloodType,
    gender: Gender,
    diagnoses: Vec<DiagnosisCode>,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Prescription {
    medication: String,
    dosage_mg: u32,
    duration_days: u16,
    refills: u8,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_diagnosis(icd: &str, desc: &str, severity: u8) -> DiagnosisCode {
    DiagnosisCode {
        icd_code: icd.to_string(),
        description: desc.to_string(),
        severity,
    }
}

fn make_record_with_notes(
    patient_id: u64,
    blood_type: BloodType,
    gender: Gender,
    notes: Option<String>,
) -> MedicalRecord {
    MedicalRecord {
        patient_id,
        blood_type,
        gender,
        diagnoses: vec![make_diagnosis(
            "J06.9",
            "Acute upper respiratory infection",
            2,
        )],
        notes,
    }
}

// ── test 1: BloodType all 8 variants roundtrip and pairwise distinct ──────────

#[test]
fn test_blood_type_all_variants_roundtrip_and_differ() {
    let variants = [
        BloodType::APos,
        BloodType::ANeg,
        BloodType::BPos,
        BloodType::BNeg,
        BloodType::ABPos,
        BloodType::ABNeg,
        BloodType::OPos,
        BloodType::ONeg,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode BloodType variant"))
        .collect();

    // Pairwise uniqueness — each discriminant must be distinct
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "BloodType variants {i} and {j} must differ"
            );
        }
    }

    // Roundtrip each encoding back to the original variant
    let expected = [
        BloodType::APos,
        BloodType::ANeg,
        BloodType::BPos,
        BloodType::BNeg,
        BloodType::ABPos,
        BloodType::ABNeg,
        BloodType::OPos,
        BloodType::ONeg,
    ];
    for (bytes, exp) in encodings.iter().zip(expected.iter()) {
        let (decoded, _): (BloodType, usize) =
            decode_from_slice(bytes).expect("decode BloodType variant");
        assert_eq!(&decoded, exp);
    }
}

// ── test 2: Gender all variants roundtrip ─────────────────────────────────────

#[test]
fn test_gender_all_variants_roundtrip() {
    let variants = [
        Gender::Male,
        Gender::Female,
        Gender::NonBinary,
        Gender::PreferNotToSay,
    ];

    for variant in &variants {
        let bytes = encode_to_vec(variant).expect("encode Gender");
        let (decoded, consumed): (Gender, usize) =
            decode_from_slice(&bytes).expect("decode Gender");
        assert_eq!(variant, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for Gender"
        );
    }
}

// ── test 3: Gender discriminant uniqueness ────────────────────────────────────

#[test]
fn test_gender_discriminant_uniqueness() {
    let variants = [
        Gender::Male,
        Gender::Female,
        Gender::NonBinary,
        Gender::PreferNotToSay,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode Gender for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "Gender variants {i} and {j} must differ"
            );
        }
    }
}

// ── test 4: DiagnosisCode struct roundtrip ────────────────────────────────────

#[test]
fn test_diagnosis_code_roundtrip() {
    let val = make_diagnosis("E11.9", "Type 2 diabetes mellitus without complications", 5);
    let bytes = encode_to_vec(&val).expect("encode DiagnosisCode");
    let (decoded, _): (DiagnosisCode, usize) =
        decode_from_slice(&bytes).expect("decode DiagnosisCode");
    assert_eq!(val, decoded);
}

// ── test 5: DiagnosisCode with minimal (empty) fields ─────────────────────────

#[test]
fn test_diagnosis_code_empty_strings_roundtrip() {
    let val = DiagnosisCode {
        icd_code: String::new(),
        description: String::new(),
        severity: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode empty DiagnosisCode");
    let (decoded, consumed): (DiagnosisCode, usize) =
        decode_from_slice(&bytes).expect("decode empty DiagnosisCode");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// ── test 6: Prescription roundtrip ───────────────────────────────────────────

#[test]
fn test_prescription_roundtrip() {
    let val = Prescription {
        medication: "Metformin".to_string(),
        dosage_mg: 500,
        duration_days: 90,
        refills: 3,
    };
    let bytes = encode_to_vec(&val).expect("encode Prescription");
    let (decoded, _): (Prescription, usize) =
        decode_from_slice(&bytes).expect("decode Prescription");
    assert_eq!(val, decoded);
}

// ── test 7: Prescription with zero values ─────────────────────────────────────

#[test]
fn test_prescription_zero_values_roundtrip() {
    let val = Prescription {
        medication: "Placebo".to_string(),
        dosage_mg: 0,
        duration_days: 0,
        refills: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode zero Prescription");
    let (decoded, consumed): (Prescription, usize) =
        decode_from_slice(&bytes).expect("decode zero Prescription");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// ── test 8: MedicalRecord with Option::Some notes roundtrip ──────────────────

#[test]
fn test_medical_record_with_some_notes_roundtrip() {
    let val = make_record_with_notes(
        100001,
        BloodType::OPos,
        Gender::Female,
        Some("Patient reports mild fatigue after meals.".to_string()),
    );
    let bytes = encode_to_vec(&val).expect("encode MedicalRecord Some notes");
    let (decoded, _): (MedicalRecord, usize) =
        decode_from_slice(&bytes).expect("decode MedicalRecord Some notes");
    assert_eq!(val, decoded);
}

// ── test 9: MedicalRecord with Option::None notes roundtrip ──────────────────

#[test]
fn test_medical_record_with_none_notes_roundtrip() {
    let val = make_record_with_notes(100002, BloodType::ABNeg, Gender::Male, None);
    let bytes = encode_to_vec(&val).expect("encode MedicalRecord None notes");
    let (decoded, _): (MedicalRecord, usize) =
        decode_from_slice(&bytes).expect("decode MedicalRecord None notes");
    assert_eq!(val, decoded);
}

// ── test 10: Some vs None notes must encode differently ───────────────────────

#[test]
fn test_medical_record_some_none_notes_differ() {
    let with_notes = make_record_with_notes(
        200000,
        BloodType::BPos,
        Gender::NonBinary,
        Some("Follow-up in 2 weeks.".to_string()),
    );
    let without_notes = make_record_with_notes(200000, BloodType::BPos, Gender::NonBinary, None);

    let bytes_some = encode_to_vec(&with_notes).expect("encode with notes");
    let bytes_none = encode_to_vec(&without_notes).expect("encode without notes");
    assert_ne!(
        bytes_some, bytes_none,
        "Some and None notes must yield different encodings"
    );
}

// ── test 11: MedicalRecord with empty diagnoses Vec ───────────────────────────

#[test]
fn test_medical_record_empty_diagnoses_roundtrip() {
    let val = MedicalRecord {
        patient_id: 300001,
        blood_type: BloodType::ANeg,
        gender: Gender::PreferNotToSay,
        diagnoses: vec![],
        notes: None,
    };
    let bytes = encode_to_vec(&val).expect("encode MedicalRecord empty diagnoses");
    let (decoded, consumed): (MedicalRecord, usize) =
        decode_from_slice(&bytes).expect("decode MedicalRecord empty diagnoses");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}

// ── test 12: MedicalRecord with multiple diagnoses ────────────────────────────

#[test]
fn test_medical_record_multiple_diagnoses_roundtrip() {
    let val = MedicalRecord {
        patient_id: 400001,
        blood_type: BloodType::ABPos,
        gender: Gender::Female,
        diagnoses: vec![
            make_diagnosis("I10", "Essential (primary) hypertension", 4),
            make_diagnosis("E11.9", "Type 2 diabetes mellitus without complications", 5),
            make_diagnosis("M54.5", "Low back pain", 2),
            make_diagnosis("J45.909", "Unspecified asthma, uncomplicated", 3),
        ],
        notes: Some("Patient is on multiple medications; monitor interactions.".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode MedicalRecord multiple diagnoses");
    let (decoded, _): (MedicalRecord, usize) =
        decode_from_slice(&bytes).expect("decode MedicalRecord multiple diagnoses");
    assert_eq!(val, decoded);
}

// ── test 13: Vec<MedicalRecord> roundtrip ─────────────────────────────────────

#[test]
fn test_vec_medical_record_roundtrip() {
    let val: Vec<MedicalRecord> = vec![
        make_record_with_notes(500001, BloodType::OPos, Gender::Male, None),
        make_record_with_notes(
            500002,
            BloodType::BNeg,
            Gender::Female,
            Some("Allergy to penicillin.".to_string()),
        ),
        MedicalRecord {
            patient_id: 500003,
            blood_type: BloodType::ABNeg,
            gender: Gender::NonBinary,
            diagnoses: vec![make_diagnosis("F41.1", "Generalized anxiety disorder", 3)],
            notes: None,
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<MedicalRecord>");
    let (decoded, _): (Vec<MedicalRecord>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<MedicalRecord>");
    assert_eq!(val, decoded);
}

// ── test 14: Vec<Prescription> roundtrip ──────────────────────────────────────

#[test]
fn test_vec_prescription_roundtrip() {
    let val: Vec<Prescription> = vec![
        Prescription {
            medication: "Lisinopril".to_string(),
            dosage_mg: 10,
            duration_days: 365,
            refills: 11,
        },
        Prescription {
            medication: "Atorvastatin".to_string(),
            dosage_mg: 40,
            duration_days: 180,
            refills: 5,
        },
        Prescription {
            medication: "Metformin".to_string(),
            dosage_mg: 1000,
            duration_days: 90,
            refills: 3,
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Prescription>");
    let (decoded, _): (Vec<Prescription>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Prescription>");
    assert_eq!(val, decoded);
}

// ── test 15: consumed bytes equals encoded length for MedicalRecord ───────────

#[test]
fn test_medical_record_consumed_bytes_equals_encoded_length() {
    let val = MedicalRecord {
        patient_id: 600001,
        blood_type: BloodType::OPos,
        gender: Gender::Male,
        diagnoses: vec![make_diagnosis(
            "Z00.00",
            "Encounter for general adult exam",
            1,
        )],
        notes: Some("Annual physical; no concerns.".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode MedicalRecord for length check");
    let (decoded, consumed): (MedicalRecord, usize) =
        decode_from_slice(&bytes).expect("decode MedicalRecord for length check");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ── test 16: consumed bytes equals encoded length for Prescription ────────────

#[test]
fn test_prescription_consumed_bytes_equals_encoded_length() {
    let val = Prescription {
        medication: "Amoxicillin".to_string(),
        dosage_mg: 500,
        duration_days: 10,
        refills: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode Prescription for length check");
    let (_, consumed): (Prescription, usize) =
        decode_from_slice(&bytes).expect("decode Prescription for length check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ── test 17: big-endian config MedicalRecord roundtrip ───────────────────────

#[test]
fn test_medical_record_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = MedicalRecord {
        patient_id: 700001,
        blood_type: BloodType::ABPos,
        gender: Gender::Female,
        diagnoses: vec![make_diagnosis("K21.0", "GERD with esophagitis", 3)],
        notes: Some("Recommend dietary changes.".to_string()),
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode MedicalRecord big-endian");
    let (decoded, _): (MedicalRecord, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode MedicalRecord big-endian");
    assert_eq!(val, decoded);
}

// ── test 18: big-endian config Prescription roundtrip ────────────────────────

#[test]
fn test_prescription_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = Prescription {
        medication: "Ibuprofen".to_string(),
        dosage_mg: 400,
        duration_days: 7,
        refills: 1,
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Prescription big-endian");
    let (decoded, _): (Prescription, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Prescription big-endian");
    assert_eq!(val, decoded);
}

// ── test 19: fixed-int config MedicalRecord roundtrip ────────────────────────

#[test]
fn test_medical_record_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = MedicalRecord {
        patient_id: 800001,
        blood_type: BloodType::ONeg,
        gender: Gender::PreferNotToSay,
        diagnoses: vec![make_diagnosis(
            "C34.10",
            "Malignant neoplasm of upper lobe bronchus",
            9,
        )],
        notes: None,
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode MedicalRecord fixed-int");
    let (decoded, _): (MedicalRecord, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode MedicalRecord fixed-int");
    assert_eq!(val, decoded);
}

// ── test 20: fixed-int config Prescription roundtrip ─────────────────────────

#[test]
fn test_prescription_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = Prescription {
        medication: "Warfarin".to_string(),
        dosage_mg: 5,
        duration_days: 365,
        refills: 11,
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Prescription fixed-int");
    let (decoded, _): (Prescription, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Prescription fixed-int");
    assert_eq!(val, decoded);
}

// ── test 21: different BloodTypes in otherwise identical records encode differently

#[test]
fn test_different_blood_types_encode_differently() {
    let base = |bt: BloodType| MedicalRecord {
        patient_id: 999001,
        blood_type: bt,
        gender: Gender::Male,
        diagnoses: vec![],
        notes: None,
    };

    let bytes_apos = encode_to_vec(&base(BloodType::APos)).expect("encode APos record");
    let bytes_bpos = encode_to_vec(&base(BloodType::BPos)).expect("encode BPos record");
    let bytes_oneg = encode_to_vec(&base(BloodType::ONeg)).expect("encode ONeg record");

    assert_ne!(
        bytes_apos, bytes_bpos,
        "APos and BPos blood types must differ"
    );
    assert_ne!(
        bytes_apos, bytes_oneg,
        "APos and ONeg blood types must differ"
    );
    assert_ne!(
        bytes_bpos, bytes_oneg,
        "BPos and ONeg blood types must differ"
    );
}

// ── test 22: nested Vec<DiagnosisCode> content is faithfully preserved ────────

#[test]
fn test_medical_record_diagnosis_vec_content_preserved() {
    let diagnoses = vec![
        make_diagnosis("A00.0", "Cholera due to Vibrio cholerae 01", 8),
        make_diagnosis("B34.9", "Viral infection, unspecified", 3),
        make_diagnosis(
            "C50.911",
            "Malignant neoplasm of unspecified site of right female breast",
            9,
        ),
        make_diagnosis("D50.0", "Iron deficiency anemia secondary to blood loss", 4),
        make_diagnosis(
            "E66.01",
            "Morbid (severe) obesity due to excess calories",
            6,
        ),
    ];
    let val = MedicalRecord {
        patient_id: 1_000_001,
        blood_type: BloodType::ABPos,
        gender: Gender::Female,
        diagnoses,
        notes: Some("Complex case — multidisciplinary team review recommended.".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode MedicalRecord diagnosis content");
    let (decoded, consumed): (MedicalRecord, usize) =
        decode_from_slice(&bytes).expect("decode MedicalRecord diagnosis content");
    assert_eq!(val, decoded);
    assert_eq!(
        decoded.diagnoses.len(),
        5,
        "decoded diagnoses Vec must contain 5 entries"
    );
    assert_eq!(decoded.diagnoses[0].icd_code, "A00.0");
    assert_eq!(decoded.diagnoses[4].severity, 6);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}
