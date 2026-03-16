//! Advanced tests for #[oxicode(flatten)] field attribute in OxiCode — 22 scenarios.
//!
//! The flatten attribute inlines the fields of an inner struct into the encoding
//! of the outer struct, producing byte-for-byte identical output to a manually
//! flattened struct (no wrapper overhead).

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
    encode_to_vec_with_config, encoded_size, Decode, Encode,
};
use std::collections::HashMap;
use std::f64::consts::{E, PI};

// ---------------------------------------------------------------------------
// Test 1: Basic flatten — outer struct with one flattened inner struct
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_basic_outer_with_inner() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        a: u32,
        b: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        #[oxicode(flatten)]
        inner: Inner,
        c: u32,
    }

    let value = Outer {
        inner: Inner { a: 10, b: 20 },
        c: 30,
    };
    let encoded = encode_to_vec(&value).expect("encode outer");
    let (decoded, _): (Outer, _) = decode_from_slice(&encoded).expect("decode outer");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Flatten with multiple outer fields before the flattened inner
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_fields_before_inner() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Coords {
        lat: f32,
        lon: f32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Location {
        id: u64,
        name_code: u32,
        #[oxicode(flatten)]
        coords: Coords,
    }

    let value = Location {
        id: 1,
        name_code: 42,
        coords: Coords { lat: 1.5, lon: 2.5 },
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Location, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Flatten with multiple outer fields after the flattened inner
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_fields_after_inner() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Header {
        version: u8,
        flags: u16,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Packet {
        #[oxicode(flatten)]
        header: Header,
        payload_len: u32,
        checksum: u32,
    }

    let value = Packet {
        header: Header {
            version: 1,
            flags: 0xFF,
        },
        payload_len: 256,
        checksum: 0xDEAD,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Packet, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Flatten — encoded size equals sum of all fields (no wrapper overhead)
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_encoded_size_equals_sum_of_fields() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Position {
        x: u8,
        y: u8,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Entity {
        id: u8,
        #[oxicode(flatten)]
        pos: Position,
        hp: u8,
    }

    // With fixed-int config, each u8 = 1 byte → total = 4 bytes.
    let cfg = config::standard().with_fixed_int_encoding();
    let value = Entity {
        id: 1,
        pos: Position { x: 5, y: 10 },
        hp: 100,
    };
    let encoded = encode_to_vec_with_config(&value, cfg).expect("encode");
    // id(1) + x(1) + y(1) + hp(1) = 4 bytes
    assert_eq!(encoded.len(), 4, "flatten must not add any wrapper bytes");
}

// ---------------------------------------------------------------------------
// Test 5: Two-level flatten (flatten of flatten)
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_two_level() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level1 {
        x: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level2 {
        #[oxicode(flatten)]
        l1: Level1,
        y: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Level3 {
        #[oxicode(flatten)]
        l2: Level2,
        z: u32,
    }

    let value = Level3 {
        l2: Level2 {
            l1: Level1 { x: 1 },
            y: 2,
        },
        z: 3,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Level3, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Flatten with Vec field in inner struct
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_with_vec_in_inner() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Items {
        tags: Vec<u32>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Record {
        id: u64,
        #[oxicode(flatten)]
        items: Items,
    }

    let value = Record {
        id: 99,
        items: Items {
            tags: vec![1, 2, 3, 4],
        },
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Record, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
    assert_eq!(decoded.items.tags.len(), 4);
}

// ---------------------------------------------------------------------------
// Test 7: Flatten with Option field in inner struct (Some and None)
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_with_option_in_inner() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Metadata {
        label: Option<u32>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Node {
        id: u32,
        #[oxicode(flatten)]
        meta: Metadata,
    }

    let value_some = Node {
        id: 1,
        meta: Metadata { label: Some(42) },
    };
    let encoded_some = encode_to_vec(&value_some).expect("encode some");
    let (decoded_some, _): (Node, _) = decode_from_slice(&encoded_some).expect("decode some");
    assert_eq!(value_some, decoded_some);

    let value_none = Node {
        id: 2,
        meta: Metadata { label: None },
    };
    let encoded_none = encode_to_vec(&value_none).expect("encode none");
    let (decoded_none, _): (Node, _) = decode_from_slice(&encoded_none).expect("decode none");
    assert_eq!(value_none, decoded_none);

    // None encoding must be smaller than Some encoding
    assert!(
        encoded_none.len() < encoded_some.len(),
        "None option should encode to fewer bytes than Some"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Flatten inner struct with String field
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_inner_with_string() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct NamePart {
        name: String,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Person {
        age: u32,
        #[oxicode(flatten)]
        name_part: NamePart,
    }

    let value = Person {
        age: 30,
        name_part: NamePart {
            name: "Alice".to_string(),
        },
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Person, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
    assert_eq!(decoded.name_part.name, "Alice");
}

// ---------------------------------------------------------------------------
// Test 9: Flatten inner struct with bool fields
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_inner_with_bools() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Flags {
        active: bool,
        visible: bool,
        locked: bool,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Widget {
        id: u32,
        #[oxicode(flatten)]
        flags: Flags,
    }

    let value = Widget {
        id: 7,
        flags: Flags {
            active: true,
            visible: false,
            locked: true,
        },
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Widget, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
    assert!(decoded.flags.active);
    assert!(!decoded.flags.visible);
    assert!(decoded.flags.locked);
}

// ---------------------------------------------------------------------------
// Test 10: Flatten inner struct with u64 fields
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_inner_with_u64_fields() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Timestamps {
        created_at: u64,
        updated_at: u64,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Document {
        #[oxicode(flatten)]
        timestamps: Timestamps,
        size: u64,
    }

    let value = Document {
        timestamps: Timestamps {
            created_at: 1_700_000_000,
            updated_at: 1_700_100_000,
        },
        size: 4096,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Document, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Flatten + fixed int encoding
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_with_fixed_int_encoding() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Pixel {
        x: u32,
        y: u32,
        #[oxicode(flatten)]
        color: Color,
    }

    let cfg = config::standard().with_fixed_int_encoding();
    let value = Pixel {
        x: 100,
        y: 200,
        color: Color {
            r: 255,
            g: 128,
            b: 64,
        },
    };
    let encoded = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (Pixel, _) = decode_from_slice_with_config(&encoded, cfg).expect("decode");
    assert_eq!(value, decoded);
    // x(4) + y(4) + r(1) + g(1) + b(1) = 11 bytes
    assert_eq!(encoded.len(), 11);
}

// ---------------------------------------------------------------------------
// Test 12: Flatten + big endian config
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_with_big_endian_config() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Origin {
        ox: u32,
        oy: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Frame {
        #[oxicode(flatten)]
        origin: Origin,
        width: u32,
        height: u32,
    }

    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value = Frame {
        origin: Origin {
            ox: 0x0102_0304,
            oy: 0x0506_0708,
        },
        width: 1920,
        height: 1080,
    };
    let encoded = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (Frame, _) = decode_from_slice_with_config(&encoded, cfg).expect("decode");
    assert_eq!(value, decoded);
    // Verify big-endian byte order for origin.ox (0x01020304)
    assert_eq!(encoded[0], 0x01);
    assert_eq!(encoded[1], 0x02);
    assert_eq!(encoded[2], 0x03);
    assert_eq!(encoded[3], 0x04);
}

// ---------------------------------------------------------------------------
// Test 13: Flatten with multiple flattened fields (outer has 2 flatten fields)
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_multiple_flattened_fields() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Part1 {
        a: u32,
        b: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Part2 {
        c: u32,
        d: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Composite {
        prefix: u8,
        #[oxicode(flatten)]
        part1: Part1,
        #[oxicode(flatten)]
        part2: Part2,
        suffix: u8,
    }

    // Manually flattened equivalent
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ManualFlat {
        prefix: u8,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        suffix: u8,
    }

    let value = Composite {
        prefix: 1,
        part1: Part1 { a: 10, b: 20 },
        part2: Part2 { c: 30, d: 40 },
        suffix: 2,
    };
    let encoded = encode_to_vec(&value).expect("encode composite");
    let (decoded, _): (Composite, _) = decode_from_slice(&encoded).expect("decode composite");
    assert_eq!(value, decoded);

    let flat = ManualFlat {
        prefix: 1,
        a: 10,
        b: 20,
        c: 30,
        d: 40,
        suffix: 2,
    };
    let flat_encoded = encode_to_vec(&flat).expect("encode flat");
    assert_eq!(
        encoded, flat_encoded,
        "two flattened structs must match manual layout"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Flatten where inner has skip attribute on a field
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_inner_with_skip_field() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Config {
        timeout: u32,
        #[oxicode(skip)]
        cache: String,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Request {
        id: u32,
        #[oxicode(flatten)]
        config: Config,
    }

    let value = Request {
        id: 42,
        config: Config {
            timeout: 30,
            cache: "skip-me".to_string(),
        },
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Request, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.id, 42);
    assert_eq!(decoded.config.timeout, 30);
    // skipped field restores to Default
    assert_eq!(decoded.config.cache, String::default());
}

// ---------------------------------------------------------------------------
// Test 15: Flatten where inner has rename attribute (no-op on wire)
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_inner_with_rename_field() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        #[oxicode(rename = "userId")]
        user_id: u64,
        score: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        #[oxicode(flatten)]
        inner: Inner,
        level: u32,
    }

    // rename is a no-op on the wire — binary layout is unchanged
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct OuterFlat {
        user_id: u64,
        score: u32,
        level: u32,
    }

    let value = Outer {
        inner: Inner {
            user_id: 123,
            score: 9999,
        },
        level: 5,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Outer, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);

    let flat = OuterFlat {
        user_id: 123,
        score: 9999,
        level: 5,
    };
    let flat_encoded = encode_to_vec(&flat).expect("encode flat");
    assert_eq!(encoded, flat_encoded, "rename must not change wire format");
}

// ---------------------------------------------------------------------------
// Test 16: Flatten with generic inner struct
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_with_generic_inner() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Wrapper<T: Encode + Decode> {
        value: T,
        count: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Container {
        name_code: u32,
        #[oxicode(flatten)]
        data: Wrapper<u64>,
    }

    let value = Container {
        name_code: 1,
        data: Wrapper {
            value: 100_u64,
            count: 5,
        },
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Container, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Flatten + derive with generics on outer struct
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_derive_with_outer_generics() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Dims {
        width: u32,
        height: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Typed<T: Encode + Decode> {
        #[oxicode(flatten)]
        dims: Dims,
        payload: T,
    }

    let value = Typed::<String> {
        dims: Dims {
            width: 800,
            height: 600,
        },
        payload: "hello".to_string(),
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Typed<String>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Vec<OuterWithFlattened> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_vec_of_outer_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Stats {
        min: u32,
        max: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Entry {
        id: u32,
        #[oxicode(flatten)]
        stats: Stats,
    }

    let entries: Vec<Entry> = vec![
        Entry {
            id: 1,
            stats: Stats { min: 0, max: 10 },
        },
        Entry {
            id: 2,
            stats: Stats { min: 5, max: 50 },
        },
        Entry {
            id: 3,
            stats: Stats { min: 100, max: 200 },
        },
    ];
    let encoded = encode_to_vec(&entries).expect("encode vec");
    let (decoded, _): (Vec<Entry>, _) = decode_from_slice(&encoded).expect("decode vec");
    assert_eq!(entries, decoded);
    assert_eq!(decoded.len(), 3);
}

// ---------------------------------------------------------------------------
// Test 19: Option<OuterWithFlattened> Some/None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_option_outer_some_and_none() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        value: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        id: u32,
        #[oxicode(flatten)]
        inner: Inner,
    }

    let some_val: Option<Outer> = Some(Outer {
        id: 5,
        inner: Inner { value: 42 },
    });
    let encoded_some = encode_to_vec(&some_val).expect("encode some");
    let (decoded_some, _): (Option<Outer>, _) =
        decode_from_slice(&encoded_some).expect("decode some");
    assert_eq!(some_val, decoded_some);

    let none_val: Option<Outer> = None;
    let encoded_none = encode_to_vec(&none_val).expect("encode none");
    let (decoded_none, _): (Option<Outer>, _) =
        decode_from_slice(&encoded_none).expect("decode none");
    assert_eq!(none_val, decoded_none);
}

// ---------------------------------------------------------------------------
// Test 20: Nested flatten: A contains flattened B, B contains flattened C
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_nested_three_levels() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct C {
        c1: u32,
        c2: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct B {
        b1: u32,
        #[oxicode(flatten)]
        c: C,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct A {
        a1: u32,
        #[oxicode(flatten)]
        b: B,
    }

    // Equivalent manual flat struct
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Flat {
        a1: u32,
        b1: u32,
        c1: u32,
        c2: u32,
    }

    let value = A {
        a1: 1,
        b: B {
            b1: 2,
            c: C { c1: 3, c2: 4 },
        },
    };
    let encoded = encode_to_vec(&value).expect("encode A");
    let (decoded, _): (A, _) = decode_from_slice(&encoded).expect("decode A");
    assert_eq!(value, decoded);

    let flat = Flat {
        a1: 1,
        b1: 2,
        c1: 3,
        c2: 4,
    };
    let flat_encoded = encode_to_vec(&flat).expect("encode Flat");
    assert_eq!(
        encoded, flat_encoded,
        "three-level flatten must match manual layout"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Flatten preserves field ordering in wire format
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_preserves_field_order_in_wire_format() {
    // With fixed-int encoding we can reason about exact byte positions.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Mid {
        m: u8,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Ordered {
        first: u8,
        #[oxicode(flatten)]
        mid: Mid,
        last: u8,
    }

    let cfg = config::standard().with_fixed_int_encoding();
    let value = Ordered {
        first: 0xAA,
        mid: Mid { m: 0xBB },
        last: 0xCC,
    };
    let encoded = encode_to_vec_with_config(&value, cfg).expect("encode");
    // Expected byte order: first(0xAA), m(0xBB), last(0xCC)
    assert_eq!(
        encoded,
        vec![0xAA, 0xBB, 0xCC],
        "field order must be preserved across flatten"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Flatten size comparison: with vs without flatten (should be same size)
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_size_equals_non_flatten_equivalent() {
    // Struct with flatten
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct InnerFields {
        p: u32,
        q: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WithFlatten {
        n: u32,
        #[oxicode(flatten)]
        inner: InnerFields,
        m: u32,
    }

    // Struct without flatten — manually inlined
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WithoutFlatten {
        n: u32,
        p: u32,
        q: u32,
        m: u32,
    }

    let with_val = WithFlatten {
        n: 1,
        inner: InnerFields { p: 2, q: 3 },
        m: 4,
    };
    let without_val = WithoutFlatten {
        n: 1,
        p: 2,
        q: 3,
        m: 4,
    };

    let size_with = encoded_size(&with_val).expect("size with flatten");
    let size_without = encoded_size(&without_val).expect("size without flatten");

    assert_eq!(
        size_with, size_without,
        "flatten must produce identical encoded size to manual inlining"
    );

    // Also verify byte-for-byte identity
    let encoded_with = encode_to_vec(&with_val).expect("encode with");
    let encoded_without = encode_to_vec(&without_val).expect("encode without");
    assert_eq!(
        encoded_with, encoded_without,
        "flatten must produce identical bytes to manually inlined struct"
    );
}

// ---------------------------------------------------------------------------
// Bonus integration: flatten with HashMap in inner struct
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_inner_with_hashmap() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WithMap {
        entries: HashMap<String, u32>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        version: u32,
        #[oxicode(flatten)]
        payload: WithMap,
    }

    let mut map = HashMap::new();
    map.insert("alpha".to_string(), 1_u32);
    map.insert("beta".to_string(), 2_u32);

    let value = Outer {
        version: 1,
        payload: WithMap { entries: map },
    };
    let encoded = encode_to_vec(&value).expect("encode outer with hashmap");
    let (decoded, _): (Outer, _) = decode_from_slice(&encoded).expect("decode outer with hashmap");
    assert_eq!(decoded.version, 1);
    assert_eq!(decoded.payload.entries.len(), 2);
    assert_eq!(decoded.payload.entries.get("alpha"), Some(&1_u32));
    assert_eq!(decoded.payload.entries.get("beta"), Some(&2_u32));
}

// ---------------------------------------------------------------------------
// Bonus: flatten with f64 PI/E values
// ---------------------------------------------------------------------------
#[test]
fn test_flatten_with_pi_e_values() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MathConstants {
        pi: f64,
        e: f64,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Calculation {
        #[oxicode(flatten)]
        constants: MathConstants,
        multiplier: f64,
    }

    let value = Calculation {
        constants: MathConstants { pi: PI, e: E },
        multiplier: PI * E,
    };
    let encoded = encode_to_vec(&value).expect("encode");
    let (decoded, _): (Calculation, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.constants.pi, PI, "PI must round-trip exactly");
    assert_eq!(decoded.constants.e, E, "E must round-trip exactly");
    assert_eq!(decoded.multiplier, PI * E, "PI*E must round-trip exactly");
}
