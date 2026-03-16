//! Advanced checksum tests — digital identity / KYC domain theme.
//! Tests OxiCode's checksum API using identity verification and KYC data types.

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    Expired,
    Flagged,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum DocumentType {
    Passport,
    NationalId,
    DriversLicense,
    ResidencePermit,
    BirthCertificate,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct IdentityDocument {
    doc_type: DocumentType,
    number: String,
    issuer_country: String,
    issue_date: u64,
    expiry_date: u64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct BiometricData {
    face_hash: String,
    fingerprint_hash: String,
    confidence: f32,
    captured_at: u64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct KycRecord {
    user_id: u64,
    status: VerificationStatus,
    documents: Vec<IdentityDocument>,
    biometrics: Option<BiometricData>,
    verified_at: Option<u64>,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_passport(country: &str, number: &str) -> IdentityDocument {
    IdentityDocument {
        doc_type: DocumentType::Passport,
        number: number.to_string(),
        issuer_country: country.to_string(),
        issue_date: 1_640_000_000,
        expiry_date: 1_955_000_000,
    }
}

fn make_biometric(confidence: f32) -> BiometricData {
    BiometricData {
        face_hash: "sha256:abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
            .to_string(),
        fingerprint_hash: "sha256:fedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321"
            .to_string(),
        confidence,
        captured_at: 1_700_000_000,
    }
}

fn make_kyc_record(
    user_id: u64,
    status: VerificationStatus,
    documents: Vec<IdentityDocument>,
    biometrics: Option<BiometricData>,
) -> KycRecord {
    let verified_at = match status {
        VerificationStatus::Verified => Some(1_700_000_001),
        _ => None,
    };
    KycRecord {
        user_id,
        status,
        documents,
        biometrics,
        verified_at,
    }
}

// ---------------------------------------------------------------------------
// Test 1: HEADER_SIZE constant equals 16
// ---------------------------------------------------------------------------

#[test]
fn test_header_size_is_16() {
    assert_eq!(
        HEADER_SIZE, 16,
        "HEADER_SIZE must be exactly 16 bytes (MAGIC(3)+VERSION(1)+LEN(8)+CRC32(4))"
    );
}

// ---------------------------------------------------------------------------
// Test 2: wrap/unwrap roundtrip for IdentityDocument (Passport)
// ---------------------------------------------------------------------------

#[test]
fn test_identity_document_passport_roundtrip() {
    let doc = make_passport("DEU", "C01X00T47");
    let bytes = encode_to_vec(&doc).expect("encode IdentityDocument Passport failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap IdentityDocument Passport failed");
    let (decoded, _): (IdentityDocument, usize) =
        decode_from_slice(&unwrapped).expect("decode IdentityDocument Passport failed");
    assert_eq!(
        doc, decoded,
        "IdentityDocument Passport roundtrip must preserve all fields"
    );
}

// ---------------------------------------------------------------------------
// Test 3: wrap/unwrap roundtrip for BiometricData
// ---------------------------------------------------------------------------

#[test]
fn test_biometric_data_roundtrip() {
    let bio = make_biometric(0.9987);
    let bytes = encode_to_vec(&bio).expect("encode BiometricData failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap BiometricData failed");
    let (decoded, _): (BiometricData, usize) =
        decode_from_slice(&unwrapped).expect("decode BiometricData failed");
    assert_eq!(
        bio, decoded,
        "BiometricData roundtrip must preserve all fields"
    );
}

// ---------------------------------------------------------------------------
// Test 4: KycRecord with biometrics (Verified status) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_with_biometrics_roundtrip() {
    let docs = vec![make_passport("GBR", "PASSPORT123456")];
    let bio = Some(make_biometric(0.998));
    let record = make_kyc_record(1001, VerificationStatus::Verified, docs, bio);
    let bytes = encode_to_vec(&record).expect("encode KycRecord with biometrics failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap KycRecord with biometrics failed");
    let (decoded, _): (KycRecord, usize) =
        decode_from_slice(&unwrapped).expect("decode KycRecord with biometrics failed");
    assert_eq!(record, decoded);
    assert!(
        decoded.biometrics.is_some(),
        "biometrics must be Some after roundtrip"
    );
    assert_eq!(decoded.verified_at, Some(1_700_000_001));
}

// ---------------------------------------------------------------------------
// Test 5: KycRecord without biometrics (Pending status) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_without_biometrics_roundtrip() {
    let docs = vec![make_passport("USA", "US9876543")];
    let record = make_kyc_record(2002, VerificationStatus::Pending, docs, None);
    let bytes = encode_to_vec(&record).expect("encode KycRecord without biometrics failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap KycRecord without biometrics failed");
    let (decoded, _): (KycRecord, usize) =
        decode_from_slice(&unwrapped).expect("decode KycRecord without biometrics failed");
    assert_eq!(record, decoded);
    assert!(
        decoded.biometrics.is_none(),
        "biometrics must be None after roundtrip"
    );
}

// ---------------------------------------------------------------------------
// Test 6: KycRecord with empty documents list roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_empty_documents_roundtrip() {
    let record = make_kyc_record(3003, VerificationStatus::Pending, vec![], None);
    let bytes = encode_to_vec(&record).expect("encode KycRecord empty documents failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped =
        unwrap_with_checksum(&wrapped).expect("unwrap KycRecord empty documents failed");
    let (decoded, _): (KycRecord, usize) =
        decode_from_slice(&unwrapped).expect("decode KycRecord empty documents failed");
    assert_eq!(record, decoded);
    assert!(
        decoded.documents.is_empty(),
        "documents must be empty after roundtrip"
    );
}

// ---------------------------------------------------------------------------
// Test 7: all VerificationStatus variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_verification_status_variants_roundtrip() {
    let statuses = vec![
        VerificationStatus::Pending,
        VerificationStatus::Verified,
        VerificationStatus::Failed,
        VerificationStatus::Expired,
        VerificationStatus::Flagged,
    ];
    for (idx, status) in statuses.into_iter().enumerate() {
        let record = make_kyc_record(idx as u64 + 100, status, vec![], None);
        let bytes = encode_to_vec(&record).expect("encode VerificationStatus variant failed");
        let wrapped = wrap_with_checksum(&bytes);
        let unwrapped =
            unwrap_with_checksum(&wrapped).expect("unwrap VerificationStatus variant failed");
        let (decoded, _): (KycRecord, usize) =
            decode_from_slice(&unwrapped).expect("decode VerificationStatus variant failed");
        assert_eq!(
            record, decoded,
            "VerificationStatus variant at index {} failed",
            idx
        );
    }
}

// ---------------------------------------------------------------------------
// Test 8: all DocumentType variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_document_type_variants_roundtrip() {
    let doc_types = vec![
        DocumentType::Passport,
        DocumentType::NationalId,
        DocumentType::DriversLicense,
        DocumentType::ResidencePermit,
        DocumentType::BirthCertificate,
    ];
    for (idx, doc_type) in doc_types.into_iter().enumerate() {
        let doc = IdentityDocument {
            doc_type,
            number: format!("DOC{:08}", idx),
            issuer_country: "EST".to_string(),
            issue_date: 1_600_000_000 + idx as u64 * 1_000_000,
            expiry_date: 1_900_000_000 + idx as u64 * 1_000_000,
        };
        let bytes = encode_to_vec(&doc).expect("encode DocumentType variant failed");
        let wrapped = wrap_with_checksum(&bytes);
        let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap DocumentType variant failed");
        let (decoded, _): (IdentityDocument, usize) =
            decode_from_slice(&unwrapped).expect("decode DocumentType variant failed");
        assert_eq!(doc, decoded, "DocumentType variant at index {} failed", idx);
    }
}

