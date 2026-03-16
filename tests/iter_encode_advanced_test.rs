// Advanced iterator-based encoding tests for OxiCode.
// 22 top-level #[test] functions — no #[cfg(test)] module wrapper.

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
// ── Test 1 ──────────────────────────────────────────────────────────────────
// Encoding an empty iterator must produce the same bytes as encoding an
// empty Vec (a single varint-zero length prefix).
#[test]
fn iter_encode_empty_matches_empty_vec() {
    let iter_bytes =
        oxicode::encode_iter_to_vec(std::iter::empty::<u32>()).expect("encode empty iter failed");
    let vec_bytes = oxicode::encode_to_vec(&Vec::<u32>::new()).expect("encode empty vec failed");
    assert_eq!(
        iter_bytes, vec_bytes,
        "empty iterator must produce identical bytes to empty Vec"
    );
}

// ── Test 2 ──────────────────────────────────────────────────────────────────
// Encode [1u32, 2, 3] via iterator and decode back via decode_iter_from_slice;
// the three items must round-trip without loss.
#[test]
fn iter_encode_three_u32s_roundtrip() {
    let original = [1u32, 2, 3];
    let encoded =
        oxicode::encode_iter_to_vec(original.iter().copied()).expect("encode [1,2,3] failed");
    let decoded: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&encoded)
        .expect("decode_iter init failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect failed");
    assert_eq!(decoded, original.as_slice());
}

// ── Test 3 ──────────────────────────────────────────────────────────────────
// Encoding 100 u64 values from a range iterator and verifying the decoded
// sum equals the Gauss formula 0+1+…+99 = 4950.
#[test]
fn iter_encode_100_u64_sum_roundtrip() {
    let encoded = oxicode::encode_iter_to_vec(0u64..100).expect("encode 0..100 u64 failed");
    let sum: u64 = oxicode::decode_iter_from_slice::<u64>(&encoded)
        .expect("decode_iter init for 100 u64s failed")
        .map(|r| r.expect("item decode failed"))
        .sum();
    assert_eq!(sum, 4950, "sum of 0..100 must be 4950");
}

// ── Test 4 ──────────────────────────────────────────────────────────────────
// Encode a String iterator and verify full round-trip preservation.
#[test]
fn iter_encode_strings_roundtrip() {
    let items: Vec<String> = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
        String::new(),
    ];
    let encoded =
        oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode string iter failed");
    let decoded: Vec<String> = oxicode::decode_iter_from_slice::<String>(&encoded)
        .expect("decode_iter init for strings failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect strings failed");
    assert_eq!(decoded, items);
}

// ── Test 5 ──────────────────────────────────────────────────────────────────
// Encode a bool iterator; true/false values must round-trip exactly.
#[test]
fn iter_encode_bool_roundtrip() {
    let items: Vec<bool> = vec![true, false, false, true, true];
    let encoded =
        oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode bool iter failed");
    let decoded: Vec<bool> = oxicode::decode_iter_from_slice::<bool>(&encoded)
        .expect("decode_iter init for bools failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect bools failed");
    assert_eq!(decoded, items);
}

// ── Test 6 ──────────────────────────────────────────────────────────────────
// Encode a u8 iterator covering the full 0-255 range; byte count and values
// must be preserved after decode.
#[test]
fn iter_encode_u8_full_range() {
    let encoded = oxicode::encode_iter_to_vec(0u8..=255).expect("encode u8 0..=255 iter failed");
    let decoded: Vec<u8> = oxicode::decode_iter_from_slice::<u8>(&encoded)
        .expect("decode_iter init for u8 range failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect u8 range failed");
    assert_eq!(decoded.len(), 256, "must decode 256 bytes");
    assert_eq!(decoded[0], 0u8);
    assert_eq!(decoded[255], 255u8);
}

