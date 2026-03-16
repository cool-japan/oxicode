//! Multi-threaded encoding/decoding tests using std threads and Arc.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::sync::Arc;
use std::thread;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimpleStruct {
    id: u32,
    value: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NamedRecord {
    name: String,
    score: u32,
}

// 1. Encode u32 in 4 threads simultaneously, verify all produce same bytes
#[test]
fn test_parallel_encode_u32_four_threads_same_bytes() {
    let expected = encode_to_vec(&42u32).expect("baseline encode failed");
    let expected_arc = Arc::new(expected);

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let expected_clone = Arc::clone(&expected_arc);
            thread::spawn(move || {
                let enc = encode_to_vec(&42u32).expect("thread encode failed");
                assert_eq!(
                    enc, *expected_clone,
                    "encoded bytes mismatch across threads"
                );
                enc
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("thread panicked");
    }
}

// 2. Decode u32 in 4 threads simultaneously, verify all produce same value
#[test]
fn test_parallel_decode_u32_four_threads_same_value() {
    let encoded = encode_to_vec(&99u32).expect("encode failed");
    let encoded_arc = Arc::new(encoded);

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let enc = Arc::clone(&encoded_arc);
            thread::spawn(move || {
                let (val, _): (u32, _) = decode_from_slice(&enc).expect("thread decode failed");
                assert_eq!(val, 99u32, "decoded value mismatch across threads");
                val
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("thread panicked");
    }
}

// 3. Encode String in 2 threads, compare results
#[test]
fn test_parallel_encode_string_two_threads_compare() {
    let s = String::from("hello oxicode");
    let s_arc = Arc::new(s);

    let handles: Vec<_> = (0..2)
        .map(|_| {
            let s_clone = Arc::clone(&s_arc);
            thread::spawn(move || encode_to_vec(&*s_clone).expect("string encode failed"))
        })
        .collect();

    let results: Vec<Vec<u8>> = handles
        .into_iter()
        .map(|h| h.join().expect("thread panicked"))
        .collect();

    assert_eq!(
        results[0], results[1],
        "string encoding differs between threads"
    );
}

// 4. Arc<Vec<u8>> shared data encoded from multiple threads
#[test]
fn test_arc_shared_vec_encoded_from_multiple_threads() {
    let data: Vec<u8> = (0u8..=127).collect();
    let data_arc = Arc::new(data);

    let expected = encode_to_vec(&*data_arc).expect("baseline encode failed");

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let d = Arc::clone(&data_arc);
            thread::spawn(move || encode_to_vec(&*d).expect("thread encode failed"))
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(result, expected, "Arc<Vec<u8>> encoding differs");
    }
}

// 5. Parallel encode of 100 u64 values using thread::spawn
#[test]
fn test_parallel_encode_100_u64_values() {
    let handles: Vec<_> = (0u64..100)
        .map(|i| {
            thread::spawn(move || {
                let enc = encode_to_vec(&i).expect("encode failed");
                (i, enc)
            })
        })
        .collect();

    let mut results: Vec<(u64, Vec<u8>)> = handles
        .into_iter()
        .map(|h| h.join().expect("thread panicked"))
        .collect();

    results.sort_by_key(|(i, _)| *i);

    for (i, enc) in &results {
        let expected = encode_to_vec(i).expect("baseline encode failed");
        assert_eq!(*enc, expected, "mismatch for u64 value {}", i);
    }
}

// 6. Parallel decode of 100 u64 values using thread::spawn
#[test]
fn test_parallel_decode_100_u64_values() {
    let encoded_pairs: Vec<(u64, Arc<Vec<u8>>)> = (0u64..100)
        .map(|i| {
            let enc = encode_to_vec(&i).expect("encode failed");
            (i, Arc::new(enc))
        })
        .collect();

    let handles: Vec<_> = encoded_pairs
        .into_iter()
        .map(|(expected_val, enc_arc)| {
            thread::spawn(move || {
                let (val, _): (u64, _) = decode_from_slice(&enc_arc).expect("decode failed");
                assert_eq!(val, expected_val, "decoded u64 mismatch");
                val
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("thread panicked");
    }
}

// 7. Producer-consumer: one thread encodes, another decodes via channel
#[test]
fn test_producer_consumer_encode_decode_via_channel() {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();

    let producer = thread::spawn(move || {
        let value = 7654321u64;
        let enc = encode_to_vec(&value).expect("producer encode failed");
        tx.send(enc).expect("channel send failed");
        value
    });

    let consumer = thread::spawn(move || {
        let enc = rx.recv().expect("channel recv failed");
        let (val, _): (u64, _) = decode_from_slice(&enc).expect("consumer decode failed");
        val
    });

    let produced = producer.join().expect("producer panicked");
    let consumed = consumer.join().expect("consumer panicked");

    assert_eq!(produced, consumed, "producer-consumer value mismatch");
}

// 8. Encode large Vec<u8> (1000 bytes) in 4 threads, results equal
#[test]
fn test_parallel_encode_large_vec_u8_four_threads() {
    let large: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let large_arc = Arc::new(large);
    let expected = encode_to_vec(&*large_arc).expect("baseline encode failed");

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let data = Arc::clone(&large_arc);
            thread::spawn(move || encode_to_vec(&*data).expect("thread encode failed"))
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(result, expected, "large Vec<u8> encoding mismatch");
    }
}

// 9. Decode large Vec<u8> (1000 bytes) in 4 threads, results equal
#[test]
fn test_parallel_decode_large_vec_u8_four_threads() {
    let large: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let encoded = encode_to_vec(&large).expect("encode failed");
    let encoded_arc = Arc::new(encoded);

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let enc = Arc::clone(&encoded_arc);
            thread::spawn(move || {
                let (decoded, _): (Vec<u8>, _) =
                    decode_from_slice(&enc).expect("thread decode failed");
                decoded
            })
        })
        .collect();

    let large_arc = Arc::new(large);
    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(result, *large_arc, "large Vec<u8> decoding mismatch");
    }
}

// 10. Thread-safe counter: encode/decode u64 counter value across threads
#[test]
fn test_thread_safe_counter_encode_decode() {
    use std::sync::atomic::{AtomicU64, Ordering};

    let counter = Arc::new(AtomicU64::new(0));

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                let val = c.fetch_add(1, Ordering::Relaxed);
                let enc = encode_to_vec(&val).expect("counter encode failed");
                let (decoded, _): (u64, _) =
                    decode_from_slice(&enc).expect("counter decode failed");
                assert_eq!(decoded, val, "counter round-trip mismatch");
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("thread panicked");
    }

    let final_val = counter.load(Ordering::Relaxed);
    assert_eq!(final_val, 8, "counter should be incremented 8 times");
}

