//! Advanced async encoding tests (seventeenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `CacheEntry` and `CacheOp`.
//!
//! Coverage matrix:
//!   1:  CacheEntry async roundtrip
//!   2:  CacheOp::Get async roundtrip
//!   3:  CacheOp::Set async roundtrip
//!   4:  CacheOp::Delete async roundtrip
//!   5:  CacheOp::Flush async roundtrip
//!   6:  CacheOp::Stats async roundtrip
//!   7:  Vec<CacheEntry> 4 items async roundtrip
//!   8:  Vec<CacheOp> all 5 variants async roundtrip
//!   9:  Option<CacheEntry> Some async roundtrip
//!  10:  Option<CacheEntry> None async roundtrip
//!  11:  u32 async roundtrip
//!  12:  u64 async roundtrip
//!  13:  String async roundtrip
//!  14:  bool async roundtrip (both values)
//!  15:  Vec<u8> 256 bytes async roundtrip
//!  16:  Empty Vec<u8> async roundtrip
//!  17:  i32 negative async roundtrip
//!  18:  f32 async bit-exact roundtrip
//!  19:  Sequential write 3 CacheOps, read back 3
//!  20:  u128 async roundtrip
//!  21:  CacheEntry with empty value
//!  22:  (String, u32) tuple async roundtrip

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
struct CacheEntry {
    key: String,
    value: Vec<u8>,
    ttl_secs: u32,
    compressed: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CacheOp {
    Get(String),
    Set(String, Vec<u8>),
    Delete(String),
    Flush,
    Stats { hits: u64, misses: u64 },
}

// ---------------------------------------------------------------------------
// Test 1: CacheEntry async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_cache_entry_roundtrip() {
    let val = CacheEntry {
        key: String::from("session:user:42"),
        value: vec![1, 2, 3, 4, 5],
        ttl_secs: 300,
        compressed: false,
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write CacheEntry");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: CacheEntry = dec
        .read_item()
        .await
        .expect("read CacheEntry no err")
        .expect("read CacheEntry some value");

    assert_eq!(val, decoded, "CacheEntry async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: CacheOp::Get async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_cache_op_get_roundtrip() {
    let val = CacheOp::Get(String::from("session:user:42"));

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write CacheOp::Get");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: CacheOp = dec
        .read_item()
        .await
        .expect("read CacheOp::Get no err")
        .expect("read CacheOp::Get some value");

    assert_eq!(val, decoded, "CacheOp::Get async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: CacheOp::Set async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_cache_op_set_roundtrip() {
    let val = CacheOp::Set(
        String::from("config:feature-flags"),
        vec![0xDE, 0xAD, 0xBE, 0xEF],
    );

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write CacheOp::Set");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: CacheOp = dec
        .read_item()
        .await
        .expect("read CacheOp::Set no err")
        .expect("read CacheOp::Set some value");

    assert_eq!(val, decoded, "CacheOp::Set async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 4: CacheOp::Delete async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_cache_op_delete_roundtrip() {
    let val = CacheOp::Delete(String::from("session:expired:99"));

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write CacheOp::Delete");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: CacheOp = dec
        .read_item()
        .await
        .expect("read CacheOp::Delete no err")
        .expect("read CacheOp::Delete some value");

    assert_eq!(val, decoded, "CacheOp::Delete async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 5: CacheOp::Flush async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_cache_op_flush_roundtrip() {
    let val = CacheOp::Flush;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write CacheOp::Flush");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: CacheOp = dec
        .read_item()
        .await
        .expect("read CacheOp::Flush no err")
        .expect("read CacheOp::Flush some value");

    assert_eq!(val, decoded, "CacheOp::Flush async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 6: CacheOp::Stats async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_cache_op_stats_roundtrip() {
    let val = CacheOp::Stats {
        hits: 1_048_576,
        misses: 32_768,
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write CacheOp::Stats");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: CacheOp = dec
        .read_item()
        .await
        .expect("read CacheOp::Stats no err")
        .expect("read CacheOp::Stats some value");

    assert_eq!(val, decoded, "CacheOp::Stats async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 7: Vec<CacheEntry> 4 items async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_vec_cache_entry_four_items_roundtrip() {
    let val: Vec<CacheEntry> = vec![
        CacheEntry {
            key: String::from("user:profile:1"),
            value: b"Alice".to_vec(),
            ttl_secs: 3600,
            compressed: false,
        },
        CacheEntry {
            key: String::from("user:profile:2"),
            value: b"Bob".to_vec(),
            ttl_secs: 7200,
            compressed: true,
        },
        CacheEntry {
            key: String::from("product:catalog:100"),
            value: vec![0xFF; 64],
            ttl_secs: 86400,
            compressed: true,
        },
        CacheEntry {
            key: String::from("rate-limit:ip:192.168.1.1"),
            value: vec![0, 0, 0, 42],
            ttl_secs: 60,
            compressed: false,
        },
    ];

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Vec<CacheEntry>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<CacheEntry> = dec
        .read_item()
        .await
        .expect("read Vec<CacheEntry> no err")
        .expect("read Vec<CacheEntry> some value");

    assert_eq!(
        val, decoded,
        "Vec<CacheEntry> 4-item async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Vec<CacheOp> all 5 variants async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_vec_cache_op_all_variants_roundtrip() {
    let val: Vec<CacheOp> = vec![
        CacheOp::Get(String::from("key:alpha")),
        CacheOp::Set(String::from("key:beta"), vec![10, 20, 30]),
        CacheOp::Delete(String::from("key:gamma")),
        CacheOp::Flush,
        CacheOp::Stats {
            hits: 999_999,
            misses: 1_001,
        },
    ];

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Vec<CacheOp>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<CacheOp> = dec
        .read_item()
        .await
        .expect("read Vec<CacheOp> no err")
        .expect("read Vec<CacheOp> some value");

    assert_eq!(
        val, decoded,
        "Vec<CacheOp> all-variants async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Option<CacheEntry> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_option_cache_entry_some_roundtrip() {
    let val: Option<CacheEntry> = Some(CacheEntry {
        key: String::from("optional:entry:present"),
        value: vec![7, 8, 9, 10],
        ttl_secs: 1800,
        compressed: false,
    });

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Option<CacheEntry> Some");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Option<CacheEntry> = dec
        .read_item()
        .await
        .expect("read Option<CacheEntry> Some no err")
        .expect("read Option<CacheEntry> Some some value");

    assert_eq!(
        val, decoded,
        "Option<CacheEntry> Some async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Option<CacheEntry> None async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_option_cache_entry_none_roundtrip() {
    let val: Option<CacheEntry> = None;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Option<CacheEntry> None");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Option<CacheEntry> = dec
        .read_item()
        .await
        .expect("read Option<CacheEntry> None no err")
        .expect("read Option<CacheEntry> None some value");

    assert_eq!(
        val, decoded,
        "Option<CacheEntry> None async roundtrip mismatch"
    );
    assert!(decoded.is_none(), "decoded Option<CacheEntry> must be None");
}

// ---------------------------------------------------------------------------
// Test 11: u32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_u32_roundtrip() {
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
        .expect("read u32 no err")
        .expect("read u32 some value");

    assert_eq!(val, decoded, "u32 async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 12: u64 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_u64_roundtrip() {
    let val: u64 = 18_446_744_073_709_551_000_u64;

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
        .expect("read u64 no err")
        .expect("read u64 some value");

    assert_eq!(val, decoded, "u64 async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 13: String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_string_roundtrip() {
    let val = String::from("cache-server-v3.oxicode-async-wave17");

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
        .expect("read String no err")
        .expect("read String some value");

    assert_eq!(val, decoded, "String async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 14: bool async roundtrip (both values)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_bool_both_values_roundtrip() {
    for &original in &[true, false] {
        let mut buf = Vec::<u8>::new();
        let mut enc = AsyncEncoder::new(&mut buf);
        enc.write_item(&original).await.expect("write bool");
        enc.finish().await.expect("finish encoder");

        let cursor = Cursor::new(buf);
        let mut reader = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(&mut reader);
        let decoded: bool = dec
            .read_item()
            .await
            .expect("read bool no err")
            .expect("read bool some value");

        assert_eq!(
            original, decoded,
            "bool {} async roundtrip mismatch",
            original
        );
    }
}

// ---------------------------------------------------------------------------
// Test 15: Vec<u8> 256 bytes async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_vec_u8_256_bytes_roundtrip() {
    let val: Vec<u8> = (0u16..256).map(|i| i as u8).collect();
    assert_eq!(val.len(), 256, "test data must be exactly 256 bytes");

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write 256-byte Vec<u8>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<u8> = dec
        .read_item()
        .await
        .expect("read 256-byte Vec<u8> no err")
        .expect("read 256-byte Vec<u8> some value");

    assert_eq!(val, decoded, "Vec<u8> 256-byte async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 16: Empty Vec<u8> async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_vec_u8_empty_roundtrip() {
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
        .expect("read empty Vec<u8> no err")
        .expect("read empty Vec<u8> some value");

    assert_eq!(val, decoded, "empty Vec<u8> async roundtrip mismatch");
    assert!(decoded.is_empty(), "decoded Vec<u8> must be empty");
}

// ---------------------------------------------------------------------------
// Test 17: i32 negative async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_i32_negative_roundtrip() {
    let val: i32 = -2_147_483_647_i32;

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
        .expect("read i32 negative no err")
        .expect("read i32 negative some value");

    assert_eq!(val, decoded, "i32 negative async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 18: f32 async bit-exact roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_f32_bit_exact_roundtrip() {
    let val: f32 = std::f32::consts::PI;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write f32 pi");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: f32 = dec
        .read_item()
        .await
        .expect("read f32 pi no err")
        .expect("read f32 pi some value");

    assert_eq!(
        val.to_bits(),
        decoded.to_bits(),
        "f32 pi async bit-exact roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Sequential write 3 CacheOps, read back 3
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_sequential_three_cache_ops_roundtrip() {
    let ops = [
        CacheOp::Get(String::from("token:auth:session-abc")),
        CacheOp::Set(
            String::from("token:auth:session-abc"),
            vec![0xCA, 0xFE, 0xBA, 0xBE],
        ),
        CacheOp::Stats {
            hits: 500_000,
            misses: 250,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let mut enc = AsyncEncoder::new(&mut buf);
        for op in &ops {
            enc.write_item(op).await.expect("write CacheOp in sequence");
        }
        enc.finish().await.expect("finish encoder");
    }

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);

    for (idx, expected) in ops.iter().enumerate() {
        let decoded: CacheOp = dec
            .read_item()
            .await
            .expect("read sequential CacheOp no err")
            .expect("read sequential CacheOp some value");
        assert_eq!(
            *expected, decoded,
            "sequential CacheOp at index {idx} mismatch"
        );
    }

    let eof: Option<CacheOp> = dec
        .read_item()
        .await
        .expect("read after 3rd CacheOp no err");
    assert_eq!(eof, None, "expected None after 3 sequential CacheOps");
}

// ---------------------------------------------------------------------------
// Test 20: u128 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_u128_roundtrip() {
    let val: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455_u128;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write u128");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: u128 = dec
        .read_item()
        .await
        .expect("read u128 no err")
        .expect("read u128 some value");

    assert_eq!(val, decoded, "u128 async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 21: CacheEntry with empty value
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_cache_entry_empty_value_roundtrip() {
    let val = CacheEntry {
        key: String::from("tombstone:deleted:key"),
        value: Vec::new(),
        ttl_secs: 0,
        compressed: false,
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write CacheEntry with empty value");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: CacheEntry = dec
        .read_item()
        .await
        .expect("read CacheEntry empty value no err")
        .expect("read CacheEntry empty value some value");

    assert_eq!(
        val, decoded,
        "CacheEntry with empty value async roundtrip mismatch"
    );
    assert!(
        decoded.value.is_empty(),
        "decoded CacheEntry value must be empty"
    );
}

// ---------------------------------------------------------------------------
// Test 22: (String, u32) tuple async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async17_tuple_string_u32_roundtrip() {
    let val: (String, u32) = (String::from("cache-namespace:wave17"), 65_536_u32);

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write (String, u32)");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: (String, u32) = dec
        .read_item()
        .await
        .expect("read (String, u32) no err")
        .expect("read (String, u32) some value");

    assert_eq!(val, decoded, "(String, u32) tuple async roundtrip mismatch");
}
