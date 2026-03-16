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
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Matrix2x2 {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Token {
    Number(i64),
    Text(String),
    Bool(bool),
    Null,
}

fn arb_matrix2x2() -> impl Strategy<Value = Matrix2x2> {
    (any::<u64>(), any::<u64>(), any::<u64>(), any::<u64>()).prop_map(|(a, b, c, d)| Matrix2x2 {
        a: f64::from_bits(a),
        b: f64::from_bits(b),
        c: f64::from_bits(c),
        d: f64::from_bits(d),
    })
}

fn arb_token() -> impl Strategy<Value = Token> {
    prop_oneof![
        any::<i64>().prop_map(Token::Number),
        proptest::string::string_regex("[a-zA-Z0-9 ]{0,50}")
            .expect("regex failed")
            .prop_map(Token::Text),
        any::<bool>().prop_map(Token::Bool),
        Just(Token::Null),
    ]
}

// 1. Matrix2x2 bit-exact f64 roundtrip
#[test]
fn test_matrix2x2_bit_exact_f64_roundtrip() {
    proptest!(|(m in arb_matrix2x2())| {
        let encoded = encode_to_vec(&m).expect("encode Matrix2x2 failed");
        let (decoded, _) = decode_from_slice::<Matrix2x2>(&encoded).expect("decode Matrix2x2 failed");
        prop_assert_eq!(m.a.to_bits(), decoded.a.to_bits());
        prop_assert_eq!(m.b.to_bits(), decoded.b.to_bits());
        prop_assert_eq!(m.c.to_bits(), decoded.c.to_bits());
        prop_assert_eq!(m.d.to_bits(), decoded.d.to_bits());
    });
}

// 2. Token::Number(i64) roundtrip
#[test]
fn test_token_number_i64_roundtrip() {
    proptest!(|(val: i64)| {
        let token = Token::Number(val);
        let encoded = encode_to_vec(&token).expect("encode Token::Number failed");
        let (decoded, _) = decode_from_slice::<Token>(&encoded).expect("decode Token::Number failed");
        prop_assert_eq!(token, decoded);
    });
}

// 3. Token::Text(String) roundtrip (0..50 chars)
#[test]
fn test_token_text_string_roundtrip() {
    proptest!(|(s in proptest::string::string_regex("[a-zA-Z0-9 ]{0,50}").expect("regex failed"))| {
        let token = Token::Text(s);
        let encoded = encode_to_vec(&token).expect("encode Token::Text failed");
        let (decoded, _) = decode_from_slice::<Token>(&encoded).expect("decode Token::Text failed");
        prop_assert_eq!(token, decoded);
    });
}

// 4. Token::Bool(bool) roundtrip (dummy u8 parameter)
#[test]
fn test_token_bool_roundtrip() {
    proptest!(|(_dummy: u8, val: bool)| {
        let token = Token::Bool(val);
        let encoded = encode_to_vec(&token).expect("encode Token::Bool failed");
        let (decoded, _) = decode_from_slice::<Token>(&encoded).expect("decode Token::Bool failed");
        prop_assert_eq!(token, decoded);
    });
}

// 5. Vec<Token> roundtrip (0..5 elements)
#[test]
fn test_vec_token_roundtrip() {
    proptest!(|(tokens in proptest::collection::vec(arb_token(), 0..5))| {
        let encoded = encode_to_vec(&tokens).expect("encode Vec<Token> failed");
        let (decoded, _) = decode_from_slice::<Vec<Token>>(&encoded).expect("decode Vec<Token> failed");
        prop_assert_eq!(tokens.len(), decoded.len());
        for (a, b) in tokens.iter().zip(decoded.iter()) {
            prop_assert_eq!(a, b);
        }
    });
}

// 6. BTreeMap<i32, i32> roundtrip (0..10 entries)
#[test]
fn test_btreemap_i32_i32_roundtrip() {
    proptest!(|(entries in proptest::collection::vec((any::<i32>(), any::<i32>()), 0..10))| {
        let map: BTreeMap<i32, i32> = entries.into_iter().collect();
        let encoded = encode_to_vec(&map).expect("encode BTreeMap<i32,i32> failed");
        let (decoded, _) = decode_from_slice::<BTreeMap<i32, i32>>(&encoded).expect("decode BTreeMap<i32,i32> failed");
        prop_assert_eq!(map, decoded);
    });
}

// 7. (i32, i32, i32) triple roundtrip
#[test]
fn test_i32_triple_roundtrip() {
    proptest!(|(a: i32, b: i32, c: i32)| {
        let triple = (a, b, c);
        let encoded = encode_to_vec(&triple).expect("encode (i32,i32,i32) failed");
        let (decoded, _) = decode_from_slice::<(i32, i32, i32)>(&encoded).expect("decode (i32,i32,i32) failed");
        prop_assert_eq!(triple, decoded);
    });
}

// 8. [i64; 2] fixed array roundtrip
#[test]
fn test_fixed_array_i64_2_roundtrip() {
    proptest!(|(arr: [i64; 2])| {
        let encoded = encode_to_vec(&arr).expect("encode [i64;2] failed");
        let (decoded, _) = decode_from_slice::<[i64; 2]>(&encoded).expect("decode [i64;2] failed");
        prop_assert_eq!(arr, decoded);
    });
}

// 9. [f64; 4] fixed array bit-exact roundtrip
#[test]
fn test_fixed_array_f64_4_bit_exact_roundtrip() {
    proptest!(|(bits: [u64; 4])| {
        let arr: [f64; 4] = [
            f64::from_bits(bits[0]),
            f64::from_bits(bits[1]),
            f64::from_bits(bits[2]),
            f64::from_bits(bits[3]),
        ];
        let encoded = encode_to_vec(&arr).expect("encode [f64;4] failed");
        let (decoded, _) = decode_from_slice::<[f64; 4]>(&encoded).expect("decode [f64;4] failed");
        for i in 0..4 {
            prop_assert_eq!(arr[i].to_bits(), decoded[i].to_bits());
        }
    });
}

// 10. i32, i64, i128 same sign same zigzag size invariant
#[test]
fn test_i32_i64_i128_same_sign_same_zigzag_size_invariant() {
    proptest!(|(small: U6)| {
        // Use a small magnitude value that fits in all three types and has same zigzag encoding size
        let v = small.0 as i32;
        let v64 = v as i64;
        let v128 = v as i128;
        let enc32 = encode_to_vec(&v).expect("encode i32 failed");
        let enc64 = encode_to_vec(&v64).expect("encode i64 failed");
        let enc128 = encode_to_vec(&v128).expect("encode i128 failed");
        // All small magnitude values (same zigzag representation) should have the same encoded length
        prop_assert_eq!(enc32.len(), enc64.len());
        prop_assert_eq!(enc32.len(), enc128.len());
    });
}

// 11. Vec<Matrix2x2> roundtrip (0..3 elements)
#[test]
fn test_vec_matrix2x2_roundtrip() {
    proptest!(|(matrices in proptest::collection::vec(arb_matrix2x2(), 0..3))| {
        let encoded = encode_to_vec(&matrices).expect("encode Vec<Matrix2x2> failed");
        let (decoded, _) = decode_from_slice::<Vec<Matrix2x2>>(&encoded).expect("decode Vec<Matrix2x2> failed");
        prop_assert_eq!(matrices.len(), decoded.len());
        for (orig, dec) in matrices.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig.a.to_bits(), dec.a.to_bits());
            prop_assert_eq!(orig.b.to_bits(), dec.b.to_bits());
            prop_assert_eq!(orig.c.to_bits(), dec.c.to_bits());
            prop_assert_eq!(orig.d.to_bits(), dec.d.to_bits());
        }
    });
}

