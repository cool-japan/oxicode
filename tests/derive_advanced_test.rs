//! Advanced derive macro tests for OxiCode — 22 comprehensive scenarios.
//!
//! Each test targets a specific derive scenario that is NOT already covered by
//! derive_test.rs, derive_combinations_test.rs, or derive_edge_cases_test.rs.
//!
//! Scenarios:
//!  1.  Struct with all primitive field types (u8..u64, i8..i64, bool, f32, f64, char)
//!  2.  Struct with String, Vec<u8>, Option<String> fields
//!  3.  Enum with multiple data variants (unit, tuple, struct)
//!  4.  Enum with large discriminant values (#[oxicode(variant)] = 100, 200, 255)
//!  5.  Generic struct Pair<T: Encode + Decode> — two concrete instantiations
//!  6.  Generic enum Container<T: Encode + Decode>
//!  7.  Struct with BTreeMap<String, u64> field
//!  8.  Struct with Vec<String> field
//!  9.  Nested struct: outer containing inner (top-level definitions)
//! 10.  Enum where all variants share the same field name & type
//! 11.  Struct with #[oxicode(skip)] — field must be Default on decode
//! 12.  Struct with #[oxicode(default = "fn_name")] — custom default function
//! 13.  Tuple struct with single signed-integer field (edge values)
//! 14.  Unit struct produces exactly 0 encoded bytes
//! 15.  Struct with usize and isize fields
//! 16.  Enum that derives both Encode and BorrowDecode (with lifetime)
//! 17.  Struct that derives BorrowDecode with two borrowed fields
//! 18.  Multiple structs referencing each other (non-recursive composition)
//! 19.  Struct with PhantomData field via #[oxicode(skip)]
//! 20.  Enum with C-like explicit discriminants (default u32 tag_type)
//! 21.  Struct with array field [u32; 8]
//! 22.  Deeply nested Option<Vec<Option<String>>>

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
use oxicode::{decode_from_slice, encode_to_vec, BorrowDecode, Decode, Encode};
use std::collections::BTreeMap;
use std::marker::PhantomData;

// ── 1: Struct with all primitive field types ─────────────────────────────────
// (derive_edge_cases_test::LargeStruct also includes String/Vec/Option — this
//  struct is purely primitives and tests each independently.)

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllPrimitives {
    a_u8: u8,
    b_u16: u16,
    c_u32: u32,
    d_u64: u64,
    e_i8: i8,
    f_i16: i16,
    g_i32: i32,
    h_i64: i64,
    i_bool: bool,
    j_f32: f32,
    k_f64: f64,
    l_char: char,
}

#[test]
fn test_all_primitive_fields_roundtrip() {
    let original = AllPrimitives {
        a_u8: 0xAB,
        b_u16: 0xABCD,
        c_u32: 0xDEAD_BEEF,
        d_u64: 0xCAFE_BABE_DEAD_C0DE,
        e_i8: -100,
        f_i16: -30_000,
        g_i32: -2_000_000_000,
        h_i64: i64::MIN,
        i_bool: true,
        j_f32: std::f32::consts::PI,
        k_f64: std::f64::consts::TAU,
        l_char: '\u{1F600}',
    };
    let enc = encode_to_vec(&original).expect("encode AllPrimitives");
    let (dec, _): (AllPrimitives, _) = decode_from_slice(&enc).expect("decode AllPrimitives");
    assert_eq!(original, dec);
}

#[test]
fn test_all_primitive_fields_boundary_values() {
    let original = AllPrimitives {
        a_u8: u8::MAX,
        b_u16: u16::MAX,
        c_u32: u32::MAX,
        d_u64: u64::MAX,
        e_i8: i8::MAX,
        f_i16: i16::MAX,
        g_i32: i32::MAX,
        h_i64: i64::MAX,
        i_bool: false,
        j_f32: f32::MAX,
        k_f64: f64::MAX,
        l_char: char::MAX,
    };
    let enc = encode_to_vec(&original).expect("encode AllPrimitives boundary");
    let (dec, _): (AllPrimitives, _) =
        decode_from_slice(&enc).expect("decode AllPrimitives boundary");
    assert_eq!(original, dec);
}

// ── 2: Struct with String, Vec<u8>, Option<String> fields ────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct StringVecOpt {
    label: String,
    payload: Vec<u8>,
    tag: Option<String>,
}

