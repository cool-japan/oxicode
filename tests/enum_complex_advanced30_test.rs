//! Advanced complex enum tests for underwater archaeological survey domain types.
//! 22 test functions covering deeply nested enums, enums with named fields,
//! and enums containing other enums.

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

// ── Shipwreck site classifications ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VesselEra {
    Ancient {
        period_name: String,
        approx_year_bce: i32,
    },
    Medieval {
        century: u8,
    },
    EarlyModern {
        year_range_start: u16,
        year_range_end: u16,
    },
    Modern {
        exact_year: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CargoType {
    Amphorae {
        count_estimate: u32,
        contents: String,
    },
    Bullion {
        metal: String,
        weight_kg: f64,
    },
    Armament {
        weapon_type: String,
        quantity: u32,
    },
    MixedGoods {
        manifest_entries: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ShipwreckClass {
    Merchant {
        cargo: CargoType,
        era: VesselEra,
        tonnage_estimate: u32,
        trade_route: String,
    },
    Warship {
        era: VesselEra,
        gun_count: u16,
        navy: String,
        battle_context: Option<String>,
    },
    Submarine {
        designation: String,
        launch_year: u16,
        cause_of_loss: String,
        crew_size: u16,
    },
    Aircraft {
        aircraft_type: String,
        mission: String,
        crash_year: u16,
        pilot_identified: bool,
    },
}

// ── Artifact recovery methods ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AirliftSpec {
    WaterDredge {
        diameter_cm: u8,
        flow_rate_lpm: f64,
    },
    AirDredge {
        compressor_psi: u16,
        hose_length_m: f32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RecoveryMethod {
    Manual {
        diver_id: String,
        tool: String,
        duration_minutes: u32,
    },
    Airlift {
        spec: AirliftSpec,
        operator: String,
    },
    Mechanical {
        crane_capacity_tonnes: f64,
        rigging_type: String,
        lift_bag_count: u8,
    },
}

// ── Preservation states ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CorrosionProduct {
    IronOxide { thickness_mm: f32 },
    CopperCarbonate { coverage_percent: f32 },
    LeadWhite,
    Mixed { components: Vec<String> },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PreservationState {
    Intact {
        structural_integrity_pct: f32,
        surface_condition: String,
    },
    Fragmented {
        fragment_count: u32,
        largest_fragment_cm: f64,
        reassembly_feasible: bool,
    },
    Corroded {
        product: CorrosionProduct,
        depth_mm: f32,
        salvageable: bool,
    },
    Encrusted {
        organism_types: Vec<String>,
        encrustation_thickness_mm: f32,
        removal_method: String,
    },
}

// ── Sonar survey types ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SonarFrequencyBand {
    Low { khz: f32 },
    Medium { khz: f32 },
    High { khz: f32 },
    DualFrequency { low_khz: f32, high_khz: f32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SonarSurvey {
    SideScan {
        frequency: SonarFrequencyBand,
        swath_width_m: f64,
        tow_speed_knots: f32,
        resolution_cm: f32,
    },
    Multibeam {
        frequency: SonarFrequencyBand,
        beam_count: u16,
        coverage_area_sqm: f64,
    },
    SubBottom {
        frequency: SonarFrequencyBand,
        penetration_depth_m: f32,
        layer_count_detected: u8,
    },
}

// ── Dive operation profiles ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreathingMix {
    Air,
    Nitrox { oxygen_pct: f32 },
    Trimix { oxygen_pct: f32, helium_pct: f32 },
    Heliox { oxygen_pct: f32, helium_pct: f32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DiveProfile {
    Scuba {
        max_depth_m: f32,
        bottom_time_min: u32,
        breathing_mix: BreathingMix,
        deco_stops: Vec<(f32, u32)>,
    },
    Rov {
        model: String,
        max_depth_m: f64,
        manipulator_arms: u8,
        camera_count: u8,
        tether_length_m: f64,
    },
    Auv {
        model: String,
        mission_duration_hours: f32,
        waypoint_count: u16,
        sensor_suite: Vec<String>,
    },
    Saturation {
        habitat_depth_m: f32,
        breathing_mix: BreathingMix,
        exposure_days: u16,
        decompression_hours: u32,
    },
}

// ── Dating methods ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DatingMethod {
    Dendrochronology {
        ring_count: u32,
        species: String,
        reference_chronology: String,
        felling_year: Option<i32>,
    },
    Radiocarbon {
        sample_material: String,
        raw_age_bp: u32,
        sigma_years: u16,
        calibrated_range_start: i32,
        calibrated_range_end: i32,
    },
    PotteryTypology {
        style: String,
        production_center: String,
        date_range: (i32, i32),
    },
    CoinIdentification {
        denomination: String,
        ruler: String,
        mint: String,
        issue_year: i32,
    },
}

// ── Hull construction materials ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WoodSpecies {
    Oak,
    Teak,
    Pine,
    Cedar,
    Other { name: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FastenerType {
    IronBolt { diameter_mm: f32 },
    CopperRivet { length_mm: f32 },
    WoodenTreenail { diameter_mm: f32 },
    BronzeNail,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HullMaterial {
    Wood {
        primary_species: WoodSpecies,
        plank_thickness_cm: f32,
        fastener: FastenerType,
        sheathing: Option<String>,
    },
    Iron {
        plate_thickness_mm: f32,
        rivet_spacing_cm: f32,
        corrosion_state: PreservationState,
    },
    Composite {
        frame_material: String,
        skin_material: String,
        fastener: FastenerType,
    },
}

// ── Navigation instruments ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NavigationInstrument {
    Compass {
        material: String,
        diameter_cm: f32,
        preservation: PreservationState,
    },
    Astrolabe {
        material: String,
        diameter_cm: f32,
        rete_stars_count: u8,
    },
    Sextant {
        maker: String,
        serial_number: Option<String>,
        arc_material: String,
    },
    Dividers {
        leg_length_cm: f32,
        material: String,
    },
    SoundingLead {
        weight_kg: f32,
        tallow_cup: bool,
    },
}

// ── Conservation treatment ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChemicalTreatment {
    Desalination {
        bath_changes: u16,
        conductivity_target: f32,
    },
    Peg {
        concentration_pct: f32,
        duration_months: u16,
    },
    SodiumSesquicarbonate {
        concentration_pct: f32,
    },
    ElectrolyticReduction {
        current_ma: f32,
        duration_hours: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConservationTreatment {
    Chemical {
        treatment: ChemicalTreatment,
        temperature_c: f32,
        monitored_by: String,
    },
    FreezeDrying {
        pre_treatment: Option<ChemicalTreatment>,
        chamber_temp_c: f32,
        vacuum_mbar: f32,
        duration_days: u16,
    },
    Consolidation {
        resin_type: String,
        application_method: String,
        layers: u8,
    },
    Storage {
        environment: String,
        humidity_pct: f32,
        temperature_c: f32,
        inert_gas: Option<String>,
    },
}

// ── 3D photogrammetry ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PhotogrammetryModel {
    SiteOverview {
        image_count: u32,
        ground_resolution_mm: f32,
        area_sqm: f64,
        coordinate_system: String,
    },
    ArtifactScan {
        image_count: u32,
        vertex_count: u64,
        texture_resolution_px: u32,
        scale_bar_used: bool,
    },
    TimeSeries {
        surveys: Vec<(String, u32)>,
        baseline_date: String,
        change_detected: bool,
    },
}

// ── Legal jurisdiction ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum JurisdictionClaim {
    TerritorialWaters {
        state: String,
        distance_nm: f32,
    },
    ExclusiveEconomicZone {
        state: String,
        distance_nm: f32,
    },
    InternationalWaters {
        convention: String,
        managing_body: String,
    },
    FlagState {
        original_flag: String,
        successor_state: Option<String>,
        claim_basis: String,
    },
    Contested {
        claimants: Vec<String>,
        dispute_summary: String,
        arbitration_body: Option<String>,
    },
}

// ── Environmental impact monitoring ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BiologicalIndicator {
    CoralCoverage {
        species: String,
        coverage_pct: f32,
    },
    InvasiveSpecies {
        species: String,
        density_per_sqm: f32,
    },
    Biodiversity {
        shannon_index: f32,
        species_count: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnvironmentalMonitoring {
    SedimentAnalysis {
        grain_size_mm: f32,
        organic_content_pct: f32,
        contaminant_levels: Vec<(String, f64)>,
    },
    WaterQuality {
        temperature_c: f32,
        salinity_ppt: f32,
        dissolved_oxygen_ppm: f32,
        turbidity_ntu: f32,
    },
    BiologicalSurvey {
        indicator: BiologicalIndicator,
        survey_date: String,
        quadrat_size_sqm: f32,
    },
    CurrentMeasurement {
        speed_cm_per_s: f32,
        direction_deg: f32,
        depth_m: f32,
        instrument: String,
    },
}

// ── Public outreach / museum displays ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DisplayMedium {
    PhysicalArtifact {
        case_type: String,
        climate_controlled: bool,
        lighting_lux: u16,
    },
    DigitalInteractive {
        screen_count: u8,
        model_3d: Option<PhotogrammetryModel>,
        languages: Vec<String>,
    },
    VirtualReality {
        headset_model: String,
        scene_duration_min: u8,
        narration_languages: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PublicOutreach {
    MuseumExhibit {
        name: String,
        venue: String,
        display: DisplayMedium,
        visitor_capacity: u32,
    },
    Publication {
        title: String,
        authors: Vec<String>,
        journal: Option<String>,
        year: u16,
    },
    DivingTrail {
        site_name: String,
        marker_count: u8,
        max_depth_m: f32,
        certification_required: String,
    },
}

// ── Composite survey record ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SurveyRecord {
    site_id: String,
    wreck: ShipwreckClass,
    sonar: SonarSurvey,
    dive: DiveProfile,
    dating: DatingMethod,
    hull: HullMaterial,
    jurisdiction: JurisdictionClaim,
}

// ── Trade route analysis ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TradeRouteEvidence {
    CargoOrigin {
        artifact_type: String,
        origin_region: String,
        dating: DatingMethod,
    },
    NavigationClue {
        instrument: NavigationInstrument,
        implied_route: String,
    },
    DocumentarySource {
        archive: String,
        document_type: String,
        reference: String,
    },
    IsotopicAnalysis {
        element: String,
        ratio: f64,
        source_match: String,
    },
}

// ── Cargo manifest reconstruction ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ManifestEntry {
    Identified {
        item: String,
        quantity: u32,
        preservation: PreservationState,
        recovery: RecoveryMethod,
    },
    Partial {
        description: String,
        estimated_quantity_range: (u32, u32),
        confidence_pct: f32,
    },
    Inferred {
        evidence_type: String,
        likely_contents: Vec<String>,
        supporting_parallels: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CargoManifest {
    wreck_name: String,
    entries: Vec<ManifestEntry>,
    completeness_pct: f32,
}

// ═══════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════

#[test]
fn test_01_merchant_shipwreck_with_amphorae_cargo() {
    let wreck = ShipwreckClass::Merchant {
        cargo: CargoType::Amphorae {
            count_estimate: 3000,
            contents: "Olive oil from Baetica".to_string(),
        },
        era: VesselEra::Ancient {
            period_name: "Late Roman Republic".to_string(),
            approx_year_bce: 75,
        },
        tonnage_estimate: 250,
        trade_route: "Baetica to Ostia".to_string(),
    };
    let bytes = encode_to_vec(&wreck).expect("encode merchant wreck");
    let (decoded, _): (ShipwreckClass, usize) =
        decode_from_slice(&bytes).expect("decode merchant wreck");
    assert_eq!(wreck, decoded);
}

#[test]
fn test_02_warship_with_battle_context() {
    let wreck = ShipwreckClass::Warship {
        era: VesselEra::EarlyModern {
            year_range_start: 1580,
            year_range_end: 1590,
        },
        gun_count: 50,
        navy: "Spanish Armada".to_string(),
        battle_context: Some("1588 English Channel engagements".to_string()),
    };
    let bytes = encode_to_vec(&wreck).expect("encode warship");
    let (decoded, _): (ShipwreckClass, usize) = decode_from_slice(&bytes).expect("decode warship");
    assert_eq!(wreck, decoded);
}

#[test]
fn test_03_submarine_wreck_classification() {
    let wreck = ShipwreckClass::Submarine {
        designation: "U-869".to_string(),
        launch_year: 1943,
        cause_of_loss: "Own torpedo malfunction".to_string(),
        crew_size: 56,
    };
    let bytes = encode_to_vec(&wreck).expect("encode submarine");
    let (decoded, _): (ShipwreckClass, usize) =
        decode_from_slice(&bytes).expect("decode submarine");
    assert_eq!(wreck, decoded);
}

#[test]
fn test_04_airlift_recovery_with_water_dredge() {
    let method = RecoveryMethod::Airlift {
        spec: AirliftSpec::WaterDredge {
            diameter_cm: 10,
            flow_rate_lpm: 150.0,
        },
        operator: "Diver-07".to_string(),
    };
    let bytes = encode_to_vec(&method).expect("encode airlift recovery");
    let (decoded, _): (RecoveryMethod, usize) =
        decode_from_slice(&bytes).expect("decode airlift recovery");
    assert_eq!(method, decoded);
}

#[test]
fn test_05_corroded_preservation_with_mixed_products() {
    let state = PreservationState::Corroded {
        product: CorrosionProduct::Mixed {
            components: vec![
                "Iron oxide".to_string(),
                "Calcium carbonate".to_string(),
                "Copper alloy concretion".to_string(),
            ],
        },
        depth_mm: 12.5,
        salvageable: true,
    };
    let bytes = encode_to_vec(&state).expect("encode corroded state");
    let (decoded, _): (PreservationState, usize) =
        decode_from_slice(&bytes).expect("decode corroded state");
    assert_eq!(state, decoded);
}

#[test]
fn test_06_dual_frequency_side_scan_sonar() {
    let survey = SonarSurvey::SideScan {
        frequency: SonarFrequencyBand::DualFrequency {
            low_khz: 100.0,
            high_khz: 400.0,
        },
        swath_width_m: 150.0,
        tow_speed_knots: 4.5,
        resolution_cm: 5.0,
    };
    let bytes = encode_to_vec(&survey).expect("encode side-scan sonar");
    let (decoded, _): (SonarSurvey, usize) =
        decode_from_slice(&bytes).expect("decode side-scan sonar");
    assert_eq!(survey, decoded);
}

#[test]
fn test_07_saturation_dive_with_trimix() {
    let profile = DiveProfile::Saturation {
        habitat_depth_m: 90.0,
        breathing_mix: BreathingMix::Trimix {
            oxygen_pct: 16.0,
            helium_pct: 50.0,
        },
        exposure_days: 14,
        decompression_hours: 168,
    };
    let bytes = encode_to_vec(&profile).expect("encode saturation dive");
    let (decoded, _): (DiveProfile, usize) =
        decode_from_slice(&bytes).expect("decode saturation dive");
    assert_eq!(profile, decoded);
}

#[test]
fn test_08_scuba_dive_with_deco_stops() {
    let profile = DiveProfile::Scuba {
        max_depth_m: 55.0,
        bottom_time_min: 25,
        breathing_mix: BreathingMix::Trimix {
            oxygen_pct: 21.0,
            helium_pct: 35.0,
        },
        deco_stops: vec![(21.0, 3), (15.0, 5), (9.0, 8), (6.0, 12), (3.0, 15)],
    };
    let bytes = encode_to_vec(&profile).expect("encode scuba profile");
    let (decoded, _): (DiveProfile, usize) =
        decode_from_slice(&bytes).expect("decode scuba profile");
    assert_eq!(profile, decoded);
}

#[test]
fn test_09_radiocarbon_dating_method() {
    let dating = DatingMethod::Radiocarbon {
        sample_material: "Ship timber, oak heartwood".to_string(),
        raw_age_bp: 1950,
        sigma_years: 40,
        calibrated_range_start: -60,
        calibrated_range_end: 130,
    };
    let bytes = encode_to_vec(&dating).expect("encode C14 dating");
    let (decoded, _): (DatingMethod, usize) = decode_from_slice(&bytes).expect("decode C14 dating");
    assert_eq!(dating, decoded);
}

#[test]
fn test_10_coin_identification_dating() {
    let dating = DatingMethod::CoinIdentification {
        denomination: "Denarius".to_string(),
        ruler: "Augustus".to_string(),
        mint: "Rome".to_string(),
        issue_year: -12,
    };
    let bytes = encode_to_vec(&dating).expect("encode coin dating");
    let (decoded, _): (DatingMethod, usize) =
        decode_from_slice(&bytes).expect("decode coin dating");
    assert_eq!(dating, decoded);
}

#[test]
fn test_11_wooden_hull_with_copper_sheathing() {
    let hull = HullMaterial::Wood {
        primary_species: WoodSpecies::Oak,
        plank_thickness_cm: 8.5,
        fastener: FastenerType::CopperRivet { length_mm: 45.0 },
        sheathing: Some("Copper sheet, Muntz metal".to_string()),
    };
    let bytes = encode_to_vec(&hull).expect("encode wooden hull");
    let (decoded, _): (HullMaterial, usize) =
        decode_from_slice(&bytes).expect("decode wooden hull");
    assert_eq!(hull, decoded);
}

#[test]
fn test_12_iron_hull_with_corroded_state() {
    let hull = HullMaterial::Iron {
        plate_thickness_mm: 12.7,
        rivet_spacing_cm: 7.5,
        corrosion_state: PreservationState::Corroded {
            product: CorrosionProduct::IronOxide { thickness_mm: 8.3 },
            depth_mm: 6.1,
            salvageable: false,
        },
    };
    let bytes = encode_to_vec(&hull).expect("encode iron hull");
    let (decoded, _): (HullMaterial, usize) = decode_from_slice(&bytes).expect("decode iron hull");
    assert_eq!(hull, decoded);
}

#[test]
fn test_13_astrolabe_navigation_instrument() {
    let instrument = NavigationInstrument::Astrolabe {
        material: "Bronze".to_string(),
        diameter_cm: 17.8,
        rete_stars_count: 24,
    };
    let bytes = encode_to_vec(&instrument).expect("encode astrolabe");
    let (decoded, _): (NavigationInstrument, usize) =
        decode_from_slice(&bytes).expect("decode astrolabe");
    assert_eq!(instrument, decoded);
}

#[test]
fn test_14_freeze_drying_conservation_treatment() {
    let treatment = ConservationTreatment::FreezeDrying {
        pre_treatment: Some(ChemicalTreatment::Peg {
            concentration_pct: 25.0,
            duration_months: 18,
        }),
        chamber_temp_c: -30.0,
        vacuum_mbar: 0.5,
        duration_days: 60,
    };
    let bytes = encode_to_vec(&treatment).expect("encode freeze-drying");
    let (decoded, _): (ConservationTreatment, usize) =
        decode_from_slice(&bytes).expect("decode freeze-drying");
    assert_eq!(treatment, decoded);
}

#[test]
fn test_15_electrolytic_reduction_conservation() {
    let treatment = ConservationTreatment::Chemical {
        treatment: ChemicalTreatment::ElectrolyticReduction {
            current_ma: 50.0,
            duration_hours: 720,
        },
        temperature_c: 22.0,
        monitored_by: "Conservation Lab B".to_string(),
    };
    let bytes = encode_to_vec(&treatment).expect("encode electrolytic reduction");
    let (decoded, _): (ConservationTreatment, usize) =
        decode_from_slice(&bytes).expect("decode electrolytic reduction");
    assert_eq!(treatment, decoded);
}

#[test]
fn test_16_artifact_3d_photogrammetry_model() {
    let model = PhotogrammetryModel::ArtifactScan {
        image_count: 450,
        vertex_count: 12_500_000,
        texture_resolution_px: 8192,
        scale_bar_used: true,
    };
    let bytes = encode_to_vec(&model).expect("encode artifact scan model");
    let (decoded, _): (PhotogrammetryModel, usize) =
        decode_from_slice(&bytes).expect("decode artifact scan model");
    assert_eq!(model, decoded);
}

#[test]
fn test_17_contested_jurisdiction_claim() {
    let claim = JurisdictionClaim::Contested {
        claimants: vec![
            "Spain".to_string(),
            "Colombia".to_string(),
            "Sea Search Armada (SSA)".to_string(),
        ],
        dispute_summary: "San Jose galleon, sunk 1708, cargo valued at billions".to_string(),
        arbitration_body: Some("International Court of Justice".to_string()),
    };
    let bytes = encode_to_vec(&claim).expect("encode contested jurisdiction");
    let (decoded, _): (JurisdictionClaim, usize) =
        decode_from_slice(&bytes).expect("decode contested jurisdiction");
    assert_eq!(claim, decoded);
}

#[test]
fn test_18_biological_environmental_monitoring() {
    let monitoring = EnvironmentalMonitoring::BiologicalSurvey {
        indicator: BiologicalIndicator::Biodiversity {
            shannon_index: 3.45,
            species_count: 87,
        },
        survey_date: "2025-06-15".to_string(),
        quadrat_size_sqm: 1.0,
    };
    let bytes = encode_to_vec(&monitoring).expect("encode bio survey");
    let (decoded, _): (EnvironmentalMonitoring, usize) =
        decode_from_slice(&bytes).expect("decode bio survey");
    assert_eq!(monitoring, decoded);
}

#[test]
fn test_19_museum_exhibit_with_vr_display() {
    let outreach = PublicOutreach::MuseumExhibit {
        name: "Depths of Antiquity: The Antikythera Legacy".to_string(),
        venue: "National Archaeological Museum, Athens".to_string(),
        display: DisplayMedium::VirtualReality {
            headset_model: "Meta Quest Pro".to_string(),
            scene_duration_min: 12,
            narration_languages: vec![
                "Greek".to_string(),
                "English".to_string(),
                "French".to_string(),
            ],
        },
        visitor_capacity: 200,
    };
    let bytes = encode_to_vec(&outreach).expect("encode museum exhibit");
    let (decoded, _): (PublicOutreach, usize) =
        decode_from_slice(&bytes).expect("decode museum exhibit");
    assert_eq!(outreach, decoded);
}

#[test]
fn test_20_full_survey_record_deeply_nested() {
    let record = SurveyRecord {
        site_id: "SITE-MED-2025-042".to_string(),
        wreck: ShipwreckClass::Merchant {
            cargo: CargoType::Bullion {
                metal: "Silver".to_string(),
                weight_kg: 1200.0,
            },
            era: VesselEra::EarlyModern {
                year_range_start: 1640,
                year_range_end: 1660,
            },
            tonnage_estimate: 800,
            trade_route: "Manila to Acapulco".to_string(),
        },
        sonar: SonarSurvey::Multibeam {
            frequency: SonarFrequencyBand::High { khz: 400.0 },
            beam_count: 512,
            coverage_area_sqm: 25000.0,
        },
        dive: DiveProfile::Rov {
            model: "Hercules".to_string(),
            max_depth_m: 850.0,
            manipulator_arms: 2,
            camera_count: 5,
            tether_length_m: 4000.0,
        },
        dating: DatingMethod::Dendrochronology {
            ring_count: 180,
            species: "Quercus robur".to_string(),
            reference_chronology: "Baltic oak master chronology".to_string(),
            felling_year: Some(1638),
        },
        hull: HullMaterial::Wood {
            primary_species: WoodSpecies::Teak,
            plank_thickness_cm: 10.0,
            fastener: FastenerType::IronBolt { diameter_mm: 22.0 },
            sheathing: None,
        },
        jurisdiction: JurisdictionClaim::TerritorialWaters {
            state: "Philippines".to_string(),
            distance_nm: 8.3,
        },
    };
    let bytes = encode_to_vec(&record).expect("encode full survey record");
    let (decoded, _): (SurveyRecord, usize) =
        decode_from_slice(&bytes).expect("decode full survey record");
    assert_eq!(record, decoded);
}

#[test]
fn test_21_trade_route_evidence_with_isotopic_analysis() {
    let evidence = TradeRouteEvidence::IsotopicAnalysis {
        element: "Lead".to_string(),
        ratio: 18.456,
        source_match: "Laurion silver mines, Attica".to_string(),
    };
    let nav_evidence = TradeRouteEvidence::NavigationClue {
        instrument: NavigationInstrument::SoundingLead {
            weight_kg: 6.3,
            tallow_cup: true,
        },
        implied_route: "Coastal cabotage, eastern Mediterranean".to_string(),
    };
    let evidence_set: Vec<TradeRouteEvidence> = vec![evidence, nav_evidence];
    let bytes = encode_to_vec(&evidence_set).expect("encode trade route evidence");
    let (decoded, _): (Vec<TradeRouteEvidence>, usize) =
        decode_from_slice(&bytes).expect("decode trade route evidence");
    assert_eq!(evidence_set, decoded);
}

#[test]
fn test_22_cargo_manifest_reconstruction() {
    let manifest = CargoManifest {
        wreck_name: "Uluburun".to_string(),
        entries: vec![
            ManifestEntry::Identified {
                item: "Copper oxhide ingots".to_string(),
                quantity: 354,
                preservation: PreservationState::Encrusted {
                    organism_types: vec![
                        "Calcareous tubeworms".to_string(),
                        "Coralline algae".to_string(),
                    ],
                    encrustation_thickness_mm: 15.0,
                    removal_method: "Micro-sandblasting".to_string(),
                },
                recovery: RecoveryMethod::Mechanical {
                    crane_capacity_tonnes: 5.0,
                    rigging_type: "Nylon sling basket".to_string(),
                    lift_bag_count: 4,
                },
            },
            ManifestEntry::Partial {
                description: "Glass ingots, cobalt blue".to_string(),
                estimated_quantity_range: (150, 200),
                confidence_pct: 72.0,
            },
            ManifestEntry::Inferred {
                evidence_type: "Residue analysis of amphorae interiors".to_string(),
                likely_contents: vec![
                    "Terebinth resin".to_string(),
                    "Orpiment pigment".to_string(),
                ],
                supporting_parallels: vec![
                    "Cape Gelidonya wreck cargo".to_string(),
                    "Egyptian New Kingdom trade records".to_string(),
                ],
            },
        ],
        completeness_pct: 65.0,
    };
    let bytes = encode_to_vec(&manifest).expect("encode cargo manifest");
    let (decoded, _): (CargoManifest, usize) =
        decode_from_slice(&bytes).expect("decode cargo manifest");
    assert_eq!(manifest, decoded);
}
