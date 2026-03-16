//! Advanced property-based tests (set 89) — Organ Transplantation Matching
//! and Allocation Systems domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers donor organ records, recipient waitlist entries, crossmatch results,
//! cold ischemia time tracking, allocation region zones, post-transplant
//! immunosuppression protocols, rejection episode classifications, graft
//! survival metrics, living donor evaluations, and more.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// ── Domain types ──────────────────────────────────────────────────────────────

/// Classification of transplantable organs.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrganType {
    Kidney,
    Liver,
    Heart,
    Lung,
    Pancreas,
    Intestine,
    KidneyPancreas,
}

/// ABO blood type classification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BloodType {
    APositive,
    ANegative,
    BPositive,
    BNegative,
    ABPositive,
    ABNegative,
    OPositive,
    ONegative,
}

/// Human Leukocyte Antigen marker set for tissue matching.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HlaMarkerSet {
    /// HLA-A allele pair.
    hla_a1: u16,
    hla_a2: u16,
    /// HLA-B allele pair.
    hla_b1: u16,
    hla_b2: u16,
    /// HLA-DR allele pair.
    hla_dr1: u16,
    hla_dr2: u16,
}

/// A deceased donor organ record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DonorOrganRecord {
    /// Unique donor identifier.
    donor_id: u64,
    /// Type of organ procured.
    organ: OrganType,
    /// Donor blood type.
    blood_type: BloodType,
    /// HLA tissue typing markers.
    hla_markers: HlaMarkerSet,
    /// Donor age in years.
    donor_age: u8,
    /// Donor weight in kilograms (tenths precision stored as integer).
    weight_dg: u16,
    /// Whether the donor had extended criteria designation.
    extended_criteria: bool,
}

/// Recipient waitlist entry with MELD/PELD scoring.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RecipientWaitlistEntry {
    /// Unique recipient identifier.
    recipient_id: u64,
    /// Organ needed.
    organ_needed: OrganType,
    /// Blood type.
    blood_type: BloodType,
    /// HLA markers.
    hla_markers: HlaMarkerSet,
    /// MELD score (6–40) for liver; repurposed for general severity.
    meld_score: u8,
    /// PELD score for pediatric patients (0 if adult).
    peld_score: u8,
    /// Days on waiting list.
    days_waiting: u32,
    /// Whether patient is currently Status 1A (highest urgency).
    status_1a: bool,
}

/// Result of a crossmatch compatibility test.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CrossmatchResult {
    /// Negative crossmatch — compatible.
    Negative {
        /// Number of HLA mismatches (0–6).
        hla_mismatches: u8,
        /// Panel reactive antibody percentage (0–100).
        pra_percent: u8,
    },
    /// Positive crossmatch — incompatible.
    Positive {
        /// Offending antibody specificity code.
        antibody_code: u16,
        /// Strength of reaction (mean fluorescence intensity).
        mfi_value: u32,
    },
    /// Virtual crossmatch (computational prediction).
    Virtual {
        /// Predicted compatibility probability (0.0–1.0).
        compatibility_prob: f32,
        /// Number of unacceptable antigens detected.
        unacceptable_count: u8,
    },
}

/// Cold ischemia time record for an organ in transit.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ColdIschemiaRecord {
    /// Organ procurement timestamp (Unix seconds).
    procurement_ts: u64,
    /// Organ implantation timestamp (Unix seconds), 0 if not yet implanted.
    implantation_ts: u64,
    /// Maximum allowable cold ischemia time in seconds.
    max_ischemia_s: u32,
    /// Current storage temperature in tenths of degrees Celsius.
    storage_temp_dc: i16,
    /// Preservation solution code.
    preservation_code: u8,
}

/// Geographic allocation region zone.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AllocationZone {
    /// Zone identifier.
    zone_id: u32,
    /// Zone name.
    zone_name: String,
    /// Radius in kilometres from procurement centre.
    radius_km: u16,
    /// Priority tier (1 = local, 2 = regional, 3 = national).
    priority_tier: u8,
    /// Number of transplant centres in zone.
    centre_count: u16,
}

