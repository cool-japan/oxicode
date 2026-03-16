//! Advanced async streaming tests (thirty-second set) for OxiCode.
//!
//! Domain: Real-time bidding / ad tech platform
//!
//! All 22 tests are top-level sync `#[test]` functions (no module wrapper,
//! no `#[cfg(test)]`) using `tokio::runtime::Builder::new_current_thread().enable_all().build().expect("runtime").block_on(...)`.
//! Gated by the `async-tokio` feature at the file level.
//!
//! Coverage:
//!   1:  Single BidRequest roundtrip via duplex channel
//!   2:  Multiple BidRequests batch write/read
//!   3:  BidResponse roundtrip
//!   4:  Campaign struct roundtrip
//!   5:  Empty stream returns None
//!   6:  Large payload BidRequest (500 items)
//!   7:  Progress tracking — items_processed > 0 after encoding
//!   8:  BidResponse with high bid_price
//!   9:  BidResponse with minimal bid_price
//!  10:  Campaign with large impressions count
//!  11:  Batch BidRequests using write_all via duplex
//!  12:  Multiple BidResponses streamed in order
//!  13:  BidRequest with zero floor_price
//!  14:  BidRequest with max floor_price (f64::MAX)
//!  15:  Campaign with empty name
//!  16:  encoder.finish() flushes remaining buffered items
//!  17:  StreamingConfig default used explicitly
//!  18:  Multiple campaigns batch roundtrip
//!  19:  Sequential read_item calls on multi-item stream
//!  20:  BidRequest ad_slot_id edge cases (0 and u32::MAX)
//!  21:  Large batch of BidRequests forces multiple chunks (small chunk_size)
//!  22:  BidResponse with zero bid_price, advertiser_id, and creative_id

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
use oxicode::streaming::{AsyncDecoder, AsyncEncoder, StreamingConfig};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use tokio::io::duplex;

// ---------------------------------------------------------------------------
// Domain types — Real-time bidding / ad tech platform
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BidRequest {
    request_id: u64,
    floor_price: f64,
    ad_slot_id: u32,
    user_segment: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BidResponse {
    request_id: u64,
    bid_price: f64,
    advertiser_id: u32,
    creative_id: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Campaign {
    campaign_id: u64,
    name: String,
    budget: f64,
    impressions: u64,
}

// ---------------------------------------------------------------------------
// Test 1: Single BidRequest roundtrip via duplex channel
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_single_bid_request_duplex_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            let original = BidRequest {
                request_id: 1_001,
                floor_price: 1.50,
                ad_slot_id: 42,
                user_segment: 7,
            };

            let (writer, reader) = duplex(65536);

            let original_clone = original.clone();
            let write_task = tokio::spawn(async move {
                let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
                encoder.write_item(&original_clone).await.expect("write");
                encoder.finish().await.expect("finish");
            });

            let read_task = tokio::spawn(async move {
                let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
                decoder
                    .read_item::<BidRequest>()
                    .await
                    .expect("read")
                    .expect("some")
            });

            write_task.await.expect("write task");
            let decoded = read_task.await.expect("read task");
            assert_eq!(original, decoded);
        });
}

// ---------------------------------------------------------------------------
// Test 2: Multiple BidRequests batch write/read
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_multiple_bid_requests_batch() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let requests: Vec<BidRequest> = (0u64..10)
                .map(|i| BidRequest {
                    request_id: i,
                    floor_price: 0.5 + i as f64 * 0.1,
                    ad_slot_id: i as u32 * 10,
                    user_segment: (i % 5) as u8,
                })
                .collect();

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder
                    .write_all(requests.clone())
                    .await
                    .expect("write_all");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Vec<BidRequest> = decoder.read_all().await.expect("read_all");

            assert_eq!(requests, decoded);
            assert_eq!(decoded.len(), 10);
        });
}

// ---------------------------------------------------------------------------
// Test 3: BidResponse roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_bid_response_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = BidResponse {
                request_id: 5_000,
                bid_price: 2.75,
                advertiser_id: 999,
                creative_id: 8_888_888,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: BidResponse = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert_eq!(decoded.bid_price, 2.75);
        });
}

// ---------------------------------------------------------------------------
// Test 4: Campaign struct roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_campaign_struct_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = Campaign {
                campaign_id: 12_345,
                name: "Summer Sale 2026".to_string(),
                budget: 50_000.0,
                impressions: 1_000_000,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Campaign = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert_eq!(decoded.name, "Summer Sale 2026");
        });
}

