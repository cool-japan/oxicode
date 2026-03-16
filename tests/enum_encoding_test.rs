//! Comprehensive enum encoding and decoding tests for OxiCode.
//!
//! Covers:
//! 1.  Simple C-like enum (no data) — discriminant values 0, 1, 2.
//! 2.  Enum with data variants (Circle, Rectangle, Triangle).
//! 3.  Large enum (256 variants) — varint discriminant boundary: ≤250 = 1 byte, >250 = 3 bytes.
//! 4.  Mixed variant kinds (unit, tuple, struct).
//! 5.  Nested enum (`Outer::A(Inner)` / `Outer::B`).
//! 6.  Roundtrip for all variants in sequence.
//! 7.  `Vec<Color>` roundtrip.
//! 8.  `Option<Shape>` roundtrip for each variant.
//! 9.  Binary format: first byte is the (varint-encoded) variant discriminant (0-based).
//! 10. Variant-level `#[oxicode(skip)]`: skipped variants share the discriminant of their
//!     nearest non-skipped successor, so encoding them produces the same bytes as encoding
//!     that successor; decoding always yields the successor.

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
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T) -> T {
    let encoded = oxicode::encode_to_vec(value).expect("encode failed");
    let (decoded, bytes_consumed) =
        oxicode::decode_from_slice::<T>(&encoded).expect("decode failed");
    assert_eq!(
        bytes_consumed,
        encoded.len(),
        "not all bytes consumed in roundtrip"
    );
    decoded
}

fn encode_bytes<T: Encode>(value: &T) -> Vec<u8> {
    oxicode::encode_to_vec(value).expect("encode failed")
}

// ---------------------------------------------------------------------------
// 1. Simple C-like enum (no data)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn enum_encoding_clike_discriminants() {
    // The default tag is u32 encoded as varint.  For values 0–250 the varint
    // is a single byte equal to the discriminant.
    let red_bytes = encode_bytes(&Color::Red);
    let green_bytes = encode_bytes(&Color::Green);
    let blue_bytes = encode_bytes(&Color::Blue);

    // Each unit variant encodes as exactly one varint byte (discriminant 0–2).
    assert_eq!(red_bytes.len(), 1, "Red should be 1 byte");
    assert_eq!(green_bytes.len(), 1, "Green should be 1 byte");
    assert_eq!(blue_bytes.len(), 1, "Blue should be 1 byte");

    assert_eq!(red_bytes[0], 0u8, "Red discriminant should be 0");
    assert_eq!(green_bytes[0], 1u8, "Green discriminant should be 1");
    assert_eq!(blue_bytes[0], 2u8, "Blue discriminant should be 2");

    // Roundtrips.
    assert_eq!(roundtrip(&Color::Red), Color::Red);
    assert_eq!(roundtrip(&Color::Green), Color::Green);
    assert_eq!(roundtrip(&Color::Blue), Color::Blue);
}

// ---------------------------------------------------------------------------
// 2. Enum with data variants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

#[test]
fn enum_encoding_data_variants_roundtrip() {
    let circle = Shape::Circle(std::f64::consts::PI);
    let rect = Shape::Rectangle(3.0, 4.0);
    let tri = Shape::Triangle {
        base: 5.0,
        height: 12.0,
    };

    assert_eq!(roundtrip(&circle), circle);
    assert_eq!(roundtrip(&rect), rect);
    assert_eq!(roundtrip(&tri), tri);
}

#[test]
fn enum_encoding_data_variants_discriminants() {
    let circle_bytes = encode_bytes(&Shape::Circle(1.0));
    let rect_bytes = encode_bytes(&Shape::Rectangle(1.0, 2.0));
    let tri_bytes = encode_bytes(&Shape::Triangle {
        base: 1.0,
        height: 2.0,
    });

    assert_eq!(circle_bytes[0], 0u8, "Circle discriminant should be 0");
    assert_eq!(rect_bytes[0], 1u8, "Rectangle discriminant should be 1");
    assert_eq!(tri_bytes[0], 2u8, "Triangle discriminant should be 2");
}

// ---------------------------------------------------------------------------
// 3. Large enum (256 variants) — varint discriminant boundary test
//
//    Only a handful of variants are tested:
//    discriminants 0, 1, 249, 250, 251, 255.
//    Discriminants ≤ 250 encode as 1 byte; ≥ 251 encode as 3 bytes
//    (tag byte 251 followed by 2-byte little-endian u16).
// ---------------------------------------------------------------------------

