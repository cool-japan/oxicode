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
// Domain types — seismic monitoring / earthquake detection / geophysics
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PhaseType {
    P,
    S,
    PP,
    SS,
    PKP,
    SKS,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WaveType {
    Longitudinal,
    Transverse,
    Surface,
    Love,
    Rayleigh,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlertLevel {
    Green,
    Yellow,
    Orange,
    Red,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeismicStation {
    station_id: u32,
    lat_x1e6: i32,
    lon_x1e6: i32,
    elevation_m: i16,
    network_code: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeismicWaveform {
    channel_id: u32,
    sample_rate_hz: u32,
    samples: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Earthquake {
    eq_id: u64,
    lat_x1e6: i32,
    lon_x1e6: i32,
    depth_km_x10: u32,
    magnitude_x100: u32,
    origin_time: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeismicPhase {
    eq_id: u64,
    station_id: u32,
    phase_type: PhaseType,
    arrival_time: u64,
    residual_ms: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroundMotionRecord {
    station_id: u32,
    eq_id: u64,
    pga_mg: u32,
    pgv_mm_s_x100: u32,
    sa_1hz_mg: u32,
}

// ---------------------------------------------------------------------------
// Proptest strategies
// ---------------------------------------------------------------------------

fn phase_type_strategy() -> impl Strategy<Value = PhaseType> {
    prop_oneof![
        Just(PhaseType::P),
        Just(PhaseType::S),
        Just(PhaseType::PP),
        Just(PhaseType::SS),
        Just(PhaseType::PKP),
        Just(PhaseType::SKS),
    ]
}

fn wave_type_strategy() -> impl Strategy<Value = WaveType> {
    prop_oneof![
        Just(WaveType::Longitudinal),
        Just(WaveType::Transverse),
        Just(WaveType::Surface),
        Just(WaveType::Love),
        Just(WaveType::Rayleigh),
    ]
}

fn alert_level_strategy() -> impl Strategy<Value = AlertLevel> {
    prop_oneof![
        Just(AlertLevel::Green),
        Just(AlertLevel::Yellow),
        Just(AlertLevel::Orange),
        Just(AlertLevel::Red),
    ]
}

fn seismic_station_strategy() -> impl Strategy<Value = SeismicStation> {
    (
        any::<u32>(),
        -90_000_000_i32..=90_000_000_i32,
        -180_000_000_i32..=180_000_000_i32,
        any::<i16>(),
        any::<u8>(),
    )
        .prop_map(
            |(station_id, lat_x1e6, lon_x1e6, elevation_m, network_code)| SeismicStation {
                station_id,
                lat_x1e6,
                lon_x1e6,
                elevation_m,
                network_code,
            },
        )
}

fn seismic_waveform_strategy() -> impl Strategy<Value = SeismicWaveform> {
    (
        any::<u32>(),
        1_u32..=1000_u32,
        prop::collection::vec(any::<i32>(), 0..8),
    )
        .prop_map(|(channel_id, sample_rate_hz, samples)| SeismicWaveform {
            channel_id,
            sample_rate_hz,
            samples,
        })
}

fn earthquake_strategy() -> impl Strategy<Value = Earthquake> {
    (
        any::<u64>(),
        -90_000_000_i32..=90_000_000_i32,
        -180_000_000_i32..=180_000_000_i32,
        0_u32..=7_000_u32,
        0_u32..=1000_u32,
        any::<u64>(),
    )
        .prop_map(
            |(eq_id, lat_x1e6, lon_x1e6, depth_km_x10, magnitude_x100, origin_time)| Earthquake {
                eq_id,
                lat_x1e6,
                lon_x1e6,
                depth_km_x10,
                magnitude_x100,
                origin_time,
            },
        )
}

fn seismic_phase_strategy() -> impl Strategy<Value = SeismicPhase> {
    (
        any::<u64>(),
        any::<u32>(),
        phase_type_strategy(),
        any::<u64>(),
        any::<i32>(),
    )
        .prop_map(
            |(eq_id, station_id, phase_type, arrival_time, residual_ms)| SeismicPhase {
                eq_id,
                station_id,
                phase_type,
                arrival_time,
                residual_ms,
            },
        )
}

fn ground_motion_record_strategy() -> impl Strategy<Value = GroundMotionRecord> {
    (
        any::<u32>(),
        any::<u64>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
    )
        .prop_map(
            |(station_id, eq_id, pga_mg, pgv_mm_s_x100, sa_1hz_mg)| GroundMotionRecord {
                station_id,
                eq_id,
                pga_mg,
                pgv_mm_s_x100,
                sa_1hz_mg,
            },
        )
}

// ---------------------------------------------------------------------------
// Tests — all 22 inside a single proptest! block
// ---------------------------------------------------------------------------

proptest! {
    // 1. SeismicStation roundtrip
    #[test]
    fn test_seismic_station_roundtrip(station in seismic_station_strategy()) {
        let bytes = encode_to_vec(&station).expect("encode SeismicStation");
        let (decoded, _consumed): (SeismicStation, usize) =
            decode_from_slice(&bytes).expect("decode SeismicStation");
        prop_assert_eq!(station, decoded);
    }

    // 2. SeismicStation deterministic encoding
    #[test]
    fn test_seismic_station_deterministic(station in seismic_station_strategy()) {
        let bytes_a = encode_to_vec(&station).expect("encode SeismicStation a");
        let bytes_b = encode_to_vec(&station).expect("encode SeismicStation b");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 3. SeismicStation consumed bytes equals encoded length
    #[test]
    fn test_seismic_station_consumed_bytes(station in seismic_station_strategy()) {
        let bytes = encode_to_vec(&station).expect("encode SeismicStation");
        let (_decoded, consumed): (SeismicStation, usize) =
            decode_from_slice(&bytes).expect("decode SeismicStation");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 4. SeismicWaveform roundtrip
    #[test]
    fn test_seismic_waveform_roundtrip(waveform in seismic_waveform_strategy()) {
        let bytes = encode_to_vec(&waveform).expect("encode SeismicWaveform");
        let (decoded, _consumed): (SeismicWaveform, usize) =
            decode_from_slice(&bytes).expect("decode SeismicWaveform");
        prop_assert_eq!(waveform, decoded);
    }

    // 5. SeismicWaveform deterministic encoding
    #[test]
    fn test_seismic_waveform_deterministic(waveform in seismic_waveform_strategy()) {
        let bytes_a = encode_to_vec(&waveform).expect("encode SeismicWaveform a");
        let bytes_b = encode_to_vec(&waveform).expect("encode SeismicWaveform b");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 6. Earthquake roundtrip
    #[test]
    fn test_earthquake_roundtrip(eq in earthquake_strategy()) {
        let bytes = encode_to_vec(&eq).expect("encode Earthquake");
        let (decoded, _consumed): (Earthquake, usize) =
            decode_from_slice(&bytes).expect("decode Earthquake");
        prop_assert_eq!(eq, decoded);
    }

    // 7. Earthquake consumed bytes equals encoded length
    #[test]
    fn test_earthquake_consumed_bytes(eq in earthquake_strategy()) {
        let bytes = encode_to_vec(&eq).expect("encode Earthquake");
        let (_decoded, consumed): (Earthquake, usize) =
            decode_from_slice(&bytes).expect("decode Earthquake");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 8. SeismicPhase roundtrip
    #[test]
    fn test_seismic_phase_roundtrip(phase in seismic_phase_strategy()) {
        let bytes = encode_to_vec(&phase).expect("encode SeismicPhase");
        let (decoded, _consumed): (SeismicPhase, usize) =
            decode_from_slice(&bytes).expect("decode SeismicPhase");
        prop_assert_eq!(phase, decoded);
    }

    // 9. SeismicPhase deterministic encoding
    #[test]
    fn test_seismic_phase_deterministic(phase in seismic_phase_strategy()) {
        let bytes_a = encode_to_vec(&phase).expect("encode SeismicPhase a");
        let bytes_b = encode_to_vec(&phase).expect("encode SeismicPhase b");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 10. GroundMotionRecord roundtrip
    #[test]
    fn test_ground_motion_record_roundtrip(gmr in ground_motion_record_strategy()) {
        let bytes = encode_to_vec(&gmr).expect("encode GroundMotionRecord");
        let (decoded, _consumed): (GroundMotionRecord, usize) =
            decode_from_slice(&bytes).expect("decode GroundMotionRecord");
        prop_assert_eq!(gmr, decoded);
    }

    // 11. GroundMotionRecord consumed bytes equals encoded length
    #[test]
    fn test_ground_motion_record_consumed_bytes(gmr in ground_motion_record_strategy()) {
        let bytes = encode_to_vec(&gmr).expect("encode GroundMotionRecord");
        let (_decoded, consumed): (GroundMotionRecord, usize) =
            decode_from_slice(&bytes).expect("decode GroundMotionRecord");
        prop_assert_eq!(consumed, bytes.len());
    }

    // 12. PhaseType enum roundtrip
    #[test]
    fn test_phase_type_roundtrip(pt in phase_type_strategy()) {
        let bytes = encode_to_vec(&pt).expect("encode PhaseType");
        let (decoded, _consumed): (PhaseType, usize) =
            decode_from_slice(&bytes).expect("decode PhaseType");
        prop_assert_eq!(pt, decoded);
    }

    // 13. WaveType enum roundtrip
    #[test]
    fn test_wave_type_roundtrip(wt in wave_type_strategy()) {
        let bytes = encode_to_vec(&wt).expect("encode WaveType");
        let (decoded, _consumed): (WaveType, usize) =
            decode_from_slice(&bytes).expect("decode WaveType");
        prop_assert_eq!(wt, decoded);
    }

    // 14. AlertLevel enum roundtrip
    #[test]
    fn test_alert_level_roundtrip(al in alert_level_strategy()) {
        let bytes = encode_to_vec(&al).expect("encode AlertLevel");
        let (decoded, _consumed): (AlertLevel, usize) =
            decode_from_slice(&bytes).expect("decode AlertLevel");
        prop_assert_eq!(al, decoded);
    }

    // 15. Vec<SeismicStation> roundtrip
    #[test]
    fn test_vec_seismic_station_roundtrip(
        stations in prop::collection::vec(seismic_station_strategy(), 0..8)
    ) {
        let bytes = encode_to_vec(&stations).expect("encode Vec<SeismicStation>");
        let (decoded, consumed): (Vec<SeismicStation>, usize) =
            decode_from_slice(&bytes).expect("decode Vec<SeismicStation>");
        prop_assert_eq!(&stations, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 16. Vec<Earthquake> roundtrip
    #[test]
    fn test_vec_earthquake_roundtrip(
        events in prop::collection::vec(earthquake_strategy(), 0..6)
    ) {
        let bytes = encode_to_vec(&events).expect("encode Vec<Earthquake>");
        let (decoded, consumed): (Vec<Earthquake>, usize) =
            decode_from_slice(&bytes).expect("decode Vec<Earthquake>");
        prop_assert_eq!(&events, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 17. Option<SeismicStation> roundtrip — Some variant
    #[test]
    fn test_option_seismic_station_some_roundtrip(station in seismic_station_strategy()) {
        let opt: Option<SeismicStation> = Some(station);
        let bytes = encode_to_vec(&opt).expect("encode Option<SeismicStation> Some");
        let (decoded, consumed): (Option<SeismicStation>, usize) =
            decode_from_slice(&bytes).expect("decode Option<SeismicStation> Some");
        prop_assert_eq!(&opt, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 18. Option<Earthquake> roundtrip — None variant
    #[test]
    fn test_option_earthquake_none_roundtrip(_unused in any::<u8>()) {
        let opt: Option<Earthquake> = None;
        let bytes = encode_to_vec(&opt).expect("encode Option<Earthquake> None");
        let (decoded, consumed): (Option<Earthquake>, usize) =
            decode_from_slice(&bytes).expect("decode Option<Earthquake> None");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 19. Primitive u64 roundtrip (origin time scalar)
    #[test]
    fn test_origin_time_u64_roundtrip(origin_time in any::<u64>()) {
        let bytes = encode_to_vec(&origin_time).expect("encode u64 origin_time");
        let (decoded, consumed): (u64, usize) =
            decode_from_slice(&bytes).expect("decode u64 origin_time");
        prop_assert_eq!(origin_time, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 20. Primitive i32 roundtrip (seismic sample value)
    #[test]
    fn test_sample_i32_roundtrip(sample in any::<i32>()) {
        let bytes = encode_to_vec(&sample).expect("encode i32 sample");
        let (decoded, consumed): (i32, usize) =
            decode_from_slice(&bytes).expect("decode i32 sample");
        prop_assert_eq!(sample, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 21. Vec<SeismicPhase> roundtrip — catalogue of phase picks
    #[test]
    fn test_vec_seismic_phase_roundtrip(
        phases in prop::collection::vec(seismic_phase_strategy(), 0..6)
    ) {
        let bytes = encode_to_vec(&phases).expect("encode Vec<SeismicPhase>");
        let (decoded, consumed): (Vec<SeismicPhase>, usize) =
            decode_from_slice(&bytes).expect("decode Vec<SeismicPhase>");
        prop_assert_eq!(&phases, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 22. Vec<GroundMotionRecord> deterministic encoding — shake-map stability
    #[test]
    fn test_vec_ground_motion_deterministic(
        records in prop::collection::vec(ground_motion_record_strategy(), 0..6)
    ) {
        let bytes_a = encode_to_vec(&records).expect("encode Vec<GroundMotionRecord> a");
        let bytes_b = encode_to_vec(&records).expect("encode Vec<GroundMotionRecord> b");
        prop_assert_eq!(bytes_a, bytes_b);
    }
}
