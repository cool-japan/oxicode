//! Extended BorrowDecode tests (set 2) — 22 novel cases not covered by existing test files.
//!
//! Covers: Option<&str/&[u8]>, Vec<&str>, Vec<Cow<str>>, fixed-int config,
//! 3-level nested structs, struct-with-Option fields, large slices,
//! special-char strings, cross-type encode-then-borrow-decode, and
//! simultaneous multi-buffer zero-copy.

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
use oxicode::{BorrowDecode, Decode, Encode};
use std::borrow::Cow;

// ─── Type definitions ─────────────────────────────────────────────────────────

/// Deep nesting: 3 levels deep.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Level3Owned {
    tag: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level2Owned {
    inner: Level3Owned,
    seq: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level1Owned {
    mid: Level2Owned,
    flag: bool,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct Level3Borrowed<'a> {
    tag: &'a str,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct Level2Borrowed<'a> {
    inner: Level3Borrowed<'a>,
    seq: u32,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct Level1Borrowed<'a> {
    mid: Level2Borrowed<'a>,
    flag: bool,
}

/// Struct with 5 borrowed string fields.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct FiveStrs<'a> {
    f1: &'a str,
    f2: &'a str,
    f3: &'a str,
    f4: &'a str,
    f5: &'a str,
}

/// Struct with Option<&str> and Option<&[u8]> fields.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct OptionalBorrows<'a> {
    maybe_str: Option<&'a str>,
    maybe_bytes: Option<&'a [u8]>,
    id: u16,
}

/// Struct mixing primitive, borrowed str, and borrowed bytes.
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct RichBorrowed<'a> {
    version: u8,
    name: &'a str,
    payload: &'a [u8],
    count: i32,
}

// ─── Tests ────────────────────────────────────────────────────────────────────

