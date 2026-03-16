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

// --- Domain types for Education Technology / LMS ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CourseCurriculum {
    course_id: u64,
    title: String,
    department: String,
    credit_hours: u8,
    modules: Vec<CurriculumModule>,
    prerequisites: Vec<String>,
    is_published: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CurriculumModule {
    module_id: u32,
    title: String,
    sequence_order: u16,
    estimated_minutes: u32,
    learning_objectives: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StudentEnrollment {
    enrollment_id: u64,
    student_id: u64,
    course_id: u64,
    section_code: String,
    semester: String,
    year: u16,
    status: EnrollmentStatus,
    enrolled_timestamp_secs: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum EnrollmentStatus {
    Active,
    Withdrawn,
    Completed,
    Auditing,
    Waitlisted,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AssignmentSubmission {
    submission_id: u64,
    assignment_id: u64,
    student_id: u64,
    submitted_at_secs: u64,
    file_names: Vec<String>,
    rubric_scores: Vec<RubricScore>,
    total_points: u32,
    max_points: u32,
    late_penalty_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RubricScore {
    criterion: String,
    points_earned: u32,
    points_possible: u32,
    feedback: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct QuizQuestionBank {
    bank_id: u64,
    title: String,
    subject_tag: String,
    questions: Vec<QuizQuestion>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum QuizQuestion {
    MultipleChoice {
        question_id: u64,
        stem: String,
        choices: Vec<String>,
        correct_index: u8,
        points: u32,
    },
    Essay {
        question_id: u64,
        prompt: String,
        word_limit: Option<u32>,
        points: u32,
    },
    Matching {
        question_id: u64,
        instruction: String,
        left_items: Vec<String>,
        right_items: Vec<String>,
        correct_pairs: Vec<(u8, u8)>,
        points: u32,
    },
    TrueFalse {
        question_id: u64,
        statement: String,
        correct_answer: bool,
        points: u32,
    },
    FillInBlank {
        question_id: u64,
        stem_with_blank: String,
        acceptable_answers: Vec<String>,
        case_sensitive: bool,
        points: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GradeBookEntry {
    student_id: u64,
    course_id: u64,
    category: String,
    item_name: String,
    points_earned: f64,
    points_possible: f64,
    weight_pct: f64,
    graded_at_secs: u64,
    grader_id: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AttendanceRecord {
    record_id: u64,
    student_id: u64,
    session_id: u64,
    course_id: u64,
    date_ymd: String,
    status: AttendanceStatus,
    check_in_secs: Option<u64>,
    check_out_secs: Option<u64>,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AttendanceStatus {
    Present,
    Absent,
    Tardy,
    Excused,
    Remote,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LearningPathProgression {
    path_id: u64,
    student_id: u64,
    path_title: String,
    milestones: Vec<Milestone>,
    current_milestone_idx: u32,
    overall_progress_pct: f64,
    started_at_secs: u64,
    last_activity_secs: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Milestone {
    milestone_id: u32,
    title: String,
    description: String,
    required_score: u32,
    achieved_score: Option<u32>,
    completed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AdaptiveTestingParams {
    item_id: u64,
    difficulty_b: f64,
    discrimination_a: f64,
    guessing_c: f64,
    item_information_at_theta_zero: f64,
    content_domain: String,
    calibration_sample_size: u32,
    standard_error_b: f64,
    model_fit_chi_sq: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LtiToolIntegration {
    tool_id: u64,
    tool_name: String,
    consumer_key: String,
    launch_url: String,
    lti_version: String,
    custom_params: Vec<(String, String)>,
    is_enabled: bool,
    supported_placements: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ScormPackageMetadata {
    package_id: u64,
    manifest_identifier: String,
    scorm_version: String,
    title: String,
    launch_resource: String,
    mastery_score: Option<f64>,
    max_time_allowed_secs: Option<u64>,
    organizations: Vec<ScormOrganization>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ScormOrganization {
    org_id: String,
    title: String,
    item_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PlagiarismResult {
    report_id: u64,
    submission_id: u64,
    student_id: u64,
    similarity_pct: f64,
    matched_sources: Vec<MatchedSource>,
    processing_time_ms: u64,
    status: PlagiarismStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MatchedSource {
    source_name: String,
    source_url: String,
    overlap_pct: f64,
    matched_word_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PlagiarismStatus {
    Clean,
    LowRisk,
    MediumRisk,
    HighRisk,
    ManualReviewRequired,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DiscussionThread {
    thread_id: u64,
    forum_id: u64,
    course_id: u64,
    author_id: u64,
    title: String,
    body: String,
    created_at_secs: u64,
    replies: Vec<DiscussionReply>,
    is_pinned: bool,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DiscussionReply {
    reply_id: u64,
    author_id: u64,
    body: String,
    created_at_secs: u64,
    upvotes: u32,
    is_endorsed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StudentLearningAnalytics {
    student_id: u64,
    course_id: u64,
    total_time_spent_secs: u64,
    pages_viewed: u32,
    videos_watched: u32,
    avg_quiz_score_pct: f64,
    assignment_completion_rate: f64,
    login_count: u32,
    last_login_secs: u64,
    risk_level: RiskLevel,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RiskLevel {
    OnTrack,
    AtRisk,
    HighRisk,
    Excelling,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PeerReviewAssignment {
    review_id: u64,
    reviewer_student_id: u64,
    reviewee_student_id: u64,
    assignment_id: u64,
    criteria_scores: Vec<PeerCriterionScore>,
    qualitative_feedback: String,
    completed: bool,
    anonymous: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PeerCriterionScore {
    criterion_name: String,
    score: u8,
    max_score: u8,
    comment: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CompetencyMastery {
    student_id: u64,
    competency_id: u64,
    competency_name: String,
    domain: String,
    mastery_level: MasteryLevel,
    evidence_count: u32,
    last_assessed_secs: u64,
    threshold_score: f64,
    current_score: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MasteryLevel {
    NotStarted,
    Emerging,
    Developing,
    Proficient,
    Advanced,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContentRelease {
    release_id: u64,
    course_id: u64,
    content_type: ContentType,
    title: String,
    available_from_secs: u64,
    available_until_secs: Option<u64>,
    prerequisite_module_ids: Vec<u32>,
    min_score_to_unlock: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ContentType {
    Lecture,
    Lab,
    Quiz,
    Assignment,
    Supplemental,
    ExternalResource,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InstructorFeedbackTemplate {
    template_id: u64,
    instructor_id: u64,
    name: String,
    category: String,
    body_template: String,
    placeholders: Vec<String>,
    usage_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GradingScheme {
    scheme_id: u64,
    course_id: u64,
    name: String,
    tiers: Vec<GradeTier>,
    uses_plus_minus: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GradeTier {
    letter: String,
    min_pct: f64,
    max_pct: f64,
    gpa_points: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CatEstimate {
    examinee_id: u64,
    theta_estimate: f64,
    standard_error: f64,
    items_administered: u32,
    responses: Vec<ItemResponse>,
    termination_reason: TerminationReason,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ItemResponse {
    item_id: u64,
    response: u8,
    correct: bool,
    response_time_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TerminationReason {
    MaxItemsReached,
    SePrecisionMet,
    TimeExpired,
    AllItemsExhausted,
}

// ---- Tests ----

#[test]
fn test_course_curriculum_roundtrip() {
    let val = CourseCurriculum {
        course_id: 10200,
        title: "Introduction to Data Structures".into(),
        department: "Computer Science".into(),
        credit_hours: 3,
        modules: vec![
            CurriculumModule {
                module_id: 1,
                title: "Arrays and Linked Lists".into(),
                sequence_order: 1,
                estimated_minutes: 180,
                learning_objectives: vec![
                    "Explain array memory layout".into(),
                    "Implement singly linked list".into(),
                ],
            },
            CurriculumModule {
                module_id: 2,
                title: "Trees and Graphs".into(),
                sequence_order: 2,
                estimated_minutes: 240,
                learning_objectives: vec![
                    "Traverse binary trees".into(),
                    "Implement BFS and DFS".into(),
                    "Analyze time complexity".into(),
                ],
            },
        ],
        prerequisites: vec!["CS101".into(), "MATH120".into()],
        is_published: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode course curriculum");
    let (decoded, _): (CourseCurriculum, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode course curriculum");
    assert_eq!(val, decoded);
}

#[test]
fn test_student_enrollment_active() {
    let val = StudentEnrollment {
        enrollment_id: 990001,
        student_id: 50432,
        course_id: 10200,
        section_code: "SEC-A".into(),
        semester: "Fall".into(),
        year: 2025,
        status: EnrollmentStatus::Active,
        enrolled_timestamp_secs: 1693500000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode enrollment");
    let (decoded, _): (StudentEnrollment, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode enrollment");
    assert_eq!(val, decoded);
}

#[test]
fn test_assignment_submission_with_rubric() {
    let val = AssignmentSubmission {
        submission_id: 7700001,
        assignment_id: 3010,
        student_id: 50432,
        submitted_at_secs: 1694000000,
        file_names: vec!["report.pdf".into(), "code.zip".into()],
        rubric_scores: vec![
            RubricScore {
                criterion: "Correctness".into(),
                points_earned: 18,
                points_possible: 20,
                feedback: "Minor off-by-one error in merge function".into(),
            },
            RubricScore {
                criterion: "Code Quality".into(),
                points_earned: 14,
                points_possible: 15,
                feedback: "Good variable naming, consider extracting helper functions".into(),
            },
            RubricScore {
                criterion: "Documentation".into(),
                points_earned: 10,
                points_possible: 15,
                feedback: "Missing complexity analysis for delete operation".into(),
            },
        ],
        total_points: 42,
        max_points: 50,
        late_penalty_pct: 0,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode assignment submission");
    let (decoded, _): (AssignmentSubmission, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode assignment submission");
    assert_eq!(val, decoded);
}

#[test]
fn test_quiz_question_bank_multiple_choice() {
    let val = QuizQuestionBank {
        bank_id: 5001,
        title: "Midterm Review - Algorithms".into(),
        subject_tag: "algorithms".into(),
        questions: vec![
            QuizQuestion::MultipleChoice {
                question_id: 1,
                stem: "What is the average-case time complexity of quicksort?".into(),
                choices: vec![
                    "O(n)".into(),
                    "O(n log n)".into(),
                    "O(n^2)".into(),
                    "O(log n)".into(),
                ],
                correct_index: 1,
                points: 5,
            },
            QuizQuestion::TrueFalse {
                question_id: 2,
                statement: "A binary search tree always has O(log n) lookup time".into(),
                correct_answer: false,
                points: 3,
            },
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode quiz bank");
    let (decoded, _): (QuizQuestionBank, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode quiz bank");
    assert_eq!(val, decoded);
}

#[test]
fn test_quiz_question_bank_essay_and_matching() {
    let val = QuizQuestionBank {
        bank_id: 5002,
        title: "Final Exam - Literature".into(),
        subject_tag: "literature".into(),
        questions: vec![
            QuizQuestion::Essay {
                question_id: 10,
                prompt: "Discuss the symbolism of the green light in The Great Gatsby".into(),
                word_limit: Some(500),
                points: 25,
            },
            QuizQuestion::Matching {
                question_id: 11,
                instruction: "Match the author to their work".into(),
                left_items: vec!["Shakespeare".into(), "Dickens".into(), "Austen".into()],
                right_items: vec![
                    "Pride and Prejudice".into(),
                    "Hamlet".into(),
                    "Oliver Twist".into(),
                ],
                correct_pairs: vec![(0, 1), (1, 2), (2, 0)],
                points: 15,
            },
            QuizQuestion::FillInBlank {
                question_id: 12,
                stem_with_blank: "The novel '1984' was written by George ___".into(),
                acceptable_answers: vec!["Orwell".into(), "orwell".into()],
                case_sensitive: false,
                points: 5,
            },
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode essay/matching bank");
    let (decoded, _): (QuizQuestionBank, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode essay/matching bank");
    assert_eq!(val, decoded);
}

#[test]
fn test_grade_book_entry_with_grader() {
    let val = GradeBookEntry {
        student_id: 50432,
        course_id: 10200,
        category: "Homework".into(),
        item_name: "HW3 - Hash Tables".into(),
        points_earned: 87.5,
        points_possible: 100.0,
        weight_pct: 20.0,
        graded_at_secs: 1694500000,
        grader_id: Some(2001),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode grade book entry");
    let (decoded, _): (GradeBookEntry, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode grade book entry");
    assert_eq!(val, decoded);
}

#[test]
fn test_attendance_record_present() {
    let val = AttendanceRecord {
        record_id: 440001,
        student_id: 50432,
        session_id: 8800,
        course_id: 10200,
        date_ymd: "2025-10-15".into(),
        status: AttendanceStatus::Present,
        check_in_secs: Some(1697360400),
        check_out_secs: Some(1697367600),
        notes: String::new(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode attendance present");
    let (decoded, _): (AttendanceRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode attendance present");
    assert_eq!(val, decoded);
}

#[test]
fn test_attendance_record_excused_absence() {
    let val = AttendanceRecord {
        record_id: 440090,
        student_id: 50700,
        session_id: 8810,
        course_id: 10200,
        date_ymd: "2025-10-17".into(),
        status: AttendanceStatus::Excused,
        check_in_secs: None,
        check_out_secs: None,
        notes: "Medical appointment - documentation provided".into(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode excused absence");
    let (decoded, _): (AttendanceRecord, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode excused absence");
    assert_eq!(val, decoded);
}

#[test]
fn test_learning_path_progression() {
    let val = LearningPathProgression {
        path_id: 3300,
        student_id: 50432,
        path_title: "Full-Stack Web Development".into(),
        milestones: vec![
            Milestone {
                milestone_id: 1,
                title: "HTML/CSS Foundations".into(),
                description: "Build responsive layouts with semantic HTML".into(),
                required_score: 80,
                achieved_score: Some(92),
                completed: true,
            },
            Milestone {
                milestone_id: 2,
                title: "JavaScript Essentials".into(),
                description: "DOM manipulation, async patterns, ES6+ features".into(),
                required_score: 80,
                achieved_score: Some(85),
                completed: true,
            },
            Milestone {
                milestone_id: 3,
                title: "Backend with Rust".into(),
                description: "Build REST APIs with Actix-web".into(),
                required_score: 80,
                achieved_score: None,
                completed: false,
            },
        ],
        current_milestone_idx: 2,
        overall_progress_pct: 66.7,
        started_at_secs: 1688000000,
        last_activity_secs: 1694800000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode learning path");
    let (decoded, _): (LearningPathProgression, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode learning path");
    assert_eq!(val, decoded);
}

#[test]
fn test_adaptive_testing_irt_params() {
    let val = AdaptiveTestingParams {
        item_id: 60001,
        difficulty_b: 1.25,
        discrimination_a: 0.85,
        guessing_c: 0.2,
        item_information_at_theta_zero: 0.144,
        content_domain: "Algebra".into(),
        calibration_sample_size: 1500,
        standard_error_b: 0.12,
        model_fit_chi_sq: 3.42,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode IRT params");
    let (decoded, _): (AdaptiveTestingParams, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode IRT params");
    assert_eq!(val, decoded);
}

#[test]
fn test_lti_tool_integration() {
    let val = LtiToolIntegration {
        tool_id: 7700,
        tool_name: "Virtual Lab Environment".into(),
        consumer_key: "vlab-consumer-2025".into(),
        launch_url: "https://vlab.example.com/lti/launch".into(),
        lti_version: "1.3".into(),
        custom_params: vec![
            ("lab_type".into(), "chemistry".into()),
            ("safety_mode".into(), "enabled".into()),
            ("max_concurrent_users".into(), "35".into()),
        ],
        is_enabled: true,
        supported_placements: vec!["course_navigation".into(), "assignment_selection".into()],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode LTI integration");
    let (decoded, _): (LtiToolIntegration, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode LTI integration");
    assert_eq!(val, decoded);
}

#[test]
fn test_scorm_package_metadata() {
    let val = ScormPackageMetadata {
        package_id: 88010,
        manifest_identifier: "com.example.scorm.safety_training_v2".into(),
        scorm_version: "2004 4th Edition".into(),
        title: "Workplace Safety Compliance Training".into(),
        launch_resource: "content/index.html".into(),
        mastery_score: Some(80.0),
        max_time_allowed_secs: Some(7200),
        organizations: vec![
            ScormOrganization {
                org_id: "ORG-001".into(),
                title: "Safety Fundamentals".into(),
                item_count: 12,
            },
            ScormOrganization {
                org_id: "ORG-002".into(),
                title: "Emergency Procedures".into(),
                item_count: 8,
            },
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode SCORM metadata");
    let (decoded, _): (ScormPackageMetadata, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode SCORM metadata");
    assert_eq!(val, decoded);
}

#[test]
fn test_plagiarism_result_clean() {
    let val = PlagiarismResult {
        report_id: 110001,
        submission_id: 7700001,
        student_id: 50432,
        similarity_pct: 3.2,
        matched_sources: vec![MatchedSource {
            source_name: "Wikipedia - Binary Tree".into(),
            source_url: "https://en.wikipedia.org/wiki/Binary_tree".into(),
            overlap_pct: 2.1,
            matched_word_count: 15,
        }],
        processing_time_ms: 4500,
        status: PlagiarismStatus::Clean,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode clean plagiarism result");
    let (decoded, _): (PlagiarismResult, _) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode clean plagiarism result");
    assert_eq!(val, decoded);
}

#[test]
fn test_plagiarism_result_high_risk() {
    let val = PlagiarismResult {
        report_id: 110099,
        submission_id: 7700050,
        student_id: 50888,
        similarity_pct: 67.8,
        matched_sources: vec![
            MatchedSource {
                source_name: "Chegg Solutions - DS Assignment 3".into(),
                source_url: "https://chegg.example.com/solution/12345".into(),
                overlap_pct: 42.3,
                matched_word_count: 890,
            },
            MatchedSource {
                source_name: "GitHub repository - student-solutions".into(),
                source_url: "https://github.example.com/solutions/ds-hw3".into(),
                overlap_pct: 25.5,
                matched_word_count: 540,
            },
        ],
        processing_time_ms: 8200,
        status: PlagiarismStatus::HighRisk,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode high-risk plagiarism");
    let (decoded, _): (PlagiarismResult, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode high-risk plagiarism");
    assert_eq!(val, decoded);
}

#[test]
fn test_discussion_thread_with_replies() {
    let val = DiscussionThread {
        thread_id: 220001,
        forum_id: 5500,
        course_id: 10200,
        author_id: 50432,
        title: "Clarification on amortized analysis of dynamic arrays".into(),
        body: "In lecture, we discussed amortized O(1) for push operations. \
               Can someone explain the accounting method vs the potential method?"
            .into(),
        created_at_secs: 1694200000,
        replies: vec![
            DiscussionReply {
                reply_id: 330001,
                author_id: 50500,
                body: "The accounting method assigns a cost to each operation that may \
                       differ from the actual cost. For dynamic arrays, we charge 3 units \
                       per push: 1 for the insert, 2 saved for future copies."
                    .into(),
                created_at_secs: 1694210000,
                upvotes: 12,
                is_endorsed: true,
            },
            DiscussionReply {
                reply_id: 330002,
                author_id: 2001,
                body: "Great explanation! The potential method uses a potential function \
                       that maps the state of the data structure to a non-negative value."
                    .into(),
                created_at_secs: 1694220000,
                upvotes: 8,
                is_endorsed: true,
            },
        ],
        is_pinned: true,
        tags: vec![
            "amortized-analysis".into(),
            "dynamic-arrays".into(),
            "midterm-review".into(),
        ],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode discussion thread");
    let (decoded, _): (DiscussionThread, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode discussion thread");
    assert_eq!(val, decoded);
}

#[test]
fn test_student_learning_analytics_at_risk() {
    let val = StudentLearningAnalytics {
        student_id: 50700,
        course_id: 10200,
        total_time_spent_secs: 18000,
        pages_viewed: 45,
        videos_watched: 3,
        avg_quiz_score_pct: 52.3,
        assignment_completion_rate: 0.6,
        login_count: 12,
        last_login_secs: 1693800000,
        risk_level: RiskLevel::AtRisk,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode at-risk analytics");
    let (decoded, _): (StudentLearningAnalytics, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode at-risk analytics");
    assert_eq!(val, decoded);
}

#[test]
fn test_peer_review_assignment() {
    let val = PeerReviewAssignment {
        review_id: 99001,
        reviewer_student_id: 50432,
        reviewee_student_id: 50500,
        assignment_id: 3010,
        criteria_scores: vec![
            PeerCriterionScore {
                criterion_name: "Clarity of Explanation".into(),
                score: 4,
                max_score: 5,
                comment: "Well-structured argument with clear transitions".into(),
            },
            PeerCriterionScore {
                criterion_name: "Depth of Analysis".into(),
                score: 3,
                max_score: 5,
                comment: "Could explore counterarguments more thoroughly".into(),
            },
            PeerCriterionScore {
                criterion_name: "Use of Evidence".into(),
                score: 5,
                max_score: 5,
                comment: "Excellent use of primary sources".into(),
            },
        ],
        qualitative_feedback: "Overall strong submission. The introduction sets up \
                               the thesis well, but the conclusion could be stronger."
            .into(),
        completed: true,
        anonymous: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode peer review");
    let (decoded, _): (PeerReviewAssignment, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode peer review");
    assert_eq!(val, decoded);
}

#[test]
fn test_competency_mastery_proficient() {
    let val = CompetencyMastery {
        student_id: 50432,
        competency_id: 4400,
        competency_name: "Algorithm Design and Analysis".into(),
        domain: "Computer Science Foundations".into(),
        mastery_level: MasteryLevel::Proficient,
        evidence_count: 7,
        last_assessed_secs: 1694700000,
        threshold_score: 75.0,
        current_score: 88.5,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode competency mastery");
    let (decoded, _): (CompetencyMastery, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode competency mastery");
    assert_eq!(val, decoded);
}

#[test]
fn test_content_release_with_prerequisites() {
    let val = ContentRelease {
        release_id: 66001,
        course_id: 10200,
        content_type: ContentType::Lab,
        title: "Lab 5 - Implementing Red-Black Trees".into(),
        available_from_secs: 1694400000,
        available_until_secs: Some(1695600000),
        prerequisite_module_ids: vec![1, 2],
        min_score_to_unlock: Some(70),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode content release");
    let (decoded, _): (ContentRelease, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode content release");
    assert_eq!(val, decoded);
}

#[test]
fn test_grading_scheme_with_tiers() {
    let val = GradingScheme {
        scheme_id: 1100,
        course_id: 10200,
        name: "Standard Letter Grade".into(),
        tiers: vec![
            GradeTier {
                letter: "A".into(),
                min_pct: 93.0,
                max_pct: 100.0,
                gpa_points: 4.0,
            },
            GradeTier {
                letter: "A-".into(),
                min_pct: 90.0,
                max_pct: 92.99,
                gpa_points: 3.7,
            },
            GradeTier {
                letter: "B+".into(),
                min_pct: 87.0,
                max_pct: 89.99,
                gpa_points: 3.3,
            },
            GradeTier {
                letter: "B".into(),
                min_pct: 83.0,
                max_pct: 86.99,
                gpa_points: 3.0,
            },
            GradeTier {
                letter: "B-".into(),
                min_pct: 80.0,
                max_pct: 82.99,
                gpa_points: 2.7,
            },
            GradeTier {
                letter: "C+".into(),
                min_pct: 77.0,
                max_pct: 79.99,
                gpa_points: 2.3,
            },
            GradeTier {
                letter: "C".into(),
                min_pct: 73.0,
                max_pct: 76.99,
                gpa_points: 2.0,
            },
            GradeTier {
                letter: "D".into(),
                min_pct: 60.0,
                max_pct: 72.99,
                gpa_points: 1.0,
            },
            GradeTier {
                letter: "F".into(),
                min_pct: 0.0,
                max_pct: 59.99,
                gpa_points: 0.0,
            },
        ],
        uses_plus_minus: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode grading scheme");
    let (decoded, _): (GradingScheme, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode grading scheme");
    assert_eq!(val, decoded);
}

#[test]
fn test_cat_estimate_with_item_responses() {
    let val = CatEstimate {
        examinee_id: 50432,
        theta_estimate: 1.35,
        standard_error: 0.28,
        items_administered: 25,
        responses: vec![
            ItemResponse {
                item_id: 60001,
                response: 1,
                correct: true,
                response_time_ms: 45000,
            },
            ItemResponse {
                item_id: 60015,
                response: 0,
                correct: false,
                response_time_ms: 62000,
            },
            ItemResponse {
                item_id: 60022,
                response: 1,
                correct: true,
                response_time_ms: 38000,
            },
            ItemResponse {
                item_id: 60044,
                response: 1,
                correct: true,
                response_time_ms: 29000,
            },
        ],
        termination_reason: TerminationReason::SePrecisionMet,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode CAT estimate");
    let (decoded, _): (CatEstimate, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode CAT estimate");
    assert_eq!(val, decoded);
}

#[test]
fn test_instructor_feedback_template() {
    let val = InstructorFeedbackTemplate {
        template_id: 5500,
        instructor_id: 2001,
        name: "Late Submission Notice".into(),
        category: "Administrative".into(),
        body_template: "Dear {student_name}, your submission for {assignment_name} \
                        was received {days_late} day(s) after the deadline. A {penalty_pct}% \
                        penalty has been applied per the course syllabus."
            .into(),
        placeholders: vec![
            "student_name".into(),
            "assignment_name".into(),
            "days_late".into(),
            "penalty_pct".into(),
        ],
        usage_count: 47,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode feedback template");
    let (decoded, _): (InstructorFeedbackTemplate, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode feedback template");
    assert_eq!(val, decoded);
}
