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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Endpoint {
    host: String,
    port: u16,
    secure: bool,
}

#[test]
fn test_box_endpoint_roundtrip() {
    let val = Box::new(Endpoint {
        host: "localhost".to_string(),
        port: 8080,
        secure: true,
    });
    let bytes = encode_to_vec(&val).expect("encode Box<Endpoint> failed");
    let (decoded, _): (Box<Endpoint>, usize) =
        decode_from_slice(&bytes).expect("decode Box<Endpoint> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_box_u32_zero_roundtrip() {
    let val: Box<u32> = Box::new(0u32);
    let bytes = encode_to_vec(&val).expect("encode Box<u32> zero failed");
    let (decoded, _): (Box<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Box<u32> zero failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_box_u64_max_roundtrip() {
    let val: Box<u64> = Box::new(u64::MAX);
    let bytes = encode_to_vec(&val).expect("encode Box<u64::MAX> failed");
    let (decoded, _): (Box<u64>, usize) =
        decode_from_slice(&bytes).expect("decode Box<u64::MAX> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_box_string_roundtrip() {
    let val: Box<String> = Box::new("hello oxicode".to_string());
    let bytes = encode_to_vec(&val).expect("encode Box<String> failed");
    let (decoded, _): (Box<String>, usize) =
        decode_from_slice(&bytes).expect("decode Box<String> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_box_vec_u8_roundtrip() {
    let val: Box<Vec<u8>> = Box::new(vec![10u8, 20, 30, 40, 50]);
    let bytes = encode_to_vec(&val).expect("encode Box<Vec<u8>> failed");
    let (decoded, _): (Box<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Box<Vec<u8>> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_box_endpoint_same_bytes_as_raw() {
    let raw = Endpoint {
        host: "example.com".to_string(),
        port: 443,
        secure: true,
    };
    let boxed = Box::new(Endpoint {
        host: "example.com".to_string(),
        port: 443,
        secure: true,
    });
    let raw_bytes = encode_to_vec(&raw).expect("encode raw Endpoint failed");
    let box_bytes = encode_to_vec(&boxed).expect("encode Box<Endpoint> failed");
    assert_eq!(raw_bytes, box_bytes);
}

#[test]
fn test_rc_u32_roundtrip() {
    let val: Rc<u32> = Rc::new(42u32);
    let bytes = encode_to_vec(&val).expect("encode Rc<u32> failed");
    let (decoded, _): (Rc<u32>, usize) = decode_from_slice(&bytes).expect("decode Rc<u32> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_rc_endpoint_roundtrip() {
    let val: Rc<Endpoint> = Rc::new(Endpoint {
        host: "api.example.org".to_string(),
        port: 9090,
        secure: false,
    });
    let bytes = encode_to_vec(&val).expect("encode Rc<Endpoint> failed");
    let (decoded, _): (Rc<Endpoint>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<Endpoint> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_rc_vec_string_roundtrip() {
    let val: Rc<Vec<String>> = Rc::new(vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
    ]);
    let bytes = encode_to_vec(&val).expect("encode Rc<Vec<String>> failed");
    let (decoded, _): (Rc<Vec<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<Vec<String>> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_arc_u32_roundtrip() {
    let val: Arc<u32> = Arc::new(99u32);
    let bytes = encode_to_vec(&val).expect("encode Arc<u32> failed");
    let (decoded, _): (Arc<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<u32> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_arc_endpoint_roundtrip() {
    let val: Arc<Endpoint> = Arc::new(Endpoint {
        host: "secure.example.net".to_string(),
        port: 8443,
        secure: true,
    });
    let bytes = encode_to_vec(&val).expect("encode Arc<Endpoint> failed");
    let (decoded, _): (Arc<Endpoint>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Endpoint> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_arc_vec_u8_roundtrip() {
    let val: Arc<Vec<u8>> = Arc::new(vec![0u8, 1, 2, 3, 255]);
    let bytes = encode_to_vec(&val).expect("encode Arc<Vec<u8>> failed");
    let (decoded, _): (Arc<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Vec<u8>> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_box_endpoint_roundtrip() {
    let val: Vec<Box<Endpoint>> = vec![
        Box::new(Endpoint {
            host: "host-a".to_string(),
            port: 80,
            secure: false,
        }),
        Box::new(Endpoint {
            host: "host-b".to_string(),
            port: 443,
            secure: true,
        }),
        Box::new(Endpoint {
            host: "host-c".to_string(),
            port: 8080,
            secure: false,
        }),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Box<Endpoint>> failed");
    let (decoded, _): (Vec<Box<Endpoint>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Box<Endpoint>> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_rc_u32_roundtrip() {
    let val: Vec<Rc<u32>> = vec![Rc::new(1u32), Rc::new(2u32), Rc::new(3u32), Rc::new(4u32)];
    let bytes = encode_to_vec(&val).expect("encode Vec<Rc<u32>> failed");
    let (decoded, _): (Vec<Rc<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Rc<u32>> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_arc_u32_roundtrip() {
    let val: Vec<Arc<u32>> = vec![
        Arc::new(10u32),
        Arc::new(20u32),
        Arc::new(30u32),
        Arc::new(40u32),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Arc<u32>> failed");
    let (decoded, _): (Vec<Arc<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Arc<u32>> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_box_endpoint_some_roundtrip() {
    let val: Option<Box<Endpoint>> = Some(Box::new(Endpoint {
        host: "optional.host".to_string(),
        port: 3000,
        secure: false,
    }));
    let bytes = encode_to_vec(&val).expect("encode Option<Box<Endpoint>> Some failed");
    let (decoded, _): (Option<Box<Endpoint>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Box<Endpoint>> Some failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_box_endpoint_none_roundtrip() {
    let val: Option<Box<Endpoint>> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<Box<Endpoint>> None failed");
    let (decoded, _): (Option<Box<Endpoint>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Box<Endpoint>> None failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_rc_and_arc_same_bytes_for_same_value() {
    let rc_val: Rc<u32> = Rc::new(777u32);
    let arc_val: Arc<u32> = Arc::new(777u32);
    let rc_bytes = encode_to_vec(&rc_val).expect("encode Rc<u32> for comparison failed");
    let arc_bytes = encode_to_vec(&arc_val).expect("encode Arc<u32> for comparison failed");
    assert_eq!(
        rc_bytes, arc_bytes,
        "Rc and Arc should produce identical bytes for the same value"
    );
}

#[test]
fn test_box_u32_fixed_int_config_exactly_4_bytes() {
    let val: Box<u32> = Box::new(1u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Box<u32> with fixed-int config failed");
    assert_eq!(
        bytes.len(),
        4,
        "Box<u32> with fixed-int encoding should be exactly 4 bytes"
    );
    let (decoded, _): (Box<u32>, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode Box<u32> with fixed-int config failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_box_u32_big_endian_config_byte_order() {
    let val: Box<u32> = Box::new(0x01020304u32);
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Box<u32> big-endian failed");
    assert_eq!(bytes.len(), 4, "Box<u32> big-endian should be 4 bytes");
    assert_eq!(
        bytes[0], 0x01,
        "most significant byte should be first in big-endian"
    );
    assert_eq!(bytes[1], 0x02);
    assert_eq!(bytes[2], 0x03);
    assert_eq!(bytes[3], 0x04);
    let (decoded, _): (Box<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Box<u32> big-endian failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_consumed_bytes_equals_encoded_length_for_box_endpoint() {
    let val = Box::new(Endpoint {
        host: "measure.example".to_string(),
        port: 9999,
        secure: true,
    });
    let bytes = encode_to_vec(&val).expect("encode Box<Endpoint> for byte count failed");
    let (_, consumed): (Box<Endpoint>, usize) =
        decode_from_slice(&bytes).expect("decode Box<Endpoint> for byte count failed");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes should equal the total encoded length"
    );
}

#[test]
fn test_box_fixed_array_u8_8_roundtrip() {
    let val: Box<[u8; 8]> = Box::new([0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44]);
    let bytes = encode_to_vec(&val).expect("encode Box<[u8; 8]> failed");
    let (decoded, _): (Box<[u8; 8]>, usize) =
        decode_from_slice(&bytes).expect("decode Box<[u8; 8]> failed");
    assert_eq!(val, decoded);
}