/// Post-transplant immunosuppression protocol.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImmunosuppressionProtocol {
    /// Protocol identifier.
    protocol_id: u32,
    /// Induction agent code.
    induction_agent: u8,
    /// Calcineurin inhibitor trough level target (ng/mL, stored as tenths).
    cni_target_dng: u16,
    /// Mycophenolate daily dose in mg.
    mmf_dose_mg: u16,
    /// Steroid dose in mg (tenths).
    steroid_dose_dmg: u16,
    /// Whether mTOR inhibitor is included.
    mtor_inhibitor: bool,
}

/// Classification of transplant rejection episodes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RejectionEpisode {
    /// Hyperacute rejection — within minutes.
    Hyperacute {
        /// Minutes post-transplant when detected.
        onset_min: u16,
    },
    /// Acute cellular rejection.
    AcuteCellular {
        /// Banff grade (1A, 1B, 2A, 2B, 3 encoded as 1–5).
        banff_grade: u8,
        /// Days post-transplant.
        post_tx_days: u32,
        /// Whether treated with pulse steroids.
        pulse_steroids: bool,
    },
    /// Acute antibody-mediated rejection.
    AcuteAntibodyMediated {
        /// C4d staining positive.
        c4d_positive: bool,
        /// Donor-specific antibody MFI.
        dsa_mfi: u32,
        /// Days post-transplant.
        post_tx_days: u32,
    },
    /// Chronic rejection / transplant glomerulopathy.
    Chronic {
        /// Months post-transplant.
        post_tx_months: u32,
        /// Interstitial fibrosis score (0–3).
        fibrosis_score: u8,
        /// Tubular atrophy score (0–3).
        atrophy_score: u8,
    },
}

/// Graft survival metrics over time.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GraftSurvivalMetric {
    /// Graft identifier.
    graft_id: u64,
    /// Months since transplant.
    months_post_tx: u32,
    /// Serum creatinine in micromol/L.
    creatinine_umol: u16,
    /// Estimated GFR in mL/min/1.73m².
    egfr_ml_min: u16,
    /// Proteinuria in mg/day.
    proteinuria_mg_day: u32,
    /// Graft still functioning.
    functioning: bool,
}

/// Living donor evaluation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LivingDonorEvaluation {
    /// Candidate donor identifier.
    candidate_id: u64,
    /// Blood type.
    blood_type: BloodType,
    /// HLA markers.
    hla_markers: HlaMarkerSet,
    /// Measured GFR in mL/min.
    measured_gfr: u16,
    /// Body mass index (stored as tenths, e.g. 255 = 25.5).
    bmi_tenths: u16,
    /// Number of comorbidities flagged.
    comorbidity_count: u8,
    /// Psychosocial evaluation passed.
    psych_cleared: bool,
    /// Anatomical suitability score (0–100).
    anatomy_score: u8,
}

/// Organ transport logistics record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrganTransportLog {
    /// Transport case identifier.
    case_id: u64,
    /// Distance in kilometres.
    distance_km: u32,
    /// Estimated transit time in minutes.
    transit_min: u16,
    /// Transport mode code (1=ground, 2=helicopter, 3=fixed-wing).
    transport_mode: u8,
    /// GPS waypoint count recorded during transit.
    waypoint_count: u16,
    /// Whether organ arrived within acceptable ischemia window.
    within_window: bool,
}

/// Paired kidney exchange chain record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PairedExchangeChain {
    /// Chain identifier.
    chain_id: u64,
    /// Number of donor-recipient pairs in the chain.
    pair_count: u8,
    /// Whether chain started with a non-directed (altruistic) donor.
    altruistic_start: bool,
    /// Number of transplants successfully completed.
    completed_tx: u8,
    /// Number of bridge donors awaiting next cycle.
    bridge_donors: u8,
    /// Maximum chain length considered in optimization.
    max_chain_len: u8,
}

/// Organ perfusion machine parameters during ex-vivo preservation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PerfusionParameters {
    /// Machine identifier.
    machine_id: u32,
    /// Flow rate in mL/min.
    flow_rate_ml_min: u16,
    /// Resistance index (mmHg/mL/min, stored as hundredths).
    resistance_idx: u16,
    /// Perfusate temperature in tenths of degrees Celsius.
    temp_dc: i16,
    /// Lactate level in mmol/L (stored as hundredths).
    lactate_hmol: u16,
    /// Pump pressure in mmHg.
    pressure_mmhg: u16,
}

