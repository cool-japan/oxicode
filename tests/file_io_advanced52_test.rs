#![cfg(feature = "std")]
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

// ── Domain types: Optometry Practice & Vision Care Management ────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum EyeSide {
    OculusDexter,
    OculusSinister,
    OculusUterque,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LensMaterial {
    Cr39,
    Polycarbonate,
    Trivex,
    HighIndex160,
    HighIndex167,
    HighIndex174,
    Glass,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LensCoating {
    AntiReflective,
    BlueLightFilter,
    Photochromic,
    ScratchResistant,
    UvProtection,
    MirrorCoating,
    Hydrophobic,
    Oleophobic,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ContactLensType {
    SoftDaily,
    SoftBiweekly,
    SoftMonthly,
    RigidGasPermeable,
    Hybrid,
    Scleral,
    OrthokeratologyOvernight,
    Toric,
    Multifocal,
    CosmeticColored,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ColorVisionDeficiency {
    Normal,
    Protanopia,
    Protanomaly,
    Deuteranopia,
    Deuteranomaly,
    Tritanopia,
    Tritanomaly,
    Achromatopsia,
    BlueConeMonochromacy,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum VisualFieldPattern {
    Full24Dash2,
    Full30Dash2,
    Central10Dash2,
    FullField120,
    KineticGoldmann,
    FrequencyDoubling,
    ShortWavelengthAutomated,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OctScanType {
    MacularCube,
    OpticDiscCube,
    RetinalNerveFiberLayer,
    GanglionCellAnalysis,
    AnteriorSegment,
    WidefieldRetina,
    OctAngiography,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ProgressiveLensDesign {
    Standard,
    ShortCorridor,
    DigitalFreeform,
    OccupationalOffice,
    DriveWear,
    SportProgressive,
    CustomWavefront,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FrameMaterial {
    Titanium,
    BetaTitanium,
    StainlessSteel,
    Acetate,
    Nylon,
    TrGrilamid,
    WoodLaminate,
    CarbonFiber,
    Memory,
    Aluminum,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FrameShape {
    Rectangular,
    Round,
    Aviator,
    CatEye,
    Wayfarer,
    Oval,
    Rimless,
    SemiRimless,
    Wraparound,
    Geometric,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DiagnosisCode {
    Myopia,
    Hyperopia,
    Astigmatism,
    Presbyopia,
    Amblyopia,
    Strabismus,
    Glaucoma,
    CataractIncipient,
    CataractMature,
    MacularDegeneration,
    DiabeticRetinopathy,
    RetinalDetachment,
    DryEyeSyndrome,
    Keratoconus,
    Uveitis,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TonometryMethod {
    GoldmannApplanation,
    NonContactAirPuff,
    Icare,
    TonoPen,
    PascalDynamic,
    OcularResponseAnalyzer,
}

// ── Composite structs ────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct RefractionMeasurement {
    eye: EyeSide,
    sphere_diopters: f64,
    cylinder_diopters: f64,
    axis_degrees: u16,
    add_power: f64,
    prism_diopters: f64,
    prism_base_degrees: u16,
    vertex_distance_mm: f64,
    pupillary_distance_mm: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VisualAcuityRecord {
    patient_id: String,
    exam_date_epoch: u64,
    uncorrected_od: String,
    uncorrected_os: String,
    best_corrected_od: String,
    best_corrected_os: String,
    pinhole_od: String,
    pinhole_os: String,
    near_acuity_od: String,
    near_acuity_os: String,
    testing_distance_ft: f64,
    chart_type: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ContactLensPrescription {
    patient_id: String,
    lens_type: ContactLensType,
    brand: String,
    od_base_curve_mm: f64,
    od_diameter_mm: f64,
    od_sphere: f64,
    od_cylinder: f64,
    od_axis: u16,
    os_base_curve_mm: f64,
    os_diameter_mm: f64,
    os_sphere: f64,
    os_cylinder: f64,
    os_axis: u16,
    add_power: f64,
    dk_t_value: f64,
    water_content_pct: f64,
    expiration_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IntraocularPressureReading {
    patient_id: String,
    timestamp_epoch: u64,
    method: TonometryMethod,
    od_pressure_mmhg: f64,
    os_pressure_mmhg: f64,
    corneal_thickness_od_um: u16,
    corneal_thickness_os_um: u16,
    corrected_iop_od: f64,
    corrected_iop_os: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RetinalImagingMetadata {
    image_id: String,
    patient_id: String,
    capture_epoch: u64,
    eye: EyeSide,
    scan_type: OctScanType,
    resolution_x_px: u32,
    resolution_y_px: u32,
    depth_slices: u32,
    field_of_view_degrees: f64,
    signal_strength: u8,
    average_rnfl_thickness_um: f64,
    cup_to_disc_ratio: f64,
    foveal_thickness_um: f64,
    device_serial: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SpectacleFrame {
    sku: String,
    brand: String,
    model_name: String,
    material: FrameMaterial,
    shape: FrameShape,
    eye_size_mm: u8,
    bridge_size_mm: u8,
    temple_length_mm: u8,
    color: String,
    is_polarized_clip_compatible: bool,
    wholesale_cents: u64,
    retail_cents: u64,
    stock_qty: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FrameInventoryBatch {
    batch_id: String,
    received_epoch: u64,
    supplier: String,
    frames: Vec<SpectacleFrame>,
    total_wholesale_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProgressiveLensOrder {
    order_id: String,
    patient_id: String,
    design: ProgressiveLensDesign,
    material: LensMaterial,
    coatings: Vec<LensCoating>,
    od_sphere: f64,
    od_cylinder: f64,
    od_axis: u16,
    od_add: f64,
    os_sphere: f64,
    os_cylinder: f64,
    os_axis: u16,
    os_add: f64,
    seg_height_od_mm: f64,
    seg_height_os_mm: f64,
    corridor_length_mm: f64,
    fitting_cross_height_mm: f64,
    pantoscopic_tilt_deg: f64,
    face_form_angle_deg: f64,
    back_vertex_distance_mm: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColorVisionTestResult {
    patient_id: String,
    test_date_epoch: u64,
    test_name: String,
    plates_shown: u16,
    plates_correct: u16,
    diagnosis: ColorVisionDeficiency,
    confusion_axis_degrees: Option<u16>,
    severity_score: f64,
    notes: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PerimetryPoint {
    x_degrees: i16,
    y_degrees: i16,
    sensitivity_db: f64,
    is_seen: bool,
    response_time_ms: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VisualFieldExam {
    exam_id: String,
    patient_id: String,
    eye: EyeSide,
    pattern: VisualFieldPattern,
    stimulus_size_goldmann: u8,
    background_luminance_asb: f64,
    fixation_losses: u16,
    false_positive_pct: f64,
    false_negative_pct: f64,
    mean_deviation_db: f64,
    pattern_std_deviation_db: f64,
    visual_field_index_pct: f64,
    test_duration_seconds: u16,
    points: Vec<PerimetryPoint>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct KeratometryReading {
    patient_id: String,
    eye: EyeSide,
    flat_k_diopters: f64,
    flat_k_axis_degrees: u16,
    steep_k_diopters: f64,
    steep_k_axis_degrees: u16,
    average_k_diopters: f64,
    corneal_astigmatism_diopters: f64,
    sim_k_flat: f64,
    sim_k_steep: f64,
    eccentricity: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OctScanSession {
    session_id: String,
    patient_id: String,
    scans: Vec<RetinalImagingMetadata>,
    technician_id: String,
    quality_index: f64,
    total_scan_time_seconds: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PatientExamSummary {
    patient_id: String,
    exam_date_epoch: u64,
    examiner: String,
    chief_complaint: String,
    diagnoses: Vec<DiagnosisCode>,
    refraction_od: RefractionMeasurement,
    refraction_os: RefractionMeasurement,
    iop: IntraocularPressureReading,
    next_visit_epoch: u64,
    referral_needed: bool,
    referral_specialty: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GlaucomaFollowUp {
    patient_id: String,
    visit_epoch: u64,
    current_medications: Vec<String>,
    target_iop_mmhg: f64,
    measured_iop_od: f64,
    measured_iop_os: f64,
    tonometry_method: TonometryMethod,
    disc_photo_taken: bool,
    oct_performed: bool,
    visual_field_performed: bool,
    mean_deviation_od_db: f64,
    mean_deviation_os_db: f64,
    rnfl_od_um: f64,
    rnfl_os_um: f64,
    progression_detected: bool,
    management_change: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DryEyeAssessment {
    patient_id: String,
    assessment_epoch: u64,
    osdi_score: f64,
    tbut_od_seconds: f64,
    tbut_os_seconds: f64,
    schirmer_od_mm: f64,
    schirmer_os_mm: f64,
    meibomian_gland_expressibility_od: u8,
    meibomian_gland_expressibility_os: u8,
    corneal_staining_grade_od: u8,
    corneal_staining_grade_os: u8,
    osmolarity_od_mosm_l: f64,
    osmolarity_os_mosm_l: f64,
    mmp9_positive: bool,
    treatment_plan: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PediatricVisionScreening {
    child_id: String,
    screening_epoch: u64,
    age_months: u16,
    autorefraction_od_sphere: f64,
    autorefraction_od_cylinder: f64,
    autorefraction_od_axis: u16,
    autorefraction_os_sphere: f64,
    autorefraction_os_cylinder: f64,
    autorefraction_os_axis: u16,
    cover_test_result: String,
    stereo_acuity_arc_seconds: u32,
    color_vision_pass: bool,
    referral_recommended: bool,
    referral_reason: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TopographyMap {
    patient_id: String,
    eye: EyeSide,
    capture_epoch: u64,
    num_rings: u16,
    num_points_per_ring: u16,
    axial_power_diopters: Vec<f64>,
    tangential_power_diopters: Vec<f64>,
    elevation_front_um: Vec<f64>,
    elevation_back_um: Vec<f64>,
    pachymetry_center_um: u16,
    pachymetry_thinnest_um: u16,
    thinnest_location_x_mm: f64,
    thinnest_location_y_mm: f64,
    kmax_diopters: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LowVisionAid {
    aid_id: String,
    category: String,
    magnification_power: f64,
    working_distance_cm: f64,
    field_of_view_degrees: f64,
    illuminated: bool,
    weight_grams: u32,
    patient_trial_success: bool,
    notes: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LowVisionEvaluation {
    patient_id: String,
    eval_epoch: u64,
    distance_acuity_best: String,
    near_acuity_best: String,
    contrast_sensitivity_log: f64,
    reading_speed_wpm: u32,
    goal_tasks: Vec<String>,
    aids_trialed: Vec<LowVisionAid>,
    aids_prescribed: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OrthokeratologyFitting {
    patient_id: String,
    fitting_epoch: u64,
    baseline_sphere_od: f64,
    baseline_sphere_os: f64,
    flat_k_od: f64,
    flat_k_os: f64,
    steep_k_od: f64,
    steep_k_os: f64,
    hvid_od_mm: f64,
    hvid_os_mm: f64,
    lens_brand: String,
    lens_base_curve_od_mm: f64,
    lens_base_curve_os_mm: f64,
    lens_diameter_od_mm: f64,
    lens_diameter_os_mm: f64,
    target_refraction_od: f64,
    target_refraction_os: f64,
    overnight_hours: f64,
    post_topography_regular: bool,
}

// ── Tests ────────────────────────────────────────────────────────────────────

// Test 1: Refraction measurement roundtrip via file
#[test]
fn test_refraction_measurement_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_refraction_52.bin");
    let original = RefractionMeasurement {
        eye: EyeSide::OculusDexter,
        sphere_diopters: -3.25,
        cylinder_diopters: -1.50,
        axis_degrees: 175,
        add_power: 0.0,
        prism_diopters: 0.0,
        prism_base_degrees: 0,
        vertex_distance_mm: 12.0,
        pupillary_distance_mm: 31.5,
    };
    encode_to_file(&original, &path).expect("encode refraction to file");
    let decoded: RefractionMeasurement =
        decode_from_file(&path).expect("decode refraction from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 2: Visual acuity record via vec
#[test]
fn test_visual_acuity_record_vec_roundtrip() {
    let original = VisualAcuityRecord {
        patient_id: "PAT-20260315-001".into(),
        exam_date_epoch: 1773724800,
        uncorrected_od: "20/80".into(),
        uncorrected_os: "20/60".into(),
        best_corrected_od: "20/20".into(),
        best_corrected_os: "20/20".into(),
        pinhole_od: "20/25".into(),
        pinhole_os: "20/20".into(),
        near_acuity_od: "J3".into(),
        near_acuity_os: "J2".into(),
        testing_distance_ft: 20.0,
        chart_type: "ETDRS LogMAR".into(),
    };
    let encoded = encode_to_vec(&original).expect("encode visual acuity to vec");
    let (decoded, _): (VisualAcuityRecord, _) =
        decode_from_slice(&encoded).expect("decode visual acuity from slice");
    assert_eq!(original, decoded);
}

// Test 3: Contact lens prescription file roundtrip
#[test]
fn test_contact_lens_prescription_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_cl_rx_52.bin");
    let original = ContactLensPrescription {
        patient_id: "PAT-CL-4421".into(),
        lens_type: ContactLensType::Toric,
        brand: "Acuvue Oasys for Astigmatism".into(),
        od_base_curve_mm: 8.5,
        od_diameter_mm: 14.5,
        od_sphere: -2.75,
        od_cylinder: -1.25,
        od_axis: 180,
        os_base_curve_mm: 8.5,
        os_diameter_mm: 14.5,
        os_sphere: -3.00,
        os_cylinder: -0.75,
        os_axis: 10,
        add_power: 0.0,
        dk_t_value: 129.0,
        water_content_pct: 38.0,
        expiration_epoch: 1806220800,
    };
    encode_to_file(&original, &path).expect("encode CL prescription to file");
    let decoded: ContactLensPrescription =
        decode_from_file(&path).expect("decode CL prescription from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 4: Intraocular pressure reading via vec
#[test]
fn test_intraocular_pressure_vec_roundtrip() {
    let original = IntraocularPressureReading {
        patient_id: "PAT-GLAU-1122".into(),
        timestamp_epoch: 1773811200,
        method: TonometryMethod::GoldmannApplanation,
        od_pressure_mmhg: 18.0,
        os_pressure_mmhg: 16.0,
        corneal_thickness_od_um: 545,
        corneal_thickness_os_um: 548,
        corrected_iop_od: 17.2,
        corrected_iop_os: 15.3,
    };
    let encoded = encode_to_vec(&original).expect("encode IOP to vec");
    let (decoded, _): (IntraocularPressureReading, _) =
        decode_from_slice(&encoded).expect("decode IOP from slice");
    assert_eq!(original, decoded);
}

// Test 5: Retinal imaging metadata file roundtrip
#[test]
fn test_retinal_imaging_metadata_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_retinal_img_52.bin");
    let original = RetinalImagingMetadata {
        image_id: "IMG-OCT-20260315-003".into(),
        patient_id: "PAT-RET-7788".into(),
        capture_epoch: 1773724800,
        eye: EyeSide::OculusSinister,
        scan_type: OctScanType::MacularCube,
        resolution_x_px: 512,
        resolution_y_px: 128,
        depth_slices: 1024,
        field_of_view_degrees: 20.0,
        signal_strength: 8,
        average_rnfl_thickness_um: 98.5,
        cup_to_disc_ratio: 0.35,
        foveal_thickness_um: 262.0,
        device_serial: "ZEISS-CIRRUS-6000-SN42195".into(),
    };
    encode_to_file(&original, &path).expect("encode retinal imaging to file");
    let decoded: RetinalImagingMetadata =
        decode_from_file(&path).expect("decode retinal imaging from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 6: Spectacle frame inventory batch via vec
#[test]
fn test_frame_inventory_batch_vec_roundtrip() {
    let original = FrameInventoryBatch {
        batch_id: "BATCH-2026-Q1-007".into(),
        received_epoch: 1773638400,
        supplier: "Luxottica Distribution".into(),
        frames: vec![
            SpectacleFrame {
                sku: "RB-5154-2000-49".into(),
                brand: "Ray-Ban".into(),
                model_name: "Clubmaster".into(),
                material: FrameMaterial::Acetate,
                shape: FrameShape::Wayfarer,
                eye_size_mm: 49,
                bridge_size_mm: 21,
                temple_length_mm: 145,
                color: "Black / Gold".into(),
                is_polarized_clip_compatible: true,
                wholesale_cents: 8500,
                retail_cents: 19900,
                stock_qty: 6,
            },
            SpectacleFrame {
                sku: "OAK-OX5038-0153".into(),
                brand: "Oakley".into(),
                model_name: "Metal Plate".into(),
                material: FrameMaterial::Titanium,
                shape: FrameShape::Rectangular,
                eye_size_mm: 53,
                bridge_size_mm: 18,
                temple_length_mm: 140,
                color: "Pewter".into(),
                is_polarized_clip_compatible: false,
                wholesale_cents: 12000,
                retail_cents: 27500,
                stock_qty: 3,
            },
        ],
        total_wholesale_cents: 20500,
    };
    let encoded = encode_to_vec(&original).expect("encode frame batch to vec");
    let (decoded, _): (FrameInventoryBatch, _) =
        decode_from_slice(&encoded).expect("decode frame batch from slice");
    assert_eq!(original, decoded);
}

// Test 7: Progressive lens order file roundtrip
#[test]
fn test_progressive_lens_order_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_prog_lens_52.bin");
    let original = ProgressiveLensOrder {
        order_id: "ORD-PROG-88234".into(),
        patient_id: "PAT-PRES-5511".into(),
        design: ProgressiveLensDesign::DigitalFreeform,
        material: LensMaterial::HighIndex167,
        coatings: vec![
            LensCoating::AntiReflective,
            LensCoating::BlueLightFilter,
            LensCoating::Hydrophobic,
        ],
        od_sphere: -1.75,
        od_cylinder: -0.50,
        od_axis: 90,
        od_add: 2.00,
        os_sphere: -2.00,
        os_cylinder: -0.75,
        os_axis: 85,
        os_add: 2.00,
        seg_height_od_mm: 18.0,
        seg_height_os_mm: 17.5,
        corridor_length_mm: 14.0,
        fitting_cross_height_mm: 22.0,
        pantoscopic_tilt_deg: 10.0,
        face_form_angle_deg: 5.0,
        back_vertex_distance_mm: 13.5,
    };
    encode_to_file(&original, &path).expect("encode progressive lens order to file");
    let decoded: ProgressiveLensOrder =
        decode_from_file(&path).expect("decode progressive lens order from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 8: Color vision test result via vec
#[test]
fn test_color_vision_result_vec_roundtrip() {
    let original = ColorVisionTestResult {
        patient_id: "PAT-CV-3309".into(),
        test_date_epoch: 1773724800,
        test_name: "Ishihara 38-Plate".into(),
        plates_shown: 38,
        plates_correct: 31,
        diagnosis: ColorVisionDeficiency::Deuteranomaly,
        confusion_axis_degrees: Some(120),
        severity_score: 0.68,
        notes: "Mild red-green deficiency, struggles with plates 5-12 transformation designs"
            .into(),
    };
    let encoded = encode_to_vec(&original).expect("encode color vision result to vec");
    let (decoded, _): (ColorVisionTestResult, _) =
        decode_from_slice(&encoded).expect("decode color vision result from slice");
    assert_eq!(original, decoded);
}

// Test 9: Visual field perimetry exam file roundtrip
#[test]
fn test_visual_field_exam_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_vf_exam_52.bin");
    let points = vec![
        PerimetryPoint {
            x_degrees: -9,
            y_degrees: 9,
            sensitivity_db: 28.5,
            is_seen: true,
            response_time_ms: 320,
        },
        PerimetryPoint {
            x_degrees: -3,
            y_degrees: 9,
            sensitivity_db: 30.2,
            is_seen: true,
            response_time_ms: 280,
        },
        PerimetryPoint {
            x_degrees: 3,
            y_degrees: 9,
            sensitivity_db: 29.8,
            is_seen: true,
            response_time_ms: 310,
        },
        PerimetryPoint {
            x_degrees: 9,
            y_degrees: 9,
            sensitivity_db: 26.1,
            is_seen: true,
            response_time_ms: 450,
        },
        PerimetryPoint {
            x_degrees: -15,
            y_degrees: 3,
            sensitivity_db: 12.3,
            is_seen: true,
            response_time_ms: 680,
        },
        PerimetryPoint {
            x_degrees: -9,
            y_degrees: 3,
            sensitivity_db: 27.4,
            is_seen: true,
            response_time_ms: 340,
        },
        PerimetryPoint {
            x_degrees: 0,
            y_degrees: 0,
            sensitivity_db: 32.0,
            is_seen: true,
            response_time_ms: 250,
        },
        PerimetryPoint {
            x_degrees: 9,
            y_degrees: -9,
            sensitivity_db: 0.0,
            is_seen: false,
            response_time_ms: 0,
        },
    ];
    let original = VisualFieldExam {
        exam_id: "VF-HFA-20260315-001".into(),
        patient_id: "PAT-GLAU-1122".into(),
        eye: EyeSide::OculusDexter,
        pattern: VisualFieldPattern::Full24Dash2,
        stimulus_size_goldmann: 3,
        background_luminance_asb: 31.5,
        fixation_losses: 1,
        false_positive_pct: 2.0,
        false_negative_pct: 3.0,
        mean_deviation_db: -4.52,
        pattern_std_deviation_db: 3.87,
        visual_field_index_pct: 89.0,
        test_duration_seconds: 420,
        points,
    };
    encode_to_file(&original, &path).expect("encode visual field exam to file");
    let decoded: VisualFieldExam =
        decode_from_file(&path).expect("decode visual field exam from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 10: Keratometry reading via vec
#[test]
fn test_keratometry_reading_vec_roundtrip() {
    let original = KeratometryReading {
        patient_id: "PAT-KC-2204".into(),
        eye: EyeSide::OculusSinister,
        flat_k_diopters: 43.25,
        flat_k_axis_degrees: 180,
        steep_k_diopters: 44.50,
        steep_k_axis_degrees: 90,
        average_k_diopters: 43.875,
        corneal_astigmatism_diopters: 1.25,
        sim_k_flat: 43.12,
        sim_k_steep: 44.37,
        eccentricity: 0.48,
    };
    let encoded = encode_to_vec(&original).expect("encode keratometry to vec");
    let (decoded, _): (KeratometryReading, _) =
        decode_from_slice(&encoded).expect("decode keratometry from slice");
    assert_eq!(original, decoded);
}

// Test 11: OCT scan session file roundtrip
#[test]
fn test_oct_scan_session_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_oct_session_52.bin");
    let original = OctScanSession {
        session_id: "OCT-SES-20260315-004".into(),
        patient_id: "PAT-GLAU-1122".into(),
        scans: vec![
            RetinalImagingMetadata {
                image_id: "IMG-OCT-001".into(),
                patient_id: "PAT-GLAU-1122".into(),
                capture_epoch: 1773724800,
                eye: EyeSide::OculusDexter,
                scan_type: OctScanType::RetinalNerveFiberLayer,
                resolution_x_px: 768,
                resolution_y_px: 768,
                depth_slices: 512,
                field_of_view_degrees: 12.0,
                signal_strength: 9,
                average_rnfl_thickness_um: 85.3,
                cup_to_disc_ratio: 0.62,
                foveal_thickness_um: 258.0,
                device_serial: "TOPCON-MAESTRO2-SN8801".into(),
            },
            RetinalImagingMetadata {
                image_id: "IMG-OCT-002".into(),
                patient_id: "PAT-GLAU-1122".into(),
                capture_epoch: 1773724860,
                eye: EyeSide::OculusDexter,
                scan_type: OctScanType::GanglionCellAnalysis,
                resolution_x_px: 512,
                resolution_y_px: 128,
                depth_slices: 1024,
                field_of_view_degrees: 6.0,
                signal_strength: 7,
                average_rnfl_thickness_um: 0.0,
                cup_to_disc_ratio: 0.0,
                foveal_thickness_um: 310.0,
                device_serial: "TOPCON-MAESTRO2-SN8801".into(),
            },
        ],
        technician_id: "TECH-JM-042".into(),
        quality_index: 8.5,
        total_scan_time_seconds: 180,
    };
    encode_to_file(&original, &path).expect("encode OCT session to file");
    let decoded: OctScanSession = decode_from_file(&path).expect("decode OCT session from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 12: Patient exam summary via vec
#[test]
fn test_patient_exam_summary_vec_roundtrip() {
    let original = PatientExamSummary {
        patient_id: "PAT-COMP-9034".into(),
        exam_date_epoch: 1773724800,
        examiner: "Dr. Elaine Nakamura, OD".into(),
        chief_complaint: "Blurry distance vision, difficulty reading highway signs".into(),
        diagnoses: vec![DiagnosisCode::Myopia, DiagnosisCode::Astigmatism],
        refraction_od: RefractionMeasurement {
            eye: EyeSide::OculusDexter,
            sphere_diopters: -4.50,
            cylinder_diopters: -1.25,
            axis_degrees: 170,
            add_power: 0.0,
            prism_diopters: 0.0,
            prism_base_degrees: 0,
            vertex_distance_mm: 12.5,
            pupillary_distance_mm: 32.0,
        },
        refraction_os: RefractionMeasurement {
            eye: EyeSide::OculusSinister,
            sphere_diopters: -4.00,
            cylinder_diopters: -0.75,
            axis_degrees: 5,
            add_power: 0.0,
            prism_diopters: 0.0,
            prism_base_degrees: 0,
            vertex_distance_mm: 12.5,
            pupillary_distance_mm: 31.0,
        },
        iop: IntraocularPressureReading {
            patient_id: "PAT-COMP-9034".into(),
            timestamp_epoch: 1773724800,
            method: TonometryMethod::NonContactAirPuff,
            od_pressure_mmhg: 14.0,
            os_pressure_mmhg: 15.0,
            corneal_thickness_od_um: 555,
            corneal_thickness_os_um: 558,
            corrected_iop_od: 13.5,
            corrected_iop_os: 14.5,
        },
        next_visit_epoch: 1805260800,
        referral_needed: false,
        referral_specialty: String::new(),
    };
    let encoded = encode_to_vec(&original).expect("encode exam summary to vec");
    let (decoded, _): (PatientExamSummary, _) =
        decode_from_slice(&encoded).expect("decode exam summary from slice");
    assert_eq!(original, decoded);
}

// Test 13: Glaucoma follow-up file roundtrip
#[test]
fn test_glaucoma_followup_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_glaucoma_fu_52.bin");
    let original = GlaucomaFollowUp {
        patient_id: "PAT-GLAU-1122".into(),
        visit_epoch: 1773724800,
        current_medications: vec![
            "Latanoprost 0.005% OU qhs".into(),
            "Timolol 0.5% OU bid".into(),
        ],
        target_iop_mmhg: 14.0,
        measured_iop_od: 13.0,
        measured_iop_os: 12.0,
        tonometry_method: TonometryMethod::GoldmannApplanation,
        disc_photo_taken: true,
        oct_performed: true,
        visual_field_performed: true,
        mean_deviation_od_db: -5.12,
        mean_deviation_os_db: -3.85,
        rnfl_od_um: 82.5,
        rnfl_os_um: 91.2,
        progression_detected: false,
        management_change: "Continue current regimen, recheck in 4 months".into(),
    };
    encode_to_file(&original, &path).expect("encode glaucoma follow-up to file");
    let decoded: GlaucomaFollowUp =
        decode_from_file(&path).expect("decode glaucoma follow-up from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 14: Dry eye assessment via vec
#[test]
fn test_dry_eye_assessment_vec_roundtrip() {
    let original = DryEyeAssessment {
        patient_id: "PAT-DRY-6678".into(),
        assessment_epoch: 1773724800,
        osdi_score: 42.5,
        tbut_od_seconds: 4.2,
        tbut_os_seconds: 3.8,
        schirmer_od_mm: 8.0,
        schirmer_os_mm: 7.5,
        meibomian_gland_expressibility_od: 2,
        meibomian_gland_expressibility_os: 1,
        corneal_staining_grade_od: 2,
        corneal_staining_grade_os: 3,
        osmolarity_od_mosm_l: 312.0,
        osmolarity_os_mosm_l: 318.0,
        mmp9_positive: true,
        treatment_plan: vec![
            "Warm compresses bid x 10 min".into(),
            "Preservative-free artificial tears q2h".into(),
            "Omega-3 supplement 2g/day".into(),
            "Cyclosporine 0.05% OU bid".into(),
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode dry eye assessment to vec");
    let (decoded, _): (DryEyeAssessment, _) =
        decode_from_slice(&encoded).expect("decode dry eye assessment from slice");
    assert_eq!(original, decoded);
}

// Test 15: Pediatric vision screening file roundtrip
#[test]
fn test_pediatric_screening_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_ped_screen_52.bin");
    let original = PediatricVisionScreening {
        child_id: "PED-SCR-20260315-042".into(),
        screening_epoch: 1773724800,
        age_months: 72,
        autorefraction_od_sphere: 0.50,
        autorefraction_od_cylinder: -0.25,
        autorefraction_od_axis: 90,
        autorefraction_os_sphere: 0.75,
        autorefraction_os_cylinder: -0.50,
        autorefraction_os_axis: 85,
        cover_test_result: "Ortho at distance and near".into(),
        stereo_acuity_arc_seconds: 40,
        color_vision_pass: true,
        referral_recommended: false,
        referral_reason: String::new(),
    };
    encode_to_file(&original, &path).expect("encode pediatric screening to file");
    let decoded: PediatricVisionScreening =
        decode_from_file(&path).expect("decode pediatric screening from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 16: Topography map via vec
#[test]
fn test_topography_map_vec_roundtrip() {
    let original = TopographyMap {
        patient_id: "PAT-KC-2204".into(),
        eye: EyeSide::OculusDexter,
        capture_epoch: 1773724800,
        num_rings: 4,
        num_points_per_ring: 6,
        axial_power_diopters: vec![
            43.1, 43.2, 43.5, 43.8, 44.1, 44.3, 43.0, 43.3, 43.6, 43.9, 44.2, 44.4, 42.9, 43.1,
            43.4, 43.7, 44.0, 44.2, 42.8, 43.0, 43.3, 43.6, 43.9, 44.1,
        ],
        tangential_power_diopters: vec![
            43.5, 43.7, 44.0, 44.3, 44.6, 44.8, 43.4, 43.6, 43.9, 44.2, 44.5, 44.7, 43.3, 43.5,
            43.8, 44.1, 44.4, 44.6, 43.2, 43.4, 43.7, 44.0, 44.3, 44.5,
        ],
        elevation_front_um: vec![0.0, 2.1, 4.5, 6.2, 8.1, 3.3],
        elevation_back_um: vec![0.0, -1.8, -3.9, -5.4, -7.2, -2.7],
        pachymetry_center_um: 542,
        pachymetry_thinnest_um: 528,
        thinnest_location_x_mm: -0.4,
        thinnest_location_y_mm: -0.8,
        kmax_diopters: 44.8,
    };
    let encoded = encode_to_vec(&original).expect("encode topography map to vec");
    let (decoded, _): (TopographyMap, _) =
        decode_from_slice(&encoded).expect("decode topography map from slice");
    assert_eq!(original, decoded);
}

// Test 17: Low vision evaluation file roundtrip
#[test]
fn test_low_vision_evaluation_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_low_vision_52.bin");
    let original = LowVisionEvaluation {
        patient_id: "PAT-LV-0091".into(),
        eval_epoch: 1773724800,
        distance_acuity_best: "20/200".into(),
        near_acuity_best: "2M at 25cm".into(),
        contrast_sensitivity_log: 1.05,
        reading_speed_wpm: 45,
        goal_tasks: vec![
            "Read newspaper print".into(),
            "Recognize faces at 3 meters".into(),
            "Use smartphone independently".into(),
        ],
        aids_trialed: vec![
            LowVisionAid {
                aid_id: "LVA-HH-4X".into(),
                category: "Handheld magnifier".into(),
                magnification_power: 4.0,
                working_distance_cm: 6.25,
                field_of_view_degrees: 28.0,
                illuminated: true,
                weight_grams: 85,
                patient_trial_success: true,
                notes: "Patient comfortable with reading newspaper at slow pace".into(),
            },
            LowVisionAid {
                aid_id: "LVA-SPEC-3X".into(),
                category: "Spectacle-mounted microscope".into(),
                magnification_power: 3.0,
                working_distance_cm: 8.33,
                field_of_view_degrees: 15.0,
                illuminated: false,
                weight_grams: 42,
                patient_trial_success: false,
                notes: "Patient found working distance too short for comfort".into(),
            },
        ],
        aids_prescribed: vec!["LVA-HH-4X".into()],
    };
    encode_to_file(&original, &path).expect("encode low vision eval to file");
    let decoded: LowVisionEvaluation =
        decode_from_file(&path).expect("decode low vision eval from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 18: Orthokeratology fitting via vec
#[test]
fn test_orthokeratology_fitting_vec_roundtrip() {
    let original = OrthokeratologyFitting {
        patient_id: "PAT-ORTHO-1455".into(),
        fitting_epoch: 1773724800,
        baseline_sphere_od: -3.00,
        baseline_sphere_os: -2.75,
        flat_k_od: 43.50,
        flat_k_os: 43.25,
        steep_k_od: 44.75,
        steep_k_os: 44.50,
        hvid_od_mm: 11.8,
        hvid_os_mm: 11.7,
        lens_brand: "Paragon CRT".into(),
        lens_base_curve_od_mm: 8.60,
        lens_base_curve_os_mm: 8.65,
        lens_diameter_od_mm: 10.5,
        lens_diameter_os_mm: 10.5,
        target_refraction_od: 0.0,
        target_refraction_os: 0.0,
        overnight_hours: 8.0,
        post_topography_regular: true,
    };
    let encoded = encode_to_vec(&original).expect("encode ortho-k fitting to vec");
    let (decoded, _): (OrthokeratologyFitting, _) =
        decode_from_slice(&encoded).expect("decode ortho-k fitting from slice");
    assert_eq!(original, decoded);
}

// Test 19: Multiple refraction measurements (bilateral) file roundtrip
#[test]
fn test_bilateral_refraction_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_bilateral_rx_52.bin");
    let original: Vec<RefractionMeasurement> = vec![
        RefractionMeasurement {
            eye: EyeSide::OculusDexter,
            sphere_diopters: 2.00,
            cylinder_diopters: -0.75,
            axis_degrees: 95,
            add_power: 2.50,
            prism_diopters: 1.5,
            prism_base_degrees: 270,
            vertex_distance_mm: 13.0,
            pupillary_distance_mm: 33.0,
        },
        RefractionMeasurement {
            eye: EyeSide::OculusSinister,
            sphere_diopters: 1.75,
            cylinder_diopters: -1.00,
            axis_degrees: 80,
            add_power: 2.50,
            prism_diopters: 1.5,
            prism_base_degrees: 270,
            vertex_distance_mm: 13.0,
            pupillary_distance_mm: 30.5,
        },
    ];
    encode_to_file(&original, &path).expect("encode bilateral refraction to file");
    let decoded: Vec<RefractionMeasurement> =
        decode_from_file(&path).expect("decode bilateral refraction from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 20: Scleral contact lens with complex params via vec
#[test]
fn test_scleral_lens_prescription_vec_roundtrip() {
    let original = ContactLensPrescription {
        patient_id: "PAT-KC-2204".into(),
        lens_type: ContactLensType::Scleral,
        brand: "BostonSight PROSE".into(),
        od_base_curve_mm: 7.40,
        od_diameter_mm: 18.2,
        od_sphere: -6.50,
        od_cylinder: -3.75,
        od_axis: 55,
        os_base_curve_mm: 7.35,
        os_diameter_mm: 18.2,
        os_sphere: -5.25,
        os_cylinder: -4.00,
        os_axis: 125,
        add_power: 0.0,
        dk_t_value: 180.0,
        water_content_pct: 0.0,
        expiration_epoch: 1805260800,
    };
    let encoded = encode_to_vec(&original).expect("encode scleral lens to vec");
    let (decoded, _): (ContactLensPrescription, _) =
        decode_from_slice(&encoded).expect("decode scleral lens from slice");
    assert_eq!(original, decoded);
}

// Test 21: Color vision normal result file roundtrip
#[test]
fn test_color_vision_normal_file_roundtrip() {
    let path = temp_dir().join("oxicode_test_cv_normal_52.bin");
    let original = ColorVisionTestResult {
        patient_id: "PAT-CDL-7722".into(),
        test_date_epoch: 1773724800,
        test_name: "Farnsworth D-15 Arrangement".into(),
        plates_shown: 15,
        plates_correct: 15,
        diagnosis: ColorVisionDeficiency::Normal,
        confusion_axis_degrees: None,
        severity_score: 0.0,
        notes: "All caps arranged correctly, no crossings observed in plot".into(),
    };
    encode_to_file(&original, &path).expect("encode normal color vision to file");
    let decoded: ColorVisionTestResult =
        decode_from_file(&path).expect("decode normal color vision from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 22: Wide-angle perimetry with many points via vec
#[test]
fn test_wide_angle_perimetry_vec_roundtrip() {
    let mut points = Vec::new();
    for ring in 0..5_i16 {
        for angle_step in 0..8_i16 {
            let radius = (ring + 1) * 6;
            let angle_rad = (angle_step as f64) * std::f64::consts::FRAC_PI_4;
            let x = (radius as f64 * angle_rad.cos()) as i16;
            let y = (radius as f64 * angle_rad.sin()) as i16;
            let sensitivity = 32.0 - (ring as f64) * 3.5 - (angle_step as f64) * 0.2;
            points.push(PerimetryPoint {
                x_degrees: x,
                y_degrees: y,
                sensitivity_db: sensitivity,
                is_seen: sensitivity > 5.0,
                response_time_ms: (300 + ring as u16 * 40 + angle_step as u16 * 15),
            });
        }
    }
    let original = VisualFieldExam {
        exam_id: "VF-WIDE-20260315-009".into(),
        patient_id: "PAT-NEURO-4401".into(),
        eye: EyeSide::OculusSinister,
        pattern: VisualFieldPattern::FullField120,
        stimulus_size_goldmann: 5,
        background_luminance_asb: 31.5,
        fixation_losses: 3,
        false_positive_pct: 5.0,
        false_negative_pct: 8.0,
        mean_deviation_db: -12.34,
        pattern_std_deviation_db: 8.91,
        visual_field_index_pct: 62.0,
        test_duration_seconds: 780,
        points,
    };
    let encoded = encode_to_vec(&original).expect("encode wide-angle perimetry to vec");
    let (decoded, _): (VisualFieldExam, _) =
        decode_from_slice(&encoded).expect("decode wide-angle perimetry from slice");
    assert_eq!(original, decoded);
}
