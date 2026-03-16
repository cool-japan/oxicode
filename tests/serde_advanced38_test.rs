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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PartyRole {
    Vendor,
    Buyer,
    Guarantor,
    Witness,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ClauseType {
    Payment,
    Delivery,
    Warranty,
    Termination,
    Confidentiality,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ContractStatus {
    Draft,
    Active,
    Expired,
    Terminated,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AuditAction {
    Created,
    Signed,
    Amended,
    Terminated,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContractParty {
    party_id: u64,
    name: String,
    role: PartyRole,
    jurisdiction: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContractClause {
    clause_id: u32,
    clause_type: ClauseType,
    text: String,
    is_mandatory: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LegalContract {
    contract_id: u64,
    title: String,
    parties: Vec<ContractParty>,
    clauses: Vec<ContractClause>,
    effective_date: u64,
    expiry_date: Option<u64>,
    status: ContractStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DocumentSignature {
    signer_id: u64,
    timestamp: u64,
    signature_hash: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AuditEntry {
    entry_id: u64,
    contract_id: u64,
    action: AuditAction,
    actor_id: u64,
    timestamp: u64,
}

#[test]
fn test_contract_party_vendor_standard() {
    let party = ContractParty {
        party_id: 1001,
        name: String::from("Acme Corporation"),
        role: PartyRole::Vendor,
        jurisdiction: String::from("Delaware, USA"),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&party, cfg).expect("encode ContractParty vendor");
    let (decoded, _): (ContractParty, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ContractParty vendor");
    assert_eq!(party, decoded);
}

#[test]
fn test_contract_party_buyer_big_endian() {
    let party = ContractParty {
        party_id: 2002,
        name: String::from("Global Imports Ltd"),
        role: PartyRole::Buyer,
        jurisdiction: String::from("Ontario, Canada"),
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&party, cfg).expect("encode ContractParty buyer big endian");
    let (decoded, _): (ContractParty, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ContractParty buyer big endian");
    assert_eq!(party, decoded);
}

#[test]
fn test_contract_party_guarantor_fixed_int() {
    let party = ContractParty {
        party_id: 3003,
        name: String::from("Securitas Bank"),
        role: PartyRole::Guarantor,
        jurisdiction: String::from("Frankfurt, Germany"),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&party, cfg).expect("encode ContractParty guarantor fixed int");
    let (decoded, _): (ContractParty, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ContractParty guarantor fixed int");
    assert_eq!(party, decoded);
}

#[test]
fn test_contract_party_witness_standard() {
    let party = ContractParty {
        party_id: 4004,
        name: String::from("Witness & Associates"),
        role: PartyRole::Witness,
        jurisdiction: String::from("London, UK"),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&party, cfg).expect("encode ContractParty witness");
    let (decoded, _): (ContractParty, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ContractParty witness");
    assert_eq!(party, decoded);
}

#[test]
fn test_contract_clause_payment_standard() {
    let clause = ContractClause {
        clause_id: 101,
        clause_type: ClauseType::Payment,
        text: String::from("Payment shall be due within 30 days of invoice receipt."),
        is_mandatory: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&clause, cfg).expect("encode ContractClause payment");
    let (decoded, _): (ContractClause, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ContractClause payment");
    assert_eq!(clause, decoded);
}

#[test]
fn test_contract_clause_confidentiality_big_endian() {
    let clause = ContractClause {
        clause_id: 202,
        clause_type: ClauseType::Confidentiality,
        text: String::from(
            "All parties agree to maintain strict confidentiality of proprietary information.",
        ),
        is_mandatory: true,
    };
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec(&clause, cfg).expect("encode ContractClause confidentiality big endian");
    let (decoded, _): (ContractClause, usize) = decode_owned_from_slice(&encoded, cfg)
        .expect("decode ContractClause confidentiality big endian");
    assert_eq!(clause, decoded);
}

#[test]
fn test_contract_clause_termination_not_mandatory() {
    let clause = ContractClause {
        clause_id: 303,
        clause_type: ClauseType::Termination,
        text: String::from("Either party may terminate with 60 days written notice."),
        is_mandatory: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&clause, cfg).expect("encode ContractClause termination");
    let (decoded, _): (ContractClause, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ContractClause termination");
    assert_eq!(clause, decoded);
}

#[test]
fn test_legal_contract_active_with_expiry() {
    let contract = LegalContract {
        contract_id: 5001,
        title: String::from("Software Licensing Agreement"),
        parties: vec![
            ContractParty {
                party_id: 1,
                name: String::from("TechVendor Inc"),
                role: PartyRole::Vendor,
                jurisdiction: String::from("California, USA"),
            },
            ContractParty {
                party_id: 2,
                name: String::from("Enterprise Corp"),
                role: PartyRole::Buyer,
                jurisdiction: String::from("Texas, USA"),
            },
        ],
        clauses: vec![ContractClause {
            clause_id: 1,
            clause_type: ClauseType::Payment,
            text: String::from("Annual license fee of $50,000 due on contract anniversary."),
            is_mandatory: true,
        }],
        effective_date: 1700000000,
        expiry_date: Some(1731536000),
        status: ContractStatus::Active,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&contract, cfg).expect("encode LegalContract active");
    let (decoded, _): (LegalContract, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode LegalContract active");
    assert_eq!(contract, decoded);
}

#[test]
fn test_legal_contract_draft_no_expiry() {
    let contract = LegalContract {
        contract_id: 6002,
        title: String::from("Consulting Services Agreement"),
        parties: vec![ContractParty {
            party_id: 10,
            name: String::from("Consultancy Group"),
            role: PartyRole::Vendor,
            jurisdiction: String::from("New York, USA"),
        }],
        clauses: vec![],
        effective_date: 1710000000,
        expiry_date: None,
        status: ContractStatus::Draft,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&contract, cfg).expect("encode LegalContract draft no expiry");
    let (decoded, _): (LegalContract, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode LegalContract draft no expiry");
    assert_eq!(contract, decoded);
}

#[test]
fn test_legal_contract_expired_big_endian() {
    let contract = LegalContract {
        contract_id: 7003,
        title: String::from("Supply Chain Agreement"),
        parties: vec![],
        clauses: vec![ContractClause {
            clause_id: 99,
            clause_type: ClauseType::Delivery,
            text: String::from("Goods shall be delivered within 14 business days of order."),
            is_mandatory: false,
        }],
        effective_date: 1600000000,
        expiry_date: Some(1631536000),
        status: ContractStatus::Expired,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&contract, cfg).expect("encode LegalContract expired big endian");
    let (decoded, _): (LegalContract, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode LegalContract expired big endian");
    assert_eq!(contract, decoded);
}

#[test]
fn test_legal_contract_terminated_fixed_int() {
    let contract = LegalContract {
        contract_id: 8004,
        title: String::from("Terminated Partnership Agreement"),
        parties: vec![
            ContractParty {
                party_id: 50,
                name: String::from("Partner A"),
                role: PartyRole::Vendor,
                jurisdiction: String::from("Paris, France"),
            },
            ContractParty {
                party_id: 51,
                name: String::from("Partner B"),
                role: PartyRole::Buyer,
                jurisdiction: String::from("Berlin, Germany"),
            },
        ],
        clauses: vec![ContractClause {
            clause_id: 10,
            clause_type: ClauseType::Warranty,
            text: String::from("Products warranted for 12 months from delivery."),
            is_mandatory: true,
        }],
        effective_date: 1580000000,
        expiry_date: None,
        status: ContractStatus::Terminated,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&contract, cfg).expect("encode LegalContract terminated fixed int");
    let (decoded, _): (LegalContract, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode LegalContract terminated fixed int");
    assert_eq!(contract, decoded);
}

#[test]
fn test_document_signature_standard() {
    let sig = DocumentSignature {
        signer_id: 9001,
        timestamp: 1720000000,
        signature_hash: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&sig, cfg).expect("encode DocumentSignature standard");
    let (decoded, _): (DocumentSignature, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode DocumentSignature standard");
    assert_eq!(sig, decoded);
}

#[test]
fn test_document_signature_big_endian() {
    let sig = DocumentSignature {
        signer_id: 9002,
        timestamp: 1720001234,
        signature_hash: vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ],
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&sig, cfg).expect("encode DocumentSignature big endian");
    let (decoded, _): (DocumentSignature, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode DocumentSignature big endian");
    assert_eq!(sig, decoded);
}

#[test]
fn test_document_signature_empty_hash() {
    let sig = DocumentSignature {
        signer_id: 9003,
        timestamp: 1720002468,
        signature_hash: vec![],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&sig, cfg).expect("encode DocumentSignature empty hash");
    let (decoded, _): (DocumentSignature, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode DocumentSignature empty hash");
    assert_eq!(sig, decoded);
}

#[test]
fn test_audit_entry_created_standard() {
    let entry = AuditEntry {
        entry_id: 1,
        contract_id: 5001,
        action: AuditAction::Created,
        actor_id: 100,
        timestamp: 1700000000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&entry, cfg).expect("encode AuditEntry created");
    let (decoded, _): (AuditEntry, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AuditEntry created");
    assert_eq!(entry, decoded);
}

#[test]
fn test_audit_entry_signed_big_endian() {
    let entry = AuditEntry {
        entry_id: 2,
        contract_id: 5001,
        action: AuditAction::Signed,
        actor_id: 101,
        timestamp: 1700100000,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&entry, cfg).expect("encode AuditEntry signed big endian");
    let (decoded, _): (AuditEntry, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AuditEntry signed big endian");
    assert_eq!(entry, decoded);
}

#[test]
fn test_audit_entry_amended_fixed_int() {
    let entry = AuditEntry {
        entry_id: 3,
        contract_id: 5001,
        action: AuditAction::Amended,
        actor_id: 102,
        timestamp: 1700200000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&entry, cfg).expect("encode AuditEntry amended fixed int");
    let (decoded, _): (AuditEntry, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AuditEntry amended fixed int");
    assert_eq!(entry, decoded);
}

#[test]
fn test_audit_entry_terminated_standard() {
    let entry = AuditEntry {
        entry_id: 4,
        contract_id: 5001,
        action: AuditAction::Terminated,
        actor_id: 103,
        timestamp: 1700300000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&entry, cfg).expect("encode AuditEntry terminated");
    let (decoded, _): (AuditEntry, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AuditEntry terminated");
    assert_eq!(entry, decoded);
}

#[test]
fn test_vec_of_contracts_roundtrip() {
    let contracts = vec![
        LegalContract {
            contract_id: 1001,
            title: String::from("Distribution Agreement"),
            parties: vec![ContractParty {
                party_id: 1,
                name: String::from("Distributor Co"),
                role: PartyRole::Vendor,
                jurisdiction: String::from("Singapore"),
            }],
            clauses: vec![ContractClause {
                clause_id: 1,
                clause_type: ClauseType::Delivery,
                text: String::from("Goods delivered FOB origin."),
                is_mandatory: true,
            }],
            effective_date: 1700000000,
            expiry_date: Some(1731536000),
            status: ContractStatus::Active,
        },
        LegalContract {
            contract_id: 1002,
            title: String::from("Non-Disclosure Agreement"),
            parties: vec![
                ContractParty {
                    party_id: 2,
                    name: String::from("Startup Alpha"),
                    role: PartyRole::Vendor,
                    jurisdiction: String::from("Estonia"),
                },
                ContractParty {
                    party_id: 3,
                    name: String::from("Investor Beta"),
                    role: PartyRole::Buyer,
                    jurisdiction: String::from("Finland"),
                },
            ],
            clauses: vec![ContractClause {
                clause_id: 10,
                clause_type: ClauseType::Confidentiality,
                text: String::from(
                    "Five-year confidentiality obligation on all disclosed materials.",
                ),
                is_mandatory: true,
            }],
            effective_date: 1710000000,
            expiry_date: None,
            status: ContractStatus::Draft,
        },
        LegalContract {
            contract_id: 1003,
            title: String::from("Service Level Agreement"),
            parties: vec![],
            clauses: vec![],
            effective_date: 1690000000,
            expiry_date: Some(1720000000),
            status: ContractStatus::Expired,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&contracts, cfg).expect("encode Vec<LegalContract>");
    let (decoded, _): (Vec<LegalContract>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Vec<LegalContract>");
    assert_eq!(contracts, decoded);
    assert_eq!(decoded.len(), 3);
}

#[test]
fn test_contract_empty_clauses_and_parties() {
    let contract = LegalContract {
        contract_id: 9999,
        title: String::from("Shell Agreement"),
        parties: vec![],
        clauses: vec![],
        effective_date: 1720000000,
        expiry_date: None,
        status: ContractStatus::Draft,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&contract, cfg).expect("encode empty clauses/parties contract");
    let (decoded, _): (LegalContract, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode empty clauses/parties contract");
    assert_eq!(contract, decoded);
    assert!(decoded.parties.is_empty());
    assert!(decoded.clauses.is_empty());
}

#[test]
fn test_contract_clause_long_text_fixed_int() {
    let long_text = "This clause constitutes the entire warranty section of the agreement. \
        The vendor warrants that all goods and services supplied under this contract shall \
        conform to the specifications set forth in Schedule A, shall be free from defects \
        in materials and workmanship for a period of twenty-four (24) months from the date \
        of delivery or acceptance, whichever occurs later. In the event of any breach of \
        this warranty, the vendor shall, at its sole expense and at the buyer's election, \
        either repair or replace the defective goods, or refund the purchase price thereof. \
        This warranty does not extend to damage caused by misuse, neglect, unauthorized \
        modification, or normal wear and tear. The buyer must notify the vendor in writing \
        within thirty (30) days of discovering any defect to preserve warranty rights.";
    let clause = ContractClause {
        clause_id: 777,
        clause_type: ClauseType::Warranty,
        text: String::from(long_text),
        is_mandatory: true,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&clause, cfg).expect("encode long text clause fixed int");
    let (decoded, _): (ContractClause, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode long text clause fixed int");
    assert_eq!(clause, decoded);
    assert_eq!(decoded.text.len(), long_text.len());
}

#[test]
fn test_legal_contract_long_title_big_endian() {
    let long_title = "Master Services and Intellectual Property Assignment Agreement Between \
        TechnoVentures International Corporation and Global Strategic Partners Limited \
        Governing the Development, Licensing, and Commercial Exploitation of Proprietary \
        Artificial Intelligence Technologies and Related Software Assets";
    let contract = LegalContract {
        contract_id: 88888,
        title: String::from(long_title),
        parties: vec![
            ContractParty {
                party_id: 200,
                name: String::from("TechnoVentures International Corporation"),
                role: PartyRole::Vendor,
                jurisdiction: String::from("State of Delaware, United States of America"),
            },
            ContractParty {
                party_id: 201,
                name: String::from("Global Strategic Partners Limited"),
                role: PartyRole::Buyer,
                jurisdiction: String::from("Cayman Islands, British Overseas Territory"),
            },
            ContractParty {
                party_id: 202,
                name: String::from("First National Guarantee Bank"),
                role: PartyRole::Guarantor,
                jurisdiction: String::from("New York, United States of America"),
            },
            ContractParty {
                party_id: 203,
                name: String::from("Independent Legal Observer"),
                role: PartyRole::Witness,
                jurisdiction: String::from("Geneva, Switzerland"),
            },
        ],
        clauses: vec![
            ContractClause {
                clause_id: 1,
                clause_type: ClauseType::Payment,
                text: String::from(
                    "Total consideration of USD 10,000,000 payable in quarterly installments.",
                ),
                is_mandatory: true,
            },
            ContractClause {
                clause_id: 2,
                clause_type: ClauseType::Confidentiality,
                text: String::from(
                    "Perpetual confidentiality obligations survive contract termination.",
                ),
                is_mandatory: true,
            },
            ContractClause {
                clause_id: 3,
                clause_type: ClauseType::Termination,
                text: String::from(
                    "Termination requires unanimous board approval and 90-day notice.",
                ),
                is_mandatory: false,
            },
        ],
        effective_date: 1715000000,
        expiry_date: Some(1840000000),
        status: ContractStatus::Active,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&contract, cfg).expect("encode long title contract big endian");
    let (decoded, _): (LegalContract, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode long title contract big endian");
    assert_eq!(contract, decoded);
    assert_eq!(decoded.parties.len(), 4);
    assert_eq!(decoded.clauses.len(), 3);
}
