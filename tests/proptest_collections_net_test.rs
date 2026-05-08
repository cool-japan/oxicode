//! Collections and network types property-based roundtrip tests using proptest
//! (split from proptest_test.rs).
//!
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
use oxicode::{decode_from_slice, encode_to_vec};
use proptest::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

// --- Task 2: HashMap/BTreeMap roundtrips ---

proptest! {
    #[test]
    fn prop_roundtrip_btreemap(
        pairs in proptest::collection::btree_map(
            proptest::string::string_regex("[a-z]{1,10}").unwrap(),
            0i32..1000i32,
            0..20
        )
    ) {
        let enc = encode_to_vec(&pairs).expect("encode");
        let (dec, _): (BTreeMap<String, i32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(pairs, dec);
    }
}

// --- Task 2: HashMap<String, u64> roundtrip ---

proptest! {
    #[test]
    fn prop_roundtrip_hashmap_string_u64(
        pairs in proptest::collection::hash_map(
            proptest::string::string_regex("[a-z]{1,15}").unwrap(),
            0u64..u64::MAX,
            0..30
        )
    ) {
        let enc = encode_to_vec(&pairs).expect("encode");
        let (dec, _): (HashMap<String, u64>, _) =
            decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(pairs, dec);
    }
}

// Ipv4Addr
proptest! {
    #[test]
    fn prop_roundtrip_ipv4addr(
        a in 0u8..=255u8, b in 0u8..=255u8, c in 0u8..=255u8, d in 0u8..=255u8
    ) {
        use std::net::Ipv4Addr;
        let addr = Ipv4Addr::new(a, b, c, d);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// Ipv6Addr
proptest! {
    #[test]
    fn prop_roundtrip_ipv6addr(
        a in 0u16..=65535u16, b in 0u16..=65535u16,
        c in 0u16..=65535u16, d in 0u16..=65535u16,
        e in 0u16..=65535u16, f in 0u16..=65535u16,
        g in 0u16..=65535u16, h in 0u16..=65535u16
    ) {
        use std::net::Ipv6Addr;
        let addr = Ipv6Addr::new(a, b, c, d, e, f, g, h);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// SocketAddrV4
proptest! {
    #[test]
    fn prop_roundtrip_socketaddrv4(
        a in 0u8..=255u8, b in 0u8..=255u8, c in 0u8..=255u8, d in 0u8..=255u8,
        port in 0u16..=65535u16
    ) {
        use std::net::{Ipv4Addr, SocketAddrV4};
        let addr = SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (SocketAddrV4, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// SocketAddrV6
proptest! {
    #[test]
    fn prop_roundtrip_socketaddrv6(
        a in 0u16..=65535u16, b in 0u16..=65535u16,
        c in 0u16..=65535u16, d in 0u16..=65535u16,
        e in 0u16..=65535u16, f in 0u16..=65535u16,
        g in 0u16..=65535u16, h in 0u16..=65535u16,
        port in 0u16..=65535u16
    ) {
        use std::net::{Ipv6Addr, SocketAddrV6};
        let addr = SocketAddrV6::new(Ipv6Addr::new(a, b, c, d, e, f, g, h), port, 0, 0);
        let enc = encode_to_vec(&addr).expect("encode");
        let (dec, _): (SocketAddrV6, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// NonZeroU32
proptest! {
    #[test]
    fn prop_roundtrip_nonzero_u32(v in 1u32..=u32::MAX) {
        use core::num::NonZeroU32;
        let nz = NonZeroU32::new(v).expect("nonzero");
        let enc = encode_to_vec(&nz).expect("encode");
        let (dec, _): (NonZeroU32, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(nz, dec);
    }
}

// Reverse<i32>
proptest! {
    #[test]
    fn prop_roundtrip_reverse_i32(v in i32::MIN..=i32::MAX) {
        use core::cmp::Reverse;
        let r = Reverse(v);
        let enc = encode_to_vec(&r).expect("encode");
        let (dec, _): (Reverse<i32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 9. prop_encoded_size_btreemap: BTreeMap<u32, u32>
proptest! {
    #[test]
    fn prop_encoded_size_btreemap(
        map in proptest::collection::btree_map(any::<u32>(), any::<u32>(), 0..50)
    ) {
        let size = oxicode::encoded_size(&map).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&map).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 12. prop_btreeset_roundtrip: BTreeSet<u32> limited to 200 elements
proptest! {
    #[test]
    fn prop_btreeset_roundtrip(
        set in proptest::collection::btree_set(any::<u32>(), 0..=200)
    ) {
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (BTreeSet<u32>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(set, dec);
    }
}

// 13. prop_hashset_roundtrip_len_preserved: HashSet<u8> roundtrip preserves length
proptest! {
    #[test]
    fn prop_hashset_roundtrip_len_preserved(
        set in proptest::collection::hash_set(any::<u8>(), 0..=256)
    ) {
        let original_len = set.len();
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (HashSet<u8>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(original_len, dec.len());
        prop_assert_eq!(set, dec);
    }
}

// 14. prop_duration_roundtrip: Duration::from_secs + from_nanos
proptest! {
    #[test]
    fn prop_duration_roundtrip(
        s in 0u64..1_000_000u64,
        n in 0u32..999_999_999u32
    ) {
        let dur = std::time::Duration::from_secs(s) + std::time::Duration::from_nanos(n as u64);
        let enc = oxicode::encode_to_vec(&dur).expect("encode");
        let (dec, _): (std::time::Duration, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(dur, dec);
    }
}

// 1. prop_vecdeque_roundtrip - VecDeque<u32> up to 500 elements
proptest! {
    #[test]
    fn prop_vecdeque_roundtrip(
        v in proptest::collection::vec(any::<u32>(), 0..=500)
    ) {
        let deque: VecDeque<u32> = v.into_iter().collect();
        let enc = oxicode::encode_to_vec(&deque).expect("encode");
        let (dec, _): (VecDeque<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(deque, dec);
    }
}

// 3. prop_hashmap_roundtrip - HashMap<u32, u32> up to 100 entries
proptest! {
    #[test]
    fn prop_hashmap_roundtrip(
        map in proptest::collection::hash_map(any::<u32>(), any::<u32>(), 0..=100)
    ) {
        let enc = oxicode::encode_to_vec(&map).expect("encode");
        let (dec, _): (HashMap<u32, u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(map, dec);
    }
}

// 4. prop_btreemap_roundtrip - BTreeMap<String, u64> up to 100 entries, strings max 50 chars
proptest! {
    #[test]
    fn prop_btreemap_roundtrip(
        map in proptest::collection::btree_map(
            proptest::string::string_regex("[a-zA-Z0-9]{1,50}").unwrap(),
            any::<u64>(),
            0..=100
        )
    ) {
        let enc = oxicode::encode_to_vec(&map).expect("encode");
        let (dec, _): (BTreeMap<String, u64>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(map, dec);
    }
}

// 5. prop_hashset_roundtrip - HashSet<u32> up to 200 elements
proptest! {
    #[test]
    fn prop_hashset_roundtrip(
        set in proptest::collection::hash_set(any::<u32>(), 0..=200)
    ) {
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (HashSet<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(set, dec);
    }
}

// 6. prop_btreeset_i32_roundtrip - BTreeSet<i32> up to 200 elements
proptest! {
    #[test]
    fn prop_btreeset_i32_roundtrip(
        set in proptest::collection::btree_set(any::<i32>(), 0..=200)
    ) {
        let enc = oxicode::encode_to_vec(&set).expect("encode");
        let (dec, _): (BTreeSet<i32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(set, dec);
    }
}

// 12. prop_large_btreemap_size - BTreeMap<u32, u32> with 500 entries, verify encoded_size matches encode_to_vec length
proptest! {
    #[test]
    fn prop_large_btreemap_size(
        map in proptest::collection::btree_map(any::<u32>(), any::<u32>(), 0..=500)
    ) {
        let size = oxicode::encoded_size(&map).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&map).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len(),
            "encoded_size must match encode_to_vec length for BTreeMap with {} entries",
            map.len()
        );
    }
}

// 11. prop_ipv4_from_octets_roundtrip - Ipv4Addr from 4 u8 values
proptest! {
    #[test]
    fn prop_ipv4_from_octets_roundtrip(a: u8, b: u8, c: u8, d: u8) {
        use std::net::Ipv4Addr;
        let addr = Ipv4Addr::new(a, b, c, d);
        let enc = oxicode::encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv4Addr, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}

// 12. prop_ipv6_from_segments_roundtrip - Ipv6Addr from 8 u16 segments
proptest! {
    #[test]
    fn prop_ipv6_from_segments_roundtrip(
        a: u16, b: u16, c: u16, d: u16,
        e: u16, f: u16, g: u16, h: u16
    ) {
        use std::net::Ipv6Addr;
        let addr = Ipv6Addr::new(a, b, c, d, e, f, g, h);
        let enc = oxicode::encode_to_vec(&addr).expect("encode");
        let (dec, _): (Ipv6Addr, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(addr, dec);
    }
}
