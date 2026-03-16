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
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ── Domain types: HR recruitment and talent management ──────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ApplicationStage {
    Applied,
    PhoneScreen,
    TechnicalInterview,
    OnsiteInterview,
    OfferExtended,
    OfferAccepted,
    Rejected,
    Withdrawn,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Internship,
    Freelance,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SkillProficiency {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BackgroundCheckStatus {
    Pending,
    InProgress,
    Cleared,
    FlaggedForReview,
    Failed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OnboardingTaskStatus {
    NotStarted,
    InProgress,
    Completed,
    Blocked,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PerformanceRating {
    NeedsImprovement,
    MeetsExpectations,
    ExceedsExpectations,
    Outstanding,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DiversityCategory {
    Gender,
    Ethnicity,
    Disability,
    VeteranStatus,
    AgeGroup,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReferenceRelationship {
    DirectManager,
    Colleague,
    DirectReport,
    Client,
    Mentor,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SuccessionReadiness {
    ReadyNow,
    ReadyInOneYear,
    ReadyInTwoYears,
    NeedsSignificantDevelopment,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InterviewFormat {
    InPerson,
    VideoCall,
    PhoneOnly,
    TakeHomeAssignment,
    PairProgramming,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OfferStatus {
    Drafting,
    PendingApproval,
    Sent,
    Accepted,
    Declined,
    Expired,
    Countered,
}

// ── Composite structs ───────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CandidateProfile {
    candidate_id: u64,
    first_name: String,
    last_name: String,
    email: String,
    years_of_experience: u16,
    current_title: String,
    desired_salary_min: u64,
    desired_salary_max: u64,
    is_relocatable: bool,
    stage: ApplicationStage,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JobRequisition {
    requisition_id: u64,
    title: String,
    department: String,
    hiring_manager: String,
    headcount: u16,
    employment_type: EmploymentType,
    salary_range_low: u64,
    salary_range_high: u64,
    is_remote_eligible: bool,
    created_epoch: u64,
    priority_level: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InterviewScorecard {
    scorecard_id: u64,
    candidate_id: u64,
    interviewer_name: String,
    format: InterviewFormat,
    technical_score: u8,
    communication_score: u8,
    culture_fit_score: u8,
    problem_solving_score: u8,
    overall_recommendation: bool,
    notes: String,
    duration_minutes: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OfferLetter {
    offer_id: u64,
    candidate_id: u64,
    requisition_id: u64,
    status: OfferStatus,
    base_salary: u64,
    signing_bonus: u64,
    equity_shares: u32,
    start_date_epoch: u64,
    expiration_epoch: u64,
    is_negotiable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CompensationPackage {
    package_id: u64,
    employee_id: u64,
    base_salary: u64,
    annual_bonus_target_pct: u8,
    equity_vest_months: u16,
    health_insurance_tier: u8,
    retirement_match_pct: u8,
    pto_days: u16,
    remote_stipend_monthly: u32,
    relocation_allowance: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SkillAssessment {
    assessment_id: u64,
    candidate_id: u64,
    skill_name: String,
    proficiency: SkillProficiency,
    score_out_of_100: u8,
    assessed_by: String,
    assessment_epoch: u64,
    is_verified: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BackgroundCheckResult {
    check_id: u64,
    candidate_id: u64,
    provider_name: String,
    status: BackgroundCheckStatus,
    criminal_clear: bool,
    education_verified: bool,
    employment_verified: bool,
    credit_check_passed: bool,
    initiated_epoch: u64,
    completed_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OnboardingTask {
    task_id: u64,
    employee_id: u64,
    task_name: String,
    status: OnboardingTaskStatus,
    due_date_epoch: u64,
    assigned_to: String,
    is_mandatory: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OnboardingChecklist {
    checklist_id: u64,
    employee_id: u64,
    department: String,
    tasks: Vec<OnboardingTask>,
    overall_progress_pct: u8,
    start_date_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PerformanceReviewCycle {
    cycle_id: u64,
    cycle_name: String,
    start_epoch: u64,
    end_epoch: u64,
    eligible_employee_count: u32,
    completed_review_count: u32,
    average_rating: u8,
    is_calibrated: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PerformanceReview {
    review_id: u64,
    employee_id: u64,
    reviewer_id: u64,
    cycle_id: u64,
    rating: PerformanceRating,
    goals_met_count: u16,
    goals_total_count: u16,
    strengths: String,
    areas_for_improvement: String,
    is_self_review: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SuccessionPlanEntry {
    entry_id: u64,
    target_role: String,
    incumbent_id: u64,
    successor_id: u64,
    readiness: SuccessionReadiness,
    development_plan: String,
    risk_of_loss: u8,
    impact_of_loss: u8,
    last_reviewed_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiversityMetricSnapshot {
    snapshot_id: u64,
    department: String,
    category: DiversityCategory,
    total_headcount: u32,
    representation_count: u32,
    representation_pct_x100: u16,
    pipeline_count: u32,
    hire_count_ytd: u32,
    attrition_count_ytd: u32,
    report_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ApplicantTrackingRecord {
    tracking_id: u64,
    candidate_id: u64,
    requisition_id: u64,
    current_stage: ApplicationStage,
    days_in_pipeline: u16,
    source_channel: String,
    recruiter_name: String,
    is_internal: bool,
    referral_employee_id: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReferenceCheckRecord {
    reference_id: u64,
    candidate_id: u64,
    referee_name: String,
    referee_title: String,
    relationship: ReferenceRelationship,
    years_known: u8,
    would_rehire: bool,
    overall_rating: u8,
    comments: String,
    contacted_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RecruitmentFunnel {
    funnel_id: u64,
    requisition_id: u64,
    applied_count: u32,
    screened_count: u32,
    phone_interview_count: u32,
    onsite_interview_count: u32,
    offer_count: u32,
    accepted_count: u32,
    average_time_to_fill_days: u16,
    cost_per_hire_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmployeeBenefitsElection {
    election_id: u64,
    employee_id: u64,
    plan_year: u16,
    medical_plan: String,
    dental_enrolled: bool,
    vision_enrolled: bool,
    life_insurance_multiple: u8,
    hsa_contribution_annual: u32,
    fsa_contribution_annual: u32,
    dependents_count: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainingCertification {
    cert_id: u64,
    employee_id: u64,
    certification_name: String,
    issuing_body: String,
    earned_epoch: u64,
    expiry_epoch: u64,
    is_mandatory: bool,
    renewal_required: bool,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HeadcountPlan {
    plan_id: u64,
    department: String,
    fiscal_year: u16,
    current_headcount: u32,
    approved_new_hires: u32,
    filled_positions: u32,
    budget_allocated_cents: u64,
    budget_spent_cents: u64,
    attrition_forecast: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExitInterview {
    interview_id: u64,
    employee_id: u64,
    tenure_months: u32,
    primary_reason: String,
    would_return: bool,
    manager_rating: u8,
    culture_rating: u8,
    compensation_rating: u8,
    growth_rating: u8,
    additional_feedback: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TalentPoolEntry {
    entry_id: u64,
    candidate_id: u64,
    pool_name: String,
    source: String,
    skills: Vec<String>,
    engagement_score: u8,
    last_contacted_epoch: u64,
    is_active: bool,
    preferred_employment: EmploymentType,
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_candidate_profile_versioning() {
    let version = Version::new(1, 0, 0);
    let val = CandidateProfile {
        candidate_id: 100_001,
        first_name: "Haruki".into(),
        last_name: "Tanaka".into(),
        email: "haruki.tanaka@example.com".into(),
        years_of_experience: 8,
        current_title: "Senior Software Engineer".into(),
        desired_salary_min: 120_000,
        desired_salary_max: 160_000,
        is_relocatable: true,
        stage: ApplicationStage::TechnicalInterview,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode candidate profile");
    let (decoded, decoded_version, _size): (CandidateProfile, Version, usize) =
        decode_versioned_value(&bytes).expect("decode candidate profile");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_job_requisition_versioning() {
    let version = Version::new(2, 1, 0);
    let val = JobRequisition {
        requisition_id: 5001,
        title: "Staff Data Scientist".into(),
        department: "Machine Learning".into(),
        hiring_manager: "Yuki Sato".into(),
        headcount: 3,
        employment_type: EmploymentType::FullTime,
        salary_range_low: 150_000,
        salary_range_high: 220_000,
        is_remote_eligible: true,
        created_epoch: 1_700_000_000,
        priority_level: 1,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode job requisition");
    let (decoded, decoded_version, _size): (JobRequisition, Version, usize) =
        decode_versioned_value(&bytes).expect("decode job requisition");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_interview_scorecard_versioning() {
    let version = Version::new(1, 2, 3);
    let val = InterviewScorecard {
        scorecard_id: 88_001,
        candidate_id: 100_001,
        interviewer_name: "Kenji Yamamoto".into(),
        format: InterviewFormat::PairProgramming,
        technical_score: 9,
        communication_score: 8,
        culture_fit_score: 7,
        problem_solving_score: 9,
        overall_recommendation: true,
        notes: "Strong systems design skills, excellent at explaining tradeoffs".into(),
        duration_minutes: 60,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode interview scorecard");
    let (decoded, decoded_version, _size): (InterviewScorecard, Version, usize) =
        decode_versioned_value(&bytes).expect("decode interview scorecard");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_offer_letter_versioning() {
    let version = Version::new(3, 0, 0);
    let val = OfferLetter {
        offer_id: 7001,
        candidate_id: 100_001,
        requisition_id: 5001,
        status: OfferStatus::Sent,
        base_salary: 175_000,
        signing_bonus: 25_000,
        equity_shares: 10_000,
        start_date_epoch: 1_710_000_000,
        expiration_epoch: 1_711_000_000,
        is_negotiable: true,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode offer letter");
    let (decoded, decoded_version, _size): (OfferLetter, Version, usize) =
        decode_versioned_value(&bytes).expect("decode offer letter");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_compensation_package_versioning() {
    let version = Version::new(1, 5, 0);
    let val = CompensationPackage {
        package_id: 30_001,
        employee_id: 200_001,
        base_salary: 185_000,
        annual_bonus_target_pct: 20,
        equity_vest_months: 48,
        health_insurance_tier: 2,
        retirement_match_pct: 6,
        pto_days: 25,
        remote_stipend_monthly: 200,
        relocation_allowance: 15_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode compensation package");
    let (decoded, decoded_version, _size): (CompensationPackage, Version, usize) =
        decode_versioned_value(&bytes).expect("decode compensation package");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_skill_assessment_versioning() {
    let version = Version::new(1, 0, 7);
    let val = SkillAssessment {
        assessment_id: 45_001,
        candidate_id: 100_002,
        skill_name: "Distributed Systems".into(),
        proficiency: SkillProficiency::Expert,
        score_out_of_100: 92,
        assessed_by: "Technical Review Board".into(),
        assessment_epoch: 1_705_000_000,
        is_verified: true,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode skill assessment");
    let (decoded, decoded_version, _size): (SkillAssessment, Version, usize) =
        decode_versioned_value(&bytes).expect("decode skill assessment");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_background_check_result_versioning() {
    let version = Version::new(2, 0, 1);
    let val = BackgroundCheckResult {
        check_id: 60_001,
        candidate_id: 100_003,
        provider_name: "SecureVetting Inc.".into(),
        status: BackgroundCheckStatus::Cleared,
        criminal_clear: true,
        education_verified: true,
        employment_verified: true,
        credit_check_passed: true,
        initiated_epoch: 1_704_000_000,
        completed_epoch: 1_704_500_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode background check");
    let (decoded, decoded_version, _size): (BackgroundCheckResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode background check");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_onboarding_checklist_versioning() {
    let version = Version::new(1, 3, 0);
    let tasks = vec![
        OnboardingTask {
            task_id: 1,
            employee_id: 200_010,
            task_name: "Complete I-9 form".into(),
            status: OnboardingTaskStatus::Completed,
            due_date_epoch: 1_710_100_000,
            assigned_to: "HR Operations".into(),
            is_mandatory: true,
        },
        OnboardingTask {
            task_id: 2,
            employee_id: 200_010,
            task_name: "Set up development environment".into(),
            status: OnboardingTaskStatus::InProgress,
            due_date_epoch: 1_710_200_000,
            assigned_to: "Engineering IT".into(),
            is_mandatory: true,
        },
        OnboardingTask {
            task_id: 3,
            employee_id: 200_010,
            task_name: "Schedule meet-and-greet with team".into(),
            status: OnboardingTaskStatus::NotStarted,
            due_date_epoch: 1_710_300_000,
            assigned_to: "Hiring Manager".into(),
            is_mandatory: false,
        },
    ];
    let val = OnboardingChecklist {
        checklist_id: 9001,
        employee_id: 200_010,
        department: "Platform Engineering".into(),
        tasks,
        overall_progress_pct: 33,
        start_date_epoch: 1_710_000_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode onboarding checklist");
    let (decoded, decoded_version, _size): (OnboardingChecklist, Version, usize) =
        decode_versioned_value(&bytes).expect("decode onboarding checklist");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_performance_review_cycle_versioning() {
    let version = Version::new(4, 0, 0);
    let val = PerformanceReviewCycle {
        cycle_id: 2026_01,
        cycle_name: "Q1 2026 Performance Cycle".into(),
        start_epoch: 1_704_067_200,
        end_epoch: 1_711_929_600,
        eligible_employee_count: 1_250,
        completed_review_count: 980,
        average_rating: 3,
        is_calibrated: false,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode review cycle");
    let (decoded, decoded_version, _size): (PerformanceReviewCycle, Version, usize) =
        decode_versioned_value(&bytes).expect("decode review cycle");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_performance_review_versioning() {
    let version = Version::new(1, 1, 0);
    let val = PerformanceReview {
        review_id: 77_001,
        employee_id: 200_050,
        reviewer_id: 200_010,
        cycle_id: 2026_01,
        rating: PerformanceRating::ExceedsExpectations,
        goals_met_count: 7,
        goals_total_count: 8,
        strengths: "Exceptional technical leadership and mentoring".into(),
        areas_for_improvement: "Cross-team communication could be more proactive".into(),
        is_self_review: false,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode performance review");
    let (decoded, decoded_version, _size): (PerformanceReview, Version, usize) =
        decode_versioned_value(&bytes).expect("decode performance review");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_succession_plan_entry_versioning() {
    let version = Version::new(2, 2, 0);
    let val = SuccessionPlanEntry {
        entry_id: 1001,
        target_role: "VP of Engineering".into(),
        incumbent_id: 200_001,
        successor_id: 200_050,
        readiness: SuccessionReadiness::ReadyInOneYear,
        development_plan: "Executive coaching program, cross-functional project lead".into(),
        risk_of_loss: 3,
        impact_of_loss: 9,
        last_reviewed_epoch: 1_708_000_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode succession plan");
    let (decoded, decoded_version, _size): (SuccessionPlanEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode succession plan");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_diversity_metric_snapshot_versioning() {
    let version = Version::new(1, 0, 2);
    let val = DiversityMetricSnapshot {
        snapshot_id: 5501,
        department: "Engineering".into(),
        category: DiversityCategory::Gender,
        total_headcount: 320,
        representation_count: 112,
        representation_pct_x100: 3500,
        pipeline_count: 85,
        hire_count_ytd: 14,
        attrition_count_ytd: 6,
        report_epoch: 1_709_000_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode diversity metrics");
    let (decoded, decoded_version, _size): (DiversityMetricSnapshot, Version, usize) =
        decode_versioned_value(&bytes).expect("decode diversity metrics");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_applicant_tracking_record_versioning() {
    let version = Version::new(3, 1, 0);
    let val = ApplicantTrackingRecord {
        tracking_id: 110_001,
        candidate_id: 100_010,
        requisition_id: 5005,
        current_stage: ApplicationStage::OnsiteInterview,
        days_in_pipeline: 23,
        source_channel: "Employee Referral".into(),
        recruiter_name: "Aiko Nakamura".into(),
        is_internal: false,
        referral_employee_id: 200_033,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode tracking record");
    let (decoded, decoded_version, _size): (ApplicantTrackingRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode tracking record");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_reference_check_record_versioning() {
    let version = Version::new(1, 4, 0);
    let val = ReferenceCheckRecord {
        reference_id: 22_001,
        candidate_id: 100_010,
        referee_name: "Daisuke Ito".into(),
        referee_title: "Engineering Director".into(),
        relationship: ReferenceRelationship::DirectManager,
        years_known: 4,
        would_rehire: true,
        overall_rating: 9,
        comments: "One of the strongest engineers I have managed in 15 years".into(),
        contacted_epoch: 1_707_500_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode reference check");
    let (decoded, decoded_version, _size): (ReferenceCheckRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode reference check");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_recruitment_funnel_versioning() {
    let version = Version::new(2, 0, 0);
    let val = RecruitmentFunnel {
        funnel_id: 8001,
        requisition_id: 5001,
        applied_count: 342,
        screened_count: 89,
        phone_interview_count: 34,
        onsite_interview_count: 12,
        offer_count: 4,
        accepted_count: 3,
        average_time_to_fill_days: 45,
        cost_per_hire_cents: 850_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode recruitment funnel");
    let (decoded, decoded_version, _size): (RecruitmentFunnel, Version, usize) =
        decode_versioned_value(&bytes).expect("decode recruitment funnel");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_employee_benefits_election_versioning() {
    let version = Version::new(1, 0, 0);
    let val = EmployeeBenefitsElection {
        election_id: 40_001,
        employee_id: 200_050,
        plan_year: 2026,
        medical_plan: "Premium PPO Family".into(),
        dental_enrolled: true,
        vision_enrolled: true,
        life_insurance_multiple: 3,
        hsa_contribution_annual: 7_300,
        fsa_contribution_annual: 0,
        dependents_count: 2,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode benefits election");
    let (decoded, decoded_version, _size): (EmployeeBenefitsElection, Version, usize) =
        decode_versioned_value(&bytes).expect("decode benefits election");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_training_certification_versioning() {
    let version = Version::new(1, 6, 0);
    let val = TrainingCertification {
        cert_id: 55_001,
        employee_id: 200_050,
        certification_name: "AWS Solutions Architect Professional".into(),
        issuing_body: "Amazon Web Services".into(),
        earned_epoch: 1_700_000_000,
        expiry_epoch: 1_794_000_000,
        is_mandatory: false,
        renewal_required: true,
        cost_cents: 30_000,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode training cert");
    let (decoded, decoded_version, _size): (TrainingCertification, Version, usize) =
        decode_versioned_value(&bytes).expect("decode training cert");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_headcount_plan_versioning() {
    let version = Version::new(5, 0, 0);
    let val = HeadcountPlan {
        plan_id: 2026,
        department: "Product Engineering".into(),
        fiscal_year: 2026,
        current_headcount: 145,
        approved_new_hires: 30,
        filled_positions: 12,
        budget_allocated_cents: 7_500_000_00,
        budget_spent_cents: 2_800_000_00,
        attrition_forecast: 18,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode headcount plan");
    let (decoded, decoded_version, _size): (HeadcountPlan, Version, usize) =
        decode_versioned_value(&bytes).expect("decode headcount plan");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_exit_interview_versioning() {
    let version = Version::new(1, 2, 0);
    let val = ExitInterview {
        interview_id: 3001,
        employee_id: 200_099,
        tenure_months: 36,
        primary_reason: "Career growth opportunity elsewhere".into(),
        would_return: true,
        manager_rating: 7,
        culture_rating: 8,
        compensation_rating: 5,
        growth_rating: 4,
        additional_feedback: "Loved the team, but felt limited in advancement paths".into(),
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode exit interview");
    let (decoded, decoded_version, _size): (ExitInterview, Version, usize) =
        decode_versioned_value(&bytes).expect("decode exit interview");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_talent_pool_entry_versioning() {
    let version = Version::new(2, 3, 1);
    let val = TalentPoolEntry {
        entry_id: 66_001,
        candidate_id: 100_020,
        pool_name: "Senior Backend Engineers".into(),
        source: "Conference Networking".into(),
        skills: vec![
            "Rust".into(),
            "Distributed Systems".into(),
            "Kubernetes".into(),
            "PostgreSQL".into(),
        ],
        engagement_score: 85,
        last_contacted_epoch: 1_708_500_000,
        is_active: true,
        preferred_employment: EmploymentType::FullTime,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode talent pool entry");
    let (decoded, decoded_version, _size): (TalentPoolEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode talent pool entry");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
}

#[test]
fn test_rejected_candidate_stage_versioning() {
    let version = Version::new(1, 0, 0);
    let val = CandidateProfile {
        candidate_id: 100_099,
        first_name: "Mika".into(),
        last_name: "Suzuki".into(),
        email: "mika.suzuki@example.com".into(),
        years_of_experience: 2,
        current_title: "Junior Developer".into(),
        desired_salary_min: 65_000,
        desired_salary_max: 80_000,
        is_relocatable: false,
        stage: ApplicationStage::Rejected,
    };
    let bytes = encode_versioned_value(&val, version.clone()).expect("encode rejected candidate");
    let (decoded, decoded_version, _size): (CandidateProfile, Version, usize) =
        decode_versioned_value(&bytes).expect("decode rejected candidate");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
    assert_eq!(decoded.stage, ApplicationStage::Rejected);
}

#[test]
fn test_background_check_flagged_versioning() {
    let version = Version::new(2, 0, 1);
    let val = BackgroundCheckResult {
        check_id: 60_099,
        candidate_id: 100_050,
        provider_name: "TrustGuard Screening".into(),
        status: BackgroundCheckStatus::FlaggedForReview,
        criminal_clear: true,
        education_verified: false,
        employment_verified: true,
        credit_check_passed: true,
        initiated_epoch: 1_706_000_000,
        completed_epoch: 1_706_800_000,
    };
    let bytes =
        encode_versioned_value(&val, version.clone()).expect("encode flagged background check");
    let (decoded, decoded_version, _size): (BackgroundCheckResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode flagged background check");
    assert_eq!(val, decoded);
    assert_eq!(version, decoded_version);
    assert_eq!(decoded.status, BackgroundCheckStatus::FlaggedForReview);
    assert!(!decoded.education_verified);
}
