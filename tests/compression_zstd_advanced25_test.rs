//! Advanced Zstd compression tests for OxiCode — Dental Implant & Prosthetics
//! Manufacturing domain.
//!
//! Covers encode -> compress -> decompress -> decode round-trips for types that
//! model real-world dental implant manufacturing data: implant dimensions, abutment
//! specifications, crown materials and shade matching, bone density classifications,
//! 3D scan point clouds, milling toolpath parameters, titanium alloy grades,
//! osseointegration tracking, bite force measurements, surgical guide coordinates,
//! and prosthetic occlusion maps.

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
enum TitaniumGrade {
    /// Commercially pure Ti Grade 1 (softest, highest corrosion resistance).
    CpGrade1,
    /// Commercially pure Ti Grade 2 (standard dental implants).
    CpGrade2,
    /// Commercially pure Ti Grade 4 (highest strength among CP grades).
    CpGrade4,
    /// Ti-6Al-4V ELI (Grade 23, extra-low interstitials).
    Ti6Al4VEli,
    /// Ti-15Zr-4Nb-2Ta-0.2Pd (Roxolid-class alloy).
    TiZrAlloy,
    /// Ti-13Nb-13Zr (beta-type, low elastic modulus).
    Ti13Nb13Zr,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThreadProfile {
    VShape,
    Buttress,
    ReverseButttress,
    SquareThread,
    MicroThread,
    DoubleHelix,
    TripleHelix,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BoneDensityClass {
    /// Type I: dense cortical bone (mandible anterior).
    TypeI,
    /// Type II: thick cortical with dense trabecular core.
    TypeII,
    /// Type III: thin cortical with dense trabecular core.
    TypeIII,
    /// Type IV: thin cortical with sparse trabecular bone.
    TypeIV,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CrownMaterial {
    Zirconia,
    LithiumDisilicate,
    PorcelainFusedToMetal,
    FullCastGold,
    FeldspathicPorcelain,
    ResinNanoCeramic,
    PeekPolymer,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ShadeSystem {
    VitaClassicA1,
    VitaClassicA2,
    VitaClassicA3,
    VitaClassicA35,
    VitaClassicA4,
    VitaClassicB1,
    VitaClassicB2,
    VitaClassicB3,
    VitaClassicC1,
    VitaClassicC2,
    VitaClassicD2,
    VitaClassicD3,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AbutmentType {
    Straight,
    Angled15,
    Angled25,
    Custom,
    BallAttachment,
    LocatorAttachment,
    MultiUnit,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SurfaceTreatment {
    Machined,
    SandBlasted,
    AcidEtched,
    SlaBlasted,
    AnodizedTiUnite,
    HydroxyapatiteCoated,
    LaserMicroTextured,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OsseointegrationPhase {
    Hemostasis,
    Inflammatory,
    Proliferative,
    Remodeling,
    Mature,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ToothPosition {
    UpperRightThirdMolar,
    UpperRightSecondMolar,
    UpperRightFirstMolar,
    UpperRightSecondPremolar,
    UpperRightFirstPremolar,
    UpperRightCanine,
    UpperRightLateralIncisor,
    UpperRightCentralIncisor,
    UpperLeftCentralIncisor,
    UpperLeftLateralIncisor,
    UpperLeftCanine,
    LowerRightFirstMolar,
    LowerRightSecondPremolar,
    LowerLeftFirstMolar,
    LowerLeftCentralIncisor,
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

/// Core implant fixture specification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantFixture {
    part_number: String,
    diameter_um: u32,
    length_um: u32,
    thread_pitch_um: u32,
    thread_profile: ThreadProfile,
    alloy: TitaniumGrade,
    surface: SurfaceTreatment,
    platform_diameter_um: u32,
    taper_angle_centideg: u16,
    internal_hex: bool,
}

/// Abutment that connects implant fixture to the crown.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AbutmentSpec {
    abutment_id: u64,
    abutment_type: AbutmentType,
    gingival_height_um: u32,
    collar_height_um: u32,
    angulation_centideg: u16,
    material: TitaniumGrade,
    torque_ncm: u16,
    compatible_fixture: String,
}

/// Crown prosthetic with shade and material info.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrownSpec {
    crown_id: u64,
    tooth_position: ToothPosition,
    material: CrownMaterial,
    shade: ShadeSystem,
    translucency_pct: u8,
    wall_thickness_um: u32,
    occlusal_thickness_um: u32,
    margin_fit_um: u16,
    cemented: bool,
}

/// 3D intraoral scan point cloud data.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IntraoralScanCloud {
    scan_id: u64,
    patient_hash: u64,
    /// Points stored as (x_um, y_um, z_um) integer microns.
    points: Vec<(i32, i32, i32)>,
    normal_vectors: Vec<(i16, i16, i16)>,
    resolution_um: u16,
    scanner_model: String,
}

/// Pre-operative bone density measurement at implant site.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoneDensityMeasurement {
    site_id: u64,
    tooth_position: ToothPosition,
    density_class: BoneDensityClass,
    hounsfield_units: i16,
    cortical_thickness_um: u32,
    trabecular_density_hu: i16,
    available_bone_height_um: u32,
    available_bone_width_um: u32,
}

/// CAD/CAM milling toolpath for prosthetic fabrication.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MillingToolpath {
    job_id: u64,
    tool_diameter_um: u32,
    spindle_rpm: u32,
    feed_rate_um_per_min: u32,
    stepover_um: u32,
    depth_per_pass_um: u32,
    /// Toolpath waypoints: (x_um, y_um, z_um).
    waypoints: Vec<(i32, i32, i32)>,
    coolant_flow_ml_per_min: u16,
    estimated_time_sec: u32,
}

/// Osseointegration tracking record over time.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OsseointegrationRecord {
    implant_id: u64,
    phase: OsseointegrationPhase,
    days_post_surgery: u32,
    isq_value: u8,
    periapical_radiograph_hash: u64,
    marginal_bone_loss_um: u32,
    mobility_detected: bool,
    notes: String,
}

/// Bite force measurement at a specific occlusal point.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BiteForceReading {
    reading_id: u64,
    tooth_position: ToothPosition,
    force_millinewtons: u32,
    contact_area_um2: u64,
    timestamp_epoch_ms: u64,
    lateral_deviation_um: i32,
    is_centric: bool,
}

/// Surgical guide coordinate for guided implant placement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SurgicalGuideCoordinate {
    guide_id: u64,
    entry_point: (i32, i32, i32),
    apical_point: (i32, i32, i32),
    insertion_axis: (i16, i16, i16),
    depth_stop_um: u32,
    drill_sequence: Vec<u32>,
    sleeve_height_um: u16,
    tissue_thickness_um: u16,
}

