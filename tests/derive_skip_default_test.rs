//! Tests for #[oxicode(skip)] and #[oxicode(default = "fn")] derive attributes.
//!
//! These tests focus on concrete field-type behaviour (u32, String, Vec<u8>),
//! positional placement of skipped fields, size invariants, byte-content
//! invariants, re-encode after mutation, and the interaction of skip/default
//! with rename, generics, Option<T>, and `default_value`.
//!
//! All tests are new — they do not duplicate derive_attr_test.rs.

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

// ---------------------------------------------------------------------------
// Module-level default functions (used by multiple tests)
// ---------------------------------------------------------------------------

fn default_nonzero_u32() -> u32 {
    42_u32
}

fn default_hello_string() -> String {
    "hello".to_string()
}

fn default_bytes_payload() -> Vec<u8> {
    vec![0xCA, 0xFE, 0xBA, 0xBE]
}

fn default_large_u64() -> u64 {
    0xDEAD_BEEF_CAFE_0000_u64
}

// ===========================================================================
// 1. Struct with one skipped u32 field — decoded value must be 0
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipU32 {
    keep: u32,
    #[oxicode(skip)]
    transient: u32,
}

#[test]
fn test_skip_u32_field_is_zero_on_decode() {
    let original = SkipU32 {
        keep: 123,
        transient: 0xDEAD_BEEF,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (SkipU32, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(
        decoded.keep, 123,
        "non-skipped field must survive round-trip"
    );
    assert_eq!(decoded.transient, 0_u32, "skipped u32 must be Default (0)");
    assert_eq!(bytes_read, encoded.len(), "all bytes consumed");
}

// ===========================================================================
// 2. Struct with skipped String field — decoded value must be ""
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipString {
    id: u64,
    #[oxicode(skip)]
    label: String,
}

#[test]
fn test_skip_string_field_is_empty_on_decode() {
    let original = SkipString {
        id: 999,
        label: "should_vanish".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SkipString, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.id, 999);
    assert_eq!(decoded.label, "", "skipped String must be Default (\"\")");
}

// ===========================================================================
// 3. Struct with skipped Vec<u8> field — decoded value must be []
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipVecU8 {
    version: u8,
    #[oxicode(skip)]
    raw_cache: Vec<u8>,
    checksum: u32,
}

#[test]
fn test_skip_vec_u8_field_is_empty_on_decode() {
    let original = SkipVecU8 {
        version: 2,
        raw_cache: vec![1, 2, 3, 4, 5, 6],
        checksum: 0xABCD,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SkipVecU8, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.version, 2);
    assert_eq!(
        decoded.raw_cache,
        Vec::<u8>::new(),
        "skipped Vec<u8> must be Default ([])"
    );
    assert_eq!(decoded.checksum, 0xABCD);
}

// ===========================================================================
// 4. Struct with multiple skipped fields — each independently zeroed
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiSkip {
    persistent_a: u32,
    #[oxicode(skip)]
    cache_x: u64,
    persistent_b: String,
    #[oxicode(skip)]
    cache_y: Vec<u8>,
    persistent_c: bool,
    #[oxicode(skip)]
    cache_z: i32,
}

#[test]
fn test_multiple_skipped_fields_all_zeroed_on_decode() {
    let original = MultiSkip {
        persistent_a: 10,
        cache_x: u64::MAX,
        persistent_b: "alive".to_string(),
        cache_y: vec![9, 8, 7],
        persistent_c: true,
        cache_z: -99,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (MultiSkip, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.persistent_a, 10);
    assert_eq!(decoded.persistent_b, "alive");
    assert!(decoded.persistent_c);
    assert_eq!(decoded.cache_x, 0_u64, "skipped u64 must be 0");
    assert_eq!(
        decoded.cache_y,
        Vec::<u8>::new(),
        "skipped Vec<u8> must be empty"
    );
    assert_eq!(decoded.cache_z, 0_i32, "skipped i32 must be 0");
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 5. Struct where EVERY non-structural field is skipped (only one real field)
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlmostAllSkipped {
    anchor: u32,
    #[oxicode(skip)]
    noise_a: String,
    #[oxicode(skip)]
    noise_b: Vec<u8>,
    #[oxicode(skip)]
    noise_c: bool,
}

#[test]
fn test_only_one_real_field_rest_skipped() {
    let original = AlmostAllSkipped {
        anchor: 7777,
        noise_a: "loud".to_string(),
        noise_b: vec![255; 100],
        noise_c: true,
    };
    let encoded = encode_to_vec(&original).expect("encode");

    // The encoded bytes should only represent `anchor` (a u32 varint).
    // A struct with only `anchor: u32 = 7777` encodes to exactly those bytes.
    #[derive(Encode)]
    struct AnchorOnly {
        anchor: u32,
    }
    let reference = encode_to_vec(&AnchorOnly { anchor: 7777 }).expect("encode reference");
    assert_eq!(
        encoded, reference,
        "skipped fields must not contribute bytes"
    );

    let (decoded, _): (AlmostAllSkipped, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.anchor, 7777);
    assert_eq!(decoded.noise_a, "");
    assert_eq!(decoded.noise_b, Vec::<u8>::new());
    assert!(!decoded.noise_c);
}

// ===========================================================================
// 6. Struct with skip on FIRST field
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipFirst {
    #[oxicode(skip)]
    ignored_leading: u32,
    real_a: String,
    real_b: u64,
}

#[test]
fn test_skip_first_field() {
    let original = SkipFirst {
        ignored_leading: 123,
        real_a: "first".to_string(),
        real_b: 4567,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (SkipFirst, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(
        decoded.ignored_leading, 0_u32,
        "first-field skip must be Default"
    );
    assert_eq!(decoded.real_a, "first");
    assert_eq!(decoded.real_b, 4567);
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 7. Struct with skip on LAST field
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipLast {
    real_x: u32,
    real_y: String,
    #[oxicode(skip)]
    trailing_cache: Vec<u8>,
}

#[test]
fn test_skip_last_field() {
    let original = SkipLast {
        real_x: 88,
        real_y: "last".to_string(),
        trailing_cache: vec![1, 2, 3],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (SkipLast, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.real_x, 88);
    assert_eq!(decoded.real_y, "last");
    assert_eq!(
        decoded.trailing_cache,
        Vec::<u8>::new(),
        "last-field skip must be Default"
    );
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 8. Struct with skip in MIDDLE
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipMiddle {
    head: u32,
    #[oxicode(skip)]
    middle_junk: String,
    tail: u32,
}

#[test]
fn test_skip_middle_field() {
    let original = SkipMiddle {
        head: 11,
        middle_junk: "noise".to_string(),
        tail: 22,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (SkipMiddle, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.head, 11);
    assert_eq!(decoded.middle_junk, "", "middle-field skip must be Default");
    assert_eq!(decoded.tail, 22);
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 9. #[oxicode(default = "fn")] — basic custom default function
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultFn {
    name: String,
    #[oxicode(default = "default_nonzero_u32")]
    priority: u32,
    active: bool,
}

#[test]
fn test_default_fn_basic_custom_function() {
    let original = WithDefaultFn {
        name: "Widget".to_string(),
        priority: 0, // value written at encode time — but field is NOT encoded
        active: false,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (WithDefaultFn, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.name, "Widget");
    assert_eq!(
        decoded.priority, 42_u32,
        "default_nonzero_u32() must supply 42"
    );
    assert!(!decoded.active);
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 10. Custom default function returning a non-zero u64
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultU64 {
    key: u32,
    #[oxicode(default = "default_large_u64")]
    magic: u64,
}

#[test]
fn test_default_fn_returns_nonzero_u64() {
    let original = WithDefaultU64 {
        key: 1,
        magic: 0xFFFF_FFFF_FFFF_FFFF,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (WithDefaultU64, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.key, 1);
    assert_eq!(
        decoded.magic, 0xDEAD_BEEF_CAFE_0000_u64,
        "default_large_u64() must supply the sentinel value"
    );
}

// ===========================================================================
// 11. Custom default returning String "hello"
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultString {
    id: u32,
    #[oxicode(default = "default_hello_string")]
    greeting: String,
    count: u16,
}

#[test]
fn test_default_fn_returns_hello_string() {
    let original = WithDefaultString {
        id: 5,
        greeting: "world".to_string(),
        count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (WithDefaultString, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.id, 5);
    assert_eq!(
        decoded.greeting, "hello",
        "default_hello_string() must return \"hello\""
    );
    assert_eq!(decoded.count, 3);
}

// ===========================================================================
// 12. Custom default returning Vec<u8> with non-trivial data
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultVec {
    seq: u32,
    #[oxicode(default = "default_bytes_payload")]
    payload: Vec<u8>,
}

#[test]
fn test_default_fn_returns_vec_with_data() {
    let original = WithDefaultVec {
        seq: 100,
        payload: vec![0x00],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (WithDefaultVec, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.seq, 100);
    assert_eq!(
        decoded.payload,
        vec![0xCA, 0xFE, 0xBA, 0xBE],
        "default_bytes_payload() must supply the magic bytes"
    );
}

// ===========================================================================
// 13. Combination: some skip, some default = "fn", some normal
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct ComboAttrs {
    real_id: u32,
    #[oxicode(skip)]
    skipped_ts: u64,
    real_name: String,
    #[oxicode(default = "default_nonzero_u32")]
    computed_score: u32,
    #[oxicode(skip)]
    skipped_buf: Vec<u8>,
    real_flag: bool,
}

#[test]
fn test_combination_skip_default_normal_fields() {
    let original = ComboAttrs {
        real_id: 42,
        skipped_ts: 9999_9999_9999,
        real_name: "combo".to_string(),
        computed_score: 1,
        skipped_buf: vec![0xFF; 16],
        real_flag: true,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (ComboAttrs, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.real_id, 42, "real_id must round-trip");
    assert_eq!(decoded.skipped_ts, 0_u64, "skipped_ts must be 0");
    assert_eq!(decoded.real_name, "combo", "real_name must round-trip");
    assert_eq!(
        decoded.computed_score, 42_u32,
        "computed_score via default fn"
    );
    assert_eq!(
        decoded.skipped_buf,
        Vec::<u8>::new(),
        "skipped_buf must be empty"
    );
    assert!(decoded.real_flag, "real_flag must round-trip");
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 14. Skip in struct with generic field T
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenericWithSkip<T> {
    value: T,
    #[oxicode(skip)]
    runtime_hint: u32,
    extra: u64,
}

#[test]
fn test_skip_in_generic_struct() {
    let original = GenericWithSkip::<Vec<u32>> {
        value: vec![10, 20, 30],
        runtime_hint: 0xABCD_EF01,
        extra: 0x1111,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (GenericWithSkip<Vec<u32>>, _) =
        decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.value, vec![10, 20, 30]);
    assert_eq!(
        decoded.runtime_hint, 0_u32,
        "skipped generic field must be Default"
    );
    assert_eq!(decoded.extra, 0x1111);
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 15. Skip field that is Option<T> — Default for Option<T> is None
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipOptionField {
    id: u32,
    #[oxicode(skip)]
    opt_cache: Option<String>,
    tag: u8,
}

#[test]
fn test_skip_option_field_is_none_on_decode() {
    let original = SkipOptionField {
        id: 55,
        opt_cache: Some("should vanish".to_string()),
        tag: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (SkipOptionField, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.id, 55);
    assert_eq!(decoded.opt_cache, None, "skipped Option<T> must be None");
    assert_eq!(decoded.tag, 7);
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 16. Two skip fields — verify encoded byte count matches only non-skip fields
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct TwoSkipSizeCheck {
    real_a: u32,
    #[oxicode(skip)]
    skip_b: u64,
    real_c: String,
    #[oxicode(skip)]
    skip_d: Vec<u8>,
}

#[test]
fn test_two_skip_fields_encoded_size_matches_non_skip_fields() {
    let value = TwoSkipSizeCheck {
        real_a: 7,
        skip_b: u64::MAX,
        real_c: "size".to_string(),
        skip_d: vec![0u8; 64],
    };

    #[derive(Encode)]
    struct OnlyReal {
        real_a: u32,
        real_c: String,
    }
    let reference = OnlyReal {
        real_a: 7,
        real_c: "size".to_string(),
    };

    let encoded_skip = encode_to_vec(&value).expect("encode with skip");
    let encoded_ref = encode_to_vec(&reference).expect("encode reference");

    assert_eq!(
        encoded_skip, encoded_ref,
        "encoding a struct with two skipped fields must equal encoding only the non-skip fields"
    );
}

// ===========================================================================
// 17. Skipped field content does NOT appear in encoded bytes
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SecretPayload {
    public_id: u32,
    #[oxicode(skip)]
    private_key: Vec<u8>,
}

#[test]
fn test_skipped_field_bytes_absent_from_encoded_output() {
    // Use a distinctive sentinel byte pattern that would be detectable if present.
    let sentinel: Vec<u8> = vec![0xDE, 0xAD, 0xC0, 0xDE, 0xFE, 0xED];
    let original = SecretPayload {
        public_id: 1,
        private_key: sentinel.clone(),
    };
    let encoded = encode_to_vec(&original).expect("encode");

    // The sentinel bytes must not appear as a contiguous sub-slice in `encoded`.
    let found = encoded
        .windows(sentinel.len())
        .any(|w| w == sentinel.as_slice());
    assert!(
        !found,
        "sentinel bytes of a skipped field must not appear in the encoded output; \
         got {} bytes: {:?}",
        encoded.len(),
        encoded
    );
}

// ===========================================================================
// 18. decode → modify skipped field → re-encode → verify new value persists
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct MutateAfterDecode {
    anchor: String,
    #[oxicode(skip)]
    mutable_cache: u32,
}

#[test]
fn test_modify_skipped_field_then_reencode_value_persists() {
    // Round 1: encode original, decode — cache is zeroed.
    let step1 = MutateAfterDecode {
        anchor: "stable".to_string(),
        mutable_cache: 0xDEAD,
    };
    let bytes1 = encode_to_vec(&step1).expect("encode step1");
    let (mut decoded1, _): (MutateAfterDecode, _) =
        decode_from_slice(&bytes1).expect("decode step1");

    assert_eq!(
        decoded1.mutable_cache, 0_u32,
        "skipped field is Default after first decode"
    );

    // Round 2: manually set the skipped field, re-encode, decode — new value IS encoded.
    decoded1.mutable_cache = 9999;
    // Now mutable_cache is set on the in-memory struct, but skip means it will NOT be
    // included in the next encode either. The anchor field must still round-trip.
    let bytes2 = encode_to_vec(&decoded1).expect("encode step2");
    let (decoded2, _): (MutateAfterDecode, _) = decode_from_slice(&bytes2).expect("decode step2");

    // The anchor must survive.
    assert_eq!(decoded2.anchor, "stable");
    // The mutated value was in memory but skip excluded it again; Default applies.
    assert_eq!(
        decoded2.mutable_cache, 0_u32,
        "after re-encode the skipped field is again Default on decode"
    );
    // Both encoded blobs are identical because anchor is the only encoded field.
    assert_eq!(
        bytes1, bytes2,
        "re-encoding with a mutated skip field yields same bytes"
    );
}

// ===========================================================================
// 19. Struct with skip + rename combination
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipWithRename {
    #[oxicode(rename = "userId")]
    user_id: u32,
    #[oxicode(skip, rename = "internalCache")]
    internal_cache: String,
    #[oxicode(rename = "displayName")]
    display_name: String,
}

#[test]
fn test_skip_and_rename_combination() {
    let original = SkipWithRename {
        user_id: 42,
        internal_cache: "do_not_encode".to_string(),
        display_name: "Alice".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (SkipWithRename, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(
        decoded.user_id, 42,
        "renamed but encoded field must round-trip"
    );
    assert_eq!(
        decoded.internal_cache, "",
        "renamed+skipped field must be Default"
    );
    assert_eq!(
        decoded.display_name, "Alice",
        "renamed but encoded field must round-trip"
    );
    assert_eq!(bytes_read, encoded.len());
}

// ===========================================================================
// 20. Verify byte count: skip reduces encoded size vs. a plain version
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipReducesSize {
    a: u32,
    #[oxicode(skip)]
    b: Vec<u8>,
    c: u32,
}

#[test]
fn test_skip_reduces_encoded_byte_count() {
    // Prepare a Vec<u8> that is large enough to guarantee the skip clearly
    // reduces the byte count (1000 bytes + length prefix).
    let large_vec = vec![0xAAu8; 1000];

    let with_skip = SkipReducesSize {
        a: 1,
        b: large_vec.clone(),
        c: 2,
    };

    #[derive(Encode)]
    struct NoSkip {
        a: u32,
        b: Vec<u8>,
        c: u32,
    }
    let no_skip = NoSkip {
        a: 1,
        b: large_vec,
        c: 2,
    };

    let bytes_with_skip = encode_to_vec(&with_skip).expect("encode with skip").len();
    let bytes_no_skip = encode_to_vec(&no_skip).expect("encode no skip").len();

    assert!(
        bytes_with_skip < bytes_no_skip,
        "skip must reduce encoded size: {bytes_with_skip} should be < {bytes_no_skip}"
    );
    // Quantitative lower bound: must have saved at least 1000 bytes.
    assert!(
        bytes_no_skip - bytes_with_skip >= 1000,
        "must save at least 1000 bytes; saved {}",
        bytes_no_skip - bytes_with_skip
    );
}

// ===========================================================================
// 21. Skip field in a nested struct — outer struct encodes normally,
//     inner struct's skipped field is zeroed on decode
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerWithSkip {
    real_val: u32,
    #[oxicode(skip)]
    ephemeral: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OuterNested {
    label: String,
    inner: InnerWithSkip,
    tag: u8,
}

#[test]
fn test_skip_in_nested_struct() {
    let original = OuterNested {
        label: "outer".to_string(),
        inner: InnerWithSkip {
            real_val: 77,
            ephemeral: "will_be_lost".to_string(),
        },
        tag: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode nested");
    let (decoded, bytes_read): (OuterNested, _) =
        decode_from_slice(&encoded).expect("decode nested");

    assert_eq!(decoded.label, "outer", "outer label must round-trip");
    assert_eq!(
        decoded.inner.real_val, 77,
        "nested real field must round-trip"
    );
    assert_eq!(
        decoded.inner.ephemeral, "",
        "skipped field inside nested struct must be Default"
    );
    assert_eq!(decoded.tag, 3, "outer tag must round-trip");
    assert_eq!(bytes_read, encoded.len(), "all bytes consumed");
}

// ===========================================================================
// 22. #[oxicode(default = "String::new")] — path-style stdlib function
//     ensures the attribute parser handles "::" in function paths
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct DefaultStringNew {
    id: u32,
    #[oxicode(default = "String::new")]
    memo: String,
    count: u16,
}

#[test]
fn test_default_fn_string_new_path() {
    let original = DefaultStringNew {
        id: 8,
        memo: "ignored on decode".to_string(),
        count: 4,
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (DefaultStringNew, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.id, 8, "id must round-trip");
    assert_eq!(
        decoded.memo, "",
        "String::new default must produce an empty string"
    );
    assert_eq!(decoded.count, 4, "count must round-trip");
    assert_eq!(bytes_read, encoded.len(), "all bytes consumed");
}
