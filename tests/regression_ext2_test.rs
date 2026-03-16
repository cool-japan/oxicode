//! Regression / correctness tests for OxiCode — second extended set.
//!
//! Verifies specific encoding behaviors: varint byte-length thresholds,
//! zigzag mapping, Unicode edge cases, nested Options, large collections,
//! 128-bit integers, NaN/Infinity bit-exact round-trips, and more.
//!
//! Rules:
//!   - No `#[cfg(test)]` module wrapper — every test is a top-level `#[test]`.
//!   - No `unwrap()` — every fallible call uses `.expect("…")`.

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

// ────────────────────────────────────────────────────────────────────────────
// Test 1: u64 boundary values encode and decode correctly
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_u64_boundary_values_roundtrip() {
    let cases: &[u64] = &[
        0,
        249,
        250,
        251,
        65_534,
        65_535,
        65_536,
        u32::MAX as u64,
        u64::MAX,
    ];
    for &v in cases {
        let enc = encode_to_vec(&v).expect("encode u64 boundary");
        let (dec, consumed): (u64, _) = decode_from_slice(&enc).expect("decode u64 boundary");
        assert_eq!(v, dec, "u64 boundary {v} must round-trip");
        assert_eq!(
            consumed,
            enc.len(),
            "u64 boundary {v}: all bytes must be consumed"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 2: varint encoding of exactly 250 is 1 byte
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_varint_250_is_one_byte() {
    let enc = encode_to_vec(&250u64).expect("encode 250");
    assert_eq!(
        enc.len(),
        1,
        "250 must be encoded in exactly 1 byte, got {} bytes",
        enc.len()
    );
    assert_eq!(enc[0], 250, "the single byte must have value 250");
}

// ────────────────────────────────────────────────────────────────────────────
// Test 3: varint encoding of 251 is 3 bytes (tag + 2-byte u16)
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_varint_251_is_three_bytes() {
    let enc = encode_to_vec(&251u64).expect("encode 251");
    assert_eq!(
        enc.len(),
        3,
        "251 must be encoded in 3 bytes (U16_BYTE tag + 2 value bytes), got {}",
        enc.len()
    );
    // Tag byte must be 251 (U16_BYTE sentinel)
    assert_eq!(enc[0], 251, "first byte must be the U16_BYTE tag (251)");
}

// ────────────────────────────────────────────────────────────────────────────
// Test 4: varint encoding of 65535 is 3 bytes
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_varint_65535_is_three_bytes() {
    let enc = encode_to_vec(&65_535u64).expect("encode 65535");
    assert_eq!(
        enc.len(),
        3,
        "65535 must be encoded in 3 bytes, got {}",
        enc.len()
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Test 5: varint encoding of 65536 is 5 bytes (tag + 4-byte u32)
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_varint_65536_is_five_bytes() {
    let enc = encode_to_vec(&65_536u64).expect("encode 65536");
    assert_eq!(
        enc.len(),
        5,
        "65536 must be encoded in 5 bytes (U32_BYTE tag + 4 value bytes), got {}",
        enc.len()
    );
    // Tag byte must be 252 (U32_BYTE sentinel)
    assert_eq!(enc[0], 252, "first byte must be the U32_BYTE tag (252)");
}

// ────────────────────────────────────────────────────────────────────────────
// Test 6: varint encoding of u32::MAX is 5 bytes
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_varint_u32_max_is_five_bytes() {
    let v = u32::MAX as u64;
    let enc = encode_to_vec(&v).expect("encode u32::MAX as u64");
    assert_eq!(
        enc.len(),
        5,
        "u32::MAX must be encoded in 5 bytes, got {}",
        enc.len()
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Test 7: varint encoding of u32::MAX + 1 is 9 bytes (tag + 8-byte u64)
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_varint_u32_max_plus_one_is_nine_bytes() {
    let v: u64 = u32::MAX as u64 + 1;
    let enc = encode_to_vec(&v).expect("encode u32::MAX+1");
    assert_eq!(
        enc.len(),
        9,
        "u32::MAX+1 must be encoded in 9 bytes (U64_BYTE tag + 8 value bytes), got {}",
        enc.len()
    );
    // Tag byte must be 253 (U64_BYTE sentinel)
    assert_eq!(enc[0], 253, "first byte must be the U64_BYTE tag (253)");
}

// ────────────────────────────────────────────────────────────────────────────
// Test 8: zigzag encoding — negative values map to odd unsigned integers
//   -1 → 1,  -2 → 3,  -3 → 5
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_zigzag_negative_values_map_to_odd_unsigned() {
    // The zigzag formula: (n << 1) ^ (n >> 63).
    // For i64:  -1 → 1,  -2 → 3,  -3 → 5
    // We verify indirectly: the encoded byte size of i64(-1) must equal
    // the encoded byte size of u64(1), and the two wire bytes must be equal.
    let neg_cases: &[(i64, u64)] = &[(-1, 1), (-2, 3), (-3, 5)];
    for &(signed, expected_unsigned) in neg_cases {
        let enc_signed = encode_to_vec(&signed).expect("encode signed");
        let enc_expected = encode_to_vec(&expected_unsigned).expect("encode unsigned");
        assert_eq!(
            enc_signed, enc_expected,
            "zigzag({signed}) must produce the same bytes as u64({expected_unsigned})"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 9: zigzag encoding — non-negative values map to even unsigned integers
//   0 → 0,  1 → 2,  2 → 4
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_zigzag_nonnegative_values_map_to_even_unsigned() {
    let pos_cases: &[(i64, u64)] = &[(0, 0), (1, 2), (2, 4)];
    for &(signed, expected_unsigned) in pos_cases {
        let enc_signed = encode_to_vec(&signed).expect("encode signed");
        let enc_expected = encode_to_vec(&expected_unsigned).expect("encode unsigned");
        assert_eq!(
            enc_signed, enc_expected,
            "zigzag({signed}) must produce the same bytes as u64({expected_unsigned})"
        );
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 10: String with embedded null bytes round-trips correctly
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_string_with_embedded_null_bytes_roundtrip() {
    let s = "hello\0world\0\0end".to_string();
    let enc = encode_to_vec(&s).expect("encode string with nulls");
    let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode string with nulls");
    assert_eq!(s, dec, "string with embedded nulls must round-trip exactly");
    assert_eq!(consumed, enc.len());
}

// ────────────────────────────────────────────────────────────────────────────
// Test 11: String spanning all Unicode planes round-trips correctly
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_string_all_unicode_planes_roundtrip() {
    // BMP (plane 0), Supplementary Multilingual Plane (plane 1),
    // Supplementary Ideographic Plane (plane 2), and the highest valid codepoint.
    let s: String = [
        '\u{0041}',   // A — BMP basic Latin
        '\u{03B1}',   // α — Greek
        '\u{4E2D}',   // 中 — CJK
        '\u{1F600}',  // 😀 — emoji (SMP)
        '\u{20000}',  // 𠀀 — CJK Extension B (SIP)
        '\u{10FFFF}', // highest valid Unicode codepoint
    ]
    .iter()
    .collect();

    let enc = encode_to_vec(&s).expect("encode unicode planes string");
    let (dec, consumed): (String, _) =
        decode_from_slice(&enc).expect("decode unicode planes string");
    assert_eq!(s, dec, "multi-plane Unicode string must round-trip exactly");
    assert_eq!(consumed, enc.len());
}

// ────────────────────────────────────────────────────────────────────────────
// Test 12: Empty struct encodes and decodes (size may be 0 bytes)
// ────────────────────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Encode, Decode)]
struct EmptyStruct;

#[test]
fn correctness_empty_struct_roundtrip() {
    let v = EmptyStruct;
    let enc = encode_to_vec(&v).expect("encode EmptyStruct");
    let (dec, consumed): (EmptyStruct, _) = decode_from_slice(&enc).expect("decode EmptyStruct");
    assert_eq!(v, dec);
    assert_eq!(
        consumed,
        enc.len(),
        "EmptyStruct must consume exactly all encoded bytes"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Test 13: Struct with only skipped fields encodes to 0 bytes
// ────────────────────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Encode, Decode)]
struct AllFieldsSkipped {
    #[oxicode(skip)]
    alpha: u32,
    #[oxicode(skip)]
    beta: String,
    #[oxicode(skip)]
    gamma: Vec<u8>,
}

#[test]
fn correctness_all_skipped_fields_encodes_to_zero_bytes() {
    let v = AllFieldsSkipped {
        alpha: 0xDEAD_BEEF,
        beta: "should not appear".to_string(),
        gamma: vec![1, 2, 3],
    };
    let enc = encode_to_vec(&v).expect("encode AllFieldsSkipped");
    assert_eq!(
        enc.len(),
        0,
        "struct with all fields skipped must produce 0 bytes, got {}",
        enc.len()
    );
    let (dec, consumed): (AllFieldsSkipped, _) =
        decode_from_slice(&enc).expect("decode AllFieldsSkipped");
    // Skipped fields decode as Default; beta/gamma will be empty, alpha will be 0.
    assert_eq!(dec.alpha, 0);
    assert_eq!(dec.beta, "");
    assert!(dec.gamma.is_empty());
    assert_eq!(consumed, 0);
}

// ────────────────────────────────────────────────────────────────────────────
// Test 14: Enum with 256 variants — index 255 requires a multi-byte varint tag
// ────────────────────────────────────────────────────────────────────────────
// We build a 256-variant enum using a macro to avoid 256 lines of boilerplate.
macro_rules! build_enum_256 {
    ($name:ident; $($v:ident = $i:expr),* $(,)?) => {
        #[derive(Debug, PartialEq, Encode, Decode)]
        enum $name {
            $($v),*
        }
    };
}

build_enum_256!(
    E256;
    V000 = 0,   V001 = 1,   V002 = 2,   V003 = 3,   V004 = 4,
    V005 = 5,   V006 = 6,   V007 = 7,   V008 = 8,   V009 = 9,
    V010 = 10,  V011 = 11,  V012 = 12,  V013 = 13,  V014 = 14,
    V015 = 15,  V016 = 16,  V017 = 17,  V018 = 18,  V019 = 19,
    V020 = 20,  V021 = 21,  V022 = 22,  V023 = 23,  V024 = 24,
    V025 = 25,  V026 = 26,  V027 = 27,  V028 = 28,  V029 = 29,
    V030 = 30,  V031 = 31,  V032 = 32,  V033 = 33,  V034 = 34,
    V035 = 35,  V036 = 36,  V037 = 37,  V038 = 38,  V039 = 39,
    V040 = 40,  V041 = 41,  V042 = 42,  V043 = 43,  V044 = 44,
    V045 = 45,  V046 = 46,  V047 = 47,  V048 = 48,  V049 = 49,
    V050 = 50,  V051 = 51,  V052 = 52,  V053 = 53,  V054 = 54,
    V055 = 55,  V056 = 56,  V057 = 57,  V058 = 58,  V059 = 59,
    V060 = 60,  V061 = 61,  V062 = 62,  V063 = 63,  V064 = 64,
    V065 = 65,  V066 = 66,  V067 = 67,  V068 = 68,  V069 = 69,
    V070 = 70,  V071 = 71,  V072 = 72,  V073 = 73,  V074 = 74,
    V075 = 75,  V076 = 76,  V077 = 77,  V078 = 78,  V079 = 79,
    V080 = 80,  V081 = 81,  V082 = 82,  V083 = 83,  V084 = 84,
    V085 = 85,  V086 = 86,  V087 = 87,  V088 = 88,  V089 = 89,
    V090 = 90,  V091 = 91,  V092 = 92,  V093 = 93,  V094 = 94,
    V095 = 95,  V096 = 96,  V097 = 97,  V098 = 98,  V099 = 99,
    V100 = 100, V101 = 101, V102 = 102, V103 = 103, V104 = 104,
    V105 = 105, V106 = 106, V107 = 107, V108 = 108, V109 = 109,
    V110 = 110, V111 = 111, V112 = 112, V113 = 113, V114 = 114,
    V115 = 115, V116 = 116, V117 = 117, V118 = 118, V119 = 119,
    V120 = 120, V121 = 121, V122 = 122, V123 = 123, V124 = 124,
    V125 = 125, V126 = 126, V127 = 127, V128 = 128, V129 = 129,
    V130 = 130, V131 = 131, V132 = 132, V133 = 133, V134 = 134,
    V135 = 135, V136 = 136, V137 = 137, V138 = 138, V139 = 139,
    V140 = 140, V141 = 141, V142 = 142, V143 = 143, V144 = 144,
    V145 = 145, V146 = 146, V147 = 147, V148 = 148, V149 = 149,
    V150 = 150, V151 = 151, V152 = 152, V153 = 153, V154 = 154,
    V155 = 155, V156 = 156, V157 = 157, V158 = 158, V159 = 159,
    V160 = 160, V161 = 161, V162 = 162, V163 = 163, V164 = 164,
    V165 = 165, V166 = 166, V167 = 167, V168 = 168, V169 = 169,
    V170 = 170, V171 = 171, V172 = 172, V173 = 173, V174 = 174,
    V175 = 175, V176 = 176, V177 = 177, V178 = 178, V179 = 179,
    V180 = 180, V181 = 181, V182 = 182, V183 = 183, V184 = 184,
    V185 = 185, V186 = 186, V187 = 187, V188 = 188, V189 = 189,
    V190 = 190, V191 = 191, V192 = 192, V193 = 193, V194 = 194,
    V195 = 195, V196 = 196, V197 = 197, V198 = 198, V199 = 199,
    V200 = 200, V201 = 201, V202 = 202, V203 = 203, V204 = 204,
    V205 = 205, V206 = 206, V207 = 207, V208 = 208, V209 = 209,
    V210 = 210, V211 = 211, V212 = 212, V213 = 213, V214 = 214,
    V215 = 215, V216 = 216, V217 = 217, V218 = 218, V219 = 219,
    V220 = 220, V221 = 221, V222 = 222, V223 = 223, V224 = 224,
    V225 = 225, V226 = 226, V227 = 227, V228 = 228, V229 = 229,
    V230 = 230, V231 = 231, V232 = 232, V233 = 233, V234 = 234,
    V235 = 235, V236 = 236, V237 = 237, V238 = 238, V239 = 239,
    V240 = 240, V241 = 241, V242 = 242, V243 = 243, V244 = 244,
    V245 = 245, V246 = 246, V247 = 247, V248 = 248, V249 = 249,
    V250 = 250, V251 = 251, V252 = 252, V253 = 253, V254 = 254,
    V255 = 255
);

#[test]
fn correctness_enum_256_variants_index_255_needs_multi_byte_tag() {
    // Variant 250 (index 250) fits in 1 byte; variant 255 (index 255) needs 3 bytes.
    let enc_first = encode_to_vec(&E256::V000).expect("encode E256::V000");
    let enc_last = encode_to_vec(&E256::V255).expect("encode E256::V255");

    // Index 0 → 1 byte
    assert_eq!(
        enc_first.len(),
        1,
        "E256::V000 (discriminant 0) must encode as 1 byte, got {}",
        enc_first.len()
    );
    // Index 255 → must be > 1 byte (varint threshold is 250)
    assert!(
        enc_last.len() > 1,
        "E256::V255 (discriminant 255) must need more than 1 byte, got {}",
        enc_last.len()
    );

    // Both must round-trip
    let (dec_first, _): (E256, _) = decode_from_slice(&enc_first).expect("decode E256::V000");
    let (dec_last, _): (E256, _) = decode_from_slice(&enc_last).expect("decode E256::V255");
    assert_eq!(dec_first, E256::V000);
    assert_eq!(dec_last, E256::V255);
}

// ────────────────────────────────────────────────────────────────────────────
// Test 15: Vec of 0 elements encodes to exactly 1 byte (the length prefix)
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_empty_vec_encodes_to_one_byte_length_prefix() {
    let v: Vec<u8> = Vec::new();
    let enc = encode_to_vec(&v).expect("encode empty Vec<u8>");
    assert_eq!(
        enc.len(),
        1,
        "empty Vec<u8> must encode as exactly 1 byte (varint 0), got {}",
        enc.len()
    );
    assert_eq!(enc[0], 0, "the single byte must be 0 (length = 0)");

    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode empty Vec<u8>");
    assert!(dec.is_empty());
    assert_eq!(consumed, 1);
}

// ────────────────────────────────────────────────────────────────────────────
// Test 16: Vec of 65536 elements has a 5-byte length prefix (U32_BYTE range)
//
// Varint thresholds:
//   0..=250        → 1 byte (direct)
//   251..=65535    → 3 bytes (U16_BYTE tag 251 + 2 value bytes)
//   65536..=u32MAX → 5 bytes (U32_BYTE tag 252 + 4 value bytes)
//
// 65536 > u16::MAX so it falls into the U32_BYTE category.
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_vec_65536_elements_has_five_byte_length_prefix() {
    let v: Vec<u8> = vec![0xABu8; 65_536];
    let enc = encode_to_vec(&v).expect("encode Vec of 65536 u8");

    // Length prefix for 65536 needs 5 bytes (U32_BYTE tag 252 + 4 value bytes)
    // Total encoded size = 5 (length prefix) + 65536 (element bytes) = 65541
    assert_eq!(
        enc.len(),
        65_541,
        "Vec<u8> of 65536 elements must encode as 65541 bytes (5 prefix + 65536 data), got {}",
        enc.len()
    );
    // The first byte must be the U32_BYTE tag (252)
    assert_eq!(
        enc[0], 252,
        "length prefix for 65536 must start with U32_BYTE tag (252)"
    );

    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec of 65536 u8");
    assert_eq!(dec.len(), 65_536);
    assert!(dec.iter().all(|&b| b == 0xABu8));
    assert_eq!(consumed, enc.len());
}

// ────────────────────────────────────────────────────────────────────────────
// Test 17: Deeply nested Option — Option<Option<Option<u32>>> round-trips
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_deeply_nested_option_roundtrip() {
    let cases: &[Option<Option<Option<u32>>>] = &[
        None,
        Some(None),
        Some(Some(None)),
        Some(Some(Some(0))),
        Some(Some(Some(u32::MAX))),
    ];

    let mut encodings: Vec<Vec<u8>> = Vec::with_capacity(cases.len());
    for case in cases {
        let enc = encode_to_vec(case).expect("encode Option<Option<Option<u32>>>");
        let (dec, consumed): (Option<Option<Option<u32>>>, _) =
            decode_from_slice(&enc).expect("decode Option<Option<Option<u32>>>");
        assert_eq!(*case, dec, "deeply nested Option must round-trip: {case:?}");
        assert_eq!(
            consumed,
            enc.len(),
            "all bytes must be consumed for case {case:?}"
        );
        encodings.push(enc);
    }

    // All five cases must encode differently
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "Option depth cases {i} and {j} must produce distinct encodings"
            );
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 18: Tuple of 12 elements (std's max PartialEq/Debug arity) round-trips
// ────────────────────────────────────────────────────────────────────────────
type Tuple12 = (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool, bool);

#[test]
fn correctness_tuple_12_elements_roundtrip() {
    // std only provides PartialEq + Debug for tuples up to 12 elements.
    let t: Tuple12 = (
        0u8, 1u16, 2u32, 3u64, 4i8, 5i16, 6i32, 7i64, 8.0f32, 9.0f64, true, false,
    );
    let enc = encode_to_vec(&t).expect("encode 12-tuple");
    let (dec, consumed): (Tuple12, _) = decode_from_slice(&enc).expect("decode 12-tuple");
    // f32/f64 do not implement Eq, so compare field by field.
    assert_eq!(t.0, dec.0, "tuple field 0 (u8) must match");
    assert_eq!(t.1, dec.1, "tuple field 1 (u16) must match");
    assert_eq!(t.2, dec.2, "tuple field 2 (u32) must match");
    assert_eq!(t.3, dec.3, "tuple field 3 (u64) must match");
    assert_eq!(t.4, dec.4, "tuple field 4 (i8) must match");
    assert_eq!(t.5, dec.5, "tuple field 5 (i16) must match");
    assert_eq!(t.6, dec.6, "tuple field 6 (i32) must match");
    assert_eq!(t.7, dec.7, "tuple field 7 (i64) must match");
    assert_eq!(
        t.8.to_bits(),
        dec.8.to_bits(),
        "tuple field 8 (f32) bits must match"
    );
    assert_eq!(
        t.9.to_bits(),
        dec.9.to_bits(),
        "tuple field 9 (f64) bits must match"
    );
    assert_eq!(t.10, dec.10, "tuple field 10 (bool) must match");
    assert_eq!(t.11, dec.11, "tuple field 11 (bool) must match");
    assert_eq!(consumed, enc.len());
}

// ────────────────────────────────────────────────────────────────────────────
// Test 19: i128::MIN and i128::MAX round-trip correctly
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_i128_min_max_roundtrip() {
    for v in [i128::MIN, i128::MAX, 0i128, -1i128, 1i128] {
        let enc = encode_to_vec(&v).expect("encode i128 extreme");
        let (dec, consumed): (i128, _) = decode_from_slice(&enc).expect("decode i128 extreme");
        assert_eq!(v, dec, "i128 value {v} must round-trip");
        assert_eq!(consumed, enc.len());
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 20: u128::MAX round-trips correctly
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_u128_max_roundtrip() {
    let cases: &[u128] = &[0, 1, u64::MAX as u128, u128::MAX];
    for &v in cases {
        let enc = encode_to_vec(&v).expect("encode u128");
        let (dec, consumed): (u128, _) = decode_from_slice(&enc).expect("decode u128");
        assert_eq!(v, dec, "u128 value {v} must round-trip");
        assert_eq!(consumed, enc.len());
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 21: f32 NaN and Infinity encode/decode bit-exact
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_f32_nan_and_infinity_bit_exact() {
    // Use to_bits() for NaN comparison since NaN != NaN.
    let cases: &[f32] = &[
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        -0.0f32,
    ];
    for &v in cases {
        let enc = encode_to_vec(&v).expect("encode f32 special");
        let (dec, consumed): (f32, _) = decode_from_slice(&enc).expect("decode f32 special");
        assert_eq!(
            v.to_bits(),
            dec.to_bits(),
            "f32 bits must be preserved exactly for value with bits={:#010x}",
            v.to_bits()
        );
        assert_eq!(consumed, enc.len());
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 22: f64 NaN and Infinity encode/decode bit-exact
// ────────────────────────────────────────────────────────────────────────────
#[test]
fn correctness_f64_nan_and_infinity_bit_exact() {
    // Use to_bits() for NaN comparison since NaN != NaN.
    let cases: &[f64] = &[
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        -0.0f64,
        0.0f64,
    ];
    for &v in cases {
        let enc = encode_to_vec(&v).expect("encode f64 special");
        let (dec, consumed): (f64, _) = decode_from_slice(&enc).expect("decode f64 special");
        assert_eq!(
            v.to_bits(),
            dec.to_bits(),
            "f64 bits must be preserved exactly for value with bits={:#018x}",
            v.to_bits()
        );
        assert_eq!(consumed, enc.len());
    }
}
