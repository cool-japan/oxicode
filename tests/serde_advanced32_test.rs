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
enum DiseaseCategory {
    Respiratory,
    Gastrointestinal,
    Vectorborne,
    Bloodborne,
    Zoonotic,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CaseStatus {
    Suspected,
    Probable,
    Confirmed,
    Recovered,
    Deceased,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum VaccinationStatus {
    Unvaccinated,
    PartiallyVaccinated,
    FullyVaccinated,
    Boosted,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EpiCase {
    case_id: u64,
    disease: String,
    category: DiseaseCategory,
    status: CaseStatus,
    age: u8,
    vaccination: VaccinationStatus,
    reported_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct OutbreakCluster {
    cluster_id: u64,
    location: String,
    cases: Vec<EpiCase>,
    contact_count: u32,
    index_case_id: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SurveillanceReport {
    report_id: u64,
    region: String,
    period_start: u64,
    period_end: u64,
    clusters: Vec<OutbreakCluster>,
    total_cases: u32,
}

fn make_case(
    case_id: u64,
    disease: &str,
    category: DiseaseCategory,
    status: CaseStatus,
    age: u8,
    vaccination: VaccinationStatus,
    reported_at: u64,
) -> EpiCase {
    EpiCase {
        case_id,
        disease: disease.to_string(),
        category,
        status,
        age,
        vaccination,
        reported_at,
    }
}

// Test 1: each DiseaseCategory variant roundtrips correctly
#[test]
fn test_disease_category_all_variants_roundtrip() {
    let cfg = config::standard();
    let variants = [
        DiseaseCategory::Respiratory,
        DiseaseCategory::Gastrointestinal,
        DiseaseCategory::Vectorborne,
        DiseaseCategory::Bloodborne,
        DiseaseCategory::Zoonotic,
    ];
    for val in &variants {
        let bytes = encode_to_vec(val, cfg).expect("encode DiseaseCategory variant");
        let (decoded, _): (DiseaseCategory, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode DiseaseCategory variant");
        assert_eq!(val, &decoded);
    }
}

// Test 2: each CaseStatus variant roundtrips correctly
#[test]
fn test_case_status_all_variants_roundtrip() {
    let cfg = config::standard();
    let variants = [
        CaseStatus::Suspected,
        CaseStatus::Probable,
        CaseStatus::Confirmed,
        CaseStatus::Recovered,
        CaseStatus::Deceased,
    ];
    for val in &variants {
        let bytes = encode_to_vec(val, cfg).expect("encode CaseStatus variant");
        let (decoded, _): (CaseStatus, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode CaseStatus variant");
        assert_eq!(val, &decoded);
    }
}

// Test 3: each VaccinationStatus variant roundtrips correctly
#[test]
fn test_vaccination_status_all_variants_roundtrip() {
    let cfg = config::standard();
    let variants = [
        VaccinationStatus::Unvaccinated,
        VaccinationStatus::PartiallyVaccinated,
        VaccinationStatus::FullyVaccinated,
        VaccinationStatus::Boosted,
    ];
    for val in &variants {
        let bytes = encode_to_vec(val, cfg).expect("encode VaccinationStatus variant");
        let (decoded, _): (VaccinationStatus, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode VaccinationStatus variant");
        assert_eq!(val, &decoded);
    }
}

// Test 4: EpiCase roundtrip standard
#[test]
fn test_epi_case_roundtrip_standard() {
    let cfg = config::standard();
    let val = make_case(
        1001,
        "Influenza A",
        DiseaseCategory::Respiratory,
        CaseStatus::Confirmed,
        34,
        VaccinationStatus::FullyVaccinated,
        1_700_000_000,
    );
    let bytes = encode_to_vec(&val, cfg).expect("encode EpiCase standard");
    let (decoded, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EpiCase standard");
    assert_eq!(val, decoded);
}

// Test 5: OutbreakCluster with 5 cases
#[test]
fn test_outbreak_cluster_five_cases_roundtrip() {
    let cfg = config::standard();
    let cases: Vec<EpiCase> = (0..5)
        .map(|i| {
            make_case(
                2000 + i as u64,
                "Salmonellosis",
                DiseaseCategory::Gastrointestinal,
                CaseStatus::Confirmed,
                20 + i as u8,
                VaccinationStatus::Unvaccinated,
                1_710_000_000 + i as u64 * 3_600,
            )
        })
        .collect();
    let val = OutbreakCluster {
        cluster_id: 100,
        location: "Springfield County".to_string(),
        cases,
        contact_count: 47,
        index_case_id: 2000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode OutbreakCluster 5 cases");
    let (decoded, _): (OutbreakCluster, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OutbreakCluster 5 cases");
    assert_eq!(val, decoded);
    assert_eq!(decoded.cases.len(), 5);
}

// Test 6: SurveillanceReport with 3 clusters
#[test]
fn test_surveillance_report_three_clusters_roundtrip() {
    let cfg = config::standard();
    let clusters: Vec<OutbreakCluster> = (0..3)
        .map(|ci| {
            let cases: Vec<EpiCase> = (0..2)
                .map(|i| {
                    make_case(
                        3000 + ci as u64 * 10 + i as u64,
                        "Dengue Fever",
                        DiseaseCategory::Vectorborne,
                        CaseStatus::Probable,
                        25 + i as u8,
                        VaccinationStatus::Unvaccinated,
                        1_720_000_000 + ci as u64 * 86_400,
                    )
                })
                .collect();
            OutbreakCluster {
                cluster_id: 200 + ci as u64,
                location: format!("District {}", ci + 1),
                cases,
                contact_count: (ci as u32 + 1) * 15,
                index_case_id: 3000 + ci as u64 * 10,
            }
        })
        .collect();
    let val = SurveillanceReport {
        report_id: 9001,
        region: "Northern Region".to_string(),
        period_start: 1_720_000_000,
        period_end: 1_722_592_000,
        clusters,
        total_cases: 6,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SurveillanceReport 3 clusters");
    let (decoded, _): (SurveillanceReport, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SurveillanceReport 3 clusters");
    assert_eq!(val, decoded);
    assert_eq!(decoded.clusters.len(), 3);
}

// Test 7: Vec<EpiCase> roundtrip
#[test]
fn test_vec_epi_case_roundtrip() {
    let cfg = config::standard();
    let val: Vec<EpiCase> = vec![
        make_case(
            4001,
            "COVID-19",
            DiseaseCategory::Respiratory,
            CaseStatus::Confirmed,
            55,
            VaccinationStatus::Boosted,
            1_700_100_000,
        ),
        make_case(
            4002,
            "Hepatitis B",
            DiseaseCategory::Bloodborne,
            CaseStatus::Probable,
            40,
            VaccinationStatus::PartiallyVaccinated,
            1_700_200_000,
        ),
        make_case(
            4003,
            "Rabies",
            DiseaseCategory::Zoonotic,
            CaseStatus::Suspected,
            29,
            VaccinationStatus::Unvaccinated,
            1_700_300_000,
        ),
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<EpiCase>");
    let (decoded, _): (Vec<EpiCase>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<EpiCase>");
    assert_eq!(val, decoded);
}

// Test 8: Vec<OutbreakCluster> roundtrip
#[test]
fn test_vec_outbreak_cluster_roundtrip() {
    let cfg = config::standard();
    let val: Vec<OutbreakCluster> = vec![
        OutbreakCluster {
            cluster_id: 301,
            location: "Eastville".to_string(),
            cases: vec![make_case(
                5001,
                "Malaria",
                DiseaseCategory::Vectorborne,
                CaseStatus::Confirmed,
                18,
                VaccinationStatus::Unvaccinated,
                1_730_000_000,
            )],
            contact_count: 10,
            index_case_id: 5001,
        },
        OutbreakCluster {
            cluster_id: 302,
            location: "Westport".to_string(),
            cases: vec![
                make_case(
                    5002,
                    "Typhoid",
                    DiseaseCategory::Gastrointestinal,
                    CaseStatus::Recovered,
                    45,
                    VaccinationStatus::FullyVaccinated,
                    1_730_100_000,
                ),
                make_case(
                    5003,
                    "Typhoid",
                    DiseaseCategory::Gastrointestinal,
                    CaseStatus::Confirmed,
                    38,
                    VaccinationStatus::Unvaccinated,
                    1_730_150_000,
                ),
            ],
            contact_count: 22,
            index_case_id: 5002,
        },
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<OutbreakCluster>");
    let (decoded, _): (Vec<OutbreakCluster>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<OutbreakCluster>");
    assert_eq!(val, decoded);
}

// Test 9: big_endian config
#[test]
fn test_big_endian_config_epi_case_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = make_case(
        6001,
        "West Nile Virus",
        DiseaseCategory::Vectorborne,
        CaseStatus::Confirmed,
        62,
        VaccinationStatus::Unvaccinated,
        1_740_000_000,
    );
    let bytes = encode_to_vec(&val, cfg).expect("encode EpiCase big_endian");
    let (decoded, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EpiCase big_endian");
    assert_eq!(val, decoded);
}

// Test 10: fixed_int config
#[test]
fn test_fixed_int_config_epi_case_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = make_case(
        7001,
        "Ebola",
        DiseaseCategory::Bloodborne,
        CaseStatus::Deceased,
        50,
        VaccinationStatus::PartiallyVaccinated,
        1_745_000_000,
    );
    let bytes = encode_to_vec(&val, cfg).expect("encode EpiCase fixed_int");
    let (decoded, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EpiCase fixed_int");
    assert_eq!(val, decoded);
}

// Test 11: combined big_endian + fixed_int
#[test]
fn test_big_endian_fixed_int_combined_surveillance_report_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let cases = vec![
        make_case(
            8001,
            "SARS-CoV-2",
            DiseaseCategory::Respiratory,
            CaseStatus::Confirmed,
            70,
            VaccinationStatus::Boosted,
            1_750_000_000,
        ),
        make_case(
            8002,
            "SARS-CoV-2",
            DiseaseCategory::Respiratory,
            CaseStatus::Recovered,
            45,
            VaccinationStatus::FullyVaccinated,
            1_750_050_000,
        ),
    ];
    let cluster = OutbreakCluster {
        cluster_id: 400,
        location: "Metro General Hospital".to_string(),
        cases,
        contact_count: 85,
        index_case_id: 8001,
    };
    let val = SurveillanceReport {
        report_id: 5555,
        region: "Capital Region".to_string(),
        period_start: 1_750_000_000,
        period_end: 1_750_604_800,
        clusters: vec![cluster],
        total_cases: 2,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SurveillanceReport big_endian+fixed_int");
    let (decoded, _): (SurveillanceReport, usize) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode SurveillanceReport big_endian+fixed_int");
    assert_eq!(val, decoded);
}

// Test 12: consumed bytes check
#[test]
fn test_consumed_bytes_check_outbreak_cluster() {
    let cfg = config::standard();
    let cases: Vec<EpiCase> = (0..3)
        .map(|i| {
            make_case(
                9000 + i as u64,
                "Norovirus",
                DiseaseCategory::Gastrointestinal,
                CaseStatus::Confirmed,
                30 + i as u8,
                VaccinationStatus::Unvaccinated,
                1_760_000_000 + i as u64 * 7_200,
            )
        })
        .collect();
    let val = OutbreakCluster {
        cluster_id: 500,
        location: "Riverside Camp".to_string(),
        cases,
        contact_count: 31,
        index_case_id: 9000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode OutbreakCluster for bytes check");
    let (_decoded, consumed): (OutbreakCluster, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode OutbreakCluster for bytes check");
    assert_eq!(consumed, bytes.len());
}

// Test 13: respiratory outbreak
#[test]
fn test_respiratory_outbreak_cluster_roundtrip() {
    let cfg = config::standard();
    let cases: Vec<EpiCase> = vec![
        make_case(
            10001,
            "Influenza B",
            DiseaseCategory::Respiratory,
            CaseStatus::Confirmed,
            8,
            VaccinationStatus::PartiallyVaccinated,
            1_765_000_000,
        ),
        make_case(
            10002,
            "Influenza B",
            DiseaseCategory::Respiratory,
            CaseStatus::Suspected,
            6,
            VaccinationStatus::Unvaccinated,
            1_765_010_000,
        ),
        make_case(
            10003,
            "Influenza B",
            DiseaseCategory::Respiratory,
            CaseStatus::Recovered,
            35,
            VaccinationStatus::FullyVaccinated,
            1_765_020_000,
        ),
    ];
    let val = OutbreakCluster {
        cluster_id: 601,
        location: "Lakeview Elementary School".to_string(),
        cases,
        contact_count: 120,
        index_case_id: 10001,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode respiratory outbreak cluster");
    let (decoded, _): (OutbreakCluster, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode respiratory outbreak cluster");
    assert_eq!(val, decoded);
    assert!(decoded
        .cases
        .iter()
        .all(|c| c.category == DiseaseCategory::Respiratory));
}

// Test 14: vectorborne disease cluster
#[test]
fn test_vectorborne_disease_cluster_roundtrip() {
    let cfg = config::standard();
    let cases: Vec<EpiCase> = vec![
        make_case(
            11001,
            "Zika Virus",
            DiseaseCategory::Vectorborne,
            CaseStatus::Confirmed,
            27,
            VaccinationStatus::Unvaccinated,
            1_770_000_000,
        ),
        make_case(
            11002,
            "Zika Virus",
            DiseaseCategory::Vectorborne,
            CaseStatus::Probable,
            23,
            VaccinationStatus::Unvaccinated,
            1_770_010_000,
        ),
    ];
    let val = OutbreakCluster {
        cluster_id: 702,
        location: "Tropical Coast Zone 7".to_string(),
        cases,
        contact_count: 8,
        index_case_id: 11001,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode vectorborne disease cluster");
    let (decoded, _): (OutbreakCluster, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vectorborne disease cluster");
    assert_eq!(val, decoded);
    assert!(decoded
        .cases
        .iter()
        .all(|c| c.category == DiseaseCategory::Vectorborne));
}

// Test 15: fully vaccinated case
#[test]
fn test_fully_vaccinated_case_roundtrip() {
    let cfg = config::standard();
    let val = make_case(
        12001,
        "COVID-19",
        DiseaseCategory::Respiratory,
        CaseStatus::Confirmed,
        55,
        VaccinationStatus::FullyVaccinated,
        1_775_000_000,
    );
    let bytes = encode_to_vec(&val, cfg).expect("encode fully vaccinated case");
    let (decoded, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode fully vaccinated case");
    assert_eq!(val, decoded);
    assert_eq!(decoded.vaccination, VaccinationStatus::FullyVaccinated);
}

// Test 16: zero contact cluster
#[test]
fn test_zero_contact_cluster_roundtrip() {
    let cfg = config::standard();
    let val = OutbreakCluster {
        cluster_id: 800,
        location: "Isolated Settlement".to_string(),
        cases: vec![make_case(
            13001,
            "Leptospirosis",
            DiseaseCategory::Zoonotic,
            CaseStatus::Suspected,
            42,
            VaccinationStatus::Unvaccinated,
            1_780_000_000,
        )],
        contact_count: 0,
        index_case_id: 13001,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode zero contact cluster");
    let (decoded, _): (OutbreakCluster, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode zero contact cluster");
    assert_eq!(val, decoded);
    assert_eq!(decoded.contact_count, 0);
}

// Test 17: large report (10 clusters)
#[test]
fn test_large_surveillance_report_ten_clusters_roundtrip() {
    let cfg = config::standard();
    let clusters: Vec<OutbreakCluster> = (0..10)
        .map(|ci| {
            let cases: Vec<EpiCase> = (0..3)
                .map(|i| {
                    make_case(
                        20000 + ci as u64 * 100 + i as u64,
                        "Cholera",
                        DiseaseCategory::Gastrointestinal,
                        CaseStatus::Confirmed,
                        15 + i as u8,
                        VaccinationStatus::Unvaccinated,
                        1_790_000_000 + ci as u64 * 86_400 + i as u64 * 3_600,
                    )
                })
                .collect();
            OutbreakCluster {
                cluster_id: 900 + ci as u64,
                location: format!("District Zone {}", ci + 1),
                cases,
                contact_count: (ci as u32 + 1) * 20,
                index_case_id: 20000 + ci as u64 * 100,
            }
        })
        .collect();
    let val = SurveillanceReport {
        report_id: 77777,
        region: "Southern Territory".to_string(),
        period_start: 1_790_000_000,
        period_end: 1_790_864_000,
        clusters,
        total_cases: 30,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode large SurveillanceReport 10 clusters");
    let (decoded, consumed): (SurveillanceReport, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large SurveillanceReport 10 clusters");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.clusters.len(), 10);
}

// Test 18: deceased case roundtrip
#[test]
fn test_deceased_case_roundtrip() {
    let cfg = config::standard();
    let val = make_case(
        30001,
        "Hemorrhagic Fever",
        DiseaseCategory::Bloodborne,
        CaseStatus::Deceased,
        67,
        VaccinationStatus::Unvaccinated,
        1_800_000_000,
    );
    let bytes = encode_to_vec(&val, cfg).expect("encode deceased case");
    let (decoded, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode deceased case");
    assert_eq!(val, decoded);
    assert_eq!(decoded.status, CaseStatus::Deceased);
}

// Test 19: suspected vs confirmed produce distinct bytes
#[test]
fn test_suspected_vs_confirmed_distinct_bytes() {
    let cfg = config::standard();
    let suspected = make_case(
        40001,
        "Unknown Pathogen",
        DiseaseCategory::Respiratory,
        CaseStatus::Suspected,
        33,
        VaccinationStatus::Unvaccinated,
        1_810_000_000,
    );
    let confirmed = EpiCase {
        status: CaseStatus::Confirmed,
        ..suspected.clone()
    };
    let bytes_suspected = encode_to_vec(&suspected, cfg).expect("encode suspected case");
    let bytes_confirmed = encode_to_vec(&confirmed, cfg).expect("encode confirmed case");
    assert_ne!(bytes_suspected, bytes_confirmed);
    let (decoded_suspected, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes_suspected, cfg).expect("decode suspected case");
    let (decoded_confirmed, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes_confirmed, cfg).expect("decode confirmed case");
    assert_eq!(decoded_suspected.status, CaseStatus::Suspected);
    assert_eq!(decoded_confirmed.status, CaseStatus::Confirmed);
}

// Test 20: boosted vs unvaccinated produce distinct bytes
#[test]
fn test_boosted_vs_unvaccinated_distinct_bytes() {
    let cfg = config::standard();
    let boosted = make_case(
        50001,
        "Influenza C",
        DiseaseCategory::Respiratory,
        CaseStatus::Confirmed,
        44,
        VaccinationStatus::Boosted,
        1_820_000_000,
    );
    let unvaccinated = EpiCase {
        vaccination: VaccinationStatus::Unvaccinated,
        ..boosted.clone()
    };
    let bytes_boosted = encode_to_vec(&boosted, cfg).expect("encode boosted case");
    let bytes_unvaccinated = encode_to_vec(&unvaccinated, cfg).expect("encode unvaccinated case");
    assert_ne!(bytes_boosted, bytes_unvaccinated);
    let (decoded_boosted, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes_boosted, cfg).expect("decode boosted case");
    let (decoded_unvaccinated, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes_unvaccinated, cfg).expect("decode unvaccinated case");
    assert_eq!(decoded_boosted.vaccination, VaccinationStatus::Boosted);
    assert_eq!(
        decoded_unvaccinated.vaccination,
        VaccinationStatus::Unvaccinated
    );
}

// Test 21: report spanning multiple periods
#[test]
fn test_report_spanning_multiple_periods_roundtrip() {
    let cfg = config::standard();
    let period_start: u64 = 1_700_000_000;
    let period_end: u64 = period_start + 7_776_000; // ~90 days
    let clusters: Vec<OutbreakCluster> = vec![
        OutbreakCluster {
            cluster_id: 1001,
            location: "Month 1 Zone".to_string(),
            cases: vec![make_case(
                60001,
                "Pertussis",
                DiseaseCategory::Respiratory,
                CaseStatus::Confirmed,
                1,
                VaccinationStatus::PartiallyVaccinated,
                period_start + 86_400,
            )],
            contact_count: 5,
            index_case_id: 60001,
        },
        OutbreakCluster {
            cluster_id: 1002,
            location: "Month 2 Zone".to_string(),
            cases: vec![make_case(
                60002,
                "Pertussis",
                DiseaseCategory::Respiratory,
                CaseStatus::Confirmed,
                3,
                VaccinationStatus::Unvaccinated,
                period_start + 2_592_000,
            )],
            contact_count: 12,
            index_case_id: 60002,
        },
        OutbreakCluster {
            cluster_id: 1003,
            location: "Month 3 Zone".to_string(),
            cases: vec![make_case(
                60003,
                "Pertussis",
                DiseaseCategory::Respiratory,
                CaseStatus::Recovered,
                5,
                VaccinationStatus::FullyVaccinated,
                period_start + 5_184_000,
            )],
            contact_count: 7,
            index_case_id: 60003,
        },
    ];
    let val = SurveillanceReport {
        report_id: 88888,
        region: "Multi-Period Watch Region".to_string(),
        period_start,
        period_end,
        clusters,
        total_cases: 3,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode multi-period report");
    let (decoded, consumed): (SurveillanceReport, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode multi-period report");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.period_end - decoded.period_start, 7_776_000);
}

// Test 22: age boundary (0 and 120)
#[test]
fn test_age_boundary_newborn_and_oldest_roundtrip() {
    let cfg = config::standard();
    let newborn = make_case(
        70001,
        "Group B Strep",
        DiseaseCategory::Bloodborne,
        CaseStatus::Confirmed,
        0,
        VaccinationStatus::Unvaccinated,
        1_830_000_000,
    );
    let oldest = make_case(
        70002,
        "Pneumonia",
        DiseaseCategory::Respiratory,
        CaseStatus::Recovered,
        120,
        VaccinationStatus::Boosted,
        1_830_010_000,
    );
    let bytes_newborn = encode_to_vec(&newborn, cfg).expect("encode newborn case");
    let (decoded_newborn, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes_newborn, cfg).expect("decode newborn case");
    assert_eq!(newborn, decoded_newborn);
    assert_eq!(decoded_newborn.age, 0);

    let bytes_oldest = encode_to_vec(&oldest, cfg).expect("encode oldest case");
    let (decoded_oldest, _): (EpiCase, usize) =
        decode_owned_from_slice(&bytes_oldest, cfg).expect("decode oldest case");
    assert_eq!(oldest, decoded_oldest);
    assert_eq!(decoded_oldest.age, 120);
}
