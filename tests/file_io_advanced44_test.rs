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

// ── Domain Types: Optometry and Eye Care ────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RefractionMeasurement {
    eye: EyeSide,
    sphere: f64,
    cylinder: f64,
    axis: u16,
    add_power: Option<f64>,
    vertex_distance_mm: f64,
    pupillary_distance_mm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EyeSide {
    OD,
    OS,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KeratometryReading {
    eye: EyeSide,
    k1_diopters: f64,
    k1_axis: u16,
    k2_diopters: f64,
    k2_axis: u16,
    delta_k: f64,
    sim_k_avg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TonometerType {
    GoldmannApplanation,
    NonContactTonometry,
    ICare,
    TonoPen,
    Pneumotonometer,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IntraocularPressure {
    eye: EyeSide,
    pressure_mmhg: f64,
    tonometer: TonometerType,
    central_corneal_thickness_um: Option<u16>,
    corrected_iop: Option<f64>,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AcuityNotation {
    Snellen { numerator: u32, denominator: u32 },
    LogMAR(f64),
    Decimal(f64),
    ETDRS { letters_read: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VisualAcuityResult {
    eye: EyeSide,
    uncorrected: AcuityNotation,
    best_corrected: AcuityNotation,
    pinhole: Option<AcuityNotation>,
    near_acuity: Option<AcuityNotation>,
    test_distance_feet: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ContactLensMaterial {
    SoftHydrogel,
    SiliconHydrogel,
    RigidGasPermeable,
    HybridLens,
    ScleralLens,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContactLensPrescription {
    eye: EyeSide,
    material: ContactLensMaterial,
    base_curve_mm: f64,
    diameter_mm: f64,
    sphere: f64,
    cylinder: Option<f64>,
    axis: Option<u16>,
    add_power: Option<f64>,
    brand_name: String,
    replacement_schedule_days: u16,
    wearing_schedule: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OctRetinalLayer {
    layer_name: String,
    thickness_um: f64,
    normal_range_min_um: f64,
    normal_range_max_um: f64,
    is_within_normal: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OctScanResult {
    eye: EyeSide,
    scan_type: String,
    signal_strength: u8,
    central_subfield_thickness_um: f64,
    layers: Vec<OctRetinalLayer>,
    average_rnfl_thickness_um: f64,
    cup_disc_ratio: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VisualFieldPoint {
    x_degrees: f64,
    y_degrees: f64,
    threshold_db: f64,
    is_defect: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PerimetryResult {
    eye: EyeSide,
    test_strategy: String,
    mean_deviation_db: f64,
    pattern_standard_deviation_db: f64,
    fixation_losses_pct: f64,
    false_positive_pct: f64,
    false_negative_pct: f64,
    reliability_index: f64,
    test_points: Vec<VisualFieldPoint>,
    glaucoma_hemifield_test: String,
    visual_field_index_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SlitLampGrading {
    None,
    Trace,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlitLampFinding {
    eye: EyeSide,
    lids_lashes: String,
    conjunctiva: String,
    cornea_clarity: SlitLampGrading,
    corneal_staining: SlitLampGrading,
    anterior_chamber_depth: String,
    anterior_chamber_cells: SlitLampGrading,
    anterior_chamber_flare: SlitLampGrading,
    iris_description: String,
    lens_opacity: SlitLampGrading,
    lens_description: String,
    angle_assessment: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FundusPhotoMetadata {
    eye: EyeSide,
    camera_model: String,
    field_angle_degrees: u16,
    image_width_px: u32,
    image_height_px: u32,
    flash_intensity: u8,
    focus_depth: f64,
    optic_disc_cup_ratio: f64,
    macular_appearance: String,
    vessel_description: String,
    periphery_notes: String,
    capture_timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TopographyRing {
    ring_index: u8,
    radius_mm: f64,
    power_diopters: f64,
    eccentricity: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CornealTopography {
    eye: EyeSide,
    device: String,
    sim_k_flat: f64,
    sim_k_steep: f64,
    sim_k_axis: u16,
    corneal_astigmatism: f64,
    surface_regularity_index: f64,
    surface_asymmetry_index: f64,
    rings: Vec<TopographyRing>,
    irregularity_flag: bool,
    keratoconus_index: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TearFilmAssessment {
    eye: EyeSide,
    tbut_seconds: f64,
    schirmer_test_mm: f64,
    schirmer_with_anesthesia: bool,
    tear_meniscus_height_mm: f64,
    meibomian_gland_expressibility: SlitLampGrading,
    osmolarity_mosml: Option<f64>,
    phenol_red_thread_mm: Option<f64>,
    dry_eye_severity: SlitLampGrading,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PediatricVisionScreening {
    patient_age_months: u16,
    screening_method: String,
    fixation_preference: String,
    cover_test_result: String,
    stereo_acuity_arc_seconds: Option<u32>,
    color_vision_result: Option<String>,
    cycloplegic_refraction: Option<RefractionMeasurement>,
    amblyopia_risk: bool,
    strabismus_detected: bool,
    referral_recommended: bool,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SurgeryType {
    LASIK,
    PRK,
    SMILE,
    ICL,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SurgicalPlanning {
    eye: EyeSide,
    surgery_type: SurgeryType,
    target_refraction: f64,
    optical_zone_mm: f64,
    transition_zone_mm: f64,
    flap_thickness_um: Option<u16>,
    flap_diameter_mm: Option<f64>,
    hinge_position: Option<String>,
    ablation_depth_um: f64,
    residual_stromal_bed_um: u16,
    pachymetry_thinnest_um: u16,
    pupil_size_scotopic_mm: f64,
    wavefront_guided: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectacleFrame {
    frame_brand: String,
    model: String,
    eye_size_mm: f64,
    bridge_size_mm: f64,
    temple_length_mm: f64,
    frame_pd_mm: f64,
    vertex_distance_mm: f64,
    pantoscopic_tilt_deg: f64,
    face_form_wrap_deg: f64,
    material: String,
    color: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AuthorizationStatus {
    Pending,
    Approved,
    Denied,
    AppealInProgress,
    Expired,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsuranceAuthorization {
    authorization_number: String,
    patient_id: String,
    procedure_code: String,
    diagnosis_codes: Vec<String>,
    status: AuthorizationStatus,
    requested_date_epoch: u64,
    expiration_date_epoch: Option<u64>,
    approved_units: Option<u16>,
    copay_amount_cents: Option<u32>,
    provider_npi: String,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ComprehensiveExam {
    patient_id: String,
    exam_date_epoch: u64,
    refraction_od: RefractionMeasurement,
    refraction_os: RefractionMeasurement,
    acuity_od: VisualAcuityResult,
    acuity_os: VisualAcuityResult,
    iop_od: IntraocularPressure,
    iop_os: IntraocularPressure,
    slit_lamp_od: SlitLampFinding,
    slit_lamp_os: SlitLampFinding,
    assessment: String,
    plan: String,
}

// ── Helper constructors ─────────────────────────────────────────────────────

fn sample_refraction(eye: EyeSide) -> RefractionMeasurement {
    RefractionMeasurement {
        eye,
        sphere: -2.50,
        cylinder: -0.75,
        axis: 180,
        add_power: Some(1.50),
        vertex_distance_mm: 12.0,
        pupillary_distance_mm: 31.5,
    }
}

fn sample_keratometry(eye: EyeSide) -> KeratometryReading {
    KeratometryReading {
        eye,
        k1_diopters: 43.25,
        k1_axis: 5,
        k2_diopters: 44.00,
        k2_axis: 95,
        delta_k: 0.75,
        sim_k_avg: 43.625,
    }
}

fn sample_iop(eye: EyeSide) -> IntraocularPressure {
    IntraocularPressure {
        eye,
        pressure_mmhg: 16.0,
        tonometer: TonometerType::GoldmannApplanation,
        central_corneal_thickness_um: Some(545),
        corrected_iop: Some(15.5),
        timestamp_epoch: 1710500000,
    }
}

fn sample_acuity(eye: EyeSide) -> VisualAcuityResult {
    VisualAcuityResult {
        eye,
        uncorrected: AcuityNotation::Snellen {
            numerator: 20,
            denominator: 40,
        },
        best_corrected: AcuityNotation::Snellen {
            numerator: 20,
            denominator: 20,
        },
        pinhole: Some(AcuityNotation::Snellen {
            numerator: 20,
            denominator: 25,
        }),
        near_acuity: Some(AcuityNotation::LogMAR(0.0)),
        test_distance_feet: 20.0,
    }
}

fn sample_slit_lamp(eye: EyeSide) -> SlitLampFinding {
    SlitLampFinding {
        eye,
        lids_lashes: "Normal, no lesions".to_string(),
        conjunctiva: "White and quiet".to_string(),
        cornea_clarity: SlitLampGrading::None,
        corneal_staining: SlitLampGrading::None,
        anterior_chamber_depth: "Deep and quiet".to_string(),
        anterior_chamber_cells: SlitLampGrading::None,
        anterior_chamber_flare: SlitLampGrading::None,
        iris_description: "Flat, no synechiae".to_string(),
        lens_opacity: SlitLampGrading::Trace,
        lens_description: "Early nuclear sclerosis".to_string(),
        angle_assessment: "Grade IV open".to_string(),
    }
}

fn unique_path(suffix: &str) -> std::path::PathBuf {
    temp_dir().join(format!(
        "oxicode_fio44_{}_{}.bin",
        suffix,
        std::process::id()
    ))
}

// ── Test 1: Refraction measurement roundtrip via file ───────────────────────

#[test]
fn test_refraction_measurement_file_roundtrip() {
    let path = unique_path("t01");
    let original = RefractionMeasurement {
        eye: EyeSide::OD,
        sphere: -3.25,
        cylinder: -1.50,
        axis: 175,
        add_power: Some(2.00),
        vertex_distance_mm: 12.0,
        pupillary_distance_mm: 32.0,
    };
    encode_to_file(&original, &path).expect("encode refraction to file");
    let decoded: RefractionMeasurement =
        decode_from_file(&path).expect("decode refraction from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 2: Keratometry readings via slice ──────────────────────────────────

#[test]
fn test_keratometry_readings_slice_roundtrip() {
    let readings = vec![
        sample_keratometry(EyeSide::OD),
        KeratometryReading {
            eye: EyeSide::OS,
            k1_diopters: 43.50,
            k1_axis: 10,
            k2_diopters: 44.25,
            k2_axis: 100,
            delta_k: 0.75,
            sim_k_avg: 43.875,
        },
    ];
    let bytes = encode_to_vec(&readings).expect("encode keratometry vec");
    let (decoded, _): (Vec<KeratometryReading>, _) =
        decode_from_slice(&bytes).expect("decode keratometry vec");
    assert_eq!(readings, decoded);
}

// ── Test 3: Intraocular pressure Goldmann vs NCT via file ───────────────────

#[test]
fn test_intraocular_pressure_file_roundtrip() {
    let path = unique_path("t03");
    let measurements = vec![
        sample_iop(EyeSide::OD),
        IntraocularPressure {
            eye: EyeSide::OS,
            pressure_mmhg: 18.0,
            tonometer: TonometerType::NonContactTonometry,
            central_corneal_thickness_um: Some(560),
            corrected_iop: Some(17.0),
            timestamp_epoch: 1710500100,
        },
        IntraocularPressure {
            eye: EyeSide::OD,
            pressure_mmhg: 14.0,
            tonometer: TonometerType::ICare,
            central_corneal_thickness_um: None,
            corrected_iop: None,
            timestamp_epoch: 1710500200,
        },
    ];
    encode_to_file(&measurements, &path).expect("encode IOP to file");
    let decoded: Vec<IntraocularPressure> = decode_from_file(&path).expect("decode IOP from file");
    assert_eq!(measurements, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 4: Visual acuity with multiple notations via slice ─────────────────

#[test]
fn test_visual_acuity_multiple_notations_slice() {
    let results = vec![
        VisualAcuityResult {
            eye: EyeSide::OD,
            uncorrected: AcuityNotation::Snellen {
                numerator: 20,
                denominator: 200,
            },
            best_corrected: AcuityNotation::Snellen {
                numerator: 20,
                denominator: 20,
            },
            pinhole: None,
            near_acuity: Some(AcuityNotation::LogMAR(0.0)),
            test_distance_feet: 20.0,
        },
        VisualAcuityResult {
            eye: EyeSide::OS,
            uncorrected: AcuityNotation::ETDRS { letters_read: 55 },
            best_corrected: AcuityNotation::Decimal(1.0),
            pinhole: Some(AcuityNotation::LogMAR(-0.1)),
            near_acuity: None,
            test_distance_feet: 4.0,
        },
    ];
    let bytes = encode_to_vec(&results).expect("encode acuity");
    let (decoded, _): (Vec<VisualAcuityResult>, _) =
        decode_from_slice(&bytes).expect("decode acuity");
    assert_eq!(results, decoded);
}

// ── Test 5: Contact lens prescription via file ──────────────────────────────

#[test]
fn test_contact_lens_prescription_file() {
    let path = unique_path("t05");
    let rx = ContactLensPrescription {
        eye: EyeSide::OD,
        material: ContactLensMaterial::SiliconHydrogel,
        base_curve_mm: 8.6,
        diameter_mm: 14.2,
        sphere: -2.75,
        cylinder: Some(-0.75),
        axis: Some(180),
        add_power: None,
        brand_name: "AirOptix Plus HydraGlyde".to_string(),
        replacement_schedule_days: 30,
        wearing_schedule: "Daily wear, 16 hours max".to_string(),
    };
    encode_to_file(&rx, &path).expect("encode CL rx to file");
    let decoded: ContactLensPrescription = decode_from_file(&path).expect("decode CL rx from file");
    assert_eq!(rx, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 6: OCT retinal scan results via slice ──────────────────────────────

#[test]
fn test_oct_retinal_scan_slice_roundtrip() {
    let scan = OctScanResult {
        eye: EyeSide::OD,
        scan_type: "Macular Cube 512x128".to_string(),
        signal_strength: 8,
        central_subfield_thickness_um: 262.0,
        layers: vec![
            OctRetinalLayer {
                layer_name: "RNFL".to_string(),
                thickness_um: 95.0,
                normal_range_min_um: 80.0,
                normal_range_max_um: 110.0,
                is_within_normal: true,
            },
            OctRetinalLayer {
                layer_name: "GCL+IPL".to_string(),
                thickness_um: 78.0,
                normal_range_min_um: 70.0,
                normal_range_max_um: 95.0,
                is_within_normal: true,
            },
            OctRetinalLayer {
                layer_name: "INL".to_string(),
                thickness_um: 36.0,
                normal_range_min_um: 30.0,
                normal_range_max_um: 42.0,
                is_within_normal: true,
            },
            OctRetinalLayer {
                layer_name: "RPE".to_string(),
                thickness_um: 14.0,
                normal_range_min_um: 10.0,
                normal_range_max_um: 18.0,
                is_within_normal: true,
            },
        ],
        average_rnfl_thickness_um: 95.0,
        cup_disc_ratio: 0.35,
    };
    let bytes = encode_to_vec(&scan).expect("encode OCT scan");
    let (decoded, _): (OctScanResult, _) = decode_from_slice(&bytes).expect("decode OCT scan");
    assert_eq!(scan, decoded);
}

// ── Test 7: Visual field perimetry results via file ─────────────────────────

#[test]
fn test_visual_field_perimetry_file_roundtrip() {
    let path = unique_path("t07");
    let test_points: Vec<VisualFieldPoint> = (0..54)
        .map(|i| {
            let x = ((i % 9) as f64 - 4.0) * 6.0;
            let y = ((i / 9) as f64 - 3.0) * 6.0;
            VisualFieldPoint {
                x_degrees: x,
                y_degrees: y,
                threshold_db: 28.0 - (x.abs() + y.abs()) * 0.3,
                is_defect: (x.abs() + y.abs()) > 20.0,
            }
        })
        .collect();

    let result = PerimetryResult {
        eye: EyeSide::OS,
        test_strategy: "SITA Standard 24-2".to_string(),
        mean_deviation_db: -2.5,
        pattern_standard_deviation_db: 1.8,
        fixation_losses_pct: 5.0,
        false_positive_pct: 2.0,
        false_negative_pct: 3.0,
        reliability_index: 0.95,
        test_points,
        glaucoma_hemifield_test: "Within Normal Limits".to_string(),
        visual_field_index_pct: 97.0,
    };
    encode_to_file(&result, &path).expect("encode perimetry to file");
    let decoded: PerimetryResult = decode_from_file(&path).expect("decode perimetry from file");
    assert_eq!(result, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 8: Slit lamp findings with various gradings via slice ──────────────

#[test]
fn test_slit_lamp_findings_slice_roundtrip() {
    let findings = vec![
        sample_slit_lamp(EyeSide::OD),
        SlitLampFinding {
            eye: EyeSide::OS,
            lids_lashes: "Mild blepharitis".to_string(),
            conjunctiva: "Trace injection nasally".to_string(),
            cornea_clarity: SlitLampGrading::Mild,
            corneal_staining: SlitLampGrading::Moderate,
            anterior_chamber_depth: "Normal depth".to_string(),
            anterior_chamber_cells: SlitLampGrading::Trace,
            anterior_chamber_flare: SlitLampGrading::None,
            iris_description: "Round, reactive".to_string(),
            lens_opacity: SlitLampGrading::Moderate,
            lens_description: "Cortical spoking inferiorly".to_string(),
            angle_assessment: "Grade III open".to_string(),
        },
    ];
    let bytes = encode_to_vec(&findings).expect("encode slit lamp findings");
    let (decoded, _): (Vec<SlitLampFinding>, _) =
        decode_from_slice(&bytes).expect("decode slit lamp findings");
    assert_eq!(findings, decoded);
}

// ── Test 9: Fundus photography metadata via file ────────────────────────────

#[test]
fn test_fundus_photo_metadata_file_roundtrip() {
    let path = unique_path("t09");
    let photo = FundusPhotoMetadata {
        eye: EyeSide::OD,
        camera_model: "Topcon TRC-NW400".to_string(),
        field_angle_degrees: 45,
        image_width_px: 4288,
        image_height_px: 2848,
        flash_intensity: 7,
        focus_depth: 0.5,
        optic_disc_cup_ratio: 0.3,
        macular_appearance: "Foveal reflex present, no drusen".to_string(),
        vessel_description: "AV ratio 2:3, no crossing changes".to_string(),
        periphery_notes: "No holes, tears, or detachments".to_string(),
        capture_timestamp_epoch: 1710501234,
    };
    encode_to_file(&photo, &path).expect("encode fundus metadata to file");
    let decoded: FundusPhotoMetadata =
        decode_from_file(&path).expect("decode fundus metadata from file");
    assert_eq!(photo, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 10: Corneal topography map via slice ───────────────────────────────

#[test]
fn test_corneal_topography_slice_roundtrip() {
    let topo = CornealTopography {
        eye: EyeSide::OS,
        device: "Pentacam HR".to_string(),
        sim_k_flat: 42.75,
        sim_k_steep: 44.00,
        sim_k_axis: 90,
        corneal_astigmatism: 1.25,
        surface_regularity_index: 0.32,
        surface_asymmetry_index: 0.28,
        rings: vec![
            TopographyRing {
                ring_index: 1,
                radius_mm: 1.5,
                power_diopters: 43.50,
                eccentricity: 0.45,
            },
            TopographyRing {
                ring_index: 2,
                radius_mm: 2.5,
                power_diopters: 43.25,
                eccentricity: 0.48,
            },
            TopographyRing {
                ring_index: 3,
                radius_mm: 3.5,
                power_diopters: 42.80,
                eccentricity: 0.52,
            },
        ],
        irregularity_flag: false,
        keratoconus_index: 1.02,
    };
    let bytes = encode_to_vec(&topo).expect("encode topography");
    let (decoded, _): (CornealTopography, _) =
        decode_from_slice(&bytes).expect("decode topography");
    assert_eq!(topo, decoded);
}

// ── Test 11: Tear film assessment via file ──────────────────────────────────

#[test]
fn test_tear_film_assessment_file_roundtrip() {
    let path = unique_path("t11");
    let assessment = TearFilmAssessment {
        eye: EyeSide::OD,
        tbut_seconds: 6.5,
        schirmer_test_mm: 8.0,
        schirmer_with_anesthesia: false,
        tear_meniscus_height_mm: 0.25,
        meibomian_gland_expressibility: SlitLampGrading::Mild,
        osmolarity_mosml: Some(312.0),
        phenol_red_thread_mm: Some(15.0),
        dry_eye_severity: SlitLampGrading::Moderate,
    };
    encode_to_file(&assessment, &path).expect("encode tear film to file");
    let decoded: TearFilmAssessment = decode_from_file(&path).expect("decode tear film from file");
    assert_eq!(assessment, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 12: Pediatric vision screening via slice ───────────────────────────

#[test]
fn test_pediatric_screening_slice_roundtrip() {
    let screening = PediatricVisionScreening {
        patient_age_months: 36,
        screening_method: "Lea Symbols".to_string(),
        fixation_preference: "Central, steady, maintained OU".to_string(),
        cover_test_result: "Ortho at distance and near".to_string(),
        stereo_acuity_arc_seconds: Some(60),
        color_vision_result: Some("Pass - Ishihara 14/14".to_string()),
        cycloplegic_refraction: Some(RefractionMeasurement {
            eye: EyeSide::OD,
            sphere: 1.00,
            cylinder: -0.50,
            axis: 90,
            add_power: None,
            vertex_distance_mm: 12.0,
            pupillary_distance_mm: 25.0,
        }),
        amblyopia_risk: false,
        strabismus_detected: false,
        referral_recommended: false,
        notes: "Age-appropriate visual development".to_string(),
    };
    let bytes = encode_to_vec(&screening).expect("encode pediatric screening");
    let (decoded, _): (PediatricVisionScreening, _) =
        decode_from_slice(&bytes).expect("decode pediatric screening");
    assert_eq!(screening, decoded);
}

// ── Test 13: LASIK surgical planning via file ───────────────────────────────

#[test]
fn test_lasik_surgical_planning_file_roundtrip() {
    let path = unique_path("t13");
    let plan = SurgicalPlanning {
        eye: EyeSide::OD,
        surgery_type: SurgeryType::LASIK,
        target_refraction: -0.25,
        optical_zone_mm: 6.5,
        transition_zone_mm: 8.0,
        flap_thickness_um: Some(110),
        flap_diameter_mm: Some(8.5),
        hinge_position: Some("Superior".to_string()),
        ablation_depth_um: 65.0,
        residual_stromal_bed_um: 325,
        pachymetry_thinnest_um: 540,
        pupil_size_scotopic_mm: 6.8,
        wavefront_guided: true,
    };
    encode_to_file(&plan, &path).expect("encode surgical plan to file");
    let decoded: SurgicalPlanning =
        decode_from_file(&path).expect("decode surgical plan from file");
    assert_eq!(plan, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 14: PRK planning without flap via slice ────────────────────────────

#[test]
fn test_prk_planning_no_flap_slice_roundtrip() {
    let plan = SurgicalPlanning {
        eye: EyeSide::OS,
        surgery_type: SurgeryType::PRK,
        target_refraction: 0.0,
        optical_zone_mm: 6.0,
        transition_zone_mm: 8.5,
        flap_thickness_um: None,
        flap_diameter_mm: None,
        hinge_position: None,
        ablation_depth_um: 72.0,
        residual_stromal_bed_um: 420,
        pachymetry_thinnest_um: 520,
        pupil_size_scotopic_mm: 5.9,
        wavefront_guided: false,
    };
    let bytes = encode_to_vec(&plan).expect("encode PRK plan");
    let (decoded, _): (SurgicalPlanning, _) = decode_from_slice(&bytes).expect("decode PRK plan");
    assert_eq!(plan, decoded);
}

// ── Test 15: Spectacle frame measurements via file ──────────────────────────

#[test]
fn test_spectacle_frame_measurements_file() {
    let path = unique_path("t15");
    let frame = SpectacleFrame {
        frame_brand: "Lindberg".to_string(),
        model: "Spirit T205".to_string(),
        eye_size_mm: 52.0,
        bridge_size_mm: 18.0,
        temple_length_mm: 140.0,
        frame_pd_mm: 70.0,
        vertex_distance_mm: 12.5,
        pantoscopic_tilt_deg: 8.0,
        face_form_wrap_deg: 5.0,
        material: "Titanium".to_string(),
        color: "Matte black".to_string(),
    };
    encode_to_file(&frame, &path).expect("encode frame to file");
    let decoded: SpectacleFrame = decode_from_file(&path).expect("decode frame from file");
    assert_eq!(frame, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 16: Insurance authorization records via slice ──────────────────────

#[test]
fn test_insurance_authorization_slice_roundtrip() {
    let auth = InsuranceAuthorization {
        authorization_number: "AUTH-2024-09876".to_string(),
        patient_id: "PT-55432".to_string(),
        procedure_code: "92004".to_string(),
        diagnosis_codes: vec![
            "H40.1111".to_string(),
            "H40.1121".to_string(),
            "H35.31".to_string(),
        ],
        status: AuthorizationStatus::Approved,
        requested_date_epoch: 1710400000,
        expiration_date_epoch: Some(1718176000),
        approved_units: Some(1),
        copay_amount_cents: Some(4000),
        provider_npi: "1234567890".to_string(),
        notes: "Approved for comprehensive exam with OCT imaging".to_string(),
    };
    let bytes = encode_to_vec(&auth).expect("encode insurance auth");
    let (decoded, _): (InsuranceAuthorization, _) =
        decode_from_slice(&bytes).expect("decode insurance auth");
    assert_eq!(auth, decoded);
}

// ── Test 17: Multiple contact lens materials via file ───────────────────────

#[test]
fn test_multiple_cl_materials_file_roundtrip() {
    let path = unique_path("t17");
    let prescriptions = vec![
        ContactLensPrescription {
            eye: EyeSide::OD,
            material: ContactLensMaterial::RigidGasPermeable,
            base_curve_mm: 7.8,
            diameter_mm: 9.5,
            sphere: -4.00,
            cylinder: None,
            axis: None,
            add_power: None,
            brand_name: "Boston ES".to_string(),
            replacement_schedule_days: 365,
            wearing_schedule: "Daily wear only".to_string(),
        },
        ContactLensPrescription {
            eye: EyeSide::OS,
            material: ContactLensMaterial::ScleralLens,
            base_curve_mm: 7.4,
            diameter_mm: 16.5,
            sphere: -6.50,
            cylinder: Some(-2.25),
            axis: Some(170),
            add_power: None,
            brand_name: "Custom scleral".to_string(),
            replacement_schedule_days: 730,
            wearing_schedule: "Up to 14 hours daily".to_string(),
        },
        ContactLensPrescription {
            eye: EyeSide::OD,
            material: ContactLensMaterial::HybridLens,
            base_curve_mm: 8.0,
            diameter_mm: 14.5,
            sphere: -3.00,
            cylinder: Some(-1.75),
            axis: Some(90),
            add_power: Some(1.50),
            brand_name: "SynergEyes Duette".to_string(),
            replacement_schedule_days: 180,
            wearing_schedule: "Daily wear, 12 hours".to_string(),
        },
    ];
    encode_to_file(&prescriptions, &path).expect("encode CL prescriptions to file");
    let decoded: Vec<ContactLensPrescription> =
        decode_from_file(&path).expect("decode CL prescriptions from file");
    assert_eq!(prescriptions, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 18: OCT with abnormal findings via file ────────────────────────────

#[test]
fn test_oct_abnormal_findings_file_roundtrip() {
    let path = unique_path("t18");
    let scan = OctScanResult {
        eye: EyeSide::OS,
        scan_type: "Optic Disc Cube 200x200".to_string(),
        signal_strength: 7,
        central_subfield_thickness_um: 315.0,
        layers: vec![
            OctRetinalLayer {
                layer_name: "RNFL".to_string(),
                thickness_um: 62.0,
                normal_range_min_um: 80.0,
                normal_range_max_um: 110.0,
                is_within_normal: false,
            },
            OctRetinalLayer {
                layer_name: "GCL+IPL".to_string(),
                thickness_um: 55.0,
                normal_range_min_um: 70.0,
                normal_range_max_um: 95.0,
                is_within_normal: false,
            },
        ],
        average_rnfl_thickness_um: 62.0,
        cup_disc_ratio: 0.72,
    };
    encode_to_file(&scan, &path).expect("encode abnormal OCT to file");
    let decoded: OctScanResult = decode_from_file(&path).expect("decode abnormal OCT from file");
    assert_eq!(scan, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 19: Comprehensive exam with both eyes via file ─────────────────────

#[test]
fn test_comprehensive_exam_file_roundtrip() {
    let path = unique_path("t19");
    let exam = ComprehensiveExam {
        patient_id: "PT-11223".to_string(),
        exam_date_epoch: 1710505000,
        refraction_od: sample_refraction(EyeSide::OD),
        refraction_os: RefractionMeasurement {
            eye: EyeSide::OS,
            sphere: -2.75,
            cylinder: -1.00,
            axis: 5,
            add_power: Some(1.50),
            vertex_distance_mm: 12.0,
            pupillary_distance_mm: 31.0,
        },
        acuity_od: sample_acuity(EyeSide::OD),
        acuity_os: sample_acuity(EyeSide::OS),
        iop_od: sample_iop(EyeSide::OD),
        iop_os: IntraocularPressure {
            eye: EyeSide::OS,
            pressure_mmhg: 17.0,
            tonometer: TonometerType::GoldmannApplanation,
            central_corneal_thickness_um: Some(550),
            corrected_iop: Some(16.5),
            timestamp_epoch: 1710505100,
        },
        slit_lamp_od: sample_slit_lamp(EyeSide::OD),
        slit_lamp_os: sample_slit_lamp(EyeSide::OS),
        assessment: "Mild myopia with astigmatism OU, early presbyopia".to_string(),
        plan: "Update spectacle Rx, discuss multifocal options, RTC 1 year".to_string(),
    };
    encode_to_file(&exam, &path).expect("encode comprehensive exam to file");
    let decoded: ComprehensiveExam =
        decode_from_file(&path).expect("decode comprehensive exam from file");
    assert_eq!(exam, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 20: Denied insurance authorization via file ────────────────────────

#[test]
fn test_denied_insurance_auth_file_roundtrip() {
    let path = unique_path("t20");
    let auth = InsuranceAuthorization {
        authorization_number: "AUTH-2024-55555".to_string(),
        patient_id: "PT-99001".to_string(),
        procedure_code: "66984".to_string(),
        diagnosis_codes: vec!["H25.11".to_string(), "H25.12".to_string()],
        status: AuthorizationStatus::Denied,
        requested_date_epoch: 1710300000,
        expiration_date_epoch: None,
        approved_units: None,
        copay_amount_cents: None,
        provider_npi: "9876543210".to_string(),
        notes: "Denied: VA does not meet surgical threshold per plan criteria".to_string(),
    };
    encode_to_file(&auth, &path).expect("encode denied auth to file");
    let decoded: InsuranceAuthorization =
        decode_from_file(&path).expect("decode denied auth from file");
    assert_eq!(auth, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 21: Tear film severe dry eye via slice ─────────────────────────────

#[test]
fn test_tear_film_severe_dry_eye_slice() {
    let assessments = vec![
        TearFilmAssessment {
            eye: EyeSide::OD,
            tbut_seconds: 2.5,
            schirmer_test_mm: 3.0,
            schirmer_with_anesthesia: true,
            tear_meniscus_height_mm: 0.10,
            meibomian_gland_expressibility: SlitLampGrading::Severe,
            osmolarity_mosml: Some(338.0),
            phenol_red_thread_mm: Some(8.0),
            dry_eye_severity: SlitLampGrading::Severe,
        },
        TearFilmAssessment {
            eye: EyeSide::OS,
            tbut_seconds: 3.0,
            schirmer_test_mm: 4.0,
            schirmer_with_anesthesia: true,
            tear_meniscus_height_mm: 0.12,
            meibomian_gland_expressibility: SlitLampGrading::Severe,
            osmolarity_mosml: Some(330.0),
            phenol_red_thread_mm: None,
            dry_eye_severity: SlitLampGrading::Severe,
        },
    ];
    let bytes = encode_to_vec(&assessments).expect("encode severe dry eye");
    let (decoded, _): (Vec<TearFilmAssessment>, _) =
        decode_from_slice(&bytes).expect("decode severe dry eye");
    assert_eq!(assessments, decoded);
}

// ── Test 22: Pediatric screening with amblyopia risk via file ───────────────

#[test]
fn test_pediatric_amblyopia_risk_file_roundtrip() {
    let path = unique_path("t22");
    let screening = PediatricVisionScreening {
        patient_age_months: 48,
        screening_method: "HOTV crowded".to_string(),
        fixation_preference: "Prefers OD, unsteady OS".to_string(),
        cover_test_result: "4pd esotropia OS at near".to_string(),
        stereo_acuity_arc_seconds: Some(400),
        color_vision_result: None,
        cycloplegic_refraction: Some(RefractionMeasurement {
            eye: EyeSide::OS,
            sphere: 3.50,
            cylinder: -1.25,
            axis: 90,
            add_power: None,
            vertex_distance_mm: 12.0,
            pupillary_distance_mm: 24.5,
        }),
        amblyopia_risk: true,
        strabismus_detected: true,
        referral_recommended: true,
        notes: "Significant hyperopic anisometropia OS with microtropia. Recommend full exam with pediatric ophthalmology, likely patching therapy indicated.".to_string(),
    };
    encode_to_file(&screening, &path).expect("encode pediatric amblyopia screening to file");
    let decoded: PediatricVisionScreening =
        decode_from_file(&path).expect("decode pediatric amblyopia screening from file");
    assert_eq!(screening, decoded);
    std::fs::remove_file(&path).ok();
}
