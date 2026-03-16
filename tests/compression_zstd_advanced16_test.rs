//! Advanced Zstd compression tests for OxiCode — Scientific Instruments domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world scientific instrument data: mass spectrometer readings,
//! NMR spectroscopy peaks, electron microscopy images, X-ray crystallography
//! diffraction patterns, atomic force microscopy scans, Raman spectroscopy
//! profiles, particle accelerator event logs, flow cytometry cell populations,
//! chromatography retention times, and fluorescence microscopy z-stacks.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IonizationMode {
    ElectroSpray,
    MatrixAssistedLaserDesorption,
    ElectronImpact,
    ChemicalIonization,
    AtmosphericPressurePhotoionization,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ScanPolarity {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MicroscopyMode {
    BrightField,
    DarkField,
    PhaseContrast,
    Fluorescence,
    ConfocalLaser,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CrystalSystem {
    Cubic,
    Tetragonal,
    Orthorhombic,
    Hexagonal,
    Trigonal,
    Monoclinic,
    Triclinic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChromatographyType {
    GasLiquid,
    HighPerformanceLiquid,
    IonExchange,
    SizeExclusion,
    AffinityCapture,
}

/// A single MS/MS scan event from a mass spectrometer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MassSpecScan {
    scan_id: u64,
    retention_time_ms: u32,
    ionization_mode: IonizationMode,
    polarity: ScanPolarity,
    precursor_mz: u64,      // stored as fixed-point × 10^6
    peaks: Vec<(u64, u32)>, // (m/z × 10^6, intensity)
}

/// A single NMR spectroscopy peak.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NmrPeak {
    chemical_shift_ppb: i32,   // parts-per-billion, signed (upfield/downfield)
    multiplicity: u8,          // 1=singlet, 2=doublet, … etc.
    coupling_constant_hz: u32, // × 10^3
    integral: u32,
    assignment: String,
}

/// An electron microscopy image represented as a flat pixel array.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmImage {
    image_id: u64,
    width: u32,
    height: u32,
    mode: MicroscopyMode,
    pixel_size_pm: u32, // picometres per pixel
    pixels: Vec<u16>,
}

/// A single Bragg reflection from an X-ray diffraction experiment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct XrayReflection {
    h: i16,
    k: i16,
    l: i16,
    intensity: u32,
    sigma_i: u32,
    crystal_system: CrystalSystem,
}

/// A line profile from an atomic force microscope scan.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AfmLineProfile {
    profile_id: u32,
    tip_radius_nm: u32, // × 10^3 — nanometres × 10^3
    scan_speed_nm_s: u32,
    height_samples: Vec<i32>, // height in picometres, signed
}

/// A Raman spectroscopy profile.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RamanProfile {
    sample_id: u64,
    excitation_wavelength_nm: u16,
    integration_time_ms: u32,
    wavenumbers: Vec<u32>,
    intensities: Vec<u32>,
}

/// A particle accelerator collision event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AcceleratorEvent {
    event_id: u64,
    run_number: u32,
    collision_energy_gev: u32, // GeV × 10
    particle_count: u16,
    vertex_x_um: i32, // micrometres, signed
    vertex_y_um: i32,
    vertex_z_um: i32,
    momentum_vectors: Vec<(i32, i32, i32)>, // (px, py, pz) in MeV/c
}

/// A flow cytometry cell population from a single sample.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowCytometryPopulation {
    sample_id: u64,
    cell_count: u32,
    fluorescence_channels: u8,
    // For each cell: one scatter and n fluorescence values.
    scatter_values: Vec<u32>,
    fluorescence_values: Vec<u32>,
}

/// A chromatography run with retention time / peak area pairs.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChromatographyRun {
    run_id: u64,
    method: ChromatographyType,
    column_length_mm: u16,
    temperature_mk: u32, // millikelvin
    // (retention_time_ms, peak_area, peak_height)
    peaks: Vec<(u32, u64, u32)>,
}

/// A fluorescence microscopy z-stack (series of 2-D frames along Z-axis).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FluorescenceZStack {
    stack_id: u64,
    frame_count: u32,
    z_step_nm: u32,
    channel: u8,
    // Flat-packed pixel data for all frames.
    pixels: Vec<u16>,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_mass_spec_scan(id: u64) -> MassSpecScan {
    MassSpecScan {
        scan_id: id,
        retention_time_ms: (id * 250) as u32,
        ionization_mode: IonizationMode::ElectroSpray,
        polarity: ScanPolarity::Positive,
        precursor_mz: 500_000_000 + id * 1_000,
        peaks: (0u64..20)
            .map(|p| (100_000_000 + p * 5_000_000, 1000 + (p * 37 % 9999) as u32))
            .collect(),
    }
}

