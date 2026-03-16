//! Legal document / contract management versioning tests for OxiCode (set 19).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and legal domain structs (ContractV1, ContractV2, ContractParty,
//! ContractClause) with the ContractType and ClauseType enums across all variants,
//! various version tags, field verification, version comparison, consumed bytes
//! accounting, Vec of contracts, amendment tracking, multi-party contracts,
//! fully executed contracts, NDA/employment specifics, and governing law variations.

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
use oxicode::versioning::Version;
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum ContractType {
    Employment,
    Nda,
    ServiceAgreement,
    LicenseAgreement,
    Partnership,
    SalesContract,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ClauseType {
    Liability,
    Indemnification,
    Confidentiality,
    Termination,
    Jurisdiction,
    PaymentTerms,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ContractParty {
    party_id: u32,
    name: String,
    role: String,
    signed: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ContractClause {
    clause_id: u16,
    clause_type: ClauseType,
    content: String,
    mandatory: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ContractV1 {
    contract_id: u64,
    contract_type: ContractType,
    parties: Vec<ContractParty>,
    effective_date: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ContractV2 {
    contract_id: u64,
    contract_type: ContractType,
    parties: Vec<ContractParty>,
    effective_date: u64,
    clauses: Vec<ContractClause>,
    governing_law: String,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// Test 1 — ContractV1 round-trips under version 1.0.0
#[test]
fn test_contract_v1_version_1_0_0() {
    let contract = ContractV1 {
        contract_id: 1001,
        contract_type: ContractType::Employment,
        parties: vec![ContractParty {
            party_id: 1,
            name: "Acme Corp".to_string(),
            role: "Employer".to_string(),
            signed: true,
        }],
        effective_date: 1_700_000_000,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode ContractV1 v1.0.0");
    let (decoded, ver, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode ContractV1 v1.0.0");
    assert_eq!(contract, decoded);
    assert_eq!(ver, Version::new(1u16, 0u16, 0u16));
}

/// Test 2 — ContractV2 round-trips under version 2.0.0
#[test]
fn test_contract_v2_version_2_0_0() {
    let contract = ContractV2 {
        contract_id: 2001,
        contract_type: ContractType::ServiceAgreement,
        parties: vec![ContractParty {
            party_id: 10,
            name: "Beta LLC".to_string(),
            role: "Client".to_string(),
            signed: false,
        }],
        effective_date: 1_710_000_000,
        clauses: vec![ContractClause {
            clause_id: 1,
            clause_type: ClauseType::Liability,
            content: "Liability limited to contract value.".to_string(),
            mandatory: true,
        }],
        governing_law: "State of Delaware".to_string(),
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode ContractV2 v2.0.0");
    let (decoded, ver, _): (ContractV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode ContractV2 v2.0.0");
    assert_eq!(contract, decoded);
    assert_eq!(ver, Version::new(2u16, 0u16, 0u16));
}

/// Test 3 — ContractType::Employment versioned
#[test]
fn test_contract_type_employment_versioned() {
    let contract = ContractV1 {
        contract_id: 3001,
        contract_type: ContractType::Employment,
        parties: vec![],
        effective_date: 1_720_000_000,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode Employment");
    let (decoded, _, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode Employment");
    assert_eq!(decoded.contract_type, ContractType::Employment);
}

/// Test 4 — ContractType::Nda versioned
#[test]
fn test_contract_type_nda_versioned() {
    let contract = ContractV1 {
        contract_id: 4001,
        contract_type: ContractType::Nda,
        parties: vec![],
        effective_date: 1_720_000_001,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode Nda");
    let (decoded, _, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode Nda");
    assert_eq!(decoded.contract_type, ContractType::Nda);
}

/// Test 5 — ContractType::LicenseAgreement versioned
#[test]
fn test_contract_type_license_agreement_versioned() {
    let contract = ContractV1 {
        contract_id: 5001,
        contract_type: ContractType::LicenseAgreement,
        parties: vec![],
        effective_date: 1_720_000_002,
    };
    let v = Version::new(1u16, 1u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode LicenseAgreement");
    let (decoded, ver, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode LicenseAgreement");
    assert_eq!(decoded.contract_type, ContractType::LicenseAgreement);
    assert_eq!(ver, Version::new(1u16, 1u16, 0u16));
}

/// Test 6 — ContractType::Partnership versioned
#[test]
fn test_contract_type_partnership_versioned() {
    let contract = ContractV1 {
        contract_id: 6001,
        contract_type: ContractType::Partnership,
        parties: vec![],
        effective_date: 1_720_000_003,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode Partnership");
    let (decoded, _, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode Partnership");
    assert_eq!(decoded.contract_type, ContractType::Partnership);
}

/// Test 7 — ContractType::SalesContract versioned
#[test]
fn test_contract_type_sales_contract_versioned() {
    let contract = ContractV1 {
        contract_id: 7001,
        contract_type: ContractType::SalesContract,
        parties: vec![],
        effective_date: 1_720_000_004,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode SalesContract");
    let (decoded, _, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode SalesContract");
    assert_eq!(decoded.contract_type, ContractType::SalesContract);
}

/// Test 8 — Each ClauseType variant versioned (Liability and Indemnification)
#[test]
fn test_clause_type_liability_and_indemnification_versioned() {
    let liability_clause = ContractClause {
        clause_id: 100,
        clause_type: ClauseType::Liability,
        content: "No consequential damages.".to_string(),
        mandatory: true,
    };
    let indem_clause = ContractClause {
        clause_id: 101,
        clause_type: ClauseType::Indemnification,
        content: "Each party shall indemnify the other.".to_string(),
        mandatory: false,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes_l = encode_versioned_value(&liability_clause, v).expect("encode Liability clause");
    let (dl, _, _): (ContractClause, Version, usize) =
        decode_versioned_value(&bytes_l).expect("decode Liability clause");
    assert_eq!(dl.clause_type, ClauseType::Liability);

    let bytes_i = encode_versioned_value(&indem_clause, v).expect("encode Indemnification clause");
    let (di, _, _): (ContractClause, Version, usize) =
        decode_versioned_value(&bytes_i).expect("decode Indemnification clause");
    assert_eq!(di.clause_type, ClauseType::Indemnification);
}

/// Test 9 — ClauseType::Confidentiality and Termination versioned
#[test]
fn test_clause_type_confidentiality_and_termination_versioned() {
    let conf_clause = ContractClause {
        clause_id: 200,
        clause_type: ClauseType::Confidentiality,
        content: "All disclosed information is confidential for 5 years.".to_string(),
        mandatory: true,
    };
    let term_clause = ContractClause {
        clause_id: 201,
        clause_type: ClauseType::Termination,
        content: "Either party may terminate with 30-day written notice.".to_string(),
        mandatory: false,
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes_c = encode_versioned_value(&conf_clause, v).expect("encode Confidentiality clause");
    let (dc, _, _): (ContractClause, Version, usize) =
        decode_versioned_value(&bytes_c).expect("decode Confidentiality clause");
    assert_eq!(dc.clause_type, ClauseType::Confidentiality);

    let bytes_t = encode_versioned_value(&term_clause, v).expect("encode Termination clause");
    let (dt, _, _): (ContractClause, Version, usize) =
        decode_versioned_value(&bytes_t).expect("decode Termination clause");
    assert_eq!(dt.clause_type, ClauseType::Termination);
}

/// Test 10 — ClauseType::Jurisdiction and PaymentTerms versioned
#[test]
fn test_clause_type_jurisdiction_and_payment_terms_versioned() {
    let juris_clause = ContractClause {
        clause_id: 300,
        clause_type: ClauseType::Jurisdiction,
        content: "Courts of New York shall have exclusive jurisdiction.".to_string(),
        mandatory: true,
    };
    let pay_clause = ContractClause {
        clause_id: 301,
        clause_type: ClauseType::PaymentTerms,
        content: "Net 30 days from invoice date.".to_string(),
        mandatory: true,
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes_j = encode_versioned_value(&juris_clause, v).expect("encode Jurisdiction clause");
    let (dj, _, _): (ContractClause, Version, usize) =
        decode_versioned_value(&bytes_j).expect("decode Jurisdiction clause");
    assert_eq!(dj.clause_type, ClauseType::Jurisdiction);

    let bytes_p = encode_versioned_value(&pay_clause, v).expect("encode PaymentTerms clause");
    let (dp, _, _): (ContractClause, Version, usize) =
        decode_versioned_value(&bytes_p).expect("decode PaymentTerms clause");
    assert_eq!(dp.clause_type, ClauseType::PaymentTerms);
}

/// Test 11 — ContractParty versioned round-trip
#[test]
fn test_contract_party_versioned_round_trip() {
    let party = ContractParty {
        party_id: 42,
        name: "Gamma Industries Inc.".to_string(),
        role: "Vendor".to_string(),
        signed: true,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&party, v).expect("encode ContractParty");
    let (decoded, ver, _): (ContractParty, Version, usize) =
        decode_versioned_value(&bytes).expect("decode ContractParty");
    assert_eq!(party, decoded);
    assert_eq!(ver, Version::new(1u16, 0u16, 0u16));
}

/// Test 12 — ContractClause versioned round-trip
#[test]
fn test_contract_clause_versioned_round_trip() {
    let clause = ContractClause {
        clause_id: 999,
        clause_type: ClauseType::PaymentTerms,
        content: "Payment due within 15 business days of milestone completion.".to_string(),
        mandatory: true,
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&clause, v).expect("encode ContractClause");
    let (decoded, ver, _): (ContractClause, Version, usize) =
        decode_versioned_value(&bytes).expect("decode ContractClause");
    assert_eq!(clause, decoded);
    assert_eq!(ver, Version::new(2u16, 0u16, 0u16));
}

/// Test 13 — Version triple (major, minor, patch) preserved exactly
#[test]
fn test_version_triple_preserved() {
    let contract = ContractV1 {
        contract_id: 13001,
        contract_type: ContractType::ServiceAgreement,
        parties: vec![],
        effective_date: 1_730_000_000,
    };
    let v = Version::new(3u16, 7u16, 12u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode with specific triple");
    let (_, ver, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode version triple");
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 12);
}

/// Test 14 — Version comparison: v1.0.0 < v2.0.0
#[test]
fn test_version_comparison_v1_lt_v2() {
    let v1 = Version::new(1u16, 0u16, 0u16);
    let v2 = Version::new(2u16, 0u16, 0u16);
    assert!(v1 < v2, "v1.0.0 must be less than v2.0.0");
    assert!(v2 > v1, "v2.0.0 must be greater than v1.0.0");
}

/// Test 15 — Vec<ContractV1> versioned round-trip
#[test]
fn test_vec_of_contracts_versioned() {
    let contracts = vec![
        ContractV1 {
            contract_id: 15001,
            contract_type: ContractType::Nda,
            parties: vec![ContractParty {
                party_id: 5,
                name: "Delta Corp".to_string(),
                role: "Disclosing Party".to_string(),
                signed: true,
            }],
            effective_date: 1_715_000_000,
        },
        ContractV1 {
            contract_id: 15002,
            contract_type: ContractType::SalesContract,
            parties: vec![ContractParty {
                party_id: 6,
                name: "Epsilon Ltd".to_string(),
                role: "Buyer".to_string(),
                signed: false,
            }],
            effective_date: 1_716_000_000,
        },
    ];
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contracts, v).expect("encode Vec<ContractV1>");
    let (decoded, ver, _): (Vec<ContractV1>, Version, usize) =
        decode_versioned_value(&bytes).expect("decode Vec<ContractV1>");
    assert_eq!(contracts, decoded);
    assert_eq!(ver, Version::new(1u16, 0u16, 0u16));
}

/// Test 16 — Amendment tracking: v1.0.0 → v1.1.0 → v1.2.0
#[test]
fn test_amendment_tracking_minor_versions() {
    let original = ContractV1 {
        contract_id: 16001,
        contract_type: ContractType::Partnership,
        parties: vec![ContractParty {
            party_id: 7,
            name: "Zeta Partners".to_string(),
            role: "Partner A".to_string(),
            signed: true,
        }],
        effective_date: 1_700_000_000,
    };
    let v100 = Version::new(1u16, 0u16, 0u16);
    let v110 = Version::new(1u16, 1u16, 0u16);
    let v120 = Version::new(1u16, 2u16, 0u16);

    let bytes_v100 = encode_versioned_value(&original, v100).expect("encode amendment v1.0.0");
    let (d100, ver100, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes_v100).expect("decode amendment v1.0.0");
    assert_eq!(ver100, v100);
    assert_eq!(d100.contract_id, 16001);

    let bytes_v110 = encode_versioned_value(&original, v110).expect("encode amendment v1.1.0");
    let (_, ver110, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes_v110).expect("decode amendment v1.1.0");
    assert_eq!(ver110, v110);
    assert!(ver110 > ver100, "v1.1.0 must be after v1.0.0");

    let bytes_v120 = encode_versioned_value(&original, v120).expect("encode amendment v1.2.0");
    let (_, ver120, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes_v120).expect("decode amendment v1.2.0");
    assert_eq!(ver120, v120);
    assert!(ver120 > ver110, "v1.2.0 must be after v1.1.0");
}

/// Test 17 — Fully executed contract (all parties signed)
#[test]
fn test_fully_executed_contract_all_signed() {
    let contract = ContractV2 {
        contract_id: 17001,
        contract_type: ContractType::ServiceAgreement,
        parties: vec![
            ContractParty {
                party_id: 20,
                name: "Alpha Services".to_string(),
                role: "Provider".to_string(),
                signed: true,
            },
            ContractParty {
                party_id: 21,
                name: "Beta Client Corp".to_string(),
                role: "Client".to_string(),
                signed: true,
            },
        ],
        effective_date: 1_720_000_000,
        clauses: vec![ContractClause {
            clause_id: 1,
            clause_type: ClauseType::PaymentTerms,
            content: "Monthly invoicing, Net 30.".to_string(),
            mandatory: true,
        }],
        governing_law: "California".to_string(),
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode fully executed");
    let (decoded, _, _): (ContractV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode fully executed");
    assert!(
        decoded.parties.iter().all(|p| p.signed),
        "all parties must be signed"
    );
}

/// Test 18 — NDA contract with confidentiality clause
#[test]
fn test_nda_with_confidentiality_clause() {
    let nda = ContractV2 {
        contract_id: 18001,
        contract_type: ContractType::Nda,
        parties: vec![
            ContractParty {
                party_id: 30,
                name: "Innovate Inc.".to_string(),
                role: "Disclosing Party".to_string(),
                signed: true,
            },
            ContractParty {
                party_id: 31,
                name: "Consult LLC".to_string(),
                role: "Receiving Party".to_string(),
                signed: true,
            },
        ],
        effective_date: 1_718_000_000,
        clauses: vec![ContractClause {
            clause_id: 50,
            clause_type: ClauseType::Confidentiality,
            content: "All proprietary information shall remain confidential for 3 years."
                .to_string(),
            mandatory: true,
        }],
        governing_law: "New York".to_string(),
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&nda, v).expect("encode NDA");
    let (decoded, ver, _): (ContractV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode NDA");
    assert_eq!(decoded.contract_type, ContractType::Nda);
    assert_eq!(decoded.clauses[0].clause_type, ClauseType::Confidentiality);
    assert_eq!(ver, Version::new(2u16, 0u16, 0u16));
}

/// Test 19 — Employment contract with payment terms clause
#[test]
fn test_employment_with_payment_terms() {
    let employment = ContractV2 {
        contract_id: 19001,
        contract_type: ContractType::Employment,
        parties: vec![
            ContractParty {
                party_id: 40,
                name: "TechCorp".to_string(),
                role: "Employer".to_string(),
                signed: true,
            },
            ContractParty {
                party_id: 41,
                name: "Jane Smith".to_string(),
                role: "Employee".to_string(),
                signed: true,
            },
        ],
        effective_date: 1_722_000_000,
        clauses: vec![ContractClause {
            clause_id: 60,
            clause_type: ClauseType::PaymentTerms,
            content: "Bi-weekly salary payment of $5,000 USD.".to_string(),
            mandatory: true,
        }],
        governing_law: "State of Washington".to_string(),
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&employment, v).expect("encode Employment contract");
    let (decoded, _, _): (ContractV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode Employment contract");
    assert_eq!(decoded.contract_type, ContractType::Employment);
    assert_eq!(decoded.clauses[0].clause_type, ClauseType::PaymentTerms);
    assert_eq!(decoded.parties[1].name, "Jane Smith");
}

/// Test 20 — Contract with 5 clauses covering all main types
#[test]
fn test_contract_with_five_clauses() {
    let contract = ContractV2 {
        contract_id: 20001,
        contract_type: ContractType::LicenseAgreement,
        parties: vec![ContractParty {
            party_id: 50,
            name: "Open Source Foundation".to_string(),
            role: "Licensor".to_string(),
            signed: true,
        }],
        effective_date: 1_724_000_000,
        clauses: vec![
            ContractClause {
                clause_id: 1,
                clause_type: ClauseType::Liability,
                content: "Licensor is not liable for indirect damages.".to_string(),
                mandatory: true,
            },
            ContractClause {
                clause_id: 2,
                clause_type: ClauseType::Indemnification,
                content: "Licensee indemnifies licensor against third-party claims.".to_string(),
                mandatory: true,
            },
            ContractClause {
                clause_id: 3,
                clause_type: ClauseType::Confidentiality,
                content: "Source code modifications kept confidential.".to_string(),
                mandatory: false,
            },
            ContractClause {
                clause_id: 4,
                clause_type: ClauseType::Termination,
                content: "License terminates on material breach.".to_string(),
                mandatory: true,
            },
            ContractClause {
                clause_id: 5,
                clause_type: ClauseType::PaymentTerms,
                content: "Annual license fee of $10,000 USD.".to_string(),
                mandatory: true,
            },
        ],
        governing_law: "European Union".to_string(),
    };
    let v = Version::new(2u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode 5-clause contract");
    let (decoded, _, _): (ContractV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode 5-clause contract");
    assert_eq!(decoded.clauses.len(), 5);
    assert_eq!(contract, decoded);
}

/// Test 21 — Multi-party contract with 3 parties
#[test]
fn test_multi_party_contract_three_parties() {
    let contract = ContractV1 {
        contract_id: 21001,
        contract_type: ContractType::Partnership,
        parties: vec![
            ContractParty {
                party_id: 60,
                name: "Alpha Corp".to_string(),
                role: "Partner A".to_string(),
                signed: true,
            },
            ContractParty {
                party_id: 61,
                name: "Beta Corp".to_string(),
                role: "Partner B".to_string(),
                signed: true,
            },
            ContractParty {
                party_id: 62,
                name: "Gamma Corp".to_string(),
                role: "Partner C".to_string(),
                signed: false,
            },
        ],
        effective_date: 1_726_000_000,
    };
    let v = Version::new(1u16, 0u16, 0u16);
    let bytes = encode_versioned_value(&contract, v).expect("encode multi-party contract");
    let (decoded, _, _): (ContractV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode multi-party contract");
    assert_eq!(decoded.parties.len(), 3);
    assert!(!decoded.parties[2].signed, "third party has not yet signed");
}

/// Test 22 — Consumed bytes check: versioned bytes > raw encoded bytes
#[test]
fn test_consumed_bytes_check() {
    let contract = ContractV2 {
        contract_id: 22001,
        contract_type: ContractType::SalesContract,
        parties: vec![ContractParty {
            party_id: 70,
            name: "Omega Sales Inc.".to_string(),
            role: "Seller".to_string(),
            signed: true,
        }],
        effective_date: 1_728_000_000,
        clauses: vec![ContractClause {
            clause_id: 77,
            clause_type: ClauseType::Jurisdiction,
            content: "Exclusive jurisdiction in the courts of Texas.".to_string(),
            mandatory: true,
        }],
        governing_law: "Texas".to_string(),
    };
    let raw_bytes = encode_to_vec(&contract).expect("encode raw ContractV2");
    let v = Version::new(2u16, 0u16, 0u16);
    let versioned_bytes =
        encode_versioned_value(&contract, v).expect("encode versioned ContractV2");
    let (_, _, consumed): (ContractV2, Version, usize) = decode_versioned_value(&versioned_bytes)
        .expect("decode versioned ContractV2 for consumed check");

    // The versioned encoding must carry more bytes than the raw payload due to the version header
    assert!(
        versioned_bytes.len() > raw_bytes.len(),
        "versioned bytes ({}) must exceed raw bytes ({})",
        versioned_bytes.len(),
        raw_bytes.len()
    );
    // consumed bytes must equal the total versioned buffer length
    assert_eq!(
        consumed,
        versioned_bytes.len(),
        "consumed bytes must equal total versioned buffer length"
    );

    // Verify the plain encode/decode still works independently
    let (decoded_raw, _bytes_consumed): (ContractV2, usize) =
        decode_from_slice(&raw_bytes).expect("decode raw ContractV2");
    assert_eq!(contract, decoded_raw);
}
