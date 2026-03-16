#![cfg(feature = "compression-lz4")]
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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WellLog {
    well_id: u32,
    depth_m: f32,
    porosity: f32,
    permeability_md: f32,
    water_saturation: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReservoirCell {
    cell_id: u64,
    pressure_psi: f32,
    temperature_f: f32,
    oil_saturation: f32,
    gas_saturation: f32,
    water_saturation: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeismicTrace {
    trace_id: u32,
    samples: Vec<f32>,
    sample_rate_hz: u32,
    offset_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrillBit {
    bit_id: u32,
    diameter_in: f32,
    weight_on_bit_klb: f32,
    rpm: u16,
    torque_ftlb: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionRecord {
    well_id: u32,
    date: u32,
    oil_bbl: f32,
    gas_mcf: f32,
    water_bbl: f32,
}

// Test 1: WellLog basic encode -> compress -> decompress -> decode roundtrip
#[test]
fn test_well_log_roundtrip_lz4() {
    let log = WellLog {
        well_id: 1001,
        depth_m: 3200.5,
        porosity: 0.18,
        permeability_md: 45.3,
        water_saturation: 0.32,
    };
    let encoded = encode_to_vec(&log).expect("WellLog encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("WellLog compress failed");
    let decompressed = decompress(&compressed).expect("WellLog decompress failed");
    let (decoded, _): (WellLog, usize) =
        decode_from_slice(&decompressed).expect("WellLog decode failed");
    assert_eq!(log, decoded, "WellLog roundtrip mismatch");
}

// Test 2: ReservoirCell basic encode -> compress -> decompress -> decode roundtrip
#[test]
fn test_reservoir_cell_roundtrip_lz4() {
    let cell = ReservoirCell {
        cell_id: 88001,
        pressure_psi: 4500.0,
        temperature_f: 210.0,
        oil_saturation: 0.55,
        gas_saturation: 0.10,
        water_saturation: 0.35,
    };
    let encoded = encode_to_vec(&cell).expect("ReservoirCell encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("ReservoirCell compress failed");
    let decompressed = decompress(&compressed).expect("ReservoirCell decompress failed");
    let (decoded, _): (ReservoirCell, usize) =
        decode_from_slice(&decompressed).expect("ReservoirCell decode failed");
    assert_eq!(cell, decoded, "ReservoirCell roundtrip mismatch");
}

// Test 3: SeismicTrace with small sample vector roundtrip
#[test]
fn test_seismic_trace_small_roundtrip_lz4() {
    let trace = SeismicTrace {
        trace_id: 5001,
        samples: vec![0.1, -0.2, 0.35, 0.05, -0.15, 0.22],
        sample_rate_hz: 250,
        offset_m: 150.0,
    };
    let encoded = encode_to_vec(&trace).expect("SeismicTrace encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("SeismicTrace compress failed");
    let decompressed = decompress(&compressed).expect("SeismicTrace decompress failed");
    let (decoded, _): (SeismicTrace, usize) =
        decode_from_slice(&decompressed).expect("SeismicTrace decode failed");
    assert_eq!(trace, decoded, "SeismicTrace small roundtrip mismatch");
}

// Test 4: DrillBit encode -> compress -> decompress -> decode roundtrip
#[test]
fn test_drill_bit_roundtrip_lz4() {
    let bit = DrillBit {
        bit_id: 301,
        diameter_in: 12.25,
        weight_on_bit_klb: 35.0,
        rpm: 120,
        torque_ftlb: 18000.0,
    };
    let encoded = encode_to_vec(&bit).expect("DrillBit encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("DrillBit compress failed");
    let decompressed = decompress(&compressed).expect("DrillBit decompress failed");
    let (decoded, _): (DrillBit, usize) =
        decode_from_slice(&decompressed).expect("DrillBit decode failed");
    assert_eq!(bit, decoded, "DrillBit roundtrip mismatch");
}

// Test 5: ProductionRecord encode -> compress -> decompress -> decode roundtrip
#[test]
fn test_production_record_roundtrip_lz4() {
    let record = ProductionRecord {
        well_id: 2002,
        date: 20251201,
        oil_bbl: 1200.5,
        gas_mcf: 850.3,
        water_bbl: 300.0,
    };
    let encoded = encode_to_vec(&record).expect("ProductionRecord encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("ProductionRecord compress failed");
    let decompressed = decompress(&compressed).expect("ProductionRecord decompress failed");
    let (decoded, _): (ProductionRecord, usize) =
        decode_from_slice(&decompressed).expect("ProductionRecord decode failed");
    assert_eq!(record, decoded, "ProductionRecord roundtrip mismatch");
}

// Test 6: Large seismic trace with 1000+ repetitive samples — compression ratio check
#[test]
fn test_seismic_trace_large_repetitive_compression_ratio_lz4() {
    let samples: Vec<f32> = vec![0.0f32; 2000];
    let trace = SeismicTrace {
        trace_id: 9999,
        samples,
        sample_rate_hz: 1000,
        offset_m: 500.0,
    };
    let encoded = encode_to_vec(&trace).expect("Large SeismicTrace encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Large SeismicTrace compress failed");
    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Expected compression ratio > 1.0 for repetitive seismic trace data, got {:.3}",
        ratio
    );
    let decompressed = decompress(&compressed).expect("Large SeismicTrace decompress failed");
    let (decoded, _): (SeismicTrace, usize) =
        decode_from_slice(&decompressed).expect("Large SeismicTrace decode failed");
    assert_eq!(trace, decoded, "Large SeismicTrace roundtrip mismatch");
}

// Test 7: Large reservoir grid (1000+ cells) compression ratio check
#[test]
fn test_large_reservoir_grid_compression_ratio_lz4() {
    let cells: Vec<ReservoirCell> = (0..1200)
        .map(|i| ReservoirCell {
            cell_id: i as u64,
            pressure_psi: 3000.0,
            temperature_f: 180.0,
            oil_saturation: 0.60,
            gas_saturation: 0.05,
            water_saturation: 0.35,
        })
        .collect();
    let encoded = encode_to_vec(&cells).expect("Large reservoir grid encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Large reservoir grid compress failed");
    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Expected compression ratio > 1.0 for reservoir grid, got {:.3}",
        ratio
    );
    let decompressed = decompress(&compressed).expect("Large reservoir grid decompress failed");
    let (decoded, _): (Vec<ReservoirCell>, usize) =
        decode_from_slice(&decompressed).expect("Large reservoir grid decode failed");
    assert_eq!(cells, decoded, "Large reservoir grid roundtrip mismatch");
}

// Test 8: Vec of well logs compression and roundtrip
#[test]
fn test_vec_well_logs_compression_lz4() {
    let logs: Vec<WellLog> = (0..50)
        .map(|i| WellLog {
            well_id: 3000 + i as u32,
            depth_m: 1500.0 + i as f32 * 10.0,
            porosity: 0.15,
            permeability_md: 30.0,
            water_saturation: 0.40,
        })
        .collect();
    let encoded = encode_to_vec(&logs).expect("Vec<WellLog> encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("Vec<WellLog> compress failed");
    let decompressed = decompress(&compressed).expect("Vec<WellLog> decompress failed");
    let (decoded, _): (Vec<WellLog>, usize) =
        decode_from_slice(&decompressed).expect("Vec<WellLog> decode failed");
    assert_eq!(logs, decoded, "Vec<WellLog> roundtrip mismatch");
}

// Test 9: Compressed size is non-zero and decompressed size matches original encoded size
#[test]
fn test_compressed_decompressed_size_matches_original_lz4() {
    let cell = ReservoirCell {
        cell_id: 42,
        pressure_psi: 5000.0,
        temperature_f: 220.0,
        oil_saturation: 0.50,
        gas_saturation: 0.15,
        water_saturation: 0.35,
    };
    let encoded = encode_to_vec(&cell).expect("ReservoirCell encode failed for size check");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("ReservoirCell compress failed for size check");
    assert!(
        !compressed.is_empty(),
        "Compressed output must be non-empty"
    );
    let decompressed =
        decompress(&compressed).expect("ReservoirCell decompress failed for size check");
    assert_eq!(
        encoded.len(),
        decompressed.len(),
        "Decompressed size must equal original encoded size"
    );
}

// Test 10: Multiple compression/decompression cycles maintain data integrity
#[test]
fn test_multiple_compression_cycles_lz4() {
    let log = WellLog {
        well_id: 7777,
        depth_m: 2800.0,
        porosity: 0.22,
        permeability_md: 120.0,
        water_saturation: 0.28,
    };
    let encoded = encode_to_vec(&log).expect("WellLog encode failed in multi-cycle test");
    let mut data = encoded.clone();
    for cycle in 0..5 {
        let compressed = compress(&data, Compression::Lz4)
            .unwrap_or_else(|e| panic!("Compress failed at cycle {}: {:?}", cycle, e));
        let decompressed = decompress(&compressed)
            .unwrap_or_else(|e| panic!("Decompress failed at cycle {}: {:?}", cycle, e));
        assert_eq!(
            encoded.len(),
            decompressed.len(),
            "Size mismatch at cycle {}",
            cycle
        );
        data = decompressed;
    }
    let (decoded, _): (WellLog, usize) =
        decode_from_slice(&data).expect("WellLog decode failed after multi-cycle");
    assert_eq!(log, decoded, "WellLog multi-cycle roundtrip mismatch");
}

// Test 11: Large Vec of production records — compression ratio check
#[test]
fn test_large_production_records_compression_ratio_lz4() {
    let records: Vec<ProductionRecord> = (0..1500)
        .map(|i| ProductionRecord {
            well_id: 1000 + (i % 10) as u32,
            date: 20250101 + i as u32,
            oil_bbl: 500.0,
            gas_mcf: 200.0,
            water_bbl: 150.0,
        })
        .collect();
    let encoded = encode_to_vec(&records).expect("Large ProductionRecord encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Large ProductionRecord compress failed");
    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Expected compression ratio > 1.0 for production records, got {:.3}",
        ratio
    );
    let decompressed = decompress(&compressed).expect("Large ProductionRecord decompress failed");
    let (decoded, _): (Vec<ProductionRecord>, usize) =
        decode_from_slice(&decompressed).expect("Large ProductionRecord decode failed");
    assert_eq!(
        records, decoded,
        "Large ProductionRecord roundtrip mismatch"
    );
}

// Test 12: SeismicTrace with 1000+ sinusoidal-like samples (non-zero repetitive values)
#[test]
fn test_seismic_trace_constant_nonzero_samples_lz4() {
    let samples: Vec<f32> = vec![1.2345f32; 1500];
    let trace = SeismicTrace {
        trace_id: 8888,
        samples,
        sample_rate_hz: 500,
        offset_m: 300.0,
    };
    let encoded = encode_to_vec(&trace).expect("Constant-value SeismicTrace encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Constant-value SeismicTrace compress failed");
    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Expected compression ratio > 1.0 for constant seismic samples, got {:.3}",
        ratio
    );
    let decompressed =
        decompress(&compressed).expect("Constant-value SeismicTrace decompress failed");
    let (decoded, _): (SeismicTrace, usize) =
        decode_from_slice(&decompressed).expect("Constant-value SeismicTrace decode failed");
    assert_eq!(
        trace, decoded,
        "Constant-value SeismicTrace roundtrip mismatch"
    );
}

// Test 13: WellLog with extreme depth and saturation boundary values
#[test]
fn test_well_log_boundary_values_lz4() {
    let log = WellLog {
        well_id: u32::MAX,
        depth_m: f32::MAX,
        porosity: 0.0,
        permeability_md: 0.0,
        water_saturation: 1.0,
    };
    let encoded = encode_to_vec(&log).expect("Boundary WellLog encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Boundary WellLog compress failed");
    let decompressed = decompress(&compressed).expect("Boundary WellLog decompress failed");
    let (decoded, _): (WellLog, usize) =
        decode_from_slice(&decompressed).expect("Boundary WellLog decode failed");
    assert_eq!(log, decoded, "Boundary WellLog roundtrip mismatch");
}

// Test 14: DrillBit batch of 200 identical entries — verifies repetitive pattern compression
#[test]
fn test_drill_bit_batch_identical_compression_lz4() {
    let bits: Vec<DrillBit> = vec![
        DrillBit {
            bit_id: 50,
            diameter_in: 8.5,
            weight_on_bit_klb: 20.0,
            rpm: 80,
            torque_ftlb: 9000.0,
        };
        200
    ];
    let encoded = encode_to_vec(&bits).expect("DrillBit batch encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("DrillBit batch compress failed");
    let decompressed = decompress(&compressed).expect("DrillBit batch decompress failed");
    let (decoded, _): (Vec<DrillBit>, usize) =
        decode_from_slice(&decompressed).expect("DrillBit batch decode failed");
    assert_eq!(bits, decoded, "DrillBit batch roundtrip mismatch");
}

// Test 15: Compressed bytes are different from the original encoded bytes
#[test]
fn test_compressed_bytes_differ_from_original_lz4() {
    let record = ProductionRecord {
        well_id: 555,
        date: 20240601,
        oil_bbl: 750.0,
        gas_mcf: 300.0,
        water_bbl: 100.0,
    };
    let encoded = encode_to_vec(&record).expect("ProductionRecord encode failed for diff test");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("ProductionRecord compress failed for diff test");
    assert_ne!(
        encoded, compressed,
        "Compressed bytes should differ from original encoded bytes"
    );
}

// Test 16: ReservoirCell with zero saturations and minimum pressure
#[test]
fn test_reservoir_cell_zero_saturation_lz4() {
    let cell = ReservoirCell {
        cell_id: 0,
        pressure_psi: 14.7,
        temperature_f: 60.0,
        oil_saturation: 0.0,
        gas_saturation: 0.0,
        water_saturation: 1.0,
    };
    let encoded = encode_to_vec(&cell).expect("Zero-saturation ReservoirCell encode failed");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("Zero-saturation ReservoirCell compress failed");
    let decompressed =
        decompress(&compressed).expect("Zero-saturation ReservoirCell decompress failed");
    let (decoded, _): (ReservoirCell, usize) =
        decode_from_slice(&decompressed).expect("Zero-saturation ReservoirCell decode failed");
    assert_eq!(
        cell, decoded,
        "Zero-saturation ReservoirCell roundtrip mismatch"
    );
}

// Test 17: Large uniform well log dataset — compression ratio check
#[test]
fn test_large_uniform_well_log_compression_ratio_lz4() {
    let logs: Vec<WellLog> = vec![
        WellLog {
            well_id: 4000,
            depth_m: 2500.0,
            porosity: 0.20,
            permeability_md: 80.0,
            water_saturation: 0.30,
        };
        1000
    ];
    let encoded = encode_to_vec(&logs).expect("Uniform WellLog encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("Uniform WellLog compress failed");
    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Expected compression ratio > 1.0 for uniform well logs, got {:.3}",
        ratio
    );
    let decompressed = decompress(&compressed).expect("Uniform WellLog decompress failed");
    let (decoded, _): (Vec<WellLog>, usize) =
        decode_from_slice(&decompressed).expect("Uniform WellLog decode failed");
    assert_eq!(logs, decoded, "Uniform WellLog roundtrip mismatch");
}

// Test 18: SeismicTrace with empty samples vector
#[test]
fn test_seismic_trace_empty_samples_lz4() {
    let trace = SeismicTrace {
        trace_id: 1,
        samples: vec![],
        sample_rate_hz: 125,
        offset_m: 0.0,
    };
    let encoded = encode_to_vec(&trace).expect("Empty SeismicTrace encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Empty SeismicTrace compress failed");
    let decompressed = decompress(&compressed).expect("Empty SeismicTrace decompress failed");
    let (decoded, _): (SeismicTrace, usize) =
        decode_from_slice(&decompressed).expect("Empty SeismicTrace decode failed");
    assert_eq!(trace, decoded, "Empty SeismicTrace roundtrip mismatch");
}

// Test 19: Mixed reservoir grid with varying pressures — full roundtrip
#[test]
fn test_mixed_reservoir_grid_roundtrip_lz4() {
    let cells: Vec<ReservoirCell> = (0..100)
        .map(|i| ReservoirCell {
            cell_id: i as u64 * 100,
            pressure_psi: 1000.0 + i as f32 * 50.0,
            temperature_f: 150.0 + i as f32 * 0.5,
            oil_saturation: 0.4 + (i as f32 % 10.0) * 0.01,
            gas_saturation: 0.1,
            water_saturation: 0.5 - (i as f32 % 10.0) * 0.01,
        })
        .collect();
    let encoded = encode_to_vec(&cells).expect("Mixed reservoir grid encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Mixed reservoir grid compress failed");
    let decompressed = decompress(&compressed).expect("Mixed reservoir grid decompress failed");
    let (decoded, _): (Vec<ReservoirCell>, usize) =
        decode_from_slice(&decompressed).expect("Mixed reservoir grid decode failed");
    assert_eq!(cells, decoded, "Mixed reservoir grid roundtrip mismatch");
}

// Test 20: Double compression/decompression cycle for WellLog
#[test]
fn test_well_log_double_cycle_lz4() {
    let log = WellLog {
        well_id: 6000,
        depth_m: 4100.0,
        porosity: 0.12,
        permeability_md: 5.0,
        water_saturation: 0.55,
    };
    let encoded = encode_to_vec(&log).expect("WellLog encode failed in double-cycle");
    // First cycle
    let compressed1 = compress(&encoded, Compression::Lz4).expect("WellLog first compress failed");
    let decompressed1 = decompress(&compressed1).expect("WellLog first decompress failed");
    assert_eq!(
        encoded, decompressed1,
        "WellLog first cycle decompressed mismatch"
    );
    // Second cycle
    let compressed2 =
        compress(&decompressed1, Compression::Lz4).expect("WellLog second compress failed");
    let decompressed2 = decompress(&compressed2).expect("WellLog second decompress failed");
    assert_eq!(
        encoded, decompressed2,
        "WellLog second cycle decompressed mismatch"
    );
    let (decoded, _): (WellLog, usize) =
        decode_from_slice(&decompressed2).expect("WellLog double-cycle decode failed");
    assert_eq!(
        log, decoded,
        "WellLog double-cycle final roundtrip mismatch"
    );
}

// Test 21: DrillBit high-frequency operation parameters roundtrip
#[test]
fn test_drill_bit_high_frequency_params_lz4() {
    let bit = DrillBit {
        bit_id: 9001,
        diameter_in: 6.0,
        weight_on_bit_klb: 50.0,
        rpm: 300,
        torque_ftlb: 25000.0,
    };
    let encoded = encode_to_vec(&bit).expect("High-freq DrillBit encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("High-freq DrillBit compress failed");
    let decompressed = decompress(&compressed).expect("High-freq DrillBit decompress failed");
    assert_eq!(
        encoded.len(),
        decompressed.len(),
        "High-freq DrillBit: decompressed size must equal original encoded size"
    );
    let (decoded, _): (DrillBit, usize) =
        decode_from_slice(&decompressed).expect("High-freq DrillBit decode failed");
    assert_eq!(bit, decoded, "High-freq DrillBit roundtrip mismatch");
}

// Test 22: Large seismic dataset with 1000+ alternating pattern samples — compression ratio and integrity
#[test]
fn test_seismic_alternating_pattern_large_lz4() {
    let samples: Vec<f32> = (0..1200)
        .map(|i| if i % 2 == 0 { 1.0f32 } else { -1.0f32 })
        .collect();
    let trace = SeismicTrace {
        trace_id: 7070,
        samples,
        sample_rate_hz: 2000,
        offset_m: 1000.0,
    };
    let encoded = encode_to_vec(&trace).expect("Alternating SeismicTrace encode failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("Alternating SeismicTrace compress failed");
    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Expected compression ratio > 1.0 for alternating seismic pattern, got {:.3}",
        ratio
    );
    let decompressed = decompress(&compressed).expect("Alternating SeismicTrace decompress failed");
    assert_eq!(
        encoded.len(),
        decompressed.len(),
        "Alternating SeismicTrace: decompressed size must equal original"
    );
    let (decoded, _): (SeismicTrace, usize) =
        decode_from_slice(&decompressed).expect("Alternating SeismicTrace decode failed");
    assert_eq!(
        trace, decoded,
        "Alternating SeismicTrace roundtrip mismatch"
    );
}