// ---------------------------------------------------------------------------
// Test 9: wrapped output is exactly HEADER_SIZE bytes longer than plain encoding
// ---------------------------------------------------------------------------

#[test]
fn test_wrapped_output_length_overhead() {
    let doc = make_passport("FRA", "FR44556677");
    let plain = encode_to_vec(&doc).expect("plain encode failed");
    let wrapped = wrap_with_checksum(&plain);
    assert_eq!(
        wrapped.len(),
        plain.len() + HEADER_SIZE,
        "wrapped output must be exactly HEADER_SIZE bytes longer than the payload"
    );
}

// ---------------------------------------------------------------------------
// Test 10: corruption detection — flip ALL bytes after index 4
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_detected_after_index_4() {
    let doc = make_passport("JPN", "TK20240001");
    let bytes = encode_to_vec(&doc).expect("encode for corruption test failed");
    let wrapped = wrap_with_checksum(&bytes);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption after index 4 must be detected, but unwrap returned Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 11: corruption detection for KycRecord
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_corruption_detected() {
    let docs = vec![make_passport("AUS", "PA6543210")];
    let bio = Some(make_biometric(0.975));
    let record = make_kyc_record(5005, VerificationStatus::Verified, docs, bio);
    let bytes = encode_to_vec(&record).expect("encode KycRecord for corruption test failed");
    let wrapped = wrap_with_checksum(&bytes);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption in KycRecord payload must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 12: double wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_double_wrap_unwrap_roundtrip() {
    let docs = vec![make_passport("NLD", "NL20230099")];
    let record = make_kyc_record(6006, VerificationStatus::Verified, docs, None);
    let bytes = encode_to_vec(&record).expect("encode for double-wrap failed");
    // First wrap
    let wrapped_once = wrap_with_checksum(&bytes);
    // Second wrap (wrapping the already-wrapped bytes)
    let wrapped_twice = wrap_with_checksum(&wrapped_once);
    // First unwrap
    let after_first_unwrap =
        unwrap_with_checksum(&wrapped_twice).expect("first unwrap of double-wrapped data failed");
    // Second unwrap
    let after_second_unwrap =
        unwrap_with_checksum(&after_first_unwrap).expect("second unwrap failed");
    let (decoded, _): (KycRecord, usize) =
        decode_from_slice(&after_second_unwrap).expect("decode after double unwrap failed");
    assert_eq!(
        record, decoded,
        "double wrap/unwrap must restore original value"
    );
}

// ---------------------------------------------------------------------------
// Test 13: KycRecord with Flagged status and multiple documents roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_flagged_multiple_documents_roundtrip() {
    let docs = vec![
        IdentityDocument {
            doc_type: DocumentType::Passport,
            number: "SUSP00001".to_string(),
            issuer_country: "XXX".to_string(),
            issue_date: 1_500_000_000,
            expiry_date: 1_800_000_000,
        },
        IdentityDocument {
            doc_type: DocumentType::NationalId,
            number: "NID999888".to_string(),
            issuer_country: "YYY".to_string(),
            issue_date: 1_550_000_000,
            expiry_date: 1_850_000_000,
        },
    ];
    let record = make_kyc_record(7007, VerificationStatus::Flagged, docs, None);
    let bytes = encode_to_vec(&record).expect("encode Flagged KycRecord failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap Flagged KycRecord failed");
    let (decoded, _): (KycRecord, usize) =
        decode_from_slice(&unwrapped).expect("decode Flagged KycRecord failed");
    assert_eq!(record, decoded);
    assert_eq!(decoded.documents.len(), 2);
}

// ---------------------------------------------------------------------------
// Test 14: IdentityDocument with DriversLicense type roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_identity_document_drivers_license_roundtrip() {
    let doc = IdentityDocument {
        doc_type: DocumentType::DriversLicense,
        number: "DL-CA-12345678".to_string(),
        issuer_country: "USA".to_string(),
        issue_date: 1_650_000_000,
        expiry_date: 1_965_000_000,
    };
    let bytes = encode_to_vec(&doc).expect("encode DriversLicense failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap DriversLicense failed");
    let (decoded, _): (IdentityDocument, usize) =
        decode_from_slice(&unwrapped).expect("decode DriversLicense failed");
    assert_eq!(doc, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: IdentityDocument with ResidencePermit type roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_identity_document_residence_permit_roundtrip() {
    let doc = IdentityDocument {
        doc_type: DocumentType::ResidencePermit,
        number: "RP-SE-20230042".to_string(),
        issuer_country: "SWE".to_string(),
        issue_date: 1_680_000_000,
        expiry_date: 1_995_000_000,
    };
    let bytes = encode_to_vec(&doc).expect("encode ResidencePermit failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap ResidencePermit failed");
    let (decoded, _): (IdentityDocument, usize) =
        decode_from_slice(&unwrapped).expect("decode ResidencePermit failed");
    assert_eq!(doc, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: IdentityDocument with BirthCertificate type roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_identity_document_birth_certificate_roundtrip() {
    let doc = IdentityDocument {
        doc_type: DocumentType::BirthCertificate,
        number: "BC-EE-19900515-001".to_string(),
        issuer_country: "EST".to_string(),
        issue_date: 0,
        expiry_date: u64::MAX,
    };
    let bytes = encode_to_vec(&doc).expect("encode BirthCertificate failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap BirthCertificate failed");
    let (decoded, _): (IdentityDocument, usize) =
        decode_from_slice(&unwrapped).expect("decode BirthCertificate failed");
    assert_eq!(doc, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: BiometricData with extreme confidence values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_biometric_data_extreme_confidence_roundtrip() {
    let bio_min = BiometricData {
        face_hash: "sha256:0000000000000000000000000000000000000000000000000000000000000000"
            .to_string(),
        fingerprint_hash: "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            .to_string(),
        confidence: 0.0,
        captured_at: 0,
    };
    let bio_max = BiometricData {
        face_hash: "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            .to_string(),
        fingerprint_hash: "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
            .to_string(),
        confidence: 1.0,
        captured_at: u64::MAX,
    };
    for bio in [bio_min, bio_max] {
        let bytes = encode_to_vec(&bio).expect("encode extreme BiometricData failed");
        let wrapped = wrap_with_checksum(&bytes);
        let unwrapped =
            unwrap_with_checksum(&wrapped).expect("unwrap extreme BiometricData failed");
        let (decoded, _): (BiometricData, usize) =
            decode_from_slice(&unwrapped).expect("decode extreme BiometricData failed");
        assert_eq!(
            bio, decoded,
            "extreme confidence BiometricData roundtrip failed"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 18: KycRecord with Expired status and no verified_at roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_expired_status_roundtrip() {
    let docs = vec![IdentityDocument {
        doc_type: DocumentType::Passport,
        number: "EXPIRED001".to_string(),
        issuer_country: "ITA".to_string(),
        issue_date: 1_300_000_000,
        expiry_date: 1_500_000_000,
    }];
    let record = KycRecord {
        user_id: 8008,
        status: VerificationStatus::Expired,
        documents: docs,
        biometrics: None,
        verified_at: None,
    };
    let bytes = encode_to_vec(&record).expect("encode Expired KycRecord failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap Expired KycRecord failed");
    let (decoded, _): (KycRecord, usize) =
        decode_from_slice(&unwrapped).expect("decode Expired KycRecord failed");
    assert_eq!(record, decoded);
    assert!(
        decoded.verified_at.is_none(),
        "verified_at must be None for Expired"
    );
}

// ---------------------------------------------------------------------------
// Test 19: unwrap returns exact payload bytes
// ---------------------------------------------------------------------------

#[test]
fn test_unwrap_returns_exact_payload_bytes() {
    let doc = make_passport("KOR", "KR19820003");
    let original_bytes = encode_to_vec(&doc).expect("encode for payload bytes test failed");
    let wrapped = wrap_with_checksum(&original_bytes);
    let recovered = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    assert_eq!(
        recovered, original_bytes,
        "unwrap_with_checksum must return the exact original payload bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 20: wrapped bytes start with OXH magic
// ---------------------------------------------------------------------------

#[test]
fn test_wrapped_bytes_have_oxh_magic() {
    let bio = make_biometric(0.88);
    let bytes = encode_to_vec(&bio).expect("encode for magic test failed");
    let wrapped = wrap_with_checksum(&bytes);
    assert!(
        wrapped.len() >= 3,
        "wrapped output must contain at least the magic bytes"
    );
    assert_eq!(
        &wrapped[..3],
        &[0x4F, 0x58, 0x48],
        "wrapped output must begin with OXH magic bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 21: KycRecord with Failed status and biometrics corruption is detected
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_failed_with_biometrics_corruption_detected() {
    let docs = vec![make_passport("BRA", "BR77665544")];
    let bio = Some(make_biometric(0.321));
    let record = make_kyc_record(9009, VerificationStatus::Failed, docs, bio);
    let bytes = encode_to_vec(&record).expect("encode Failed KycRecord for corruption test failed");
    let wrapped = wrap_with_checksum(&bytes);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption of Failed KycRecord with biometrics must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 22: KycRecord with large number of documents roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_kyc_record_many_documents_roundtrip() {
    let doc_types = [
        DocumentType::Passport,
        DocumentType::NationalId,
        DocumentType::DriversLicense,
        DocumentType::ResidencePermit,
        DocumentType::BirthCertificate,
    ];
    let documents: Vec<IdentityDocument> = (0u64..50)
        .map(|i| IdentityDocument {
            doc_type: doc_types[(i as usize) % doc_types.len()].clone(),
            number: format!("BULK-DOC-{:06}", i),
            issuer_country: "ISR".to_string(),
            issue_date: 1_600_000_000 + i * 10_000,
            expiry_date: 1_900_000_000 + i * 10_000,
        })
        .collect();
    let record = KycRecord {
        user_id: 10010,
        status: VerificationStatus::Verified,
        documents,
        biometrics: Some(make_biometric(0.9995)),
        verified_at: Some(1_710_000_000),
    };
    let bytes = encode_to_vec(&record).expect("encode many-documents KycRecord failed");
    let wrapped = wrap_with_checksum(&bytes);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap many-documents KycRecord failed");
    let (decoded, _): (KycRecord, usize) =
        decode_from_slice(&unwrapped).expect("decode many-documents KycRecord failed");
    assert_eq!(record, decoded);
    assert_eq!(decoded.documents.len(), 50);
}
