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

#[derive(Debug, PartialEq, Encode, Decode)]
enum WaveType {
    PWave,
    SWave,
    LoveSurface,
    RayleighSurface,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MagnitudeScale {
    Richter,
    MomentMagnitude,
    BodyWave,
    SurfaceWave,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SeismicSample {
    station_id: u32,
    wave_type: WaveType,
    amplitude_nm: i32,
    period_ms: u32,
    timestamp_us: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EarthquakeEvent {
    event_id: u64,
    magnitude: u32,
    scale: MagnitudeScale,
    depth_km: u32,
    lat_micro: i32,
    lon_micro: i32,
    occurred_at: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SeismicRecord {
    record_id: u64,
    station_id: u32,
    samples: Vec<SeismicSample>,
    event: Option<EarthquakeEvent>,
}

// ── 1. WaveType::PWave compress/decompress roundtrip ──────────────────────────
#[test]
fn test_zstd_wave_type_pwave_roundtrip() {
    let wave = WaveType::PWave;
    let encoded = encode_to_vec(&wave).expect("encode WaveType::PWave failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress WaveType::PWave failed");
    let decompressed = decompress(&compressed).expect("zstd decompress WaveType::PWave failed");
    let (decoded, _): (WaveType, usize) =
        decode_from_slice(&decompressed).expect("decode WaveType::PWave failed");
    assert_eq!(wave, decoded);
}

// ── 2. WaveType::SWave compress/decompress roundtrip ──────────────────────────
#[test]
fn test_zstd_wave_type_swave_roundtrip() {
    let wave = WaveType::SWave;
    let encoded = encode_to_vec(&wave).expect("encode WaveType::SWave failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress WaveType::SWave failed");
    let decompressed = decompress(&compressed).expect("zstd decompress WaveType::SWave failed");
    let (decoded, _): (WaveType, usize) =
        decode_from_slice(&decompressed).expect("decode WaveType::SWave failed");
    assert_eq!(wave, decoded);
}

// ── 3. WaveType::LoveSurface compress/decompress roundtrip ────────────────────
#[test]
fn test_zstd_wave_type_love_surface_roundtrip() {
    let wave = WaveType::LoveSurface;
    let encoded = encode_to_vec(&wave).expect("encode WaveType::LoveSurface failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress WaveType::LoveSurface failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress WaveType::LoveSurface failed");
    let (decoded, _): (WaveType, usize) =
        decode_from_slice(&decompressed).expect("decode WaveType::LoveSurface failed");
    assert_eq!(wave, decoded);
}

// ── 4. WaveType::RayleighSurface compress/decompress roundtrip ────────────────
#[test]
fn test_zstd_wave_type_rayleigh_surface_roundtrip() {
    let wave = WaveType::RayleighSurface;
    let encoded = encode_to_vec(&wave).expect("encode WaveType::RayleighSurface failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress WaveType::RayleighSurface failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress WaveType::RayleighSurface failed");
    let (decoded, _): (WaveType, usize) =
        decode_from_slice(&decompressed).expect("decode WaveType::RayleighSurface failed");
    assert_eq!(wave, decoded);
}

// ── 5. MagnitudeScale::Richter compress/decompress roundtrip ──────────────────
#[test]
fn test_zstd_magnitude_scale_richter_roundtrip() {
    let scale = MagnitudeScale::Richter;
    let encoded = encode_to_vec(&scale).expect("encode MagnitudeScale::Richter failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress MagnitudeScale::Richter failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress MagnitudeScale::Richter failed");
    let (decoded, _): (MagnitudeScale, usize) =
        decode_from_slice(&decompressed).expect("decode MagnitudeScale::Richter failed");
    assert_eq!(scale, decoded);
}

// ── 6. MagnitudeScale::MomentMagnitude compress/decompress roundtrip ──────────
#[test]
fn test_zstd_magnitude_scale_moment_magnitude_roundtrip() {
    let scale = MagnitudeScale::MomentMagnitude;
    let encoded = encode_to_vec(&scale).expect("encode MagnitudeScale::MomentMagnitude failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress MagnitudeScale::MomentMagnitude failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress MagnitudeScale::MomentMagnitude failed");
    let (decoded, _): (MagnitudeScale, usize) =
        decode_from_slice(&decompressed).expect("decode MagnitudeScale::MomentMagnitude failed");
    assert_eq!(scale, decoded);
}

// ── 7. MagnitudeScale::BodyWave compress/decompress roundtrip ─────────────────
#[test]
fn test_zstd_magnitude_scale_body_wave_roundtrip() {
    let scale = MagnitudeScale::BodyWave;
    let encoded = encode_to_vec(&scale).expect("encode MagnitudeScale::BodyWave failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress MagnitudeScale::BodyWave failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress MagnitudeScale::BodyWave failed");
    let (decoded, _): (MagnitudeScale, usize) =
        decode_from_slice(&decompressed).expect("decode MagnitudeScale::BodyWave failed");
    assert_eq!(scale, decoded);
}

// ── 8. MagnitudeScale::SurfaceWave compress/decompress roundtrip ──────────────
#[test]
fn test_zstd_magnitude_scale_surface_wave_roundtrip() {
    let scale = MagnitudeScale::SurfaceWave;
    let encoded = encode_to_vec(&scale).expect("encode MagnitudeScale::SurfaceWave failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress MagnitudeScale::SurfaceWave failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress MagnitudeScale::SurfaceWave failed");
    let (decoded, _): (MagnitudeScale, usize) =
        decode_from_slice(&decompressed).expect("decode MagnitudeScale::SurfaceWave failed");
    assert_eq!(scale, decoded);
}

// ── 9. SeismicSample compress/decompress roundtrip ────────────────────────────
#[test]
fn test_zstd_seismic_sample_roundtrip() {
    let sample = SeismicSample {
        station_id: 1042,
        wave_type: WaveType::PWave,
        amplitude_nm: 3_500,
        period_ms: 120,
        timestamp_us: 1_700_000_000_000_000,
    };
    let encoded = encode_to_vec(&sample).expect("encode SeismicSample failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress SeismicSample failed");
    let decompressed = decompress(&compressed).expect("zstd decompress SeismicSample failed");
    let (decoded, _): (SeismicSample, usize) =
        decode_from_slice(&decompressed).expect("decode SeismicSample failed");
    assert_eq!(sample, decoded);
}

// ── 10. EarthquakeEvent compress/decompress roundtrip ─────────────────────────
#[test]
fn test_zstd_earthquake_event_roundtrip() {
    let event = EarthquakeEvent {
        event_id: 20240101_000001,
        magnitude: 65,
        scale: MagnitudeScale::MomentMagnitude,
        depth_km: 35,
        lat_micro: 35_689_000,
        lon_micro: 139_692_000,
        occurred_at: 1_704_067_200_000_000,
    };
    let encoded = encode_to_vec(&event).expect("encode EarthquakeEvent failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress EarthquakeEvent failed");
    let decompressed = decompress(&compressed).expect("zstd decompress EarthquakeEvent failed");
    let (decoded, _): (EarthquakeEvent, usize) =
        decode_from_slice(&decompressed).expect("decode EarthquakeEvent failed");
    assert_eq!(event, decoded);
}

// ── 11. SeismicRecord with None event compress/decompress roundtrip ───────────
#[test]
fn test_zstd_seismic_record_no_event_roundtrip() {
    let record = SeismicRecord {
        record_id: 9_001,
        station_id: 7,
        samples: vec![
            SeismicSample {
                station_id: 7,
                wave_type: WaveType::SWave,
                amplitude_nm: -250,
                period_ms: 300,
                timestamp_us: 1_700_000_001_000_000,
            },
            SeismicSample {
                station_id: 7,
                wave_type: WaveType::SWave,
                amplitude_nm: -180,
                period_ms: 310,
                timestamp_us: 1_700_000_002_000_000,
            },
        ],
        event: None,
    };
    let encoded = encode_to_vec(&record).expect("encode SeismicRecord(no event) failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress SeismicRecord(no event) failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress SeismicRecord(no event) failed");
    let (decoded, _): (SeismicRecord, usize) =
        decode_from_slice(&decompressed).expect("decode SeismicRecord(no event) failed");
    assert_eq!(record, decoded);
}

// ── 12. SeismicRecord with Some event compress/decompress roundtrip ───────────
#[test]
fn test_zstd_seismic_record_with_event_roundtrip() {
    let record = SeismicRecord {
        record_id: 9_002,
        station_id: 12,
        samples: vec![SeismicSample {
            station_id: 12,
            wave_type: WaveType::LoveSurface,
            amplitude_nm: 12_400,
            period_ms: 800,
            timestamp_us: 1_700_500_000_000_000,
        }],
        event: Some(EarthquakeEvent {
            event_id: 20240615_080000,
            magnitude: 72,
            scale: MagnitudeScale::Richter,
            depth_km: 10,
            lat_micro: -33_865_000,
            lon_micro: 151_209_000,
            occurred_at: 1_718_438_400_000_000,
        }),
    };
    let encoded = encode_to_vec(&record).expect("encode SeismicRecord(with event) failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress SeismicRecord(with event) failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress SeismicRecord(with event) failed");
    let (decoded, _): (SeismicRecord, usize) =
        decode_from_slice(&decompressed).expect("decode SeismicRecord(with event) failed");
    assert_eq!(record, decoded);
}

// ── 13. Large record (1000 samples) — Zstd must compress smaller ──────────────
#[test]
fn test_zstd_large_record_compression_ratio() {
    let samples: Vec<SeismicSample> = (0u32..1_000)
        .map(|i| SeismicSample {
            station_id: i % 8,
            wave_type: if i % 2 == 0 {
                WaveType::PWave
            } else {
                WaveType::SWave
            },
            amplitude_nm: (i as i32) * 10,
            period_ms: 100 + (i % 50),
            timestamp_us: 1_700_000_000_000_000 + (i as u64) * 10_000,
        })
        .collect();
    let record = SeismicRecord {
        record_id: 99_999,
        station_id: 1,
        samples,
        event: None,
    };
    let encoded = encode_to_vec(&record).expect("encode large SeismicRecord failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress large SeismicRecord failed");
    assert!(
        compressed.len() <= encoded.len(),
        "Zstd compressed ({} bytes) should be <= encoded ({} bytes) for large repetitive record",
        compressed.len(),
        encoded.len()
    );
}

// ── 14. Repetitive samples (1000 identical) compress smaller ──────────────────
#[test]
fn test_zstd_repetitive_samples_compress_smaller() {
    let identical_sample = SeismicSample {
        station_id: 3,
        wave_type: WaveType::PWave,
        amplitude_nm: 0,
        period_ms: 200,
        timestamp_us: 1_700_000_000_000_000,
    };
    let samples: Vec<SeismicSample> = (0..1_000)
        .map(|_| SeismicSample {
            station_id: identical_sample.station_id,
            wave_type: WaveType::PWave,
            amplitude_nm: identical_sample.amplitude_nm,
            period_ms: identical_sample.period_ms,
            timestamp_us: identical_sample.timestamp_us,
        })
        .collect();
    let encoded = encode_to_vec(&samples).expect("encode repetitive samples failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress repetitive samples failed");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be < encoded ({} bytes) for 1000 identical samples",
        compressed.len(),
        encoded.len()
    );
}

// ── 15. Empty sample list compress/decompress roundtrip ───────────────────────
#[test]
fn test_zstd_empty_sample_list_roundtrip() {
    let record = SeismicRecord {
        record_id: 1,
        station_id: 42,
        samples: vec![],
        event: None,
    };
    let encoded = encode_to_vec(&record).expect("encode empty sample SeismicRecord failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress empty SeismicRecord failed");
    let decompressed = decompress(&compressed).expect("zstd decompress empty SeismicRecord failed");
    let (decoded, _): (SeismicRecord, usize) =
        decode_from_slice(&decompressed).expect("decode empty SeismicRecord failed");
    assert_eq!(record, decoded);
    assert!(decoded.samples.is_empty());
}

// ── 16. Vec<SeismicSample> compress/decompress roundtrip ─────────────────────
#[test]
fn test_zstd_vec_seismic_sample_roundtrip() {
    let samples = vec![
        SeismicSample {
            station_id: 1,
            wave_type: WaveType::PWave,
            amplitude_nm: 500,
            period_ms: 150,
            timestamp_us: 1_000_000,
        },
        SeismicSample {
            station_id: 2,
            wave_type: WaveType::SWave,
            amplitude_nm: -300,
            period_ms: 250,
            timestamp_us: 1_100_000,
        },
        SeismicSample {
            station_id: 3,
            wave_type: WaveType::LoveSurface,
            amplitude_nm: 800,
            period_ms: 600,
            timestamp_us: 1_200_000,
        },
        SeismicSample {
            station_id: 4,
            wave_type: WaveType::RayleighSurface,
            amplitude_nm: 1_200,
            period_ms: 900,
            timestamp_us: 1_300_000,
        },
    ];
    let encoded = encode_to_vec(&samples).expect("encode Vec<SeismicSample> failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress Vec<SeismicSample> failed");
    let decompressed = decompress(&compressed).expect("zstd decompress Vec<SeismicSample> failed");
    let (decoded, _): (Vec<SeismicSample>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<SeismicSample> failed");
    assert_eq!(samples, decoded);
}

// ── 17. Vec<EarthquakeEvent> compress/decompress roundtrip ───────────────────
#[test]
fn test_zstd_vec_earthquake_event_roundtrip() {
    let events = vec![
        EarthquakeEvent {
            event_id: 1,
            magnitude: 40,
            scale: MagnitudeScale::Richter,
            depth_km: 5,
            lat_micro: 34_000_000,
            lon_micro: -118_000_000,
            occurred_at: 1_700_000_000,
        },
        EarthquakeEvent {
            event_id: 2,
            magnitude: 71,
            scale: MagnitudeScale::MomentMagnitude,
            depth_km: 700,
            lat_micro: -9_000_000,
            lon_micro: 160_000_000,
            occurred_at: 1_700_010_000,
        },
        EarthquakeEvent {
            event_id: 3,
            magnitude: 55,
            scale: MagnitudeScale::BodyWave,
            depth_km: 20,
            lat_micro: 38_000_000,
            lon_micro: 143_000_000,
            occurred_at: 1_700_020_000,
        },
    ];
    let encoded = encode_to_vec(&events).expect("encode Vec<EarthquakeEvent> failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress Vec<EarthquakeEvent> failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress Vec<EarthquakeEvent> failed");
    let (decoded, _): (Vec<EarthquakeEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<EarthquakeEvent> failed");
    assert_eq!(events, decoded);
}

// ── 18. Deep earthquake (700 km depth) compress/decompress roundtrip ─────────
#[test]
fn test_zstd_deep_earthquake_700km_roundtrip() {
    let event = EarthquakeEvent {
        event_id: 20230815_042300,
        magnitude: 68,
        scale: MagnitudeScale::MomentMagnitude,
        depth_km: 700,
        lat_micro: -9_750_000,
        lon_micro: 160_067_000,
        occurred_at: 1_692_070_980_000_000,
    };
    let encoded = encode_to_vec(&event).expect("encode deep EarthquakeEvent failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress deep EarthquakeEvent failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress deep EarthquakeEvent failed");
    let (decoded, _): (EarthquakeEvent, usize) =
        decode_from_slice(&decompressed).expect("decode deep EarthquakeEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(decoded.depth_km, 700);
}

// ── 19. Shallow earthquake (5 km depth) compress/decompress roundtrip ────────
#[test]
fn test_zstd_shallow_earthquake_5km_roundtrip() {
    let event = EarthquakeEvent {
        event_id: 20231025_163000,
        magnitude: 47,
        scale: MagnitudeScale::SurfaceWave,
        depth_km: 5,
        lat_micro: 37_968_000,
        lon_micro: 23_716_000,
        occurred_at: 1_698_250_200_000_000,
    };
    let encoded = encode_to_vec(&event).expect("encode shallow EarthquakeEvent failed");
    let compressed = compress(&encoded, Compression::Zstd)
        .expect("zstd compress shallow EarthquakeEvent failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress shallow EarthquakeEvent failed");
    let (decoded, _): (EarthquakeEvent, usize) =
        decode_from_slice(&decompressed).expect("decode shallow EarthquakeEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(decoded.depth_km, 5);
}

// ── 20. Mega-earthquake magnitude 9 compress/decompress roundtrip ─────────────
#[test]
fn test_zstd_mega_earthquake_magnitude_9_roundtrip() {
    let event = EarthquakeEvent {
        event_id: 20110311_055846,
        magnitude: 90,
        scale: MagnitudeScale::MomentMagnitude,
        depth_km: 29,
        lat_micro: 38_297_000,
        lon_micro: 142_373_000,
        occurred_at: 1_299_826_726_000_000,
    };
    let encoded = encode_to_vec(&event).expect("encode mega EarthquakeEvent failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress mega EarthquakeEvent failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress mega EarthquakeEvent failed");
    let (decoded, _): (EarthquakeEvent, usize) =
        decode_from_slice(&decompressed).expect("decode mega EarthquakeEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(decoded.magnitude, 90);
}

// ── 21. Micro-earthquake magnitude 1 compress/decompress roundtrip ────────────
#[test]
fn test_zstd_micro_earthquake_magnitude_1_roundtrip() {
    let event = EarthquakeEvent {
        event_id: 20240301_120000,
        magnitude: 10,
        scale: MagnitudeScale::Richter,
        depth_km: 3,
        lat_micro: 37_774_000,
        lon_micro: -122_419_000,
        occurred_at: 1_709_294_400_000_000,
    };
    let encoded = encode_to_vec(&event).expect("encode micro EarthquakeEvent failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress micro EarthquakeEvent failed");
    let decompressed =
        decompress(&compressed).expect("zstd decompress micro EarthquakeEvent failed");
    let (decoded, _): (EarthquakeEvent, usize) =
        decode_from_slice(&decompressed).expect("decode micro EarthquakeEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(decoded.magnitude, 10);
}

// ── 22. Aftershock sequence — time-series with P-wave vs S-wave timing,
//         negative amplitude rebound, global network, and decompressed bytes
//         matching original bytes ──────────────────────────────────────────────
#[test]
fn test_zstd_aftershock_sequence_global_network_roundtrip() {
    // Ten stations across global fault zones reporting an aftershock sequence.
    // Odd-indexed stations observe P-waves; even-indexed observe S-waves.
    // Every third sample records a negative amplitude (ground rebound).
    let station_ids: [u32; 10] = [101, 102, 103, 104, 105, 106, 107, 108, 109, 110];
    let base_ts: u64 = 1_700_000_000_000_000;

    let records: Vec<SeismicRecord> = station_ids
        .iter()
        .enumerate()
        .map(|(station_idx, &sid)| {
            let samples: Vec<SeismicSample> = (0u32..100)
                .map(|i| {
                    let wave_type = if station_idx % 2 == 1 {
                        WaveType::PWave
                    } else {
                        WaveType::SWave
                    };
                    let amplitude_nm: i32 = if i % 3 == 0 {
                        -((i as i32) * 50 + 100)
                    } else {
                        (i as i32) * 50 + 100
                    };
                    // P-waves arrive ~8 seconds before S-waves at 50 km distance
                    let p_s_offset_us: u64 = if matches!(wave_type, WaveType::SWave) {
                        8_000_000
                    } else {
                        0
                    };
                    SeismicSample {
                        station_id: sid,
                        wave_type,
                        amplitude_nm,
                        period_ms: 50 + (i % 200),
                        timestamp_us: base_ts + (i as u64) * 500_000 + p_s_offset_us,
                    }
                })
                .collect();

            // Main shock event reported by first station; aftershocks have no event
            let event = if station_idx == 0 {
                Some(EarthquakeEvent {
                    event_id: 20240920_143000,
                    magnitude: 62,
                    scale: MagnitudeScale::MomentMagnitude,
                    depth_km: 15,
                    lat_micro: 35_362_000,
                    lon_micro: 138_731_000,
                    occurred_at: base_ts,
                })
            } else {
                None
            };

            SeismicRecord {
                record_id: 10_000 + station_idx as u64,
                station_id: sid,
                samples,
                event,
            }
        })
        .collect();

    let encoded = encode_to_vec(&records).expect("encode aftershock sequence failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress aftershock sequence failed");
    let decompressed = decompress(&compressed).expect("zstd decompress aftershock sequence failed");

    // Verify the decompressed bytes are byte-for-byte identical to the original encoded bytes
    assert_eq!(
        encoded, decompressed,
        "decompressed bytes must match original encoded bytes"
    );

    let (decoded, _): (Vec<SeismicRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode aftershock sequence failed");

    assert_eq!(
        records.len(),
        decoded.len(),
        "record count must be preserved"
    );

    // Verify global network: all 10 stations present
    for (original, restored) in records.iter().zip(decoded.iter()) {
        assert_eq!(
            original.station_id, restored.station_id,
            "station_id mismatch"
        );
        assert_eq!(
            original.samples.len(),
            restored.samples.len(),
            "sample count mismatch"
        );
        assert_eq!(
            original.event.is_some(),
            restored.event.is_some(),
            "event presence mismatch"
        );
    }

    // Verify main shock event on station 0
    let main_station = &decoded[0];
    let event = main_station
        .event
        .as_ref()
        .expect("station 0 must have a main shock event");
    assert_eq!(event.magnitude, 62);
    assert_eq!(event.depth_km, 15);

    // Verify negative amplitudes (rebound) are preserved
    let rebound_samples: Vec<&SeismicSample> = decoded[0]
        .samples
        .iter()
        .filter(|s| s.amplitude_nm < 0)
        .collect();
    assert!(
        !rebound_samples.is_empty(),
        "negative amplitude (rebound) samples must be present"
    );

    // Verify P-wave vs S-wave timing offset is preserved on station 0 (S-wave) and station 1 (P-wave)
    let s_wave_ts = decoded[0].samples[0].timestamp_us;
    let p_wave_ts = decoded[1].samples[0].timestamp_us;
    assert!(
        s_wave_ts > p_wave_ts,
        "S-wave timestamps must be offset later than P-wave timestamps (got S={}, P={})",
        s_wave_ts,
        p_wave_ts
    );

    // Compression ratio check: 10 stations × 100 samples is large repetitive data
    assert!(
        compressed.len() <= encoded.len(),
        "Zstd compressed ({} bytes) should be <= encoded ({} bytes) for aftershock sequence",
        compressed.len(),
        encoded.len()
    );
}
