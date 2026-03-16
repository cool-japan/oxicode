//! Advanced async streaming tests (eighteenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `JobTask` and `JobStatus`.
//!
//! Coverage matrix:
//!   1:  Single JobTask roundtrip via duplex pipe
//!   2:  Single JobStatus::Queued roundtrip via duplex pipe
//!   3:  Single JobStatus::Running roundtrip via duplex pipe
//!   4:  Single JobStatus::Done roundtrip via duplex pipe
//!   5:  Single JobStatus::Failed roundtrip via duplex pipe
//!   6:  Multiple JobTask items sequential write then sequential read
//!   7:  Multiple JobStatus items sequential write then sequential read
//!   8:  Empty read returns None after all items consumed (u32)
//!   9:  Empty read returns None after all items consumed (JobTask)
//!  10:  u32 async roundtrip via duplex
//!  11:  String async roundtrip via duplex
//!  12:  Vec<u8> async roundtrip via duplex
//!  13:  Vec<JobTask> async roundtrip (all fields varied)
//!  14:  Vec<JobStatus> all variants async roundtrip
//!  15:  Large Vec<u8> (4096 bytes) async roundtrip via duplex
//!  16:  Option<JobTask> Some async roundtrip
//!  17:  Option<JobTask> None async roundtrip
//!  18:  Job queue simulation: enqueue 5 tasks, dequeue all in order
//!  19:  Job queue simulation: mixed statuses written and read back
//!  20:  Zero-priority task roundtrip (boundary: priority = 0)
//!  21:  Max-retries task roundtrip (boundary: retries = u32::MAX)
//!  22:  Failed status with empty reason string roundtrip

#![cfg(feature = "async-tokio")]
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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};
use oxicode::{Decode, Encode};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, BufReader};

// ---------------------------------------------------------------------------
// Types unique to this file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct JobTask {
    job_id: u64,
    task_name: String,
    priority: u8,
    retries: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum JobStatus {
    Queued,
    Running { worker_id: u32 },
    Done(String),
    Failed { code: i32, reason: String },
}

// ---------------------------------------------------------------------------
// Helpers: encode via duplex pipe, decode via BufReader-wrapped Cursor.
// ---------------------------------------------------------------------------

async fn duplex_encode_single<T: Encode>(item: &T) -> Vec<u8> {
    let (writer, mut reader) = tokio::io::duplex(8192);
    let mut encoder = AsyncEncoder::new(writer);
    encoder
        .write_item(item)
        .await
        .expect("duplex_encode_single: write_item failed");
    encoder
        .finish()
        .await
        .expect("duplex_encode_single: finish failed");

    let mut buf = Vec::new();
    reader
        .read_to_end(&mut buf)
        .await
        .expect("duplex_encode_single: read_to_end failed");
    buf
}

async fn duplex_decode_single<T: Decode>(encoded: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(encoded);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);
    decoder
        .read_item::<T>()
        .await
        .expect("duplex_decode_single: read_item failed")
}

