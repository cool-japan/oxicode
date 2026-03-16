//! Advanced Zstd compression tests for OxiCode — DeFi (Decentralized Finance) domain.
//!
//! Covers encode -> compress -> decompress -> decode round-trips for types that
//! model real-world DeFi protocol data: liquidity pools, AMM curves, yield farming
//! positions, flash loans, governance proposals, staking delegations, oracle feeds,
//! impermanent loss records, vault strategies, token swap paths, bridge proofs,
//! and lending protocol collateral ratios.

#![cfg(feature = "compression-zstd")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AmmCurveType {
    ConstantProduct,
    ConstantSum,
    StableSwap,
    ConcentratedLiquidity,
    WeightedPool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProposalStatus {
    Pending,
    Active,
    Succeeded,
    Defeated,
    Queued,
    Executed,
    Canceled,
    Expired,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VaultStrategy {
    SingleStake,
    LpCompound,
    LeveragedYield,
    DeltaNeutral,
    RebalancingIndex,
    OptionsVault,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BridgeNetwork {
    Ethereum,
    Polygon,
    Arbitrum,
    Optimism,
    Avalanche,
    Solana,
    BinanceSmartChain,
    Base,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OracleSource {
    Chainlink,
    Pyth,
    Band,
    Twap,
    VolumeWeighted,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LendingAction {
    Deposit,
    Withdraw,
    Borrow,
    Repay,
    Liquidate,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StakingStatus {
    Bonding,
    Active,
    Unbonding,
    Withdrawn,
    Slashed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SwapDirection {
    ExactIn,
    ExactOut,
}

// ---------------------------------------------------------------------------
// Struct definitions
// ---------------------------------------------------------------------------

/// A single liquidity pool with two tokens and fee parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LiquidityPool {
    pool_id: u64,
    token_a_address: Vec<u8>,
    token_b_address: Vec<u8>,
    reserve_a: u128,
    reserve_b: u128,
    total_lp_supply: u128,
    fee_bps: u16,
    curve: AmmCurveType,
    created_at_block: u64,
    is_active: bool,
}

/// Parameters for an AMM pricing curve.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AmmCurveParams {
    curve_type: AmmCurveType,
    amplification_factor: u64,
    weights: Vec<u32>,
    tick_spacing: u32,
    min_price_sqrt_x96: u128,
    max_price_sqrt_x96: u128,
    fee_tiers: Vec<u16>,
}

/// A yield farming position with accrued rewards.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YieldFarmPosition {
    position_id: u64,
    owner: Vec<u8>,
    pool_id: u64,
    staked_lp_amount: u128,
    reward_debt: u128,
    pending_rewards: u128,
    entry_block: u64,
    lock_until_block: u64,
    boost_multiplier_bps: u16,
}

/// A flash loan execution record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlashLoanRecord {
    tx_hash: Vec<u8>,
    borrower: Vec<u8>,
    token_address: Vec<u8>,
    borrow_amount: u128,
    fee_amount: u128,
    operations: Vec<FlashLoanOp>,
    block_number: u64,
    gas_used: u64,
    success: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlashLoanOp {
    op_type: String,
    target_contract: Vec<u8>,
    token_in: Vec<u8>,
    token_out: Vec<u8>,
    amount_in: u128,
    amount_out: u128,
}

/// A governance proposal with voting tallies.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GovernanceProposal {
    proposal_id: u64,
    proposer: Vec<u8>,
    title: String,
    description: String,
    status: ProposalStatus,
    for_votes: u128,
    against_votes: u128,
    abstain_votes: u128,
    quorum_required: u128,
    start_block: u64,
    end_block: u64,
    actions: Vec<ProposalAction>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProposalAction {
    target: Vec<u8>,
    calldata: Vec<u8>,
    value_wei: u128,
}

/// Staking delegation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StakingDelegation {
    delegator: Vec<u8>,
    validator: Vec<u8>,
    amount_staked: u128,
    shares: u128,
    status: StakingStatus,
    epoch_entered: u64,
    epoch_exit_requested: Option<u64>,
    rewards_claimed: u128,
    slash_events: Vec<SlashEvent>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlashEvent {
    epoch: u64,
    penalty_bps: u16,
    reason: String,
}

/// An oracle price feed snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OraclePriceFeed {
    feed_id: u64,
    source: OracleSource,
    base_token: String,
    quote_token: String,
    price_mantissa: u128,
    price_exponent: i8,
    confidence_bps: u16,
    timestamp: u64,
    round_id: u64,
    num_sources: u8,
}

/// Impermanent loss snapshot for a position.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImpermanentLossRecord {
    position_id: u64,
    pool_id: u64,
    entry_price_ratio_x96: u128,
    current_price_ratio_x96: u128,
    il_bps: u32,
    hodl_value_usd_x8: u64,
    lp_value_usd_x8: u64,
    fees_earned_usd_x8: u64,
    net_pnl_usd_x8: i64,
    snapshot_block: u64,
}

/// Vault strategy configuration with risk parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaultConfig {
    vault_id: u64,
    name: String,
    strategy: VaultStrategy,
    underlying_tokens: Vec<String>,
    total_value_locked: u128,
    share_price_x18: u128,
    performance_fee_bps: u16,
    management_fee_bps: u16,
    max_drawdown_bps: u16,
    rebalance_threshold_bps: u16,
    last_harvest_block: u64,
}

