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
use std::sync::Arc;

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
struct SnakeStruct {
    my_field: u32,
    another_field: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CamelStruct {
    my_field: u32,
    another_field: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingStruct {
    my_field: u32,
    another_field: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
struct KebabStruct {
    my_field: u32,
    another_field: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase")]
struct PascalStruct {
    my_field: u32,
    another_field: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "UPPERCASE")]
struct UpperStruct {
    my_field: u32,
    another_field: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "lowercase")]
struct LowerStruct {
    my_field: u32,
    another_field: String,
}

// Test 1: SnakeStruct roundtrip
#[test]
fn test_snake_struct_roundtrip() {
    let original = SnakeStruct {
        my_field: 42,
        another_field: "hello".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode SnakeStruct");
    let (dec, _): (SnakeStruct, _) = decode_from_slice(&enc).expect("decode SnakeStruct");
    assert_eq!(original, dec);
}

// Test 2: CamelStruct roundtrip
#[test]
fn test_camel_struct_roundtrip() {
    let original = CamelStruct {
        my_field: 100,
        another_field: "world".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode CamelStruct");
    let (dec, _): (CamelStruct, _) = decode_from_slice(&enc).expect("decode CamelStruct");
    assert_eq!(original, dec);
}

// Test 3: ScreamingStruct roundtrip
#[test]
fn test_screaming_struct_roundtrip() {
    let original = ScreamingStruct {
        my_field: 7,
        another_field: "screaming".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode ScreamingStruct");
    let (dec, _): (ScreamingStruct, _) = decode_from_slice(&enc).expect("decode ScreamingStruct");
    assert_eq!(original, dec);
}

// Test 4: KebabStruct roundtrip
#[test]
fn test_kebab_struct_roundtrip() {
    let original = KebabStruct {
        my_field: 55,
        another_field: "kebab".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode KebabStruct");
    let (dec, _): (KebabStruct, _) = decode_from_slice(&enc).expect("decode KebabStruct");
    assert_eq!(original, dec);
}

// Test 5: PascalStruct roundtrip
#[test]
fn test_pascal_struct_roundtrip() {
    let original = PascalStruct {
        my_field: 999,
        another_field: "pascal".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode PascalStruct");
    let (dec, _): (PascalStruct, _) = decode_from_slice(&enc).expect("decode PascalStruct");
    assert_eq!(original, dec);
}

// Test 6: UpperStruct roundtrip
#[test]
fn test_upper_struct_roundtrip() {
    let original = UpperStruct {
        my_field: 1,
        another_field: "upper".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode UpperStruct");
    let (dec, _): (UpperStruct, _) = decode_from_slice(&enc).expect("decode UpperStruct");
    assert_eq!(original, dec);
}

// Test 7: LowerStruct roundtrip
#[test]
fn test_lower_struct_roundtrip() {
    let original = LowerStruct {
        my_field: 2,
        another_field: "lower".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode LowerStruct");
    let (dec, _): (LowerStruct, _) = decode_from_slice(&enc).expect("decode LowerStruct");
    assert_eq!(original, dec);
}

// Test 8: SnakeStruct and CamelStruct with SAME field values encode to SAME bytes
// (rename_all is cosmetic in binary format — no effect on wire encoding)
#[test]
fn test_snake_and_camel_same_bytes() {
    let snake = SnakeStruct {
        my_field: 123,
        another_field: "test".to_string(),
    };
    let camel = CamelStruct {
        my_field: 123,
        another_field: "test".to_string(),
    };
    let snake_enc = encode_to_vec(&snake).expect("encode snake");
    let camel_enc = encode_to_vec(&camel).expect("encode camel");
    assert_eq!(
        snake_enc, camel_enc,
        "rename_all must not affect binary wire format"
    );
}

// Test 9: SnakeStruct and PascalStruct encode identically
#[test]
fn test_snake_and_pascal_same_bytes() {
    let snake = SnakeStruct {
        my_field: 77,
        another_field: "oxicode".to_string(),
    };
    let pascal = PascalStruct {
        my_field: 77,
        another_field: "oxicode".to_string(),
    };
    let snake_enc = encode_to_vec(&snake).expect("encode snake");
    let pascal_enc = encode_to_vec(&pascal).expect("encode pascal");
    assert_eq!(
        snake_enc, pascal_enc,
        "SnakeStruct and PascalStruct must produce identical bytes"
    );
}

// Test 10: Vec<SnakeStruct> roundtrip
#[test]
fn test_vec_snake_struct_roundtrip() {
    let original = vec![
        SnakeStruct {
            my_field: 1,
            another_field: "a".to_string(),
        },
        SnakeStruct {
            my_field: 2,
            another_field: "b".to_string(),
        },
        SnakeStruct {
            my_field: 3,
            another_field: "c".to_string(),
        },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<SnakeStruct>");
    let (dec, _): (Vec<SnakeStruct>, _) = decode_from_slice(&enc).expect("decode Vec<SnakeStruct>");
    assert_eq!(original, dec);
}

// Test 11: Option<CamelStruct> Some roundtrip
#[test]
fn test_option_camel_struct_some_roundtrip() {
    let original: Option<CamelStruct> = Some(CamelStruct {
        my_field: 500,
        another_field: "some_value".to_string(),
    });
    let enc = encode_to_vec(&original).expect("encode Option<CamelStruct> Some");
    let (dec, _): (Option<CamelStruct>, _) =
        decode_from_slice(&enc).expect("decode Option<CamelStruct> Some");
    assert_eq!(original, dec);
}

// Test 12: Option<ScreamingStruct> None roundtrip
#[test]
fn test_option_screaming_struct_none_roundtrip() {
    let original: Option<ScreamingStruct> = None;
    let enc = encode_to_vec(&original).expect("encode Option<ScreamingStruct> None");
    let (dec, _): (Option<ScreamingStruct>, _) =
        decode_from_slice(&enc).expect("decode Option<ScreamingStruct> None");
    assert_eq!(original, dec);
}

// Test 13: SnakeStruct re-encoding gives same bytes
#[test]
fn test_snake_struct_reencoding_stable() {
    let original = SnakeStruct {
        my_field: 314,
        another_field: "stable".to_string(),
    };
    let enc1 = encode_to_vec(&original).expect("first encode");
    let (decoded, _): (SnakeStruct, _) = decode_from_slice(&enc1).expect("decode");
    let enc2 = encode_to_vec(&decoded).expect("second encode");
    assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
}

// Test 14: CamelStruct decoded value has original field names accessible
#[test]
fn test_camel_struct_field_names_accessible() {
    let original = CamelStruct {
        my_field: 888,
        another_field: "accessible".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode CamelStruct");
    let (dec, _): (CamelStruct, _) = decode_from_slice(&enc).expect("decode CamelStruct");
    // Access fields by their Rust names (not by any renamed form)
    assert_eq!(dec.my_field, 888);
    assert_eq!(dec.another_field, "accessible");
}

// Test 15: All 7 struct types with same field values produce identical bytes
#[test]
fn test_all_rename_all_styles_produce_identical_bytes() {
    let field_val: u32 = 42;
    let str_val = "identical".to_string();

    let snake_enc = encode_to_vec(&SnakeStruct {
        my_field: field_val,
        another_field: str_val.clone(),
    })
    .expect("encode snake");

    let camel_enc = encode_to_vec(&CamelStruct {
        my_field: field_val,
        another_field: str_val.clone(),
    })
    .expect("encode camel");

    let screaming_enc = encode_to_vec(&ScreamingStruct {
        my_field: field_val,
        another_field: str_val.clone(),
    })
    .expect("encode screaming");

    let kebab_enc = encode_to_vec(&KebabStruct {
        my_field: field_val,
        another_field: str_val.clone(),
    })
    .expect("encode kebab");

    let pascal_enc = encode_to_vec(&PascalStruct {
        my_field: field_val,
        another_field: str_val.clone(),
    })
    .expect("encode pascal");

    let upper_enc = encode_to_vec(&UpperStruct {
        my_field: field_val,
        another_field: str_val.clone(),
    })
    .expect("encode upper");

    let lower_enc = encode_to_vec(&LowerStruct {
        my_field: field_val,
        another_field: str_val.clone(),
    })
    .expect("encode lower");

    assert_eq!(snake_enc, camel_enc, "snake vs camel");
    assert_eq!(snake_enc, screaming_enc, "snake vs screaming");
    assert_eq!(snake_enc, kebab_enc, "snake vs kebab");
    assert_eq!(snake_enc, pascal_enc, "snake vs pascal");
    assert_eq!(snake_enc, upper_enc, "snake vs upper");
    assert_eq!(snake_enc, lower_enc, "snake vs lower");
}

// Test 16: Vec<CamelStruct> with 3 elements roundtrip
#[test]
fn test_vec_camel_struct_three_elements_roundtrip() {
    let original = vec![
        CamelStruct {
            my_field: 10,
            another_field: "alpha".to_string(),
        },
        CamelStruct {
            my_field: 20,
            another_field: "beta".to_string(),
        },
        CamelStruct {
            my_field: 30,
            another_field: "gamma".to_string(),
        },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<CamelStruct>");
    let (dec, _): (Vec<CamelStruct>, _) = decode_from_slice(&enc).expect("decode Vec<CamelStruct>");
    assert_eq!(original, dec);
}

// Test 17: Enum with rename_all "camelCase" — all variants roundtrip
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)]
enum RenameAllEnum {
    FirstVariant,
    SecondVariant(u32),
    ThirdVariant { payload: String },
}

#[test]
fn test_enum_rename_all_camel_case_all_variants() {
    let cases = vec![
        RenameAllEnum::FirstVariant,
        RenameAllEnum::SecondVariant(99),
        RenameAllEnum::ThirdVariant {
            payload: "enum_test".to_string(),
        },
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode RenameAllEnum variant");
        let (dec, _): (RenameAllEnum, _) =
            decode_from_slice(&enc).expect("decode RenameAllEnum variant");
        assert_eq!(case, &dec);
    }
}

// Test 18: SnakeStruct with u32::MAX and long string roundtrip
#[test]
fn test_snake_struct_max_u32_and_long_string() {
    let long_str: String = "x".repeat(10_000);
    let original = SnakeStruct {
        my_field: u32::MAX,
        another_field: long_str,
    };
    let enc = encode_to_vec(&original).expect("encode SnakeStruct with max u32 + long string");
    let (dec, _): (SnakeStruct, _) =
        decode_from_slice(&enc).expect("decode SnakeStruct with max u32 + long string");
    assert_eq!(original, dec);
}

// Test 19: Box<CamelStruct> roundtrip
#[test]
fn test_box_camel_struct_roundtrip() {
    let original = Box::new(CamelStruct {
        my_field: 256,
        another_field: "boxed".to_string(),
    });
    let enc = encode_to_vec(&original).expect("encode Box<CamelStruct>");
    let (dec, _): (Box<CamelStruct>, _) = decode_from_slice(&enc).expect("decode Box<CamelStruct>");
    assert_eq!(original, dec);
}

// Test 20: Arc<ScreamingStruct> roundtrip
#[test]
fn test_arc_screaming_struct_roundtrip() {
    let original = Arc::new(ScreamingStruct {
        my_field: 512,
        another_field: "arc_value".to_string(),
    });
    let enc = encode_to_vec(&original).expect("encode Arc<ScreamingStruct>");
    let (dec, _): (Arc<ScreamingStruct>, _) =
        decode_from_slice(&enc).expect("decode Arc<ScreamingStruct>");
    assert_eq!(original, dec);
}

// Test 21: KebabStruct with field values 0 and empty string roundtrip
#[test]
fn test_kebab_struct_zero_and_empty_string() {
    let original = KebabStruct {
        my_field: 0,
        another_field: String::new(),
    };
    let enc = encode_to_vec(&original).expect("encode KebabStruct with zeros");
    let (dec, _): (KebabStruct, _) =
        decode_from_slice(&enc).expect("decode KebabStruct with zeros");
    assert_eq!(original, dec);
}

// Test 22: PascalStruct consumed bytes equals encoded length
#[test]
fn test_pascal_struct_consumed_bytes_equals_encoded_length() {
    let original = PascalStruct {
        my_field: 111,
        another_field: "length_check".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode PascalStruct");
    let (_, consumed): (PascalStruct, usize) =
        decode_from_slice(&enc).expect("decode PascalStruct");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}
