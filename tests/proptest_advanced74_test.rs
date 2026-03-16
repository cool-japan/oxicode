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

// ---------------------------------------------------------------------------
// Domain types: DeFi protocols and decentralized finance
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TokenAmount {
    symbol: String,
    amount_raw: u128,
    decimals: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AmmPool {
    pool_id: u64,
    token_a: TokenAmount,
    token_b: TokenAmount,
    fee_bps: u16,
    total_lp_shares: u128,
    sqrt_price_x96: u128,
    tick_current: i32,
    protocol_fee_share_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YieldFarmPosition {
    farm_id: u64,
    user_address: Vec<u8>,
    staked_lp_amount: u128,
    reward_debt: u128,
    pending_reward: u128,
    lock_until_epoch: u64,
    boost_multiplier_bps: u16,
    auto_compound: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlashLoanParams {
    loan_token: String,
    borrow_amount: u128,
    fee_amount: u128,
    callback_data: Vec<u8>,
    initiator: Vec<u8>,
    max_slippage_bps: u16,
    deadline_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OraclePriceFeed {
    feed_id: u64,
    base_asset: String,
    quote_asset: String,
    price_mantissa: i128,
    price_exponent: i8,
    confidence_interval: u64,
    publish_time: u64,
    source_count: u8,
    status: OracleStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OracleStatus {
    Active,
    Stale { last_update_epoch: u64 },
    Halted { reason_code: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GovernanceProposal {
    proposal_id: u64,
    proposer: Vec<u8>,
    title: String,
    description_hash: Vec<u8>,
    for_votes: u128,
    against_votes: u128,
    abstain_votes: u128,
    quorum_threshold: u128,
    eta_epoch: u64,
    status: ProposalStatus,
    actions: Vec<GovAction>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProposalStatus {
    Pending,
    Active,
    Defeated,
    Succeeded,
    Queued,
    Executed,
    Canceled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GovAction {
    target_contract: Vec<u8>,
    calldata: Vec<u8>,
    value_wei: u128,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StakingRecord {
    validator_id: u64,
    delegator: Vec<u8>,
    staked_amount: u128,
    reward_accumulated: u128,
    slash_count: u32,
    activation_epoch: u64,
    exit_epoch: Option<u64>,
    withdrawal_credentials: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollateralPosition {
    position_id: u64,
    owner: Vec<u8>,
    collateral_token: String,
    collateral_amount: u128,
    debt_token: String,
    debt_amount: u128,
    collateral_ratio_bps: u32,
    liquidation_threshold_bps: u32,
    interest_rate_bps: u16,
    last_accrual_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LiquidationEvent {
    position_id: u64,
    liquidator: Vec<u8>,
    collateral_seized: u128,
    debt_repaid: u128,
    liquidation_penalty_bps: u16,
    remaining_collateral: u128,
    remaining_debt: u128,
    block_number: u64,
    tx_index: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImpermanentLossSnapshot {
    pool_id: u64,
    entry_price_ratio_num: u128,
    entry_price_ratio_den: u128,
    current_price_ratio_num: u128,
    current_price_ratio_den: u128,
    il_basis_points: i32,
    holding_value_raw: u128,
    lp_value_raw: u128,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GasFeeEstimate {
    chain_id: u64,
    base_fee_gwei: u64,
    priority_fee_gwei: u64,
    max_fee_gwei: u64,
    estimated_gas_units: u64,
    l1_data_fee: Option<u64>,
    blob_base_fee: Option<u64>,
    congestion_level: CongestionLevel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CongestionLevel {
    Low,
    Medium,
    High,
    Extreme { pending_tx_count: u64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BridgeTransfer {
    transfer_id: Vec<u8>,
    source_chain_id: u64,
    dest_chain_id: u64,
    sender: Vec<u8>,
    recipient: Vec<u8>,
    token_symbol: String,
    amount: u128,
    bridge_fee: u128,
    nonce: u64,
    status: BridgeStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BridgeStatus {
    Initiated,
    SourceConfirmed { confirmations: u32 },
    Relayed,
    DestConfirmed { block_number: u64 },
    Completed,
    Failed { error_code: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwapRoute {
    route_id: u32,
    input_token: String,
    output_token: String,
    hops: Vec<SwapHop>,
    total_fee_bps: u16,
    min_output_amount: u128,
    deadline_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwapHop {
    pool_address: Vec<u8>,
    token_in: String,
    token_out: String,
    fee_tier: u16,
    sqrt_price_limit: u128,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LendingPosition {
    market_id: u64,
    lender: Vec<u8>,
    supplied_token: String,
    supplied_amount: u128,
    atoken_balance: u128,
    supply_apy_bps: u16,
    is_collateral_enabled: bool,
    last_update_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BorrowPosition {
    market_id: u64,
    borrower: Vec<u8>,
    borrowed_token: String,
    principal_amount: u128,
    accrued_interest: u128,
    borrow_apy_bps: u16,
    rate_mode: RateMode,
    health_factor_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RateMode {
    Stable,
    Variable,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaultStrategy {
    vault_id: u64,
    vault_name: String,
    underlying_token: String,
    total_assets: u128,
    total_shares: u128,
    performance_fee_bps: u16,
    management_fee_bps: u16,
    deposit_limit: u128,
    allocations: Vec<StrategyAllocation>,
    is_paused: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StrategyAllocation {
    protocol_name: String,
    allocation_bps: u16,
    current_value: u128,
    expected_apy_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeeTierConfig {
    tier_id: u8,
    fee_bps: u16,
    tick_spacing: i32,
    min_liquidity: u128,
    max_positions_per_user: u32,
    referral_discount_bps: u16,
    protocol_share_bps: u16,
}

// ---------------------------------------------------------------------------
// prop_compose! strategies
// ---------------------------------------------------------------------------

prop_compose! {
    fn arb_token_amount()(
        symbol in "[A-Z]{3,6}",
        amount_raw in any::<u128>(),
        decimals in 0u8..=18,
    ) -> TokenAmount {
        TokenAmount { symbol, amount_raw, decimals }
    }
}

prop_compose! {
    fn arb_amm_pool()(
        pool_id in any::<u64>(),
        token_a in arb_token_amount(),
        token_b in arb_token_amount(),
        fee_bps in 0u16..=10000,
        total_lp_shares in any::<u128>(),
        sqrt_price_x96 in any::<u128>(),
        tick_current in -887272i32..=887272,
        protocol_fee_share_bps in 0u16..=5000,
    ) -> AmmPool {
        AmmPool {
            pool_id, token_a, token_b, fee_bps,
            total_lp_shares, sqrt_price_x96, tick_current,
            protocol_fee_share_bps,
        }
    }
}

prop_compose! {
    fn arb_address()(data in proptest::collection::vec(any::<u8>(), 20..=20)) -> Vec<u8> {
        data
    }
}

prop_compose! {
    fn arb_yield_farm()(
        farm_id in any::<u64>(),
        user_address in arb_address(),
        staked_lp_amount in any::<u128>(),
        reward_debt in any::<u128>(),
        pending_reward in any::<u128>(),
        lock_until_epoch in any::<u64>(),
        boost_multiplier_bps in 10000u16..=30000,
        auto_compound in any::<bool>(),
    ) -> YieldFarmPosition {
        YieldFarmPosition {
            farm_id, user_address, staked_lp_amount,
            reward_debt, pending_reward, lock_until_epoch,
            boost_multiplier_bps, auto_compound,
        }
    }
}

prop_compose! {
    fn arb_flash_loan()(
        loan_token in "[A-Z]{3,6}",
        borrow_amount in any::<u128>(),
        fee_amount in any::<u128>(),
        callback_data in proptest::collection::vec(any::<u8>(), 0..64),
        initiator in arb_address(),
        max_slippage_bps in 0u16..=1000,
        deadline_epoch in any::<u64>(),
    ) -> FlashLoanParams {
        FlashLoanParams {
            loan_token, borrow_amount, fee_amount,
            callback_data, initiator, max_slippage_bps,
            deadline_epoch,
        }
    }
}

fn arb_oracle_status() -> impl Strategy<Value = OracleStatus> {
    prop_oneof![
        Just(OracleStatus::Active),
        any::<u64>().prop_map(|e| OracleStatus::Stale {
            last_update_epoch: e
        }),
        (0u16..=999).prop_map(|c| OracleStatus::Halted { reason_code: c }),
    ]
}

prop_compose! {
    fn arb_oracle_feed()(
        feed_id in any::<u64>(),
        base_asset in "[A-Z]{3,5}",
        quote_asset in "[A-Z]{3,5}",
        price_mantissa in any::<i128>(),
        price_exponent in -18i8..=18,
        confidence_interval in any::<u64>(),
        publish_time in any::<u64>(),
        source_count in 1u8..=32,
        status in arb_oracle_status(),
    ) -> OraclePriceFeed {
        OraclePriceFeed {
            feed_id, base_asset, quote_asset,
            price_mantissa, price_exponent, confidence_interval,
            publish_time, source_count, status,
        }
    }
}

fn arb_proposal_status() -> impl Strategy<Value = ProposalStatus> {
    prop_oneof![
        Just(ProposalStatus::Pending),
        Just(ProposalStatus::Active),
        Just(ProposalStatus::Defeated),
        Just(ProposalStatus::Succeeded),
        Just(ProposalStatus::Queued),
        Just(ProposalStatus::Executed),
        Just(ProposalStatus::Canceled),
    ]
}

prop_compose! {
    fn arb_gov_action()(
        target_contract in arb_address(),
        calldata in proptest::collection::vec(any::<u8>(), 4..68),
        value_wei in any::<u128>(),
    ) -> GovAction {
        GovAction { target_contract, calldata, value_wei }
    }
}

prop_compose! {
    fn arb_governance_proposal()(
        proposal_id in any::<u64>(),
        proposer in arb_address(),
        title in "[a-zA-Z0-9 ]{10,40}",
        description_hash in proptest::collection::vec(any::<u8>(), 32..=32),
        for_votes in any::<u128>(),
        against_votes in any::<u128>(),
        abstain_votes in any::<u128>(),
        quorum_threshold in any::<u128>(),
        eta_epoch in any::<u64>(),
        status in arb_proposal_status(),
        actions in proptest::collection::vec(arb_gov_action(), 1..4),
    ) -> GovernanceProposal {
        GovernanceProposal {
            proposal_id, proposer, title, description_hash,
            for_votes, against_votes, abstain_votes,
            quorum_threshold, eta_epoch, status, actions,
        }
    }
}

prop_compose! {
    fn arb_staking_record()(
        validator_id in any::<u64>(),
        delegator in arb_address(),
        staked_amount in any::<u128>(),
        reward_accumulated in any::<u128>(),
        slash_count in 0u32..=100,
        activation_epoch in any::<u64>(),
        exit_epoch in proptest::option::of(any::<u64>()),
        withdrawal_credentials in proptest::collection::vec(any::<u8>(), 32..=32),
    ) -> StakingRecord {
        StakingRecord {
            validator_id, delegator, staked_amount,
            reward_accumulated, slash_count, activation_epoch,
            exit_epoch, withdrawal_credentials,
        }
    }
}

prop_compose! {
    fn arb_collateral_position()(
        position_id in any::<u64>(),
        owner in arb_address(),
        collateral_token in "[A-Z]{3,6}",
        collateral_amount in any::<u128>(),
        debt_token in "[A-Z]{3,6}",
        debt_amount in any::<u128>(),
        collateral_ratio_bps in 10000u32..=50000,
        liquidation_threshold_bps in 5000u32..=20000,
        interest_rate_bps in 0u16..=5000,
        last_accrual_epoch in any::<u64>(),
    ) -> CollateralPosition {
        CollateralPosition {
            position_id, owner, collateral_token, collateral_amount,
            debt_token, debt_amount, collateral_ratio_bps,
            liquidation_threshold_bps, interest_rate_bps,
            last_accrual_epoch,
        }
    }
}

prop_compose! {
    fn arb_liquidation_event()(
        position_id in any::<u64>(),
        liquidator in arb_address(),
        collateral_seized in any::<u128>(),
        debt_repaid in any::<u128>(),
        liquidation_penalty_bps in 0u16..=2000,
        remaining_collateral in any::<u128>(),
        remaining_debt in any::<u128>(),
        block_number in any::<u64>(),
        tx_index in any::<u32>(),
    ) -> LiquidationEvent {
        LiquidationEvent {
            position_id, liquidator, collateral_seized,
            debt_repaid, liquidation_penalty_bps,
            remaining_collateral, remaining_debt,
            block_number, tx_index,
        }
    }
}

prop_compose! {
    fn arb_il_snapshot()(
        pool_id in any::<u64>(),
        entry_price_ratio_num in 1u128..=u128::MAX,
        entry_price_ratio_den in 1u128..=u128::MAX,
        current_price_ratio_num in 1u128..=u128::MAX,
        current_price_ratio_den in 1u128..=u128::MAX,
        il_basis_points in -10000i32..=0,
        holding_value_raw in any::<u128>(),
        lp_value_raw in any::<u128>(),
    ) -> ImpermanentLossSnapshot {
        ImpermanentLossSnapshot {
            pool_id, entry_price_ratio_num, entry_price_ratio_den,
            current_price_ratio_num, current_price_ratio_den,
            il_basis_points, holding_value_raw, lp_value_raw,
        }
    }
}

fn arb_congestion_level() -> impl Strategy<Value = CongestionLevel> {
    prop_oneof![
        Just(CongestionLevel::Low),
        Just(CongestionLevel::Medium),
        Just(CongestionLevel::High),
        any::<u64>().prop_map(|c| CongestionLevel::Extreme {
            pending_tx_count: c
        }),
    ]
}

prop_compose! {
    fn arb_gas_fee_estimate()(
        chain_id in prop_oneof![Just(1u64), Just(10), Just(137), Just(42161), Just(8453)],
        base_fee_gwei in 1u64..=500,
        priority_fee_gwei in 0u64..=50,
        max_fee_gwei in 1u64..=1000,
        estimated_gas_units in 21000u64..=15000000,
        l1_data_fee in proptest::option::of(0u64..=1000000),
        blob_base_fee in proptest::option::of(0u64..=1000),
        congestion_level in arb_congestion_level(),
    ) -> GasFeeEstimate {
        GasFeeEstimate {
            chain_id, base_fee_gwei, priority_fee_gwei,
            max_fee_gwei, estimated_gas_units, l1_data_fee,
            blob_base_fee, congestion_level,
        }
    }
}

fn arb_bridge_status() -> impl Strategy<Value = BridgeStatus> {
    prop_oneof![
        Just(BridgeStatus::Initiated),
        (1u32..=64).prop_map(|c| BridgeStatus::SourceConfirmed { confirmations: c }),
        Just(BridgeStatus::Relayed),
        any::<u64>().prop_map(|b| BridgeStatus::DestConfirmed { block_number: b }),
        Just(BridgeStatus::Completed),
        (0u16..=999).prop_map(|e| BridgeStatus::Failed { error_code: e }),
    ]
}

prop_compose! {
    fn arb_bridge_transfer()(
        transfer_id in proptest::collection::vec(any::<u8>(), 32..=32),
        source_chain_id in prop_oneof![Just(1u64), Just(10), Just(137), Just(56)],
        dest_chain_id in prop_oneof![Just(1u64), Just(10), Just(137), Just(56)],
        sender in arb_address(),
        recipient in arb_address(),
        token_symbol in "[A-Z]{3,6}",
        amount in any::<u128>(),
        bridge_fee in any::<u128>(),
        nonce in any::<u64>(),
        status in arb_bridge_status(),
    ) -> BridgeTransfer {
        BridgeTransfer {
            transfer_id, source_chain_id, dest_chain_id,
            sender, recipient, token_symbol, amount,
            bridge_fee, nonce, status,
        }
    }
}

prop_compose! {
    fn arb_swap_hop()(
        pool_address in arb_address(),
        token_in in "[A-Z]{3,6}",
        token_out in "[A-Z]{3,6}",
        fee_tier in prop_oneof![Just(100u16), Just(500), Just(3000), Just(10000)],
        sqrt_price_limit in any::<u128>(),
    ) -> SwapHop {
        SwapHop { pool_address, token_in, token_out, fee_tier, sqrt_price_limit }
    }
}

prop_compose! {
    fn arb_swap_route()(
        route_id in any::<u32>(),
        input_token in "[A-Z]{3,6}",
        output_token in "[A-Z]{3,6}",
        hops in proptest::collection::vec(arb_swap_hop(), 1..=4),
        total_fee_bps in 0u16..=1000,
        min_output_amount in any::<u128>(),
        deadline_epoch in any::<u64>(),
    ) -> SwapRoute {
        SwapRoute {
            route_id, input_token, output_token, hops,
            total_fee_bps, min_output_amount, deadline_epoch,
        }
    }
}

prop_compose! {
    fn arb_lending_position()(
        market_id in any::<u64>(),
        lender in arb_address(),
        supplied_token in "[A-Z]{3,6}",
        supplied_amount in any::<u128>(),
        atoken_balance in any::<u128>(),
        supply_apy_bps in 0u16..=5000,
        is_collateral_enabled in any::<bool>(),
        last_update_epoch in any::<u64>(),
    ) -> LendingPosition {
        LendingPosition {
            market_id, lender, supplied_token, supplied_amount,
            atoken_balance, supply_apy_bps, is_collateral_enabled,
            last_update_epoch,
        }
    }
}

fn arb_rate_mode() -> impl Strategy<Value = RateMode> {
    prop_oneof![Just(RateMode::Stable), Just(RateMode::Variable)]
}

prop_compose! {
    fn arb_borrow_position()(
        market_id in any::<u64>(),
        borrower in arb_address(),
        borrowed_token in "[A-Z]{3,6}",
        principal_amount in any::<u128>(),
        accrued_interest in any::<u128>(),
        borrow_apy_bps in 0u16..=10000,
        rate_mode in arb_rate_mode(),
        health_factor_bps in 0u32..=100000,
    ) -> BorrowPosition {
        BorrowPosition {
            market_id, borrower, borrowed_token, principal_amount,
            accrued_interest, borrow_apy_bps, rate_mode,
            health_factor_bps,
        }
    }
}

prop_compose! {
    fn arb_strategy_allocation()(
        protocol_name in "[a-z]{4,12}",
        allocation_bps in 0u16..=10000,
        current_value in any::<u128>(),
        expected_apy_bps in 0u16..=5000,
    ) -> StrategyAllocation {
        StrategyAllocation {
            protocol_name, allocation_bps, current_value, expected_apy_bps,
        }
    }
}

prop_compose! {
    fn arb_vault_strategy()(
        vault_id in any::<u64>(),
        vault_name in "[a-zA-Z0-9 ]{5,30}",
        underlying_token in "[A-Z]{3,6}",
        total_assets in any::<u128>(),
        total_shares in any::<u128>(),
        performance_fee_bps in 0u16..=3000,
        management_fee_bps in 0u16..=500,
        deposit_limit in any::<u128>(),
        allocations in proptest::collection::vec(arb_strategy_allocation(), 1..=5),
        is_paused in any::<bool>(),
    ) -> VaultStrategy {
        VaultStrategy {
            vault_id, vault_name, underlying_token,
            total_assets, total_shares, performance_fee_bps,
            management_fee_bps, deposit_limit, allocations,
            is_paused,
        }
    }
}

prop_compose! {
    fn arb_fee_tier_config()(
        tier_id in any::<u8>(),
        fee_bps in prop_oneof![Just(1u16), Just(5), Just(30), Just(100)],
        tick_spacing in prop_oneof![Just(1i32), Just(10), Just(60), Just(200)],
        min_liquidity in any::<u128>(),
        max_positions_per_user in 1u32..=10000,
        referral_discount_bps in 0u16..=5000,
        protocol_share_bps in 0u16..=5000,
    ) -> FeeTierConfig {
        FeeTierConfig {
            tier_id, fee_bps, tick_spacing, min_liquidity,
            max_positions_per_user, referral_discount_bps,
            protocol_share_bps,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests: exactly 22 #[test] functions
// ---------------------------------------------------------------------------

#[test]
fn test_amm_pool_roundtrip() {
    proptest!(|(pool in arb_amm_pool())| {
        let encoded = encode_to_vec(&pool).expect("encode amm pool failed");
        let (decoded, _) = decode_from_slice::<AmmPool>(&encoded)
            .expect("decode amm pool failed");
        prop_assert_eq!(pool, decoded);
    });
}

#[test]
fn test_yield_farm_position_roundtrip() {
    proptest!(|(pos in arb_yield_farm())| {
        let encoded = encode_to_vec(&pos).expect("encode yield farm failed");
        let (decoded, _) = decode_from_slice::<YieldFarmPosition>(&encoded)
            .expect("decode yield farm failed");
        prop_assert_eq!(pos, decoded);
    });
}

#[test]
fn test_flash_loan_params_roundtrip() {
    proptest!(|(params in arb_flash_loan())| {
        let encoded = encode_to_vec(&params).expect("encode flash loan failed");
        let (decoded, _) = decode_from_slice::<FlashLoanParams>(&encoded)
            .expect("decode flash loan failed");
        prop_assert_eq!(params, decoded);
    });
}

#[test]
fn test_oracle_price_feed_roundtrip() {
    proptest!(|(feed in arb_oracle_feed())| {
        let encoded = encode_to_vec(&feed).expect("encode oracle feed failed");
        let (decoded, _) = decode_from_slice::<OraclePriceFeed>(&encoded)
            .expect("decode oracle feed failed");
        prop_assert_eq!(feed, decoded);
    });
}

#[test]
fn test_governance_proposal_roundtrip() {
    proptest!(|(proposal in arb_governance_proposal())| {
        let encoded = encode_to_vec(&proposal).expect("encode governance proposal failed");
        let (decoded, _) = decode_from_slice::<GovernanceProposal>(&encoded)
            .expect("decode governance proposal failed");
        prop_assert_eq!(proposal, decoded);
    });
}

#[test]
fn test_staking_record_roundtrip() {
    proptest!(|(record in arb_staking_record())| {
        let encoded = encode_to_vec(&record).expect("encode staking record failed");
        let (decoded, _) = decode_from_slice::<StakingRecord>(&encoded)
            .expect("decode staking record failed");
        prop_assert_eq!(record, decoded);
    });
}

#[test]
fn test_collateral_position_roundtrip() {
    proptest!(|(pos in arb_collateral_position())| {
        let encoded = encode_to_vec(&pos).expect("encode collateral position failed");
        let (decoded, _) = decode_from_slice::<CollateralPosition>(&encoded)
            .expect("decode collateral position failed");
        prop_assert_eq!(pos, decoded);
    });
}

#[test]
fn test_liquidation_event_roundtrip() {
    proptest!(|(event in arb_liquidation_event())| {
        let encoded = encode_to_vec(&event).expect("encode liquidation event failed");
        let (decoded, _) = decode_from_slice::<LiquidationEvent>(&encoded)
            .expect("decode liquidation event failed");
        prop_assert_eq!(event, decoded);
    });
}

#[test]
fn test_impermanent_loss_snapshot_roundtrip() {
    proptest!(|(snap in arb_il_snapshot())| {
        let encoded = encode_to_vec(&snap).expect("encode il snapshot failed");
        let (decoded, _) = decode_from_slice::<ImpermanentLossSnapshot>(&encoded)
            .expect("decode il snapshot failed");
        prop_assert_eq!(snap, decoded);
    });
}

#[test]
fn test_gas_fee_estimate_roundtrip() {
    proptest!(|(estimate in arb_gas_fee_estimate())| {
        let encoded = encode_to_vec(&estimate).expect("encode gas fee failed");
        let (decoded, _) = decode_from_slice::<GasFeeEstimate>(&encoded)
            .expect("decode gas fee failed");
        prop_assert_eq!(estimate, decoded);
    });
}

#[test]
fn test_bridge_transfer_roundtrip() {
    proptest!(|(transfer in arb_bridge_transfer())| {
        let encoded = encode_to_vec(&transfer).expect("encode bridge transfer failed");
        let (decoded, _) = decode_from_slice::<BridgeTransfer>(&encoded)
            .expect("decode bridge transfer failed");
        prop_assert_eq!(transfer, decoded);
    });
}

#[test]
fn test_swap_route_roundtrip() {
    proptest!(|(route in arb_swap_route())| {
        let encoded = encode_to_vec(&route).expect("encode swap route failed");
        let (decoded, _) = decode_from_slice::<SwapRoute>(&encoded)
            .expect("decode swap route failed");
        prop_assert_eq!(route, decoded);
    });
}

#[test]
fn test_lending_position_roundtrip() {
    proptest!(|(pos in arb_lending_position())| {
        let encoded = encode_to_vec(&pos).expect("encode lending position failed");
        let (decoded, _) = decode_from_slice::<LendingPosition>(&encoded)
            .expect("decode lending position failed");
        prop_assert_eq!(pos, decoded);
    });
}

#[test]
fn test_borrow_position_roundtrip() {
    proptest!(|(pos in arb_borrow_position())| {
        let encoded = encode_to_vec(&pos).expect("encode borrow position failed");
        let (decoded, _) = decode_from_slice::<BorrowPosition>(&encoded)
            .expect("decode borrow position failed");
        prop_assert_eq!(pos, decoded);
    });
}

#[test]
fn test_vault_strategy_roundtrip() {
    proptest!(|(vault in arb_vault_strategy())| {
        let encoded = encode_to_vec(&vault).expect("encode vault strategy failed");
        let (decoded, _) = decode_from_slice::<VaultStrategy>(&encoded)
            .expect("decode vault strategy failed");
        prop_assert_eq!(vault, decoded);
    });
}

#[test]
fn test_fee_tier_config_roundtrip() {
    proptest!(|(config in arb_fee_tier_config())| {
        let encoded = encode_to_vec(&config).expect("encode fee tier failed");
        let (decoded, _) = decode_from_slice::<FeeTierConfig>(&encoded)
            .expect("decode fee tier failed");
        prop_assert_eq!(config, decoded);
    });
}

#[test]
fn test_token_amount_roundtrip() {
    proptest!(|(tok in arb_token_amount())| {
        let encoded = encode_to_vec(&tok).expect("encode token amount failed");
        let (decoded, _) = decode_from_slice::<TokenAmount>(&encoded)
            .expect("decode token amount failed");
        prop_assert_eq!(tok, decoded);
    });
}

#[test]
fn test_oracle_status_enum_roundtrip() {
    proptest!(|(status in arb_oracle_status())| {
        let encoded = encode_to_vec(&status).expect("encode oracle status failed");
        let (decoded, _) = decode_from_slice::<OracleStatus>(&encoded)
            .expect("decode oracle status failed");
        prop_assert_eq!(status, decoded);
    });
}

#[test]
fn test_bridge_status_enum_roundtrip() {
    proptest!(|(status in arb_bridge_status())| {
        let encoded = encode_to_vec(&status).expect("encode bridge status failed");
        let (decoded, _) = decode_from_slice::<BridgeStatus>(&encoded)
            .expect("decode bridge status failed");
        prop_assert_eq!(status, decoded);
    });
}

#[test]
fn test_swap_hop_roundtrip() {
    proptest!(|(hop in arb_swap_hop())| {
        let encoded = encode_to_vec(&hop).expect("encode swap hop failed");
        let (decoded, _) = decode_from_slice::<SwapHop>(&encoded)
            .expect("decode swap hop failed");
        prop_assert_eq!(hop, decoded);
    });
}

#[test]
fn test_gov_action_roundtrip() {
    proptest!(|(action in arb_gov_action())| {
        let encoded = encode_to_vec(&action).expect("encode gov action failed");
        let (decoded, _) = decode_from_slice::<GovAction>(&encoded)
            .expect("decode gov action failed");
        prop_assert_eq!(action, decoded);
    });
}

#[test]
fn test_strategy_allocation_roundtrip() {
    proptest!(|(alloc in arb_strategy_allocation())| {
        let encoded = encode_to_vec(&alloc).expect("encode strategy allocation failed");
        let (decoded, _) = decode_from_slice::<StrategyAllocation>(&encoded)
            .expect("decode strategy allocation failed");
        prop_assert_eq!(alloc, decoded);
    });
}
