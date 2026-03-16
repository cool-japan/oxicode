//! Advanced tests (set 3) for Box<T>/Rc<T>/Arc<T> encoding in OxiCode.
//! 22 tests covering struct/enum roundtrips, collections, Option wrappers,
//! wire-byte identity, fixed-int config, and multi-record scenarios.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};
use std::rc::Rc;
use std::sync::Arc;

// ── Shared test types ─────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Record {
    id: u32,
    name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Suspended(String),
}

// ── Test 1: Box<Record> roundtrip ─────────────────────────────────────────────

#[test]
fn test_box_record_roundtrip() {
    let original: Box<Record> = Box::new(Record {
        id: 1,
        name: "Alice".to_string(),
    });
    let bytes = encode_to_vec(&original).expect("encode Box<Record>");
    let (decoded, _): (Box<Record>, usize) = decode_from_slice(&bytes).expect("decode Box<Record>");
    assert_eq!(*decoded, *original);
}

// ── Test 2: Box<Status::Active> roundtrip ────────────────────────────────────

#[test]
fn test_box_status_active_roundtrip() {
    let original: Box<Status> = Box::new(Status::Active);
    let bytes = encode_to_vec(&original).expect("encode Box<Status::Active>");
    let (decoded, _): (Box<Status>, usize) =
        decode_from_slice(&bytes).expect("decode Box<Status::Active>");
    assert_eq!(*decoded, Status::Active);
}

// ── Test 3: Box<Status::Suspended> roundtrip ─────────────────────────────────

#[test]
fn test_box_status_suspended_roundtrip() {
    let original: Box<Status> = Box::new(Status::Suspended("maintenance".to_string()));
    let bytes = encode_to_vec(&original).expect("encode Box<Status::Suspended>");
    let (decoded, _): (Box<Status>, usize) =
        decode_from_slice(&bytes).expect("decode Box<Status::Suspended>");
    assert_eq!(*decoded, Status::Suspended("maintenance".to_string()));
}

// ── Test 4: Rc<Record> roundtrip ─────────────────────────────────────────────

#[test]
fn test_rc_record_roundtrip() {
    let original: Rc<Record> = Rc::new(Record {
        id: 42,
        name: "Bob".to_string(),
    });
    let bytes = encode_to_vec(&original).expect("encode Rc<Record>");
    let (decoded, _): (Rc<Record>, usize) = decode_from_slice(&bytes).expect("decode Rc<Record>");
    assert_eq!(*decoded, *original);
}

// ── Test 5: Rc<Status> roundtrip ─────────────────────────────────────────────

#[test]
fn test_rc_status_roundtrip() {
    let original: Rc<Status> = Rc::new(Status::Inactive);
    let bytes = encode_to_vec(&original).expect("encode Rc<Status>");
    let (decoded, _): (Rc<Status>, usize) = decode_from_slice(&bytes).expect("decode Rc<Status>");
    assert_eq!(*decoded, Status::Inactive);
}

// ── Test 6: Arc<Record> roundtrip ────────────────────────────────────────────

#[test]
fn test_arc_record_roundtrip() {
    let original: Arc<Record> = Arc::new(Record {
        id: 100,
        name: "Carol".to_string(),
    });
    let bytes = encode_to_vec(&original).expect("encode Arc<Record>");
    let (decoded, _): (Arc<Record>, usize) = decode_from_slice(&bytes).expect("decode Arc<Record>");
    assert_eq!(*decoded, *original);
}

// ── Test 7: Arc<Status> roundtrip ────────────────────────────────────────────

#[test]
fn test_arc_status_roundtrip() {
    let original: Arc<Status> = Arc::new(Status::Suspended("network_issue".to_string()));
    let bytes = encode_to_vec(&original).expect("encode Arc<Status>");
    let (decoded, _): (Arc<Status>, usize) = decode_from_slice(&bytes).expect("decode Arc<Status>");
    assert_eq!(*decoded, Status::Suspended("network_issue".to_string()));
}

// ── Test 8: Box<Vec<u32>> roundtrip ──────────────────────────────────────────

