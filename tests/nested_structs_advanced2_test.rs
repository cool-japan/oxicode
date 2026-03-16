//! Advanced nested structs test — organizational hierarchy theme, 22 tests.

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

#[derive(Debug, PartialEq, Encode, Decode)]
enum Role {
    Admin,
    Manager,
    Engineer,
    Intern,
    Contractor,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Address {
    street: String,
    city: String,
    country: String,
    postal_code: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Employee {
    id: u64,
    name: String,
    role: Role,
    address: Address,
    skills: Vec<String>,
    salary: Option<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Team {
    name: String,
    lead: Employee,
    members: Vec<Employee>,
    budget: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Department {
    name: String,
    teams: Vec<Team>,
    head_count: u32,
    location: Address,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sample_address(prefix: &str) -> Address {
    Address {
        street: format!("{} Main St", prefix),
        city: format!("{} City", prefix),
        country: "Testland".to_string(),
        postal_code: format!("{}-0001", prefix),
    }
}

fn sample_employee(id: u64, name: &str, role: Role) -> Employee {
    Employee {
        id,
        name: name.to_string(),
        role,
        address: sample_address(name),
        skills: vec!["Rust".to_string(), "Testing".to_string()],
        salary: Some(100_000),
    }
}

// ---------------------------------------------------------------------------
// Test 1: Address roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_address_roundtrip() {
    let val = sample_address("Tokyo");
    let bytes = encode_to_vec(&val).expect("encode address");
    let (decoded, _): (Address, usize) = decode_from_slice(&bytes).expect("decode address");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Employee with Admin role roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_employee_admin_roundtrip() {
    let val = Employee {
        id: 1,
        name: "Alice".to_string(),
        role: Role::Admin,
        address: sample_address("Admin"),
        skills: vec!["Rust".to_string()],
        salary: Some(200_000),
    };
    let bytes = encode_to_vec(&val).expect("encode employee admin");
    let (decoded, _): (Employee, usize) = decode_from_slice(&bytes).expect("decode employee admin");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Employee with Manager role and None salary
// ---------------------------------------------------------------------------

#[test]
fn test_employee_manager_none_salary() {
    let val = Employee {
        id: 2,
        name: "Bob".to_string(),
        role: Role::Manager,
        address: sample_address("Manager"),
        skills: vec!["Leadership".to_string()],
        salary: None,
    };
    let bytes = encode_to_vec(&val).expect("encode employee manager");
    let (decoded, _): (Employee, usize) =
        decode_from_slice(&bytes).expect("decode employee manager");
    assert_eq!(val, decoded);
    assert_eq!(decoded.salary, None);
}

// ---------------------------------------------------------------------------
// Test 4: Employee with Engineer role and Some salary
// ---------------------------------------------------------------------------

#[test]
fn test_employee_engineer_some_salary() {
    let val = Employee {
        id: 3,
        name: "Carol".to_string(),
        role: Role::Engineer,
        address: sample_address("Eng"),
        skills: vec!["Rust".to_string(), "Systems".to_string()],
        salary: Some(150_000),
    };
    let bytes = encode_to_vec(&val).expect("encode employee engineer");
    let (decoded, _): (Employee, usize) =
        decode_from_slice(&bytes).expect("decode employee engineer");
    assert_eq!(val, decoded);
    assert_eq!(decoded.salary, Some(150_000));
}

// ---------------------------------------------------------------------------
// Test 5: Employee with empty skills vec
// ---------------------------------------------------------------------------

#[test]
fn test_employee_empty_skills() {
    let val = Employee {
        id: 4,
        name: "Dave".to_string(),
        role: Role::Intern,
        address: sample_address("Intern"),
        skills: vec![],
        salary: Some(30_000),
    };
    let bytes = encode_to_vec(&val).expect("encode employee empty skills");
    let (decoded, _): (Employee, usize) =
        decode_from_slice(&bytes).expect("decode employee empty skills");
    assert_eq!(val, decoded);
    assert!(decoded.skills.is_empty());
}

// ---------------------------------------------------------------------------
// Test 6: Employee with many skills
// ---------------------------------------------------------------------------

#[test]
fn test_employee_many_skills() {
    let skills: Vec<String> = (0..20).map(|i| format!("Skill_{}", i)).collect();
    let val = Employee {
        id: 5,
        name: "Eve".to_string(),
        role: Role::Engineer,
        address: sample_address("Many"),
        skills,
        salary: Some(120_000),
    };
    let bytes = encode_to_vec(&val).expect("encode employee many skills");
    let (decoded, _): (Employee, usize) =
        decode_from_slice(&bytes).expect("decode employee many skills");
    assert_eq!(val, decoded);
    assert_eq!(decoded.skills.len(), 20);
}

// ---------------------------------------------------------------------------
// Test 7: Team with no members
// ---------------------------------------------------------------------------

#[test]
fn test_team_no_members() {
    let val = Team {
        name: "Solo Team".to_string(),
        lead: sample_employee(10, "Frank", Role::Manager),
        members: vec![],
        budget: 50_000,
    };
    let bytes = encode_to_vec(&val).expect("encode team no members");
    let (decoded, _): (Team, usize) = decode_from_slice(&bytes).expect("decode team no members");
    assert_eq!(val, decoded);
    assert!(decoded.members.is_empty());
}

// ---------------------------------------------------------------------------
// Test 8: Team with 3 members
// ---------------------------------------------------------------------------

#[test]
fn test_team_three_members() {
    let val = Team {
        name: "Alpha Team".to_string(),
        lead: sample_employee(20, "Grace", Role::Manager),
        members: vec![
            sample_employee(21, "Hank", Role::Engineer),
            sample_employee(22, "Ivy", Role::Engineer),
            sample_employee(23, "Jack", Role::Intern),
        ],
        budget: 300_000,
    };
    let bytes = encode_to_vec(&val).expect("encode team three members");
    let (decoded, _): (Team, usize) = decode_from_slice(&bytes).expect("decode team three members");
    assert_eq!(val, decoded);
    assert_eq!(decoded.members.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 9: Department with 2 teams
// ---------------------------------------------------------------------------

#[test]
fn test_department_two_teams() {
    let val = Department {
        name: "Engineering".to_string(),
        teams: vec![
            Team {
                name: "Backend".to_string(),
                lead: sample_employee(30, "Karl", Role::Manager),
                members: vec![sample_employee(31, "Lena", Role::Engineer)],
                budget: 200_000,
            },
            Team {
                name: "Frontend".to_string(),
                lead: sample_employee(40, "Mira", Role::Manager),
                members: vec![sample_employee(41, "Noel", Role::Engineer)],
                budget: 180_000,
            },
        ],
        head_count: 6,
        location: sample_address("HQ"),
    };
    let bytes = encode_to_vec(&val).expect("encode department two teams");
    let (decoded, _): (Department, usize) =
        decode_from_slice(&bytes).expect("decode department two teams");
    assert_eq!(val, decoded);
    assert_eq!(decoded.teams.len(), 2);
}

// ---------------------------------------------------------------------------
// Test 10: Department with empty teams
// ---------------------------------------------------------------------------

#[test]
fn test_department_empty_teams() {
    let val = Department {
        name: "Research".to_string(),
        teams: vec![],
        head_count: 0,
        location: sample_address("Lab"),
    };
    let bytes = encode_to_vec(&val).expect("encode department empty teams");
    let (decoded, _): (Department, usize) =
        decode_from_slice(&bytes).expect("decode department empty teams");
    assert_eq!(val, decoded);
    assert!(decoded.teams.is_empty());
}

// ---------------------------------------------------------------------------
// Test 11: Role::Contractor roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_role_contractor_roundtrip() {
    let val = Role::Contractor;
    let bytes = encode_to_vec(&val).expect("encode role contractor");
    let (decoded, _): (Role, usize) = decode_from_slice(&bytes).expect("decode role contractor");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Role::Intern roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_role_intern_roundtrip() {
    let val = Role::Intern;
    let bytes = encode_to_vec(&val).expect("encode role intern");
    let (decoded, _): (Role, usize) = decode_from_slice(&bytes).expect("decode role intern");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Vec<Employee> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_employee_roundtrip() {
    let val: Vec<Employee> = vec![
        sample_employee(50, "Oscar", Role::Admin),
        sample_employee(51, "Pam", Role::Engineer),
        sample_employee(52, "Quinn", Role::Contractor),
    ];
    let bytes = encode_to_vec(&val).expect("encode vec employee");
    let (decoded, _): (Vec<Employee>, usize) =
        decode_from_slice(&bytes).expect("decode vec employee");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 14: Vec<Team> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_team_roundtrip() {
    let val: Vec<Team> = vec![
        Team {
            name: "Team A".to_string(),
            lead: sample_employee(60, "Rita", Role::Manager),
            members: vec![sample_employee(61, "Sam", Role::Engineer)],
            budget: 100_000,
        },
        Team {
            name: "Team B".to_string(),
            lead: sample_employee(70, "Tina", Role::Manager),
            members: vec![],
            budget: 75_000,
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode vec team");
    let (decoded, _): (Vec<Team>, usize) = decode_from_slice(&bytes).expect("decode vec team");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Option<Employee> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_employee_some_roundtrip() {
    let val: Option<Employee> = Some(sample_employee(80, "Uma", Role::Admin));
    let bytes = encode_to_vec(&val).expect("encode option employee some");
    let (decoded, _): (Option<Employee>, usize) =
        decode_from_slice(&bytes).expect("decode option employee some");
    assert_eq!(val, decoded);
    assert!(decoded.is_some());
}

// ---------------------------------------------------------------------------
// Test 16: Option<Employee> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_employee_none_roundtrip() {
    let val: Option<Employee> = None;
    let bytes = encode_to_vec(&val).expect("encode option employee none");
    let (decoded, _): (Option<Employee>, usize) =
        decode_from_slice(&bytes).expect("decode option employee none");
    assert_eq!(val, decoded);
    assert!(decoded.is_none());
}

// ---------------------------------------------------------------------------
// Test 17: Deeply nested Department (2 teams × 3 members each)
// ---------------------------------------------------------------------------

#[test]
fn test_deeply_nested_department_2x3() {
    let make_team = |team_idx: u64, name: &str| Team {
        name: name.to_string(),
        lead: sample_employee(team_idx * 100, &format!("Lead_{}", name), Role::Manager),
        members: vec![
            sample_employee(
                team_idx * 100 + 1,
                &format!("Mem_{}_1", name),
                Role::Engineer,
            ),
            sample_employee(
                team_idx * 100 + 2,
                &format!("Mem_{}_2", name),
                Role::Engineer,
            ),
            sample_employee(team_idx * 100 + 3, &format!("Mem_{}_3", name), Role::Intern),
        ],
        budget: 250_000,
    };

    let val = Department {
        name: "Platform".to_string(),
        teams: vec![make_team(1, "Infra"), make_team(2, "Cloud")],
        head_count: 8,
        location: sample_address("Platform HQ"),
    };

    let bytes = encode_to_vec(&val).expect("encode deeply nested department");
    let (decoded, _): (Department, usize) =
        decode_from_slice(&bytes).expect("decode deeply nested department");
    assert_eq!(val, decoded);
    assert_eq!(decoded.teams.len(), 2);
    assert_eq!(decoded.teams[0].members.len(), 3);
    assert_eq!(decoded.teams[1].members.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 18: Big-endian config Department roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_config_department_roundtrip() {
    let val = Department {
        name: "Finance".to_string(),
        teams: vec![Team {
            name: "Accounting".to_string(),
            lead: sample_employee(200, "Victor", Role::Manager),
            members: vec![sample_employee(201, "Wendy", Role::Engineer)],
            budget: 500_000,
        }],
        head_count: 4,
        location: sample_address("Finance HQ"),
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode big-endian department");
    let (decoded, _): (Department, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian department");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Fixed-int config Employee roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_employee_roundtrip() {
    let val = Employee {
        id: 999,
        name: "Xavier".to_string(),
        role: Role::Contractor,
        address: sample_address("Remote"),
        skills: vec!["Consulting".to_string()],
        salary: Some(180_000),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode fixed-int employee");
    let (decoded, _): (Employee, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed-int employee");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Consumed bytes == encoded length for Department
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_equals_encoded_length_department() {
    let val = Department {
        name: "Ops".to_string(),
        teams: vec![Team {
            name: "SRE".to_string(),
            lead: sample_employee(300, "Yara", Role::Engineer),
            members: vec![],
            budget: 90_000,
        }],
        head_count: 3,
        location: sample_address("Ops"),
    };
    let bytes = encode_to_vec(&val).expect("encode department for length check");
    let (_, consumed): (Department, usize) =
        decode_from_slice(&bytes).expect("decode department for length check");
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 21: Encoding determinism for Team
// ---------------------------------------------------------------------------

#[test]
fn test_encoding_determinism_team() {
    let val = Team {
        name: "Stable Team".to_string(),
        lead: sample_employee(400, "Zara", Role::Admin),
        members: vec![
            sample_employee(401, "Aaron", Role::Engineer),
            sample_employee(402, "Beth", Role::Engineer),
        ],
        budget: 400_000,
    };
    let bytes_a = encode_to_vec(&val).expect("encode team determinism first");
    let bytes_b = encode_to_vec(&val).expect("encode team determinism second");
    assert_eq!(bytes_a, bytes_b, "encoding must be deterministic");
}

// ---------------------------------------------------------------------------
// Test 22: Unicode names in Employee (Japanese characters)
// ---------------------------------------------------------------------------

#[test]
fn test_unicode_japanese_names_in_employee() {
    let val = Employee {
        id: 9001,
        name: "山田太郎".to_string(),
        role: Role::Engineer,
        address: Address {
            street: "東京都渋谷区1-1".to_string(),
            city: "東京".to_string(),
            country: "日本".to_string(),
            postal_code: "150-0001".to_string(),
        },
        skills: vec!["Rust".to_string(), "日本語".to_string(), "設計".to_string()],
        salary: Some(8_000_000),
    };
    let bytes = encode_to_vec(&val).expect("encode unicode employee");
    let (decoded, _): (Employee, usize) =
        decode_from_slice(&bytes).expect("decode unicode employee");
    assert_eq!(val, decoded);
    assert_eq!(decoded.name, "山田太郎");
    assert_eq!(decoded.address.city, "東京");
    assert_eq!(decoded.skills[1], "日本語");
}
