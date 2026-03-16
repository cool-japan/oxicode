//! Advanced file I/O tests for OxiCode — domain: digital pathology (medical imaging for pathology labs)

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StainType {
    HematoxylinAndEosin,
    Immunohistochemistry {
        antibody: String,
        clone_name: String,
    },
    PeriodicAcidSchiff,
    Trichrome,
    GiemsaStain,
    SilverStain,
    MucicarmineStain,
    CongoRed,
    IronStainPrussianBlue,
    ElasticVanGieson,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ScanMagnification {
    X5,
    X10,
    X20,
    X40,
    X60Oil,
    X100Oil,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TumorGrade {
    GradeX,
    Grade1WellDifferentiated,
    Grade2ModeratelyDifferentiated,
    Grade3PoorlyDifferentiated,
    Grade4Undifferentiated,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BiomarkerResult {
    Negative,
    WeakPositive { h_score: u16 },
    ModeratePositive { h_score: u16 },
    StrongPositive { h_score: u16 },
    Equivocal,
    NotEvaluable,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpecimenType {
    Biopsy,
    Resection,
    Cytology,
    FrozenSection,
    FineNeedleAspirate,
    CoreNeedleBiopsy,
    ExcisionalBiopsy,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DiagnosisCategory {
    Benign,
    Atypical,
    SuspiciousForMalignancy,
    Malignant { icd_o_code: String },
    Insufficient,
    DeferredToSpecialStudies,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AnnotationShape {
    Rectangle {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    Ellipse {
        cx: u32,
        cy: u32,
        rx: u32,
        ry: u32,
    },
    Polygon {
        vertices_x: Vec<u32>,
        vertices_y: Vec<u32>,
    },
    FreehandPath {
        points_x: Vec<u32>,
        points_y: Vec<u32>,
    },
    Arrow {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QcStatus {
    Passed,
    ConditionalPass { reason: String },
    Failed { reason: String },
    PendingReview,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TissueSlideMetadata {
    slide_id: String,
    accession_number: String,
    patient_id: String,
    specimen_type: SpecimenType,
    tissue_site: String,
    fixation_duration_hours: u16,
    block_id: String,
    section_thickness_um_x10: u16,
    stain: StainType,
    created_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScanResolution {
    scan_id: u64,
    slide_id: String,
    magnification: ScanMagnification,
    pixel_size_nm_x: u32,
    pixel_size_nm_y: u32,
    image_width_px: u64,
    image_height_px: u64,
    num_focal_planes: u8,
    compression_format: String,
    file_size_bytes: u64,
    scanner_model: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RoiCoordinates {
    roi_id: u32,
    slide_id: String,
    label: String,
    top_left_x: u32,
    top_left_y: u32,
    width: u32,
    height: u32,
    magnification: ScanMagnification,
    area_um2_x100: u64,
    annotator_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellMorphologyMeasurement {
    cell_id: u64,
    roi_id: u32,
    nucleus_area_um2_x100: u32,
    cytoplasm_area_um2_x100: u32,
    nc_ratio_x1000: u16,
    nuclear_perimeter_um_x100: u32,
    nuclear_circularity_x1000: u16,
    chromatin_density_x1000: u16,
    mitotic_figure: bool,
    pleomorphism_score: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiagnosisRecord {
    record_id: u64,
    accession_number: String,
    category: DiagnosisCategory,
    primary_site: String,
    histologic_type: String,
    tumor_grade: TumorGrade,
    margin_status_positive: bool,
    lymphovascular_invasion: bool,
    perineural_invasion: bool,
    pathologist_id: String,
    sign_out_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BiomarkerPanel {
    panel_id: u32,
    accession_number: String,
    er_status: BiomarkerResult,
    pr_status: BiomarkerResult,
    her2_status: BiomarkerResult,
    ki67_percent_x10: u16,
    p53_status: BiomarkerResult,
    pdl1_tps_x10: u16,
    mismatch_repair_intact: bool,
    report_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlideAnnotation {
    annotation_id: u64,
    slide_id: String,
    shape: AnnotationShape,
    label: String,
    color_rgba: u32,
    layer_index: u8,
    author_id: String,
    created_timestamp: u64,
    comment: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PathologyReportMeta {
    report_id: u64,
    accession_number: String,
    patient_name_hash: Vec<u8>,
    specimen_count: u8,
    stain_count: u8,
    slide_count: u16,
    addendum_count: u8,
    turnaround_hours: u32,
    is_amended: bool,
    synoptic_report: bool,
    finalized_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpecimenTracking {
    specimen_id: String,
    accession_number: String,
    collection_timestamp: u64,
    received_timestamp: u64,
    grossing_timestamp: u64,
    embedding_timestamp: u64,
    sectioning_timestamp: u64,
    staining_timestamp: u64,
    scanning_timestamp: u64,
    fixative_type: String,
    cassette_count: u8,
    block_ids: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityControlMetric {
    qc_id: u64,
    slide_id: String,
    focus_score_x1000: u16,
    tissue_coverage_percent_x10: u16,
    stain_uniformity_x1000: u16,
    background_noise_x1000: u16,
    artifact_count: u16,
    pen_mark_detected: bool,
    air_bubble_count: u8,
    fold_count: u8,
    status: QcStatus,
    reviewed_by: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TumorMeasurement {
    measurement_id: u64,
    accession_number: String,
    greatest_dimension_mm_x10: u32,
    second_dimension_mm_x10: u32,
    third_dimension_mm_x10: u32,
    depth_of_invasion_mm_x10: u32,
    closest_margin_mm_x10: u32,
    margin_location: String,
    necrosis_percent_x10: u16,
    viable_tumor_percent_x10: u16,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn unique_tmp(name: &str) -> std::path::PathBuf {
    temp_dir().join(name)
}

// ---------------------------------------------------------------------------
// Tests — 22 total
// ---------------------------------------------------------------------------

/// 1. TissueSlideMetadata with H&E stain — vec roundtrip
#[test]
fn test_tissue_slide_metadata_roundtrip() {
    let slide = TissueSlideMetadata {
        slide_id: "SL-2026-00142-01".into(),
        accession_number: "SP-2026-03421".into(),
        patient_id: "PT-88291".into(),
        specimen_type: SpecimenType::Resection,
        tissue_site: "Left breast, upper outer quadrant".into(),
        fixation_duration_hours: 24,
        block_id: "A1".into(),
        section_thickness_um_x10: 40,
        stain: StainType::HematoxylinAndEosin,
        created_timestamp: 1_773_840_000,
    };
    let bytes = encode_to_vec(&slide).expect("encode TissueSlideMetadata");
    let (decoded, consumed): (TissueSlideMetadata, usize) =
        decode_from_slice(&bytes).expect("decode TissueSlideMetadata");
    assert_eq!(slide, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 2. TissueSlideMetadata with IHC stain — file roundtrip
#[test]
fn test_tissue_slide_ihc_file_roundtrip() {
    let path = unique_tmp("pathology_slide_ihc_36.bin");
    let slide = TissueSlideMetadata {
        slide_id: "SL-2026-00142-05".into(),
        accession_number: "SP-2026-03421".into(),
        patient_id: "PT-88291".into(),
        specimen_type: SpecimenType::CoreNeedleBiopsy,
        tissue_site: "Liver, segment VII".into(),
        fixation_duration_hours: 18,
        block_id: "B2".into(),
        section_thickness_um_x10: 40,
        stain: StainType::Immunohistochemistry {
            antibody: "CK7".into(),
            clone_name: "OV-TL 12/30".into(),
        },
        created_timestamp: 1_773_840_600,
    };
    encode_to_file(&slide, &path).expect("encode_to_file TissueSlideMetadata IHC");
    let decoded: TissueSlideMetadata =
        decode_from_file(&path).expect("decode_from_file TissueSlideMetadata IHC");
    assert_eq!(slide, decoded);
    std::fs::remove_file(&path).expect("cleanup pathology_slide_ihc_36.bin");
}

/// 3. All stain type variants — vec roundtrip
#[test]
fn test_stain_type_all_variants_vec_roundtrip() {
    let stains = vec![
        StainType::HematoxylinAndEosin,
        StainType::Immunohistochemistry {
            antibody: "ER".into(),
            clone_name: "SP1".into(),
        },
        StainType::PeriodicAcidSchiff,
        StainType::Trichrome,
        StainType::GiemsaStain,
        StainType::SilverStain,
        StainType::MucicarmineStain,
        StainType::CongoRed,
        StainType::IronStainPrussianBlue,
        StainType::ElasticVanGieson,
    ];
    let bytes = encode_to_vec(&stains).expect("encode all StainType variants");
    let (decoded, consumed): (Vec<StainType>, usize) =
        decode_from_slice(&bytes).expect("decode all StainType variants");
    assert_eq!(stains, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 4. ScanResolution at 40x — vec roundtrip
#[test]
fn test_scan_resolution_40x_vec_roundtrip() {
    let scan = ScanResolution {
        scan_id: 500_001,
        slide_id: "SL-2026-00142-01".into(),
        magnification: ScanMagnification::X40,
        pixel_size_nm_x: 250,
        pixel_size_nm_y: 250,
        image_width_px: 120_000,
        image_height_px: 80_000,
        num_focal_planes: 5,
        compression_format: "JPEG2000".into(),
        file_size_bytes: 2_400_000_000,
        scanner_model: "Aperio GT 450 DX".into(),
    };
    let bytes = encode_to_vec(&scan).expect("encode ScanResolution 40x");
    let (decoded, consumed): (ScanResolution, usize) =
        decode_from_slice(&bytes).expect("decode ScanResolution 40x");
    assert_eq!(scan, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 5. ScanResolution file roundtrip — oil immersion 100x
#[test]
fn test_scan_resolution_100x_file_roundtrip() {
    let path = unique_tmp("scan_resolution_100x_36.bin");
    let scan = ScanResolution {
        scan_id: 500_099,
        slide_id: "SL-2026-00190-02".into(),
        magnification: ScanMagnification::X100Oil,
        pixel_size_nm_x: 65,
        pixel_size_nm_y: 65,
        image_width_px: 50_000,
        image_height_px: 50_000,
        num_focal_planes: 11,
        compression_format: "JPEG-XL".into(),
        file_size_bytes: 5_800_000_000,
        scanner_model: "Hamamatsu NanoZoomer S360MD".into(),
    };
    encode_to_file(&scan, &path).expect("encode_to_file ScanResolution 100x");
    let decoded: ScanResolution =
        decode_from_file(&path).expect("decode_from_file ScanResolution 100x");
    assert_eq!(scan, decoded);
    std::fs::remove_file(&path).expect("cleanup scan_resolution_100x_36.bin");
}

/// 6. RoiCoordinates — vec roundtrip
#[test]
fn test_roi_coordinates_vec_roundtrip() {
    let roi = RoiCoordinates {
        roi_id: 10_001,
        slide_id: "SL-2026-00142-01".into(),
        label: "Invasive front - hotspot".into(),
        top_left_x: 45_200,
        top_left_y: 31_800,
        width: 2048,
        height: 2048,
        magnification: ScanMagnification::X40,
        area_um2_x100: 26_214_400,
        annotator_id: "PATH-DR-1042".into(),
    };
    let bytes = encode_to_vec(&roi).expect("encode RoiCoordinates");
    let (decoded, consumed): (RoiCoordinates, usize) =
        decode_from_slice(&bytes).expect("decode RoiCoordinates");
    assert_eq!(roi, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 7. Multiple ROI regions — file roundtrip
#[test]
fn test_multiple_roi_regions_file_roundtrip() {
    let path = unique_tmp("multi_roi_36.bin");
    let rois: Vec<RoiCoordinates> = (0..8)
        .map(|i| RoiCoordinates {
            roi_id: 20_000 + i,
            slide_id: "SL-2026-00200-01".into(),
            label: format!("Region-{}", i + 1),
            top_left_x: 10_000 + i * 3_000,
            top_left_y: 5_000 + i * 2_000,
            width: 1024,
            height: 1024,
            magnification: ScanMagnification::X20,
            area_um2_x100: 104_857_600,
            annotator_id: "AI-MODEL-v3".into(),
        })
        .collect();
    encode_to_file(&rois, &path).expect("encode_to_file multi ROI");
    let decoded: Vec<RoiCoordinates> = decode_from_file(&path).expect("decode_from_file multi ROI");
    assert_eq!(rois, decoded);
    std::fs::remove_file(&path).expect("cleanup multi_roi_36.bin");
}

/// 8. CellMorphologyMeasurement — vec roundtrip
#[test]
fn test_cell_morphology_measurement_vec_roundtrip() {
    let cell = CellMorphologyMeasurement {
        cell_id: 1_000_000_001,
        roi_id: 10_001,
        nucleus_area_um2_x100: 4_520,
        cytoplasm_area_um2_x100: 12_300,
        nc_ratio_x1000: 367,
        nuclear_perimeter_um_x100: 2_380,
        nuclear_circularity_x1000: 820,
        chromatin_density_x1000: 650,
        mitotic_figure: false,
        pleomorphism_score: 2,
    };
    let bytes = encode_to_vec(&cell).expect("encode CellMorphologyMeasurement");
    let (decoded, consumed): (CellMorphologyMeasurement, usize) =
        decode_from_slice(&bytes).expect("decode CellMorphologyMeasurement");
    assert_eq!(cell, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 9. Batch of cell morphology measurements with mitotic figures — vec roundtrip
#[test]
fn test_cell_morphology_batch_with_mitotic_figures_vec_roundtrip() {
    let cells: Vec<CellMorphologyMeasurement> = (0..50)
        .map(|i| CellMorphologyMeasurement {
            cell_id: 2_000_000_000 + i as u64,
            roi_id: 10_005,
            nucleus_area_um2_x100: 3_800 + (i % 20) * 100,
            cytoplasm_area_um2_x100: 10_000 + (i % 30) * 200,
            nc_ratio_x1000: 300 + (i % 15) as u16 * 20,
            nuclear_perimeter_um_x100: 2_100 + (i % 10) * 50,
            nuclear_circularity_x1000: 700 + (i % 10) as u16 * 30,
            chromatin_density_x1000: 500 + (i % 8) as u16 * 50,
            mitotic_figure: i % 7 == 0,
            pleomorphism_score: (i % 4) as u8 + 1,
        })
        .collect();
    let bytes = encode_to_vec(&cells).expect("encode cell morphology batch");
    let (decoded, consumed): (Vec<CellMorphologyMeasurement>, usize) =
        decode_from_slice(&bytes).expect("decode cell morphology batch");
    assert_eq!(cells, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 10. DiagnosisRecord with malignant category — vec roundtrip
#[test]
fn test_diagnosis_record_malignant_vec_roundtrip() {
    let dx = DiagnosisRecord {
        record_id: 300_001,
        accession_number: "SP-2026-05678".into(),
        category: DiagnosisCategory::Malignant {
            icd_o_code: "8500/3".into(),
        },
        primary_site: "Breast, left, 2 o'clock".into(),
        histologic_type: "Invasive ductal carcinoma, NST".into(),
        tumor_grade: TumorGrade::Grade2ModeratelyDifferentiated,
        margin_status_positive: false,
        lymphovascular_invasion: true,
        perineural_invasion: false,
        pathologist_id: "DR-PATH-0042".into(),
        sign_out_timestamp: 1_773_926_400,
    };
    let bytes = encode_to_vec(&dx).expect("encode DiagnosisRecord malignant");
    let (decoded, consumed): (DiagnosisRecord, usize) =
        decode_from_slice(&bytes).expect("decode DiagnosisRecord malignant");
    assert_eq!(dx, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 11. DiagnosisRecord all categories — file roundtrip
#[test]
fn test_diagnosis_categories_file_roundtrip() {
    let path = unique_tmp("diagnosis_categories_36.bin");
    let categories = vec![
        DiagnosisCategory::Benign,
        DiagnosisCategory::Atypical,
        DiagnosisCategory::SuspiciousForMalignancy,
        DiagnosisCategory::Malignant {
            icd_o_code: "8140/3".into(),
        },
        DiagnosisCategory::Insufficient,
        DiagnosisCategory::DeferredToSpecialStudies,
    ];
    encode_to_file(&categories, &path).expect("encode_to_file diagnosis categories");
    let decoded: Vec<DiagnosisCategory> =
        decode_from_file(&path).expect("decode_from_file diagnosis categories");
    assert_eq!(categories, decoded);
    std::fs::remove_file(&path).expect("cleanup diagnosis_categories_36.bin");
}

/// 12. TumorGrade all variants — vec roundtrip
#[test]
fn test_tumor_grade_all_variants_vec_roundtrip() {
    let grades = vec![
        TumorGrade::GradeX,
        TumorGrade::Grade1WellDifferentiated,
        TumorGrade::Grade2ModeratelyDifferentiated,
        TumorGrade::Grade3PoorlyDifferentiated,
        TumorGrade::Grade4Undifferentiated,
    ];
    let bytes = encode_to_vec(&grades).expect("encode all TumorGrade variants");
    let (decoded, consumed): (Vec<TumorGrade>, usize) =
        decode_from_slice(&bytes).expect("decode all TumorGrade variants");
    assert_eq!(grades, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 13. BiomarkerPanel — breast cancer profile — vec roundtrip
#[test]
fn test_biomarker_panel_breast_cancer_vec_roundtrip() {
    let panel = BiomarkerPanel {
        panel_id: 7001,
        accession_number: "SP-2026-05678".into(),
        er_status: BiomarkerResult::StrongPositive { h_score: 280 },
        pr_status: BiomarkerResult::ModeratePositive { h_score: 150 },
        her2_status: BiomarkerResult::Equivocal,
        ki67_percent_x10: 220,
        p53_status: BiomarkerResult::WeakPositive { h_score: 40 },
        pdl1_tps_x10: 10,
        mismatch_repair_intact: true,
        report_timestamp: 1_773_930_000,
    };
    let bytes = encode_to_vec(&panel).expect("encode BiomarkerPanel breast");
    let (decoded, consumed): (BiomarkerPanel, usize) =
        decode_from_slice(&bytes).expect("decode BiomarkerPanel breast");
    assert_eq!(panel, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 14. BiomarkerPanel triple-negative profile — file roundtrip
#[test]
fn test_biomarker_panel_triple_negative_file_roundtrip() {
    let path = unique_tmp("biomarker_tnbc_36.bin");
    let panel = BiomarkerPanel {
        panel_id: 7055,
        accession_number: "SP-2026-06100".into(),
        er_status: BiomarkerResult::Negative,
        pr_status: BiomarkerResult::Negative,
        her2_status: BiomarkerResult::Negative,
        ki67_percent_x10: 750,
        p53_status: BiomarkerResult::StrongPositive { h_score: 300 },
        pdl1_tps_x10: 450,
        mismatch_repair_intact: false,
        report_timestamp: 1_773_933_600,
    };
    encode_to_file(&panel, &path).expect("encode_to_file BiomarkerPanel TNBC");
    let decoded: BiomarkerPanel =
        decode_from_file(&path).expect("decode_from_file BiomarkerPanel TNBC");
    assert_eq!(panel, decoded);
    std::fs::remove_file(&path).expect("cleanup biomarker_tnbc_36.bin");
}

/// 15. SlideAnnotation with polygon shape — vec roundtrip
#[test]
fn test_slide_annotation_polygon_vec_roundtrip() {
    let annotation = SlideAnnotation {
        annotation_id: 900_001,
        slide_id: "SL-2026-00142-01".into(),
        shape: AnnotationShape::Polygon {
            vertices_x: vec![100, 250, 400, 350, 150],
            vertices_y: vec![50, 30, 80, 200, 180],
        },
        label: "Tumor boundary".into(),
        color_rgba: 0xFF_00_00_80,
        layer_index: 0,
        author_id: "DR-PATH-0042".into(),
        created_timestamp: 1_773_940_000,
        comment: "Invasive carcinoma border demarcation".into(),
    };
    let bytes = encode_to_vec(&annotation).expect("encode SlideAnnotation polygon");
    let (decoded, consumed): (SlideAnnotation, usize) =
        decode_from_slice(&bytes).expect("decode SlideAnnotation polygon");
    assert_eq!(annotation, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 16. SlideAnnotation all shapes — file roundtrip
#[test]
fn test_slide_annotation_all_shapes_file_roundtrip() {
    let path = unique_tmp("annotations_all_shapes_36.bin");
    let annotations = vec![
        SlideAnnotation {
            annotation_id: 900_010,
            slide_id: "SL-2026-00300-01".into(),
            shape: AnnotationShape::Rectangle {
                x: 1000,
                y: 2000,
                width: 512,
                height: 512,
            },
            label: "Hotspot 1".into(),
            color_rgba: 0x00_FF_00_C0,
            layer_index: 1,
            author_id: "AI-MITOSIS-v2".into(),
            created_timestamp: 1_773_941_000,
            comment: "Mitotic hotspot region".into(),
        },
        SlideAnnotation {
            annotation_id: 900_011,
            slide_id: "SL-2026-00300-01".into(),
            shape: AnnotationShape::Ellipse {
                cx: 5000,
                cy: 3000,
                rx: 200,
                ry: 150,
            },
            label: "Necrotic area".into(),
            color_rgba: 0xFF_FF_00_A0,
            layer_index: 1,
            author_id: "DR-PATH-0042".into(),
            created_timestamp: 1_773_941_100,
            comment: "Central necrosis".into(),
        },
        SlideAnnotation {
            annotation_id: 900_012,
            slide_id: "SL-2026-00300-01".into(),
            shape: AnnotationShape::Arrow {
                x1: 7000,
                y1: 4000,
                x2: 7200,
                y2: 4100,
            },
            label: "Vascular invasion".into(),
            color_rgba: 0x00_00_FF_FF,
            layer_index: 2,
            author_id: "DR-PATH-0042".into(),
            created_timestamp: 1_773_941_200,
            comment: "Lymphovascular space invasion noted".into(),
        },
        SlideAnnotation {
            annotation_id: 900_013,
            slide_id: "SL-2026-00300-01".into(),
            shape: AnnotationShape::FreehandPath {
                points_x: vec![100, 110, 125, 140, 160, 185, 200],
                points_y: vec![200, 210, 205, 220, 230, 225, 240],
            },
            label: "Margin ink line".into(),
            color_rgba: 0x00_00_00_FF,
            layer_index: 0,
            author_id: "DR-PATH-0042".into(),
            created_timestamp: 1_773_941_300,
            comment: "Inked surgical margin trace".into(),
        },
    ];
    encode_to_file(&annotations, &path).expect("encode_to_file annotations all shapes");
    let decoded: Vec<SlideAnnotation> =
        decode_from_file(&path).expect("decode_from_file annotations all shapes");
    assert_eq!(annotations, decoded);
    std::fs::remove_file(&path).expect("cleanup annotations_all_shapes_36.bin");
}

/// 17. PathologyReportMeta — vec roundtrip
#[test]
fn test_pathology_report_meta_vec_roundtrip() {
    let report = PathologyReportMeta {
        report_id: 400_001,
        accession_number: "SP-2026-05678".into(),
        patient_name_hash: vec![0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89],
        specimen_count: 3,
        stain_count: 12,
        slide_count: 18,
        addendum_count: 1,
        turnaround_hours: 52,
        is_amended: true,
        synoptic_report: true,
        finalized_timestamp: 1_773_950_000,
    };
    let bytes = encode_to_vec(&report).expect("encode PathologyReportMeta");
    let (decoded, consumed): (PathologyReportMeta, usize) =
        decode_from_slice(&bytes).expect("decode PathologyReportMeta");
    assert_eq!(report, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 18. SpecimenTracking with full timestamps — file roundtrip
#[test]
fn test_specimen_tracking_full_timeline_file_roundtrip() {
    let path = unique_tmp("specimen_tracking_36.bin");
    let specimen = SpecimenTracking {
        specimen_id: "SPEC-2026-00891".into(),
        accession_number: "SP-2026-07200".into(),
        collection_timestamp: 1_773_800_000,
        received_timestamp: 1_773_803_600,
        grossing_timestamp: 1_773_810_000,
        embedding_timestamp: 1_773_830_000,
        sectioning_timestamp: 1_773_840_000,
        staining_timestamp: 1_773_845_000,
        scanning_timestamp: 1_773_850_000,
        fixative_type: "10% neutral buffered formalin".into(),
        cassette_count: 6,
        block_ids: vec![
            "A1".into(),
            "A2".into(),
            "B1".into(),
            "B2".into(),
            "C1".into(),
            "LN-1".into(),
        ],
    };
    encode_to_file(&specimen, &path).expect("encode_to_file SpecimenTracking");
    let decoded: SpecimenTracking =
        decode_from_file(&path).expect("decode_from_file SpecimenTracking");
    assert_eq!(specimen, decoded);
    std::fs::remove_file(&path).expect("cleanup specimen_tracking_36.bin");
}

/// 19. QualityControlMetric — passed status — vec roundtrip
#[test]
fn test_quality_control_metric_passed_vec_roundtrip() {
    let qc = QualityControlMetric {
        qc_id: 600_001,
        slide_id: "SL-2026-00142-01".into(),
        focus_score_x1000: 950,
        tissue_coverage_percent_x10: 875,
        stain_uniformity_x1000: 920,
        background_noise_x1000: 30,
        artifact_count: 2,
        pen_mark_detected: false,
        air_bubble_count: 0,
        fold_count: 1,
        status: QcStatus::Passed,
        reviewed_by: "QC-TECH-011".into(),
    };
    let bytes = encode_to_vec(&qc).expect("encode QualityControlMetric passed");
    let (decoded, consumed): (QualityControlMetric, usize) =
        decode_from_slice(&bytes).expect("decode QualityControlMetric passed");
    assert_eq!(qc, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 20. QualityControlMetric — failed with reason — file roundtrip
#[test]
fn test_quality_control_metric_failed_file_roundtrip() {
    let path = unique_tmp("qc_failed_36.bin");
    let qc = QualityControlMetric {
        qc_id: 600_088,
        slide_id: "SL-2026-00505-03".into(),
        focus_score_x1000: 320,
        tissue_coverage_percent_x10: 450,
        stain_uniformity_x1000: 400,
        background_noise_x1000: 280,
        artifact_count: 15,
        pen_mark_detected: true,
        air_bubble_count: 3,
        fold_count: 7,
        status: QcStatus::Failed {
            reason: "Severe tissue folding and out-of-focus regions exceeding 40% of scan area"
                .into(),
        },
        reviewed_by: "QC-TECH-022".into(),
    };
    encode_to_file(&qc, &path).expect("encode_to_file QualityControlMetric failed");
    let decoded: QualityControlMetric =
        decode_from_file(&path).expect("decode_from_file QualityControlMetric failed");
    assert_eq!(qc, decoded);
    std::fs::remove_file(&path).expect("cleanup qc_failed_36.bin");
}

/// 21. TumorMeasurement — vec roundtrip
#[test]
fn test_tumor_measurement_vec_roundtrip() {
    let measurement = TumorMeasurement {
        measurement_id: 800_001,
        accession_number: "SP-2026-05678".into(),
        greatest_dimension_mm_x10: 235,
        second_dimension_mm_x10: 180,
        third_dimension_mm_x10: 120,
        depth_of_invasion_mm_x10: 85,
        closest_margin_mm_x10: 22,
        margin_location: "Deep (posterior)".into(),
        necrosis_percent_x10: 150,
        viable_tumor_percent_x10: 780,
    };
    let bytes = encode_to_vec(&measurement).expect("encode TumorMeasurement");
    let (decoded, consumed): (TumorMeasurement, usize) =
        decode_from_slice(&bytes).expect("decode TumorMeasurement");
    assert_eq!(measurement, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 22. Full case composite — file roundtrip combining multiple domain types
#[test]
fn test_full_pathology_case_composite_file_roundtrip() {
    let path = unique_tmp("full_case_composite_36.bin");

    let case = (
        TissueSlideMetadata {
            slide_id: "SL-2026-09999-01".into(),
            accession_number: "SP-2026-09999".into(),
            patient_id: "PT-55001".into(),
            specimen_type: SpecimenType::ExcisionalBiopsy,
            tissue_site: "Colon, sigmoid".into(),
            fixation_duration_hours: 20,
            block_id: "A1".into(),
            section_thickness_um_x10: 40,
            stain: StainType::HematoxylinAndEosin,
            created_timestamp: 1_773_960_000,
        },
        DiagnosisRecord {
            record_id: 300_999,
            accession_number: "SP-2026-09999".into(),
            category: DiagnosisCategory::Malignant {
                icd_o_code: "8480/3".into(),
            },
            primary_site: "Sigmoid colon".into(),
            histologic_type: "Mucinous adenocarcinoma".into(),
            tumor_grade: TumorGrade::Grade3PoorlyDifferentiated,
            margin_status_positive: true,
            lymphovascular_invasion: true,
            perineural_invasion: true,
            pathologist_id: "DR-PATH-0099".into(),
            sign_out_timestamp: 1_773_970_000,
        },
        BiomarkerPanel {
            panel_id: 7999,
            accession_number: "SP-2026-09999".into(),
            er_status: BiomarkerResult::NotEvaluable,
            pr_status: BiomarkerResult::NotEvaluable,
            her2_status: BiomarkerResult::NotEvaluable,
            ki67_percent_x10: 600,
            p53_status: BiomarkerResult::Negative,
            pdl1_tps_x10: 200,
            mismatch_repair_intact: false,
            report_timestamp: 1_773_975_000,
        },
        QualityControlMetric {
            qc_id: 600_999,
            slide_id: "SL-2026-09999-01".into(),
            focus_score_x1000: 880,
            tissue_coverage_percent_x10: 920,
            stain_uniformity_x1000: 870,
            background_noise_x1000: 45,
            artifact_count: 3,
            pen_mark_detected: false,
            air_bubble_count: 0,
            fold_count: 2,
            status: QcStatus::ConditionalPass {
                reason: "Minor fold near tissue edge, acceptable for diagnosis".into(),
            },
            reviewed_by: "QC-TECH-005".into(),
        },
    );

    encode_to_file(&case, &path).expect("encode_to_file full case composite");
    let decoded: (
        TissueSlideMetadata,
        DiagnosisRecord,
        BiomarkerPanel,
        QualityControlMetric,
    ) = decode_from_file(&path).expect("decode_from_file full case composite");
    assert_eq!(case, decoded);
    std::fs::remove_file(&path).expect("cleanup full_case_composite_36.bin");
}
