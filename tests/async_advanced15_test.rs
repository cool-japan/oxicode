//! Advanced async streaming tests (fifteenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Pipeline` and `PipelineStatus`.
//!
//! Coverage matrix:
//!   1:  Pipeline async roundtrip
//!   2:  PipelineStatus::Pending async roundtrip
//!   3:  PipelineStatus::Running async roundtrip
//!   4:  PipelineStatus::Completed async roundtrip
//!   5:  PipelineStatus::Failed async roundtrip
//!   6:  u32 async roundtrip
//!   7:  String async roundtrip
//!   8:  Vec<u8> async roundtrip
//!   9:  bool true async roundtrip
//!  10:  bool false async roundtrip
//!  11:  u64 async roundtrip
//!  12:  f32 async roundtrip
//!  13:  f64 async roundtrip
//!  14:  Option<String> Some async roundtrip
//!  15:  Option<String> None async roundtrip
//!  16:  Vec<Pipeline> 3-item async roundtrip
//!  17:  Vec<PipelineStatus> 4-variant async roundtrip
//!  18:  i32 negative async roundtrip
//!  19:  Empty Vec<u8> async roundtrip
//!  20:  Large Vec<u8> 1000 bytes async roundtrip
//!  21:  Tuple (u32, String) async roundtrip
//!  22:  Sequential writes — write 3 values, read back 3 values

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
use tokio::io::BufReader;

// ---------------------------------------------------------------------------
// Types unique to this file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pipeline {
    id: u32,
    name: String,
    stages: Vec<String>,
    enabled: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PipelineStatus {
    Pending,
    Running(u32),
    Completed { duration_ms: u64 },
    Failed(String),
}

