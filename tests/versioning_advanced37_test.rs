#![cfg(feature = "versioning")]
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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PoolStatus {
    Active,
    Paused,
    Deprecated,
    Bootstrapping,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SwapDirection {
    AtoB,
    BtoA,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProposalState {
    Pending,
    Active,
    Succeeded,
    Defeated,
    Queued,
    Executed,
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VoteChoice {
    For,
    Against,
    Abstain,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OracleSource {
    Chainlink,
    Pyth,
    Band,
    InternalTwap,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LiquidityPool {
    pool_id: u64,
    token_a: String,
    token_b: String,
    reserve_a: u128,
    reserve_b: u128,
    fee_bps: u16,
    status: PoolStatus,
    lp_token_supply: u128,
    creation_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AmmSwapEvent {
    event_id: u64,
    pool_id: u64,
    trader: String,
    direction: SwapDirection,
    amount_in: u128,
    amount_out: u128,
    fee_paid: u64,
    price_impact_bps: u16,
    block_number: u64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YieldFarmingPosition {
    position_id: u64,
    farmer: String,
    pool_id: u64,
    lp_tokens_staked: u128,
    reward_token: String,
    accrued_rewards: u128,
    entry_epoch: u64,
    last_harvest_epoch: u64,
    boost_multiplier_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GovernanceProposal {
    proposal_id: u64,
    proposer: String,
    title: String,
    description_hash: String,
    state: ProposalState,
    votes_for: u128,
    votes_against: u128,
    votes_abstain: u128,
    quorum_required: u128,
    voting_end_block: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StakingReward {
    staker: String,
    token: String,
    amount_staked: u128,
    reward_per_epoch: u64,
    total_claimed: u128,
    lock_until_epoch: u64,
    apy_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlashLoanParams {
    loan_id: u64,
    borrower: String,
    asset: String,
    amount: u128,
    fee_bps: u16,
    initiated_at: u64,
    repaid_in_block: u64,
    successful: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OraclePriceFeed {
    feed_id: u64,
    asset_pair: String,
    price_x1e8: u64,
    confidence_x1e8: u64,
    source: OracleSource,
    published_at: u64,
    valid_until: u64,
    round_id: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TokenBridgeTransfer {
    transfer_id: u64,
    sender: String,
    recipient: String,
    token: String,
    amount: u128,
    source_chain_id: u32,
    dest_chain_id: u32,
    nonce: u64,
    finalized: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DaoVotingRecord {
    record_id: u64,
    proposal_id: u64,
    voter: String,
    choice: VoteChoice,
    weight: u128,
    cast_at_block: u64,
    delegated_from: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProtocolFeeConfig {
    config_id: u32,
    protocol: String,
    swap_fee_bps: u16,
    lp_share_bps: u16,
    treasury_share_bps: u16,
    referral_share_bps: u16,
    effective_from_block: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LiquidationThreshold {
    asset: String,
    collateral_factor_bps: u16,
    liquidation_threshold_bps: u16,
    liquidation_bonus_bps: u16,
    max_liquidation_close_factor_bps: u16,
    price_oracle_address: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NftMarketplaceListing {
    listing_id: u64,
    seller: String,
    collection: String,
    token_id: u128,
    price_wei: u128,
    payment_token: String,
    royalty_bps: u16,
    expiry_block: u64,
    active: bool,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_liquidity_pool_active_roundtrip() {
    let pool = LiquidityPool {
        pool_id: 1001,
        token_a: "WETH".to_string(),
        token_b: "USDC".to_string(),
        reserve_a: 5_000_000_000_000_000_000_000u128,
        reserve_b: 9_750_000_000_000u128,
        fee_bps: 30,
        status: PoolStatus::Active,
        lp_token_supply: 7_000_000_000_000_000_000_000u128,
        creation_epoch: 1_700_000_000,
    };
    let bytes = encode_to_vec(&pool).expect("encode LiquidityPool Active failed");
    let (decoded, consumed) =
        decode_from_slice::<LiquidityPool>(&bytes).expect("decode LiquidityPool Active failed");
    assert_eq!(pool, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_liquidity_pool_versioned_v1_0_0() {
    let pool = LiquidityPool {
        pool_id: 2002,
        token_a: "WBTC".to_string(),
        token_b: "DAI".to_string(),
        reserve_a: 100_000_000u128,
        reserve_b: 4_200_000_000_000_000_000_000u128,
        fee_bps: 5,
        status: PoolStatus::Bootstrapping,
        lp_token_supply: 204_939_015_319_191_819u128,
        creation_epoch: 1_710_000_000,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&pool, version)
        .expect("encode versioned LiquidityPool v1.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<LiquidityPool>(&bytes)
        .expect("decode versioned LiquidityPool v1.0.0 failed");
    assert_eq!(pool, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_amm_swap_event_atob_roundtrip() {
    let event = AmmSwapEvent {
        event_id: 3003,
        pool_id: 1001,
        trader: "0xdeadbeef00000000000000000000000000000001".to_string(),
        direction: SwapDirection::AtoB,
        amount_in: 1_000_000_000_000_000_000u128,
        amount_out: 1_947_000_000u128,
        fee_paid: 3_000_000_000_000_000u64,
        price_impact_bps: 4,
        block_number: 19_000_000,
        timestamp: 1_711_000_000,
    };
    let bytes = encode_to_vec(&event).expect("encode AmmSwapEvent AtoB failed");
    let (decoded, consumed) =
        decode_from_slice::<AmmSwapEvent>(&bytes).expect("decode AmmSwapEvent AtoB failed");
    assert_eq!(event, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_amm_swap_event_versioned_v2_0_0() {
    let event = AmmSwapEvent {
        event_id: 4004,
        pool_id: 2002,
        trader: "0xabcdef0000000000000000000000000000000099".to_string(),
        direction: SwapDirection::BtoA,
        amount_in: 50_000_000_000u128,
        amount_out: 26_315_789_473_684_210_526u128,
        fee_paid: 150_000_000u64,
        price_impact_bps: 12,
        block_number: 19_500_000,
        timestamp: 1_712_000_000,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&event, version)
        .expect("encode versioned AmmSwapEvent v2.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<AmmSwapEvent>(&bytes)
        .expect("decode versioned AmmSwapEvent v2.0.0 failed");
    assert_eq!(event, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_yield_farming_position_roundtrip() {
    let position = YieldFarmingPosition {
        position_id: 5005,
        farmer: "0xfarmer000000000000000000000000000000007".to_string(),
        pool_id: 1001,
        lp_tokens_staked: 500_000_000_000_000_000_000u128,
        reward_token: "SUSHI".to_string(),
        accrued_rewards: 12_340_000_000_000_000_000u128,
        entry_epoch: 1_700_500_000,
        last_harvest_epoch: 1_711_000_000,
        boost_multiplier_bps: 15000,
    };
    let bytes = encode_to_vec(&position).expect("encode YieldFarmingPosition failed");
    let (decoded, consumed) = decode_from_slice::<YieldFarmingPosition>(&bytes)
        .expect("decode YieldFarmingPosition failed");
    assert_eq!(position, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_yield_farming_position_versioned_v1_3_5() {
    let position = YieldFarmingPosition {
        position_id: 6006,
        farmer: "0xfarmer000000000000000000000000000000042".to_string(),
        pool_id: 2002,
        lp_tokens_staked: 1_200_000_000_000_000_000_000u128,
        reward_token: "UNI".to_string(),
        accrued_rewards: 0u128,
        entry_epoch: 1_712_000_000,
        last_harvest_epoch: 1_712_000_000,
        boost_multiplier_bps: 10000,
    };
    let version = Version::new(1, 3, 5);
    let bytes = encode_versioned_value(&position, version)
        .expect("encode versioned YieldFarmingPosition v1.3.5 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<YieldFarmingPosition>(&bytes)
        .expect("decode versioned YieldFarmingPosition v1.3.5 failed");
    assert_eq!(position, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_governance_proposal_active_versioned_v2_0_0() {
    let proposal = GovernanceProposal {
        proposal_id: 7007,
        proposer: "0xdao000000000000000000000000000000000001".to_string(),
        title: "Upgrade Fee Tier to 0.05%".to_string(),
        description_hash: "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string(),
        state: ProposalState::Active,
        votes_for: 4_200_000_000_000_000_000_000_000u128,
        votes_against: 800_000_000_000_000_000_000_000u128,
        votes_abstain: 100_000_000_000_000_000_000_000u128,
        quorum_required: 1_000_000_000_000_000_000_000_000u128,
        voting_end_block: 19_600_000,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&proposal, version)
        .expect("encode versioned GovernanceProposal Active v2.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<GovernanceProposal>(&bytes)
        .expect("decode versioned GovernanceProposal Active v2.0.0 failed");
    assert_eq!(proposal, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_governance_proposal_executed_roundtrip() {
    let proposal = GovernanceProposal {
        proposal_id: 8008,
        proposer: "0xdao000000000000000000000000000000000002".to_string(),
        title: "Add LINK/USDC Pool".to_string(),
        description_hash: "QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco".to_string(),
        state: ProposalState::Executed,
        votes_for: 9_000_000_000_000_000_000_000_000u128,
        votes_against: 500_000_000_000_000_000_000_000u128,
        votes_abstain: 50_000_000_000_000_000_000_000u128,
        quorum_required: 2_000_000_000_000_000_000_000_000u128,
        voting_end_block: 18_800_000,
    };
    let bytes = encode_to_vec(&proposal).expect("encode GovernanceProposal Executed failed");
    let (decoded, consumed) = decode_from_slice::<GovernanceProposal>(&bytes)
        .expect("decode GovernanceProposal Executed failed");
    assert_eq!(proposal, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_staking_reward_versioned_v1_0_0() {
    let reward = StakingReward {
        staker: "0xstaker00000000000000000000000000000000a".to_string(),
        token: "CRV".to_string(),
        amount_staked: 10_000_000_000_000_000_000_000u128,
        reward_per_epoch: 500_000_000_000_000_000u64,
        total_claimed: 3_250_000_000_000_000_000_000u128,
        lock_until_epoch: 1_750_000_000,
        apy_bps: 1850,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&reward, version)
        .expect("encode versioned StakingReward v1.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<StakingReward>(&bytes)
        .expect("decode versioned StakingReward v1.0.0 failed");
    assert_eq!(reward, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_staking_reward_zero_claimed_roundtrip() {
    let reward = StakingReward {
        staker: "0xstaker00000000000000000000000000000000b".to_string(),
        token: "AAVE".to_string(),
        amount_staked: 250_000_000_000_000_000_000u128,
        reward_per_epoch: 12_500_000_000_000_000u64,
        total_claimed: 0u128,
        lock_until_epoch: 1_720_000_000,
        apy_bps: 720,
    };
    let bytes = encode_to_vec(&reward).expect("encode StakingReward zero claimed failed");
    let (decoded, consumed) = decode_from_slice::<StakingReward>(&bytes)
        .expect("decode StakingReward zero claimed failed");
    assert_eq!(reward, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_flash_loan_params_successful_versioned_v2_0_0() {
    let loan = FlashLoanParams {
        loan_id: 9009,
        borrower: "0xarb000000000000000000000000000000000001".to_string(),
        asset: "USDC".to_string(),
        amount: 10_000_000_000_000u128,
        fee_bps: 9,
        initiated_at: 1_711_100_000,
        repaid_in_block: 19_100_001,
        successful: true,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&loan, version)
        .expect("encode versioned FlashLoanParams successful v2.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<FlashLoanParams>(&bytes)
        .expect("decode versioned FlashLoanParams successful v2.0.0 failed");
    assert_eq!(loan, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_flash_loan_params_failed_roundtrip() {
    let loan = FlashLoanParams {
        loan_id: 10010,
        borrower: "0xarb000000000000000000000000000000000002".to_string(),
        asset: "WETH".to_string(),
        amount: 500_000_000_000_000_000_000u128,
        fee_bps: 9,
        initiated_at: 1_711_200_000,
        repaid_in_block: 0,
        successful: false,
    };
    let bytes = encode_to_vec(&loan).expect("encode FlashLoanParams failed roundtrip failed");
    let (decoded, consumed) = decode_from_slice::<FlashLoanParams>(&bytes)
        .expect("decode FlashLoanParams failed roundtrip failed");
    assert_eq!(loan, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_oracle_price_feed_chainlink_versioned_v1_3_5() {
    let feed = OraclePriceFeed {
        feed_id: 11011,
        asset_pair: "ETH/USD".to_string(),
        price_x1e8: 350000_00000000u64,
        confidence_x1e8: 150_00000000u64,
        source: OracleSource::Chainlink,
        published_at: 1_711_300_000,
        valid_until: 1_711_300_060,
        round_id: 180_443_837,
    };
    let version = Version::new(1, 3, 5);
    let bytes = encode_versioned_value(&feed, version)
        .expect("encode versioned OraclePriceFeed Chainlink v1.3.5 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<OraclePriceFeed>(&bytes)
        .expect("decode versioned OraclePriceFeed Chainlink v1.3.5 failed");
    assert_eq!(feed, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_oracle_price_feed_pyth_roundtrip() {
    let feed = OraclePriceFeed {
        feed_id: 12012,
        asset_pair: "BTC/USD".to_string(),
        price_x1e8: 7000000_00000000u64,
        confidence_x1e8: 5000_00000000u64,
        source: OracleSource::Pyth,
        published_at: 1_711_400_000,
        valid_until: 1_711_400_030,
        round_id: 291_003_124,
    };
    let bytes = encode_to_vec(&feed).expect("encode OraclePriceFeed Pyth failed");
    let (decoded, consumed) =
        decode_from_slice::<OraclePriceFeed>(&bytes).expect("decode OraclePriceFeed Pyth failed");
    assert_eq!(feed, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_token_bridge_transfer_finalized_versioned_v2_0_0() {
    let transfer = TokenBridgeTransfer {
        transfer_id: 13013,
        sender: "0xsender000000000000000000000000000000001".to_string(),
        recipient: "0xrecipient00000000000000000000000000001".to_string(),
        token: "WETH".to_string(),
        amount: 2_000_000_000_000_000_000u128,
        source_chain_id: 1,
        dest_chain_id: 42161,
        nonce: 99_000_001,
        finalized: true,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&transfer, version)
        .expect("encode versioned TokenBridgeTransfer finalized v2.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<TokenBridgeTransfer>(&bytes)
        .expect("decode versioned TokenBridgeTransfer finalized v2.0.0 failed");
    assert_eq!(transfer, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_token_bridge_transfer_pending_roundtrip() {
    let transfer = TokenBridgeTransfer {
        transfer_id: 14014,
        sender: "0xsender000000000000000000000000000000002".to_string(),
        recipient: "0xrecipient00000000000000000000000000002".to_string(),
        token: "USDC".to_string(),
        amount: 50_000_000_000u128,
        source_chain_id: 137,
        dest_chain_id: 10,
        nonce: 55_000_777,
        finalized: false,
    };
    let bytes = encode_to_vec(&transfer).expect("encode TokenBridgeTransfer pending failed");
    let (decoded, consumed) = decode_from_slice::<TokenBridgeTransfer>(&bytes)
        .expect("decode TokenBridgeTransfer pending failed");
    assert_eq!(transfer, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_dao_voting_record_with_delegation_versioned_v1_0_0() {
    let record = DaoVotingRecord {
        record_id: 15015,
        proposal_id: 7007,
        voter: "0xvoter00000000000000000000000000000000a".to_string(),
        choice: VoteChoice::For,
        weight: 1_500_000_000_000_000_000_000u128,
        cast_at_block: 19_550_000,
        delegated_from: Some("0xdelegate000000000000000000000000000001".to_string()),
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&record, version)
        .expect("encode versioned DaoVotingRecord with delegation v1.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<DaoVotingRecord>(&bytes)
        .expect("decode versioned DaoVotingRecord with delegation v1.0.0 failed");
    assert_eq!(record, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_dao_voting_record_no_delegation_against_roundtrip() {
    let record = DaoVotingRecord {
        record_id: 16016,
        proposal_id: 8008,
        voter: "0xvoter00000000000000000000000000000000b".to_string(),
        choice: VoteChoice::Against,
        weight: 250_000_000_000_000_000_000u128,
        cast_at_block: 18_790_000,
        delegated_from: None,
    };
    let bytes =
        encode_to_vec(&record).expect("encode DaoVotingRecord no delegation Against failed");
    let (decoded, consumed) = decode_from_slice::<DaoVotingRecord>(&bytes)
        .expect("decode DaoVotingRecord no delegation Against failed");
    assert_eq!(record, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_protocol_fee_config_versioned_v1_3_5() {
    let fee_cfg = ProtocolFeeConfig {
        config_id: 17017,
        protocol: "UniswapV4".to_string(),
        swap_fee_bps: 30,
        lp_share_bps: 8333,
        treasury_share_bps: 1667,
        referral_share_bps: 0,
        effective_from_block: 20_000_000,
    };
    let version = Version::new(1, 3, 5);
    let bytes = encode_versioned_value(&fee_cfg, version)
        .expect("encode versioned ProtocolFeeConfig v1.3.5 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<ProtocolFeeConfig>(&bytes)
        .expect("decode versioned ProtocolFeeConfig v1.3.5 failed");
    assert_eq!(fee_cfg, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
}

#[test]
fn test_liquidation_threshold_versioned_v2_0_0() {
    let threshold = LiquidationThreshold {
        asset: "WBTC".to_string(),
        collateral_factor_bps: 7500,
        liquidation_threshold_bps: 8000,
        liquidation_bonus_bps: 500,
        max_liquidation_close_factor_bps: 5000,
        price_oracle_address: "0xoracle00000000000000000000000000000001".to_string(),
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&threshold, version)
        .expect("encode versioned LiquidationThreshold v2.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<LiquidationThreshold>(&bytes)
        .expect("decode versioned LiquidationThreshold v2.0.0 failed");
    assert_eq!(threshold, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_nft_marketplace_listing_active_versioned_v1_0_0() {
    let listing = NftMarketplaceListing {
        listing_id: 19019,
        seller: "0xseller00000000000000000000000000000001".to_string(),
        collection: "BoredApeYachtClub".to_string(),
        token_id: 1234u128,
        price_wei: 60_000_000_000_000_000_000u128,
        payment_token: "WETH".to_string(),
        royalty_bps: 250,
        expiry_block: 20_100_000,
        active: true,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&listing, version)
        .expect("encode versioned NftMarketplaceListing active v1.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<NftMarketplaceListing>(&bytes)
        .expect("decode versioned NftMarketplaceListing active v1.0.0 failed");
    assert_eq!(listing, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_vec_of_amm_swap_events_versioned_v2_0_0() {
    let events = vec![
        AmmSwapEvent {
            event_id: 20001,
            pool_id: 1001,
            trader: "0xtrader0000000000000000000000000000001".to_string(),
            direction: SwapDirection::AtoB,
            amount_in: 500_000_000_000_000_000u128,
            amount_out: 973_500_000u128,
            fee_paid: 1_500_000_000_000_000u64,
            price_impact_bps: 2,
            block_number: 19_200_000,
            timestamp: 1_712_100_000,
        },
        AmmSwapEvent {
            event_id: 20002,
            pool_id: 2002,
            trader: "0xtrader0000000000000000000000000000002".to_string(),
            direction: SwapDirection::BtoA,
            amount_in: 2_000_000_000u128,
            amount_out: 1_050_000_000_000_000_000u128,
            fee_paid: 6_000_000u64,
            price_impact_bps: 1,
            block_number: 19_200_005,
            timestamp: 1_712_100_060,
        },
        AmmSwapEvent {
            event_id: 20003,
            pool_id: 1001,
            trader: "0xtrader0000000000000000000000000000003".to_string(),
            direction: SwapDirection::AtoB,
            amount_in: 10_000_000_000_000_000_000u128,
            amount_out: 19_456_000_000u128,
            fee_paid: 30_000_000_000_000_000u64,
            price_impact_bps: 18,
            block_number: 19_200_010,
            timestamp: 1_712_100_120,
        },
    ];
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&events, version)
        .expect("encode versioned Vec<AmmSwapEvent> v2.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<Vec<AmmSwapEvent>>(&bytes)
        .expect("decode versioned Vec<AmmSwapEvent> v2.0.0 failed");
    assert_eq!(events, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_proposal_state_all_variants_roundtrip() {
    let states = vec![
        ProposalState::Pending,
        ProposalState::Active,
        ProposalState::Succeeded,
        ProposalState::Defeated,
        ProposalState::Queued,
        ProposalState::Executed,
        ProposalState::Cancelled,
    ];
    let vote_choices = vec![VoteChoice::For, VoteChoice::Against, VoteChoice::Abstain];
    let oracle_sources = vec![
        OracleSource::Chainlink,
        OracleSource::Pyth,
        OracleSource::Band,
        OracleSource::InternalTwap,
    ];

    let states_bytes = encode_to_vec(&states).expect("encode Vec<ProposalState> failed");
    let (decoded_states, consumed_states) = decode_from_slice::<Vec<ProposalState>>(&states_bytes)
        .expect("decode Vec<ProposalState> failed");
    assert_eq!(states, decoded_states);
    assert_eq!(consumed_states, states_bytes.len());

    let choices_bytes = encode_to_vec(&vote_choices).expect("encode Vec<VoteChoice> failed");
    let (decoded_choices, consumed_choices) = decode_from_slice::<Vec<VoteChoice>>(&choices_bytes)
        .expect("decode Vec<VoteChoice> failed");
    assert_eq!(vote_choices, decoded_choices);
    assert_eq!(consumed_choices, choices_bytes.len());

    let sources_bytes = encode_to_vec(&oracle_sources).expect("encode Vec<OracleSource> failed");
    let (decoded_sources, consumed_sources) =
        decode_from_slice::<Vec<OracleSource>>(&sources_bytes)
            .expect("decode Vec<OracleSource> failed");
    assert_eq!(oracle_sources, decoded_sources);
    assert_eq!(consumed_sources, sources_bytes.len());
}
