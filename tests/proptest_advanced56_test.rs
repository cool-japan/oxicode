//! Proptest-based tests for renewable energy / power generation domain.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnergySource {
    Solar,
    Wind,
    Hydro,
    Geothermal,
    Tidal,
    Biomass,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GridState {
    Stable,
    Surplus,
    Deficit,
    Islanded,
    Fault,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerReading {
    plant_id: u32,
    source: EnergySource,
    output_mw_micro: u64,
    capacity_mw_micro: u64,
    efficiency_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridSnapshot {
    snapshot_id: u64,
    state: GridState,
    readings: Vec<PowerReading>,
    total_demand_mw_micro: u64,
    timestamp_s: u64,
}

fn energy_source_strategy() -> impl Strategy<Value = EnergySource> {
    (0u8..6).prop_map(|v| match v {
        0 => EnergySource::Solar,
        1 => EnergySource::Wind,
        2 => EnergySource::Hydro,
        3 => EnergySource::Geothermal,
        4 => EnergySource::Tidal,
        _ => EnergySource::Biomass,
    })
}

fn grid_state_strategy() -> impl Strategy<Value = GridState> {
    (0u8..5).prop_map(|v| match v {
        0 => GridState::Stable,
        1 => GridState::Surplus,
        2 => GridState::Deficit,
        3 => GridState::Islanded,
        _ => GridState::Fault,
    })
}

fn power_reading_strategy() -> impl Strategy<Value = PowerReading> {
    (
        any::<u32>(),
        energy_source_strategy(),
        any::<u64>(),
        any::<u64>(),
        any::<u8>(),
    )
        .prop_map(
            |(plant_id, source, output_mw_micro, capacity_mw_micro, efficiency_pct)| PowerReading {
                plant_id,
                source,
                output_mw_micro,
                capacity_mw_micro,
                efficiency_pct,
            },
        )
}

fn grid_snapshot_strategy() -> impl Strategy<Value = GridSnapshot> {
    (
        any::<u64>(),
        grid_state_strategy(),
        prop::collection::vec(power_reading_strategy(), 0..8),
        any::<u64>(),
        any::<u64>(),
    )
        .prop_map(
            |(snapshot_id, state, readings, total_demand_mw_micro, timestamp_s)| GridSnapshot {
                snapshot_id,
                state,
                readings,
                total_demand_mw_micro,
                timestamp_s,
            },
        )
}

proptest! {
    #[test]
    fn test_power_reading_roundtrip(reading in power_reading_strategy()) {
        let encoded = encode_to_vec(&reading).expect("PowerReading encode failed");
        let (decoded, _): (PowerReading, usize) =
            decode_from_slice(&encoded).expect("PowerReading decode failed");
        prop_assert_eq!(reading, decoded);
    }

    #[test]
    fn test_grid_snapshot_roundtrip(snapshot in grid_snapshot_strategy()) {
        let encoded = encode_to_vec(&snapshot).expect("GridSnapshot encode failed");
        let (decoded, _): (GridSnapshot, usize) =
            decode_from_slice(&encoded).expect("GridSnapshot decode failed");
        prop_assert_eq!(snapshot, decoded);
    }

    #[test]
    fn test_power_reading_consumed_bytes_equals_encoded_length(reading in power_reading_strategy()) {
        let encoded = encode_to_vec(&reading).expect("PowerReading encode failed");
        let (_, consumed): (PowerReading, usize) =
            decode_from_slice(&encoded).expect("PowerReading decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_grid_snapshot_consumed_bytes_equals_encoded_length(snapshot in grid_snapshot_strategy()) {
        let encoded = encode_to_vec(&snapshot).expect("GridSnapshot encode failed");
        let (_, consumed): (GridSnapshot, usize) =
            decode_from_slice(&encoded).expect("GridSnapshot decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_power_reading_encode_deterministic(reading in power_reading_strategy()) {
        let encoded_a = encode_to_vec(&reading).expect("PowerReading first encode failed");
        let encoded_b = encode_to_vec(&reading).expect("PowerReading second encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }

    #[test]
    fn test_grid_snapshot_encode_deterministic(snapshot in grid_snapshot_strategy()) {
        let encoded_a = encode_to_vec(&snapshot).expect("GridSnapshot first encode failed");
        let encoded_b = encode_to_vec(&snapshot).expect("GridSnapshot second encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }

    #[test]
    fn test_vec_power_reading_roundtrip(readings in prop::collection::vec(power_reading_strategy(), 0..8)) {
        let encoded = encode_to_vec(&readings).expect("Vec<PowerReading> encode failed");
        let (decoded, _): (Vec<PowerReading>, usize) =
            decode_from_slice(&encoded).expect("Vec<PowerReading> decode failed");
        prop_assert_eq!(readings, decoded);
    }

    #[test]
    fn test_option_power_reading_roundtrip(reading in proptest::option::of(power_reading_strategy())) {
        let encoded = encode_to_vec(&reading).expect("Option<PowerReading> encode failed");
        let (decoded, _): (Option<PowerReading>, usize) =
            decode_from_slice(&encoded).expect("Option<PowerReading> decode failed");
        prop_assert_eq!(reading, decoded);
    }

    #[test]
    fn test_energy_source_variant_roundtrip(idx in 0u8..6u8) {
        let source = match idx {
            0 => EnergySource::Solar,
            1 => EnergySource::Wind,
            2 => EnergySource::Hydro,
            3 => EnergySource::Geothermal,
            4 => EnergySource::Tidal,
            _ => EnergySource::Biomass,
        };
        let encoded = encode_to_vec(&source).expect("EnergySource encode failed");
        let (decoded, _): (EnergySource, usize) =
            decode_from_slice(&encoded).expect("EnergySource decode failed");
        prop_assert_eq!(source, decoded);
    }

    #[test]
    fn test_grid_state_variant_roundtrip(idx in 0u8..5u8) {
        let state = match idx {
            0 => GridState::Stable,
            1 => GridState::Surplus,
            2 => GridState::Deficit,
            3 => GridState::Islanded,
            _ => GridState::Fault,
        };
        let encoded = encode_to_vec(&state).expect("GridState encode failed");
        let (decoded, _): (GridState, usize) =
            decode_from_slice(&encoded).expect("GridState decode failed");
        prop_assert_eq!(state, decoded);
    }

    #[test]
    fn test_u8_basic_roundtrip(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("u8 encode failed");
        let (decoded, _): (u8, usize) = decode_from_slice(&encoded).expect("u8 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i32_basic_roundtrip(val in any::<i32>()) {
        let encoded = encode_to_vec(&val).expect("i32 encode failed");
        let (decoded, _): (i32, usize) = decode_from_slice(&encoded).expect("i32 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64_basic_roundtrip(val in any::<u64>()) {
        let encoded = encode_to_vec(&val).expect("u64 encode failed");
        let (decoded, _): (u64, usize) = decode_from_slice(&encoded).expect("u64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i64_basic_roundtrip(val in any::<i64>()) {
        let encoded = encode_to_vec(&val).expect("i64 encode failed");
        let (decoded, _): (i64, usize) = decode_from_slice(&encoded).expect("i64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool_basic_roundtrip(val in any::<bool>()) {
        let encoded = encode_to_vec(&val).expect("bool encode failed");
        let (decoded, _): (bool, usize) =
            decode_from_slice(&encoded).expect("bool decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_string_basic_roundtrip(val in "\\PC*") {
        let encoded = encode_to_vec(&val).expect("String encode failed");
        let (decoded, _): (String, usize) =
            decode_from_slice(&encoded).expect("String decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_f32_basic_roundtrip(val in any::<f32>()) {
        let encoded = encode_to_vec(&val).expect("f32 encode failed");
        let (decoded, _): (f32, usize) = decode_from_slice(&encoded).expect("f32 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_f64_basic_roundtrip(val in any::<f64>()) {
        let encoded = encode_to_vec(&val).expect("f64 encode failed");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("f64 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_vec_u8_roundtrip(val in prop::collection::vec(any::<u8>(), 0..64)) {
        let encoded = encode_to_vec(&val).expect("Vec<u8> encode failed");
        let (decoded, _): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("Vec<u8> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_string_roundtrip(val in prop::collection::vec("\\PC*", 0..8)) {
        let encoded = encode_to_vec(&val).expect("Vec<String> encode failed");
        let (decoded, _): (Vec<String>, usize) =
            decode_from_slice(&encoded).expect("Vec<String> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_option_u64_roundtrip(val in proptest::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&val).expect("Option<u64> encode failed");
        let (decoded, _): (Option<u64>, usize) =
            decode_from_slice(&encoded).expect("Option<u64> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_distinct_power_readings_encode_differently(
        plant_id_a in 0u32..1_000_000u32,
        plant_id_b in 1_000_001u32..u32::MAX,
    ) {
        let reading_a = PowerReading {
            plant_id: plant_id_a,
            source: EnergySource::Solar,
            output_mw_micro: 500_000,
            capacity_mw_micro: 1_000_000,
            efficiency_pct: 80,
        };
        let reading_b = PowerReading {
            plant_id: plant_id_b,
            source: EnergySource::Solar,
            output_mw_micro: 500_000,
            capacity_mw_micro: 1_000_000,
            efficiency_pct: 80,
        };
        let encoded_a = encode_to_vec(&reading_a).expect("reading_a encode failed");
        let encoded_b = encode_to_vec(&reading_b).expect("reading_b encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }

    #[test]
    fn test_zero_output_power_reading_roundtrip(
        plant_id in any::<u32>(),
        capacity_mw_micro in any::<u64>(),
        efficiency_pct in any::<u8>(),
        source_idx in 0u8..6u8,
    ) {
        let source = match source_idx {
            0 => EnergySource::Solar,
            1 => EnergySource::Wind,
            2 => EnergySource::Hydro,
            3 => EnergySource::Geothermal,
            4 => EnergySource::Tidal,
            _ => EnergySource::Biomass,
        };
        let reading = PowerReading {
            plant_id,
            source,
            output_mw_micro: 0,
            capacity_mw_micro,
            efficiency_pct,
        };
        let encoded = encode_to_vec(&reading).expect("zero output PowerReading encode failed");
        let (decoded, _): (PowerReading, usize) =
            decode_from_slice(&encoded).expect("zero output PowerReading decode failed");
        prop_assert_eq!(reading, decoded);
    }

    #[test]
    fn test_max_capacity_power_reading_roundtrip(
        plant_id in any::<u32>(),
        output_mw_micro in any::<u64>(),
        efficiency_pct in any::<u8>(),
        source_idx in 0u8..6u8,
    ) {
        let source = match source_idx {
            0 => EnergySource::Solar,
            1 => EnergySource::Wind,
            2 => EnergySource::Hydro,
            3 => EnergySource::Geothermal,
            4 => EnergySource::Tidal,
            _ => EnergySource::Biomass,
        };
        let reading = PowerReading {
            plant_id,
            source,
            output_mw_micro,
            capacity_mw_micro: u64::MAX,
            efficiency_pct,
        };
        let encoded =
            encode_to_vec(&reading).expect("max capacity PowerReading encode failed");
        let (decoded, _): (PowerReading, usize) =
            decode_from_slice(&encoded).expect("max capacity PowerReading decode failed");
        prop_assert_eq!(reading, decoded);
    }
}
