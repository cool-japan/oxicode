//! Advanced LZ4 compression tests for aerospace materials / structural testing domain.
//!
//! Covers stress-strain data, composite materials, fatigue cycles, fracture mechanics,
//! material properties, test specimens, tensile strength, thermal expansion, and
//! vibration analysis — all using the compression-lz4 feature.

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

// ---------------------------------------------------------------------------
// Domain structs and enums
// ---------------------------------------------------------------------------

/// Material category used in aerospace structural testing.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaterialClass {
    CarbonFiberReinforced,
    AluminumAlloy,
    TitaniumAlloy,
    CeramicMatrixComposite,
    NickelSuperalloy,
    GlassFiberReinforced,
}

/// Load type applied during a structural test.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LoadType {
    Tensile,
    Compressive,
    Shear,
    Bending,
    Torsional,
    Biaxial,
}

/// Failure mode observed at specimen fracture.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FailureMode {
    BrittleFracture,
    DuctileFracture,
    Delamination,
    FiberPullout,
    MatrixCracking,
    FatigueStriations,
    CreepRupture,
    BucklingInstability,
}

/// A single stress-strain data point.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StressStrainPoint {
    strain_mm_per_mm: f64,
    stress_mpa: f64,
}

/// A complete stress-strain curve from a tensile test.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StressStrainCurve {
    specimen_id: String,
    material_class: MaterialClass,
    load_type: LoadType,
    temperature_celsius: f64,
    points: Vec<StressStrainPoint>,
}

/// Fatigue test cycle record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FatigueCycleRecord {
    cycle_number: u64,
    max_stress_mpa: f64,
    min_stress_mpa: f64,
    crack_length_mm: f64,
    failure_mode: Option<FailureMode>,
}

/// Composite ply layup definition.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CompositeLayup {
    ply_count: u32,
    fiber_orientation_degrees: Vec<f32>,
    ply_thickness_mm: f32,
    material_class: MaterialClass,
    fiber_volume_fraction: f64,
}

/// Fracture toughness test result (K-Ic).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FractureToughnessResult {
    specimen_id: String,
    k_ic_mpa_sqrt_m: f64,
    crack_tip_opening_displacement_mm: f64,
    failure_mode: FailureMode,
    material_class: MaterialClass,
    thickness_mm: f64,
    width_mm: f64,
}

/// Thermal expansion measurement across a temperature range.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThermalExpansionData {
    material_class: MaterialClass,
    reference_temperature_celsius: f64,
    temperature_range: Vec<f64>,
    cte_per_kelvin: Vec<f64>,
}

/// Vibration analysis result from modal testing.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ModalAnalysisResult {
    specimen_id: String,
    natural_frequencies_hz: Vec<f64>,
    damping_ratios: Vec<f64>,
    mode_shapes: Vec<Vec<f32>>,
    material_class: MaterialClass,
}

/// Nested test batch: multiple specimens tested in one campaign.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TestCampaign {
    campaign_id: String,
    load_type: LoadType,
    fatigue_records: Vec<FatigueCycleRecord>,
    layup: CompositeLayup,
    fracture_result: FractureToughnessResult,
}

// ---------------------------------------------------------------------------
// Helper: build a representative stress-strain curve
// ---------------------------------------------------------------------------

fn make_stress_strain_curve(n_points: usize) -> StressStrainCurve {
    let points = (0..n_points)
        .map(|i| StressStrainPoint {
            strain_mm_per_mm: i as f64 * 0.0001,
            stress_mpa: (i as f64).sqrt() * 10.0,
        })
        .collect();
    StressStrainCurve {
        specimen_id: "SPEC-CFRP-001".to_string(),
        material_class: MaterialClass::CarbonFiberReinforced,
        load_type: LoadType::Tensile,
        temperature_celsius: 23.0,
        points,
    }
}

