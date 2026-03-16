//! Advanced complex enum encoding tests for OxiCode - set 3

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
enum Network {
    Mainnet,
    Testnet(String),
    Custom { chain_id: u64, name: String },
    Unknown(u32, String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Instruction {
    Nop,
    Push(u64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Jump(u32),
    Call { target: u32, args: Vec<u64> },
    Return(Option<u64>),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Block {
    instructions: Vec<Instruction>,
    label: String,
}

#[test]
fn test_network_mainnet_roundtrip() {
    let val = Network::Mainnet;
    let bytes = encode_to_vec(&val).expect("encode Network::Mainnet");
    let (decoded, _): (Network, usize) =
        decode_from_slice(&bytes).expect("decode Network::Mainnet");
    assert_eq!(val, decoded);
}

#[test]
fn test_network_testnet_roundtrip() {
    let val = Network::Testnet("goerli".to_string());
    let bytes = encode_to_vec(&val).expect("encode Network::Testnet");
    let (decoded, _): (Network, usize) =
        decode_from_slice(&bytes).expect("decode Network::Testnet");
    assert_eq!(val, decoded);
}

#[test]
fn test_network_custom_roundtrip() {
    let val = Network::Custom {
        chain_id: 1337,
        name: "devnet".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Network::Custom");
    let (decoded, _): (Network, usize) = decode_from_slice(&bytes).expect("decode Network::Custom");
    assert_eq!(val, decoded);
}

#[test]
fn test_network_unknown_roundtrip() {
    let val = Network::Unknown(9999, "mystery-chain".to_string());
    let bytes = encode_to_vec(&val).expect("encode Network::Unknown");
    let (decoded, _): (Network, usize) =
        decode_from_slice(&bytes).expect("decode Network::Unknown");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_nop_roundtrip() {
    let val = Instruction::Nop;
    let bytes = encode_to_vec(&val).expect("encode Instruction::Nop");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Nop");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_push_roundtrip() {
    let val = Instruction::Push(0xDEAD_BEEF_CAFE_1234u64);
    let bytes = encode_to_vec(&val).expect("encode Instruction::Push");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Push");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_pop_roundtrip() {
    let val = Instruction::Pop;
    let bytes = encode_to_vec(&val).expect("encode Instruction::Pop");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Pop");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_add_roundtrip() {
    let val = Instruction::Add;
    let bytes = encode_to_vec(&val).expect("encode Instruction::Add");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Add");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_jump_roundtrip() {
    let val = Instruction::Jump(0x0000_4F00u32);
    let bytes = encode_to_vec(&val).expect("encode Instruction::Jump");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Jump");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_call_with_three_args_roundtrip() {
    let val = Instruction::Call {
        target: 0x1000,
        args: vec![42u64, 100u64, 255u64],
    };
    let bytes = encode_to_vec(&val).expect("encode Instruction::Call");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Call");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_return_some_roundtrip() {
    let val = Instruction::Return(Some(0xFFFF_FFFF_FFFF_FFFFu64));
    let bytes = encode_to_vec(&val).expect("encode Instruction::Return(Some)");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Return(Some)");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_return_none_roundtrip() {
    let val = Instruction::Return(None);
    let bytes = encode_to_vec(&val).expect("encode Instruction::Return(None)");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice(&bytes).expect("decode Instruction::Return(None)");
    assert_eq!(val, decoded);
}

#[test]
fn test_block_with_five_mixed_instructions_roundtrip() {
    let val = Block {
        instructions: vec![
            Instruction::Nop,
            Instruction::Push(7),
            Instruction::Push(13),
            Instruction::Add,
            Instruction::Return(Some(20)),
        ],
        label: "entry".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Block with 5 mixed instructions");
    let (decoded, _): (Block, usize) =
        decode_from_slice(&bytes).expect("decode Block with 5 mixed instructions");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_instruction_six_items_roundtrip() {
    let val: Vec<Instruction> = vec![
        Instruction::Nop,
        Instruction::Push(1),
        Instruction::Pop,
        Instruction::Sub,
        Instruction::Jump(0x200),
        Instruction::Return(None),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Instruction> 6 items");
    let (decoded, _): (Vec<Instruction>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Instruction> 6 items");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_network_four_variants_roundtrip() {
    let val: Vec<Network> = vec![
        Network::Mainnet,
        Network::Testnet("ropsten".to_string()),
        Network::Custom {
            chain_id: 42,
            name: "kovan".to_string(),
        },
        Network::Unknown(1234, "unnamed".to_string()),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Network> 4 variants");
    let (decoded, _): (Vec<Network>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Network> 4 variants");
    assert_eq!(val, decoded);
}

#[test]
fn test_mainnet_encodes_as_first_discriminant() {
    let mainnet = Network::Mainnet;
    let testnet = Network::Testnet("x".to_string());

    let mainnet_bytes = encode_to_vec(&mainnet).expect("encode Mainnet for discriminant check");
    let testnet_bytes = encode_to_vec(&testnet).expect("encode Testnet for discriminant check");

    // Mainnet is variant 0; Testnet is variant 1.
    // The first encoded byte (varint) for Mainnet must be less than for Testnet.
    assert!(
        mainnet_bytes[0] < testnet_bytes[0],
        "Mainnet discriminant ({}) must be less than Testnet discriminant ({})",
        mainnet_bytes[0],
        testnet_bytes[0]
    );
}

#[test]
fn test_different_variants_produce_different_bytes() {
    let nop_bytes = encode_to_vec(&Instruction::Nop).expect("encode Nop");
    let pop_bytes = encode_to_vec(&Instruction::Pop).expect("encode Pop");
    let add_bytes = encode_to_vec(&Instruction::Add).expect("encode Add");
    let sub_bytes = encode_to_vec(&Instruction::Sub).expect("encode Sub");

    assert_ne!(nop_bytes, pop_bytes, "Nop and Pop must differ");
    assert_ne!(nop_bytes, add_bytes, "Nop and Add must differ");
    assert_ne!(pop_bytes, sub_bytes, "Pop and Sub must differ");
    assert_ne!(add_bytes, sub_bytes, "Add and Sub must differ");
}

#[test]
fn test_all_instruction_variants_roundtrip() {
    let variants: Vec<Instruction> = vec![
        Instruction::Nop,
        Instruction::Push(99),
        Instruction::Pop,
        Instruction::Add,
        Instruction::Sub,
        Instruction::Mul,
        Instruction::Div,
        Instruction::Jump(0xABCD),
        Instruction::Call {
            target: 0x500,
            args: vec![1, 2],
        },
        Instruction::Return(Some(42)),
    ];
    for variant in &variants {
        let bytes = encode_to_vec(variant).expect("encode Instruction variant");
        let (decoded, _): (Instruction, usize) =
            decode_from_slice(&bytes).expect("decode Instruction variant");
        assert_eq!(variant, &decoded);
    }
}

#[test]
fn test_block_with_empty_instructions_roundtrip() {
    let val = Block {
        instructions: vec![],
        label: "empty-block".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Block with empty instructions");
    let (decoded, _): (Block, usize) =
        decode_from_slice(&bytes).expect("decode Block with empty instructions");
    assert_eq!(val, decoded);
}

#[test]
fn test_instruction_with_fixed_int_config_roundtrip() {
    let val = Instruction::Push(0x1234_5678_9ABC_DEF0u64);
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Instruction with fixint config");
    let (decoded, _): (Instruction, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Instruction with fixint config");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_block_three_blocks_roundtrip() {
    let val: Vec<Block> = vec![
        Block {
            instructions: vec![Instruction::Nop, Instruction::Push(1)],
            label: "block-a".to_string(),
        },
        Block {
            instructions: vec![Instruction::Add, Instruction::Return(Some(1))],
            label: "block-b".to_string(),
        },
        Block {
            instructions: vec![Instruction::Jump(0x100), Instruction::Pop, Instruction::Mul],
            label: "block-c".to_string(),
        },
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Block> 3 blocks");
    let (decoded, _): (Vec<Block>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Block> 3 blocks");
    assert_eq!(val, decoded);
}

#[test]
fn test_consumed_bytes_equals_encoded_length_for_block() {
    let val = Block {
        instructions: vec![
            Instruction::Push(10),
            Instruction::Push(20),
            Instruction::Add,
            Instruction::Return(Some(30)),
        ],
        label: "measure-block".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Block for consumed-bytes check");
    let (_, consumed): (Block, usize) =
        decode_from_slice(&bytes).expect("decode Block for consumed-bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes ({consumed}) must equal encoded length ({})",
        bytes.len()
    );
}
