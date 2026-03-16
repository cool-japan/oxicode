//! Advanced validation tests — set 8: Medical data validation.
//! 22 top-level #[test] functions, no module wrapper, no #[cfg(test)].

#![cfg(feature = "validation")]
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
use oxicode::validation::{
    CollectionValidator, Constraints, NumericValidator, ValidationError, Validator,
};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types used throughout this test file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct PatientRecord {
    patient_id: u64,
    age: u8,           // must be 0..=150
    weight_kg: f32,    // must be > 0.0
    height_cm: u32,    // must be 1..=300
    diagnosis: String, // must not be empty
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BloodType {
    A,
    B,
    AB,
    O,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MedicalTest {
    test_id: u64,
    patient_id: u64,
    test_name: String,
    result_value: f64,
    reference_min: f64,
    reference_max: f64,
    blood_type: BloodType,
}

// ---------------------------------------------------------------------------
// 1. PatientRecord encode → decode roundtrip preserves all fields
// ---------------------------------------------------------------------------
#[test]
fn test_patient_record_encode_decode_roundtrip_preserves_all_fields() {
    let record = PatientRecord {
        patient_id: 1001,
        age: 45,
        weight_kg: 72.5,
        height_cm: 175,
        diagnosis: "Type 2 Diabetes".to_string(),
    };

    let bytes = encode_to_vec(&record).expect("encode PatientRecord must succeed");
    let (decoded, bytes_read): (PatientRecord, usize) =
        decode_from_slice(&bytes).expect("decode PatientRecord must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, record,
        "roundtrip must preserve PatientRecord fields"
    );
}

// ---------------------------------------------------------------------------
// 2. MedicalTest encode → decode roundtrip preserves all fields
// ---------------------------------------------------------------------------
#[test]
fn test_medical_test_encode_decode_roundtrip_preserves_all_fields() {
    let test = MedicalTest {
        test_id: 5001,
        patient_id: 1001,
        test_name: "Fasting Blood Glucose".to_string(),
        result_value: 5.6,
        reference_min: 3.9,
        reference_max: 5.5,
        blood_type: BloodType::A,
    };

    let bytes = encode_to_vec(&test).expect("encode MedicalTest must succeed");
    let (decoded, bytes_read): (MedicalTest, usize) =
        decode_from_slice(&bytes).expect("decode MedicalTest must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(decoded, test, "roundtrip must preserve MedicalTest fields");
}

// ---------------------------------------------------------------------------
// 3. BloodType all variants roundtrip correctly
// ---------------------------------------------------------------------------
#[test]
fn test_blood_type_all_variants_roundtrip_correctly() {
    let variants = [BloodType::A, BloodType::B, BloodType::AB, BloodType::O];

    for variant in variants {
        let bytes = encode_to_vec(&variant).expect("encode BloodType must succeed");
        let (decoded, bytes_read): (BloodType, usize) =
            decode_from_slice(&bytes).expect("decode BloodType must succeed");
        assert!(
            bytes_read > 0,
            "must consume at least one byte for BloodType"
        );
        assert_eq!(decoded, variant, "BloodType roundtrip must match original");
    }
}

// ---------------------------------------------------------------------------
// 4. age validator accepts valid range 0..=150
// ---------------------------------------------------------------------------
#[test]
fn test_age_validator_accepts_valid_range_0_to_150() {
    let v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(0u8), Some(150u8)));

    assert!(v.validate(&0u8).is_ok(), "age 0 must pass [0, 150]");
    assert!(v.validate(&75u8).is_ok(), "age 75 must pass [0, 150]");
    assert!(v.validate(&150u8).is_ok(), "age 150 must pass [0, 150]");
}

// ---------------------------------------------------------------------------
// 5. age validator rejects value above 150
// ---------------------------------------------------------------------------
#[test]
fn test_age_validator_rejects_value_above_150() {
    let v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(0u8), Some(150u8)));

    assert!(v.validate(&151u8).is_err(), "age 151 must fail [0, 150]");
    assert!(v.validate(&255u8).is_err(), "age 255 must fail [0, 150]");
}

// ---------------------------------------------------------------------------
// 6. weight_kg custom validator rejects zero and negative values
// ---------------------------------------------------------------------------
#[test]
fn test_weight_kg_custom_validator_rejects_zero_and_negative_values() {
    let v: Validator<f32> = Validator::new().constraint(
        "weight_kg",
        Constraints::custom(
            |w: &f32| *w > 0.0,
            "weight_kg must be greater than zero",
            "positive-weight",
        ),
    );

    assert!(
        v.validate(&0.1f32).is_ok(),
        "weight 0.1 kg must pass positive check"
    );
    assert!(
        v.validate(&72.5f32).is_ok(),
        "weight 72.5 kg must pass positive check"
    );
    assert!(
        v.validate(&0.0f32).is_err(),
        "weight 0.0 kg must fail positive check"
    );
    assert!(
        v.validate(&(-1.0f32)).is_err(),
        "negative weight must fail positive check"
    );
}

