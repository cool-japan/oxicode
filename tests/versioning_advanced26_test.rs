#![cfg(feature = "versioning")]

//! Pharmaceutical cold chain / drug distribution domain — versioning feature tests.
//!
//! 22 #[test] functions covering temperature excursions, storage conditions, batch
//! tracking, expiry dates, regulatory compliance, distribution network, chain of
//! custody, and drug recalls using encode_versioned_value / decode_versioned_value.

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
use oxicode::versioning::Version;
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StorageClass {
    Frozen,            // < -15 °C
    Refrigerated,      // +2 to +8 °C
    CoolRoom,          // +8 to +15 °C
    Ambient,           // +15 to +25 °C
    ControlledAmbient, // +15 to +30 °C
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExcursionSeverity {
    Minor,
    Moderate,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RecallClass {
    ClassI,   // Serious health hazard
    ClassII,  // Temporary adverse health consequence
    ClassIII, // Unlikely to cause health hazard
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ComplianceStatus {
    Compliant,
    NonCompliant,
    PendingReview,
    Exempted,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemperatureReading {
    sensor_id: String,
    location: String,
    temperature_milli_celsius: i32, // milli-°C for precision without floats
    recorded_at_unix: u64,
    excursion: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemperatureExcursionV1 {
    excursion_id: u64,
    batch_number: String,
    location: String,
    min_temp_milli_celsius: i32,
    max_temp_milli_celsius: i32,
    duration_minutes: u32,
    severity: ExcursionSeverity,
    reported_by: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemperatureExcursionV2 {
    excursion_id: u64,
    batch_number: String,
    location: String,
    min_temp_milli_celsius: i32,
    max_temp_milli_celsius: i32,
    duration_minutes: u32,
    severity: ExcursionSeverity,
    reported_by: String,
    investigation_ref: Option<String>,
    quarantine_triggered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrugBatch {
    batch_number: String,
    product_code: String,
    product_name: String,
    manufacturer: String,
    manufacture_date_iso: String,
    expiry_date_iso: String,
    quantity_units: u32,
    storage_class: StorageClass,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CustodyRecord {
    record_id: u64,
    batch_number: String,
    from_entity: String,
    to_entity: String,
    transfer_date_iso: String,
    received_by: String,
    condition_on_receipt: String,
    temperature_verified: bool,
    seal_intact: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrugRecall {
    recall_id: u64,
    product_code: String,
    affected_batches: Vec<String>,
    reason: String,
    class: RecallClass,
    initiated_by: String,
    initiated_date_iso: String,
    resolved_date_iso: Option<String>,
    units_recovered: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RegulatorySubmission {
    submission_id: String,
    batch_number: String,
    regulatory_body: String,
    submission_type: String,
    submitted_date_iso: String,
    approval_date_iso: Option<String>,
    status: ComplianceStatus,
    documents: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DistributionNode {
    node_id: u32,
    name: String,
    country_code: String,
    license_number: String,
    storage_classes_supported: Vec<StorageClass>,
    active: bool,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_drug_batch_basic_roundtrip() {
    let batch = DrugBatch {
        batch_number: String::from("BATCH-2026-0001"),
        product_code: String::from("INS-HUMAN-100IU"),
        product_name: String::from("Human Insulin 100 IU/mL"),
        manufacturer: String::from("PharmaCore GmbH"),
        manufacture_date_iso: String::from("2025-11-01"),
        expiry_date_iso: String::from("2027-11-01"),
        quantity_units: 50_000,
        storage_class: StorageClass::Refrigerated,
    };

    let encoded = encode_to_vec(&batch).expect("encode DrugBatch failed");
    let (decoded, _): (DrugBatch, _) =
        decode_from_slice(&encoded).expect("decode DrugBatch failed");

    assert_eq!(batch, decoded);
}

#[test]
fn test_drug_batch_versioned_encode_decode() {
    let batch = DrugBatch {
        batch_number: String::from("BATCH-2026-0002"),
        product_code: String::from("VAC-FLU-0.5ML"),
        product_name: String::from("Influenza Vaccine Quadrivalent"),
        manufacturer: String::from("VaccineX S.A."),
        manufacture_date_iso: String::from("2025-09-15"),
        expiry_date_iso: String::from("2026-09-15"),
        quantity_units: 120_000,
        storage_class: StorageClass::Frozen,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&batch, ver).expect("versioned encode DrugBatch failed");
    let (decoded, decoded_ver, _): (DrugBatch, Version, usize) =
        decode_versioned_value::<DrugBatch>(&bytes).expect("versioned decode DrugBatch failed");

    assert_eq!(batch, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_version_fields_are_public() {
    let ver = Version::new(3, 7, 12);

    // Access public fields directly (not methods)
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 12);
}

#[test]
fn test_temperature_excursion_v1_roundtrip() {
    let excursion = TemperatureExcursionV1 {
        excursion_id: 10001,
        batch_number: String::from("BATCH-2026-0003"),
        location: String::from("Warehouse-A Cold Room 3"),
        min_temp_milli_celsius: 12_500, // 12.5 °C — above +8 threshold
        max_temp_milli_celsius: 18_000, // 18.0 °C
        duration_minutes: 47,
        severity: ExcursionSeverity::Minor,
        reported_by: String::from("sensor-cr3-01"),
    };
    let ver = Version::new(1, 0, 0);

    let bytes =
        encode_versioned_value(&excursion, ver).expect("versioned encode ExcursionV1 failed");
    let (decoded, decoded_ver, _): (TemperatureExcursionV1, Version, usize) =
        decode_versioned_value::<TemperatureExcursionV1>(&bytes)
            .expect("versioned decode ExcursionV1 failed");

    assert_eq!(excursion, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded.severity, ExcursionSeverity::Minor);
}

#[test]
fn test_temperature_excursion_v2_with_option_none() {
    let excursion = TemperatureExcursionV2 {
        excursion_id: 10002,
        batch_number: String::from("BATCH-2026-0004"),
        location: String::from("Transit Vehicle TRK-007"),
        min_temp_milli_celsius: -5_000, // -5 °C for frozen goods — breach
        max_temp_milli_celsius: 2_000,  // 2 °C
        duration_minutes: 180,
        severity: ExcursionSeverity::Critical,
        reported_by: String::from("driver-gps-logger"),
        investigation_ref: None,
        quarantine_triggered: false,
    };
    let ver = Version::new(2, 0, 0);

    let bytes =
        encode_versioned_value(&excursion, ver).expect("versioned encode ExcursionV2 failed");
    let (decoded, decoded_ver, _): (TemperatureExcursionV2, Version, usize) =
        decode_versioned_value::<TemperatureExcursionV2>(&bytes)
            .expect("versioned decode ExcursionV2 failed");

    assert_eq!(excursion, decoded);
    assert!(decoded.investigation_ref.is_none());
    assert_eq!(decoded_ver.major, 2);
}

#[test]
fn test_temperature_excursion_v2_with_option_some() {
    let excursion = TemperatureExcursionV2 {
        excursion_id: 10003,
        batch_number: String::from("BATCH-2026-0005"),
        location: String::from("Dispensary Pharmacy 44"),
        min_temp_milli_celsius: 28_000, // 28 °C for refrigerated goods
        max_temp_milli_celsius: 35_000,
        duration_minutes: 240,
        severity: ExcursionSeverity::Moderate,
        reported_by: String::from("pharmacist-J-Smith"),
        investigation_ref: Some(String::from("INV-2026-00089")),
        quarantine_triggered: true,
    };
    let ver = Version::new(2, 1, 0);

    let bytes =
        encode_versioned_value(&excursion, ver).expect("encode ExcursionV2 with ref failed");
    let (decoded, decoded_ver, consumed): (TemperatureExcursionV2, Version, usize) =
        decode_versioned_value::<TemperatureExcursionV2>(&bytes)
            .expect("decode ExcursionV2 with ref failed");

    assert_eq!(
        decoded.investigation_ref,
        Some(String::from("INV-2026-00089"))
    );
    assert!(decoded.quarantine_triggered);
    assert_eq!(decoded_ver.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_custody_record_seal_intact_some() {
    let record = CustodyRecord {
        record_id: 50001,
        batch_number: String::from("BATCH-2026-0006"),
        from_entity: String::from("Manufacturer Depot Berlin"),
        to_entity: String::from("Regional DC Warsaw"),
        transfer_date_iso: String::from("2026-01-10"),
        received_by: String::from("Receiver: P. Kowalski"),
        condition_on_receipt: String::from("Good — all seals intact, temperature within range"),
        temperature_verified: true,
        seal_intact: Some(true),
    };
    let ver = Version::new(1, 0, 0);

    let bytes =
        encode_versioned_value(&record, ver).expect("versioned encode CustodyRecord failed");
    let (decoded, decoded_ver, _): (CustodyRecord, Version, usize) =
        decode_versioned_value::<CustodyRecord>(&bytes)
            .expect("versioned decode CustodyRecord failed");

    assert_eq!(record, decoded);
    assert_eq!(decoded.seal_intact, Some(true));
    assert_eq!(decoded_ver.major, 1);
}

#[test]
fn test_custody_record_seal_not_checked() {
    let record = CustodyRecord {
        record_id: 50002,
        batch_number: String::from("BATCH-2026-0007"),
        from_entity: String::from("Regional DC Warsaw"),
        to_entity: String::from("Hospital Pharmacy Krakow"),
        transfer_date_iso: String::from("2026-01-12"),
        received_by: String::from("Dr. M. Nowak"),
        condition_on_receipt: String::from("Accepted under emergency protocol"),
        temperature_verified: false,
        seal_intact: None,
    };

    let encoded = encode_to_vec(&record).expect("encode CustodyRecord None seal failed");
    let (decoded, _): (CustodyRecord, _) =
        decode_from_slice(&encoded).expect("decode CustodyRecord None seal failed");

    assert!(decoded.seal_intact.is_none());
    assert!(!decoded.temperature_verified);
}

#[test]
fn test_drug_recall_class_i_versioned() {
    let recall = DrugRecall {
        recall_id: 3001,
        product_code: String::from("ANT-AMOX-500MG"),
        affected_batches: vec![
            String::from("BATCH-2025-1101"),
            String::from("BATCH-2025-1102"),
            String::from("BATCH-2025-1103"),
        ],
        reason: String::from("Contamination with foreign particulate matter"),
        class: RecallClass::ClassI,
        initiated_by: String::from("PharmaCo Quality Dept"),
        initiated_date_iso: String::from("2026-02-01"),
        resolved_date_iso: None,
        units_recovered: 0,
    };
    let ver = Version::new(1, 0, 0);

    let bytes =
        encode_versioned_value(&recall, ver).expect("versioned encode DrugRecall ClassI failed");
    let (decoded, decoded_ver, _): (DrugRecall, Version, usize) =
        decode_versioned_value::<DrugRecall>(&bytes)
            .expect("versioned decode DrugRecall ClassI failed");

    assert_eq!(recall, decoded);
    assert_eq!(decoded.affected_batches.len(), 3);
    assert_eq!(decoded.class, RecallClass::ClassI);
    assert_eq!(decoded_ver.major, 1);
}

#[test]
fn test_drug_recall_class_iii_resolved() {
    let recall = DrugRecall {
        recall_id: 3002,
        product_code: String::from("VIT-D3-1000IU"),
        affected_batches: vec![String::from("BATCH-2025-0500")],
        reason: String::from("Minor label discrepancy — incorrect storage icon"),
        class: RecallClass::ClassIII,
        initiated_by: String::from("Regulatory Affairs Team"),
        initiated_date_iso: String::from("2026-01-05"),
        resolved_date_iso: Some(String::from("2026-01-20")),
        units_recovered: 12_500,
    };

    let encoded = encode_to_vec(&recall).expect("encode ClassIII recall failed");
    let (decoded, _): (DrugRecall, _) =
        decode_from_slice(&encoded).expect("decode ClassIII recall failed");

    assert_eq!(decoded.class, RecallClass::ClassIII);
    assert_eq!(decoded.resolved_date_iso, Some(String::from("2026-01-20")));
    assert_eq!(decoded.units_recovered, 12_500);
}

#[test]
fn test_regulatory_submission_pending_review() {
    let submission = RegulatorySubmission {
        submission_id: String::from("EMA-2026-SUB-00441"),
        batch_number: String::from("BATCH-2026-0010"),
        regulatory_body: String::from("EMA"),
        submission_type: String::from("Batch Release Certificate"),
        submitted_date_iso: String::from("2026-03-01"),
        approval_date_iso: None,
        status: ComplianceStatus::PendingReview,
        documents: vec![
            String::from("CoA_BATCH-2026-0010.pdf"),
            String::from("GMP_cert_2026.pdf"),
        ],
    };
    let ver = Version::new(1, 2, 0);

    let bytes =
        encode_versioned_value(&submission, ver).expect("versioned encode Submission failed");
    let (decoded, decoded_ver, _): (RegulatorySubmission, Version, usize) =
        decode_versioned_value::<RegulatorySubmission>(&bytes)
            .expect("versioned decode Submission failed");

    assert_eq!(submission, decoded);
    assert_eq!(decoded.status, ComplianceStatus::PendingReview);
    assert!(decoded.approval_date_iso.is_none());
    assert_eq!(decoded_ver.minor, 2);
}

#[test]
fn test_regulatory_submission_approved() {
    let submission = RegulatorySubmission {
        submission_id: String::from("FDA-2026-NDA-00099"),
        batch_number: String::from("BATCH-2026-0011"),
        regulatory_body: String::from("FDA"),
        submission_type: String::from("New Drug Application"),
        submitted_date_iso: String::from("2025-06-01"),
        approval_date_iso: Some(String::from("2026-02-15")),
        status: ComplianceStatus::Compliant,
        documents: vec![
            String::from("clinical_trials_phase3.pdf"),
            String::from("safety_report.pdf"),
            String::from("manufacturing_process.pdf"),
        ],
    };

    let encoded = encode_to_vec(&submission).expect("encode approved Submission failed");
    let (decoded, _): (RegulatorySubmission, _) =
        decode_from_slice(&encoded).expect("decode approved Submission failed");

    assert_eq!(decoded.status, ComplianceStatus::Compliant);
    assert_eq!(decoded.approval_date_iso, Some(String::from("2026-02-15")));
    assert_eq!(decoded.documents.len(), 3);
}

#[test]
fn test_distribution_node_multi_storage_class() {
    let node = DistributionNode {
        node_id: 101,
        name: String::from("Central Distribution Hub Tallinn"),
        country_code: String::from("EE"),
        license_number: String::from("EE-GDP-2025-0042"),
        storage_classes_supported: vec![
            StorageClass::Frozen,
            StorageClass::Refrigerated,
            StorageClass::CoolRoom,
            StorageClass::Ambient,
        ],
        active: true,
    };
    let ver = Version::new(1, 0, 0);

    let bytes =
        encode_versioned_value(&node, ver).expect("versioned encode DistributionNode failed");
    let (decoded, decoded_ver, consumed): (DistributionNode, Version, usize) =
        decode_versioned_value::<DistributionNode>(&bytes)
            .expect("versioned decode DistributionNode failed");

    assert_eq!(node, decoded);
    assert_eq!(decoded.storage_classes_supported.len(), 4);
    assert_eq!(decoded_ver.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_distribution_node_inactive_empty_storage() {
    let node = DistributionNode {
        node_id: 999,
        name: String::from("Decommissioned Depot Riga"),
        country_code: String::from("LV"),
        license_number: String::from("LV-GDP-2019-0001"),
        storage_classes_supported: vec![],
        active: false,
    };

    let encoded = encode_to_vec(&node).expect("encode inactive DistributionNode failed");
    let (decoded, _): (DistributionNode, _) =
        decode_from_slice(&encoded).expect("decode inactive DistributionNode failed");

    assert!(!decoded.active);
    assert!(decoded.storage_classes_supported.is_empty());
}

#[test]
fn test_multiple_versions_same_batch_struct() {
    let batch = DrugBatch {
        batch_number: String::from("BATCH-MULTIVER-001"),
        product_code: String::from("EPO-ERYTHRO-4000IU"),
        product_name: String::from("Erythropoietin 4000 IU"),
        manufacturer: String::from("BioSynth Labs"),
        manufacture_date_iso: String::from("2025-07-01"),
        expiry_date_iso: String::from("2027-07-01"),
        quantity_units: 8_000,
        storage_class: StorageClass::Refrigerated,
    };

    let ver_a = Version::new(1, 0, 0);
    let ver_b = Version::new(2, 4, 1);

    let bytes_a = encode_versioned_value(&batch, ver_a).expect("encode v1.0.0 batch failed");
    let bytes_b = encode_versioned_value(&batch, ver_b).expect("encode v2.4.1 batch failed");

    let (_, dver_a, _): (DrugBatch, Version, usize) =
        decode_versioned_value::<DrugBatch>(&bytes_a).expect("decode v1.0.0 batch failed");
    let (_, dver_b, _): (DrugBatch, Version, usize) =
        decode_versioned_value::<DrugBatch>(&bytes_b).expect("decode v2.4.1 batch failed");

    assert_eq!(dver_a.major, 1);
    assert_eq!(dver_a.minor, 0);
    assert_eq!(dver_a.patch, 0);

    assert_eq!(dver_b.major, 2);
    assert_eq!(dver_b.minor, 4);
    assert_eq!(dver_b.patch, 1);
}

#[test]
fn test_vec_of_temperature_readings_roundtrip() {
    let readings = vec![
        TemperatureReading {
            sensor_id: String::from("SENS-CR3-01"),
            location: String::from("Cold Room 3 Entry"),
            temperature_milli_celsius: 4_500,
            recorded_at_unix: 1_741_000_000,
            excursion: false,
        },
        TemperatureReading {
            sensor_id: String::from("SENS-CR3-02"),
            location: String::from("Cold Room 3 Middle"),
            temperature_milli_celsius: 5_200,
            recorded_at_unix: 1_741_000_060,
            excursion: false,
        },
        TemperatureReading {
            sensor_id: String::from("SENS-CR3-03"),
            location: String::from("Cold Room 3 Exit"),
            temperature_milli_celsius: 9_800, // Excursion
            recorded_at_unix: 1_741_000_120,
            excursion: true,
        },
    ];
    let ver = Version::new(1, 0, 0);

    let bytes =
        encode_versioned_value(&readings, ver).expect("encode Vec<TemperatureReading> failed");
    let (decoded, decoded_ver, consumed): (Vec<TemperatureReading>, Version, usize) =
        decode_versioned_value::<Vec<TemperatureReading>>(&bytes)
            .expect("decode Vec<TemperatureReading> failed");

    assert_eq!(readings, decoded);
    assert_eq!(decoded.len(), 3);
    assert!(decoded[2].excursion);
    assert_eq!(decoded_ver.major, 1);
    // consumed now includes the full versioned envelope (header + payload)
    assert!(consumed > 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_bytes_consumed_is_payload_portion() {
    let record = CustodyRecord {
        record_id: 99999,
        batch_number: String::from("BATCH-CONSUMED-CHECK"),
        from_entity: String::from("Manufacturer"),
        to_entity: String::from("Wholesaler"),
        transfer_date_iso: String::from("2026-03-15"),
        received_by: String::from("Warehouse Mgr"),
        condition_on_receipt: String::from("OK"),
        temperature_verified: true,
        seal_intact: Some(true),
    };
    let ver = Version::new(1, 0, 0);

    // Also encode without version header to know the bare payload size
    let payload_only = encode_to_vec(&record).expect("encode payload-only failed");
    let bytes = encode_versioned_value(&record, ver).expect("encode for consumed-check failed");
    let (decoded, _, consumed): (CustodyRecord, Version, usize) =
        decode_versioned_value::<CustodyRecord>(&bytes).expect("decode for consumed-check failed");

    // consumed now includes the full versioned envelope (header + payload)
    assert_eq!(decoded, record);
    assert_eq!(consumed, bytes.len());
    // versioned encoding is always larger than the bare payload
    assert!(bytes.len() > payload_only.len());
}

#[test]
fn test_excursion_severity_all_variants() {
    let minor = TemperatureExcursionV1 {
        excursion_id: 1,
        batch_number: String::from("B-MINOR"),
        location: String::from("Loc A"),
        min_temp_milli_celsius: 9_000,
        max_temp_milli_celsius: 10_000,
        duration_minutes: 5,
        severity: ExcursionSeverity::Minor,
        reported_by: String::from("auto-sensor"),
    };
    let moderate = TemperatureExcursionV1 {
        excursion_id: 2,
        batch_number: String::from("B-MOD"),
        location: String::from("Loc B"),
        min_temp_milli_celsius: 15_000,
        max_temp_milli_celsius: 20_000,
        duration_minutes: 60,
        severity: ExcursionSeverity::Moderate,
        reported_by: String::from("auto-sensor"),
    };
    let critical = TemperatureExcursionV1 {
        excursion_id: 3,
        batch_number: String::from("B-CRIT"),
        location: String::from("Loc C"),
        min_temp_milli_celsius: 30_000,
        max_temp_milli_celsius: 40_000,
        duration_minutes: 360,
        severity: ExcursionSeverity::Critical,
        reported_by: String::from("manual-report"),
    };

    for excursion in [&minor, &moderate, &critical] {
        let encoded = encode_to_vec(excursion).expect("encode severity variant failed");
        let (decoded, _): (TemperatureExcursionV1, _) =
            decode_from_slice(&encoded).expect("decode severity variant failed");
        assert_eq!(excursion, &decoded);
    }
}

#[test]
fn test_compliance_status_all_variants() {
    let statuses = [
        ComplianceStatus::Compliant,
        ComplianceStatus::NonCompliant,
        ComplianceStatus::PendingReview,
        ComplianceStatus::Exempted,
    ];

    for status in &statuses {
        let sub = RegulatorySubmission {
            submission_id: String::from("SUB-STATUS-TEST"),
            batch_number: String::from("B-STATUS"),
            regulatory_body: String::from("EMA"),
            submission_type: String::from("Periodic Safety Update"),
            submitted_date_iso: String::from("2026-01-01"),
            approval_date_iso: None,
            status: status.clone(),
            documents: vec![],
        };
        let encoded = encode_to_vec(&sub).expect("encode ComplianceStatus variant failed");
        let (decoded, _): (RegulatorySubmission, _) =
            decode_from_slice(&encoded).expect("decode ComplianceStatus variant failed");
        assert_eq!(&decoded.status, status);
    }
}

#[test]
fn test_recall_class_ii_with_versioning() {
    let recall = DrugRecall {
        recall_id: 4001,
        product_code: String::from("ANT-CIPRO-250MG"),
        affected_batches: vec![
            String::from("BATCH-2025-0801"),
            String::from("BATCH-2025-0802"),
        ],
        reason: String::from("Subpotent — active ingredient below specification"),
        class: RecallClass::ClassII,
        initiated_by: String::from("MHRA Enforcement"),
        initiated_date_iso: String::from("2026-02-20"),
        resolved_date_iso: None,
        units_recovered: 3_400,
    };
    let ver = Version::new(1, 3, 0);

    let bytes =
        encode_versioned_value(&recall, ver).expect("versioned encode ClassII recall failed");
    let (decoded, decoded_ver, _): (DrugRecall, Version, usize) =
        decode_versioned_value::<DrugRecall>(&bytes)
            .expect("versioned decode ClassII recall failed");

    assert_eq!(recall, decoded);
    assert_eq!(decoded.class, RecallClass::ClassII);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 3);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_custody_chain_vec_versioned() {
    let chain = vec![
        CustodyRecord {
            record_id: 60001,
            batch_number: String::from("BATCH-CHAIN-001"),
            from_entity: String::from("Manufacturer"),
            to_entity: String::from("National Distributor"),
            transfer_date_iso: String::from("2026-01-01"),
            received_by: String::from("John D."),
            condition_on_receipt: String::from("Good"),
            temperature_verified: true,
            seal_intact: Some(true),
        },
        CustodyRecord {
            record_id: 60002,
            batch_number: String::from("BATCH-CHAIN-001"),
            from_entity: String::from("National Distributor"),
            to_entity: String::from("Regional Wholesaler"),
            transfer_date_iso: String::from("2026-01-05"),
            received_by: String::from("Anna K."),
            condition_on_receipt: String::from("Good"),
            temperature_verified: true,
            seal_intact: Some(true),
        },
        CustodyRecord {
            record_id: 60003,
            batch_number: String::from("BATCH-CHAIN-001"),
            from_entity: String::from("Regional Wholesaler"),
            to_entity: String::from("Hospital Pharmacy"),
            transfer_date_iso: String::from("2026-01-08"),
            received_by: String::from("Dr. B. Patel"),
            condition_on_receipt: String::from("Acceptable — one outer carton damaged"),
            temperature_verified: true,
            seal_intact: Some(false),
        },
    ];
    let ver = Version::new(2, 0, 0);

    let bytes = encode_versioned_value(&chain, ver).expect("encode custody chain Vec failed");
    let (decoded, decoded_ver, consumed): (Vec<CustodyRecord>, Version, usize) =
        decode_versioned_value::<Vec<CustodyRecord>>(&bytes)
            .expect("decode custody chain Vec failed");

    assert_eq!(chain, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[2].seal_intact, Some(false));
    assert_eq!(decoded_ver.major, 2);
    // consumed now includes the full versioned envelope (header + payload)
    assert!(consumed > 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_storage_class_frozen_batch_versioned_high_patch() {
    let batch = DrugBatch {
        batch_number: String::from("BATCH-FROZEN-001"),
        product_code: String::from("BIO-FILGRAS-300MCG"),
        product_name: String::from("Filgrastim Biosimilar 300 mcg"),
        manufacturer: String::from("BioGenix"),
        manufacture_date_iso: String::from("2025-12-01"),
        expiry_date_iso: String::from("2027-12-01"),
        quantity_units: 2_500,
        storage_class: StorageClass::Frozen,
    };
    let ver = Version::new(1, 0, 99);

    let bytes = encode_versioned_value(&batch, ver).expect("encode frozen batch v1.0.99 failed");
    let (decoded, decoded_ver, _): (DrugBatch, Version, usize) =
        decode_versioned_value::<DrugBatch>(&bytes).expect("decode frozen batch v1.0.99 failed");

    assert_eq!(batch, decoded);
    assert_eq!(decoded.storage_class, StorageClass::Frozen);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 99);
}
