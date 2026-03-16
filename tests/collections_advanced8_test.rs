//! Advanced collection tests: Task management / project tracking theme
//! Covers Priority, TaskStatus, Task, Project with various collection types.

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
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

#[derive(Debug, PartialEq, Encode, Decode)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Task {
    id: u64,
    title: String,
    priority: Priority,
    status: TaskStatus,
    assignee_id: Option<u64>,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Project {
    id: u64,
    name: String,
    tasks: Vec<Task>,
    member_ids: Vec<u64>,
}

fn make_task(
    id: u64,
    title: &str,
    priority: Priority,
    status: TaskStatus,
    assignee_id: Option<u64>,
    tags: Vec<&str>,
) -> Task {
    Task {
        id,
        title: title.to_string(),
        priority,
        status,
        assignee_id,
        tags: tags.into_iter().map(|s| s.to_string()).collect(),
    }
}

// 1. Basic Task roundtrip with assignee and tags
#[test]
fn test_task_basic_roundtrip() {
    let task = make_task(
        1,
        "Fix login bug",
        Priority::High,
        TaskStatus::InProgress,
        Some(42),
        vec!["bug", "auth"],
    );
    let encoded = encode_to_vec(&task).expect("encode failed");
    let (decoded, _): (Task, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(task, decoded);
}

// 2. Task with no assignee (None) and no tags
#[test]
fn test_task_no_assignee_no_tags() {
    let task = make_task(
        2,
        "Write docs",
        Priority::Low,
        TaskStatus::Todo,
        None,
        vec![],
    );
    let encoded = encode_to_vec(&task).expect("encode failed");
    let (decoded, _): (Task, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(task, decoded);
    assert!(decoded.assignee_id.is_none());
    assert!(decoded.tags.is_empty());
}

// 3. Priority::Critical and TaskStatus::Done roundtrip
#[test]
fn test_priority_critical_status_done() {
    let task = make_task(
        3,
        "Deploy release",
        Priority::Critical,
        TaskStatus::Done,
        Some(99),
        vec!["release", "ci"],
    );
    let encoded = encode_to_vec(&task).expect("encode failed");
    let (decoded, _): (Task, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded.priority, Priority::Critical);
    assert_eq!(decoded.status, TaskStatus::Done);
}

// 4. Priority::Medium and TaskStatus::Cancelled roundtrip
#[test]
fn test_priority_medium_status_cancelled() {
    let task = make_task(
        4,
        "Refactor DB layer",
        Priority::Medium,
        TaskStatus::Cancelled,
        None,
        vec!["refactor"],
    );
    let encoded = encode_to_vec(&task).expect("encode failed");
    let (decoded, _): (Task, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded.priority, Priority::Medium);
    assert_eq!(decoded.status, TaskStatus::Cancelled);
}

// 5. All Priority variants in a Vec<Task>
#[test]
fn test_all_priority_variants_in_vec() {
    let tasks: Vec<Task> = vec![
        make_task(
            10,
            "Low task",
            Priority::Low,
            TaskStatus::Todo,
            None,
            vec![],
        ),
        make_task(
            11,
            "Medium task",
            Priority::Medium,
            TaskStatus::InProgress,
            Some(1),
            vec!["wip"],
        ),
        make_task(
            12,
            "High task",
            Priority::High,
            TaskStatus::Done,
            Some(2),
            vec!["done"],
        ),
        make_task(
            13,
            "Critical task",
            Priority::Critical,
            TaskStatus::Cancelled,
            Some(3),
            vec!["crit"],
        ),
    ];
    let encoded = encode_to_vec(&tasks).expect("encode failed");
    let (decoded, _): (Vec<Task>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(tasks, decoded);
    assert_eq!(decoded[0].priority, Priority::Low);
    assert_eq!(decoded[3].priority, Priority::Critical);
}

// 6. All TaskStatus variants in a Vec<Task>
#[test]
fn test_all_task_status_variants_in_vec() {
    let tasks: Vec<Task> = vec![
        make_task(20, "t1", Priority::Low, TaskStatus::Todo, None, vec![]),
        make_task(
            21,
            "t2",
            Priority::Low,
            TaskStatus::InProgress,
            None,
            vec![],
        ),
        make_task(22, "t3", Priority::Low, TaskStatus::Done, None, vec![]),
        make_task(23, "t4", Priority::Low, TaskStatus::Cancelled, None, vec![]),
    ];
    let encoded = encode_to_vec(&tasks).expect("encode failed");
    let (decoded, _): (Vec<Task>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded[0].status, TaskStatus::Todo);
    assert_eq!(decoded[1].status, TaskStatus::InProgress);
    assert_eq!(decoded[2].status, TaskStatus::Done);
    assert_eq!(decoded[3].status, TaskStatus::Cancelled);
}

// 7. Project with multiple tasks roundtrip
#[test]
fn test_project_roundtrip() {
    let project = Project {
        id: 100,
        name: "OxiCode Improvements".to_string(),
        tasks: vec![
            make_task(
                1,
                "Add BTreeMap support",
                Priority::High,
                TaskStatus::Done,
                Some(5),
                vec!["feature"],
            ),
            make_task(
                2,
                "Write tests",
                Priority::Medium,
                TaskStatus::InProgress,
                Some(6),
                vec!["test", "qa"],
            ),
        ],
        member_ids: vec![5, 6, 7],
    };
    let encoded = encode_to_vec(&project).expect("encode failed");
    let (decoded, _): (Project, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(project, decoded);
    assert_eq!(decoded.tasks.len(), 2);
    assert_eq!(decoded.member_ids, vec![5, 6, 7]);
}

// 8. Empty Project (no tasks, no members)
#[test]
fn test_project_empty_collections() {
    let project = Project {
        id: 200,
        name: "Empty Project".to_string(),
        tasks: vec![],
        member_ids: vec![],
    };
    let encoded = encode_to_vec(&project).expect("encode failed");
    let (decoded, _): (Project, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(project, decoded);
    assert!(decoded.tasks.is_empty());
    assert!(decoded.member_ids.is_empty());
}

// 9. BTreeMap<u64, Task> roundtrip
#[test]
fn test_btreemap_u64_task() {
    let mut map: BTreeMap<u64, Task> = BTreeMap::new();
    map.insert(
        1,
        make_task(
            1,
            "Task Alpha",
            Priority::High,
            TaskStatus::Todo,
            Some(10),
            vec!["alpha"],
        ),
    );
    map.insert(
        2,
        make_task(
            2,
            "Task Beta",
            Priority::Low,
            TaskStatus::InProgress,
            None,
            vec!["beta"],
        ),
    );
    map.insert(
        3,
        make_task(
            3,
            "Task Gamma",
            Priority::Critical,
            TaskStatus::Done,
            Some(11),
            vec![],
        ),
    );
    let encoded = encode_to_vec(&map).expect("encode failed");
    let (decoded, _): (BTreeMap<u64, Task>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(map, decoded);
    assert_eq!(decoded.len(), 3);
}

// 10. BTreeMap<u64, Project> roundtrip
#[test]
fn test_btreemap_u64_project() {
    let mut map: BTreeMap<u64, Project> = BTreeMap::new();
    map.insert(
        1,
        Project {
            id: 1,
            name: "Alpha".to_string(),
            tasks: vec![make_task(
                1,
                "t1",
                Priority::Low,
                TaskStatus::Todo,
                None,
                vec![],
            )],
            member_ids: vec![1, 2],
        },
    );
    map.insert(
        2,
        Project {
            id: 2,
            name: "Beta".to_string(),
            tasks: vec![],
            member_ids: vec![3],
        },
    );
    let encoded = encode_to_vec(&map).expect("encode failed");
    let (decoded, _): (BTreeMap<u64, Project>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(map, decoded);
}

// 11. BTreeSet<u64> as a set of task IDs
#[test]
fn test_btreeset_task_ids() {
    let mut set: BTreeSet<u64> = BTreeSet::new();
    set.insert(100);
    set.insert(200);
    set.insert(300);
    set.insert(400);
    let encoded = encode_to_vec(&set).expect("encode failed");
    let (decoded, _): (BTreeSet<u64>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(set, decoded);
    assert!(decoded.contains(&200));
    assert!(!decoded.contains(&999));
}

// 12. HashMap<u64, Task> roundtrip
#[test]
fn test_hashmap_u64_task() {
    let mut map: HashMap<u64, Task> = HashMap::new();
    map.insert(
        1,
        make_task(
            1,
            "Hash Task 1",
            Priority::Medium,
            TaskStatus::InProgress,
            Some(55),
            vec!["hash"],
        ),
    );
    map.insert(
        2,
        make_task(
            2,
            "Hash Task 2",
            Priority::High,
            TaskStatus::Cancelled,
            None,
            vec![],
        ),
    );
    let encoded = encode_to_vec(&map).expect("encode failed");
    let (decoded, _): (HashMap<u64, Task>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(map, decoded);
}

// 13. VecDeque<Task> roundtrip (task queue)
#[test]
fn test_vecdeque_task_queue() {
    let mut queue: VecDeque<Task> = VecDeque::new();
    queue.push_back(make_task(
        1,
        "Queue Task 1",
        Priority::Critical,
        TaskStatus::Todo,
        None,
        vec!["urgent"],
    ));
    queue.push_back(make_task(
        2,
        "Queue Task 2",
        Priority::High,
        TaskStatus::Todo,
        Some(7),
        vec![],
    ));
    queue.push_back(make_task(
        3,
        "Queue Task 3",
        Priority::Low,
        TaskStatus::Todo,
        None,
        vec!["later"],
    ));
    let encoded = encode_to_vec(&queue).expect("encode failed");
    let (decoded, _): (VecDeque<Task>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(queue, decoded);
    assert_eq!(decoded.len(), 3);
}

// 14. Nested Vec<Vec<Task>> (tasks grouped by sprint)
#[test]
fn test_nested_vec_of_vec_task() {
    let sprints: Vec<Vec<Task>> = vec![
        vec![
            make_task(
                1,
                "Sprint1-T1",
                Priority::High,
                TaskStatus::Done,
                Some(1),
                vec!["s1"],
            ),
            make_task(
                2,
                "Sprint1-T2",
                Priority::Medium,
                TaskStatus::Done,
                Some(2),
                vec!["s1"],
            ),
        ],
        vec![make_task(
            3,
            "Sprint2-T1",
            Priority::Low,
            TaskStatus::InProgress,
            Some(3),
            vec!["s2"],
        )],
        vec![],
    ];
    let encoded = encode_to_vec(&sprints).expect("encode failed");
    let (decoded, _): (Vec<Vec<Task>>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(sprints, decoded);
    assert_eq!(decoded[2].len(), 0);
}

// 15. BTreeMap<String, Vec<Task>> (tasks grouped by label)
#[test]
fn test_btreemap_string_vec_task() {
    let mut map: BTreeMap<String, Vec<Task>> = BTreeMap::new();
    map.insert(
        "bug".to_string(),
        vec![make_task(
            1,
            "Fix null ptr",
            Priority::Critical,
            TaskStatus::Todo,
            None,
            vec!["bug"],
        )],
    );
    map.insert(
        "feature".to_string(),
        vec![
            make_task(
                2,
                "Add export",
                Priority::Medium,
                TaskStatus::InProgress,
                Some(9),
                vec!["feature"],
            ),
            make_task(
                3,
                "Add import",
                Priority::Medium,
                TaskStatus::Todo,
                Some(9),
                vec!["feature"],
            ),
        ],
    );
    map.insert("docs".to_string(), vec![]);
    let encoded = encode_to_vec(&map).expect("encode failed");
    let (decoded, _): (BTreeMap<String, Vec<Task>>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(map, decoded);
    assert_eq!(decoded["feature"].len(), 2);
}

// 16. encode_to_vec_with_config / decode_from_slice_with_config (standard config)
#[test]
fn test_task_with_standard_config() {
    let cfg = config::standard();
    let task = make_task(
        99,
        "Config Test Task",
        Priority::High,
        TaskStatus::InProgress,
        Some(77),
        vec!["config", "test"],
    );
    let encoded = encode_to_vec_with_config(&task, cfg).expect("encode failed");
    let (decoded, _): (Task, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(task, decoded);
}

// 17. encode_to_vec_with_config / decode_from_slice_with_config (little-endian config)
#[test]
fn test_project_with_little_endian_config() {
    let cfg = config::standard().with_little_endian();
    let project = Project {
        id: 777,
        name: "LE Project".to_string(),
        tasks: vec![make_task(
            1,
            "LE Task",
            Priority::Low,
            TaskStatus::Done,
            None,
            vec![],
        )],
        member_ids: vec![10, 20, 30],
    };
    let encoded = encode_to_vec_with_config(&project, cfg).expect("encode failed");
    let (decoded, _): (Project, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(project, decoded);
}

// 18. Consumed bytes reported correctly for a Task
#[test]
fn test_task_consumed_bytes() {
    let task = make_task(
        5,
        "Byte count task",
        Priority::Medium,
        TaskStatus::Done,
        Some(55),
        vec!["x", "y"],
    );
    let encoded = encode_to_vec(&task).expect("encode failed");
    let (_, consumed): (Task, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes should equal encoded length"
    );
}

// 19. Consumed bytes reported correctly for a Project
#[test]
fn test_project_consumed_bytes() {
    let project = Project {
        id: 42,
        name: "Consumed Bytes Project".to_string(),
        tasks: vec![make_task(
            1,
            "t1",
            Priority::Critical,
            TaskStatus::Cancelled,
            None,
            vec!["a"],
        )],
        member_ids: vec![1, 2, 3, 4, 5],
    };
    let encoded = encode_to_vec(&project).expect("encode failed");
    let (_, consumed): (Project, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(consumed, encoded.len());
}

// 20. Large Vec<Task> with many items
#[test]
fn test_large_vec_task() {
    let tasks: Vec<Task> = (0u64..100)
        .map(|i| {
            let priority = match i % 4 {
                0 => Priority::Low,
                1 => Priority::Medium,
                2 => Priority::High,
                _ => Priority::Critical,
            };
            let status = match i % 4 {
                0 => TaskStatus::Todo,
                1 => TaskStatus::InProgress,
                2 => TaskStatus::Done,
                _ => TaskStatus::Cancelled,
            };
            make_task(
                i,
                &format!("Task {}", i),
                priority,
                status,
                if i % 2 == 0 { Some(i * 10) } else { None },
                vec!["batch"],
            )
        })
        .collect();
    let encoded = encode_to_vec(&tasks).expect("encode failed");
    let (decoded, consumed): (Vec<Task>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(tasks.len(), decoded.len());
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded[50].id, 50);
}

// 21. Option<u64> field (assignee_id) — Some and None both survive roundtrip within same Vec
#[test]
fn test_option_assignee_id_variants() {
    let tasks: Vec<Task> = vec![
        make_task(
            1,
            "Assigned",
            Priority::High,
            TaskStatus::Todo,
            Some(1001),
            vec![],
        ),
        make_task(
            2,
            "Unassigned",
            Priority::Low,
            TaskStatus::Todo,
            None,
            vec![],
        ),
        make_task(
            3,
            "Reassigned",
            Priority::Medium,
            TaskStatus::InProgress,
            Some(9999),
            vec!["reassign"],
        ),
    ];
    let encoded = encode_to_vec(&tasks).expect("encode failed");
    let (decoded, _): (Vec<Task>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded[0].assignee_id, Some(1001));
    assert_eq!(decoded[1].assignee_id, None);
    assert_eq!(decoded[2].assignee_id, Some(9999));
}

// 22. BTreeMap<u64, BTreeSet<u64>> mapping project_id -> set of task_ids (nested collections)
#[test]
fn test_btreemap_project_to_task_id_set() {
    let mut project_tasks: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
    let mut set1: BTreeSet<u64> = BTreeSet::new();
    set1.insert(101);
    set1.insert(102);
    set1.insert(103);
    project_tasks.insert(1, set1);

    let mut set2: BTreeSet<u64> = BTreeSet::new();
    set2.insert(201);
    project_tasks.insert(2, set2);

    project_tasks.insert(3, BTreeSet::new());

    let encoded = encode_to_vec(&project_tasks).expect("encode failed");
    let (decoded, consumed): (BTreeMap<u64, BTreeSet<u64>>, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(project_tasks, decoded);
    assert_eq!(consumed, encoded.len());
    assert!(decoded[&1].contains(&102));
    assert!(decoded[&3].is_empty());
}