fn make_nmr_peak(idx: u32) -> NmrPeak {
    NmrPeak {
        chemical_shift_ppb: (idx as i32 * 250) - 5000,
        multiplicity: (1 + idx % 5) as u8,
        coupling_constant_hz: 7_000 + idx * 500,
        integral: idx + 1,
        assignment: format!("H{idx}"),
    }
}

fn make_xray_reflection(h: i16, k: i16, l: i16) -> XrayReflection {
    XrayReflection {
        h,
        k,
        l,
        intensity: (h.unsigned_abs() as u32 + k.unsigned_abs() as u32 + l.unsigned_abs() as u32)
            * 10_000,
        sigma_i: 500,
        crystal_system: CrystalSystem::Orthorhombic,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Basic round-trip for a single MassSpecScan.
#[test]
fn test_zstd_mass_spec_scan_roundtrip() {
    let scan = make_mass_spec_scan(42);
    let encoded = encode_to_vec(&scan).expect("encode MassSpecScan failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (MassSpecScan, usize) =
        decode_from_slice(&decompressed).expect("decode MassSpecScan failed");
    assert_eq!(scan, decoded);
}

/// 2. Round-trip for a Vec of NMR peaks.
#[test]
fn test_zstd_nmr_peaks_roundtrip() {
    let peaks: Vec<NmrPeak> = (0u32..30).map(make_nmr_peak).collect();
    let encoded = encode_to_vec(&peaks).expect("encode Vec<NmrPeak> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<NmrPeak>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<NmrPeak> failed");
    assert_eq!(peaks, decoded);
}

/// 3. Round-trip for an electron microscopy image with a real pixel array.
#[test]
fn test_zstd_em_image_roundtrip() {
    let image = EmImage {
        image_id: 1001,
        width: 32,
        height: 32,
        mode: MicroscopyMode::BrightField,
        pixel_size_pm: 200,
        pixels: (0u16..1024).collect(),
    };
    let encoded = encode_to_vec(&image).expect("encode EmImage failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (EmImage, usize) =
        decode_from_slice(&decompressed).expect("decode EmImage failed");
    assert_eq!(image, decoded);
}

/// 4. Round-trip for an X-ray crystallography reflection list.
#[test]
fn test_zstd_xray_reflections_roundtrip() {
    let reflections: Vec<XrayReflection> = (-5i16..=5)
        .flat_map(|h| (-5i16..=5).map(move |k| make_xray_reflection(h, k, 1)))
        .collect();
    let encoded = encode_to_vec(&reflections).expect("encode Vec<XrayReflection> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<XrayReflection>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<XrayReflection> failed");
    assert_eq!(reflections, decoded);
}

/// 5. Round-trip for an AFM line profile with 500 height samples.
#[test]
fn test_zstd_afm_line_profile_roundtrip() {
    let profile = AfmLineProfile {
        profile_id: 7,
        tip_radius_nm: 10_000,
        scan_speed_nm_s: 500,
        height_samples: (0i32..500).map(|i| i * 3 - 750).collect(),
    };
    let encoded = encode_to_vec(&profile).expect("encode AfmLineProfile failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AfmLineProfile, usize) =
        decode_from_slice(&decompressed).expect("decode AfmLineProfile failed");
    assert_eq!(profile, decoded);
}

/// 6. Round-trip for a Raman spectroscopy profile.
#[test]
fn test_zstd_raman_profile_roundtrip() {
    let profile = RamanProfile {
        sample_id: 55,
        excitation_wavelength_nm: 532,
        integration_time_ms: 1000,
        wavenumbers: (200u32..=3200).step_by(4).collect(),
        intensities: (200u32..=3200).step_by(4).map(|w| w * 10 + 500).collect(),
    };
    let encoded = encode_to_vec(&profile).expect("encode RamanProfile failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (RamanProfile, usize) =
        decode_from_slice(&decompressed).expect("decode RamanProfile failed");
    assert_eq!(profile, decoded);
}

/// 7. Round-trip for a particle accelerator event with momentum vectors.
#[test]
fn test_zstd_accelerator_event_roundtrip() {
    let event = AcceleratorEvent {
        event_id: 123_456_789,
        run_number: 42,
        collision_energy_gev: 136_000, // 13.6 TeV
        particle_count: 400,
        vertex_x_um: -15,
        vertex_y_um: 8,
        vertex_z_um: 312,
        momentum_vectors: (0..400i32)
            .map(|i| (i * 100 - 20_000, i * 50, -(i * 75)))
            .collect(),
    };
    let encoded = encode_to_vec(&event).expect("encode AcceleratorEvent failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AcceleratorEvent, usize) =
        decode_from_slice(&decompressed).expect("decode AcceleratorEvent failed");
    assert_eq!(event, decoded);
}

/// 8. Round-trip for a flow cytometry population.
#[test]
fn test_zstd_flow_cytometry_population_roundtrip() {
    let population = FlowCytometryPopulation {
        sample_id: 99,
        cell_count: 10_000,
        fluorescence_channels: 4,
        scatter_values: (0u32..10_000).map(|i| 100 + i % 900).collect(),
        fluorescence_values: (0u32..40_000).map(|i| 50 + i % 450).collect(),
    };
    let encoded = encode_to_vec(&population).expect("encode FlowCytometryPopulation failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (FlowCytometryPopulation, usize) =
        decode_from_slice(&decompressed).expect("decode FlowCytometryPopulation failed");
    assert_eq!(population, decoded);
}

/// 9. Round-trip for a chromatography run.
#[test]
fn test_zstd_chromatography_run_roundtrip() {
    let run = ChromatographyRun {
        run_id: 7,
        method: ChromatographyType::HighPerformanceLiquid,
        column_length_mm: 250,
        temperature_mk: 298_150,
        peaks: (0u32..50)
            .map(|i| (i * 2_000 + 500, (i as u64 + 1) * 50_000, 5_000 + i * 100))
            .collect(),
    };
    let encoded = encode_to_vec(&run).expect("encode ChromatographyRun failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ChromatographyRun, usize) =
        decode_from_slice(&decompressed).expect("decode ChromatographyRun failed");
    assert_eq!(run, decoded);
}

/// 10. Round-trip for a fluorescence microscopy z-stack.
#[test]
fn test_zstd_fluorescence_zstack_roundtrip() {
    let stack = FluorescenceZStack {
        stack_id: 12,
        frame_count: 20,
        z_step_nm: 250,
        channel: 1,
        // 20 frames × 16×16 pixels = 5 120 u16 values
        pixels: (0u16..5_120).collect(),
    };
    let encoded = encode_to_vec(&stack).expect("encode FluorescenceZStack failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (FluorescenceZStack, usize) =
        decode_from_slice(&decompressed).expect("decode FluorescenceZStack failed");
    assert_eq!(stack, decoded);
}

/// 11. Compression ratio check: 1 200 nearly-identical MassSpecScans.
#[test]
fn test_zstd_large_mass_spec_scan_log_compression_ratio() {
    let scans: Vec<MassSpecScan> = (0u64..1_200)
        .map(|i| MassSpecScan {
            scan_id: i,
            retention_time_ms: 500,
            ionization_mode: IonizationMode::ElectroSpray,
            polarity: ScanPolarity::Positive,
            precursor_mz: 500_000_000,
            peaks: vec![(100_000_000, 10_000), (200_000_000, 5_000)],
        })
        .collect();

    let encoded = encode_to_vec(&scans).expect("encode large MassSpecScan log failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 1 200 repetitive scans",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(decompressed.len(), encoded.len());
    let (decoded, _): (Vec<MassSpecScan>, usize) =
        decode_from_slice(&decompressed).expect("decode large MassSpecScan log failed");
    assert_eq!(scans, decoded);
}

/// 12. Compression ratio check: 1 000 identical XrayReflections.
#[test]
fn test_zstd_large_xray_reflection_compression_ratio() {
    let reflections: Vec<XrayReflection> = (0u64..1_000)
        .map(|_| XrayReflection {
            h: 3,
            k: 1,
            l: 2,
            intensity: 100_000,
            sigma_i: 500,
            crystal_system: CrystalSystem::Cubic,
        })
        .collect();

    let encoded = encode_to_vec(&reflections).expect("encode large XrayReflection log failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 1 000 identical reflections",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(decompressed.len(), encoded.len());
    let (decoded, _): (Vec<XrayReflection>, usize) =
        decode_from_slice(&decompressed).expect("decode large XrayReflection log failed");
    assert_eq!(reflections, decoded);
}

/// 13. Compression ratio check: 1 100 identical chromatography peaks.
#[test]
fn test_zstd_large_chromatography_peaks_compression_ratio() {
    let runs: Vec<ChromatographyRun> = (0u64..1_100)
        .map(|i| ChromatographyRun {
            run_id: i,
            method: ChromatographyType::GasLiquid,
            column_length_mm: 30,
            temperature_mk: 303_150,
            peaks: vec![(12_000, 1_500_000, 75_000), (18_500, 2_200_000, 110_000)],
        })
        .collect();

    let encoded = encode_to_vec(&runs).expect("encode large chromatography runs failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 1 100 repetitive runs",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(decompressed.len(), encoded.len());
    let (decoded, _): (Vec<ChromatographyRun>, usize) =
        decode_from_slice(&decompressed).expect("decode large chromatography runs failed");
    assert_eq!(runs, decoded);
}

/// 14. Multiple compress/decompress cycles on the same Raman profile yield identical results.
#[test]
fn test_zstd_multiple_cycles_raman_profile() {
    let profile = RamanProfile {
        sample_id: 88,
        excitation_wavelength_nm: 785,
        integration_time_ms: 500,
        wavenumbers: (500u32..2500).step_by(2).collect(),
        intensities: (500u32..2500)
            .step_by(2)
            .map(|w| (w - 500) * 3 + 200)
            .collect(),
    };
    let encoded = encode_to_vec(&profile).expect("encode RamanProfile failed");

    for cycle in 1u32..=5 {
        let compressed = compress(&encoded, Compression::Zstd)
            .unwrap_or_else(|e| panic!("compress failed on cycle {cycle}: {e}"));
        let decompressed = decompress(&compressed)
            .unwrap_or_else(|e| panic!("decompress failed on cycle {cycle}: {e}"));
        let (decoded, _): (RamanProfile, usize) = decode_from_slice(&decompressed)
            .unwrap_or_else(|e| panic!("decode failed on cycle {cycle}: {e}"));
        assert_eq!(profile, decoded, "round-trip mismatch on cycle {cycle}");
    }
}

/// 15. All IonizationMode variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_ionization_modes_roundtrip() {
    let modes = vec![
        IonizationMode::ElectroSpray,
        IonizationMode::MatrixAssistedLaserDesorption,
        IonizationMode::ElectronImpact,
        IonizationMode::ChemicalIonization,
        IonizationMode::AtmosphericPressurePhotoionization,
    ];
    let encoded = encode_to_vec(&modes).expect("encode Vec<IonizationMode> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<IonizationMode>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<IonizationMode> failed");
    assert_eq!(modes, decoded);
}

/// 16. All CrystalSystem variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_crystal_systems_roundtrip() {
    let systems = vec![
        CrystalSystem::Cubic,
        CrystalSystem::Tetragonal,
        CrystalSystem::Orthorhombic,
        CrystalSystem::Hexagonal,
        CrystalSystem::Trigonal,
        CrystalSystem::Monoclinic,
        CrystalSystem::Triclinic,
    ];
    let encoded = encode_to_vec(&systems).expect("encode Vec<CrystalSystem> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CrystalSystem>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<CrystalSystem> failed");
    assert_eq!(systems, decoded);
}

/// 17. All MicroscopyMode variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_microscopy_modes_roundtrip() {
    let modes = vec![
        MicroscopyMode::BrightField,
        MicroscopyMode::DarkField,
        MicroscopyMode::PhaseContrast,
        MicroscopyMode::Fluorescence,
        MicroscopyMode::ConfocalLaser,
    ];
    let encoded = encode_to_vec(&modes).expect("encode Vec<MicroscopyMode> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<MicroscopyMode>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<MicroscopyMode> failed");
    assert_eq!(modes, decoded);
}

/// 18. All ChromatographyType variants survive a compress/decompress round-trip.
#[test]
fn test_zstd_all_chromatography_types_roundtrip() {
    let types = vec![
        ChromatographyType::GasLiquid,
        ChromatographyType::HighPerformanceLiquid,
        ChromatographyType::IonExchange,
        ChromatographyType::SizeExclusion,
        ChromatographyType::AffinityCapture,
    ];
    let encoded = encode_to_vec(&types).expect("encode Vec<ChromatographyType> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ChromatographyType>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<ChromatographyType> failed");
    assert_eq!(types, decoded);
}

/// 19. Compressed bytes differ from the original encoded bytes.
#[test]
fn test_zstd_compressed_differs_from_encoded_em_image() {
    let image = EmImage {
        image_id: 3,
        width: 64,
        height: 64,
        mode: MicroscopyMode::ConfocalLaser,
        pixel_size_pm: 50,
        pixels: (0u16..4096).collect(),
    };
    let encoded = encode_to_vec(&image).expect("encode EmImage failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert_ne!(
        encoded, compressed,
        "Compressed data must differ from the original encoded bytes"
    );
}

/// 20. Decompressed length equals original encoded length for an AcceleratorEvent.
#[test]
fn test_zstd_decompressed_length_equals_original_accelerator_event() {
    let event = AcceleratorEvent {
        event_id: 1,
        run_number: 1,
        collision_energy_gev: 7_000,
        particle_count: 200,
        vertex_x_um: 0,
        vertex_y_um: 0,
        vertex_z_um: 0,
        momentum_vectors: (0..200i32).map(|i| (i * 10, -i * 5, i * 3)).collect(),
    };
    let encoded = encode_to_vec(&event).expect("encode AcceleratorEvent failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(
        decompressed.len(),
        encoded.len(),
        "Decompressed length ({}) must equal original encoded length ({})",
        decompressed.len(),
        encoded.len()
    );
}

/// 21. Error returned on truncated zstd frame from an AFM scan.
#[test]
fn test_zstd_truncated_afm_data_returns_error() {
    let profile = AfmLineProfile {
        profile_id: 9,
        tip_radius_nm: 5_000,
        scan_speed_nm_s: 1_000,
        height_samples: (0i32..256).collect(),
    };
    let encoded = encode_to_vec(&profile).expect("encode AfmLineProfile failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    let truncated = &compressed[..8.min(compressed.len())];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress() must return Err for truncated zstd data"
    );
}

/// 22. Heterogeneous scientific snapshot — all domain types in one struct — round-trip.
#[test]
fn test_zstd_scientific_snapshot_all_types_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ScientificSnapshot {
        ms_scan: MassSpecScan,
        nmr_peak: NmrPeak,
        em_image: EmImage,
        xray: XrayReflection,
        afm: AfmLineProfile,
        raman: RamanProfile,
        accel: AcceleratorEvent,
        facs: FlowCytometryPopulation,
        chrom: ChromatographyRun,
        zstack: FluorescenceZStack,
    }

    let snapshot = ScientificSnapshot {
        ms_scan: make_mass_spec_scan(1),
        nmr_peak: make_nmr_peak(3),
        em_image: EmImage {
            image_id: 7,
            width: 16,
            height: 16,
            mode: MicroscopyMode::DarkField,
            pixel_size_pm: 100,
            pixels: (0u16..256).collect(),
        },
        xray: make_xray_reflection(1, 0, 0),
        afm: AfmLineProfile {
            profile_id: 1,
            tip_radius_nm: 8_000,
            scan_speed_nm_s: 200,
            height_samples: vec![-10, -5, 0, 5, 10, 5, 0, -5, -10],
        },
        raman: RamanProfile {
            sample_id: 1,
            excitation_wavelength_nm: 514,
            integration_time_ms: 200,
            wavenumbers: vec![500, 1000, 1500, 2000, 2500],
            intensities: vec![100, 500, 750, 300, 150],
        },
        accel: AcceleratorEvent {
            event_id: 1,
            run_number: 1,
            collision_energy_gev: 14_000,
            particle_count: 10,
            vertex_x_um: 1,
            vertex_y_um: -2,
            vertex_z_um: 3,
            momentum_vectors: vec![(100, -50, 200), (-100, 50, -200)],
        },
        facs: FlowCytometryPopulation {
            sample_id: 1,
            cell_count: 100,
            fluorescence_channels: 2,
            scatter_values: (0u32..100).collect(),
            fluorescence_values: (0u32..200).collect(),
        },
        chrom: ChromatographyRun {
            run_id: 1,
            method: ChromatographyType::SizeExclusion,
            column_length_mm: 300,
            temperature_mk: 298_000,
            peaks: vec![(5_000, 200_000, 10_000), (15_000, 500_000, 25_000)],
        },
        zstack: FluorescenceZStack {
            stack_id: 1,
            frame_count: 5,
            z_step_nm: 500,
            channel: 0,
            pixels: (0u16..320).collect(),
        },
    };

    let encoded = encode_to_vec(&snapshot).expect("encode ScientificSnapshot failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert_ne!(
        encoded, compressed,
        "Compressed snapshot must differ from encoded snapshot"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    assert_eq!(
        decompressed.len(),
        encoded.len(),
        "Decompressed length must equal original encoded length"
    );
    let (decoded, _): (ScientificSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode ScientificSnapshot failed");
    assert_eq!(snapshot, decoded);
}
