//! Tests for derive macro field attributes: #[oxicode(skip)] and #[oxicode(default = "fn_path")]
//!
//! These tests verify that:
//! - #[oxicode(skip)] excludes a field from the binary stream on encode and
//!   fills it with Default::default() on decode.
//! - #[oxicode(default = "fn_path")] excludes a field from the binary stream on
//!   encode and calls fn_path() on decode.
//! - Non-annotated fields continue to round-trip correctly alongside annotated fields.
//! - The attributes work on named-field structs, tuple structs, and enum variants.
//! - BorrowDecode respects the same attributes.

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

// ---------------------------------------------------------------------------
// Helper default functions used by `#[oxicode(default = "...")]` tests
// ---------------------------------------------------------------------------

fn default_score() -> u32 {
    999
}

fn default_tag() -> String {
    "untagged".to_string()
}

fn default_vec() -> Vec<u8> {
    vec![0xDE, 0xAD]
}

fn default_pair() -> (u8, u8) {
    (42, 43)
}

// ---------------------------------------------------------------------------
// 1. Named-field struct — #[oxicode(skip)]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipNamed {
    id: u32,
    name: String,
    #[oxicode(skip)]
    cached_hash: u64, // not in binary stream; restored as 0
}

#[test]
fn test_skip_named_field_roundtrip() {
    let original = SkipNamed {
        id: 7,
        name: "Alice".to_string(),
        cached_hash: 0xDEAD_BEEF,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (SkipNamed, _) = decode_from_slice(&encoded).expect("decode failed");

    // Encoded / decoded content for non-skipped fields must match.
    assert_eq!(decoded.id, 7);
    assert_eq!(decoded.name, "Alice");
    // Skipped field must be Default::default() (0u64) after decode.
    assert_eq!(decoded.cached_hash, 0u64);
    // Ensure all bytes were consumed.
    assert_eq!(bytes_read, encoded.len());
}

#[test]
fn test_skip_named_field_bytes_smaller() {
    // Encoding a struct without the skipped field should produce fewer bytes
    // than one where the u64 field is fully encoded.
    let with_skip = SkipNamed {
        id: 1,
        name: "x".to_string(),
        cached_hash: u64::MAX,
    };

    #[derive(Encode)]
    struct NoSkip {
        id: u32,
        name: String,
        cached_hash: u64,
    }

    let no_skip = NoSkip {
        id: 1,
        name: "x".to_string(),
        cached_hash: u64::MAX,
    };

    let bytes_with_skip = encode_to_vec(&with_skip)
        .expect("encode with skip failed")
        .len();
    let bytes_no_skip = encode_to_vec(&no_skip)
        .expect("encode no skip failed")
        .len();

    // The skipped version should be strictly smaller because u64::MAX needs
    // 9 bytes in varint, while the skipped version emits nothing for that field.
    assert!(
        bytes_with_skip < bytes_no_skip,
        "expected {bytes_with_skip} < {bytes_no_skip}"
    );
}

// ---------------------------------------------------------------------------
// 2. Named-field struct — #[oxicode(default = "fn_path")]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct DefaultFnNamed {
    name: String,
    #[oxicode(default = "default_score")]
    score: u32,
    active: bool,
}

#[test]
fn test_default_fn_named_field() {
    let original = DefaultFnNamed {
        name: "Bob".to_string(),
        score: 42, // this value is NOT encoded
        active: true,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (DefaultFnNamed, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.name, "Bob");
    // score was not encoded; default_score() returns 999
    assert_eq!(decoded.score, 999);
    assert!(decoded.active);
}

#[test]
fn test_default_fn_string_field() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Tagged {
        id: u64,
        #[oxicode(default = "default_tag")]
        tag: String,
    }

    let original = Tagged {
        id: 55,
        tag: "encoded_tag".to_string(),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Tagged, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 55);
    // tag was not encoded; default_tag() returns "untagged"
    assert_eq!(decoded.tag, "untagged");
}

// ---------------------------------------------------------------------------
// 3. Multiple annotated fields together
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultipleAttrs {
    id: u32,
    #[oxicode(skip)]
    temp_state: bool,
    name: String,
    #[oxicode(default = "default_score")]
    score: u32,
    #[oxicode(skip)]
    debug_info: Option<String>,
}

#[test]
fn test_multiple_annotated_fields() {
    let original = MultipleAttrs {
        id: 99,
        temp_state: true,
        name: "Carol".to_string(),
        score: 500,
        debug_info: Some("debug".to_string()),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MultipleAttrs, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 99);
    assert!(!decoded.temp_state); // Default::default() for bool = false
    assert_eq!(decoded.name, "Carol");
    assert_eq!(decoded.score, 999); // default_score()
    assert_eq!(decoded.debug_info, None); // Default::default() for Option<_> = None
}