// 1. Option<&str> Some — borrow_decode returns the borrowed value
#[test]
fn test_option_str_some_borrow_decode() {
    let original: Option<&str> = Some("option str value");
    let encoded = oxicode::encode_to_vec(&original).expect("encode Option<&str> Some");
    let (decoded, consumed): (Option<&str>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Option<&str> Some");
    assert_eq!(decoded, Some("option str value"));
    assert_eq!(consumed, encoded.len());
}

// 2. Option<&str> None — decodes to None without error
#[test]
fn test_option_str_none_borrow_decode() {
    let original: Option<&str> = None;
    let encoded = oxicode::encode_to_vec(&original).expect("encode Option<&str> None");
    let (decoded, consumed): (Option<&str>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Option<&str> None");
    assert_eq!(decoded, None);
    assert_eq!(consumed, encoded.len());
}

// 3. Option<&[u8]> Some — borrowed bytes returned correctly
#[test]
fn test_option_bytes_some_borrow_decode() {
    let original: Option<&[u8]> = Some(&[0xCA, 0xFE, 0xBA, 0xBE]);
    let encoded = oxicode::encode_to_vec(&original).expect("encode Option<&[u8]> Some");
    let (decoded, consumed): (Option<&[u8]>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Option<&[u8]> Some");
    assert_eq!(decoded, Some([0xCA_u8, 0xFE, 0xBA, 0xBE].as_ref()));
    assert_eq!(consumed, encoded.len());
}

// 4. Option<&[u8]> None — decodes to None without error
#[test]
fn test_option_bytes_none_borrow_decode() {
    let original: Option<&[u8]> = None;
    let encoded = oxicode::encode_to_vec(&original).expect("encode Option<&[u8]> None");
    let (decoded, consumed): (Option<&[u8]>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Option<&[u8]> None");
    assert_eq!(decoded, None);
    assert_eq!(consumed, encoded.len());
}

// 5. Vec<&str> — each element borrows from the encoded buffer
#[test]
fn test_vec_of_str_borrow_decode() {
    let items: Vec<&str> = vec!["alpha", "beta", "gamma", "delta"];
    let encoded = oxicode::encode_to_vec(&items).expect("encode Vec<&str>");
    let (decoded, consumed): (Vec<&str>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Vec<&str>");
    assert_eq!(decoded, items);
    assert_eq!(consumed, encoded.len());
}

// 6. Vec<Cow<str>> decoded via borrow_decode — all elements should be Borrowed variants
#[test]
fn test_vec_cow_str_borrow_decode_all_borrowed() {
    let items: Vec<&str> = vec!["cow_one", "cow_two", "cow_three"];
    let encoded = oxicode::encode_to_vec(&items).expect("encode for Vec<Cow<str>>");
    let (decoded, consumed): (Vec<Cow<str>>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Vec<Cow<str>>");
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
    for (idx, cow) in decoded.iter().enumerate() {
        assert!(
            matches!(cow, Cow::Borrowed(_)),
            "element {idx} should be Cow::Borrowed"
        );
    }
    assert_eq!(decoded[0].as_ref(), "cow_one");
    assert_eq!(decoded[1].as_ref(), "cow_two");
    assert_eq!(decoded[2].as_ref(), "cow_three");
}

// 7. Fixed-int encoding config: &str round-trip
#[test]
fn test_borrow_decode_with_fixed_int_config_str() {
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let original = "fixed_int_str";
    let encoded =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode with fixed_int config");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, cfg)
            .expect("borrow_decode with fixed_int config");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// 8. Fixed-int encoding config: &[u8] round-trip
#[test]
fn test_borrow_decode_with_fixed_int_config_bytes() {
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let original: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let encoded =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode bytes fixed_int");
    let (decoded, consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, cfg)
            .expect("borrow_decode bytes fixed_int");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// 9. 3-level nested struct — deepest tag borrows from the buffer
#[test]
fn test_three_level_nested_borrow_decode() {
    let original = Level1Owned {
        mid: Level2Owned {
            inner: Level3Owned {
                tag: "deep_borrow".to_string(),
            },
            seq: 777,
        },
        flag: true,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode 3-level owned");
    let (decoded, consumed): (Level1Borrowed, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode 3-level borrowed");
    assert_eq!(decoded.mid.inner.tag, "deep_borrow");
    assert_eq!(decoded.mid.seq, 777);
    assert!(decoded.flag);
    assert_eq!(consumed, encoded.len());

    // Verify the deepest tag pointer lies inside the encoded buffer
    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();
    let tag_ptr = decoded.mid.inner.tag.as_ptr() as usize;
    assert!(
        tag_ptr >= buf_start && tag_ptr < buf_end,
        "deep tag does not point into encoded buffer"
    );
}

// 10. Struct with 5 &str fields — all pointers within buffer
#[test]
fn test_five_str_fields_all_point_into_buffer() {
    let original = FiveStrs {
        f1: "one",
        f2: "two",
        f3: "three",
        f4: "four",
        f5: "five",
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode FiveStrs");
    let (decoded, consumed): (FiveStrs, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode FiveStrs");
    assert_eq!(decoded.f1, "one");
    assert_eq!(decoded.f2, "two");
    assert_eq!(decoded.f3, "three");
    assert_eq!(decoded.f4, "four");
    assert_eq!(decoded.f5, "five");
    assert_eq!(consumed, encoded.len());

    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();
    for (field_name, ptr) in [
        ("f1", decoded.f1.as_ptr() as usize),
        ("f2", decoded.f2.as_ptr() as usize),
        ("f3", decoded.f3.as_ptr() as usize),
        ("f4", decoded.f4.as_ptr() as usize),
        ("f5", decoded.f5.as_ptr() as usize),
    ] {
        assert!(
            ptr >= buf_start && ptr < buf_end,
            "{field_name} does not point into encoded buffer"
        );
    }
}

// 11. Struct with Option<&str> Some and Option<&[u8]> None fields
#[test]
fn test_struct_with_optional_borrows_mixed() {
    let original = OptionalBorrows {
        maybe_str: Some("present_str"),
        maybe_bytes: None,
        id: 42,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode OptionalBorrows mixed");
    let (decoded, consumed): (OptionalBorrows, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode OptionalBorrows mixed");
    assert_eq!(decoded.maybe_str, Some("present_str"));
    assert_eq!(decoded.maybe_bytes, None);
    assert_eq!(decoded.id, 42);
    assert_eq!(consumed, encoded.len());
}

// 12. Struct with Option<&str> None and Option<&[u8]> Some fields
#[test]
fn test_struct_with_optional_borrows_inverted() {
    let bytes_data: &[u8] = &[10, 20, 30, 40, 50];
    let original = OptionalBorrows {
        maybe_str: None,
        maybe_bytes: Some(bytes_data),
        id: 99,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode OptionalBorrows inverted");
    let (decoded, consumed): (OptionalBorrows, _) = oxicode::borrow_decode_from_slice(&encoded)
        .expect("borrow_decode OptionalBorrows inverted");
    assert_eq!(decoded.maybe_str, None);
    assert_eq!(decoded.maybe_bytes, Some([10_u8, 20, 30, 40, 50].as_ref()));
    assert_eq!(decoded.id, 99);
    assert_eq!(consumed, encoded.len());
}

// 13. Large &[u8] of 1000 bytes — borrowed slice points inside buffer
#[test]
fn test_borrow_decode_large_slice_1000_bytes() {
    let large: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let encoded = oxicode::encode_to_vec(&large).expect("encode 1000-byte slice");
    let (decoded, consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode 1000-byte slice");
    assert_eq!(decoded.len(), 1000);
    assert_eq!(decoded, large.as_slice());
    assert_eq!(consumed, encoded.len());

    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();
    let slice_ptr = decoded.as_ptr() as usize;
    assert!(
        slice_ptr >= buf_start && slice_ptr < buf_end,
        "1000-byte slice does not point into encoded buffer"
    );
}

// 14. &str of 1000 Unicode chars — borrowed pointer inside buffer
#[test]
fn test_borrow_decode_str_1000_unicode_chars() {
    let original: String = "日本語テスト".chars().cycle().take(1000).collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode 1000-char unicode string");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode 1000-char unicode");
    assert_eq!(decoded, original.as_str());
    assert_eq!(consumed, encoded.len());

    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();
    let str_ptr = decoded.as_ptr() as usize;
    assert!(
        str_ptr >= buf_start && str_ptr < buf_end,
        "1000-char unicode string does not point into encoded buffer"
    );
}

// 15. &str with control characters and special ASCII — correct content preserved
#[test]
fn test_borrow_decode_str_with_control_and_special_chars() {
    let original = "line1\nline2\ttabbed\rcarriage\x00null\x1Bescape";
    let encoded = oxicode::encode_to_vec(&original).expect("encode special-char str");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode special-char str");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// 16. Cross-type: encode Vec<u8> (owned), borrow_decode as &[u8]
#[test]
fn test_cross_type_vec_u8_encode_borrow_decode_as_slice() {
    let original: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF];
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<u8>");
    let (decoded, consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &[u8] from Vec<u8> enc");
    assert_eq!(decoded, original.as_slice());
    assert_eq!(consumed, encoded.len());
}

// 17. Cross-type: encode String (owned), borrow_decode as &str
#[test]
fn test_cross_type_string_encode_borrow_decode_as_str() {
    let original = String::from("owned string decoded as borrowed str");
    let encoded = oxicode::encode_to_vec(&original).expect("encode String");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &str from String enc");
    assert_eq!(decoded, original.as_str());
    assert_eq!(consumed, encoded.len());
}

// 18. RichBorrowed struct: u8 + &str + &[u8] + i32 — all fields correct
#[test]
fn test_rich_borrowed_struct_all_fields() {
    let payload: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
    let original = RichBorrowed {
        version: 3,
        name: "rich_struct_test",
        payload,
        count: -42,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode RichBorrowed");
    let (decoded, consumed): (RichBorrowed, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode RichBorrowed");
    assert_eq!(decoded.version, 3);
    assert_eq!(decoded.name, "rich_struct_test");
    assert_eq!(decoded.payload, payload);
    assert_eq!(decoded.count, -42);
    assert_eq!(consumed, encoded.len());
}

// 19. Two independent buffers alive simultaneously — no aliasing issues
#[test]
fn test_simultaneous_multi_buffer_borrow_decode() {
    let s1 = "first buffer content";
    let s2 = "second buffer content";
    let enc1 = oxicode::encode_to_vec(&s1).expect("encode s1");
    let enc2 = oxicode::encode_to_vec(&s2).expect("encode s2");

    // Both decodings are alive at the same time
    let (decoded1, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&enc1).expect("borrow_decode s1");
    let (decoded2, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&enc2).expect("borrow_decode s2");

    assert_eq!(decoded1, s1);
    assert_eq!(decoded2, s2);
    // Verify the two decoded strings point to different buffers
    assert_ne!(
        decoded1.as_ptr(),
        decoded2.as_ptr(),
        "decoded strings should not alias each other's buffers"
    );
}

// 20. &str whose content is only ASCII digits — no special-case corruption
#[test]
fn test_borrow_decode_str_digits_only() {
    let original = "1234567890".repeat(10);
    let encoded = oxicode::encode_to_vec(&original).expect("encode digit-only str");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode digit-only str");
    assert_eq!(decoded, original.as_str());
    assert_eq!(consumed, encoded.len());
}

// 21. Cow<str> retains exact content and is Borrowed even after re-borrowing from slice
#[test]
fn test_cow_str_borrow_decode_retains_exact_bytes() {
    let original = "Ça va? Ñoño 中文";
    let encoded = oxicode::encode_to_vec(&original).expect("encode multibyte Cow<str>");
    let (decoded, consumed): (Cow<str>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<str> multibyte");
    assert_eq!(decoded.as_ref(), original);
    assert_eq!(consumed, encoded.len());
    assert!(
        matches!(decoded, Cow::Borrowed(_)),
        "Cow<str> should be Borrowed variant for multibyte content"
    );
}

// 22. Struct with both Option fields set to Some — all borrowed fields within buffer
#[test]
fn test_struct_with_all_optional_borrows_some() {
    let bytes_val: &[u8] = &[0xAA, 0xBB, 0xCC, 0xDD];
    let original = OptionalBorrows {
        maybe_str: Some("both options set"),
        maybe_bytes: Some(bytes_val),
        id: 1024,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode OptionalBorrows all-Some");
    let (decoded, consumed): (OptionalBorrows, _) = oxicode::borrow_decode_from_slice(&encoded)
        .expect("borrow_decode OptionalBorrows all-Some");

    assert_eq!(decoded.maybe_str, Some("both options set"));
    assert_eq!(
        decoded.maybe_bytes,
        Some([0xAA_u8, 0xBB, 0xCC, 0xDD].as_ref())
    );
    assert_eq!(decoded.id, 1024);
    assert_eq!(consumed, encoded.len());

    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();

    let str_ptr = decoded
        .maybe_str
        .expect("maybe_str should be Some")
        .as_ptr() as usize;
    let bytes_ptr = decoded
        .maybe_bytes
        .expect("maybe_bytes should be Some")
        .as_ptr() as usize;

    assert!(
        str_ptr >= buf_start && str_ptr < buf_end,
        "maybe_str does not point into encoded buffer"
    );
    assert!(
        bytes_ptr >= buf_start && bytes_ptr < buf_end,
        "maybe_bytes does not point into encoded buffer"
    );
}