// ---------------------------------------------------------------------------
// Test 5: Empty stream returns None
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_empty_stream_returns_none() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.finish().await.expect("finish empty");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let result: Option<BidRequest> = decoder.read_item().await.expect("read empty");

            assert!(result.is_none(), "expected None from empty stream");
        });
}

// ---------------------------------------------------------------------------
// Test 6: Large payload — 500 BidRequests all survive roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_large_payload_bid_requests() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let requests: Vec<BidRequest> = (0u64..500)
                .map(|i| BidRequest {
                    request_id: i,
                    floor_price: i as f64 * 0.01,
                    ad_slot_id: (i % u32::MAX as u64) as u32,
                    user_segment: (i % 255) as u8,
                })
                .collect();

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder
                    .write_all(requests.clone())
                    .await
                    .expect("write_all large");
                encoder.finish().await.expect("finish large");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Vec<BidRequest> = decoder.read_all().await.expect("read_all large");

            assert_eq!(decoded.len(), 500);
            assert_eq!(requests, decoded);
        });
}

// ---------------------------------------------------------------------------
// Test 7: Progress tracking — items_processed > 0 after encoding
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_progress_tracking_items_processed() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let config = StreamingConfig::default().with_flush_per_item(true);
            let requests: Vec<BidRequest> = (0u64..5)
                .map(|i| BidRequest {
                    request_id: i,
                    floor_price: 1.0,
                    ad_slot_id: i as u32,
                    user_segment: 0,
                })
                .collect();

            let mut buf = Vec::<u8>::new();
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::with_config(cursor, config);

            for req in &requests {
                encoder.write_item(req).await.expect("write");
            }

            assert!(
                encoder.progress().items_processed > 0,
                "expected items_processed > 0 after writes with flush_per_item"
            );

            encoder.finish().await.expect("finish");
        });
}

// ---------------------------------------------------------------------------
// Test 8: BidResponse with high bid_price (auction winner)
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_bid_response_high_bid_price() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = BidResponse {
                request_id: 9_001,
                bid_price: 99.99,
                advertiser_id: 1_234,
                creative_id: 5_678_901,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: BidResponse = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert!(decoded.bid_price > 50.0, "expected high bid_price");
        });
}

// ---------------------------------------------------------------------------
// Test 9: BidResponse with minimal bid_price (just above floor)
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_bid_response_minimal_bid_price() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = BidResponse {
                request_id: 9_002,
                bid_price: 0.01,
                advertiser_id: 77,
                creative_id: 1,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: BidResponse = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert!(decoded.bid_price < 1.0, "expected low bid_price");
        });
}

// ---------------------------------------------------------------------------
// Test 10: Campaign with very large impressions count
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_campaign_large_impressions() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = Campaign {
                campaign_id: 99_999,
                name: "Mega Brand Awareness".to_string(),
                budget: 1_000_000.0,
                impressions: u64::MAX,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Campaign = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert_eq!(decoded.impressions, u64::MAX);
        });
}

// ---------------------------------------------------------------------------
// Test 11: Batch BidRequests using write_all via duplex channel
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_batch_bid_requests_via_duplex() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            let requests: Vec<BidRequest> = (0u64..8)
                .map(|i| BidRequest {
                    request_id: 200 + i,
                    floor_price: 3.0 + i as f64,
                    ad_slot_id: (100 + i) as u32,
                    user_segment: (i % 10) as u8,
                })
                .collect();

            let (writer, reader) = duplex(65536);
            let requests_to_write = requests.clone();

            let write_task = tokio::spawn(async move {
                let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
                encoder
                    .write_all(requests_to_write)
                    .await
                    .expect("write_all duplex");
                encoder.finish().await.expect("finish duplex");
            });

            let read_task = tokio::spawn(async move {
                let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
                decoder
                    .read_all::<BidRequest>()
                    .await
                    .expect("read_all duplex")
            });

            write_task.await.expect("write task");
            let decoded = read_task.await.expect("read task");

            assert_eq!(requests, decoded);
            assert_eq!(decoded.len(), 8);
        });
}

