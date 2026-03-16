//! Advanced file I/O tests for the medical imaging / DICOM domain.
//!
//! Covers CT scans, MRI images, image series, patient metadata, modality types,
//! pixel data, windowing parameters, DICOM tags, and imaging protocols.

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
use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ============================================================
// Domain types
// ============================================================

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Modality {
    CT,
    Mri,
    Pet,
    Ultrasound,
    XRay,
    Mammography,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PhotometricInterpretation {
    Monochrome1,
    Monochrome2,
    Rgb,
    YbrFull,
    PaletteColor,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransferSyntax {
    ImplicitVrLittleEndian,
    ExplicitVrLittleEndian,
    ExplicitVrBigEndian,
    JpegBaseline,
    Jpeg2000Lossless,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DicomTag {
    group: u16,
    element: u16,
    vr: String,
    value: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PatientMetadata {
    patient_id: String,
    patient_name: String,
    birth_date: String,
    sex: String,
    weight_kg: Option<f32>,
    age_years: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WindowingParameters {
    window_center: f64,
    window_width: f64,
    rescale_intercept: f64,
    rescale_slope: f64,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PixelDataDescriptor {
    rows: u16,
    columns: u16,
    bits_allocated: u8,
    bits_stored: u8,
    high_bit: u8,
    pixel_representation: u8,
    photometric_interpretation: PhotometricInterpretation,
    samples_per_pixel: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DicomImage {
    sop_instance_uid: String,
    modality: Modality,
    transfer_syntax: TransferSyntax,
    patient: PatientMetadata,
    pixel_descriptor: PixelDataDescriptor,
    windowing: Option<WindowingParameters>,
    instance_number: u32,
    slice_location_mm: Option<f64>,
    pixel_data: Vec<u16>,
    acquisition_date: String,
    kvp: Option<f64>,
    exposure_mas: Option<f64>,
    extra_tags: Vec<DicomTag>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImagingProtocol {
    protocol_name: String,
    modality: Modality,
    slice_thickness_mm: f64,
    kvp: f64,
    tube_current_ma: f64,
    pitch: Option<f64>,
    reconstruction_kernel: String,
    field_of_view_mm: f64,
    contrast_agent: bool,
    series_description: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImageSeries {
    series_instance_uid: String,
    series_number: u32,
    modality: Modality,
    protocol: ImagingProtocol,
    images: Vec<DicomImage>,
    body_part_examined: String,
    patient: PatientMetadata,
}

// ============================================================
// Helper constructors
// ============================================================

fn sample_patient() -> PatientMetadata {
    PatientMetadata {
        patient_id: "PAT-000042".to_string(),
        patient_name: "DOE^JOHN^M".to_string(),
        birth_date: "19680315".to_string(),
        sex: "M".to_string(),
        weight_kg: Some(78.5),
        age_years: Some(57),
    }
}

fn sample_pixel_descriptor(rows: u16, cols: u16) -> PixelDataDescriptor {
    PixelDataDescriptor {
        rows,
        columns: cols,
        bits_allocated: 16,
        bits_stored: 12,
        high_bit: 11,
        pixel_representation: 0,
        photometric_interpretation: PhotometricInterpretation::Monochrome2,
        samples_per_pixel: 1,
    }
}

fn sample_windowing() -> WindowingParameters {
    WindowingParameters {
        window_center: 40.0,
        window_width: 400.0,
        rescale_intercept: -1024.0,
        rescale_slope: 1.0,
        label: "SOFT_TISSUE".to_string(),
    }
}

fn sample_dicom_tag(group: u16, element: u16) -> DicomTag {
    DicomTag {
        group,
        element,
        vr: "LO".to_string(),
        value: format!("VALUE_{:04X}{:04X}", group, element),
    }
}

fn sample_imaging_protocol() -> ImagingProtocol {
    ImagingProtocol {
        protocol_name: "CHEST_CT_STANDARD".to_string(),
        modality: Modality::CT,
        slice_thickness_mm: 1.25,
        kvp: 120.0,
        tube_current_ma: 200.0,
        pitch: Some(1.375),
        reconstruction_kernel: "STANDARD".to_string(),
        field_of_view_mm: 380.0,
        contrast_agent: false,
        series_description: "Chest CT without contrast".to_string(),
    }
}

fn sample_dicom_image(instance_number: u32) -> DicomImage {
    let pixel_count = 512usize * 512;
    let pixel_data: Vec<u16> = (0..pixel_count)
        .map(|i| ((i as u32 + instance_number * 1000) % 4096) as u16)
        .collect();
    DicomImage {
        sop_instance_uid: format!("1.2.840.10008.5.1.4.1.1.2.{}", instance_number),
        modality: Modality::CT,
        transfer_syntax: TransferSyntax::ExplicitVrLittleEndian,
        patient: sample_patient(),
        pixel_descriptor: sample_pixel_descriptor(512, 512),
        windowing: Some(sample_windowing()),
        instance_number,
        slice_location_mm: Some(-100.0 + (instance_number as f64) * 1.25),
        pixel_data,
        acquisition_date: "20260315".to_string(),
        kvp: Some(120.0),
        exposure_mas: Some(100.0),
        extra_tags: vec![
            sample_dicom_tag(0x0008, 0x0070),
            sample_dicom_tag(0x0018, 0x0050),
        ],
    }
}

fn sample_image_series() -> ImageSeries {
    ImageSeries {
        series_instance_uid: "1.2.840.10008.5.1.4.1.1.2.9999".to_string(),
        series_number: 1,
        modality: Modality::CT,
        protocol: sample_imaging_protocol(),
        images: (1..=4).map(sample_dicom_image).collect(),
        body_part_examined: "CHEST".to_string(),
        patient: sample_patient(),
    }
}

// ============================================================
// Tests
// ============================================================

#[test]
fn test_patient_metadata_basic_roundtrip() {
    let original = sample_patient();
    let encoded = encode_to_vec(&original).expect("encode PatientMetadata");
    let (decoded, _): (PatientMetadata, usize) =
        decode_from_slice(&encoded).expect("decode PatientMetadata");
    assert_eq!(original, decoded);
}

#[test]
fn test_dicom_image_basic_roundtrip() {
    let original = sample_dicom_image(1);
    let encoded = encode_to_vec(&original).expect("encode DicomImage");
    let (decoded, _): (DicomImage, usize) = decode_from_slice(&encoded).expect("decode DicomImage");
    assert_eq!(original, decoded);
}

#[test]
fn test_image_series_file_io_roundtrip() {
    let original = sample_image_series();
    let path = temp_dir().join("oxicode_dicom_image_series_27.bin");
    encode_to_file(&original, &path).expect("encode_to_file ImageSeries");
    let decoded: ImageSeries = decode_from_file(&path).expect("decode_from_file ImageSeries");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_dicom_image_file_io_roundtrip() {
    let original = sample_dicom_image(7);
    let path = temp_dir().join("oxicode_dicom_image_instance7_27.bin");
    encode_to_file(&original, &path).expect("encode_to_file DicomImage");
    let decoded: DicomImage = decode_from_file(&path).expect("decode_from_file DicomImage");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_modality_enum_all_variants_roundtrip() {
    let variants = [
        Modality::CT,
        Modality::Mri,
        Modality::Pet,
        Modality::Ultrasound,
        Modality::XRay,
        Modality::Mammography,
    ];
    for modality in &variants {
        let encoded = encode_to_vec(modality).expect("encode Modality variant");
        let (decoded, _): (Modality, usize) =
            decode_from_slice(&encoded).expect("decode Modality variant");
        assert_eq!(modality, &decoded);
    }
}

#[test]
fn test_transfer_syntax_enum_all_variants_roundtrip() {
    let variants = [
        TransferSyntax::ImplicitVrLittleEndian,
        TransferSyntax::ExplicitVrLittleEndian,
        TransferSyntax::ExplicitVrBigEndian,
        TransferSyntax::JpegBaseline,
        TransferSyntax::Jpeg2000Lossless,
    ];
    for ts in &variants {
        let encoded = encode_to_vec(ts).expect("encode TransferSyntax variant");
        let (decoded, _): (TransferSyntax, usize) =
            decode_from_slice(&encoded).expect("decode TransferSyntax variant");
        assert_eq!(ts, &decoded);
    }
}

#[test]
fn test_photometric_interpretation_enum_roundtrip() {
    let variants = [
        PhotometricInterpretation::Monochrome1,
        PhotometricInterpretation::Monochrome2,
        PhotometricInterpretation::Rgb,
        PhotometricInterpretation::YbrFull,
        PhotometricInterpretation::PaletteColor,
    ];
    for pi in &variants {
        let encoded = encode_to_vec(pi).expect("encode PhotometricInterpretation variant");
        let (decoded, _): (PhotometricInterpretation, usize) =
            decode_from_slice(&encoded).expect("decode PhotometricInterpretation variant");
        assert_eq!(pi, &decoded);
    }
}

#[test]
fn test_big_endian_config_dicom_image_roundtrip() {
    let original = sample_dicom_image(3);
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode DicomImage big-endian");
    let (decoded, _): (DicomImage, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode DicomImage big-endian");
    assert_eq!(original, decoded);
}

#[test]
fn test_fixed_int_config_imaging_protocol_roundtrip() {
    let original = sample_imaging_protocol();
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode ImagingProtocol fixed-int");
    let (decoded, _): (ImagingProtocol, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode ImagingProtocol fixed-int");
    assert_eq!(original, decoded);
}

#[test]
fn test_big_endian_fixed_int_image_series_roundtrip() {
    let original = sample_image_series();
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode ImageSeries big-endian+fixed");
    let (decoded, _): (ImageSeries, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode ImageSeries big-endian+fixed");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_of_dicom_images_roundtrip() {
    let original: Vec<DicomImage> = (1..=6).map(sample_dicom_image).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<DicomImage>");
    let (decoded, _): (Vec<DicomImage>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<DicomImage>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_of_dicom_tags_roundtrip() {
    let original: Vec<DicomTag> = vec![
        sample_dicom_tag(0x0008, 0x0060),
        sample_dicom_tag(0x0010, 0x0020),
        sample_dicom_tag(0x0028, 0x0010),
        sample_dicom_tag(0x0028, 0x0011),
        sample_dicom_tag(0x7FE0, 0x0010),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<DicomTag>");
    let (decoded, _): (Vec<DicomTag>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<DicomTag>");
    assert_eq!(original, decoded);
}

#[test]
fn test_nested_image_series_roundtrip() {
    let original = sample_image_series();
    let encoded = encode_to_vec(&original).expect("encode nested ImageSeries");
    let (decoded, _): (ImageSeries, usize) =
        decode_from_slice(&encoded).expect("decode nested ImageSeries");
    assert_eq!(original, decoded);
    assert_eq!(decoded.images.len(), 4);
    assert_eq!(decoded.patient.patient_id, "PAT-000042");
}

#[test]
fn test_option_windowing_some_roundtrip() {
    let original: Option<WindowingParameters> = Some(sample_windowing());
    let encoded = encode_to_vec(&original).expect("encode Some(WindowingParameters)");
    let (decoded, _): (Option<WindowingParameters>, usize) =
        decode_from_slice(&encoded).expect("decode Some(WindowingParameters)");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_windowing_none_roundtrip() {
    let original: Option<WindowingParameters> = None;
    let encoded = encode_to_vec(&original).expect("encode None<WindowingParameters>");
    let (decoded, _): (Option<WindowingParameters>, usize) =
        decode_from_slice(&encoded).expect("decode None<WindowingParameters>");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_patient_fields_none_roundtrip() {
    let original = PatientMetadata {
        patient_id: "PAT-ANON".to_string(),
        patient_name: "ANONYMOUS^PATIENT".to_string(),
        birth_date: "".to_string(),
        sex: "O".to_string(),
        weight_kg: None,
        age_years: None,
    };
    let encoded = encode_to_vec(&original).expect("encode anonymous PatientMetadata");
    let (decoded, _): (PatientMetadata, usize) =
        decode_from_slice(&encoded).expect("decode anonymous PatientMetadata");
    assert_eq!(original, decoded);
    assert!(decoded.weight_kg.is_none());
    assert!(decoded.age_years.is_none());
}

#[test]
fn test_large_pixel_data_roundtrip() {
    // Simulate a 1024x1024 grayscale frame (16-bit pixels)
    let pixel_count = 1024usize * 1024;
    let pixel_data: Vec<u16> = (0..pixel_count).map(|i| (i % 4096) as u16).collect();
    let original = DicomImage {
        sop_instance_uid: "1.2.840.10008.5.1.4.1.1.2.large".to_string(),
        modality: Modality::CT,
        transfer_syntax: TransferSyntax::ExplicitVrLittleEndian,
        patient: sample_patient(),
        pixel_descriptor: sample_pixel_descriptor(1024, 1024),
        windowing: Some(sample_windowing()),
        instance_number: 1,
        slice_location_mm: Some(0.0),
        pixel_data,
        acquisition_date: "20260315".to_string(),
        kvp: Some(120.0),
        exposure_mas: Some(200.0),
        extra_tags: vec![],
    };
    let encoded = encode_to_vec(&original).expect("encode large pixel DicomImage");
    let (decoded, bytes_consumed): (DicomImage, usize) =
        decode_from_slice(&encoded).expect("decode large pixel DicomImage");
    assert_eq!(original.pixel_data.len(), decoded.pixel_data.len());
    assert_eq!(original, decoded);
    assert_eq!(bytes_consumed, encoded.len());
}

#[test]
fn test_bytes_consumed_matches_encoded_length() {
    let original = sample_image_series();
    let encoded = encode_to_vec(&original).expect("encode ImageSeries");
    let (_, consumed): (ImageSeries, usize) =
        decode_from_slice(&encoded).expect("decode ImageSeries");
    assert_eq!(
        consumed,
        encoded.len(),
        "bytes consumed must equal total encoded length"
    );
}

#[test]
fn test_overwrite_file_io_roundtrip() {
    let path = temp_dir().join("oxicode_dicom_overwrite_27.bin");

    // First write: small image
    let first = sample_dicom_image(1);
    encode_to_file(&first, &path).expect("first encode_to_file DicomImage");

    // Second write: different image — must overwrite cleanly
    let second = sample_dicom_image(99);
    encode_to_file(&second, &path).expect("second (overwrite) encode_to_file DicomImage");

    let decoded: DicomImage = decode_from_file(&path).expect("decode_from_file after overwrite");
    assert_eq!(second, decoded);
    assert_ne!(first.instance_number, decoded.instance_number);

    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_missing_file_returns_error() {
    let path = temp_dir().join("oxicode_dicom_nonexistent_27_should_not_exist.bin");
    // Ensure it is gone if it somehow exists
    let _ = std::fs::remove_file(&path);
    let result = decode_from_file::<DicomImage>(&path);
    assert!(
        result.is_err(),
        "decode_from_file on missing path must return Err"
    );
}

#[test]
fn test_file_io_bytes_match_encode_to_vec() {
    let original = sample_dicom_image(5);
    let path = temp_dir().join("oxicode_dicom_file_vs_vec_27.bin");
    encode_to_file(&original, &path).expect("encode_to_file");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = encode_to_vec(&original).expect("encode_to_vec");
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must match encode_to_vec output"
    );
    std::fs::remove_file(&path).expect("remove temp file");
}

#[test]
fn test_imaging_protocol_contrast_variants_roundtrip() {
    let mut proto_plain = sample_imaging_protocol();
    proto_plain.contrast_agent = false;

    let mut proto_contrast = sample_imaging_protocol();
    proto_contrast.contrast_agent = true;
    proto_contrast.protocol_name = "CHEST_CT_CONTRAST".to_string();
    proto_contrast.kvp = 100.0;
    proto_contrast.series_description = "Chest CT with IV contrast".to_string();

    for proto in &[proto_plain, proto_contrast] {
        let encoded = encode_to_vec(proto).expect("encode ImagingProtocol");
        let (decoded, bytes_consumed): (ImagingProtocol, usize) =
            decode_from_slice(&encoded).expect("decode ImagingProtocol");
        assert_eq!(proto, &decoded);
        assert_eq!(bytes_consumed, encoded.len());
    }
}