// ── Test 7 ──────────────────────────────────────────────────────────────────
// Encode an i32 iterator that includes negative values; round-trip must
// preserve sign and magnitude.
#[test]
fn iter_encode_i32_negatives_roundtrip() {
    let items: Vec<i32> = vec![-1_000_000, -1, 0, 1, 1_000_000, i32::MIN, i32::MAX];
    let encoded =
        oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode i32 iter failed");
    let decoded: Vec<i32> = oxicode::decode_iter_from_slice::<i32>(&encoded)
        .expect("decode_iter init for i32 failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect i32 values failed");
    assert_eq!(decoded, items);
}

// ── Test 8 ──────────────────────────────────────────────────────────────────
// DecodeIter::collect() into a Vec must yield the same items as encoding a Vec
// directly; both paths should give byte-identical encodings.
#[test]
fn decode_iter_collect_matches_encode_to_vec() {
    let source: Vec<u16> = (0u16..50).collect();
    let encoded = oxicode::encode_to_vec(&source).expect("encode Vec<u16> failed");
    let decoded: Vec<u16> = oxicode::decode_iter_from_slice::<u16>(&encoded)
        .expect("decode_iter init failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect u16 items failed");
    assert_eq!(decoded, source);
}

// ── Test 9 ──────────────────────────────────────────────────────────────────
// DecodeIter must stop at end of data returning None, never panicking.
#[test]
fn decode_iter_stops_at_end_of_data() {
    let data: Vec<u32> = vec![10, 20, 30];
    let encoded = oxicode::encode_to_vec(&data).expect("encode for eos test failed");
    let mut iter = oxicode::decode_iter_from_slice::<u32>(&encoded)
        .expect("decode_iter init for eos test failed");

    // Drain all items
    for _ in 0..3 {
        iter.next()
            .expect("expected Some")
            .expect("expected Ok item");
    }
    // Next call must return None
    assert!(
        iter.next().is_none(),
        "iterator must return None after all items are consumed"
    );
}

// ── Test 10 ─────────────────────────────────────────────────────────────────
// Encode then decode 1000 items; full round-trip must preserve every value.
#[test]
fn iter_encode_1000_items_full_roundtrip() {
    let encoded = oxicode::encode_iter_to_vec(0u32..1000).expect("encode 0..1000 failed");
    let decoded: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&encoded)
        .expect("decode_iter init for 1000 items failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect 1000 items failed");
    assert_eq!(decoded.len(), 1000, "must decode exactly 1000 items");
    assert_eq!(decoded[0], 0u32);
    assert_eq!(decoded[999], 999u32);
}

// ── Test 11 ─────────────────────────────────────────────────────────────────
// Encode an iterator of structs (requires derive feature).
#[cfg(feature = "derive")]
#[test]
fn iter_encode_structs_roundtrip() {
    use oxicode::{Decode, Encode};

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    struct Pair {
        key: u32,
        value: i64,
    }

    let items: Vec<Pair> = (0u32..5)
        .map(|k| Pair {
            key: k,
            value: -(k as i64) * 7,
        })
        .collect();

    let encoded =
        oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode Pair iter failed");
    let decoded: Vec<Pair> = oxicode::decode_iter_from_slice::<Pair>(&encoded)
        .expect("decode_iter init for Pair failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect Pairs failed");
    assert_eq!(decoded, items);
}

// ── Test 12 ─────────────────────────────────────────────────────────────────
// Encode an iterator of Option<u32> containing a mix of Some and None.
#[test]
fn iter_encode_option_some_none_mix() {
    let items: Vec<Option<u32>> = vec![Some(1), None, Some(u32::MAX), None, None, Some(0)];
    let encoded =
        oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode Option iter failed");
    let decoded: Vec<Option<u32>> = oxicode::decode_iter_from_slice::<Option<u32>>(&encoded)
        .expect("decode_iter init for Option failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect Option items failed");
    assert_eq!(decoded, items);
}

// ── Test 13 ─────────────────────────────────────────────────────────────────
// Encode an iterator of enums covering all variants.
#[cfg(feature = "derive")]
#[test]
fn iter_encode_enum_all_variants() {
    use oxicode::{Decode, Encode};

    #[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let items = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::North,
    ];
    let encoded =
        oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode Direction iter failed");
    let decoded: Vec<Direction> = oxicode::decode_iter_from_slice::<Direction>(&encoded)
        .expect("decode_iter init for Direction failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect Direction items failed");
    assert_eq!(decoded, items);
}