/// Occlusion contact map for prosthetic verification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OcclusionMap {
    map_id: u64,
    patient_hash: u64,
    /// Contact points: (x_um, y_um, pressure_kpa).
    contacts: Vec<(i32, i32, u32)>,
    max_intercuspation_force_mn: u32,
    lateral_excursion_contacts: Vec<(i32, i32, u32)>,
    protrusive_contacts: Vec<(i32, i32, u32)>,
}

/// Multi-unit bridge spanning several implant positions.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantBridge {
    bridge_id: u64,
    fixture_ids: Vec<u64>,
    pontic_positions: Vec<ToothPosition>,
    framework_material: CrownMaterial,
    veneer_material: CrownMaterial,
    connector_cross_section_um2: Vec<u64>,
    passive_fit_gap_um: u16,
    screw_retained: bool,
}

/// Healing abutment sizing record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HealingAbutment {
    abutment_id: u64,
    diameter_um: u32,
    height_um: u32,
    emergence_profile: u8,
    alloy: TitaniumGrade,
    placement_torque_ncm: u16,
}

/// Material lot traceability for regulatory compliance.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaterialLotRecord {
    lot_number: String,
    alloy: TitaniumGrade,
    tensile_strength_mpa: u32,
    yield_strength_mpa: u32,
    elongation_pct_x10: u16,
    oxygen_ppm: u16,
    nitrogen_ppm: u16,
    hydrogen_ppm: u16,
    iron_ppm: u16,
    certification_hash: u64,
    expiry_epoch_sec: u64,
}