/// Histocompatibility laboratory report.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HistocompatibilityReport {
    /// Lab accession number.
    accession_id: u64,
    /// Panel reactive antibody percentage.
    pra_percent: u8,
    /// Calculated PRA using single antigen bead testing.
    cpra_percent: u8,
    /// Number of unacceptable antigens identified.
    unacceptable_ag_count: u16,
    /// Number of donor-specific antibodies detected.
    dsa_count: u8,
    /// Highest MFI among detected DSAs.
    peak_dsa_mfi: u32,
}

/// Post-transplant infection surveillance event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InfectionEvent {
    /// CMV viremia detected.
    CmvViremia {
        /// Viral load copies/mL.
        viral_load: u32,
        /// Days post-transplant.
        post_tx_days: u32,
    },
    /// BK polyomavirus nephropathy.
    BkVirus {
        /// Plasma viral load copies/mL.
        viral_load: u32,
        /// Whether biopsy confirmed nephropathy.
        biopsy_confirmed: bool,
    },
    /// Opportunistic fungal infection.
    FungalInfection {
        /// Organism code.
        organism_code: u8,
        /// Site code.
        site_code: u8,
        /// Severity (1–4).
        severity: u8,
    },
    /// Urinary tract infection.
    UrinaryTractInfection {
        /// Colony count category (1=low, 2=moderate, 3=high).
        colony_category: u8,
        /// Recurrent episode flag.
        recurrent: bool,
    },
}

/// Organ allocation scoring composite.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AllocationScore {
    /// Recipient identifier.
    recipient_id: u64,
    /// HLA mismatch points.
    hla_points: u16,
    /// Waiting time points.
    wait_time_points: u32,
    /// Distance/logistics points.
    distance_points: u16,
    /// Medical urgency points.
    urgency_points: u16,
    /// Pediatric priority bonus.
    pediatric_bonus: u16,
    /// Prior living donor bonus.
    prior_donor_bonus: u16,
    /// Final composite score.
    composite_score: u32,
}

/// Desensitization treatment protocol for highly sensitized recipients.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DesensitizationProtocol {
    /// Protocol identifier.
    protocol_id: u32,
    /// Number of plasmapheresis sessions.
    plasmapheresis_sessions: u8,
    /// IVIg dose in g/kg (stored as hundredths).
    ivig_dose_hgkg: u16,
    /// Rituximab administered.
    rituximab: bool,
    /// Bortezomib cycles.
    bortezomib_cycles: u8,
    /// Target PRA reduction percentage.
    target_pra_reduction: u8,
    /// Days of treatment.
    treatment_days: u16,
}

// ── Proptest strategies ───────────────────────────────────────────────────────

fn arb_organ_type() -> impl Strategy<Value = OrganType> {
    prop_oneof![
        Just(OrganType::Kidney),
        Just(OrganType::Liver),
        Just(OrganType::Heart),
        Just(OrganType::Lung),
        Just(OrganType::Pancreas),
        Just(OrganType::Intestine),
        Just(OrganType::KidneyPancreas),
    ]
}

fn arb_blood_type() -> impl Strategy<Value = BloodType> {
    prop_oneof![
        Just(BloodType::APositive),
        Just(BloodType::ANegative),
        Just(BloodType::BPositive),
        Just(BloodType::BNegative),
        Just(BloodType::ABPositive),
        Just(BloodType::ABNegative),
        Just(BloodType::OPositive),
        Just(BloodType::ONegative),
    ]
}

prop_compose! {
    fn arb_hla_markers()(
        hla_a1 in 1u16..100,
        hla_a2 in 1u16..100,
        hla_b1 in 1u16..200,
        hla_b2 in 1u16..200,
        hla_dr1 in 1u16..50,
        hla_dr2 in 1u16..50,
    ) -> HlaMarkerSet {
        HlaMarkerSet { hla_a1, hla_a2, hla_b1, hla_b2, hla_dr1, hla_dr2 }
    }
}

prop_compose! {
    fn arb_donor_organ_record()(
        donor_id: u64,
        organ in arb_organ_type(),
        blood_type in arb_blood_type(),
        hla_markers in arb_hla_markers(),
        donor_age in 1u8..80,
        weight_dg in 300u16..2000,
        extended_criteria: bool,
    ) -> DonorOrganRecord {
        DonorOrganRecord {
            donor_id, organ, blood_type, hla_markers,
            donor_age, weight_dg, extended_criteria,
        }
    }
}

