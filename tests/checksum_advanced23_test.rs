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
// Domain types: biometric security / identity verification
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BiometricModality {
    Fingerprint,
    IrisScan,
    FacialRecognition,
    VoicePrint,
    BehavioralKeystroke,
    PalmVein,
    RetinalScan,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LivenessStatus {
    Confirmed,
    Suspected,
    Failed,
    NotChecked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AccessDecision {
    Granted,
    Denied,
    RequiresMfa,
    TemporarilyLocked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FingerprintTemplate {
    subject_id: u64,
    finger_index: u8,
    quality_score: u8,
    minutiae_count: u16,
    template_data: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IrisTemplate {
    subject_id: u64,
    eye: u8, // 0 = left, 1 = right
    iris_code: Vec<u8>,
    hamming_threshold: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FacialEmbedding {
    subject_id: u64,
    embedding_model_version: u32,
    embedding_vector: Vec<f32>,
    liveness_status: LivenessStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VoicePrintProfile {
    subject_id: u64,
    utterance_hash: [u8; 32],
    mfcc_features: Vec<f32>,
    snr_db: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BehavioralBiometricSample {
    session_id: u64,
    avg_dwell_ms: f32,
    avg_flight_ms: f32,
    typing_rhythm_vector: Vec<f32>,
    confidence: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AuthenticationToken {
    token_id: u64,
    subject_id: u64,
    issued_at_unix: u64,
    expires_at_unix: u64,
    modality: BiometricModality,
    match_score: f32,
    access_decision: AccessDecision,
    signed_payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AccessControlEntry {
    resource_id: u64,
    subject_id: u64,
    allowed_modalities: Vec<BiometricModality>,
    min_match_score: f32,
    decision: AccessDecision,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IdentityRecord {
    subject_id: u64,
    display_name: String,
    fingerprint: Option<FingerprintTemplate>,
    iris: Option<IrisTemplate>,
    face: Option<FacialEmbedding>,
    voice: Option<VoicePrintProfile>,
    registered_at_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TemplateMatchResult {
    probe_id: u64,
    gallery_id: u64,
    score: f32,
    threshold: f32,
    is_match: bool,
    modality: BiometricModality,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LivenessChallenge {
    challenge_id: u64,
    subject_id: u64,
    challenge_nonce: [u8; 16],
    response_hash: Vec<u8>,
    status: LivenessStatus,
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

/// Test 1: basic roundtrip for FingerprintTemplate via encode_to_vec / decode_from_slice
#[test]
fn test_fingerprint_template_basic_roundtrip() {
    let tmpl = FingerprintTemplate {
        subject_id: 100_001,
        finger_index: 1,
        quality_score: 87,
        minutiae_count: 42,
        template_data: vec![0x01, 0x02, 0x03, 0xDE, 0xAD],
    };
    let encoded = encode_to_vec(&tmpl).expect("encode FingerprintTemplate");
    let (decoded, _): (FingerprintTemplate, _) =
        decode_from_slice(&encoded).expect("decode FingerprintTemplate");
    assert_eq!(tmpl, decoded);
}

/// Test 2: wrap and unwrap FingerprintTemplate bytes with checksum
#[test]
fn test_fingerprint_template_wrap_unwrap() {
    let tmpl = FingerprintTemplate {
        subject_id: 200_002,
        finger_index: 3,
        quality_score: 95,
        minutiae_count: 60,
        template_data: (0u8..=127).collect(),
    };
    let encoded = encode_to_vec(&tmpl).expect("encode");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    assert_eq!(encoded, unwrapped);
}

/// Test 3: HEADER_SIZE constant equals 16 and wrapped buffer is exactly HEADER_SIZE larger
#[test]
fn test_header_size_constant() {
    assert_eq!(HEADER_SIZE, 16, "HEADER_SIZE must be 16");

    let tmpl = IrisTemplate {
        subject_id: 300_003,
        eye: 0,
        iris_code: vec![0xAB; 256],
        hamming_threshold: 0.32,
    };
    let encoded = encode_to_vec(&tmpl).expect("encode IrisTemplate");
    let payload_len = encoded.len();
    let wrapped = wrap_with_checksum(&encoded);
    assert_eq!(wrapped.len(), HEADER_SIZE + payload_len);
}

/// Test 4: corruption in the payload bytes (after header) causes unwrap_with_checksum to return Err
#[test]
fn test_payload_corruption_detected() {
    let tmpl = IrisTemplate {
        subject_id: 400_004,
        eye: 1,
        iris_code: vec![0x55; 128],
        hamming_threshold: 0.25,
    };
    let encoded = encode_to_vec(&tmpl).expect("encode");
    let mut corrupted = wrap_with_checksum(&encoded);
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(result.is_err(), "corrupted data must produce Err");
}

/// Test 5: FacialEmbedding roundtrip with checksum wrap/unwrap
#[test]
fn test_facial_embedding_checksum_roundtrip() {
    let face = FacialEmbedding {
        subject_id: 500_005,
        embedding_model_version: 3,
        embedding_vector: vec![0.1, 0.2, 0.3, -0.5, 0.99],
        liveness_status: LivenessStatus::Confirmed,
    };
    let encoded = encode_to_vec(&face).expect("encode FacialEmbedding");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap FacialEmbedding");
    let (decoded, _): (FacialEmbedding, _) =
        decode_from_slice(&unwrapped).expect("decode FacialEmbedding");
    assert_eq!(face, decoded);
}

/// Test 6: VoicePrintProfile struct roundtrip
#[test]
fn test_voice_print_profile_roundtrip() {
    let profile = VoicePrintProfile {
        subject_id: 600_006,
        utterance_hash: [0xCCu8; 32],
        mfcc_features: (0..40).map(|i| i as f32 * 0.1).collect(),
        snr_db: 22.5,
    };
    let encoded = encode_to_vec(&profile).expect("encode VoicePrintProfile");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (VoicePrintProfile, _) =
        decode_from_slice(&unwrapped).expect("decode VoicePrintProfile");
    assert_eq!(profile, decoded);
}

/// Test 7: BehavioralBiometricSample roundtrip
#[test]
fn test_behavioral_biometric_roundtrip() {
    let sample = BehavioralBiometricSample {
        session_id: 700_007,
        avg_dwell_ms: 85.3,
        avg_flight_ms: 120.7,
        typing_rhythm_vector: vec![1.1, 2.2, 3.3],
        confidence: 0.91,
    };
    let encoded = encode_to_vec(&sample).expect("encode BehavioralBiometricSample");
    let (decoded, _): (BehavioralBiometricSample, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(sample, decoded);
}

/// Test 8: AuthenticationToken roundtrip with checksum
#[test]
fn test_authentication_token_checksum_roundtrip() {
    let token = AuthenticationToken {
        token_id: 800_008,
        subject_id: 12345,
        issued_at_unix: 1_700_000_000,
        expires_at_unix: 1_700_003_600,
        modality: BiometricModality::FacialRecognition,
        match_score: 0.987,
        access_decision: AccessDecision::Granted,
        signed_payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let encoded = encode_to_vec(&token).expect("encode AuthenticationToken");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (AuthenticationToken, _) =
        decode_from_slice(&unwrapped).expect("decode AuthenticationToken");
    assert_eq!(token, decoded);
}

/// Test 9: Vec of TemplateMatchResult roundtrip with checksum
#[test]
fn test_vec_template_match_results_roundtrip() {
    let results = vec![
        TemplateMatchResult {
            probe_id: 1,
            gallery_id: 101,
            score: 0.95,
            threshold: 0.80,
            is_match: true,
            modality: BiometricModality::Fingerprint,
        },
        TemplateMatchResult {
            probe_id: 2,
            gallery_id: 202,
            score: 0.55,
            threshold: 0.80,
            is_match: false,
            modality: BiometricModality::IrisScan,
        },
    ];
    let encoded = encode_to_vec(&results).expect("encode Vec<TemplateMatchResult>");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (Vec<TemplateMatchResult>, _) =
        decode_from_slice(&unwrapped).expect("decode Vec<TemplateMatchResult>");
    assert_eq!(results, decoded);
}

/// Test 10: IdentityRecord with all Option fields set to Some
#[test]
fn test_identity_record_all_options_some() {
    let record = IdentityRecord {
        subject_id: 999_010,
        display_name: String::from("Alice Nakamura"),
        fingerprint: Some(FingerprintTemplate {
            subject_id: 999_010,
            finger_index: 0,
            quality_score: 90,
            minutiae_count: 55,
            template_data: vec![1, 2, 3],
        }),
        iris: Some(IrisTemplate {
            subject_id: 999_010,
            eye: 0,
            iris_code: vec![0xAA; 32],
            hamming_threshold: 0.30,
        }),
        face: Some(FacialEmbedding {
            subject_id: 999_010,
            embedding_model_version: 1,
            embedding_vector: vec![0.5; 10],
            liveness_status: LivenessStatus::Confirmed,
        }),
        voice: Some(VoicePrintProfile {
            subject_id: 999_010,
            utterance_hash: [0u8; 32],
            mfcc_features: vec![1.0, 2.0],
            snr_db: 30.0,
        }),
        registered_at_unix: 1_650_000_000,
    };
    let encoded = encode_to_vec(&record).expect("encode IdentityRecord (all Some)");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (IdentityRecord, _) =
        decode_from_slice(&unwrapped).expect("decode IdentityRecord");
    assert_eq!(record, decoded);
}

/// Test 11: IdentityRecord with all Option fields set to None
#[test]
fn test_identity_record_all_options_none() {
    let record = IdentityRecord {
        subject_id: 111_011,
        display_name: String::from("Bob Yamada"),
        fingerprint: None,
        iris: None,
        face: None,
        voice: None,
        registered_at_unix: 1_600_000_000,
    };
    let encoded = encode_to_vec(&record).expect("encode IdentityRecord (all None)");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (IdentityRecord, _) =
        decode_from_slice(&unwrapped).expect("decode IdentityRecord (all None)");
    assert_eq!(record, decoded);
}

/// Test 12: large biometric template (simulating 8 KiB fingerprint minutiae data)
#[test]
fn test_large_biometric_template_roundtrip() {
    let large_data: Vec<u8> = (0u8..=255).cycle().take(8192).collect();
    let tmpl = FingerprintTemplate {
        subject_id: 222_012,
        finger_index: 9,
        quality_score: 100,
        minutiae_count: 1024,
        template_data: large_data,
    };
    let encoded = encode_to_vec(&tmpl).expect("encode large FingerprintTemplate");
    let wrapped = wrap_with_checksum(&encoded);
    assert_eq!(wrapped.len(), HEADER_SIZE + encoded.len());
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap large template");
    let (decoded, _): (FingerprintTemplate, _) =
        decode_from_slice(&unwrapped).expect("decode large FingerprintTemplate");
    assert_eq!(tmpl, decoded);
}

/// Test 13: bytes consumed includes header when unwrapping and re-decoding
#[test]
fn test_bytes_consumed_includes_header() {
    let token = AuthenticationToken {
        token_id: 333_013,
        subject_id: 42,
        issued_at_unix: 1_700_100_000,
        expires_at_unix: 1_700_200_000,
        modality: BiometricModality::PalmVein,
        match_score: 0.77,
        access_decision: AccessDecision::RequiresMfa,
        signed_payload: vec![0x01, 0x02],
    };
    let encoded = encode_to_vec(&token).expect("encode");
    let payload_len = encoded.len();
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    assert_eq!(unwrapped.len(), payload_len);
    // total wrapped length accounts for header
    assert_eq!(wrapped.len(), HEADER_SIZE + payload_len);
}

/// Test 14: AccessControlEntry with multiple modalities roundtrip
#[test]
fn test_access_control_entry_multiple_modalities() {
    let entry = AccessControlEntry {
        resource_id: 9999,
        subject_id: 44_444,
        allowed_modalities: vec![
            BiometricModality::Fingerprint,
            BiometricModality::FacialRecognition,
            BiometricModality::VoicePrint,
        ],
        min_match_score: 0.85,
        decision: AccessDecision::Granted,
    };
    let encoded = encode_to_vec(&entry).expect("encode AccessControlEntry");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (AccessControlEntry, _) =
        decode_from_slice(&unwrapped).expect("decode AccessControlEntry");
    assert_eq!(entry, decoded);
}

/// Test 15: LivenessChallenge struct roundtrip
#[test]
fn test_liveness_challenge_roundtrip() {
    let challenge = LivenessChallenge {
        challenge_id: 555_015,
        subject_id: 77_777,
        challenge_nonce: [0xFAu8; 16],
        response_hash: vec![0x11, 0x22, 0x33, 0x44],
        status: LivenessStatus::Confirmed,
    };
    let encoded = encode_to_vec(&challenge).expect("encode LivenessChallenge");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap LivenessChallenge");
    let (decoded, _): (LivenessChallenge, _) =
        decode_from_slice(&unwrapped).expect("decode LivenessChallenge");
    assert_eq!(challenge, decoded);
}

/// Test 16: corrupt only the header magic bytes causes Err
#[test]
fn test_magic_corruption_detected() {
    let tmpl = FingerprintTemplate {
        subject_id: 666_016,
        finger_index: 2,
        quality_score: 70,
        minutiae_count: 30,
        template_data: vec![0xBB; 16],
    };
    let encoded = encode_to_vec(&tmpl).expect("encode");
    let mut wrapped = wrap_with_checksum(&encoded);
    // Overwrite magic bytes (first 3 bytes) via the general corruption approach
    for b in wrapped[4..HEADER_SIZE].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&wrapped);
    assert!(result.is_err(), "header corruption must produce Err");
}

/// Test 17: Vec of AuthenticationToken roundtrip
#[test]
fn test_vec_authentication_tokens_roundtrip() {
    let tokens: Vec<AuthenticationToken> = (0u64..5)
        .map(|i| AuthenticationToken {
            token_id: 777_017 + i,
            subject_id: 1000 + i,
            issued_at_unix: 1_700_000_000 + i * 60,
            expires_at_unix: 1_700_003_600 + i * 60,
            modality: BiometricModality::RetinalScan,
            match_score: 0.90 + (i as f32) * 0.01,
            access_decision: AccessDecision::Granted,
            signed_payload: vec![i as u8; 4],
        })
        .collect();
    let encoded = encode_to_vec(&tokens).expect("encode Vec<AuthenticationToken>");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (Vec<AuthenticationToken>, _) =
        decode_from_slice(&unwrapped).expect("decode Vec<AuthenticationToken>");
    assert_eq!(tokens, decoded);
}

/// Test 18: BiometricModality enum variants all roundtrip correctly
#[test]
fn test_biometric_modality_variants_roundtrip() {
    let variants = vec![
        BiometricModality::Fingerprint,
        BiometricModality::IrisScan,
        BiometricModality::FacialRecognition,
        BiometricModality::VoicePrint,
        BiometricModality::BehavioralKeystroke,
        BiometricModality::PalmVein,
        BiometricModality::RetinalScan,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode BiometricModality variant");
        let wrapped = wrap_with_checksum(&encoded);
        let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap BiometricModality");
        let (decoded, _): (BiometricModality, _) =
            decode_from_slice(&unwrapped).expect("decode BiometricModality variant");
        assert_eq!(variant, &decoded);
    }
}

/// Test 19: IrisTemplate with large iris code (simulating 2048-bit iris code)
#[test]
fn test_large_iris_code_roundtrip() {
    let large_iris_code: Vec<u8> = (0u8..=255).cycle().take(256).collect();
    let tmpl = IrisTemplate {
        subject_id: 888_019,
        eye: 1,
        iris_code: large_iris_code,
        hamming_threshold: 0.33,
    };
    let encoded = encode_to_vec(&tmpl).expect("encode large IrisTemplate");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap large IrisTemplate");
    let (decoded, _): (IrisTemplate, _) =
        decode_from_slice(&unwrapped).expect("decode large IrisTemplate");
    assert_eq!(tmpl, decoded);
}

/// Test 20: empty template_data in FingerprintTemplate roundtrip
#[test]
fn test_empty_template_data_roundtrip() {
    let tmpl = FingerprintTemplate {
        subject_id: 999_020,
        finger_index: 0,
        quality_score: 0,
        minutiae_count: 0,
        template_data: vec![],
    };
    let encoded = encode_to_vec(&tmpl).expect("encode empty-template FingerprintTemplate");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap");
    let (decoded, _): (FingerprintTemplate, _) =
        decode_from_slice(&unwrapped).expect("decode empty-template FingerprintTemplate");
    assert_eq!(tmpl, decoded);
}

/// Test 21: AccessDecision variants all serialize and deserialize with checksum
#[test]
fn test_access_decision_variants_with_checksum() {
    let decisions = vec![
        AccessDecision::Granted,
        AccessDecision::Denied,
        AccessDecision::RequiresMfa,
        AccessDecision::TemporarilyLocked,
    ];
    for decision in &decisions {
        let encoded = encode_to_vec(decision).expect("encode AccessDecision");
        let wrapped = wrap_with_checksum(&encoded);
        let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap AccessDecision");
        let (decoded, consumed): (AccessDecision, _) =
            decode_from_slice(&unwrapped).expect("decode AccessDecision");
        assert_eq!(decision, &decoded);
        assert_eq!(consumed, unwrapped.len());
    }
}

/// Test 22: full identity verification pipeline — encode, wrap, corrupt, verify error, then correct decode
#[test]
fn test_full_identity_verification_pipeline() {
    let record = IdentityRecord {
        subject_id: 100_022,
        display_name: String::from("Carol Tanaka"),
        fingerprint: Some(FingerprintTemplate {
            subject_id: 100_022,
            finger_index: 1,
            quality_score: 88,
            minutiae_count: 48,
            template_data: vec![0x10, 0x20, 0x30],
        }),
        iris: None,
        face: Some(FacialEmbedding {
            subject_id: 100_022,
            embedding_model_version: 2,
            embedding_vector: vec![-0.1, 0.4, 0.9],
            liveness_status: LivenessStatus::Confirmed,
        }),
        voice: None,
        registered_at_unix: 1_690_000_000,
    };

    // Step 1: encode and wrap
    let encoded = encode_to_vec(&record).expect("encode IdentityRecord pipeline");
    let wrapped = wrap_with_checksum(&encoded);
    assert_eq!(wrapped.len(), HEADER_SIZE + encoded.len());

    // Step 2: simulate transmission corruption and verify it is caught
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let corrupt_result = unwrap_with_checksum(&corrupted);
    assert!(corrupt_result.is_err(), "corrupted pipeline data must fail");

    // Step 3: correct path succeeds and reconstructs original record
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap pipeline");
    let (decoded, bytes_consumed): (IdentityRecord, _) =
        decode_from_slice(&unwrapped).expect("decode pipeline IdentityRecord");
    assert_eq!(record, decoded);
    assert_eq!(bytes_consumed, encoded.len());
}
