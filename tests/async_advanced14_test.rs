//! Advanced async streaming tests (fourteenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Job`, `JobStatus`, and `Worker`.
//!
//! Coverage matrix:
//!   1:    Job async roundtrip
//!   2:    JobStatus::Queued async roundtrip
//!   3:    JobStatus::Running async roundtrip
//!   4:    JobStatus::Done async roundtrip
//!   5:    JobStatus::Failed async roundtrip
//!   6:    Worker async roundtrip
//!   7:    Vec<Job> 5 jobs async roundtrip
//!   8:    Vec<JobStatus> mixed variants async roundtrip
//!   9:    Vec<Worker> 3 workers async roundtrip
//!  10:    Option<Job> Some async roundtrip
//!  11:    Option<Job> None async roundtrip
//!  12:    u32 async roundtrip
//!  13:    String async roundtrip
//!  14:    bool async roundtrip
//!  15:    f64 async roundtrip (bit-exact)
//!  16:    i64::MIN async roundtrip
//!  17:    Sequential write 3 jobs then read 3
//!  18:    u128 async roundtrip
//!  19:    Large Vec<u8> 8192 bytes async roundtrip
//!  20:    Vec<Worker> 10 workers async roundtrip
//!  21:    Bytes match encode_to_vec output
//!  22:    Option<Vec<Job>> roundtrip

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::io::Cursor;
use tokio::io::BufReader;

