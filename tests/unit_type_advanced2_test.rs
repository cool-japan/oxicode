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
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Encode, Decode)]
struct Unit;

#[derive(Debug, PartialEq, Encode, Decode)]
struct UnitStruct {}

#[derive(Debug, PartialEq, Encode, Decode)]
enum UnitEnum {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MixedEnum {
    Unit,
    Data(u32),
    Pair(u8, u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Phantom<T>(PhantomData<T>);

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithUnit {
    name: u32,
    tag: Unit,
    value: u64,
}

// Test 1: Unit encodes to exactly 0 bytes
#[test]
fn test_unit_encodes_to_zero_bytes() {
    let enc = encode_to_vec(&Unit).expect("encode Unit");
    assert_eq!(enc.len(), 0, "Unit should encode to 0 bytes");
}

// Test 2: UnitStruct {} encodes to exactly 0 bytes
#[test]
fn test_unit_struct_encodes_to_zero_bytes() {
    let enc = encode_to_vec(&UnitStruct {}).expect("encode UnitStruct");
    assert_eq!(enc.len(), 0, "UnitStruct should encode to 0 bytes");
}

// Test 3: () (unit tuple) encodes to exactly 0 bytes
#[test]
fn test_unit_tuple_encodes_to_zero_bytes() {
    let enc = encode_to_vec(&()).expect("encode unit tuple");
    assert_eq!(enc.len(), 0, "() should encode to 0 bytes");
}

// Test 4: Unit roundtrip
#[test]
fn test_unit_roundtrip() {
    let enc = encode_to_vec(&Unit).expect("encode Unit");
    let (val, _) = decode_from_slice::<Unit>(&enc).expect("decode Unit");
    assert_eq!(val, Unit);
}

// Test 5: UnitStruct {} roundtrip
#[test]
fn test_unit_struct_roundtrip() {
    let enc = encode_to_vec(&UnitStruct {}).expect("encode UnitStruct");
    let (val, _) = decode_from_slice::<UnitStruct>(&enc).expect("decode UnitStruct");
    assert_eq!(val, UnitStruct {});
}

// Test 6: () roundtrip
#[test]
fn test_unit_tuple_roundtrip() {
    let enc = encode_to_vec(&()).expect("encode unit tuple");
    let (val, _) = decode_from_slice::<()>(&enc).expect("decode unit tuple");
    assert_eq!(val, ());
}

// Test 7: UnitEnum::A roundtrip, discriminant byte = 0
#[test]
fn test_unit_enum_a_roundtrip_discriminant_zero() {
    let enc = encode_to_vec(&UnitEnum::A).expect("encode UnitEnum::A");
    assert_eq!(enc[0], 0, "UnitEnum::A discriminant should be 0");
    let (val, _) = decode_from_slice::<UnitEnum>(&enc).expect("decode UnitEnum::A");
    assert_eq!(val, UnitEnum::A);
}

// Test 8: UnitEnum::B roundtrip, discriminant byte = 1
#[test]
fn test_unit_enum_b_roundtrip_discriminant_one() {
    let enc = encode_to_vec(&UnitEnum::B).expect("encode UnitEnum::B");
    assert_eq!(enc[0], 1, "UnitEnum::B discriminant should be 1");
    let (val, _) = decode_from_slice::<UnitEnum>(&enc).expect("decode UnitEnum::B");
    assert_eq!(val, UnitEnum::B);
}

// Test 9: UnitEnum::C roundtrip, discriminant byte = 2
#[test]
fn test_unit_enum_c_roundtrip_discriminant_two() {
    let enc = encode_to_vec(&UnitEnum::C).expect("encode UnitEnum::C");
    assert_eq!(enc[0], 2, "UnitEnum::C discriminant should be 2");
    let (val, _) = decode_from_slice::<UnitEnum>(&enc).expect("decode UnitEnum::C");
    assert_eq!(val, UnitEnum::C);
}

// Test 10: All 3 unit enum variants have distinct encodings
#[test]
fn test_unit_enum_variants_distinct_encodings() {
    let enc_a = encode_to_vec(&UnitEnum::A).expect("encode UnitEnum::A");
    let enc_b = encode_to_vec(&UnitEnum::B).expect("encode UnitEnum::B");
    let enc_c = encode_to_vec(&UnitEnum::C).expect("encode UnitEnum::C");
    assert_ne!(
        enc_a, enc_b,
        "UnitEnum::A and B should have distinct encodings"
    );
    assert_ne!(
        enc_b, enc_c,
        "UnitEnum::B and C should have distinct encodings"
    );
    assert_ne!(
        enc_a, enc_c,
        "UnitEnum::A and C should have distinct encodings"
    );
}

// Test 11: MixedEnum::Unit roundtrip
#[test]
fn test_mixed_enum_unit_roundtrip() {
    let enc = encode_to_vec(&MixedEnum::Unit).expect("encode MixedEnum::Unit");
    let (val, _) = decode_from_slice::<MixedEnum>(&enc).expect("decode MixedEnum::Unit");
    assert_eq!(val, MixedEnum::Unit);
}

// Test 12: MixedEnum::Data(42) roundtrip
#[test]
fn test_mixed_enum_data_roundtrip() {
    let enc = encode_to_vec(&MixedEnum::Data(42)).expect("encode MixedEnum::Data(42)");
    let (val, _) = decode_from_slice::<MixedEnum>(&enc).expect("decode MixedEnum::Data(42)");
    assert_eq!(val, MixedEnum::Data(42));
}

// Test 13: MixedEnum::Pair(1, 2) roundtrip
#[test]
fn test_mixed_enum_pair_roundtrip() {
    let enc = encode_to_vec(&MixedEnum::Pair(1, 2)).expect("encode MixedEnum::Pair(1, 2)");
    let (val, _) = decode_from_slice::<MixedEnum>(&enc).expect("decode MixedEnum::Pair(1, 2)");
    assert_eq!(val, MixedEnum::Pair(1, 2));
}

// Test 14: Vec<UnitEnum> with all 3 variants roundtrip
#[test]
fn test_vec_unit_enum_all_variants_roundtrip() {
    let original: Vec<UnitEnum> = vec![UnitEnum::A, UnitEnum::B, UnitEnum::C];
    let enc = encode_to_vec(&original).expect("encode Vec<UnitEnum>");
    let (val, _) = decode_from_slice::<Vec<UnitEnum>>(&enc).expect("decode Vec<UnitEnum>");
    assert_eq!(val, original);
}

// Test 15: Option<Unit> Some roundtrip
#[test]
fn test_option_unit_some_roundtrip() {
    let original: Option<Unit> = Some(Unit);
    let enc = encode_to_vec(&original).expect("encode Option<Unit> Some");
    let (val, _) = decode_from_slice::<Option<Unit>>(&enc).expect("decode Option<Unit> Some");
    assert_eq!(val, Some(Unit));
}

// Test 16: Option<Unit> None roundtrip
#[test]
fn test_option_unit_none_roundtrip() {
    let original: Option<Unit> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Unit> None");
    let (val, _) = decode_from_slice::<Option<Unit>>(&enc).expect("decode Option<Unit> None");
    assert_eq!(val, None);
}

// Test 17: Phantom::<u32>(PhantomData) encodes to 0 bytes
#[test]
fn test_phantom_u32_encodes_to_zero_bytes() {
    let phantom: Phantom<u32> = Phantom(PhantomData);
    let enc = encode_to_vec(&phantom).expect("encode Phantom<u32>");
    assert_eq!(enc.len(), 0, "Phantom<u32> should encode to 0 bytes");
}

// Test 18: Phantom::<String>(PhantomData) roundtrip
#[test]
fn test_phantom_string_roundtrip() {
    let original: Phantom<String> = Phantom(PhantomData);
    let enc = encode_to_vec(&original).expect("encode Phantom<String>");
    let (val, _) = decode_from_slice::<Phantom<String>>(&enc).expect("decode Phantom<String>");
    assert_eq!(val, original);
}

// Test 19: (Unit, Unit) tuple roundtrip — still 0 bytes
#[test]
fn test_unit_unit_tuple_zero_bytes_roundtrip() {
    let original = (Unit, Unit);
    let enc = encode_to_vec(&original).expect("encode (Unit, Unit)");
    assert_eq!(enc.len(), 0, "(Unit, Unit) should encode to 0 bytes");
    let (val, _) = decode_from_slice::<(Unit, Unit)>(&enc).expect("decode (Unit, Unit)");
    assert_eq!(val, original);
}

// Test 20: Struct containing a Unit field roundtrip (only non-unit fields affect size)
#[test]
fn test_struct_with_unit_field_roundtrip() {
    let original = WithUnit {
        name: 100,
        tag: Unit,
        value: 999,
    };
    let enc = encode_to_vec(&original).expect("encode WithUnit");
    // The unit field contributes 0 bytes; name (u32) and value (u64) contribute bytes
    let unit_enc = encode_to_vec(&Unit).expect("encode standalone Unit");
    let u32_enc = encode_to_vec(&100u32).expect("encode u32");
    let u64_enc = encode_to_vec(&999u64).expect("encode u64");
    assert_eq!(
        enc.len(),
        unit_enc.len() + u32_enc.len() + u64_enc.len(),
        "Struct encoding size should match sum of field encodings"
    );
    let (val, _) = decode_from_slice::<WithUnit>(&enc).expect("decode WithUnit");
    assert_eq!(val, original);
}

// Test 21: Vec<Unit> with 3 elements — encodes to just the length varint
#[test]
fn test_vec_unit_encodes_to_length_varint_only() {
    let original: Vec<Unit> = vec![Unit, Unit, Unit];
    let enc = encode_to_vec(&original).expect("encode Vec<Unit>");
    // Vec length 3 fits in 1 byte varint
    assert_eq!(
        enc.len(),
        1,
        "Vec<Unit> with 3 elements should encode to 1 byte (varint 3)"
    );
    assert_eq!(enc[0], 3, "The single byte should be the varint for 3");
}

// Test 22: Unit consumed == 0 bytes
#[test]
fn test_unit_consumed_zero_bytes() {
    let enc = encode_to_vec(&Unit).expect("encode Unit");
    let (_, consumed) = decode_from_slice::<Unit>(&enc).expect("decode Unit");
    assert_eq!(consumed, 0, "Decoding Unit should consume 0 bytes");
}