macro_rules! define_large_enum {
    // Build `enum Large256 { V0, V1, ..., V255 }` using recursive macro expansion.
    (@arms $name:ident, ) => {};
    (@arms $name:ident, $head:ident $($tail:ident)*) => {
        $head,
        define_large_enum!(@arms $name, $($tail)*);
    };

    ($( $variant:ident ),* $(,)?) => {
        #[derive(Debug, PartialEq, Encode, Decode)]
        enum Large256 {
            $( $variant, )*
        }
    };
}

define_large_enum!(
    V000, V001, V002, V003, V004, V005, V006, V007, V008, V009, V010, V011, V012, V013, V014, V015,
    V016, V017, V018, V019, V020, V021, V022, V023, V024, V025, V026, V027, V028, V029, V030, V031,
    V032, V033, V034, V035, V036, V037, V038, V039, V040, V041, V042, V043, V044, V045, V046, V047,
    V048, V049, V050, V051, V052, V053, V054, V055, V056, V057, V058, V059, V060, V061, V062, V063,
    V064, V065, V066, V067, V068, V069, V070, V071, V072, V073, V074, V075, V076, V077, V078, V079,
    V080, V081, V082, V083, V084, V085, V086, V087, V088, V089, V090, V091, V092, V093, V094, V095,
    V096, V097, V098, V099, V100, V101, V102, V103, V104, V105, V106, V107, V108, V109, V110, V111,
    V112, V113, V114, V115, V116, V117, V118, V119, V120, V121, V122, V123, V124, V125, V126, V127,
    V128, V129, V130, V131, V132, V133, V134, V135, V136, V137, V138, V139, V140, V141, V142, V143,
    V144, V145, V146, V147, V148, V149, V150, V151, V152, V153, V154, V155, V156, V157, V158, V159,
    V160, V161, V162, V163, V164, V165, V166, V167, V168, V169, V170, V171, V172, V173, V174, V175,
    V176, V177, V178, V179, V180, V181, V182, V183, V184, V185, V186, V187, V188, V189, V190, V191,
    V192, V193, V194, V195, V196, V197, V198, V199, V200, V201, V202, V203, V204, V205, V206, V207,
    V208, V209, V210, V211, V212, V213, V214, V215, V216, V217, V218, V219, V220, V221, V222, V223,
    V224, V225, V226, V227, V228, V229, V230, V231, V232, V233, V234, V235, V236, V237, V238, V239,
    V240, V241, V242, V243, V244, V245, V246, V247, V248, V249, V250, V251, V252, V253, V254, V255,
);

// Varint encoding constants (from src/varint/mod.rs):
//   SINGLE_BYTE_MAX = 250   (values 0..=250 encode as one byte)
//   U16_BYTE = 251          (tag for a following 2-byte u16)
const VARINT_SINGLE_BYTE_MAX: u8 = 250;
const VARINT_U16_TAG: u8 = 251;

#[test]
fn enum_encoding_large_enum_low_discriminants() {
    // V000 → discriminant 0, 1 byte
    let bytes = encode_bytes(&Large256::V000);
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 0u8);
    assert_eq!(roundtrip(&Large256::V000), Large256::V000);

    // V001 → discriminant 1, 1 byte
    let bytes = encode_bytes(&Large256::V001);
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 1u8);
    assert_eq!(roundtrip(&Large256::V001), Large256::V001);
}

#[test]
fn enum_encoding_large_enum_boundary_discriminant_249() {
    // V249 → discriminant 249, still fits in 1 byte (< 251 tag)
    let bytes = encode_bytes(&Large256::V249);
    assert_eq!(bytes.len(), 1, "V249 should be 1 byte");
    assert_eq!(bytes[0], 249u8);
    assert_eq!(roundtrip(&Large256::V249), Large256::V249);
}

#[test]
fn enum_encoding_large_enum_boundary_discriminant_250() {
    // V250 → discriminant 250, exactly SINGLE_BYTE_MAX, still 1 byte.
    let bytes = encode_bytes(&Large256::V250);
    assert_eq!(bytes.len(), 1, "V250 should be 1 byte (SINGLE_BYTE_MAX)");
    assert_eq!(bytes[0], VARINT_SINGLE_BYTE_MAX);
    assert_eq!(roundtrip(&Large256::V250), Large256::V250);
}

#[test]
fn enum_encoding_large_enum_boundary_discriminant_251() {
    // V251 → discriminant 251, crosses into multi-byte territory.
    // Encoding: [251, 251, 0] — tag byte 251, then u16 LE (251 = 0xFB, 0x00).
    let bytes = encode_bytes(&Large256::V251);
    assert_eq!(bytes.len(), 3, "V251 should be 3 bytes (varint u16 path)");
    assert_eq!(
        bytes[0], VARINT_U16_TAG,
        "first byte must be the U16_BYTE tag"
    );
    // u16 value 251 in little-endian = [0xFB, 0x00]
    let disc_u16 = u16::from_le_bytes([bytes[1], bytes[2]]);
    assert_eq!(disc_u16, 251u16);
    assert_eq!(roundtrip(&Large256::V251), Large256::V251);
}