// 12. Option<Matrix2x2> roundtrip
#[test]
fn test_option_matrix2x2_roundtrip() {
    proptest!(|(m in arb_matrix2x2())| {
        let opt: Option<Matrix2x2> = Some(m.clone());
        let encoded = encode_to_vec(&opt).expect("encode Option<Matrix2x2> failed");
        let (decoded, _) = decode_from_slice::<Option<Matrix2x2>>(&encoded).expect("decode Option<Matrix2x2> failed");
        let dec = decoded.expect("expected Some(Matrix2x2) but got None");
        prop_assert_eq!(m.a.to_bits(), dec.a.to_bits());
        prop_assert_eq!(m.b.to_bits(), dec.b.to_bits());
        prop_assert_eq!(m.c.to_bits(), dec.c.to_bits());
        prop_assert_eq!(m.d.to_bits(), dec.d.to_bits());
    });
}

// 13. Result<Matrix2x2, String> Ok roundtrip
#[test]
fn test_result_ok_matrix2x2_roundtrip() {
    proptest!(|(m in arb_matrix2x2())| {
        let result: Result<Matrix2x2, String> = Ok(m.clone());
        let encoded = encode_to_vec(&result).expect("encode Result<Matrix2x2,String> failed");
        let (decoded, _) = decode_from_slice::<Result<Matrix2x2, String>>(&encoded).expect("decode Result<Matrix2x2,String> failed");
        match decoded {
            Ok(dec) => {
                prop_assert_eq!(m.a.to_bits(), dec.a.to_bits());
                prop_assert_eq!(m.b.to_bits(), dec.b.to_bits());
                prop_assert_eq!(m.c.to_bits(), dec.c.to_bits());
                prop_assert_eq!(m.d.to_bits(), dec.d.to_bits());
            },
            Err(_) => return Err(TestCaseError::fail("expected Ok but got Err")),
        }
    });
}

// 14. (Token, Token) pair roundtrip
#[test]
fn test_token_pair_roundtrip() {
    proptest!(|(a in arb_token(), b in arb_token())| {
        let pair = (a.clone(), b.clone());
        let encoded = encode_to_vec(&pair).expect("encode (Token,Token) failed");
        let (decoded, _) = decode_from_slice::<(Token, Token)>(&encoded).expect("decode (Token,Token) failed");
        prop_assert_eq!(pair.0, decoded.0);
        prop_assert_eq!(pair.1, decoded.1);
    });
}