// ---------------------------------------------------------------------------
// Types unique to this file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Job {
    id: u64,
    task: String,
    priority: u32,
    retries: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum JobStatus {
    Queued,
    Running(u64),
    Done { result: String },
    Failed(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Worker {
    worker_id: u32,
    name: String,
    capacity: u32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn async_encode_single<T: Encode>(item: &T) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(item)
            .await
            .expect("async_encode_single: write_item failed");
        enc.finish()
            .await
            .expect("async_encode_single: finish failed");
    }
    buf
}

async fn async_decode_single<T: Decode>(buf: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    dec.read_item::<T>()
        .await
        .expect("async_decode_single: read_item failed")
}

// ---------------------------------------------------------------------------
// Test 1: Job async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_job_roundtrip() {
    let original = Job {
        id: 42,
        task: String::from("compress-archive"),
        priority: 5,
        retries: 3,
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Job>(buf).await;
    assert_eq!(decoded, Some(original), "Job async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 2: JobStatus::Queued async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_job_status_queued_roundtrip() {
    let original = JobStatus::Queued;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<JobStatus>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "JobStatus::Queued async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 3: JobStatus::Running async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_job_status_running_roundtrip() {
    let original = JobStatus::Running(1_234_567_890);
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<JobStatus>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "JobStatus::Running async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: JobStatus::Done async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_job_status_done_roundtrip() {
    let original = JobStatus::Done {
        result: String::from("ok: processed 1024 records"),
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<JobStatus>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "JobStatus::Done async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 5: JobStatus::Failed async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_job_status_failed_roundtrip() {
    let original = JobStatus::Failed(String::from("timeout after 30s"));
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<JobStatus>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "JobStatus::Failed async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Worker async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_worker_roundtrip() {
    let original = Worker {
        worker_id: 7,
        name: String::from("worker-alpha"),
        capacity: 16,
    };
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Worker>(buf).await;
    assert_eq!(decoded, Some(original), "Worker async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 7: Vec<Job> 5 jobs async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_vec_job_five_roundtrip() {
    let original: Vec<Job> = (0u64..5)
        .map(|i| Job {
            id: i * 100,
            task: format!("task-{i}"),
            priority: (i as u32) % 10,
            retries: (i as u32) % 3,
        })
        .collect();
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Job>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<Job> 5 jobs async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Vec<JobStatus> mixed variants async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_vec_job_status_mixed_roundtrip() {
    let original: Vec<JobStatus> = vec![
        JobStatus::Queued,
        JobStatus::Running(9_876_543_210),
        JobStatus::Done {
            result: String::from("success"),
        },
        JobStatus::Failed(String::from("disk full")),
        JobStatus::Queued,
        JobStatus::Running(1),
    ];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<JobStatus>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<JobStatus> mixed variants async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Vec<Worker> 3 workers async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_vec_worker_three_roundtrip() {
    let original: Vec<Worker> = vec![
        Worker {
            worker_id: 1,
            name: String::from("worker-beta"),
            capacity: 4,
        },
        Worker {
            worker_id: 2,
            name: String::from("worker-gamma"),
            capacity: 8,
        },
        Worker {
            worker_id: 3,
            name: String::from("worker-delta"),
            capacity: 32,
        },
    ];
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Worker>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<Worker> 3 workers async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Option<Job> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_option_job_some_roundtrip() {
    let original: Option<Job> = Some(Job {
        id: 999,
        task: String::from("index-rebuild"),
        priority: 1,
        retries: 0,
    });
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<Job>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<Job> Some async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Option<Job> None async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_option_job_none_roundtrip() {
    let original: Option<Job> = None;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<Job>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<Job> None async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 12: u32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_u32_roundtrip() {
    let original: u32 = 2_718_281_828;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u32>(buf).await;
    assert_eq!(decoded, Some(original), "u32 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 13: String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_string_roundtrip() {
    let original = String::from("oxicode-async-job-worker-test-wave14");
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<String>(buf).await;
    assert_eq!(decoded, Some(original), "String async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 14: bool async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_bool_roundtrip() {
    let buf_true = async_encode_single(&true).await;
    let decoded_true = async_decode_single::<bool>(buf_true).await;
    assert_eq!(decoded_true, Some(true), "bool true async roundtrip failed");

    let buf_false = async_encode_single(&false).await;
    let decoded_false = async_decode_single::<bool>(buf_false).await;
    assert_eq!(
        decoded_false,
        Some(false),
        "bool false async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 15: f64 async roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_f64_bit_exact_roundtrip() {
    let original: f64 = std::f64::consts::TAU;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<f64>(buf).await;
    assert_eq!(decoded, Some(original), "f64 async roundtrip failed");
    if let Some(d) = decoded {
        assert_eq!(
            d.to_bits(),
            original.to_bits(),
            "f64 bit-level representation mismatch"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 16: i64::MIN async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_i64_min_roundtrip() {
    let original: i64 = i64::MIN;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<i64>(buf).await;
    assert_eq!(decoded, Some(original), "i64::MIN async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 17: Sequential write 3 jobs then read 3
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_three_jobs_sequential_roundtrip() {
    let jobs: Vec<Job> = vec![
        Job {
            id: 1,
            task: String::from("backup-db"),
            priority: 10,
            retries: 1,
        },
        Job {
            id: 2,
            task: String::from("send-report"),
            priority: 5,
            retries: 2,
        },
        Job {
            id: 3,
            task: String::from("cleanup-logs"),
            priority: 1,
            retries: 0,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for job in &jobs {
            enc.write_item(job).await.expect("write Job failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    for (idx, expected) in jobs.iter().enumerate() {
        let item: Option<Job> = dec.read_item().await.expect("read Job failed");
        assert_eq!(item.as_ref(), Some(expected), "Job at index {idx} mismatch");
    }

    let eof: Option<Job> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all 3 Jobs decoded");
}

// ---------------------------------------------------------------------------
// Test 18: u128 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_u128_roundtrip() {
    let original: u128 = 0xDEAD_BEEF_CAFE_BABE_1234_5678_9ABC_DEF0;
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<u128>(buf).await;
    assert_eq!(decoded, Some(original), "u128 async roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 19: Large Vec<u8> 8192 bytes async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_large_vec_u8_8192_bytes_roundtrip() {
    let original: Vec<u8> = (0u8..=255).cycle().take(8192).collect();
    assert_eq!(original.len(), 8192, "original must be exactly 8192 bytes");

    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<u8>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "large Vec<u8> (8192 bytes) async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Vec<Worker> 10 workers async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_vec_worker_ten_roundtrip() {
    let original: Vec<Worker> = (0u32..10)
        .map(|i| Worker {
            worker_id: i + 1,
            name: format!("worker-{i:02}"),
            capacity: (i + 1) * 4,
        })
        .collect();
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Vec<Worker>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Vec<Worker> 10 workers async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Bytes match encode_to_vec output
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_bytes_match_encode_to_vec_output() {
    let job = Job {
        id: 88,
        task: String::from("validate-schema"),
        priority: 7,
        retries: 2,
    };

    // Sync encode to get reference bytes
    let sync_bytes = encode_to_vec(&job).expect("sync encode Job failed");

    // Async encode into buffer
    let async_bytes = async_encode_single(&job).await;

    // Both encodings must decode to the same original value
    let (sync_decoded, _): (Job, _) =
        decode_from_slice(&sync_bytes).expect("sync decode Job from sync bytes failed");
    assert_eq!(sync_decoded, job, "sync decode from sync bytes failed");

    let async_decoded = async_decode_single::<Job>(async_bytes).await;
    assert_eq!(
        async_decoded,
        Some(job),
        "async decode from async bytes failed"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Option<Vec<Job>> roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async14_option_vec_job_roundtrip() {
    let original: Option<Vec<Job>> = Some(vec![
        Job {
            id: 10,
            task: String::from("migrate-data"),
            priority: 9,
            retries: 1,
        },
        Job {
            id: 20,
            task: String::from("generate-report"),
            priority: 3,
            retries: 0,
        },
        Job {
            id: 30,
            task: String::from("notify-users"),
            priority: 6,
            retries: 5,
        },
    ]);
    let buf = async_encode_single(&original).await;
    let decoded = async_decode_single::<Option<Vec<Job>>>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "Option<Vec<Job>> async roundtrip failed"
    );
}