// ---------------------------------------------------------------------------
// 4. Tuple struct — #[oxicode(skip)]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipTuple(
    u32,
    #[oxicode(skip)] u64, // skipped field
    String,
);

#[test]
fn test_skip_tuple_struct() {
    let original = SkipTuple(10, 0xFFFF_FFFF, "hello".to_string());

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipTuple, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.0, 10);
    assert_eq!(decoded.1, 0u64); // Default::default()
    assert_eq!(decoded.2, "hello");
}

// ---------------------------------------------------------------------------
// 5. Tuple struct — #[oxicode(default = "fn_path")]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct DefaultFnTuple(u32, #[oxicode(default = "default_score")] u32, bool);

#[test]
fn test_default_fn_tuple_struct() {
    let original = DefaultFnTuple(1, 42, false);

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (DefaultFnTuple, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.0, 1);
    assert_eq!(decoded.1, 999); // default_score()
    assert!(!decoded.2);
}

// ---------------------------------------------------------------------------
// 6. Enum variants — #[oxicode(skip)]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum EventKind {
    Login {
        user_id: u32,
        #[oxicode(skip)]
        ip_cache: u32,
    },
    Upload(String, #[oxicode(skip)] u64),
    Logout,
}

#[test]
fn test_enum_named_variant_skip() {
    let original = EventKind::Login {
        user_id: 42,
        ip_cache: 0xC0A8_0001,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (EventKind, _) = decode_from_slice(&encoded).expect("decode failed");

    if let EventKind::Login { user_id, ip_cache } = decoded {
        assert_eq!(user_id, 42);
        assert_eq!(ip_cache, 0u32); // Default::default()
    } else {
        panic!("Expected EventKind::Login");
    }
}

#[test]
fn test_enum_tuple_variant_skip() {
    let original = EventKind::Upload("report.pdf".to_string(), 12345678);

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (EventKind, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded, EventKind::Upload("report.pdf".to_string(), 0u64));
}

#[test]
fn test_enum_unit_variant_unaffected() {
    let original = EventKind::Logout;

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (EventKind, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded, EventKind::Logout);
}

// ---------------------------------------------------------------------------
// 7. Enum variants — #[oxicode(default = "fn_path")]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Report {
    Summary {
        title: String,
        #[oxicode(default = "default_score")]
        rating: u32,
    },
    Raw(Vec<u8>, #[oxicode(default = "default_vec")] Vec<u8>),
}

#[test]
fn test_enum_named_variant_default_fn() {
    let original = Report::Summary {
        title: "Q4 Report".to_string(),
        rating: 85,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Report, _) = decode_from_slice(&encoded).expect("decode failed");

    if let Report::Summary { title, rating } = decoded {
        assert_eq!(title, "Q4 Report");
        assert_eq!(rating, 999); // default_score()
    } else {
        panic!("Expected Report::Summary");
    }
}

#[test]
fn test_enum_tuple_variant_default_fn() {
    let original = Report::Raw(vec![1, 2, 3], vec![9, 8, 7]);

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Report, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(
        decoded,
        Report::Raw(vec![1, 2, 3], vec![0xDE, 0xAD]) // default_vec()
    );
}

// ---------------------------------------------------------------------------
// 8. Generic struct — #[oxicode(skip)]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenericSkip<T> {
    value: T,
    #[oxicode(skip)]
    internal_counter: u32,
}

#[test]
fn test_generic_struct_skip() {
    let original = GenericSkip::<String> {
        value: "test".to_string(),
        internal_counter: 1000,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (GenericSkip<String>, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.value, "test");
    assert_eq!(decoded.internal_counter, 0u32);
}

// ---------------------------------------------------------------------------
// 9. BorrowDecode — #[oxicode(skip)]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct BorrowSkip<'a> {
    data: &'a [u8],
    name: &'a str,
    #[oxicode(skip)]
    flags: u32,
}

