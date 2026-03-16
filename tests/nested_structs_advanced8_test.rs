//! Advanced nested struct tests for OxiCode — cryptocurrency exchange operations theme.
//! Exactly 22 tests covering deeply nested domain types (3-4 levels).

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

// ---------------------------------------------------------------------------
// Domain types — Level 1 (leaf / shallow)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PriceLevel {
    price: u64,
    quantity: u64,
    order_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderBookSide {
    levels: Vec<PriceLevel>,
    total_volume: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderBookDepth {
    pair: String,
    bids: OrderBookSide,
    asks: OrderBookSide,
    sequence_number: u64,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrderKind {
    Limit,
    Market,
    StopLoss,
    StopLimit,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OpenOrder {
    order_id: u64,
    side: OrderSide,
    kind: OrderKind,
    price: u64,
    remaining_qty: u64,
    filled_qty: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MatchResult {
    maker_order_id: u64,
    taker_order_id: u64,
    price: u64,
    quantity: u64,
    maker_fee: u64,
    taker_fee: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MatchingEngineState {
    pair: String,
    open_orders: Vec<OpenOrder>,
    recent_matches: Vec<MatchResult>,
    last_trade_price: u64,
    engine_sequence: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WalletKind {
    Hot,
    Cold,
    Custodial,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssetBalance {
    asset: String,
    available: u64,
    locked: u64,
    pending_withdrawal: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Wallet {
    wallet_id: String,
    kind: WalletKind,
    balances: Vec<AssetBalance>,
    last_audit_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WalletSystem {
    wallets: Vec<Wallet>,
    total_assets_usd: u64,
    reconciliation_seq: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum KycTier {
    Unverified,
    Basic,
    Intermediate,
    Advanced,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AmlCheck {
    check_id: u64,
    provider: String,
    score: u32,
    passed: bool,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KycVerification {
    user_id: u64,
    tier: KycTier,
    document_hashes: Vec<String>,
    aml_checks: Vec<AmlCheck>,
    verified_at_ts: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KycAmlSnapshot {
    verifications: Vec<KycVerification>,
    pending_count: u32,
    flagged_count: u32,
    snapshot_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeeRate {
    base_bps: u32,
    discount_bps: u32,
    minimum_fee: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WithdrawalFee {
    asset: String,
    flat_fee: u64,
    percentage_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeeSchedule {
    tier_name: String,
    maker: FeeRate,
    taker: FeeRate,
    withdrawal_fees: Vec<WithdrawalFee>,
    volume_threshold_usd: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeeConfig {
    schedules: Vec<FeeSchedule>,
    default_tier: String,
    last_updated_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarginPosition {
    position_id: u64,
    pair: String,
    side: OrderSide,
    leverage: u32,
    entry_price: u64,
    current_price: u64,
    liquidation_price: u64,
    collateral: u64,
    unrealized_pnl: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarginAccount {
    user_id: u64,
    positions: Vec<MarginPosition>,
    total_collateral: u64,
    maintenance_margin: u64,
    is_liquidatable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarginSystem {
    accounts: Vec<MarginAccount>,
    insurance_fund: u64,
    global_open_interest: u64,
    last_funding_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Delegation {
    delegator: String,
    amount: u64,
    reward_share_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RewardEpoch {
    epoch: u64,
    total_reward: u64,
    distributed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StakingValidator {
    validator_id: String,
    name: String,
    commission_bps: u32,
    delegations: Vec<Delegation>,
    reward_epochs: Vec<RewardEpoch>,
    total_staked: u64,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StakingNetwork {
    validators: Vec<StakingValidator>,
    total_staked_network: u64,
    current_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirdropRecipient {
    address: String,
    amount: u64,
    claimed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirdropTier {
    tier_name: String,
    recipients: Vec<AirdropRecipient>,
    total_allocation: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirdropSnapshot {
    campaign_id: String,
    asset: String,
    tiers: Vec<AirdropTier>,
    snapshot_block: u64,
    expiry_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CircuitBreaker {
    max_price_change_bps: u32,
    cooldown_seconds: u32,
    triggered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarketPairConfig {
    pair: String,
    tick_size: u64,
    lot_size: u64,
    min_notional: u64,
    max_order_size: u64,
    circuit_breaker: CircuitBreaker,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExchangeMarketConfig {
    pairs: Vec<MarketPairConfig>,
    global_halt: bool,
    config_version: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RateLimitRule {
    endpoint_pattern: String,
    requests_per_second: u32,
    burst_limit: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ApiRateLimitTier {
    tier_name: String,
    rules: Vec<RateLimitRule>,
    weight_per_request: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ApiRateLimitConfig {
    tiers: Vec<ApiRateLimitTier>,
    global_max_rps: u32,
    ban_threshold: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AuditAction {
    Deposit,
    Withdrawal,
    Trade,
    SettingsChange,
    Login,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AuditEntry {
    entry_id: u64,
    user_id: u64,
    action: AuditAction,
    details: String,
    ip_address: String,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AuditTrail {
    entries: Vec<AuditEntry>,
    retention_days: u32,
    last_export_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsuranceFundAsset {
    asset: String,
    balance: u64,
    last_contribution_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsuranceFund {
    assets: Vec<InsuranceFundAsset>,
    total_value_usd: u64,
    target_ratio_bps: u32,
    current_ratio_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EscrowStatus {
    Pending,
    Funded,
    Released,
    Disputed,
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EscrowParty {
    user_id: u64,
    address: String,
    confirmed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct P2pEscrow {
    escrow_id: u64,
    asset: String,
    amount: u64,
    buyer: EscrowParty,
    seller: EscrowParty,
    status: EscrowStatus,
    created_ts: u64,
    expiry_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct P2pMarketplace {
    active_escrows: Vec<P2pEscrow>,
    completed_count: u64,
    disputed_count: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReferralReward {
    from_user_id: u64,
    asset: String,
    amount: u64,
    earned_ts: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Referrer {
    user_id: u64,
    referral_code: String,
    referred_users: Vec<u64>,
    rewards: Vec<ReferralReward>,
    total_earned_usd: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReferralProgram {
    referrers: Vec<Referrer>,
    commission_bps: u32,
    active: bool,
    program_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReportingAssetAggregate {
    asset: String,
    total_deposits: u64,
    total_withdrawals: u64,
    net_flow: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReportingPeriod {
    period_label: String,
    asset_aggregates: Vec<ReportingAssetAggregate>,
    total_trades: u64,
    total_volume_usd: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RegulatoryReport {
    report_id: String,
    jurisdiction: String,
    periods: Vec<ReportingPeriod>,
    generated_ts: u64,
    approved: bool,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_price_level(price: u64, qty: u64, count: u32) -> PriceLevel {
    PriceLevel {
        price,
        quantity: qty,
        order_count: count,
    }
}

fn make_order_book_depth() -> OrderBookDepth {
    OrderBookDepth {
        pair: "BTC/USDT".to_string(),
        bids: OrderBookSide {
            levels: vec![
                make_price_level(69_000_00, 150, 12),
                make_price_level(68_999_00, 320, 24),
                make_price_level(68_998_00, 80, 5),
            ],
            total_volume: 550,
        },
        asks: OrderBookSide {
            levels: vec![
                make_price_level(69_001_00, 200, 18),
                make_price_level(69_002_00, 440, 30),
            ],
            total_volume: 640,
        },
        sequence_number: 982_374_123,
        timestamp_ms: 1_700_000_000_000,
    }
}

fn make_matching_engine_state() -> MatchingEngineState {
    MatchingEngineState {
        pair: "ETH/USDT".to_string(),
        open_orders: vec![
            OpenOrder {
                order_id: 1001,
                side: OrderSide::Buy,
                kind: OrderKind::Limit,
                price: 3_500_00,
                remaining_qty: 10,
                filled_qty: 5,
            },
            OpenOrder {
                order_id: 1002,
                side: OrderSide::Sell,
                kind: OrderKind::StopLoss,
                price: 3_400_00,
                remaining_qty: 20,
                filled_qty: 0,
            },
        ],
        recent_matches: vec![MatchResult {
            maker_order_id: 999,
            taker_order_id: 1000,
            price: 3_500_00,
            quantity: 5,
            maker_fee: 175,
            taker_fee: 350,
        }],
        last_trade_price: 3_500_00,
        engine_sequence: 5_000_001,
    }
}

fn make_wallet_system() -> WalletSystem {
    WalletSystem {
        wallets: vec![
            Wallet {
                wallet_id: "hot-001".to_string(),
                kind: WalletKind::Hot,
                balances: vec![
                    AssetBalance {
                        asset: "BTC".to_string(),
                        available: 500_000_000,
                        locked: 10_000_000,
                        pending_withdrawal: 2_000_000,
                    },
                    AssetBalance {
                        asset: "ETH".to_string(),
                        available: 8_000_000_000,
                        locked: 500_000_000,
                        pending_withdrawal: 0,
                    },
                ],
                last_audit_ts: 1_700_000_000,
            },
            Wallet {
                wallet_id: "cold-001".to_string(),
                kind: WalletKind::Cold,
                balances: vec![AssetBalance {
                    asset: "BTC".to_string(),
                    available: 50_000_000_000,
                    locked: 0,
                    pending_withdrawal: 0,
                }],
                last_audit_ts: 1_699_999_000,
            },
        ],
        total_assets_usd: 1_200_000_000,
        reconciliation_seq: 44_321,
    }
}

fn make_kyc_aml_snapshot() -> KycAmlSnapshot {
    KycAmlSnapshot {
        verifications: vec![
            KycVerification {
                user_id: 42,
                tier: KycTier::Advanced,
                document_hashes: vec!["sha256:aabb1122".to_string(), "sha256:ccdd3344".to_string()],
                aml_checks: vec![AmlCheck {
                    check_id: 1,
                    provider: "Chainalysis".to_string(),
                    score: 95,
                    passed: true,
                    notes: "Clean history".to_string(),
                }],
                verified_at_ts: Some(1_700_000_500),
            },
            KycVerification {
                user_id: 99,
                tier: KycTier::Basic,
                document_hashes: vec!["sha256:eeff5566".to_string()],
                aml_checks: vec![],
                verified_at_ts: None,
            },
        ],
        pending_count: 5,
        flagged_count: 1,
        snapshot_ts: 1_700_001_000,
    }
}

fn make_fee_config() -> FeeConfig {
    FeeConfig {
        schedules: vec![
            FeeSchedule {
                tier_name: "VIP-0".to_string(),
                maker: FeeRate {
                    base_bps: 10,
                    discount_bps: 0,
                    minimum_fee: 100,
                },
                taker: FeeRate {
                    base_bps: 20,
                    discount_bps: 0,
                    minimum_fee: 200,
                },
                withdrawal_fees: vec![
                    WithdrawalFee {
                        asset: "BTC".to_string(),
                        flat_fee: 50_000,
                        percentage_bps: 0,
                    },
                    WithdrawalFee {
                        asset: "ETH".to_string(),
                        flat_fee: 500_000,
                        percentage_bps: 0,
                    },
                ],
                volume_threshold_usd: 0,
            },
            FeeSchedule {
                tier_name: "VIP-1".to_string(),
                maker: FeeRate {
                    base_bps: 8,
                    discount_bps: 2,
                    minimum_fee: 80,
                },
                taker: FeeRate {
                    base_bps: 16,
                    discount_bps: 4,
                    minimum_fee: 160,
                },
                withdrawal_fees: vec![WithdrawalFee {
                    asset: "BTC".to_string(),
                    flat_fee: 40_000,
                    percentage_bps: 0,
                }],
                volume_threshold_usd: 100_000_000,
            },
        ],
        default_tier: "VIP-0".to_string(),
        last_updated_ts: 1_700_000_000,
    }
}

fn make_margin_system() -> MarginSystem {
    MarginSystem {
        accounts: vec![MarginAccount {
            user_id: 42,
            positions: vec![
                MarginPosition {
                    position_id: 1,
                    pair: "BTC/USDT".to_string(),
                    side: OrderSide::Buy,
                    leverage: 10,
                    entry_price: 68_000_00,
                    current_price: 69_000_00,
                    liquidation_price: 62_000_00,
                    collateral: 1_000_000,
                    unrealized_pnl: 147_058,
                },
                MarginPosition {
                    position_id: 2,
                    pair: "ETH/USDT".to_string(),
                    side: OrderSide::Sell,
                    leverage: 5,
                    entry_price: 3_600_00,
                    current_price: 3_500_00,
                    liquidation_price: 4_200_00,
                    collateral: 500_000,
                    unrealized_pnl: 138_888,
                },
            ],
            total_collateral: 1_500_000,
            maintenance_margin: 750_000,
            is_liquidatable: false,
        }],
        insurance_fund: 50_000_000,
        global_open_interest: 2_000_000_000,
        last_funding_ts: 1_700_000_000,
    }
}

fn make_staking_network() -> StakingNetwork {
    StakingNetwork {
        validators: vec![
            StakingValidator {
                validator_id: "val-001".to_string(),
                name: "CoolJapanValidator".to_string(),
                commission_bps: 500,
                delegations: vec![
                    Delegation {
                        delegator: "addr1_user42".to_string(),
                        amount: 100_000_000,
                        reward_share_bps: 9500,
                    },
                    Delegation {
                        delegator: "addr1_user99".to_string(),
                        amount: 50_000_000,
                        reward_share_bps: 9500,
                    },
                ],
                reward_epochs: vec![
                    RewardEpoch {
                        epoch: 100,
                        total_reward: 5_000_000,
                        distributed: true,
                    },
                    RewardEpoch {
                        epoch: 101,
                        total_reward: 5_200_000,
                        distributed: false,
                    },
                ],
                total_staked: 150_000_000,
                active: true,
            },
            StakingValidator {
                validator_id: "val-002".to_string(),
                name: "TokyoStakePool".to_string(),
                commission_bps: 300,
                delegations: vec![],
                reward_epochs: vec![],
                total_staked: 0,
                active: false,
            },
        ],
        total_staked_network: 150_000_000,
        current_epoch: 101,
    }
}

fn make_airdrop_snapshot() -> AirdropSnapshot {
    AirdropSnapshot {
        campaign_id: "airdrop-2024-q1".to_string(),
        asset: "OXI".to_string(),
        tiers: vec![
            AirdropTier {
                tier_name: "early-adopter".to_string(),
                recipients: vec![
                    AirdropRecipient {
                        address: "0xaabb11".to_string(),
                        amount: 10_000,
                        claimed: true,
                    },
                    AirdropRecipient {
                        address: "0xccdd22".to_string(),
                        amount: 10_000,
                        claimed: false,
                    },
                ],
                total_allocation: 20_000,
            },
            AirdropTier {
                tier_name: "community".to_string(),
                recipients: vec![AirdropRecipient {
                    address: "0xeeff33".to_string(),
                    amount: 5_000,
                    claimed: false,
                }],
                total_allocation: 5_000,
            },
        ],
        snapshot_block: 18_500_000,
        expiry_ts: 1_710_000_000,
    }
}

fn make_exchange_market_config() -> ExchangeMarketConfig {
    ExchangeMarketConfig {
        pairs: vec![
            MarketPairConfig {
                pair: "BTC/USDT".to_string(),
                tick_size: 100,
                lot_size: 1_000,
                min_notional: 10_000_000,
                max_order_size: 100_000_000_000,
                circuit_breaker: CircuitBreaker {
                    max_price_change_bps: 1000,
                    cooldown_seconds: 300,
                    triggered: false,
                },
                active: true,
            },
            MarketPairConfig {
                pair: "ETH/USDT".to_string(),
                tick_size: 10,
                lot_size: 10_000,
                min_notional: 5_000_000,
                max_order_size: 50_000_000_000,
                circuit_breaker: CircuitBreaker {
                    max_price_change_bps: 1500,
                    cooldown_seconds: 600,
                    triggered: true,
                },
                active: true,
            },
        ],
        global_halt: false,
        config_version: 47,
    }
}

fn make_api_rate_limit_config() -> ApiRateLimitConfig {
    ApiRateLimitConfig {
        tiers: vec![
            ApiRateLimitTier {
                tier_name: "free".to_string(),
                rules: vec![
                    RateLimitRule {
                        endpoint_pattern: "/api/v1/ticker/*".to_string(),
                        requests_per_second: 10,
                        burst_limit: 20,
                    },
                    RateLimitRule {
                        endpoint_pattern: "/api/v1/order".to_string(),
                        requests_per_second: 5,
                        burst_limit: 10,
                    },
                ],
                weight_per_request: 1,
            },
            ApiRateLimitTier {
                tier_name: "pro".to_string(),
                rules: vec![RateLimitRule {
                    endpoint_pattern: "/api/v1/**".to_string(),
                    requests_per_second: 100,
                    burst_limit: 200,
                }],
                weight_per_request: 1,
            },
        ],
        global_max_rps: 10_000,
        ban_threshold: 500,
    }
}

fn make_audit_trail() -> AuditTrail {
    AuditTrail {
        entries: vec![
            AuditEntry {
                entry_id: 1,
                user_id: 42,
                action: AuditAction::Deposit,
                details: "Deposited 1.5 BTC from external wallet".to_string(),
                ip_address: "198.51.100.1".to_string(),
                timestamp_ms: 1_700_000_100_000,
            },
            AuditEntry {
                entry_id: 2,
                user_id: 42,
                action: AuditAction::Trade,
                details: "Market buy 0.5 ETH at 3500 USDT".to_string(),
                ip_address: "198.51.100.1".to_string(),
                timestamp_ms: 1_700_000_200_000,
            },
            AuditEntry {
                entry_id: 3,
                user_id: 99,
                action: AuditAction::Login,
                details: "Logged in via API key".to_string(),
                ip_address: "203.0.113.55".to_string(),
                timestamp_ms: 1_700_000_300_000,
            },
        ],
        retention_days: 365,
        last_export_ts: 1_699_900_000,
    }
}

fn make_insurance_fund() -> InsuranceFund {
    InsuranceFund {
        assets: vec![
            InsuranceFundAsset {
                asset: "USDT".to_string(),
                balance: 50_000_000_000,
                last_contribution_ts: 1_700_000_000,
            },
            InsuranceFundAsset {
                asset: "BTC".to_string(),
                balance: 500_000_000,
                last_contribution_ts: 1_699_999_000,
            },
        ],
        total_value_usd: 80_000_000_000,
        target_ratio_bps: 200,
        current_ratio_bps: 185,
    }
}

fn make_p2p_marketplace() -> P2pMarketplace {
    P2pMarketplace {
        active_escrows: vec![
            P2pEscrow {
                escrow_id: 5001,
                asset: "BTC".to_string(),
                amount: 10_000_000,
                buyer: EscrowParty {
                    user_id: 42,
                    address: "bc1q_buyer_42".to_string(),
                    confirmed: true,
                },
                seller: EscrowParty {
                    user_id: 99,
                    address: "bc1q_seller_99".to_string(),
                    confirmed: true,
                },
                status: EscrowStatus::Funded,
                created_ts: 1_700_000_000,
                expiry_ts: 1_700_086_400,
            },
            P2pEscrow {
                escrow_id: 5002,
                asset: "USDT".to_string(),
                amount: 50_000_000,
                buyer: EscrowParty {
                    user_id: 100,
                    address: "0x_buyer_100".to_string(),
                    confirmed: false,
                },
                seller: EscrowParty {
                    user_id: 101,
                    address: "0x_seller_101".to_string(),
                    confirmed: true,
                },
                status: EscrowStatus::Pending,
                created_ts: 1_700_001_000,
                expiry_ts: 1_700_087_400,
            },
        ],
        completed_count: 12_345,
        disputed_count: 23,
    }
}

fn make_referral_program() -> ReferralProgram {
    ReferralProgram {
        referrers: vec![
            Referrer {
                user_id: 42,
                referral_code: "REF42COOL".to_string(),
                referred_users: vec![100, 101, 102, 103],
                rewards: vec![
                    ReferralReward {
                        from_user_id: 100,
                        asset: "USDT".to_string(),
                        amount: 50_000,
                        earned_ts: 1_700_000_100,
                    },
                    ReferralReward {
                        from_user_id: 101,
                        asset: "USDT".to_string(),
                        amount: 30_000,
                        earned_ts: 1_700_000_200,
                    },
                ],
                total_earned_usd: 80_000,
            },
            Referrer {
                user_id: 55,
                referral_code: "REF55JAPAN".to_string(),
                referred_users: vec![200],
                rewards: vec![],
                total_earned_usd: 0,
            },
        ],
        commission_bps: 20,
        active: true,
        program_name: "CryptoReferral-2024".to_string(),
    }
}

fn make_regulatory_report() -> RegulatoryReport {
    RegulatoryReport {
        report_id: "REG-2024-Q4-001".to_string(),
        jurisdiction: "EU-MiCA".to_string(),
        periods: vec![
            ReportingPeriod {
                period_label: "2024-10".to_string(),
                asset_aggregates: vec![
                    ReportingAssetAggregate {
                        asset: "BTC".to_string(),
                        total_deposits: 500_000_000,
                        total_withdrawals: 300_000_000,
                        net_flow: 200_000_000,
                    },
                    ReportingAssetAggregate {
                        asset: "ETH".to_string(),
                        total_deposits: 2_000_000_000,
                        total_withdrawals: 2_500_000_000,
                        net_flow: -500_000_000,
                    },
                ],
                total_trades: 1_500_000,
                total_volume_usd: 50_000_000_000,
            },
            ReportingPeriod {
                period_label: "2024-11".to_string(),
                asset_aggregates: vec![ReportingAssetAggregate {
                    asset: "BTC".to_string(),
                    total_deposits: 600_000_000,
                    total_withdrawals: 400_000_000,
                    net_flow: 200_000_000,
                }],
                total_trades: 1_800_000,
                total_volume_usd: 62_000_000_000,
            },
        ],
        generated_ts: 1_701_000_000,
        approved: false,
    }
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

/// Test 1: Order book depth with bid/ask ladders
#[test]
fn test_order_book_depth_roundtrip() {
    let book = make_order_book_depth();
    let bytes = encode_to_vec(&book).expect("encode order book depth");
    let (decoded, _): (OrderBookDepth, usize) =
        decode_from_slice(&bytes).expect("decode order book depth");
    assert_eq!(book, decoded);
}

/// Test 2: Trade matching engine state
#[test]
fn test_matching_engine_state_roundtrip() {
    let state = make_matching_engine_state();
    let bytes = encode_to_vec(&state).expect("encode matching engine state");
    let (decoded, _): (MatchingEngineState, usize) =
        decode_from_slice(&bytes).expect("decode matching engine state");
    assert_eq!(state, decoded);
}

/// Test 3: Wallet balances (hot/cold/custodial)
#[test]
fn test_wallet_system_roundtrip() {
    let ws = make_wallet_system();
    let bytes = encode_to_vec(&ws).expect("encode wallet system");
    let (decoded, _): (WalletSystem, usize) =
        decode_from_slice(&bytes).expect("decode wallet system");
    assert_eq!(ws, decoded);
}

/// Test 4: KYC/AML verification tiers
#[test]
fn test_kyc_aml_snapshot_roundtrip() {
    let snap = make_kyc_aml_snapshot();
    let bytes = encode_to_vec(&snap).expect("encode KYC/AML snapshot");
    let (decoded, _): (KycAmlSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode KYC/AML snapshot");
    assert_eq!(snap, decoded);
}

/// Test 5: Fee schedules (maker/taker/withdrawal)
#[test]
fn test_fee_config_roundtrip() {
    let cfg = make_fee_config();
    let bytes = encode_to_vec(&cfg).expect("encode fee config");
    let (decoded, _): (FeeConfig, usize) = decode_from_slice(&bytes).expect("decode fee config");
    assert_eq!(cfg, decoded);
}

/// Test 6: Margin positions (leverage, liquidation)
#[test]
fn test_margin_system_roundtrip() {
    let ms = make_margin_system();
    let bytes = encode_to_vec(&ms).expect("encode margin system");
    let (decoded, _): (MarginSystem, usize) =
        decode_from_slice(&bytes).expect("decode margin system");
    assert_eq!(ms, decoded);
}

/// Test 7: Staking validators (delegations, rewards)
#[test]
fn test_staking_network_roundtrip() {
    let net = make_staking_network();
    let bytes = encode_to_vec(&net).expect("encode staking network");
    let (decoded, _): (StakingNetwork, usize) =
        decode_from_slice(&bytes).expect("decode staking network");
    assert_eq!(net, decoded);
}

/// Test 8: Airdrop distribution snapshots
#[test]
fn test_airdrop_snapshot_roundtrip() {
    let snap = make_airdrop_snapshot();
    let bytes = encode_to_vec(&snap).expect("encode airdrop snapshot");
    let (decoded, _): (AirdropSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode airdrop snapshot");
    assert_eq!(snap, decoded);
}

/// Test 9: Market pair configs (tick size, lot size, circuit breakers)
#[test]
fn test_exchange_market_config_roundtrip() {
    let cfg = make_exchange_market_config();
    let bytes = encode_to_vec(&cfg).expect("encode exchange market config");
    let (decoded, _): (ExchangeMarketConfig, usize) =
        decode_from_slice(&bytes).expect("decode exchange market config");
    assert_eq!(cfg, decoded);
}

/// Test 10: API rate limit configs
#[test]
fn test_api_rate_limit_config_roundtrip() {
    let cfg = make_api_rate_limit_config();
    let bytes = encode_to_vec(&cfg).expect("encode API rate limit config");
    let (decoded, _): (ApiRateLimitConfig, usize) =
        decode_from_slice(&bytes).expect("decode API rate limit config");
    assert_eq!(cfg, decoded);
}

/// Test 11: Audit trail records
#[test]
fn test_audit_trail_roundtrip() {
    let trail = make_audit_trail();
    let bytes = encode_to_vec(&trail).expect("encode audit trail");
    let (decoded, _): (AuditTrail, usize) = decode_from_slice(&bytes).expect("decode audit trail");
    assert_eq!(trail, decoded);
}

/// Test 12: Insurance fund balances
#[test]
fn test_insurance_fund_roundtrip() {
    let fund = make_insurance_fund();
    let bytes = encode_to_vec(&fund).expect("encode insurance fund");
    let (decoded, _): (InsuranceFund, usize) =
        decode_from_slice(&bytes).expect("decode insurance fund");
    assert_eq!(fund, decoded);
}

/// Test 13: P2P escrow transactions
#[test]
fn test_p2p_marketplace_roundtrip() {
    let mp = make_p2p_marketplace();
    let bytes = encode_to_vec(&mp).expect("encode P2P marketplace");
    let (decoded, _): (P2pMarketplace, usize) =
        decode_from_slice(&bytes).expect("decode P2P marketplace");
    assert_eq!(mp, decoded);
}

/// Test 14: Referral program structures
#[test]
fn test_referral_program_roundtrip() {
    let rp = make_referral_program();
    let bytes = encode_to_vec(&rp).expect("encode referral program");
    let (decoded, _): (ReferralProgram, usize) =
        decode_from_slice(&bytes).expect("decode referral program");
    assert_eq!(rp, decoded);
}

/// Test 15: Regulatory reporting aggregates
#[test]
fn test_regulatory_report_roundtrip() {
    let rr = make_regulatory_report();
    let bytes = encode_to_vec(&rr).expect("encode regulatory report");
    let (decoded, _): (RegulatoryReport, usize) =
        decode_from_slice(&bytes).expect("decode regulatory report");
    assert_eq!(rr, decoded);
}

/// Test 16: Empty order book (no bids/asks)
#[test]
fn test_empty_order_book_roundtrip() {
    let book = OrderBookDepth {
        pair: "DOGE/USDT".to_string(),
        bids: OrderBookSide {
            levels: vec![],
            total_volume: 0,
        },
        asks: OrderBookSide {
            levels: vec![],
            total_volume: 0,
        },
        sequence_number: 0,
        timestamp_ms: 1_700_000_000_000,
    };
    let bytes = encode_to_vec(&book).expect("encode empty order book");
    let (decoded, _): (OrderBookDepth, usize) =
        decode_from_slice(&bytes).expect("decode empty order book");
    assert_eq!(book, decoded);
}

/// Test 17: Margin account at liquidation threshold
#[test]
fn test_margin_account_liquidatable_roundtrip() {
    let system = MarginSystem {
        accounts: vec![MarginAccount {
            user_id: 777,
            positions: vec![MarginPosition {
                position_id: 99,
                pair: "SOL/USDT".to_string(),
                side: OrderSide::Buy,
                leverage: 20,
                entry_price: 150_00,
                current_price: 78_00,
                liquidation_price: 80_00,
                collateral: 100_000,
                unrealized_pnl: -480_000,
            }],
            total_collateral: 100_000,
            maintenance_margin: 95_000,
            is_liquidatable: true,
        }],
        insurance_fund: 50_000_000,
        global_open_interest: 500_000_000,
        last_funding_ts: 1_700_000_000,
    };
    let bytes = encode_to_vec(&system).expect("encode liquidatable margin");
    let (decoded, _): (MarginSystem, usize) =
        decode_from_slice(&bytes).expect("decode liquidatable margin");
    assert_eq!(system, decoded);
}

/// Test 18: Disputed P2P escrow
#[test]
fn test_disputed_escrow_roundtrip() {
    let mp = P2pMarketplace {
        active_escrows: vec![P2pEscrow {
            escrow_id: 9999,
            asset: "USDC".to_string(),
            amount: 100_000_000,
            buyer: EscrowParty {
                user_id: 200,
                address: "0x_disputed_buyer".to_string(),
                confirmed: true,
            },
            seller: EscrowParty {
                user_id: 201,
                address: "0x_disputed_seller".to_string(),
                confirmed: true,
            },
            status: EscrowStatus::Disputed,
            created_ts: 1_699_900_000,
            expiry_ts: 1_700_000_000,
        }],
        completed_count: 0,
        disputed_count: 1,
    };
    let bytes = encode_to_vec(&mp).expect("encode disputed escrow");
    let (decoded, _): (P2pMarketplace, usize) =
        decode_from_slice(&bytes).expect("decode disputed escrow");
    assert_eq!(mp, decoded);
}

/// Test 19: Multiple fee schedule tiers with many withdrawal fees
#[test]
fn test_fee_config_many_assets_roundtrip() {
    let assets = ["BTC", "ETH", "SOL", "AVAX", "MATIC", "DOT", "ADA"];
    let withdrawal_fees: Vec<WithdrawalFee> = assets
        .iter()
        .enumerate()
        .map(|(i, a)| WithdrawalFee {
            asset: a.to_string(),
            flat_fee: (i as u64 + 1) * 10_000,
            percentage_bps: (i as u32) * 5,
        })
        .collect();
    let cfg = FeeConfig {
        schedules: vec![FeeSchedule {
            tier_name: "whale".to_string(),
            maker: FeeRate {
                base_bps: 2,
                discount_bps: 1,
                minimum_fee: 10,
            },
            taker: FeeRate {
                base_bps: 4,
                discount_bps: 2,
                minimum_fee: 20,
            },
            withdrawal_fees,
            volume_threshold_usd: 10_000_000_000,
        }],
        default_tier: "whale".to_string(),
        last_updated_ts: 1_700_500_000,
    };
    let bytes = encode_to_vec(&cfg).expect("encode many-asset fee config");
    let (decoded, _): (FeeConfig, usize) =
        decode_from_slice(&bytes).expect("decode many-asset fee config");
    assert_eq!(cfg, decoded);
}

/// Test 20: Staking validator with large delegation set
#[test]
fn test_staking_large_delegations_roundtrip() {
    let delegations: Vec<Delegation> = (0..50)
        .map(|i| Delegation {
            delegator: format!("delegator_{:04}", i),
            amount: (i as u64 + 1) * 1_000_000,
            reward_share_bps: 9500,
        })
        .collect();
    let epochs: Vec<RewardEpoch> = (0..20)
        .map(|e| RewardEpoch {
            epoch: e,
            total_reward: (e + 1) * 100_000,
            distributed: e < 18,
        })
        .collect();
    let network = StakingNetwork {
        validators: vec![StakingValidator {
            validator_id: "val-big".to_string(),
            name: "MegaStakePool".to_string(),
            commission_bps: 100,
            delegations,
            reward_epochs: epochs,
            total_staked: 1_275_000_000,
            active: true,
        }],
        total_staked_network: 1_275_000_000,
        current_epoch: 20,
    };
    let bytes = encode_to_vec(&network).expect("encode large staking network");
    let (decoded, _): (StakingNetwork, usize) =
        decode_from_slice(&bytes).expect("decode large staking network");
    assert_eq!(network, decoded);
}

/// Test 21: Combined regulatory report with negative net flows
#[test]
fn test_regulatory_report_negative_flows_roundtrip() {
    let report = RegulatoryReport {
        report_id: "REG-STRESS-001".to_string(),
        jurisdiction: "US-FinCEN".to_string(),
        periods: vec![ReportingPeriod {
            period_label: "2024-12".to_string(),
            asset_aggregates: vec![
                ReportingAssetAggregate {
                    asset: "BTC".to_string(),
                    total_deposits: 100_000_000,
                    total_withdrawals: 900_000_000,
                    net_flow: -800_000_000,
                },
                ReportingAssetAggregate {
                    asset: "ETH".to_string(),
                    total_deposits: 50_000_000,
                    total_withdrawals: 500_000_000,
                    net_flow: -450_000_000,
                },
                ReportingAssetAggregate {
                    asset: "USDT".to_string(),
                    total_deposits: 10_000_000_000,
                    total_withdrawals: 2_000_000_000,
                    net_flow: 8_000_000_000,
                },
            ],
            total_trades: 5_000_000,
            total_volume_usd: 200_000_000_000,
        }],
        generated_ts: 1_704_000_000,
        approved: true,
    };
    let bytes = encode_to_vec(&report).expect("encode negative-flow report");
    let (decoded, _): (RegulatoryReport, usize) =
        decode_from_slice(&bytes).expect("decode negative-flow report");
    assert_eq!(report, decoded);
}

/// Test 22: Full exchange snapshot — deeply nested composite of all subsystems
#[test]
fn test_full_exchange_snapshot_roundtrip() {
    // Encode and decode each major subsystem, verifying the full composite
    // works when multiple deeply-nested structures are serialized sequentially.
    let components: Vec<Vec<u8>> = vec![
        encode_to_vec(&make_order_book_depth()).expect("encode order book"),
        encode_to_vec(&make_matching_engine_state()).expect("encode matching engine"),
        encode_to_vec(&make_wallet_system()).expect("encode wallet system"),
        encode_to_vec(&make_kyc_aml_snapshot()).expect("encode kyc snapshot"),
        encode_to_vec(&make_fee_config()).expect("encode fee config"),
        encode_to_vec(&make_margin_system()).expect("encode margin system"),
        encode_to_vec(&make_staking_network()).expect("encode staking network"),
        encode_to_vec(&make_airdrop_snapshot()).expect("encode airdrop snapshot"),
        encode_to_vec(&make_exchange_market_config()).expect("encode market config"),
        encode_to_vec(&make_api_rate_limit_config()).expect("encode rate limit config"),
        encode_to_vec(&make_audit_trail()).expect("encode audit trail"),
        encode_to_vec(&make_insurance_fund()).expect("encode insurance fund"),
        encode_to_vec(&make_p2p_marketplace()).expect("encode p2p marketplace"),
        encode_to_vec(&make_referral_program()).expect("encode referral program"),
        encode_to_vec(&make_regulatory_report()).expect("encode regulatory report"),
    ];

    // Decode each back and verify
    let (d_book, _): (OrderBookDepth, usize) =
        decode_from_slice(&components[0]).expect("decode order book");
    assert_eq!(make_order_book_depth(), d_book);

    let (d_engine, _): (MatchingEngineState, usize) =
        decode_from_slice(&components[1]).expect("decode matching engine");
    assert_eq!(make_matching_engine_state(), d_engine);

    let (d_wallet, _): (WalletSystem, usize) =
        decode_from_slice(&components[2]).expect("decode wallet system");
    assert_eq!(make_wallet_system(), d_wallet);

    let (d_kyc, _): (KycAmlSnapshot, usize) =
        decode_from_slice(&components[3]).expect("decode kyc snapshot");
    assert_eq!(make_kyc_aml_snapshot(), d_kyc);

    let (d_fee, _): (FeeConfig, usize) =
        decode_from_slice(&components[4]).expect("decode fee config");
    assert_eq!(make_fee_config(), d_fee);

    let (d_margin, _): (MarginSystem, usize) =
        decode_from_slice(&components[5]).expect("decode margin system");
    assert_eq!(make_margin_system(), d_margin);

    let (d_staking, _): (StakingNetwork, usize) =
        decode_from_slice(&components[6]).expect("decode staking network");
    assert_eq!(make_staking_network(), d_staking);

    let (d_airdrop, _): (AirdropSnapshot, usize) =
        decode_from_slice(&components[7]).expect("decode airdrop snapshot");
    assert_eq!(make_airdrop_snapshot(), d_airdrop);

    let (d_market, _): (ExchangeMarketConfig, usize) =
        decode_from_slice(&components[8]).expect("decode market config");
    assert_eq!(make_exchange_market_config(), d_market);

    let (d_rate, _): (ApiRateLimitConfig, usize) =
        decode_from_slice(&components[9]).expect("decode rate limit config");
    assert_eq!(make_api_rate_limit_config(), d_rate);

    let (d_audit, _): (AuditTrail, usize) =
        decode_from_slice(&components[10]).expect("decode audit trail");
    assert_eq!(make_audit_trail(), d_audit);

    let (d_insurance, _): (InsuranceFund, usize) =
        decode_from_slice(&components[11]).expect("decode insurance fund");
    assert_eq!(make_insurance_fund(), d_insurance);

    let (d_p2p, _): (P2pMarketplace, usize) =
        decode_from_slice(&components[12]).expect("decode p2p marketplace");
    assert_eq!(make_p2p_marketplace(), d_p2p);

    let (d_referral, _): (ReferralProgram, usize) =
        decode_from_slice(&components[13]).expect("decode referral program");
    assert_eq!(make_referral_program(), d_referral);

    let (d_report, _): (RegulatoryReport, usize) =
        decode_from_slice(&components[14]).expect("decode regulatory report");
    assert_eq!(make_regulatory_report(), d_report);
}