// ── Test 14 ─────────────────────────────────────────────────────────────────
// Encode iterator with fixed-int encoding config; bytes must be wider (4 bytes
// per u32 instead of varint) but round-trip must still be correct.
#[test]
fn iter_encode_fixed_int_config_roundtrip() {
    let items = [10u32, 20, 30, 40];
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let encoded = oxicode::encode_iter_to_vec_with_config(items.iter().copied(), cfg)
        .expect("encode with fixed-int config failed");
    // With fixed int: 8 bytes (u64 count) + 4 × 4 bytes = 24 bytes total
    assert_eq!(
        encoded.len(),
        8 + 4 * 4,
        "fixed-int: 8-byte length + 4×4-byte u32 = 24 bytes"
    );
    let decoded: Vec<u32> = oxicode::decode_iter_from_slice_with_config::<u32, _>(&encoded, cfg)
        .expect("decode_iter init with fixed-int config failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect fixed-int items failed");
    assert_eq!(decoded, items.as_slice());
}

// ── Test 15 ─────────────────────────────────────────────────────────────────
// encode_iter_to_vec bytes are identical to encode_to_vec for the same data.
// Verifies both paths produce the same wire format.
#[test]
fn iter_encode_bytes_match_vec_encoding() {
    let data: Vec<u64> = (0u64..10).map(|x| x * x).collect();
    let iter_bytes =
        oxicode::encode_iter_to_vec(data.iter().copied()).expect("encode_iter_to_vec failed");
    let vec_bytes = oxicode::encode_to_vec(&data).expect("encode_to_vec failed");
    assert_eq!(
        iter_bytes, vec_bytes,
        "encode_iter_to_vec and encode_to_vec must produce identical bytes"
    );
}

// ── Test 16 ─────────────────────────────────────────────────────────────────
// Encode two separate iterators sequentially into the same Vec; decode them
// back in order by feeding consecutive slices.
#[test]
fn iter_encode_two_sequences_sequential_decode() {
    let first: Vec<u32> = vec![1, 2, 3];
    let second: Vec<u32> = vec![4, 5, 6, 7];

    let mut buf = oxicode::encode_to_vec(&first).expect("encode first failed");
    let second_bytes = oxicode::encode_to_vec(&second).expect("encode second failed");
    buf.extend_from_slice(&second_bytes);

    // Decode first sequence
    let (decoded_first, consumed): (Vec<u32>, _) =
        oxicode::decode_from_slice(&buf).expect("decode first failed");
    assert_eq!(decoded_first, first);

    // Decode second sequence from the remainder
    let (decoded_second, _): (Vec<u32>, _) =
        oxicode::decode_from_slice(&buf[consumed..]).expect("decode second failed");
    assert_eq!(decoded_second, second);
}

// ── Test 17 ─────────────────────────────────────────────────────────────────
// Decode only the first N items from an encoded sequence using take(); the
// remaining items are silently ignored.
#[test]
fn decode_iter_take_first_n_items() {
    let source: Vec<u32> = (0u32..50).collect();
    let encoded = oxicode::encode_to_vec(&source).expect("encode 50-item source failed");

    let first_ten: Vec<u32> = oxicode::decode_iter_from_slice::<u32>(&encoded)
        .expect("decode_iter init for take test failed")
        .take(10)
        .collect::<Result<Vec<_>, _>>()
        .expect("collect first 10 items failed");

    assert_eq!(first_ten.len(), 10, "take(10) must yield exactly 10 items");
    let expected: Vec<u32> = (0u32..10).collect();
    assert_eq!(first_ten, expected);
}

