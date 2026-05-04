//! Compatibility tests between oxicode and bincode
//!
//! This crate verifies that oxicode produces binary-compatible output with bincode.

#[cfg(test)]
mod tests {

    /// Test that oxicode and bincode produce identical binary output for primitives
    #[test]
    fn test_primitive_u32_binary_identical() {
        let value = 42u32;

        // Encode with oxicode
        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");

        // Encode with bincode
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Binary output should be identical for u32"
        );

        // Verify cross-decoding
        let (oxi_from_bin, _): (u32, _) =
            oxicode::decode_from_slice(&bin_bytes).expect("oxicode decode bincode failed");
        let (bin_from_oxi, _): (u32, _) = bincode::decode_from_slice(&oxi_bytes, bin_config)
            .expect("bincode decode oxicode failed");

        assert_eq!(value, oxi_from_bin);
        assert_eq!(value, bin_from_oxi);
    }

    #[test]
    fn test_primitive_i64_binary_identical() {
        let value = -12345i64;

        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Binary output should be identical for i64"
        );
    }

    #[test]
    fn test_string_binary_identical() {
        let value = "Hello, bincode! 🦀".to_string();

        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(&value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Binary output should be identical for String"
        );

        // Cross-decode
        let (decoded, _): (String, _) =
            oxicode::decode_from_slice(&bin_bytes).expect("Failed to decode");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_vec_binary_identical() {
        let value = vec![1u32, 2, 3, 4, 5];

        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(&value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Binary output should be identical for Vec<u32>"
        );
    }

    #[test]
    fn test_option_some_binary_identical() {
        let value: Option<u64> = Some(999);

        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Binary output should be identical for Option::Some"
        );
    }

    #[test]
    fn test_option_none_binary_identical() {
        let value: Option<u64> = None;

        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Binary output should be identical for Option::None"
        );
    }

    #[test]
    fn test_tuple_binary_identical() {
        let value = (42u32, "test".to_string(), true);

        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(&value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Binary output should be identical for tuples"
        );
    }

    #[test]
    fn test_legacy_config_compatibility() {
        let value = 65535u32; // Requires u32 in fixed encoding

        // oxicode with legacy config
        let oxi_config = oxicode::config::legacy();
        let oxi_bytes =
            oxicode::encode_to_vec_with_config(&value, oxi_config).expect("oxicode encode failed");

        // bincode with legacy config
        let bin_config = bincode::config::legacy();
        let bin_bytes = bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Legacy config should produce identical output"
        );
    }

    #[test]
    fn test_big_endian_compatibility() {
        let value = 0x12345678u32;

        let oxi_config = oxicode::config::standard().with_big_endian();
        let oxi_bytes =
            oxicode::encode_to_vec_with_config(&value, oxi_config).expect("oxicode encode failed");

        let bin_config = bincode::config::standard().with_big_endian();
        let bin_bytes = bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Big-endian encoding should be identical"
        );
    }

    #[test]
    fn test_fixed_int_encoding_compatibility() {
        let value = 250u32; // At varint boundary

        let oxi_config = oxicode::config::standard().with_fixed_int_encoding();
        let oxi_bytes =
            oxicode::encode_to_vec_with_config(&value, oxi_config).expect("oxicode encode failed");

        let bin_config = bincode::config::standard().with_fixed_int_encoding();
        let bin_bytes = bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Fixed-int encoding should be identical"
        );
    }

    // Separate structs for bincode and oxicode to avoid trait conflicts
    mod bincode_types {
        use bincode::{Decode as BDecode, Encode as BEncode};

        #[derive(Debug, PartialEq, BEncode, BDecode)]
        pub struct TestStruct {
            pub id: u32,
            pub name: String,
            pub active: bool,
        }

        #[derive(Debug, PartialEq, BEncode, BDecode)]
        pub enum TestEnum {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }
    }

    mod oxicode_types {
        use oxicode::{Decode as ODecode, Encode as OEncode};

        #[derive(Debug, PartialEq, OEncode, ODecode)]
        pub struct TestStruct {
            pub id: u32,
            pub name: String,
            pub active: bool,
        }

        #[derive(Debug, PartialEq, OEncode, ODecode)]
        pub enum TestEnum {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }
    }

    #[test]
    fn test_struct_binary_identical() {
        let bin_value = bincode_types::TestStruct {
            id: 123,
            name: "test struct".to_string(),
            active: true,
        };

        let oxi_value = oxicode_types::TestStruct {
            id: 123,
            name: "test struct".to_string(),
            active: true,
        };

        let oxi_bytes = oxicode::encode_to_vec(&oxi_value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes =
            bincode::encode_to_vec(&bin_value, bin_config).expect("bincode encode failed");

        assert_eq!(oxi_bytes, bin_bytes, "Struct encoding should be identical");
    }

    #[test]
    fn test_enum_unit_variant_binary_identical() {
        let bin_value = bincode_types::TestEnum::Quit;
        let oxi_value = oxicode_types::TestEnum::Quit;

        let oxi_bytes = oxicode::encode_to_vec(&oxi_value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes =
            bincode::encode_to_vec(&bin_value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Enum unit variant should be identical"
        );
    }

    #[test]
    fn test_enum_struct_variant_binary_identical() {
        let bin_value = bincode_types::TestEnum::Move { x: 10, y: 20 };
        let oxi_value = oxicode_types::TestEnum::Move { x: 10, y: 20 };

        let oxi_bytes = oxicode::encode_to_vec(&oxi_value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes =
            bincode::encode_to_vec(&bin_value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Enum struct variant should be identical"
        );
    }

    #[test]
    fn test_enum_tuple_variant_binary_identical() {
        let bin_value = bincode_types::TestEnum::Write("message".to_string());
        let oxi_value = oxicode_types::TestEnum::Write("message".to_string());

        let oxi_bytes = oxicode::encode_to_vec(&oxi_value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes =
            bincode::encode_to_vec(&bin_value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Enum tuple variant should be identical"
        );
    }

    #[test]
    fn test_complex_nested_binary_identical() {
        type Complex = Vec<Option<(String, Vec<u32>)>>;

        let value: Complex = vec![
            Some(("first".to_string(), vec![1, 2, 3])),
            None,
            Some(("second".to_string(), vec![4, 5])),
        ];

        let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
        let bin_config = bincode::config::standard();
        let bin_bytes = bincode::encode_to_vec(&value, bin_config).expect("bincode encode failed");

        assert_eq!(
            oxi_bytes, bin_bytes,
            "Complex nested types should produce identical binary"
        );

        // Verify cross-decoding
        let (oxi_decoded, _): (Complex, _) =
            oxicode::decode_from_slice(&bin_bytes).expect("oxicode decode bincode failed");
        let (bin_decoded, _): (Complex, _) = bincode::decode_from_slice(&oxi_bytes, bin_config)
            .expect("bincode decode oxicode failed");

        assert_eq!(value, oxi_decoded);
        assert_eq!(value, bin_decoded);
    }

    #[test]
    fn test_char_utf8_binary_identical() {
        // Test various Unicode characters
        let chars = vec!['a', 'é', '中', '🦀', '\u{10FFFF}'];

        for ch in chars {
            let oxi_bytes = oxicode::encode_to_vec(&ch).expect("oxicode encode failed");
            let bin_config = bincode::config::standard();
            let bin_bytes = bincode::encode_to_vec(ch, bin_config).expect("bincode encode failed");

            assert_eq!(
                oxi_bytes, bin_bytes,
                "Char '{}' should encode identically",
                ch
            );

            // Cross-decode
            let (decoded, _): (char, _) =
                oxicode::decode_from_slice(&bin_bytes).expect("decode failed");
            assert_eq!(ch, decoded);
        }
    }

    #[test]
    fn test_varint_boundary_values() {
        // Test varint boundary values
        let values = vec![
            0u64,
            250,         // Last single-byte value
            251,         // First u16 value
            65535,       // Max u16
            65536,       // First u32
            0xFFFFFFFF,  // Max u32
            0x100000000, // First u64
        ];

        for value in values {
            let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
            let bin_config = bincode::config::standard();
            let bin_bytes =
                bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

            assert_eq!(
                oxi_bytes, bin_bytes,
                "Varint encoding for {} should be identical",
                value
            );
        }
    }

    #[test]
    fn test_zigzag_encoding() {
        // Test zigzag encoding for signed integers
        let values = vec![0i64, -1, 1, -2, 2, -1000, 1000, i64::MIN, i64::MAX];

        for value in values {
            let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
            let bin_config = bincode::config::standard();
            let bin_bytes =
                bincode::encode_to_vec(value, bin_config).expect("bincode encode failed");

            assert_eq!(
                oxi_bytes, bin_bytes,
                "Zigzag encoding for {} should be identical",
                value
            );
        }
    }
}