// 15. BTreeMap<String, Token> roundtrip (0..5 entries)
#[test]
fn test_btreemap_string_token_roundtrip() {
    let key_strat = proptest::string::string_regex("[a-z]{1,10}").expect("regex failed");
    proptest!(|(entries in proptest::collection::vec((key_strat, arb_token()), 0..5))| {
        let map: BTreeMap<String, Token> = entries.into_iter().collect();
        let encoded = encode_to_vec(&map).expect("encode BTreeMap<String,Token> failed");
        let (decoded, _) = decode_from_slice::<BTreeMap<String, Token>>(&encoded).expect("decode BTreeMap<String,Token> failed");
        prop_assert_eq!(map.len(), decoded.len());
        for (k, v) in &map {
            let dv = decoded.get(k).expect("key missing after decode");
            prop_assert_eq!(v, dv);
        }
    });
}

// 16. Re-encoding Token gives same bytes
#[test]
fn test_token_reencode_gives_same_bytes() {
    proptest!(|(token in arb_token())| {
        let encoded1 = encode_to_vec(&token).expect("first encode Token failed");
        let (decoded, _) = decode_from_slice::<Token>(&encoded1).expect("decode Token for reencode failed");
        let encoded2 = encode_to_vec(&decoded).expect("second encode Token failed");
        prop_assert_eq!(encoded1, encoded2);
    });
}

// 17. i8 roundtrip using arbitrary i8
#[test]
fn test_i8_roundtrip() {
    proptest!(|(val: i8)| {
        let encoded = encode_to_vec(&val).expect("encode i8 failed");
        let (decoded, _) = decode_from_slice::<i8>(&encoded).expect("decode i8 failed");
        prop_assert_eq!(val, decoded);
    });
}

// 18. u8 roundtrip — always 1 byte
#[test]
fn test_u8_roundtrip_always_one_byte() {
    proptest!(|(val: u8)| {
        let encoded = encode_to_vec(&val).expect("encode u8 failed");
        prop_assert_eq!(encoded.len(), 1usize, "u8 must encode to exactly 1 byte");
        let (decoded, _) = decode_from_slice::<u8>(&encoded).expect("decode u8 failed");
        prop_assert_eq!(val, decoded);
    });
}

// 19. Vec<i32> roundtrip (0..20 elements)
#[test]
fn test_vec_i32_roundtrip() {
    proptest!(|(values in proptest::collection::vec(any::<i32>(), 0..20))| {
        let encoded = encode_to_vec(&values).expect("encode Vec<i32> failed");
        let (decoded, _) = decode_from_slice::<Vec<i32>>(&encoded).expect("decode Vec<i32> failed");
        prop_assert_eq!(values, decoded);
    });
}

// 20. Vec<f64> bit-exact roundtrip (0..10)
#[test]
fn test_vec_f64_bit_exact_roundtrip() {
    proptest!(|(bit_values in proptest::collection::vec(any::<u64>(), 0..10))| {
        let values: Vec<f64> = bit_values.iter().map(|&b| f64::from_bits(b)).collect();
        let encoded = encode_to_vec(&values).expect("encode Vec<f64> failed");
        let (decoded, _) = decode_from_slice::<Vec<f64>>(&encoded).expect("decode Vec<f64> failed");
        prop_assert_eq!(values.len(), decoded.len());
        for (orig, dec) in values.iter().zip(decoded.iter()) {
            prop_assert_eq!(orig.to_bits(), dec.to_bits());
        }
    });
}

// 21. (u64, String, bool, u32) 4-tuple roundtrip
#[test]
fn test_u64_string_bool_u32_4tuple_roundtrip() {
    proptest!(|(
        n: u64,
        s in proptest::string::string_regex("[a-zA-Z0-9]{0,30}").expect("regex failed"),
        b: bool,
        m: u32
    )| {
        let tuple = (n, s, b, m);
        let encoded = encode_to_vec(&tuple).expect("encode (u64,String,bool,u32) failed");
        let (decoded, _) = decode_from_slice::<(u64, String, bool, u32)>(&encoded).expect("decode (u64,String,bool,u32) failed");
        prop_assert_eq!(tuple, decoded);
    });
}

// 22. Consumed bytes equals encoded length for Matrix2x2
#[test]
fn test_consumed_bytes_equals_encoded_length_matrix2x2() {
    proptest!(|(m in arb_matrix2x2())| {
        let encoded = encode_to_vec(&m).expect("encode Matrix2x2 for consumed bytes test failed");
        let expected_len = encoded.len();
        let (_, consumed) = decode_from_slice::<Matrix2x2>(&encoded).expect("decode Matrix2x2 for consumed bytes test failed");
        prop_assert_eq!(expected_len, consumed);
    });
}

/// Newtype wrapper for u8 values in range 0..=63 (6-bit), used for small-magnitude zigzag tests.
#[derive(Debug, Clone, Copy)]
struct U6(u8);

impl Arbitrary for U6 {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (0u8..=63u8).prop_map(U6).boxed()
    }
}
