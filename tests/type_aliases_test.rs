//! Tests using type aliases and newtypes.

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

// Type alias
type UserId = u64;
type Score = f64;
type Tags = Vec<String>;

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserScore {
    user_id: UserId,
    score: Score,
    tags: Tags,
}

#[test]
fn test_type_aliases_roundtrip() {
    let v = UserScore {
        user_id: 12345,
        score: 99.5,
        tags: vec!["fast".to_string(), "safe".to_string()],
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (UserScore, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// Newtype pattern
#[derive(Debug, PartialEq, Encode, Decode)]
struct Meters(f64);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Kilograms(f64);

#[derive(Debug, PartialEq, Encode, Decode)]
struct PhysicsObject {
    position: Meters,
    mass: Kilograms,
    name: String,
}

#[test]
fn test_newtypes_roundtrip() {
    let obj = PhysicsObject {
        position: Meters(3.5),
        mass: Kilograms(72.5),
        name: "particle".to_string(),
    };
    let enc = encode_to_vec(&obj).expect("encode");
    let (dec, _): (PhysicsObject, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(obj, dec);
}

// Newtype same encoding as inner type
#[test]
fn test_newtype_same_encoding_as_inner() {
    let meters = Meters(42.0);
    let raw: f64 = 42.0;

    let enc_newtype = encode_to_vec(&meters).expect("encode newtype");
    let enc_raw = encode_to_vec(&raw).expect("encode raw");
    assert_eq!(
        enc_newtype, enc_raw,
        "newtype should encode identically to inner"
    );
}