/// Digital impression comparison between pre-op and post-op scans.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScanComparison {
    comparison_id: u64,
    pre_scan_id: u64,
    post_scan_id: u64,
    rms_deviation_um: u32,
    max_deviation_um: u32,
    deviation_histogram: Vec<u32>,
    regions_of_interest: Vec<(i32, i32, i32, u32)>,
}

/// Complete implant case combining fixture, abutment, and crown.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantCase {
    case_id: u64,
    fixture: ImplantFixture,
    abutment: AbutmentSpec,
    crown: CrownSpec,
    bone_measurement: BoneDensityMeasurement,
    surgical_guide: SurgicalGuideCoordinate,
    osseo_records: Vec<OsseointegrationRecord>,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_implant_fixture(id: u32) -> ImplantFixture {
    ImplantFixture {
        part_number: format!("IMP-{id:06}"),
        diameter_um: 3_500 + (id % 5) * 500,
        length_um: 8_000 + (id % 6) * 2_000,
        thread_pitch_um: 600 + (id % 3) * 200,
        thread_profile: match id % 4 {
            0 => ThreadProfile::VShape,
            1 => ThreadProfile::Buttress,
            2 => ThreadProfile::MicroThread,
            _ => ThreadProfile::DoubleHelix,
        },
        alloy: match id % 3 {
            0 => TitaniumGrade::CpGrade4,
            1 => TitaniumGrade::Ti6Al4VEli,
            _ => TitaniumGrade::TiZrAlloy,
        },
        surface: match id % 4 {
            0 => SurfaceTreatment::SlaBlasted,
            1 => SurfaceTreatment::AnodizedTiUnite,
            2 => SurfaceTreatment::LaserMicroTextured,
            _ => SurfaceTreatment::AcidEtched,
        },
        platform_diameter_um: 3_400 + (id % 4) * 300,
        taper_angle_centideg: 200 + (id % 5) as u16 * 50,
        internal_hex: id % 2 == 0,
    }
}

fn make_abutment(id: u64) -> AbutmentSpec {
    AbutmentSpec {
        abutment_id: id,
        abutment_type: match id % 4 {
            0 => AbutmentType::Straight,
            1 => AbutmentType::Angled15,
            2 => AbutmentType::Custom,
            _ => AbutmentType::MultiUnit,
        },
        gingival_height_um: 1_000 + (id as u32 % 5) * 500,
        collar_height_um: 500 + (id as u32 % 4) * 250,
        angulation_centideg: (id % 3) as u16 * 750,
        material: TitaniumGrade::Ti6Al4VEli,
        torque_ncm: 25 + (id % 4) as u16 * 5,
        compatible_fixture: format!("IMP-{:06}", id),
    }
}

fn make_crown(id: u64, pos: ToothPosition) -> CrownSpec {
    CrownSpec {
        crown_id: id,
        tooth_position: pos,
        material: match id % 4 {
            0 => CrownMaterial::Zirconia,
            1 => CrownMaterial::LithiumDisilicate,
            2 => CrownMaterial::PorcelainFusedToMetal,
            _ => CrownMaterial::ResinNanoCeramic,
        },
        shade: match id % 5 {
            0 => ShadeSystem::VitaClassicA1,
            1 => ShadeSystem::VitaClassicA2,
            2 => ShadeSystem::VitaClassicA3,
            3 => ShadeSystem::VitaClassicB1,
            _ => ShadeSystem::VitaClassicC1,
        },
        translucency_pct: 40 + (id % 30) as u8,
        wall_thickness_um: 800 + (id as u32 % 4) * 200,
        occlusal_thickness_um: 1_500 + (id as u32 % 3) * 250,
        margin_fit_um: 30 + (id % 20) as u16,
        cemented: id % 2 == 0,
    }
}

