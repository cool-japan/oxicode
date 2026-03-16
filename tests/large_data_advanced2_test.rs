//! Advanced large data encoding tests for OxiCode

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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Block {
    index: u32,
    data: Vec<u8>,
    hash: [u8; 32],
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Chain {
    blocks: Vec<Block>,
    height: u64,
}

fn lcg_bytes(count: usize) -> Vec<u8> {
    let mut state = 12345u64;
    (0..count)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as u8
        })
        .collect()
}

#[test]
fn test_large_vec_u8_100k_roundtrip() {
    let original: Vec<u8> = lcg_bytes(100_000);
    let encoded = encode_to_vec(&original).expect("encode 100k bytes");
    let (decoded, _): (Vec<u8>, _) = decode_from_slice(&encoded).expect("decode 100k bytes");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_vec_u32_10k_roundtrip() {
    let original: Vec<u32> = (0u32..10_000).collect();
    let encoded = encode_to_vec(&original).expect("encode 10k u32");
    let (decoded, _): (Vec<u32>, _) = decode_from_slice(&encoded).expect("decode 10k u32");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_vec_u64_5k_roundtrip() {
    let original: Vec<u64> = (0u64..5_000).map(|i| i * 1_000_000_007).collect();
    let encoded = encode_to_vec(&original).expect("encode 5k u64");
    let (decoded, _): (Vec<u64>, _) = decode_from_slice(&encoded).expect("decode 5k u64");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_vec_string_1k_roundtrip() {
    let original: Vec<String> = (0..1_000).map(|i| format!("{:0>100}", i)).collect();
    let encoded = encode_to_vec(&original).expect("encode 1k strings");
    let (decoded, _): (Vec<String>, _) = decode_from_slice(&encoded).expect("decode 1k strings");
    assert_eq!(original, decoded);
}

#[test]
fn test_block_4096_bytes_payload_roundtrip() {
    let payload = lcg_bytes(4096);
    let original = Block {
        index: 42,
        data: payload,
        hash: [0xAB; 32],
    };
    let encoded = encode_to_vec(&original).expect("encode block with 4096 bytes");
    let (decoded, _): (Block, _) =
        decode_from_slice(&encoded).expect("decode block with 4096 bytes");
    assert_eq!(original, decoded);
}

#[test]
fn test_chain_100_blocks_256_bytes_each_roundtrip() {
    let blocks: Vec<Block> = (0..100)
        .map(|i| {
            let data = lcg_bytes(256);
            let mut hash = [0u8; 32];
            hash[0] = (i % 256) as u8;
            hash[31] = (i / 256 % 256) as u8;
            Block {
                index: i as u32,
                data,
                hash,
            }
        })
        .collect();
    let original = Chain {
        blocks,
        height: 100,
    };
    let encoded = encode_to_vec(&original).expect("encode chain 100 blocks");
    let (decoded, _): (Chain, _) = decode_from_slice(&encoded).expect("decode chain 100 blocks");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_block_50_roundtrip() {
    let original: Vec<Block> = (0..50)
        .map(|i| {
            let data = lcg_bytes(128);
            let mut hash = [0u8; 32];
            for (j, b) in hash.iter_mut().enumerate() {
                *b = ((i + j) % 256) as u8;
            }
            Block {
                index: i as u32,
                data,
                hash,
            }
        })
        .collect();
    let encoded = encode_to_vec(&original).expect("encode vec of 50 blocks");
    let (decoded, _): (Vec<Block>, _) =
        decode_from_slice(&encoded).expect("decode vec of 50 blocks");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_string_100k_chars_roundtrip() {
    let original: String = std::iter::repeat('x').take(100_000).collect();
    let encoded = encode_to_vec(&original).expect("encode 100k char string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode 100k char string");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_vec_u8_100_inner_vecs_roundtrip() {
    let original: Vec<Vec<u8>> = (0..100)
        .map(|i| {
            let mut v = lcg_bytes(100);
            v[0] = i as u8;
            v
        })
        .collect();
    let encoded = encode_to_vec(&original).expect("encode vec of vec<u8>");
    let (decoded, _): (Vec<Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode vec of vec<u8>");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_vec_consumed_bytes_equals_encoded_len() {
    let data: Vec<u8> = lcg_bytes(50_000);
    let encoded = encode_to_vec(&data).expect("encode for consumed bytes test");
    let encoded_len = encoded.len();
    let (_, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode for consumed bytes test");
    assert_eq!(
        consumed, encoded_len,
        "consumed bytes should equal total encoded length"
    );
}

#[test]
fn test_large_vec_i32_20k_roundtrip() {
    let original: Vec<i32> = (0i32..20_000).map(|i| i * 3 - 30_000).collect();
    let encoded = encode_to_vec(&original).expect("encode 20k i32");
    let (decoded, _): (Vec<i32>, _) = decode_from_slice(&encoded).expect("decode 20k i32");
    assert_eq!(original, decoded);
}

#[test]
fn test_fixed_array_1024_roundtrip() {
    let original: [u8; 1024] = {
        let bytes = lcg_bytes(1024);
        let mut arr = [0u8; 1024];
        arr.copy_from_slice(&bytes);
        arr
    };
    let encoded = encode_to_vec(&original).expect("encode [u8; 1024]");
    let (decoded, _): ([u8; 1024], _) = decode_from_slice(&encoded).expect("decode [u8; 1024]");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_vec_f64_10k_roundtrip() {
    let original: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
    let encoded = encode_to_vec(&original).expect("encode 10k f64");
    let (decoded, _): (Vec<f64>, _) = decode_from_slice(&encoded).expect("decode 10k f64");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_vec_bool_10k_roundtrip() {
    let original: Vec<bool> = (0..10_000).map(|i| i % 3 == 0).collect();
    let encoded = encode_to_vec(&original).expect("encode 10k bools");
    let (decoded, _): (Vec<bool>, _) = decode_from_slice(&encoded).expect("decode 10k bools");
    assert_eq!(original, decoded);
}

#[test]
fn test_large_data_fixed_int_config_roundtrip() {
    let original: Vec<u32> = (0u32..1_000).collect();
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode with fixed int config");
    let (decoded, _): (Vec<u32>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed int config");
    assert_eq!(original, decoded);
}

#[test]
fn test_chain_1000_empty_blocks_roundtrip() {
    let blocks: Vec<Block> = (0..1_000)
        .map(|i| Block {
            index: i as u32,
            data: Vec::new(),
            hash: [0u8; 32],
        })
        .collect();
    let original = Chain {
        blocks,
        height: 1_000,
    };
    let encoded = encode_to_vec(&original).expect("encode chain with 1000 empty blocks");
    let (decoded, _): (Chain, _) =
        decode_from_slice(&encoded).expect("decode chain with 1000 empty blocks");
    assert_eq!(original, decoded);
}

#[test]
fn test_block_max_hash_all_ff_roundtrip() {
    let original = Block {
        index: u32::MAX,
        data: lcg_bytes(512),
        hash: [0xFF; 32],
    };
    let encoded = encode_to_vec(&original).expect("encode block with 0xFF hash");
    let (decoded, _): (Block, _) =
        decode_from_slice(&encoded).expect("decode block with 0xFF hash");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u8_1_000_000_bytes_roundtrip() {
    let original: Vec<u8> = lcg_bytes(1_000_000);
    let encoded = encode_to_vec(&original).expect("encode 1M bytes");
    let (decoded, _): (Vec<u8>, _) = decode_from_slice(&encoded).expect("decode 1M bytes");
    assert_eq!(original, decoded);
}

#[test]
fn test_encode_size_1m_bytes_greater_than_1m() {
    let data: Vec<u8> = lcg_bytes(1_000_000);
    let encoded = encode_to_vec(&data).expect("encode 1M bytes for size check");
    assert!(
        encoded.len() > 1_000_000,
        "encoded length {} should exceed 1_000_000 due to length prefix",
        encoded.len()
    );
}

#[test]
fn test_vec_fixed_array_u8_32_100_roundtrip() {
    let original: Vec<[u8; 32]> = (0u8..100)
        .map(|i| {
            let mut arr = [0u8; 32];
            for (j, b) in arr.iter_mut().enumerate() {
                *b = i.wrapping_add(j as u8);
            }
            arr
        })
        .collect();
    let encoded = encode_to_vec(&original).expect("encode vec of [u8; 32]");
    let (decoded, _): (Vec<[u8; 32]>, _) =
        decode_from_slice(&encoded).expect("decode vec of [u8; 32]");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_u32_repetitive_data_5000_roundtrip() {
    let original: Vec<u32> = vec![0xDEADBEEFu32; 5_000];
    let encoded = encode_to_vec(&original).expect("encode repetitive vec<u32>");
    let (decoded, _): (Vec<u32>, _) =
        decode_from_slice(&encoded).expect("decode repetitive vec<u32>");
    assert_eq!(original, decoded);
}

#[test]
fn test_multiple_roundtrips_give_identical_encoded_bytes() {
    let data: Vec<u8> = lcg_bytes(10_000);
    let encoded1 = encode_to_vec(&data).expect("first encode");
    let encoded2 = encode_to_vec(&data).expect("second encode");
    let encoded3 = encode_to_vec(&data).expect("third encode");
    assert_eq!(encoded1, encoded2, "first and second encodings must match");
    assert_eq!(encoded2, encoded3, "second and third encodings must match");
    let (decoded, _): (Vec<u8>, _) =
        decode_from_slice(&encoded1).expect("decode after triple encode check");
    assert_eq!(data, decoded);
}
