//! Tests for Education / Learning Management System — advanced enum roundtrip coverage.
//!
//! Domain types model a simplified LMS with course lifecycle states, assignment types,
//! grading scales, content types, individual assignments, and full course records
//! with nested vecs and optional fields.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CourseStatus {
    Draft,
    Published,
    Archived,
    Deleted,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AssignmentType {
    Quiz,
    Essay,
    Project,
    PeerReview,
    Lab,
    Presentation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GradeScale {
    Letter,
    Percentage,
    PassFail,
    Points { max: u32 },
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContentType {
    Video { duration_s: u32 },
    Text { word_count: u32 },
    Interactive { modules: u8 },
    Quiz { questions: u16 },
    File { size_bytes: u64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Assignment {
    id: u64,
    title: String,
    assignment_type: AssignmentType,
    grade_scale: GradeScale,
    due_date_unix: u64,
    points: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Course {
    id: u64,
    title: String,
    status: CourseStatus,
    assignments: Vec<Assignment>,
    content: Vec<ContentType>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_assignment(
    id: u64,
    title: &str,
    assignment_type: AssignmentType,
    grade_scale: GradeScale,
    due_date_unix: u64,
    points: u32,
) -> Assignment {
    Assignment {
        id,
        title: title.to_string(),
        assignment_type,
        grade_scale,
        due_date_unix,
        points,
    }
}

fn make_course(
    id: u64,
    title: &str,
    status: CourseStatus,
    assignments: Vec<Assignment>,
    content: Vec<ContentType>,
) -> Course {
    Course {
        id,
        title: title.to_string(),
        status,
        assignments,
        content,
    }
}

// ---------------------------------------------------------------------------
// Test 1: CourseStatus — all 4 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_status_all_variants_roundtrip() {
    let statuses = vec![
        CourseStatus::Draft,
        CourseStatus::Published,
        CourseStatus::Archived,
        CourseStatus::Deleted,
    ];

    for status in &statuses {
        let bytes = encode_to_vec(status).expect("encode CourseStatus");
        let (decoded, consumed): (CourseStatus, usize) =
            decode_from_slice(&bytes).expect("decode CourseStatus");
        assert_eq!(status, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for CourseStatus"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 2: CourseStatus — discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_course_status_discriminant_uniqueness() {
    let statuses = vec![
        CourseStatus::Draft,
        CourseStatus::Published,
        CourseStatus::Archived,
        CourseStatus::Deleted,
    ];

    let encodings: Vec<Vec<u8>> = statuses
        .iter()
        .map(|s| encode_to_vec(s).expect("encode CourseStatus for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "CourseStatus variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 3: AssignmentType — all 6 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_assignment_type_all_variants_roundtrip() {
    let types = vec![
        AssignmentType::Quiz,
        AssignmentType::Essay,
        AssignmentType::Project,
        AssignmentType::PeerReview,
        AssignmentType::Lab,
        AssignmentType::Presentation,
    ];

    for at in &types {
        let bytes = encode_to_vec(at).expect("encode AssignmentType");
        let (decoded, consumed): (AssignmentType, usize) =
            decode_from_slice(&bytes).expect("decode AssignmentType");
        assert_eq!(at, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for AssignmentType"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: AssignmentType — discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_assignment_type_discriminant_uniqueness() {
    let types = vec![
        AssignmentType::Quiz,
        AssignmentType::Essay,
        AssignmentType::Project,
        AssignmentType::PeerReview,
        AssignmentType::Lab,
        AssignmentType::Presentation,
    ];

    let encodings: Vec<Vec<u8>> = types
        .iter()
        .map(|at| encode_to_vec(at).expect("encode AssignmentType for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "AssignmentType variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 5: GradeScale — all 5 variants roundtrip (including Points and Custom)
// ---------------------------------------------------------------------------

#[test]
fn test_grade_scale_all_variants_roundtrip() {
    let scales = vec![
        GradeScale::Letter,
        GradeScale::Percentage,
        GradeScale::PassFail,
        GradeScale::Points { max: 100 },
        GradeScale::Custom("Rubric-4pt".to_string()),
    ];

    for scale in &scales {
        let bytes = encode_to_vec(scale).expect("encode GradeScale");
        let (decoded, consumed): (GradeScale, usize) =
            decode_from_slice(&bytes).expect("decode GradeScale");
        assert_eq!(scale, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for GradeScale"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 6: GradeScale::Points — named field with various max values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_grade_scale_points_named_field_roundtrip() {
    let point_scales = vec![
        GradeScale::Points { max: 0 },
        GradeScale::Points { max: 10 },
        GradeScale::Points { max: 100 },
        GradeScale::Points { max: 1_000 },
        GradeScale::Points { max: u32::MAX },
    ];

    for scale in &point_scales {
        let bytes = encode_to_vec(scale).expect("encode GradeScale::Points");
        let (decoded, consumed): (GradeScale, usize) =
            decode_from_slice(&bytes).expect("decode GradeScale::Points");
        assert_eq!(scale, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for GradeScale::Points"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 7: GradeScale::Custom — various string payloads roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_grade_scale_custom_string_roundtrip() {
    let custom_scales = vec![
        GradeScale::Custom(String::new()),
        GradeScale::Custom("Pass/Merit/Distinction".to_string()),
        GradeScale::Custom("1-10 scale".to_string()),
        GradeScale::Custom("Competency-based assessment framework v2".to_string()),
        GradeScale::Custom("A+/A/A-/B+/B/B-/C+/C/C-/D/F".to_string()),
    ];

    for scale in &custom_scales {
        let bytes = encode_to_vec(scale).expect("encode GradeScale::Custom");
        let (decoded, consumed): (GradeScale, usize) =
            decode_from_slice(&bytes).expect("decode GradeScale::Custom");
        assert_eq!(scale, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for GradeScale::Custom"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 8: ContentType — all 5 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_content_type_all_variants_roundtrip() {
    let contents = vec![
        ContentType::Video { duration_s: 3600 },
        ContentType::Text { word_count: 2500 },
        ContentType::Interactive { modules: 8 },
        ContentType::Quiz { questions: 50 },
        ContentType::File {
            size_bytes: 4_194_304,
        },
    ];

    for ct in &contents {
        let bytes = encode_to_vec(ct).expect("encode ContentType");
        let (decoded, consumed): (ContentType, usize) =
            decode_from_slice(&bytes).expect("decode ContentType");
        assert_eq!(ct, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for ContentType"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 9: ContentType — discriminant uniqueness across all 5 variants
// ---------------------------------------------------------------------------

#[test]
fn test_content_type_discriminant_uniqueness() {
    let contents = vec![
        ContentType::Video { duration_s: 100 },
        ContentType::Text { word_count: 100 },
        ContentType::Interactive { modules: 5 },
        ContentType::Quiz { questions: 100 },
        ContentType::File { size_bytes: 100 },
    ];

    let encodings: Vec<Vec<u8>> = contents
        .iter()
        .map(|c| encode_to_vec(c).expect("encode ContentType for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "ContentType variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 10: ContentType::Video — boundary duration values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_content_type_video_boundary_values_roundtrip() {
    let videos = vec![
        ContentType::Video { duration_s: 0 },
        ContentType::Video { duration_s: 1 },
        ContentType::Video { duration_s: 3_600 },
        ContentType::Video { duration_s: 86_400 },
        ContentType::Video {
            duration_s: u32::MAX,
        },
    ];

    for ct in &videos {
        let bytes = encode_to_vec(ct).expect("encode ContentType::Video boundary");
        let (decoded, consumed): (ContentType, usize) =
            decode_from_slice(&bytes).expect("decode ContentType::Video boundary");
        assert_eq!(ct, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal ContentType::Video boundary encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 11: ContentType::File — large size_bytes values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_content_type_file_large_size_roundtrip() {
    let files = vec![
        ContentType::File { size_bytes: 0 },
        ContentType::File { size_bytes: 1_024 },
        ContentType::File {
            size_bytes: 1_073_741_824,
        }, // 1 GiB
        ContentType::File {
            size_bytes: 10_737_418_240,
        }, // 10 GiB
        ContentType::File {
            size_bytes: u64::MAX,
        },
    ];

    for ct in &files {
        let bytes = encode_to_vec(ct).expect("encode ContentType::File large");
        let (decoded, consumed): (ContentType, usize) =
            decode_from_slice(&bytes).expect("decode ContentType::File large");
        assert_eq!(ct, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal ContentType::File large encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 12: Assignment — struct with nested enum fields roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_assignment_nested_enums_roundtrip() {
    let assignments = vec![
        make_assignment(
            1,
            "Midterm Essay",
            AssignmentType::Essay,
            GradeScale::Letter,
            1_700_000_000,
            0,
        ),
        make_assignment(
            2,
            "Final Exam",
            AssignmentType::Quiz,
            GradeScale::Percentage,
            1_710_000_000,
            100,
        ),
        make_assignment(
            3,
            "Group Project",
            AssignmentType::Project,
            GradeScale::Points { max: 500 },
            1_720_000_000,
            500,
        ),
        make_assignment(
            4,
            "Peer Code Review",
            AssignmentType::PeerReview,
            GradeScale::PassFail,
            1_730_000_000,
            1,
        ),
        make_assignment(
            5,
            "Chemistry Lab Report",
            AssignmentType::Lab,
            GradeScale::Custom("Excellent/Satisfactory/Unsatisfactory".to_string()),
            1_740_000_000,
            50,
        ),
        make_assignment(
            6,
            "Final Presentation",
            AssignmentType::Presentation,
            GradeScale::Points { max: 200 },
            1_750_000_000,
            200,
        ),
    ];

    for assignment in &assignments {
        let bytes = encode_to_vec(assignment).expect("encode Assignment nested enums");
        let (decoded, consumed): (Assignment, usize) =
            decode_from_slice(&bytes).expect("decode Assignment nested enums");
        assert_eq!(assignment, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal Assignment encoding length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 13: Course — published course with mixed content and assignments roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_published_with_content_and_assignments_roundtrip() {
    let assignments = vec![
        make_assignment(
            101,
            "Week 1 Quiz",
            AssignmentType::Quiz,
            GradeScale::Points { max: 20 },
            1_700_100_000,
            20,
        ),
        make_assignment(
            102,
            "Week 4 Lab",
            AssignmentType::Lab,
            GradeScale::PassFail,
            1_700_400_000,
            0,
        ),
    ];
    let content = vec![
        ContentType::Video { duration_s: 1_800 },
        ContentType::Text { word_count: 3_200 },
        ContentType::Quiz { questions: 20 },
    ];
    let course = make_course(
        1001,
        "Introduction to Computer Science",
        CourseStatus::Published,
        assignments,
        content,
    );

    let bytes = encode_to_vec(&course).expect("encode published Course");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice(&bytes).expect("decode published Course");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal published Course encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Course — draft course with no assignments or content roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_draft_empty_vectors_roundtrip() {
    let course = make_course(
        2002,
        "Advanced Quantum Mechanics (DRAFT)",
        CourseStatus::Draft,
        vec![],
        vec![],
    );

    let bytes = encode_to_vec(&course).expect("encode draft empty Course");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice(&bytes).expect("decode draft empty Course");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal draft empty Course encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Course — archived course with all ContentType variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_archived_all_content_types_roundtrip() {
    let content = vec![
        ContentType::Video { duration_s: 7_200 },
        ContentType::Text { word_count: 8_000 },
        ContentType::Interactive { modules: 12 },
        ContentType::Quiz { questions: 30 },
        ContentType::File {
            size_bytes: 52_428_800,
        },
    ];
    let course = make_course(
        3003,
        "Digital Photography Fundamentals",
        CourseStatus::Archived,
        vec![],
        content,
    );

    let bytes = encode_to_vec(&course).expect("encode archived Course all content types");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice(&bytes).expect("decode archived Course all content types");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal archived Course all content types encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Course — deleted course retains data roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_deleted_retains_data_roundtrip() {
    let assignments = vec![make_assignment(
        999,
        "Deleted Assignment",
        AssignmentType::Essay,
        GradeScale::Letter,
        0,
        0,
    )];
    let course = make_course(
        4004,
        "Obsolete Course on Floppy Disk Storage",
        CourseStatus::Deleted,
        assignments,
        vec![ContentType::File {
            size_bytes: 1_474_560,
        }],
    );

    let bytes = encode_to_vec(&course).expect("encode deleted Course");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice(&bytes).expect("decode deleted Course");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal deleted Course encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Vec<Course> — batch of multiple courses roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_courses_roundtrip() {
    let courses: Vec<Course> = vec![
        make_course(
            10,
            "Python for Beginners",
            CourseStatus::Published,
            vec![make_assignment(
                1,
                "Hello World Assignment",
                AssignmentType::Lab,
                GradeScale::PassFail,
                1_700_000_100,
                0,
            )],
            vec![
                ContentType::Video { duration_s: 900 },
                ContentType::Text { word_count: 500 },
            ],
        ),
        make_course(
            20,
            "Data Structures",
            CourseStatus::Draft,
            vec![],
            vec![ContentType::Interactive { modules: 5 }],
        ),
        make_course(
            30,
            "Machine Learning",
            CourseStatus::Published,
            vec![
                make_assignment(
                    2,
                    "Linear Regression Project",
                    AssignmentType::Project,
                    GradeScale::Points { max: 150 },
                    1_720_000_000,
                    150,
                ),
                make_assignment(
                    3,
                    "Neural Network Essay",
                    AssignmentType::Essay,
                    GradeScale::Custom("Outstanding/Proficient/Developing".to_string()),
                    1_730_000_000,
                    30,
                ),
            ],
            vec![
                ContentType::Video { duration_s: 5_400 },
                ContentType::Quiz { questions: 40 },
                ContentType::File {
                    size_bytes: 209_715_200,
                },
            ],
        ),
    ];

    let bytes = encode_to_vec(&courses).expect("encode Vec<Course>");
    let (decoded, consumed): (Vec<Course>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Course>");
    assert_eq!(courses, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal Vec<Course> encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Big-endian config — Course roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let assignments = vec![make_assignment(
        77,
        "Big-Endian Quiz",
        AssignmentType::Quiz,
        GradeScale::Percentage,
        1_760_000_000,
        25,
    )];
    let content = vec![ContentType::Video { duration_s: 600 }];
    let course = make_course(
        5005,
        "Network Protocols",
        CourseStatus::Published,
        assignments,
        content,
    );

    let bytes = encode_to_vec_with_config(&course, cfg).expect("encode big-endian Course");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian Course");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian Course encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Fixed-int config — Course roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let assignments = vec![make_assignment(
        u64::MAX,
        "Boundary Assignment",
        AssignmentType::Project,
        GradeScale::Points { max: u32::MAX },
        u64::MAX,
        u32::MAX,
    )];
    let content = vec![
        ContentType::File {
            size_bytes: u64::MAX,
        },
        ContentType::Video {
            duration_s: u32::MAX,
        },
    ];
    let course = make_course(
        u64::MAX,
        "Boundary Course",
        CourseStatus::Draft,
        assignments,
        content,
    );

    let bytes = encode_to_vec_with_config(&course, cfg).expect("encode fixed-int Course");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed-int Course");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal fixed-int Course encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Big-endian + fixed-int combined config — Course roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_course_big_endian_fixed_int_combined_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let assignments = vec![
        make_assignment(
            10,
            "Combined Config Essay",
            AssignmentType::Essay,
            GradeScale::Letter,
            1_770_000_000,
            40,
        ),
        make_assignment(
            11,
            "Combined Config Presentation",
            AssignmentType::Presentation,
            GradeScale::Custom("Honors/Pass/Fail".to_string()),
            1_780_000_000,
            60,
        ),
    ];
    let content = vec![
        ContentType::Interactive { modules: u8::MAX },
        ContentType::Quiz {
            questions: u16::MAX,
        },
        ContentType::Text {
            word_count: u32::MAX,
        },
    ];
    let course = make_course(
        6006,
        "Advanced Academic Writing",
        CourseStatus::Published,
        assignments,
        content,
    );

    let bytes = encode_to_vec_with_config(&course, cfg).expect("encode big-endian+fixed Course");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian+fixed Course");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian+fixed Course encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Consumed bytes accuracy — sequential decode from concatenated buffer
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_accuracy_sequential_courses() {
    let course1 = make_course(
        100,
        "Calculus I",
        CourseStatus::Published,
        vec![make_assignment(
            1,
            "Derivatives HW",
            AssignmentType::Lab,
            GradeScale::Points { max: 30 },
            1_700_200_000,
            30,
        )],
        vec![ContentType::Video { duration_s: 2_700 }],
    );
    let course2 = make_course(
        200,
        "Linear Algebra",
        CourseStatus::Archived,
        vec![],
        vec![
            ContentType::Text { word_count: 4_000 },
            ContentType::File { size_bytes: 2_048 },
        ],
    );
    let course3 = make_course(
        300,
        "Discrete Mathematics",
        CourseStatus::Draft,
        vec![make_assignment(
            2,
            "Graph Theory Assignment",
            AssignmentType::Project,
            GradeScale::Percentage,
            1_730_300_000,
            80,
        )],
        vec![
            ContentType::Interactive { modules: 6 },
            ContentType::Quiz { questions: 25 },
        ],
    );

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(encode_to_vec(&course1).expect("encode course1"));
    buffer.extend(encode_to_vec(&course2).expect("encode course2"));
    buffer.extend(encode_to_vec(&course3).expect("encode course3"));

    let (decoded1, consumed1): (Course, usize) =
        decode_from_slice(&buffer).expect("decode course1");
    assert_eq!(course1, decoded1);

    let (decoded2, consumed2): (Course, usize) =
        decode_from_slice(&buffer[consumed1..]).expect("decode course2");
    assert_eq!(course2, decoded2);

    let (decoded3, consumed3): (Course, usize) =
        decode_from_slice(&buffer[consumed1 + consumed2..]).expect("decode course3");
    assert_eq!(course3, decoded3);

    assert_eq!(
        consumed1 + consumed2 + consumed3,
        buffer.len(),
        "sum of consumed bytes must equal total buffer length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Large course — many assignments and content items roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_course_many_assignments_and_content_roundtrip() {
    let assignment_types = [
        AssignmentType::Quiz,
        AssignmentType::Essay,
        AssignmentType::Project,
        AssignmentType::PeerReview,
        AssignmentType::Lab,
        AssignmentType::Presentation,
    ];
    let grade_scales = [
        GradeScale::Letter,
        GradeScale::Percentage,
        GradeScale::PassFail,
        GradeScale::Points { max: 100 },
        GradeScale::Custom("Custom Scale".to_string()),
    ];

    let assignments: Vec<Assignment> = (0_u64..50)
        .map(|i| {
            make_assignment(
                i,
                &format!("Assignment {i}"),
                assignment_types[(i as usize) % assignment_types.len()].clone(),
                grade_scales[(i as usize) % grade_scales.len()].clone(),
                1_700_000_000 + i * 86_400,
                (i as u32) * 10,
            )
        })
        .collect();

    let content: Vec<ContentType> = (0_u64..50)
        .map(|i| match i % 5 {
            0 => ContentType::Video {
                duration_s: (i as u32) * 60,
            },
            1 => ContentType::Text {
                word_count: (i as u32) * 200,
            },
            2 => ContentType::Interactive {
                modules: (i % 256) as u8,
            },
            3 => ContentType::Quiz {
                questions: (i % 65536) as u16,
            },
            _ => ContentType::File {
                size_bytes: i * 1_048_576,
            },
        })
        .collect();

    let course = make_course(
        9999,
        "Comprehensive Full-Stack Web Development Bootcamp",
        CourseStatus::Published,
        assignments,
        content,
    );

    let bytes = encode_to_vec(&course).expect("encode large Course");
    let (decoded, consumed): (Course, usize) =
        decode_from_slice(&bytes).expect("decode large Course");
    assert_eq!(course, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal large Course encoding length"
    );
}
