#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Domain types: Archaeology & Cultural Heritage Preservation ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ExcavationSite {
    site_code: String,
    grid_easting: f64,
    grid_northing: f64,
    elevation_m: f64,
    stratigraphy_layers: Vec<StratigraphyLayer>,
    country: String,
    region: String,
    is_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StratigraphyLayer {
    layer_id: u32,
    context_number: u32,
    description: String,
    depth_top_cm: f64,
    depth_bottom_cm: f64,
    soil_color_munsell: String,
    inclusions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ArtifactCatalogEntry {
    catalog_id: u64,
    site_code: String,
    context_number: u32,
    material: String,
    period: String,
    provenance: String,
    dimensions_mm: (f64, f64, f64),
    weight_grams: f64,
    condition: String,
    photograph_ids: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RadiocarbonResult {
    lab_code: String,
    sample_id: String,
    c14_age_bp: u32,
    c14_error: u32,
    calibrated_range_start_bce: i32,
    calibrated_range_end_bce: i32,
    calibration_curve: String,
    delta_c13: f64,
    material_dated: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PotteryTypology {
    type_code: String,
    ware_name: String,
    fabric_group: String,
    form: String,
    rim_diameter_cm: Option<f64>,
    wall_thickness_mm: f64,
    surface_treatment: String,
    decoration: Vec<String>,
    period_attribution: String,
    parallels: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FaunalAnalysisRecord {
    specimen_id: u64,
    species: String,
    common_name: String,
    element: String,
    portion: String,
    side: String,
    taphonomy: Vec<String>,
    butchery_marks: bool,
    burned: bool,
    age_at_death: String,
    weight_grams: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GisSpatialData {
    feature_id: u64,
    feature_type: String,
    polygon_coords: Vec<(f64, f64)>,
    area_sq_m: f64,
    centroid: (f64, f64),
    crs_epsg: u32,
    attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ConservationTreatment {
    treatment_id: u64,
    artifact_id: u64,
    treatment_date: String,
    conservator: String,
    condition_before: String,
    condition_after: String,
    materials_used: Vec<String>,
    procedures: Vec<String>,
    duration_hours: f64,
    follow_up_date: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PhotogrammetryModel {
    model_id: u64,
    artifact_id: u64,
    num_photographs: u32,
    resolution_dpi: u32,
    point_cloud_size: u64,
    mesh_faces: u64,
    texture_resolution: (u32, u32),
    file_format: String,
    file_size_bytes: u64,
    georeferenced: bool,
    processing_software: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DendrochronologySequence {
    sample_id: String,
    species: String,
    ring_widths_mm: Vec<f64>,
    start_year: i32,
    end_year: i32,
    cross_date_t_value: f64,
    master_chronology: String,
    bark_edge_present: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PalynologyRecord {
    sample_id: String,
    depth_cm: f64,
    pollen_counts: Vec<PollenTaxon>,
    total_land_pollen: u32,
    concentration_grains_per_ml: f64,
    charcoal_fragments: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PollenTaxon {
    taxon_name: String,
    count: u32,
    percentage: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HarrisMatrixRelation {
    relation_id: u64,
    upper_context: u32,
    lower_context: u32,
    relationship_type: String,
    certainty: String,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MuseumAccessionRecord {
    accession_number: String,
    object_name: String,
    description: String,
    donor_or_source: String,
    date_accessioned: String,
    department: String,
    storage_location: String,
    insurance_value_usd: f64,
    display_history: Vec<String>,
    condition_report: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RepatriationClaim {
    claim_id: u64,
    claiming_nation: String,
    holding_institution: String,
    object_description: String,
    legal_basis: String,
    date_filed: String,
    status: String,
    provenance_chain: Vec<ProvenanceEntry>,
    supporting_documents: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProvenanceEntry {
    date_range: String,
    holder: String,
    location: String,
    documentation: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DigitalArchiveMetadata {
    archive_id: String,
    title: String,
    creator: String,
    date_created: String,
    format: String,
    file_count: u32,
    total_size_bytes: u64,
    checksum_sha256: String,
    access_level: String,
    related_site_codes: Vec<String>,
    keywords: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FieldworkPermit {
    permit_number: String,
    site_code: String,
    principal_investigator: String,
    institution: String,
    issuing_authority: String,
    date_issued: String,
    date_expires: String,
    permitted_activities: Vec<String>,
    conditions: Vec<String>,
    is_renewable: bool,
}

// --- Tests ---

#[test]
fn test_excavation_site_roundtrip() {
    let site = ExcavationSite {
        site_code: "TEL-MEGIDDO-2025".to_string(),
        grid_easting: 234567.89,
        grid_northing: 3456789.12,
        elevation_m: 182.5,
        stratigraphy_layers: vec![
            StratigraphyLayer {
                layer_id: 1,
                context_number: 1001,
                description: "Topsoil, modern disturbance".to_string(),
                depth_top_cm: 0.0,
                depth_bottom_cm: 25.0,
                soil_color_munsell: "10YR 4/3".to_string(),
                inclusions: vec!["roots".to_string(), "modern debris".to_string()],
            },
            StratigraphyLayer {
                layer_id: 2,
                context_number: 1002,
                description: "Iron Age II destruction layer".to_string(),
                depth_top_cm: 25.0,
                depth_bottom_cm: 68.0,
                soil_color_munsell: "7.5YR 3/2".to_string(),
                inclusions: vec![
                    "ash".to_string(),
                    "charcoal".to_string(),
                    "burnt mudbrick".to_string(),
                ],
            },
        ],
        country: "Israel".to_string(),
        region: "Jezreel Valley".to_string(),
        is_active: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&site, cfg).expect("encode excavation site");
    let (decoded, _): (ExcavationSite, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode excavation site");
    assert_eq!(site, decoded);
}

#[test]
fn test_artifact_catalog_roundtrip() {
    let artifact = ArtifactCatalogEntry {
        catalog_id: 50231,
        site_code: "POMPEII-REG-IX".to_string(),
        context_number: 4010,
        material: "Bronze".to_string(),
        period: "1st century CE".to_string(),
        provenance: "Insula 5, Room 12, floor deposit".to_string(),
        dimensions_mm: (142.5, 38.0, 12.3),
        weight_grams: 267.8,
        condition: "Good, minor patina".to_string(),
        photograph_ids: vec![
            "POM-50231-01.tif".to_string(),
            "POM-50231-02.tif".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&artifact, cfg).expect("encode artifact");
    let (decoded, _): (ArtifactCatalogEntry, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode artifact");
    assert_eq!(artifact, decoded);
}

#[test]
fn test_radiocarbon_dating_roundtrip() {
    let result = RadiocarbonResult {
        lab_code: "OxA-39421".to_string(),
        sample_id: "SAMP-2025-0087".to_string(),
        c14_age_bp: 3250,
        c14_error: 35,
        calibrated_range_start_bce: 1614,
        calibrated_range_end_bce: 1500,
        calibration_curve: "IntCal20".to_string(),
        delta_c13: -25.3,
        material_dated: "Charred cereal grain (Triticum aestivum)".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&result, cfg).expect("encode C14 result");
    let (decoded, _): (RadiocarbonResult, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode C14 result");
    assert_eq!(result, decoded);
}

#[test]
fn test_pottery_typology_roundtrip() {
    let pottery = PotteryTypology {
        type_code: "LBA-PW-III".to_string(),
        ware_name: "Philistine Bichrome Ware".to_string(),
        fabric_group: "Fabric G, calcareous".to_string(),
        form: "Bell-shaped krater".to_string(),
        rim_diameter_cm: Some(28.0),
        wall_thickness_mm: 6.5,
        surface_treatment: "Slipped and burnished".to_string(),
        decoration: vec![
            "Horizontal bands".to_string(),
            "Stylized bird motif".to_string(),
            "Geometric spirals".to_string(),
        ],
        period_attribution: "Iron Age I (1175-1000 BCE)".to_string(),
        parallels: vec![
            "Dothan 1982, Type 14".to_string(),
            "Ben-Shlomo 2006, Fig. 3.12".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&pottery, cfg).expect("encode pottery");
    let (decoded, _): (PotteryTypology, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode pottery");
    assert_eq!(pottery, decoded);
}

#[test]
fn test_faunal_analysis_roundtrip() {
    let faunal = FaunalAnalysisRecord {
        specimen_id: 88234,
        species: "Ovis aries".to_string(),
        common_name: "Domestic sheep".to_string(),
        element: "Humerus".to_string(),
        portion: "Distal shaft and epiphysis".to_string(),
        side: "Right".to_string(),
        taphonomy: vec![
            "Root etching".to_string(),
            "Weathering stage 2".to_string(),
            "Carnivore gnawing".to_string(),
        ],
        butchery_marks: true,
        burned: false,
        age_at_death: "Adult (fused epiphysis)".to_string(),
        weight_grams: 34.7,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&faunal, cfg).expect("encode faunal record");
    let (decoded, _): (FaunalAnalysisRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode faunal record");
    assert_eq!(faunal, decoded);
}

#[test]
fn test_gis_spatial_data_roundtrip() {
    let gis = GisSpatialData {
        feature_id: 7001,
        feature_type: "Excavation trench polygon".to_string(),
        polygon_coords: vec![
            (35.18450, 31.77620),
            (35.18460, 31.77620),
            (35.18460, 31.77630),
            (35.18450, 31.77630),
            (35.18450, 31.77620),
        ],
        area_sq_m: 110.25,
        centroid: (35.18455, 31.77625),
        crs_epsg: 4326,
        attributes: vec![
            ("trench".to_string(), "Area F".to_string()),
            ("season".to_string(), "2025".to_string()),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&gis, cfg).expect("encode GIS data");
    let (decoded, _): (GisSpatialData, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode GIS data");
    assert_eq!(gis, decoded);
}

#[test]
fn test_conservation_treatment_roundtrip() {
    let treatment = ConservationTreatment {
        treatment_id: 12045,
        artifact_id: 50231,
        treatment_date: "2025-09-15".to_string(),
        conservator: "Dr. Elena Vasquez".to_string(),
        condition_before: "Active bronze disease, flaking patina, structural cracks".to_string(),
        condition_after: "Stabilized, consolidant applied, cracks filled".to_string(),
        materials_used: vec![
            "Paraloid B-72 (5% in acetone)".to_string(),
            "Benzotriazole solution".to_string(),
            "Microcrystalline wax".to_string(),
        ],
        procedures: vec![
            "Mechanical cleaning with scalpel".to_string(),
            "Chemical treatment for chlorides".to_string(),
            "Consolidation with adhesive".to_string(),
            "Protective wax coating".to_string(),
        ],
        duration_hours: 14.5,
        follow_up_date: Some("2026-03-15".to_string()),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&treatment, cfg).expect("encode conservation treatment");
    let (decoded, _): (ConservationTreatment, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode conservation treatment");
    assert_eq!(treatment, decoded);
}

#[test]
fn test_photogrammetry_model_roundtrip() {
    let model = PhotogrammetryModel {
        model_id: 3301,
        artifact_id: 50231,
        num_photographs: 287,
        resolution_dpi: 600,
        point_cloud_size: 42_000_000,
        mesh_faces: 8_500_000,
        texture_resolution: (8192, 8192),
        file_format: "OBJ + MTL + PNG".to_string(),
        file_size_bytes: 1_250_000_000,
        georeferenced: true,
        processing_software: "Agisoft Metashape Pro 2.1".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&model, cfg).expect("encode photogrammetry model");
    let (decoded, _): (PhotogrammetryModel, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode photogrammetry model");
    assert_eq!(model, decoded);
}

#[test]
fn test_dendrochronology_sequence_roundtrip() {
    let dendro = DendrochronologySequence {
        sample_id: "DENDRO-OAK-2025-014".to_string(),
        species: "Quercus robur".to_string(),
        ring_widths_mm: vec![
            1.23, 0.98, 1.45, 1.12, 0.87, 1.56, 1.34, 0.76, 1.01, 1.67, 0.92, 1.38, 1.05, 0.83,
            1.49, 1.21, 0.95, 1.58, 1.10, 0.88,
        ],
        start_year: 1245,
        end_year: 1264,
        cross_date_t_value: 7.82,
        master_chronology: "British Isles Oak Master".to_string(),
        bark_edge_present: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&dendro, cfg).expect("encode dendro sequence");
    let (decoded, _): (DendrochronologySequence, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode dendro sequence");
    assert_eq!(dendro, decoded);
}

#[test]
fn test_palynology_record_roundtrip() {
    let pollen = PalynologyRecord {
        sample_id: "PAL-CORE-07-L12".to_string(),
        depth_cm: 145.0,
        pollen_counts: vec![
            PollenTaxon {
                taxon_name: "Quercus (oak)".to_string(),
                count: 87,
                percentage: 29.0,
            },
            PollenTaxon {
                taxon_name: "Poaceae (grasses)".to_string(),
                count: 65,
                percentage: 21.7,
            },
            PollenTaxon {
                taxon_name: "Cerealia-type".to_string(),
                count: 42,
                percentage: 14.0,
            },
            PollenTaxon {
                taxon_name: "Plantago lanceolata".to_string(),
                count: 23,
                percentage: 7.7,
            },
        ],
        total_land_pollen: 300,
        concentration_grains_per_ml: 45000.0,
        charcoal_fragments: 156,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&pollen, cfg).expect("encode palynology record");
    let (decoded, _): (PalynologyRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode palynology record");
    assert_eq!(pollen, decoded);
}

#[test]
fn test_harris_matrix_relations_roundtrip() {
    let relations = vec![
        HarrisMatrixRelation {
            relation_id: 1,
            upper_context: 1001,
            lower_context: 1002,
            relationship_type: "Overlies".to_string(),
            certainty: "Certain".to_string(),
            notes: "Clear interface visible in section".to_string(),
        },
        HarrisMatrixRelation {
            relation_id: 2,
            upper_context: 1003,
            lower_context: 1004,
            relationship_type: "Cuts".to_string(),
            certainty: "Probable".to_string(),
            notes: "Pit cut into earlier floor surface".to_string(),
        },
        HarrisMatrixRelation {
            relation_id: 3,
            upper_context: 1002,
            lower_context: 1005,
            relationship_type: "Same as".to_string(),
            certainty: "Certain".to_string(),
            notes: "Continuous layer across baulk".to_string(),
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&relations, cfg).expect("encode Harris matrix");
    let (decoded, _): (Vec<HarrisMatrixRelation>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Harris matrix");
    assert_eq!(relations, decoded);
}

#[test]
fn test_museum_accession_roundtrip() {
    let record = MuseumAccessionRecord {
        accession_number: "BM-2025.0412.1".to_string(),
        object_name: "Terracotta figurine of Aphrodite".to_string(),
        description: "Hellenistic mould-made terracotta figurine depicting Aphrodite in contrapposto pose, traces of original polychromy on drapery".to_string(),
        donor_or_source: "Excavation, Knossos 2024 Season".to_string(),
        date_accessioned: "2025-04-12".to_string(),
        department: "Greek and Roman Antiquities".to_string(),
        storage_location: "Store Room G, Cabinet 14, Shelf 3B".to_string(),
        insurance_value_usd: 125000.0,
        display_history: vec![
            "Main Gallery, Case 7 (2025-06 to 2025-12)".to_string(),
            "Traveling Exhibition: Hellenistic Arts (2026-01 to 2026-06)".to_string(),
        ],
        condition_report: "Complete, minor surface abrasion on nose, stable condition".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("encode accession record");
    let (decoded, _): (MuseumAccessionRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode accession record");
    assert_eq!(record, decoded);
}

#[test]
fn test_repatriation_claim_roundtrip() {
    let claim = RepatriationClaim {
        claim_id: 9920,
        claiming_nation: "Greece".to_string(),
        holding_institution: "National Museum of Antiquities, Leiden".to_string(),
        object_description: "Marble relief fragment depicting processional scene, likely from Parthenon north frieze".to_string(),
        legal_basis: "UNESCO 1970 Convention, bilateral agreement 2019".to_string(),
        date_filed: "2024-11-01".to_string(),
        status: "Under review".to_string(),
        provenance_chain: vec![
            ProvenanceEntry {
                date_range: "5th century BCE - 1801".to_string(),
                holder: "In situ, Parthenon".to_string(),
                location: "Athens, Greece".to_string(),
                documentation: "Historical record".to_string(),
            },
            ProvenanceEntry {
                date_range: "1801-1816".to_string(),
                holder: "Lord Elgin collection".to_string(),
                location: "London, England".to_string(),
                documentation: "Firman (contested legality)".to_string(),
            },
            ProvenanceEntry {
                date_range: "1816-1920".to_string(),
                holder: "Private collection, Netherlands".to_string(),
                location: "Amsterdam, Netherlands".to_string(),
                documentation: "Auction record 1816".to_string(),
            },
        ],
        supporting_documents: vec![
            "CLAIM-9920-DOC-001.pdf".to_string(),
            "CLAIM-9920-DOC-002.pdf".to_string(),
            "CLAIM-9920-PHOTO-001.jpg".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&claim, cfg).expect("encode repatriation claim");
    let (decoded, _): (RepatriationClaim, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode repatriation claim");
    assert_eq!(claim, decoded);
}

#[test]
fn test_digital_archive_metadata_roundtrip() {
    let archive = DigitalArchiveMetadata {
        archive_id: "OASIS-2025-007823".to_string(),
        title: "Excavations at Roman Fort Vindolanda, Season 2025".to_string(),
        creator: "Vindolanda Archaeological Trust".to_string(),
        date_created: "2025-12-01".to_string(),
        format: "ADS Grey Literature Archive".to_string(),
        file_count: 1247,
        total_size_bytes: 8_500_000_000,
        checksum_sha256: "a3f2b8c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1"
            .to_string(),
        access_level: "Open Access".to_string(),
        related_site_codes: vec!["VINDO-2025".to_string(), "VINDO-2024".to_string()],
        keywords: vec![
            "Roman military".to_string(),
            "Vindolanda".to_string(),
            "Hadrian's Wall".to_string(),
            "Leather artifacts".to_string(),
            "Writing tablets".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&archive, cfg).expect("encode digital archive");
    let (decoded, _): (DigitalArchiveMetadata, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode digital archive");
    assert_eq!(archive, decoded);
}

#[test]
fn test_fieldwork_permit_roundtrip() {
    let permit = FieldworkPermit {
        permit_number: "IAA-2025-EXC-0342".to_string(),
        site_code: "TEL-MEGIDDO-2025".to_string(),
        principal_investigator: "Prof. Israel Finkelstein".to_string(),
        institution: "Tel Aviv University".to_string(),
        issuing_authority: "Israel Antiquities Authority".to_string(),
        date_issued: "2025-01-15".to_string(),
        date_expires: "2025-12-31".to_string(),
        permitted_activities: vec![
            "Full excavation".to_string(),
            "Survey and mapping".to_string(),
            "Environmental sampling".to_string(),
        ],
        conditions: vec![
            "Weekly progress reports required".to_string(),
            "All finds to be deposited with IAA".to_string(),
            "Backfill trenches at season end".to_string(),
        ],
        is_renewable: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&permit, cfg).expect("encode fieldwork permit");
    let (decoded, _): (FieldworkPermit, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode fieldwork permit");
    assert_eq!(permit, decoded);
}

#[test]
fn test_multiple_radiocarbon_dates_roundtrip() {
    let dates = vec![
        RadiocarbonResult {
            lab_code: "Beta-612345".to_string(),
            sample_id: "SAMP-A".to_string(),
            c14_age_bp: 2450,
            c14_error: 40,
            calibrated_range_start_bce: 760,
            calibrated_range_end_bce: 410,
            calibration_curve: "IntCal20".to_string(),
            delta_c13: -24.8,
            material_dated: "Charcoal (Olea europaea)".to_string(),
        },
        RadiocarbonResult {
            lab_code: "Beta-612346".to_string(),
            sample_id: "SAMP-B".to_string(),
            c14_age_bp: 5730,
            c14_error: 60,
            calibrated_range_start_bce: 4720,
            calibrated_range_end_bce: 4460,
            calibration_curve: "IntCal20".to_string(),
            delta_c13: -26.1,
            material_dated: "Bone collagen (Bos taurus)".to_string(),
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&dates, cfg).expect("encode multiple C14 dates");
    let (decoded, _): (Vec<RadiocarbonResult>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode multiple C14 dates");
    assert_eq!(dates, decoded);
}

#[test]
fn test_pottery_with_none_rim_diameter_roundtrip() {
    let sherd = PotteryTypology {
        type_code: "MBA-CW-I".to_string(),
        ware_name: "Chocolate-on-White Ware".to_string(),
        fabric_group: "Fabric A, sandy".to_string(),
        form: "Body sherd (form indeterminate)".to_string(),
        rim_diameter_cm: None,
        wall_thickness_mm: 8.2,
        surface_treatment: "Smooth, no slip".to_string(),
        decoration: vec!["Painted chocolate-brown band".to_string()],
        period_attribution: "Middle Bronze Age IIB (1650-1550 BCE)".to_string(),
        parallels: vec!["Oren 1969, Plate XII.5".to_string()],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&sherd, cfg).expect("encode sherd with None rim");
    let (decoded, _): (PotteryTypology, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode sherd with None rim");
    assert_eq!(sherd, decoded);
}

#[test]
fn test_conservation_without_followup_roundtrip() {
    let treatment = ConservationTreatment {
        treatment_id: 12100,
        artifact_id: 88001,
        treatment_date: "2025-11-20".to_string(),
        conservator: "Dr. Tomoko Ishii".to_string(),
        condition_before: "Fragmented, salt crystallization damage".to_string(),
        condition_after: "Reconstructed and desalinated".to_string(),
        materials_used: vec![
            "Deionized water bath".to_string(),
            "Primal AC-33".to_string(),
        ],
        procedures: vec![
            "Desalination (14-day soak cycle)".to_string(),
            "Fragment joining with adhesive".to_string(),
            "Gap filling with plaster".to_string(),
        ],
        duration_hours: 42.0,
        follow_up_date: None,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&treatment, cfg).expect("encode treatment no followup");
    let (decoded, _): (ConservationTreatment, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode treatment no followup");
    assert_eq!(treatment, decoded);
}

#[test]
fn test_large_gis_polygon_roundtrip() {
    let mut coords = Vec::new();
    for i in 0..50 {
        let angle = (i as f64) * 2.0 * std::f64::consts::PI / 50.0;
        let lng = 35.2000 + 0.005 * angle.cos();
        let lat = 31.7800 + 0.003 * angle.sin();
        coords.push((lng, lat));
    }
    coords.push(coords[0]);

    let gis = GisSpatialData {
        feature_id: 9999,
        feature_type: "Site boundary polygon".to_string(),
        polygon_coords: coords,
        area_sq_m: 47120.0,
        centroid: (35.2000, 31.7800),
        crs_epsg: 4326,
        attributes: vec![
            (
                "site_name".to_string(),
                "Tell es-Sultan (Jericho)".to_string(),
            ),
            ("period".to_string(), "Neolithic to Islamic".to_string()),
            (
                "protection_status".to_string(),
                "UNESCO World Heritage (tentative)".to_string(),
            ),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&gis, cfg).expect("encode large polygon");
    let (decoded, _): (GisSpatialData, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode large polygon");
    assert_eq!(gis, decoded);
}

#[test]
fn test_dendro_long_sequence_roundtrip() {
    let ring_widths: Vec<f64> = (0..150)
        .map(|i| {
            let base = 1.5 + 0.5 * ((i as f64) * 0.3).sin();
            (base * 100.0).round() / 100.0
        })
        .collect();

    let dendro = DendrochronologySequence {
        sample_id: "DENDRO-ELM-YORK-001".to_string(),
        species: "Ulmus glabra".to_string(),
        ring_widths_mm: ring_widths,
        start_year: 1050,
        end_year: 1199,
        cross_date_t_value: 9.14,
        master_chronology: "Northern England Elm Master".to_string(),
        bark_edge_present: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&dendro, cfg).expect("encode long dendro");
    let (decoded, _): (DendrochronologySequence, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode long dendro");
    assert_eq!(dendro, decoded);
}

#[test]
fn test_empty_collections_roundtrip() {
    let site = ExcavationSite {
        site_code: "SURVEY-ONLY-001".to_string(),
        grid_easting: 500000.0,
        grid_northing: 4000000.0,
        elevation_m: 350.0,
        stratigraphy_layers: vec![],
        country: "Turkey".to_string(),
        region: "Cappadocia".to_string(),
        is_active: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&site, cfg).expect("encode empty layers site");
    let (decoded, _): (ExcavationSite, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode empty layers site");
    assert_eq!(site, decoded);
}

#[test]
fn test_complex_repatriation_provenance_chain_roundtrip() {
    let claim = RepatriationClaim {
        claim_id: 10500,
        claiming_nation: "Nigeria".to_string(),
        holding_institution: "Smithsonian National Museum of African Art".to_string(),
        object_description:
            "Benin Bronze plaque depicting warrior chief with attendants, cast brass, 16th century"
                .to_string(),
        legal_basis: "Voluntary restitution agreement, 2024 bilateral framework".to_string(),
        date_filed: "2025-03-01".to_string(),
        status: "Approved for return".to_string(),
        provenance_chain: vec![
            ProvenanceEntry {
                date_range: "c. 1550 - 1897".to_string(),
                holder: "Oba of Benin, Royal Palace".to_string(),
                location: "Benin City, Kingdom of Benin".to_string(),
                documentation: "Oral history, comparative stylistic analysis".to_string(),
            },
            ProvenanceEntry {
                date_range: "1897".to_string(),
                holder: "British Punitive Expedition".to_string(),
                location: "Benin City".to_string(),
                documentation: "Military dispatch records, Admiralty files".to_string(),
            },
            ProvenanceEntry {
                date_range: "1897-1910".to_string(),
                holder: "Foreign and Commonwealth Office".to_string(),
                location: "London, England".to_string(),
                documentation: "Government inventory FCO/23/1897".to_string(),
            },
            ProvenanceEntry {
                date_range: "1910-1955".to_string(),
                holder: "Private collection, Sir Herbert Palmer".to_string(),
                location: "Bournemouth, England".to_string(),
                documentation: "Christie's auction catalog, 1910".to_string(),
            },
            ProvenanceEntry {
                date_range: "1955-2025".to_string(),
                holder: "Smithsonian Institution".to_string(),
                location: "Washington, D.C., USA".to_string(),
                documentation: "Accession record 1955.012.003".to_string(),
            },
        ],
        supporting_documents: vec![
            "CLAIM-10500-HIST-001.pdf".to_string(),
            "CLAIM-10500-PROV-001.pdf".to_string(),
            "CLAIM-10500-LEGAL-001.pdf".to_string(),
            "CLAIM-10500-PHOTO-001.tif".to_string(),
            "CLAIM-10500-PHOTO-002.tif".to_string(),
            "CLAIM-10500-3D-MODEL.obj".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&claim, cfg).expect("encode complex provenance claim");
    let (decoded, _): (RepatriationClaim, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode complex provenance claim");
    assert_eq!(claim, decoded);
}