#[test]
fn test_string_vec_opt_some_roundtrip() {
    let original = StringVecOpt {
        label: "hello oxicode".to_string(),
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        tag: Some("present".to_string()),
    };
    let enc = encode_to_vec(&original).expect("encode StringVecOpt Some");
    let (dec, _): (StringVecOpt, _) = decode_from_slice(&enc).expect("decode StringVecOpt Some");
    assert_eq!(original, dec);
}

#[test]
fn test_string_vec_opt_none_roundtrip() {
    let original = StringVecOpt {
        label: String::new(),
        payload: vec![],
        tag: None,
    };
    let enc = encode_to_vec(&original).expect("encode StringVecOpt None");
    let (dec, _): (StringVecOpt, _) = decode_from_slice(&enc).expect("decode StringVecOpt None");
    assert_eq!(original, dec);
}

// ── 3: Enum with multiple data variants (unit, tuple, struct) ─────────────────
// (distinct from derive_test::Message: richer payloads with i64 and nested tuple)

#[derive(Debug, PartialEq, Encode, Decode)]
enum DataVariants {
    Empty,
    Single(i64),
    Pair(u32, u32),
    Record { name: String, value: i64 },
}

#[test]
fn test_enum_multi_data_variants_all_cases() {
    let cases = [
        DataVariants::Empty,
        DataVariants::Single(i64::MIN),
        DataVariants::Pair(0xDEAD, 0xBEEF),
        DataVariants::Record {
            name: "oxicode_field".to_string(),
            value: 42_000_000_000,
        },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode DataVariants");
        let (dec, _): (DataVariants, _) = decode_from_slice(&enc).expect("decode DataVariants");
        assert_eq!(case, &dec);
    }
}

// ── 4: Enum with large discriminant values (100, 200, 255) ───────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum LargeDiscriminants {
    #[oxicode(variant = 100)]
    Alpha,
    #[oxicode(variant = 200)]
    Beta(u64),
    #[oxicode(variant = 255)]
    Gamma { x: i32, y: i32 },
}

#[test]
fn test_enum_large_discriminants_roundtrip() {
    let cases = [
        LargeDiscriminants::Alpha,
        LargeDiscriminants::Beta(u64::MAX),
        LargeDiscriminants::Gamma { x: -1, y: 1 },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode LargeDiscriminants");
        let (dec, _): (LargeDiscriminants, _) =
            decode_from_slice(&enc).expect("decode LargeDiscriminants");
        assert_eq!(case, &dec);
    }
}

#[test]
fn test_enum_large_discriminant_wire_bytes() {
    // legacy config: u8 tag_type = exactly 1 byte for unit variant
    let enc_alpha =
        oxicode::encode_to_vec_with_config(&LargeDiscriminants::Alpha, oxicode::config::legacy())
            .expect("encode Alpha");
    assert_eq!(enc_alpha.len(), 1, "unit variant: only tag byte");
    assert_eq!(enc_alpha[0], 100u8, "Alpha tag = 100");

    let enc_beta =
        oxicode::encode_to_vec_with_config(&LargeDiscriminants::Beta(0), oxicode::config::legacy())
            .expect("encode Beta");
    assert_eq!(enc_beta[0], 200u8, "Beta tag = 200");

    let enc_gamma = oxicode::encode_to_vec_with_config(
        &LargeDiscriminants::Gamma { x: 0, y: 0 },
        oxicode::config::legacy(),
    )
    .expect("encode Gamma");
    assert_eq!(enc_gamma[0], 255u8, "Gamma tag = 255");
}

// ── 5: Generic struct <T: Encode + Decode> with concrete types ────────────────
// (different from derive_test::Generic<T> which uses {value, count}; here Pair<T>
//  has {left, right} and the bound is written inline on the struct.)

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<T: Encode + Decode> {
    left: T,
    right: T,
}

#[test]
fn test_generic_pair_u64_boundary() {
    let p = Pair {
        left: u64::MIN,
        right: u64::MAX,
    };
    let enc = encode_to_vec(&p).expect("encode Pair<u64>");
    let (dec, _): (Pair<u64>, _) = decode_from_slice(&enc).expect("decode Pair<u64>");
    assert_eq!(p, dec);
}