#[test]
fn enum_encoding_large_enum_boundary_discriminant_255() {
    // V255 → discriminant 255, also multi-byte.
    let bytes = encode_bytes(&Large256::V255);
    assert_eq!(bytes.len(), 3, "V255 should be 3 bytes (varint u16 path)");
    assert_eq!(bytes[0], VARINT_U16_TAG);
    let disc_u16 = u16::from_le_bytes([bytes[1], bytes[2]]);
    assert_eq!(disc_u16, 255u16);
    assert_eq!(roundtrip(&Large256::V255), Large256::V255);
}

// ---------------------------------------------------------------------------
// 4. Enum with unit, tuple, and struct variants mixed
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Event {
    Tick,
    Move(i32, i32),
    Resize { width: u32, height: u32 },
    Quit,
    Data(Vec<u8>),
}

#[test]
fn enum_encoding_mixed_variants_roundtrip() {
    let variants = vec![
        Event::Tick,
        Event::Move(-10, 20),
        Event::Resize {
            width: 1920,
            height: 1080,
        },
        Event::Quit,
        Event::Data(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    ];
    for variant in &variants {
        assert_eq!(roundtrip(variant), *variant);
    }
}

#[test]
fn enum_encoding_mixed_variants_discriminants() {
    assert_eq!(encode_bytes(&Event::Tick)[0], 0u8);
    assert_eq!(encode_bytes(&Event::Move(0, 0))[0], 1u8);
    assert_eq!(
        encode_bytes(&Event::Resize {
            width: 0,
            height: 0
        })[0],
        2u8
    );
    assert_eq!(encode_bytes(&Event::Quit)[0], 3u8);
    assert_eq!(encode_bytes(&Event::Data(vec![]))[0], 4u8);
}

// ---------------------------------------------------------------------------
// 5. Nested enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Inner {
    X,
    Y,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Outer {
    A(Inner),
    B,
}

#[test]
fn enum_encoding_nested_roundtrip() {
    assert_eq!(roundtrip(&Outer::A(Inner::X)), Outer::A(Inner::X));
    assert_eq!(roundtrip(&Outer::A(Inner::Y)), Outer::A(Inner::Y));
    assert_eq!(roundtrip(&Outer::B), Outer::B);
}

#[test]
fn enum_encoding_nested_binary_layout() {
    // Outer::A(Inner::X):  outer discriminant 0 | inner discriminant 0
    let bytes = encode_bytes(&Outer::A(Inner::X));
    assert_eq!(bytes[0], 0u8, "outer variant A discriminant");
    assert_eq!(bytes[1], 0u8, "inner variant X discriminant");

    // Outer::A(Inner::Y):  outer discriminant 0 | inner discriminant 1
    let bytes = encode_bytes(&Outer::A(Inner::Y));
    assert_eq!(bytes[0], 0u8);
    assert_eq!(bytes[1], 1u8, "inner variant Y discriminant");

    // Outer::B:  single byte, discriminant 1
    let bytes = encode_bytes(&Outer::B);
    assert_eq!(bytes.len(), 1);
    assert_eq!(bytes[0], 1u8);
}

// ---------------------------------------------------------------------------
// 6. Roundtrip for all variants in sequence (using Color)
// ---------------------------------------------------------------------------

#[test]
fn enum_encoding_all_variants_sequence_roundtrip() {
    let sequence = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Blue,
    ];

    let encoded = oxicode::encode_to_vec(&sequence).expect("encode sequence");
    let (decoded, bytes_read): (Vec<Color>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode sequence");

    assert_eq!(decoded, sequence);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 7. Vec<Color> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn enum_encoding_vec_roundtrip() {
    let colors = vec![
        Color::Blue,
        Color::Red,
        Color::Green,
        Color::Green,
        Color::Blue,
    ];
    let encoded = oxicode::encode_to_vec(&colors).expect("encode Vec<Color>");
    let (decoded, _): (Vec<Color>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode Vec<Color>");
    assert_eq!(decoded, colors);
}

#[test]
fn enum_encoding_empty_vec_roundtrip() {
    let empty: Vec<Color> = vec![];
    assert_eq!(roundtrip(&empty), empty);
}

// ---------------------------------------------------------------------------
// 8. Option<Shape> roundtrip for each variant
// ---------------------------------------------------------------------------

#[test]
fn enum_encoding_option_shape_none() {
    let none: Option<Shape> = None;
    assert_eq!(roundtrip(&none), none);
}

#[test]
fn enum_encoding_option_shape_circle() {
    let some = Some(Shape::Circle(std::f64::consts::E));
    assert_eq!(roundtrip(&some), some);
}

#[test]
fn enum_encoding_option_shape_rectangle() {
    let some = Some(Shape::Rectangle(100.0, 200.0));
    assert_eq!(roundtrip(&some), some);
}

#[test]
fn enum_encoding_option_shape_triangle() {
    let some = Some(Shape::Triangle {
        base: 3.0,
        height: 4.0,
    });
    assert_eq!(roundtrip(&some), some);
}

// ---------------------------------------------------------------------------
// 9. Binary format check: first byte is variant discriminant (0-based)
//    Uses the legacy (fixed-int) config so that the u32 discriminant is
//    exactly 4 bytes, making the offset unambiguous even for data variants.
// ---------------------------------------------------------------------------

#[test]
fn enum_encoding_binary_format_first_byte_is_discriminant() {
    // With the standard varint config, unit variants produce exactly one byte
    // (the discriminant), because values 0–250 encode as a single varint byte.
    for (i, variant) in [Color::Red, Color::Green, Color::Blue].iter().enumerate() {
        let bytes = encode_bytes(variant);
        assert_eq!(
            bytes[0], i as u8,
            "variant {:?}: expected first byte {}, got {}",
            variant, i, bytes[0]
        );
    }

    // For data variants the discriminant is still the first byte(s).
    // Circle(discriminant=0), Rectangle(discriminant=1), Triangle(discriminant=2).
    let shape_tests: &[(Shape, u8)] = &[
        (Shape::Circle(1.0), 0),
        (Shape::Rectangle(1.0, 2.0), 1),
        (
            Shape::Triangle {
                base: 1.0,
                height: 1.0,
            },
            2,
        ),
    ];
    for (shape, expected_disc) in shape_tests {
        let bytes = encode_bytes(shape);
        assert_eq!(
            bytes[0], *expected_disc,
            "{:?}: expected discriminant byte {}",
            shape, expected_disc
        );
    }
}

// ---------------------------------------------------------------------------
// 10. Enum with skip attribute on a variant
//
//     `#[oxicode(skip)]` on a variant causes it to be excluded from the
//     discriminant space during decode.  On encode it receives the same
//     discriminant as its nearest non-skipped successor, so:
//       - encoding E::B produces the same bytes as encoding E::C
//       - decoding those bytes yields E::C
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum SkippedVariantEnum {
    A,
    #[oxicode(skip)]
    B,
    C,
}

#[test]
fn enum_encoding_skip_variant_b_encodes_as_c() {
    let bytes_b = encode_bytes(&SkippedVariantEnum::B);
    let bytes_c = encode_bytes(&SkippedVariantEnum::C);

    assert_eq!(
        bytes_b, bytes_c,
        "encoding B must produce the same bytes as encoding C"
    );
}

#[test]
fn enum_encoding_skip_variant_b_decodes_as_c() {
    // Encode B — which gets C's discriminant — then decode; must yield C.
    let bytes_b = encode_bytes(&SkippedVariantEnum::B);
    let (decoded, _): (SkippedVariantEnum, _) =
        oxicode::decode_from_slice(&bytes_b).expect("decode failed");
    assert_eq!(
        decoded,
        SkippedVariantEnum::C,
        "decoding bytes written for B must yield C"
    );
}

#[test]
fn enum_encoding_skip_variant_a_and_c_roundtrip_unchanged() {
    // A and C are not affected by the skip on B.
    assert_eq!(roundtrip(&SkippedVariantEnum::A), SkippedVariantEnum::A);
    assert_eq!(roundtrip(&SkippedVariantEnum::C), SkippedVariantEnum::C);
}

#[test]
fn enum_encoding_skip_variant_a_keeps_discriminant_zero() {
    let bytes_a = encode_bytes(&SkippedVariantEnum::A);
    assert_eq!(bytes_a.len(), 1);
    assert_eq!(bytes_a[0], 0u8, "A must retain discriminant 0");
}

#[test]
fn enum_encoding_skip_variant_c_discriminant() {
    // C is at position index 2 so its natural discriminant is 2.
    let bytes_c = encode_bytes(&SkippedVariantEnum::C);
    assert_eq!(bytes_c.len(), 1);
    assert_eq!(bytes_c[0], 2u8, "C must have discriminant 2");
}
