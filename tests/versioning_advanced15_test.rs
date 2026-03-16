//! Healthcare / Electronic Health Records versioning tests for OxiCode (set 15).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and three EHR domain structs (PatientRecord, Diagnosis, Prescription)
//! with various version tags, field verification, version comparison, consumed bytes
//! accounting, multiple versions of the same type, and plain encode/decode baseline.

#![cfg(feature = "versioning")]
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
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value,
    versioning::Version, Decode, Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PatientRecord {
    patient_id: u64,
    name: String,
    age: u8,
    blood_type: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Diagnosis {
    diagnosis_id: u64,
    patient_id: u64,
    code: String,
    severity: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Prescription {
    rx_id: u64,
    patient_id: u64,
    medication: String,
    dosage_mg: u32,
    days: u16,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// PatientRecord encode/decode roundtrip at version 1.0.0
#[test]
fn test_patient_record_roundtrip_v1_0_0() {
    let version = Version::new(1, 0, 0);
    let original = PatientRecord {
        patient_id: 10000001,
        name: String::from("Alice Nguyen"),
        age: 34,
        blood_type: 2, // e.g. 2 = A+
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (PatientRecord, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// PatientRecord encode/decode roundtrip at version 2.0.0
#[test]
fn test_patient_record_roundtrip_v2_0_0() {
    let version = Version::new(2, 0, 0);
    let original = PatientRecord {
        patient_id: 10000002,
        name: String::from("Bob Martinez"),
        age: 57,
        blood_type: 5, // e.g. 5 = B-
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (PatientRecord, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(ver.major, 2);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// Diagnosis versioned roundtrip at version 1.0.0
#[test]
fn test_diagnosis_roundtrip_v1_0_0() {
    let version = Version::new(1, 0, 0);
    let original = Diagnosis {
        diagnosis_id: 20000001,
        patient_id: 10000001,
        code: String::from("J18.9"),
        severity: 3,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (Diagnosis, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// Prescription versioned roundtrip at version 1.0.0
#[test]
fn test_prescription_roundtrip_v1_0_0() {
    let version = Version::new(1, 0, 0);
    let original = Prescription {
        rx_id: 30000001,
        patient_id: 10000001,
        medication: String::from("Amoxicillin"),
        dosage_mg: 500,
        days: 10,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (Prescription, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// Version field verification: major, minor, patch are preserved exactly
#[test]
fn test_version_field_verification_after_decode() {
    let version = Version::new(3, 7, 12);
    let original = PatientRecord {
        patient_id: 10000005,
        name: String::from("Carol Smith"),
        age: 45,
        blood_type: 0, // O+
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (PatientRecord, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 12);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// Version comparison — major ordering: 1.0.0 < 2.0.0 < 3.0.0
#[test]
fn test_version_major_comparison() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);
    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v1 < v3);
    assert_ne!(v1, v2);
    assert_ne!(v2, v3);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// Version comparison — minor ordering within same major
#[test]
fn test_version_minor_comparison() {
    let v_low = Version::new(1, 0, 0);
    let v_mid = Version::new(1, 5, 0);
    let v_high = Version::new(1, 10, 0);
    assert!(v_low < v_mid);
    assert!(v_mid < v_high);
    assert!(v_low < v_high);
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// Version comparison — patch ordering within same major/minor
#[test]
fn test_version_patch_comparison() {
    let v_base = Version::new(2, 3, 0);
    let v_patched = Version::new(2, 3, 9);
    assert!(v_base < v_patched);
    assert!(v_patched > v_base);
    assert_ne!(v_base, v_patched);
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// Consumed bytes check: positive and within total encoded length
#[test]
fn test_consumed_bytes_within_bounds() {
    let version = Version::new(1, 0, 0);
    let original = Diagnosis {
        diagnosis_id: 20000009,
        patient_id: 10000009,
        code: String::from("E11.9"),
        severity: 5,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let total_len = encoded.len();
    let (_decoded, _ver, consumed): (Diagnosis, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert!(consumed > 0, "consumed must be positive");
    assert!(
        consumed <= total_len,
        "consumed ({consumed}) must not exceed total encoded length ({total_len})"
    );
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// Multiple versions of PatientRecord: same data, different version tags
#[test]
fn test_patient_record_multiple_version_tags() {
    let v_old = Version::new(1, 0, 0);
    let v_new = Version::new(2, 1, 3);
    let patient = PatientRecord {
        patient_id: 10000010,
        name: String::from("David Lee"),
        age: 62,
        blood_type: 7, // AB-
    };

    let encoded_old =
        encode_versioned_value(&patient, v_old).expect("encode_versioned_value v_old failed");
    let encoded_new =
        encode_versioned_value(&patient, v_new).expect("encode_versioned_value v_new failed");

    let (decoded1, ver1, _): (PatientRecord, Version, usize) =
        decode_versioned_value(&encoded_old).expect("decode v_old failed");
    let (decoded2, ver2, _): (PatientRecord, Version, usize) =
        decode_versioned_value(&encoded_new).expect("decode v_new failed");

    assert_eq!(decoded1, patient);
    assert_eq!(decoded2, patient);
    assert_eq!(ver1, v_old);
    assert_eq!(ver2, v_new);
    assert_ne!(ver1, ver2);
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// Multiple versions of Diagnosis: same data tagged at v1.0.0 and v1.5.0
#[test]
fn test_diagnosis_multiple_version_tags() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 5, 0);
    let diag = Diagnosis {
        diagnosis_id: 20000011,
        patient_id: 10000011,
        code: String::from("I10"),
        severity: 4,
    };

    let enc1 = encode_versioned_value(&diag, v1).expect("encode v1 failed");
    let enc2 = encode_versioned_value(&diag, v2).expect("encode v2 failed");

    let (d1, ver1, _): (Diagnosis, Version, usize) =
        decode_versioned_value(&enc1).expect("decode v1 failed");
    let (d2, ver2, _): (Diagnosis, Version, usize) =
        decode_versioned_value(&enc2).expect("decode v2 failed");

    assert_eq!(d1, diag);
    assert_eq!(d2, diag);
    assert_eq!(ver1, v1);
    assert_eq!(ver2, v2);
    assert_ne!(ver1, ver2);
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// Multiple versions of Prescription: same data tagged at v2.0.0 and v3.0.0
#[test]
fn test_prescription_multiple_version_tags() {
    let v_a = Version::new(2, 0, 0);
    let v_b = Version::new(3, 0, 0);
    let rx = Prescription {
        rx_id: 30000012,
        patient_id: 10000012,
        medication: String::from("Metformin"),
        dosage_mg: 1000,
        days: 90,
    };

    let enc_a = encode_versioned_value(&rx, v_a).expect("encode v_a failed");
    let enc_b = encode_versioned_value(&rx, v_b).expect("encode v_b failed");

    let (rx_a, ver_a, _): (Prescription, Version, usize) =
        decode_versioned_value(&enc_a).expect("decode v_a failed");
    let (rx_b, ver_b, _): (Prescription, Version, usize) =
        decode_versioned_value(&enc_b).expect("decode v_b failed");

    assert_eq!(rx_a, rx);
    assert_eq!(rx_b, rx);
    assert_eq!(ver_a, v_a);
    assert_eq!(ver_b, v_b);
    assert_ne!(ver_a, ver_b);
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// PatientRecord with long name and edge-case age (newborn, age=0)
#[test]
fn test_patient_record_newborn_edge_case() {
    let version = Version::new(1, 0, 0);
    let original = PatientRecord {
        patient_id: 10000013,
        name: String::from("Baby Doe-Johnson-Williams-Hernandez"),
        age: 0,
        blood_type: 1, // A+
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (PatientRecord, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.age, 0);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// PatientRecord with maximum age (elderly, age=255)
#[test]
fn test_patient_record_max_age_edge_case() {
    let version = Version::new(1, 2, 0);
    let original = PatientRecord {
        patient_id: 10000014,
        name: String::from("Centenarian Patient"),
        age: 255,
        blood_type: 6, // AB+
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (PatientRecord, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.age, 255);
    assert_eq!(ver.minor, 2);
    assert!(consumed > 0);
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// Diagnosis with ICD-10 code containing special characters at version 2.0.0
#[test]
fn test_diagnosis_icd10_code_roundtrip_v2() {
    let version = Version::new(2, 0, 0);
    let original = Diagnosis {
        diagnosis_id: 20000015,
        patient_id: 10000015,
        code: String::from("Z00.00"),
        severity: 1,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (Diagnosis, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.code, "Z00.00");
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// Prescription with high dosage and long treatment at version 1.1.0
#[test]
fn test_prescription_high_dosage_roundtrip() {
    let version = Version::new(1, 1, 0);
    let original = Prescription {
        rx_id: 30000016,
        patient_id: 10000016,
        medication: String::from("Vancomycin"),
        dosage_mg: 2000,
        days: 365,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (Prescription, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.dosage_mg, 2000);
    assert_eq!(decoded.days, 365);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Prescription with minimum dosage (1 mg) and single day treatment
#[test]
fn test_prescription_minimum_dosage_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = Prescription {
        rx_id: 30000017,
        patient_id: 10000017,
        medication: String::from("Folic Acid"),
        dosage_mg: 1,
        days: 1,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (Prescription, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.dosage_mg, 1);
    assert_eq!(decoded.days, 1);
    assert_eq!(ver, version);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// Version equality check: identical version objects compare equal
#[test]
fn test_version_equality_check() {
    let v_a = Version::new(4, 15, 200);
    let v_b = Version::new(4, 15, 200);
    assert_eq!(v_a, v_b);
    assert!(!(v_a < v_b));
    assert!(!(v_a > v_b));
    assert!(v_a <= v_b);
    assert!(v_a >= v_b);
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// PatientRecord plain encode/decode baseline (no versioning wrapper)
#[test]
fn test_patient_record_plain_encode_decode_baseline() {
    let original = PatientRecord {
        patient_id: 10000019,
        name: String::from("Eve Thompson"),
        age: 29,
        blood_type: 3, // B+
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (PatientRecord, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// Diagnosis plain encode/decode baseline (no versioning wrapper)
#[test]
fn test_diagnosis_plain_encode_decode_baseline() {
    let original = Diagnosis {
        diagnosis_id: 20000020,
        patient_id: 10000020,
        code: String::from("M54.5"),
        severity: 2,
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (Diagnosis, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.code, "M54.5");
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// Prescription plain encode/decode baseline (no versioning wrapper)
#[test]
fn test_prescription_plain_encode_decode_baseline() {
    let original = Prescription {
        rx_id: 30000021,
        patient_id: 10000021,
        medication: String::from("Lisinopril"),
        dosage_mg: 10,
        days: 30,
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (Prescription, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.medication, "Lisinopril");
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// Batch of PatientRecords, each versioned independently, all decode correctly
#[test]
fn test_batch_patient_records_versioned_independently() {
    let version = Version::new(1, 0, 0);
    let records = vec![
        PatientRecord {
            patient_id: 10000022,
            name: String::from("Frank Okafor"),
            age: 38,
            blood_type: 4,
        },
        PatientRecord {
            patient_id: 10000023,
            name: String::from("Grace Kimura"),
            age: 51,
            blood_type: 0,
        },
        PatientRecord {
            patient_id: 10000024,
            name: String::from("Hector Ramirez"),
            age: 22,
            blood_type: 6,
        },
    ];

    for original in &records {
        let encoded =
            encode_versioned_value(original, version).expect("encode_versioned_value failed");
        let (decoded, ver, consumed): (PatientRecord, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(&decoded, original);
        assert_eq!(ver, version);
        assert!(consumed > 0);
    }
}