#[test]
fn test_generic_pair_string() {
    let p = Pair {
        left: "left_value".to_string(),
        right: "right_value".to_string(),
    };
    let enc = encode_to_vec(&p).expect("encode Pair<String>");
    let (dec, _): (Pair<String>, _) = decode_from_slice(&enc).expect("decode Pair<String>");
    assert_eq!(p, dec);
}

// ── 6: Generic enum <T: Encode + Decode> ─────────────────────────────────────
// (distinct from derive_generics_test::Either<L,R> which has two type params)

#[derive(Debug, PartialEq, Encode, Decode)]
enum Container<T: Encode + Decode> {
    Empty,
    One(T),
    Two(T, T),
    Tagged { label: String, item: T },
}

#[test]
fn test_generic_enum_all_variants_u32() {
    let cases: Vec<Container<u32>> = vec![
        Container::Empty,
        Container::One(42),
        Container::Two(0, u32::MAX),
        Container::Tagged {
            label: "item".to_string(),
            item: 999,
        },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode Container<u32>");
        let (dec, _): (Container<u32>, _) = decode_from_slice(&enc).expect("decode Container<u32>");
        assert_eq!(case, &dec);
    }
}

// ── 7: Struct with BTreeMap field ────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithBTreeMap {
    id: u32,
    metadata: BTreeMap<String, u64>,
}

#[test]
fn test_struct_btreemap_populated_roundtrip() {
    let mut metadata = BTreeMap::new();
    metadata.insert("alpha".to_string(), 1_000_000u64);
    metadata.insert("beta".to_string(), 2_000_000u64);
    metadata.insert("gamma".to_string(), 3_000_000u64);
    let original = WithBTreeMap { id: 7, metadata };
    let enc = encode_to_vec(&original).expect("encode WithBTreeMap");
    let (dec, _): (WithBTreeMap, _) = decode_from_slice(&enc).expect("decode WithBTreeMap");
    assert_eq!(original, dec);
}

#[test]
fn test_struct_btreemap_empty_roundtrip() {
    let original = WithBTreeMap {
        id: 0,
        metadata: BTreeMap::new(),
    };
    let enc = encode_to_vec(&original).expect("encode WithBTreeMap empty");
    let (dec, _): (WithBTreeMap, _) = decode_from_slice(&enc).expect("decode WithBTreeMap empty");
    assert_eq!(original, dec);
}

// ── 8: Struct with Vec<String> field ─────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithVecString {
    tags: Vec<String>,
    priority: u8,
}

#[test]
fn test_struct_vec_string_populated_roundtrip() {
    let original = WithVecString {
        tags: vec![
            "rust".to_string(),
            "binary".to_string(),
            "oxicode".to_string(),
        ],
        priority: 5,
    };
    let enc = encode_to_vec(&original).expect("encode WithVecString");
    let (dec, _): (WithVecString, _) = decode_from_slice(&enc).expect("decode WithVecString");
    assert_eq!(original, dec);
}

#[test]
fn test_struct_vec_string_with_empty_elements() {
    let original = WithVecString {
        tags: vec![String::new(), "non-empty".to_string(), String::new()],
        priority: 0,
    };
    let enc = encode_to_vec(&original).expect("encode WithVecString empty elements");
    let (dec, _): (WithVecString, _) =
        decode_from_slice(&enc).expect("decode WithVecString empty elements");
    assert_eq!(original, dec);
}

// ── 9: Nested struct: outer containing inner (top-level definitions) ──────────
// (distinct from derive_test::test_nested_structs which defines structs locally
//  inside a test function — here they are module-level and can be composed.)

#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerRecord {
    x: i32,
    y: i32,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OuterRecord {
    id: u64,
    inner: InnerRecord,
    active: bool,
}

#[test]
fn test_nested_outer_inner_extreme_values() {
    let original = OuterRecord {
        id: u64::MAX,
        inner: InnerRecord {
            x: i32::MIN,
            y: i32::MAX,
            label: "boundary".to_string(),
        },
        active: true,
    };
    let enc = encode_to_vec(&original).expect("encode OuterRecord");
    let (dec, _): (OuterRecord, _) = decode_from_slice(&enc).expect("decode OuterRecord");
    assert_eq!(original, dec);
}

