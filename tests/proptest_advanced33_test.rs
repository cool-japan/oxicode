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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KeyPair {
    public_key: Vec<u8>,
    key_id: u64,
    algorithm: u8,
    created_at: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SignatureAlgorithm {
    Ed25519,
    Secp256k1,
    Rsa2048,
    Rsa4096,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SignedMessage {
    payload: Vec<u8>,
    signature: Vec<u8>,
    algorithm: SignatureAlgorithm,
    signer_id: u64,
}

// Test 1: KeyPair roundtrip with various field values
proptest! {
    #[test]
    fn test_keypair_roundtrip(
        public_key in proptest::collection::vec(any::<u8>(), 0..64),
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val = KeyPair { public_key, key_id, algorithm, created_at };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<KeyPair>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 2: SignatureAlgorithm::Ed25519 roundtrip
proptest! {
    #[test]
    fn test_signature_algorithm_ed25519_roundtrip(_dummy in any::<u8>()) {
        let val = SignatureAlgorithm::Ed25519;
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<SignatureAlgorithm>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 3: SignatureAlgorithm::Secp256k1 roundtrip
proptest! {
    #[test]
    fn test_signature_algorithm_secp256k1_roundtrip(_dummy in any::<u8>()) {
        let val = SignatureAlgorithm::Secp256k1;
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<SignatureAlgorithm>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 4: SignatureAlgorithm::Rsa2048 roundtrip
proptest! {
    #[test]
    fn test_signature_algorithm_rsa2048_roundtrip(_dummy in any::<u8>()) {
        let val = SignatureAlgorithm::Rsa2048;
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<SignatureAlgorithm>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 5: SignatureAlgorithm::Rsa4096 roundtrip (variant index in 0u8..4u8)
proptest! {
    #[test]
    fn test_signature_algorithm_rsa4096_roundtrip(variant_index in 0u8..4u8) {
        let val = match variant_index % 4 {
            0 => SignatureAlgorithm::Ed25519,
            1 => SignatureAlgorithm::Secp256k1,
            2 => SignatureAlgorithm::Rsa2048,
            _ => SignatureAlgorithm::Rsa4096,
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<SignatureAlgorithm>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 6: SignedMessage roundtrip
proptest! {
    #[test]
    fn test_signed_message_roundtrip(
        payload in proptest::collection::vec(any::<u8>(), 0..128),
        signature in proptest::collection::vec(any::<u8>(), 0..128),
        variant_index in 0u8..4u8,
        signer_id in any::<u64>(),
    ) {
        let algorithm = match variant_index % 4 {
            0 => SignatureAlgorithm::Ed25519,
            1 => SignatureAlgorithm::Secp256k1,
            2 => SignatureAlgorithm::Rsa2048,
            _ => SignatureAlgorithm::Rsa4096,
        };
        let val = SignedMessage { payload, signature, algorithm, signer_id };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<SignedMessage>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 7: Vec<KeyPair> roundtrip (0..8 items)
proptest! {
    #[test]
    fn test_vec_keypair_roundtrip(
        items in proptest::collection::vec(
            (
                proptest::collection::vec(any::<u8>(), 0..32),
                any::<u64>(),
                any::<u8>(),
                any::<u64>(),
            ),
            0..8
        )
    ) {
        let val: Vec<KeyPair> = items
            .into_iter()
            .map(|(public_key, key_id, algorithm, created_at)| KeyPair {
                public_key,
                key_id,
                algorithm,
                created_at,
            })
            .collect();
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<Vec<KeyPair>>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 8: Vec<SignedMessage> roundtrip (0..4 items)
proptest! {
    #[test]
    fn test_vec_signed_message_roundtrip(
        items in proptest::collection::vec(
            (
                proptest::collection::vec(any::<u8>(), 0..64),
                proptest::collection::vec(any::<u8>(), 0..64),
                0u8..4u8,
                any::<u64>(),
            ),
            0..4
        )
    ) {
        let val: Vec<SignedMessage> = items
            .into_iter()
            .map(|(payload, signature, variant_index, signer_id)| {
                let algorithm = match variant_index % 4 {
                    0 => SignatureAlgorithm::Ed25519,
                    1 => SignatureAlgorithm::Secp256k1,
                    2 => SignatureAlgorithm::Rsa2048,
                    _ => SignatureAlgorithm::Rsa4096,
                };
                SignedMessage { payload, signature, algorithm, signer_id }
            })
            .collect();
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<Vec<SignedMessage>>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 9: Option<KeyPair> roundtrip
proptest! {
    #[test]
    fn test_option_keypair_roundtrip(
        present in any::<bool>(),
        public_key in proptest::collection::vec(any::<u8>(), 0..32),
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val: Option<KeyPair> = if present {
            Some(KeyPair { public_key, key_id, algorithm, created_at })
        } else {
            None
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<Option<KeyPair>>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 10: Option<SignedMessage> roundtrip
proptest! {
    #[test]
    fn test_option_signed_message_roundtrip(
        present in any::<bool>(),
        payload in proptest::collection::vec(any::<u8>(), 0..64),
        signature in proptest::collection::vec(any::<u8>(), 0..64),
        variant_index in 0u8..4u8,
        signer_id in any::<u64>(),
    ) {
        let val: Option<SignedMessage> = if present {
            let algorithm = match variant_index % 4 {
                0 => SignatureAlgorithm::Ed25519,
                1 => SignatureAlgorithm::Secp256k1,
                2 => SignatureAlgorithm::Rsa2048,
                _ => SignatureAlgorithm::Rsa4096,
            };
            Some(SignedMessage { payload, signature, algorithm, signer_id })
        } else {
            None
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<Option<SignedMessage>>(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// Test 11: KeyPair deterministic encoding
proptest! {
    #[test]
    fn test_keypair_deterministic_encoding(
        public_key in proptest::collection::vec(any::<u8>(), 0..32),
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val = KeyPair { public_key, key_id, algorithm, created_at };
        let bytes1 = encode_to_vec(&val).expect("encode failed");
        let bytes2 = encode_to_vec(&val).expect("encode failed");
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Test 12: SignedMessage deterministic encoding
proptest! {
    #[test]
    fn test_signed_message_deterministic_encoding(
        payload in proptest::collection::vec(any::<u8>(), 0..64),
        signature in proptest::collection::vec(any::<u8>(), 0..64),
        variant_index in 0u8..4u8,
        signer_id in any::<u64>(),
    ) {
        let algorithm = match variant_index % 4 {
            0 => SignatureAlgorithm::Ed25519,
            1 => SignatureAlgorithm::Secp256k1,
            2 => SignatureAlgorithm::Rsa2048,
            _ => SignatureAlgorithm::Rsa4096,
        };
        let val = SignedMessage { payload, signature, algorithm, signer_id };
        let bytes1 = encode_to_vec(&val).expect("encode failed");
        let bytes2 = encode_to_vec(&val).expect("encode failed");
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Test 13: Consumed bytes == encoded length for KeyPair
proptest! {
    #[test]
    fn test_keypair_consumed_bytes_equals_encoded_length(
        public_key in proptest::collection::vec(any::<u8>(), 0..32),
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val = KeyPair { public_key, key_id, algorithm, created_at };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let encoded_len = bytes.len();
        let (_, consumed) = decode_from_slice::<KeyPair>(&bytes).expect("decode failed");
        prop_assert_eq!(consumed, encoded_len);
    }
}

// Test 14: Consumed bytes == encoded length for SignedMessage
proptest! {
    #[test]
    fn test_signed_message_consumed_bytes_equals_encoded_length(
        payload in proptest::collection::vec(any::<u8>(), 0..64),
        signature in proptest::collection::vec(any::<u8>(), 0..64),
        variant_index in 0u8..4u8,
        signer_id in any::<u64>(),
    ) {
        let algorithm = match variant_index % 4 {
            0 => SignatureAlgorithm::Ed25519,
            1 => SignatureAlgorithm::Secp256k1,
            2 => SignatureAlgorithm::Rsa2048,
            _ => SignatureAlgorithm::Rsa4096,
        };
        let val = SignedMessage { payload, signature, algorithm, signer_id };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let encoded_len = bytes.len();
        let (_, consumed) = decode_from_slice::<SignedMessage>(&bytes).expect("decode failed");
        prop_assert_eq!(consumed, encoded_len);
    }
}

// Test 15: Empty public_key vec roundtrip
proptest! {
    #[test]
    fn test_empty_public_key_roundtrip(
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val = KeyPair {
            public_key: Vec::new(),
            key_id,
            algorithm,
            created_at,
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<KeyPair>(&bytes).expect("decode failed");
        prop_assert!(decoded.public_key.is_empty());
        prop_assert_eq!(val, decoded);
    }
}

// Test 16: Large public_key (32-byte public key) roundtrip
proptest! {
    #[test]
    fn test_large_public_key_32_bytes_roundtrip(
        public_key in proptest::collection::vec(any::<u8>(), 32..=32),
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val = KeyPair { public_key, key_id, algorithm, created_at };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<KeyPair>(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.public_key.len(), 32);
        prop_assert_eq!(val, decoded);
    }
}

// Test 17: Large signature (64-byte signature) roundtrip
proptest! {
    #[test]
    fn test_large_signature_64_bytes_roundtrip(
        payload in proptest::collection::vec(any::<u8>(), 0..64),
        signature in proptest::collection::vec(any::<u8>(), 64..=64),
        variant_index in 0u8..4u8,
        signer_id in any::<u64>(),
    ) {
        let algorithm = match variant_index % 4 {
            0 => SignatureAlgorithm::Ed25519,
            1 => SignatureAlgorithm::Secp256k1,
            2 => SignatureAlgorithm::Rsa2048,
            _ => SignatureAlgorithm::Rsa4096,
        };
        let val = SignedMessage { payload, signature, algorithm, signer_id };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<SignedMessage>(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.signature.len(), 64);
        prop_assert_eq!(val, decoded);
    }
}

// Test 18: Key id u64::MAX roundtrip
proptest! {
    #[test]
    fn test_key_id_u64_max_roundtrip(
        public_key in proptest::collection::vec(any::<u8>(), 0..32),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val = KeyPair {
            public_key,
            key_id: u64::MAX,
            algorithm,
            created_at,
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<KeyPair>(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.key_id, u64::MAX);
        prop_assert_eq!(val, decoded);
    }
}

// Test 19: Distinct KeyPairs encode differently
proptest! {
    #[test]
    fn test_distinct_keypairs_encode_differently(
        key_id_a in any::<u64>(),
        key_id_b in any::<u64>(),
    ) {
        prop_assume!(key_id_a != key_id_b);
        let val_a = KeyPair {
            public_key: vec![0xAB; 16],
            key_id: key_id_a,
            algorithm: 1,
            created_at: 1000,
        };
        let val_b = KeyPair {
            public_key: vec![0xAB; 16],
            key_id: key_id_b,
            algorithm: 1,
            created_at: 1000,
        };
        let bytes_a = encode_to_vec(&val_a).expect("encode failed");
        let bytes_b = encode_to_vec(&val_b).expect("encode failed");
        prop_assert_ne!(bytes_a, bytes_b);
    }
}

// Test 20: algorithm field 0..=255 roundtrip
proptest! {
    #[test]
    fn test_algorithm_field_full_u8_range_roundtrip(
        public_key in proptest::collection::vec(any::<u8>(), 0..16),
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
    ) {
        let val = KeyPair { public_key, key_id, algorithm, created_at };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _) = decode_from_slice::<KeyPair>(&bytes).expect("decode failed");
        prop_assert_eq!(val.algorithm, decoded.algorithm);
    }
}

// Test 21: Double encode/decode identity for SignedMessage
proptest! {
    #[test]
    fn test_signed_message_double_encode_decode_identity(
        payload in proptest::collection::vec(any::<u8>(), 0..32),
        signature in proptest::collection::vec(any::<u8>(), 0..64),
        variant_index in 0u8..4u8,
        signer_id in any::<u64>(),
    ) {
        let algorithm = match variant_index % 4 {
            0 => SignatureAlgorithm::Ed25519,
            1 => SignatureAlgorithm::Secp256k1,
            2 => SignatureAlgorithm::Rsa2048,
            _ => SignatureAlgorithm::Rsa4096,
        };
        let original = SignedMessage { payload, signature, algorithm, signer_id };
        let bytes1 = encode_to_vec(&original).expect("encode failed");
        let (decoded1, _) = decode_from_slice::<SignedMessage>(&bytes1).expect("decode failed");
        let bytes2 = encode_to_vec(&decoded1).expect("encode failed");
        let (decoded2, _) = decode_from_slice::<SignedMessage>(&bytes2).expect("decode failed");
        prop_assert_eq!(original, decoded2);
        prop_assert_eq!(bytes1, bytes2);
    }
}

// Test 22: Non-empty encoded bytes for all types
proptest! {
    #[test]
    fn test_non_empty_encoded_bytes_for_all_types(
        public_key in proptest::collection::vec(any::<u8>(), 0..16),
        key_id in any::<u64>(),
        algorithm in any::<u8>(),
        created_at in any::<u64>(),
        payload in proptest::collection::vec(any::<u8>(), 0..16),
        signature in proptest::collection::vec(any::<u8>(), 0..32),
        variant_index in 0u8..4u8,
        signer_id in any::<u64>(),
    ) {
        let sig_algorithm = match variant_index % 4 {
            0 => SignatureAlgorithm::Ed25519,
            1 => SignatureAlgorithm::Secp256k1,
            2 => SignatureAlgorithm::Rsa2048,
            _ => SignatureAlgorithm::Rsa4096,
        };
        let keypair = KeyPair { public_key, key_id, algorithm, created_at };
        let signed_msg = SignedMessage {
            payload,
            signature,
            algorithm: sig_algorithm.clone(),
            signer_id,
        };
        let keypair_bytes = encode_to_vec(&keypair).expect("encode failed");
        let sig_algo_bytes = encode_to_vec(&sig_algorithm).expect("encode failed");
        let signed_msg_bytes = encode_to_vec(&signed_msg).expect("encode failed");
        prop_assert!(!keypair_bytes.is_empty());
        prop_assert!(!sig_algo_bytes.is_empty());
        prop_assert!(!signed_msg_bytes.is_empty());
    }
}