// ---------------------------------------------------------------------------
// 7. height_cm validator enforces range 1..=300
// ---------------------------------------------------------------------------
#[test]
fn test_height_cm_validator_enforces_range_1_to_300() {
    let v: Validator<u32> =
        Validator::new().constraint("height_cm", Constraints::range(Some(1u32), Some(300u32)));

    assert!(v.validate(&1u32).is_ok(), "height 1 cm must pass [1, 300]");
    assert!(
        v.validate(&170u32).is_ok(),
        "height 170 cm must pass [1, 300]"
    );
    assert!(
        v.validate(&300u32).is_ok(),
        "height 300 cm must pass [1, 300]"
    );
    assert!(v.validate(&0u32).is_err(), "height 0 cm must fail [1, 300]");
    assert!(
        v.validate(&301u32).is_err(),
        "height 301 cm must fail [1, 300]"
    );
}

// ---------------------------------------------------------------------------
// 8. diagnosis non_empty validator passes for non-empty string
// ---------------------------------------------------------------------------
#[test]
fn test_diagnosis_non_empty_validator_passes_for_non_empty_string() {
    let v: Validator<String> = Validator::new().constraint("diagnosis", Constraints::non_empty());

    assert!(
        v.validate(&"Hypertension".to_string()).is_ok(),
        "non-empty diagnosis must pass non_empty"
    );
    assert!(
        v.validate(&"Unknown".to_string()).is_ok(),
        "'Unknown' diagnosis must pass non_empty"
    );
}

// ---------------------------------------------------------------------------
// 9. diagnosis non_empty validator fails for empty string
// ---------------------------------------------------------------------------
#[test]
fn test_diagnosis_non_empty_validator_fails_for_empty_string() {
    let v: Validator<String> = Validator::new().constraint("diagnosis", Constraints::non_empty());

    assert!(
        v.validate(&String::new()).is_err(),
        "empty diagnosis must fail non_empty"
    );
}

// ---------------------------------------------------------------------------
// 10. test_name validator enforces min_len and max_len
// ---------------------------------------------------------------------------
#[test]
fn test_test_name_validator_enforces_min_len_and_max_len() {
    let v: Validator<String> = Validator::new()
        .constraint("test_name", Constraints::min_len(3))
        .constraint("test_name", Constraints::max_len(128));

    assert!(
        v.validate(&"CBC".to_string()).is_ok(),
        "3-char test name must pass min_len(3)"
    );
    assert!(
        v.validate(&"Complete Blood Count".to_string()).is_ok(),
        "standard test name must pass [3, 128]"
    );
    assert!(
        v.validate(&"AB".to_string()).is_err(),
        "2-char test name must fail min_len(3)"
    );
    let too_long = "X".repeat(129);
    assert!(
        v.validate(&too_long).is_err(),
        "129-char test name must fail max_len(128)"
    );
}

// ---------------------------------------------------------------------------
// 11. result_value range validator for blood glucose reference range
// ---------------------------------------------------------------------------
#[test]
fn test_result_value_range_validator_for_blood_glucose_reference_range() {
    // Normal fasting glucose: 3.9 – 5.5 mmol/L
    let v: Validator<f64> = Validator::new().constraint(
        "result_value",
        Constraints::range(Some(0.0f64), Some(50.0f64)),
    );

    assert!(
        v.validate(&5.6f64).is_ok(),
        "glucose 5.6 must pass physiological range"
    );
    assert!(
        v.validate(&0.0f64).is_ok(),
        "glucose 0.0 (boundary) must pass"
    );
    assert!(
        v.validate(&(-0.1f64)).is_err(),
        "negative glucose must fail"
    );
    assert!(
        v.validate(&50.1f64).is_err(),
        "glucose 50.1 exceeds physiological maximum"
    );
}

// ---------------------------------------------------------------------------
// 12. Validator::validate_or_default returns value when age is valid
// ---------------------------------------------------------------------------
#[test]
fn test_validator_validate_or_default_returns_value_when_age_is_valid() {
    let v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(0u8), Some(150u8)));

    assert_eq!(
        v.validate_or_default(42u8, 0u8),
        42u8,
        "valid age 42 must be returned unchanged"
    );
    assert_eq!(
        v.validate_or_default(0u8, 99u8),
        0u8,
        "boundary age 0 must be returned unchanged"
    );
}

