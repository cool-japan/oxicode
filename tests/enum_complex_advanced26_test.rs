//! Advanced tests for construction and Building Information Modeling (BIM) domain types.
//! 22 test functions covering complex enums, nested enums, and struct/enum compositions.

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
enum BuildingElement {
    Wall {
        thickness_mm: u32,
        height_mm: u32,
        material: MaterialSpec,
        fire_rating: FireRating,
        openings: Vec<WallOpening>,
    },
    Slab {
        thickness_mm: u32,
        span_mm: u32,
        reinforcement: RebarSpec,
        finish: SlabFinish,
    },
    Column {
        section: ColumnSection,
        height_mm: u32,
        material: MaterialSpec,
        splice: Option<SpliceDetail>,
    },
    Beam {
        section: BeamSection,
        span_mm: u32,
        camber_mm: Option<u16>,
        connections: Vec<ConnectionType>,
    },
    Roof {
        system: RoofSystem,
        slope_deg_x10: u16,
        insulation_r_value: u16,
        waterproofing: WaterproofingSystem,
    },
    Foundation {
        kind: FoundationType,
        depth_mm: u32,
        bearing_capacity_kpa: u32,
        soil_condition: SoilCondition,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaterialSpec {
    Concrete {
        strength_mpa: u32,
        mix_code: u16,
        admixtures: Vec<ConcreteAdmixture>,
        slump_mm: u16,
    },
    Steel {
        grade: SteelGrade,
        yield_strength_mpa: u32,
        coating: SteelCoating,
    },
    Timber {
        species_code: u16,
        grade_mark: TimberGrade,
        treatment: TimberTreatment,
        moisture_pct: u8,
    },
    Masonry {
        unit_type: MasonryUnit,
        mortar_type: MortarType,
        grout_filled: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConcreteAdmixture {
    WaterReducer {
        dosage_ml_per_m3: u16,
    },
    Accelerator {
        dosage_ml_per_m3: u16,
    },
    Retarder {
        dosage_ml_per_m3: u16,
    },
    AirEntrainer {
        target_pct: u8,
    },
    Superplasticizer {
        dosage_ml_per_m3: u16,
    },
    FiberReinforcement {
        fiber_type: u8,
        dosage_kg_per_m3: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SteelGrade {
    A36,
    A992,
    A500GrB,
    A572Gr50,
    A588,
    Custom { designation: u32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SteelCoating {
    None,
    HotDipGalvanized { thickness_um: u16 },
    Painted { system_code: u16 },
    Weathering,
    Epoxy { thickness_um: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TimberGrade {
    SelectStructural,
    NumberOne,
    NumberTwo,
    Stud,
    GluLam { lamination_class: u8 },
    Clt { layers: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TimberTreatment {
    Untreated,
    PressureTreated { retention_kg_per_m3: u16 },
    FireRetardant,
    BorateTreated,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
#[allow(clippy::enum_variant_names)]
enum MasonryUnit {
    ConcreteMasonryUnit { width_mm: u16, hollow: bool },
    ClayBrick { compressive_mpa: u16 },
    GlassBlock { size_mm: u16 },
    Stone { stone_type: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MortarType {
    TypeM,
    TypeS,
    TypeN,
    TypeO,
    TypeK,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FireRating {
    Unrated,
    OneHour,
    TwoHour,
    ThreeHour,
    FourHour,
    Custom { minutes: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WallOpening {
    Window {
        width_mm: u32,
        height_mm: u32,
        sill_height_mm: u32,
        glazing: GlazingType,
    },
    Door {
        width_mm: u32,
        height_mm: u32,
        door_type: DoorType,
    },
    Penetration {
        diameter_mm: u32,
        purpose: PenetrationPurpose,
        firestopped: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GlazingType {
    SinglePane,
    DoublePane { gap_mm: u8, gas_fill: u8 },
    TriplePane { u_value_x100: u16 },
    Laminated { interlayer_mm: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DoorType {
    Swing { handed: Handedness },
    Sliding,
    Revolving { diameter_mm: u32 },
    Overhead { height_mm: u32 },
    Fire { rating: FireRating },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Handedness {
    LeftHand,
    RightHand,
    LeftHandReverse,
    RightHandReverse,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PenetrationPurpose {
    MechanicalDuct,
    Plumbing,
    Electrical,
    FireSprinkler,
    Telecom,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SlabFinish {
    BroomFinish,
    TrowelFinish,
    Polished { grit_level: u8 },
    Exposed { aggregate_size_mm: u8 },
    Stamped { pattern_code: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RebarSpec {
    Standard {
        bar_size: u8,
        spacing_mm: u16,
        cover_mm: u16,
        layers: u8,
    },
    PostTensioned {
        strand_count: u16,
        jacking_force_kn: u32,
        profile: TendonProfile,
    },
    FiberOnly {
        fiber_type: u8,
        dosage_kg_per_m3: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TendonProfile {
    Parabolic { drape_mm: u32 },
    Harped { hold_down_points: u8 },
    Straight,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ColumnSection {
    Rectangular {
        width_mm: u32,
        depth_mm: u32,
    },
    Circular {
        diameter_mm: u32,
    },
    WideFlange {
        designation_code: u32,
    },
    Pipe {
        outer_diameter_mm: u32,
        wall_thickness_mm: u16,
    },
    Composite {
        steel_section: u32,
        concrete_strength_mpa: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpliceDetail {
    MechanicalCoupler { bar_size: u8, coupler_type: u8 },
    LapSplice { lap_length_mm: u32 },
    WeldedSplice { weld_type: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BeamSection {
    WideFlange {
        designation_code: u32,
    },
    Channel {
        designation_code: u32,
    },
    ConcreteRect {
        width_mm: u32,
        depth_mm: u32,
        rebar: RebarSpec,
    },
    ConcreteTee {
        flange_width_mm: u32,
        web_depth_mm: u32,
    },
    Glulam {
        width_mm: u32,
        depth_mm: u32,
        laminations: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConnectionType {
    BoltedEndPlate {
        bolt_count: u8,
        bolt_diameter_mm: u8,
    },
    WeldedMoment {
        flange_weld: u8,
        web_weld: u8,
    },
    ShearTab {
        bolt_count: u8,
    },
    ClipAngle {
        angle_size_mm: u16,
    },
    Seated {
        stiffened: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RoofSystem {
    BuiltUp {
        ply_count: u8,
    },
    SinglePlyMembrane {
        membrane_type: u8,
        thickness_mm: u8,
    },
    StandingSeam {
        panel_width_mm: u16,
        material: u8,
    },
    Shingle {
        material: u8,
    },
    GreenRoof {
        substrate_depth_mm: u16,
        vegetation_type: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WaterproofingSystem {
    SheetMembrane { thickness_mm: u8, material: u8 },
    LiquidApplied { coats: u8, thickness_mils: u16 },
    CrystallineAdmixture,
    BentonitePanels { thickness_mm: u8 },
    None,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FoundationType {
    SpreadFooting {
        width_mm: u32,
        length_mm: u32,
    },
    MatFoundation {
        thickness_mm: u32,
    },
    DrivenPile {
        pile_type: PileType,
        count: u16,
        depth_mm: u32,
    },
    DrilledShaft {
        diameter_mm: u32,
        depth_mm: u32,
    },
    HelicalPier {
        helix_count: u8,
        shaft_diameter_mm: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PileType {
    SteelH { section_code: u32 },
    Pipe { diameter_mm: u32, filled: bool },
    PrecastConcrete { dimension_mm: u32 },
    Timber { diameter_mm: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SoilCondition {
    Rock { rqd_pct: u8 },
    DenseGravel,
    StiffClay { undrained_shear_kpa: u32 },
    LooseSand { spt_n: u8 },
    OrganicSoil { depth_mm: u32 },
    Fill { compaction_pct: u8 },
}

// ----------- Structural Load Cases -----------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StructuralLoadCase {
    Dead {
        self_weight_kpa: u32,
        superimposed_kpa: u32,
    },
    Live {
        occupancy_kpa: u32,
        reducible: bool,
        partition_kpa: Option<u32>,
    },
    Wind {
        speed_mph: u16,
        exposure: WindExposure,
        direction_deg: u16,
        pressure_kpa_x100: i32,
    },
    Seismic {
        sds_x1000: u32,
        sd1_x1000: u32,
        importance_factor_x100: u16,
        risk_category: u8,
        site_class: SiteClass,
    },
    Snow {
        ground_load_kpa: u32,
        exposure_factor_x100: u16,
        thermal_factor_x100: u16,
        drift: Option<SnowDrift>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WindExposure {
    B,
    C,
    D,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SiteClass {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SnowDrift {
    Leeward { height_mm: u32 },
    Windward { height_mm: u32 },
    Balanced,
}

// ----------- MEP Systems -----------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MepSystem {
    Hvac(HvacComponent),
    Plumbing(PlumbingComponent),
    Electrical(ElectricalComponent),
    FireProtection(FireProtectionComponent),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HvacComponent {
    AirHandler {
        cfm: u32,
        static_pressure_pa: u32,
        filter_type: u8,
        heating_kw: u32,
        cooling_kw: u32,
    },
    Ductwork {
        width_mm: u32,
        height_mm: u32,
        length_mm: u32,
        insulated: bool,
    },
    Diffuser {
        cfm: u16,
        throw_mm: u16,
        diffuser_type: u8,
    },
    Chiller {
        capacity_kw: u32,
        cop_x100: u16,
        refrigerant: u8,
    },
    Boiler {
        capacity_kw: u32,
        efficiency_pct: u8,
        fuel_type: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PlumbingComponent {
    DomesticWater {
        pipe_diameter_mm: u16,
        material: PipeMaterial,
        pressure_kpa: u32,
    },
    Sanitary {
        pipe_diameter_mm: u16,
        slope_pct_x10: u16,
        material: PipeMaterial,
    },
    StormDrain {
        pipe_diameter_mm: u16,
        capacity_lps: u32,
    },
    Fixture {
        fixture_type: FixtureType,
        ada_compliant: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PipeMaterial {
    Copper,
    Pex,
    Pvc,
    CastIron,
    Hdpe,
    StainlessSteel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FixtureType {
    WaterCloset { gpf_x10: u8 },
    Lavatory { gpm_x10: u8 },
    Shower { gpm_x10: u8 },
    Sink { basin_count: u8 },
    DrinkingFountain,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ElectricalComponent {
    Panel {
        amperage: u16,
        voltage: u16,
        phase: u8,
        circuits: u16,
    },
    Transformer {
        kva: u32,
        primary_voltage: u16,
        secondary_voltage: u16,
    },
    Conduit {
        diameter_mm: u16,
        material: u8,
        length_mm: u32,
    },
    Receptacle {
        amperage: u8,
        voltage: u16,
        dedicated: bool,
    },
    Luminaire {
        wattage: u16,
        lumens: u32,
        color_temp_k: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FireProtectionComponent {
    Sprinkler {
        head_type: SprinklerHead,
        coverage_m2: u16,
        k_factor_x100: u16,
    },
    Standpipe {
        class: u8,
        diameter_mm: u16,
    },
    FirePump {
        capacity_lpm: u32,
        pressure_kpa: u32,
        driver: FirePumpDriver,
    },
    Extinguisher {
        agent_type: u8,
        rating: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SprinklerHead {
    Pendant,
    Upright,
    Sidewall,
    Concealed,
    Esfr { k_factor_x100: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FirePumpDriver {
    Electric { hp: u16 },
    Diesel { hp: u16, fuel_tank_liters: u16 },
}

// ----------- Construction Phases -----------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConstructionPhase {
    Design {
        stage: DesignStage,
        discipline: Discipline,
        deliverables: Vec<Deliverable>,
    },
    Permit {
        jurisdiction_code: u32,
        permit_type: PermitType,
        status: PermitStatus,
    },
    Excavation {
        volume_m3: u32,
        shoring: ShoringType,
        dewatering: bool,
    },
    FoundationWork {
        foundation: FoundationType,
        pour_schedule: Vec<ConcretePour>,
    },
    Framing {
        system: FramingSystem,
        progress_pct: u8,
    },
    Finishing {
        scope: FinishingScope,
        punch_list_items: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DesignStage {
    Schematic,
    DesignDevelopment,
    ConstructionDocuments,
    BiddingNegotiation,
    ConstructionAdmin,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Discipline {
    Architectural,
    Structural,
    Mechanical,
    Electrical,
    Plumbing,
    Civil,
    Landscape,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Deliverable {
    DrawingSet { sheet_count: u16 },
    Specification { section_count: u16 },
    Model { lod: u8 },
    Report { page_count: u16 },
    Schedule { item_count: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PermitType {
    Building,
    Grading,
    Demolition,
    Electrical,
    Mechanical,
    Plumbing,
    FireAlarm,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PermitStatus {
    Submitted,
    UnderReview,
    RevisionRequired { comment_count: u16 },
    Approved { permit_number: u32 },
    Denied { reason_code: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ShoringType {
    None,
    SheetPile { depth_mm: u32 },
    SoldierPile { spacing_mm: u32, lagging: bool },
    SoilNail { length_mm: u32, spacing_mm: u32 },
    Secant { pile_diameter_mm: u32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FramingSystem {
    SteelMomentFrame,
    SteelBracedFrame { brace_type: BraceType },
    ConcreteFrame { post_tensioned: bool },
    WoodFrame { stud_spacing_mm: u16 },
    MasonryBearing,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BraceType {
    Chevron,
    XBrace,
    Eccentric { link_length_mm: u32 },
    BucketRestrained,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FinishingScope {
    Interior {
        flooring: bool,
        ceilings: bool,
        millwork: bool,
        paint: bool,
    },
    Exterior {
        cladding: bool,
        glazing: bool,
        roofing: bool,
    },
    SiteWork {
        paving: bool,
        landscaping: bool,
        utilities: bool,
    },
}

// ----------- RFI & Change Orders -----------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RfiRecord {
    Open {
        rfi_number: u32,
        discipline: Discipline,
        priority: RfiPriority,
        question_hash: u64,
        attachments: u8,
    },
    Responded {
        rfi_number: u32,
        response_hash: u64,
        days_to_respond: u16,
    },
    Closed {
        rfi_number: u32,
        resolution: RfiResolution,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RfiPriority {
    Urgent,
    High,
    Normal,
    Low,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RfiResolution {
    AsDesigned,
    Revised { revision_number: u16 },
    DeferredToSubmittal { submittal_number: u32 },
    Withdrawn,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChangeOrder {
    Proposed {
        co_number: u32,
        reason: ChangeReason,
        cost_impact_cents: i64,
        schedule_impact_days: i32,
    },
    Approved {
        co_number: u32,
        approved_cost_cents: i64,
        approved_days: i32,
    },
    Rejected {
        co_number: u32,
        rejection_reason_code: u16,
    },
    Voided {
        co_number: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChangeReason {
    OwnerRequest,
    DesignError,
    UnforeseenCondition,
    CodeChange,
    ValueEngineering { savings_cents: i64 },
    Coordination { clash_id: u32 },
}

// ----------- Inspection & Safety -----------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InspectionResult {
    Pass {
        inspector_id: u32,
        timestamp_epoch: u64,
        notes: Option<u32>,
    },
    Fail {
        inspector_id: u32,
        deficiency: Deficiency,
        reinspection_required: bool,
    },
    Conditional {
        inspector_id: u32,
        conditions: Vec<InspectionCondition>,
        deadline_epoch: u64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Deficiency {
    Structural { severity: u8, element_code: u32 },
    FireSafety { code_section: u32 },
    Accessibility { ada_section: u16 },
    Electrical { nec_article: u16 },
    MechanicalCode { imc_section: u16 },
    PlumbingCode { ipc_section: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InspectionCondition {
    SubmitDocumentation { doc_type: u8 },
    CorrectDeficiency { deficiency: Deficiency },
    ProvideTestResults { test_type: u8 },
    EngineersLetter,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SafetyIncident {
    NearMiss {
        category: HazardCategory,
        location_code: u32,
    },
    FirstAid {
        injury_type: InjuryType,
        body_part: u8,
        worker_trade: u8,
    },
    Recordable {
        injury_type: InjuryType,
        lost_time_days: u16,
        osha_form_number: u32,
    },
    Fatality {
        cause: FatalityCause,
        investigation_id: u64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HazardCategory {
    FallHazard { height_mm: u32 },
    StruckBy { object_type: u8 },
    CaughtBetween,
    Electrocution { voltage: u16 },
    TrenchCollapse { depth_mm: u32 },
    HeatStress,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InjuryType {
    Laceration,
    Fracture,
    Sprain,
    Burn { degree: u8 },
    Contusion,
    ForeignBody,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FatalityCause {
    Fall,
    StruckBy,
    Electrocution,
    CaughtInBetween,
}

// ----------- Equipment & Concrete Pours -----------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EquipmentUtilization {
    Active {
        equipment_id: u32,
        equipment_type: EquipmentType,
        hours_today_x10: u16,
        fuel_level_pct: u8,
        operator_id: u32,
    },
    Idle {
        equipment_id: u32,
        reason: IdleReason,
    },
    Maintenance {
        equipment_id: u32,
        maintenance_type: MaintenanceType,
        estimated_hours: u16,
    },
    OffSite {
        equipment_id: u32,
        expected_return_epoch: u64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EquipmentType {
    TowerCrane {
        capacity_tonnes: u16,
        jib_length_m: u16,
    },
    Excavator {
        bucket_m3_x10: u16,
    },
    BoomPump {
        reach_m: u16,
    },
    ConcreteTruck {
        capacity_m3: u8,
    },
    Loader {
        bucket_m3_x10: u16,
    },
    Forklift {
        capacity_kg: u32,
    },
    Scaffolding {
        levels: u8,
        bays: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IdleReason {
    WeatherDelay,
    WaitingForMaterial,
    NoOperator,
    BetweenTasks,
    PermitHold,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaintenanceType {
    Preventive { interval_hours: u32 },
    Corrective { fault_code: u16 },
    Inspection { next_due_epoch: u64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConcretePour {
    Scheduled {
        pour_id: u32,
        volume_m3_x10: u32,
        element: PourElement,
        mix_code: u16,
        pump_required: bool,
    },
    InProgress {
        pour_id: u32,
        placed_m3_x10: u32,
        total_m3_x10: u32,
        trucks_dispatched: u8,
        slump_readings: Vec<u16>,
    },
    Completed {
        pour_id: u32,
        actual_volume_m3_x10: u32,
        cylinder_sets: u8,
        ambient_temp_c_x10: i16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PourElement {
    Footing,
    PierCap,
    SlabOnGrade,
    ElevatedSlab { floor_number: u8 },
    Wall { height_mm: u32 },
    Column { floor_from: u8, floor_to: u8 },
}

// ----------- Sustainability Certifications -----------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SustainabilityCertification {
    Leed {
        version: LeedVersion,
        level: LeedLevel,
        credits: Vec<LeedCredit>,
    },
    Breeam {
        rating: BreeamRating,
        score_x10: u16,
        categories_achieved: u8,
    },
    Well {
        version: u8,
        level: WellLevel,
        preconditions_met: u16,
        optimizations_met: u16,
    },
    LivingBuilding {
        petals_achieved: u8,
        imperatives_met: u16,
    },
    PassiveHouse {
        heating_demand_kwh_m2: u16,
        airtightness_ach: u16,
        primary_energy_kwh_m2: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LeedVersion {
    V4,
    V41,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LeedLevel {
    Certified,
    Silver,
    Gold,
    Platinum,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LeedCredit {
    SustainableSites { points: u8 },
    WaterEfficiency { points: u8 },
    EnergyAtmosphere { points: u8 },
    MaterialsResources { points: u8 },
    IndoorEnvironmental { points: u8 },
    Innovation { points: u8 },
    RegionalPriority { points: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreeamRating {
    Pass,
    Good,
    VeryGood,
    Excellent,
    Outstanding,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WellLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_building_element_wall_with_openings() {
    let wall = BuildingElement::Wall {
        thickness_mm: 200,
        height_mm: 3000,
        material: MaterialSpec::Concrete {
            strength_mpa: 35,
            mix_code: 4500,
            admixtures: vec![
                ConcreteAdmixture::WaterReducer {
                    dosage_ml_per_m3: 500,
                },
                ConcreteAdmixture::AirEntrainer { target_pct: 6 },
            ],
            slump_mm: 100,
        },
        fire_rating: FireRating::TwoHour,
        openings: vec![
            WallOpening::Window {
                width_mm: 1200,
                height_mm: 1500,
                sill_height_mm: 900,
                glazing: GlazingType::DoublePane {
                    gap_mm: 12,
                    gas_fill: 1,
                },
            },
            WallOpening::Door {
                width_mm: 900,
                height_mm: 2100,
                door_type: DoorType::Fire {
                    rating: FireRating::OneHour,
                },
            },
        ],
    };
    let encoded = encode_to_vec(&wall).expect("encode wall");
    let (decoded, _) = decode_from_slice::<BuildingElement>(&encoded).expect("decode wall");
    assert_eq!(wall, decoded);
}

#[test]
fn test_building_element_slab_post_tensioned() {
    let slab = BuildingElement::Slab {
        thickness_mm: 250,
        span_mm: 9000,
        reinforcement: RebarSpec::PostTensioned {
            strand_count: 24,
            jacking_force_kn: 186000,
            profile: TendonProfile::Parabolic { drape_mm: 180 },
        },
        finish: SlabFinish::Polished { grit_level: 3 },
    };
    let encoded = encode_to_vec(&slab).expect("encode slab");
    let (decoded, _) = decode_from_slice::<BuildingElement>(&encoded).expect("decode slab");
    assert_eq!(slab, decoded);
}

#[test]
fn test_building_element_column_composite() {
    let col = BuildingElement::Column {
        section: ColumnSection::Composite {
            steel_section: 14500,
            concrete_strength_mpa: 50,
        },
        height_mm: 4200,
        material: MaterialSpec::Steel {
            grade: SteelGrade::A992,
            yield_strength_mpa: 345,
            coating: SteelCoating::Painted { system_code: 2010 },
        },
        splice: Some(SpliceDetail::MechanicalCoupler {
            bar_size: 10,
            coupler_type: 2,
        }),
    };
    let encoded = encode_to_vec(&col).expect("encode column");
    let (decoded, _) = decode_from_slice::<BuildingElement>(&encoded).expect("decode column");
    assert_eq!(col, decoded);
}

#[test]
fn test_building_element_beam_with_connections() {
    let beam = BuildingElement::Beam {
        section: BeamSection::ConcreteRect {
            width_mm: 400,
            depth_mm: 600,
            rebar: RebarSpec::Standard {
                bar_size: 8,
                spacing_mm: 150,
                cover_mm: 40,
                layers: 2,
            },
        },
        span_mm: 7500,
        camber_mm: Some(25),
        connections: vec![
            ConnectionType::BoltedEndPlate {
                bolt_count: 8,
                bolt_diameter_mm: 22,
            },
            ConnectionType::WeldedMoment {
                flange_weld: 1,
                web_weld: 2,
            },
            ConnectionType::Seated { stiffened: true },
        ],
    };
    let encoded = encode_to_vec(&beam).expect("encode beam");
    let (decoded, _) = decode_from_slice::<BuildingElement>(&encoded).expect("decode beam");
    assert_eq!(beam, decoded);
}

#[test]
fn test_building_element_roof_green() {
    let roof = BuildingElement::Roof {
        system: RoofSystem::GreenRoof {
            substrate_depth_mm: 300,
            vegetation_type: 2,
        },
        slope_deg_x10: 20,
        insulation_r_value: 30,
        waterproofing: WaterproofingSystem::LiquidApplied {
            coats: 3,
            thickness_mils: 60,
        },
    };
    let encoded = encode_to_vec(&roof).expect("encode roof");
    let (decoded, _) = decode_from_slice::<BuildingElement>(&encoded).expect("decode roof");
    assert_eq!(roof, decoded);
}

#[test]
fn test_building_element_foundation_piles() {
    let foundation = BuildingElement::Foundation {
        kind: FoundationType::DrivenPile {
            pile_type: PileType::SteelH {
                section_code: 14117,
            },
            count: 12,
            depth_mm: 18000,
        },
        depth_mm: 20000,
        bearing_capacity_kpa: 400,
        soil_condition: SoilCondition::StiffClay {
            undrained_shear_kpa: 120,
        },
    };
    let encoded = encode_to_vec(&foundation).expect("encode foundation");
    let (decoded, _) = decode_from_slice::<BuildingElement>(&encoded).expect("decode foundation");
    assert_eq!(foundation, decoded);
}

#[test]
fn test_material_spec_timber_with_treatment() {
    let mat = MaterialSpec::Timber {
        species_code: 42,
        grade_mark: TimberGrade::GluLam {
            lamination_class: 3,
        },
        treatment: TimberTreatment::PressureTreated {
            retention_kg_per_m3: 640,
        },
        moisture_pct: 12,
    };
    let encoded = encode_to_vec(&mat).expect("encode timber");
    let (decoded, _) = decode_from_slice::<MaterialSpec>(&encoded).expect("decode timber");
    assert_eq!(mat, decoded);
}

#[test]
fn test_material_spec_masonry_grouted() {
    let mat = MaterialSpec::Masonry {
        unit_type: MasonryUnit::ConcreteMasonryUnit {
            width_mm: 200,
            hollow: true,
        },
        mortar_type: MortarType::TypeS,
        grout_filled: true,
    };
    let encoded = encode_to_vec(&mat).expect("encode masonry");
    let (decoded, _) = decode_from_slice::<MaterialSpec>(&encoded).expect("decode masonry");
    assert_eq!(mat, decoded);
}

#[test]
fn test_structural_load_case_seismic() {
    let load = StructuralLoadCase::Seismic {
        sds_x1000: 1200,
        sd1_x1000: 600,
        importance_factor_x100: 150,
        risk_category: 4,
        site_class: SiteClass::D,
    };
    let encoded = encode_to_vec(&load).expect("encode seismic");
    let (decoded, _) = decode_from_slice::<StructuralLoadCase>(&encoded).expect("decode seismic");
    assert_eq!(load, decoded);
}

#[test]
fn test_structural_load_case_snow_with_drift() {
    let load = StructuralLoadCase::Snow {
        ground_load_kpa: 2400,
        exposure_factor_x100: 90,
        thermal_factor_x100: 110,
        drift: Some(SnowDrift::Leeward { height_mm: 1200 }),
    };
    let encoded = encode_to_vec(&load).expect("encode snow load");
    let (decoded, _) = decode_from_slice::<StructuralLoadCase>(&encoded).expect("decode snow load");
    assert_eq!(load, decoded);
}

#[test]
fn test_mep_system_hvac_chiller() {
    let mep = MepSystem::Hvac(HvacComponent::Chiller {
        capacity_kw: 3500,
        cop_x100: 560,
        refrigerant: 3,
    });
    let encoded = encode_to_vec(&mep).expect("encode hvac");
    let (decoded, _) = decode_from_slice::<MepSystem>(&encoded).expect("decode hvac");
    assert_eq!(mep, decoded);
}

#[test]
fn test_mep_system_fire_protection_pump() {
    let mep = MepSystem::FireProtection(FireProtectionComponent::FirePump {
        capacity_lpm: 5678,
        pressure_kpa: 860,
        driver: FirePumpDriver::Diesel {
            hp: 150,
            fuel_tank_liters: 400,
        },
    });
    let encoded = encode_to_vec(&mep).expect("encode fire pump");
    let (decoded, _) = decode_from_slice::<MepSystem>(&encoded).expect("decode fire pump");
    assert_eq!(mep, decoded);
}

#[test]
fn test_mep_system_plumbing_fixture() {
    let mep = MepSystem::Plumbing(PlumbingComponent::Fixture {
        fixture_type: FixtureType::WaterCloset { gpf_x10: 16 },
        ada_compliant: true,
    });
    let encoded = encode_to_vec(&mep).expect("encode plumbing fixture");
    let (decoded, _) = decode_from_slice::<MepSystem>(&encoded).expect("decode plumbing fixture");
    assert_eq!(mep, decoded);
}

#[test]
fn test_construction_phase_design() {
    let phase = ConstructionPhase::Design {
        stage: DesignStage::ConstructionDocuments,
        discipline: Discipline::Structural,
        deliverables: vec![
            Deliverable::DrawingSet { sheet_count: 85 },
            Deliverable::Specification { section_count: 12 },
            Deliverable::Model { lod: 4 },
        ],
    };
    let encoded = encode_to_vec(&phase).expect("encode design phase");
    let (decoded, _) =
        decode_from_slice::<ConstructionPhase>(&encoded).expect("decode design phase");
    assert_eq!(phase, decoded);
}

#[test]
fn test_construction_phase_foundation_with_pours() {
    let phase = ConstructionPhase::FoundationWork {
        foundation: FoundationType::MatFoundation { thickness_mm: 1200 },
        pour_schedule: vec![
            ConcretePour::Scheduled {
                pour_id: 101,
                volume_m3_x10: 850,
                element: PourElement::Footing,
                mix_code: 4000,
                pump_required: true,
            },
            ConcretePour::Completed {
                pour_id: 100,
                actual_volume_m3_x10: 920,
                cylinder_sets: 4,
                ambient_temp_c_x10: 225,
            },
        ],
    };
    let encoded = encode_to_vec(&phase).expect("encode foundation phase");
    let (decoded, _) =
        decode_from_slice::<ConstructionPhase>(&encoded).expect("decode foundation phase");
    assert_eq!(phase, decoded);
}

#[test]
fn test_rfi_record_open_and_closed() {
    let rfis: Vec<RfiRecord> = vec![
        RfiRecord::Open {
            rfi_number: 42,
            discipline: Discipline::Mechanical,
            priority: RfiPriority::High,
            question_hash: 0xDEADBEEF_CAFEBABE,
            attachments: 3,
        },
        RfiRecord::Responded {
            rfi_number: 40,
            response_hash: 0x1234_5678_9ABC_DEF0,
            days_to_respond: 7,
        },
        RfiRecord::Closed {
            rfi_number: 38,
            resolution: RfiResolution::Revised { revision_number: 3 },
        },
    ];
    let encoded = encode_to_vec(&rfis).expect("encode rfis");
    let (decoded, _) = decode_from_slice::<Vec<RfiRecord>>(&encoded).expect("decode rfis");
    assert_eq!(rfis, decoded);
}

#[test]
fn test_change_order_value_engineering() {
    let co = ChangeOrder::Proposed {
        co_number: 15,
        reason: ChangeReason::ValueEngineering {
            savings_cents: -125_000_00,
        },
        cost_impact_cents: -125_000_00,
        schedule_impact_days: -5,
    };
    let encoded = encode_to_vec(&co).expect("encode change order");
    let (decoded, _) = decode_from_slice::<ChangeOrder>(&encoded).expect("decode change order");
    assert_eq!(co, decoded);
}

#[test]
fn test_inspection_result_conditional() {
    let result = InspectionResult::Conditional {
        inspector_id: 9001,
        conditions: vec![
            InspectionCondition::CorrectDeficiency {
                deficiency: Deficiency::Structural {
                    severity: 3,
                    element_code: 44200,
                },
            },
            InspectionCondition::SubmitDocumentation { doc_type: 2 },
            InspectionCondition::EngineersLetter,
        ],
        deadline_epoch: 1_710_000_000,
    };
    let encoded = encode_to_vec(&result).expect("encode inspection");
    let (decoded, _) = decode_from_slice::<InspectionResult>(&encoded).expect("decode inspection");
    assert_eq!(result, decoded);
}

#[test]
fn test_safety_incident_recordable() {
    let incident = SafetyIncident::Recordable {
        injury_type: InjuryType::Fracture,
        lost_time_days: 14,
        osha_form_number: 300_123,
    };
    let encoded = encode_to_vec(&incident).expect("encode incident");
    let (decoded, _) = decode_from_slice::<SafetyIncident>(&encoded).expect("decode incident");
    assert_eq!(incident, decoded);
}

#[test]
fn test_equipment_utilization_active_crane() {
    let equip = EquipmentUtilization::Active {
        equipment_id: 5001,
        equipment_type: EquipmentType::TowerCrane {
            capacity_tonnes: 12,
            jib_length_m: 60,
        },
        hours_today_x10: 85,
        fuel_level_pct: 72,
        operator_id: 300,
    };
    let encoded = encode_to_vec(&equip).expect("encode equipment");
    let (decoded, _) =
        decode_from_slice::<EquipmentUtilization>(&encoded).expect("decode equipment");
    assert_eq!(equip, decoded);
}

#[test]
fn test_concrete_pour_in_progress() {
    let pour = ConcretePour::InProgress {
        pour_id: 205,
        placed_m3_x10: 340,
        total_m3_x10: 500,
        trucks_dispatched: 6,
        slump_readings: vec![95, 100, 105, 90, 100, 100],
    };
    let encoded = encode_to_vec(&pour).expect("encode pour");
    let (decoded, _) = decode_from_slice::<ConcretePour>(&encoded).expect("decode pour");
    assert_eq!(pour, decoded);
}

#[test]
fn test_sustainability_leed_platinum() {
    let cert = SustainabilityCertification::Leed {
        version: LeedVersion::V41,
        level: LeedLevel::Platinum,
        credits: vec![
            LeedCredit::SustainableSites { points: 10 },
            LeedCredit::WaterEfficiency { points: 11 },
            LeedCredit::EnergyAtmosphere { points: 33 },
            LeedCredit::MaterialsResources { points: 13 },
            LeedCredit::IndoorEnvironmental { points: 16 },
            LeedCredit::Innovation { points: 6 },
            LeedCredit::RegionalPriority { points: 4 },
        ],
    };
    let encoded = encode_to_vec(&cert).expect("encode leed");
    let (decoded, _) =
        decode_from_slice::<SustainabilityCertification>(&encoded).expect("decode leed");
    assert_eq!(cert, decoded);
}