// ---------------------------------------------------------------------------
// Test 12: Multiple BidResponses streamed in order
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_multiple_bid_responses_order_preserved() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let responses: Vec<BidResponse> = (0u64..12)
                .map(|i| BidResponse {
                    request_id: 3_000 + i,
                    bid_price: 1.0 + i as f64 * 0.5,
                    advertiser_id: (500 + i) as u32,
                    creative_id: 10_000 + i,
                })
                .collect();

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder
                    .write_all(responses.clone())
                    .await
                    .expect("write_all responses");
                encoder.finish().await.expect("finish responses");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Vec<BidResponse> = decoder.read_all().await.expect("read_all responses");

            assert_eq!(responses, decoded);
            assert_eq!(decoded.len(), 12);
            assert_eq!(decoded[0].request_id, 3_000);
            assert_eq!(decoded[11].request_id, 3_011);
        });
}

// ---------------------------------------------------------------------------
// Test 13: BidRequest with zero floor_price
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_bid_request_zero_floor_price() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = BidRequest {
                request_id: 7_001,
                floor_price: 0.0,
                ad_slot_id: 1,
                user_segment: 3,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: BidRequest = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert_eq!(decoded.floor_price, 0.0);
        });
}

// ---------------------------------------------------------------------------
// Test 14: BidRequest with max floor_price (f64::MAX)
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_bid_request_max_floor_price() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = BidRequest {
                request_id: 7_002,
                floor_price: f64::MAX,
                ad_slot_id: u32::MAX,
                user_segment: 255,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: BidRequest = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert_eq!(decoded.floor_price, f64::MAX);
            assert_eq!(decoded.ad_slot_id, u32::MAX);
        });
}

// ---------------------------------------------------------------------------
// Test 15: Campaign with empty name
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_campaign_empty_name() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = Campaign {
                campaign_id: 0,
                name: String::new(),
                budget: 0.0,
                impressions: 0,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Campaign = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert!(decoded.name.is_empty(), "name must be empty");
        });
}

// ---------------------------------------------------------------------------
// Test 16: encoder.finish() flushes remaining buffered items
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_finish_flushes_remaining_items() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let items: Vec<BidRequest> = (0u64..4)
                .map(|i| BidRequest {
                    request_id: 8_000 + i,
                    floor_price: 2.0,
                    ad_slot_id: i as u32,
                    user_segment: 1,
                })
                .collect();

            let mut buf = Vec::<u8>::new();
            let cursor = Cursor::new(&mut buf);
            // Default config: no flush_per_item, so items remain buffered until finish()
            let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
            for item in &items {
                encoder.write_item(item).await.expect("write");
            }
            // finish() must flush all buffered items
            encoder.finish().await.expect("finish");

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Vec<BidRequest> = decoder.read_all().await.expect("read_all after finish");

            assert_eq!(items, decoded);
            assert_eq!(decoded.len(), 4);
        });
}

// ---------------------------------------------------------------------------
// Test 17: StreamingConfig default used explicitly
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_streaming_config_default_explicit() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let config = StreamingConfig::default();
            let original = BidRequest {
                request_id: 4_242,
                floor_price: 1.25,
                ad_slot_id: 101,
                user_segment: 5,
            };

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, config);
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: BidRequest = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
        });
}

// ---------------------------------------------------------------------------
// Test 18: Multiple campaigns batch roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_multiple_campaigns_batch_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let campaigns: Vec<Campaign> = vec![
                Campaign {
                    campaign_id: 1,
                    name: "Brand Alpha".to_string(),
                    budget: 10_000.0,
                    impressions: 500_000,
                },
                Campaign {
                    campaign_id: 2,
                    name: "Promo Beta".to_string(),
                    budget: 5_000.0,
                    impressions: 250_000,
                },
                Campaign {
                    campaign_id: 3,
                    name: "Retarget Gamma".to_string(),
                    budget: 2_500.0,
                    impressions: 100_000,
                },
                Campaign {
                    campaign_id: 4,
                    name: "Awareness Delta".to_string(),
                    budget: 75_000.0,
                    impressions: 8_000_000,
                },
            ];

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder
                    .write_all(campaigns.clone())
                    .await
                    .expect("write_all campaigns");
                encoder.finish().await.expect("finish campaigns");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Vec<Campaign> = decoder.read_all().await.expect("read_all campaigns");

            assert_eq!(campaigns, decoded);
            assert_eq!(decoded.len(), 4);
            assert_eq!(decoded[3].name, "Awareness Delta");
        });
}