/// A multi-hop token swap path.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwapPath {
    path_id: u64,
    direction: SwapDirection,
    hops: Vec<SwapHop>,
    total_input: u128,
    total_output: u128,
    min_output: u128,
    deadline_block: u64,
    slippage_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwapHop {
    pool_id: u64,
    token_in: String,
    token_out: String,
    amount_in: u128,
    amount_out: u128,
    fee_bps: u16,
}

/// Cross-chain bridge transaction proof.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BridgeProof {
    proof_id: u64,
    source_chain: BridgeNetwork,
    dest_chain: BridgeNetwork,
    sender: Vec<u8>,
    receiver: Vec<u8>,
    token_symbol: String,
    amount: u128,
    nonce: u64,
    source_tx_hash: Vec<u8>,
    merkle_proof: Vec<Vec<u8>>,
    block_confirmations: u32,
    status: BridgeStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BridgeStatus {
    Initiated,
    Confirmed,
    Relayed,
    Finalized,
    Challenged,
    Reverted,
}

/// Lending protocol collateral record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollateralPosition {
    position_id: u64,
    borrower: Vec<u8>,
    collateral_token: String,
    debt_token: String,
    collateral_amount: u128,
    debt_amount: u128,
    ltv_bps: u16,
    liquidation_threshold_bps: u16,
    health_factor_x18: u128,
    interest_rate_bps: u16,
    last_update_block: u64,
    actions: Vec<LendingAction>,
}

/// A DEX aggregator order with split execution.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AggregatorOrder {
    order_id: u64,
    trader: Vec<u8>,
    token_in: String,
    token_out: String,
    total_amount_in: u128,
    min_total_out: u128,
    splits: Vec<OrderSplit>,
    executed_at_block: u64,
    gas_cost: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderSplit {
    dex_name: String,
    pool_id: u64,
    fraction_bps: u16,
    amount_in: u128,
    amount_out: u128,
}

/// Liquidation event from a lending protocol.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LiquidationEvent {
    event_id: u64,
    liquidator: Vec<u8>,
    borrower: Vec<u8>,
    collateral_token: String,
    debt_token: String,
    debt_repaid: u128,
    collateral_seized: u128,
    bonus_bps: u16,
    health_factor_before_x18: u128,
    health_factor_after_x18: u128,
    block_number: u64,
}

