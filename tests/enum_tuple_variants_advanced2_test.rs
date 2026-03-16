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

#[derive(Debug, PartialEq, Encode, Decode)]
enum Shape {
    Point,
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Payload {
    Empty,
    Bytes(Vec<u8>),
    Text(String),
    Pair(u32, u32),
    Triple(u8, u16, u32),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Nested {
    Inner(Shape),
    Outer(Box<Nested>),
    Leaf(u32),
}

#[test]
fn test_shape_point_roundtrip() {
    let val = Shape::Point;
    let bytes = encode_to_vec(&val).expect("encode Shape::Point");
    let (decoded, _): (Shape, usize) = decode_from_slice(&bytes).expect("decode Shape::Point");
    assert_eq!(val, decoded);
}

#[test]
fn test_shape_circle_roundtrip_bit_exact() {
    let val = Shape::Circle(3.14_f64);
    let bytes = encode_to_vec(&val).expect("encode Shape::Circle");
    let (decoded, _): (Shape, usize) = decode_from_slice(&bytes).expect("decode Shape::Circle");
    assert_eq!(val, decoded);
    // Verify bit-exact f64 reconstruction
    if let Shape::Circle(r) = decoded {
        assert_eq!(r.to_bits(), 3.14_f64.to_bits());
    } else {
        panic!("expected Shape::Circle");
    }
}

#[test]
fn test_shape_rectangle_roundtrip() {
    let val = Shape::Rectangle(2.0, 4.0);
    let bytes = encode_to_vec(&val).expect("encode Shape::Rectangle");
    let (decoded, _): (Shape, usize) = decode_from_slice(&bytes).expect("decode Shape::Rectangle");
    assert_eq!(val, decoded);
}

#[test]
fn test_shape_triangle_roundtrip() {
    let val = Shape::Triangle(3.0, 4.0, 5.0);
    let bytes = encode_to_vec(&val).expect("encode Shape::Triangle");
    let (decoded, _): (Shape, usize) = decode_from_slice(&bytes).expect("decode Shape::Triangle");
    assert_eq!(val, decoded);
}

#[test]
fn test_payload_empty_roundtrip() {
    let val = Payload::Empty;
    let bytes = encode_to_vec(&val).expect("encode Payload::Empty");
    let (decoded, _): (Payload, usize) = decode_from_slice(&bytes).expect("decode Payload::Empty");
    assert_eq!(val, decoded);
}

#[test]
fn test_payload_bytes_roundtrip() {
    let val = Payload::Bytes(vec![1, 2, 3]);
    let bytes = encode_to_vec(&val).expect("encode Payload::Bytes");
    let (decoded, _): (Payload, usize) = decode_from_slice(&bytes).expect("decode Payload::Bytes");
    assert_eq!(val, decoded);
}

#[test]
fn test_payload_text_roundtrip() {
    let val = Payload::Text(String::from("hello"));
    let bytes = encode_to_vec(&val).expect("encode Payload::Text");
    let (decoded, _): (Payload, usize) = decode_from_slice(&bytes).expect("decode Payload::Text");
    assert_eq!(val, decoded);
}

#[test]
fn test_payload_pair_roundtrip() {
    let val = Payload::Pair(10, 20);
    let bytes = encode_to_vec(&val).expect("encode Payload::Pair");
    let (decoded, _): (Payload, usize) = decode_from_slice(&bytes).expect("decode Payload::Pair");
    assert_eq!(val, decoded);
}

#[test]
fn test_payload_triple_roundtrip() {
    let val = Payload::Triple(1, 2, 3);
    let bytes = encode_to_vec(&val).expect("encode Payload::Triple");
    let (decoded, _): (Payload, usize) = decode_from_slice(&bytes).expect("decode Payload::Triple");
    assert_eq!(val, decoded);
}

#[test]
fn test_different_shape_variants_produce_different_discriminant_bytes() {
    let point_bytes = encode_to_vec(&Shape::Point).expect("encode Point");
    let circle_bytes = encode_to_vec(&Shape::Circle(1.0)).expect("encode Circle");
    let rect_bytes = encode_to_vec(&Shape::Rectangle(1.0, 2.0)).expect("encode Rectangle");
    let tri_bytes = encode_to_vec(&Shape::Triangle(1.0, 2.0, 3.0)).expect("encode Triangle");

    // First byte is discriminant — all four must differ
    assert_ne!(
        point_bytes[0], circle_bytes[0],
        "Point and Circle must have different discriminant bytes"
    );
    assert_ne!(
        point_bytes[0], rect_bytes[0],
        "Point and Rectangle must have different discriminant bytes"
    );
    assert_ne!(
        point_bytes[0], tri_bytes[0],
        "Point and Triangle must have different discriminant bytes"
    );
    assert_ne!(
        circle_bytes[0], rect_bytes[0],
        "Circle and Rectangle must have different discriminant bytes"
    );
    assert_ne!(
        circle_bytes[0], tri_bytes[0],
        "Circle and Triangle must have different discriminant bytes"
    );
    assert_ne!(
        rect_bytes[0], tri_bytes[0],
        "Rectangle and Triangle must have different discriminant bytes"
    );
}

#[test]
fn test_circle_discriminant_byte_is_1() {
    // Shape::Circle is the second variant (index 1)
    let bytes = encode_to_vec(&Shape::Circle(0.0)).expect("encode Circle");
    // With variable-length encoding, discriminant 1 encodes as 0x01
    assert_eq!(
        bytes[0], 1,
        "Circle (second variant, index 1) must have discriminant byte == 1"
    );
}

#[test]
fn test_vec_of_all_shape_variants_roundtrip() {
    let shapes = vec![
        Shape::Point,
        Shape::Circle(1.5),
        Shape::Rectangle(3.0, 6.0),
        Shape::Triangle(5.0, 12.0, 13.0),
    ];
    let bytes = encode_to_vec(&shapes).expect("encode Vec<Shape>");
    let (decoded, _): (Vec<Shape>, usize) = decode_from_slice(&bytes).expect("decode Vec<Shape>");
    assert_eq!(shapes, decoded);
}

#[test]
fn test_nested_leaf_roundtrip() {
    let val = Nested::Leaf(42);
    let bytes = encode_to_vec(&val).expect("encode Nested::Leaf");
    let (decoded, _): (Nested, usize) = decode_from_slice(&bytes).expect("decode Nested::Leaf");
    assert_eq!(val, decoded);
}

#[test]
fn test_nested_inner_shape_circle_roundtrip() {
    let val = Nested::Inner(Shape::Circle(1.0));
    let bytes = encode_to_vec(&val).expect("encode Nested::Inner(Circle)");
    let (decoded, _): (Nested, usize) =
        decode_from_slice(&bytes).expect("decode Nested::Inner(Circle)");
    assert_eq!(val, decoded);
}

#[test]
fn test_nested_outer_box_roundtrip() {
    let val = Nested::Outer(Box::new(Nested::Leaf(99)));
    let bytes = encode_to_vec(&val).expect("encode Nested::Outer(Box::Leaf)");
    let (decoded, _): (Nested, usize) =
        decode_from_slice(&bytes).expect("decode Nested::Outer(Box::Leaf)");
    assert_eq!(val, decoded);
}

#[test]
fn test_consumed_bytes_equals_encoded_length_for_payload_text() {
    let val = Payload::Text(String::from("oxicode"));
    let bytes = encode_to_vec(&val).expect("encode Payload::Text for consumed-bytes check");
    let (_, consumed): (Payload, usize) =
        decode_from_slice(&bytes).expect("decode Payload::Text for consumed-bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

#[test]
fn test_shape_with_fixed_int_encoding_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = Shape::Circle(2.718_f64);
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Shape::Circle with fixed_int_encoding");
    let (decoded, _): (Shape, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode Shape::Circle with fixed_int_encoding");
    assert_eq!(val, decoded);
}

#[test]
fn test_shape_with_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = Shape::Rectangle(7.0, 11.0);
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Shape::Rectangle with big_endian");
    let (decoded, _): (Shape, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode Shape::Rectangle with big_endian");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_shape_some_circle_roundtrip() {
    let val: Option<Shape> = Some(Shape::Circle(9.81));
    let bytes = encode_to_vec(&val).expect("encode Option<Shape>::Some(Circle)");
    let (decoded, _): (Option<Shape>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Shape>::Some(Circle)");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_shape_none_roundtrip() {
    let val: Option<Shape> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<Shape>::None");
    let (decoded, _): (Option<Shape>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Shape>::None");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_of_mixed_payload_variants_roundtrip() {
    let payloads = vec![
        Payload::Empty,
        Payload::Bytes(vec![10, 20, 30]),
        Payload::Text(String::from("rust")),
        Payload::Pair(100, 200),
        Payload::Triple(7, 8, 9),
    ];
    let bytes = encode_to_vec(&payloads).expect("encode Vec<Payload> mixed variants");
    let (decoded, _): (Vec<Payload>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Payload> mixed variants");
    assert_eq!(payloads, decoded);
}

#[test]
fn test_shape_point_encodes_to_1_byte() {
    // Shape::Point is the first (unit) variant — no payload, only a discriminant byte
    let bytes = encode_to_vec(&Shape::Point).expect("encode Shape::Point for size check");
    assert_eq!(
        bytes.len(),
        1,
        "Shape::Point (unit variant) must encode to exactly 1 byte (discriminant only)"
    );
}
