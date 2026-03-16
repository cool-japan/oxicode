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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder, StreamingConfig};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HealthCheck {
    service: String,
    healthy: bool,
    latency_ms: u32,
    checks: Vec<(String, bool)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HealthStatus {
    Up,
    Down { reason: String },
    Degraded { pct: u8 },
    Unknown,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_health_check(service: &str, healthy: bool, latency_ms: u32) -> HealthCheck {
    HealthCheck {
        service: service.to_string(),
        healthy,
        latency_ms,
        checks: vec![
            ("db".to_string(), true),
            ("cache".to_string(), healthy),
            ("queue".to_string(), true),
        ],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Basic HealthCheck roundtrip through a duplex pipe.
#[tokio::test]
async fn test_health_check_basic_roundtrip() {
    let hc = make_health_check("api", true, 12);
    let (client, server) = tokio::io::duplex(4096);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&hc).await.expect("write_item failed");
    let writer = enc.finish().await.expect("finish failed");
    drop(writer);

    let mut dec = AsyncDecoder::new(server);
    let result: HealthCheck = dec
        .read_item()
        .await
        .expect("read_item failed")
        .expect("expected Some(HealthCheck)");

    assert_eq!(hc, result);
    assert_eq!(
        dec.read_item::<HealthCheck>().await.expect("read None"),
        None
    );
}

/// 2. HealthStatus::Up roundtrip.
#[tokio::test]
async fn test_health_status_up_roundtrip() {
    let status = HealthStatus::Up;
    let (client, server) = tokio::io::duplex(1024);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: HealthStatus = dec.read_item().await.expect("read").expect("Some");
    assert_eq!(status, got);
}

/// 3. HealthStatus::Down roundtrip.
#[tokio::test]
async fn test_health_status_down_roundtrip() {
    let status = HealthStatus::Down {
        reason: "connection refused".to_string(),
    };
    let (client, server) = tokio::io::duplex(1024);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: HealthStatus = dec.read_item().await.expect("read").expect("Some");
    assert_eq!(status, got);
}

/// 4. HealthStatus::Degraded roundtrip.
#[tokio::test]
async fn test_health_status_degraded_roundtrip() {
    let status = HealthStatus::Degraded { pct: 73 };
    let (client, server) = tokio::io::duplex(1024);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: HealthStatus = dec.read_item().await.expect("read").expect("Some");
    assert_eq!(status, got);
}

/// 5. HealthStatus::Unknown roundtrip.
#[tokio::test]
async fn test_health_status_unknown_roundtrip() {
    let status = HealthStatus::Unknown;
    let (client, server) = tokio::io::duplex(1024);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: HealthStatus = dec.read_item().await.expect("read").expect("Some");
    assert_eq!(status, got);
}

/// 6. Sequential reads of multiple HealthCheck values.
#[tokio::test]
async fn test_sequential_health_checks() {
    let checks = vec![
        make_health_check("svc-a", true, 5),
        make_health_check("svc-b", false, 300),
        make_health_check("svc-c", true, 42),
    ];
    let (client, server) = tokio::io::duplex(8192);

    let mut enc = AsyncEncoder::new(client);
    for hc in &checks {
        enc.write_item(hc).await.expect("write_item");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    for expected in &checks {
        let got: HealthCheck = dec.read_item().await.expect("read").expect("Some");
        assert_eq!(expected, &got);
    }
    assert_eq!(dec.read_item::<HealthCheck>().await.expect("end"), None);
}

/// 7. read_all collects all HealthStatus variants at once.
#[tokio::test]
async fn test_read_all_health_statuses() {
    let statuses = vec![
        HealthStatus::Up,
        HealthStatus::Down {
            reason: "timeout".to_string(),
        },
        HealthStatus::Degraded { pct: 50 },
        HealthStatus::Unknown,
    ];
    let (client, server) = tokio::io::duplex(4096);

    let mut enc = AsyncEncoder::new(client);
    for s in &statuses {
        enc.write_item(s).await.expect("write_item");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<HealthStatus> = dec.read_all().await.expect("read_all");
    assert_eq!(statuses, got);
}

/// 8. Empty stream — reading from a stream with zero items returns None immediately.
#[tokio::test]
async fn test_empty_stream_returns_none() {
    let (client, server) = tokio::io::duplex(256);
    let enc: AsyncEncoder<_> = AsyncEncoder::new(client);
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let result = dec.read_item::<HealthCheck>().await.expect("read empty");
    assert_eq!(result, None);
    assert!(dec.is_finished());
}

/// 9. write_all helper encodes multiple items in one call.
#[tokio::test]
async fn test_write_all_and_read_all() {
    let checks: Vec<HealthCheck> = (0..8)
        .map(|i| make_health_check(&format!("svc-{}", i), i % 2 == 0, i * 10))
        .collect();
    let (client, server) = tokio::io::duplex(16384);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(checks.clone().into_iter())
        .await
        .expect("write_all");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<HealthCheck> = dec.read_all().await.expect("read_all");
    assert_eq!(checks, got);
}

/// 10. Small chunk size forces multiple chunks; data integrity must hold.
#[tokio::test]
async fn test_multiple_chunks_health_checks() {
    let config = StreamingConfig::new().with_chunk_size(32);
    let checks: Vec<HealthCheck> = (0..20)
        .map(|i| make_health_check(&format!("service-{:03}", i), i % 3 != 0, i * 5))
        .collect();
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for hc in &checks {
        enc.write_item(hc).await.expect("write_item");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<HealthCheck> = dec.read_all().await.expect("read_all");
    assert_eq!(checks, got);
    assert!(dec.progress().items_processed > 0);
}

/// 11. Progress tracking: items_processed equals number of written items.
#[tokio::test]
async fn test_progress_items_processed() {
    const N: u64 = 15;
    let (client, server) = tokio::io::duplex(32768);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    for i in 0..N {
        enc.write_item(&HealthStatus::Degraded { pct: i as u8 })
            .await
            .expect("write_item");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<HealthStatus> = dec.read_all().await.expect("read_all");
    assert_eq!(dec.progress().items_processed, N);
    assert!(dec.progress().bytes_processed > 0);
}

/// 12. Mixed type interleaving: encode HealthCheck, then HealthStatus on separate streams.
#[tokio::test]
async fn test_independent_streams_mixed_types() {
    let hc = make_health_check("mixer", true, 99);
    let hs = HealthStatus::Down {
        reason: "disk full".to_string(),
    };

    let (c1, s1) = tokio::io::duplex(4096);
    let (c2, s2) = tokio::io::duplex(4096);

    let (r1, r2) = tokio::join!(
        async {
            let mut enc = AsyncEncoder::new(c1);
            enc.write_item(&hc).await.expect("write hc");
            enc.finish().await.expect("finish hc");
            let mut dec = AsyncDecoder::new(s1);
            dec.read_item::<HealthCheck>()
                .await
                .expect("read hc")
                .expect("Some hc")
        },
        async {
            let mut enc = AsyncEncoder::new(c2);
            enc.write_item(&hs).await.expect("write hs");
            enc.finish().await.expect("finish hs");
            let mut dec = AsyncDecoder::new(s2);
            dec.read_item::<HealthStatus>()
                .await
                .expect("read hs")
                .expect("Some hs")
        }
    );

    assert_eq!(hc, r1);
    assert_eq!(hs, r2);
}

/// 13. Flush-per-item mode: each item is sent as its own chunk.
#[tokio::test]
async fn test_flush_per_item_mode() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let checks = vec![
        make_health_check("alpha", true, 1),
        make_health_check("beta", false, 2),
    ];
    let (client, server) = tokio::io::duplex(8192);

    let mut enc = AsyncEncoder::with_config(client, config);
    for hc in &checks {
        enc.write_item(hc).await.expect("write_item");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<HealthCheck> = dec.read_all().await.expect("read_all");
    assert_eq!(checks, got);
    // Each item is its own chunk, so chunks_processed >= 2
    assert!(dec.progress().chunks_processed >= 2);
}

/// 14. Large HealthCheck with many sub-checks exercises longer payloads.
#[tokio::test]
async fn test_large_health_check_many_sub_checks() {
    let checks_inner: Vec<(String, bool)> = (0..200)
        .map(|i| (format!("probe-{}", i), i % 5 != 0))
        .collect();
    let hc = HealthCheck {
        service: "monolith".to_string(),
        healthy: false,
        latency_ms: 9999,
        checks: checks_inner.clone(),
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&hc).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: HealthCheck = dec.read_item().await.expect("read").expect("Some");
    assert_eq!(hc, got);
    assert_eq!(got.checks.len(), 200);
}

/// 15. HealthCheck with empty checks vector.
#[tokio::test]
async fn test_health_check_empty_checks_vec() {
    let hc = HealthCheck {
        service: "minimal".to_string(),
        healthy: true,
        latency_ms: 0,
        checks: vec![],
    };
    let (client, server) = tokio::io::duplex(2048);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&hc).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: HealthCheck = dec.read_item().await.expect("read").expect("Some");
    assert_eq!(hc, got);
    assert!(got.checks.is_empty());
}

/// 16. Calling read_item after is_finished returns None without error.
#[tokio::test]
async fn test_read_after_finished_is_none() {
    let (client, server) = tokio::io::duplex(1024);
    let enc: AsyncEncoder<_> = AsyncEncoder::new(client);
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let first = dec.read_item::<HealthStatus>().await.expect("first read");
    assert_eq!(first, None);
    assert!(dec.is_finished());

    // Additional read after is_finished must also return None safely.
    let second = dec.read_item::<HealthStatus>().await.expect("second read");
    assert_eq!(second, None);
}

/// 17. Single-item stream of HealthStatus::Degraded with boundary pct values.
#[tokio::test]
async fn test_degraded_boundary_values() {
    for pct in [0u8, 1, 50, 99, 255] {
        let status = HealthStatus::Degraded { pct };
        let (client, server) = tokio::io::duplex(512);

        let mut enc = AsyncEncoder::new(client);
        enc.write_item(&status).await.expect("write_item");
        enc.finish().await.expect("finish");

        let mut dec = AsyncDecoder::new(server);
        let got: HealthStatus = dec.read_item().await.expect("read").expect("Some");
        assert_eq!(status, got, "pct={pct} roundtrip failed");
    }
}

/// 18. Encoding 100 HealthStatus items and confirming exact count after read_all.
#[tokio::test]
async fn test_hundred_health_statuses_roundtrip() {
    let statuses: Vec<HealthStatus> = (0..100u8)
        .map(|i| match i % 4 {
            0 => HealthStatus::Up,
            1 => HealthStatus::Down {
                reason: format!("err-{}", i),
            },
            2 => HealthStatus::Degraded { pct: i },
            _ => HealthStatus::Unknown,
        })
        .collect();
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(statuses.clone().into_iter())
        .await
        .expect("write_all");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<HealthStatus> = dec.read_all().await.expect("read_all");
    assert_eq!(statuses.len(), got.len());
    assert_eq!(statuses, got);
    assert_eq!(dec.progress().items_processed, 100);
}

/// 19. Encoder get_ref returns the underlying writer reference.
#[tokio::test]
async fn test_encoder_get_ref() {
    let (client, _server) = tokio::io::duplex(256);
    let enc = AsyncEncoder::new(client);
    // get_ref() must not panic; we just verify it exists and returns a ref
    let _ref = enc.get_ref();
}

/// 20. Decoder get_ref returns the underlying reader reference.
#[tokio::test]
async fn test_decoder_get_ref() {
    let (client, server) = tokio::io::duplex(256);
    let enc: AsyncEncoder<_> = AsyncEncoder::new(client);
    enc.finish().await.expect("finish");

    let dec = AsyncDecoder::new(server);
    let _ref = dec.get_ref();
}

/// 21. Interleaved encode/decode of HealthCheck and HealthStatus on the same stream
///     (two separate encoder/decoder pairs sharing independent pipes).
#[tokio::test]
async fn test_interleaved_type_streams() {
    let hc_items: Vec<HealthCheck> = (0..5)
        .map(|i| make_health_check(&format!("node-{}", i), i % 2 == 0, i * 7))
        .collect();
    let hs_items: Vec<HealthStatus> = vec![
        HealthStatus::Up,
        HealthStatus::Degraded { pct: 10 },
        HealthStatus::Down {
            reason: "oom".to_string(),
        },
    ];

    let (c_hc, s_hc) = tokio::io::duplex(16384);
    let (c_hs, s_hs) = tokio::io::duplex(4096);

    let mut enc_hc = AsyncEncoder::new(c_hc);
    let mut enc_hs = AsyncEncoder::new(c_hs);

    for hc in &hc_items {
        enc_hc.write_item(hc).await.expect("write hc");
    }
    for hs in &hs_items {
        enc_hs.write_item(hs).await.expect("write hs");
    }
    enc_hc.finish().await.expect("finish hc");
    enc_hs.finish().await.expect("finish hs");

    let mut dec_hc = AsyncDecoder::new(s_hc);
    let mut dec_hs = AsyncDecoder::new(s_hs);

    let got_hc: Vec<HealthCheck> = dec_hc.read_all().await.expect("read_all hc");
    let got_hs: Vec<HealthStatus> = dec_hs.read_all().await.expect("read_all hs");

    assert_eq!(hc_items, got_hc);
    assert_eq!(hs_items, got_hs);
}

/// 22. Very small duplex buffer (64 bytes) stresses backpressure and partial-write paths.
#[tokio::test]
async fn test_small_duplex_buffer_backpressure() {
    let hc = make_health_check("stress", false, 1234);
    // Use a very small in-memory pipe; tokio::io::duplex handles backpressure.
    let (client, server) = tokio::io::duplex(64);

    let write_task = tokio::spawn(async move {
        let mut enc = AsyncEncoder::new(client);
        enc.write_item(&hc)
            .await
            .expect("write_item under backpressure");
        enc.finish().await.expect("finish under backpressure");
    });

    let hc_expected = make_health_check("stress", false, 1234);
    let read_task = tokio::spawn(async move {
        let mut dec = AsyncDecoder::new(server);
        dec.read_item::<HealthCheck>()
            .await
            .expect("read under backpressure")
            .expect("Some under backpressure")
    });

    write_task.await.expect("write task panicked");
    let got = read_task.await.expect("read task panicked");
    assert_eq!(hc_expected, got);
}