fn make_bone_density(id: u64, pos: ToothPosition) -> BoneDensityMeasurement {
    BoneDensityMeasurement {
        site_id: id,
        tooth_position: pos,
        density_class: match id % 4 {
            0 => BoneDensityClass::TypeI,
            1 => BoneDensityClass::TypeII,
            2 => BoneDensityClass::TypeIII,
            _ => BoneDensityClass::TypeIV,
        },
        hounsfield_units: 300 + (id as i16 % 800),
        cortical_thickness_um: 1_000 + (id as u32 % 3) * 500,
        trabecular_density_hu: 150 + (id as i16 % 400),
        available_bone_height_um: 10_000 + (id as u32 % 8) * 1_000,
        available_bone_width_um: 5_000 + (id as u32 % 6) * 500,
    }
}

fn make_osseo_record(implant_id: u64, day: u32) -> OsseointegrationRecord {
    OsseointegrationRecord {
        implant_id,
        phase: match day {
            0..=3 => OsseointegrationPhase::Hemostasis,
            4..=14 => OsseointegrationPhase::Inflammatory,
            15..=60 => OsseointegrationPhase::Proliferative,
            61..=180 => OsseointegrationPhase::Remodeling,
            _ => OsseointegrationPhase::Mature,
        },
        days_post_surgery: day,
        isq_value: 55 + (day / 10).min(25) as u8,
        periapical_radiograph_hash: implant_id.wrapping_mul(day as u64 + 1),
        marginal_bone_loss_um: (day / 30).min(200) * 10,
        mobility_detected: false,
        notes: format!("Follow-up day {day}"),
    }
}

fn make_surgical_guide(id: u64) -> SurgicalGuideCoordinate {
    let base_x = (id as i32) * 1_000;
    SurgicalGuideCoordinate {
        guide_id: id,
        entry_point: (base_x, 5_000, 12_000),
        apical_point: (base_x + 200, 5_100, 0),
        insertion_axis: (10, 5, -1000),
        depth_stop_um: 10_000 + (id as u32 % 5) * 1_000,
        drill_sequence: vec![2_000, 2_800, 3_200, 3_500],
        sleeve_height_um: 4_000 + (id as u16 % 3) * 500,
        tissue_thickness_um: 1_500 + (id as u16 % 4) * 250,
    }
}

fn make_bite_force(id: u64, pos: ToothPosition) -> BiteForceReading {
    BiteForceReading {
        reading_id: id,
        tooth_position: pos,
        force_millinewtons: 200_000 + (id as u32 % 20) * 50_000,
        contact_area_um2: 1_500_000 + id * 100_000,
        timestamp_epoch_ms: 1_700_000_000_000 + id * 60_000,
        lateral_deviation_um: (id as i32 % 200) - 100,
        is_centric: id % 3 != 0,
    }
}

fn make_milling_toolpath(id: u64, waypoint_count: usize) -> MillingToolpath {
    MillingToolpath {
        job_id: id,
        tool_diameter_um: 500 + (id as u32 % 4) * 250,
        spindle_rpm: 40_000 + (id as u32 % 5) * 5_000,
        feed_rate_um_per_min: 800_000 + (id as u32 % 4) * 100_000,
        stepover_um: 50 + (id as u32 % 6) * 25,
        depth_per_pass_um: 100 + (id as u32 % 3) * 50,
        waypoints: (0..waypoint_count)
            .map(|i| {
                let t = i as i32;
                (t * 100, t * 50 + (t % 7) * 30, -(t * 20))
            })
            .collect(),
        coolant_flow_ml_per_min: 30 + (id as u16 % 10) * 5,
        estimated_time_sec: 600 + (waypoint_count as u32) * 2,
    }
}

