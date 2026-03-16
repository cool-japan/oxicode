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

// --- Domain types ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ClassificationLevel {
    Minimum,
    Low,
    Medium,
    High,
    Maximum,
    Administrative,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InmateClassification {
    inmate_id: String,
    current_level: ClassificationLevel,
    previous_level: Option<ClassificationLevel>,
    points_score: u32,
    override_reason: Option<String>,
    review_due_days: u16,
    is_protective_custody: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HousingUnit {
    unit_code: String,
    building: String,
    floor: u8,
    wing: String,
    capacity: u32,
    current_occupancy: u32,
    security_level: ClassificationLevel,
    is_restrictive: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HousingAssignment {
    inmate_id: String,
    unit_code: String,
    cell_number: String,
    bunk_position: String,
    assigned_epoch_secs: u64,
    reason: String,
    is_temporary: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum VisitationType {
    Contact,
    NonContact,
    Video,
    Legal,
    Clergy,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VisitationSchedule {
    visit_id: u64,
    inmate_id: String,
    visitor_name: String,
    visitor_relationship: String,
    visit_type: VisitationType,
    scheduled_epoch_secs: u64,
    duration_minutes: u16,
    approved: bool,
    denied_reason: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CommissaryTransaction {
    transaction_id: u64,
    inmate_id: String,
    items: Vec<CommissaryItem>,
    total_cents: u64,
    balance_after_cents: u64,
    transaction_epoch_secs: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CommissaryItem {
    item_code: String,
    description: String,
    quantity: u32,
    unit_price_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DisciplinaryOutcome {
    Dismissed,
    Warning,
    LossOfPrivileges { days: u32 },
    SolitaryConfinement { days: u32 },
    GoodTimeLoss { days: u32 },
    TransferRecommended,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DisciplinaryHearing {
    hearing_id: u64,
    inmate_id: String,
    incident_report_id: String,
    charge_codes: Vec<String>,
    hearing_epoch_secs: u64,
    hearing_officer: String,
    plea: String,
    finding: String,
    outcome: DisciplinaryOutcome,
    appeal_filed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ParoleEligibility {
    inmate_id: String,
    sentence_start_epoch_secs: u64,
    sentence_length_days: u32,
    good_time_earned_days: u32,
    good_time_lost_days: u32,
    mandatory_minimum_days: u32,
    earliest_release_epoch_secs: u64,
    parole_hearing_epoch_secs: Option<u64>,
    prior_denials: u16,
    risk_score: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ProgramType {
    GED,
    CollegeCourse,
    VocationalTraining,
    SubstanceAbuse,
    AngerManagement,
    LifeSkills,
    Literacy,
    ESL,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EducationalEnrollment {
    enrollment_id: u64,
    inmate_id: String,
    program: ProgramType,
    program_name: String,
    instructor: String,
    start_epoch_secs: u64,
    expected_end_epoch_secs: u64,
    hours_completed: f64,
    hours_required: f64,
    grade: Option<String>,
    completed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ScreeningCategory {
    IntakePhysical,
    DentalExam,
    MentalHealth,
    TBTest,
    VisionHearing,
    ChronicCareReview,
    EmergencyFollowUp,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MedicalScreening {
    screening_id: u64,
    inmate_id: String,
    category: ScreeningCategory,
    provider_name: String,
    screening_epoch_secs: u64,
    findings: Vec<String>,
    referrals: Vec<String>,
    follow_up_needed: bool,
    next_screening_epoch_secs: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WorkAssignment {
    assignment_id: u64,
    inmate_id: String,
    job_title: String,
    department: String,
    supervisor: String,
    shift_start_hour: u8,
    shift_end_hour: u8,
    daily_pay_cents: u32,
    rotation_week: u8,
    performance_rating: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ContrabandType {
    CellPhone,
    Weapon,
    Narcotics,
    Alcohol,
    Tobacco,
    Currency,
    UnauthorizedFood,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DetectionMethod {
    PatDown,
    CellSearch,
    BodyScanner,
    CanineUnit,
    MailInspection,
    TipFromInformant,
    RoutineInspection,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContrabandLog {
    log_id: u64,
    detection_epoch_secs: u64,
    location: String,
    contraband_type: ContrabandType,
    detection_method: DetectionMethod,
    quantity_description: String,
    associated_inmate_id: Option<String>,
    officer_badge: String,
    evidence_tag: String,
    referred_for_prosecution: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ReentryMilestone {
    ResumeCompleted,
    JobInterviewScheduled,
    HousingPlanApproved,
    IdentificationObtained,
    BankAccountOpened,
    MedicaidEnrolled,
    TransportationArranged,
    MentorAssigned,
    CommunityServiceStarted,
    VictimImpactCompleted,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReentryProgress {
    inmate_id: String,
    target_release_epoch_secs: u64,
    milestones_achieved: Vec<ReentryMilestone>,
    milestones_pending: Vec<ReentryMilestone>,
    case_manager: String,
    reentry_plan_approved: bool,
    community_sponsor: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FacilityHeadcount {
    count_id: u64,
    facility_code: String,
    count_epoch_secs: u64,
    expected_total: u32,
    actual_total: u32,
    discrepancy: i32,
    units_reporting: Vec<UnitCount>,
    count_type: String,
    cleared: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UnitCount {
    unit_code: String,
    expected: u32,
    actual: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GrievanceFiling {
    grievance_id: u64,
    inmate_id: String,
    filed_epoch_secs: u64,
    category: String,
    description: String,
    unit_at_time: String,
    response: Option<String>,
    response_epoch_secs: Option<u64>,
    escalated: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TransferRequest {
    request_id: u64,
    inmate_id: String,
    from_facility: String,
    to_facility: String,
    reason: String,
    requested_epoch_secs: u64,
    approved: Option<bool>,
    scheduled_transfer_epoch_secs: Option<u64>,
    security_escort_required: bool,
    medical_escort_required: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SentenceRecord {
    inmate_id: String,
    case_number: String,
    charges: Vec<String>,
    sentence_days: u32,
    consecutive: bool,
    court: String,
    judge: String,
    sentencing_epoch_secs: u64,
    credit_for_time_served_days: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TrustAccountBalance {
    inmate_id: String,
    balance_cents: i64,
    deposits_this_month_cents: u64,
    withdrawals_this_month_cents: u64,
    pending_obligations_cents: u64,
    restitution_deduction_pct: u8,
    last_deposit_epoch_secs: Option<u64>,
}

// --- Tests ---

#[test]
fn test_inmate_classification_roundtrip() {
    let val = InmateClassification {
        inmate_id: "DOC-2024-88431".into(),
        current_level: ClassificationLevel::Medium,
        previous_level: Some(ClassificationLevel::High),
        points_score: 47,
        override_reason: Some("Completed anger management program".into()),
        review_due_days: 180,
        is_protective_custody: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode inmate classification");
    let (decoded, _): (InmateClassification, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode inmate classification");
    assert_eq!(val, decoded);
}

#[test]
fn test_housing_unit_capacity() {
    let val = HousingUnit {
        unit_code: "B-WEST-3".into(),
        building: "B".into(),
        floor: 3,
        wing: "West".into(),
        capacity: 128,
        current_occupancy: 119,
        security_level: ClassificationLevel::Low,
        is_restrictive: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode housing unit");
    let (decoded, _): (HousingUnit, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode housing unit");
    assert_eq!(val, decoded);
}

#[test]
fn test_housing_assignment_roundtrip() {
    let val = HousingAssignment {
        inmate_id: "DOC-2023-55102".into(),
        unit_code: "C-EAST-1".into(),
        cell_number: "C1-214".into(),
        bunk_position: "Lower".into(),
        assigned_epoch_secs: 1_700_000_000,
        reason: "Initial classification placement".into(),
        is_temporary: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode housing assignment");
    let (decoded, _): (HousingAssignment, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode housing assignment");
    assert_eq!(val, decoded);
}

#[test]
fn test_visitation_contact_visit() {
    let val = VisitationSchedule {
        visit_id: 990_123,
        inmate_id: "DOC-2022-41009".into(),
        visitor_name: "Maria Santos".into(),
        visitor_relationship: "Spouse".into(),
        visit_type: VisitationType::Contact,
        scheduled_epoch_secs: 1_710_000_000,
        duration_minutes: 60,
        approved: true,
        denied_reason: None,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode contact visit");
    let (decoded, _): (VisitationSchedule, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode contact visit");
    assert_eq!(val, decoded);
}

#[test]
fn test_visitation_denied() {
    let val = VisitationSchedule {
        visit_id: 990_456,
        inmate_id: "DOC-2021-33781".into(),
        visitor_name: "John Doe".into(),
        visitor_relationship: "Friend".into(),
        visit_type: VisitationType::NonContact,
        scheduled_epoch_secs: 1_711_000_000,
        duration_minutes: 30,
        approved: false,
        denied_reason: Some("Visitor failed background check".into()),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode denied visit");
    let (decoded, _): (VisitationSchedule, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode denied visit");
    assert_eq!(val, decoded);
}

#[test]
fn test_commissary_transaction_multiple_items() {
    let val = CommissaryTransaction {
        transaction_id: 4_500_789,
        inmate_id: "DOC-2024-10233".into(),
        items: vec![
            CommissaryItem {
                item_code: "SNK-001".into(),
                description: "Ramen noodles 6-pack".into(),
                quantity: 2,
                unit_price_cents: 350,
            },
            CommissaryItem {
                item_code: "HYG-015".into(),
                description: "Deodorant".into(),
                quantity: 1,
                unit_price_cents: 425,
            },
            CommissaryItem {
                item_code: "STA-008".into(),
                description: "Writing pad".into(),
                quantity: 3,
                unit_price_cents: 175,
            },
        ],
        total_cents: 1650,
        balance_after_cents: 4_320,
        transaction_epoch_secs: 1_709_500_000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode commissary transaction");
    let (decoded, _): (CommissaryTransaction, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode commissary transaction");
    assert_eq!(val, decoded);
}

#[test]
fn test_disciplinary_hearing_solitary() {
    let val = DisciplinaryHearing {
        hearing_id: 78_001,
        inmate_id: "DOC-2023-66210".into(),
        incident_report_id: "IR-2024-3301".into(),
        charge_codes: vec!["201".into(), "305".into()],
        hearing_epoch_secs: 1_708_000_000,
        hearing_officer: "Captain Williams".into(),
        plea: "Not Guilty".into(),
        finding: "Guilty on charge 201, dismissed charge 305".into(),
        outcome: DisciplinaryOutcome::SolitaryConfinement { days: 15 },
        appeal_filed: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode disciplinary hearing");
    let (decoded, _): (DisciplinaryHearing, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode disciplinary hearing");
    assert_eq!(val, decoded);
}

#[test]
fn test_disciplinary_hearing_dismissed() {
    let val = DisciplinaryHearing {
        hearing_id: 78_055,
        inmate_id: "DOC-2024-11900".into(),
        incident_report_id: "IR-2024-3488".into(),
        charge_codes: vec!["102".into()],
        hearing_epoch_secs: 1_709_200_000,
        hearing_officer: "Lieutenant Chen".into(),
        plea: "Not Guilty".into(),
        finding: "Insufficient evidence".into(),
        outcome: DisciplinaryOutcome::Dismissed,
        appeal_filed: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode dismissed hearing");
    let (decoded, _): (DisciplinaryHearing, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode dismissed hearing");
    assert_eq!(val, decoded);
}

#[test]
fn test_parole_eligibility_calculation() {
    let val = ParoleEligibility {
        inmate_id: "DOC-2019-22340".into(),
        sentence_start_epoch_secs: 1_560_000_000,
        sentence_length_days: 3650,
        good_time_earned_days: 547,
        good_time_lost_days: 30,
        mandatory_minimum_days: 1825,
        earliest_release_epoch_secs: 1_717_000_000,
        parole_hearing_epoch_secs: Some(1_715_000_000),
        prior_denials: 1,
        risk_score: 3.7,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode parole eligibility");
    let (decoded, _): (ParoleEligibility, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode parole eligibility");
    assert_eq!(val, decoded);
}

#[test]
fn test_parole_no_hearing_scheduled() {
    let val = ParoleEligibility {
        inmate_id: "DOC-2023-90100".into(),
        sentence_start_epoch_secs: 1_690_000_000,
        sentence_length_days: 730,
        good_time_earned_days: 45,
        good_time_lost_days: 0,
        mandatory_minimum_days: 365,
        earliest_release_epoch_secs: 1_748_000_000,
        parole_hearing_epoch_secs: None,
        prior_denials: 0,
        risk_score: 2.1,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode parole no hearing");
    let (decoded, _): (ParoleEligibility, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode parole no hearing");
    assert_eq!(val, decoded);
}

#[test]
fn test_educational_enrollment_ged() {
    let val = EducationalEnrollment {
        enrollment_id: 15_670,
        inmate_id: "DOC-2022-77512".into(),
        program: ProgramType::GED,
        program_name: "GED Preparation Course".into(),
        instructor: "Ms. Thompson".into(),
        start_epoch_secs: 1_700_000_000,
        expected_end_epoch_secs: 1_715_000_000,
        hours_completed: 96.5,
        hours_required: 200.0,
        grade: None,
        completed: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode educational enrollment");
    let (decoded, _): (EducationalEnrollment, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode educational enrollment");
    assert_eq!(val, decoded);
}

#[test]
fn test_educational_vocational_completed() {
    let val = EducationalEnrollment {
        enrollment_id: 15_801,
        inmate_id: "DOC-2021-44028".into(),
        program: ProgramType::VocationalTraining,
        program_name: "Welding Certification Program".into(),
        instructor: "Mr. Kowalski".into(),
        start_epoch_secs: 1_685_000_000,
        expected_end_epoch_secs: 1_700_000_000,
        hours_completed: 480.0,
        hours_required: 480.0,
        grade: Some("Pass with Distinction".into()),
        completed: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode vocational completion");
    let (decoded, _): (EducationalEnrollment, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode vocational completion");
    assert_eq!(val, decoded);
}

#[test]
fn test_medical_screening_intake() {
    let val = MedicalScreening {
        screening_id: 330_100,
        inmate_id: "DOC-2024-99001".into(),
        category: ScreeningCategory::IntakePhysical,
        provider_name: "Dr. Patel".into(),
        screening_epoch_secs: 1_710_500_000,
        findings: vec![
            "Blood pressure elevated 145/92".into(),
            "BMI 31.2".into(),
            "Previous surgical scar left knee".into(),
        ],
        referrals: vec!["Chronic care clinic for hypertension".into()],
        follow_up_needed: true,
        next_screening_epoch_secs: Some(1_711_500_000),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode medical screening");
    let (decoded, _): (MedicalScreening, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode medical screening");
    assert_eq!(val, decoded);
}

#[test]
fn test_medical_screening_mental_health() {
    let val = MedicalScreening {
        screening_id: 330_245,
        inmate_id: "DOC-2024-99001".into(),
        category: ScreeningCategory::MentalHealth,
        provider_name: "Dr. Nguyen, PsyD".into(),
        screening_epoch_secs: 1_710_600_000,
        findings: vec!["PHQ-9 score 14, moderate depression".into()],
        referrals: vec![
            "Weekly individual therapy".into(),
            "Psychiatry evaluation for medication".into(),
        ],
        follow_up_needed: true,
        next_screening_epoch_secs: Some(1_711_200_000),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode mental health screening");
    let (decoded, _): (MedicalScreening, _) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode mental health screening");
    assert_eq!(val, decoded);
}

#[test]
fn test_work_assignment_rotation() {
    let val = WorkAssignment {
        assignment_id: 56_200,
        inmate_id: "DOC-2022-18334".into(),
        job_title: "Kitchen Worker".into(),
        department: "Food Services".into(),
        supervisor: "Sgt. Martinez".into(),
        shift_start_hour: 4,
        shift_end_hour: 12,
        daily_pay_cents: 225,
        rotation_week: 2,
        performance_rating: Some(4.2),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode work assignment");
    let (decoded, _): (WorkAssignment, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode work assignment");
    assert_eq!(val, decoded);
}

#[test]
fn test_contraband_detection_narcotics() {
    let val = ContrabandLog {
        log_id: 8_900,
        detection_epoch_secs: 1_709_800_000,
        location: "Unit D mailroom".into(),
        contraband_type: ContrabandType::Narcotics,
        detection_method: DetectionMethod::MailInspection,
        quantity_description: "Substance-soaked paper, 2 sheets".into(),
        associated_inmate_id: Some("DOC-2023-50712".into()),
        officer_badge: "B-4421".into(),
        evidence_tag: "EV-2024-0891".into(),
        referred_for_prosecution: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode contraband log");
    let (decoded, _): (ContrabandLog, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode contraband log");
    assert_eq!(val, decoded);
}

#[test]
fn test_contraband_detection_cell_phone() {
    let val = ContrabandLog {
        log_id: 8_945,
        detection_epoch_secs: 1_710_100_000,
        location: "Recreation yard, south fence line".into(),
        contraband_type: ContrabandType::CellPhone,
        detection_method: DetectionMethod::CanineUnit,
        quantity_description: "1 smartphone with charger, wrapped in plastic".into(),
        associated_inmate_id: None,
        officer_badge: "B-3107".into(),
        evidence_tag: "EV-2024-0923".into(),
        referred_for_prosecution: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode cell phone contraband");
    let (decoded, _): (ContrabandLog, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode cell phone contraband");
    assert_eq!(val, decoded);
}

#[test]
fn test_reentry_progress_milestones() {
    let val = ReentryProgress {
        inmate_id: "DOC-2018-30455".into(),
        target_release_epoch_secs: 1_720_000_000,
        milestones_achieved: vec![
            ReentryMilestone::ResumeCompleted,
            ReentryMilestone::IdentificationObtained,
            ReentryMilestone::MentorAssigned,
            ReentryMilestone::VictimImpactCompleted,
        ],
        milestones_pending: vec![
            ReentryMilestone::HousingPlanApproved,
            ReentryMilestone::JobInterviewScheduled,
            ReentryMilestone::BankAccountOpened,
            ReentryMilestone::MedicaidEnrolled,
            ReentryMilestone::TransportationArranged,
        ],
        case_manager: "Ms. Rodriguez".into(),
        reentry_plan_approved: false,
        community_sponsor: Some("Second Chance Alliance".into()),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode reentry progress");
    let (decoded, _): (ReentryProgress, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode reentry progress");
    assert_eq!(val, decoded);
}

#[test]
fn test_facility_headcount_clear() {
    let val = FacilityHeadcount {
        count_id: 1_200_456,
        facility_code: "FCLTY-NORTH-07".into(),
        count_epoch_secs: 1_710_300_000,
        expected_total: 1543,
        actual_total: 1543,
        discrepancy: 0,
        units_reporting: vec![
            UnitCount {
                unit_code: "A-1".into(),
                expected: 128,
                actual: 128,
            },
            UnitCount {
                unit_code: "A-2".into(),
                expected: 126,
                actual: 126,
            },
            UnitCount {
                unit_code: "B-1".into(),
                expected: 120,
                actual: 120,
            },
            UnitCount {
                unit_code: "B-2".into(),
                expected: 115,
                actual: 115,
            },
            UnitCount {
                unit_code: "SHU".into(),
                expected: 34,
                actual: 34,
            },
        ],
        count_type: "Standing count 2300hrs".into(),
        cleared: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode facility headcount");
    let (decoded, _): (FacilityHeadcount, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode facility headcount");
    assert_eq!(val, decoded);
}

#[test]
fn test_grievance_filing_with_response() {
    let val = GrievanceFiling {
        grievance_id: 44_320,
        inmate_id: "DOC-2023-88100".into(),
        filed_epoch_secs: 1_708_500_000,
        category: "Medical care".into(),
        description: "Requested dental appointment three weeks ago, no response received".into(),
        unit_at_time: "C-EAST-2".into(),
        response: Some("Dental appointment scheduled for next available slot".into()),
        response_epoch_secs: Some(1_709_000_000),
        escalated: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode grievance filing");
    let (decoded, _): (GrievanceFiling, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode grievance filing");
    assert_eq!(val, decoded);
}

#[test]
fn test_transfer_request_pending() {
    let val = TransferRequest {
        request_id: 67_800,
        inmate_id: "DOC-2020-12905".into(),
        from_facility: "FCLTY-NORTH-07".into(),
        to_facility: "FCLTY-CENTRAL-02".into(),
        reason: "Closer to family for visitation access".into(),
        requested_epoch_secs: 1_709_000_000,
        approved: None,
        scheduled_transfer_epoch_secs: None,
        security_escort_required: false,
        medical_escort_required: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode transfer request");
    let (decoded, _): (TransferRequest, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode transfer request");
    assert_eq!(val, decoded);
}

#[test]
fn test_sentence_record_and_trust_account() {
    let sentence = SentenceRecord {
        inmate_id: "DOC-2021-60099".into(),
        case_number: "CR-2021-4455".into(),
        charges: vec![
            "Burglary, second degree".into(),
            "Criminal mischief, third degree".into(),
        ],
        sentence_days: 1460,
        consecutive: false,
        court: "District Court, Division 3".into(),
        judge: "Hon. Patricia Hayes".into(),
        sentencing_epoch_secs: 1_630_000_000,
        credit_for_time_served_days: 87,
    };
    let sentence_bytes =
        encode_to_vec(&sentence, config::standard()).expect("encode sentence record");
    let (decoded_sentence, _): (SentenceRecord, _) =
        decode_owned_from_slice(&sentence_bytes, config::standard())
            .expect("decode sentence record");
    assert_eq!(sentence, decoded_sentence);

    let account = TrustAccountBalance {
        inmate_id: "DOC-2021-60099".into(),
        balance_cents: 12_340,
        deposits_this_month_cents: 5_000,
        withdrawals_this_month_cents: 2_150,
        pending_obligations_cents: 800,
        restitution_deduction_pct: 20,
        last_deposit_epoch_secs: Some(1_709_600_000),
    };
    let account_bytes = encode_to_vec(&account, config::standard()).expect("encode trust account");
    let (decoded_account, _): (TrustAccountBalance, _) =
        decode_owned_from_slice(&account_bytes, config::standard()).expect("decode trust account");
    assert_eq!(account, decoded_account);
}
