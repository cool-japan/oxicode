//! Advanced tests for museum collection management and cultural heritage preservation.
//! 22 test functions covering artifact classification, conservation, provenance,
//! exhibition layout, loan agreements, environmental monitoring, digitization,
//! accession workflows, authentication, storage, insurance, and more.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// --- Artifact classification ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ArtifactMedium {
    Painting {
        technique: String,
        support: String,
    },
    Sculpture {
        material: String,
        weight_grams: u64,
    },
    Textile {
        fiber_type: String,
        weave_pattern: String,
    },
    Ceramic {
        clay_body: String,
        glaze: Option<String>,
    },
    Manuscript {
        script_type: String,
        page_count: u32,
    },
    Photograph {
        process: String,
        negative_exists: bool,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ArtifactRecord {
    accession_number: String,
    title: String,
    artist_or_maker: Option<String>,
    creation_year: i32,
    medium: ArtifactMedium,
    dimensions_mm: (u32, u32, u32),
}

// --- Conservation treatment ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ConservationUrgency {
    Routine,
    Priority,
    Emergency,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TreatmentType {
    Cleaning {
        solvent: String,
        method: String,
    },
    Consolidation {
        adhesive: String,
        area_cm2: u32,
    },
    Inpainting {
        pigment_system: String,
        reversible: bool,
    },
    Relining {
        fabric: String,
    },
    Deacidification {
        agent: String,
        ph_target: u8,
    },
    PestTreatment {
        method: String,
        duration_hours: u16,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConservationReport {
    report_id: u64,
    artifact_accession: String,
    urgency: ConservationUrgency,
    treatment: TreatmentType,
    conservator_name: String,
    before_condition_grade: u8,
    after_condition_grade: u8,
    cost_cents: u64,
}

// --- Provenance chain ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ProvenanceEvent {
    Creation {
        location: String,
    },
    Sale {
        auction_house: Option<String>,
        price_cents: u64,
    },
    Gift {
        donor: String,
    },
    Bequest {
        estate_name: String,
    },
    Seizure {
        authority: String,
        case_ref: String,
    },
    Restitution {
        claimant: String,
    },
    MuseumAcquisition {
        method: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProvenanceLink {
    year: i32,
    event: ProvenanceEvent,
    owner_name: String,
    location: String,
    documentation: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProvenanceChain {
    artifact_accession: String,
    links: Vec<ProvenanceLink>,
    gaps_noted: bool,
}

// --- Exhibition layout ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum GalleryZone {
    PermanentCollection,
    TemporaryExhibition,
    StudyRoom,
    OutdoorCourt,
    VirtualOnly,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MountType {
    WallHung { hook_count: u8 },
    Pedestal { height_cm: u16 },
    Vitrine { climate_controlled: bool },
    Suspended { cable_count: u8 },
    FreeStanding,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ExhibitionPlacement {
    artifact_accession: String,
    zone: GalleryZone,
    room_number: u16,
    mount: MountType,
    label_text: String,
    lux_level: u16,
}

// --- Loan agreement ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum LoanDirection {
    Incoming { lender_institution: String },
    Outgoing { borrower_institution: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum LoanStatus {
    Requested,
    Approved,
    InTransit,
    OnDisplay,
    Returned,
    Disputed,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LoanAgreement {
    loan_id: u64,
    direction: LoanDirection,
    status: LoanStatus,
    artifact_accession: String,
    start_epoch: u64,
    end_epoch: u64,
    insurance_value_cents: u64,
    courier_required: bool,
}

// --- Environmental monitoring ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum SensorAlert {
    TemperatureExcursion {
        reading_c_x100: i32,
        threshold_c_x100: i32,
    },
    HumidityExcursion {
        reading_rh_x10: u16,
        threshold_rh_x10: u16,
    },
    LightExcursion {
        lux_reading: u32,
        max_lux: u32,
    },
    VibrationDetected {
        magnitude_mg: u16,
        source: String,
    },
    NoAlert,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EnvironmentalSnapshot {
    sensor_id: String,
    room_number: u16,
    timestamp_epoch: u64,
    temp_c_x100: i32,
    humidity_rh_x10: u16,
    lux: u32,
    alert: SensorAlert,
}

// --- Digitization workflow ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum DigitizationStage {
    Queued,
    ImageCapture {
        camera_model: String,
        resolution_dpi: u32,
    },
    ColorCalibration {
        profile_name: String,
    },
    MetadataEntry {
        schema: String,
    },
    QualityReview {
        reviewer: String,
        passed: bool,
    },
    Published {
        repository_url: String,
    },
    Rejected {
        reason: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DigitizationRecord {
    artifact_accession: String,
    stage: DigitizationStage,
    file_size_bytes: u64,
    file_count: u32,
}

// --- Accession / deaccession ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum AccessionAction {
    Accessioned {
        committee_vote: bool,
        date_epoch: u64,
    },
    ProposedDeaccession {
        reason: String,
    },
    DeaccessionApproved {
        buyer: Option<String>,
        sale_price_cents: Option<u64>,
    },
    Transferred {
        receiving_institution: String,
    },
    Destroyed {
        reason: String,
        witness: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AccessionEvent {
    artifact_accession: String,
    action: AccessionAction,
    authorized_by: String,
}

// --- Authentication ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum AuthenticationMethod {
    ProvenanceResearch {
        documents_reviewed: u16,
    },
    ScientificAnalysis {
        technique: String,
        result_summary: String,
    },
    StylisticComparison {
        comparanda_count: u8,
    },
    ExpertOpinion {
        expert_name: String,
        confidence_pct: u8,
    },
    Forgery {
        evidence: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AuthenticationDossier {
    artifact_accession: String,
    methods: Vec<AuthenticationMethod>,
    conclusion_authentic: bool,
}

// --- Storage condition ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum StorageGrade {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum StorageLocation {
    OnSiteVault {
        vault_id: u16,
    },
    OffSiteFacility {
        facility_name: String,
        distance_km: u32,
    },
    OnDisplay {
        gallery_room: u16,
    },
    InTransit {
        shipment_id: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StorageAssessment {
    artifact_accession: String,
    location: StorageLocation,
    grade: StorageGrade,
    notes: Vec<String>,
}

// --- Insurance valuation ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ValuationCategory {
    FairMarketValue {
        amount_cents: u64,
    },
    ReplacementCost {
        amount_cents: u64,
    },
    AgreedValue {
        amount_cents: u64,
        policy_ref: String,
    },
    Uninsurable {
        reason: String,
    },
    PendingAppraisal,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct InsuranceValuation {
    artifact_accession: String,
    category: ValuationCategory,
    appraiser: Option<String>,
    valuation_year: u16,
}

// ===== Tests =====

#[test]
fn test_artifact_painting_roundtrip() {
    let artifact = ArtifactRecord {
        accession_number: "2024.001.0042".into(),
        title: "Moonrise over Kyoto".into(),
        artist_or_maker: Some("Hasegawa Tohaku".into()),
        creation_year: 1595,
        medium: ArtifactMedium::Painting {
            technique: "ink wash on gold leaf".into(),
            support: "six-panel folding screen".into(),
        },
        dimensions_mm: (3560, 1560, 40),
    };
    let bytes = encode_to_vec(&artifact).expect("encode painting artifact");
    let (decoded, _) =
        decode_from_slice::<ArtifactRecord>(&bytes).expect("decode painting artifact");
    assert_eq!(artifact, decoded);
}

#[test]
fn test_artifact_ceramic_no_glaze_roundtrip() {
    let artifact = ArtifactRecord {
        accession_number: "2019.087.0003".into(),
        title: "Jomon period deep bowl".into(),
        artist_or_maker: None,
        creation_year: -3000,
        medium: ArtifactMedium::Ceramic {
            clay_body: "coil-built earthenware".into(),
            glaze: None,
        },
        dimensions_mm: (280, 350, 280),
    };
    let bytes = encode_to_vec(&artifact).expect("encode unglazed ceramic");
    let (decoded, _) =
        decode_from_slice::<ArtifactRecord>(&bytes).expect("decode unglazed ceramic");
    assert_eq!(artifact, decoded);
}

#[test]
fn test_artifact_manuscript_roundtrip() {
    let artifact = ArtifactRecord {
        accession_number: "MS.2021.014".into(),
        title: "Illustrated Book of Hours".into(),
        artist_or_maker: Some("Limbourg Brothers workshop".into()),
        creation_year: 1416,
        medium: ArtifactMedium::Manuscript {
            script_type: "Gothic textura".into(),
            page_count: 206,
        },
        dimensions_mm: (292, 210, 55),
    };
    let bytes = encode_to_vec(&artifact).expect("encode manuscript");
    let (decoded, _) = decode_from_slice::<ArtifactRecord>(&bytes).expect("decode manuscript");
    assert_eq!(artifact, decoded);
}

#[test]
fn test_conservation_emergency_consolidation() {
    let report = ConservationReport {
        report_id: 88201,
        artifact_accession: "2018.055.0001".into(),
        urgency: ConservationUrgency::Emergency,
        treatment: TreatmentType::Consolidation {
            adhesive: "Paraloid B-72 in acetone".into(),
            area_cm2: 450,
        },
        conservator_name: "Dr. Yuki Tanaka".into(),
        before_condition_grade: 2,
        after_condition_grade: 6,
        cost_cents: 1_250_000,
    };
    let bytes = encode_to_vec(&report).expect("encode conservation report");
    let (decoded, _) =
        decode_from_slice::<ConservationReport>(&bytes).expect("decode conservation report");
    assert_eq!(report, decoded);
}

#[test]
fn test_conservation_routine_cleaning() {
    let report = ConservationReport {
        report_id: 90010,
        artifact_accession: "2020.003.0017".into(),
        urgency: ConservationUrgency::Routine,
        treatment: TreatmentType::Cleaning {
            solvent: "deionized water".into(),
            method: "surface swab with cotton".into(),
        },
        conservator_name: "Maria Costello".into(),
        before_condition_grade: 7,
        after_condition_grade: 9,
        cost_cents: 35_000,
    };
    let bytes = encode_to_vec(&report).expect("encode cleaning report");
    let (decoded, _) =
        decode_from_slice::<ConservationReport>(&bytes).expect("decode cleaning report");
    assert_eq!(report, decoded);
}

#[test]
fn test_provenance_chain_with_gap() {
    let chain = ProvenanceChain {
        artifact_accession: "2015.200.0008".into(),
        links: vec![
            ProvenanceLink {
                year: 1680,
                event: ProvenanceEvent::Creation {
                    location: "Delft, Netherlands".into(),
                },
                owner_name: "Johannes Vermeer estate".into(),
                location: "Delft".into(),
                documentation: vec!["estate inventory 1680".into()],
            },
            ProvenanceLink {
                year: 1696,
                event: ProvenanceEvent::Sale {
                    auction_house: Some("Amsterdam auction".into()),
                    price_cents: 0,
                },
                owner_name: "Jacob Dissius".into(),
                location: "Amsterdam".into(),
                documentation: vec![],
            },
            ProvenanceLink {
                year: 1903,
                event: ProvenanceEvent::MuseumAcquisition {
                    method: "purchase".into(),
                },
                owner_name: "National Gallery".into(),
                location: "London".into(),
                documentation: vec!["acquisition register entry".into(), "board minutes".into()],
            },
        ],
        gaps_noted: true,
    };
    let bytes = encode_to_vec(&chain).expect("encode provenance chain");
    let (decoded, _) =
        decode_from_slice::<ProvenanceChain>(&bytes).expect("decode provenance chain");
    assert_eq!(chain, decoded);
}

#[test]
fn test_provenance_restitution_event() {
    let chain = ProvenanceChain {
        artifact_accession: "REST.2023.001".into(),
        links: vec![
            ProvenanceLink {
                year: 1938,
                event: ProvenanceEvent::Seizure {
                    authority: "ERR Taskforce".into(),
                    case_ref: "ERR-1938-4421".into(),
                },
                owner_name: "confiscated".into(),
                location: "Vienna".into(),
                documentation: vec!["ERR card".into()],
            },
            ProvenanceLink {
                year: 2023,
                event: ProvenanceEvent::Restitution {
                    claimant: "Rothschild heirs".into(),
                },
                owner_name: "Rothschild family".into(),
                location: "Vienna".into(),
                documentation: vec!["court order 2023-CV-881".into()],
            },
        ],
        gaps_noted: false,
    };
    let bytes = encode_to_vec(&chain).expect("encode restitution provenance");
    let (decoded, _) =
        decode_from_slice::<ProvenanceChain>(&bytes).expect("decode restitution provenance");
    assert_eq!(chain, decoded);
}

#[test]
fn test_exhibition_vitrine_placement() {
    let placement = ExhibitionPlacement {
        artifact_accession: "2022.010.0055".into(),
        zone: GalleryZone::TemporaryExhibition,
        room_number: 304,
        mount: MountType::Vitrine {
            climate_controlled: true,
        },
        label_text: "Edo-period netsuke, ivory, 18th century".into(),
        lux_level: 50,
    };
    let bytes = encode_to_vec(&placement).expect("encode vitrine placement");
    let (decoded, _) =
        decode_from_slice::<ExhibitionPlacement>(&bytes).expect("decode vitrine placement");
    assert_eq!(placement, decoded);
}

#[test]
fn test_exhibition_wall_hung_permanent() {
    let placement = ExhibitionPlacement {
        artifact_accession: "1978.001.0001".into(),
        zone: GalleryZone::PermanentCollection,
        room_number: 101,
        mount: MountType::WallHung { hook_count: 2 },
        label_text: "Oil on canvas, gilt frame, 1889".into(),
        lux_level: 150,
    };
    let bytes = encode_to_vec(&placement).expect("encode wall-hung placement");
    let (decoded, _) =
        decode_from_slice::<ExhibitionPlacement>(&bytes).expect("decode wall-hung placement");
    assert_eq!(placement, decoded);
}

#[test]
fn test_loan_outgoing_in_transit() {
    let loan = LoanAgreement {
        loan_id: 5501,
        direction: LoanDirection::Outgoing {
            borrower_institution: "Musee du Louvre".into(),
        },
        status: LoanStatus::InTransit,
        artifact_accession: "2010.045.0012".into(),
        start_epoch: 1_700_000_000,
        end_epoch: 1_720_000_000,
        insurance_value_cents: 500_000_000,
        courier_required: true,
    };
    let bytes = encode_to_vec(&loan).expect("encode outgoing loan");
    let (decoded, _) = decode_from_slice::<LoanAgreement>(&bytes).expect("decode outgoing loan");
    assert_eq!(loan, decoded);
}

#[test]
fn test_loan_incoming_disputed() {
    let loan = LoanAgreement {
        loan_id: 6620,
        direction: LoanDirection::Incoming {
            lender_institution: "State Hermitage Museum".into(),
        },
        status: LoanStatus::Disputed,
        artifact_accession: "LOAN.2024.003".into(),
        start_epoch: 1_680_000_000,
        end_epoch: 1_700_000_000,
        insurance_value_cents: 12_000_000_000,
        courier_required: true,
    };
    let bytes = encode_to_vec(&loan).expect("encode disputed incoming loan");
    let (decoded, _) =
        decode_from_slice::<LoanAgreement>(&bytes).expect("decode disputed incoming loan");
    assert_eq!(loan, decoded);
}

#[test]
fn test_environmental_temperature_alert() {
    let snapshot = EnvironmentalSnapshot {
        sensor_id: "ENV-RM204-A".into(),
        room_number: 204,
        timestamp_epoch: 1_710_500_000,
        temp_c_x100: 2650,
        humidity_rh_x10: 580,
        lux: 48,
        alert: SensorAlert::TemperatureExcursion {
            reading_c_x100: 2650,
            threshold_c_x100: 2200,
        },
    };
    let bytes = encode_to_vec(&snapshot).expect("encode temp alert snapshot");
    let (decoded, _) =
        decode_from_slice::<EnvironmentalSnapshot>(&bytes).expect("decode temp alert snapshot");
    assert_eq!(snapshot, decoded);
}

#[test]
fn test_environmental_vibration_detected() {
    let snapshot = EnvironmentalSnapshot {
        sensor_id: "VIB-RM101-C".into(),
        room_number: 101,
        timestamp_epoch: 1_710_600_000,
        temp_c_x100: 2100,
        humidity_rh_x10: 500,
        lux: 120,
        alert: SensorAlert::VibrationDetected {
            magnitude_mg: 85,
            source: "nearby construction".into(),
        },
    };
    let bytes = encode_to_vec(&snapshot).expect("encode vibration snapshot");
    let (decoded, _) =
        decode_from_slice::<EnvironmentalSnapshot>(&bytes).expect("decode vibration snapshot");
    assert_eq!(snapshot, decoded);
}

#[test]
fn test_environmental_no_alert() {
    let snapshot = EnvironmentalSnapshot {
        sensor_id: "ENV-RM310-B".into(),
        room_number: 310,
        timestamp_epoch: 1_710_700_000,
        temp_c_x100: 2100,
        humidity_rh_x10: 500,
        lux: 45,
        alert: SensorAlert::NoAlert,
    };
    let bytes = encode_to_vec(&snapshot).expect("encode no-alert snapshot");
    let (decoded, _) =
        decode_from_slice::<EnvironmentalSnapshot>(&bytes).expect("decode no-alert snapshot");
    assert_eq!(snapshot, decoded);
}

#[test]
fn test_digitization_published_stage() {
    let record = DigitizationRecord {
        artifact_accession: "2020.088.0001".into(),
        stage: DigitizationStage::Published {
            repository_url: "https://collections.museum.org/objects/2020-088-0001".into(),
        },
        file_size_bytes: 2_400_000_000,
        file_count: 47,
    };
    let bytes = encode_to_vec(&record).expect("encode published digitization");
    let (decoded, _) =
        decode_from_slice::<DigitizationRecord>(&bytes).expect("decode published digitization");
    assert_eq!(record, decoded);
}

#[test]
fn test_digitization_quality_review_failed() {
    let record = DigitizationRecord {
        artifact_accession: "2019.055.0009".into(),
        stage: DigitizationStage::QualityReview {
            reviewer: "Kenji Murakami".into(),
            passed: false,
        },
        file_size_bytes: 980_000_000,
        file_count: 12,
    };
    let bytes = encode_to_vec(&record).expect("encode failed QA digitization");
    let (decoded, _) =
        decode_from_slice::<DigitizationRecord>(&bytes).expect("decode failed QA digitization");
    assert_eq!(record, decoded);
}

#[test]
fn test_accession_with_committee_vote() {
    let event = AccessionEvent {
        artifact_accession: "2025.001.0001".into(),
        action: AccessionAction::Accessioned {
            committee_vote: true,
            date_epoch: 1_740_000_000,
        },
        authorized_by: "Director Elena Vasquez".into(),
    };
    let bytes = encode_to_vec(&event).expect("encode accession event");
    let (decoded, _) = decode_from_slice::<AccessionEvent>(&bytes).expect("decode accession event");
    assert_eq!(event, decoded);
}

#[test]
fn test_deaccession_sold() {
    let event = AccessionEvent {
        artifact_accession: "1985.040.0022".into(),
        action: AccessionAction::DeaccessionApproved {
            buyer: Some("Private collector, Geneva".into()),
            sale_price_cents: Some(7_500_000_00),
        },
        authorized_by: "Board resolution 2025-03".into(),
    };
    let bytes = encode_to_vec(&event).expect("encode deaccession sale");
    let (decoded, _) =
        decode_from_slice::<AccessionEvent>(&bytes).expect("decode deaccession sale");
    assert_eq!(event, decoded);
}

#[test]
fn test_authentication_multiple_methods() {
    let dossier = AuthenticationDossier {
        artifact_accession: "AUTH.2024.077".into(),
        methods: vec![
            AuthenticationMethod::ProvenanceResearch {
                documents_reviewed: 34,
            },
            AuthenticationMethod::ScientificAnalysis {
                technique: "X-ray fluorescence".into(),
                result_summary: "pigment composition consistent with 17th century Dutch palette"
                    .into(),
            },
            AuthenticationMethod::StylisticComparison {
                comparanda_count: 12,
            },
            AuthenticationMethod::ExpertOpinion {
                expert_name: "Prof. Hans van der Berg".into(),
                confidence_pct: 95,
            },
        ],
        conclusion_authentic: true,
    };
    let bytes = encode_to_vec(&dossier).expect("encode authentication dossier");
    let (decoded, _) =
        decode_from_slice::<AuthenticationDossier>(&bytes).expect("decode authentication dossier");
    assert_eq!(dossier, decoded);
}

#[test]
fn test_authentication_forgery_detected() {
    let dossier = AuthenticationDossier {
        artifact_accession: "SUSPECT.2024.002".into(),
        methods: vec![
            AuthenticationMethod::ScientificAnalysis {
                technique: "carbon-14 dating".into(),
                result_summary: "canvas dates to 1920s, not 16th century".into(),
            },
            AuthenticationMethod::Forgery {
                evidence: "anachronistic synthetic pigment titanium white detected".into(),
            },
        ],
        conclusion_authentic: false,
    };
    let bytes = encode_to_vec(&dossier).expect("encode forgery dossier");
    let (decoded, _) =
        decode_from_slice::<AuthenticationDossier>(&bytes).expect("decode forgery dossier");
    assert_eq!(dossier, decoded);
}

#[test]
fn test_storage_offsite_poor_grade() {
    let assessment = StorageAssessment {
        artifact_accession: "1960.110.0044".into(),
        location: StorageLocation::OffSiteFacility {
            facility_name: "Regional Conservation Center".into(),
            distance_km: 85,
        },
        grade: StorageGrade::Poor,
        notes: vec![
            "visible mold on mounting board".into(),
            "acid-free tissue replacement needed".into(),
            "container does not meet archival standards".into(),
        ],
    };
    let bytes = encode_to_vec(&assessment).expect("encode poor storage assessment");
    let (decoded, _) =
        decode_from_slice::<StorageAssessment>(&bytes).expect("decode poor storage assessment");
    assert_eq!(assessment, decoded);
}

#[test]
fn test_insurance_agreed_value_and_valuation() {
    let valuation = InsuranceValuation {
        artifact_accession: "2005.300.0001".into(),
        category: ValuationCategory::AgreedValue {
            amount_cents: 45_000_000_00,
            policy_ref: "INS-2025-AV-0091".into(),
        },
        appraiser: Some("Christie's Appraisals Ltd.".into()),
        valuation_year: 2025,
    };
    let bytes = encode_to_vec(&valuation).expect("encode agreed-value insurance");
    let (decoded, _) =
        decode_from_slice::<InsuranceValuation>(&bytes).expect("decode agreed-value insurance");
    assert_eq!(valuation, decoded);
}
