#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ExperimentStatus {
    Running,
    Completed,
    Failed,
    Paused,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Measurement {
    timestamp: u64,
    value: f64,
    unit: String,
    sensor_id: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Experiment {
    experiment_id: u64,
    name: String,
    status: ExperimentStatus,
    measurements: Vec<Measurement>,
    sample_count: u32,
}

fn make_measurement(timestamp: u64, value: f64, unit: &str, sensor_id: u32) -> Measurement {
    Measurement {
        timestamp,
        value,
        unit: unit.to_string(),
        sensor_id,
    }
}

fn make_experiment(
    id: u64,
    name: &str,
    status: ExperimentStatus,
    measurements: Vec<Measurement>,
    sample_count: u32,
) -> Experiment {
    Experiment {
        experiment_id: id,
        name: name.to_string(),
        status,
        measurements,
        sample_count,
    }
}

// Test 1: Measurement roundtrip with standard config
#[test]
fn test_measurement_roundtrip_standard() {
    let cfg = config::standard();
    let m = make_measurement(1_700_000_000, 23.5, "Celsius", 42);
    let bytes = encode_to_vec(&m, cfg).expect("encode Measurement standard");
    let (decoded, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Measurement standard");
    assert_eq!(m, decoded);
}

// Test 2: Measurement roundtrip with big_endian config
#[test]
fn test_measurement_roundtrip_big_endian() {
    let cfg = config::standard().with_big_endian();
    let m = make_measurement(1_700_000_001, 98.6, "Fahrenheit", 7);
    let bytes = encode_to_vec(&m, cfg).expect("encode Measurement big_endian");
    let (decoded, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Measurement big_endian");
    assert_eq!(m, decoded);
}

// Test 3: Measurement roundtrip with fixed_int_encoding config
#[test]
fn test_measurement_roundtrip_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let m = make_measurement(9_999_999_999, 0.001, "Pascal", 100);
    let bytes = encode_to_vec(&m, cfg).expect("encode Measurement fixed_int");
    let (decoded, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Measurement fixed_int");
    assert_eq!(m, decoded);
}

// Test 4: ExperimentStatus::Running roundtrip
#[test]
fn test_experiment_status_running_roundtrip() {
    let cfg = config::standard();
    let status = ExperimentStatus::Running;
    let bytes = encode_to_vec(&status, cfg).expect("encode ExperimentStatus::Running");
    let (decoded, _): (ExperimentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ExperimentStatus::Running");
    assert_eq!(status, decoded);
}

// Test 5: ExperimentStatus::Completed roundtrip
#[test]
fn test_experiment_status_completed_roundtrip() {
    let cfg = config::standard();
    let status = ExperimentStatus::Completed;
    let bytes = encode_to_vec(&status, cfg).expect("encode ExperimentStatus::Completed");
    let (decoded, _): (ExperimentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ExperimentStatus::Completed");
    assert_eq!(status, decoded);
}

// Test 6: ExperimentStatus::Failed roundtrip
#[test]
fn test_experiment_status_failed_roundtrip() {
    let cfg = config::standard();
    let status = ExperimentStatus::Failed;
    let bytes = encode_to_vec(&status, cfg).expect("encode ExperimentStatus::Failed");
    let (decoded, _): (ExperimentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ExperimentStatus::Failed");
    assert_eq!(status, decoded);
}

// Test 7: ExperimentStatus::Paused roundtrip
#[test]
fn test_experiment_status_paused_roundtrip() {
    let cfg = config::standard();
    let status = ExperimentStatus::Paused;
    let bytes = encode_to_vec(&status, cfg).expect("encode ExperimentStatus::Paused");
    let (decoded, _): (ExperimentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ExperimentStatus::Paused");
    assert_eq!(status, decoded);
}

// Test 8: Experiment with empty measurements
#[test]
fn test_experiment_empty_measurements() {
    let cfg = config::standard();
    let exp = make_experiment(1, "Empty Trial", ExperimentStatus::Paused, vec![], 0);
    let bytes = encode_to_vec(&exp, cfg).expect("encode Experiment empty measurements");
    let (decoded, _): (Experiment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Experiment empty measurements");
    assert_eq!(exp, decoded);
    assert_eq!(decoded.measurements.len(), 0);
    assert_eq!(decoded.sample_count, 0);
}

// Test 9: Experiment with many measurements
#[test]
fn test_experiment_many_measurements() {
    let cfg = config::standard();
    let measurements: Vec<Measurement> = (0..50)
        .map(|i| make_measurement(1_700_000_000 + i as u64, i as f64 * 0.5, "mV", i as u32))
        .collect();
    let count = measurements.len() as u32;
    let exp = make_experiment(
        99,
        "Large Dataset Experiment",
        ExperimentStatus::Completed,
        measurements,
        count,
    );
    let bytes = encode_to_vec(&exp, cfg).expect("encode Experiment many measurements");
    let (decoded, _): (Experiment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Experiment many measurements");
    assert_eq!(exp, decoded);
    assert_eq!(decoded.measurements.len(), 50);
}

// Test 10: Consumed bytes equals encoded length for Measurement
#[test]
fn test_measurement_consumed_bytes_equals_len() {
    let cfg = config::standard();
    let m = make_measurement(42, 3.14159, "rad/s", 5);
    let bytes = encode_to_vec(&m, cfg).expect("encode Measurement for size check");
    let (_decoded, consumed): (Measurement, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Measurement for size check");
    assert_eq!(consumed, bytes.len());
}

// Test 11: Consumed bytes equals encoded length for Experiment
#[test]
fn test_experiment_consumed_bytes_equals_len() {
    let cfg = config::standard();
    let exp = make_experiment(
        77,
        "Byte Count Test",
        ExperimentStatus::Running,
        vec![
            make_measurement(100, 1.0, "Hz", 1),
            make_measurement(200, 2.0, "Hz", 2),
        ],
        2,
    );
    let bytes = encode_to_vec(&exp, cfg).expect("encode Experiment for size check");
    let (_decoded, consumed): (Experiment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Experiment for size check");
    assert_eq!(consumed, bytes.len());
}

// Test 12: Vec<Measurement> roundtrip
#[test]
fn test_vec_measurement_roundtrip() {
    let cfg = config::standard();
    let measurements = vec![
        make_measurement(1_000, 10.0, "kg", 1),
        make_measurement(2_000, 20.0, "kg", 2),
        make_measurement(3_000, 30.0, "kg", 3),
    ];
    let bytes = encode_to_vec(&measurements, cfg).expect("encode Vec<Measurement>");
    let (decoded, _): (Vec<Measurement>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Measurement>");
    assert_eq!(measurements, decoded);
}

// Test 13: Vec<Experiment> roundtrip (nested structures)
#[test]
fn test_vec_experiment_roundtrip() {
    let cfg = config::standard();
    let experiments = vec![
        make_experiment(
            1,
            "Alpha",
            ExperimentStatus::Completed,
            vec![make_measurement(100, 5.5, "V", 10)],
            1,
        ),
        make_experiment(
            2,
            "Beta",
            ExperimentStatus::Running,
            vec![
                make_measurement(200, 7.3, "A", 11),
                make_measurement(300, 8.1, "A", 12),
            ],
            2,
        ),
    ];
    let bytes = encode_to_vec(&experiments, cfg).expect("encode Vec<Experiment>");
    let (decoded, _): (Vec<Experiment>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Experiment>");
    assert_eq!(experiments, decoded);
}

// Test 14: Measurement with big_endian and fixed_int combined
#[test]
fn test_measurement_big_endian_fixed_int() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let m = make_measurement(u64::MAX / 2, f64::EPSILON, "nm", u32::MAX / 2);
    let bytes = encode_to_vec(&m, cfg).expect("encode Measurement big_endian + fixed_int");
    let (decoded, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Measurement big_endian + fixed_int");
    assert_eq!(m, decoded);
}

// Test 15: Experiment with big_endian config
#[test]
fn test_experiment_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let exp = make_experiment(
        512,
        "Spectroscopy Run",
        ExperimentStatus::Completed,
        vec![
            make_measurement(1_609_459_200, 400.0, "nm", 3),
            make_measurement(1_609_459_260, 450.0, "nm", 3),
        ],
        2,
    );
    let bytes = encode_to_vec(&exp, cfg).expect("encode Experiment big_endian");
    let (decoded, _): (Experiment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Experiment big_endian");
    assert_eq!(exp, decoded);
}

// Test 16: Experiment with fixed_int config
#[test]
fn test_experiment_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let exp = make_experiment(
        1024,
        "Pressure Sweep",
        ExperimentStatus::Failed,
        vec![make_measurement(0, f64::NAN.abs(), "bar", 9)],
        1,
    );
    let bytes = encode_to_vec(&exp, cfg).expect("encode Experiment fixed_int");
    let (decoded, _): (Experiment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Experiment fixed_int");
    assert_eq!(decoded.experiment_id, 1024);
    assert_eq!(decoded.status, ExperimentStatus::Failed);
    assert_eq!(decoded.measurements.len(), 1);
}

// Test 17: Measurement with extreme float values
#[test]
fn test_measurement_extreme_float_values() {
    let cfg = config::standard();
    let m_max = make_measurement(0, f64::MAX, "raw", 0);
    let m_min = make_measurement(1, f64::MIN_POSITIVE, "raw", 1);
    let m_neg = make_measurement(2, -f64::MAX, "raw", 2);

    let bytes_max = encode_to_vec(&m_max, cfg).expect("encode Measurement f64::MAX");
    let (decoded_max, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes_max, cfg).expect("decode Measurement f64::MAX");
    assert_eq!(m_max.value, decoded_max.value);

    let bytes_min = encode_to_vec(&m_min, cfg).expect("encode Measurement f64::MIN_POSITIVE");
    let (decoded_min, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes_min, cfg).expect("decode Measurement f64::MIN_POSITIVE");
    assert_eq!(m_min.value, decoded_min.value);

    let bytes_neg = encode_to_vec(&m_neg, cfg).expect("encode Measurement -f64::MAX");
    let (decoded_neg, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes_neg, cfg).expect("decode Measurement -f64::MAX");
    assert_eq!(m_neg.value, decoded_neg.value);
}

// Test 18: All ExperimentStatus variants in a Vec
#[test]
fn test_all_experiment_status_variants_vec() {
    let cfg = config::standard();
    let statuses = vec![
        ExperimentStatus::Running,
        ExperimentStatus::Completed,
        ExperimentStatus::Failed,
        ExperimentStatus::Paused,
    ];
    let bytes = encode_to_vec(&statuses, cfg).expect("encode Vec<ExperimentStatus>");
    let (decoded, _): (Vec<ExperimentStatus>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ExperimentStatus>");
    assert_eq!(statuses, decoded);
    assert_eq!(decoded.len(), 4);
}

// Test 19: Experiment with unicode name and unit fields
#[test]
fn test_experiment_unicode_fields() {
    let cfg = config::standard();
    let exp = make_experiment(
        2048,
        "実験データ収集 — Série α",
        ExperimentStatus::Running,
        vec![
            make_measurement(1_700_500_000, 273.15, "°C", 88),
            make_measurement(1_700_500_060, 274.00, "°C", 88),
        ],
        2,
    );
    let bytes = encode_to_vec(&exp, cfg).expect("encode Experiment unicode");
    let (decoded, _): (Experiment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Experiment unicode");
    assert_eq!(exp, decoded);
    assert_eq!(decoded.name, "実験データ収集 — Série α");
}

// Test 20: Measurement sensor_id boundary values (u32::MAX, 0)
#[test]
fn test_measurement_sensor_id_boundaries() {
    let cfg = config::standard();
    let m_zero = make_measurement(0, 0.0, "unit", 0);
    let m_max = make_measurement(u64::MAX, -0.0, "unit", u32::MAX);

    let bytes_zero = encode_to_vec(&m_zero, cfg).expect("encode Measurement sensor_id=0");
    let (decoded_zero, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes_zero, cfg).expect("decode Measurement sensor_id=0");
    assert_eq!(m_zero.sensor_id, decoded_zero.sensor_id);
    assert_eq!(decoded_zero.sensor_id, 0);

    let bytes_max = encode_to_vec(&m_max, cfg).expect("encode Measurement sensor_id=u32::MAX");
    let (decoded_max, _): (Measurement, usize) =
        decode_owned_from_slice(&bytes_max, cfg).expect("decode Measurement sensor_id=u32::MAX");
    assert_eq!(m_max.sensor_id, decoded_max.sensor_id);
    assert_eq!(decoded_max.sensor_id, u32::MAX);
}

// Test 21: Experiment sample_count field integrity
#[test]
fn test_experiment_sample_count_integrity() {
    let cfg = config::standard();
    let measurements: Vec<Measurement> = (0..10)
        .map(|i| make_measurement(i * 1000, i as f64, "mol/L", i as u32))
        .collect();
    let exp = make_experiment(
        4096,
        "Concentration Study",
        ExperimentStatus::Completed,
        measurements,
        10,
    );
    let bytes = encode_to_vec(&exp, cfg).expect("encode Experiment sample_count");
    let (decoded, _): (Experiment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Experiment sample_count");
    assert_eq!(decoded.sample_count, 10);
    assert_eq!(decoded.measurements.len(), decoded.sample_count as usize);
}

// Test 22: Cross-config non-interoperability (standard vs big_endian encode different bytes)
#[test]
fn test_config_cross_non_interoperability() {
    let cfg_std = config::standard();
    let cfg_be = config::standard().with_big_endian();
    let m = make_measurement(1_234_567_890, 1.23456789, "T", 55);

    let bytes_std = encode_to_vec(&m, cfg_std).expect("encode Measurement std for cross-config");
    let bytes_be = encode_to_vec(&m, cfg_be).expect("encode Measurement be for cross-config");

    // Verify each config decodes its own output correctly
    let (decoded_std, consumed_std): (Measurement, usize) =
        decode_owned_from_slice(&bytes_std, cfg_std).expect("decode Measurement std");
    let (decoded_be, consumed_be): (Measurement, usize) =
        decode_owned_from_slice(&bytes_be, cfg_be).expect("decode Measurement be");

    assert_eq!(m, decoded_std);
    assert_eq!(m, decoded_be);
    assert_eq!(consumed_std, bytes_std.len());
    assert_eq!(consumed_be, bytes_be.len());

    // big_endian fixed-width encoding produces a different byte sequence for multi-byte integers
    assert_ne!(
        bytes_std, bytes_be,
        "standard and big_endian configs should produce different byte representations"
    );
}