/// Token vesting schedule for a DAO treasury.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VestingSchedule {
    schedule_id: u64,
    beneficiary: Vec<u8>,
    token: String,
    total_amount: u128,
    released_amount: u128,
    start_timestamp: u64,
    cliff_duration_secs: u64,
    vesting_duration_secs: u64,
    revocable: bool,
    revoked: bool,
    milestones: Vec<VestingMilestone>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VestingMilestone {
    timestamp: u64,
    unlock_bps: u16,
    description: String,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_address(seed: u8) -> Vec<u8> {
    (0u8..20)
        .map(|i| seed.wrapping_add(i).wrapping_mul(7))
        .collect()
}

fn make_tx_hash(seed: u8) -> Vec<u8> {
    (0u8..32)
        .map(|i| seed.wrapping_add(i).wrapping_mul(13))
        .collect()
}

fn make_liquidity_pool(id: u64) -> LiquidityPool {
    LiquidityPool {
        pool_id: id,
        token_a_address: make_address((id & 0xFF) as u8),
        token_b_address: make_address(((id >> 8) & 0xFF) as u8),
        reserve_a: 1_000_000_000_000_u128 + id as u128 * 123_456,
        reserve_b: 500_000_000_000_u128 + id as u128 * 654_321,
        total_lp_supply: 750_000_000_000_u128 + id as u128 * 111_111,
        fee_bps: 30,
        curve: AmmCurveType::ConstantProduct,
        created_at_block: 15_000_000 + id,
        is_active: id % 3 != 0,
    }
}

fn make_yield_farm_position(id: u64) -> YieldFarmPosition {
    YieldFarmPosition {
        position_id: id,
        owner: make_address((id & 0xFF) as u8),
        pool_id: id % 10,
        staked_lp_amount: 50_000_000_000_u128 + id as u128 * 7777,
        reward_debt: 1_200_000_u128 + id as u128 * 333,
        pending_rewards: 800_000_u128 + id as u128 * 111,
        entry_block: 14_500_000 + id * 100,
        lock_until_block: 15_000_000 + id * 200,
        boost_multiplier_bps: 10000 + (id as u16 % 5000),
    }
}

fn make_oracle_feed(id: u64) -> OraclePriceFeed {
    let sources = [
        OracleSource::Chainlink,
        OracleSource::Pyth,
        OracleSource::Band,
        OracleSource::Twap,
        OracleSource::VolumeWeighted,
    ];
    OraclePriceFeed {
        feed_id: id,
        source: sources[(id as usize) % sources.len()].clone(),
        base_token: format!("TOKEN_{}", id),
        quote_token: "USD".to_string(),
        price_mantissa: 2_500_000_000_u128 + id as u128 * 100_000,
        price_exponent: -8,
        confidence_bps: 5 + (id as u16 % 20),
        timestamp: 1_700_000_000 + id * 12,
        round_id: 10_000 + id,
        num_sources: 3 + (id as u8 % 5),
    }
}

fn make_collateral_position(id: u64) -> CollateralPosition {
    CollateralPosition {
        position_id: id,
        borrower: make_address((id & 0xFF) as u8),
        collateral_token: format!("COL_{}", id % 5),
        debt_token: "USDC".to_string(),
        collateral_amount: 10_000_000_000_u128 + id as u128 * 50_000,
        debt_amount: 5_000_000_000_u128 + id as u128 * 25_000,
        ltv_bps: 7000 + (id as u16 % 1000),
        liquidation_threshold_bps: 8000 + (id as u16 % 500),
        health_factor_x18: 1_200_000_000_000_000_000_u128 + id as u128 * 10_000,
        interest_rate_bps: 300 + (id as u16 % 200),
        last_update_block: 16_000_000 + id,
        actions: vec![LendingAction::Deposit, LendingAction::Borrow],
    }
}

fn make_swap_hop(idx: u64) -> SwapHop {
    SwapHop {
        pool_id: 100 + idx,
        token_in: format!("TKN_{}", idx),
        token_out: format!("TKN_{}", idx + 1),
        amount_in: 1_000_000_u128 * (idx as u128 + 1),
        amount_out: 990_000_u128 * (idx as u128 + 1),
        fee_bps: 30,
    }
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

/// 1. Single liquidity pool round-trip.
#[test]
fn test_zstd_liquidity_pool_roundtrip() {
    let pool = make_liquidity_pool(1);
    let encoded = encode_to_vec(&pool).expect("encode LiquidityPool");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress LiquidityPool");
    let decompressed = decompress(&compressed).expect("decompress LiquidityPool");
    let (decoded, _): (LiquidityPool, usize) =
        decode_from_slice(&decompressed).expect("decode LiquidityPool");
    assert_eq!(pool, decoded);
}

/// 2. Vec of liquidity pools.
#[test]
fn test_zstd_liquidity_pool_batch_roundtrip() {
    let pools: Vec<LiquidityPool> = (1..=50).map(make_liquidity_pool).collect();
    let encoded = encode_to_vec(&pools).expect("encode Vec<LiquidityPool>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress pool batch");
    let decompressed = decompress(&compressed).expect("decompress pool batch");
    let (decoded, _): (Vec<LiquidityPool>, usize) =
        decode_from_slice(&decompressed).expect("decode pool batch");
    assert_eq!(pools, decoded);
}

/// 3. AMM curve parameters with varying weights and fee tiers.
#[test]
fn test_zstd_amm_curve_params_roundtrip() {
    let params = AmmCurveParams {
        curve_type: AmmCurveType::ConcentratedLiquidity,
        amplification_factor: 2000,
        weights: vec![5000, 3000, 2000],
        tick_spacing: 60,
        min_price_sqrt_x96: 4_295_128_739_u128,
        max_price_sqrt_x96: 340_282_366_920_938_463_463_374_607_431_768_211_455_u128,
        fee_tiers: vec![100, 500, 3000, 10000],
    };
    let encoded = encode_to_vec(&params).expect("encode AmmCurveParams");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress AmmCurveParams");
    let decompressed = decompress(&compressed).expect("decompress AmmCurveParams");
    let (decoded, _): (AmmCurveParams, usize) =
        decode_from_slice(&decompressed).expect("decode AmmCurveParams");
    assert_eq!(params, decoded);
}

/// 4. Yield farming position round-trip.
#[test]
fn test_zstd_yield_farm_position_roundtrip() {
    let pos = make_yield_farm_position(42);
    let encoded = encode_to_vec(&pos).expect("encode YieldFarmPosition");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress YieldFarmPosition");
    let decompressed = decompress(&compressed).expect("decompress YieldFarmPosition");
    let (decoded, _): (YieldFarmPosition, usize) =
        decode_from_slice(&decompressed).expect("decode YieldFarmPosition");
    assert_eq!(pos, decoded);
}

/// 5. Batch of yield farming positions.
#[test]
fn test_zstd_yield_farm_positions_batch_roundtrip() {
    let positions: Vec<YieldFarmPosition> = (1..=100).map(make_yield_farm_position).collect();
    let encoded = encode_to_vec(&positions).expect("encode Vec<YieldFarmPosition>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress farm batch");
    let decompressed = decompress(&compressed).expect("decompress farm batch");
    let (decoded, _): (Vec<YieldFarmPosition>, usize) =
        decode_from_slice(&decompressed).expect("decode farm batch");
    assert_eq!(positions, decoded);
}

/// 6. Flash loan record with multiple operations.
#[test]
fn test_zstd_flash_loan_record_roundtrip() {
    let record = FlashLoanRecord {
        tx_hash: make_tx_hash(0xAB),
        borrower: make_address(0xCD),
        token_address: make_address(0xEF),
        borrow_amount: 1_000_000_000_000_000_000_000_u128,
        fee_amount: 900_000_000_000_000_000_u128,
        operations: vec![
            FlashLoanOp {
                op_type: "swap".to_string(),
                target_contract: make_address(0x11),
                token_in: make_address(0x22),
                token_out: make_address(0x33),
                amount_in: 500_000_000_000_000_000_000_u128,
                amount_out: 1_200_000_000_000_u128,
            },
            FlashLoanOp {
                op_type: "deposit".to_string(),
                target_contract: make_address(0x44),
                token_in: make_address(0x33),
                token_out: make_address(0x55),
                amount_in: 1_200_000_000_000_u128,
                amount_out: 1_180_000_000_000_u128,
            },
            FlashLoanOp {
                op_type: "withdraw".to_string(),
                target_contract: make_address(0x66),
                token_in: make_address(0x55),
                token_out: make_address(0x22),
                amount_in: 1_180_000_000_000_u128,
                amount_out: 1_001_000_000_000_000_000_000_u128,
            },
        ],
        block_number: 17_500_000,
        gas_used: 450_000,
        success: true,
    };
    let encoded = encode_to_vec(&record).expect("encode FlashLoanRecord");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress FlashLoanRecord");
    let decompressed = decompress(&compressed).expect("decompress FlashLoanRecord");
    let (decoded, _): (FlashLoanRecord, usize) =
        decode_from_slice(&decompressed).expect("decode FlashLoanRecord");
    assert_eq!(record, decoded);
}

/// 7. Governance proposal with actions and vote tallies.
#[test]
fn test_zstd_governance_proposal_roundtrip() {
    let proposal = GovernanceProposal {
        proposal_id: 42,
        proposer: make_address(0x01),
        title: "Increase staking rewards multiplier".to_string(),
        description: "This proposal adjusts the staking APY from 5% to 8% to incentivize \
                       long-term holders and reduce selling pressure during the next epoch."
            .to_string(),
        status: ProposalStatus::Active,
        for_votes: 12_500_000_000_000_000_000_000_u128,
        against_votes: 3_200_000_000_000_000_000_000_u128,
        abstain_votes: 800_000_000_000_000_000_000_u128,
        quorum_required: 10_000_000_000_000_000_000_000_u128,
        start_block: 17_000_000,
        end_block: 17_100_000,
        actions: vec![
            ProposalAction {
                target: make_address(0xAA),
                calldata: vec![0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x08],
                value_wei: 0,
            },
            ProposalAction {
                target: make_address(0xBB),
                calldata: vec![0xAB, 0xCD, 0xEF, 0x01],
                value_wei: 0,
            },
        ],
    };
    let encoded = encode_to_vec(&proposal).expect("encode GovernanceProposal");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress GovernanceProposal");
    let decompressed = decompress(&compressed).expect("decompress GovernanceProposal");
    let (decoded, _): (GovernanceProposal, usize) =
        decode_from_slice(&decompressed).expect("decode GovernanceProposal");
    assert_eq!(proposal, decoded);
}

/// 8. Staking delegation with slash events.
#[test]
fn test_zstd_staking_delegation_roundtrip() {
    let delegation = StakingDelegation {
        delegator: make_address(0x10),
        validator: make_address(0x20),
        amount_staked: 32_000_000_000_000_000_000_u128,
        shares: 31_800_000_000_000_000_000_u128,
        status: StakingStatus::Active,
        epoch_entered: 200,
        epoch_exit_requested: None,
        rewards_claimed: 1_500_000_000_000_000_000_u128,
        slash_events: vec![
            SlashEvent {
                epoch: 180,
                penalty_bps: 50,
                reason: "Double signing detected".to_string(),
            },
            SlashEvent {
                epoch: 195,
                penalty_bps: 100,
                reason: "Extended downtime > 24h".to_string(),
            },
        ],
    };
    let encoded = encode_to_vec(&delegation).expect("encode StakingDelegation");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress StakingDelegation");
    let decompressed = decompress(&compressed).expect("decompress StakingDelegation");
    let (decoded, _): (StakingDelegation, usize) =
        decode_from_slice(&decompressed).expect("decode StakingDelegation");
    assert_eq!(delegation, decoded);
}

/// 9. Oracle price feed snapshot.
#[test]
fn test_zstd_oracle_price_feed_roundtrip() {
    let feed = make_oracle_feed(1);
    let encoded = encode_to_vec(&feed).expect("encode OraclePriceFeed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress OraclePriceFeed");
    let decompressed = decompress(&compressed).expect("decompress OraclePriceFeed");
    let (decoded, _): (OraclePriceFeed, usize) =
        decode_from_slice(&decompressed).expect("decode OraclePriceFeed");
    assert_eq!(feed, decoded);
}

/// 10. Batch of oracle feeds from multiple sources.
#[test]
fn test_zstd_oracle_feeds_batch_roundtrip() {
    let feeds: Vec<OraclePriceFeed> = (1..=200).map(make_oracle_feed).collect();
    let encoded = encode_to_vec(&feeds).expect("encode Vec<OraclePriceFeed>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress oracle batch");
    let decompressed = decompress(&compressed).expect("decompress oracle batch");
    let (decoded, _): (Vec<OraclePriceFeed>, usize) =
        decode_from_slice(&decompressed).expect("decode oracle batch");
    assert_eq!(feeds, decoded);
}

/// 11. Impermanent loss record with net P&L.
#[test]
fn test_zstd_impermanent_loss_roundtrip() {
    let il = ImpermanentLossRecord {
        position_id: 7,
        pool_id: 3,
        entry_price_ratio_x96: 79_228_162_514_264_337_593_543_950_336_u128,
        current_price_ratio_x96: 85_000_000_000_000_000_000_000_000_000_u128,
        il_bps: 56,
        hodl_value_usd_x8: 10_500_000_000,
        lp_value_usd_x8: 10_440_000_000,
        fees_earned_usd_x8: 180_000_000,
        net_pnl_usd_x8: 120_000_000,
        snapshot_block: 17_200_000,
    };
    let encoded = encode_to_vec(&il).expect("encode ImpermanentLossRecord");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress IL record");
    let decompressed = decompress(&compressed).expect("decompress IL record");
    let (decoded, _): (ImpermanentLossRecord, usize) =
        decode_from_slice(&decompressed).expect("decode IL record");
    assert_eq!(il, decoded);
}

/// 12. Vault strategy configuration.
#[test]
fn test_zstd_vault_config_roundtrip() {
    let vault = VaultConfig {
        vault_id: 5,
        name: "Delta-Neutral ETH Vault".to_string(),
        strategy: VaultStrategy::DeltaNeutral,
        underlying_tokens: vec!["WETH".to_string(), "USDC".to_string(), "stETH".to_string()],
        total_value_locked: 25_000_000_000_000_000_000_000_u128,
        share_price_x18: 1_050_000_000_000_000_000_u128,
        performance_fee_bps: 2000,
        management_fee_bps: 200,
        max_drawdown_bps: 500,
        rebalance_threshold_bps: 100,
        last_harvest_block: 17_150_000,
    };
    let encoded = encode_to_vec(&vault).expect("encode VaultConfig");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress VaultConfig");
    let decompressed = decompress(&compressed).expect("decompress VaultConfig");
    let (decoded, _): (VaultConfig, usize) =
        decode_from_slice(&decompressed).expect("decode VaultConfig");
    assert_eq!(vault, decoded);
}

/// 13. Multi-hop token swap path.
#[test]
fn test_zstd_swap_path_roundtrip() {
    let path = SwapPath {
        path_id: 99,
        direction: SwapDirection::ExactIn,
        hops: (0..4).map(make_swap_hop).collect(),
        total_input: 1_000_000_000_u128,
        total_output: 970_000_000_u128,
        min_output: 960_000_000_u128,
        deadline_block: 17_300_100,
        slippage_bps: 50,
    };
    let encoded = encode_to_vec(&path).expect("encode SwapPath");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress SwapPath");
    let decompressed = decompress(&compressed).expect("decompress SwapPath");
    let (decoded, _): (SwapPath, usize) =
        decode_from_slice(&decompressed).expect("decode SwapPath");
    assert_eq!(path, decoded);
}

/// 14. Cross-chain bridge proof with Merkle data.
#[test]
fn test_zstd_bridge_proof_roundtrip() {
    let proof = BridgeProof {
        proof_id: 1001,
        source_chain: BridgeNetwork::Ethereum,
        dest_chain: BridgeNetwork::Arbitrum,
        sender: make_address(0xA1),
        receiver: make_address(0xB2),
        token_symbol: "USDC".to_string(),
        amount: 50_000_000_000_u128,
        nonce: 77_777,
        source_tx_hash: make_tx_hash(0xDD),
        merkle_proof: (0..8).map(|i| make_tx_hash(i)).collect(),
        block_confirmations: 64,
        status: BridgeStatus::Finalized,
    };
    let encoded = encode_to_vec(&proof).expect("encode BridgeProof");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress BridgeProof");
    let decompressed = decompress(&compressed).expect("decompress BridgeProof");
    let (decoded, _): (BridgeProof, usize) =
        decode_from_slice(&decompressed).expect("decode BridgeProof");
    assert_eq!(proof, decoded);
}

/// 15. Lending protocol collateral position.
#[test]
fn test_zstd_collateral_position_roundtrip() {
    let pos = make_collateral_position(1);
    let encoded = encode_to_vec(&pos).expect("encode CollateralPosition");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress CollateralPosition");
    let decompressed = decompress(&compressed).expect("decompress CollateralPosition");
    let (decoded, _): (CollateralPosition, usize) =
        decode_from_slice(&decompressed).expect("decode CollateralPosition");
    assert_eq!(pos, decoded);
}

/// 16. DEX aggregator order with split execution.
#[test]
fn test_zstd_aggregator_order_roundtrip() {
    let order = AggregatorOrder {
        order_id: 5555,
        trader: make_address(0x77),
        token_in: "WETH".to_string(),
        token_out: "DAI".to_string(),
        total_amount_in: 10_000_000_000_000_000_000_u128,
        min_total_out: 25_000_000_000_000_000_000_000_u128,
        splits: vec![
            OrderSplit {
                dex_name: "UniswapV3".to_string(),
                pool_id: 101,
                fraction_bps: 6000,
                amount_in: 6_000_000_000_000_000_000_u128,
                amount_out: 15_200_000_000_000_000_000_000_u128,
            },
            OrderSplit {
                dex_name: "SushiSwap".to_string(),
                pool_id: 202,
                fraction_bps: 2500,
                amount_in: 2_500_000_000_000_000_000_u128,
                amount_out: 6_300_000_000_000_000_000_000_u128,
            },
            OrderSplit {
                dex_name: "Curve".to_string(),
                pool_id: 303,
                fraction_bps: 1500,
                amount_in: 1_500_000_000_000_000_000_u128,
                amount_out: 3_780_000_000_000_000_000_000_u128,
            },
        ],
        executed_at_block: 17_400_000,
        gas_cost: 380_000,
    };
    let encoded = encode_to_vec(&order).expect("encode AggregatorOrder");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress AggregatorOrder");
    let decompressed = decompress(&compressed).expect("decompress AggregatorOrder");
    let (decoded, _): (AggregatorOrder, usize) =
        decode_from_slice(&decompressed).expect("decode AggregatorOrder");
    assert_eq!(order, decoded);
}

/// 17. Liquidation event from a lending protocol.
#[test]
fn test_zstd_liquidation_event_roundtrip() {
    let event = LiquidationEvent {
        event_id: 8888,
        liquidator: make_address(0xF0),
        borrower: make_address(0xE0),
        collateral_token: "WETH".to_string(),
        debt_token: "USDC".to_string(),
        debt_repaid: 5_000_000_000_u128,
        collateral_seized: 2_750_000_000_000_000_000_u128,
        bonus_bps: 500,
        health_factor_before_x18: 980_000_000_000_000_000_u128,
        health_factor_after_x18: 1_500_000_000_000_000_000_u128,
        block_number: 17_250_000,
    };
    let encoded = encode_to_vec(&event).expect("encode LiquidationEvent");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress LiquidationEvent");
    let decompressed = decompress(&compressed).expect("decompress LiquidationEvent");
    let (decoded, _): (LiquidationEvent, usize) =
        decode_from_slice(&decompressed).expect("decode LiquidationEvent");
    assert_eq!(event, decoded);
}

/// 18. Token vesting schedule with milestones.
#[test]
fn test_zstd_vesting_schedule_roundtrip() {
    let schedule = VestingSchedule {
        schedule_id: 12,
        beneficiary: make_address(0x30),
        token: "GOV".to_string(),
        total_amount: 1_000_000_000_000_000_000_000_000_u128,
        released_amount: 250_000_000_000_000_000_000_000_u128,
        start_timestamp: 1_700_000_000,
        cliff_duration_secs: 15_778_800,
        vesting_duration_secs: 63_115_200,
        revocable: true,
        revoked: false,
        milestones: vec![
            VestingMilestone {
                timestamp: 1_715_778_800,
                unlock_bps: 2500,
                description: "Cliff reached — 25% unlock".to_string(),
            },
            VestingMilestone {
                timestamp: 1_731_557_600,
                unlock_bps: 5000,
                description: "12-month mark — 50% total".to_string(),
            },
            VestingMilestone {
                timestamp: 1_747_336_400,
                unlock_bps: 7500,
                description: "18-month mark — 75% total".to_string(),
            },
            VestingMilestone {
                timestamp: 1_763_115_200,
                unlock_bps: 10000,
                description: "Full vesting complete".to_string(),
            },
        ],
    };
    let encoded = encode_to_vec(&schedule).expect("encode VestingSchedule");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress VestingSchedule");
    let decompressed = decompress(&compressed).expect("decompress VestingSchedule");
    let (decoded, _): (VestingSchedule, usize) =
        decode_from_slice(&decompressed).expect("decode VestingSchedule");
    assert_eq!(schedule, decoded);
}

/// 19. Batch of collateral positions simulating a lending market snapshot.
#[test]
fn test_zstd_collateral_positions_batch_roundtrip() {
    let positions: Vec<CollateralPosition> = (1..=80).map(make_collateral_position).collect();
    let encoded = encode_to_vec(&positions).expect("encode Vec<CollateralPosition>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress collateral batch");
    let decompressed = decompress(&compressed).expect("decompress collateral batch");
    let (decoded, _): (Vec<CollateralPosition>, usize) =
        decode_from_slice(&decompressed).expect("decode collateral batch");
    assert_eq!(positions, decoded);
}

/// 20. Mixed DeFi snapshot: pools + farms + oracle feeds together.
#[test]
fn test_zstd_mixed_defi_snapshot_roundtrip() {
    let snapshot = (
        (1..=10).map(make_liquidity_pool).collect::<Vec<_>>(),
        (1..=10).map(make_yield_farm_position).collect::<Vec<_>>(),
        (1..=10).map(make_oracle_feed).collect::<Vec<_>>(),
    );
    let encoded = encode_to_vec(&snapshot).expect("encode DeFi snapshot tuple");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress DeFi snapshot");
    let decompressed = decompress(&compressed).expect("decompress DeFi snapshot");
    let (decoded, _): (
        (
            Vec<LiquidityPool>,
            Vec<YieldFarmPosition>,
            Vec<OraclePriceFeed>,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode DeFi snapshot");
    assert_eq!(snapshot, decoded);
}

/// 21. Staking delegation with optional exit epoch set.
#[test]
fn test_zstd_staking_delegation_unbonding_roundtrip() {
    let delegation = StakingDelegation {
        delegator: make_address(0x50),
        validator: make_address(0x60),
        amount_staked: 16_000_000_000_000_000_000_u128,
        shares: 15_900_000_000_000_000_000_u128,
        status: StakingStatus::Unbonding,
        epoch_entered: 100,
        epoch_exit_requested: Some(250),
        rewards_claimed: 2_400_000_000_000_000_000_u128,
        slash_events: vec![],
    };
    let encoded = encode_to_vec(&delegation).expect("encode unbonding StakingDelegation");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress unbonding delegation");
    let decompressed = decompress(&compressed).expect("decompress unbonding delegation");
    let (decoded, _): (StakingDelegation, usize) =
        decode_from_slice(&decompressed).expect("decode unbonding delegation");
    assert_eq!(delegation, decoded);
}

/// 22. Large batch of bridge proofs simulating cross-chain settlement.
#[test]
fn test_zstd_bridge_proofs_settlement_batch_roundtrip() {
    let chains = [
        BridgeNetwork::Ethereum,
        BridgeNetwork::Polygon,
        BridgeNetwork::Arbitrum,
        BridgeNetwork::Optimism,
        BridgeNetwork::Base,
    ];
    let statuses = [
        BridgeStatus::Initiated,
        BridgeStatus::Confirmed,
        BridgeStatus::Relayed,
        BridgeStatus::Finalized,
    ];
    let proofs: Vec<BridgeProof> = (0u64..60)
        .map(|i| BridgeProof {
            proof_id: 2000 + i,
            source_chain: chains[(i as usize) % chains.len()].clone(),
            dest_chain: chains[((i as usize) + 1) % chains.len()].clone(),
            sender: make_address((i & 0xFF) as u8),
            receiver: make_address(((i + 50) & 0xFF) as u8),
            token_symbol: format!("TKN_{}", i % 5),
            amount: 1_000_000_u128 * (i as u128 + 1),
            nonce: 10_000 + i,
            source_tx_hash: make_tx_hash((i & 0xFF) as u8),
            merkle_proof: (0..6).map(|j| make_tx_hash((i + j) as u8)).collect(),
            block_confirmations: 12 + (i as u32 % 100),
            status: statuses[(i as usize) % statuses.len()].clone(),
        })
        .collect();
    let encoded = encode_to_vec(&proofs).expect("encode Vec<BridgeProof>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress bridge batch");
    let decompressed = decompress(&compressed).expect("decompress bridge batch");
    let (decoded, _): (Vec<BridgeProof>, usize) =
        decode_from_slice(&decompressed).expect("decode bridge batch");
    assert_eq!(proofs, decoded);
}