fn make_composite_layup() -> CompositeLayup {
    CompositeLayup {
        ply_count: 16,
        fiber_orientation_degrees: vec![
            0.0, 45.0, -45.0, 90.0, 0.0, 45.0, -45.0, 90.0, 90.0, -45.0, 45.0, 0.0, 90.0, -45.0,
            45.0, 0.0,
        ],
        ply_thickness_mm: 0.125,
        material_class: MaterialClass::CarbonFiberReinforced,
        fiber_volume_fraction: 0.60,
    }
}

fn make_fracture_result() -> FractureToughnessResult {
    FractureToughnessResult {
        specimen_id: "KIC-TI-003".to_string(),
        k_ic_mpa_sqrt_m: 55.4,
        crack_tip_opening_displacement_mm: 0.043,
        failure_mode: FailureMode::BrittleFracture,
        material_class: MaterialClass::TitaniumAlloy,
        thickness_mm: 25.0,
        width_mm: 50.0,
    }
}

// ---------------------------------------------------------------------------
// Test 1: roundtrip — MaterialClass::CarbonFiberReinforced
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_material_class_cfrp() {
    let value = MaterialClass::CarbonFiberReinforced;
    let encoded = encode_to_vec(&value).expect("encode MaterialClass failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress failed");
    let (decoded, _): (MaterialClass, usize) =
        decode_from_slice(&decompressed).expect("decode MaterialClass failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: roundtrip — MaterialClass::NickelSuperalloy
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_material_class_nickel_superalloy() {
    let value = MaterialClass::NickelSuperalloy;
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (MaterialClass, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: roundtrip — LoadType::Biaxial
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_load_type_biaxial() {
    let value = LoadType::Biaxial;
    let encoded = encode_to_vec(&value).expect("encode LoadType failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (LoadType, usize) =
        decode_from_slice(&decompressed).expect("decode LoadType failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: roundtrip — LoadType::Torsional
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_load_type_torsional() {
    let value = LoadType::Torsional;
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (LoadType, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: roundtrip — FailureMode::Delamination
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_failure_mode_delamination() {
    let value = FailureMode::Delamination;
    let encoded = encode_to_vec(&value).expect("encode FailureMode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (FailureMode, usize) =
        decode_from_slice(&decompressed).expect("decode FailureMode failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: roundtrip — FailureMode::CreepRupture
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_failure_mode_creep_rupture() {
    let value = FailureMode::CreepRupture;
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (FailureMode, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: roundtrip — StressStrainCurve (small, 20 points)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_stress_strain_curve_small() {
    let curve = make_stress_strain_curve(20);
    let encoded = encode_to_vec(&curve).expect("encode StressStrainCurve failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (StressStrainCurve, usize) =
        decode_from_slice(&decompressed).expect("decode StressStrainCurve failed");
    assert_eq!(curve, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: roundtrip — CompositeLayup
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_composite_layup() {
    let layup = make_composite_layup();
    let encoded = encode_to_vec(&layup).expect("encode CompositeLayup failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (CompositeLayup, usize) =
        decode_from_slice(&decompressed).expect("decode CompositeLayup failed");
    assert_eq!(layup, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: roundtrip — FractureToughnessResult
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_fracture_toughness_result() {
    let result = make_fracture_result();
    let encoded = encode_to_vec(&result).expect("encode FractureToughnessResult failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (FractureToughnessResult, usize) =
        decode_from_slice(&decompressed).expect("decode FractureToughnessResult failed");
    assert_eq!(result, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: roundtrip — ThermalExpansionData
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_thermal_expansion_data() {
    let data = ThermalExpansionData {
        material_class: MaterialClass::AluminumAlloy,
        reference_temperature_celsius: 20.0,
        temperature_range: vec![-55.0, 0.0, 25.0, 100.0, 150.0, 200.0],
        cte_per_kelvin: vec![21.5e-6, 22.0e-6, 23.1e-6, 24.3e-6, 25.0e-6, 25.8e-6],
    };
    let encoded = encode_to_vec(&data).expect("encode ThermalExpansionData failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (ThermalExpansionData, usize) =
        decode_from_slice(&decompressed).expect("decode ThermalExpansionData failed");
    assert_eq!(data, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: roundtrip — ModalAnalysisResult with nested Vec<Vec<f32>>
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_modal_analysis_result_nested() {
    let modal = ModalAnalysisResult {
        specimen_id: "MODAL-CFRP-005".to_string(),
        natural_frequencies_hz: vec![120.3, 345.7, 612.1, 987.4, 1543.0],
        damping_ratios: vec![0.012, 0.015, 0.018, 0.021, 0.025],
        mode_shapes: vec![
            vec![0.1, 0.4, 0.7, 1.0, 0.7, 0.4, 0.1],
            vec![-0.3, -0.1, 0.5, 1.0, 0.5, -0.1, -0.3],
            vec![0.6, -0.4, -0.8, 0.0, 0.8, 0.4, -0.6],
        ],
        material_class: MaterialClass::CarbonFiberReinforced,
    };
    let encoded = encode_to_vec(&modal).expect("encode ModalAnalysisResult failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (ModalAnalysisResult, usize) =
        decode_from_slice(&decompressed).expect("decode ModalAnalysisResult failed");
    assert_eq!(modal, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: roundtrip — FatigueCycleRecord with Some(FailureMode)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_fatigue_cycle_record_with_failure() {
    let record = FatigueCycleRecord {
        cycle_number: 1_500_000,
        max_stress_mpa: 350.0,
        min_stress_mpa: 35.0,
        crack_length_mm: 4.7,
        failure_mode: Some(FailureMode::FatigueStriations),
    };
    let encoded = encode_to_vec(&record).expect("encode FatigueCycleRecord failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (FatigueCycleRecord, usize) =
        decode_from_slice(&decompressed).expect("decode FatigueCycleRecord failed");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: roundtrip — FatigueCycleRecord with None failure mode
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_fatigue_cycle_record_no_failure() {
    let record = FatigueCycleRecord {
        cycle_number: 250_000,
        max_stress_mpa: 280.0,
        min_stress_mpa: 28.0,
        crack_length_mm: 1.2,
        failure_mode: None,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (FatigueCycleRecord, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: roundtrip — nested TestCampaign struct
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_test_campaign_nested_struct() {
    let campaign = TestCampaign {
        campaign_id: "CAMP-AERO-2025-Q4".to_string(),
        load_type: LoadType::Bending,
        fatigue_records: vec![
            FatigueCycleRecord {
                cycle_number: 100_000,
                max_stress_mpa: 400.0,
                min_stress_mpa: 40.0,
                crack_length_mm: 0.5,
                failure_mode: None,
            },
            FatigueCycleRecord {
                cycle_number: 2_000_000,
                max_stress_mpa: 400.0,
                min_stress_mpa: 40.0,
                crack_length_mm: 6.3,
                failure_mode: Some(FailureMode::BucklingInstability),
            },
        ],
        layup: make_composite_layup(),
        fracture_result: make_fracture_result(),
    };
    let encoded = encode_to_vec(&campaign).expect("encode TestCampaign failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (TestCampaign, usize) =
        decode_from_slice(&decompressed).expect("decode TestCampaign failed");
    assert_eq!(campaign, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Vec of StressStrainCurve roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_vec_of_stress_strain_curves() {
    let curves: Vec<StressStrainCurve> = (0..8)
        .map(|i| StressStrainCurve {
            specimen_id: format!("SPEC-{:03}", i),
            material_class: MaterialClass::GlassFiberReinforced,
            load_type: LoadType::Shear,
            temperature_celsius: -40.0 + (i as f64) * 20.0,
            points: (0..30)
                .map(|j| StressStrainPoint {
                    strain_mm_per_mm: j as f64 * 0.00015,
                    stress_mpa: (j as f64) * 5.5,
                })
                .collect(),
        })
        .collect();

    let encoded = encode_to_vec(&curves).expect("encode Vec<StressStrainCurve> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<StressStrainCurve>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<StressStrainCurve> failed");
    assert_eq!(curves, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Vec of FatigueCycleRecord roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_vec_of_fatigue_cycle_records() {
    let records: Vec<FatigueCycleRecord> = (0u64..50)
        .map(|i| FatigueCycleRecord {
            cycle_number: i * 10_000,
            max_stress_mpa: 300.0 + i as f64 * 0.5,
            min_stress_mpa: 30.0 + i as f64 * 0.05,
            crack_length_mm: i as f64 * 0.1,
            failure_mode: if i == 49 {
                Some(FailureMode::MatrixCracking)
            } else {
                None
            },
        })
        .collect();

    let encoded = encode_to_vec(&records).expect("encode Vec<FatigueCycleRecord> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<FatigueCycleRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(records, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: large repetitive stress-strain data compression ratio > 1.0
//          (>= 1000 identical StressStrainPoint elements)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_compression_ratio_large_repetitive_stress_strain_data() {
    let point = StressStrainPoint {
        strain_mm_per_mm: 0.002,
        stress_mpa: 450.0,
    };
    // 2000 identical stress-strain points — highly repetitive, must compress well
    let points: Vec<StressStrainPoint> = vec![point; 2_000];
    let encoded = encode_to_vec(&points).expect("encode large repetitive data failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compressed ({} bytes) must be smaller than encoded ({} bytes) for 2000 identical points",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 18: large repetitive fatigue cycles compression ratio
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_compression_ratio_large_repetitive_fatigue_cycles() {
    let record = FatigueCycleRecord {
        cycle_number: 500_000,
        max_stress_mpa: 385.0,
        min_stress_mpa: 38.5,
        crack_length_mm: 2.0,
        failure_mode: None,
    };
    // 1500 identical fatigue records
    let records: Vec<FatigueCycleRecord> = vec![record; 1_500];
    let encoded = encode_to_vec(&records).expect("encode large fatigue data failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compressed ({} bytes) must be smaller than encoded ({} bytes) for 1500 identical records",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 19: empty StressStrainCurve (zero points) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_empty_stress_strain_curve() {
    let curve = StressStrainCurve {
        specimen_id: "EMPTY-SPEC".to_string(),
        material_class: MaterialClass::CeramicMatrixComposite,
        load_type: LoadType::Compressive,
        temperature_celsius: 1200.0,
        points: vec![],
    };
    let encoded = encode_to_vec(&curve).expect("encode empty curve failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (StressStrainCurve, usize) =
        decode_from_slice(&decompressed).expect("decode empty curve failed");
    assert_eq!(curve, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: empty Vec<FatigueCycleRecord> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_empty_vec_fatigue_records() {
    let records: Vec<FatigueCycleRecord> = vec![];
    let encoded = encode_to_vec(&records).expect("encode empty vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<FatigueCycleRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode empty vec failed");
    assert_eq!(records, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: truncated compressed data must return an error
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_truncated_compressed_data_returns_error() {
    let curve = make_stress_strain_curve(100);
    let encoded = encode_to_vec(&curve).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    // Keep only the first quarter of the compressed buffer to simulate truncation
    let truncated_len = (compressed.len() / 4).max(1);
    let truncated = &compressed[..truncated_len];

    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress() must fail on truncated LZ4 payload, but got Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 22: corrupted (bit-flipped) compressed data must return an error
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_corrupted_compressed_data_returns_error() {
    let curve = make_stress_strain_curve(50);
    let encoded = encode_to_vec(&curve).expect("encode failed");
    let mut compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    // Flip bytes in the LZ4 payload region (after the oxicode header, ~8 bytes)
    // to corrupt the compressed content without destroying the magic header
    let header_len = compressed.len().min(8);
    for byte in compressed.iter_mut().skip(header_len) {
        *byte = byte.wrapping_add(0xAB);
    }

    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress() must fail on corrupted LZ4 payload, but got Ok"
    );
}