// ── Test 18 ─────────────────────────────────────────────────────────────────
// Encoding an empty iterator produces minimal bytes: exactly 1 byte (varint 0).
#[test]
fn iter_encode_empty_produces_single_byte() {
    let encoded = oxicode::encode_iter_to_vec(std::iter::empty::<u64>())
        .expect("encode empty u64 iter failed");
    assert_eq!(
        encoded.len(),
        1,
        "empty iterator must encode to exactly 1 byte"
    );
    assert_eq!(encoded[0], 0x00, "that 1 byte must be 0x00 (varint zero)");
}

// ── Test 19 ─────────────────────────────────────────────────────────────────
// Encode an iterator of Vec<u8> items; each inner Vec is variable length and
// must survive the round-trip intact.
#[test]
fn iter_encode_vec_u8_items_roundtrip() {
    let items: Vec<Vec<u8>> = vec![
        vec![],
        vec![0xDE, 0xAD],
        vec![0x00; 32],
        (0u8..=127).collect(),
    ];
    let encoded =
        oxicode::encode_iter_to_vec(items.iter().cloned()).expect("encode Vec<u8> iter failed");
    let decoded: Vec<Vec<u8>> = oxicode::decode_iter_from_slice::<Vec<u8>>(&encoded)
        .expect("decode_iter init for Vec<u8> items failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect Vec<u8> items failed");
    assert_eq!(decoded, items);
}

// ── Test 20 ─────────────────────────────────────────────────────────────────
// Encode an iterator of (u32, u32) tuples; heterogeneous layout must
// round-trip without corruption.
#[test]
fn iter_encode_tuple_u32_u32_roundtrip() {
    let items: Vec<(u32, u32)> = vec![(0, u32::MAX), (1, 1), (42, 84), (100, 200)];
    let encoded =
        oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode (u32,u32) iter failed");
    let decoded: Vec<(u32, u32)> = oxicode::decode_iter_from_slice::<(u32, u32)>(&encoded)
        .expect("decode_iter init for tuples failed")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect tuples failed");
    assert_eq!(decoded, items);
}

// ── Test 21 ─────────────────────────────────────────────────────────────────
// Verify that encoded byte count grows with item count: encoding N+1 items
// must produce at least as many bytes as encoding N items for non-empty items.
#[test]
fn iter_encode_byte_count_grows_with_items() {
    let encoded_3 = oxicode::encode_iter_to_vec(std::iter::repeat(1u32).take(3))
        .expect("encode 3 items failed");
    let encoded_6 = oxicode::encode_iter_to_vec(std::iter::repeat(1u32).take(6))
        .expect("encode 6 items failed");
    let encoded_12 = oxicode::encode_iter_to_vec(std::iter::repeat(1u32).take(12))
        .expect("encode 12 items failed");

    assert!(
        encoded_6.len() > encoded_3.len(),
        "encoding 6 items must produce more bytes than 3 items"
    );
    assert!(
        encoded_12.len() > encoded_6.len(),
        "encoding 12 items must produce more bytes than 6 items"
    );
}

// ── Test 22 ─────────────────────────────────────────────────────────────────
// encode_seq_to_vec (ExactSizeIterator path) produces the same bytes as
// encode_iter_to_vec (generic IntoIterator path) for the same data.
// Also validates encode_seq_into_slice round-trip.
#[test]
fn encode_seq_to_vec_matches_encode_iter_to_vec() {
    let data: Vec<u32> = vec![10, 20, 30, 40, 50];

    let iter_bytes =
        oxicode::encode_iter_to_vec(data.iter().copied()).expect("encode_iter_to_vec failed");
    let seq_bytes =
        oxicode::encode_seq_to_vec(data.iter().copied()).expect("encode_seq_to_vec failed");
    assert_eq!(
        iter_bytes, seq_bytes,
        "encode_iter_to_vec and encode_seq_to_vec must produce identical bytes"
    );

    // Also verify encode_seq_into_slice round-trip
    let mut buf = vec![0u8; 128];
    let written = oxicode::encode_seq_into_slice(data.iter().copied(), &mut buf)
        .expect("encode_seq_into_slice failed");
    let (decoded_slice, _): (Vec<u32>, _) =
        oxicode::decode_from_slice(&buf[..written]).expect("decode from slice buf failed");
    assert_eq!(decoded_slice, data);
}