// ---------------------------------------------------------------------------
// Test 19: Sequential read_item calls on multi-item stream
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_sequential_read_item_by_item() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let items: Vec<BidRequest> = vec![
                BidRequest {
                    request_id: 101,
                    floor_price: 0.5,
                    ad_slot_id: 1,
                    user_segment: 0,
                },
                BidRequest {
                    request_id: 102,
                    floor_price: 1.0,
                    ad_slot_id: 2,
                    user_segment: 1,
                },
                BidRequest {
                    request_id: 103,
                    floor_price: 1.5,
                    ad_slot_id: 3,
                    user_segment: 2,
                },
            ];

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder
                    .write_all(items.clone())
                    .await
                    .expect("write_all seq");
                encoder.finish().await.expect("finish seq");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());

            for (idx, expected) in items.iter().enumerate() {
                let item: BidRequest = decoder
                    .read_item()
                    .await
                    .unwrap_or_else(|e| panic!("read_item[{}] failed: {}", idx, e))
                    .unwrap_or_else(|| panic!("expected Some at index {}", idx));
                assert_eq!(expected, &item, "mismatch at index {}", idx);
            }

            let eof: Option<BidRequest> = decoder.read_item().await.expect("eof read");
            assert!(eof.is_none(), "expected None after last item");
        });
}

// ---------------------------------------------------------------------------
// Test 20: BidRequest ad_slot_id edge cases (0 and u32::MAX)
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_bid_request_ad_slot_id_edge_cases() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let items = vec![
                BidRequest {
                    request_id: 6_001,
                    floor_price: 1.0,
                    ad_slot_id: 0,
                    user_segment: 0,
                },
                BidRequest {
                    request_id: 6_002,
                    floor_price: 2.0,
                    ad_slot_id: u32::MAX,
                    user_segment: 255,
                },
            ];

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder
                    .write_all(items.clone())
                    .await
                    .expect("write_all edge");
                encoder.finish().await.expect("finish edge");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Vec<BidRequest> = decoder.read_all().await.expect("read_all edge");

            assert_eq!(items, decoded);
            assert_eq!(decoded[0].ad_slot_id, 0);
            assert_eq!(decoded[1].ad_slot_id, u32::MAX);
        });
}

// ---------------------------------------------------------------------------
// Test 21: Large batch of BidRequests forces multiple chunks (small chunk_size)
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_large_batch_forces_multiple_chunks() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let config = StreamingConfig::default().with_chunk_size(1024);
            let requests: Vec<BidRequest> = (0u64..100)
                .map(|i| BidRequest {
                    request_id: i,
                    floor_price: i as f64 * 0.25,
                    ad_slot_id: (i % 1000) as u32,
                    user_segment: (i % 100) as u8,
                })
                .collect();

            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, config);
                encoder
                    .write_all(requests.clone())
                    .await
                    .expect("write_all multi-chunk");
                encoder.finish().await.expect("finish multi-chunk");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: Vec<BidRequest> = decoder.read_all().await.expect("read_all multi-chunk");

            assert_eq!(requests.len(), decoded.len());
            assert_eq!(requests, decoded);
            assert!(
                decoder.progress().chunks_processed > 1,
                "expected more than 1 chunk, got {}",
                decoder.progress().chunks_processed
            );
        });
}

// ---------------------------------------------------------------------------
// Test 22: BidResponse with zero bid_price, advertiser_id, and creative_id
// ---------------------------------------------------------------------------
#[test]
fn test_rtb_bid_response_zero_fields() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime")
        .block_on(async {
            use std::io::Cursor;

            let original = BidResponse {
                request_id: 0,
                bid_price: 0.0,
                advertiser_id: 0,
                creative_id: 0,
            };

            // Verify sync encode/decode consistency via encode_to_vec / decode_from_slice
            let sync_bytes = encode_to_vec(&original).expect("encode_to_vec");
            let (sync_decoded, _): (BidResponse, _) =
                decode_from_slice(&sync_bytes).expect("decode_from_slice");
            assert_eq!(original, sync_decoded, "sync roundtrip mismatch");

            // Async streaming roundtrip
            let mut buf = Vec::<u8>::new();
            {
                let cursor = Cursor::new(&mut buf);
                let mut encoder = AsyncEncoder::with_config(cursor, StreamingConfig::default());
                encoder.write_item(&original).await.expect("write");
                encoder.finish().await.expect("finish");
            }

            let cursor = Cursor::new(buf);
            let mut decoder = AsyncDecoder::with_config(cursor, StreamingConfig::default());
            let decoded: BidResponse = decoder.read_item().await.expect("read").expect("some");

            assert_eq!(original, decoded);
            assert_eq!(decoded.bid_price, 0.0);
            assert_eq!(decoded.advertiser_id, 0);
            assert_eq!(decoded.creative_id, 0);
        });
}