prop_compose! {
    fn arb_recipient_waitlist()(
        recipient_id: u64,
        organ_needed in arb_organ_type(),
        blood_type in arb_blood_type(),
        hla_markers in arb_hla_markers(),
        meld_score in 6u8..41,
        peld_score in 0u8..50,
        days_waiting in 0u32..7300,
        status_1a: bool,
    ) -> RecipientWaitlistEntry {
        RecipientWaitlistEntry {
            recipient_id, organ_needed, blood_type, hla_markers,
            meld_score, peld_score, days_waiting, status_1a,
        }
    }
}

fn arb_crossmatch_result() -> impl Strategy<Value = CrossmatchResult> {
    prop_oneof![
        (0u8..7, 0u8..101).prop_map(|(m, p)| CrossmatchResult::Negative {
            hla_mismatches: m,
            pra_percent: p,
        }),
        (1u16..5000, 500u32..25000).prop_map(|(c, m)| CrossmatchResult::Positive {
            antibody_code: c,
            mfi_value: m,
        }),
        (0.0f32..1.0, 0u8..20).prop_map(|(p, u)| CrossmatchResult::Virtual {
            compatibility_prob: p,
            unacceptable_count: u,
        }),
    ]
}

prop_compose! {
    fn arb_cold_ischemia()(
        procurement_ts in 1_600_000_000u64..1_700_000_000,
        offset in 0u64..86400,
        max_ischemia_s in 3600u32..172800,
        storage_temp_dc in (-50i16)..100,
        preservation_code in 1u8..6,
    ) -> ColdIschemiaRecord {
        ColdIschemiaRecord {
            procurement_ts,
            implantation_ts: procurement_ts + offset,
            max_ischemia_s,
            storage_temp_dc,
            preservation_code,
        }
    }
}

prop_compose! {
    fn arb_allocation_zone()(
        zone_id: u32,
        zone_name in "[A-Z]{2}-[0-9]{3}",
        radius_km in 1u16..3000,
        priority_tier in 1u8..4,
        centre_count in 1u16..200,
    ) -> AllocationZone {
        AllocationZone { zone_id, zone_name, radius_km, priority_tier, centre_count }
    }
}

prop_compose! {
    fn arb_immunosuppression()(
        protocol_id: u32,
        induction_agent in 1u8..5,
        cni_target_dng in 30u16..200,
        mmf_dose_mg in 250u16..3000,
        steroid_dose_dmg in 0u16..500,
        mtor_inhibitor: bool,
    ) -> ImmunosuppressionProtocol {
        ImmunosuppressionProtocol {
            protocol_id, induction_agent, cni_target_dng,
            mmf_dose_mg, steroid_dose_dmg, mtor_inhibitor,
        }
    }
}

fn arb_rejection_episode() -> impl Strategy<Value = RejectionEpisode> {
    prop_oneof![
        (1u16..60).prop_map(|m| RejectionEpisode::Hyperacute { onset_min: m }),
        (1u8..6, 1u32..3650, any::<bool>()).prop_map(|(g, d, s)| {
            RejectionEpisode::AcuteCellular {
                banff_grade: g,
                post_tx_days: d,
                pulse_steroids: s,
            }
        }),
        (any::<bool>(), 500u32..25000, 1u32..3650).prop_map(|(c, m, d)| {
            RejectionEpisode::AcuteAntibodyMediated {
                c4d_positive: c,
                dsa_mfi: m,
                post_tx_days: d,
            }
        }),
        (6u32..360, 0u8..4, 0u8..4).prop_map(|(m, f, a)| {
            RejectionEpisode::Chronic {
                post_tx_months: m,
                fibrosis_score: f,
                atrophy_score: a,
            }
        }),
    ]
}

prop_compose! {
    fn arb_graft_survival()(
        graft_id: u64,
        months_post_tx in 1u32..360,
        creatinine_umol in 50u16..1500,
        egfr_ml_min in 5u16..120,
        proteinuria_mg_day in 0u32..15000,
        functioning: bool,
    ) -> GraftSurvivalMetric {
        GraftSurvivalMetric {
            graft_id, months_post_tx, creatinine_umol,
            egfr_ml_min, proteinuria_mg_day, functioning,
        }
    }
}

