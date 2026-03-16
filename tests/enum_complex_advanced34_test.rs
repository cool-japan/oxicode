//! Advanced tests for marine vessel operations and port management systems.
//! 22 test functions covering vessel types, AIS messages, ballast water management,
//! engine room alarms, cargo stowage, SOLAS safety equipment, port clearance,
//! VTS zones, bunker fuel grades, classification surveys, ISPS security levels,
//! and marine weather routing decisions.

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

// --- Vessel classification ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum VesselType {
    CrudeTanker,
    ProductTanker,
    ChemicalTanker,
    LngCarrier,
    BulkCarrier,
    ContainerShip,
    RoRoCargo,
    RoPax,
    GeneralCargo,
    VehicleCarrier,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VesselIdentity {
    mmsi: u32,
    imo_number: u32,
    call_sign: String,
    vessel_name: String,
    vessel_type: VesselType,
    gross_tonnage: u32,
    deadweight_tonnes: u32,
    length_overall_cm: u32,
    beam_cm: u32,
    draft_max_mm: u32,
    flag_state: String,
}

// --- AIS message types ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum AisMessageType {
    PositionReportClassA {
        nav_status: u8,
        rate_of_turn: i16,
        speed_over_ground_tenths: u16,
        longitude_ten_thousandths: i32,
        latitude_ten_thousandths: i32,
        course_over_ground_tenths: u16,
        true_heading: u16,
        timestamp_second: u8,
    },
    PositionReportClassB {
        speed_over_ground_tenths: u16,
        longitude_ten_thousandths: i32,
        latitude_ten_thousandths: i32,
        course_over_ground_tenths: u16,
        true_heading: u16,
    },
    StaticVoyageData {
        imo_number: u32,
        destination: String,
        eta_month: u8,
        eta_day: u8,
        eta_hour: u8,
        eta_minute: u8,
        draught_tenths: u16,
    },
    SafetyBroadcast {
        text: String,
    },
    BinaryAddressed {
        dest_mmsi: u32,
        payload: Vec<u8>,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AisFrame {
    mmsi: u32,
    repeat_indicator: u8,
    message: AisMessageType,
    receive_timestamp_epoch: u64,
}

// --- Ballast water management ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum BallastWaterState {
    Empty,
    Filling {
        tank_id: u16,
        target_volume_litres: u64,
    },
    Full {
        tank_id: u16,
        volume_litres: u64,
    },
    Exchanging {
        tank_id: u16,
        exchange_latitude: i32,
        exchange_longitude: i32,
        cycle: u8,
    },
    TreatmentUv {
        tank_id: u16,
        uv_dosage_mj_per_cm2: u32,
    },
    TreatmentElectrolysis {
        tank_id: u16,
        tro_ppm: u16,
        contact_minutes: u16,
    },
    Discharging {
        tank_id: u16,
        remaining_litres: u64,
    },
    Settled {
        tank_id: u16,
        sediment_depth_mm: u16,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BallastRecord {
    record_id: u64,
    vessel_imo: u32,
    state: BallastWaterState,
    salinity_ppt_tenths: u16,
    temperature_celsius_tenths: i16,
    timestamp_epoch: u64,
}

// --- Engine room alarm classification ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum AlarmSeverity {
    Advisory,
    Caution,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum EngineRoomAlarm {
    MainEngineBearingTemp {
        cylinder: u8,
        temp_celsius_tenths: u16,
        severity: AlarmSeverity,
    },
    ExhaustGasTemp {
        cylinder: u8,
        temp_celsius_tenths: u16,
        deviation_from_mean: i16,
    },
    LubeOilPressure {
        system: String,
        pressure_kpa: u32,
        severity: AlarmSeverity,
    },
    CoolantFlow {
        circuit: String,
        flow_litres_per_min: u32,
        severity: AlarmSeverity,
    },
    TurbochargerSurge {
        tc_id: u8,
        rpm: u32,
        vibration_mm_per_s: u16,
    },
    ScavengeFireDetected {
        cylinder: u8,
    },
    CrankcaseOilMist {
        concentration_ppm: u32,
        severity: AlarmSeverity,
    },
    BilgeHighLevel {
        compartment: String,
        level_mm: u32,
    },
    GeneratorOverload {
        gen_id: u8,
        load_percent: u16,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlarmEvent {
    event_id: u64,
    alarm: EngineRoomAlarm,
    acknowledged: bool,
    timestamp_epoch: u64,
    officer_on_watch: String,
}

// --- Cargo stowage plan ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum CargoCategory {
    DryBulk {
        commodity: String,
        stowage_factor: u32,
    },
    LiquidBulk {
        product_name: String,
        flash_point_celsius: i16,
        un_number: u16,
    },
    Container {
        teu_count: u16,
        reefer_count: u16,
        imdg_count: u16,
    },
    BreakBulk {
        package_count: u32,
        average_weight_kg: u32,
    },
    RoRo {
        lane_metres: u32,
        vehicle_count: u16,
    },
    HeavyLift {
        piece_weight_tonnes: u32,
        crane_required_tonnes: u32,
    },
    Livestock {
        head_count: u32,
        species: String,
    },
    ProjectCargo {
        description: String,
        dimensions_cm: (u32, u32, u32),
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StowagePlan {
    plan_id: u64,
    voyage_number: String,
    cargo: CargoCategory,
    hold_number: u8,
    weight_mt: u32,
    volume_cbm: u32,
    loading_port: String,
    discharge_port: String,
}

// --- SOLAS safety equipment ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum SolasEquipmentType {
    Lifeboat {
        capacity_persons: u16,
        davit_type: String,
    },
    LifeRaft {
        capacity_persons: u16,
        hydrostatic_release: bool,
    },
    RescueBoat {
        speed_knots_tenths: u16,
        length_cm: u16,
    },
    Epirb {
        mmsi: u32,
        battery_expiry_epoch: u64,
    },
    Sart {
        battery_expiry_epoch: u64,
        range_nm: u8,
    },
    FireExtinguisher {
        agent: String,
        capacity_kg: u16,
        location: String,
    },
    FixedFireSystem {
        agent: String,
        coverage_zone: String,
        capacity_kg: u32,
    },
    BreathingApparatus {
        cylinder_minutes: u16,
        set_count: u8,
    },
    ImmersionSuit {
        size: String,
        count: u16,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SafetyInventory {
    vessel_imo: u32,
    equipment: Vec<SolasEquipmentType>,
    last_inspection_epoch: u64,
    next_inspection_epoch: u64,
    inspector_name: String,
}

// --- Port authority clearance stages ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum ClearanceStage {
    PreArrivalNotification {
        eta_epoch: u64,
        last_port: String,
        crew_count: u16,
        pax_count: u16,
    },
    PilotRequested {
        boarding_position_lat: i32,
        boarding_position_lon: i32,
        pilot_eta_epoch: u64,
    },
    PilotOnBoard {
        pilot_id: String,
    },
    TugAssistEngaged {
        tug_count: u8,
        bollard_pull_total_tonnes: u32,
    },
    BerthAssigned {
        berth_code: String,
        side_alongside: u8,
    },
    CustomsCleared {
        declaration_number: String,
    },
    HealthCleared {
        pratique_granted: bool,
        quarantine_flag: bool,
    },
    ImmigrationCleared {
        shore_pass_count: u16,
    },
    CargoOperationsApproved {
        terminal_operator: String,
    },
    DepartureCleared {
        next_port: String,
        etd_epoch: u64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PortCall {
    call_id: u64,
    vessel_imo: u32,
    port_unlocode: String,
    stages: Vec<ClearanceStage>,
}

// --- VTS zones ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum VtsZoneType {
    TrafficSeparationScheme {
        lane_id: String,
        direction_degrees: u16,
    },
    PrecautionaryArea {
        radius_metres: u32,
    },
    InshoreTrafficZone,
    AnchorageArea {
        max_draft_dm: u16,
        holding_quality: String,
    },
    PilotBoardingArea {
        vhf_channel: u8,
    },
    ReportingPoint {
        mandatory: bool,
        vhf_channel: u8,
    },
    SpeedRestrictionZone {
        max_speed_knots_tenths: u16,
        reason: String,
    },
    EnvironmentallyProtected {
        restriction_details: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VtsPassage {
    passage_id: u64,
    vessel_mmsi: u32,
    zone: VtsZoneType,
    entry_epoch: u64,
    exit_epoch: Option<u64>,
    compliant: bool,
}

// --- Bunker fuel quality ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum BunkerFuelGrade {
    VlsfoZeroPointFive {
        sulphur_ppm: u32,
    },
    UlsfoZeroPointOne {
        sulphur_ppm: u32,
    },
    Hfo {
        sulphur_percent_hundredths: u16,
    },
    Mgo {
        cetane_number: u8,
        cloud_point_celsius: i8,
    },
    Mdo {
        viscosity_cst_tenths: u16,
    },
    Lng {
        methane_number: u8,
        boil_off_rate_percent_hundredths: u16,
    },
    Methanol {
        purity_percent_tenths: u16,
    },
    Ammonia {
        purity_percent_tenths: u16,
        nox_treatment: String,
    },
    Biofuel {
        fame_percent: u8,
        blend_base: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BunkerDeliveryNote {
    bdn_number: String,
    grade: BunkerFuelGrade,
    quantity_mt_tenths: u32,
    density_kg_per_m3: u16,
    water_content_ppm: u16,
    supplier: String,
    port_of_delivery: String,
    delivery_epoch: u64,
}

// --- Classification society survey ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum SurveyType {
    AnnualHull,
    AnnualMachinery,
    IntermediateHull {
        underwater_inspection: bool,
    },
    SpecialSurvey {
        dry_dock_required: bool,
        thickness_measurements: u32,
    },
    BottomSurvey {
        method: String,
    },
    BoilerSurvey {
        internal_exam: bool,
        hydraulic_test_bar: u16,
    },
    ShaftSurvey {
        bearing_clearances_ok: bool,
    },
    TailShaftDrawup {
        interval_years: u8,
    },
    ContinuousSurvey {
        item_numbers: Vec<u16>,
    },
    ConditionAssessment {
        rating: u8,
        findings_count: u16,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ClassSurveyReport {
    report_id: u64,
    vessel_imo: u32,
    society_code: String,
    survey: SurveyType,
    surveyor_name: String,
    date_epoch: u64,
    next_due_epoch: u64,
    conditions_of_class: Vec<String>,
}

// --- ISPS security levels ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum IspsSecurityLevel {
    Level1Normal,
    Level2Heightened {
        threat_type: String,
    },
    Level3Exceptional {
        threat_type: String,
        armed_guard_count: u16,
        restricted_zones: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ShipSecurityAssessment {
    assessment_id: u64,
    vessel_imo: u32,
    security_level: IspsSecurityLevel,
    cso_name: String,
    sso_name: String,
    drills_conducted: u16,
    last_drill_epoch: u64,
    access_control_points: Vec<String>,
    cctv_camera_count: u16,
}

// --- Marine weather routing ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum WeatherRoutingDecision {
    ProceedAsPlanned,
    AlterCourse {
        new_heading_tenths: u16,
        reason: String,
    },
    ReduceSpeed {
        target_knots_tenths: u16,
        sea_state_beaufort: u8,
    },
    Heave {
        expected_duration_hours: u16,
        wind_force_beaufort: u8,
        wave_height_dm: u16,
    },
    DivertToPort {
        port_unlocode: String,
        reason: String,
    },
    AwaitWeatherWindow {
        resume_epoch: u64,
    },
    FollowGreatCircle {
        waypoints_count: u16,
        savings_nm: u16,
    },
    TakeRhumbLine {
        bearing_tenths: u16,
    },
    AvoidTropicalCyclone {
        cyclone_name: String,
        closest_approach_nm: u16,
        avoidance_side: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VoyageWeatherLog {
    log_id: u64,
    voyage_number: String,
    decision: WeatherRoutingDecision,
    position_lat_tenths: i32,
    position_lon_tenths: i32,
    decision_epoch: u64,
    master_approval: bool,
}

// ===== TESTS =====

#[test]
fn test_vessel_identity_crude_tanker() {
    let vessel = VesselIdentity {
        mmsi: 211_000_001,
        imo_number: 9_800_001,
        call_sign: "DJHQ".to_string(),
        vessel_name: "EUROPA SPIRIT".to_string(),
        vessel_type: VesselType::CrudeTanker,
        gross_tonnage: 160_000,
        deadweight_tonnes: 300_000,
        length_overall_cm: 33_300,
        beam_cm: 6_000,
        draft_max_mm: 22_500,
        flag_state: "DEU".to_string(),
    };
    let bytes = encode_to_vec(&vessel).expect("encode VesselIdentity crude tanker");
    let (decoded, _) =
        decode_from_slice::<VesselIdentity>(&bytes).expect("decode VesselIdentity crude tanker");
    assert_eq!(vessel, decoded);
}

#[test]
fn test_ais_position_report_class_a() {
    let frame = AisFrame {
        mmsi: 244_000_999,
        repeat_indicator: 0,
        message: AisMessageType::PositionReportClassA {
            nav_status: 0, // under way using engine
            rate_of_turn: 127,
            speed_over_ground_tenths: 142,
            longitude_ten_thousandths: 40_000_000,
            latitude_ten_thousandths: 515_000_00,
            course_over_ground_tenths: 2100,
            true_heading: 210,
            timestamp_second: 34,
        },
        receive_timestamp_epoch: 1_700_000_000,
    };
    let bytes = encode_to_vec(&frame).expect("encode AIS class A position");
    let (decoded, _) = decode_from_slice::<AisFrame>(&bytes).expect("decode AIS class A position");
    assert_eq!(frame, decoded);
}

#[test]
fn test_ais_static_voyage_data() {
    let frame = AisFrame {
        mmsi: 311_000_555,
        repeat_indicator: 1,
        message: AisMessageType::StaticVoyageData {
            imo_number: 9_876_543,
            destination: "USNYC".to_string(),
            eta_month: 3,
            eta_day: 15,
            eta_hour: 14,
            eta_minute: 30,
            draught_tenths: 125,
        },
        receive_timestamp_epoch: 1_700_001_000,
    };
    let bytes = encode_to_vec(&frame).expect("encode AIS static voyage data");
    let (decoded, _) =
        decode_from_slice::<AisFrame>(&bytes).expect("decode AIS static voyage data");
    assert_eq!(frame, decoded);
}

#[test]
fn test_ais_safety_broadcast() {
    let frame = AisFrame {
        mmsi: 2_190_001,
        repeat_indicator: 0,
        message: AisMessageType::SafetyBroadcast {
            text: "SECURITE SECURITE. DRIFTING CONTAINER REPORTED 51-30N 003-15E. ALL VESSELS NAVIGATE WITH CAUTION.".to_string(),
        },
        receive_timestamp_epoch: 1_700_002_000,
    };
    let bytes = encode_to_vec(&frame).expect("encode AIS safety broadcast");
    let (decoded, _) = decode_from_slice::<AisFrame>(&bytes).expect("decode AIS safety broadcast");
    assert_eq!(frame, decoded);
}

#[test]
fn test_ballast_water_treatment_uv() {
    let record = BallastRecord {
        record_id: 10_001,
        vessel_imo: 9_800_001,
        state: BallastWaterState::TreatmentUv {
            tank_id: 3,
            uv_dosage_mj_per_cm2: 40,
        },
        salinity_ppt_tenths: 350,
        temperature_celsius_tenths: 185,
        timestamp_epoch: 1_700_010_000,
    };
    let bytes = encode_to_vec(&record).expect("encode ballast UV treatment");
    let (decoded, _) =
        decode_from_slice::<BallastRecord>(&bytes).expect("decode ballast UV treatment");
    assert_eq!(record, decoded);
}

#[test]
fn test_ballast_water_exchanging_mid_ocean() {
    let record = BallastRecord {
        record_id: 10_002,
        vessel_imo: 9_800_002,
        state: BallastWaterState::Exchanging {
            tank_id: 5,
            exchange_latitude: 35_000_000,
            exchange_longitude: -45_000_000,
            cycle: 3,
        },
        salinity_ppt_tenths: 355,
        temperature_celsius_tenths: 220,
        timestamp_epoch: 1_700_020_000,
    };
    let bytes = encode_to_vec(&record).expect("encode ballast exchange");
    let (decoded, _) = decode_from_slice::<BallastRecord>(&bytes).expect("decode ballast exchange");
    assert_eq!(record, decoded);
}

#[test]
fn test_engine_room_alarm_bearing_temp_critical() {
    let event = AlarmEvent {
        event_id: 50_001,
        alarm: EngineRoomAlarm::MainEngineBearingTemp {
            cylinder: 4,
            temp_celsius_tenths: 1_850,
            severity: AlarmSeverity::Critical,
        },
        acknowledged: false,
        timestamp_epoch: 1_700_100_000,
        officer_on_watch: "2/E Nakamura".to_string(),
    };
    let bytes = encode_to_vec(&event).expect("encode bearing temp alarm");
    let (decoded, _) = decode_from_slice::<AlarmEvent>(&bytes).expect("decode bearing temp alarm");
    assert_eq!(event, decoded);
}

#[test]
fn test_engine_room_alarm_scavenge_fire() {
    let event = AlarmEvent {
        event_id: 50_002,
        alarm: EngineRoomAlarm::ScavengeFireDetected { cylinder: 6 },
        acknowledged: true,
        timestamp_epoch: 1_700_100_500,
        officer_on_watch: "C/E Petersen".to_string(),
    };
    let bytes = encode_to_vec(&event).expect("encode scavenge fire alarm");
    let (decoded, _) = decode_from_slice::<AlarmEvent>(&bytes).expect("decode scavenge fire alarm");
    assert_eq!(event, decoded);
}

#[test]
fn test_engine_room_alarm_turbocharger_surge() {
    let event = AlarmEvent {
        event_id: 50_003,
        alarm: EngineRoomAlarm::TurbochargerSurge {
            tc_id: 1,
            rpm: 18_500,
            vibration_mm_per_s: 45,
        },
        acknowledged: false,
        timestamp_epoch: 1_700_101_000,
        officer_on_watch: "3/E Kim".to_string(),
    };
    let bytes = encode_to_vec(&event).expect("encode turbocharger surge alarm");
    let (decoded, _) =
        decode_from_slice::<AlarmEvent>(&bytes).expect("decode turbocharger surge alarm");
    assert_eq!(event, decoded);
}

#[test]
fn test_cargo_stowage_container_plan() {
    let plan = StowagePlan {
        plan_id: 70_001,
        voyage_number: "V2024-E-012".to_string(),
        cargo: CargoCategory::Container {
            teu_count: 8_500,
            reefer_count: 320,
            imdg_count: 45,
        },
        hold_number: 3,
        weight_mt: 95_000,
        volume_cbm: 150_000,
        loading_port: "SGSIN".to_string(),
        discharge_port: "NLRTM".to_string(),
    };
    let bytes = encode_to_vec(&plan).expect("encode container stowage plan");
    let (decoded, _) =
        decode_from_slice::<StowagePlan>(&bytes).expect("decode container stowage plan");
    assert_eq!(plan, decoded);
}

#[test]
fn test_cargo_stowage_heavy_lift() {
    let plan = StowagePlan {
        plan_id: 70_002,
        voyage_number: "V2024-P-003".to_string(),
        cargo: CargoCategory::HeavyLift {
            piece_weight_tonnes: 450,
            crane_required_tonnes: 500,
        },
        hold_number: 2,
        weight_mt: 450,
        volume_cbm: 1_200,
        loading_port: "KRPUS".to_string(),
        discharge_port: "BRSSZ".to_string(),
    };
    let bytes = encode_to_vec(&plan).expect("encode heavy lift stowage");
    let (decoded, _) = decode_from_slice::<StowagePlan>(&bytes).expect("decode heavy lift stowage");
    assert_eq!(plan, decoded);
}

#[test]
fn test_solas_equipment_inventory() {
    let inventory = SafetyInventory {
        vessel_imo: 9_800_010,
        equipment: vec![
            SolasEquipmentType::Lifeboat {
                capacity_persons: 150,
                davit_type: "gravity".to_string(),
            },
            SolasEquipmentType::LifeRaft {
                capacity_persons: 25,
                hydrostatic_release: true,
            },
            SolasEquipmentType::Epirb {
                mmsi: 211_000_001,
                battery_expiry_epoch: 1_750_000_000,
            },
            SolasEquipmentType::ImmersionSuit {
                size: "universal".to_string(),
                count: 30,
            },
        ],
        last_inspection_epoch: 1_690_000_000,
        next_inspection_epoch: 1_720_000_000,
        inspector_name: "Capt. Johansson (DNV)".to_string(),
    };
    let bytes = encode_to_vec(&inventory).expect("encode SOLAS inventory");
    let (decoded, _) =
        decode_from_slice::<SafetyInventory>(&bytes).expect("decode SOLAS inventory");
    assert_eq!(inventory, decoded);
}

#[test]
fn test_solas_fixed_fire_system() {
    let inventory = SafetyInventory {
        vessel_imo: 9_800_020,
        equipment: vec![
            SolasEquipmentType::FixedFireSystem {
                agent: "CO2".to_string(),
                coverage_zone: "engine room".to_string(),
                capacity_kg: 5_000,
            },
            SolasEquipmentType::BreathingApparatus {
                cylinder_minutes: 30,
                set_count: 5,
            },
        ],
        last_inspection_epoch: 1_695_000_000,
        next_inspection_epoch: 1_725_000_000,
        inspector_name: "Surveyor Lee (LR)".to_string(),
    };
    let bytes = encode_to_vec(&inventory).expect("encode fire system inventory");
    let (decoded, _) =
        decode_from_slice::<SafetyInventory>(&bytes).expect("decode fire system inventory");
    assert_eq!(inventory, decoded);
}

#[test]
fn test_port_clearance_full_sequence() {
    let port_call = PortCall {
        call_id: 90_001,
        vessel_imo: 9_900_123,
        port_unlocode: "NLRTM".to_string(),
        stages: vec![
            ClearanceStage::PreArrivalNotification {
                eta_epoch: 1_700_200_000,
                last_port: "GBFXT".to_string(),
                crew_count: 22,
                pax_count: 0,
            },
            ClearanceStage::PilotRequested {
                boarding_position_lat: 519_500_00,
                boarding_position_lon: 39_000_00,
                pilot_eta_epoch: 1_700_199_000,
            },
            ClearanceStage::PilotOnBoard {
                pilot_id: "NL-RTM-P42".to_string(),
            },
            ClearanceStage::BerthAssigned {
                berth_code: "ECTDV-7".to_string(),
                side_alongside: 1,
            },
            ClearanceStage::CustomsCleared {
                declaration_number: "NL2024-ENS-88432".to_string(),
            },
            ClearanceStage::CargoOperationsApproved {
                terminal_operator: "ECT Delta Terminal".to_string(),
            },
        ],
    };
    let bytes = encode_to_vec(&port_call).expect("encode port call full sequence");
    let (decoded, _) =
        decode_from_slice::<PortCall>(&bytes).expect("decode port call full sequence");
    assert_eq!(port_call, decoded);
}

#[test]
fn test_port_clearance_health_quarantine() {
    let port_call = PortCall {
        call_id: 90_002,
        vessel_imo: 9_850_077,
        port_unlocode: "SGSIN".to_string(),
        stages: vec![ClearanceStage::HealthCleared {
            pratique_granted: false,
            quarantine_flag: true,
        }],
    };
    let bytes = encode_to_vec(&port_call).expect("encode health quarantine");
    let (decoded, _) = decode_from_slice::<PortCall>(&bytes).expect("decode health quarantine");
    assert_eq!(port_call, decoded);
}

#[test]
fn test_vts_traffic_separation_scheme() {
    let passage = VtsPassage {
        passage_id: 200_001,
        vessel_mmsi: 244_000_999,
        zone: VtsZoneType::TrafficSeparationScheme {
            lane_id: "DOVER-NE-1".to_string(),
            direction_degrees: 45,
        },
        entry_epoch: 1_700_300_000,
        exit_epoch: Some(1_700_303_600),
        compliant: true,
    };
    let bytes = encode_to_vec(&passage).expect("encode VTS TSS passage");
    let (decoded, _) = decode_from_slice::<VtsPassage>(&bytes).expect("decode VTS TSS passage");
    assert_eq!(passage, decoded);
}

#[test]
fn test_vts_speed_restriction_zone() {
    let passage = VtsPassage {
        passage_id: 200_002,
        vessel_mmsi: 538_000_042,
        zone: VtsZoneType::SpeedRestrictionZone {
            max_speed_knots_tenths: 80,
            reason: "Right whale seasonal protection".to_string(),
        },
        entry_epoch: 1_700_400_000,
        exit_epoch: None,
        compliant: false,
    };
    let bytes = encode_to_vec(&passage).expect("encode VTS speed restriction");
    let (decoded, _) =
        decode_from_slice::<VtsPassage>(&bytes).expect("decode VTS speed restriction");
    assert_eq!(passage, decoded);
}

#[test]
fn test_bunker_fuel_vlsfo_delivery() {
    let bdn = BunkerDeliveryNote {
        bdn_number: "BDN-RTM-2024-04521".to_string(),
        grade: BunkerFuelGrade::VlsfoZeroPointFive { sulphur_ppm: 4_500 },
        quantity_mt_tenths: 15_000,
        density_kg_per_m3: 991,
        water_content_ppm: 200,
        supplier: "Vitol Bunkers BV".to_string(),
        port_of_delivery: "NLRTM".to_string(),
        delivery_epoch: 1_700_500_000,
    };
    let bytes = encode_to_vec(&bdn).expect("encode VLSFO bunker delivery");
    let (decoded, _) =
        decode_from_slice::<BunkerDeliveryNote>(&bytes).expect("decode VLSFO bunker delivery");
    assert_eq!(bdn, decoded);
}

#[test]
fn test_bunker_fuel_lng_delivery() {
    let bdn = BunkerDeliveryNote {
        bdn_number: "BDN-SIN-2024-LNG-0088".to_string(),
        grade: BunkerFuelGrade::Lng {
            methane_number: 75,
            boil_off_rate_percent_hundredths: 15,
        },
        quantity_mt_tenths: 30_000,
        density_kg_per_m3: 450,
        water_content_ppm: 0,
        supplier: "Pavilion Energy".to_string(),
        port_of_delivery: "SGSIN".to_string(),
        delivery_epoch: 1_700_510_000,
    };
    let bytes = encode_to_vec(&bdn).expect("encode LNG bunker delivery");
    let (decoded, _) =
        decode_from_slice::<BunkerDeliveryNote>(&bytes).expect("decode LNG bunker delivery");
    assert_eq!(bdn, decoded);
}

#[test]
fn test_classification_special_survey_with_conditions() {
    let report = ClassSurveyReport {
        report_id: 300_001,
        vessel_imo: 9_800_001,
        society_code: "DNV".to_string(),
        survey: SurveyType::SpecialSurvey {
            dry_dock_required: true,
            thickness_measurements: 2_500,
        },
        surveyor_name: "Ing. Muller".to_string(),
        date_epoch: 1_700_600_000,
        next_due_epoch: 1_858_000_000,
        conditions_of_class: vec![
            "Renew No.3 ballast tank coating within 6 months".to_string(),
            "Submit fatigue analysis for midship bracket".to_string(),
            "Repair pitting on weather deck plating frame 120-125".to_string(),
        ],
    };
    let bytes = encode_to_vec(&report).expect("encode special survey report");
    let (decoded, _) =
        decode_from_slice::<ClassSurveyReport>(&bytes).expect("decode special survey report");
    assert_eq!(report, decoded);
}

#[test]
fn test_isps_security_level3_exceptional() {
    let assessment = ShipSecurityAssessment {
        assessment_id: 400_001,
        vessel_imo: 9_900_123,
        security_level: IspsSecurityLevel::Level3Exceptional {
            threat_type: "Piracy - Gulf of Aden transit".to_string(),
            armed_guard_count: 4,
            restricted_zones: vec![
                "bridge wings".to_string(),
                "poop deck".to_string(),
                "forecastle".to_string(),
            ],
        },
        cso_name: "J. Hansen".to_string(),
        sso_name: "Capt. R. Singh".to_string(),
        drills_conducted: 12,
        last_drill_epoch: 1_700_650_000,
        access_control_points: vec![
            "gangway".to_string(),
            "pilot ladder".to_string(),
            "stern ramp".to_string(),
        ],
        cctv_camera_count: 48,
    };
    let bytes = encode_to_vec(&assessment).expect("encode ISPS level 3 assessment");
    let (decoded, _) = decode_from_slice::<ShipSecurityAssessment>(&bytes)
        .expect("decode ISPS level 3 assessment");
    assert_eq!(assessment, decoded);
}

#[test]
fn test_weather_routing_cyclone_avoidance() {
    let log = VoyageWeatherLog {
        log_id: 500_001,
        voyage_number: "V2024-W-007".to_string(),
        decision: WeatherRoutingDecision::AvoidTropicalCyclone {
            cyclone_name: "TYPHOON MAWAR".to_string(),
            closest_approach_nm: 250,
            avoidance_side: "south".to_string(),
        },
        position_lat_tenths: 200_000,
        position_lon_tenths: 1_350_000,
        decision_epoch: 1_700_700_000,
        master_approval: true,
    };
    let bytes = encode_to_vec(&log).expect("encode cyclone avoidance decision");
    let (decoded, _) =
        decode_from_slice::<VoyageWeatherLog>(&bytes).expect("decode cyclone avoidance decision");
    assert_eq!(log, decoded);
}
