#![cfg(feature = "compression-lz4")]

//! Advanced LZ4 compression tests for the cryptocurrency mining pool management domain.
//!
//! Covers hash rate measurements, block header data, mining pool share submissions,
//! difficulty adjustment records, ASIC performance metrics, power consumption readings,
//! cooling system states, reward distribution calculations, stratum protocol messages,
//! nonce ranges, merkle tree nodes, and related mining infrastructure types.

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

#[derive(Debug, PartialEq, Encode, Decode)]
struct HashRateMeasurement {
    worker_id: String,
    timestamp_epoch: u64,
    hash_rate_mh: f64,
    accepted_shares: u64,
    rejected_shares: u64,
    stale_shares: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BlockHeader {
    version: u32,
    prev_block_hash: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: u64,
    bits: u32,
    nonce: u32,
    height: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ShareResult {
    Accepted,
    Rejected { reason: String },
    Stale { block_height: u64 },
    Duplicate,
    LowDifficulty { submitted: f64, required: f64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PoolShareSubmission {
    miner_address: String,
    worker_name: String,
    job_id: String,
    nonce: u32,
    extra_nonce2: Vec<u8>,
    ntime: u32,
    result: ShareResult,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DifficultyAdjustment {
    block_height: u64,
    old_difficulty: f64,
    new_difficulty: f64,
    adjustment_factor: f64,
    retarget_epoch: u64,
    blocks_in_period: u32,
    average_block_time_secs: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AsicModel {
    AntminerS19,
    AntminerS19Pro,
    WhatsMinerM30S,
    AvalonMiner1246,
    Custom { manufacturer: String, model: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AsicPerformanceMetrics {
    device_id: String,
    model: AsicModel,
    nominal_hash_rate_th: f64,
    actual_hash_rate_th: f64,
    chip_temp_celsius: Vec<f32>,
    fan_speeds_rpm: Vec<u32>,
    uptime_seconds: u64,
    hardware_errors: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PowerConsumptionReading {
    pdu_id: String,
    circuit_label: String,
    voltage_v: f64,
    current_a: f64,
    power_watts: f64,
    power_factor: f32,
    energy_kwh_cumulative: f64,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CoolingMode {
    AirCooled {
        intake_temp_c: f32,
        exhaust_temp_c: f32,
    },
    ImmersionCooled {
        fluid_temp_c: f32,
        fluid_type: String,
    },
    HybridCooled {
        air_pct: f32,
        liquid_pct: f32,
    },
    Off,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CoolingSystemState {
    zone_id: u32,
    mode: CoolingMode,
    ambient_temp_celsius: f32,
    target_temp_celsius: f32,
    humidity_pct: f32,
    fan_count_active: u16,
    fan_count_total: u16,
    alert_active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RewardDistribution {
    block_height: u64,
    block_reward_satoshis: u64,
    total_fees_satoshis: u64,
    pool_fee_pct: f32,
    payouts: Vec<MinerPayout>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MinerPayout {
    address: String,
    share_pct: f32,
    amount_satoshis: u64,
    tx_fee_satoshis: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum StratumMethod {
    Subscribe {
        agent: String,
        session_id: Option<String>,
    },
    Authorize {
        username: String,
        password: String,
    },
    Notify {
        job_id: String,
        prev_hash: Vec<u8>,
        coinbase1: Vec<u8>,
        coinbase2: Vec<u8>,
        merkle_branches: Vec<Vec<u8>>,
        version: u32,
        nbits: u32,
        ntime: u32,
        clean_jobs: bool,
    },
    Submit {
        worker: String,
        job_id: String,
        extra_nonce2: Vec<u8>,
        ntime: u32,
        nonce: u32,
    },
    SetDifficulty {
        difficulty: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StratumMessage {
    id: u64,
    method: StratumMethod,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NonceRange {
    worker_id: String,
    start: u64,
    end: u64,
    step: u32,
    assigned_epoch: u64,
    exhausted: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MerkleTreeNode {
    level: u8,
    index: u32,
    hash: Vec<u8>,
    left_child_hash: Option<Vec<u8>>,
    right_child_hash: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PoolStats {
    pool_name: String,
    total_hash_rate_ph: f64,
    active_workers: u32,
    blocks_found_24h: u16,
    luck_pct: f64,
    last_block_epoch: u64,
    pplns_window: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FirmwareVersion {
    major: u16,
    minor: u16,
    patch: u16,
    build_hash: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MinerEvent {
    Started { firmware: FirmwareVersion },
    PoolConnected { pool_url: String },
    PoolDisconnected { reason: String },
    TemperatureWarning { chip_index: u32, temp_c: f32 },
    TemperatureCritical { chip_index: u32, temp_c: f32 },
    HashBoardFault { board_index: u8, error_code: u32 },
    Shutdown { clean: bool },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MinerEventLog {
    device_id: String,
    events: Vec<(u64, MinerEvent)>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ElectricityCostRecord {
    period_start_epoch: u64,
    period_end_epoch: u64,
    kwh_consumed: f64,
    rate_per_kwh_usd: f64,
    total_cost_usd: f64,
    btc_mined_satoshis: u64,
    profit_usd: f64,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn lz4_roundtrip<T: Encode + Decode + std::fmt::Debug + PartialEq>(val: &T, label: &str) -> T {
    let encoded = encode_to_vec(val).unwrap_or_else(|e| {
        panic!("encode {} failed: {}", label, e);
    });
    let compressed = compress(&encoded, Compression::Lz4).unwrap_or_else(|e| {
        panic!("compress {} failed: {}", label, e);
    });
    let decompressed = decompress(&compressed).unwrap_or_else(|e| {
        panic!("decompress {} failed: {}", label, e);
    });
    let (decoded, _): (T, usize) = decode_from_slice(&decompressed).unwrap_or_else(|e| {
        panic!("decode {} failed: {}", label, e);
    });
    decoded
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_hash_rate_measurement_roundtrip() {
    let val = HashRateMeasurement {
        worker_id: "rig-alpha-07".to_string(),
        timestamp_epoch: 1_710_000_000,
        hash_rate_mh: 98_500.75,
        accepted_shares: 142_857,
        rejected_shares: 23,
        stale_shares: 11,
    };
    assert_eq!(val, lz4_roundtrip(&val, "HashRateMeasurement"));
}

#[test]
fn test_lz4_block_header_roundtrip() {
    let val = BlockHeader {
        version: 0x2000_0000,
        prev_block_hash: vec![
            0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45,
            0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01,
            0x23, 0x45, 0x67, 0x89,
        ],
        merkle_root: vec![
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
            0xff, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
            0xdd, 0xee, 0xff, 0x00,
        ],
        timestamp: 1_710_001_234,
        bits: 0x1703_0e69,
        nonce: 0xDEAD_BEEF,
        height: 835_000,
    };
    assert_eq!(val, lz4_roundtrip(&val, "BlockHeader"));
}

#[test]
fn test_lz4_pool_share_accepted_roundtrip() {
    let val = PoolShareSubmission {
        miner_address: "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4".to_string(),
        worker_name: "asic-rack-3.unit-17".to_string(),
        job_id: "job_0x1a2b3c".to_string(),
        nonce: 0x4F3C_2D1E,
        extra_nonce2: vec![0x00, 0x00, 0x00, 0x07],
        ntime: 1_710_005_000,
        result: ShareResult::Accepted,
    };
    assert_eq!(val, lz4_roundtrip(&val, "PoolShareSubmission accepted"));
}

#[test]
fn test_lz4_pool_share_rejected_roundtrip() {
    let val = PoolShareSubmission {
        miner_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        worker_name: "gpu-rig-02".to_string(),
        job_id: "job_0xff0099".to_string(),
        nonce: 0x0000_1234,
        extra_nonce2: vec![0x01, 0x02],
        ntime: 1_710_005_100,
        result: ShareResult::Rejected {
            reason: "job not found".to_string(),
        },
    };
    assert_eq!(val, lz4_roundtrip(&val, "PoolShareSubmission rejected"));
}

#[test]
fn test_lz4_pool_share_low_difficulty_roundtrip() {
    let val = PoolShareSubmission {
        miner_address: "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy".to_string(),
        worker_name: "fpga-array-01".to_string(),
        job_id: "job_0xaaa".to_string(),
        nonce: 0xBBBB_CCCC,
        extra_nonce2: vec![0xDE, 0xAD],
        ntime: 1_710_006_000,
        result: ShareResult::LowDifficulty {
            submitted: 1_024.5,
            required: 65_536.0,
        },
    };
    assert_eq!(
        val,
        lz4_roundtrip(&val, "PoolShareSubmission low difficulty")
    );
}

#[test]
fn test_lz4_difficulty_adjustment_roundtrip() {
    let val = DifficultyAdjustment {
        block_height: 840_000,
        old_difficulty: 79_350_225_189_073.0,
        new_difficulty: 83_148_355_189_239.0,
        adjustment_factor: 1.0479,
        retarget_epoch: 1_710_100_000,
        blocks_in_period: 2016,
        average_block_time_secs: 573.2,
    };
    assert_eq!(val, lz4_roundtrip(&val, "DifficultyAdjustment"));
}

#[test]
fn test_lz4_asic_antminer_metrics_roundtrip() {
    let val = AsicPerformanceMetrics {
        device_id: "ASIC-S19P-0042".to_string(),
        model: AsicModel::AntminerS19Pro,
        nominal_hash_rate_th: 110.0,
        actual_hash_rate_th: 107.3,
        chip_temp_celsius: vec![68.5, 71.2, 69.8, 70.1, 72.4, 67.9],
        fan_speeds_rpm: vec![4200, 4350, 4180, 4290],
        uptime_seconds: 2_592_000,
        hardware_errors: 37,
    };
    assert_eq!(val, lz4_roundtrip(&val, "AsicPerformanceMetrics antminer"));
}

#[test]
fn test_lz4_asic_custom_model_roundtrip() {
    let val = AsicPerformanceMetrics {
        device_id: "CUSTOM-001".to_string(),
        model: AsicModel::Custom {
            manufacturer: "HyperChipCo".to_string(),
            model: "HC-7nm-Turbo".to_string(),
        },
        nominal_hash_rate_th: 250.0,
        actual_hash_rate_th: 243.7,
        chip_temp_celsius: vec![55.0, 56.1, 54.8],
        fan_speeds_rpm: vec![3000, 3100],
        uptime_seconds: 86_400,
        hardware_errors: 0,
    };
    assert_eq!(val, lz4_roundtrip(&val, "AsicPerformanceMetrics custom"));
}

#[test]
fn test_lz4_power_consumption_roundtrip() {
    let val = PowerConsumptionReading {
        pdu_id: "PDU-RACK-05".to_string(),
        circuit_label: "Circuit-A3".to_string(),
        voltage_v: 240.1,
        current_a: 13.75,
        power_watts: 3301.4,
        power_factor: 0.98,
        energy_kwh_cumulative: 79_234.56,
        timestamp_epoch: 1_710_050_000,
    };
    assert_eq!(val, lz4_roundtrip(&val, "PowerConsumptionReading"));
}

#[test]
fn test_lz4_cooling_air_roundtrip() {
    let val = CoolingSystemState {
        zone_id: 3,
        mode: CoolingMode::AirCooled {
            intake_temp_c: 22.5,
            exhaust_temp_c: 38.7,
        },
        ambient_temp_celsius: 24.0,
        target_temp_celsius: 25.0,
        humidity_pct: 45.0,
        fan_count_active: 12,
        fan_count_total: 12,
        alert_active: false,
    };
    assert_eq!(val, lz4_roundtrip(&val, "CoolingSystemState air"));
}

#[test]
fn test_lz4_cooling_immersion_roundtrip() {
    let val = CoolingSystemState {
        zone_id: 7,
        mode: CoolingMode::ImmersionCooled {
            fluid_temp_c: 42.3,
            fluid_type: "Novec 7100".to_string(),
        },
        ambient_temp_celsius: 30.0,
        target_temp_celsius: 40.0,
        humidity_pct: 55.0,
        fan_count_active: 0,
        fan_count_total: 0,
        alert_active: false,
    };
    assert_eq!(val, lz4_roundtrip(&val, "CoolingSystemState immersion"));
}

#[test]
fn test_lz4_reward_distribution_roundtrip() {
    let val = RewardDistribution {
        block_height: 840_001,
        block_reward_satoshis: 312_500_000,
        total_fees_satoshis: 8_750_000,
        pool_fee_pct: 2.0,
        payouts: vec![
            MinerPayout {
                address: "bc1qminer1aaa".to_string(),
                share_pct: 35.5,
                amount_satoshis: 111_781_250,
                tx_fee_satoshis: 1_500,
            },
            MinerPayout {
                address: "bc1qminer2bbb".to_string(),
                share_pct: 22.0,
                amount_satoshis: 69_300_000,
                tx_fee_satoshis: 1_500,
            },
            MinerPayout {
                address: "bc1qminer3ccc".to_string(),
                share_pct: 42.5,
                amount_satoshis: 133_918_750,
                tx_fee_satoshis: 1_500,
            },
        ],
    };
    assert_eq!(val, lz4_roundtrip(&val, "RewardDistribution"));
}

#[test]
fn test_lz4_stratum_subscribe_roundtrip() {
    let val = StratumMessage {
        id: 1,
        method: StratumMethod::Subscribe {
            agent: "oxicode-miner/0.2.0".to_string(),
            session_id: None,
        },
    };
    assert_eq!(val, lz4_roundtrip(&val, "StratumMessage subscribe"));
}

#[test]
fn test_lz4_stratum_notify_roundtrip() {
    let val = StratumMessage {
        id: 42,
        method: StratumMethod::Notify {
            job_id: "job_8f3a".to_string(),
            prev_hash: vec![0xaa; 32],
            coinbase1: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            coinbase2: vec![0xf0, 0xe0, 0xd0, 0xc0],
            merkle_branches: vec![
                vec![0x11; 32],
                vec![0x22; 32],
                vec![0x33; 32],
                vec![0x44; 32],
            ],
            version: 0x2000_0004,
            nbits: 0x1703_0e69,
            ntime: 1_710_010_000,
            clean_jobs: true,
        },
    };
    assert_eq!(val, lz4_roundtrip(&val, "StratumMessage notify"));
}

#[test]
fn test_lz4_stratum_set_difficulty_roundtrip() {
    let val = StratumMessage {
        id: 99,
        method: StratumMethod::SetDifficulty {
            difficulty: 65_536.0,
        },
    };
    assert_eq!(val, lz4_roundtrip(&val, "StratumMessage set_difficulty"));
}

#[test]
fn test_lz4_nonce_range_roundtrip() {
    let val = NonceRange {
        worker_id: "worker-gpufarm-12".to_string(),
        start: 0x0000_0000_0000_0000,
        end: 0x0000_0000_FFFF_FFFF,
        step: 1,
        assigned_epoch: 1_710_020_000,
        exhausted: false,
    };
    assert_eq!(val, lz4_roundtrip(&val, "NonceRange"));
}

#[test]
fn test_lz4_merkle_tree_node_roundtrip() {
    let val = MerkleTreeNode {
        level: 3,
        index: 5,
        hash: vec![
            0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x01, 0x02, 0x03, 0x04,
            0x05, 0x06, 0x07, 0x08,
        ],
        left_child_hash: Some(vec![0xaa; 32]),
        right_child_hash: Some(vec![0xbb; 32]),
    };
    assert_eq!(val, lz4_roundtrip(&val, "MerkleTreeNode"));
}

#[test]
fn test_lz4_merkle_tree_leaf_node_roundtrip() {
    let val = MerkleTreeNode {
        level: 0,
        index: 42,
        hash: vec![0xff; 32],
        left_child_hash: None,
        right_child_hash: None,
    };
    assert_eq!(val, lz4_roundtrip(&val, "MerkleTreeNode leaf"));
}

#[test]
fn test_lz4_pool_stats_roundtrip() {
    let val = PoolStats {
        pool_name: "DeepHash Mining Collective".to_string(),
        total_hash_rate_ph: 42.75,
        active_workers: 8_453,
        blocks_found_24h: 3,
        luck_pct: 112.5,
        last_block_epoch: 1_710_090_000,
        pplns_window: 100_000,
    };
    assert_eq!(val, lz4_roundtrip(&val, "PoolStats"));
}

#[test]
fn test_lz4_miner_event_log_roundtrip() {
    let val = MinerEventLog {
        device_id: "ASIC-S19-0099".to_string(),
        events: vec![
            (
                1_710_000_000,
                MinerEvent::Started {
                    firmware: FirmwareVersion {
                        major: 2,
                        minor: 14,
                        patch: 3,
                        build_hash: "a1b2c3d4".to_string(),
                    },
                },
            ),
            (
                1_710_000_005,
                MinerEvent::PoolConnected {
                    pool_url: "stratum+tcp://pool.example.com:3333".to_string(),
                },
            ),
            (
                1_710_043_200,
                MinerEvent::TemperatureWarning {
                    chip_index: 14,
                    temp_c: 85.3,
                },
            ),
            (
                1_710_043_210,
                MinerEvent::TemperatureCritical {
                    chip_index: 14,
                    temp_c: 95.1,
                },
            ),
            (1_710_043_215, MinerEvent::Shutdown { clean: true }),
        ],
    };
    assert_eq!(val, lz4_roundtrip(&val, "MinerEventLog"));
}

#[test]
fn test_lz4_electricity_cost_record_roundtrip() {
    let val = ElectricityCostRecord {
        period_start_epoch: 1_709_251_200,
        period_end_epoch: 1_709_337_600,
        kwh_consumed: 792.4,
        rate_per_kwh_usd: 0.065,
        total_cost_usd: 51.506,
        btc_mined_satoshis: 87_500,
        profit_usd: 12.34,
    };
    assert_eq!(val, lz4_roundtrip(&val, "ElectricityCostRecord"));
}

#[test]
fn test_lz4_bulk_hash_rate_compression_ratio() {
    let measurements: Vec<HashRateMeasurement> = (0u32..200)
        .map(|i| HashRateMeasurement {
            worker_id: format!("rig-{:04}", i),
            timestamp_epoch: 1_710_000_000 + u64::from(i) * 60,
            hash_rate_mh: 95_000.0 + f64::from(i) * 10.0,
            accepted_shares: 1000 + u64::from(i) * 7,
            rejected_shares: u64::from(i % 5),
            stale_shares: u64::from(i % 3),
        })
        .collect();

    let encoded = encode_to_vec(&measurements).expect("encode bulk hash rates");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress bulk hash rates");
    let decompressed = decompress(&compressed).expect("decompress bulk hash rates");
    let (decoded, _): (Vec<HashRateMeasurement>, usize) =
        decode_from_slice(&decompressed).expect("decode bulk hash rates");

    assert_eq!(measurements.len(), decoded.len());
    assert_eq!(measurements, decoded);
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
}
