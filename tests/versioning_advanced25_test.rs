#![cfg(feature = "versioning")]

//! Blockchain / smart contract domain — versioning feature tests.
//!
//! 22 #[test] functions covering contract state, token balances, transaction
//! history, governance proposals, DAO voting, NFT metadata, DeFi positions,
//! and protocol upgrades using encode_versioned_value / decode_versioned_value.

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
use oxicode::versioning::Version;
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContractStatus {
    Active,
    Paused,
    Deprecated,
    Destroyed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VoteChoice {
    For,
    Against,
    Abstain,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TokenStandard {
    Erc20,
    Erc721,
    Erc1155,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContractStateV1 {
    contract_address: String,
    owner: String,
    balance_wei: u64,
    status: ContractStatus,
    block_deployed: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContractStateV2 {
    contract_address: String,
    owner: String,
    balance_wei: u64,
    status: ContractStatus,
    block_deployed: u64,
    nonce: u32,
    paused_at_block: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TokenBalance {
    holder: String,
    token_id: u64,
    amount: u64,
    standard: TokenStandard,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Transaction {
    tx_hash: String,
    from: String,
    to: String,
    value_wei: u64,
    gas_used: u64,
    block_number: u64,
    success: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GovernanceProposal {
    proposal_id: u64,
    proposer: String,
    title: String,
    description: String,
    votes_for: u64,
    votes_against: u64,
    votes_abstain: u64,
    start_block: u64,
    end_block: u64,
    executed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DaoVote {
    voter: String,
    proposal_id: u64,
    choice: VoteChoice,
    voting_power: u64,
    cast_at_block: u64,
    delegate: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NftMetadata {
    token_id: u64,
    collection: String,
    owner: String,
    name: String,
    description: String,
    attributes: Vec<String>,
    royalty_bps: u16,
    transferable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefiPosition {
    position_id: u64,
    protocol: String,
    owner: String,
    collateral_wei: u64,
    debt_wei: u64,
    collateral_token: String,
    debt_token: String,
    liquidation_threshold_bps: u16,
    health_factor_micro: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProtocolUpgrade {
    upgrade_id: u32,
    name: String,
    old_impl: String,
    new_impl: String,
    scheduled_block: u64,
    applied_block: Option<u64>,
    approved_by: Vec<String>,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_contract_state_v1_basic_roundtrip() {
    let state = ContractStateV1 {
        contract_address: String::from("0xDEADBEEF00000001"),
        owner: String::from("0xALICE"),
        balance_wei: 1_000_000_000_000_000_000,
        status: ContractStatus::Active,
        block_deployed: 18_000_000,
    };

    let encoded = encode_to_vec(&state).expect("encode ContractStateV1 failed");
    let (decoded, _): (ContractStateV1, _) =
        decode_from_slice(&encoded).expect("decode ContractStateV1 failed");

    assert_eq!(state, decoded);
}

#[test]
fn test_contract_state_v1_versioned_encode_decode() {
    let state = ContractStateV1 {
        contract_address: String::from("0xCAFEBABE00000002"),
        owner: String::from("0xBOB"),
        balance_wei: 500_000_000,
        status: ContractStatus::Paused,
        block_deployed: 17_500_000,
    };
    let ver = Version::new(1, 0, 0);

    let bytes =
        encode_versioned_value(&state, ver).expect("versioned encode ContractStateV1 failed");
    let (decoded, decoded_ver, _): (ContractStateV1, Version, usize) =
        decode_versioned_value::<ContractStateV1>(&bytes)
            .expect("versioned decode ContractStateV1 failed");

    assert_eq!(state, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_version_field_access_major_minor_patch() {
    let ver = Version::new(2, 7, 3);

    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 3);
}

#[test]
fn test_contract_state_v2_with_option_nonce() {
    let state = ContractStateV2 {
        contract_address: String::from("0xFEEDFACE00000003"),
        owner: String::from("0xCHARLIE"),
        balance_wei: 9_999_999,
        status: ContractStatus::Active,
        block_deployed: 19_100_000,
        nonce: 42,
        paused_at_block: None,
    };
    let ver = Version::new(2, 0, 0);

    let bytes = encode_versioned_value(&state, ver).expect("versioned encode V2 failed");
    let (decoded, decoded_ver, _): (ContractStateV2, Version, usize) =
        decode_versioned_value::<ContractStateV2>(&bytes).expect("versioned decode V2 failed");

    assert_eq!(state, decoded);
    assert_eq!(decoded_ver.major, 2);
    assert!(decoded.paused_at_block.is_none());
}

#[test]
fn test_contract_state_v2_paused_option_some() {
    let state = ContractStateV2 {
        contract_address: String::from("0xABCDEF0000000004"),
        owner: String::from("0xDAVE"),
        balance_wei: 0,
        status: ContractStatus::Paused,
        block_deployed: 16_000_000,
        nonce: 100,
        paused_at_block: Some(19_200_000),
    };
    let ver = Version::new(2, 1, 0);

    let bytes = encode_versioned_value(&state, ver).expect("encode paused state failed");
    let (decoded, decoded_ver, consumed): (ContractStateV2, Version, usize) =
        decode_versioned_value::<ContractStateV2>(&bytes).expect("decode paused state failed");

    assert_eq!(decoded.paused_at_block, Some(19_200_000));
    assert_eq!(decoded_ver.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_token_balance_erc20_roundtrip() {
    let bal = TokenBalance {
        holder: String::from("0xHOLDER01"),
        token_id: 0,
        amount: 1_000_000_000_000_000_000,
        standard: TokenStandard::Erc20,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&bal, ver).expect("encode TokenBalance failed");
    let (decoded, _, consumed): (TokenBalance, Version, usize) =
        decode_versioned_value::<TokenBalance>(&bytes).expect("decode TokenBalance failed");

    assert_eq!(bal, decoded);
    assert!(consumed > 0);
    assert!(consumed <= bytes.len());
}

#[test]
fn test_token_balance_erc721_nft() {
    let bal = TokenBalance {
        holder: String::from("0xNFT_HOLDER"),
        token_id: 42_001,
        amount: 1,
        standard: TokenStandard::Erc721,
    };

    let encoded = encode_to_vec(&bal).expect("encode Erc721 balance failed");
    let (decoded, _): (TokenBalance, _) =
        decode_from_slice(&encoded).expect("decode Erc721 balance failed");

    assert_eq!(bal, decoded);
    assert_eq!(decoded.amount, 1);
}

#[test]
fn test_transaction_success_versioned() {
    let tx = Transaction {
        tx_hash: String::from("0xTXHASH_ABCDEF1234567890"),
        from: String::from("0xSENDER"),
        to: String::from("0xRECEIVER"),
        value_wei: 50_000_000_000_000_000,
        gas_used: 21_000,
        block_number: 18_500_000,
        success: true,
    };
    let ver = Version::new(1, 2, 0);

    let bytes = encode_versioned_value(&tx, ver).expect("encode Transaction failed");
    let (decoded, decoded_ver, _): (Transaction, Version, usize) =
        decode_versioned_value::<Transaction>(&bytes).expect("decode Transaction failed");

    assert!(decoded.success);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_governance_proposal_roundtrip() {
    let proposal = GovernanceProposal {
        proposal_id: 7,
        proposer: String::from("0xPROPOSER"),
        title: String::from("Upgrade treasury multisig"),
        description: String::from("Replace 3-of-5 multisig with 5-of-9 for higher security"),
        votes_for: 12_500_000,
        votes_against: 3_000_000,
        votes_abstain: 500_000,
        start_block: 19_100_000,
        end_block: 19_200_000,
        executed: false,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&proposal, ver).expect("encode GovernanceProposal failed");
    let (decoded, decoded_ver, _): (GovernanceProposal, Version, usize) =
        decode_versioned_value::<GovernanceProposal>(&bytes)
            .expect("decode GovernanceProposal failed");

    assert_eq!(proposal, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert!(!decoded.executed);
}

#[test]
fn test_governance_proposal_executed() {
    let proposal = GovernanceProposal {
        proposal_id: 12,
        proposer: String::from("0xCOREDEV"),
        title: String::from("Enable protocol fee"),
        description: String::from("Activate 0.05% protocol fee on swaps"),
        votes_for: 80_000_000,
        votes_against: 5_000_000,
        votes_abstain: 1_000_000,
        start_block: 18_900_000,
        end_block: 19_000_000,
        executed: true,
    };

    let encoded = encode_to_vec(&proposal).expect("encode executed proposal failed");
    let (decoded, _): (GovernanceProposal, _) =
        decode_from_slice(&encoded).expect("decode executed proposal failed");

    assert!(decoded.executed);
    assert_eq!(decoded.proposal_id, 12);
}

#[test]
fn test_dao_vote_for_with_delegate() {
    let vote = DaoVote {
        voter: String::from("0xVOTER_A"),
        proposal_id: 7,
        choice: VoteChoice::For,
        voting_power: 500_000,
        cast_at_block: 19_150_000,
        delegate: Some(String::from("0xDELEGATE_X")),
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&vote, ver).expect("encode DaoVote failed");
    let (decoded, decoded_ver, consumed): (DaoVote, Version, usize) =
        decode_versioned_value::<DaoVote>(&bytes).expect("decode DaoVote failed");

    assert_eq!(vote, decoded);
    assert_eq!(decoded.choice, VoteChoice::For);
    assert_eq!(decoded.delegate, Some(String::from("0xDELEGATE_X")));
    assert_eq!(decoded_ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_dao_vote_against_no_delegate() {
    let vote = DaoVote {
        voter: String::from("0xVOTER_B"),
        proposal_id: 7,
        choice: VoteChoice::Against,
        voting_power: 250_000,
        cast_at_block: 19_155_000,
        delegate: None,
    };

    let encoded = encode_to_vec(&vote).expect("encode DaoVote Against failed");
    let (decoded, _): (DaoVote, _) =
        decode_from_slice(&encoded).expect("decode DaoVote Against failed");

    assert_eq!(decoded.choice, VoteChoice::Against);
    assert!(decoded.delegate.is_none());
}

#[test]
fn test_nft_metadata_with_attributes_vec() {
    let nft = NftMetadata {
        token_id: 1337,
        collection: String::from("CoolCats"),
        owner: String::from("0xOWNER_KITTY"),
        name: String::from("Cool Cat #1337"),
        description: String::from("A very cool cat with rare attributes"),
        attributes: vec![
            String::from("background:blue"),
            String::from("hat:wizard"),
            String::from("eyes:laser"),
        ],
        royalty_bps: 500,
        transferable: true,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&nft, ver).expect("encode NftMetadata failed");
    let (decoded, decoded_ver, _): (NftMetadata, Version, usize) =
        decode_versioned_value::<NftMetadata>(&bytes).expect("decode NftMetadata failed");

    assert_eq!(nft, decoded);
    assert_eq!(decoded.attributes.len(), 3);
    assert_eq!(decoded_ver.major, 1);
}

#[test]
fn test_nft_metadata_empty_attributes() {
    let nft = NftMetadata {
        token_id: 9999,
        collection: String::from("MinimalCollection"),
        owner: String::from("0xMINIMAL_OWNER"),
        name: String::from("Minimal #9999"),
        description: String::from(""),
        attributes: vec![],
        royalty_bps: 0,
        transferable: false,
    };

    let encoded = encode_to_vec(&nft).expect("encode empty-attr NFT failed");
    let (decoded, _): (NftMetadata, _) =
        decode_from_slice(&encoded).expect("decode empty-attr NFT failed");

    assert!(decoded.attributes.is_empty());
    assert!(!decoded.transferable);
}

#[test]
fn test_defi_position_healthy_versioned() {
    let pos = DefiPosition {
        position_id: 501,
        protocol: String::from("AaveV3"),
        owner: String::from("0xDEFI_USER"),
        collateral_wei: 10_000_000_000_000_000_000,
        debt_wei: 5_000_000_000_000_000_000,
        collateral_token: String::from("WETH"),
        debt_token: String::from("USDC"),
        liquidation_threshold_bps: 8000,
        health_factor_micro: 1_600_000,
    };
    let ver = Version::new(3, 0, 0);

    let bytes = encode_versioned_value(&pos, ver).expect("encode DefiPosition failed");
    let (decoded, decoded_ver, consumed): (DefiPosition, Version, usize) =
        decode_versioned_value::<DefiPosition>(&bytes).expect("decode DefiPosition failed");

    assert_eq!(pos, decoded);
    assert_eq!(decoded_ver.major, 3);
    assert_eq!(decoded_ver.minor, 0);
    assert!(consumed > 0);
}

#[test]
fn test_defi_position_near_liquidation() {
    let pos = DefiPosition {
        position_id: 888,
        protocol: String::from("CompoundV3"),
        owner: String::from("0xRISKY_USER"),
        collateral_wei: 2_000_000_000_000_000_000,
        debt_wei: 1_900_000_000_000_000_000,
        collateral_token: String::from("WBTC"),
        debt_token: String::from("DAI"),
        liquidation_threshold_bps: 7500,
        health_factor_micro: 1_050_000,
    };

    let encoded = encode_to_vec(&pos).expect("encode near-liquidation position failed");
    let (decoded, _): (DefiPosition, _) =
        decode_from_slice(&encoded).expect("decode near-liquidation position failed");

    assert_eq!(decoded.health_factor_micro, 1_050_000);
    assert_eq!(decoded.protocol, "CompoundV3");
}

#[test]
fn test_protocol_upgrade_pending_with_approvals() {
    let upgrade = ProtocolUpgrade {
        upgrade_id: 5,
        name: String::from("SafeMath2"),
        old_impl: String::from("0xOLD_IMPL_ADDRESS"),
        new_impl: String::from("0xNEW_IMPL_ADDRESS"),
        scheduled_block: 19_500_000,
        applied_block: None,
        approved_by: vec![
            String::from("0xGUARD_1"),
            String::from("0xGUARD_2"),
            String::from("0xGUARD_3"),
        ],
    };
    let ver = Version::new(1, 1, 0);

    let bytes = encode_versioned_value(&upgrade, ver).expect("encode ProtocolUpgrade failed");
    let (decoded, decoded_ver, _): (ProtocolUpgrade, Version, usize) =
        decode_versioned_value::<ProtocolUpgrade>(&bytes).expect("decode ProtocolUpgrade failed");

    assert_eq!(upgrade, decoded);
    assert!(decoded.applied_block.is_none());
    assert_eq!(decoded.approved_by.len(), 3);
    assert_eq!(decoded_ver.minor, 1);
}

#[test]
fn test_protocol_upgrade_applied() {
    let upgrade = ProtocolUpgrade {
        upgrade_id: 6,
        name: String::from("FlashLoanGuard"),
        old_impl: String::from("0xPREV_IMPL"),
        new_impl: String::from("0xNEXT_IMPL"),
        scheduled_block: 19_300_000,
        applied_block: Some(19_301_000),
        approved_by: vec![String::from("0xMULTISIG")],
    };

    let encoded = encode_to_vec(&upgrade).expect("encode applied upgrade failed");
    let (decoded, _): (ProtocolUpgrade, _) =
        decode_from_slice(&encoded).expect("decode applied upgrade failed");

    assert_eq!(decoded.applied_block, Some(19_301_000));
    assert_eq!(decoded.upgrade_id, 6);
}

#[test]
fn test_multiple_versions_same_struct() {
    let state = ContractStateV1 {
        contract_address: String::from("0xMULTIVER"),
        owner: String::from("0xOWNER"),
        balance_wei: 1_234_567,
        status: ContractStatus::Active,
        block_deployed: 15_000_000,
    };

    let ver_a = Version::new(1, 0, 0);
    let ver_b = Version::new(2, 3, 1);

    let bytes_a = encode_versioned_value(&state, ver_a).expect("encode v1.0.0 failed");
    let bytes_b = encode_versioned_value(&state, ver_b).expect("encode v2.3.1 failed");

    let (_, dver_a, _): (ContractStateV1, Version, usize) =
        decode_versioned_value::<ContractStateV1>(&bytes_a).expect("decode v1.0.0 failed");
    let (_, dver_b, _): (ContractStateV1, Version, usize) =
        decode_versioned_value::<ContractStateV1>(&bytes_b).expect("decode v2.3.1 failed");

    assert_eq!(dver_a.major, 1);
    assert_eq!(dver_a.minor, 0);
    assert_eq!(dver_a.patch, 0);

    assert_eq!(dver_b.major, 2);
    assert_eq!(dver_b.minor, 3);
    assert_eq!(dver_b.patch, 1);
}

#[test]
fn test_vec_of_token_balances_roundtrip() {
    let balances = vec![
        TokenBalance {
            holder: String::from("0xHOLDER_A"),
            token_id: 1,
            amount: 100_000,
            standard: TokenStandard::Erc20,
        },
        TokenBalance {
            holder: String::from("0xHOLDER_B"),
            token_id: 2,
            amount: 1,
            standard: TokenStandard::Erc721,
        },
        TokenBalance {
            holder: String::from("0xHOLDER_C"),
            token_id: 3,
            amount: 50,
            standard: TokenStandard::Erc1155,
        },
        TokenBalance {
            holder: String::from("0xHOLDER_D"),
            token_id: 4,
            amount: 999,
            standard: TokenStandard::Custom(String::from("OxiToken")),
        },
    ];
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&balances, ver).expect("encode Vec<TokenBalance> failed");
    let (decoded, decoded_ver, consumed): (Vec<TokenBalance>, Version, usize) =
        decode_versioned_value::<Vec<TokenBalance>>(&bytes)
            .expect("decode Vec<TokenBalance> failed");

    assert_eq!(balances, decoded);
    assert_eq!(decoded.len(), 4);
    assert_eq!(decoded_ver.major, 1);
    assert!(consumed > 0);
    assert!(consumed <= bytes.len());
}

#[test]
fn test_bytes_consumed_equals_encoded_length() {
    let tx = Transaction {
        tx_hash: String::from("0xCONSUMED_CHECK"),
        from: String::from("0xF"),
        to: String::from("0xT"),
        value_wei: 1,
        gas_used: 21_000,
        block_number: 20_000_000,
        success: true,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&tx, ver).expect("encode for consumed check failed");
    let (_, _, consumed): (Transaction, Version, usize) =
        decode_versioned_value::<Transaction>(&bytes).expect("decode for consumed check failed");

    // consumed now includes the full versioned envelope (header + payload).
    assert!(consumed > 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_contract_status_deprecated_and_destroyed() {
    let deprecated = ContractStateV1 {
        contract_address: String::from("0xDEPRECATED"),
        owner: String::from("0xOLD_OWNER"),
        balance_wei: 0,
        status: ContractStatus::Deprecated,
        block_deployed: 10_000_000,
    };
    let destroyed = ContractStateV1 {
        contract_address: String::from("0xDESTROYED"),
        owner: String::from("0xOLD_OWNER2"),
        balance_wei: 0,
        status: ContractStatus::Destroyed,
        block_deployed: 10_100_000,
    };

    let encoded_dep = encode_to_vec(&deprecated).expect("encode deprecated contract failed");
    let encoded_des = encode_to_vec(&destroyed).expect("encode destroyed contract failed");

    let (dec_dep, _): (ContractStateV1, _) =
        decode_from_slice(&encoded_dep).expect("decode deprecated contract failed");
    let (dec_des, _): (ContractStateV1, _) =
        decode_from_slice(&encoded_des).expect("decode destroyed contract failed");

    assert_eq!(dec_dep.status, ContractStatus::Deprecated);
    assert_eq!(dec_des.status, ContractStatus::Destroyed);
}