// ── 10: Enum where all variants share the same field name & type ──────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum SameFieldType {
    First { count: u64 },
    Second { count: u64 },
    Third { count: u64 },
}

#[test]
fn test_enum_same_field_type_all_variants() {
    let cases = [
        SameFieldType::First { count: 0 },
        SameFieldType::Second {
            count: u64::MAX / 2,
        },
        SameFieldType::Third { count: u64::MAX },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode SameFieldType");
        let (dec, _): (SameFieldType, _) = decode_from_slice(&enc).expect("decode SameFieldType");
        assert_eq!(case, &dec);
    }
}

// ── 11: Struct with #[oxicode(skip)] — skipped field must be Default ──────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSkip {
    essential: String,
    #[oxicode(skip)]
    transient_cache: Vec<u8>,
    version: u32,
}

#[test]
fn test_skip_field_uses_default_on_decode() {
    let original = WithSkip {
        essential: "important".to_string(),
        transient_cache: vec![1, 2, 3, 4, 5], // must NOT appear on the wire
        version: 7,
    };
    let enc = encode_to_vec(&original).expect("encode WithSkip");
    let (dec, _): (WithSkip, _) = decode_from_slice(&enc).expect("decode WithSkip");

    assert_eq!(dec.essential, "important");
    assert_eq!(dec.version, 7);
    // skipped field is restored as Default::default() = empty Vec
    assert_eq!(dec.transient_cache, Vec::<u8>::new());
}

#[test]
fn test_skip_field_encoding_smaller_than_no_skip() {
    let with_skip = WithSkip {
        essential: "x".to_string(),
        transient_cache: vec![0u8; 100], // 100 bytes NOT written to wire
        version: 1,
    };

    #[derive(Encode)]
    struct NoSkip {
        essential: String,
        transient_cache: Vec<u8>,
        version: u32,
    }
    let no_skip = NoSkip {
        essential: "x".to_string(),
        transient_cache: vec![0u8; 100],
        version: 1,
    };

    let skip_len = encode_to_vec(&with_skip).expect("encode with skip").len();
    let no_skip_len = encode_to_vec(&no_skip).expect("encode no skip").len();
    assert!(
        skip_len < no_skip_len,
        "skipped transient_cache must not appear on wire: {skip_len} vs {no_skip_len}"
    );
}

// ── 12: Struct with #[oxicode(default = "fn_name")] field ────────────────────

fn default_threshold() -> f64 {
    std::f64::consts::PI
}

#[derive(Debug, Encode, Decode)]
struct WithDefaultFn {
    name: String,
    #[oxicode(default = "default_threshold")]
    threshold: f64,
}

#[test]
fn test_default_fn_field_applied_on_decode() {
    let original = WithDefaultFn {
        name: "config_item".to_string(),
        threshold: 9999.0, // encoded value is skipped; fn default is used on decode
    };
    let enc = encode_to_vec(&original).expect("encode WithDefaultFn");
    let (dec, _): (WithDefaultFn, _) = decode_from_slice(&enc).expect("decode WithDefaultFn");

    assert_eq!(dec.name, "config_item");
    // The default_fn overrides the on-wire value
    assert!(
        (dec.threshold - std::f64::consts::PI).abs() < f64::EPSILON,
        "default_fn threshold mismatch: {}",
        dec.threshold
    );
}

// ── 13: Tuple struct with single field — edge values ─────────────────────────
// (derive_edge_cases_test::Wrapper(u64). Here we use i32 and test signed edges.)

#[derive(Debug, PartialEq, Encode, Decode)]
struct SignedWrapper(i32);

#[test]
fn test_single_field_tuple_struct_i32_min() {
    let v = SignedWrapper(i32::MIN);
    let enc = encode_to_vec(&v).expect("encode SignedWrapper MIN");
    let (dec, _): (SignedWrapper, _) = decode_from_slice(&enc).expect("decode SignedWrapper MIN");
    assert_eq!(v, dec);
}

#[test]
fn test_single_field_tuple_struct_i32_max() {
    let v = SignedWrapper(i32::MAX);
    let enc = encode_to_vec(&v).expect("encode SignedWrapper MAX");
    let (dec, _): (SignedWrapper, _) = decode_from_slice(&enc).expect("decode SignedWrapper MAX");
    assert_eq!(v, dec);
}

