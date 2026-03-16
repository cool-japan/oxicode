//! Advanced complex enum tests for OxiCode — museum curation and exhibit management domain.
//! 22 test functions covering deeply nested enums, enums with named fields, and enums containing enums.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Medium {
    OilOnCanvas,
    Acrylic,
    Watercolor,
    Fresco,
    Tempera,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SculptureMaterial {
    Marble,
    Bronze,
    Wood,
    Glass,
    MixedMedia { components: Vec<String> },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InstallationType {
    Immersive {
        room_count: u8,
        requires_darkness: bool,
    },
    Interactive {
        sensor_count: u16,
    },
    SiteSpecific {
        location_description: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DigitalFormat {
    Video { codec: String, duration_secs: u32 },
    Generative { algorithm: String, seed: u64 },
    Nft { token_id: String, chain: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TextileTechnique {
    Weaving,
    Embroidery,
    Dyeing { method: String },
    Felting,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ArtworkClassification {
    Painting {
        medium: Medium,
        width_cm: u32,
        height_cm: u32,
    },
    Sculpture {
        material: SculptureMaterial,
        weight_kg: f64,
        height_cm: u32,
    },
    Installation(InstallationType),
    Digital(DigitalFormat),
    Textile {
        technique: TextileTechnique,
        width_cm: u32,
        height_cm: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProvenanceEntry {
    year: u16,
    owner: String,
    location: String,
    verified: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProvenanceChain {
    Unknown,
    Partial {
        entries: Vec<ProvenanceEntry>,
        gap_years: Vec<u16>,
    },
    Complete {
        entries: Vec<ProvenanceEntry>,
    },
    Disputed {
        entries: Vec<ProvenanceEntry>,
        dispute_note: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DamageType {
    Crack { length_mm: u32, depth_mm: u32 },
    Discoloration { area_sq_cm: u32 },
    Tear { length_mm: u32 },
    WaterDamage { severity_percent: u8 },
    PestDamage { species: String },
    Structural { description: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConservationCondition {
    Excellent,
    Good {
        minor_notes: Vec<String>,
    },
    Fair {
        damages: Vec<DamageType>,
    },
    Poor {
        damages: Vec<DamageType>,
        urgent: bool,
        estimated_cost_cents: u64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WallPlacement {
    North { offset_cm: u32 },
    South { offset_cm: u32 },
    East { offset_cm: u32 },
    West { offset_cm: u32 },
    Center,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LightingType {
    Spotlight { lumens: u32, color_temp_k: u16 },
    Ambient,
    Natural { uv_filtered: bool },
    Led { lumens: u32, dimmable: bool },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExhibitSlot {
    artwork_id: u64,
    wall: WallPlacement,
    lighting: LightingType,
    label_text: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExhibitionLayout {
    SingleRoom {
        slots: Vec<ExhibitSlot>,
    },
    MultiRoom {
        rooms: Vec<Vec<ExhibitSlot>>,
    },
    Touring {
        current_venue: String,
        remaining_venues: Vec<String>,
        slots: Vec<ExhibitSlot>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LoanDuration {
    FixedMonths(u16),
    Indefinite,
    Renewable {
        initial_months: u16,
        max_renewals: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LoanAgreement {
    Outgoing {
        artwork_id: u64,
        borrower_institution: String,
        duration: LoanDuration,
        insurance_value_cents: u64,
        courier_required: bool,
    },
    Incoming {
        artwork_id: u64,
        lender_institution: String,
        duration: LoanDuration,
        insurance_value_cents: u64,
        climate_requirements: ClimateSpec,
    },
    Exchange {
        outgoing_ids: Vec<u64>,
        incoming_ids: Vec<u64>,
        partner_institution: String,
        duration: LoanDuration,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClimateSpec {
    temp_min_c: i8,
    temp_max_c: i8,
    humidity_min_pct: u8,
    humidity_max_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StorageZone {
    GeneralVault {
        zone_id: u16,
    },
    ClimateControlled {
        zone_id: u16,
        spec: ClimateSpec,
    },
    ColdStorage {
        zone_id: u16,
        temp_c: i8,
    },
    HazardousMaterials {
        zone_id: u16,
        material_class: String,
    },
    Offsite {
        facility_name: String,
        zone_id: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VisitorDemographic {
    Child,
    Adult,
    Senior,
    Student,
    GroupTour { group_size: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VisitorAnalytics {
    exhibit_id: u64,
    date_ordinal: u32,
    demographics: Vec<VisitorDemographic>,
    avg_dwell_secs: u32,
    satisfaction_score: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AudioGuideContent {
    Narration {
        script: String,
        language: String,
        duration_secs: u16,
    },
    Interview {
        speaker: String,
        topic: String,
        duration_secs: u16,
    },
    AmbientSound {
        description: String,
        looping: bool,
    },
    ChildFriendly {
        script: String,
        language: String,
        age_range_min: u8,
        age_range_max: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CurrencyCode {
    Usd,
    Eur,
    Gbp,
    Jpy,
    Other(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InsuranceValuation {
    SingleItem {
        artwork_id: u64,
        value_cents: u64,
        currency: CurrencyCode,
    },
    Collection {
        artwork_ids: Vec<u64>,
        total_value_cents: u64,
        currency: CurrencyCode,
    },
    InTransit {
        artwork_id: u64,
        value_cents: u64,
        currency: CurrencyCode,
        origin: String,
        destination: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RestorationTechnique {
    Cleaning { solvent: String },
    Inpainting { area_sq_cm: u32 },
    Relining,
    StructuralRepair { description: String },
    DigitalReconstruction { software: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RestorationIntervention {
    Planned {
        artwork_id: u64,
        techniques: Vec<RestorationTechnique>,
        estimated_days: u16,
        estimated_cost_cents: u64,
    },
    InProgress {
        artwork_id: u64,
        techniques: Vec<RestorationTechnique>,
        days_elapsed: u16,
        notes: Vec<String>,
    },
    Completed {
        artwork_id: u64,
        techniques: Vec<RestorationTechnique>,
        total_days: u16,
        total_cost_cents: u64,
        condition_after: ConservationCondition,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ArchiveFormat {
    HighResTiff {
        width_px: u32,
        height_px: u32,
        bit_depth: u8,
    },
    Raw3dScan {
        polygon_count: u64,
        file_size_bytes: u64,
    },
    MultispectralImage {
        bands: Vec<String>,
        width_px: u32,
        height_px: u32,
    },
    DocumentPdf {
        page_count: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GiftShopCategory {
    Print {
        artwork_id: u64,
        size: String,
    },
    Book {
        title: String,
        author: String,
    },
    Replica {
        artwork_id: u64,
        scale_percent: u8,
    },
    Apparel {
        description: String,
        sizes: Vec<String>,
    },
    Accessory {
        description: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GiftShopItem {
    sku: String,
    category: GiftShopCategory,
    price_cents: u32,
    stock_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MembershipTier {
    Individual {
        annual_fee_cents: u32,
    },
    Family {
        annual_fee_cents: u32,
        max_members: u8,
    },
    Patron {
        annual_fee_cents: u32,
        benefits: Vec<String>,
    },
    Corporate {
        annual_fee_cents: u32,
        company_name: String,
        employee_limit: u16,
    },
    LifetimeBenefactor {
        donation_cents: u64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EducationalProgram {
    Workshop {
        title: String,
        age_min: u8,
        age_max: u8,
        capacity: u16,
    },
    Lecture {
        title: String,
        speaker: String,
        duration_mins: u16,
    },
    GuidedTour {
        exhibit_id: u64,
        language: String,
        max_group: u16,
    },
    OnlineCourse {
        title: String,
        module_count: u8,
        platform: String,
    },
    SchoolPartnership {
        school_name: String,
        programs: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AccessibilityAccommodation {
    Wheelchair {
        ramp_available: bool,
        elevator_access: bool,
    },
    VisualImpairment {
        tactile_models: Vec<u64>,
        braille_labels: bool,
        audio_described: bool,
    },
    HearingImpairment {
        sign_language_tours: bool,
        captioned_media: bool,
    },
    Cognitive {
        easy_read_guides: bool,
        quiet_hours: Vec<String>,
    },
    Multilingual {
        languages: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MuseumExhibit {
    exhibit_id: u64,
    title: String,
    classification: ArtworkClassification,
    provenance: ProvenanceChain,
    condition: ConservationCondition,
    storage: StorageZone,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_painting_classification_roundtrip() {
    let val = ArtworkClassification::Painting {
        medium: Medium::OilOnCanvas,
        width_cm: 120,
        height_cm: 80,
    };
    let bytes = encode_to_vec(&val).expect("encode painting");
    let (decoded, _): (ArtworkClassification, usize) =
        decode_from_slice(&bytes).expect("decode painting");
    assert_eq!(val, decoded);
}

#[test]
fn test_sculpture_mixed_media_roundtrip() {
    let val = ArtworkClassification::Sculpture {
        material: SculptureMaterial::MixedMedia {
            components: vec!["steel".into(), "resin".into(), "copper wire".into()],
        },
        weight_kg: 340.5,
        height_cm: 210,
    };
    let bytes = encode_to_vec(&val).expect("encode sculpture");
    let (decoded, _): (ArtworkClassification, usize) =
        decode_from_slice(&bytes).expect("decode sculpture");
    assert_eq!(val, decoded);
}

#[test]
fn test_immersive_installation_roundtrip() {
    let val = ArtworkClassification::Installation(InstallationType::Immersive {
        room_count: 3,
        requires_darkness: true,
    });
    let bytes = encode_to_vec(&val).expect("encode installation");
    let (decoded, _): (ArtworkClassification, usize) =
        decode_from_slice(&bytes).expect("decode installation");
    assert_eq!(val, decoded);
}

#[test]
fn test_digital_generative_roundtrip() {
    let val = ArtworkClassification::Digital(DigitalFormat::Generative {
        algorithm: "perlin_noise_fractal".into(),
        seed: 0xDEAD_BEEF_CAFE,
    });
    let bytes = encode_to_vec(&val).expect("encode digital");
    let (decoded, _): (ArtworkClassification, usize) =
        decode_from_slice(&bytes).expect("decode digital");
    assert_eq!(val, decoded);
}

#[test]
fn test_textile_dyeing_roundtrip() {
    let val = ArtworkClassification::Textile {
        technique: TextileTechnique::Dyeing {
            method: "shibori".into(),
        },
        width_cm: 150,
        height_cm: 200,
    };
    let bytes = encode_to_vec(&val).expect("encode textile");
    let (decoded, _): (ArtworkClassification, usize) =
        decode_from_slice(&bytes).expect("decode textile");
    assert_eq!(val, decoded);
}

#[test]
fn test_disputed_provenance_chain_roundtrip() {
    let val = ProvenanceChain::Disputed {
        entries: vec![
            ProvenanceEntry {
                year: 1893,
                owner: "Galerie Durand-Ruel".into(),
                location: "Paris".into(),
                verified: true,
            },
            ProvenanceEntry {
                year: 1941,
                owner: "Unknown".into(),
                location: "Berlin".into(),
                verified: false,
            },
            ProvenanceEntry {
                year: 1962,
                owner: "Private Collection".into(),
                location: "New York".into(),
                verified: true,
            },
        ],
        dispute_note: "Wartime gap 1941-1962 under investigation".into(),
    };
    let bytes = encode_to_vec(&val).expect("encode provenance");
    let (decoded, _): (ProvenanceChain, usize) =
        decode_from_slice(&bytes).expect("decode provenance");
    assert_eq!(val, decoded);
}

#[test]
fn test_poor_condition_multiple_damages_roundtrip() {
    let val = ConservationCondition::Poor {
        damages: vec![
            DamageType::Crack {
                length_mm: 85,
                depth_mm: 3,
            },
            DamageType::WaterDamage {
                severity_percent: 40,
            },
            DamageType::PestDamage {
                species: "Anobium punctatum".into(),
            },
        ],
        urgent: true,
        estimated_cost_cents: 450_000_00,
    };
    let bytes = encode_to_vec(&val).expect("encode condition");
    let (decoded, _): (ConservationCondition, usize) =
        decode_from_slice(&bytes).expect("decode condition");
    assert_eq!(val, decoded);
}

#[test]
fn test_multi_room_exhibition_layout_roundtrip() {
    let slot_a = ExhibitSlot {
        artwork_id: 1001,
        wall: WallPlacement::North { offset_cm: 150 },
        lighting: LightingType::Spotlight {
            lumens: 800,
            color_temp_k: 3000,
        },
        label_text: "Monet, Water Lilies, 1906".into(),
    };
    let slot_b = ExhibitSlot {
        artwork_id: 1002,
        wall: WallPlacement::Center,
        lighting: LightingType::Ambient,
        label_text: "Rodin, The Thinker (bronze cast)".into(),
    };
    let val = ExhibitionLayout::MultiRoom {
        rooms: vec![vec![slot_a], vec![slot_b]],
    };
    let bytes = encode_to_vec(&val).expect("encode layout");
    let (decoded, _): (ExhibitionLayout, usize) = decode_from_slice(&bytes).expect("decode layout");
    assert_eq!(val, decoded);
}

#[test]
fn test_touring_exhibition_roundtrip() {
    let slot = ExhibitSlot {
        artwork_id: 2001,
        wall: WallPlacement::East { offset_cm: 200 },
        lighting: LightingType::Led {
            lumens: 600,
            dimmable: true,
        },
        label_text: "Kusama, Infinity Mirrored Room".into(),
    };
    let val = ExhibitionLayout::Touring {
        current_venue: "Tate Modern, London".into(),
        remaining_venues: vec!["MoMA, New York".into(), "Centre Pompidou, Paris".into()],
        slots: vec![slot],
    };
    let bytes = encode_to_vec(&val).expect("encode touring");
    let (decoded, _): (ExhibitionLayout, usize) =
        decode_from_slice(&bytes).expect("decode touring");
    assert_eq!(val, decoded);
}

#[test]
fn test_loan_agreement_exchange_roundtrip() {
    let val = LoanAgreement::Exchange {
        outgoing_ids: vec![101, 102],
        incoming_ids: vec![501, 502, 503],
        partner_institution: "Rijksmuseum, Amsterdam".into(),
        duration: LoanDuration::Renewable {
            initial_months: 12,
            max_renewals: 3,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode loan exchange");
    let (decoded, _): (LoanAgreement, usize) =
        decode_from_slice(&bytes).expect("decode loan exchange");
    assert_eq!(val, decoded);
}

#[test]
fn test_incoming_loan_with_climate_roundtrip() {
    let val = LoanAgreement::Incoming {
        artwork_id: 777,
        lender_institution: "Uffizi Gallery, Florence".into(),
        duration: LoanDuration::FixedMonths(6),
        insurance_value_cents: 50_000_000_00,
        climate_requirements: ClimateSpec {
            temp_min_c: 18,
            temp_max_c: 22,
            humidity_min_pct: 45,
            humidity_max_pct: 55,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode incoming loan");
    let (decoded, _): (LoanAgreement, usize) =
        decode_from_slice(&bytes).expect("decode incoming loan");
    assert_eq!(val, decoded);
}

#[test]
fn test_climate_controlled_storage_roundtrip() {
    let val = StorageZone::ClimateControlled {
        zone_id: 14,
        spec: ClimateSpec {
            temp_min_c: 15,
            temp_max_c: 20,
            humidity_min_pct: 40,
            humidity_max_pct: 50,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode storage");
    let (decoded, _): (StorageZone, usize) = decode_from_slice(&bytes).expect("decode storage");
    assert_eq!(val, decoded);
}

#[test]
fn test_visitor_analytics_with_demographics_roundtrip() {
    let val = VisitorAnalytics {
        exhibit_id: 42,
        date_ordinal: 20260315,
        demographics: vec![
            VisitorDemographic::Adult,
            VisitorDemographic::Adult,
            VisitorDemographic::Child,
            VisitorDemographic::Senior,
            VisitorDemographic::GroupTour { group_size: 25 },
            VisitorDemographic::Student,
        ],
        avg_dwell_secs: 187,
        satisfaction_score: 88,
    };
    let bytes = encode_to_vec(&val).expect("encode analytics");
    let (decoded, _): (VisitorAnalytics, usize) =
        decode_from_slice(&bytes).expect("decode analytics");
    assert_eq!(val, decoded);
}

#[test]
fn test_audio_guide_child_friendly_roundtrip() {
    let val = AudioGuideContent::ChildFriendly {
        script: "Welcome young explorers! This painting shows a garden full of flowers.".into(),
        language: "en".into(),
        age_range_min: 4,
        age_range_max: 10,
    };
    let bytes = encode_to_vec(&val).expect("encode audio guide");
    let (decoded, _): (AudioGuideContent, usize) =
        decode_from_slice(&bytes).expect("decode audio guide");
    assert_eq!(val, decoded);
}

#[test]
fn test_insurance_in_transit_roundtrip() {
    let val = InsuranceValuation::InTransit {
        artwork_id: 3001,
        value_cents: 120_000_000_00,
        currency: CurrencyCode::Gbp,
        origin: "National Gallery, London".into(),
        destination: "Metropolitan Museum, New York".into(),
    };
    let bytes = encode_to_vec(&val).expect("encode insurance");
    let (decoded, _): (InsuranceValuation, usize) =
        decode_from_slice(&bytes).expect("decode insurance");
    assert_eq!(val, decoded);
}

#[test]
fn test_completed_restoration_roundtrip() {
    let val = RestorationIntervention::Completed {
        artwork_id: 8080,
        techniques: vec![
            RestorationTechnique::Cleaning {
                solvent: "isopropanol 5%".into(),
            },
            RestorationTechnique::Inpainting { area_sq_cm: 12 },
            RestorationTechnique::Relining,
        ],
        total_days: 45,
        total_cost_cents: 75_000_00,
        condition_after: ConservationCondition::Good {
            minor_notes: vec!["slight varnish unevenness in lower left corner".into()],
        },
    };
    let bytes = encode_to_vec(&val).expect("encode restoration");
    let (decoded, _): (RestorationIntervention, usize) =
        decode_from_slice(&bytes).expect("decode restoration");
    assert_eq!(val, decoded);
}

#[test]
fn test_multispectral_archive_format_roundtrip() {
    let val = ArchiveFormat::MultispectralImage {
        bands: vec![
            "visible_rgb".into(),
            "near_infrared".into(),
            "ultraviolet".into(),
            "x_ray".into(),
        ],
        width_px: 8192,
        height_px: 6144,
    };
    let bytes = encode_to_vec(&val).expect("encode archive");
    let (decoded, _): (ArchiveFormat, usize) = decode_from_slice(&bytes).expect("decode archive");
    assert_eq!(val, decoded);
}

#[test]
fn test_gift_shop_replica_item_roundtrip() {
    let val = GiftShopItem {
        sku: "GS-REP-0042".into(),
        category: GiftShopCategory::Replica {
            artwork_id: 1001,
            scale_percent: 25,
        },
        price_cents: 4500,
        stock_count: 120,
    };
    let bytes = encode_to_vec(&val).expect("encode gift shop item");
    let (decoded, _): (GiftShopItem, usize) =
        decode_from_slice(&bytes).expect("decode gift shop item");
    assert_eq!(val, decoded);
}

#[test]
fn test_corporate_membership_tier_roundtrip() {
    let val = MembershipTier::Corporate {
        annual_fee_cents: 25_000_00,
        company_name: "Artisan Technologies Ltd".into(),
        employee_limit: 500,
    };
    let bytes = encode_to_vec(&val).expect("encode membership");
    let (decoded, _): (MembershipTier, usize) =
        decode_from_slice(&bytes).expect("decode membership");
    assert_eq!(val, decoded);
}

#[test]
fn test_school_partnership_program_roundtrip() {
    let val = EducationalProgram::SchoolPartnership {
        school_name: "Tokyo International School".into(),
        programs: vec![
            "Weekly art workshop".into(),
            "Annual exhibition visit".into(),
            "Student curator program".into(),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode educational program");
    let (decoded, _): (EducationalProgram, usize) =
        decode_from_slice(&bytes).expect("decode educational program");
    assert_eq!(val, decoded);
}

#[test]
fn test_visual_impairment_accessibility_roundtrip() {
    let val = AccessibilityAccommodation::VisualImpairment {
        tactile_models: vec![1001, 1002, 1050],
        braille_labels: true,
        audio_described: true,
    };
    let bytes = encode_to_vec(&val).expect("encode accessibility");
    let (decoded, _): (AccessibilityAccommodation, usize) =
        decode_from_slice(&bytes).expect("decode accessibility");
    assert_eq!(val, decoded);
}

#[test]
fn test_full_museum_exhibit_deeply_nested_roundtrip() {
    let exhibit = MuseumExhibit {
        exhibit_id: 9999,
        title: "The Persistence of Memory".into(),
        classification: ArtworkClassification::Painting {
            medium: Medium::OilOnCanvas,
            width_cm: 33,
            height_cm: 24,
        },
        provenance: ProvenanceChain::Complete {
            entries: vec![
                ProvenanceEntry {
                    year: 1931,
                    owner: "Salvador Dali".into(),
                    location: "Figueres, Spain".into(),
                    verified: true,
                },
                ProvenanceEntry {
                    year: 1934,
                    owner: "Museum of Modern Art".into(),
                    location: "New York".into(),
                    verified: true,
                },
            ],
        },
        condition: ConservationCondition::Fair {
            damages: vec![DamageType::Discoloration { area_sq_cm: 2 }],
        },
        storage: StorageZone::ClimateControlled {
            zone_id: 1,
            spec: ClimateSpec {
                temp_min_c: 19,
                temp_max_c: 21,
                humidity_min_pct: 48,
                humidity_max_pct: 52,
            },
        },
    };
    let bytes = encode_to_vec(&exhibit).expect("encode full exhibit");
    let (decoded, _): (MuseumExhibit, usize) =
        decode_from_slice(&bytes).expect("decode full exhibit");
    assert_eq!(exhibit, decoded);
}
