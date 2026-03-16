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

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OptionType {
    Call,
    Put,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExerciseStyle {
    European,
    American,
    Bermudan,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RiskFactor {
    Equity,
    Interest,
    Credit,
    Commodity,
    Fx,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OptionContract {
    contract_id: u64,
    underlying_id: u32,
    strike_price_cents: u64,
    expiry_date: u32,
    contract_type: OptionType,
    premium_cents: u64,
    delta_x1e6: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuturesPosition {
    position_id: u64,
    trader_id: u64,
    futures_id: u32,
    quantity: i32,
    entry_price_cents: u64,
    margin_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RiskMetrics {
    portfolio_id: u64,
    var_95_cents: u64,
    var_99_cents: u64,
    expected_shortfall_cents: u64,
    beta_x1e6: i32,
    sharpe_x1e6: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Volatility {
    underlying_id: u32,
    tenor_days: u16,
    implied_vol_x1e6: u32,
    historical_vol_x1e6: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Greeks {
    delta_x1e6: i32,
    gamma_x1e9: i32,
    theta_x1e6: i32,
    vega_x1e6: i32,
    rho_x1e6: i32,
}

// ── Strategy helpers ──────────────────────────────────────────────────────────

fn option_type_strategy() -> impl Strategy<Value = OptionType> {
    prop_oneof![Just(OptionType::Call), Just(OptionType::Put),]
}

fn exercise_style_strategy() -> impl Strategy<Value = ExerciseStyle> {
    prop_oneof![
        Just(ExerciseStyle::European),
        Just(ExerciseStyle::American),
        Just(ExerciseStyle::Bermudan),
    ]
}

fn risk_factor_strategy() -> impl Strategy<Value = RiskFactor> {
    prop_oneof![
        Just(RiskFactor::Equity),
        Just(RiskFactor::Interest),
        Just(RiskFactor::Credit),
        Just(RiskFactor::Commodity),
        Just(RiskFactor::Fx),
    ]
}

fn option_contract_strategy() -> impl Strategy<Value = OptionContract> {
    (
        any::<u64>(),
        any::<u32>(),
        any::<u64>(),
        any::<u32>(),
        option_type_strategy(),
        any::<u64>(),
        any::<i32>(),
    )
        .prop_map(
            |(
                contract_id,
                underlying_id,
                strike_price_cents,
                expiry_date,
                contract_type,
                premium_cents,
                delta_x1e6,
            )| {
                OptionContract {
                    contract_id,
                    underlying_id,
                    strike_price_cents,
                    expiry_date,
                    contract_type,
                    premium_cents,
                    delta_x1e6,
                }
            },
        )
}

fn futures_position_strategy() -> impl Strategy<Value = FuturesPosition> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<u32>(),
        any::<i32>(),
        any::<u64>(),
        any::<u64>(),
    )
        .prop_map(
            |(position_id, trader_id, futures_id, quantity, entry_price_cents, margin_cents)| {
                FuturesPosition {
                    position_id,
                    trader_id,
                    futures_id,
                    quantity,
                    entry_price_cents,
                    margin_cents,
                }
            },
        )
}

fn risk_metrics_strategy() -> impl Strategy<Value = RiskMetrics> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<i32>(),
        any::<i32>(),
    )
        .prop_map(
            |(
                portfolio_id,
                var_95_cents,
                var_99_cents,
                expected_shortfall_cents,
                beta_x1e6,
                sharpe_x1e6,
            )| {
                RiskMetrics {
                    portfolio_id,
                    var_95_cents,
                    var_99_cents,
                    expected_shortfall_cents,
                    beta_x1e6,
                    sharpe_x1e6,
                }
            },
        )
}

fn volatility_strategy() -> impl Strategy<Value = Volatility> {
    (any::<u32>(), any::<u16>(), any::<u32>(), any::<u32>()).prop_map(
        |(underlying_id, tenor_days, implied_vol_x1e6, historical_vol_x1e6)| Volatility {
            underlying_id,
            tenor_days,
            implied_vol_x1e6,
            historical_vol_x1e6,
        },
    )
}

fn greeks_strategy() -> impl Strategy<Value = Greeks> {
    (
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
    )
        .prop_map(
            |(delta_x1e6, gamma_x1e9, theta_x1e6, vega_x1e6, rho_x1e6)| Greeks {
                delta_x1e6,
                gamma_x1e9,
                theta_x1e6,
                vega_x1e6,
                rho_x1e6,
            },
        )
}

// ── Tests ─────────────────────────────────────────────────────────────────────

proptest! {
    // 1. OptionContract roundtrip
    #[test]
    fn test_option_contract_roundtrip(contract in option_contract_strategy()) {
        let encoded = encode_to_vec(&contract).expect("encode OptionContract");
        let (decoded, _): (OptionContract, usize) =
            decode_from_slice(&encoded).expect("decode OptionContract");
        prop_assert_eq!(contract, decoded);
    }

    // 2. FuturesPosition roundtrip
    #[test]
    fn test_futures_position_roundtrip(pos in futures_position_strategy()) {
        let encoded = encode_to_vec(&pos).expect("encode FuturesPosition");
        let (decoded, _): (FuturesPosition, usize) =
            decode_from_slice(&encoded).expect("decode FuturesPosition");
        prop_assert_eq!(pos, decoded);
    }

    // 3. RiskMetrics roundtrip
    #[test]
    fn test_risk_metrics_roundtrip(metrics in risk_metrics_strategy()) {
        let encoded = encode_to_vec(&metrics).expect("encode RiskMetrics");
        let (decoded, _): (RiskMetrics, usize) =
            decode_from_slice(&encoded).expect("decode RiskMetrics");
        prop_assert_eq!(metrics, decoded);
    }

    // 4. Volatility roundtrip
    #[test]
    fn test_volatility_roundtrip(vol in volatility_strategy()) {
        let encoded = encode_to_vec(&vol).expect("encode Volatility");
        let (decoded, _): (Volatility, usize) =
            decode_from_slice(&encoded).expect("decode Volatility");
        prop_assert_eq!(vol, decoded);
    }

    // 5. Greeks roundtrip
    #[test]
    fn test_greeks_roundtrip(greeks in greeks_strategy()) {
        let encoded = encode_to_vec(&greeks).expect("encode Greeks");
        let (decoded, _): (Greeks, usize) =
            decode_from_slice(&encoded).expect("decode Greeks");
        prop_assert_eq!(greeks, decoded);
    }

    // 6. OptionType enum roundtrip
    #[test]
    fn test_option_type_roundtrip(ot in option_type_strategy()) {
        let encoded = encode_to_vec(&ot).expect("encode OptionType");
        let (decoded, _): (OptionType, usize) =
            decode_from_slice(&encoded).expect("decode OptionType");
        prop_assert_eq!(ot, decoded);
    }

    // 7. ExerciseStyle enum roundtrip
    #[test]
    fn test_exercise_style_roundtrip(style in exercise_style_strategy()) {
        let encoded = encode_to_vec(&style).expect("encode ExerciseStyle");
        let (decoded, _): (ExerciseStyle, usize) =
            decode_from_slice(&encoded).expect("decode ExerciseStyle");
        prop_assert_eq!(style, decoded);
    }

    // 8. RiskFactor enum roundtrip
    #[test]
    fn test_risk_factor_roundtrip(rf in risk_factor_strategy()) {
        let encoded = encode_to_vec(&rf).expect("encode RiskFactor");
        let (decoded, _): (RiskFactor, usize) =
            decode_from_slice(&encoded).expect("decode RiskFactor");
        prop_assert_eq!(rf, decoded);
    }

    // 9. Deterministic encoding of OptionContract — encoding same value twice yields identical bytes
    #[test]
    fn test_option_contract_encoding_is_deterministic(contract in option_contract_strategy()) {
        let enc1 = encode_to_vec(&contract).expect("encode first");
        let enc2 = encode_to_vec(&contract).expect("encode second");
        prop_assert_eq!(enc1, enc2);
    }

    // 10. Deterministic encoding of FuturesPosition
    #[test]
    fn test_futures_position_encoding_is_deterministic(pos in futures_position_strategy()) {
        let enc1 = encode_to_vec(&pos).expect("encode first");
        let enc2 = encode_to_vec(&pos).expect("encode second");
        prop_assert_eq!(enc1, enc2);
    }

    // 11. Consumed bytes equals total encoded length for OptionContract
    #[test]
    fn test_option_contract_consumed_bytes(contract in option_contract_strategy()) {
        let encoded = encode_to_vec(&contract).expect("encode OptionContract");
        let total_len = encoded.len();
        let (_, consumed): (OptionContract, usize) =
            decode_from_slice(&encoded).expect("decode OptionContract");
        prop_assert_eq!(consumed, total_len);
    }

    // 12. Consumed bytes equals total encoded length for Greeks
    #[test]
    fn test_greeks_consumed_bytes(greeks in greeks_strategy()) {
        let encoded = encode_to_vec(&greeks).expect("encode Greeks");
        let total_len = encoded.len();
        let (_, consumed): (Greeks, usize) =
            decode_from_slice(&encoded).expect("decode Greeks");
        prop_assert_eq!(consumed, total_len);
    }

    // 13. Vec<OptionContract> roundtrip
    #[test]
    fn test_vec_option_contract_roundtrip(
        contracts in proptest::collection::vec(option_contract_strategy(), 0..=10)
    ) {
        let encoded = encode_to_vec(&contracts).expect("encode Vec<OptionContract>");
        let (decoded, _): (Vec<OptionContract>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<OptionContract>");
        prop_assert_eq!(contracts, decoded);
    }

    // 14. Vec<FuturesPosition> roundtrip
    #[test]
    fn test_vec_futures_position_roundtrip(
        positions in proptest::collection::vec(futures_position_strategy(), 0..=10)
    ) {
        let encoded = encode_to_vec(&positions).expect("encode Vec<FuturesPosition>");
        let (decoded, _): (Vec<FuturesPosition>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<FuturesPosition>");
        prop_assert_eq!(positions, decoded);
    }

    // 15. Vec<RiskFactor> roundtrip — exercises all enum variants in sequence
    #[test]
    fn test_vec_risk_factor_roundtrip(
        factors in proptest::collection::vec(risk_factor_strategy(), 0..=20)
    ) {
        let encoded = encode_to_vec(&factors).expect("encode Vec<RiskFactor>");
        let (decoded, _): (Vec<RiskFactor>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<RiskFactor>");
        prop_assert_eq!(factors, decoded);
    }

    // 16. Option<OptionContract> roundtrip — Some branch
    #[test]
    fn test_option_some_option_contract_roundtrip(contract in option_contract_strategy()) {
        let wrapped: Option<OptionContract> = Some(contract);
        let encoded = encode_to_vec(&wrapped).expect("encode Option<OptionContract>");
        let (decoded, _): (Option<OptionContract>, usize) =
            decode_from_slice(&encoded).expect("decode Option<OptionContract>");
        prop_assert_eq!(wrapped, decoded);
    }

    // 17. Option<Greeks> roundtrip — None branch exercised via proptest bool
    #[test]
    fn test_option_greeks_roundtrip(greeks in greeks_strategy(), present in any::<bool>()) {
        let wrapped: Option<Greeks> = if present { Some(greeks) } else { None };
        let encoded = encode_to_vec(&wrapped).expect("encode Option<Greeks>");
        let (decoded, _): (Option<Greeks>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Greeks>");
        prop_assert_eq!(wrapped, decoded);
    }

    // 18. Primitive u64 (strike price cents) roundtrip
    #[test]
    fn test_u64_strike_price_roundtrip(strike_price_cents in any::<u64>()) {
        let encoded = encode_to_vec(&strike_price_cents).expect("encode u64");
        let (decoded, _): (u64, usize) =
            decode_from_slice(&encoded).expect("decode u64");
        prop_assert_eq!(strike_price_cents, decoded);
    }

    // 19. Primitive i32 (delta scaled 1e6) roundtrip
    #[test]
    fn test_i32_delta_roundtrip(delta_x1e6 in any::<i32>()) {
        let encoded = encode_to_vec(&delta_x1e6).expect("encode i32");
        let (decoded, _): (i32, usize) =
            decode_from_slice(&encoded).expect("decode i32");
        prop_assert_eq!(delta_x1e6, decoded);
    }

    // 20. Primitive u16 (tenor days) roundtrip
    #[test]
    fn test_u16_tenor_days_roundtrip(tenor_days in any::<u16>()) {
        let encoded = encode_to_vec(&tenor_days).expect("encode u16");
        let (decoded, _): (u16, usize) =
            decode_from_slice(&encoded).expect("decode u16");
        prop_assert_eq!(tenor_days, decoded);
    }

    // 21. Encoded length of Volatility is non-zero
    #[test]
    fn test_volatility_encoded_length_is_nonzero(vol in volatility_strategy()) {
        let encoded = encode_to_vec(&vol).expect("encode Volatility");
        prop_assert!(
            !encoded.is_empty(),
            "encoded Volatility must produce at least one byte"
        );
    }

    // 22. Two distinct OptionContracts with different contract_id produce different encodings
    #[test]
    fn test_distinct_option_contracts_differ_when_id_differs(
        base in option_contract_strategy(),
        alt_id in any::<u64>(),
    ) {
        prop_assume!(base.contract_id != alt_id);
        let mut alt = base.clone();
        alt.contract_id = alt_id;
        let enc_base = encode_to_vec(&base).expect("encode base OptionContract");
        let enc_alt  = encode_to_vec(&alt).expect("encode alt OptionContract");
        prop_assert_ne!(
            enc_base, enc_alt,
            "contracts with different contract_id must encode differently"
        );
    }
}