// ── 14: Unit struct roundtrip & exactly 0 encoded bytes ──────────────────────
// (derive_test::test_unit_struct uses `Unit`. This struct is named `Sentinel`
//  and additionally asserts the byte length is 0.)

#[derive(Debug, PartialEq, Encode, Decode)]
struct Sentinel;

#[test]
fn test_unit_struct_roundtrip_sentinel() {
    let original = Sentinel;
    let enc = encode_to_vec(&original).expect("encode Sentinel");
    let (dec, _): (Sentinel, _) = decode_from_slice(&enc).expect("decode Sentinel");
    assert_eq!(original, dec);
}

#[test]
fn test_unit_struct_produces_zero_payload_bytes() {
    let enc = encode_to_vec(&Sentinel).expect("encode Sentinel zero bytes");
    assert_eq!(enc.len(), 0, "unit struct must encode to exactly 0 bytes");
}

// ── 15: Struct with usize and isize fields ────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SizeFields {
    unsigned_size: usize,
    signed_size: isize,
    label: String,
}

#[test]
fn test_usize_isize_fields_max_min() {
    let original = SizeFields {
        unsigned_size: usize::MAX,
        signed_size: isize::MIN,
        label: "size_extremes".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode SizeFields extremes");
    let (dec, _): (SizeFields, _) = decode_from_slice(&enc).expect("decode SizeFields extremes");
    assert_eq!(original, dec);
}

#[test]
fn test_usize_isize_zero_values() {
    let original = SizeFields {
        unsigned_size: 0,
        signed_size: 0,
        label: String::new(),
    };
    let enc = encode_to_vec(&original).expect("encode SizeFields zero");
    let (dec, _): (SizeFields, _) = decode_from_slice(&enc).expect("decode SizeFields zero");
    assert_eq!(original, dec);
}

// ── 16: Enum that derives both Encode and BorrowDecode (lifetime) ─────────────
// (borrow_decode_derive_test.rs tests Commands<'a> which has borrowed slice/str
//  fields on variants. Here the borrowed variant carries &'a [u8], and we also
//  verify the OwnedVariant and UnitVariant decode correctly via borrow_decode.)

#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
enum OwnedOrBorrowedEnum<'a> {
    OwnedVariant { id: u32, name: String },
    BorrowedVariant { data: &'a [u8] },
    UnitVariant,
}

// Mirror enum used to encode OwnedOrBorrowedEnum variants via owned types
// (avoids lifetime constraints at encode-time).
#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Encode)]
enum OwnedOrBorrowedMirror {
    OwnedVariant {
        id: u32,
        name: String,
    },
    #[allow(dead_code)]
    BorrowedVariant {
        data: Vec<u8>,
    },
    UnitVariant,
}

#[test]
fn test_enum_borrow_decode_owned_variant() {
    let enc = encode_to_vec(&OwnedOrBorrowedMirror::OwnedVariant {
        id: 42,
        name: "owned_name".to_string(),
    })
    .expect("encode OwnedVariant mirror");
    let (dec, _): (OwnedOrBorrowedEnum<'_>, _) =
        oxicode::borrow_decode_from_slice(&enc).expect("borrow_decode OwnedVariant");
    assert_eq!(
        dec,
        OwnedOrBorrowedEnum::OwnedVariant {
            id: 42,
            name: "owned_name".to_string(),
        }
    );
}

#[test]
fn test_enum_borrow_decode_unit_variant() {
    let enc =
        encode_to_vec(&OwnedOrBorrowedMirror::UnitVariant).expect("encode UnitVariant mirror");
    let (dec, _): (OwnedOrBorrowedEnum<'_>, _) =
        oxicode::borrow_decode_from_slice(&enc).expect("borrow_decode UnitVariant");
    assert_eq!(dec, OwnedOrBorrowedEnum::UnitVariant);
}

// ── 17: Struct that derives BorrowDecode with multiple borrowed fields ─────────
// (borrow_decode_derive_test.rs tests single-field structs ZeroCopyBytes and
//  ZeroCopyStr separately. Here MultiBorrow has both fields simultaneously
//  plus an owned integer field.)

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct MultiBorrow<'a> {
    slice_data: &'a [u8],
    str_data: &'a str,
    owned_id: u64,
}

#[test]
fn test_borrow_decode_multiple_borrowed_fields() {
    let enc = {
        #[derive(Encode)]
        struct Mirror {
            slice_data: Vec<u8>,
            str_data: String,
            owned_id: u64,
        }
        encode_to_vec(&Mirror {
            slice_data: vec![0xCA, 0xFE, 0xBA, 0xBE],
            str_data: "borrow_test".to_string(),
            owned_id: 0xDEAD_C0DE_C0DE_CAFE,
        })
        .expect("encode MultiBorrow mirror")
    };

    let (dec, _): (MultiBorrow<'_>, _) =
        oxicode::borrow_decode_from_slice(&enc).expect("borrow_decode MultiBorrow");
    assert_eq!(dec.slice_data, &[0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(dec.str_data, "borrow_test");
    assert_eq!(dec.owned_id, 0xDEAD_C0DE_C0DE_CAFE);
}

// ── 18: Multiple structs referencing each other (non-recursive composition) ────

#[derive(Debug, PartialEq, Encode, Decode)]
struct FrameHeader {
    version: u8,
    flags: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FrameBody {
    payload: Vec<u8>,
    checksum: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Frame {
    header: FrameHeader,
    body: FrameBody,
    sequence: u64,
}

#[test]
fn test_multi_struct_non_recursive_composition() {
    let original = Frame {
        header: FrameHeader {
            version: 2,
            flags: 0b0000_1111_0000_1111,
        },
        body: FrameBody {
            payload: vec![0x01, 0x02, 0x03, 0xFE, 0xFF],
            checksum: 0xCAFE_BABE,
        },
        sequence: 1_000_000_000_000,
    };
    let enc = encode_to_vec(&original).expect("encode Frame");
    let (dec, _): (Frame, _) = decode_from_slice(&enc).expect("decode Frame");
    assert_eq!(original, dec);
}

// ── 19: Struct with PhantomData field via #[oxicode(skip)] ───────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct TypedHandle<T> {
    raw_id: u64,
    #[oxicode(skip)]
    _marker: PhantomData<T>,
}

#[test]
fn test_phantom_data_skip_roundtrip() {
    let original: TypedHandle<u32> = TypedHandle {
        raw_id: 0xFEED_FACE_CAFE_BABE,
        _marker: PhantomData,
    };
    let enc = encode_to_vec(&original).expect("encode TypedHandle<u32>");
    let (dec, _): (TypedHandle<u32>, _) = decode_from_slice(&enc).expect("decode TypedHandle<u32>");
    assert_eq!(original, dec);
}

#[test]
fn test_phantom_data_skip_encoding_identical_across_type_params() {
    // PhantomData is skipped, so TypedHandle<u8> and TypedHandle<String>
    // with the same raw_id must produce byte-identical encodings.
    let handle_u8: TypedHandle<u8> = TypedHandle {
        raw_id: 99,
        _marker: PhantomData,
    };
    let handle_str: TypedHandle<String> = TypedHandle {
        raw_id: 99,
        _marker: PhantomData,
    };
    let enc_u8 = encode_to_vec(&handle_u8).expect("encode TypedHandle<u8>");
    let enc_str = encode_to_vec(&handle_str).expect("encode TypedHandle<String>");
    assert_eq!(
        enc_u8, enc_str,
        "PhantomData is skipped: encodings must be byte-identical"
    );
}

// ── 20: Enum with C-like explicit discriminants (default u32 tag_type) ────────
// (distinct from LargeDiscriminants which uses tag_type="u8"; here the default
//  u32 tag is used with HTTP-inspired numeric codes.)

#[derive(Debug, PartialEq, Encode, Decode)]
enum StatusCode {
    #[oxicode(variant = 200)]
    Ok,
    #[oxicode(variant = 400)]
    BadRequest,
    #[oxicode(variant = 404)]
    NotFound,
    #[oxicode(variant = 500)]
    InternalError,
}

#[test]
fn test_c_like_explicit_discriminants_all_roundtrip() {
    let cases = [
        StatusCode::Ok,
        StatusCode::BadRequest,
        StatusCode::NotFound,
        StatusCode::InternalError,
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode StatusCode");
        let (dec, _): (StatusCode, _) = decode_from_slice(&enc).expect("decode StatusCode");
        assert_eq!(case, &dec);
    }
}

#[test]
fn test_c_like_explicit_discriminant_wire_values() {
    // Default tag_type = u32, legacy config: 4 bytes little-endian
    let enc_ok = oxicode::encode_to_vec_with_config(&StatusCode::Ok, oxicode::config::legacy())
        .expect("encode Ok");
    assert_eq!(enc_ok.len(), 4, "u32 discriminant = 4 bytes");
    assert_eq!(
        u32::from_le_bytes([enc_ok[0], enc_ok[1], enc_ok[2], enc_ok[3]]),
        200u32,
        "Ok discriminant must be 200"
    );

    let enc_nf =
        oxicode::encode_to_vec_with_config(&StatusCode::NotFound, oxicode::config::legacy())
            .expect("encode NotFound");
    assert_eq!(
        u32::from_le_bytes([enc_nf[0], enc_nf[1], enc_nf[2], enc_nf[3]]),
        404u32,
        "NotFound discriminant must be 404"
    );
}

// ── 21: Struct with array field [u32; 8] ──────────────────────────────────────
// (derive_edge_cases does not test [u32;8]; derive_advanced already had [u8;32]
//  and [u64;8]. This targets [u32;8] specifically.)

#[derive(Debug, PartialEq, Encode, Decode)]
struct MatrixRow {
    row_index: usize,
    cells: [u32; 8],
}

#[test]
fn test_fixed_array_u32_8_roundtrip() {
    let original = MatrixRow {
        row_index: 3,
        cells: [0, 1, 2, 3, u32::MAX, u32::MAX - 1, 0xDEAD_BEEF, 0xCAFE_BABE],
    };
    let enc = encode_to_vec(&original).expect("encode MatrixRow");
    let (dec, _): (MatrixRow, _) = decode_from_slice(&enc).expect("decode MatrixRow");
    assert_eq!(original, dec);
}

#[test]
fn test_fixed_array_u32_8_all_zeros_and_max() {
    let zeros = MatrixRow {
        row_index: 0,
        cells: [0u32; 8],
    };
    let enc_z = encode_to_vec(&zeros).expect("encode MatrixRow zeros");
    let (dec_z, _): (MatrixRow, _) = decode_from_slice(&enc_z).expect("decode MatrixRow zeros");
    assert_eq!(zeros, dec_z);

    let maxes = MatrixRow {
        row_index: usize::MAX,
        cells: [u32::MAX; 8],
    };
    let enc_m = encode_to_vec(&maxes).expect("encode MatrixRow maxes");
    let (dec_m, _): (MatrixRow, _) = decode_from_slice(&enc_m).expect("decode MatrixRow maxes");
    assert_eq!(maxes, dec_m);
}

// ── 22: Deeply nested Option<Vec<Option<String>>> ────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeeplyNested {
    data: Option<Vec<Option<String>>>,
    id: u32,
}

#[test]
fn test_deeply_nested_opt_vec_opt_string_some_values() {
    let original = DeeplyNested {
        data: Some(vec![
            Some("first".to_string()),
            None,
            Some("third".to_string()),
            None,
            Some(String::new()),
        ]),
        id: 99,
    };
    let enc = encode_to_vec(&original).expect("encode DeeplyNested some");
    let (dec, _): (DeeplyNested, _) = decode_from_slice(&enc).expect("decode DeeplyNested some");
    assert_eq!(original, dec);
}

#[test]
fn test_deeply_nested_opt_vec_opt_string_outer_none() {
    let original = DeeplyNested { data: None, id: 0 };
    let enc = encode_to_vec(&original).expect("encode DeeplyNested outer None");
    let (dec, _): (DeeplyNested, _) =
        decode_from_slice(&enc).expect("decode DeeplyNested outer None");
    assert_eq!(original, dec);
}

#[test]
fn test_deeply_nested_opt_vec_opt_string_empty_vec() {
    let original = DeeplyNested {
        data: Some(vec![]),
        id: 1,
    };
    let enc = encode_to_vec(&original).expect("encode DeeplyNested empty vec");
    let (dec, _): (DeeplyNested, _) =
        decode_from_slice(&enc).expect("decode DeeplyNested empty vec");
    assert_eq!(original, dec);
}