prop_compose! {
    fn arb_living_donor_eval()(
        candidate_id: u64,
        blood_type in arb_blood_type(),
        hla_markers in arb_hla_markers(),
        measured_gfr in 60u16..150,
        bmi_tenths in 180u16..450,
        comorbidity_count in 0u8..6,
        psych_cleared: bool,
        anatomy_score in 0u8..101,
    ) -> LivingDonorEvaluation {
        LivingDonorEvaluation {
            candidate_id, blood_type, hla_markers, measured_gfr,
            bmi_tenths, comorbidity_count, psych_cleared, anatomy_score,
        }
    }
}

prop_compose! {
    fn arb_transport_log()(
        case_id: u64,
        distance_km in 1u32..5000,
        transit_min in 10u16..1440,
        transport_mode in 1u8..4,
        waypoint_count in 0u16..500,
        within_window: bool,
    ) -> OrganTransportLog {
        OrganTransportLog {
            case_id, distance_km, transit_min,
            transport_mode, waypoint_count, within_window,
        }
    }
}

prop_compose! {
    fn arb_paired_exchange()(
        chain_id: u64,
        pair_count in 2u8..12,
        altruistic_start: bool,
        completed_tx in 0u8..12,
        bridge_donors in 0u8..4,
        max_chain_len in 2u8..20,
    ) -> PairedExchangeChain {
        PairedExchangeChain {
            chain_id, pair_count, altruistic_start,
            completed_tx, bridge_donors, max_chain_len,
        }
    }
}

prop_compose! {
    fn arb_perfusion_params()(
        machine_id: u32,
        flow_rate_ml_min in 50u16..500,
        resistance_idx in 10u16..100,
        temp_dc in (-20i16)..100,
        lactate_hmol in 10u16..5000,
        pressure_mmhg in 10u16..100,
    ) -> PerfusionParameters {
        PerfusionParameters {
            machine_id, flow_rate_ml_min, resistance_idx,
            temp_dc, lactate_hmol, pressure_mmhg,
        }
    }
}

prop_compose! {
    fn arb_histocompat_report()(
        accession_id: u64,
        pra_percent in 0u8..101,
        cpra_percent in 0u8..101,
        unacceptable_ag_count in 0u16..100,
        dsa_count in 0u8..15,
        peak_dsa_mfi in 0u32..30000,
    ) -> HistocompatibilityReport {
        HistocompatibilityReport {
            accession_id, pra_percent, cpra_percent,
            unacceptable_ag_count, dsa_count, peak_dsa_mfi,
        }
    }
}

fn arb_infection_event() -> impl Strategy<Value = InfectionEvent> {
    prop_oneof![
        (100u32..1_000_000, 1u32..365).prop_map(|(v, d)| InfectionEvent::CmvViremia {
            viral_load: v,
            post_tx_days: d,
        }),
        (100u32..1_000_000, any::<bool>()).prop_map(|(v, b)| InfectionEvent::BkVirus {
            viral_load: v,
            biopsy_confirmed: b,
        }),
        (1u8..10, 1u8..20, 1u8..5).prop_map(|(o, s, sev)| InfectionEvent::FungalInfection {
            organism_code: o,
            site_code: s,
            severity: sev,
        }),
        (1u8..4, any::<bool>()).prop_map(|(c, r)| InfectionEvent::UrinaryTractInfection {
            colony_category: c,
            recurrent: r,
        }),
    ]
}

prop_compose! {
    fn arb_allocation_score()(
        recipient_id: u64,
        hla_points in 0u16..1000,
        wait_time_points in 0u32..50000,
        distance_points in 0u16..500,
        urgency_points in 0u16..2000,
        pediatric_bonus in 0u16..500,
        prior_donor_bonus in 0u16..200,
    ) -> AllocationScore {
        let composite_score = hla_points as u32
            + wait_time_points
            + distance_points as u32
            + urgency_points as u32
            + pediatric_bonus as u32
            + prior_donor_bonus as u32;
        AllocationScore {
            recipient_id, hla_points, wait_time_points,
            distance_points, urgency_points, pediatric_bonus,
            prior_donor_bonus, composite_score,
        }
    }
}