#[test]
fn test_borrow_decode_skip() {
    let original = BorrowSkip {
        data: b"binary",
        name: "zero-copy",
        flags: 0xABCD,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BorrowSkip<'_>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow decode failed");

    assert_eq!(decoded.data, b"binary");
    assert_eq!(decoded.name, "zero-copy");
    assert_eq!(decoded.flags, 0u32); // Default::default()
}

// ---------------------------------------------------------------------------
// 10. BorrowDecode — #[oxicode(default = "fn_path")]
// ---------------------------------------------------------------------------

fn default_marker() -> u8 {
    0xFF
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct BorrowDefault<'a> {
    payload: &'a [u8],
    #[oxicode(default = "default_marker")]
    version_marker: u8,
}

#[test]
fn test_borrow_decode_default_fn() {
    let original = BorrowDefault {
        payload: b"data",
        version_marker: 1,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BorrowDefault<'_>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow decode failed");

    assert_eq!(decoded.payload, b"data");
    assert_eq!(decoded.version_marker, 0xFF); // default_marker()
}

// ---------------------------------------------------------------------------
// 11. Verify that non-annotated round-trip still works alongside annotated
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Mixed {
    a: u32,
    #[oxicode(skip)]
    b: u32,
    c: String,
    #[oxicode(default = "default_score")]
    d: u32,
    e: bool,
}

#[test]
fn test_mixed_fields_roundtrip() {
    let original = Mixed {
        a: 1,
        b: 2,
        c: "hello".to_string(),
        d: 3,
        e: true,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (Mixed, _) = decode_from_slice(&encoded).expect("decode failed");

    // Encoded fields are faithfully restored.
    assert_eq!(decoded.a, 1);
    assert_eq!(decoded.c, "hello");
    assert!(decoded.e);
    // Skipped / default-fn fields get their alternate values.
    assert_eq!(decoded.b, 0u32);
    assert_eq!(decoded.d, 999);
    // All bytes consumed.
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 12. Tuple field with complex type and default fn
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithComplexDefault(u32, #[oxicode(default = "default_pair")] (u8, u8), u32);

#[test]
fn test_complex_default_fn() {
    let original = WithComplexDefault(10, (1, 2), 20);

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithComplexDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.0, 10);
    assert_eq!(decoded.1, (42u8, 43u8)); // default_pair()
    assert_eq!(decoded.2, 20);
}

// ---------------------------------------------------------------------------
// 13. All fields skipped (degenerate but valid case)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllSkipped {
    #[oxicode(skip)]
    a: u32,
    #[oxicode(skip)]
    b: String,
    #[oxicode(skip)]
    c: bool,
}

#[test]
fn test_all_fields_skipped() {
    let original = AllSkipped {
        a: 99,
        b: "ignored".to_string(),
        c: true,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    // The encoded form should be empty (no bytes).
    assert!(
        encoded.is_empty(),
        "expected zero bytes but got {}",
        encoded.len()
    );

    let (decoded, _): (AllSkipped, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.a, 0);
    assert_eq!(decoded.b, "");
    assert!(!decoded.c);
}

// ---------------------------------------------------------------------------
// 14. Unit struct is unaffected (regression guard)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct UnitStruct;

#[test]
fn test_unit_struct_regression() {
    let original = UnitStruct;
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (UnitStruct, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 15. Enum custom variant tags — #[oxicode(variant = N)]
// ---------------------------------------------------------------------------

#[test]
fn test_enum_custom_variant_tags() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum Protocol {
        #[oxicode(variant = 0x10)]
        Connect,
        #[oxicode(variant = 0x20)]
        Disconnect,
        #[oxicode(variant = 0x30)]
        Data(Vec<u8>),
    }

    let msg = Protocol::Data(vec![1, 2, 3]);
    let encoded = encode_to_vec(&msg).expect("encode failed");
    let (decoded, _): (Protocol, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(msg, decoded);

    // Also test unit variants round-trip correctly with custom tags.
    let msg2 = Protocol::Connect;
    let encoded2 = encode_to_vec(&msg2).expect("encode failed");
    let (decoded2, _): (Protocol, _) = decode_from_slice(&encoded2).expect("decode failed");
    assert_eq!(msg2, decoded2);

    let msg3 = Protocol::Disconnect;
    let encoded3 = encode_to_vec(&msg3).expect("encode failed");
    let (decoded3, _): (Protocol, _) = decode_from_slice(&encoded3).expect("decode failed");
    assert_eq!(msg3, decoded3);
}

#[test]
fn test_enum_mixed_tags_and_default() {
    // Some variants have explicit tags, some use default position-index numbering.
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum MixedTags {
        First, // 0 (default position index)
        #[oxicode(variant = 100)]
        Special, // 100 (explicit)
        Third, // 2 (default position index)
    }

    for val in [MixedTags::First, MixedTags::Special, MixedTags::Third] {
        let encoded = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (MixedTags, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(val, decoded);
    }
}

// ---------------------------------------------------------------------------
// 16. rename attribute — accepted as no-op on wire format
// ---------------------------------------------------------------------------

#[test]
fn test_rename_accepted_noop() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Person {
        #[oxicode(rename = "firstName")]
        first_name: String,
        #[oxicode(rename = "lastName")]
        last_name: String,
    }

    let p = Person {
        first_name: "Alice".into(),
        last_name: "Smith".into(),
    };
    let enc = oxicode::encode_to_vec(&p).expect("encode");
    let (dec, _): (Person, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(p, dec);
}

#[test]
fn test_rename_on_enum_variant() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum Status {
        #[oxicode(rename = "active")]
        Active,
        #[oxicode(rename = "inactive")]
        Inactive,
    }

    for s in [Status::Active, Status::Inactive] {
        let enc = oxicode::encode_to_vec(&s).expect("encode");
        let (dec, _): (Status, _) = oxicode::decode_from_slice(&enc).expect("decode");
        assert_eq!(s, dec);
    }
}
