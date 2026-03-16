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
enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Intern,
    Freelance,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PayFrequency {
    Weekly,
    Biweekly,
    SemiMonthly,
    Monthly,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum LeaveType {
    Vacation,
    Sick,
    Maternity,
    Paternity,
    Bereavement,
    Unpaid,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PerformanceRating {
    Unsatisfactory,
    NeedsImprovement,
    MeetsExpectations,
    Exceeds,
    Outstanding,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Employee {
    emp_id: u64,
    name: String,
    department: String,
    title: String,
    employment_type: EmploymentType,
    hire_date: u64,
    manager_id: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PayrollEntry {
    emp_id: u64,
    period_start: u64,
    period_end: u64,
    gross_pay_cents: u64,
    deductions_cents: u32,
    net_pay_cents: u64,
    frequency: PayFrequency,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LeaveRequest {
    request_id: u64,
    emp_id: u64,
    leave_type: LeaveType,
    start_date: u64,
    end_date: u64,
    approved: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PerformanceReview {
    review_id: u64,
    emp_id: u64,
    reviewer_id: u64,
    review_date: u64,
    rating: PerformanceRating,
    comments: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Benefit {
    benefit_id: u32,
    emp_id: u64,
    benefit_type: String,
    cost_per_period_cents: u32,
    effective_date: u64,
    end_date: Option<u64>,
}

// Test 1: Employee with manager (Some) using standard config
#[test]
fn test_employee_with_manager_standard() {
    let emp = Employee {
        emp_id: 1001,
        name: "Alice Johnson".to_string(),
        department: "Engineering".to_string(),
        title: "Senior Software Engineer".to_string(),
        employment_type: EmploymentType::FullTime,
        hire_date: 1_609_459_200,
        manager_id: Some(500),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&emp, cfg).expect("encode employee with manager");
    let (decoded, consumed): (Employee, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode employee with manager");
    assert_eq!(emp, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 2: Employee without manager (None) using standard config
#[test]
fn test_employee_no_manager_standard() {
    let emp = Employee {
        emp_id: 9999,
        name: "Bob Martinez".to_string(),
        department: "Executive".to_string(),
        title: "Chief Executive Officer".to_string(),
        employment_type: EmploymentType::FullTime,
        hire_date: 1_500_000_000,
        manager_id: None,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&emp, cfg).expect("encode employee no manager");
    let (decoded, consumed): (Employee, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode employee no manager");
    assert_eq!(emp, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 3: Employee as Freelance with big-endian config
#[test]
fn test_employee_freelance_big_endian() {
    let emp = Employee {
        emp_id: 4242,
        name: "Carol Smith".to_string(),
        department: "Design".to_string(),
        title: "UX Consultant".to_string(),
        employment_type: EmploymentType::Freelance,
        hire_date: 1_650_000_000,
        manager_id: Some(300),
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&emp, cfg).expect("encode freelance employee big endian");
    let (decoded, consumed): (Employee, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode freelance employee big endian");
    assert_eq!(emp, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 4: Employee as Intern with fixed int encoding
#[test]
fn test_employee_intern_fixed_int() {
    let emp = Employee {
        emp_id: 7777,
        name: "Dave Lee".to_string(),
        department: "Marketing".to_string(),
        title: "Marketing Intern".to_string(),
        employment_type: EmploymentType::Intern,
        hire_date: 1_700_000_000,
        manager_id: None,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&emp, cfg).expect("encode intern employee fixed int");
    let (decoded, consumed): (Employee, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode intern employee fixed int");
    assert_eq!(emp, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 5: PayrollEntry with weekly frequency, standard config
#[test]
fn test_payroll_entry_weekly_standard() {
    let entry = PayrollEntry {
        emp_id: 1001,
        period_start: 1_700_000_000,
        period_end: 1_700_604_800,
        gross_pay_cents: 250_000,
        deductions_cents: 50_000,
        net_pay_cents: 200_000,
        frequency: PayFrequency::Weekly,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&entry, cfg).expect("encode weekly payroll");
    let (decoded, consumed): (PayrollEntry, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode weekly payroll");
    assert_eq!(entry, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 6: PayrollEntry with biweekly frequency, big-endian config
#[test]
fn test_payroll_entry_biweekly_big_endian() {
    let entry = PayrollEntry {
        emp_id: 2002,
        period_start: 1_710_000_000,
        period_end: 1_711_209_600,
        gross_pay_cents: 500_000,
        deductions_cents: 100_000,
        net_pay_cents: 400_000,
        frequency: PayFrequency::Biweekly,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&entry, cfg).expect("encode biweekly payroll big endian");
    let (decoded, consumed): (PayrollEntry, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode biweekly payroll big endian");
    assert_eq!(entry, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 7: PayrollEntry with monthly frequency, fixed int config
#[test]
fn test_payroll_entry_monthly_fixed_int() {
    let entry = PayrollEntry {
        emp_id: 3003,
        period_start: 1_720_000_000,
        period_end: 1_722_678_400,
        gross_pay_cents: 1_000_000,
        deductions_cents: 200_000,
        net_pay_cents: 800_000,
        frequency: PayFrequency::Monthly,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&entry, cfg).expect("encode monthly payroll fixed int");
    let (decoded, consumed): (PayrollEntry, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode monthly payroll fixed int");
    assert_eq!(entry, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 8: PayrollEntry semi-monthly frequency verification
#[test]
fn test_payroll_entry_semi_monthly_standard() {
    let entry = PayrollEntry {
        emp_id: 4004,
        period_start: 1_730_000_000,
        period_end: 1_731_296_000,
        gross_pay_cents: 750_000,
        deductions_cents: 150_000,
        net_pay_cents: 600_000,
        frequency: PayFrequency::SemiMonthly,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&entry, cfg).expect("encode semi-monthly payroll");
    let (decoded, consumed): (PayrollEntry, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode semi-monthly payroll");
    assert_eq!(entry, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 9: LeaveRequest with vacation, pending approval (None), standard config
#[test]
fn test_leave_request_vacation_pending_standard() {
    let req = LeaveRequest {
        request_id: 8001,
        emp_id: 1001,
        leave_type: LeaveType::Vacation,
        start_date: 1_750_000_000,
        end_date: 1_750_864_000,
        approved: None,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&req, cfg).expect("encode vacation leave pending");
    let (decoded, consumed): (LeaveRequest, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vacation leave pending");
    assert_eq!(req, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 10: LeaveRequest with sick leave approved, big-endian config
#[test]
fn test_leave_request_sick_approved_big_endian() {
    let req = LeaveRequest {
        request_id: 8002,
        emp_id: 2002,
        leave_type: LeaveType::Sick,
        start_date: 1_760_000_000,
        end_date: 1_760_172_800,
        approved: Some(true),
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&req, cfg).expect("encode sick leave approved big endian");
    let (decoded, consumed): (LeaveRequest, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode sick leave approved big endian");
    assert_eq!(req, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 11: LeaveRequest with maternity leave denied, fixed int config
#[test]
fn test_leave_request_maternity_denied_fixed_int() {
    let req = LeaveRequest {
        request_id: 8003,
        emp_id: 3003,
        leave_type: LeaveType::Maternity,
        start_date: 1_770_000_000,
        end_date: 1_777_776_000,
        approved: Some(false),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&req, cfg).expect("encode maternity leave denied fixed int");
    let (decoded, consumed): (LeaveRequest, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode maternity leave denied fixed int");
    assert_eq!(req, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 12: LeaveRequest with paternity leave using standard config
#[test]
fn test_leave_request_paternity_standard() {
    let req = LeaveRequest {
        request_id: 8004,
        emp_id: 4004,
        leave_type: LeaveType::Paternity,
        start_date: 1_780_000_000,
        end_date: 1_781_728_000,
        approved: Some(true),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&req, cfg).expect("encode paternity leave");
    let (decoded, consumed): (LeaveRequest, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode paternity leave");
    assert_eq!(req, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 13: PerformanceReview with Outstanding rating and comment, standard config
#[test]
fn test_performance_review_outstanding_with_comment_standard() {
    let review = PerformanceReview {
        review_id: 9001,
        emp_id: 1001,
        reviewer_id: 500,
        review_date: 1_790_000_000,
        rating: PerformanceRating::Outstanding,
        comments: Some("Exceeded all goals; promoted to Tech Lead.".to_string()),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&review, cfg).expect("encode outstanding review");
    let (decoded, consumed): (PerformanceReview, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode outstanding review");
    assert_eq!(review, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 14: PerformanceReview with NeedsImprovement and no comment, big-endian config
#[test]
fn test_performance_review_needs_improvement_no_comment_big_endian() {
    let review = PerformanceReview {
        review_id: 9002,
        emp_id: 2002,
        reviewer_id: 600,
        review_date: 1_800_000_000,
        rating: PerformanceRating::NeedsImprovement,
        comments: None,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&review, cfg).expect("encode needs improvement review big endian");
    let (decoded, consumed): (PerformanceReview, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode needs improvement review big endian");
    assert_eq!(review, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 15: PerformanceReview with MeetsExpectations and detailed comment, fixed int config
#[test]
fn test_performance_review_meets_expectations_fixed_int() {
    let review = PerformanceReview {
        review_id: 9003,
        emp_id: 3003,
        reviewer_id: 700,
        review_date: 1_810_000_000,
        rating: PerformanceRating::MeetsExpectations,
        comments: Some("Consistently delivered on schedule, good team player.".to_string()),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&review, cfg).expect("encode meets expectations review fixed int");
    let (decoded, consumed): (PerformanceReview, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode meets expectations review fixed int");
    assert_eq!(review, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 16: Benefit with no end date (active benefit), standard config
#[test]
fn test_benefit_active_no_end_date_standard() {
    let benefit = Benefit {
        benefit_id: 101,
        emp_id: 1001,
        benefit_type: "Health Insurance".to_string(),
        cost_per_period_cents: 45_000,
        effective_date: 1_609_459_200,
        end_date: None,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&benefit, cfg).expect("encode active benefit");
    let (decoded, consumed): (Benefit, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode active benefit");
    assert_eq!(benefit, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 17: Benefit with end date (expired benefit), big-endian config
#[test]
fn test_benefit_expired_with_end_date_big_endian() {
    let benefit = Benefit {
        benefit_id: 202,
        emp_id: 2002,
        benefit_type: "Dental Plan".to_string(),
        cost_per_period_cents: 12_000,
        effective_date: 1_620_000_000,
        end_date: Some(1_700_000_000),
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&benefit, cfg).expect("encode expired benefit big endian");
    let (decoded, consumed): (Benefit, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode expired benefit big endian");
    assert_eq!(benefit, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 18: Benefit with vision plan, fixed int encoding
#[test]
fn test_benefit_vision_plan_fixed_int() {
    let benefit = Benefit {
        benefit_id: 303,
        emp_id: 3003,
        benefit_type: "Vision Plan".to_string(),
        cost_per_period_cents: 8_500,
        effective_date: 1_630_000_000,
        end_date: None,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&benefit, cfg).expect("encode vision benefit fixed int");
    let (decoded, consumed): (Benefit, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vision benefit fixed int");
    assert_eq!(benefit, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 19: Vec<Employee> roundtrip, standard config
#[test]
fn test_vec_employees_roundtrip_standard() {
    let employees = vec![
        Employee {
            emp_id: 1,
            name: "Eve Torres".to_string(),
            department: "HR".to_string(),
            title: "HR Manager".to_string(),
            employment_type: EmploymentType::FullTime,
            hire_date: 1_580_000_000,
            manager_id: None,
        },
        Employee {
            emp_id: 2,
            name: "Frank Nguyen".to_string(),
            department: "HR".to_string(),
            title: "HR Specialist".to_string(),
            employment_type: EmploymentType::PartTime,
            hire_date: 1_600_000_000,
            manager_id: Some(1),
        },
        Employee {
            emp_id: 3,
            name: "Grace Kim".to_string(),
            department: "Finance".to_string(),
            title: "Finance Analyst".to_string(),
            employment_type: EmploymentType::Contract,
            hire_date: 1_620_000_000,
            manager_id: Some(10),
        },
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&employees, cfg).expect("encode vec employees");
    let (decoded, consumed): (Vec<Employee>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec employees");
    assert_eq!(employees, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 3);
}

// Test 20: Vec<PayrollEntry> roundtrip, big-endian config
#[test]
fn test_vec_payroll_entries_roundtrip_big_endian() {
    let entries = vec![
        PayrollEntry {
            emp_id: 10,
            period_start: 1_700_000_000,
            period_end: 1_700_604_800,
            gross_pay_cents: 300_000,
            deductions_cents: 60_000,
            net_pay_cents: 240_000,
            frequency: PayFrequency::Weekly,
        },
        PayrollEntry {
            emp_id: 20,
            period_start: 1_700_000_000,
            period_end: 1_701_209_600,
            gross_pay_cents: 600_000,
            deductions_cents: 120_000,
            net_pay_cents: 480_000,
            frequency: PayFrequency::Biweekly,
        },
        PayrollEntry {
            emp_id: 30,
            period_start: 1_700_000_000,
            period_end: 1_702_678_400,
            gross_pay_cents: 1_200_000,
            deductions_cents: 240_000,
            net_pay_cents: 960_000,
            frequency: PayFrequency::Monthly,
        },
    ];
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&entries, cfg).expect("encode vec payroll big endian");
    let (decoded, consumed): (Vec<PayrollEntry>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec payroll big endian");
    assert_eq!(entries, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 3);
}

// Test 21: Vec<LeaveRequest> with mixed leave types and approval states, fixed int config
#[test]
fn test_vec_leave_requests_mixed_states_fixed_int() {
    let requests = vec![
        LeaveRequest {
            request_id: 100,
            emp_id: 1,
            leave_type: LeaveType::Bereavement,
            start_date: 1_710_000_000,
            end_date: 1_710_432_000,
            approved: Some(true),
        },
        LeaveRequest {
            request_id: 101,
            emp_id: 2,
            leave_type: LeaveType::Unpaid,
            start_date: 1_720_000_000,
            end_date: 1_722_592_000,
            approved: None,
        },
        LeaveRequest {
            request_id: 102,
            emp_id: 3,
            leave_type: LeaveType::Paternity,
            start_date: 1_730_000_000,
            end_date: 1_731_728_000,
            approved: Some(false),
        },
    ];
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&requests, cfg).expect("encode vec leave requests fixed int");
    let (decoded, consumed): (Vec<LeaveRequest>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec leave requests fixed int");
    assert_eq!(requests, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].approved, Some(true));
    assert_eq!(decoded[1].approved, None);
    assert_eq!(decoded[2].approved, Some(false));
}

// Test 22: Vec<PerformanceReview> and Vec<Benefit> combined scenario, standard config with consumed bytes verification
#[test]
fn test_performance_reviews_all_ratings_standard() {
    let reviews = vec![
        PerformanceReview {
            review_id: 1,
            emp_id: 11,
            reviewer_id: 99,
            review_date: 1_740_000_000,
            rating: PerformanceRating::Unsatisfactory,
            comments: Some("Failed to meet minimum requirements.".to_string()),
        },
        PerformanceReview {
            review_id: 2,
            emp_id: 22,
            reviewer_id: 99,
            review_date: 1_740_000_000,
            rating: PerformanceRating::NeedsImprovement,
            comments: None,
        },
        PerformanceReview {
            review_id: 3,
            emp_id: 33,
            reviewer_id: 99,
            review_date: 1_740_000_000,
            rating: PerformanceRating::MeetsExpectations,
            comments: Some("Solid, reliable contributor.".to_string()),
        },
        PerformanceReview {
            review_id: 4,
            emp_id: 44,
            reviewer_id: 99,
            review_date: 1_740_000_000,
            rating: PerformanceRating::Exceeds,
            comments: Some("Delivered 20% above target.".to_string()),
        },
        PerformanceReview {
            review_id: 5,
            emp_id: 55,
            reviewer_id: 99,
            review_date: 1_740_000_000,
            rating: PerformanceRating::Outstanding,
            comments: Some("Transformed the department; top performer.".to_string()),
        },
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&reviews, cfg).expect("encode all rating reviews");
    let (decoded, consumed): (Vec<PerformanceReview>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode all rating reviews");
    assert_eq!(reviews, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(consumed > 0);
    assert_eq!(decoded.len(), 5);
    assert_eq!(decoded[0].rating, PerformanceRating::Unsatisfactory);
    assert_eq!(decoded[4].rating, PerformanceRating::Outstanding);
    assert!(decoded[1].comments.is_none());
}
