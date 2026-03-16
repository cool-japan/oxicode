//! Advanced container-level derive attribute tests for OxiCode.
//!
//! These tests cover combinations, edge cases, and scenarios that are distinct
//! from the basic coverage in derive_container_attr_test.rs, derive_rename_all_test.rs,
//! derive_tag_type_test.rs, derive_bound_test.rs, and derive_crate_path_test.rs.
//!
//! Key properties verified:
//!  - All rename_all variants roundtrip correctly with fields containing f64 constants
//!  - tag_type interacts correctly with data-carrying enum variants
//!  - Container attrs are independent across multiple struct definitions
//!  - Unit structs and single-variant enums handle container attrs correctly
//!  - Field-level rename overrides rename_all (semantic precedence, both no-ops on wire)
//!  - Tuple structs work with container-level attrs

#![allow(dead_code)]
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
use std::f64::consts::{E, PI};

mod derive_container_advanced_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Test 1: rename_all = "snake_case" on struct with f64 fields — roundtrip
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "snake_case")]
    struct SnakeCaseMathStruct {
        pi_value: f64,
        euler_number: f64,
        max_iterations: u32,
    }

    #[test]
    fn test_snake_case_rename_all_f64_roundtrip() {
        let original = SnakeCaseMathStruct {
            pi_value: PI,
            euler_number: E,
            max_iterations: 1000,
        };
        let encoded = encode_to_vec(&original).expect("encode snake_case struct with f64");
        let (decoded, bytes_read): (SnakeCaseMathStruct, usize) =
            decode_from_slice(&encoded).expect("decode snake_case struct with f64");
        assert_eq!(decoded, original);
        assert_eq!(bytes_read, encoded.len());
    }

    // -----------------------------------------------------------------------
    // Test 2: rename_all = "camelCase" on struct with f64 fields — roundtrip
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "camelCase")]
    struct CamelCaseMathStruct {
        pi_constant: f64,
        euler_constant: f64,
        precision_level: u8,
    }

    #[test]
    fn test_camel_case_rename_all_f64_roundtrip() {
        let original = CamelCaseMathStruct {
            pi_constant: PI,
            euler_constant: E,
            precision_level: 15,
        };
        let encoded = encode_to_vec(&original).expect("encode camelCase struct with f64");
        let (decoded, bytes_read): (CamelCaseMathStruct, usize) =
            decode_from_slice(&encoded).expect("decode camelCase struct with f64");
        assert_eq!(decoded, original);
        assert_eq!(bytes_read, encoded.len());
    }

    // -----------------------------------------------------------------------
    // Test 3: rename_all = "PascalCase" on struct with multiple types — roundtrip
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "PascalCase")]
    struct PascalCaseMixedStruct {
        circle_radius: f64,
        vertex_count: u32,
        label_text: String,
        is_closed: bool,
    }

    #[test]
    fn test_pascal_case_rename_all_mixed_types_roundtrip() {
        let original = PascalCaseMixedStruct {
            circle_radius: PI * 2.0,
            vertex_count: 6,
            label_text: "hexagon".to_string(),
            is_closed: true,
        };
        let encoded = encode_to_vec(&original).expect("encode PascalCase mixed struct");
        let (decoded, bytes_read): (PascalCaseMixedStruct, usize) =
            decode_from_slice(&encoded).expect("decode PascalCase mixed struct");
        assert_eq!(decoded, original);
        assert_eq!(bytes_read, encoded.len());
    }

    // -----------------------------------------------------------------------
    // Test 4: rename_all = "SCREAMING_SNAKE_CASE" with negative integers — roundtrip
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
    struct ScreamingSnakeSigned {
        min_temperature: i32,
        max_temperature: i32,
        base_offset: i64,
    }

    #[test]
    fn test_screaming_snake_case_signed_ints_roundtrip() {
        let original = ScreamingSnakeSigned {
            min_temperature: -273,
            max_temperature: 1_000_000,
            base_offset: i64::MIN,
        };
        let encoded = encode_to_vec(&original).expect("encode SCREAMING_SNAKE_CASE signed");
        let (decoded, bytes_read): (ScreamingSnakeSigned, usize) =
            decode_from_slice(&encoded).expect("decode SCREAMING_SNAKE_CASE signed");
        assert_eq!(decoded, original);
        assert_eq!(bytes_read, encoded.len());
    }

    // -----------------------------------------------------------------------
    // Test 5: rename_all = "kebab-case" on struct with Vec<f64> — roundtrip
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "kebab-case")]
    struct KebabCaseVecF64 {
        sample_values: Vec<f64>,
        sample_count: usize,
    }

    #[test]
    fn test_kebab_case_rename_all_vec_f64_roundtrip() {
        let original = KebabCaseVecF64 {
            sample_values: vec![PI, E, PI / E, E / PI],
            sample_count: 4,
        };
        let encoded = encode_to_vec(&original).expect("encode kebab-case vec f64");
        let (decoded, bytes_read): (KebabCaseVecF64, usize) =
            decode_from_slice(&encoded).expect("decode kebab-case vec f64");
        assert_eq!(decoded, original);
        assert_eq!(bytes_read, encoded.len());
    }

    // -----------------------------------------------------------------------
    // Test 6: rename_all doesn't change binary wire format (identity check)
    // Uses two structs with identical layout but different rename_all settings
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WireBaselineAdvanced {
        seq_num: u64,
        payload_len: u32,
        checksum: u16,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "PascalCase")]
    struct WireRenamedAdvanced {
        seq_num: u64,
        payload_len: u32,
        checksum: u16,
    }

    #[test]
    fn test_rename_all_does_not_change_wire_format_advanced() {
        let baseline = WireBaselineAdvanced {
            seq_num: 0xDEAD_BEEF_CAFE_0001,
            payload_len: 512,
            checksum: 0xABCD,
        };
        let renamed = WireRenamedAdvanced {
            seq_num: 0xDEAD_BEEF_CAFE_0001,
            payload_len: 512,
            checksum: 0xABCD,
        };
        let baseline_bytes = encode_to_vec(&baseline).expect("encode baseline");
        let renamed_bytes = encode_to_vec(&renamed).expect("encode renamed");
        assert_eq!(
            baseline_bytes, renamed_bytes,
            "rename_all must not alter binary wire bytes (PascalCase vs no rename)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: Enum with tag_type = "u8" — data-carrying variants roundtrip
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u8")]
    enum U8TagDataEnum {
        Empty,
        WithFloat(f64),
        WithPair { x: f64, y: f64 },
        WithVec(Vec<u8>),
    }

    #[test]
    fn test_tag_type_u8_data_carrying_variants_roundtrip() {
        let cases = vec![
            U8TagDataEnum::Empty,
            U8TagDataEnum::WithFloat(PI),
            U8TagDataEnum::WithPair { x: PI, y: E },
            U8TagDataEnum::WithVec(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        ];
        for case in cases {
            let encoded = encode_to_vec(&case).expect("encode u8 tag enum");
            let (decoded, bytes_read): (U8TagDataEnum, usize) =
                decode_from_slice(&encoded).expect("decode u8 tag enum");
            assert_eq!(decoded, case);
            assert_eq!(bytes_read, encoded.len());
        }
    }

    // -----------------------------------------------------------------------
    // Test 8: Enum with tag_type = "u16" — 2-byte discriminant verified
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u16")]
    enum U16TagEnum {
        Alpha,
        Beta { value: f64 },
        Gamma(String, u32),
    }

    #[test]
    fn test_tag_type_u16_discriminant_and_roundtrip() {
        // In fixed-int mode the u16 discriminant is exactly 2 bytes.
        let unit_bytes =
            oxicode::encode_to_vec_with_config(&U16TagEnum::Alpha, oxicode::config::legacy())
                .expect("encode fixed u16");
        assert_eq!(
            unit_bytes.len(),
            2,
            "u16 unit variant must be 2 bytes in fixed mode"
        );

        let cases = vec![
            U16TagEnum::Alpha,
            U16TagEnum::Beta { value: E },
            U16TagEnum::Gamma("hello".to_string(), 99),
        ];
        for case in cases {
            let enc = encode_to_vec(&case).expect("encode u16 tag");
            let (dec, _): (U16TagEnum, usize) = decode_from_slice(&enc).expect("decode u16 tag");
            assert_eq!(dec, case);
        }
    }

    // -----------------------------------------------------------------------
    // Test 9: Enum with tag_type = "u32" — 4-byte discriminant verified
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u32")]
    enum U32TagEnum {
        First,
        Second(f64),
        Third { a: u32, b: u32 },
    }

    #[test]
    fn test_tag_type_u32_discriminant_and_roundtrip() {
        let unit_bytes =
            oxicode::encode_to_vec_with_config(&U32TagEnum::First, oxicode::config::legacy())
                .expect("encode fixed u32");
        assert_eq!(
            unit_bytes.len(),
            4,
            "u32 unit variant must be 4 bytes in fixed mode"
        );

        let cases = vec![
            U32TagEnum::First,
            U32TagEnum::Second(PI * E),
            U32TagEnum::Third { a: 100, b: 200 },
        ];
        for case in cases {
            let enc = encode_to_vec(&case).expect("encode u32 tag");
            let (dec, _): (U32TagEnum, usize) = decode_from_slice(&enc).expect("decode u32 tag");
            assert_eq!(dec, case);
        }
    }

    // -----------------------------------------------------------------------
    // Test 10: Enum with tag_type = "u64" — 8-byte discriminant verified
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u64")]
    enum U64TagEnum {
        Only,
        WithData(Vec<f64>),
    }

    #[test]
    fn test_tag_type_u64_discriminant_and_roundtrip() {
        let unit_bytes =
            oxicode::encode_to_vec_with_config(&U64TagEnum::Only, oxicode::config::legacy())
                .expect("encode fixed u64");
        assert_eq!(
            unit_bytes.len(),
            8,
            "u64 unit variant must be 8 bytes in fixed mode"
        );

        let cases = vec![
            U64TagEnum::Only,
            U64TagEnum::WithData(vec![PI, E, 1.0, 0.0]),
        ];
        for case in cases {
            let enc = encode_to_vec(&case).expect("encode u64 tag");
            let (dec, _): (U64TagEnum, usize) = decode_from_slice(&enc).expect("decode u64 tag");
            assert_eq!(dec, case);
        }
    }

    // -----------------------------------------------------------------------
    // Test 11: Mixed container attrs: tag_type + rename_all on same enum
    // Distinct from derive_rename_all_test.rs test 18 by using u32 tag + snake_case
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u32", rename_all = "snake_case")]
    enum U32TagSnakeCaseEnum {
        ConnectRequest { host_name: String, port_number: u16 },
        DisconnectRequest,
        DataTransfer(Vec<u8>),
    }

    #[test]
    fn test_mixed_tag_type_u32_and_rename_all_snake_case() {
        let cases = vec![
            U32TagSnakeCaseEnum::ConnectRequest {
                host_name: "localhost".to_string(),
                port_number: 8080,
            },
            U32TagSnakeCaseEnum::DisconnectRequest,
            U32TagSnakeCaseEnum::DataTransfer(vec![1, 2, 3, 4, 5]),
        ];
        for case in cases {
            let enc = encode_to_vec(&case).expect("encode u32+snake_case enum");
            let (dec, bytes_read): (U32TagSnakeCaseEnum, usize) =
                decode_from_slice(&enc).expect("decode u32+snake_case enum");
            assert_eq!(dec, case);
            assert_eq!(bytes_read, enc.len());
        }
    }

    // -----------------------------------------------------------------------
    // Test 12: Struct with #[oxicode(crate = "oxicode")] — f64 field roundtrip
    // Distinct from derive_crate_path_test.rs by adding f64 constants
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(crate = "oxicode")]
    struct ExplicitCrateF64 {
        pi: f64,
        e: f64,
        label: String,
    }

    #[test]
    fn test_explicit_crate_attr_with_f64_fields() {
        let original = ExplicitCrateF64 {
            pi: PI,
            e: E,
            label: "math".to_string(),
        };
        let enc = encode_to_vec(&original).expect("encode crate attr struct with f64");
        let (dec, bytes_read): (ExplicitCrateF64, usize) =
            decode_from_slice(&enc).expect("decode crate attr struct with f64");
        assert_eq!(dec, original);
        assert_eq!(bytes_read, enc.len());
    }

    // -----------------------------------------------------------------------
    // Test 13: Enum with #[oxicode(crate = "oxicode")] — all variant kinds
    // Distinct from derive_crate_path_test.rs by exercising named + tuple + unit
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(crate = "oxicode")]
    #[allow(clippy::enum_variant_names)]
    enum ExplicitCrateEnum {
        UnitVariant,
        TupleVariant(f64, f64),
        NamedVariant {
            radius: f64,
            center_x: f64,
            center_y: f64,
        },
    }

    #[test]
    fn test_explicit_crate_attr_enum_all_variant_kinds() {
        let cases = vec![
            ExplicitCrateEnum::UnitVariant,
            ExplicitCrateEnum::TupleVariant(PI, E),
            ExplicitCrateEnum::NamedVariant {
                radius: PI,
                center_x: 0.0,
                center_y: 0.0,
            },
        ];
        for case in cases {
            let enc = encode_to_vec(&case).expect("encode crate attr enum");
            let (dec, bytes_read): (ExplicitCrateEnum, usize) =
                decode_from_slice(&enc).expect("decode crate attr enum");
            assert_eq!(dec, case);
            assert_eq!(bytes_read, enc.len());
        }
    }

    // -----------------------------------------------------------------------
    // Test 14: Generic struct with container bound override — Vec<T> element
    // Distinct from derive_bound_test.rs by using PI/E values and a Vec payload
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(bound = "T: Encode + Decode + core::fmt::Debug + PartialEq")]
    struct BoundWithMathPayload<T> {
        samples: Vec<T>,
        pi_scale: f64,
        e_scale: f64,
    }

    #[test]
    fn test_generic_struct_custom_bound_with_math_payload() {
        let original = BoundWithMathPayload {
            samples: vec![1u32, 2, 4, 8, 16],
            pi_scale: PI,
            e_scale: E,
        };
        let enc = encode_to_vec(&original).expect("encode bound override generic");
        let (dec, bytes_read): (BoundWithMathPayload<u32>, usize) =
            decode_from_slice(&enc).expect("decode bound override generic");
        assert_eq!(dec, original);
        assert_eq!(bytes_read, enc.len());
    }

    // -----------------------------------------------------------------------
    // Test 15: Container attrs don't interfere with field-level attrs
    // rename_all + field-level skip: skipped field decodes as Default
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "camelCase")]
    struct ContainerWithSkippedField {
        active_count: u32,
        #[oxicode(skip)]
        internal_cache: u64, // not encoded; must decode as Default (0)
        record_label: String,
    }

    #[test]
    fn test_container_rename_all_does_not_interfere_with_field_skip() {
        let original = ContainerWithSkippedField {
            active_count: 42,
            internal_cache: 0xFFFF_FFFF_FFFF_FFFF, // will NOT be encoded
            record_label: "integration".to_string(),
        };
        let enc = encode_to_vec(&original).expect("encode container+skip");
        let (dec, bytes_read): (ContainerWithSkippedField, usize) =
            decode_from_slice(&enc).expect("decode container+skip");
        assert_eq!(dec.active_count, original.active_count);
        assert_eq!(
            dec.internal_cache, 0u64,
            "skipped field must decode as 0 (Default)"
        );
        assert_eq!(dec.record_label, original.record_label);
        assert_eq!(bytes_read, enc.len());
    }

    // -----------------------------------------------------------------------
    // Test 16: Unit struct with container attrs — rename_all no-op
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
    struct UnitStructWithContainerAttr;

    #[test]
    fn test_unit_struct_with_rename_all_container_attr() {
        let original = UnitStructWithContainerAttr;
        let enc = encode_to_vec(&original).expect("encode unit struct with container attr");
        let (dec, bytes_read): (UnitStructWithContainerAttr, usize) =
            decode_from_slice(&enc).expect("decode unit struct with container attr");
        assert_eq!(dec, original);
        assert_eq!(bytes_read, enc.len());
        // Unit struct encodes to 0 bytes
        assert_eq!(enc.len(), 0, "unit struct should encode to 0 bytes");
    }

    // -----------------------------------------------------------------------
    // Test 17: Enum with 1 variant and container attrs
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u8", rename_all = "camelCase")]
    enum SingleVariantEnum {
        OnlyOption { value: f64, label: String },
    }

    #[test]
    fn test_single_variant_enum_with_container_attrs() {
        let original = SingleVariantEnum::OnlyOption {
            value: PI,
            label: "solo".to_string(),
        };
        let enc = encode_to_vec(&original).expect("encode single-variant enum");
        let (dec, bytes_read): (SingleVariantEnum, usize) =
            decode_from_slice(&enc).expect("decode single-variant enum");
        assert_eq!(dec, original);
        assert_eq!(bytes_read, enc.len());

        // In fixed mode the single u8 discriminant is exactly 1 byte + payload
        let fixed_bytes = oxicode::encode_to_vec_with_config(&original, oxicode::config::legacy())
            .expect("encode fixed single-variant");
        // Discriminant (1 byte u8) + f64 (8 bytes) + string length + string bytes
        assert!(
            fixed_bytes.len() >= 9,
            "must have at least 1 byte tag + 8 byte f64"
        );
        assert_eq!(fixed_bytes[0], 0u8, "single variant discriminant must be 0");
    }

    // -----------------------------------------------------------------------
    // Test 18: Enum discriminant tag_type=u8 with high discriminant value 250
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u8")]
    enum U8HighDiscriminantEnum {
        #[oxicode(variant = 250)]
        HighValue,
        #[oxicode(variant = 251)]
        EvenHigher(u32),
    }

    #[test]
    fn test_tag_type_u8_high_discriminant_250_is_one_byte() {
        let fixed_bytes = oxicode::encode_to_vec_with_config(
            &U8HighDiscriminantEnum::HighValue,
            oxicode::config::legacy(),
        )
        .expect("encode fixed high discriminant");
        assert_eq!(
            fixed_bytes.len(),
            1,
            "u8 discriminant 250 must fit in exactly 1 byte"
        );
        assert_eq!(fixed_bytes[0], 250u8, "discriminant byte must equal 250");

        // Roundtrip with standard config
        let enc = encode_to_vec(&U8HighDiscriminantEnum::HighValue).expect("encode");
        let (dec, _): (U8HighDiscriminantEnum, usize) =
            decode_from_slice(&enc).expect("decode high discriminant");
        assert_eq!(dec, U8HighDiscriminantEnum::HighValue);

        let enc2 = encode_to_vec(&U8HighDiscriminantEnum::EvenHigher(42)).expect("encode");
        let (dec2, _): (U8HighDiscriminantEnum, usize) =
            decode_from_slice(&enc2).expect("decode even higher");
        assert_eq!(dec2, U8HighDiscriminantEnum::EvenHigher(42));
    }

    // -----------------------------------------------------------------------
    // Test 19: Enum discriminant tag_type=u16 with value 1000 — 2 bytes in fixed mode
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u16")]
    enum U16HighDiscriminantEnum {
        #[oxicode(variant = 1000)]
        HighValue,
        #[oxicode(variant = 1001)]
        AlsoHigh(f64),
    }

    #[test]
    fn test_tag_type_u16_discriminant_1000_is_two_bytes() {
        let fixed_bytes = oxicode::encode_to_vec_with_config(
            &U16HighDiscriminantEnum::HighValue,
            oxicode::config::legacy(),
        )
        .expect("encode fixed u16 discriminant 1000");
        assert_eq!(
            fixed_bytes.len(),
            2,
            "u16 discriminant 1000 must be exactly 2 bytes"
        );
        // Little-endian: 1000 = 0x03E8 → [0xE8, 0x03]
        assert_eq!(
            u16::from_le_bytes([fixed_bytes[0], fixed_bytes[1]]),
            1000u16,
            "discriminant value must be 1000"
        );

        let enc = encode_to_vec(&U16HighDiscriminantEnum::AlsoHigh(PI)).expect("encode");
        let (dec, _): (U16HighDiscriminantEnum, usize) =
            decode_from_slice(&enc).expect("decode u16 high discriminant");
        assert_eq!(dec, U16HighDiscriminantEnum::AlsoHigh(PI));
    }

    // -----------------------------------------------------------------------
    // Test 20: Struct with rename_all + individual field rename (field rename wins)
    // Uses f64 constants to ensure values are preserved correctly
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
    struct RenameAllWithFieldOverride {
        #[oxicode(rename = "pi")]
        pi_value: f64,
        // This field uses the rename_all transformation (no individual rename)
        euler_constant: f64,
        #[oxicode(rename = "n")]
        iteration_count: u32,
    }

    #[test]
    fn test_rename_all_with_field_level_rename_override() {
        let original = RenameAllWithFieldOverride {
            pi_value: PI,
            euler_constant: E,
            iteration_count: 42,
        };
        let enc = encode_to_vec(&original).expect("encode rename_all+field override");
        let (dec, bytes_read): (RenameAllWithFieldOverride, usize) =
            decode_from_slice(&enc).expect("decode rename_all+field override");
        assert_eq!(dec, original);
        assert_eq!(bytes_read, enc.len());

        // Wire format is same as an identical struct without any rename attrs
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct EquivalentNoRename {
            pi_value: f64,
            euler_constant: f64,
            iteration_count: u32,
        }
        let baseline = EquivalentNoRename {
            pi_value: PI,
            euler_constant: E,
            iteration_count: 42,
        };
        let baseline_bytes = encode_to_vec(&baseline).expect("encode baseline");
        assert_eq!(
            enc, baseline_bytes,
            "rename attrs must not change wire bytes"
        );
    }

    // -----------------------------------------------------------------------
    // Test 21: Multiple structs with the same container attr are independent
    // Each struct's container attr is self-contained and doesn't affect others
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "camelCase")]
    struct IndependentStructA {
        field_one: u32,
        field_two: String,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(rename_all = "camelCase")]
    struct IndependentStructB {
        field_one: f64,
        field_two: bool,
    }

    #[test]
    fn test_multiple_structs_same_container_attr_are_independent() {
        let a = IndependentStructA {
            field_one: 100,
            field_two: "struct-a".to_string(),
        };
        let b = IndependentStructB {
            field_one: PI,
            field_two: true,
        };

        let enc_a = encode_to_vec(&a).expect("encode IndependentStructA");
        let enc_b = encode_to_vec(&b).expect("encode IndependentStructB");

        // They should decode back independently
        let (dec_a, bytes_a): (IndependentStructA, usize) =
            decode_from_slice(&enc_a).expect("decode IndependentStructA");
        let (dec_b, bytes_b): (IndependentStructB, usize) =
            decode_from_slice(&enc_b).expect("decode IndependentStructB");

        assert_eq!(dec_a, a);
        assert_eq!(dec_b, b);
        assert_eq!(bytes_a, enc_a.len());
        assert_eq!(bytes_b, enc_b.len());

        // Encoding one struct does not affect the other's bytes
        let enc_a2 = encode_to_vec(&a).expect("re-encode IndependentStructA");
        assert_eq!(enc_a, enc_a2, "re-encoding must produce identical bytes");
    }

    // -----------------------------------------------------------------------
    // Test 22: Container attr on tuple struct — rename_all + crate path
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(crate = "oxicode", rename_all = "camelCase")]
    struct TupleStructWithContainerAttrs(f64, f64, u32);

    #[test]
    fn test_tuple_struct_with_crate_and_rename_all_attrs() {
        let original = TupleStructWithContainerAttrs(PI, E, 42);
        let enc = encode_to_vec(&original).expect("encode tuple struct with container attrs");
        let (dec, bytes_read): (TupleStructWithContainerAttrs, usize) =
            decode_from_slice(&enc).expect("decode tuple struct with container attrs");
        assert_eq!(dec, original);
        assert_eq!(bytes_read, enc.len());

        // Verify wire format matches a plain tuple struct with same fields
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct PlainTupleStruct(f64, f64, u32);

        let plain = PlainTupleStruct(PI, E, 42);
        let plain_bytes = encode_to_vec(&plain).expect("encode plain tuple struct");
        assert_eq!(
            enc, plain_bytes,
            "container attrs on tuple struct must not change wire bytes"
        );
    }
}
