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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types: Education & Learning Management Systems ───────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CourseStatus {
    Draft,
    Published,
    Archived,
    UnderReview,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnrollmentStatus {
    Active,
    Completed,
    Withdrawn,
    Suspended,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SubmissionState {
    NotStarted,
    InProgress,
    Submitted,
    Graded,
    Returned,
    Resubmitted,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QuestionType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Essay,
    FillInBlank,
    Matching,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CompetencyLevel {
    Novice,
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AttendanceType {
    Present,
    Absent,
    Tardy,
    Excused,
    Remote,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PlagiarismVerdict {
    Clear,
    LowSimilarity,
    ModerateSimilarity,
    HighSimilarity,
    Confirmed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CredentialType {
    CourseCertificate,
    ProfessionalCertification,
    MicroCredential,
    DigitalBadge,
    Diploma,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CourseCatalogEntry {
    course_id: u64,
    title: String,
    department: String,
    credit_hours: u16,
    max_enrollment: u32,
    status: CourseStatus,
    prerequisite_ids: Vec<u64>,
    semester_code: String,
    instructor_id: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StudentEnrollmentRecord {
    enrollment_id: u64,
    student_id: u64,
    course_id: u64,
    status: EnrollmentStatus,
    enrolled_at_epoch: u64,
    final_grade_points: Option<u32>,
    credits_earned: u16,
    section_number: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssignmentSubmission {
    submission_id: u64,
    student_id: u64,
    assignment_id: u64,
    state: SubmissionState,
    submitted_at_epoch: u64,
    file_hash: String,
    word_count: u32,
    attempt_number: u8,
    late_penalty_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GradingRubric {
    rubric_id: u64,
    assignment_id: u64,
    criterion_name: String,
    max_points: u32,
    weight_bps: u16,
    description: String,
    levels: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuizQuestion {
    question_id: u64,
    bank_id: u64,
    question_type: QuestionType,
    prompt_text: String,
    correct_answer_hash: String,
    points: u32,
    difficulty_rating: u16,
    time_limit_secs: u32,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LearningPathProgression {
    path_id: u64,
    learner_id: u64,
    total_modules: u32,
    completed_modules: u32,
    current_module_id: u64,
    xp_earned: u64,
    streak_days: u32,
    last_activity_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CompetencyFrameworkEntry {
    framework_id: u64,
    competency_name: String,
    domain: String,
    level: CompetencyLevel,
    evidence_count: u32,
    assessed_at_epoch: u64,
    assessor_id: u64,
    mastery_score_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AttendanceRecord {
    record_id: u64,
    student_id: u64,
    course_id: u64,
    session_date_epoch: u64,
    attendance: AttendanceType,
    check_in_epoch: u64,
    duration_minutes: u32,
    location: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiscussionForumThread {
    thread_id: u64,
    course_id: u64,
    author_id: u64,
    title: String,
    body_preview: String,
    reply_count: u32,
    upvotes: u32,
    created_at_epoch: u64,
    pinned: bool,
    resolved: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PeerReviewAssignment {
    review_id: u64,
    reviewer_id: u64,
    author_id: u64,
    submission_id: u64,
    score_bps: u16,
    feedback_text: String,
    completed: bool,
    assigned_at_epoch: u64,
    due_at_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlagiarismDetectionResult {
    scan_id: u64,
    submission_id: u64,
    similarity_score_bps: u16,
    matched_source_count: u32,
    verdict: PlagiarismVerdict,
    scanned_at_epoch: u64,
    report_hash: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CertificateIssuance {
    certificate_id: u64,
    recipient_id: u64,
    credential_type: CredentialType,
    title: String,
    issuer_org: String,
    issued_at_epoch: u64,
    expiry_epoch: Option<u64>,
    verification_hash: String,
    revoked: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdaptiveLearningRecommendation {
    recommendation_id: u64,
    learner_id: u64,
    recommended_module_id: u64,
    confidence_score_bps: u16,
    reason: String,
    predicted_mastery_gain_bps: u16,
    generated_at_epoch: u64,
    accepted: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScormLearningRecord {
    record_id: u64,
    learner_id: u64,
    sco_id: String,
    verb: String,
    score_raw: u32,
    score_max: u32,
    completion_status: String,
    success_status: String,
    session_duration_secs: u64,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FacultyWorkloadAllocation {
    allocation_id: u64,
    faculty_id: u64,
    semester_code: String,
    teaching_hours: u16,
    research_hours: u16,
    admin_hours: u16,
    course_count: u8,
    advisee_count: u32,
    overload: bool,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_course_catalog_entry_roundtrip() {
    let entry = CourseCatalogEntry {
        course_id: 10001,
        title: "Introduction to Algorithms".to_string(),
        department: "Computer Science".to_string(),
        credit_hours: 4,
        max_enrollment: 120,
        status: CourseStatus::Published,
        prerequisite_ids: vec![9001, 9002],
        semester_code: "2026-SPRING".to_string(),
        instructor_id: 50001,
    };
    let bytes = encode_to_vec(&entry).expect("encode CourseCatalogEntry failed");
    let (decoded, consumed) =
        decode_from_slice::<CourseCatalogEntry>(&bytes).expect("decode CourseCatalogEntry failed");
    assert_eq!(entry, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_course_catalog_entry_versioned_v1_0_0() {
    let entry = CourseCatalogEntry {
        course_id: 10002,
        title: "Discrete Mathematics".to_string(),
        department: "Mathematics".to_string(),
        credit_hours: 3,
        max_enrollment: 80,
        status: CourseStatus::Draft,
        prerequisite_ids: vec![],
        semester_code: "2026-FALL".to_string(),
        instructor_id: 50002,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&entry, version)
        .expect("encode versioned CourseCatalogEntry v1.0.0 failed");
    let (decoded, ver, _consumed): (CourseCatalogEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned CourseCatalogEntry v1.0.0 failed");
    assert_eq!(entry, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_student_enrollment_record_roundtrip() {
    let record = StudentEnrollmentRecord {
        enrollment_id: 20001,
        student_id: 300001,
        course_id: 10001,
        status: EnrollmentStatus::Active,
        enrolled_at_epoch: 1_740_000_000,
        final_grade_points: None,
        credits_earned: 0,
        section_number: 3,
    };
    let bytes = encode_to_vec(&record).expect("encode StudentEnrollmentRecord failed");
    let (decoded, consumed) = decode_from_slice::<StudentEnrollmentRecord>(&bytes)
        .expect("decode StudentEnrollmentRecord failed");
    assert_eq!(record, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_student_enrollment_completed_versioned_v2_1_0() {
    let record = StudentEnrollmentRecord {
        enrollment_id: 20002,
        student_id: 300002,
        course_id: 10002,
        status: EnrollmentStatus::Completed,
        enrolled_at_epoch: 1_735_000_000,
        final_grade_points: Some(385),
        credits_earned: 3,
        section_number: 1,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&record, version)
        .expect("encode versioned StudentEnrollmentRecord v2.1.0 failed");
    let (decoded, ver, consumed): (StudentEnrollmentRecord, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned StudentEnrollmentRecord v2.1.0 failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_assignment_submission_graded_roundtrip() {
    let submission = AssignmentSubmission {
        submission_id: 30001,
        student_id: 300001,
        assignment_id: 40001,
        state: SubmissionState::Graded,
        submitted_at_epoch: 1_741_000_000,
        file_hash: "sha256:abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
            .to_string(),
        word_count: 2_847,
        attempt_number: 1,
        late_penalty_bps: 0,
    };
    let bytes = encode_to_vec(&submission).expect("encode AssignmentSubmission failed");
    let (decoded, consumed) = decode_from_slice::<AssignmentSubmission>(&bytes)
        .expect("decode AssignmentSubmission failed");
    assert_eq!(submission, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_assignment_submission_resubmitted_versioned_v1_2_0() {
    let submission = AssignmentSubmission {
        submission_id: 30002,
        student_id: 300003,
        assignment_id: 40001,
        state: SubmissionState::Resubmitted,
        submitted_at_epoch: 1_741_500_000,
        file_hash: "sha256:fedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321"
            .to_string(),
        word_count: 3_102,
        attempt_number: 3,
        late_penalty_bps: 1000,
    };
    let version = Version::new(1, 2, 0);
    let bytes = encode_versioned_value(&submission, version)
        .expect("encode versioned AssignmentSubmission v1.2.0 failed");
    let (decoded, ver, _consumed): (AssignmentSubmission, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned AssignmentSubmission v1.2.0 failed");
    assert_eq!(submission, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_grading_rubric_roundtrip() {
    let rubric = GradingRubric {
        rubric_id: 50001,
        assignment_id: 40001,
        criterion_name: "Critical Analysis".to_string(),
        max_points: 25,
        weight_bps: 3500,
        description: "Demonstrates depth of analysis with supporting evidence".to_string(),
        levels: vec![
            "Exemplary".to_string(),
            "Proficient".to_string(),
            "Developing".to_string(),
            "Beginning".to_string(),
        ],
    };
    let bytes = encode_to_vec(&rubric).expect("encode GradingRubric failed");
    let (decoded, consumed) =
        decode_from_slice::<GradingRubric>(&bytes).expect("decode GradingRubric failed");
    assert_eq!(rubric, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_quiz_question_bank_versioned_v3_0_0() {
    let question = QuizQuestion {
        question_id: 60001,
        bank_id: 7001,
        question_type: QuestionType::MultipleChoice,
        prompt_text: "Which sorting algorithm has O(n log n) average-case complexity?".to_string(),
        correct_answer_hash: "sha256:mergesort_hash_placeholder".to_string(),
        points: 5,
        difficulty_rating: 6500,
        time_limit_secs: 120,
        tags: vec![
            "algorithms".to_string(),
            "sorting".to_string(),
            "complexity".to_string(),
        ],
    };
    let version = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&question, version)
        .expect("encode versioned QuizQuestion v3.0.0 failed");
    let (decoded, ver, consumed): (QuizQuestion, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned QuizQuestion v3.0.0 failed");
    assert_eq!(question, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_learning_path_progression_roundtrip() {
    let path = LearningPathProgression {
        path_id: 80001,
        learner_id: 300004,
        total_modules: 24,
        completed_modules: 18,
        current_module_id: 80019,
        xp_earned: 14_500,
        streak_days: 42,
        last_activity_epoch: 1_742_000_000,
    };
    let bytes = encode_to_vec(&path).expect("encode LearningPathProgression failed");
    let (decoded, consumed) = decode_from_slice::<LearningPathProgression>(&bytes)
        .expect("decode LearningPathProgression failed");
    assert_eq!(path, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_competency_framework_versioned_v1_5_2() {
    let competency = CompetencyFrameworkEntry {
        framework_id: 90001,
        competency_name: "Data Structures Mastery".to_string(),
        domain: "Computer Science Fundamentals".to_string(),
        level: CompetencyLevel::Advanced,
        evidence_count: 12,
        assessed_at_epoch: 1_741_800_000,
        assessor_id: 50003,
        mastery_score_bps: 8750,
    };
    let version = Version::new(1, 5, 2);
    let bytes = encode_versioned_value(&competency, version)
        .expect("encode versioned CompetencyFrameworkEntry v1.5.2 failed");
    let (decoded, ver, _consumed): (CompetencyFrameworkEntry, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned CompetencyFrameworkEntry v1.5.2 failed");
    assert_eq!(competency, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 2);
}

#[test]
fn test_attendance_record_remote_roundtrip() {
    let attendance = AttendanceRecord {
        record_id: 100001,
        student_id: 300005,
        course_id: 10003,
        session_date_epoch: 1_742_100_000,
        attendance: AttendanceType::Remote,
        check_in_epoch: 1_742_100_300,
        duration_minutes: 75,
        location: "Zoom Session ID: 985-223-4410".to_string(),
    };
    let bytes = encode_to_vec(&attendance).expect("encode AttendanceRecord Remote failed");
    let (decoded, consumed) = decode_from_slice::<AttendanceRecord>(&bytes)
        .expect("decode AttendanceRecord Remote failed");
    assert_eq!(attendance, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_attendance_record_tardy_versioned_v2_0_1() {
    let attendance = AttendanceRecord {
        record_id: 100002,
        student_id: 300006,
        course_id: 10001,
        session_date_epoch: 1_742_200_000,
        attendance: AttendanceType::Tardy,
        check_in_epoch: 1_742_200_900,
        duration_minutes: 60,
        location: "Lecture Hall B-204".to_string(),
    };
    let version = Version::new(2, 0, 1);
    let bytes = encode_versioned_value(&attendance, version)
        .expect("encode versioned AttendanceRecord Tardy v2.0.1 failed");
    let (decoded, ver, consumed): (AttendanceRecord, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned AttendanceRecord Tardy v2.0.1 failed");
    assert_eq!(attendance, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 1);
    assert!(consumed > 0);
}

#[test]
fn test_discussion_forum_thread_pinned_roundtrip() {
    let thread = DiscussionForumThread {
        thread_id: 110001,
        course_id: 10001,
        author_id: 300007,
        title: "Week 5: Dynamic Programming Strategies".to_string(),
        body_preview: "I found that breaking problems into subproblems and memoizing..."
            .to_string(),
        reply_count: 23,
        upvotes: 47,
        created_at_epoch: 1_741_600_000,
        pinned: true,
        resolved: false,
    };
    let bytes = encode_to_vec(&thread).expect("encode DiscussionForumThread failed");
    let (decoded, consumed) = decode_from_slice::<DiscussionForumThread>(&bytes)
        .expect("decode DiscussionForumThread failed");
    assert_eq!(thread, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_peer_review_assignment_versioned_v1_0_3() {
    let review = PeerReviewAssignment {
        review_id: 120001,
        reviewer_id: 300008,
        author_id: 300009,
        submission_id: 30003,
        score_bps: 8200,
        feedback_text: "Strong methodology section. Consider adding more recent citations to support your claim about distributed consensus.".to_string(),
        completed: true,
        assigned_at_epoch: 1_741_700_000,
        due_at_epoch: 1_742_300_000,
    };
    let version = Version::new(1, 0, 3);
    let bytes = encode_versioned_value(&review, version)
        .expect("encode versioned PeerReviewAssignment v1.0.3 failed");
    let (decoded, ver, consumed): (PeerReviewAssignment, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned PeerReviewAssignment v1.0.3 failed");
    assert_eq!(review, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_plagiarism_detection_high_similarity_roundtrip() {
    let result = PlagiarismDetectionResult {
        scan_id: 130001,
        submission_id: 30004,
        similarity_score_bps: 7800,
        matched_source_count: 5,
        verdict: PlagiarismVerdict::HighSimilarity,
        scanned_at_epoch: 1_742_400_000,
        report_hash: "sha256:plagiarism_report_001_hash".to_string(),
    };
    let bytes = encode_to_vec(&result).expect("encode PlagiarismDetectionResult failed");
    let (decoded, consumed) = decode_from_slice::<PlagiarismDetectionResult>(&bytes)
        .expect("decode PlagiarismDetectionResult failed");
    assert_eq!(result, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_plagiarism_detection_clear_versioned_v4_0_0() {
    let result = PlagiarismDetectionResult {
        scan_id: 130002,
        submission_id: 30005,
        similarity_score_bps: 200,
        matched_source_count: 1,
        verdict: PlagiarismVerdict::Clear,
        scanned_at_epoch: 1_742_500_000,
        report_hash: "sha256:plagiarism_report_002_hash".to_string(),
    };
    let version = Version::new(4, 0, 0);
    let bytes = encode_versioned_value(&result, version)
        .expect("encode versioned PlagiarismDetectionResult Clear v4.0.0 failed");
    let (decoded, ver, _consumed): (PlagiarismDetectionResult, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned PlagiarismDetectionResult Clear v4.0.0 failed");
    assert_eq!(result, decoded);
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_certificate_issuance_micro_credential_roundtrip() {
    let cert = CertificateIssuance {
        certificate_id: 140001,
        recipient_id: 300010,
        credential_type: CredentialType::MicroCredential,
        title: "Machine Learning Foundations".to_string(),
        issuer_org: "COOLJAPAN University".to_string(),
        issued_at_epoch: 1_742_600_000,
        expiry_epoch: Some(1_774_136_000),
        verification_hash: "sha256:cert_verify_mc_001".to_string(),
        revoked: false,
    };
    let bytes = encode_to_vec(&cert).expect("encode CertificateIssuance failed");
    let (decoded, consumed) = decode_from_slice::<CertificateIssuance>(&bytes)
        .expect("decode CertificateIssuance failed");
    assert_eq!(cert, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_certificate_issuance_diploma_versioned_v2_3_0() {
    let cert = CertificateIssuance {
        certificate_id: 140002,
        recipient_id: 300011,
        credential_type: CredentialType::Diploma,
        title: "Bachelor of Science in Computer Engineering".to_string(),
        issuer_org: "COOLJAPAN Institute of Technology".to_string(),
        issued_at_epoch: 1_742_700_000,
        expiry_epoch: None,
        verification_hash: "sha256:cert_verify_diploma_002".to_string(),
        revoked: false,
    };
    let version = Version::new(2, 3, 0);
    let bytes = encode_versioned_value(&cert, version)
        .expect("encode versioned CertificateIssuance Diploma v2.3.0 failed");
    let (decoded, ver, consumed): (CertificateIssuance, Version, usize) =
        decode_versioned_value(&bytes)
            .expect("decode versioned CertificateIssuance Diploma v2.3.0 failed");
    assert_eq!(cert, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_adaptive_learning_recommendation_accepted_roundtrip() {
    let rec = AdaptiveLearningRecommendation {
        recommendation_id: 150001,
        learner_id: 300012,
        recommended_module_id: 80025,
        confidence_score_bps: 9200,
        reason:
            "Learner demonstrates strong grasp of recursion; graph traversal is a natural next step"
                .to_string(),
        predicted_mastery_gain_bps: 1500,
        generated_at_epoch: 1_742_800_000,
        accepted: true,
    };
    let bytes = encode_to_vec(&rec).expect("encode AdaptiveLearningRecommendation failed");
    let (decoded, consumed) = decode_from_slice::<AdaptiveLearningRecommendation>(&bytes)
        .expect("decode AdaptiveLearningRecommendation failed");
    assert_eq!(rec, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_scorm_learning_record_versioned_v1_1_0() {
    let record = ScormLearningRecord {
        record_id: 160001,
        learner_id: 300013,
        sco_id: "urn:sco:algorithms-module-07".to_string(),
        verb: "completed".to_string(),
        score_raw: 87,
        score_max: 100,
        completion_status: "completed".to_string(),
        success_status: "passed".to_string(),
        session_duration_secs: 2_700,
        timestamp_epoch: 1_742_900_000,
    };
    let version = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&record, version)
        .expect("encode versioned ScormLearningRecord v1.1.0 failed");
    let (decoded, ver, consumed): (ScormLearningRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned ScormLearningRecord v1.1.0 failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_faculty_workload_allocation_overload_roundtrip() {
    let alloc = FacultyWorkloadAllocation {
        allocation_id: 170001,
        faculty_id: 50004,
        semester_code: "2026-SPRING".to_string(),
        teaching_hours: 18,
        research_hours: 10,
        admin_hours: 6,
        course_count: 4,
        advisee_count: 15,
        overload: true,
    };
    let bytes = encode_to_vec(&alloc).expect("encode FacultyWorkloadAllocation failed");
    let (decoded, consumed) = decode_from_slice::<FacultyWorkloadAllocation>(&bytes)
        .expect("decode FacultyWorkloadAllocation failed");
    assert_eq!(alloc, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_faculty_workload_version_upgrade_v1_to_v2() {
    let alloc = FacultyWorkloadAllocation {
        allocation_id: 170002,
        faculty_id: 50005,
        semester_code: "2025-FALL".to_string(),
        teaching_hours: 12,
        research_hours: 15,
        admin_hours: 3,
        course_count: 3,
        advisee_count: 8,
        overload: false,
    };

    let v1 = Version::new(1, 0, 0);
    let bytes_v1 = encode_versioned_value(&alloc, v1)
        .expect("encode versioned FacultyWorkloadAllocation v1.0.0 failed");
    let (decoded_v1, ver_v1, _consumed_v1): (FacultyWorkloadAllocation, Version, usize) =
        decode_versioned_value(&bytes_v1)
            .expect("decode versioned FacultyWorkloadAllocation v1.0.0 failed");
    assert_eq!(alloc, decoded_v1);
    assert_eq!(ver_v1.major, 1);

    let v2 = Version::new(2, 0, 0);
    let bytes_v2 = encode_versioned_value(&decoded_v1, v2)
        .expect("re-encode versioned FacultyWorkloadAllocation v2.0.0 failed");
    let (decoded_v2, ver_v2, consumed_v2): (FacultyWorkloadAllocation, Version, usize) =
        decode_versioned_value(&bytes_v2)
            .expect("decode versioned FacultyWorkloadAllocation v2.0.0 failed");
    assert_eq!(alloc, decoded_v2);
    assert_eq!(ver_v2.major, 2);
    assert_eq!(ver_v2.minor, 0);
    assert!(consumed_v2 > 0);
}
