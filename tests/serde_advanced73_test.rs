#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Domain types: Notary Public & Legal Document Management ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum NotarizationType {
    Acknowledgment,
    Jurat,
    Oath,
    Affirmation,
    CopyAttestation,
    SignatureWitnessing,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NotarizationRecord {
    record_id: u64,
    notary_name: String,
    commission_number: String,
    notarization_type: NotarizationType,
    signer_name: String,
    document_title: String,
    date_performed: String,
    county: String,
    state: String,
    fee_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PowerOfAttorneyScope {
    General,
    Limited,
    Durable,
    Springing,
    Medical,
    Financial,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PowerOfAttorneyDocument {
    document_id: u64,
    principal_name: String,
    agent_name: String,
    scope: PowerOfAttorneyScope,
    effective_date: String,
    expiration_date: Option<String>,
    is_revoked: bool,
    witness_names: Vec<String>,
    jurisdiction: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TrustType {
    Revocable,
    Irrevocable,
    Living,
    Testamentary,
    SpecialNeeds,
    Charitable,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WillTrustExecutionRecord {
    execution_id: u64,
    testator_name: String,
    trust_type: Option<TrustType>,
    executor_name: String,
    beneficiaries: Vec<String>,
    date_executed: String,
    witness_count: u8,
    notarized: bool,
    codicil_count: u8,
    estate_value_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RealEstateClosingDocument {
    closing_id: u64,
    property_address: String,
    buyer_name: String,
    seller_name: String,
    sale_price_cents: u64,
    closing_date: String,
    title_company: String,
    deed_type: String,
    parcels: Vec<String>,
    recording_number: Option<String>,
    escrow_amount_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ApostilleDocType {
    BirthCertificate,
    MarriageCertificate,
    DiplomaOrDegree,
    CourtOrder,
    CorporateDocument,
    PowerOfAttorney,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ApostilleCertification {
    apostille_id: u64,
    issuing_authority: String,
    document_type: ApostilleDocType,
    destination_country: String,
    bearer_name: String,
    issue_date: String,
    authentication_number: String,
    is_valid: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AffidavitRecord {
    affidavit_id: u64,
    affiant_name: String,
    subject_matter: String,
    sworn_date: String,
    notary_name: String,
    county: String,
    state: String,
    paragraph_count: u16,
    exhibits_attached: u8,
    penalties_acknowledged: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NotaryCommission {
    commission_id: u64,
    notary_name: String,
    state: String,
    commission_number: String,
    issue_date: String,
    expiration_date: String,
    bond_amount_cents: u64,
    surety_company: String,
    electronic_notary: bool,
    remote_online: bool,
    counties_authorized: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct JournalEntryLog {
    entry_id: u64,
    timestamp: String,
    signer_name: String,
    signer_address: String,
    id_type: String,
    id_number: String,
    document_type: String,
    notarization_type: NotarizationType,
    fee_charged_cents: u32,
    thumbprint_taken: bool,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum IdentityVerificationMethod {
    GovernmentPhotoId {
        id_type: String,
        id_number: String,
        expiry: String,
    },
    CredibleWitness {
        witness_name: String,
        relationship: String,
    },
    PersonalKnowledge,
    KnowledgeBasedAuth {
        provider: String,
        score: u8,
    },
    BiometricVerification {
        method: String,
        confidence_pct: f32,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct IdentityVerificationRecord {
    verification_id: u64,
    signer_name: String,
    method: IdentityVerificationMethod,
    verified_at: String,
    verified_by: String,
    passed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SignatureWitnessingRecord {
    witnessing_id: u64,
    signer_name: String,
    document_title: String,
    signing_date: String,
    witnesses: Vec<WitnessInfo>,
    location: String,
    voluntary_declaration: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WitnessInfo {
    name: String,
    address: String,
    relationship: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RonPlatform {
    Notarize,
    DocVerify,
    Pavaso,
    Nexsys,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ElectronicNotarizationSession {
    session_id: u64,
    platform: RonPlatform,
    notary_name: String,
    signer_name: String,
    start_time: String,
    end_time: String,
    audio_video_recording_url: String,
    credential_analysis_passed: bool,
    knowledge_based_auth_passed: bool,
    digital_certificate_serial: String,
    tamper_sealed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DocumentAuthenticationLink {
    link_id: u64,
    document_hash: String,
    authenticator_name: String,
    authentication_date: String,
    seal_type: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DocumentAuthenticationChain {
    chain_id: u64,
    original_document_title: String,
    links: Vec<DocumentAuthenticationLink>,
    final_status: String,
    chain_complete: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum LegalDocumentCategory {
    Contract,
    Deed,
    Affidavit,
    Will,
    Trust,
    PowerOfAttorney,
    CourtFiling,
    CorporateResolution,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LegalDocumentIndex {
    index_id: u64,
    category: LegalDocumentCategory,
    title: String,
    parties: Vec<String>,
    date_filed: String,
    recording_book: Option<String>,
    recording_page: Option<u32>,
    instrument_number: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NotarialCertificate {
    certificate_id: u64,
    notarization_type: NotarizationType,
    state: String,
    county: String,
    signer_appeared_before: bool,
    signer_identified: bool,
    signer_acknowledged_voluntary: bool,
    notary_seal_impression: String,
    notary_signature_date: String,
    commission_expiry: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EscrowInstruction {
    instruction_id: u64,
    escrow_number: String,
    buyer_name: String,
    seller_name: String,
    escrow_agent: String,
    deposit_amount_cents: u64,
    conditions: Vec<String>,
    deadline_date: String,
    amendments: Vec<EscrowAmendment>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EscrowAmendment {
    amendment_id: u32,
    description: String,
    effective_date: String,
    approved_by_all: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DepositionSummary {
    deposition_id: u64,
    case_number: String,
    deponent_name: String,
    attorney_conducting: String,
    court_reporter: String,
    date_taken: String,
    location: String,
    page_count: u32,
    exhibits_referenced: Vec<String>,
    objections_count: u32,
    sworn_under_oath: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NotaryFeeSchedule {
    schedule_id: u32,
    state: String,
    effective_date: String,
    acknowledgment_fee_cents: u32,
    jurat_fee_cents: u32,
    oath_fee_cents: u32,
    copy_attestation_fee_cents: u32,
    travel_fee_per_mile_cents: u32,
    max_total_fee_cents: Option<u32>,
}

// --- Tests ---

#[test]
fn test_notarization_record_acknowledgment() {
    let record = NotarizationRecord {
        record_id: 100001,
        notary_name: "Maria Gonzalez".to_string(),
        commission_number: "NP-2025-48291".to_string(),
        notarization_type: NotarizationType::Acknowledgment,
        signer_name: "John D. Henderson".to_string(),
        document_title: "Grant Deed - 1450 Oak Avenue".to_string(),
        date_performed: "2026-03-10".to_string(),
        county: "Los Angeles".to_string(),
        state: "California".to_string(),
        fee_cents: 1500,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("failed to encode notarization record");
    let (decoded, _): (NotarizationRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode notarization record");
    assert_eq!(record, decoded);
}

#[test]
fn test_notarization_record_jurat() {
    let record = NotarizationRecord {
        record_id: 100002,
        notary_name: "Robert Kim".to_string(),
        commission_number: "NP-2024-77120".to_string(),
        notarization_type: NotarizationType::Jurat,
        signer_name: "Patricia Wells".to_string(),
        document_title: "Affidavit of Heirship".to_string(),
        date_performed: "2026-02-28".to_string(),
        county: "Cook".to_string(),
        state: "Illinois".to_string(),
        fee_cents: 1000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("failed to encode jurat record");
    let (decoded, _): (NotarizationRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode jurat record");
    assert_eq!(record, decoded);
}

#[test]
fn test_power_of_attorney_durable() {
    let poa = PowerOfAttorneyDocument {
        document_id: 200001,
        principal_name: "Eleanor M. Whitfield".to_string(),
        agent_name: "Thomas R. Whitfield".to_string(),
        scope: PowerOfAttorneyScope::Durable,
        effective_date: "2026-01-15".to_string(),
        expiration_date: None,
        is_revoked: false,
        witness_names: vec!["Sarah Johnson".to_string(), "Michael Chen".to_string()],
        jurisdiction: "State of New York".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&poa, cfg).expect("failed to encode power of attorney");
    let (decoded, _): (PowerOfAttorneyDocument, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode power of attorney");
    assert_eq!(poa, decoded);
}

#[test]
fn test_power_of_attorney_limited_with_expiration() {
    let poa = PowerOfAttorneyDocument {
        document_id: 200002,
        principal_name: "David Nakamura".to_string(),
        agent_name: "Keiko Nakamura-Smith".to_string(),
        scope: PowerOfAttorneyScope::Limited,
        effective_date: "2026-03-01".to_string(),
        expiration_date: Some("2026-09-01".to_string()),
        is_revoked: false,
        witness_names: vec!["Alan Brooks".to_string()],
        jurisdiction: "State of Texas".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&poa, cfg).expect("failed to encode limited POA");
    let (decoded, _): (PowerOfAttorneyDocument, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode limited POA");
    assert_eq!(poa, decoded);
}

#[test]
fn test_will_trust_execution_revocable_trust() {
    let record = WillTrustExecutionRecord {
        execution_id: 300001,
        testator_name: "Harold F. Morrison".to_string(),
        trust_type: Some(TrustType::Revocable),
        executor_name: "First National Trust Co.".to_string(),
        beneficiaries: vec![
            "Linda Morrison".to_string(),
            "James Morrison".to_string(),
            "Morrison Family Foundation".to_string(),
        ],
        date_executed: "2026-02-14".to_string(),
        witness_count: 2,
        notarized: true,
        codicil_count: 0,
        estate_value_cents: 250_000_000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("failed to encode will/trust record");
    let (decoded, _): (WillTrustExecutionRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode will/trust record");
    assert_eq!(record, decoded);
}

#[test]
fn test_real_estate_closing_document() {
    let closing = RealEstateClosingDocument {
        closing_id: 400001,
        property_address: "2718 Elm Street, Austin, TX 78701".to_string(),
        buyer_name: "Angela R. Prescott".to_string(),
        seller_name: "Riverside Development LLC".to_string(),
        sale_price_cents: 52_500_000,
        closing_date: "2026-03-12".to_string(),
        title_company: "Lone Star Title & Escrow".to_string(),
        deed_type: "General Warranty Deed".to_string(),
        parcels: vec![
            "Travis-0412-005-0031".to_string(),
            "Travis-0412-005-0032".to_string(),
        ],
        recording_number: Some("2026-0312-004891".to_string()),
        escrow_amount_cents: 5_250_000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&closing, cfg).expect("failed to encode closing document");
    let (decoded, _): (RealEstateClosingDocument, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode closing document");
    assert_eq!(closing, decoded);
}

#[test]
fn test_apostille_certification_birth_certificate() {
    let apostille = ApostilleCertification {
        apostille_id: 500001,
        issuing_authority: "Secretary of State, California".to_string(),
        document_type: ApostilleDocType::BirthCertificate,
        destination_country: "Germany".to_string(),
        bearer_name: "Sophia K. Reiter".to_string(),
        issue_date: "2026-03-05".to_string(),
        authentication_number: "APO-CA-2026-0018472".to_string(),
        is_valid: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&apostille, cfg).expect("failed to encode apostille");
    let (decoded, _): (ApostilleCertification, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode apostille");
    assert_eq!(apostille, decoded);
}

#[test]
fn test_apostille_certification_other_type() {
    let apostille = ApostilleCertification {
        apostille_id: 500002,
        issuing_authority: "Secretary of State, New York".to_string(),
        document_type: ApostilleDocType::Other("Adoption Decree".to_string()),
        destination_country: "Japan".to_string(),
        bearer_name: "Yuki Tanaka-Williams".to_string(),
        issue_date: "2026-01-22".to_string(),
        authentication_number: "APO-NY-2026-0004218".to_string(),
        is_valid: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&apostille, cfg).expect("failed to encode other-type apostille");
    let (decoded, _): (ApostilleCertification, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode other-type apostille");
    assert_eq!(apostille, decoded);
}

#[test]
fn test_affidavit_record() {
    let affidavit = AffidavitRecord {
        affidavit_id: 600001,
        affiant_name: "Margaret O'Brien".to_string(),
        subject_matter: "Declaration of domicile and residency".to_string(),
        sworn_date: "2026-03-08".to_string(),
        notary_name: "Carlos Mendoza".to_string(),
        county: "Miami-Dade".to_string(),
        state: "Florida".to_string(),
        paragraph_count: 12,
        exhibits_attached: 3,
        penalties_acknowledged: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&affidavit, cfg).expect("failed to encode affidavit");
    let (decoded, _): (AffidavitRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode affidavit");
    assert_eq!(affidavit, decoded);
}

#[test]
fn test_notary_commission_tracking() {
    let commission = NotaryCommission {
        commission_id: 700001,
        notary_name: "Jennifer L. Park".to_string(),
        state: "Washington".to_string(),
        commission_number: "WA-NOT-2024-091744".to_string(),
        issue_date: "2024-06-01".to_string(),
        expiration_date: "2028-06-01".to_string(),
        bond_amount_cents: 1_000_000,
        surety_company: "Pacific Surety Group".to_string(),
        electronic_notary: true,
        remote_online: true,
        counties_authorized: vec![
            "King".to_string(),
            "Pierce".to_string(),
            "Snohomish".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&commission, cfg).expect("failed to encode commission");
    let (decoded, _): (NotaryCommission, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode commission");
    assert_eq!(commission, decoded);
}

#[test]
fn test_journal_entry_log() {
    let entry = JournalEntryLog {
        entry_id: 800001,
        timestamp: "2026-03-14T09:30:00Z".to_string(),
        signer_name: "Franklin D. Ayers".to_string(),
        signer_address: "890 Birch Lane, Portland, OR 97201".to_string(),
        id_type: "Oregon Driver License".to_string(),
        id_number: "OR-8812-4490".to_string(),
        document_type: "Quitclaim Deed".to_string(),
        notarization_type: NotarizationType::Acknowledgment,
        fee_charged_cents: 1500,
        thumbprint_taken: true,
        notes: Some("Signer appeared in person at mobile signing".to_string()),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&entry, cfg).expect("failed to encode journal entry");
    let (decoded, _): (JournalEntryLog, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode journal entry");
    assert_eq!(entry, decoded);
}

#[test]
fn test_identity_verification_government_id() {
    let verification = IdentityVerificationRecord {
        verification_id: 900001,
        signer_name: "Christine Larsson".to_string(),
        method: IdentityVerificationMethod::GovernmentPhotoId {
            id_type: "US Passport".to_string(),
            id_number: "C04817293".to_string(),
            expiry: "2031-08-22".to_string(),
        },
        verified_at: "2026-03-14T10:15:00Z".to_string(),
        verified_by: "Notary Maria Gonzalez".to_string(),
        passed: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&verification, cfg).expect("failed to encode ID verification");
    let (decoded, _): (IdentityVerificationRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode ID verification");
    assert_eq!(verification, decoded);
}

#[test]
fn test_identity_verification_knowledge_based_auth() {
    let verification = IdentityVerificationRecord {
        verification_id: 900002,
        signer_name: "Dennis W. Okoro".to_string(),
        method: IdentityVerificationMethod::KnowledgeBasedAuth {
            provider: "IDology".to_string(),
            score: 92,
        },
        verified_at: "2026-03-13T14:00:00Z".to_string(),
        verified_by: "RON Platform - Notarize".to_string(),
        passed: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&verification, cfg).expect("failed to encode KBA verification");
    let (decoded, _): (IdentityVerificationRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode KBA verification");
    assert_eq!(verification, decoded);
}

#[test]
fn test_signature_witnessing_record() {
    let witnessing = SignatureWitnessingRecord {
        witnessing_id: 1000001,
        signer_name: "George B. Whitman".to_string(),
        document_title: "Last Will and Testament of George B. Whitman".to_string(),
        signing_date: "2026-03-11".to_string(),
        witnesses: vec![
            WitnessInfo {
                name: "Alice Fernandez".to_string(),
                address: "312 Pine St, Denver, CO 80202".to_string(),
                relationship: None,
            },
            WitnessInfo {
                name: "Rajesh Patel".to_string(),
                address: "715 Maple Ave, Denver, CO 80203".to_string(),
                relationship: Some("neighbor".to_string()),
            },
        ],
        location: "Law Offices of Whitman & Associates, Denver, CO".to_string(),
        voluntary_declaration: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&witnessing, cfg).expect("failed to encode witnessing record");
    let (decoded, _): (SignatureWitnessingRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode witnessing record");
    assert_eq!(witnessing, decoded);
}

#[test]
fn test_electronic_notarization_session() {
    let session = ElectronicNotarizationSession {
        session_id: 1100001,
        platform: RonPlatform::Notarize,
        notary_name: "Samantha J. Reed".to_string(),
        signer_name: "Victor Andersen".to_string(),
        start_time: "2026-03-14T11:00:00Z".to_string(),
        end_time: "2026-03-14T11:28:00Z".to_string(),
        audio_video_recording_url: "https://vault.notarize.com/sessions/abc123def456".to_string(),
        credential_analysis_passed: true,
        knowledge_based_auth_passed: true,
        digital_certificate_serial: "SN-2026-0314-REED-0041".to_string(),
        tamper_sealed: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&session, cfg).expect("failed to encode RON session");
    let (decoded, _): (ElectronicNotarizationSession, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode RON session");
    assert_eq!(session, decoded);
}

#[test]
fn test_electronic_notarization_custom_platform() {
    let session = ElectronicNotarizationSession {
        session_id: 1100002,
        platform: RonPlatform::Custom("StateNotary Pro v3".to_string()),
        notary_name: "Liam O'Donnell".to_string(),
        signer_name: "Beatrice Fontaine".to_string(),
        start_time: "2026-03-13T16:00:00Z".to_string(),
        end_time: "2026-03-13T16:42:00Z".to_string(),
        audio_video_recording_url: "s3://snp-vault/session-1100002.webm".to_string(),
        credential_analysis_passed: true,
        knowledge_based_auth_passed: true,
        digital_certificate_serial: "SN-2026-0313-ODON-0019".to_string(),
        tamper_sealed: true,
    };
    let cfg = config::standard();
    let encoded =
        encode_to_vec(&session, cfg).expect("failed to encode custom-platform RON session");
    let (decoded, _): (ElectronicNotarizationSession, _) = decode_owned_from_slice(&encoded, cfg)
        .expect("failed to decode custom-platform RON session");
    assert_eq!(session, decoded);
}

#[test]
fn test_document_authentication_chain() {
    let chain = DocumentAuthenticationChain {
        chain_id: 1200001,
        original_document_title: "Articles of Incorporation - Horizon Tech LLC".to_string(),
        links: vec![
            DocumentAuthenticationLink {
                link_id: 1,
                document_hash: "sha256:a1b2c3d4e5f6...".to_string(),
                authenticator_name: "County Clerk, Harris County".to_string(),
                authentication_date: "2026-02-20".to_string(),
                seal_type: "County Seal".to_string(),
            },
            DocumentAuthenticationLink {
                link_id: 2,
                document_hash: "sha256:f6e5d4c3b2a1...".to_string(),
                authenticator_name: "Secretary of State, Texas".to_string(),
                authentication_date: "2026-02-25".to_string(),
                seal_type: "State Authentication".to_string(),
            },
            DocumentAuthenticationLink {
                link_id: 3,
                document_hash: "sha256:1a2b3c4d5e6f...".to_string(),
                authenticator_name: "US Department of State".to_string(),
                authentication_date: "2026-03-01".to_string(),
                seal_type: "Apostille".to_string(),
            },
        ],
        final_status: "Fully Authenticated".to_string(),
        chain_complete: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&chain, cfg).expect("failed to encode authentication chain");
    let (decoded, _): (DocumentAuthenticationChain, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode authentication chain");
    assert_eq!(chain, decoded);
}

#[test]
fn test_legal_document_index() {
    let index = LegalDocumentIndex {
        index_id: 1300001,
        category: LegalDocumentCategory::Deed,
        title: "Warranty Deed - Lot 14 Block 7 Sunset Estates".to_string(),
        parties: vec![
            "Ronald F. Ingram (Grantor)".to_string(),
            "Cheryl A. Donovan (Grantee)".to_string(),
        ],
        date_filed: "2026-03-10".to_string(),
        recording_book: Some("Book 4412".to_string()),
        recording_page: Some(271),
        instrument_number: Some("2026-0310-001847".to_string()),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&index, cfg).expect("failed to encode legal document index");
    let (decoded, _): (LegalDocumentIndex, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode legal document index");
    assert_eq!(index, decoded);
}

#[test]
fn test_notarial_certificate() {
    let cert = NotarialCertificate {
        certificate_id: 1400001,
        notarization_type: NotarizationType::Oath,
        state: "Massachusetts".to_string(),
        county: "Suffolk".to_string(),
        signer_appeared_before: true,
        signer_identified: true,
        signer_acknowledged_voluntary: true,
        notary_seal_impression: "JENNIFER PARK - NOTARY PUBLIC - COMMONWEALTH OF MASSACHUSETTS - MY COMMISSION EXPIRES 06/01/2028".to_string(),
        notary_signature_date: "2026-03-14".to_string(),
        commission_expiry: "2028-06-01".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&cert, cfg).expect("failed to encode notarial certificate");
    let (decoded, _): (NotarialCertificate, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode notarial certificate");
    assert_eq!(cert, decoded);
}

#[test]
fn test_escrow_instructions_with_amendments() {
    let instructions = EscrowInstruction {
        instruction_id: 1500001,
        escrow_number: "ESC-2026-041872".to_string(),
        buyer_name: "Nathan and Lisa Choi".to_string(),
        seller_name: "Greenfield Properties Inc.".to_string(),
        escrow_agent: "Pacific Coast Escrow Services".to_string(),
        deposit_amount_cents: 7_500_000,
        conditions: vec![
            "Buyer to obtain financing within 21 days".to_string(),
            "Property inspection satisfactory to buyer".to_string(),
            "Title search clear of liens".to_string(),
            "HOA documents reviewed and approved".to_string(),
        ],
        deadline_date: "2026-04-15".to_string(),
        amendments: vec![
            EscrowAmendment {
                amendment_id: 1,
                description: "Extended financing contingency to 30 days".to_string(),
                effective_date: "2026-03-20".to_string(),
                approved_by_all: true,
            },
            EscrowAmendment {
                amendment_id: 2,
                description: "Reduced purchase price by $15,000 per inspection findings"
                    .to_string(),
                effective_date: "2026-03-25".to_string(),
                approved_by_all: true,
            },
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&instructions, cfg).expect("failed to encode escrow instructions");
    let (decoded, _): (EscrowInstruction, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode escrow instructions");
    assert_eq!(instructions, decoded);
}

#[test]
fn test_deposition_summary() {
    let deposition = DepositionSummary {
        deposition_id: 1600001,
        case_number: "2025-CV-04817".to_string(),
        deponent_name: "Dr. Richard Halverson".to_string(),
        attorney_conducting: "Amanda Liu, Esq.".to_string(),
        court_reporter: "Certified Reporting Inc. - Reporter #4419".to_string(),
        date_taken: "2026-03-07".to_string(),
        location: "Conference Room B, 500 Market St, San Francisco, CA".to_string(),
        page_count: 247,
        exhibits_referenced: vec![
            "Exhibit A - Contract dated 2024-06-15".to_string(),
            "Exhibit B - Email correspondence chain".to_string(),
            "Exhibit C - Financial statements Q3 2025".to_string(),
            "Exhibit D - Photographs of property condition".to_string(),
        ],
        objections_count: 34,
        sworn_under_oath: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&deposition, cfg).expect("failed to encode deposition summary");
    let (decoded, _): (DepositionSummary, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode deposition summary");
    assert_eq!(deposition, decoded);
}

#[test]
fn test_notary_fee_schedule() {
    let schedule = NotaryFeeSchedule {
        schedule_id: 1700,
        state: "California".to_string(),
        effective_date: "2026-01-01".to_string(),
        acknowledgment_fee_cents: 1500,
        jurat_fee_cents: 1500,
        oath_fee_cents: 1500,
        copy_attestation_fee_cents: 1500,
        travel_fee_per_mile_cents: 0,
        max_total_fee_cents: Some(1500),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&schedule, cfg).expect("failed to encode fee schedule");
    let (decoded, _): (NotaryFeeSchedule, _) =
        decode_owned_from_slice(&encoded, cfg).expect("failed to decode fee schedule");
    assert_eq!(schedule, decoded);
}