// ---------------------------------------------------------------------------
// 13. Validator::validate_or_default returns default when age is invalid
// ---------------------------------------------------------------------------
#[test]
fn test_validator_validate_or_default_returns_default_when_age_is_invalid() {
    let v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(0u8), Some(150u8)));

    assert_eq!(
        v.validate_or_default(255u8, 0u8),
        0u8,
        "age 255 must return default 0"
    );
    assert_eq!(
        v.validate_or_default(200u8, 0u8),
        0u8,
        "age 200 must return default 0"
    );
}

// ---------------------------------------------------------------------------
// 14. Validator::validate_or_default_with uses closure only on failure
// ---------------------------------------------------------------------------
#[test]
fn test_validator_validate_or_default_with_uses_closure_only_on_failure() {
    let v: Validator<f64> = Validator::new().constraint(
        "result_value",
        Constraints::range(Some(0.0f64), Some(50.0f64)),
    );

    let mut closure_called = false;
    let result = v.validate_or_default_with(&5.6f64, || {
        closure_called = true;
        -1.0f64
    });
    assert_eq!(result, 5.6f64, "valid result must be returned unchanged");
    assert!(
        !closure_called,
        "default closure must not be called when value is valid"
    );

    let result_invalid = v.validate_or_default_with(&(-0.5f64), || 0.0f64);
    assert_eq!(
        result_invalid, 0.0f64,
        "invalid result must return closure value"
    );
}

// ---------------------------------------------------------------------------
// 15. Vec<PatientRecord> encode → decode roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_patient_record_encode_decode_roundtrip() {
    let records: Vec<PatientRecord> = vec![
        PatientRecord {
            patient_id: 1,
            age: 30,
            weight_kg: 65.0,
            height_cm: 168,
            diagnosis: "Asthma".to_string(),
        },
        PatientRecord {
            patient_id: 2,
            age: 55,
            weight_kg: 85.5,
            height_cm: 180,
            diagnosis: "Hypertension".to_string(),
        },
        PatientRecord {
            patient_id: 3,
            age: 7,
            weight_kg: 22.0,
            height_cm: 120,
            diagnosis: "Fever".to_string(),
        },
    ];

    let bytes = encode_to_vec(&records).expect("encode Vec<PatientRecord> must succeed");
    let (decoded, bytes_read): (Vec<PatientRecord>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<PatientRecord> must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(decoded.len(), 3, "decoded Vec must have 3 elements");
    assert_eq!(
        decoded, records,
        "Vec<PatientRecord> roundtrip must preserve all records"
    );
}

// ---------------------------------------------------------------------------
// 16. Option<MedicalTest> Some variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_medical_test_some_variant_roundtrip() {
    let test: Option<MedicalTest> = Some(MedicalTest {
        test_id: 9001,
        patient_id: 2002,
        test_name: "Hemoglobin A1c".to_string(),
        result_value: 6.2,
        reference_min: 4.0,
        reference_max: 5.7,
        blood_type: BloodType::O,
    });

    let bytes = encode_to_vec(&test).expect("encode Option<MedicalTest> Some must succeed");
    let (decoded, bytes_read): (Option<MedicalTest>, usize) =
        decode_from_slice(&bytes).expect("decode Option<MedicalTest> Some must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, test,
        "Option<MedicalTest> Some roundtrip must match original"
    );
}

// ---------------------------------------------------------------------------
// 17. Option<MedicalTest> None variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_medical_test_none_variant_roundtrip() {
    let test: Option<MedicalTest> = None;

    let bytes = encode_to_vec(&test).expect("encode Option<MedicalTest> None must succeed");
    let (decoded, bytes_read): (Option<MedicalTest>, usize) =
        decode_from_slice(&bytes).expect("decode Option<MedicalTest> None must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, None,
        "Option<MedicalTest> None roundtrip must yield None"
    );
}

// ---------------------------------------------------------------------------
// 18. Big-endian config: PatientRecord roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_big_endian_config_patient_record_roundtrip() {
    let record = PatientRecord {
        patient_id: 3003,
        age: 60,
        weight_kg: 78.0,
        height_cm: 172,
        diagnosis: "Chronic Kidney Disease".to_string(),
    };

    let config = oxicode::config::standard().with_big_endian();
    let bytes = oxicode::encode_to_vec_with_config(&record, config)
        .expect("big-endian encode must succeed");
    let (decoded, bytes_read): (PatientRecord, usize) =
        oxicode::decode_from_slice_with_config(&bytes, config)
            .expect("big-endian decode must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, record,
        "big-endian PatientRecord roundtrip must match original"
    );
}

