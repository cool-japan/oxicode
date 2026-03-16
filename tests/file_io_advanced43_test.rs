#![cfg(feature = "std")]
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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ── Domain Types: Legal Technology & Case Management ────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CaseStatus {
    Filed,
    Discovery,
    PreTrial,
    Trial,
    PostTrial,
    Settled,
    Dismissed,
    Appealed,
    Closed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DocketEntry {
    entry_number: u32,
    case_number: String,
    filing_date_epoch: i64,
    description: String,
    filed_by: String,
    status: CaseStatus,
    sealed: bool,
    page_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReviewClassification {
    Privileged,
    Relevant,
    Responsive,
    NonResponsive,
    Redacted,
    NeedsSecondReview,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DocumentReview {
    document_id: String,
    reviewer_id: String,
    classification: ReviewClassification,
    confidence_pct: u8,
    notes: String,
    review_time_secs: u32,
    batch_id: u64,
    custodian: String,
    date_range_start_epoch: i64,
    date_range_end_epoch: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DepositionSchedule {
    case_id: String,
    deponent_name: String,
    scheduled_epoch: i64,
    location: String,
    court_reporter: String,
    videographer_present: bool,
    estimated_hours: u8,
    attorneys_present: Vec<String>,
    topics: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FilingType {
    Complaint,
    Answer,
    Motion,
    Brief,
    Memorandum,
    Stipulation,
    Order,
    Judgment,
    Notice,
    Subpoena,
    Exhibit,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CourtFiling {
    filing_id: u64,
    case_number: String,
    filing_type: FilingType,
    title: String,
    filed_epoch: i64,
    court_name: String,
    judge_name: String,
    electronic: bool,
    confidential: bool,
    attachment_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EDiscoverySearch {
    search_id: u64,
    query_terms: Vec<String>,
    date_range_start: i64,
    date_range_end: i64,
    custodians: Vec<String>,
    file_types: Vec<String>,
    min_confidence: u8,
    max_results: u32,
    include_metadata: bool,
    deduplicate: bool,
    near_duplicate_threshold_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClauseType {
    Indemnification,
    LimitationOfLiability,
    Termination,
    ChangeOfControl,
    NonCompete,
    Confidentiality,
    ForceMAjeure,
    GoverningLaw,
    DisputeResolution,
    Assignment,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContractClause {
    clause_id: u32,
    contract_id: String,
    clause_type: ClauseType,
    text_excerpt: String,
    page_number: u16,
    risk_score: u8,
    flagged: bool,
    reviewer_notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TimeEntryCategory {
    Billable,
    NonBillable,
    ProBono,
    AdminOverhead,
    CourtAppearance,
    ClientMeeting,
    Research,
    Drafting,
    Review,
    Travel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BillingTimeEntry {
    entry_id: u64,
    attorney_id: String,
    case_id: String,
    category: TimeEntryCategory,
    description: String,
    duration_minutes: u32,
    rate_cents_per_hour: u32,
    date_epoch: i64,
    approved: bool,
    invoice_id: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConflictCheck {
    check_id: u64,
    requesting_attorney: String,
    prospective_client: String,
    adverse_parties: Vec<String>,
    related_entities: Vec<String>,
    conflict_found: bool,
    conflicting_case_ids: Vec<String>,
    waiver_obtained: bool,
    check_date_epoch: i64,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StatuteOfLimitations {
    jurisdiction: String,
    cause_of_action: String,
    limitation_years: u8,
    accrual_date_epoch: i64,
    deadline_epoch: i64,
    tolling_days: u32,
    tolling_reason: Option<String>,
    case_id: String,
    alert_sent: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JurorProfile {
    juror_id: u32,
    panel_number: u16,
    occupation: String,
    age_range: String,
    prior_jury_service: bool,
    hardship_claim: bool,
    cause_challenge: bool,
    peremptory_strike: bool,
    seated: bool,
    questionnaire_score: u8,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SettlementRecord {
    case_id: String,
    offer_number: u16,
    offered_by_plaintiff: bool,
    amount_cents: u64,
    offer_date_epoch: i64,
    expiration_epoch: i64,
    accepted: bool,
    conditions: Vec<String>,
    mediator_name: Option<String>,
    confidential: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PatentClaim {
    patent_number: String,
    claim_number: u16,
    independent: bool,
    claim_text: String,
    dependent_on: Vec<u16>,
    keywords: Vec<String>,
    prior_art_refs: Vec<String>,
    validity_score: u8,
    infringement_score: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ComplianceItem {
    item_id: u32,
    regulation_code: String,
    description: String,
    compliant: bool,
    evidence_doc_ids: Vec<String>,
    reviewer: String,
    review_date_epoch: i64,
    remediation_deadline_epoch: Option<i64>,
    severity: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ComplianceChecklist {
    checklist_id: u64,
    entity_name: String,
    framework: String,
    items: Vec<ComplianceItem>,
    overall_score_pct: u8,
    last_audit_epoch: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LegalHoldNotice {
    hold_id: u64,
    case_id: String,
    hold_name: String,
    issued_epoch: i64,
    custodians: Vec<String>,
    data_sources: Vec<String>,
    active: bool,
    reminder_interval_days: u16,
    last_reminder_epoch: Option<i64>,
    acknowledged_by: Vec<String>,
    scope_description: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrecedentCitation {
    citing_case: String,
    cited_case: String,
    citation_string: String,
    treatment: String,
    relevance_score: u8,
    headnote_topics: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrecedentNetwork {
    root_case: String,
    citations: Vec<PrecedentCitation>,
    depth: u8,
    total_cases: u32,
    jurisdiction_filter: Option<String>,
}

// ── Test 1: Docket entries via file ─────────────────────────────────────────

#[test]
fn test_docket_entries_file_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio43_t1_{}.bin", std::process::id()));
    let entries = vec![
        DocketEntry {
            entry_number: 1,
            case_number: "2025-CV-04421".into(),
            filing_date_epoch: 1748000000,
            description: "Complaint for Patent Infringement".into(),
            filed_by: "Plaintiff Corp".into(),
            status: CaseStatus::Filed,
            sealed: false,
            page_count: 47,
        },
        DocketEntry {
            entry_number: 2,
            case_number: "2025-CV-04421".into(),
            filing_date_epoch: 1748100000,
            description: "Motion to Dismiss for Failure to State a Claim".into(),
            filed_by: "Defendant LLC".into(),
            status: CaseStatus::Discovery,
            sealed: false,
            page_count: 22,
        },
        DocketEntry {
            entry_number: 3,
            case_number: "2025-CV-04421".into(),
            filing_date_epoch: 1748200000,
            description: "Sealed Ex-Parte Application".into(),
            filed_by: "Plaintiff Corp".into(),
            status: CaseStatus::Discovery,
            sealed: true,
            page_count: 8,
        },
    ];
    encode_to_file(&entries, &path).expect("encode docket entries to file");
    let decoded: Vec<DocketEntry> = decode_from_file(&path).expect("decode docket entries");
    assert_eq!(entries, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 2: Document review workflow via slice ──────────────────────────────

#[test]
fn test_document_review_workflow_slice() {
    let reviews = vec![
        DocumentReview {
            document_id: "DOC-00001".into(),
            reviewer_id: "ATT-042".into(),
            classification: ReviewClassification::Privileged,
            confidence_pct: 95,
            notes: "Attorney-client communication re: merger strategy".into(),
            review_time_secs: 120,
            batch_id: 5001,
            custodian: "J. Smith".into(),
            date_range_start_epoch: 1700000000,
            date_range_end_epoch: 1710000000,
        },
        DocumentReview {
            document_id: "DOC-00002".into(),
            reviewer_id: "ATT-042".into(),
            classification: ReviewClassification::Responsive,
            confidence_pct: 88,
            notes: "Relevant financial projections".into(),
            review_time_secs: 45,
            batch_id: 5001,
            custodian: "J. Smith".into(),
            date_range_start_epoch: 1700000000,
            date_range_end_epoch: 1710000000,
        },
        DocumentReview {
            document_id: "DOC-00003".into(),
            reviewer_id: "ATT-017".into(),
            classification: ReviewClassification::NonResponsive,
            confidence_pct: 72,
            notes: "Personal email, no business content".into(),
            review_time_secs: 15,
            batch_id: 5001,
            custodian: "A. Lee".into(),
            date_range_start_epoch: 1700000000,
            date_range_end_epoch: 1710000000,
        },
    ];
    let encoded = encode_to_vec(&reviews).expect("encode reviews to vec");
    let (decoded, bytes_read): (Vec<DocumentReview>, usize) =
        decode_from_slice(&encoded).expect("decode reviews from slice");
    assert_eq!(reviews, decoded);
    assert!(bytes_read > 0);
}

// ── Test 3: Deposition scheduling via file ──────────────────────────────────

#[test]
fn test_deposition_schedule_file_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio43_t3_{}.bin", std::process::id()));
    let depo = DepositionSchedule {
        case_id: "2025-CV-04421".into(),
        deponent_name: "Dr. Robert Chen".into(),
        scheduled_epoch: 1755000000,
        location: "Suite 1200, 555 Market St, San Francisco, CA 94105".into(),
        court_reporter: "Acme Reporting Services".into(),
        videographer_present: true,
        estimated_hours: 7,
        attorneys_present: vec![
            "Sarah Williams (Lead Counsel)".into(),
            "Mark Johnson (Associate)".into(),
            "Lisa Park (Defense Counsel)".into(),
        ],
        topics: vec![
            "Product design decisions Q3-Q4 2024".into(),
            "Internal testing protocols".into(),
            "Communications with regulatory bodies".into(),
        ],
    };
    encode_to_file(&depo, &path).expect("encode deposition schedule");
    let decoded: DepositionSchedule = decode_from_file(&path).expect("decode deposition schedule");
    assert_eq!(depo, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 4: Court filing metadata via slice ─────────────────────────────────

#[test]
fn test_court_filing_metadata_slice() {
    let filings = vec![
        CourtFiling {
            filing_id: 10001,
            case_number: "2025-CV-04421".into(),
            filing_type: FilingType::Motion,
            title: "Motion for Summary Judgment".into(),
            filed_epoch: 1750000000,
            court_name: "U.S. District Court, Northern District of California".into(),
            judge_name: "Hon. Patricia Martinez".into(),
            electronic: true,
            confidential: false,
            attachment_count: 12,
        },
        CourtFiling {
            filing_id: 10002,
            case_number: "2025-CV-04421".into(),
            filing_type: FilingType::Brief,
            title: "Opposition Brief to Motion for Summary Judgment".into(),
            filed_epoch: 1750500000,
            court_name: "U.S. District Court, Northern District of California".into(),
            judge_name: "Hon. Patricia Martinez".into(),
            electronic: true,
            confidential: false,
            attachment_count: 8,
        },
    ];
    let encoded = encode_to_vec(&filings).expect("encode court filings");
    let (decoded, _): (Vec<CourtFiling>, usize) =
        decode_from_slice(&encoded).expect("decode court filings");
    assert_eq!(filings, decoded);
}

// ── Test 5: E-discovery search parameters via file ──────────────────────────

#[test]
fn test_ediscovery_search_params_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t5_{}.bin", std::process::id()));
    let search = EDiscoverySearch {
        search_id: 77001,
        query_terms: vec![
            "merger".into(),
            "acquisition".into(),
            "due diligence".into(),
            "valuation".into(),
        ],
        date_range_start: 1690000000,
        date_range_end: 1720000000,
        custodians: vec!["jsmith@corp.com".into(), "alee@corp.com".into()],
        file_types: vec!["eml".into(), "pst".into(), "docx".into(), "xlsx".into()],
        min_confidence: 70,
        max_results: 50000,
        include_metadata: true,
        deduplicate: true,
        near_duplicate_threshold_pct: 85,
    };
    encode_to_file(&search, &path).expect("encode e-discovery search");
    let decoded: EDiscoverySearch = decode_from_file(&path).expect("decode e-discovery search");
    assert_eq!(search, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 6: Contract clause extraction via slice ────────────────────────────

#[test]
fn test_contract_clause_extraction_slice() {
    let clauses = vec![
        ContractClause {
            clause_id: 1,
            contract_id: "MSA-2025-0091".into(),
            clause_type: ClauseType::Indemnification,
            text_excerpt: "Party A shall indemnify and hold harmless Party B from any \
                and all claims, damages, losses arising out of..."
                .into(),
            page_number: 12,
            risk_score: 78,
            flagged: true,
            reviewer_notes: "Broad indemnification scope; consider carve-outs".into(),
        },
        ContractClause {
            clause_id: 2,
            contract_id: "MSA-2025-0091".into(),
            clause_type: ClauseType::LimitationOfLiability,
            text_excerpt: "In no event shall either party's aggregate liability exceed \
                the total fees paid in the twelve months preceding..."
                .into(),
            page_number: 14,
            risk_score: 45,
            flagged: false,
            reviewer_notes: "Standard 12-month lookback cap".into(),
        },
        ContractClause {
            clause_id: 3,
            contract_id: "MSA-2025-0091".into(),
            clause_type: ClauseType::NonCompete,
            text_excerpt: "For a period of two years following termination, neither party \
                shall solicit or engage..."
                .into(),
            page_number: 18,
            risk_score: 92,
            flagged: true,
            reviewer_notes: "Overly broad geographic and temporal scope".into(),
        },
    ];
    let encoded = encode_to_vec(&clauses).expect("encode contract clauses");
    let (decoded, bytes_read): (Vec<ContractClause>, usize) =
        decode_from_slice(&encoded).expect("decode contract clauses");
    assert_eq!(clauses, decoded);
    assert_eq!(bytes_read, encoded.len());
}

// ── Test 7: Billing time entries via file ───────────────────────────────────

#[test]
fn test_billing_time_entries_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t7_{}.bin", std::process::id()));
    let entries = vec![
        BillingTimeEntry {
            entry_id: 90001,
            attorney_id: "ATT-042".into(),
            case_id: "2025-CV-04421".into(),
            category: TimeEntryCategory::Research,
            description: "Research prior art for patent claims 1-5".into(),
            duration_minutes: 180,
            rate_cents_per_hour: 65000,
            date_epoch: 1748000000,
            approved: true,
            invoice_id: Some(5500),
        },
        BillingTimeEntry {
            entry_id: 90002,
            attorney_id: "ATT-042".into(),
            case_id: "2025-CV-04421".into(),
            category: TimeEntryCategory::Drafting,
            description: "Draft claim construction brief".into(),
            duration_minutes: 240,
            rate_cents_per_hour: 65000,
            date_epoch: 1748100000,
            approved: true,
            invoice_id: Some(5500),
        },
        BillingTimeEntry {
            entry_id: 90003,
            attorney_id: "ATT-042".into(),
            case_id: "ADMIN".into(),
            category: TimeEntryCategory::NonBillable,
            description: "Firm-wide CLE training session".into(),
            duration_minutes: 120,
            rate_cents_per_hour: 0,
            date_epoch: 1748200000,
            approved: true,
            invoice_id: None,
        },
        BillingTimeEntry {
            entry_id: 90004,
            attorney_id: "ATT-017".into(),
            case_id: "2025-CV-04421".into(),
            category: TimeEntryCategory::CourtAppearance,
            description: "Markman hearing attendance".into(),
            duration_minutes: 300,
            rate_cents_per_hour: 55000,
            date_epoch: 1748300000,
            approved: false,
            invoice_id: None,
        },
    ];
    encode_to_file(&entries, &path).expect("encode billing entries");
    let decoded: Vec<BillingTimeEntry> = decode_from_file(&path).expect("decode billing entries");
    assert_eq!(entries, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 8: Conflict of interest check via slice ────────────────────────────

#[test]
fn test_conflict_check_slice() {
    let check = ConflictCheck {
        check_id: 3001,
        requesting_attorney: "ATT-042".into(),
        prospective_client: "Acme Technologies Inc.".into(),
        adverse_parties: vec!["Beta Innovations LLC".into(), "Gamma Research Corp.".into()],
        related_entities: vec![
            "Acme Holdings (parent)".into(),
            "Acme Ventures (subsidiary)".into(),
        ],
        conflict_found: true,
        conflicting_case_ids: vec!["2024-CV-01122".into()],
        waiver_obtained: false,
        check_date_epoch: 1747000000,
        notes: "Prior representation of Beta Innovations in unrelated IP matter".into(),
    };
    let encoded = encode_to_vec(&check).expect("encode conflict check");
    let (decoded, _): (ConflictCheck, usize) =
        decode_from_slice(&encoded).expect("decode conflict check");
    assert_eq!(check, decoded);
}

// ── Test 9: Statute of limitations tracking via file ────────────────────────

#[test]
fn test_statute_of_limitations_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t9_{}.bin", std::process::id()));
    let statutes = vec![
        StatuteOfLimitations {
            jurisdiction: "California".into(),
            cause_of_action: "Breach of Contract (written)".into(),
            limitation_years: 4,
            accrual_date_epoch: 1680000000,
            deadline_epoch: 1806000000,
            tolling_days: 0,
            tolling_reason: None,
            case_id: "2025-CV-04421".into(),
            alert_sent: true,
        },
        StatuteOfLimitations {
            jurisdiction: "New York".into(),
            cause_of_action: "Fraud".into(),
            limitation_years: 6,
            accrual_date_epoch: 1660000000,
            deadline_epoch: 1849000000,
            tolling_days: 90,
            tolling_reason: Some("Defendant absent from jurisdiction".into()),
            case_id: "2024-CV-08811".into(),
            alert_sent: false,
        },
    ];
    encode_to_file(&statutes, &path).expect("encode statutes of limitations");
    let decoded: Vec<StatuteOfLimitations> =
        decode_from_file(&path).expect("decode statutes of limitations");
    assert_eq!(statutes, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 10: Jury selection profiles via slice ──────────────────────────────

#[test]
fn test_jury_selection_profiles_slice() {
    let jurors = vec![
        JurorProfile {
            juror_id: 101,
            panel_number: 3,
            occupation: "Software Engineer".into(),
            age_range: "30-39".into(),
            prior_jury_service: false,
            hardship_claim: false,
            cause_challenge: false,
            peremptory_strike: false,
            seated: true,
            questionnaire_score: 72,
            notes: "Expressed familiarity with technology industry".into(),
        },
        JurorProfile {
            juror_id: 102,
            panel_number: 3,
            occupation: "Retired Teacher".into(),
            age_range: "60-69".into(),
            prior_jury_service: true,
            hardship_claim: false,
            cause_challenge: false,
            peremptory_strike: true,
            seated: false,
            questionnaire_score: 55,
            notes: "Strong opinions about corporate responsibility".into(),
        },
        JurorProfile {
            juror_id: 103,
            panel_number: 3,
            occupation: "Nurse Practitioner".into(),
            age_range: "40-49".into(),
            prior_jury_service: false,
            hardship_claim: true,
            cause_challenge: false,
            peremptory_strike: false,
            seated: false,
            questionnaire_score: 68,
            notes: "Childcare hardship; excused by court".into(),
        },
    ];
    let encoded = encode_to_vec(&jurors).expect("encode juror profiles");
    let (decoded, bytes_read): (Vec<JurorProfile>, usize) =
        decode_from_slice(&encoded).expect("decode juror profiles");
    assert_eq!(jurors, decoded);
    assert!(bytes_read > 0);
}

// ── Test 11: Settlement negotiation records via file ────────────────────────

#[test]
fn test_settlement_negotiations_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t11_{}.bin", std::process::id()));
    let records = vec![
        SettlementRecord {
            case_id: "2025-CV-04421".into(),
            offer_number: 1,
            offered_by_plaintiff: true,
            amount_cents: 5_000_000_00,
            offer_date_epoch: 1748500000,
            expiration_epoch: 1749500000,
            accepted: false,
            conditions: vec![
                "Mutual non-disparagement".into(),
                "No admission of fault".into(),
            ],
            mediator_name: None,
            confidential: true,
        },
        SettlementRecord {
            case_id: "2025-CV-04421".into(),
            offer_number: 2,
            offered_by_plaintiff: false,
            amount_cents: 1_500_000_00,
            offer_date_epoch: 1749000000,
            expiration_epoch: 1750000000,
            accepted: false,
            conditions: vec![
                "License agreement for disputed patent".into(),
                "Dismissal with prejudice".into(),
            ],
            mediator_name: None,
            confidential: true,
        },
        SettlementRecord {
            case_id: "2025-CV-04421".into(),
            offer_number: 3,
            offered_by_plaintiff: true,
            amount_cents: 3_200_000_00,
            offer_date_epoch: 1750000000,
            expiration_epoch: 1751000000,
            accepted: true,
            conditions: vec![
                "Royalty-free license grant".into(),
                "Joint press release".into(),
                "Mutual release of all claims".into(),
            ],
            mediator_name: Some("Hon. James Wu (Ret.)".into()),
            confidential: true,
        },
    ];
    encode_to_file(&records, &path).expect("encode settlement records");
    let decoded: Vec<SettlementRecord> =
        decode_from_file(&path).expect("decode settlement records");
    assert_eq!(records, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 12: Patent claim analysis via slice ────────────────────────────────

#[test]
fn test_patent_claim_analysis_slice() {
    let claims = vec![
        PatentClaim {
            patent_number: "US11,234,567".into(),
            claim_number: 1,
            independent: true,
            claim_text: "A method for encoding structured data into a binary format comprising \
                the steps of traversing a type tree, emitting variable-length integers..."
                .into(),
            dependent_on: vec![],
            keywords: vec![
                "binary encoding".into(),
                "variable-length integer".into(),
                "type tree".into(),
            ],
            prior_art_refs: vec!["US10,111,222".into(), "EP3456789A1".into()],
            validity_score: 65,
            infringement_score: 82,
        },
        PatentClaim {
            patent_number: "US11,234,567".into(),
            claim_number: 2,
            independent: false,
            claim_text: "The method of claim 1, wherein the variable-length integers are \
                encoded using a continuation bit scheme..."
                .into(),
            dependent_on: vec![1],
            keywords: vec!["continuation bit".into(), "varint".into()],
            prior_art_refs: vec!["US10,111,222".into()],
            validity_score: 40,
            infringement_score: 90,
        },
        PatentClaim {
            patent_number: "US11,234,567".into(),
            claim_number: 3,
            independent: false,
            claim_text: "The method of claim 1, further comprising a checksum verification \
                step applied to the encoded output..."
                .into(),
            dependent_on: vec![1],
            keywords: vec!["checksum".into(), "verification".into(), "integrity".into()],
            prior_art_refs: vec![],
            validity_score: 75,
            infringement_score: 55,
        },
    ];
    let encoded = encode_to_vec(&claims).expect("encode patent claims");
    let (decoded, _): (Vec<PatentClaim>, usize) =
        decode_from_slice(&encoded).expect("decode patent claims");
    assert_eq!(claims, decoded);
}

// ── Test 13: Compliance checklist via file ──────────────────────────────────

#[test]
fn test_compliance_checklist_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t13_{}.bin", std::process::id()));
    let checklist = ComplianceChecklist {
        checklist_id: 8001,
        entity_name: "Acme Technologies Inc.".into(),
        framework: "SOC 2 Type II".into(),
        items: vec![
            ComplianceItem {
                item_id: 1,
                regulation_code: "CC6.1".into(),
                description: "Logical and physical access controls".into(),
                compliant: true,
                evidence_doc_ids: vec!["EVD-001".into(), "EVD-002".into()],
                reviewer: "Compliance Officer A".into(),
                review_date_epoch: 1747000000,
                remediation_deadline_epoch: None,
                severity: 3,
            },
            ComplianceItem {
                item_id: 2,
                regulation_code: "CC7.2".into(),
                description: "Monitoring of system components for anomalies".into(),
                compliant: false,
                evidence_doc_ids: vec!["EVD-003".into()],
                reviewer: "Compliance Officer A".into(),
                review_date_epoch: 1747000000,
                remediation_deadline_epoch: Some(1753000000),
                severity: 7,
            },
            ComplianceItem {
                item_id: 3,
                regulation_code: "CC8.1".into(),
                description: "Change management processes".into(),
                compliant: true,
                evidence_doc_ids: vec!["EVD-004".into(), "EVD-005".into(), "EVD-006".into()],
                reviewer: "Compliance Officer B".into(),
                review_date_epoch: 1747100000,
                remediation_deadline_epoch: None,
                severity: 5,
            },
        ],
        overall_score_pct: 85,
        last_audit_epoch: 1747100000,
    };
    encode_to_file(&checklist, &path).expect("encode compliance checklist");
    let decoded: ComplianceChecklist =
        decode_from_file(&path).expect("decode compliance checklist");
    assert_eq!(checklist, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 14: Legal hold notices via slice ────────────────────────────────────

#[test]
fn test_legal_hold_notices_slice() {
    let hold = LegalHoldNotice {
        hold_id: 6001,
        case_id: "2025-CV-04421".into(),
        hold_name: "Patent Infringement Hold - Project Phoenix".into(),
        issued_epoch: 1746000000,
        custodians: vec![
            "jsmith@corp.com".into(),
            "alee@corp.com".into(),
            "bchen@corp.com".into(),
            "dwong@corp.com".into(),
        ],
        data_sources: vec![
            "Email (Exchange)".into(),
            "Slack channels".into(),
            "SharePoint sites".into(),
            "Engineering Git repos".into(),
            "JIRA tickets".into(),
        ],
        active: true,
        reminder_interval_days: 90,
        last_reminder_epoch: Some(1754000000),
        acknowledged_by: vec!["jsmith@corp.com".into(), "alee@corp.com".into()],
        scope_description: "All documents, communications, and data related to Project Phoenix \
            development from January 2024 to present, including design specifications, \
            test results, and external communications with vendors."
            .into(),
    };
    let encoded = encode_to_vec(&hold).expect("encode legal hold notice");
    let (decoded, bytes_read): (LegalHoldNotice, usize) =
        decode_from_slice(&encoded).expect("decode legal hold notice");
    assert_eq!(hold, decoded);
    assert_eq!(bytes_read, encoded.len());
}

// ── Test 15: Precedent citation network via file ────────────────────────────

#[test]
fn test_precedent_citation_network_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t15_{}.bin", std::process::id()));
    let network = PrecedentNetwork {
        root_case: "Alice Corp. v. CLS Bank International, 573 U.S. 208 (2014)".into(),
        citations: vec![
            PrecedentCitation {
                citing_case: "Alice Corp. v. CLS Bank".into(),
                cited_case: "Mayo Collaborative Servs. v. Prometheus Labs., 566 U.S. 66".into(),
                citation_string: "566 U.S. 66".into(),
                treatment: "Followed".into(),
                relevance_score: 95,
                headnote_topics: vec![
                    "Patent eligibility".into(),
                    "Abstract ideas".into(),
                    "Section 101".into(),
                ],
            },
            PrecedentCitation {
                citing_case: "Alice Corp. v. CLS Bank".into(),
                cited_case: "Bilski v. Kappos, 561 U.S. 593".into(),
                citation_string: "561 U.S. 593".into(),
                treatment: "Distinguished".into(),
                relevance_score: 80,
                headnote_topics: vec![
                    "Business method patents".into(),
                    "Machine-or-transformation test".into(),
                ],
            },
            PrecedentCitation {
                citing_case: "Alice Corp. v. CLS Bank".into(),
                cited_case: "Diamond v. Diehr, 450 U.S. 175".into(),
                citation_string: "450 U.S. 175".into(),
                treatment: "Cited".into(),
                relevance_score: 70,
                headnote_topics: vec!["Software patents".into(), "Process claims".into()],
            },
        ],
        depth: 2,
        total_cases: 4,
        jurisdiction_filter: Some("U.S. Supreme Court".into()),
    };
    encode_to_file(&network, &path).expect("encode precedent network");
    let decoded: PrecedentNetwork = decode_from_file(&path).expect("decode precedent network");
    assert_eq!(network, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 16: Mixed case statuses via slice ──────────────────────────────────

#[test]
fn test_all_case_statuses_roundtrip() {
    let statuses = vec![
        CaseStatus::Filed,
        CaseStatus::Discovery,
        CaseStatus::PreTrial,
        CaseStatus::Trial,
        CaseStatus::PostTrial,
        CaseStatus::Settled,
        CaseStatus::Dismissed,
        CaseStatus::Appealed,
        CaseStatus::Closed,
    ];
    let encoded = encode_to_vec(&statuses).expect("encode all case statuses");
    let (decoded, _): (Vec<CaseStatus>, usize) =
        decode_from_slice(&encoded).expect("decode all case statuses");
    assert_eq!(statuses, decoded);
}

// ── Test 17: Large docket via file ──────────────────────────────────────────

#[test]
fn test_large_docket_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t17_{}.bin", std::process::id()));
    let entries: Vec<DocketEntry> = (1..=100)
        .map(|i| DocketEntry {
            entry_number: i,
            case_number: "2025-CV-09999".into(),
            filing_date_epoch: 1748000000 + (i as i64 * 86400),
            description: format!("Docket entry number {} - procedural filing", i),
            filed_by: if i % 2 == 0 {
                "Plaintiff".into()
            } else {
                "Defendant".into()
            },
            status: match i % 5 {
                0 => CaseStatus::Discovery,
                1 => CaseStatus::Filed,
                2 => CaseStatus::PreTrial,
                3 => CaseStatus::Trial,
                _ => CaseStatus::PostTrial,
            },
            sealed: i % 7 == 0,
            page_count: (i * 3) + 1,
        })
        .collect();
    encode_to_file(&entries, &path).expect("encode large docket");
    let decoded: Vec<DocketEntry> = decode_from_file(&path).expect("decode large docket");
    assert_eq!(entries.len(), decoded.len());
    assert_eq!(entries, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 18: Multiple billing categories via slice ──────────────────────────

#[test]
fn test_all_billing_categories_slice() {
    let categories = vec![
        TimeEntryCategory::Billable,
        TimeEntryCategory::NonBillable,
        TimeEntryCategory::ProBono,
        TimeEntryCategory::AdminOverhead,
        TimeEntryCategory::CourtAppearance,
        TimeEntryCategory::ClientMeeting,
        TimeEntryCategory::Research,
        TimeEntryCategory::Drafting,
        TimeEntryCategory::Review,
        TimeEntryCategory::Travel,
    ];
    let encoded = encode_to_vec(&categories).expect("encode billing categories");
    let (decoded, bytes_read): (Vec<TimeEntryCategory>, usize) =
        decode_from_slice(&encoded).expect("decode billing categories");
    assert_eq!(categories, decoded);
    assert_eq!(bytes_read, encoded.len());
}

// ── Test 19: Complex conflict check with waiver via file ────────────────────

#[test]
fn test_conflict_check_with_waiver_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t19_{}.bin", std::process::id()));
    let checks = vec![
        ConflictCheck {
            check_id: 3002,
            requesting_attorney: "ATT-017".into(),
            prospective_client: "Global Widgets Corp.".into(),
            adverse_parties: vec!["Regional Supply Co.".into()],
            related_entities: vec![
                "Global Widgets Holdings".into(),
                "Global Widgets Europe GmbH".into(),
                "Global Widgets Asia Pte Ltd".into(),
            ],
            conflict_found: false,
            conflicting_case_ids: vec![],
            waiver_obtained: false,
            check_date_epoch: 1748000000,
            notes: "No conflicts identified across all firm databases".into(),
        },
        ConflictCheck {
            check_id: 3003,
            requesting_attorney: "ATT-099".into(),
            prospective_client: "NextGen Pharmaceuticals".into(),
            adverse_parties: vec!["PharmaCore Inc.".into(), "BioResearch Labs".into()],
            related_entities: vec!["NextGen Holdings".into()],
            conflict_found: true,
            conflicting_case_ids: vec!["2023-CV-05533".into(), "2024-CV-01177".into()],
            waiver_obtained: true,
            check_date_epoch: 1748100000,
            notes: "Conflicts identified; informed consent waivers obtained from all \
                affected clients per RPC 1.7(b)"
                .into(),
        },
    ];
    encode_to_file(&checks, &path).expect("encode conflict checks");
    let decoded: Vec<ConflictCheck> = decode_from_file(&path).expect("decode conflict checks");
    assert_eq!(checks, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 20: Review classifications roundtrip via slice ─────────────────────

#[test]
fn test_review_classifications_exhaustive() {
    let classifications = vec![
        ReviewClassification::Privileged,
        ReviewClassification::Relevant,
        ReviewClassification::Responsive,
        ReviewClassification::NonResponsive,
        ReviewClassification::Redacted,
        ReviewClassification::NeedsSecondReview,
    ];
    let encoded = encode_to_vec(&classifications).expect("encode review classifications");
    let (decoded, _): (Vec<ReviewClassification>, usize) =
        decode_from_slice(&encoded).expect("decode review classifications");
    assert_eq!(classifications, decoded);
}

// ── Test 21: Nested compliance items with remediation via file ──────────────

#[test]
fn test_compliance_with_remediation_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t21_{}.bin", std::process::id()));
    let checklist = ComplianceChecklist {
        checklist_id: 8002,
        entity_name: "SecureBank Financial Services".into(),
        framework: "GDPR Article 30 Records of Processing".into(),
        items: vec![
            ComplianceItem {
                item_id: 10,
                regulation_code: "Art.30(1)(a)".into(),
                description: "Name and contact details of the controller".into(),
                compliant: true,
                evidence_doc_ids: vec!["GDPR-EVD-101".into()],
                reviewer: "DPO".into(),
                review_date_epoch: 1747500000,
                remediation_deadline_epoch: None,
                severity: 2,
            },
            ComplianceItem {
                item_id: 11,
                regulation_code: "Art.30(1)(b)".into(),
                description: "Purposes of processing".into(),
                compliant: true,
                evidence_doc_ids: vec!["GDPR-EVD-102".into(), "GDPR-EVD-103".into()],
                reviewer: "DPO".into(),
                review_date_epoch: 1747500000,
                remediation_deadline_epoch: None,
                severity: 4,
            },
            ComplianceItem {
                item_id: 12,
                regulation_code: "Art.30(1)(d)".into(),
                description: "Categories of recipients including third countries".into(),
                compliant: false,
                evidence_doc_ids: vec![],
                reviewer: "DPO".into(),
                review_date_epoch: 1747500000,
                remediation_deadline_epoch: Some(1750000000),
                severity: 8,
            },
            ComplianceItem {
                item_id: 13,
                regulation_code: "Art.30(1)(f)".into(),
                description: "Time limits for erasure of different categories of data".into(),
                compliant: false,
                evidence_doc_ids: vec!["GDPR-EVD-110".into()],
                reviewer: "DPO".into(),
                review_date_epoch: 1747500000,
                remediation_deadline_epoch: Some(1752000000),
                severity: 6,
            },
        ],
        overall_score_pct: 50,
        last_audit_epoch: 1747500000,
    };
    encode_to_file(&checklist, &path).expect("encode GDPR compliance checklist");
    let decoded: ComplianceChecklist =
        decode_from_file(&path).expect("decode GDPR compliance checklist");
    assert_eq!(checklist, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 22: Full case lifecycle via file ───────────────────────────────────

#[test]
fn test_full_case_lifecycle_file() {
    let path = temp_dir().join(format!("oxicode_fio43_t22_{}.bin", std::process::id()));

    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct CaseLifecycle {
        case_id: String,
        docket_entries: Vec<DocketEntry>,
        filings: Vec<CourtFiling>,
        settlement_offers: Vec<SettlementRecord>,
        hold_notice: LegalHoldNotice,
        billing_entries: Vec<BillingTimeEntry>,
        statute: StatuteOfLimitations,
        final_status: CaseStatus,
    }

    let lifecycle = CaseLifecycle {
        case_id: "2025-CV-04421".into(),
        docket_entries: vec![
            DocketEntry {
                entry_number: 1,
                case_number: "2025-CV-04421".into(),
                filing_date_epoch: 1748000000,
                description: "Initial Complaint".into(),
                filed_by: "Plaintiff".into(),
                status: CaseStatus::Filed,
                sealed: false,
                page_count: 30,
            },
            DocketEntry {
                entry_number: 2,
                case_number: "2025-CV-04421".into(),
                filing_date_epoch: 1749000000,
                description: "Answer and Counterclaims".into(),
                filed_by: "Defendant".into(),
                status: CaseStatus::Discovery,
                sealed: false,
                page_count: 45,
            },
        ],
        filings: vec![CourtFiling {
            filing_id: 20001,
            case_number: "2025-CV-04421".into(),
            filing_type: FilingType::Stipulation,
            title: "Stipulation of Settlement".into(),
            filed_epoch: 1755000000,
            court_name: "N.D. Cal.".into(),
            judge_name: "Hon. P. Martinez".into(),
            electronic: true,
            confidential: true,
            attachment_count: 3,
        }],
        settlement_offers: vec![SettlementRecord {
            case_id: "2025-CV-04421".into(),
            offer_number: 1,
            offered_by_plaintiff: false,
            amount_cents: 2_800_000_00,
            offer_date_epoch: 1754000000,
            expiration_epoch: 1755000000,
            accepted: true,
            conditions: vec!["Mutual release".into(), "Confidentiality".into()],
            mediator_name: Some("Hon. J. Wu (Ret.)".into()),
            confidential: true,
        }],
        hold_notice: LegalHoldNotice {
            hold_id: 6010,
            case_id: "2025-CV-04421".into(),
            hold_name: "Post-settlement preservation".into(),
            issued_epoch: 1746000000,
            custodians: vec!["legal@corp.com".into()],
            data_sources: vec!["All litigation files".into()],
            active: false,
            reminder_interval_days: 0,
            last_reminder_epoch: None,
            acknowledged_by: vec!["legal@corp.com".into()],
            scope_description: "Retain all case-related documents per retention policy".into(),
        },
        billing_entries: vec![BillingTimeEntry {
            entry_id: 99001,
            attorney_id: "ATT-042".into(),
            case_id: "2025-CV-04421".into(),
            category: TimeEntryCategory::Billable,
            description: "Final case closure and file transfer".into(),
            duration_minutes: 60,
            rate_cents_per_hour: 65000,
            date_epoch: 1756000000,
            approved: true,
            invoice_id: Some(6000),
        }],
        statute: StatuteOfLimitations {
            jurisdiction: "California".into(),
            cause_of_action: "Patent Infringement".into(),
            limitation_years: 6,
            accrual_date_epoch: 1700000000,
            deadline_epoch: 1889000000,
            tolling_days: 0,
            tolling_reason: None,
            case_id: "2025-CV-04421".into(),
            alert_sent: true,
        },
        final_status: CaseStatus::Settled,
    };

    encode_to_file(&lifecycle, &path).expect("encode case lifecycle");
    let decoded: CaseLifecycle = decode_from_file(&path).expect("decode case lifecycle");
    assert_eq!(lifecycle, decoded);
    assert_eq!(decoded.docket_entries.len(), 2);
    assert_eq!(decoded.filings.len(), 1);
    assert_eq!(decoded.settlement_offers.len(), 1);
    assert_eq!(decoded.final_status, CaseStatus::Settled);
    std::fs::remove_file(&path).ok();
}
