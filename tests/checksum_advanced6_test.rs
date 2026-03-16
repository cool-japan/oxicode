//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced6_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{
    decode_with_checksum, encode_with_checksum, verify_checksum, wrap_with_checksum, HEADER_SIZE,
};
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Shared helper types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Event {
    id: u64,
    name: String,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Level {
    Debug,
    Info,
    Warn,
    Error(String),
}

// ---------------------------------------------------------------------------
// Test 1: u64 primitive checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_primitive_checksum_roundtrip() {
    let value: u64 = 18_446_744_073_709_551_000u64;
    let encoded = encode_with_checksum(&value).expect("encode u64 failed");
    let (decoded, consumed): (u64, _) = decode_with_checksum(&encoded).expect("decode u64 failed");
    assert_eq!(decoded, value, "decoded u64 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 2: i32 primitive checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i32_primitive_checksum_roundtrip() {
    let value: i32 = -2_147_483_000i32;
    let encoded = encode_with_checksum(&value).expect("encode i32 failed");
    let (decoded, consumed): (i32, _) = decode_with_checksum(&encoded).expect("decode i32 failed");
    assert_eq!(decoded, value, "decoded i32 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 3: f64 primitive checksum roundtrip (bit-exact)
// ---------------------------------------------------------------------------
#[test]
fn test_f64_primitive_checksum_roundtrip_bit_exact() {
    let value: f64 = std::f64::consts::E;
    let encoded = encode_with_checksum(&value).expect("encode f64 E failed");
    let (decoded, consumed): (f64, _) =
        decode_with_checksum(&encoded).expect("decode f64 E failed");
    assert_eq!(
        decoded.to_bits(),
        value.to_bits(),
        "decoded f64 must be bit-exact equal to original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 4: bool primitive checksum roundtrip (both true and false)
// ---------------------------------------------------------------------------
#[test]
fn test_bool_primitive_checksum_roundtrip() {
    let encoded_true = encode_with_checksum(&true).expect("encode bool true failed");
    let (decoded_true, consumed_true): (bool, _) =
        decode_with_checksum(&encoded_true).expect("decode bool true failed");
    assert!(decoded_true, "decoded value must be true");
    assert_eq!(
        consumed_true,
        encoded_true.len(),
        "consumed must equal encoded_true length"
    );

    let encoded_false = encode_with_checksum(&false).expect("encode bool false failed");
    let (decoded_false, consumed_false): (bool, _) =
        decode_with_checksum(&encoded_false).expect("decode bool false failed");
    assert!(!decoded_false, "decoded value must be false");
    assert_eq!(
        consumed_false,
        encoded_false.len(),
        "consumed must equal encoded_false length"
    );

    assert_ne!(
        encoded_true, encoded_false,
        "true and false must produce different checksummed byte sequences"
    );
}

// ---------------------------------------------------------------------------
// Test 5: u128 primitive checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u128_primitive_checksum_roundtrip() {
    let value: u128 = u128::MAX / 3;
    let encoded = encode_with_checksum(&value).expect("encode u128 failed");
    let (decoded, consumed): (u128, _) =
        decode_with_checksum(&encoded).expect("decode u128 failed");
    assert_eq!(decoded, value, "decoded u128 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Event struct checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_event_struct_checksum_roundtrip() {
    let value = Event {
        id: 42,
        name: String::from("user.login"),
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let encoded = encode_with_checksum(&value).expect("encode Event failed");
    let (decoded, consumed): (Event, _) =
        decode_with_checksum(&encoded).expect("decode Event failed");
    assert_eq!(decoded, value, "decoded Event must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Level enum variants checksum roundtrip (all four variants)
// ---------------------------------------------------------------------------
#[test]
fn test_level_enum_variants_checksum_roundtrip() {
    let variants: Vec<Level> = vec![
        Level::Debug,
        Level::Info,
        Level::Warn,
        Level::Error(String::from("disk full")),
    ];
    for variant in &variants {
        let encoded = encode_with_checksum(variant).expect("encode Level variant failed");
        let (decoded, consumed): (Level, _) =
            decode_with_checksum(&encoded).expect("decode Level variant failed");
        assert_eq!(
            &decoded, variant,
            "decoded Level variant must equal original"
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed must equal total encoded length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 8: Vec<Event> checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_event_checksum_roundtrip() {
    let value: Vec<Event> = vec![
        Event {
            id: 1,
            name: String::from("start"),
            data: vec![0x01],
        },
        Event {
            id: 2,
            name: String::from("process"),
            data: vec![0x02, 0x03],
        },
        Event {
            id: 3,
            name: String::from("end"),
            data: vec![],
        },
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<Event> failed");
    let (decoded, consumed): (Vec<Event>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<Event> failed");
    assert_eq!(decoded, value, "decoded Vec<Event> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Option<Event> Some and None checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_event_checksum_roundtrip() {
    let some_value: Option<Event> = Some(Event {
        id: 999,
        name: String::from("optional"),
        data: vec![0xFF],
    });
    let encoded_some = encode_with_checksum(&some_value).expect("encode Option<Event> Some failed");
    let (decoded_some, consumed_some): (Option<Event>, _) =
        decode_with_checksum(&encoded_some).expect("decode Option<Event> Some failed");
    assert_eq!(
        decoded_some, some_value,
        "decoded Option<Event> Some must equal original"
    );
    assert_eq!(
        consumed_some,
        encoded_some.len(),
        "consumed must equal encoded_some length"
    );

    let none_value: Option<Event> = None;
    let encoded_none = encode_with_checksum(&none_value).expect("encode Option<Event> None failed");
    let (decoded_none, consumed_none): (Option<Event>, _) =
        decode_with_checksum(&encoded_none).expect("decode Option<Event> None failed");
    assert_eq!(
        decoded_none, none_value,
        "decoded Option<Event> None must equal original"
    );
    assert!(decoded_none.is_none(), "decoded option must be None");
    assert_eq!(
        consumed_none,
        encoded_none.len(),
        "consumed must equal encoded_none length"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Level::Error with long string checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_level_error_long_string_checksum_roundtrip() {
    let long_msg = "e".repeat(1024);
    let value = Level::Error(long_msg.clone());
    let encoded = encode_with_checksum(&value).expect("encode Level::Error long string failed");
    let (decoded, consumed): (Level, _) =
        decode_with_checksum(&encoded).expect("decode Level::Error long string failed");
    assert_eq!(decoded, value, "decoded Level::Error must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
    if let Level::Error(msg) = decoded {
        assert_eq!(msg, long_msg, "inner error message must equal original");
    } else {
        panic!("decoded value must be Level::Error");
    }
}

// ---------------------------------------------------------------------------
// Test 11: Corruption detection — single byte flip in Event payload
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detection_single_byte_flip_in_event_payload() {
    let value = Event {
        id: 7,
        name: String::from("corrupt-me"),
        data: vec![0xAA; 32],
    };
    let mut encoded = encode_with_checksum(&value).expect("encode Event failed");
    let mid = HEADER_SIZE + (encoded.len() - HEADER_SIZE) / 2;
    encoded[mid] ^= 0x01;
    let result = verify_checksum(&encoded);
    assert!(
        result.is_err(),
        "verify_checksum must detect a single byte flip in Event payload"
    );
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "error must be ChecksumMismatch, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 12: Empty data field in Event checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_empty_data_field_in_event_checksum_roundtrip() {
    let value = Event {
        id: 0,
        name: String::new(),
        data: vec![],
    };
    let encoded = encode_with_checksum(&value).expect("encode empty-data Event failed");
    let (decoded, consumed): (Event, _) =
        decode_with_checksum(&encoded).expect("decode empty-data Event failed");
    assert_eq!(
        decoded, value,
        "decoded empty-data Event must equal original"
    );
    assert!(decoded.name.is_empty(), "decoded event name must be empty");
    assert!(decoded.data.is_empty(), "decoded event data must be empty");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Large Event data (5000 bytes) checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_large_event_data_5000_bytes_checksum_roundtrip() {
    let large_data: Vec<u8> = (0u8..=255).cycle().take(5000).collect();
    let value = Event {
        id: u64::MAX,
        name: String::from("large-event"),
        data: large_data.clone(),
    };
    let encoded = encode_with_checksum(&value).expect("encode large Event failed");
    let (decoded, consumed): (Event, _) =
        decode_with_checksum(&encoded).expect("decode large Event failed");
    assert_eq!(decoded, value, "decoded large Event must equal original");
    assert_eq!(
        decoded.data.len(),
        5000,
        "decoded event data length must be 5000"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Unicode string in Event name checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_unicode_string_in_event_name_checksum_roundtrip() {
    let value = Event {
        id: 256,
        name: String::from("日本語テスト — émojis 🦀 and symbols ∑∞√"),
        data: vec![0x01, 0x02, 0x03],
    };
    let encoded = encode_with_checksum(&value).expect("encode unicode Event failed");
    let (decoded, consumed): (Event, _) =
        decode_with_checksum(&encoded).expect("decode unicode Event failed");
    assert_eq!(decoded, value, "decoded unicode Event must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Vec of Level variants checksum roundtrip (mixed)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_level_variants_mixed_checksum_roundtrip() {
    let value: Vec<Level> = vec![
        Level::Debug,
        Level::Error(String::from("timeout")),
        Level::Info,
        Level::Warn,
        Level::Error(String::from("out of memory")),
        Level::Debug,
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<Level> mixed failed");
    let (decoded, consumed): (Vec<Level>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<Level> mixed failed");
    assert_eq!(decoded, value, "decoded Vec<Level> must equal original");
    assert_eq!(decoded.len(), 6, "decoded vec must have exactly 6 elements");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Checksum encoded size is strictly larger than plain encoded size
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_encoded_size_strictly_larger_than_plain_encoded_size() {
    let value = Event {
        id: 1,
        name: String::from("size-check"),
        data: vec![0x00, 0x01, 0x02],
    };
    let plain = encode_to_vec(&value).expect("plain encode Event failed");
    let checked = encode_with_checksum(&value).expect("checksum encode Event failed");
    assert!(
        checked.len() > plain.len(),
        "checksum encoded size ({}) must be strictly larger than plain encoded size ({})",
        checked.len(),
        plain.len()
    );
    assert_eq!(
        checked.len() - plain.len(),
        HEADER_SIZE,
        "size difference must be exactly HEADER_SIZE ({}) bytes",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 17: Encoding the same Event twice is deterministic
// ---------------------------------------------------------------------------
#[test]
fn test_encoding_same_event_twice_is_deterministic() {
    let value = Event {
        id: 42,
        name: String::from("deterministic"),
        data: vec![0xCA, 0xFE],
    };
    let first = encode_with_checksum(&value).expect("first encode Event failed");
    let second = encode_with_checksum(&value).expect("second encode Event failed");
    assert_eq!(
        first, second,
        "encoding the same Event twice must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Standard config encode_to_vec can decode plain, checksum path separate
// ---------------------------------------------------------------------------
#[test]
fn test_standard_config_and_checksum_paths_are_independent() {
    let value = Level::Warn;
    let plain = encode_to_vec(&value).expect("plain encode Level::Warn failed");
    let (decoded_plain, _): (Level, _) =
        decode_from_slice(&plain).expect("plain decode Level::Warn failed");
    assert_eq!(
        decoded_plain, value,
        "plain decoded Level must equal original"
    );

    let checked = encode_with_checksum(&value).expect("checksum encode Level::Warn failed");
    let (decoded_checked, _): (Level, _) =
        decode_with_checksum(&checked).expect("checksum decode Level::Warn failed");
    assert_eq!(
        decoded_checked, value,
        "checksum decoded Level must equal original"
    );

    assert_ne!(
        plain, checked,
        "plain and checksum encoded bytes must differ"
    );

    let _cfg = config::standard();
}

// ---------------------------------------------------------------------------
// Test 19: Consumed bytes equals total slice length for all Event variants
// ---------------------------------------------------------------------------
#[test]
fn test_consumed_bytes_equals_total_slice_length_for_events() {
    let events: Vec<Event> = vec![
        Event {
            id: 0,
            name: String::from("zero"),
            data: vec![],
        },
        Event {
            id: 1,
            name: String::from("one"),
            data: vec![1],
        },
        Event {
            id: 100,
            name: String::from("hundred"),
            data: vec![1; 100],
        },
    ];
    for event in &events {
        let encoded = encode_with_checksum(event).expect("encode Event failed");
        let (_decoded, consumed): (Event, _) =
            decode_with_checksum(&encoded).expect("decode Event failed");
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed bytes ({}) must equal total encoded length ({}) for event id={}",
            consumed,
            encoded.len(),
            event.id
        );
    }
}

// ---------------------------------------------------------------------------
// Test 20: Truncated Event checksum data returns Err
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_event_checksum_data_returns_err() {
    let value = Event {
        id: 77,
        name: String::from("truncate-test"),
        data: vec![0xBE, 0xEF],
    };
    let encoded = encode_with_checksum(&value).expect("encode Event failed");
    let half = encoded.len() / 2;
    let truncated = &encoded[..half];
    let result = decode_with_checksum::<Event>(truncated);
    assert!(
        result.is_err(),
        "decode_with_checksum must return Err when Event payload is truncated"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec<Level> with all four variant types checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_level_all_four_variants_checksum_roundtrip() {
    let value: Vec<Level> = vec![
        Level::Debug,
        Level::Info,
        Level::Warn,
        Level::Error(String::from("critical failure: connection refused")),
        Level::Error(String::new()),
        Level::Debug,
        Level::Info,
        Level::Warn,
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<Level> all variants failed");
    let (decoded, consumed): (Vec<Level>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<Level> all variants failed");
    assert_eq!(decoded, value, "decoded Vec<Level> must equal original");
    assert_eq!(decoded.len(), 8, "decoded vec must have exactly 8 elements");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );

    // Verify wrap_with_checksum + verify_checksum on a raw sub-payload
    let raw = b"raw level data check";
    let wrapped = wrap_with_checksum(raw);
    let recovered = verify_checksum(&wrapped).expect("verify wrapped raw data failed");
    assert_eq!(
        recovered, raw,
        "wrap_with_checksum + verify_checksum must recover original"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Nested Option<Vec<Event>> checksum roundtrip (Some and None)
// ---------------------------------------------------------------------------
#[test]
fn test_nested_option_vec_event_checksum_roundtrip() {
    let some_value: Option<Vec<Event>> = Some(vec![
        Event {
            id: 1,
            name: String::from("alpha"),
            data: vec![0x01, 0x02],
        },
        Event {
            id: 2,
            name: String::from("beta"),
            data: vec![],
        },
        Event {
            id: 3,
            name: String::from("gamma — unicode 日本語"),
            data: (0u8..128).collect(),
        },
    ]);
    let encoded_some =
        encode_with_checksum(&some_value).expect("encode Option<Vec<Event>> Some failed");
    let (decoded_some, consumed_some): (Option<Vec<Event>>, _) =
        decode_with_checksum(&encoded_some).expect("decode Option<Vec<Event>> Some failed");
    assert_eq!(
        decoded_some, some_value,
        "decoded Option<Vec<Event>> Some must equal original"
    );
    assert_eq!(
        consumed_some,
        encoded_some.len(),
        "consumed must equal encoded_some length"
    );
    if let Some(ref events) = decoded_some {
        assert_eq!(events.len(), 3, "decoded inner vec must have 3 events");
        assert_eq!(
            events[2].data.len(),
            128,
            "third event data must have 128 bytes"
        );
    } else {
        panic!("decoded value must be Some");
    }

    let none_value: Option<Vec<Event>> = None;
    let encoded_none =
        encode_with_checksum(&none_value).expect("encode Option<Vec<Event>> None failed");
    let (decoded_none, consumed_none): (Option<Vec<Event>>, _) =
        decode_with_checksum(&encoded_none).expect("decode Option<Vec<Event>> None failed");
    assert_eq!(
        decoded_none, none_value,
        "decoded Option<Vec<Event>> None must equal original"
    );
    assert!(decoded_none.is_none(), "decoded option must be None");
    assert_eq!(
        consumed_none,
        encoded_none.len(),
        "consumed must equal encoded_none length"
    );
}
