//! Advanced tests for enum discriminant encoding, roundtrip correctness,
//! tag_type attributes, large enums, and composite types.

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
enum Status {
    Active,
    Inactive,
    Pending,
    Suspended,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Variant {
    A(u32),
    B(String),
    C { x: i32, y: i32 },
    D,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum SmallTag {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BigEnum {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
}

// Helper: encode with fixed-int (legacy) config
fn encode_fixed<T: Encode>(val: &T) -> Vec<u8> {
    encode_to_vec_with_config(val, config::legacy()).expect("encode_fixed failed")
}

// Helper: decode with fixed-int (legacy) config
fn decode_fixed<T: Decode>(bytes: &[u8]) -> T {
    let (val, _) =
        decode_from_slice_with_config(bytes, config::legacy()).expect("decode_fixed failed");
    val
}

// 1. Status::Active discriminant byte = 0
#[test]
fn test_status_active_discriminant_zero() {
    let enc = encode_fixed(&Status::Active);
    // Fixed-int u32 discriminant → first 4 bytes represent 0u32 little-endian
    assert_eq!(
        enc[0], 0,
        "Status::Active discriminant first byte should be 0, got: {:?}",
        enc
    );
}

// 2. Status::Inactive discriminant byte = 1
#[test]
fn test_status_inactive_discriminant_one() {
    let enc = encode_fixed(&Status::Inactive);
    assert_eq!(
        enc[0], 1,
        "Status::Inactive discriminant first byte should be 1, got: {:?}",
        enc
    );
}

// 3. Status::Pending discriminant byte = 2
#[test]
fn test_status_pending_discriminant_two() {
    let enc = encode_fixed(&Status::Pending);
    assert_eq!(
        enc[0], 2,
        "Status::Pending discriminant first byte should be 2, got: {:?}",
        enc
    );
}

// 4. Status::Suspended discriminant byte = 3
#[test]
fn test_status_suspended_discriminant_three() {
    let enc = encode_fixed(&Status::Suspended);
    assert_eq!(
        enc[0], 3,
        "Status::Suspended discriminant first byte should be 3, got: {:?}",
        enc
    );
}

// 5. All 4 Status variants roundtrip
#[test]
fn test_status_all_variants_roundtrip() {
    let variants = [
        Status::Active,
        Status::Inactive,
        Status::Pending,
        Status::Suspended,
    ];
    for val in variants {
        let enc = encode_to_vec(&val).expect("encode Status failed");
        let (decoded, _): (Status, usize) = decode_from_slice(&enc).expect("decode Status failed");
        assert_eq!(val, decoded);
    }
}

// 6. Variant::D (unit) discriminant byte = 3
#[test]
fn test_variant_d_discriminant_three() {
    let enc = encode_fixed(&Variant::D);
    assert_eq!(
        enc[0], 3,
        "Variant::D discriminant first byte should be 3, got: {:?}",
        enc
    );
}

// 7. Variant::A(42) roundtrip
#[test]
fn test_variant_a_roundtrip() {
    let val = Variant::A(42);
    let enc = encode_to_vec(&val).expect("encode Variant::A failed");
    let (decoded, _): (Variant, usize) = decode_from_slice(&enc).expect("decode Variant::A failed");
    assert_eq!(val, decoded);
}

// 8. Variant::B("hi".into()) roundtrip
#[test]
fn test_variant_b_roundtrip() {
    let val = Variant::B("hi".into());
    let enc = encode_to_vec(&val).expect("encode Variant::B failed");
    let (decoded, _): (Variant, usize) = decode_from_slice(&enc).expect("decode Variant::B failed");
    assert_eq!(val, decoded);
}

// 9. Variant::C { x: 1, y: 2 } roundtrip
#[test]
fn test_variant_c_roundtrip() {
    let val = Variant::C { x: 1, y: 2 };
    let enc = encode_to_vec(&val).expect("encode Variant::C failed");
    let (decoded, _): (Variant, usize) = decode_from_slice(&enc).expect("decode Variant::C failed");
    assert_eq!(val, decoded);
}

// 10. All 4 Variant variants have distinct first bytes
#[test]
fn test_variant_all_distinct_first_bytes() {
    let a_enc = encode_fixed(&Variant::A(0));
    let b_enc = encode_fixed(&Variant::B(String::new()));
    let c_enc = encode_fixed(&Variant::C { x: 0, y: 0 });
    let d_enc = encode_fixed(&Variant::D);

    let first_bytes = [a_enc[0], b_enc[0], c_enc[0], d_enc[0]];
    // All four discriminant first bytes must be distinct
    for i in 0..first_bytes.len() {
        for j in (i + 1)..first_bytes.len() {
            assert_ne!(
                first_bytes[i], first_bytes[j],
                "Variant discriminants at index {} and {} must differ",
                i, j
            );
        }
    }
}

// 11. SmallTag::X with tag_type=u8 roundtrip
#[test]
fn test_small_tag_x_roundtrip() {
    let val = SmallTag::X;
    let enc = encode_to_vec(&val).expect("encode SmallTag::X failed");
    let (decoded, _): (SmallTag, usize) =
        decode_from_slice(&enc).expect("decode SmallTag::X failed");
    assert_eq!(val, decoded);
}

// 12. SmallTag::Y with tag_type=u8 roundtrip
#[test]
fn test_small_tag_y_roundtrip() {
    let val = SmallTag::Y;
    let enc = encode_to_vec(&val).expect("encode SmallTag::Y failed");
    let (decoded, _): (SmallTag, usize) =
        decode_from_slice(&enc).expect("decode SmallTag::Y failed");
    assert_eq!(val, decoded);
}

// 13. SmallTag variants with u8 tag each encode to exactly 1 byte (unit variants, fixed-int)
#[test]
fn test_small_tag_unit_variants_one_byte_fixed() {
    let x_enc = encode_fixed(&SmallTag::X);
    let y_enc = encode_fixed(&SmallTag::Y);
    let z_enc = encode_fixed(&SmallTag::Z);

    assert_eq!(
        x_enc.len(),
        1,
        "SmallTag::X (u8 tag, fixed-int) should be 1 byte, got: {:?}",
        x_enc
    );
    assert_eq!(
        y_enc.len(),
        1,
        "SmallTag::Y (u8 tag, fixed-int) should be 1 byte, got: {:?}",
        y_enc
    );
    assert_eq!(
        z_enc.len(),
        1,
        "SmallTag::Z (u8 tag, fixed-int) should be 1 byte, got: {:?}",
        z_enc
    );
}

// 14. BigEnum::V0 roundtrip, discriminant 0
#[test]
fn test_big_enum_v0_roundtrip_discriminant() {
    let val = BigEnum::V0;
    let enc = encode_to_vec(&val).expect("encode BigEnum::V0 failed");
    let (decoded, _): (BigEnum, usize) =
        decode_from_slice(&enc).expect("decode BigEnum::V0 failed");
    assert_eq!(val, decoded);

    let enc_fixed = encode_fixed(&BigEnum::V0);
    assert_eq!(
        enc_fixed[0], 0,
        "BigEnum::V0 first discriminant byte should be 0"
    );
}

// 15. BigEnum::V19 roundtrip, discriminant 19
#[test]
fn test_big_enum_v19_roundtrip_discriminant() {
    let val = BigEnum::V19;
    let enc = encode_to_vec(&val).expect("encode BigEnum::V19 failed");
    let (decoded, _): (BigEnum, usize) =
        decode_from_slice(&enc).expect("decode BigEnum::V19 failed");
    assert_eq!(val, decoded);

    let enc_fixed = encode_fixed(&BigEnum::V19);
    assert_eq!(
        enc_fixed[0], 19,
        "BigEnum::V19 first discriminant byte should be 19"
    );
}

// 16. BigEnum::V10 discriminant byte = 10
#[test]
fn test_big_enum_v10_discriminant_ten() {
    let enc = encode_fixed(&BigEnum::V10);
    assert_eq!(
        enc[0], 10,
        "BigEnum::V10 discriminant first byte should be 10, got: {:?}",
        enc
    );
}

// 17. Vec<Status> with all 4 variants roundtrip
#[test]
fn test_vec_status_all_variants_roundtrip() {
    let vals: Vec<Status> = vec![
        Status::Active,
        Status::Inactive,
        Status::Pending,
        Status::Suspended,
    ];
    let enc = encode_to_vec(&vals).expect("encode Vec<Status> failed");
    let (decoded, _): (Vec<Status>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Status> failed");
    assert_eq!(vals, decoded);
}

// 18. Vec<BigEnum> with first 5 variants roundtrip
#[test]
fn test_vec_big_enum_first_five_roundtrip() {
    let vals: Vec<BigEnum> = vec![
        BigEnum::V0,
        BigEnum::V1,
        BigEnum::V2,
        BigEnum::V3,
        BigEnum::V4,
    ];
    let enc = encode_to_vec(&vals).expect("encode Vec<BigEnum> failed");
    let (decoded, _): (Vec<BigEnum>, usize) =
        decode_from_slice(&enc).expect("decode Vec<BigEnum> failed");
    assert_eq!(vals, decoded);
}

// 19. Option<Status> Some roundtrip
#[test]
fn test_option_status_some_roundtrip() {
    let val: Option<Status> = Some(Status::Pending);
    let enc = encode_to_vec(&val).expect("encode Option<Status> Some failed");
    let (decoded, _): (Option<Status>, usize) =
        decode_from_slice(&enc).expect("decode Option<Status> Some failed");
    assert_eq!(val, decoded);
}

// 20. Option<Variant> None roundtrip
#[test]
fn test_option_variant_none_roundtrip() {
    let val: Option<Variant> = None;
    let enc = encode_to_vec(&val).expect("encode Option<Variant> None failed");
    let (decoded, _): (Option<Variant>, usize) =
        decode_from_slice(&enc).expect("decode Option<Variant> None failed");
    assert_eq!(val, decoded);
}

// 21. Fixed-int config with Variant::A(u32::MAX) roundtrip
#[test]
fn test_variant_a_u32_max_fixed_int_roundtrip() {
    let val = Variant::A(u32::MAX);
    let enc = encode_fixed(&val);
    let decoded: Variant = decode_fixed(&enc);
    assert_eq!(val, decoded);
}

// 22. Consumed bytes equals encoded length for Variant::B
#[test]
fn test_variant_b_consumed_bytes_equals_encoded_length() {
    let val = Variant::B("oxicode".into());
    let enc = encode_to_vec(&val).expect("encode Variant::B for size check failed");
    let (decoded, consumed): (Variant, usize) =
        decode_from_slice(&enc).expect("decode Variant::B for size check failed");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes ({}) should equal encoded length ({})",
        consumed,
        enc.len()
    );
}
