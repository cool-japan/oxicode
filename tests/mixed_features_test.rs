//! Tests verifying that multiple features work correctly in combination.

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
// Test versioning + derive + checksum
#[cfg(all(feature = "checksum", feature = "derive"))]
#[test]
fn test_versioned_checksum_derive_struct() {
    use oxicode::versioning::Version;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct VersionedData {
        id: u64,
        payload: Vec<u8>,
        version: u32,
    }

    let data = VersionedData {
        id: 1,
        payload: vec![1, 2, 3],
        version: 42,
    };

    // Encode with version header
    let versioned =
        oxicode::encode_versioned_value(&data, Version::new(1, 0, 0)).expect("encode versioned");

    // Wrap the versioned bytes with a checksum
    let checked = oxicode::checksum::wrap_with_checksum(&versioned);

    // Verify checksum to get back the versioned bytes
    let ver_data = oxicode::checksum::verify_checksum(&checked).expect("verify checksum");

    // Decode versioned payload
    let (decoded, ver, _): (VersionedData, _, _) =
        oxicode::decode_versioned_value(ver_data).expect("decode versioned");

    assert_eq!(data, decoded);
    assert_eq!(ver, Version::new(1, 0, 0));
}

// Test compression + derive
#[cfg(all(feature = "compression-lz4", feature = "derive"))]
#[test]
fn test_compression_derive_large_struct() {
    use oxicode::compression::{compress, decompress, Compression};
    use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LargeRecord {
        id: u64,
        data: Vec<u8>,
        tags: Vec<String>,
    }

    let record = LargeRecord {
        id: 42,
        data: vec![0xABu8; 10_000], // Highly compressible
        tags: (0..100).map(|i| format!("tag_{}", i)).collect(),
    };

    let enc = encode_to_vec(&record).expect("encode");
    let compressed = compress(&enc, Compression::Lz4).expect("compress");

    assert!(compressed.len() < enc.len(), "should compress");

    let decompressed = decompress(&compressed).expect("decompress");
    let (decoded, _): (LargeRecord, _) = decode_from_slice(&decompressed).expect("decode");
    assert_eq!(record, decoded);
}

// Test serde + validate
#[cfg(all(feature = "serde", feature = "derive"))]
#[test]
fn test_serde_with_validation() {
    use oxicode::validation::{Constraints, Validator};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct UserInput {
        username: String,
        age: u8,
    }

    let user = UserInput {
        username: "alice".to_string(),
        age: 25,
    };
    let bytes = oxicode::serde::encode_serde(&user).expect("encode");
    let decoded: UserInput = oxicode::serde::decode_serde(&bytes).expect("decode");

    // Validate after decode
    let validator: Validator<String> = Validator::new()
        .constraint("username", Constraints::max_len(50))
        .constraint("username", Constraints::min_len(3));

    validator
        .validate(&decoded.username)
        .expect("validate username");
    assert_eq!(user, decoded);
}

// Test async streaming + derived types
#[cfg(all(feature = "async-tokio", feature = "derive"))]
#[tokio::test]
async fn test_async_with_complex_derived_type() {
    use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder};
    use oxicode::{Decode, Encode};
    use std::collections::BTreeMap;
    use std::io::Cursor;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ComplexMsg {
        id: u64,
        data: Vec<Vec<u8>>,
        tags: BTreeMap<String, String>,
    }

    let msg = ComplexMsg {
        id: 99,
        data: vec![vec![1, 2, 3], vec![4, 5, 6]],
        tags: {
            let mut m = BTreeMap::new();
            m.insert("key".to_string(), "value".to_string());
            m
        },
    };

    let mut buf = Vec::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncStreamingEncoder::new(cursor);
        encoder.write_item(&msg).await.expect("encode async");
        encoder.finish().await.expect("finish async");
    }

    let cursor = Cursor::new(buf);
    let mut decoder = AsyncStreamingDecoder::new(cursor);
    let decoded: Option<ComplexMsg> = decoder.read_item().await.expect("decode async");
    assert_eq!(Some(msg), decoded);
}
