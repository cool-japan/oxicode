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

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ContractStatus {
    Draft,
    UnderReview,
    Signed,
    Active,
    Expired,
    Terminated,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PartyRole {
    Client,
    Vendor,
    Contractor,
    Partner,
    Guarantor,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContractParty {
    party_id: u64,
    name: String,
    role: PartyRole,
    jurisdiction: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Clause {
    clause_id: u32,
    title: String,
    content: String,
    mandatory: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Contract {
    contract_id: u64,
    title: String,
    status: ContractStatus,
    parties: Vec<ContractParty>,
    clauses: Vec<Clause>,
    effective_date: u64,
    expiry_date: u64,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn make_party(id: u64, name: &str, role: PartyRole, jurisdiction: &str) -> ContractParty {
    ContractParty {
        party_id: id,
        name: name.to_string(),
        role,
        jurisdiction: jurisdiction.to_string(),
    }
}

fn make_clause(id: u32, title: &str, content: &str, mandatory: bool) -> Clause {
    Clause {
        clause_id: id,
        title: title.to_string(),
        content: content.to_string(),
        mandatory,
    }
}

fn make_basic_contract(status: ContractStatus) -> Contract {
    Contract {
        contract_id: 1001,
        title: "Master Service Agreement".to_string(),
        status,
        parties: vec![
            make_party(1, "Acme Corp", PartyRole::Client, "US-NY"),
            make_party(2, "TechVendor Ltd", PartyRole::Vendor, "UK-LON"),
        ],
        clauses: vec![
            make_clause(
                1,
                "Scope of Work",
                "Vendor shall provide software services.",
                true,
            ),
            make_clause(2, "Payment Terms", "Client shall pay within 30 days.", true),
        ],
        effective_date: 1_700_000_000,
        expiry_date: 1_731_536_000,
    }
}

// ── Test 1: ContractParty roundtrip (standard config) ────────────────────────

#[test]
fn test_contract_party_roundtrip_standard() {
    let cfg = config::standard();
    let party = make_party(42, "GlobalLaw LLC", PartyRole::Contractor, "DE-BER");
    let bytes = encode_to_vec(&party, cfg).expect("encode ContractParty");
    let (decoded, _consumed): (ContractParty, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ContractParty");
    assert_eq!(party, decoded);
}

// ── Test 2: Clause roundtrip (standard config) ────────────────────────────────

#[test]
fn test_clause_roundtrip_standard() {
    let cfg = config::standard();
    let clause = make_clause(
        7,
        "Confidentiality",
        "All information is confidential.",
        true,
    );
    let bytes = encode_to_vec(&clause, cfg).expect("encode Clause");
    let (decoded, _consumed): (Clause, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Clause");
    assert_eq!(clause, decoded);
}

// ── Test 3: Contract roundtrip (standard config) ──────────────────────────────

#[test]
fn test_contract_roundtrip_standard() {
    let cfg = config::standard();
    let contract = make_basic_contract(ContractStatus::Active);
    let bytes = encode_to_vec(&contract, cfg).expect("encode Contract");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Contract");
    assert_eq!(contract, decoded);
}

// ── Test 4: ContractStatus::Draft variant ────────────────────────────────────

#[test]
fn test_contract_status_draft() {
    let cfg = config::standard();
    let status = ContractStatus::Draft;
    let bytes = encode_to_vec(&status, cfg).expect("encode Draft");
    let (decoded, _consumed): (ContractStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Draft");
    assert_eq!(status, decoded);
}

// ── Test 5: ContractStatus::UnderReview variant ──────────────────────────────

#[test]
fn test_contract_status_under_review() {
    let cfg = config::standard();
    let status = ContractStatus::UnderReview;
    let bytes = encode_to_vec(&status, cfg).expect("encode UnderReview");
    let (decoded, _consumed): (ContractStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode UnderReview");
    assert_eq!(status, decoded);
}

// ── Test 6: ContractStatus::Signed variant ───────────────────────────────────

#[test]
fn test_contract_status_signed() {
    let cfg = config::standard();
    let status = ContractStatus::Signed;
    let bytes = encode_to_vec(&status, cfg).expect("encode Signed");
    let (decoded, _consumed): (ContractStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Signed");
    assert_eq!(status, decoded);
}

// ── Test 7: ContractStatus::Expired variant ──────────────────────────────────

#[test]
fn test_contract_status_expired() {
    let cfg = config::standard();
    let status = ContractStatus::Expired;
    let bytes = encode_to_vec(&status, cfg).expect("encode Expired");
    let (decoded, _consumed): (ContractStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Expired");
    assert_eq!(status, decoded);
}

// ── Test 8: ContractStatus::Terminated variant ───────────────────────────────

#[test]
fn test_contract_status_terminated() {
    let cfg = config::standard();
    let status = ContractStatus::Terminated;
    let bytes = encode_to_vec(&status, cfg).expect("encode Terminated");
    let (decoded, _consumed): (ContractStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Terminated");
    assert_eq!(status, decoded);
}

// ── Test 9: PartyRole::Client variant ────────────────────────────────────────

#[test]
fn test_party_role_client() {
    let cfg = config::standard();
    let role = PartyRole::Client;
    let bytes = encode_to_vec(&role, cfg).expect("encode Client role");
    let (decoded, _consumed): (PartyRole, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Client role");
    assert_eq!(role, decoded);
}

// ── Test 10: PartyRole::Guarantor variant ────────────────────────────────────

#[test]
fn test_party_role_guarantor() {
    let cfg = config::standard();
    let role = PartyRole::Guarantor;
    let bytes = encode_to_vec(&role, cfg).expect("encode Guarantor role");
    let (decoded, _consumed): (PartyRole, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Guarantor role");
    assert_eq!(role, decoded);
}

// ── Test 11: Contract with big-endian config ──────────────────────────────────

#[test]
fn test_contract_roundtrip_big_endian() {
    let cfg = config::standard().with_big_endian();
    let contract = make_basic_contract(ContractStatus::Signed);
    let bytes = encode_to_vec(&contract, cfg).expect("encode Contract big-endian");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Contract big-endian");
    assert_eq!(contract, decoded);
}

// ── Test 12: Contract with fixed-int encoding ─────────────────────────────────

#[test]
fn test_contract_roundtrip_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let contract = make_basic_contract(ContractStatus::UnderReview);
    let bytes = encode_to_vec(&contract, cfg).expect("encode Contract fixed-int");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Contract fixed-int");
    assert_eq!(contract, decoded);
}

// ── Test 13: Contract with big-endian + fixed-int encoding ────────────────────

#[test]
fn test_contract_roundtrip_big_endian_fixed_int() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let contract = make_basic_contract(ContractStatus::Active);
    let bytes = encode_to_vec(&contract, cfg).expect("encode Contract big-endian fixed-int");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Contract big-endian fixed-int");
    assert_eq!(contract, decoded);
}

// ── Test 14: Empty contract (no parties, no clauses) ─────────────────────────

#[test]
fn test_empty_contract() {
    let cfg = config::standard();
    let contract = Contract {
        contract_id: 9999,
        title: "Empty Framework Agreement".to_string(),
        status: ContractStatus::Draft,
        parties: vec![],
        clauses: vec![],
        effective_date: 0,
        expiry_date: 0,
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode empty Contract");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode empty Contract");
    assert_eq!(contract, decoded);
}

// ── Test 15: Contract with many clauses (stress) ─────────────────────────────

#[test]
fn test_contract_many_clauses() {
    let cfg = config::standard();
    let clauses: Vec<Clause> = (1..=50)
        .map(|i| {
            make_clause(
                i,
                &format!("Clause {i}"),
                &format!("Legal text for clause number {i} defining obligations and rights."),
                i % 3 == 0,
            )
        })
        .collect();
    let contract = Contract {
        contract_id: 5000,
        title: "Comprehensive Service Contract".to_string(),
        status: ContractStatus::Active,
        parties: vec![make_party(1, "Alpha Inc", PartyRole::Client, "US-CA")],
        clauses,
        effective_date: 1_690_000_000,
        expiry_date: 1_721_536_000,
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode Contract many clauses");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Contract many clauses");
    assert_eq!(contract, decoded);
    assert_eq!(decoded.clauses.len(), 50);
}

// ── Test 16: Contract with multiple parties ───────────────────────────────────

#[test]
fn test_contract_multiple_parties() {
    let cfg = config::standard();
    let parties = vec![
        make_party(1, "BuilderCo", PartyRole::Contractor, "US-TX"),
        make_party(2, "FinanceCorp", PartyRole::Guarantor, "CH-ZUG"),
        make_party(3, "ClientGroup", PartyRole::Client, "SG"),
        make_party(4, "SoftwareFirm", PartyRole::Vendor, "EE-TAL"),
        make_party(5, "ConsultingLLP", PartyRole::Partner, "UK-EDI"),
    ];
    let contract = Contract {
        contract_id: 7777,
        title: "Multi-Party Joint Venture Agreement".to_string(),
        status: ContractStatus::UnderReview,
        parties,
        clauses: vec![make_clause(
            1,
            "Governance",
            "Governance structure details.",
            true,
        )],
        effective_date: 1_700_000_000,
        expiry_date: 1_762_536_000,
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode multi-party Contract");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode multi-party Contract");
    assert_eq!(contract, decoded);
    assert_eq!(decoded.parties.len(), 5);
}

// ── Test 17: Clause with non-mandatory flag ───────────────────────────────────

#[test]
fn test_clause_non_mandatory() {
    let cfg = config::standard();
    let clause = make_clause(
        99,
        "Optional Arbitration",
        "Parties may opt for arbitration.",
        false,
    );
    let bytes = encode_to_vec(&clause, cfg).expect("encode non-mandatory Clause");
    let (decoded, _consumed): (Clause, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode non-mandatory Clause");
    assert_eq!(clause, decoded);
    assert!(!decoded.mandatory);
}

// ── Test 18: PartyRole all variants roundtrip in one pass ─────────────────────

#[test]
fn test_all_party_roles_roundtrip() {
    let cfg = config::standard();
    let roles = vec![
        PartyRole::Client,
        PartyRole::Vendor,
        PartyRole::Contractor,
        PartyRole::Partner,
        PartyRole::Guarantor,
    ];
    for role in &roles {
        let bytes = encode_to_vec(role, cfg).expect("encode PartyRole variant");
        let (decoded, _consumed): (PartyRole, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode PartyRole variant");
        assert_eq!(role, &decoded);
    }
}

// ── Test 19: All ContractStatus variants roundtrip in one pass ────────────────

#[test]
fn test_all_contract_statuses_roundtrip() {
    let cfg = config::standard();
    let statuses = vec![
        ContractStatus::Draft,
        ContractStatus::UnderReview,
        ContractStatus::Signed,
        ContractStatus::Active,
        ContractStatus::Expired,
        ContractStatus::Terminated,
    ];
    for status in &statuses {
        let bytes = encode_to_vec(status, cfg).expect("encode ContractStatus variant");
        let (decoded, _consumed): (ContractStatus, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode ContractStatus variant");
        assert_eq!(status, &decoded);
    }
}

// ── Test 20: Bytes consumed is non-zero ──────────────────────────────────────

#[test]
fn test_bytes_consumed_non_zero() {
    let cfg = config::standard();
    let contract = make_basic_contract(ContractStatus::Active);
    let bytes = encode_to_vec(&contract, cfg).expect("encode Contract for size check");
    let (_decoded, consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Contract for size check");
    assert!(consumed > 0, "bytes consumed should be greater than zero");
    assert_eq!(consumed, bytes.len(), "all bytes should be consumed");
}

// ── Test 21: Contract with unicode content in clauses ─────────────────────────

#[test]
fn test_contract_unicode_content() {
    let cfg = config::standard();
    let clause = make_clause(
        1,
        "Haftungsausschluss",
        "Alle Parteien sind einverstanden: 合同条款 및 条件 — αρμοδιότητα.",
        true,
    );
    let contract = Contract {
        contract_id: 2002,
        title: "Internationales Rahmenabkommen 国際協定".to_string(),
        status: ContractStatus::Signed,
        parties: vec![make_party(
            10,
            "München GmbH 东京支社",
            PartyRole::Partner,
            "DE-MUC",
        )],
        clauses: vec![clause],
        effective_date: 1_705_000_000,
        expiry_date: 1_736_536_000,
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode unicode Contract");
    let (decoded, _consumed): (Contract, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode unicode Contract");
    assert_eq!(contract, decoded);
}

// ── Test 22: Large contract stress test with all configs ──────────────────────

#[test]
fn test_large_contract_all_configs() {
    let parties: Vec<ContractParty> = (1..=10)
        .map(|i| {
            make_party(
                i,
                &format!("Party Organization {i}"),
                if i % 2 == 0 {
                    PartyRole::Vendor
                } else {
                    PartyRole::Client
                },
                &format!("JURISDICTION-{i:03}"),
            )
        })
        .collect();
    let clauses: Vec<Clause> = (1..=100)
        .map(|i| {
            make_clause(
                i,
                &format!("Legal Clause {i:03}"),
                &format!(
                    "This clause number {i} establishes binding obligations \
                 for all parties as defined herein. Violation may result \
                 in penalties as described in Appendix {i}.",
                ),
                i <= 20,
            )
        })
        .collect();
    let contract = Contract {
        contract_id: u64::MAX / 2,
        title: "Omnibus Framework Agreement for Enterprise Services".to_string(),
        status: ContractStatus::Active,
        parties,
        clauses,
        effective_date: 1_700_000_000,
        expiry_date: 1_893_456_000,
    };

    // Standard (little-endian varint)
    let cfg_std = config::standard();
    let bytes_std = encode_to_vec(&contract, cfg_std).expect("encode large Contract standard");
    let (decoded_std, _): (Contract, usize) =
        decode_owned_from_slice(&bytes_std, cfg_std).expect("decode large Contract standard");
    assert_eq!(contract, decoded_std);

    // Big-endian varint
    let cfg_be = config::standard().with_big_endian();
    let bytes_be = encode_to_vec(&contract, cfg_be).expect("encode large Contract big-endian");
    let (decoded_be, _): (Contract, usize) =
        decode_owned_from_slice(&bytes_be, cfg_be).expect("decode large Contract big-endian");
    assert_eq!(contract, decoded_be);

    // Little-endian fixed-int
    let cfg_fx = config::standard().with_fixed_int_encoding();
    let bytes_fx = encode_to_vec(&contract, cfg_fx).expect("encode large Contract fixed-int");
    let (decoded_fx, _): (Contract, usize) =
        decode_owned_from_slice(&bytes_fx, cfg_fx).expect("decode large Contract fixed-int");
    assert_eq!(contract, decoded_fx);

    assert_eq!(decoded_std.clauses.len(), 100);
    assert_eq!(decoded_std.parties.len(), 10);
}
