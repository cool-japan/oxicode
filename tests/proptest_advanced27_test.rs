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
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Event {
    id: u64,
    kind: u8,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EventKind {
    Start,
    Stop,
    Data(Vec<u8>),
}

// Test 1: prop u8 array [u8; 4] roundtrip
proptest! {
    #[test]
    fn prop_u8_array4_roundtrip(a in any::<u8>(), b in any::<u8>(), c in any::<u8>(), d in any::<u8>()) {
        let val: [u8; 4] = [a, b, c, d];
        let bytes = encode_to_vec(&val).expect("encode [u8; 4] failed");
        let (decoded, _) = decode_from_slice::<[u8; 4]>(&bytes).expect("decode [u8; 4] failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 2: prop u8 array [u8; 8] roundtrip
proptest! {
    #[test]
    fn prop_u8_array8_roundtrip(
        a in any::<u8>(), b in any::<u8>(), c in any::<u8>(), d in any::<u8>(),
        e in any::<u8>(), f in any::<u8>(), g in any::<u8>(), h in any::<u8>()
    ) {
        let val: [u8; 8] = [a, b, c, d, e, f, g, h];
        let bytes = encode_to_vec(&val).expect("encode [u8; 8] failed");
        let (decoded, _) = decode_from_slice::<[u8; 8]>(&bytes).expect("decode [u8; 8] failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 3: prop [u32; 4] roundtrip
proptest! {
    #[test]
    fn prop_u32_array4_roundtrip(
        a in any::<u32>(), b in any::<u32>(), c in any::<u32>(), d in any::<u32>()
    ) {
        let val: [u32; 4] = [a, b, c, d];
        let bytes = encode_to_vec(&val).expect("encode [u32; 4] failed");
        let (decoded, _) = decode_from_slice::<[u32; 4]>(&bytes).expect("decode [u32; 4] failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 4: prop Vec<bool> roundtrip (0..30 items)
proptest! {
    #[test]
    fn prop_vec_bool_roundtrip(val in prop::collection::vec(any::<bool>(), 0..30)) {
        let bytes = encode_to_vec(&val).expect("encode Vec<bool> failed");
        let (decoded, _) = decode_from_slice::<Vec<bool>>(&bytes).expect("decode Vec<bool> failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 5: prop Vec<u64> roundtrip (0..20 items)
proptest! {
    #[test]
    fn prop_vec_u64_roundtrip(val in prop::collection::vec(any::<u64>(), 0..20)) {
        let bytes = encode_to_vec(&val).expect("encode Vec<u64> failed");
        let (decoded, _) = decode_from_slice::<Vec<u64>>(&bytes).expect("decode Vec<u64> failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 6: prop Vec<i32> roundtrip (0..15 items)
proptest! {
    #[test]
    fn prop_vec_i32_roundtrip(val in prop::collection::vec(any::<i32>(), 0..15)) {
        let bytes = encode_to_vec(&val).expect("encode Vec<i32> failed");
        let (decoded, _) = decode_from_slice::<Vec<i32>>(&bytes).expect("decode Vec<i32> failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 7: prop (u32, u32, u32) triple roundtrip
proptest! {
    #[test]
    fn prop_u32_triple_roundtrip(a in any::<u32>(), b in any::<u32>(), c in any::<u32>()) {
        let val = (a, b, c);
        let bytes = encode_to_vec(&val).expect("encode (u32, u32, u32) failed");
        let (decoded, _) = decode_from_slice::<(u32, u32, u32)>(&bytes).expect("decode (u32, u32, u32) failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 8: prop (String, u64) pair roundtrip
proptest! {
    #[test]
    fn prop_string_u64_pair_roundtrip(s in "[a-zA-Z0-9 ]{0,30}", n in any::<u64>()) {
        let val = (s, n);
        let bytes = encode_to_vec(&val).expect("encode (String, u64) failed");
        let (decoded, _) = decode_from_slice::<(String, u64)>(&bytes).expect("decode (String, u64) failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 9: prop Event roundtrip (any u64/u8, 0..20 payload bytes)
proptest! {
    #[test]
    fn prop_event_roundtrip(
        id in any::<u64>(),
        kind in any::<u8>(),
        payload in prop::collection::vec(any::<u8>(), 0..20)
    ) {
        let val = Event { id, kind, payload };
        let bytes = encode_to_vec(&val).expect("encode Event failed");
        let (decoded, _) = decode_from_slice::<Event>(&bytes).expect("decode Event failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 10: prop EventKind::Start roundtrip (constant, but via proptest to be consistent)
proptest! {
    #[test]
    fn prop_event_kind_start_roundtrip(_dummy in any::<u8>()) {
        let val = EventKind::Start;
        let bytes = encode_to_vec(&val).expect("encode EventKind::Start failed");
        let (decoded, _) = decode_from_slice::<EventKind>(&bytes).expect("decode EventKind::Start failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 11: prop EventKind::Data roundtrip (0..50 bytes)
proptest! {
    #[test]
    fn prop_event_kind_data_roundtrip(data in prop::collection::vec(any::<u8>(), 0..50)) {
        let val = EventKind::Data(data);
        let bytes = encode_to_vec(&val).expect("encode EventKind::Data failed");
        let (decoded, _) = decode_from_slice::<EventKind>(&bytes).expect("decode EventKind::Data failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 12: prop Vec<Event> roundtrip (0..5 items)
proptest! {
    #[test]
    fn prop_vec_event_roundtrip(
        events in prop::collection::vec(
            (any::<u64>(), any::<u8>(), prop::collection::vec(any::<u8>(), 0..20))
                .prop_map(|(id, kind, payload)| Event { id, kind, payload }),
            0..5
        )
    ) {
        let bytes = encode_to_vec(&events).expect("encode Vec<Event> failed");
        let (decoded, _) = decode_from_slice::<Vec<Event>>(&bytes).expect("decode Vec<Event> failed");
        prop_assert_eq!(events, decoded);
    }
}

// Test 13: prop double-encode roundtrip for Event (encode→decode→encode, compare second bytes with first)
proptest! {
    #[test]
    fn prop_event_double_encode_roundtrip(
        id in any::<u64>(),
        kind in any::<u8>(),
        payload in prop::collection::vec(any::<u8>(), 0..20)
    ) {
        let val = Event { id, kind, payload };
        let bytes1 = encode_to_vec(&val).expect("first encode Event failed");
        let (decoded, _) = decode_from_slice::<Event>(&bytes1).expect("decode Event failed");
        let bytes2 = encode_to_vec(&decoded).expect("second encode Event failed");
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Test 14: prop consumed bytes == encoded length for Vec<u8>
proptest! {
    #[test]
    fn prop_consumed_bytes_vec_u8(val in prop::collection::vec(any::<u8>(), 0..30)) {
        let bytes = encode_to_vec(&val).expect("encode Vec<u8> failed");
        let encoded_len = bytes.len();
        let (_, consumed) = decode_from_slice::<Vec<u8>>(&bytes).expect("decode Vec<u8> failed");
        prop_assert_eq!(consumed, encoded_len);
    }
}

// Test 15: prop consumed bytes == encoded length for Event
proptest! {
    #[test]
    fn prop_consumed_bytes_event(
        id in any::<u64>(),
        kind in any::<u8>(),
        payload in prop::collection::vec(any::<u8>(), 0..20)
    ) {
        let val = Event { id, kind, payload };
        let bytes = encode_to_vec(&val).expect("encode Event failed");
        let encoded_len = bytes.len();
        let (_, consumed) = decode_from_slice::<Event>(&bytes).expect("decode Event failed");
        prop_assert_eq!(consumed, encoded_len);
    }
}

// Test 16: prop usize roundtrip
proptest! {
    #[test]
    fn prop_usize_roundtrip(val in any::<usize>()) {
        let bytes = encode_to_vec(&val).expect("encode usize failed");
        let (decoded, _) = decode_from_slice::<usize>(&bytes).expect("decode usize failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 17: prop isize roundtrip
proptest! {
    #[test]
    fn prop_isize_roundtrip(val in any::<isize>()) {
        let bytes = encode_to_vec(&val).expect("encode isize failed");
        let (decoded, _) = decode_from_slice::<isize>(&bytes).expect("decode isize failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 18: prop (u8, i8) roundtrip
proptest! {
    #[test]
    fn prop_u8_i8_pair_roundtrip(a in any::<u8>(), b in any::<i8>()) {
        let val = (a, b);
        let bytes = encode_to_vec(&val).expect("encode (u8, i8) failed");
        let (decoded, _) = decode_from_slice::<(u8, i8)>(&bytes).expect("decode (u8, i8) failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 19: prop String roundtrip (use "[a-zA-Z0-9 ]{0,30}")
proptest! {
    #[test]
    fn prop_string_roundtrip(val in "[a-zA-Z0-9 ]{0,30}") {
        let bytes = encode_to_vec(&val).expect("encode String failed");
        let (decoded, _) = decode_from_slice::<String>(&bytes).expect("decode String failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 20: prop Option<Event> roundtrip
proptest! {
    #[test]
    fn prop_option_event_roundtrip(
        opt in prop::option::of(
            (any::<u64>(), any::<u8>(), prop::collection::vec(any::<u8>(), 0..20))
                .prop_map(|(id, kind, payload)| Event { id, kind, payload })
        )
    ) {
        let bytes = encode_to_vec(&opt).expect("encode Option<Event> failed");
        let (decoded, _) = decode_from_slice::<Option<Event>>(&bytes).expect("decode Option<Event> failed");
        prop_assert_eq!(opt, decoded);
    }
}

// Test 21: prop encode same Event twice gives same bytes
proptest! {
    #[test]
    fn prop_event_encode_deterministic(
        id in any::<u64>(),
        kind in any::<u8>(),
        payload in prop::collection::vec(any::<u8>(), 0..20)
    ) {
        let val = Event { id, kind, payload };
        let bytes1 = encode_to_vec(&val).expect("first encode Event (determinism) failed");
        let bytes2 = encode_to_vec(&val).expect("second encode Event (determinism) failed");
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Test 22: prop [u8; 16] roundtrip
proptest! {
    #[test]
    fn prop_u8_array16_roundtrip(
        a0 in any::<u8>(), a1 in any::<u8>(), a2 in any::<u8>(), a3 in any::<u8>(),
        a4 in any::<u8>(), a5 in any::<u8>(), a6 in any::<u8>(), a7 in any::<u8>(),
        a8 in any::<u8>(), a9 in any::<u8>(), a10 in any::<u8>(), a11 in any::<u8>(),
        a12 in any::<u8>(), a13 in any::<u8>(), a14 in any::<u8>(), a15 in any::<u8>()
    ) {
        let val: [u8; 16] = [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15];
        let bytes = encode_to_vec(&val).expect("encode [u8; 16] failed");
        let (decoded, _) = decode_from_slice::<[u8; 16]>(&bytes).expect("decode [u8; 16] failed");
        prop_assert_eq!(val, decoded);
    }
}
