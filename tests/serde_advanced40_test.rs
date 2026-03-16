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
enum PolicyType {
    Auto,
    Home,
    Life,
    Health,
    Commercial,
    Travel,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ClaimStatus {
    Filed,
    UnderReview,
    Approved,
    Denied,
    Paid,
    Closed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CoverageType {
    Liability,
    Collision,
    Comprehensive,
    Medical,
    Property,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RiskCategory {
    Low,
    Medium,
    High,
    Extreme,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Policyholder {
    holder_id: u64,
    name: String,
    dob_year: u16,
    risk_category: RiskCategory,
    policy_count: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Policy {
    policy_id: u64,
    holder_id: u64,
    policy_type: PolicyType,
    coverage_type: CoverageType,
    premium_cents: u32,
    deductible_cents: u32,
    effective_date: u64,
    expiry_date: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Claim {
    claim_id: u64,
    policy_id: u64,
    filed_date: u64,
    incident_date: u64,
    amount_cents: u32,
    status: ClaimStatus,
    description: String,
    adjuster_notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ClaimDocument {
    doc_id: u64,
    claim_id: u64,
    doc_type: String,
    file_size_kb: u32,
    uploaded_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ActuarialTable {
    table_id: u32,
    policy_type: PolicyType,
    age_bracket: u8,
    base_rate_x10000: u32,
    risk_multipliers: Vec<f32>,
}

// Test 1: Basic Policyholder roundtrip with standard config
#[test]
fn test_policyholder_standard_roundtrip() {
    let holder = Policyholder {
        holder_id: 100001,
        name: String::from("Alice Johnson"),
        dob_year: 1978,
        risk_category: RiskCategory::Low,
        policy_count: 3,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&holder, cfg).expect("encode Policyholder");
    let (decoded, consumed): (Policyholder, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Policyholder");
    assert_eq!(holder, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 2: Policyholder with Extreme risk category
#[test]
fn test_policyholder_extreme_risk() {
    let holder = Policyholder {
        holder_id: 999999,
        name: String::from("Carlos Danger"),
        dob_year: 1955,
        risk_category: RiskCategory::Extreme,
        policy_count: 12,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&holder, cfg).expect("encode extreme risk holder");
    let (decoded, consumed): (Policyholder, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode extreme risk holder");
    assert_eq!(holder, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 3: Policy with Auto type and Collision coverage, standard config
#[test]
fn test_policy_auto_collision_standard() {
    let policy = Policy {
        policy_id: 20230001,
        holder_id: 100001,
        policy_type: PolicyType::Auto,
        coverage_type: CoverageType::Collision,
        premium_cents: 120000,
        deductible_cents: 50000,
        effective_date: 1672531200,
        expiry_date: 1704067200,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&policy, cfg).expect("encode auto collision policy");
    let (decoded, consumed): (Policy, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode auto collision policy");
    assert_eq!(policy, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 4: Policy with Home type and Property coverage, big endian
#[test]
fn test_policy_home_property_big_endian() {
    let policy = Policy {
        policy_id: 20230002,
        holder_id: 100002,
        policy_type: PolicyType::Home,
        coverage_type: CoverageType::Property,
        premium_cents: 85000,
        deductible_cents: 100000,
        effective_date: 1675209600,
        expiry_date: 1706745600,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&policy, cfg).expect("encode home property policy big endian");
    let (decoded, consumed): (Policy, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode home property policy big endian");
    assert_eq!(policy, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 5: Claim with Some adjuster_notes and Approved status
#[test]
fn test_claim_approved_with_adjuster_notes() {
    let claim = Claim {
        claim_id: 5000001,
        policy_id: 20230001,
        filed_date: 1680307200,
        incident_date: 1680220800,
        amount_cents: 350000,
        status: ClaimStatus::Approved,
        description: String::from("Rear-end collision on I-95 northbound"),
        adjuster_notes: Some(String::from(
            "Damage consistent with reported incident. Approved for full claim amount.",
        )),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&claim, cfg).expect("encode approved claim");
    let (decoded, consumed): (Claim, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode approved claim");
    assert_eq!(claim, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 6: Claim with None adjuster_notes and Filed status
#[test]
fn test_claim_filed_no_adjuster_notes() {
    let claim = Claim {
        claim_id: 5000002,
        policy_id: 20230002,
        filed_date: 1682899200,
        incident_date: 1682812800,
        amount_cents: 780000,
        status: ClaimStatus::Filed,
        description: String::from("Water damage from burst pipe in basement"),
        adjuster_notes: None,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&claim, cfg).expect("encode filed claim no notes");
    let (decoded, consumed): (Claim, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode filed claim no notes");
    assert_eq!(claim, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 7: Claim with Denied status and adjuster notes explaining denial
#[test]
fn test_claim_denied_with_notes() {
    let claim = Claim {
        claim_id: 5000003,
        policy_id: 20230003,
        filed_date: 1685577600,
        incident_date: 1685491200,
        amount_cents: 1200000,
        status: ClaimStatus::Denied,
        description: String::from("Roof damage claimed as storm-related"),
        adjuster_notes: Some(String::from(
            "Inspection reveals pre-existing damage not covered under current policy terms.",
        )),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&claim, cfg).expect("encode denied claim");
    let (decoded, _): (Claim, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode denied claim");
    assert_eq!(claim, decoded);
}

// Test 8: ClaimDocument with standard config
#[test]
fn test_claim_document_standard_roundtrip() {
    let doc = ClaimDocument {
        doc_id: 9000001,
        claim_id: 5000001,
        doc_type: String::from("police_report"),
        file_size_kb: 1024,
        uploaded_at: 1680393600,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&doc, cfg).expect("encode claim document");
    let (decoded, consumed): (ClaimDocument, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode claim document");
    assert_eq!(doc, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 9: ClaimDocument with fixed int encoding
#[test]
fn test_claim_document_fixed_int() {
    let doc = ClaimDocument {
        doc_id: 9000002,
        claim_id: 5000002,
        doc_type: String::from("damage_photos"),
        file_size_kb: 40960,
        uploaded_at: 1682985600,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&doc, cfg).expect("encode claim document fixed int");
    let (decoded, consumed): (ClaimDocument, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode claim document fixed int");
    assert_eq!(doc, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 10: ActuarialTable with multiple risk multipliers
#[test]
fn test_actuarial_table_roundtrip() {
    let table = ActuarialTable {
        table_id: 1001,
        policy_type: PolicyType::Life,
        age_bracket: 45,
        base_rate_x10000: 2350,
        risk_multipliers: vec![1.0, 1.15, 1.35, 1.60, 2.10],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&table, cfg).expect("encode actuarial table");
    let (decoded, consumed): (ActuarialTable, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode actuarial table");
    assert_eq!(table, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 11: Large actuarial table with many risk multipliers
#[test]
fn test_large_actuarial_table() {
    let multipliers: Vec<f32> = (0..200).map(|i| 1.0_f32 + (i as f32) * 0.01).collect();
    let table = ActuarialTable {
        table_id: 9999,
        policy_type: PolicyType::Health,
        age_bracket: 65,
        base_rate_x10000: 8750,
        risk_multipliers: multipliers,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&table, cfg).expect("encode large actuarial table");
    let (decoded, consumed): (ActuarialTable, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode large actuarial table");
    assert_eq!(table.table_id, decoded.table_id);
    assert_eq!(table.risk_multipliers.len(), decoded.risk_multipliers.len());
    assert_eq!(consumed, encoded.len());
}

// Test 12: Vec of claims roundtrip
#[test]
fn test_vec_of_claims_roundtrip() {
    let claims = vec![
        Claim {
            claim_id: 6000001,
            policy_id: 20230010,
            filed_date: 1688256000,
            incident_date: 1688169600,
            amount_cents: 250000,
            status: ClaimStatus::UnderReview,
            description: String::from("Minor fender bender in parking lot"),
            adjuster_notes: None,
        },
        Claim {
            claim_id: 6000002,
            policy_id: 20230010,
            filed_date: 1691020800,
            incident_date: 1690934400,
            amount_cents: 1500000,
            status: ClaimStatus::Paid,
            description: String::from("Total loss vehicle in major accident"),
            adjuster_notes: Some(String::from(
                "Settlement agreed upon after independent appraisal.",
            )),
        },
        Claim {
            claim_id: 6000003,
            policy_id: 20230011,
            filed_date: 1693699200,
            incident_date: 1693612800,
            amount_cents: 45000,
            status: ClaimStatus::Closed,
            description: String::from("Windshield chip repair"),
            adjuster_notes: Some(String::from("Covered under glass repair provision.")),
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&claims, cfg).expect("encode vec of claims");
    let (decoded, consumed): (Vec<Claim>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode vec of claims");
    assert_eq!(claims, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
}

// Test 13: Policyholder with big endian config
#[test]
fn test_policyholder_big_endian() {
    let holder = Policyholder {
        holder_id: 200001,
        name: String::from("Bernard Okafor"),
        dob_year: 1965,
        risk_category: RiskCategory::Medium,
        policy_count: 5,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&holder, cfg).expect("encode policyholder big endian");
    let (decoded, consumed): (Policyholder, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode policyholder big endian");
    assert_eq!(holder, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 14: Commercial policy with Liability coverage and fixed int encoding
#[test]
fn test_policy_commercial_liability_fixed_int() {
    let policy = Policy {
        policy_id: 30000001,
        holder_id: 300001,
        policy_type: PolicyType::Commercial,
        coverage_type: CoverageType::Liability,
        premium_cents: 5000000,
        deductible_cents: 1000000,
        effective_date: 1704067200,
        expiry_date: 1735689600,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&policy, cfg).expect("encode commercial liability policy");
    let (decoded, consumed): (Policy, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode commercial liability policy");
    assert_eq!(policy, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 15: Travel policy with Medical coverage
#[test]
fn test_policy_travel_medical() {
    let policy = Policy {
        policy_id: 40000001,
        holder_id: 400001,
        policy_type: PolicyType::Travel,
        coverage_type: CoverageType::Medical,
        premium_cents: 25000,
        deductible_cents: 10000,
        effective_date: 1706745600,
        expiry_date: 1707350400,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&policy, cfg).expect("encode travel medical policy");
    let (decoded, consumed): (Policy, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode travel medical policy");
    assert_eq!(policy, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 16: Claim with UnderReview status and big endian
#[test]
fn test_claim_under_review_big_endian() {
    let claim = Claim {
        claim_id: 7000001,
        policy_id: 30000001,
        filed_date: 1709424000,
        incident_date: 1709337600,
        amount_cents: 8500000,
        status: ClaimStatus::UnderReview,
        description: String::from(
            "Warehouse fire causing significant property and inventory damage",
        ),
        adjuster_notes: Some(String::from(
            "Awaiting fire marshal report and inventory audit documentation.",
        )),
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&claim, cfg).expect("encode under review claim big endian");
    let (decoded, consumed): (Claim, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode under review claim big endian");
    assert_eq!(claim, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 17: Consumed bytes verification for multiple sequential items
#[test]
fn test_consumed_bytes_verification_multiple_items() {
    let cfg = config::standard();

    let holder = Policyholder {
        holder_id: 500001,
        name: String::from("Diana Prince"),
        dob_year: 1990,
        risk_category: RiskCategory::High,
        policy_count: 2,
    };
    let policy = Policy {
        policy_id: 50000001,
        holder_id: 500001,
        policy_type: PolicyType::Health,
        coverage_type: CoverageType::Comprehensive,
        premium_cents: 300000,
        deductible_cents: 150000,
        effective_date: 1704067200,
        expiry_date: 1735689600,
    };

    let encoded_holder = encode_to_vec(&holder, cfg).expect("encode holder for bytes check");
    let encoded_policy = encode_to_vec(&policy, cfg).expect("encode policy for bytes check");

    let (decoded_holder, h_consumed): (Policyholder, usize) =
        decode_owned_from_slice(&encoded_holder, cfg).expect("decode holder for bytes check");
    let (decoded_policy, p_consumed): (Policy, usize) =
        decode_owned_from_slice(&encoded_policy, cfg).expect("decode policy for bytes check");

    assert_eq!(holder, decoded_holder);
    assert_eq!(policy, decoded_policy);
    assert_eq!(h_consumed, encoded_holder.len());
    assert_eq!(p_consumed, encoded_policy.len());
}

// Test 18: ActuarialTable for Auto type with big endian
#[test]
fn test_actuarial_table_auto_big_endian() {
    let table = ActuarialTable {
        table_id: 2001,
        policy_type: PolicyType::Auto,
        age_bracket: 25,
        base_rate_x10000: 4800,
        risk_multipliers: vec![1.0, 1.25, 1.75, 2.50, 4.00],
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&table, cfg).expect("encode auto actuarial table big endian");
    let (decoded, consumed): (ActuarialTable, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode auto actuarial table big endian");
    assert_eq!(table, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 19: ClaimDocument with big endian for audit trail
#[test]
fn test_claim_document_big_endian_audit_trail() {
    let doc = ClaimDocument {
        doc_id: 9100001,
        claim_id: 7000001,
        doc_type: String::from("fire_marshal_report"),
        file_size_kb: 2048,
        uploaded_at: 1709596800,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&doc, cfg).expect("encode audit trail document big endian");
    let (decoded, consumed): (ClaimDocument, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode audit trail document big endian");
    assert_eq!(doc, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 20: Vec of policyholders with diverse risk categories
#[test]
fn test_vec_of_policyholders_all_risk_categories() {
    let holders = vec![
        Policyholder {
            holder_id: 600001,
            name: String::from("Ethan Low"),
            dob_year: 1985,
            risk_category: RiskCategory::Low,
            policy_count: 1,
        },
        Policyholder {
            holder_id: 600002,
            name: String::from("Fiona Medium"),
            dob_year: 1972,
            risk_category: RiskCategory::Medium,
            policy_count: 4,
        },
        Policyholder {
            holder_id: 600003,
            name: String::from("George High"),
            dob_year: 1960,
            risk_category: RiskCategory::High,
            policy_count: 7,
        },
        Policyholder {
            holder_id: 600004,
            name: String::from("Hannah Extreme"),
            dob_year: 1948,
            risk_category: RiskCategory::Extreme,
            policy_count: 10,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&holders, cfg).expect("encode vec of policyholders");
    let (decoded, consumed): (Vec<Policyholder>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode vec of policyholders");
    assert_eq!(holders, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 4);
}

// Test 21: All ClaimStatus variants roundtrip
#[test]
fn test_all_claim_statuses_roundtrip() {
    let statuses = vec![
        ClaimStatus::Filed,
        ClaimStatus::UnderReview,
        ClaimStatus::Approved,
        ClaimStatus::Denied,
        ClaimStatus::Paid,
        ClaimStatus::Closed,
    ];
    let cfg = config::standard();
    for status in &statuses {
        let encoded = encode_to_vec(status, cfg).expect("encode claim status");
        let (decoded, consumed): (ClaimStatus, usize) =
            decode_owned_from_slice(&encoded, cfg).expect("decode claim status");
        assert_eq!(status, &decoded);
        assert_eq!(consumed, encoded.len());
    }
}

// Test 22: Full insurance processing pipeline — policyholder, policy, claim, and documents together
#[test]
fn test_full_insurance_pipeline_roundtrip() {
    let cfg = config::standard();

    let holder = Policyholder {
        holder_id: 700001,
        name: String::from("Ingrid Sorensen"),
        dob_year: 1980,
        risk_category: RiskCategory::Medium,
        policy_count: 3,
    };
    let policy = Policy {
        policy_id: 70000001,
        holder_id: 700001,
        policy_type: PolicyType::Home,
        coverage_type: CoverageType::Comprehensive,
        premium_cents: 175000,
        deductible_cents: 250000,
        effective_date: 1701388800,
        expiry_date: 1732924800,
    };
    let claim = Claim {
        claim_id: 8000001,
        policy_id: 70000001,
        filed_date: 1715299200,
        incident_date: 1715212800,
        amount_cents: 620000,
        status: ClaimStatus::Approved,
        description: String::from("Hail damage to roof and siding, storm on 2024-05-09"),
        adjuster_notes: Some(String::from("Hail damage verified via weather data cross-reference. Repair estimates from two contractors reviewed.")),
    };
    let documents = vec![
        ClaimDocument {
            doc_id: 9200001,
            claim_id: 8000001,
            doc_type: String::from("contractor_estimate_1"),
            file_size_kb: 512,
            uploaded_at: 1715385600,
        },
        ClaimDocument {
            doc_id: 9200002,
            claim_id: 8000001,
            doc_type: String::from("contractor_estimate_2"),
            file_size_kb: 480,
            uploaded_at: 1715472000,
        },
        ClaimDocument {
            doc_id: 9200003,
            claim_id: 8000001,
            doc_type: String::from("weather_report"),
            file_size_kb: 256,
            uploaded_at: 1715558400,
        },
    ];
    let actuarial = ActuarialTable {
        table_id: 3001,
        policy_type: PolicyType::Home,
        age_bracket: 44,
        base_rate_x10000: 1750,
        risk_multipliers: vec![1.0, 1.10, 1.25, 1.45, 1.80, 2.30],
    };

    let enc_holder = encode_to_vec(&holder, cfg).expect("encode pipeline holder");
    let enc_policy = encode_to_vec(&policy, cfg).expect("encode pipeline policy");
    let enc_claim = encode_to_vec(&claim, cfg).expect("encode pipeline claim");
    let enc_docs = encode_to_vec(&documents, cfg).expect("encode pipeline documents");
    let enc_actuarial = encode_to_vec(&actuarial, cfg).expect("encode pipeline actuarial");

    let (dec_holder, hc): (Policyholder, usize) =
        decode_owned_from_slice(&enc_holder, cfg).expect("decode pipeline holder");
    let (dec_policy, pc): (Policy, usize) =
        decode_owned_from_slice(&enc_policy, cfg).expect("decode pipeline policy");
    let (dec_claim, cc): (Claim, usize) =
        decode_owned_from_slice(&enc_claim, cfg).expect("decode pipeline claim");
    let (dec_docs, dc): (Vec<ClaimDocument>, usize) =
        decode_owned_from_slice(&enc_docs, cfg).expect("decode pipeline documents");
    let (dec_actuarial, ac): (ActuarialTable, usize) =
        decode_owned_from_slice(&enc_actuarial, cfg).expect("decode pipeline actuarial");

    assert_eq!(holder, dec_holder);
    assert_eq!(policy, dec_policy);
    assert_eq!(claim, dec_claim);
    assert_eq!(documents, dec_docs);
    assert_eq!(actuarial, dec_actuarial);

    assert_eq!(hc, enc_holder.len());
    assert_eq!(pc, enc_policy.len());
    assert_eq!(cc, enc_claim.len());
    assert_eq!(dc, enc_docs.len());
    assert_eq!(ac, enc_actuarial.len());

    assert_eq!(dec_docs.len(), 3);
    assert_eq!(dec_claim.status, ClaimStatus::Approved);
    assert!(dec_claim.adjuster_notes.is_some());
    assert_eq!(dec_holder.risk_category, RiskCategory::Medium);
    assert_eq!(dec_policy.policy_type, PolicyType::Home);
}