// ---------------------------------------------------------------------------
// 19. Fixed-int config: MedicalTest roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_config_medical_test_roundtrip() {
    let test = MedicalTest {
        test_id: 7007,
        patient_id: 4004,
        test_name: "TSH".to_string(),
        result_value: 2.1,
        reference_min: 0.4,
        reference_max: 4.0,
        blood_type: BloodType::B,
    };

    let config = oxicode::config::standard().with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&test, config).expect("fixed-int encode must succeed");
    let (decoded, bytes_read): (MedicalTest, usize) =
        oxicode::decode_from_slice_with_config(&bytes, config)
            .expect("fixed-int decode must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, test,
        "fixed-int MedicalTest roundtrip must match original"
    );
}

// ---------------------------------------------------------------------------
// 20. Consumed bytes verification: bytes_read equals encoded length
// ---------------------------------------------------------------------------
#[test]
fn test_consumed_bytes_equals_encoded_length_for_patient_record() {
    let record = PatientRecord {
        patient_id: 5005,
        age: 25,
        weight_kg: 60.0,
        height_cm: 165,
        diagnosis: "Migraine".to_string(),
    };

    let bytes = encode_to_vec(&record).expect("encode PatientRecord must succeed");
    let (_decoded, bytes_read): (PatientRecord, usize) =
        decode_from_slice(&bytes).expect("decode PatientRecord must succeed");

    assert_eq!(
        bytes_read,
        bytes.len(),
        "bytes_read ({}) must equal encoded length ({})",
        bytes_read,
        bytes.len()
    );
}

// ---------------------------------------------------------------------------
// 21. Encoding determinism: two encodes of equal PatientRecord produce equal bytes
// ---------------------------------------------------------------------------
#[test]
fn test_encoding_determinism_equal_patient_records_produce_equal_bytes() {
    let record_a = PatientRecord {
        patient_id: 6006,
        age: 38,
        weight_kg: 70.2,
        height_cm: 178,
        diagnosis: "Osteoarthritis".to_string(),
    };
    let record_b = PatientRecord {
        patient_id: 6006,
        age: 38,
        weight_kg: 70.2,
        height_cm: 178,
        diagnosis: "Osteoarthritis".to_string(),
    };

    let bytes_a = encode_to_vec(&record_a).expect("first encode must succeed");
    let bytes_b = encode_to_vec(&record_b).expect("second encode must succeed");

    assert_eq!(
        bytes_a, bytes_b,
        "identical PatientRecord values must produce identical encoded bytes"
    );
}

// ---------------------------------------------------------------------------
// 22. CollectionValidator on Vec<PatientRecord> checks collection bounds and
//     per-record field validation with NumericValidator
// ---------------------------------------------------------------------------
#[test]
fn test_collection_validator_on_vec_patient_record_checks_bounds_and_per_record_fields() {
    let records: Vec<PatientRecord> = vec![
        PatientRecord {
            patient_id: 10,
            age: 20,
            weight_kg: 55.0,
            height_cm: 160,
            diagnosis: "Anemia".to_string(),
        },
        PatientRecord {
            patient_id: 11,
            age: 80,
            weight_kg: 68.0,
            height_cm: 170,
            diagnosis: "Arthritis".to_string(),
        },
    ];

    let coll_v = CollectionValidator::new()
        .min_len(1)
        .max_len(500)
        .non_empty();
    assert!(
        coll_v.validate(&records).is_ok(),
        "2-element Vec<PatientRecord> must pass collection constraints"
    );

    let empty: Vec<PatientRecord> = vec![];
    assert!(
        coll_v.validate(&empty).is_err(),
        "empty Vec<PatientRecord> must fail non_empty constraint"
    );

    // Validate each record's age and height using NumericValidator
    let age_nv = NumericValidator::<u8>::new().min(0u8).max(150u8);
    let height_nv = NumericValidator::<u32>::new().min(1u32).max(300u32);

    for rec in &records {
        assert!(
            age_nv.validate(&rec.age).is_ok(),
            "age {} must pass NumericValidator [0, 150]",
            rec.age
        );
        assert!(
            height_nv.validate(&rec.height_cm).is_ok(),
            "height_cm {} must pass NumericValidator [1, 300]",
            rec.height_cm
        );
    }

    // Validate with ValidationError construction to verify field names
    let age_err = ValidationError::new("patient.age", "age out of medical range");
    assert_eq!(
        age_err.field, "patient.age",
        "ValidationError field must match 'patient.age'"
    );
    assert!(
        format!("{}", age_err).contains("patient.age"),
        "ValidationError Display must contain field name"
    );
}