// ---------------------------------------------------------------------------
// Test 1: Pipeline async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_pipeline_roundtrip() {
    let val = Pipeline {
        id: 1,
        name: String::from("build-and-test"),
        stages: vec![
            String::from("checkout"),
            String::from("build"),
            String::from("test"),
        ],
        enabled: true,
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Pipeline");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Pipeline = dec
        .read_item()
        .await
        .expect("read Pipeline")
        .expect("expected Some(Pipeline)");

    assert_eq!(val, decoded, "Pipeline async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: PipelineStatus::Pending async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_pipeline_status_pending_roundtrip() {
    let val = PipelineStatus::Pending;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write PipelineStatus::Pending");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: PipelineStatus = dec
        .read_item()
        .await
        .expect("read PipelineStatus::Pending")
        .expect("expected Some(PipelineStatus)");

    assert_eq!(
        val, decoded,
        "PipelineStatus::Pending async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 3: PipelineStatus::Running async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_pipeline_status_running_roundtrip() {
    let val = PipelineStatus::Running(42);

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write PipelineStatus::Running");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: PipelineStatus = dec
        .read_item()
        .await
        .expect("read PipelineStatus::Running")
        .expect("expected Some(PipelineStatus)");

    assert_eq!(
        val, decoded,
        "PipelineStatus::Running async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 4: PipelineStatus::Completed async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_pipeline_status_completed_roundtrip() {
    let val = PipelineStatus::Completed {
        duration_ms: 12_345_678,
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write PipelineStatus::Completed");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: PipelineStatus = dec
        .read_item()
        .await
        .expect("read PipelineStatus::Completed")
        .expect("expected Some(PipelineStatus)");

    assert_eq!(
        val, decoded,
        "PipelineStatus::Completed async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 5: PipelineStatus::Failed async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_pipeline_status_failed_roundtrip() {
    let val = PipelineStatus::Failed(String::from("out of memory during build step"));

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write PipelineStatus::Failed");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: PipelineStatus = dec
        .read_item()
        .await
        .expect("read PipelineStatus::Failed")
        .expect("expected Some(PipelineStatus)");

    assert_eq!(
        val, decoded,
        "PipelineStatus::Failed async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 6: u32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_u32_roundtrip() {
    let val: u32 = 3_141_592_653;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write u32");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: u32 = dec
        .read_item()
        .await
        .expect("read u32")
        .expect("expected Some(u32)");

    assert_eq!(val, decoded, "u32 async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 7: String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_string_roundtrip() {
    let val = String::from("oxicode-async-pipeline-wave15");

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write String");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: String = dec
        .read_item()
        .await
        .expect("read String")
        .expect("expected Some(String)");

    assert_eq!(val, decoded, "String async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 8: Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_vec_u8_roundtrip() {
    let val: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80];

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Vec<u8>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<u8> = dec
        .read_item()
        .await
        .expect("read Vec<u8>")
        .expect("expected Some(Vec<u8>)");

    assert_eq!(val, decoded, "Vec<u8> async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 9: bool true async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_bool_true_roundtrip() {
    let val: bool = true;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write bool true");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: bool = dec
        .read_item()
        .await
        .expect("read bool true")
        .expect("expected Some(bool)");

    assert_eq!(val, decoded, "bool true async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 10: bool false async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_bool_false_roundtrip() {
    let val: bool = false;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write bool false");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: bool = dec
        .read_item()
        .await
        .expect("read bool false")
        .expect("expected Some(bool)");

    assert_eq!(val, decoded, "bool false async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 11: u64 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_u64_roundtrip() {
    let val: u64 = 18_446_744_073_709_551_000;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write u64");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: u64 = dec
        .read_item()
        .await
        .expect("read u64")
        .expect("expected Some(u64)");

    assert_eq!(val, decoded, "u64 async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 12: f32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_f32_roundtrip() {
    let val: f32 = std::f32::consts::PI;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write f32");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: f32 = dec
        .read_item()
        .await
        .expect("read f32")
        .expect("expected Some(f32)");

    assert_eq!(
        val.to_bits(),
        decoded.to_bits(),
        "f32 async roundtrip bit mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 13: f64 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_f64_roundtrip() {
    let val: f64 = std::f64::consts::E;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write f64");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: f64 = dec
        .read_item()
        .await
        .expect("read f64")
        .expect("expected Some(f64)");

    assert_eq!(
        val.to_bits(),
        decoded.to_bits(),
        "f64 async roundtrip bit mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Option<String> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_option_string_some_roundtrip() {
    let val: Option<String> = Some(String::from("pipeline-deploy-prod"));

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Option<String> Some");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Option<String> = dec
        .read_item()
        .await
        .expect("read Option<String> Some")
        .expect("expected Some(Option<String>)");

    assert_eq!(val, decoded, "Option<String> Some async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: Option<String> None async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_option_string_none_roundtrip() {
    let val: Option<String> = None;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Option<String> None");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Option<String> = dec
        .read_item()
        .await
        .expect("read Option<String> None")
        .expect("expected Some(Option<String>)");

    assert_eq!(val, decoded, "Option<String> None async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 16: Vec<Pipeline> 3-item async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_vec_pipeline_three_roundtrip() {
    let val: Vec<Pipeline> = vec![
        Pipeline {
            id: 100,
            name: String::from("ci-pipeline"),
            stages: vec![String::from("lint"), String::from("test")],
            enabled: true,
        },
        Pipeline {
            id: 200,
            name: String::from("cd-pipeline"),
            stages: vec![
                String::from("build"),
                String::from("push"),
                String::from("deploy"),
            ],
            enabled: true,
        },
        Pipeline {
            id: 300,
            name: String::from("nightly-pipeline"),
            stages: vec![String::from("benchmark"), String::from("report")],
            enabled: false,
        },
    ];

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Vec<Pipeline>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<Pipeline> = dec
        .read_item()
        .await
        .expect("read Vec<Pipeline>")
        .expect("expected Some(Vec<Pipeline>)");

    assert_eq!(
        val, decoded,
        "Vec<Pipeline> 3-item async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Vec<PipelineStatus> 4-variant async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_vec_pipeline_status_four_variants_roundtrip() {
    let val: Vec<PipelineStatus> = vec![
        PipelineStatus::Pending,
        PipelineStatus::Running(7),
        PipelineStatus::Completed {
            duration_ms: 98_765,
        },
        PipelineStatus::Failed(String::from("network timeout")),
    ];

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Vec<PipelineStatus>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<PipelineStatus> = dec
        .read_item()
        .await
        .expect("read Vec<PipelineStatus>")
        .expect("expected Some(Vec<PipelineStatus>)");

    assert_eq!(
        val, decoded,
        "Vec<PipelineStatus> 4-variant async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 18: i32 negative async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_i32_negative_roundtrip() {
    let val: i32 = -2_147_483_647;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write i32 negative");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: i32 = dec
        .read_item()
        .await
        .expect("read i32 negative")
        .expect("expected Some(i32)");

    assert_eq!(val, decoded, "i32 negative async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 19: Empty Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_empty_vec_u8_roundtrip() {
    let val: Vec<u8> = Vec::new();

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write empty Vec<u8>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<u8> = dec
        .read_item()
        .await
        .expect("read empty Vec<u8>")
        .expect("expected Some(Vec<u8>)");

    assert_eq!(val, decoded, "empty Vec<u8> async roundtrip mismatch");
    assert!(decoded.is_empty(), "decoded Vec<u8> must be empty");
}

// ---------------------------------------------------------------------------
// Test 20: Large Vec<u8> 1000 bytes async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_large_vec_u8_1000_bytes_roundtrip() {
    let val: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    assert_eq!(val.len(), 1000, "test data must be exactly 1000 bytes");

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write large Vec<u8>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<u8> = dec
        .read_item()
        .await
        .expect("read large Vec<u8>")
        .expect("expected Some(Vec<u8>)");

    assert_eq!(
        val, decoded,
        "large Vec<u8> (1000 bytes) async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Tuple (u32, String) async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_tuple_u32_string_roundtrip() {
    let val: (u32, String) = (9_999, String::from("pipeline-stage-label"));

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write (u32, String)");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: (u32, String) = dec
        .read_item()
        .await
        .expect("read (u32, String)")
        .expect("expected Some((u32, String))");

    assert_eq!(val, decoded, "(u32, String) async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 22: Sequential writes — write 3 values, read back 3 values
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async15_sequential_three_values_roundtrip() {
    let pipeline = Pipeline {
        id: 55,
        name: String::from("integration-tests"),
        stages: vec![
            String::from("setup"),
            String::from("run"),
            String::from("teardown"),
        ],
        enabled: true,
    };
    let status = PipelineStatus::Completed { duration_ms: 4_200 };
    let count: u32 = 777;

    let mut buf = Vec::<u8>::new();
    {
        let mut enc = AsyncEncoder::new(&mut buf);
        enc.write_item(&pipeline).await.expect("write Pipeline");
        enc.write_item(&status).await.expect("write PipelineStatus");
        enc.write_item(&count).await.expect("write u32");
        enc.finish().await.expect("finish encoder");
    }

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);

    let decoded_pipeline: Pipeline = dec
        .read_item()
        .await
        .expect("read Pipeline")
        .expect("expected Some(Pipeline)");
    let decoded_status: PipelineStatus = dec
        .read_item()
        .await
        .expect("read PipelineStatus")
        .expect("expected Some(PipelineStatus)");
    let decoded_count: u32 = dec
        .read_item()
        .await
        .expect("read u32")
        .expect("expected Some(u32)");

    assert_eq!(
        pipeline, decoded_pipeline,
        "sequential read: Pipeline mismatch"
    );
    assert_eq!(
        status, decoded_status,
        "sequential read: PipelineStatus mismatch"
    );
    assert_eq!(count, decoded_count, "sequential read: u32 mismatch");

    let eof: Option<u32> = dec.read_item().await.expect("read after end of stream");
    assert_eq!(eof, None, "expected None after all 3 values decoded");
}