prop_compose! {
    fn arb_desensitization()(
        protocol_id: u32,
        plasmapheresis_sessions in 1u8..20,
        ivig_dose_hgkg in 50u16..400,
        rituximab: bool,
        bortezomib_cycles in 0u8..6,
        target_pra_reduction in 10u8..100,
        treatment_days in 7u16..90,
    ) -> DesensitizationProtocol {
        DesensitizationProtocol {
            protocol_id, plasmapheresis_sessions, ivig_dose_hgkg,
            rituximab, bortezomib_cycles, target_pra_reduction,
            treatment_days,
        }
    }
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. DonorOrganRecord roundtrip ─────────────────────────────────────────────

#[test]
fn test_donor_organ_record_roundtrip() {
    proptest!(|(val in arb_donor_organ_record())| {
        let enc = encode_to_vec(&val).expect("encode DonorOrganRecord failed");
        let (dec, consumed): (DonorOrganRecord, usize) =
            decode_from_slice(&enc).expect("decode DonorOrganRecord failed");
        prop_assert_eq!(&val, &dec, "DonorOrganRecord roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 2. DonorOrganRecord deterministic encoding ────────────────────────────────

#[test]
fn test_donor_organ_record_determinism() {
    proptest!(|(val in arb_donor_organ_record())| {
        let enc1 = encode_to_vec(&val).expect("first encode DonorOrganRecord failed");
        let enc2 = encode_to_vec(&val).expect("second encode DonorOrganRecord failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 3. RecipientWaitlistEntry roundtrip ───────────────────────────────────────

#[test]
fn test_recipient_waitlist_roundtrip() {
    proptest!(|(val in arb_recipient_waitlist())| {
        let enc = encode_to_vec(&val).expect("encode RecipientWaitlistEntry failed");
        let (dec, consumed): (RecipientWaitlistEntry, usize) =
            decode_from_slice(&enc).expect("decode RecipientWaitlistEntry failed");
        prop_assert_eq!(&val, &dec, "RecipientWaitlistEntry roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 4. CrossmatchResult roundtrip ─────────────────────────────────────────────

#[test]
fn test_crossmatch_result_roundtrip() {
    proptest!(|(val in arb_crossmatch_result())| {
        let enc = encode_to_vec(&val).expect("encode CrossmatchResult failed");
        let (dec, consumed): (CrossmatchResult, usize) =
            decode_from_slice(&enc).expect("decode CrossmatchResult failed");
        prop_assert_eq!(&val, &dec, "CrossmatchResult roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 5. ColdIschemiaRecord roundtrip ───────────────────────────────────────────

#[test]
fn test_cold_ischemia_roundtrip() {
    proptest!(|(val in arb_cold_ischemia())| {
        let enc = encode_to_vec(&val).expect("encode ColdIschemiaRecord failed");
        let (dec, consumed): (ColdIschemiaRecord, usize) =
            decode_from_slice(&enc).expect("decode ColdIschemiaRecord failed");
        prop_assert_eq!(&val, &dec, "ColdIschemiaRecord roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 6. AllocationZone roundtrip ───────────────────────────────────────────────

#[test]
fn test_allocation_zone_roundtrip() {
    proptest!(|(val in arb_allocation_zone())| {
        let enc = encode_to_vec(&val).expect("encode AllocationZone failed");
        let (dec, consumed): (AllocationZone, usize) =
            decode_from_slice(&enc).expect("decode AllocationZone failed");
        prop_assert_eq!(&val, &dec, "AllocationZone roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 7. ImmunosuppressionProtocol roundtrip ────────────────────────────────────

#[test]
fn test_immunosuppression_roundtrip() {
    proptest!(|(val in arb_immunosuppression())| {
        let enc = encode_to_vec(&val).expect("encode ImmunosuppressionProtocol failed");
        let (dec, consumed): (ImmunosuppressionProtocol, usize) =
            decode_from_slice(&enc).expect("decode ImmunosuppressionProtocol failed");
        prop_assert_eq!(&val, &dec, "ImmunosuppressionProtocol roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 8. RejectionEpisode roundtrip ─────────────────────────────────────────────

#[test]
fn test_rejection_episode_roundtrip() {
    proptest!(|(val in arb_rejection_episode())| {
        let enc = encode_to_vec(&val).expect("encode RejectionEpisode failed");
        let (dec, consumed): (RejectionEpisode, usize) =
            decode_from_slice(&enc).expect("decode RejectionEpisode failed");
        prop_assert_eq!(&val, &dec, "RejectionEpisode roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 9. GraftSurvivalMetric roundtrip ──────────────────────────────────────────

#[test]
fn test_graft_survival_roundtrip() {
    proptest!(|(val in arb_graft_survival())| {
        let enc = encode_to_vec(&val).expect("encode GraftSurvivalMetric failed");
        let (dec, consumed): (GraftSurvivalMetric, usize) =
            decode_from_slice(&enc).expect("decode GraftSurvivalMetric failed");
        prop_assert_eq!(&val, &dec, "GraftSurvivalMetric roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 10. LivingDonorEvaluation roundtrip ───────────────────────────────────────

#[test]
fn test_living_donor_eval_roundtrip() {
    proptest!(|(val in arb_living_donor_eval())| {
        let enc = encode_to_vec(&val).expect("encode LivingDonorEvaluation failed");
        let (dec, consumed): (LivingDonorEvaluation, usize) =
            decode_from_slice(&enc).expect("decode LivingDonorEvaluation failed");
        prop_assert_eq!(&val, &dec, "LivingDonorEvaluation roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 11. OrganTransportLog roundtrip ───────────────────────────────────────────

#[test]
fn test_transport_log_roundtrip() {
    proptest!(|(val in arb_transport_log())| {
        let enc = encode_to_vec(&val).expect("encode OrganTransportLog failed");
        let (dec, consumed): (OrganTransportLog, usize) =
            decode_from_slice(&enc).expect("decode OrganTransportLog failed");
        prop_assert_eq!(&val, &dec, "OrganTransportLog roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 12. PairedExchangeChain roundtrip ─────────────────────────────────────────

#[test]
fn test_paired_exchange_roundtrip() {
    proptest!(|(val in arb_paired_exchange())| {
        let enc = encode_to_vec(&val).expect("encode PairedExchangeChain failed");
        let (dec, consumed): (PairedExchangeChain, usize) =
            decode_from_slice(&enc).expect("decode PairedExchangeChain failed");
        prop_assert_eq!(&val, &dec, "PairedExchangeChain roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 13. PerfusionParameters roundtrip ─────────────────────────────────────────

#[test]
fn test_perfusion_params_roundtrip() {
    proptest!(|(val in arb_perfusion_params())| {
        let enc = encode_to_vec(&val).expect("encode PerfusionParameters failed");
        let (dec, consumed): (PerfusionParameters, usize) =
            decode_from_slice(&enc).expect("decode PerfusionParameters failed");
        prop_assert_eq!(&val, &dec, "PerfusionParameters roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 14. HistocompatibilityReport roundtrip ────────────────────────────────────

#[test]
fn test_histocompat_report_roundtrip() {
    proptest!(|(val in arb_histocompat_report())| {
        let enc = encode_to_vec(&val).expect("encode HistocompatibilityReport failed");
        let (dec, consumed): (HistocompatibilityReport, usize) =
            decode_from_slice(&enc).expect("decode HistocompatibilityReport failed");
        prop_assert_eq!(&val, &dec, "HistocompatibilityReport roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 15. InfectionEvent roundtrip ──────────────────────────────────────────────

#[test]
fn test_infection_event_roundtrip() {
    proptest!(|(val in arb_infection_event())| {
        let enc = encode_to_vec(&val).expect("encode InfectionEvent failed");
        let (dec, consumed): (InfectionEvent, usize) =
            decode_from_slice(&enc).expect("decode InfectionEvent failed");
        prop_assert_eq!(&val, &dec, "InfectionEvent roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 16. AllocationScore roundtrip ─────────────────────────────────────────────

#[test]
fn test_allocation_score_roundtrip() {
    proptest!(|(val in arb_allocation_score())| {
        let enc = encode_to_vec(&val).expect("encode AllocationScore failed");
        let (dec, consumed): (AllocationScore, usize) =
            decode_from_slice(&enc).expect("decode AllocationScore failed");
        prop_assert_eq!(&val, &dec, "AllocationScore roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 17. DesensitizationProtocol roundtrip ─────────────────────────────────────

#[test]
fn test_desensitization_roundtrip() {
    proptest!(|(val in arb_desensitization())| {
        let enc = encode_to_vec(&val).expect("encode DesensitizationProtocol failed");
        let (dec, consumed): (DesensitizationProtocol, usize) =
            decode_from_slice(&enc).expect("decode DesensitizationProtocol failed");
        prop_assert_eq!(&val, &dec, "DesensitizationProtocol roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 18. Vec<DonorOrganRecord> batch roundtrip ─────────────────────────────────

#[test]
fn test_donor_record_batch_roundtrip() {
    proptest!(|(vals in prop::collection::vec(arb_donor_organ_record(), 0..8))| {
        let enc = encode_to_vec(&vals).expect("encode Vec<DonorOrganRecord> failed");
        let (dec, consumed): (Vec<DonorOrganRecord>, usize) =
            decode_from_slice(&enc).expect("decode Vec<DonorOrganRecord> failed");
        prop_assert_eq!(&vals, &dec, "Vec<DonorOrganRecord> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 19. Crossmatch paired with recipient re-encode stability ──────────────────

#[test]
fn test_crossmatch_with_recipient_stability() {
    proptest!(|(
        recipient in arb_recipient_waitlist(),
        crossmatch in arb_crossmatch_result(),
    )| {
        let pair = (recipient.clone(), crossmatch.clone());
        let enc1 = encode_to_vec(&pair).expect("first encode pair failed");
        let (dec, _): ((RecipientWaitlistEntry, CrossmatchResult), usize) =
            decode_from_slice(&enc1).expect("decode pair failed");
        let enc2 = encode_to_vec(&dec).expect("re-encode pair failed");
        prop_assert_eq!(enc1, enc2, "re-encode must be identical");
    });
}

// ── 20. Rejection episode vec with graft survival combined ────────────────────

#[test]
fn test_rejection_and_graft_survival_combined() {
    proptest!(|(
        episodes in prop::collection::vec(arb_rejection_episode(), 0..5),
        graft in arb_graft_survival(),
    )| {
        let combined = (episodes.clone(), graft.clone());
        let enc = encode_to_vec(&combined).expect("encode combined failed");
        let (dec, consumed): ((Vec<RejectionEpisode>, GraftSurvivalMetric), usize) =
            decode_from_slice(&enc).expect("decode combined failed");
        prop_assert_eq!(&combined, &dec, "combined roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 21. Full allocation pipeline tuple roundtrip ──────────────────────────────

#[test]
fn test_full_allocation_pipeline() {
    proptest!(|(
        donor in arb_donor_organ_record(),
        recipient in arb_recipient_waitlist(),
        score in arb_allocation_score(),
        transport in arb_transport_log(),
    )| {
        let pipeline = (donor.clone(), recipient.clone(), score.clone(), transport.clone());
        let enc = encode_to_vec(&pipeline).expect("encode pipeline failed");
        let (dec, consumed): (
            (DonorOrganRecord, RecipientWaitlistEntry, AllocationScore, OrganTransportLog),
            usize,
        ) = decode_from_slice(&enc).expect("decode pipeline failed");
        prop_assert_eq!(&pipeline, &dec, "pipeline roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}

// ── 22. Living donor chain with perfusion and histocompat roundtrip ───────────

#[test]
fn test_living_donor_chain_comprehensive() {
    proptest!(|(
        donor_eval in arb_living_donor_eval(),
        exchange in arb_paired_exchange(),
        perfusion in arb_perfusion_params(),
        histo in arb_histocompat_report(),
        desens in arb_desensitization(),
    )| {
        let record = (
            donor_eval.clone(),
            exchange.clone(),
            perfusion.clone(),
            histo.clone(),
            desens.clone(),
        );
        let enc = encode_to_vec(&record).expect("encode comprehensive record failed");
        let (dec, consumed): (
            (
                LivingDonorEvaluation,
                PairedExchangeChain,
                PerfusionParameters,
                HistocompatibilityReport,
                DesensitizationProtocol,
            ),
            usize,
        ) = decode_from_slice(&enc).expect("decode comprehensive record failed");
        prop_assert_eq!(&record, &dec, "comprehensive record roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes mismatch");
    });
}
