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
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Point,
}

fn arb_point3d() -> impl Strategy<Value = Point3D> {
    (any::<f32>(), any::<f32>(), any::<f32>()).prop_map(|(x, y, z)| Point3D { x, y, z })
}

fn arb_shape() -> impl Strategy<Value = Shape> {
    prop_oneof![
        any::<f32>().prop_map(|r| Shape::Circle { radius: r }),
        (any::<f32>(), any::<f32>()).prop_map(|(w, h)| Shape::Rectangle {
            width: w,
            height: h
        }),
        Just(Shape::Point),
    ]
}

#[test]
fn test_u8_roundtrip_identity() {
    proptest!(|(val: u8)| {
        let encoded = encode_to_vec(&val).expect("encode u8 failed");
        let (decoded, _) = decode_from_slice::<u8>(&encoded).expect("decode u8 failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_u16_roundtrip_identity() {
    proptest!(|(val: u16)| {
        let encoded = encode_to_vec(&val).expect("encode u16 failed");
        let (decoded, _) = decode_from_slice::<u16>(&encoded).expect("decode u16 failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_u64_roundtrip_identity() {
    proptest!(|(val: u64)| {
        let encoded = encode_to_vec(&val).expect("encode u64 failed");
        let (decoded, _) = decode_from_slice::<u64>(&encoded).expect("decode u64 failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_i64_roundtrip_identity() {
    proptest!(|(val: i64)| {
        let encoded = encode_to_vec(&val).expect("encode i64 failed");
        let (decoded, _) = decode_from_slice::<i64>(&encoded).expect("decode i64 failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_f32_bit_exact_roundtrip() {
    proptest!(|(bits: u32)| {
        let val = f32::from_bits(bits);
        let encoded = encode_to_vec(&val).expect("encode f32 failed");
        let (decoded, _) = decode_from_slice::<f32>(&encoded).expect("decode f32 failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    });
}

#[test]
fn test_f64_bit_exact_roundtrip() {
    proptest!(|(bits: u64)| {
        let val = f64::from_bits(bits);
        let encoded = encode_to_vec(&val).expect("encode f64 failed");
        let (decoded, _) = decode_from_slice::<f64>(&encoded).expect("decode f64 failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    });
}

#[test]
fn test_char_roundtrip() {
    proptest!(|(val: char)| {
        let encoded = encode_to_vec(&val).expect("encode char failed");
        let (decoded, _) = decode_from_slice::<char>(&encoded).expect("decode char failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_point3d_roundtrip() {
    proptest!(|(p in arb_point3d())| {
        let encoded = encode_to_vec(&p).expect("encode Point3D failed");
        let (decoded, _) = decode_from_slice::<Point3D>(&encoded).expect("decode Point3D failed");
        prop_assert_eq!(p.x.to_bits(), decoded.x.to_bits());
        prop_assert_eq!(p.y.to_bits(), decoded.y.to_bits());
        prop_assert_eq!(p.z.to_bits(), decoded.z.to_bits());
    });
}

#[test]
fn test_shape_circle_roundtrip() {
    proptest!(|(radius: f32)| {
        let shape = Shape::Circle { radius };
        let encoded = encode_to_vec(&shape).expect("encode Shape::Circle failed");
        let (decoded, _) = decode_from_slice::<Shape>(&encoded).expect("decode Shape::Circle failed");
        match decoded {
            Shape::Circle { radius: r } => prop_assert_eq!(radius.to_bits(), r.to_bits()),
            _ => return Err(TestCaseError::fail("expected Circle")),
        }
    });
}

#[test]
fn test_shape_rectangle_roundtrip() {
    proptest!(|(width: f32, height: f32)| {
        let shape = Shape::Rectangle { width, height };
        let encoded = encode_to_vec(&shape).expect("encode Shape::Rectangle failed");
        let (decoded, _) = decode_from_slice::<Shape>(&encoded).expect("decode Shape::Rectangle failed");
        match decoded {
            Shape::Rectangle { width: w, height: h } => {
                prop_assert_eq!(width.to_bits(), w.to_bits());
                prop_assert_eq!(height.to_bits(), h.to_bits());
            },
            _ => return Err(TestCaseError::fail("expected Rectangle")),
        }
    });
}

#[test]
fn test_shape_point_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let shape = Shape::Point;
        let encoded = encode_to_vec(&shape).expect("encode Shape::Point failed");
        let (decoded, _) = decode_from_slice::<Shape>(&encoded).expect("decode Shape::Point failed");
        prop_assert_eq!(Shape::Point, decoded);
    });
}

#[test]
fn test_vec_shape_roundtrip() {
    proptest!(|(shapes in proptest::collection::vec(arb_shape(), 0..5))| {
        let encoded = encode_to_vec(&shapes).expect("encode Vec<Shape> failed");
        let (decoded, _) = decode_from_slice::<Vec<Shape>>(&encoded).expect("decode Vec<Shape> failed");
        prop_assert_eq!(shapes.len(), decoded.len());
    });
}

#[test]
fn test_hashmap_u32_u32_roundtrip() {
    proptest!(|(map in proptest::collection::hash_map(any::<u32>(), any::<u32>(), 0..10))| {
        let encoded = encode_to_vec(&map).expect("encode HashMap<u32,u32> failed");
        let (decoded, _) = decode_from_slice::<HashMap<u32, u32>>(&encoded).expect("decode HashMap<u32,u32> failed");
        prop_assert_eq!(map, decoded);
    });
}

#[test]
fn test_fixed_array_u8_8_roundtrip() {
    proptest!(|(arr: [u8; 8])| {
        let encoded = encode_to_vec(&arr).expect("encode [u8;8] failed");
        let (decoded, _) = decode_from_slice::<[u8; 8]>(&encoded).expect("decode [u8;8] failed");
        prop_assert_eq!(arr, decoded);
    });
}

#[test]
fn test_fixed_array_u32_4_roundtrip() {
    proptest!(|(arr: [u32; 4])| {
        let encoded = encode_to_vec(&arr).expect("encode [u32;4] failed");
        let (decoded, _) = decode_from_slice::<[u32; 4]>(&encoded).expect("decode [u32;4] failed");
        prop_assert_eq!(arr, decoded);
    });
}

#[test]
fn test_point3d_reencode_gives_same_bytes() {
    proptest!(|(p in arb_point3d())| {
        let encoded1 = encode_to_vec(&p).expect("first encode Point3D failed");
        let (decoded, _) = decode_from_slice::<Point3D>(&encoded1).expect("decode Point3D for reencode failed");
        let encoded2 = encode_to_vec(&decoded).expect("second encode Point3D failed");
        prop_assert_eq!(encoded1, encoded2);
    });
}

#[test]
fn test_option_shape_some_roundtrip() {
    proptest!(|(shape in arb_shape())| {
        let opt: Option<Shape> = Some(shape);
        let encoded = encode_to_vec(&opt).expect("encode Option<Shape> failed");
        let (decoded, _) = decode_from_slice::<Option<Shape>>(&encoded).expect("decode Option<Shape> failed");
        prop_assert!(decoded.is_some());
    });
}

#[test]
fn test_tuple_point3d_pair_roundtrip() {
    proptest!(|(a in arb_point3d(), b in arb_point3d())| {
        let pair = (a.clone(), b.clone());
        let encoded = encode_to_vec(&pair).expect("encode (Point3D, Point3D) failed");
        let (decoded, _) = decode_from_slice::<(Point3D, Point3D)>(&encoded).expect("decode (Point3D, Point3D) failed");
        prop_assert_eq!(a.x.to_bits(), decoded.0.x.to_bits());
        prop_assert_eq!(b.z.to_bits(), decoded.1.z.to_bits());
    });
}

#[test]
fn test_result_ok_point3d_roundtrip() {
    proptest!(|(p in arb_point3d())| {
        let result: Result<Point3D, String> = Ok(p.clone());
        let encoded = encode_to_vec(&result).expect("encode Result<Point3D,String> failed");
        let (decoded, _) = decode_from_slice::<Result<Point3D, String>>(&encoded).expect("decode Result<Point3D,String> failed");
        match decoded {
            Ok(dp) => {
                prop_assert_eq!(p.x.to_bits(), dp.x.to_bits());
                prop_assert_eq!(p.y.to_bits(), dp.y.to_bits());
                prop_assert_eq!(p.z.to_bits(), dp.z.to_bits());
            },
            Err(_) => return Err(TestCaseError::fail("expected Ok")),
        }
    });
}

#[test]
fn test_vec_u32_pairs_roundtrip() {
    proptest!(|(pairs in proptest::collection::vec((any::<u32>(), any::<u32>()), 0..10))| {
        let encoded = encode_to_vec(&pairs).expect("encode Vec<(u32,u32)> failed");
        let (decoded, _) = decode_from_slice::<Vec<(u32, u32)>>(&encoded).expect("decode Vec<(u32,u32)> failed");
        prop_assert_eq!(pairs, decoded);
    });
}

#[test]
fn test_btreemap_u32_point3d_roundtrip() {
    proptest!(|(entries in proptest::collection::vec((any::<u32>(), arb_point3d()), 0..5))| {
        let map: BTreeMap<u32, Point3D> = entries.into_iter().collect();
        let encoded = encode_to_vec(&map).expect("encode BTreeMap<u32,Point3D> failed");
        let (decoded, _) = decode_from_slice::<BTreeMap<u32, Point3D>>(&encoded).expect("decode BTreeMap<u32,Point3D> failed");
        prop_assert_eq!(map.len(), decoded.len());
        for (k, v) in &map {
            let dv = decoded.get(k).expect("key missing in decoded BTreeMap");
            prop_assert_eq!(v.x.to_bits(), dv.x.to_bits());
            prop_assert_eq!(v.y.to_bits(), dv.y.to_bits());
            prop_assert_eq!(v.z.to_bits(), dv.z.to_bits());
        }
    });
}

#[test]
fn test_consumed_bytes_equals_encoded_length_point3d() {
    proptest!(|(p in arb_point3d())| {
        let encoded = encode_to_vec(&p).expect("encode Point3D for consumed bytes test failed");
        let expected_len = encoded.len();
        let (_, consumed) = decode_from_slice::<Point3D>(&encoded).expect("decode Point3D for consumed bytes test failed");
        prop_assert_eq!(expected_len, consumed);
    });
}