// 11. Parallel encode of struct in 4 threads
#[test]
fn test_parallel_encode_struct_four_threads() {
    let record = SimpleStruct {
        id: 42,
        value: 9999,
    };
    let record_arc = Arc::new(record);
    let expected = encode_to_vec(&*record_arc).expect("baseline encode failed");

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let r = Arc::clone(&record_arc);
            thread::spawn(move || encode_to_vec(&*r).expect("struct encode failed"))
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(result, expected, "struct encoding mismatch across threads");
    }
}

// 12. Parallel decode of struct in 4 threads
#[test]
fn test_parallel_decode_struct_four_threads() {
    let record = SimpleStruct {
        id: 7,
        value: 12345,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let encoded_arc = Arc::new(encoded);

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let enc = Arc::clone(&encoded_arc);
            thread::spawn(move || {
                let (decoded, _): (SimpleStruct, _) =
                    decode_from_slice(&enc).expect("struct decode failed");
                decoded
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(result, record, "struct decoding mismatch across threads");
    }
}

// 13. Two threads encode different types, no interference
#[test]
fn test_two_threads_encode_different_types_no_interference() {
    let u32_val: u32 = 1234;
    let str_val = String::from("no interference");

    let h1 = thread::spawn(move || encode_to_vec(&u32_val).expect("u32 encode failed"));
    let h2 = thread::spawn(move || encode_to_vec(&str_val).expect("str encode failed"));

    let u32_enc = h1.join().expect("u32 thread panicked");
    let str_enc = h2.join().expect("str thread panicked");

    let (decoded_u32, _): (u32, _) = decode_from_slice(&u32_enc).expect("u32 decode failed");
    let (decoded_str, _): (String, _) = decode_from_slice(&str_enc).expect("str decode failed");

    assert_eq!(decoded_u32, 1234u32, "u32 value mismatch");
    assert_eq!(decoded_str, "no interference", "String value mismatch");
}

// 14. Encode bool values in parallel (true/false alternating)
#[test]
fn test_parallel_encode_bool_alternating() {
    let handles: Vec<_> = (0..8)
        .map(|i| {
            thread::spawn(move || {
                let b = i % 2 == 0;
                let enc = encode_to_vec(&b).expect("bool encode failed");
                (b, enc)
            })
        })
        .collect();

    for handle in handles {
        let (original_bool, enc) = handle.join().expect("thread panicked");
        let (decoded, _): (bool, _) = decode_from_slice(&enc).expect("bool decode failed");
        assert_eq!(decoded, original_bool, "bool round-trip mismatch");
    }
}

// 15. Encode Option<String> in parallel threads
#[test]
fn test_parallel_encode_option_string() {
    let values: Vec<Option<String>> = vec![
        Some(String::from("alpha")),
        None,
        Some(String::from("beta")),
        None,
        Some(String::from("gamma")),
        None,
        Some(String::from("delta")),
        None,
    ];

    let values_arc = Arc::new(values);

    let handles: Vec<_> = (0..8)
        .map(|i| {
            let v = Arc::clone(&values_arc);
            thread::spawn(move || {
                let opt = v[i].clone();
                let enc = encode_to_vec(&opt).expect("Option<String> encode failed");
                let (decoded, _): (Option<String>, _) =
                    decode_from_slice(&enc).expect("Option<String> decode failed");
                assert_eq!(
                    decoded, v[i],
                    "Option<String> round-trip mismatch at index {}",
                    i
                );
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("thread panicked");
    }
}

// 16. Encode Vec<u32> in thread, send bytes over channel
#[test]
fn test_encode_vec_u32_in_thread_send_over_channel() {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let original = vec![10u32, 20, 30, 40, 50];
    let original_arc = Arc::new(original.clone());

    let producer = thread::spawn(move || {
        let enc = encode_to_vec(&*original_arc).expect("Vec<u32> encode failed");
        tx.send(enc).expect("channel send failed");
    });

    producer.join().expect("producer panicked");

    let received = rx.recv().expect("channel recv failed");
    let (decoded, _): (Vec<u32>, _) = decode_from_slice(&received).expect("Vec<u32> decode failed");
    assert_eq!(decoded, original, "Vec<u32> channel round-trip mismatch");
}

// 17. Decode Vec<u32> in thread, receive bytes from channel
#[test]
fn test_decode_vec_u32_in_thread_receive_from_channel() {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let original = vec![100u32, 200, 300, 400, 500];
    let original_for_assert = original.clone();

    let encoded = encode_to_vec(&original).expect("Vec<u32> encode failed");
    tx.send(encoded).expect("channel send failed");

    let consumer = thread::spawn(move || {
        let enc = rx.recv().expect("channel recv failed");
        let (decoded, _): (Vec<u32>, _) =
            decode_from_slice(&enc).expect("Vec<u32> decode in thread failed");
        decoded
    });

    let result = consumer.join().expect("consumer panicked");
    assert_eq!(
        result, original_for_assert,
        "Vec<u32> decoded in thread mismatch"
    );
}

// 18. Encode fixed array [u8; 8] across 4 threads
#[test]
fn test_parallel_encode_fixed_array_u8_8_four_threads() {
    let arr: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
    let expected = encode_to_vec(&arr).expect("baseline encode failed");

    let handles: Vec<_> = (0..4)
        .map(|_| thread::spawn(move || encode_to_vec(&arr).expect("fixed array encode failed")))
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(
            result, expected,
            "fixed array encoding mismatch across threads"
        );
    }
}

// 19. Multiple threads each encode unique data, collect all results
#[test]
fn test_multiple_threads_encode_unique_data_collect_results() {
    let handles: Vec<_> = (0u32..8)
        .map(|i| {
            thread::spawn(move || {
                let unique_val = i * 1000 + i;
                let enc = encode_to_vec(&unique_val).expect("unique encode failed");
                (i, unique_val, enc)
            })
        })
        .collect();

    let mut results: Vec<(u32, u32, Vec<u8>)> = handles
        .into_iter()
        .map(|h| h.join().expect("thread panicked"))
        .collect();

    results.sort_by_key(|(i, _, _)| *i);
    assert_eq!(results.len(), 8, "expected 8 results");

    for (i, unique_val, enc) in &results {
        let expected = encode_to_vec(unique_val).expect("baseline encode failed");
        assert_eq!(*enc, expected, "unique encode mismatch for thread {}", i);
    }
}

// 20. Thread panic safety: encode doesn't affect other threads
#[test]
fn test_thread_panic_safety_encode_does_not_affect_others() {
    let (tx, rx) = std::sync::mpsc::channel::<Result<Vec<u8>, String>>();

    // Thread that will succeed
    let tx1 = tx.clone();
    let success_thread = thread::spawn(move || {
        let enc = encode_to_vec(&12345u64).expect("success encode failed");
        tx1.send(Ok(enc)).expect("send failed");
    });

    // Thread that intentionally panics — we catch it via join result
    let panic_thread = thread::spawn(|| -> () {
        panic!("intentional panic for thread safety test");
    });

    // Thread that will also succeed
    let tx2 = tx.clone();
    let success_thread2 = thread::spawn(move || {
        let enc = encode_to_vec(&67890u64).expect("success encode 2 failed");
        tx2.send(Ok(enc)).expect("send 2 failed");
    });

    drop(tx); // close sender so receiver terminates

    success_thread.join().expect("success thread 1 panicked");
    let panic_result = panic_thread.join();
    assert!(
        panic_result.is_err(),
        "panic thread should have returned Err"
    );
    success_thread2.join().expect("success thread 2 panicked");

    let mut success_count = 0;
    for msg in rx.iter() {
        let enc = msg.expect("unexpected error from success thread");
        assert!(!enc.is_empty(), "encoded bytes should not be empty");
        success_count += 1;
    }
    assert_eq!(success_count, 2, "exactly 2 success encodes expected");
}

// 21. Encode struct with String field across 8 threads
#[test]
fn test_encode_struct_with_string_field_eight_threads() {
    let record = NamedRecord {
        name: String::from("OxiCode"),
        score: 100,
    };
    let record_arc = Arc::new(record);
    let expected = encode_to_vec(&*record_arc).expect("baseline encode failed");

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let r = Arc::clone(&record_arc);
            thread::spawn(move || encode_to_vec(&*r).expect("named record encode failed"))
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(
            result, expected,
            "NamedRecord encoding mismatch across threads"
        );
    }
}

// 22. All 8 threads decode same bytes, produce equal results
#[test]
fn test_all_eight_threads_decode_same_bytes_produce_equal_results() {
    let record = NamedRecord {
        name: String::from("shared decode"),
        score: 255,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let encoded_arc = Arc::new(encoded);

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let enc = Arc::clone(&encoded_arc);
            thread::spawn(move || {
                let (decoded, _): (NamedRecord, _) =
                    decode_from_slice(&enc).expect("named record decode failed");
                decoded
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread panicked");
        assert_eq!(
            result, record,
            "all 8 threads must decode to the same NamedRecord"
        );
    }
}
