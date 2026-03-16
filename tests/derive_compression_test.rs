//! Tests that derived structs work correctly with compression.

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
#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
mod compression_derive_tests {
    use oxicode::{compression, compression::Compression, Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LargeData {
        name: String,
        values: Vec<u64>,
        metadata: Vec<String>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum Command {
        Store { key: String, value: Vec<u8> },
        Delete(String),
        List,
    }

    #[cfg(feature = "compression-lz4")]
    #[test]
    fn test_derived_struct_lz4_roundtrip() {
        let data = LargeData {
            name: "test".to_string(),
            values: (0..1000).map(|i| i * i).collect(),
            metadata: (0..50).map(|i| format!("meta_{}", i)).collect(),
        };
        let enc = oxicode::encode_to_vec(&data).expect("encode");
        let compressed = compression::compress(&enc, Compression::Lz4).expect("compress");
        let decompressed = compression::decompress(&compressed).expect("decompress");
        let (dec, _): (LargeData, _) = oxicode::decode_from_slice(&decompressed).expect("decode");
        assert_eq!(data, dec);
        // Should actually compress
        assert!(
            compressed.len() < enc.len(),
            "compression should reduce size for repetitive data"
        );
    }

    #[cfg(feature = "compression-lz4")]
    #[test]
    fn test_derived_enum_lz4_roundtrip() {
        let cmds = vec![
            Command::Store {
                key: "key1".to_string(),
                value: vec![1, 2, 3],
            },
            Command::Delete("key2".to_string()),
            Command::List,
        ];
        for cmd in cmds {
            let enc = oxicode::encode_to_vec(&cmd).expect("encode");
            let compressed = compression::compress(&enc, Compression::Lz4).expect("compress");
            let decompressed = compression::decompress(&compressed).expect("decompress");
            let (dec, _): (Command, _) = oxicode::decode_from_slice(&decompressed).expect("decode");
            assert_eq!(cmd, dec);
        }
    }
}
