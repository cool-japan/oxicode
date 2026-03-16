//! Advanced checksum integration tests — financial risk management domain.
//!
//! Tests the OxiCode checksum API using financial risk management data structures
//! covering wrap/unwrap roundtrips, header size invariants, corruption detection,
//! and edge cases with nested data.

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{unwrap_with_checksum, wrap_with_checksum, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types: financial risk management
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum RiskCategory {
    Market,
    Credit,
    Liquidity,
    Operational,
    Legal,
    Reputational,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct RiskFactor {
    id: u32,
    category: RiskCategory,
    level: RiskLevel,
    score: f64,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Portfolio {
    portfolio_id: u64,
    name: String,
    factors: Vec<RiskFactor>,
    total_exposure: f64,
    var_95: f64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct RiskReport {
    report_id: u64,
    timestamp: u64,
    portfolios: Vec<Portfolio>,
    aggregate_risk: RiskLevel,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_risk_factor(id: u32, category: RiskCategory, level: RiskLevel) -> RiskFactor {
    RiskFactor {
        id,
        category,
        level,
        score: id as f64 * 1.5,
        description: format!("Risk factor #{} description text", id),
    }
}

fn make_portfolio(portfolio_id: u64, name: &str) -> Portfolio {
    Portfolio {
        portfolio_id,
        name: name.to_string(),
        factors: vec![
            make_risk_factor(1, RiskCategory::Market, RiskLevel::High),
            make_risk_factor(2, RiskCategory::Credit, RiskLevel::Medium),
        ],
        total_exposure: 1_500_000.0,
        var_95: 42_500.0,
    }
}

// ---------------------------------------------------------------------------
// Test 1: RiskFactor wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_risk_factor_wrap_unwrap_roundtrip() {
    let factor = make_risk_factor(101, RiskCategory::Market, RiskLevel::High);
    let encoded = encode_to_vec(&factor).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (RiskFactor, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(factor, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Portfolio wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_portfolio_wrap_unwrap_roundtrip() {
    let portfolio = make_portfolio(9001, "Alpha Fund");
    let encoded = encode_to_vec(&portfolio).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (Portfolio, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(portfolio, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: RiskReport wrap/unwrap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_risk_report_wrap_unwrap_roundtrip() {
    let report = RiskReport {
        report_id: 20260315_001,
        timestamp: 1_742_000_000,
        portfolios: vec![
            make_portfolio(1, "Equity Fund"),
            make_portfolio(2, "Fixed Income Fund"),
        ],
        aggregate_risk: RiskLevel::Medium,
    };
    let encoded = encode_to_vec(&report).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (RiskReport, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: HEADER_SIZE is exactly 16
// ---------------------------------------------------------------------------

#[test]
fn test_header_size_is_16() {
    assert_eq!(HEADER_SIZE, 16, "HEADER_SIZE must be exactly 16 bytes");
}

// ---------------------------------------------------------------------------
// Test 5: wrapped length equals payload length + HEADER_SIZE
// ---------------------------------------------------------------------------

#[test]
fn test_wrapped_length_equals_payload_plus_header() {
    let factor = make_risk_factor(42, RiskCategory::Credit, RiskLevel::Low);
    let encoded = encode_to_vec(&factor).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    assert_eq!(
        wrapped.len(),
        encoded.len() + HEADER_SIZE,
        "wrapped length must be payload length plus HEADER_SIZE"
    );
}

// ---------------------------------------------------------------------------
// Test 6: corruption detection — all bytes after index 4 flipped
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_detected_full_flip_after_index_4() {
    let report = RiskReport {
        report_id: 777,
        timestamp: 1_700_000_000,
        portfolios: vec![make_portfolio(10, "Corrupted Fund")],
        aggregate_risk: RiskLevel::Critical,
    };
    let encoded = encode_to_vec(&report).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);

    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }

    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "fully corrupted data (all bytes after index 4 flipped) must return Err"
    );
}

// ---------------------------------------------------------------------------
// Test 7: empty portfolio — Portfolio with no risk factors
// ---------------------------------------------------------------------------

#[test]
fn test_empty_portfolio_roundtrip() {
    let empty_portfolio = Portfolio {
        portfolio_id: 0,
        name: "Empty Portfolio".to_string(),
        factors: vec![],
        total_exposure: 0.0,
        var_95: 0.0,
    };
    let encoded = encode_to_vec(&empty_portfolio).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (Portfolio, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(empty_portfolio, decoded);
    assert!(
        decoded.factors.is_empty(),
        "decoded portfolio must have no risk factors"
    );
}

// ---------------------------------------------------------------------------
// Test 8: large risk factor list (1000 factors)
// ---------------------------------------------------------------------------

#[test]
fn test_large_risk_factor_list_roundtrip() {
    let factors: Vec<RiskFactor> = (0..1000)
        .map(|i| {
            let category = match i % 6 {
                0 => RiskCategory::Market,
                1 => RiskCategory::Credit,
                2 => RiskCategory::Liquidity,
                3 => RiskCategory::Operational,
                4 => RiskCategory::Legal,
                _ => RiskCategory::Reputational,
            };
            let level = match i % 4 {
                0 => RiskLevel::Low,
                1 => RiskLevel::Medium,
                2 => RiskLevel::High,
                _ => RiskLevel::Critical,
            };
            make_risk_factor(i as u32, category, level)
        })
        .collect();

    let portfolio = Portfolio {
        portfolio_id: 8888,
        name: "Large Factor Portfolio".to_string(),
        factors,
        total_exposure: 500_000_000.0,
        var_95: 12_750_000.0,
    };

    let encoded = encode_to_vec(&portfolio).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (Portfolio, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(portfolio, decoded);
    assert_eq!(
        decoded.factors.len(),
        1000,
        "decoded portfolio must have 1000 factors"
    );
}

// ---------------------------------------------------------------------------
// Test 9: nested portfolio with deeply nested RiskFactor data
// ---------------------------------------------------------------------------

#[test]
fn test_nested_portfolio_roundtrip() {
    let portfolio = Portfolio {
        portfolio_id: 1234,
        name: "Nested Structure Portfolio".to_string(),
        factors: vec![
            RiskFactor {
                id: 1,
                category: RiskCategory::Liquidity,
                level: RiskLevel::Critical,
                score: 99.9,
                description: "Highly illiquid position with overnight funding risk".to_string(),
            },
            RiskFactor {
                id: 2,
                category: RiskCategory::Operational,
                level: RiskLevel::High,
                score: 78.5,
                description: "System outage risk during peak trading hours".to_string(),
            },
            RiskFactor {
                id: 3,
                category: RiskCategory::Legal,
                level: RiskLevel::Medium,
                score: 45.0,
                description: "Pending regulatory compliance review".to_string(),
            },
        ],
        total_exposure: 250_000_000.0,
        var_95: 8_500_000.0,
    };

    let encoded = encode_to_vec(&portfolio).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (Portfolio, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(portfolio, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: all six RiskCategory variants survive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_risk_categories_roundtrip() {
    let categories = vec![
        RiskCategory::Market,
        RiskCategory::Credit,
        RiskCategory::Liquidity,
        RiskCategory::Operational,
        RiskCategory::Legal,
        RiskCategory::Reputational,
    ];

    for category in categories {
        let factor = RiskFactor {
            id: 0,
            category: category.clone(),
            level: RiskLevel::Low,
            score: 0.0,
            description: format!("Testing {:?} category", category),
        };
        let encoded = encode_to_vec(&factor).expect("encode_to_vec failed");
        let wrapped = wrap_with_checksum(&encoded);
        let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
        let (decoded, _): (RiskFactor, usize) =
            decode_from_slice(&unwrapped).expect("decode_from_slice failed");
        assert_eq!(factor, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 11: all four RiskLevel variants survive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_risk_levels_roundtrip() {
    let levels = vec![
        RiskLevel::Low,
        RiskLevel::Medium,
        RiskLevel::High,
        RiskLevel::Critical,
    ];

    for level in levels {
        let factor = RiskFactor {
            id: 99,
            category: RiskCategory::Market,
            level: level.clone(),
            score: 50.0,
            description: format!("Testing {:?} level", level),
        };
        let encoded = encode_to_vec(&factor).expect("encode_to_vec failed");
        let wrapped = wrap_with_checksum(&encoded);
        let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
        let (decoded, _): (RiskFactor, usize) =
            decode_from_slice(&unwrapped).expect("decode_from_slice failed");
        assert_eq!(factor, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 12: RiskReport with multiple portfolios
// ---------------------------------------------------------------------------

#[test]
fn test_risk_report_with_multiple_portfolios() {
    let report = RiskReport {
        report_id: 20260315_002,
        timestamp: 1_742_050_000,
        portfolios: vec![
            make_portfolio(101, "Equity Long/Short"),
            make_portfolio(102, "Global Macro"),
            make_portfolio(103, "Credit Arbitrage"),
            make_portfolio(104, "Volatility Fund"),
        ],
        aggregate_risk: RiskLevel::High,
    };

    let encoded = encode_to_vec(&report).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (RiskReport, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(report, decoded);
    assert_eq!(
        decoded.portfolios.len(),
        4,
        "decoded report must have 4 portfolios"
    );
}

// ---------------------------------------------------------------------------
// Test 13: wrap of raw bytes is exactly HEADER_SIZE bytes longer
// ---------------------------------------------------------------------------

#[test]
fn test_wrap_overhead_is_exactly_header_size() {
    let payload = b"raw financial payload bytes";
    let wrapped = wrap_with_checksum(payload);
    assert_eq!(
        wrapped.len() - payload.len(),
        HEADER_SIZE,
        "wrap overhead must be exactly HEADER_SIZE bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 14: RiskReport with Critical aggregate risk level
// ---------------------------------------------------------------------------

#[test]
fn test_risk_report_critical_aggregate_roundtrip() {
    let report = RiskReport {
        report_id: 999,
        timestamp: 1_742_100_000,
        portfolios: vec![Portfolio {
            portfolio_id: 500,
            name: "Crisis Portfolio".to_string(),
            factors: vec![
                make_risk_factor(1, RiskCategory::Market, RiskLevel::Critical),
                make_risk_factor(2, RiskCategory::Liquidity, RiskLevel::Critical),
                make_risk_factor(3, RiskCategory::Credit, RiskLevel::Critical),
            ],
            total_exposure: 2_000_000_000.0,
            var_95: 100_000_000.0,
        }],
        aggregate_risk: RiskLevel::Critical,
    };

    let encoded = encode_to_vec(&report).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (RiskReport, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(report, decoded);
    assert_eq!(decoded.aggregate_risk, RiskLevel::Critical);
}

// ---------------------------------------------------------------------------
// Test 15: single byte corruption at payload start is detected
// ---------------------------------------------------------------------------

#[test]
fn test_single_byte_corruption_at_payload_start_detected() {
    let factor = make_risk_factor(200, RiskCategory::Reputational, RiskLevel::Medium);
    let encoded = encode_to_vec(&factor).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);

    let mut corrupted = wrapped.clone();
    corrupted[HEADER_SIZE] ^= 0x01;

    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "single byte flip at payload start must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 16: portfolio with Reputational risk factors
// ---------------------------------------------------------------------------

#[test]
fn test_portfolio_with_reputational_factors() {
    let portfolio = Portfolio {
        portfolio_id: 3333,
        name: "ESG Compliance Portfolio".to_string(),
        factors: vec![
            RiskFactor {
                id: 1,
                category: RiskCategory::Reputational,
                level: RiskLevel::High,
                score: 82.3,
                description: "ESG compliance violation risk".to_string(),
            },
            RiskFactor {
                id: 2,
                category: RiskCategory::Legal,
                level: RiskLevel::High,
                score: 77.1,
                description: "Regulatory penalty exposure".to_string(),
            },
        ],
        total_exposure: 75_000_000.0,
        var_95: 3_200_000.0,
    };

    let encoded = encode_to_vec(&portfolio).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (Portfolio, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(portfolio, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: RiskReport with empty portfolios list
// ---------------------------------------------------------------------------

#[test]
fn test_risk_report_with_no_portfolios() {
    let report = RiskReport {
        report_id: 1,
        timestamp: 0,
        portfolios: vec![],
        aggregate_risk: RiskLevel::Low,
    };

    let encoded = encode_to_vec(&report).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (RiskReport, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(report, decoded);
    assert!(
        decoded.portfolios.is_empty(),
        "decoded report must have no portfolios"
    );
}

// ---------------------------------------------------------------------------
// Test 18: multiple sequential wrap/unwrap operations on the same data
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_sequential_wrap_unwrap_operations() {
    let factor = make_risk_factor(300, RiskCategory::Operational, RiskLevel::Critical);
    let encoded = encode_to_vec(&factor).expect("encode_to_vec failed");

    // Perform 5 wrap/unwrap cycles — each must yield the original encoded bytes
    let mut current = encoded.clone();
    for cycle in 0..5 {
        let wrapped = wrap_with_checksum(&current);
        current = unwrap_with_checksum(&wrapped)
            .unwrap_or_else(|e| panic!("unwrap failed on cycle {}: {}", cycle, e));
    }
    assert_eq!(
        current, encoded,
        "bytes must be identical after 5 wrap/unwrap cycles"
    );
}

// ---------------------------------------------------------------------------
// Test 19: high-precision f64 scores survive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_high_precision_f64_scores_roundtrip() {
    let factor = RiskFactor {
        id: 404,
        category: RiskCategory::Market,
        level: RiskLevel::High,
        score: core::f64::consts::PI * 1_000_000.0,
        description: "High-precision volatility score".to_string(),
    };

    let encoded = encode_to_vec(&factor).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (RiskFactor, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(
        factor.score, decoded.score,
        "f64 score must survive roundtrip without loss"
    );
}

// ---------------------------------------------------------------------------
// Test 20: portfolio with maximum u64 id
// ---------------------------------------------------------------------------

#[test]
fn test_portfolio_with_max_u64_id() {
    let portfolio = Portfolio {
        portfolio_id: u64::MAX,
        name: "Max ID Portfolio".to_string(),
        factors: vec![make_risk_factor(1, RiskCategory::Market, RiskLevel::Low)],
        total_exposure: f64::MAX / 2.0,
        var_95: f64::EPSILON,
    };

    let encoded = encode_to_vec(&portfolio).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (Portfolio, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(portfolio.portfolio_id, decoded.portfolio_id);
}

// ---------------------------------------------------------------------------
// Test 21: corruption detection with last byte of payload flipped
// ---------------------------------------------------------------------------

#[test]
fn test_corruption_detected_at_last_payload_byte() {
    let portfolio = make_portfolio(7777, "Last Byte Test Fund");
    let encoded = encode_to_vec(&portfolio).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);

    let mut corrupted = wrapped.clone();
    // Flip the very last byte of the wrapped data (which is the last payload byte)
    let last_idx = corrupted.len() - 1;
    corrupted[last_idx] ^= 0xFF;

    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "corruption at the last payload byte must be detected"
    );
}

// ---------------------------------------------------------------------------
// Test 22: large RiskReport with many portfolios each having many factors
// ---------------------------------------------------------------------------

#[test]
fn test_large_risk_report_many_portfolios_many_factors() {
    let portfolios: Vec<Portfolio> = (0..10)
        .map(|p_idx| {
            let factors: Vec<RiskFactor> = (0..50)
                .map(|f_idx| {
                    let category = match (p_idx + f_idx) % 6 {
                        0 => RiskCategory::Market,
                        1 => RiskCategory::Credit,
                        2 => RiskCategory::Liquidity,
                        3 => RiskCategory::Operational,
                        4 => RiskCategory::Legal,
                        _ => RiskCategory::Reputational,
                    };
                    let level = match (p_idx * f_idx) % 4 {
                        0 => RiskLevel::Low,
                        1 => RiskLevel::Medium,
                        2 => RiskLevel::High,
                        _ => RiskLevel::Critical,
                    };
                    RiskFactor {
                        id: (p_idx * 100 + f_idx) as u32,
                        category,
                        level,
                        score: (p_idx as f64 * 10.0) + (f_idx as f64 * 0.5),
                        description: format!("Portfolio {} Factor {}", p_idx, f_idx),
                    }
                })
                .collect();

            Portfolio {
                portfolio_id: p_idx as u64 + 1000,
                name: format!("Fund Portfolio {}", p_idx),
                factors,
                total_exposure: (p_idx as f64 + 1.0) * 10_000_000.0,
                var_95: (p_idx as f64 + 1.0) * 250_000.0,
            }
        })
        .collect();

    let report = RiskReport {
        report_id: 20260315_003,
        timestamp: 1_742_200_000,
        portfolios,
        aggregate_risk: RiskLevel::High,
    };

    let encoded = encode_to_vec(&report).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let unwrapped = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    let (decoded, _): (RiskReport, usize) =
        decode_from_slice(&unwrapped).expect("decode_from_slice failed");
    assert_eq!(report, decoded);
    assert_eq!(decoded.portfolios.len(), 10);
    assert_eq!(decoded.portfolios[0].factors.len(), 50);
}
