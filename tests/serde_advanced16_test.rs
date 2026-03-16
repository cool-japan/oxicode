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
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct TaskQueue {
    queue_name: String,
    max_size: u32,
    timeout_ms: u64,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum TaskPriority {
    Low,
    Normal,
    High,
    Critical { deadline_ms: u64 },
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct WorkItem {
    item_id: u64,
    queue: String,
    priority: TaskPriority,
    payload: Vec<u8>,
    retry_count: u8,
}

fn make_task_queue() -> TaskQueue {
    TaskQueue {
        queue_name: "default".to_string(),
        max_size: 1000,
        timeout_ms: 30000,
        tags: vec!["urgent".to_string(), "batch".to_string()],
    }
}

fn make_work_item(id: u64, priority: TaskPriority) -> WorkItem {
    WorkItem {
        item_id: id,
        queue: "default".to_string(),
        priority,
        payload: vec![0xde, 0xad, 0xbe, 0xef],
        retry_count: 0,
    }
}

#[test]
fn test_task_queue_roundtrip() {
    let tq = make_task_queue();
    let bytes = encode_to_vec(&tq, config::standard()).expect("encode TaskQueue");
    let (decoded, _): (TaskQueue, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode TaskQueue");
    assert_eq!(tq, decoded);
}

#[test]
fn test_task_queue_empty_tags() {
    let tq = TaskQueue {
        queue_name: "empty".to_string(),
        max_size: 0,
        timeout_ms: 0,
        tags: vec![],
    };
    let bytes = encode_to_vec(&tq, config::standard()).expect("encode empty-tags TaskQueue");
    let (decoded, _): (TaskQueue, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode empty-tags TaskQueue");
    assert_eq!(tq, decoded);
}

#[test]
fn test_task_priority_low_roundtrip() {
    let p = TaskPriority::Low;
    let bytes = encode_to_vec(&p, config::standard()).expect("encode Low");
    let (decoded, _): (TaskPriority, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Low");
    assert_eq!(p, decoded);
}

#[test]
fn test_task_priority_normal_roundtrip() {
    let p = TaskPriority::Normal;
    let bytes = encode_to_vec(&p, config::standard()).expect("encode Normal");
    let (decoded, _): (TaskPriority, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Normal");
    assert_eq!(p, decoded);
}

#[test]
fn test_task_priority_high_roundtrip() {
    let p = TaskPriority::High;
    let bytes = encode_to_vec(&p, config::standard()).expect("encode High");
    let (decoded, _): (TaskPriority, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode High");
    assert_eq!(p, decoded);
}

#[test]
fn test_task_priority_critical_roundtrip() {
    let p = TaskPriority::Critical {
        deadline_ms: 999999,
    };
    let bytes = encode_to_vec(&p, config::standard()).expect("encode Critical");
    let (decoded, _): (TaskPriority, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Critical");
    assert_eq!(p, decoded);
}

#[test]
fn test_work_item_low_priority_roundtrip() {
    let wi = make_work_item(1, TaskPriority::Low);
    let bytes = encode_to_vec(&wi, config::standard()).expect("encode WorkItem Low");
    let (decoded, _): (WorkItem, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode WorkItem Low");
    assert_eq!(wi, decoded);
}

#[test]
fn test_work_item_critical_priority_roundtrip() {
    let wi = make_work_item(42, TaskPriority::Critical { deadline_ms: 5000 });
    let bytes = encode_to_vec(&wi, config::standard()).expect("encode WorkItem Critical");
    let (decoded, _): (WorkItem, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode WorkItem Critical");
    assert_eq!(wi, decoded);
}

#[test]
fn test_vec_work_items_roundtrip() {
    let items = vec![
        make_work_item(1, TaskPriority::Low),
        make_work_item(2, TaskPriority::Normal),
        make_work_item(3, TaskPriority::High),
        make_work_item(4, TaskPriority::Critical { deadline_ms: 1000 }),
    ];
    let bytes = encode_to_vec(&items, config::standard()).expect("encode Vec<WorkItem>");
    let (decoded, _): (Vec<WorkItem>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Vec<WorkItem>");
    assert_eq!(items, decoded);
}

#[test]
fn test_option_work_item_some_roundtrip() {
    let maybe: Option<WorkItem> = Some(make_work_item(7, TaskPriority::High));
    let bytes = encode_to_vec(&maybe, config::standard()).expect("encode Some(WorkItem)");
    let (decoded, _): (Option<WorkItem>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Some(WorkItem)");
    assert_eq!(maybe, decoded);
}

#[test]
fn test_option_work_item_none_roundtrip() {
    let maybe: Option<WorkItem> = None;
    let bytes = encode_to_vec(&maybe, config::standard()).expect("encode None<WorkItem>");
    let (decoded, _): (Option<WorkItem>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode None<WorkItem>");
    assert_eq!(maybe, decoded);
}

#[test]
fn test_consumed_bytes_nonzero() {
    let tq = make_task_queue();
    let bytes = encode_to_vec(&tq, config::standard()).expect("encode for consumed-bytes check");
    let (_decoded, consumed): (TaskQueue, usize) =
        decode_owned_from_slice(&bytes, config::standard())
            .expect("decode for consumed-bytes check");
    assert!(consumed > 0, "consumed bytes must be positive");
    assert_eq!(consumed, bytes.len(), "all bytes should be consumed");
}

#[test]
fn test_consumed_bytes_work_item() {
    let wi = make_work_item(99, TaskPriority::Normal);
    let bytes = encode_to_vec(&wi, config::standard()).expect("encode WorkItem for consumed check");
    let (_decoded, consumed): (WorkItem, usize) =
        decode_owned_from_slice(&bytes, config::standard())
            .expect("decode WorkItem for consumed check");
    assert_eq!(
        consumed,
        bytes.len(),
        "all bytes of WorkItem should be consumed"
    );
}

#[test]
fn test_fixed_int_config_task_queue() {
    let tq = make_task_queue();
    let bytes = encode_to_vec(&tq, config::standard().with_fixed_int_encoding())
        .expect("encode fixed-int TaskQueue");
    let (decoded, _): (TaskQueue, usize) =
        decode_owned_from_slice(&bytes, config::standard().with_fixed_int_encoding())
            .expect("decode fixed-int TaskQueue");
    assert_eq!(tq, decoded);
}

#[test]
fn test_fixed_int_config_work_item() {
    let wi = make_work_item(12, TaskPriority::Critical { deadline_ms: 2048 });
    let bytes = encode_to_vec(&wi, config::standard().with_fixed_int_encoding())
        .expect("encode fixed-int WorkItem");
    let (decoded, _): (WorkItem, usize) =
        decode_owned_from_slice(&bytes, config::standard().with_fixed_int_encoding())
            .expect("decode fixed-int WorkItem");
    assert_eq!(wi, decoded);
}

#[test]
fn test_big_endian_config_task_queue() {
    let tq = make_task_queue();
    let bytes = encode_to_vec(&tq, config::standard().with_big_endian())
        .expect("encode big-endian TaskQueue");
    let (decoded, _): (TaskQueue, usize) =
        decode_owned_from_slice(&bytes, config::standard().with_big_endian())
            .expect("decode big-endian TaskQueue");
    assert_eq!(tq, decoded);
}

#[test]
fn test_big_endian_config_work_item() {
    let wi = make_work_item(77, TaskPriority::High);
    let bytes = encode_to_vec(&wi, config::standard().with_big_endian())
        .expect("encode big-endian WorkItem");
    let (decoded, _): (WorkItem, usize) =
        decode_owned_from_slice(&bytes, config::standard().with_big_endian())
            .expect("decode big-endian WorkItem");
    assert_eq!(wi, decoded);
}

#[test]
fn test_primitive_u64_roundtrip() {
    let val: u64 = u64::MAX;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode u64::MAX");
    let (decoded, _): (u64, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode u64::MAX");
    assert_eq!(val, decoded);
}

#[test]
fn test_primitive_string_roundtrip() {
    let val = "task-queue-integration-test".to_string();
    let bytes = encode_to_vec(&val, config::standard()).expect("encode String");
    let (decoded, _): (String, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode String");
    assert_eq!(val, decoded);
}

#[test]
fn test_empty_vec_u8_payload() {
    let wi = WorkItem {
        item_id: 0,
        queue: "q".to_string(),
        priority: TaskPriority::Low,
        payload: vec![],
        retry_count: 0,
    };
    let bytes = encode_to_vec(&wi, config::standard()).expect("encode empty-payload WorkItem");
    let (decoded, _): (WorkItem, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode empty-payload WorkItem");
    assert_eq!(wi, decoded);
}

#[test]
fn test_empty_vec_work_items() {
    let items: Vec<WorkItem> = vec![];
    let bytes = encode_to_vec(&items, config::standard()).expect("encode empty Vec<WorkItem>");
    let (decoded, _): (Vec<WorkItem>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode empty Vec<WorkItem>");
    assert_eq!(items, decoded);
}

#[test]
fn test_large_payload_work_item() {
    let large_payload: Vec<u8> = (0u8..=255u8).cycle().take(4096).collect();
    let wi = WorkItem {
        item_id: 9999,
        queue: "large-payload-queue".to_string(),
        priority: TaskPriority::Critical {
            deadline_ms: u64::MAX / 2,
        },
        payload: large_payload,
        retry_count: 255,
    };
    let bytes = encode_to_vec(&wi, config::standard()).expect("encode large-payload WorkItem");
    let (decoded, consumed): (WorkItem, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode large-payload WorkItem");
    assert_eq!(wi, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "all bytes consumed for large payload"
    );
}

#[test]
fn test_many_tags_task_queue() {
    let tq = TaskQueue {
        queue_name: "multi-tag-queue".to_string(),
        max_size: u32::MAX,
        timeout_ms: u64::MAX,
        tags: (0..50).map(|i| format!("tag-{i}")).collect(),
    };
    let bytes = encode_to_vec(&tq, config::standard()).expect("encode many-tags TaskQueue");
    let (decoded, consumed): (TaskQueue, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode many-tags TaskQueue");
    assert_eq!(tq, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "all bytes consumed for many-tags TaskQueue"
    );
}