fn make_occlusion_map(id: u64, contact_count: usize) -> OcclusionMap {
    OcclusionMap {
        map_id: id,
        patient_hash: id.wrapping_mul(0xDEAD_BEEF_CAFE_1234),
        contacts: (0..contact_count)
            .map(|i| {
                let t = i as i32;
                (t * 200 - 5_000, t * 150 - 3_000, 50 + (t as u32 % 200) * 10)
            })
            .collect(),
        max_intercuspation_force_mn: 500_000 + (id as u32 % 10) * 50_000,
        lateral_excursion_contacts: (0..contact_count / 3)
            .map(|i| {
                let t = i as i32;
                (t * 300, t * 200, 30 + (t as u32 % 100) * 5)
            })
            .collect(),
        protrusive_contacts: (0..contact_count / 4)
            .map(|i| {
                let t = i as i32;
                (t * 250, t * 180, 20 + (t as u32 % 80) * 8)
            })
            .collect(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Single implant fixture round-trip.
#[test]
fn test_zstd_implant_fixture_roundtrip() {
    let fixture = make_implant_fixture(1);
    let encoded = encode_to_vec(&fixture).expect("encode ImplantFixture failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ImplantFixture, usize) =
        decode_from_slice(&decompressed).expect("decode ImplantFixture failed");
    assert_eq!(fixture, decoded);
}

/// 2. Vector of implant fixtures with varying dimensions.
#[test]
fn test_zstd_implant_fixture_batch() {
    let fixtures: Vec<ImplantFixture> = (0u32..50).map(make_implant_fixture).collect();
    let encoded = encode_to_vec(&fixtures).expect("encode fixture batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ImplantFixture>, usize) =
        decode_from_slice(&decompressed).expect("decode fixture batch failed");
    assert_eq!(fixtures, decoded);
}

/// 3. Abutment specification round-trip.
#[test]
fn test_zstd_abutment_roundtrip() {
    let abutment = make_abutment(100);
    let encoded = encode_to_vec(&abutment).expect("encode AbutmentSpec failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AbutmentSpec, usize) =
        decode_from_slice(&decompressed).expect("decode AbutmentSpec failed");
    assert_eq!(abutment, decoded);
}

/// 4. Crown specification with shade matching data.
#[test]
fn test_zstd_crown_shade_matching() {
    let crowns: Vec<CrownSpec> = vec![
        make_crown(1, ToothPosition::UpperRightCentralIncisor),
        make_crown(2, ToothPosition::UpperRightLateralIncisor),
        make_crown(3, ToothPosition::UpperLeftCentralIncisor),
        make_crown(4, ToothPosition::UpperLeftLateralIncisor),
    ];
    let encoded = encode_to_vec(&crowns).expect("encode crown batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<CrownSpec>, usize) =
        decode_from_slice(&decompressed).expect("decode crown batch failed");
    assert_eq!(crowns, decoded);
}

/// 5. Bone density classification across multiple sites.
#[test]
fn test_zstd_bone_density_multi_site() {
    let measurements: Vec<BoneDensityMeasurement> = vec![
        make_bone_density(1, ToothPosition::LowerRightFirstMolar),
        make_bone_density(2, ToothPosition::LowerLeftFirstMolar),
        make_bone_density(3, ToothPosition::UpperRightFirstMolar),
        make_bone_density(4, ToothPosition::UpperLeftCanine),
    ];
    let encoded = encode_to_vec(&measurements).expect("encode bone density failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<BoneDensityMeasurement>, usize) =
        decode_from_slice(&decompressed).expect("decode bone density failed");
    assert_eq!(measurements, decoded);
}

/// 6. Intraoral 3D scan point cloud with normals.
#[test]
fn test_zstd_intraoral_scan_cloud() {
    let scan = IntraoralScanCloud {
        scan_id: 9001,
        patient_hash: 0xABCD_1234_5678_9ABC,
        points: (0..500)
            .map(|i| (i * 100 - 25_000, i * 80 - 20_000, i * 10 + 5_000))
            .collect(),
        normal_vectors: (0..500).map(|i| (0, 0, 1000 + i as i16)).collect(),
        resolution_um: 50,
        scanner_model: "CEREC Primescan 2".to_string(),
    };
    let encoded = encode_to_vec(&scan).expect("encode IntraoralScanCloud failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "point cloud should compress well"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (IntraoralScanCloud, usize) =
        decode_from_slice(&decompressed).expect("decode IntraoralScanCloud failed");
    assert_eq!(scan, decoded);
}

/// 7. Milling toolpath with many waypoints.
#[test]
fn test_zstd_milling_toolpath_roundtrip() {
    let toolpath = make_milling_toolpath(42, 1_000);
    let encoded = encode_to_vec(&toolpath).expect("encode MillingToolpath failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (MillingToolpath, usize) =
        decode_from_slice(&decompressed).expect("decode MillingToolpath failed");
    assert_eq!(toolpath, decoded);
}

/// 8. Osseointegration timeline over 365 days.
#[test]
fn test_zstd_osseointegration_timeline() {
    let records: Vec<OsseointegrationRecord> = [0, 1, 3, 7, 14, 30, 60, 90, 120, 180, 365]
        .iter()
        .map(|&day| make_osseo_record(1001, day))
        .collect();
    let encoded = encode_to_vec(&records).expect("encode osseo timeline failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<OsseointegrationRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode osseo timeline failed");
    assert_eq!(records, decoded);
}

/// 9. Bite force measurement series across arch.
#[test]
fn test_zstd_bite_force_series() {
    let readings: Vec<BiteForceReading> = vec![
        make_bite_force(1, ToothPosition::UpperRightFirstMolar),
        make_bite_force(2, ToothPosition::UpperRightSecondPremolar),
        make_bite_force(3, ToothPosition::UpperRightCanine),
        make_bite_force(4, ToothPosition::UpperRightCentralIncisor),
        make_bite_force(5, ToothPosition::LowerRightFirstMolar),
        make_bite_force(6, ToothPosition::LowerLeftFirstMolar),
    ];
    let encoded = encode_to_vec(&readings).expect("encode bite force series failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<BiteForceReading>, usize) =
        decode_from_slice(&decompressed).expect("decode bite force series failed");
    assert_eq!(readings, decoded);
}

/// 10. Surgical guide coordinates for multi-implant placement.
#[test]
fn test_zstd_surgical_guide_coordinates() {
    let guides: Vec<SurgicalGuideCoordinate> = (1u64..=6).map(make_surgical_guide).collect();
    let encoded = encode_to_vec(&guides).expect("encode surgical guides failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<SurgicalGuideCoordinate>, usize) =
        decode_from_slice(&decompressed).expect("decode surgical guides failed");
    assert_eq!(guides, decoded);
}

/// 11. Occlusion map with dense contact data.
#[test]
fn test_zstd_occlusion_map_dense() {
    let map = make_occlusion_map(77, 200);
    let encoded = encode_to_vec(&map).expect("encode OcclusionMap failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (OcclusionMap, usize) =
        decode_from_slice(&decompressed).expect("decode OcclusionMap failed");
    assert_eq!(map, decoded);
}

/// 12. Multi-unit implant bridge spanning four fixtures.
#[test]
fn test_zstd_implant_bridge_roundtrip() {
    let bridge = ImplantBridge {
        bridge_id: 5050,
        fixture_ids: vec![101, 102, 103, 104],
        pontic_positions: vec![
            ToothPosition::UpperRightSecondPremolar,
            ToothPosition::UpperRightFirstPremolar,
        ],
        framework_material: CrownMaterial::Zirconia,
        veneer_material: CrownMaterial::FeldspathicPorcelain,
        connector_cross_section_um2: vec![9_000_000, 8_500_000, 9_200_000],
        passive_fit_gap_um: 15,
        screw_retained: true,
    };
    let encoded = encode_to_vec(&bridge).expect("encode ImplantBridge failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ImplantBridge, usize) =
        decode_from_slice(&decompressed).expect("decode ImplantBridge failed");
    assert_eq!(bridge, decoded);
}

/// 13. Healing abutment sizing batch.
#[test]
fn test_zstd_healing_abutment_batch() {
    let abutments: Vec<HealingAbutment> = (0u64..20)
        .map(|id| HealingAbutment {
            abutment_id: id,
            diameter_um: 3_500 + (id as u32 % 5) * 500,
            height_um: 2_000 + (id as u32 % 4) * 1_000,
            emergence_profile: 1 + (id % 3) as u8,
            alloy: TitaniumGrade::CpGrade2,
            placement_torque_ncm: 15 + (id % 5) as u16 * 5,
        })
        .collect();
    let encoded = encode_to_vec(&abutments).expect("encode healing abutments failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<HealingAbutment>, usize) =
        decode_from_slice(&decompressed).expect("decode healing abutments failed");
    assert_eq!(abutments, decoded);
}

/// 14. Material lot traceability records for regulatory compliance.
#[test]
fn test_zstd_material_lot_traceability() {
    let lots: Vec<MaterialLotRecord> = (0u64..10)
        .map(|i| MaterialLotRecord {
            lot_number: format!("LOT-2026-{i:04}"),
            alloy: match i % 3 {
                0 => TitaniumGrade::CpGrade4,
                1 => TitaniumGrade::Ti6Al4VEli,
                _ => TitaniumGrade::Ti13Nb13Zr,
            },
            tensile_strength_mpa: 860 + (i as u32 % 40) * 5,
            yield_strength_mpa: 790 + (i as u32 % 30) * 5,
            elongation_pct_x10: 100 + (i as u16 % 50) * 2,
            oxygen_ppm: 1200 + (i as u16 % 300),
            nitrogen_ppm: 50 + (i as u16 % 40),
            hydrogen_ppm: 10 + (i as u16 % 15),
            iron_ppm: 200 + (i as u16 % 100),
            certification_hash: 0xCAFE_BABE_0000_0000 + i,
            expiry_epoch_sec: 1_800_000_000 + i * 86_400 * 365,
        })
        .collect();
    let encoded = encode_to_vec(&lots).expect("encode material lots failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<MaterialLotRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode material lots failed");
    assert_eq!(lots, decoded);
}

/// 15. Digital impression scan comparison.
#[test]
fn test_zstd_scan_comparison_roundtrip() {
    let comparison = ScanComparison {
        comparison_id: 3003,
        pre_scan_id: 1001,
        post_scan_id: 2002,
        rms_deviation_um: 42,
        max_deviation_um: 185,
        deviation_histogram: (0u32..50).map(|i| i * i + 10).collect(),
        regions_of_interest: vec![
            (1000, 2000, 3000, 120),
            (-500, 1500, 2800, 85),
            (2200, -100, 3100, 175),
        ],
    };
    let encoded = encode_to_vec(&comparison).expect("encode ScanComparison failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ScanComparison, usize) =
        decode_from_slice(&decompressed).expect("decode ScanComparison failed");
    assert_eq!(comparison, decoded);
}

/// 16. Complete implant case combining all components.
#[test]
fn test_zstd_full_implant_case() {
    let case = ImplantCase {
        case_id: 7777,
        fixture: make_implant_fixture(7),
        abutment: make_abutment(7),
        crown: make_crown(7, ToothPosition::LowerRightFirstMolar),
        bone_measurement: make_bone_density(7, ToothPosition::LowerRightFirstMolar),
        surgical_guide: make_surgical_guide(7),
        osseo_records: vec![
            make_osseo_record(7, 0),
            make_osseo_record(7, 7),
            make_osseo_record(7, 30),
            make_osseo_record(7, 90),
            make_osseo_record(7, 180),
            make_osseo_record(7, 365),
        ],
    };
    let encoded = encode_to_vec(&case).expect("encode ImplantCase failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ImplantCase, usize) =
        decode_from_slice(&decompressed).expect("decode ImplantCase failed");
    assert_eq!(case, decoded);
}

/// 17. Large point cloud with compression ratio verification.
#[test]
fn test_zstd_large_point_cloud_compression_ratio() {
    let scan = IntraoralScanCloud {
        scan_id: 2024,
        patient_hash: 0x1234_5678_9ABC_DEF0,
        points: (0..5_000)
            .map(|i| {
                let t = i as i32;
                (
                    t * 10 - 25_000,
                    (t * t) % 40_000 - 20_000,
                    5_000 + t % 3_000,
                )
            })
            .collect(),
        normal_vectors: (0..5_000)
            .map(|i| {
                let n = i as i16;
                (n % 100, n % 50, 1000)
            })
            .collect(),
        resolution_um: 25,
        scanner_model: "iTero Element 5D Plus".to_string(),
    };
    let encoded = encode_to_vec(&scan).expect("encode large point cloud failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.2,
        "expected compression ratio > 1.2, got {ratio:.2}"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (IntraoralScanCloud, usize) =
        decode_from_slice(&decompressed).expect("decode large point cloud failed");
    assert_eq!(scan, decoded);
}

/// 18. Many milling toolpaths in a production batch.
#[test]
fn test_zstd_milling_production_batch() {
    let batch: Vec<MillingToolpath> = (0u64..15)
        .map(|id| make_milling_toolpath(id, 200))
        .collect();
    let encoded = encode_to_vec(&batch).expect("encode milling batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<MillingToolpath>, usize) =
        decode_from_slice(&decompressed).expect("decode milling batch failed");
    assert_eq!(batch, decoded);
}

/// 19. Multiple occlusion maps for before/after prosthetic adjustment.
#[test]
fn test_zstd_occlusion_before_after() {
    let before = make_occlusion_map(1, 150);
    let after = make_occlusion_map(2, 120);
    let pair = (before.clone(), after.clone());
    let encoded = encode_to_vec(&pair).expect("encode occlusion pair failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): ((OcclusionMap, OcclusionMap), usize) =
        decode_from_slice(&decompressed).expect("decode occlusion pair failed");
    assert_eq!(pair, decoded);
}

/// 20. Titanium alloy grade enum exhaustive round-trip.
#[test]
fn test_zstd_titanium_grades_exhaustive() {
    let grades = vec![
        TitaniumGrade::CpGrade1,
        TitaniumGrade::CpGrade2,
        TitaniumGrade::CpGrade4,
        TitaniumGrade::Ti6Al4VEli,
        TitaniumGrade::TiZrAlloy,
        TitaniumGrade::Ti13Nb13Zr,
    ];
    let encoded = encode_to_vec(&grades).expect("encode titanium grades failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<TitaniumGrade>, usize) =
        decode_from_slice(&decompressed).expect("decode titanium grades failed");
    assert_eq!(grades, decoded);
}

/// 21. Multiple complete implant cases (clinic daily batch).
#[test]
fn test_zstd_daily_case_batch() {
    let positions = vec![
        ToothPosition::UpperRightFirstMolar,
        ToothPosition::UpperLeftCanine,
        ToothPosition::LowerRightFirstMolar,
        ToothPosition::LowerLeftCentralIncisor,
        ToothPosition::UpperRightCentralIncisor,
    ];
    let cases: Vec<ImplantCase> = positions
        .into_iter()
        .enumerate()
        .map(|(i, pos)| {
            let id = i as u64 + 100;
            ImplantCase {
                case_id: id,
                fixture: make_implant_fixture(i as u32),
                abutment: make_abutment(id),
                crown: make_crown(id, pos.clone()),
                bone_measurement: make_bone_density(id, pos),
                surgical_guide: make_surgical_guide(id),
                osseo_records: vec![
                    make_osseo_record(id, 0),
                    make_osseo_record(id, 14),
                    make_osseo_record(id, 90),
                ],
            }
        })
        .collect();
    let encoded = encode_to_vec(&cases).expect("encode daily case batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ImplantCase>, usize) =
        decode_from_slice(&decompressed).expect("decode daily case batch failed");
    assert_eq!(cases, decoded);
}

/// 22. Surface treatment and thread profile enum combinations.
#[test]
fn test_zstd_surface_thread_combinations() {
    let surfaces = vec![
        SurfaceTreatment::Machined,
        SurfaceTreatment::SandBlasted,
        SurfaceTreatment::AcidEtched,
        SurfaceTreatment::SlaBlasted,
        SurfaceTreatment::AnodizedTiUnite,
        SurfaceTreatment::HydroxyapatiteCoated,
        SurfaceTreatment::LaserMicroTextured,
    ];
    let threads = vec![
        ThreadProfile::VShape,
        ThreadProfile::Buttress,
        ThreadProfile::ReverseButttress,
        ThreadProfile::SquareThread,
        ThreadProfile::MicroThread,
        ThreadProfile::DoubleHelix,
        ThreadProfile::TripleHelix,
    ];
    let combinations: Vec<(SurfaceTreatment, ThreadProfile)> =
        surfaces.into_iter().zip(threads).collect();
    let encoded = encode_to_vec(&combinations).expect("encode surface-thread combos failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<(SurfaceTreatment, ThreadProfile)>, usize) =
        decode_from_slice(&decompressed).expect("decode surface-thread combos failed");
    assert_eq!(combinations, decoded);
}
