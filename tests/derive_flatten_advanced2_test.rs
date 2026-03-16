//! Advanced tests for #[oxicode(flatten)] — 22 top-level scenarios (set 2).
//!
//! Covers: Extended/Base roundtrips, wire-format byte verification, multiple
//! flattened fields, configs (fixed-int, big-endian), Vec/Option containers,
//! field-value access, max-value boundaries, skip-inside-flatten, three-level
//! nesting, and re-encode identity.

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

// ---------------------------------------------------------------------------
// Shared module-level type definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Base {
    id: u32,
    name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Extended {
    #[oxicode(flatten)]
    base: Base,
    extra: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TwoFlattened {
    prefix: u8,
    #[oxicode(flatten)]
    first: Base,
    #[oxicode(flatten)]
    second: Base,
    suffix: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Labeled {
    label: String,
    #[oxicode(flatten)]
    coords: Point,
}

// ---------------------------------------------------------------------------
// Test 1: Extended struct roundtrip (flatten base into extended)
// ---------------------------------------------------------------------------

#[test]
fn test_extended_roundtrip() {
    let value = Extended {
        base: Base {
            id: 1,
            name: "Alice".to_string(),
        },
        extra: 999,
    };
    let encoded = encode_to_vec(&value).expect("encode Extended");
    let (decoded, _): (Extended, usize) = decode_from_slice(&encoded).expect("decode Extended");
    assert_eq!(decoded.base.id, value.base.id);
    assert_eq!(decoded.base.name, value.base.name);
    assert_eq!(decoded.extra, value.extra);
}

// ---------------------------------------------------------------------------
// Test 2: Extended struct — wire bytes equal Base fields then extra field
// ---------------------------------------------------------------------------

#[test]
fn test_extended_wire_bytes_equal_base_then_extra() {
    // Build what the wire should look like: encode Base fields then the extra u64.
    // With the default (standard) config the fields appear in declaration order
    // without any struct-level framing.  We verify that the encoded Extended is
    // byte-for-byte identical to a manually-inlined equivalent.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ManualInlined {
        id: u32,
        name: String,
        extra: u64,
    }

    let value = Extended {
        base: Base {
            id: 42,
            name: "Bob".to_string(),
        },
        extra: 7,
    };
    let inlined = ManualInlined {
        id: 42,
        name: "Bob".to_string(),
        extra: 7,
    };

    let enc_extended = encode_to_vec(&value).expect("encode Extended");
    let enc_inlined = encode_to_vec(&inlined).expect("encode ManualInlined");

    assert_eq!(
        enc_extended, enc_inlined,
        "flatten must produce the same bytes as a manually inlined struct"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Extended struct — consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_extended_consumed_equals_encoded_length() {
    let value = Extended {
        base: Base {
            id: 5,
            name: "Test".to_string(),
        },
        extra: 1234,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (_, consumed): (Extended, usize) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Extended with fixed-int config
// ---------------------------------------------------------------------------

#[test]
fn test_extended_with_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let value = Extended {
        base: Base {
            id: 100,
            name: "cfg".to_string(),
        },
        extra: 200,
    };
    let encoded = encode_to_vec_with_config(&value, cfg).expect("encode with fixed-int");
    let (decoded, consumed): (Extended, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed-int");
    assert_eq!(decoded.base.id, 100);
    assert_eq!(decoded.base.name, "cfg");
    assert_eq!(decoded.extra, 200);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Extended with big-endian config
// ---------------------------------------------------------------------------

#[test]
fn test_extended_with_big_endian_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value = Extended {
        base: Base {
            id: 0x0102_0304_u32,
            name: "be".to_string(),
        },
        extra: 0x0506_0708_090A_0B0C_u64,
    };
    let encoded = encode_to_vec_with_config(&value, cfg).expect("encode big-endian");
    let (decoded, consumed): (Extended, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian");
    assert_eq!(decoded.base.id, 0x0102_0304_u32);
    assert_eq!(decoded.extra, 0x0506_0708_090A_0B0C_u64);
    assert_eq!(consumed, encoded.len());
    // Verify big-endian byte order: first 4 bytes of the encoded id field.
    assert_eq!(encoded[0], 0x01);
    assert_eq!(encoded[1], 0x02);
    assert_eq!(encoded[2], 0x03);
    assert_eq!(encoded[3], 0x04);
}

// ---------------------------------------------------------------------------
// Test 6: TwoFlattened struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_two_flattened_roundtrip() {
    let value = TwoFlattened {
        prefix: 0xAB,
        first: Base {
            id: 1,
            name: "first".to_string(),
        },
        second: Base {
            id: 2,
            name: "second".to_string(),
        },
        suffix: 0xCD,
    };
    let encoded = encode_to_vec(&value).expect("encode TwoFlattened");
    let (decoded, consumed): (TwoFlattened, usize) =
        decode_from_slice(&encoded).expect("decode TwoFlattened");
    assert_eq!(decoded.prefix, 0xAB);
    assert_eq!(decoded.first.id, 1);
    assert_eq!(decoded.first.name, "first");
    assert_eq!(decoded.second.id, 2);
    assert_eq!(decoded.second.name, "second");
    assert_eq!(decoded.suffix, 0xCD);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Labeled (flatten Point) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_labeled_flatten_point_roundtrip() {
    let value = Labeled {
        label: "origin".to_string(),
        coords: Point { x: 1.5, y: 2.5 },
    };
    let encoded = encode_to_vec(&value).expect("encode Labeled");
    let (decoded, consumed): (Labeled, usize) =
        decode_from_slice(&encoded).expect("decode Labeled");
    assert_eq!(decoded.label, "origin");
    assert_eq!(decoded.coords.x, 1.5);
    assert_eq!(decoded.coords.y, 2.5);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Vec<Extended> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_extended_roundtrip() {
    let values: Vec<Extended> = vec![
        Extended {
            base: Base {
                id: 10,
                name: "a".to_string(),
            },
            extra: 100,
        },
        Extended {
            base: Base {
                id: 20,
                name: "b".to_string(),
            },
            extra: 200,
        },
        Extended {
            base: Base {
                id: 30,
                name: "c".to_string(),
            },
            extra: 300,
        },
    ];
    let encoded = encode_to_vec(&values).expect("encode Vec<Extended>");
    let (decoded, consumed): (Vec<Extended>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Extended>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].base.id, 10);
    assert_eq!(decoded[1].extra, 200);
    assert_eq!(decoded[2].base.name, "c");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Option<Extended> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_extended_some_roundtrip() {
    let value: Option<Extended> = Some(Extended {
        base: Base {
            id: 77,
            name: "some".to_string(),
        },
        extra: 888,
    });
    let encoded = encode_to_vec(&value).expect("encode Some(Extended)");
    let (decoded, consumed): (Option<Extended>, usize) =
        decode_from_slice(&encoded).expect("decode Some(Extended)");
    let inner = decoded.expect("expected Some");
    assert_eq!(inner.base.id, 77);
    assert_eq!(inner.base.name, "some");
    assert_eq!(inner.extra, 888);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Option<Extended> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_extended_none_roundtrip() {
    let value: Option<Extended> = None;
    let encoded = encode_to_vec(&value).expect("encode None");
    let (decoded, consumed): (Option<Extended>, usize) =
        decode_from_slice(&encoded).expect("decode None");
    assert!(decoded.is_none(), "decoded Option must be None");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Extended decoded values match original base fields
// ---------------------------------------------------------------------------

#[test]
fn test_extended_decoded_values_match_base_fields() {
    let original_base = Base {
        id: 55,
        name: "match_check".to_string(),
    };
    let value = Extended {
        base: original_base.clone(),
        extra: 123,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Extended, usize) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(
        decoded.base, original_base,
        "decoded base must match original base"
    );
    assert_eq!(decoded.extra, 123);
}

// ---------------------------------------------------------------------------
// Test 12: Flatten preserves field order in encoding
// ---------------------------------------------------------------------------

#[test]
fn test_flatten_preserves_field_order() {
    // Use fixed-int encoding so byte boundaries are predictable.
    // TwoFlattened layout: prefix(u8) | first.id(u32) | first.name | second.id(u32) | second.name | suffix(u8)
    // We use single-char names to keep string lengths deterministic.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct OrderedInlined {
        prefix: u8,
        first_id: u32,
        first_name: String,
        second_id: u32,
        second_name: String,
        suffix: u8,
    }

    let flat_value = TwoFlattened {
        prefix: 0x01,
        first: Base {
            id: 11,
            name: "X".to_string(),
        },
        second: Base {
            id: 22,
            name: "Y".to_string(),
        },
        suffix: 0x02,
    };
    let inlined_value = OrderedInlined {
        prefix: 0x01,
        first_id: 11,
        first_name: "X".to_string(),
        second_id: 22,
        second_name: "Y".to_string(),
        suffix: 0x02,
    };

    let enc_flat = encode_to_vec(&flat_value).expect("encode TwoFlattened");
    let enc_inlined = encode_to_vec(&inlined_value).expect("encode OrderedInlined");

    assert_eq!(
        enc_flat, enc_inlined,
        "field order must be preserved across two consecutive flatten attributes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Extended struct with all-zero values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_extended_all_zero_values_roundtrip() {
    let value = Extended {
        base: Base {
            id: 0,
            name: String::new(),
        },
        extra: 0,
    };
    let encoded = encode_to_vec(&value).expect("encode zero Extended");
    let (decoded, consumed): (Extended, usize) =
        decode_from_slice(&encoded).expect("decode zero Extended");
    assert_eq!(decoded.base.id, 0);
    assert!(decoded.base.name.is_empty());
    assert_eq!(decoded.extra, 0);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Extended struct with max values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_extended_max_values_roundtrip() {
    let value = Extended {
        base: Base {
            id: u32::MAX,
            name: "max".repeat(100),
        },
        extra: u64::MAX,
    };
    let encoded = encode_to_vec(&value).expect("encode max Extended");
    let (decoded, consumed): (Extended, usize) =
        decode_from_slice(&encoded).expect("decode max Extended");
    assert_eq!(decoded.base.id, u32::MAX);
    assert_eq!(decoded.base.name, "max".repeat(100));
    assert_eq!(decoded.extra, u64::MAX);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Labeled struct decoded label correct
// ---------------------------------------------------------------------------

#[test]
fn test_labeled_decoded_label_correct() {
    let value = Labeled {
        label: "point-label".to_string(),
        coords: Point { x: 3.0, y: 4.0 },
    };
    let encoded = encode_to_vec(&value).expect("encode Labeled");
    let (decoded, _): (Labeled, usize) = decode_from_slice(&encoded).expect("decode Labeled");
    assert_eq!(
        decoded.label, "point-label",
        "decoded label must match original"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Labeled struct decoded coords correct
// ---------------------------------------------------------------------------

#[test]
fn test_labeled_decoded_coords_correct() {
    let value = Labeled {
        label: "coords-check".to_string(),
        coords: Point { x: 10.25, y: -5.75 },
    };
    let encoded = encode_to_vec(&value).expect("encode Labeled");
    let (decoded, _): (Labeled, usize) = decode_from_slice(&encoded).expect("decode Labeled");
    assert_eq!(
        decoded.coords.x, 10.25,
        "x coordinate must round-trip exactly"
    );
    assert_eq!(
        decoded.coords.y, -5.75,
        "y coordinate must round-trip exactly"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Multiple Extended instances encode independently
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_extended_encode_independently() {
    let a = Extended {
        base: Base {
            id: 1,
            name: "a".to_string(),
        },
        extra: 10,
    };
    let b = Extended {
        base: Base {
            id: 2,
            name: "b".to_string(),
        },
        extra: 20,
    };

    let enc_a = encode_to_vec(&a).expect("encode a");
    let enc_b = encode_to_vec(&b).expect("encode b");

    // Different instances must produce different bytes.
    assert_ne!(
        enc_a, enc_b,
        "distinct Extended instances must not encode identically"
    );

    // Each must decode independently to the correct value.
    let (dec_a, consumed_a): (Extended, usize) = decode_from_slice(&enc_a).expect("decode a");
    let (dec_b, consumed_b): (Extended, usize) = decode_from_slice(&enc_b).expect("decode b");

    assert_eq!(dec_a.base.id, 1);
    assert_eq!(dec_b.base.id, 2);
    assert_eq!(dec_a.extra, 10);
    assert_eq!(dec_b.extra, 20);
    assert_eq!(consumed_a, enc_a.len());
    assert_eq!(consumed_b, enc_b.len());
}

// ---------------------------------------------------------------------------
// Test 18: Extended struct with different base values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_extended_different_base_values_roundtrip() {
    let values = [
        Extended {
            base: Base {
                id: 0,
                name: "zero".to_string(),
            },
            extra: 0,
        },
        Extended {
            base: Base {
                id: 1,
                name: "one".to_string(),
            },
            extra: 1,
        },
        Extended {
            base: Base {
                id: 1000,
                name: "thousand".to_string(),
            },
            extra: 1000,
        },
        Extended {
            base: Base {
                id: u32::MAX / 2,
                name: "half-max".to_string(),
            },
            extra: u64::MAX / 2,
        },
    ];

    for (idx, value) in values.iter().enumerate() {
        let encoded = encode_to_vec(value).expect("encode");
        let (decoded, consumed): (Extended, usize) = decode_from_slice(&encoded).expect("decode");
        assert_eq!(decoded.base.id, value.base.id, "id mismatch at index {idx}");
        assert_eq!(
            decoded.base.name, value.base.name,
            "name mismatch at index {idx}"
        );
        assert_eq!(decoded.extra, value.extra, "extra mismatch at index {idx}");
        assert_eq!(consumed, encoded.len(), "consumed mismatch at index {idx}");
    }
}

// ---------------------------------------------------------------------------
// Test 19: Flatten: inner struct skipped field still defaults on decode
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct BaseWithSkip {
    id: u32,
    #[oxicode(skip)]
    skipped: u64,
    name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ExtendedWithSkip {
    #[oxicode(flatten)]
    base: BaseWithSkip,
    extra: u64,
}

#[test]
fn test_flatten_inner_skipped_field_defaults_on_decode() {
    let value = ExtendedWithSkip {
        base: BaseWithSkip {
            id: 7,
            skipped: 0xDEAD_BEEF_CAFE_0000,
            name: "skip-test".to_string(),
        },
        extra: 42,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, consumed): (ExtendedWithSkip, usize) =
        decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.base.id, 7);
    assert_eq!(decoded.base.name, "skip-test");
    // Skipped field must be reset to Default::default() (0u64) after decode.
    assert_eq!(
        decoded.base.skipped, 0u64,
        "skipped field inside flattened inner must default to 0 on decode"
    );
    assert_eq!(decoded.extra, 42);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: Flatten: nested flattened struct with 3 levels
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level3 {
    z: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level2 {
    #[oxicode(flatten)]
    l3: Level3,
    y: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level1 {
    #[oxicode(flatten)]
    l2: Level2,
    x: u8,
}

#[test]
fn test_flatten_three_level_nesting() {
    // Manually inlined equivalent: z then y then x (declaration order).
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ManualFlat3 {
        z: u8,
        y: u8,
        x: u8,
    }

    let value = Level1 {
        l2: Level2 {
            l3: Level3 { z: 0x11 },
            y: 0x22,
        },
        x: 0x33,
    };
    let encoded = encode_to_vec(&value).expect("encode Level1");
    let (decoded, consumed): (Level1, usize) = decode_from_slice(&encoded).expect("decode Level1");

    assert_eq!(decoded.l2.l3.z, 0x11);
    assert_eq!(decoded.l2.y, 0x22);
    assert_eq!(decoded.x, 0x33);
    assert_eq!(consumed, encoded.len());

    // Byte-for-byte identity with the manual inlined struct.
    let flat = ManualFlat3 {
        z: 0x11,
        y: 0x22,
        x: 0x33,
    };
    let enc_flat = encode_to_vec(&flat).expect("encode ManualFlat3");
    assert_eq!(
        encoded, enc_flat,
        "three-level flatten must be byte-identical to the manually inlined struct"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec<Labeled> roundtrip with multiple elements
// ---------------------------------------------------------------------------

#[test]
fn test_vec_labeled_roundtrip_multiple_elements() {
    let values: Vec<Labeled> = vec![
        Labeled {
            label: "alpha".to_string(),
            coords: Point { x: 0.0, y: 0.0 },
        },
        Labeled {
            label: "beta".to_string(),
            coords: Point { x: 1.0, y: -1.0 },
        },
        Labeled {
            label: "gamma".to_string(),
            coords: Point {
                x: 100.5,
                y: 200.75,
            },
        },
    ];
    let encoded = encode_to_vec(&values).expect("encode Vec<Labeled>");
    let (decoded, consumed): (Vec<Labeled>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Labeled>");

    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].label, "alpha");
    assert_eq!(decoded[0].coords.x, 0.0);
    assert_eq!(decoded[1].label, "beta");
    assert_eq!(decoded[1].coords.y, -1.0);
    assert_eq!(decoded[2].label, "gamma");
    assert_eq!(decoded[2].coords.x, 100.5);
    assert_eq!(decoded[2].coords.y, 200.75);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: Extended struct — re-encode after decode produces same bytes
// ---------------------------------------------------------------------------

#[test]
fn test_extended_reencode_after_decode_same_bytes() {
    let original = Extended {
        base: Base {
            id: 314,
            name: "reencode".to_string(),
        },
        extra: 271,
    };
    let first_encoding = encode_to_vec(&original).expect("first encode");
    let (decoded, _): (Extended, usize) =
        decode_from_slice(&first_encoding).expect("decode after first encode");
    let second_encoding = encode_to_vec(&decoded).expect("second encode (re-encode)");

    assert_eq!(
        first_encoding, second_encoding,
        "re-encoding a decoded Extended struct must produce identical bytes"
    );
}