// ---------------------------------------------------------------------------
// Test 1: Single JobTask roundtrip via duplex pipe
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_task_single_roundtrip() {
    let original = JobTask {
        job_id: 1_000_000_001,
        task_name: String::from("compress-archive"),
        priority: 128,
        retries: 3,
    };
    let encoded = duplex_encode_single(&original).await;
    let decoded = duplex_decode_single::<JobTask>(encoded).await;
    assert_eq!(
        decoded,
        Some(original),
        "single JobTask roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Single JobStatus::Queued roundtrip via duplex pipe
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_status_queued_roundtrip() {
    let original = JobStatus::Queued;
    let encoded = duplex_encode_single(&original).await;
    let decoded = duplex_decode_single::<JobStatus>(encoded).await;
    assert_eq!(
        decoded,
        Some(JobStatus::Queued),
        "JobStatus::Queued roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Single JobStatus::Running roundtrip via duplex pipe
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_status_running_roundtrip() {
    let original = JobStatus::Running { worker_id: 42 };
    let encoded = duplex_encode_single(&original).await;
    let decoded = duplex_decode_single::<JobStatus>(encoded).await;
    assert_eq!(
        decoded,
        Some(JobStatus::Running { worker_id: 42 }),
        "JobStatus::Running roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Single JobStatus::Done roundtrip via duplex pipe
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_status_done_roundtrip() {
    let original = JobStatus::Done(String::from("output-file-abc123.tar.gz"));
    let encoded = duplex_encode_single(&original).await;
    let decoded = duplex_decode_single::<JobStatus>(encoded).await;
    assert_eq!(
        decoded,
        Some(JobStatus::Done(String::from("output-file-abc123.tar.gz"))),
        "JobStatus::Done roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Single JobStatus::Failed roundtrip via duplex pipe
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_status_failed_roundtrip() {
    let original = JobStatus::Failed {
        code: -127,
        reason: String::from("out of disk space"),
    };
    let encoded = duplex_encode_single(&original).await;
    let decoded = duplex_decode_single::<JobStatus>(encoded).await;
    assert_eq!(
        decoded,
        Some(JobStatus::Failed {
            code: -127,
            reason: String::from("out of disk space"),
        }),
        "JobStatus::Failed roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Multiple JobTask items sequential write then sequential read
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_multiple_job_tasks_sequential() {
    let tasks = vec![
        JobTask {
            job_id: 1,
            task_name: String::from("fetch-data"),
            priority: 10,
            retries: 0,
        },
        JobTask {
            job_id: 2,
            task_name: String::from("transform-data"),
            priority: 50,
            retries: 1,
        },
        JobTask {
            job_id: 3,
            task_name: String::from("upload-results"),
            priority: 90,
            retries: 2,
        },
    ];

    let (writer, mut pipe_reader) = tokio::io::duplex(8192);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for task in &tasks {
            encoder
                .write_item(task)
                .await
                .expect("write JobTask in sequence failed");
        }
        encoder
            .finish()
            .await
            .expect("finish encoder after multiple tasks failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("read_to_end from duplex reader failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    for (idx, expected) in tasks.iter().enumerate() {
        let decoded: Option<JobTask> = decoder
            .read_item()
            .await
            .expect("read sequential JobTask failed");
        assert_eq!(
            decoded.as_ref(),
            Some(expected),
            "sequential JobTask mismatch at index {idx}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 7: Multiple JobStatus items sequential write then sequential read
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_multiple_job_statuses_sequential() {
    let statuses = vec![
        JobStatus::Queued,
        JobStatus::Running { worker_id: 7 },
        JobStatus::Done(String::from("result-payload")),
        JobStatus::Failed {
            code: 1,
            reason: String::from("timeout"),
        },
    ];

    let (writer, mut pipe_reader) = tokio::io::duplex(8192);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for status in &statuses {
            encoder
                .write_item(status)
                .await
                .expect("write JobStatus in sequence failed");
        }
        encoder
            .finish()
            .await
            .expect("finish encoder after multiple statuses failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("read_to_end from duplex reader for statuses failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    for (idx, expected) in statuses.iter().enumerate() {
        let decoded: Option<JobStatus> = decoder
            .read_item()
            .await
            .expect("read sequential JobStatus failed");
        assert_eq!(
            decoded.as_ref(),
            Some(expected),
            "sequential JobStatus mismatch at index {idx}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 8: Empty read returns None after all items consumed (u32)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_empty_read_none_after_u32_consumed() {
    let values: Vec<u32> = vec![100, 200, 300];

    let (writer, mut pipe_reader) = tokio::io::duplex(4096);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for &v in &values {
            encoder
                .write_item(&v)
                .await
                .expect("write u32 in sequence failed");
        }
        encoder
            .finish()
            .await
            .expect("finish encoder for u32 stream failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("read_to_end for u32 stream failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    for &expected in &values {
        let item: Option<u32> = decoder.read_item().await.expect("read u32 item failed");
        assert_eq!(
            item,
            Some(expected),
            "u32 item mismatch: expected {expected}"
        );
    }

    let eof: Option<u32> = decoder
        .read_item()
        .await
        .expect("read after all u32 items failed");
    assert_eq!(eof, None, "expected None after all u32 items consumed");
}

// ---------------------------------------------------------------------------
// Test 9: Empty read returns None after all items consumed (JobTask)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_empty_read_none_after_job_tasks_consumed() {
    let tasks = vec![
        JobTask {
            job_id: 10,
            task_name: String::from("task-a"),
            priority: 1,
            retries: 0,
        },
        JobTask {
            job_id: 11,
            task_name: String::from("task-b"),
            priority: 2,
            retries: 0,
        },
    ];

    let (writer, mut pipe_reader) = tokio::io::duplex(8192);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for task in &tasks {
            encoder
                .write_item(task)
                .await
                .expect("write JobTask for eof-test failed");
        }
        encoder
            .finish()
            .await
            .expect("finish encoder for eof-test failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("read_to_end for eof-test failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    for (idx, expected) in tasks.iter().enumerate() {
        let decoded: Option<JobTask> = decoder
            .read_item()
            .await
            .expect("read JobTask for eof-test failed");
        assert_eq!(
            decoded.as_ref(),
            Some(expected),
            "JobTask mismatch at index {idx} in eof-test"
        );
    }

    let eof: Option<JobTask> = decoder
        .read_item()
        .await
        .expect("read after all JobTasks failed");
    assert_eq!(eof, None, "expected None after all JobTask items consumed");
}

// ---------------------------------------------------------------------------
// Test 10: u32 async roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_u32_roundtrip_duplex() {
    let val: u32 = 0xCAFE_F00D;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<u32>(encoded).await;
    assert_eq!(decoded, Some(val), "u32 async roundtrip via duplex failed");
}

// ---------------------------------------------------------------------------
// Test 11: String async roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_string_roundtrip_duplex() {
    let val = String::from("job-scheduler-v2-oxicode-async-wave18");
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<String>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "String async roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Vec<u8> async roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_vec_u8_roundtrip_duplex() {
    let val: Vec<u8> = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<u8>>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "Vec<u8> async roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Vec<JobTask> async roundtrip (all fields varied)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_vec_job_task_all_fields_varied_roundtrip() {
    let val: Vec<JobTask> = vec![
        JobTask {
            job_id: 0,
            task_name: String::from(""),
            priority: 0,
            retries: 0,
        },
        JobTask {
            job_id: u64::MAX,
            task_name: String::from("max-id-task"),
            priority: 255,
            retries: 1_000_000,
        },
        JobTask {
            job_id: 42,
            task_name: String::from("send-notification-batch"),
            priority: 64,
            retries: 5,
        },
    ];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<JobTask>>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "Vec<JobTask> all-fields-varied async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Vec<JobStatus> all variants async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_vec_job_status_all_variants_roundtrip() {
    let val: Vec<JobStatus> = vec![
        JobStatus::Queued,
        JobStatus::Running { worker_id: 1 },
        JobStatus::Running { worker_id: 99_999 },
        JobStatus::Done(String::from("success")),
        JobStatus::Failed {
            code: -1,
            reason: String::from("segmentation fault"),
        },
    ];
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Vec<JobStatus>>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "Vec<JobStatus> all-variants async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Large Vec<u8> (4096 bytes) async roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_large_vec_u8_4096_roundtrip_duplex() {
    let val: Vec<u8> = (0u16..4096).map(|i| (i % 256) as u8).collect();
    assert_eq!(val.len(), 4096, "test data must be exactly 4096 bytes");

    let (writer, mut pipe_reader) = tokio::io::duplex(16384);
    {
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&val)
            .await
            .expect("write large Vec<u8> via duplex failed");
        encoder
            .finish()
            .await
            .expect("finish encoder after large Vec<u8> failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("read_to_end for large Vec<u8> via duplex failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);
    let decoded: Option<Vec<u8>> = decoder
        .read_item()
        .await
        .expect("read large Vec<u8> via duplex failed");

    assert_eq!(
        decoded,
        Some(val),
        "large Vec<u8> (4096 bytes) async roundtrip via duplex failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Option<JobTask> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_option_job_task_some_roundtrip() {
    let val: Option<JobTask> = Some(JobTask {
        job_id: 7777,
        task_name: String::from("optional-task-present"),
        priority: 200,
        retries: 10,
    });
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Option<JobTask>>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "Option<JobTask> Some async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Option<JobTask> None async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_option_job_task_none_roundtrip() {
    let val: Option<JobTask> = None;
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<Option<JobTask>>(encoded).await;
    assert_eq!(
        decoded,
        Some(None),
        "Option<JobTask> None async roundtrip failed"
    );
    assert!(
        decoded.expect("outer Option must be Some").is_none(),
        "inner Option<JobTask> must be None"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Job queue simulation — enqueue 5 tasks, dequeue all in order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_queue_enqueue_dequeue_five_tasks() {
    let queue: Vec<JobTask> = (1u64..=5)
        .map(|i| JobTask {
            job_id: i,
            task_name: format!("queue-task-{i:03}"),
            priority: (i as u8) * 20,
            retries: (i as u32) - 1,
        })
        .collect();

    let (writer, mut pipe_reader) = tokio::io::duplex(8192);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for task in &queue {
            encoder
                .write_item(task)
                .await
                .expect("enqueue JobTask via duplex encoder failed");
        }
        encoder
            .finish()
            .await
            .expect("finish encoder after enqueue simulation failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("read_to_end after enqueue simulation failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    for (idx, expected) in queue.iter().enumerate() {
        let dequeued: Option<JobTask> = decoder
            .read_item()
            .await
            .expect("dequeue JobTask from simulation failed");
        assert_eq!(
            dequeued.as_ref(),
            Some(expected),
            "job queue simulation dequeue mismatch at position {idx} (job_id={})",
            expected.job_id
        );
    }

    let empty: Option<JobTask> = decoder
        .read_item()
        .await
        .expect("read after queue drain failed");
    assert_eq!(
        empty, None,
        "queue must be empty after dequeuing all 5 tasks"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Job queue simulation — mixed statuses written and read back
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_queue_mixed_statuses_simulation() {
    let statuses = vec![
        (1u64, JobStatus::Queued),
        (2u64, JobStatus::Running { worker_id: 3 }),
        (3u64, JobStatus::Done(String::from("artifact-v1.bin"))),
        (4u64, JobStatus::Queued),
        (
            5u64,
            JobStatus::Failed {
                code: 137,
                reason: String::from("OOM killed"),
            },
        ),
        (6u64, JobStatus::Running { worker_id: 8 }),
    ];

    let (writer, mut pipe_reader) = tokio::io::duplex(8192);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for (_, status) in &statuses {
            encoder
                .write_item(status)
                .await
                .expect("write JobStatus in mixed queue simulation failed");
        }
        encoder
            .finish()
            .await
            .expect("finish encoder for mixed queue simulation failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("read_to_end for mixed queue simulation failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    for (idx, (job_id, expected_status)) in statuses.iter().enumerate() {
        let decoded: Option<JobStatus> = decoder
            .read_item()
            .await
            .expect("read JobStatus in mixed queue simulation failed");
        assert_eq!(
            decoded.as_ref(),
            Some(expected_status),
            "mixed queue simulation mismatch at index {idx} (job_id={job_id})"
        );
    }

    let drain: Option<JobStatus> = decoder
        .read_item()
        .await
        .expect("read after mixed queue drain failed");
    assert_eq!(
        drain, None,
        "mixed queue must be empty after reading all statuses"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Zero-priority task roundtrip (boundary: priority = 0)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_task_zero_priority_boundary_roundtrip() {
    let val = JobTask {
        job_id: 999_999_999,
        task_name: String::from("lowest-priority-background-sweep"),
        priority: 0,
        retries: 0,
    };
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<JobTask>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "zero-priority JobTask boundary roundtrip failed"
    );
    assert_eq!(
        decoded.expect("decoded must be Some").priority,
        0,
        "decoded priority must be 0"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Max-retries task roundtrip (boundary: retries = u32::MAX)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_task_max_retries_boundary_roundtrip() {
    let val = JobTask {
        job_id: 1,
        task_name: String::from("perpetual-retry-task"),
        priority: 255,
        retries: u32::MAX,
    };
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<JobTask>(encoded).await;
    assert_eq!(
        decoded,
        Some(val),
        "max-retries JobTask boundary roundtrip failed"
    );
    assert_eq!(
        decoded.expect("decoded must be Some").retries,
        u32::MAX,
        "decoded retries must be u32::MAX"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Failed status with empty reason string roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async18_job_status_failed_empty_reason_roundtrip() {
    let val = JobStatus::Failed {
        code: 0,
        reason: String::new(),
    };
    let encoded = duplex_encode_single(&val).await;
    let decoded = duplex_decode_single::<JobStatus>(encoded).await;
    assert_eq!(
        decoded,
        Some(JobStatus::Failed {
            code: 0,
            reason: String::new(),
        }),
        "JobStatus::Failed with empty reason async roundtrip failed"
    );
    if let Some(JobStatus::Failed { code, reason }) = decoded {
        assert_eq!(code, 0, "error code must be 0");
        assert!(reason.is_empty(), "reason string must be empty");
    } else {
        panic!("decoded value must be JobStatus::Failed");
    }
}