#[test]
fn test_box_vec_u32_roundtrip() {
    let original: Box<Vec<u32>> = Box::new(vec![1u32, 2, 3, 4, 5]);
    let bytes = encode_to_vec(&original).expect("encode Box<Vec<u32>>");
    let (decoded, _): (Box<Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Box<Vec<u32>>");
    assert_eq!(*decoded, vec![1u32, 2, 3, 4, 5]);
}

// ── Test 9: Rc<Vec<String>> roundtrip ────────────────────────────────────────

#[test]
fn test_rc_vec_string_roundtrip() {
    let original: Rc<Vec<String>> = Rc::new(vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
    ]);
    let bytes = encode_to_vec(&original).expect("encode Rc<Vec<String>>");
    let (decoded, _): (Rc<Vec<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<Vec<String>>");
    assert_eq!(*decoded, *original);
}

// ── Test 10: Arc<Vec<u8>> roundtrip ──────────────────────────────────────────

#[test]
fn test_arc_vec_u8_roundtrip() {
    let original: Arc<Vec<u8>> = Arc::new(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let bytes = encode_to_vec(&original).expect("encode Arc<Vec<u8>>");
    let (decoded, _): (Arc<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Vec<u8>>");
    assert_eq!(*decoded, vec![0xDE_u8, 0xAD, 0xBE, 0xEF]);
}

// ── Test 11: Vec<Box<Record>> roundtrip ──────────────────────────────────────

#[test]
fn test_vec_box_record_roundtrip() {
    let original: Vec<Box<Record>> = vec![
        Box::new(Record {
            id: 1,
            name: "r1".to_string(),
        }),
        Box::new(Record {
            id: 2,
            name: "r2".to_string(),
        }),
        Box::new(Record {
            id: 3,
            name: "r3".to_string(),
        }),
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<Box<Record>>");
    let (decoded, _): (Vec<Box<Record>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Box<Record>>");
    assert_eq!(decoded.len(), 3);
    for (dec, orig) in decoded.iter().zip(original.iter()) {
        assert_eq!(**dec, **orig);
    }
}

// ── Test 12: Vec<Rc<u32>> roundtrip ──────────────────────────────────────────

#[test]
fn test_vec_rc_u32_roundtrip() {
    let original: Vec<Rc<u32>> = vec![Rc::new(10u32), Rc::new(20u32), Rc::new(30u32)];
    let bytes = encode_to_vec(&original).expect("encode Vec<Rc<u32>>");
    let (decoded, _): (Vec<Rc<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Rc<u32>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(*decoded[0], 10u32);
    assert_eq!(*decoded[1], 20u32);
    assert_eq!(*decoded[2], 30u32);
}

// ── Test 13: Vec<Arc<Record>> roundtrip ──────────────────────────────────────

#[test]
fn test_vec_arc_record_roundtrip() {
    let original: Vec<Arc<Record>> = vec![
        Arc::new(Record {
            id: 7,
            name: "seven".to_string(),
        }),
        Arc::new(Record {
            id: 8,
            name: "eight".to_string(),
        }),
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<Arc<Record>>");
    let (decoded, _): (Vec<Arc<Record>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Arc<Record>>");
    assert_eq!(decoded.len(), 2);
    assert_eq!(*decoded[0], *original[0]);
    assert_eq!(*decoded[1], *original[1]);
}

// ── Test 14: Option<Box<Record>> Some roundtrip ───────────────────────────────

#[test]
fn test_option_box_record_some_roundtrip() {
    let original: Option<Box<Record>> = Some(Box::new(Record {
        id: 99,
        name: "opt_some".to_string(),
    }));
    let bytes = encode_to_vec(&original).expect("encode Option<Box<Record>>(Some)");
    let (decoded, _): (Option<Box<Record>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Box<Record>>(Some)");
    assert!(decoded.is_some());
    assert_eq!(
        *decoded.expect("inner Some"),
        Record {
            id: 99,
            name: "opt_some".to_string()
        }
    );
}

// ── Test 15: Option<Arc<Record>> None roundtrip ───────────────────────────────

#[test]
fn test_option_arc_record_none_roundtrip() {
    let original: Option<Arc<Record>> = None;
    let bytes = encode_to_vec(&original).expect("encode Option<Arc<Record>>(None)");
    let (decoded, _): (Option<Arc<Record>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Arc<Record>>(None)");
    assert!(decoded.is_none());
}

// ── Test 16: Box<u32> and raw u32 produce identical wire bytes ────────────────

#[test]
fn test_box_u32_identical_wire_bytes_to_plain_u32() {
    let plain: u32 = 1234u32;
    let boxed: Box<u32> = Box::new(1234u32);

    let bytes_plain = encode_to_vec(&plain).expect("encode u32");
    let bytes_boxed = encode_to_vec(&boxed).expect("encode Box<u32>");

    assert_eq!(
        bytes_plain, bytes_boxed,
        "Box<u32> must produce identical wire bytes to plain u32"
    );
}

// ── Test 17: Rc<u32> and Arc<u32> produce identical wire bytes ───────────────

#[test]
fn test_rc_u32_and_arc_u32_identical_wire_bytes() {
    let rc_val: Rc<u32> = Rc::new(5678u32);
    let arc_val: Arc<u32> = Arc::new(5678u32);

    let bytes_rc = encode_to_vec(&rc_val).expect("encode Rc<u32>");
    let bytes_arc = encode_to_vec(&arc_val).expect("encode Arc<u32>");

    assert_eq!(
        bytes_rc, bytes_arc,
        "Rc<u32> and Arc<u32> must produce identical wire bytes"
    );
}

// ── Test 18: Box<u32> fixed int config (4 bytes) ─────────────────────────────

#[test]
fn test_box_u32_fixed_int_config_four_bytes() {
    let original: Box<u32> = Box::new(7u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode Box<u32> fixed int");
    // Fixed-int encoding always uses exactly 4 bytes for u32
    assert_eq!(
        bytes.len(),
        4,
        "Box<u32> with fixed int encoding should be exactly 4 bytes"
    );
    let (decoded, _): (Box<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Box<u32> fixed int");
    assert_eq!(*decoded, 7u32);
}

// ── Test 19: Arc<u64> fixed int config (8 bytes) ─────────────────────────────

#[test]
fn test_arc_u64_fixed_int_config_eight_bytes() {
    let original: Arc<u64> = Arc::new(255u64);
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode Arc<u64> fixed int");
    // Fixed-int encoding always uses exactly 8 bytes for u64
    assert_eq!(
        bytes.len(),
        8,
        "Arc<u64> with fixed int encoding should be exactly 8 bytes"
    );
    let (decoded, _): (Arc<u64>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Arc<u64> fixed int");
    assert_eq!(*decoded, 255u64);
}

// ── Test 20: Box<u128> roundtrip ─────────────────────────────────────────────

#[test]
fn test_box_u128_roundtrip() {
    let val: u128 = u128::MAX / 3;
    let original: Box<u128> = Box::new(val);
    let bytes = encode_to_vec(&original).expect("encode Box<u128>");
    let (decoded, _): (Box<u128>, usize) = decode_from_slice(&bytes).expect("decode Box<u128>");
    assert_eq!(*decoded, val);
}

// ── Test 21: Box<Record> consumed bytes equals encoded len ───────────────────

#[test]
fn test_box_record_consumed_bytes_equals_encoded_len() {
    let original: Box<Record> = Box::new(Record {
        id: 77,
        name: "consumed_check".to_string(),
    });
    let bytes = encode_to_vec(&original).expect("encode Box<Record> for size check");
    let (_decoded, consumed): (Box<Record>, usize) =
        decode_from_slice(&bytes).expect("decode Box<Record> for size check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ── Test 22: Rc<Vec<Record>> 5 records roundtrip ─────────────────────────────

#[test]
fn test_rc_vec_record_five_records_roundtrip() {
    let records: Vec<Record> = (1u32..=5)
        .map(|i| Record {
            id: i,
            name: format!("record_{i}"),
        })
        .collect();
    let original: Rc<Vec<Record>> = Rc::new(records);
    let bytes = encode_to_vec(&original).expect("encode Rc<Vec<Record>>");
    let (decoded, consumed): (Rc<Vec<Record>>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<Vec<Record>>");
    assert_eq!(decoded.len(), 5, "should have 5 records");
    assert_eq!(consumed, bytes.len(), "all bytes should be consumed");
    for (i, record) in decoded.iter().enumerate() {
        let expected_id = (i + 1) as u32;
        assert_eq!(record.id, expected_id);
        assert_eq!(record.name, format!("record_{expected_id}"));
    }
}
