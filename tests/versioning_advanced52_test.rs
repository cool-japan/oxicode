#![cfg(feature = "versioning")]

//! Versioning tests for OxiCode: archaeological excavation and cultural resource management.

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
use oxicode::versioning::Version;
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SoilTexture {
    Clay,
    Silt,
    Sand,
    Loam,
    Gravel,
    Peat,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ArtifactMaterial {
    Pottery,
    Lithic,
    Bone,
    Metal,
    Glass,
    Organic,
    Shell,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FeatureType {
    Pit,
    Hearth,
    Wall,
    Burial,
    Ditch,
    Posthole,
    Floor,
    Midden,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConservationStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Fragmentary,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StratigraphicRelation {
    Above,
    Below,
    CutsThrough,
    CutBy,
    SameAs,
    Abuts,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DatingMethod {
    Radiocarbon,
    Thermoluminescence,
    OpticallyStimulated,
    Dendrochronology,
    Relative,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExcavationUnit {
    trench_id: u32,
    grid_easting: i32,
    grid_northing: i32,
    width_cm: u16,
    length_cm: u16,
    excavator_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StratigraphyLayer {
    context_number: u32,
    description: String,
    munsell_color: String,
    soil_texture: SoilTexture,
    depth_top_cm: u16,
    depth_bottom_cm: u16,
    inclusions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarrisMatrixEdge {
    upper_context: u32,
    lower_context: u32,
    relation: StratigraphicRelation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ArtifactCatalogEntry {
    catalog_id: u64,
    context_number: u32,
    material: ArtifactMaterial,
    object_type: String,
    weight_grams_x10: u32,
    length_mm: u16,
    width_mm: u16,
    thickness_mm: u16,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiocarbonResult {
    lab_code: String,
    sample_id: u64,
    context_number: u32,
    bp_years: u32,
    sigma_plus: u16,
    sigma_minus: u16,
    delta_c13_x100: i32,
    calibrated_range_start_bce: i32,
    calibrated_range_end_bce: i32,
    material_dated: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PhotogrammetrySurveyPoint {
    point_id: u64,
    easting_x1000: i64,
    northing_x1000: i64,
    elevation_x1000: i64,
    accuracy_mm: u16,
    photo_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SoilSampleAnalysis {
    sample_id: u64,
    context_number: u32,
    ph_x100: u16,
    phosphate_ppm: u32,
    organic_content_x100: u16,
    magnetic_susceptibility_x10: u32,
    flotation_volume_ml: u32,
    seed_count: u32,
    charcoal_weight_mg: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeatureRecord {
    feature_id: u32,
    feature_type: FeatureType,
    trench_id: u32,
    plan_number: String,
    section_number: String,
    context_fills: Vec<u32>,
    cut_number: u32,
    interpretation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SiteFormationProcess {
    process_id: u32,
    description: String,
    is_natural: bool,
    affected_contexts: Vec<u32>,
    evidence: String,
    severity_1_to_5: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConservationTreatmentLog {
    log_id: u64,
    artifact_id: u64,
    treatment_date_days_since_epoch: u32,
    before_status: ConservationStatus,
    after_status: ConservationStatus,
    treatment_description: String,
    chemicals_used: Vec<String>,
    conservator_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GisSpatialPolygon {
    polygon_id: u64,
    trench_id: u32,
    vertex_eastings_x1000: Vec<i64>,
    vertex_northings_x1000: Vec<i64>,
    area_sq_m_x100: u64,
    layer_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PotterySherdRecord {
    sherd_id: u64,
    context_number: u32,
    fabric_code: String,
    form_type: String,
    rim_diameter_mm: Option<u16>,
    base_diameter_mm: Option<u16>,
    wall_thickness_mm: u16,
    decoration: Option<String>,
    weight_grams_x10: u32,
    sherd_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LithicAnalysis {
    lithic_id: u64,
    raw_material: String,
    platform_type: String,
    dorsal_scar_count: u8,
    cortex_percent: u8,
    is_retouched: bool,
    length_mm: u16,
    width_mm: u16,
    thickness_mm: u16,
    weight_grams_x10: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FaunalRemain {
    bone_id: u64,
    context_number: u32,
    taxon: String,
    element: String,
    side: String,
    portion: String,
    burned: bool,
    gnaw_marks: bool,
    cut_marks: bool,
    fusion_state: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DatingAssociation {
    association_id: u32,
    method: DatingMethod,
    context_number: u32,
    date_bp: u32,
    error_margin: u16,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ArchaeobotanicalSample {
    sample_id: u64,
    context_number: u32,
    taxa: Vec<String>,
    counts: Vec<u32>,
    volume_liters_x10: u32,
    charred: bool,
    waterlogged: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SitePhaseAssignment {
    phase_id: u16,
    phase_name: String,
    start_year_bce: i32,
    end_year_bce: i32,
    assigned_contexts: Vec<u32>,
    confidence_percent: u8,
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_excavation_unit_basic_versioned_roundtrip() {
    let unit = ExcavationUnit {
        trench_id: 3,
        grid_easting: 504230,
        grid_northing: 179850,
        width_cm: 100,
        length_cm: 200,
        excavator_name: "Dr. Helena Vasquez".into(),
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&unit, ver).expect("encode excavation unit");
    let (decoded, version, consumed): (ExcavationUnit, Version, usize) =
        decode_versioned_value(&bytes).expect("decode excavation unit");
    assert_eq!(decoded, unit);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_stratigraphy_layer_with_inclusions() {
    let layer = StratigraphyLayer {
        context_number: 1042,
        description: "Compact reddish-brown silty clay with frequent charcoal flecks".into(),
        munsell_color: "5YR 4/4".into(),
        soil_texture: SoilTexture::Clay,
        depth_top_cm: 45,
        depth_bottom_cm: 63,
        inclusions: vec![
            "charcoal".into(),
            "burnt bone fragments".into(),
            "small pebbles <2cm".into(),
        ],
    };
    let ver = Version::new(1, 2, 0);
    let bytes = encode_versioned_value(&layer, ver).expect("encode stratigraphy layer");
    let (decoded, version, _consumed): (StratigraphyLayer, Version, usize) =
        decode_versioned_value(&bytes).expect("decode stratigraphy layer");
    assert_eq!(decoded, layer);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
}

#[test]
fn test_harris_matrix_edge_superposition() {
    let edge = HarrisMatrixEdge {
        upper_context: 1003,
        lower_context: 1004,
        relation: StratigraphicRelation::Above,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&edge, ver).expect("encode harris edge");
    let (decoded, version, consumed): (HarrisMatrixEdge, Version, usize) =
        decode_versioned_value(&bytes).expect("decode harris edge");
    assert_eq!(decoded, edge);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_harris_matrix_cut_relation() {
    let edge = HarrisMatrixEdge {
        upper_context: 2050,
        lower_context: 2045,
        relation: StratigraphicRelation::CutsThrough,
    };
    let bytes = encode_to_vec(&edge).expect("encode harris cut relation");
    let (decoded, consumed): (HarrisMatrixEdge, usize) =
        decode_from_slice(&bytes).expect("decode harris cut relation");
    assert_eq!(decoded, edge);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_artifact_catalog_pottery_entry() {
    let entry = ArtifactCatalogEntry {
        catalog_id: 20240001,
        context_number: 1042,
        material: ArtifactMaterial::Pottery,
        object_type: "rim sherd - everted".into(),
        weight_grams_x10: 342,
        length_mm: 67,
        width_mm: 45,
        thickness_mm: 8,
        notes: "Oxidised fabric with quartz temper, possible LBA vessel".into(),
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&entry, ver).expect("encode pottery artifact");
    let (decoded, version, _consumed): (ArtifactCatalogEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode pottery artifact");
    assert_eq!(decoded, entry);
    assert_eq!(version.major, 2);
}

#[test]
fn test_artifact_catalog_lithic_tool() {
    let entry = ArtifactCatalogEntry {
        catalog_id: 20240087,
        context_number: 1058,
        material: ArtifactMaterial::Lithic,
        object_type: "end scraper on flake".into(),
        weight_grams_x10: 156,
        length_mm: 42,
        width_mm: 31,
        thickness_mm: 12,
        notes: "Fine-grained flint, invasive retouch on distal end".into(),
    };
    let ver = Version::new(2, 1, 3);
    let bytes = encode_versioned_value(&entry, ver).expect("encode lithic artifact");
    let (decoded, version, consumed): (ArtifactCatalogEntry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode lithic artifact");
    assert_eq!(decoded, entry);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_radiocarbon_dating_result_neolithic() {
    let rc = RadiocarbonResult {
        lab_code: "OxA-39421".into(),
        sample_id: 500,
        context_number: 1042,
        bp_years: 4950,
        sigma_plus: 40,
        sigma_minus: 35,
        delta_c13_x100: -2560,
        calibrated_range_start_bce: 3790,
        calibrated_range_end_bce: 3650,
        material_dated: "charred hazelnut shell".into(),
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&rc, ver).expect("encode radiocarbon");
    let (decoded, version, _consumed): (RadiocarbonResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode radiocarbon");
    assert_eq!(decoded, rc);
    assert_eq!(version.major, 1);
}

#[test]
fn test_photogrammetry_survey_points_with_photos() {
    let point = PhotogrammetrySurveyPoint {
        point_id: 77001,
        easting_x1000: 504230_500,
        northing_x1000: 179850_250,
        elevation_x1000: 42_750,
        accuracy_mm: 3,
        photo_ids: vec![1001, 1002, 1003, 1004],
    };
    let ver = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&point, ver).expect("encode survey point");
    let (decoded, version, consumed): (PhotogrammetrySurveyPoint, Version, usize) =
        decode_versioned_value(&bytes).expect("decode survey point");
    assert_eq!(decoded, point);
    assert_eq!(version.major, 3);
    assert!(consumed > 0);
}

#[test]
fn test_soil_sample_phosphate_analysis() {
    let sample = SoilSampleAnalysis {
        sample_id: 9001,
        context_number: 1042,
        ph_x100: 720,
        phosphate_ppm: 1850,
        organic_content_x100: 345,
        magnetic_susceptibility_x10: 892,
        flotation_volume_ml: 10000,
        seed_count: 47,
        charcoal_weight_mg: 3200,
    };
    let ver = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&sample, ver).expect("encode soil sample");
    let (decoded, version, _consumed): (SoilSampleAnalysis, Version, usize) =
        decode_versioned_value(&bytes).expect("decode soil sample");
    assert_eq!(decoded, sample);
    assert_eq!(version.minor, 1);
}

#[test]
fn test_feature_record_hearth_with_fills() {
    let feature = FeatureRecord {
        feature_id: 42,
        feature_type: FeatureType::Hearth,
        trench_id: 3,
        plan_number: "P-042".into(),
        section_number: "S-018".into(),
        context_fills: vec![1060, 1061, 1062],
        cut_number: 1059,
        interpretation: "Sub-circular hearth with reddened clay base and charcoal-rich fill".into(),
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&feature, ver).expect("encode hearth feature");
    let (decoded, version, consumed): (FeatureRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode hearth feature");
    assert_eq!(decoded, feature);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_feature_record_burial_pit() {
    let feature = FeatureRecord {
        feature_id: 88,
        feature_type: FeatureType::Burial,
        trench_id: 5,
        plan_number: "P-088".into(),
        section_number: "S-044".into(),
        context_fills: vec![2010, 2011, 2012, 2013],
        cut_number: 2009,
        interpretation: "Crouched inhumation with grave goods, E-W orientation".into(),
    };
    let bytes = encode_to_vec(&feature).expect("encode burial feature");
    let (decoded, consumed): (FeatureRecord, usize) =
        decode_from_slice(&bytes).expect("decode burial feature");
    assert_eq!(decoded, feature);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_site_formation_process_bioturbation() {
    let process = SiteFormationProcess {
        process_id: 7,
        description: "Root disturbance from mature oak tree".into(),
        is_natural: true,
        affected_contexts: vec![1003, 1004, 1005],
        evidence: "Root channels visible in section, displaced finds at context boundary".into(),
        severity_1_to_5: 3,
    };
    let ver = Version::new(1, 0, 1);
    let bytes = encode_versioned_value(&process, ver).expect("encode formation process");
    let (decoded, version, _consumed): (SiteFormationProcess, Version, usize) =
        decode_versioned_value(&bytes).expect("decode formation process");
    assert_eq!(decoded, process);
    assert_eq!(version.patch, 1);
}

#[test]
fn test_conservation_treatment_ceramic_consolidation() {
    let log = ConservationTreatmentLog {
        log_id: 30001,
        artifact_id: 20240001,
        treatment_date_days_since_epoch: 19900,
        before_status: ConservationStatus::Poor,
        after_status: ConservationStatus::Fair,
        treatment_description: "Consolidation with Paraloid B-72 in acetone (5% w/v)".into(),
        chemicals_used: vec!["Paraloid B-72".into(), "Acetone".into()],
        conservator_name: "Maria Gonzalez".into(),
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&log, ver).expect("encode conservation log");
    let (decoded, version, consumed): (ConservationTreatmentLog, Version, usize) =
        decode_versioned_value(&bytes).expect("decode conservation log");
    assert_eq!(decoded, log);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
}

#[test]
fn test_gis_spatial_polygon_trench_outline() {
    let polygon = GisSpatialPolygon {
        polygon_id: 5001,
        trench_id: 3,
        vertex_eastings_x1000: vec![504230_000, 504232_000, 504232_000, 504230_000],
        vertex_northings_x1000: vec![179850_000, 179850_000, 179854_000, 179854_000],
        area_sq_m_x100: 800,
        layer_name: "trench_outlines".into(),
    };
    let ver = Version::new(1, 3, 0);
    let bytes = encode_versioned_value(&polygon, ver).expect("encode GIS polygon");
    let (decoded, version, _consumed): (GisSpatialPolygon, Version, usize) =
        decode_versioned_value(&bytes).expect("decode GIS polygon");
    assert_eq!(decoded, polygon);
    assert_eq!(version.minor, 3);
}

#[test]
fn test_pottery_sherd_decorated_rim() {
    let sherd = PotterySherdRecord {
        sherd_id: 60001,
        context_number: 1042,
        fabric_code: "F12".into(),
        form_type: "Jar - storage vessel".into(),
        rim_diameter_mm: Some(280),
        base_diameter_mm: None,
        wall_thickness_mm: 9,
        decoration: Some("incised chevron band below rim".into()),
        weight_grams_x10: 485,
        sherd_count: 1,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&sherd, ver).expect("encode pottery sherd");
    let (decoded, version, consumed): (PotterySherdRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode pottery sherd");
    assert_eq!(decoded, sherd);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_pottery_sherd_undecorated_base() {
    let sherd = PotterySherdRecord {
        sherd_id: 60042,
        context_number: 1061,
        fabric_code: "F03".into(),
        form_type: "Bowl - open form".into(),
        rim_diameter_mm: None,
        base_diameter_mm: Some(120),
        wall_thickness_mm: 6,
        decoration: None,
        weight_grams_x10: 210,
        sherd_count: 3,
    };
    let bytes = encode_to_vec(&sherd).expect("encode base sherd");
    let (decoded, consumed): (PotterySherdRecord, usize) =
        decode_from_slice(&bytes).expect("decode base sherd");
    assert_eq!(decoded, sherd);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_lithic_analysis_retouched_scraper() {
    let lithic = LithicAnalysis {
        lithic_id: 70001,
        raw_material: "Flint - East Anglian nodular".into(),
        platform_type: "Plain".into(),
        dorsal_scar_count: 4,
        cortex_percent: 15,
        is_retouched: true,
        length_mm: 52,
        width_mm: 38,
        thickness_mm: 11,
        weight_grams_x10: 218,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&lithic, ver).expect("encode lithic scraper");
    let (decoded, version, consumed): (LithicAnalysis, Version, usize) =
        decode_versioned_value(&bytes).expect("decode lithic scraper");
    assert_eq!(decoded, lithic);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_faunal_remain_cattle_tibia() {
    let bone = FaunalRemain {
        bone_id: 80001,
        context_number: 1042,
        taxon: "Bos taurus".into(),
        element: "Tibia".into(),
        side: "Left".into(),
        portion: "Proximal shaft".into(),
        burned: false,
        gnaw_marks: true,
        cut_marks: true,
        fusion_state: "Fused".into(),
    };
    let ver = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&bone, ver).expect("encode faunal remain");
    let (decoded, version, _consumed): (FaunalRemain, Version, usize) =
        decode_versioned_value(&bytes).expect("decode faunal remain");
    assert_eq!(decoded, bone);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 1);
}

#[test]
fn test_dating_association_thermoluminescence() {
    let assoc = DatingAssociation {
        association_id: 12,
        method: DatingMethod::Thermoluminescence,
        context_number: 1060,
        date_bp: 3200,
        error_margin: 150,
        notes: "TL date from burnt clay lining of hearth F42".into(),
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&assoc, ver).expect("encode TL dating");
    let (decoded, version, consumed): (DatingAssociation, Version, usize) =
        decode_versioned_value(&bytes).expect("decode TL dating");
    assert_eq!(decoded, assoc);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_archaeobotanical_charred_cereal_assemblage() {
    let sample = ArchaeobotanicalSample {
        sample_id: 90001,
        context_number: 1061,
        taxa: vec![
            "Triticum dicoccum".into(),
            "Hordeum vulgare".into(),
            "Corylus avellana".into(),
            "Chenopodium album".into(),
        ],
        counts: vec![23, 15, 8, 42],
        volume_liters_x10: 100,
        charred: true,
        waterlogged: false,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&sample, ver).expect("encode archaeobotanical sample");
    let (decoded, version, _consumed): (ArchaeobotanicalSample, Version, usize) =
        decode_versioned_value(&bytes).expect("decode archaeobotanical sample");
    assert_eq!(decoded, sample);
    assert_eq!(version.major, 2);
}

#[test]
fn test_site_phase_assignment_with_contexts() {
    let phase = SitePhaseAssignment {
        phase_id: 3,
        phase_name: "Middle Neolithic Occupation".into(),
        start_year_bce: 3800,
        end_year_bce: 3500,
        assigned_contexts: vec![1040, 1041, 1042, 1043, 1058, 1059, 1060, 1061, 1062],
        confidence_percent: 85,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&phase, ver).expect("encode site phase");
    let (decoded, version, consumed): (SitePhaseAssignment, Version, usize) =
        decode_versioned_value(&bytes).expect("decode site phase");
    assert_eq!(decoded, phase);
    assert_eq!(version.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_radiocarbon_iron_age_bone_collagen() {
    let rc = RadiocarbonResult {
        lab_code: "Beta-612033".into(),
        sample_id: 501,
        context_number: 2013,
        bp_years: 2450,
        sigma_plus: 30,
        sigma_minus: 30,
        delta_c13_x100: -2010,
        calibrated_range_start_bce: 750,
        calibrated_range_end_bce: 410,
        material_dated: "human bone collagen from burial F88".into(),
    };
    let ver = Version::new(1, 2, 1);
    let bytes = encode_versioned_value(&rc, ver).expect("encode iron age radiocarbon");
    let (decoded, version, _consumed): (RadiocarbonResult, Version, usize) =
        decode_versioned_value(&bytes).expect("decode iron age radiocarbon");
    assert_eq!(decoded, rc);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 1);
}
