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

// ── Domain types: Fire Protection Engineering & Suppression Systems ─────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum SprinklerHeadType {
    Pendant,
    Upright,
    Sidewall,
    Concealed,
    ExtendedCoverage,
    EarlySuppressionFastResponse,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SuppressionAgent {
    Water,
    WetChemical,
    DryChemical,
    CleanAgentFm200,
    CleanAgentNovec1230,
    Co2,
    FoamAfff,
    FoamArAfff,
    InertGasInergen,
    InertGasArgonite,
    WaterMist,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum AlarmDeviceType {
    PhotoelectricSmoke,
    IonizationSmoke,
    HeatFixedTemp,
    HeatRateOfRise,
    MultiSensor,
    BeamDetector,
    AspiratingSample,
    FlameIr,
    FlameUv,
    FlameUvIr,
    ManualPullStation,
    WaterflowSwitch,
    TamperSwitch,
    DuctDetector,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FireResistanceRating {
    Rating30Min,
    Rating60Min,
    Rating90Min,
    Rating120Min,
    Rating180Min,
    Rating240Min,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OccupancyHazardClass {
    LightHazard,
    OrdinaryHazardGroup1,
    OrdinaryHazardGroup2,
    ExtraHazardGroup1,
    ExtraHazardGroup2,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum InspectionResult {
    Pass,
    PassWithNotes {
        notes: String,
    },
    Fail {
        deficiency_code: String,
        description: String,
    },
    NotApplicable,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FireCauseCategory {
    Electrical,
    Cooking,
    Heating,
    Smoking,
    Arson,
    NaturalCauses,
    MechanicalFailure,
    ChildrenPlaying,
    Undetermined,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SprinklerHead {
    head_id: String,
    head_type: SprinklerHeadType,
    temperature_rating_f: u16,
    k_factor: f64,
    coverage_area_sqft: f64,
    floor_number: i8,
    installed_year: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SprinklerSystemLayout {
    system_id: String,
    building_name: String,
    hazard_class: OccupancyHazardClass,
    design_density_gpm_sqft: f64,
    remote_area_sqft: f64,
    number_of_heads: u32,
    heads: Vec<SprinklerHead>,
    water_supply_psi: f64,
    main_pipe_diameter_inches: f64,
    is_antifreeze_system: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlarmZone {
    zone_id: u16,
    zone_name: String,
    floor: i8,
    device_count: u16,
    devices: Vec<AlarmDevice>,
    is_cross_zoned: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AlarmDevice {
    device_address: String,
    device_type: AlarmDeviceType,
    location_description: String,
    sensitivity_percent: f32,
    last_test_epoch: u64,
    is_active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FireAlarmPanel {
    panel_id: String,
    manufacturer: String,
    model: String,
    firmware_version: String,
    total_zones: u16,
    zones: Vec<AlarmZone>,
    has_voice_evac: bool,
    has_mass_notification: bool,
    battery_backup_hours: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SmokeDetectorReading {
    detector_id: String,
    timestamp_epoch: u64,
    obscuration_percent_per_foot: f32,
    chamber_voltage_mv: f32,
    ambient_temperature_f: f32,
    is_alarm_state: bool,
    is_trouble_state: bool,
    drift_compensation_percent: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FirePumpCurvePoint {
    flow_gpm: f64,
    pressure_psi: f64,
    power_hp: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FirePumpPerformance {
    pump_id: String,
    rated_flow_gpm: f64,
    rated_pressure_psi: f64,
    rated_speed_rpm: u32,
    driver_type: String,
    curve_points: Vec<FirePumpCurvePoint>,
    churn_pressure_psi: f64,
    overload_flow_gpm: f64,
    overload_pressure_psi: f64,
    net_positive_suction_head_ft: f64,
    test_date_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StandpipeRiser {
    riser_id: String,
    floor_served: i8,
    static_pressure_psi: f64,
    residual_pressure_psi: f64,
    hose_connection_count: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StandpipeSystem {
    system_id: String,
    class: String,
    system_type: String,
    risers: Vec<StandpipeRiser>,
    fire_department_connection_count: u8,
    roof_manifold_present: bool,
    pressure_reducing_valves: bool,
    max_pressure_psi: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FireDoorInspection {
    door_id: String,
    location: String,
    fire_rating: FireResistanceRating,
    inspection_date_epoch: u64,
    inspector_name: String,
    self_closing_functional: InspectionResult,
    latching_functional: InspectionResult,
    no_visible_damage: InspectionResult,
    gap_clearance_ok: InspectionResult,
    signage_present: InspectionResult,
    glazing_intact: InspectionResult,
    overall_result: InspectionResult,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EgressPathSegment {
    segment_id: String,
    description: String,
    travel_distance_ft: f64,
    width_inches: f64,
    occupant_load: u32,
    is_accessible: bool,
    has_emergency_lighting: bool,
    has_exit_signage: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EgressAnalysis {
    building_id: String,
    floor: i8,
    total_occupant_load: u32,
    required_exits: u8,
    provided_exits: u8,
    max_common_path_ft: f64,
    max_dead_end_ft: f64,
    max_travel_distance_ft: f64,
    segments: Vec<EgressPathSegment>,
    compliant: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FireResistanceAssembly {
    assembly_id: String,
    ul_design_number: String,
    rating: FireResistanceRating,
    assembly_type: String,
    thickness_inches: f64,
    components: Vec<String>,
    hourly_rating_achieved: f64,
    tested_per_astm_e119: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SuppressionSystemConfig {
    system_id: String,
    protected_area_name: String,
    agent: SuppressionAgent,
    agent_quantity_lbs: f64,
    design_concentration_percent: f64,
    discharge_time_seconds: f64,
    hold_time_minutes: f64,
    number_of_nozzles: u16,
    abort_switch_present: bool,
    pre_discharge_alarm_seconds: u16,
    enclosure_integrity_tested: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FireInvestigationReport {
    case_number: String,
    incident_date_epoch: u64,
    address: String,
    cause_category: FireCauseCategory,
    area_of_origin: String,
    estimated_damage_dollars: u64,
    civilian_injuries: u16,
    civilian_fatalities: u16,
    firefighter_injuries: u16,
    sprinkler_present: bool,
    sprinkler_operated: bool,
    alarm_present: bool,
    alarm_operated: bool,
    narrative: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CodeComplianceItem {
    code_section: String,
    description: String,
    result: InspectionResult,
    corrective_action: Option<String>,
    due_date_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CodeComplianceChecklist {
    checklist_id: String,
    building_id: String,
    inspector_id: String,
    inspection_date_epoch: u64,
    code_edition: String,
    items: Vec<CodeComplianceItem>,
    overall_compliant: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct HydrantFlowTest {
    hydrant_id: String,
    test_date_epoch: u64,
    static_pressure_psi: f64,
    residual_pressure_psi: f64,
    pitot_pressure_psi: f64,
    flow_gpm: f64,
    coefficient: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EmergencyLightingFixture {
    fixture_id: String,
    location: String,
    battery_type: String,
    illumination_fc: f32,
    duration_test_passed: bool,
    functional_test_passed: bool,
    last_90day_test_epoch: u64,
    last_annual_test_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FireExtinguisher {
    extinguisher_id: String,
    location: String,
    agent_type: String,
    size_lbs: f32,
    rating: String,
    manufacture_year: u16,
    last_inspection_epoch: u64,
    last_hydrostatic_test_epoch: u64,
    pressure_gauge_ok: bool,
    tamper_seal_intact: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SmokeControlZone {
    zone_id: String,
    zone_name: String,
    floor: i8,
    supply_cfm: f64,
    exhaust_cfm: f64,
    pressure_differential_inches_wg: f64,
    damper_count: u16,
    fan_operational: bool,
    mode: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SmokeControlSystem {
    system_id: String,
    building_id: String,
    system_type: String,
    zones: Vec<SmokeControlZone>,
    activation_method: String,
    backup_power: bool,
    last_test_epoch: u64,
}

// ── Test 1: Sprinkler system layout file roundtrip ──────────────────────────

#[test]
fn test_sprinkler_system_layout_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_sprinkler_layout_{}.bin",
        std::process::id()
    ));
    let original = SprinklerSystemLayout {
        system_id: "SPK-2026-001".into(),
        building_name: "Meridian Office Tower".into(),
        hazard_class: OccupancyHazardClass::LightHazard,
        design_density_gpm_sqft: 0.10,
        remote_area_sqft: 1500.0,
        number_of_heads: 3,
        heads: vec![
            SprinklerHead {
                head_id: "H-001".into(),
                head_type: SprinklerHeadType::Pendant,
                temperature_rating_f: 155,
                k_factor: 5.6,
                coverage_area_sqft: 225.0,
                floor_number: 3,
                installed_year: 2024,
            },
            SprinklerHead {
                head_id: "H-002".into(),
                head_type: SprinklerHeadType::Upright,
                temperature_rating_f: 200,
                k_factor: 8.0,
                coverage_area_sqft: 196.0,
                floor_number: 3,
                installed_year: 2024,
            },
            SprinklerHead {
                head_id: "H-003".into(),
                head_type: SprinklerHeadType::EarlySuppressionFastResponse,
                temperature_rating_f: 165,
                k_factor: 14.0,
                coverage_area_sqft: 100.0,
                floor_number: -1,
                installed_year: 2025,
            },
        ],
        water_supply_psi: 85.0,
        main_pipe_diameter_inches: 6.0,
        is_antifreeze_system: false,
    };
    encode_to_file(&original, &path).expect("encode sprinkler layout to file");
    let decoded: SprinklerSystemLayout =
        decode_from_file(&path).expect("decode sprinkler layout from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 2: Fire alarm panel with zones via encode_to_vec ───────────────────

#[test]
fn test_fire_alarm_panel_vec_roundtrip() {
    let original = FireAlarmPanel {
        panel_id: "FAP-B12-MAIN".into(),
        manufacturer: "Notifier".into(),
        model: "NFS2-3030".into(),
        firmware_version: "7.2.1".into(),
        total_zones: 2,
        zones: vec![
            AlarmZone {
                zone_id: 1,
                zone_name: "Lobby / Main Entrance".into(),
                floor: 1,
                device_count: 2,
                devices: vec![
                    AlarmDevice {
                        device_address: "L1-SD-001".into(),
                        device_type: AlarmDeviceType::PhotoelectricSmoke,
                        location_description: "Above reception desk".into(),
                        sensitivity_percent: 2.5,
                        last_test_epoch: 1735689600,
                        is_active: true,
                    },
                    AlarmDevice {
                        device_address: "L1-MPS-001".into(),
                        device_type: AlarmDeviceType::ManualPullStation,
                        location_description: "East stairwell door".into(),
                        sensitivity_percent: 0.0,
                        last_test_epoch: 1735689600,
                        is_active: true,
                    },
                ],
                is_cross_zoned: false,
            },
            AlarmZone {
                zone_id: 2,
                zone_name: "Server Room B".into(),
                floor: -1,
                device_count: 1,
                devices: vec![AlarmDevice {
                    device_address: "B1-VESDA-001".into(),
                    device_type: AlarmDeviceType::AspiratingSample,
                    location_description: "VESDA pipe network in ceiling plenum".into(),
                    sensitivity_percent: 0.03,
                    last_test_epoch: 1735689600,
                    is_active: true,
                }],
                is_cross_zoned: true,
            },
        ],
        has_voice_evac: true,
        has_mass_notification: false,
        battery_backup_hours: 24.0,
    };
    let bytes = encode_to_vec(&original).expect("encode alarm panel");
    let (decoded, _): (FireAlarmPanel, _) = decode_from_slice(&bytes).expect("decode alarm panel");
    assert_eq!(original, decoded);
}

// ── Test 3: Smoke detector readings batch file roundtrip ────────────────────

#[test]
fn test_smoke_detector_readings_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_smoke_readings_{}.bin",
        std::process::id()
    ));
    let original: Vec<SmokeDetectorReading> = (0..50)
        .map(|i| SmokeDetectorReading {
            detector_id: format!("SD-{:04}", i),
            timestamp_epoch: 1735689600 + i * 60,
            obscuration_percent_per_foot: 0.5 + (i as f32) * 0.02,
            chamber_voltage_mv: 1200.0 + (i as f32) * 3.5,
            ambient_temperature_f: 68.0 + (i as f32) * 0.1,
            is_alarm_state: i > 40,
            is_trouble_state: i == 25,
            drift_compensation_percent: 2.0 + (i as f32) * 0.05,
        })
        .collect();
    encode_to_file(&original, &path).expect("encode smoke readings to file");
    let decoded: Vec<SmokeDetectorReading> =
        decode_from_file(&path).expect("decode smoke readings from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 4: Fire pump performance curve ─────────────────────────────────────

#[test]
fn test_fire_pump_performance_vec_roundtrip() {
    let original = FirePumpPerformance {
        pump_id: "FP-001-ELEC".into(),
        rated_flow_gpm: 1000.0,
        rated_pressure_psi: 100.0,
        rated_speed_rpm: 3550,
        driver_type: "Electric Motor 75HP".into(),
        curve_points: vec![
            FirePumpCurvePoint {
                flow_gpm: 0.0,
                pressure_psi: 130.0,
                power_hp: 15.0,
            },
            FirePumpCurvePoint {
                flow_gpm: 250.0,
                pressure_psi: 125.0,
                power_hp: 30.0,
            },
            FirePumpCurvePoint {
                flow_gpm: 500.0,
                pressure_psi: 118.0,
                power_hp: 45.0,
            },
            FirePumpCurvePoint {
                flow_gpm: 750.0,
                pressure_psi: 110.0,
                power_hp: 58.0,
            },
            FirePumpCurvePoint {
                flow_gpm: 1000.0,
                pressure_psi: 100.0,
                power_hp: 70.0,
            },
            FirePumpCurvePoint {
                flow_gpm: 1250.0,
                pressure_psi: 88.0,
                power_hp: 78.0,
            },
            FirePumpCurvePoint {
                flow_gpm: 1500.0,
                pressure_psi: 65.0,
                power_hp: 82.0,
            },
        ],
        churn_pressure_psi: 130.0,
        overload_flow_gpm: 1500.0,
        overload_pressure_psi: 65.0,
        net_positive_suction_head_ft: 15.0,
        test_date_epoch: 1738368000,
    };
    let bytes = encode_to_vec(&original).expect("encode pump performance");
    let (decoded, _): (FirePumpPerformance, _) =
        decode_from_slice(&bytes).expect("decode pump performance");
    assert_eq!(original, decoded);
}

// ── Test 5: Standpipe system file roundtrip ─────────────────────────────────

#[test]
fn test_standpipe_system_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_standpipe_{}.bin",
        std::process::id()
    ));
    let original = StandpipeSystem {
        system_id: "STP-WEST-01".into(),
        class: "Class I".into(),
        system_type: "Automatic Wet".into(),
        risers: vec![
            StandpipeRiser {
                riser_id: "R-W-01".into(),
                floor_served: 1,
                static_pressure_psi: 175.0,
                residual_pressure_psi: 100.0,
                hose_connection_count: 2,
            },
            StandpipeRiser {
                riser_id: "R-W-02".into(),
                floor_served: 10,
                static_pressure_psi: 135.0,
                residual_pressure_psi: 85.0,
                hose_connection_count: 2,
            },
            StandpipeRiser {
                riser_id: "R-W-03".into(),
                floor_served: 20,
                static_pressure_psi: 90.0,
                residual_pressure_psi: 65.0,
                hose_connection_count: 2,
            },
        ],
        fire_department_connection_count: 2,
        roof_manifold_present: true,
        pressure_reducing_valves: true,
        max_pressure_psi: 175.0,
    };
    encode_to_file(&original, &path).expect("encode standpipe to file");
    let decoded: StandpipeSystem = decode_from_file(&path).expect("decode standpipe from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 6: Fire door inspection records ────────────────────────────────────

#[test]
fn test_fire_door_inspection_vec_roundtrip() {
    let original = FireDoorInspection {
        door_id: "FD-3-042".into(),
        location: "Floor 3, Corridor B at Stairwell 2".into(),
        fire_rating: FireResistanceRating::Rating90Min,
        inspection_date_epoch: 1738368000,
        inspector_name: "J. Martinez".into(),
        self_closing_functional: InspectionResult::Pass,
        latching_functional: InspectionResult::Pass,
        no_visible_damage: InspectionResult::Fail {
            deficiency_code: "NFPA80-5.2.4".into(),
            description: "Bottom seal missing, daylight visible at threshold".into(),
        },
        gap_clearance_ok: InspectionResult::PassWithNotes {
            notes: "Hinge-side gap 1/8 inch, within tolerance".into(),
        },
        signage_present: InspectionResult::Pass,
        glazing_intact: InspectionResult::NotApplicable,
        overall_result: InspectionResult::Fail {
            deficiency_code: "NFPA80-5.2.4".into(),
            description: "Door seal replacement required".into(),
        },
    };
    let bytes = encode_to_vec(&original).expect("encode door inspection");
    let (decoded, _): (FireDoorInspection, _) =
        decode_from_slice(&bytes).expect("decode door inspection");
    assert_eq!(original, decoded);
}

// ── Test 7: Egress analysis file roundtrip ──────────────────────────────────

#[test]
fn test_egress_analysis_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_egress_{}.bin",
        std::process::id()
    ));
    let original = EgressAnalysis {
        building_id: "BLD-2026-CAMPUS-A".into(),
        floor: 4,
        total_occupant_load: 320,
        required_exits: 2,
        provided_exits: 3,
        max_common_path_ft: 75.0,
        max_dead_end_ft: 20.0,
        max_travel_distance_ft: 250.0,
        segments: vec![
            EgressPathSegment {
                segment_id: "SEG-4A".into(),
                description: "Open office to Corridor A".into(),
                travel_distance_ft: 80.0,
                width_inches: 44.0,
                occupant_load: 120,
                is_accessible: true,
                has_emergency_lighting: true,
                has_exit_signage: true,
            },
            EgressPathSegment {
                segment_id: "SEG-4B".into(),
                description: "Corridor A to Stairwell 1".into(),
                travel_distance_ft: 110.0,
                width_inches: 44.0,
                occupant_load: 200,
                is_accessible: true,
                has_emergency_lighting: true,
                has_exit_signage: true,
            },
            EgressPathSegment {
                segment_id: "SEG-4C".into(),
                description: "Conference wing to Stairwell 2".into(),
                travel_distance_ft: 60.0,
                width_inches: 36.0,
                occupant_load: 120,
                is_accessible: false,
                has_emergency_lighting: true,
                has_exit_signage: true,
            },
        ],
        compliant: true,
    };
    encode_to_file(&original, &path).expect("encode egress analysis to file");
    let decoded: EgressAnalysis =
        decode_from_file(&path).expect("decode egress analysis from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 8: Fire resistance assembly vec roundtrip ──────────────────────────

#[test]
fn test_fire_resistance_assembly_vec_roundtrip() {
    let original = FireResistanceAssembly {
        assembly_id: "FRA-W-001".into(),
        ul_design_number: "U419".into(),
        rating: FireResistanceRating::Rating120Min,
        assembly_type: "Wall".into(),
        thickness_inches: 5.125,
        components: vec![
            "5/8\" Type X Gypsum Board (face)".into(),
            "3-5/8\" Steel Studs @ 24\" O.C.".into(),
            "R-11 Fiberglass Batt Insulation".into(),
            "5/8\" Type X Gypsum Board (back)".into(),
        ],
        hourly_rating_achieved: 2.25,
        tested_per_astm_e119: true,
    };
    let bytes = encode_to_vec(&original).expect("encode fire resistance assembly");
    let (decoded, _): (FireResistanceAssembly, _) =
        decode_from_slice(&bytes).expect("decode fire resistance assembly");
    assert_eq!(original, decoded);
}

// ── Test 9: Suppression system clean agent file roundtrip ───────────────────

#[test]
fn test_suppression_clean_agent_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_clean_agent_{}.bin",
        std::process::id()
    ));
    let original = SuppressionSystemConfig {
        system_id: "SUP-DC-001".into(),
        protected_area_name: "Primary Data Center Hall A".into(),
        agent: SuppressionAgent::CleanAgentNovec1230,
        agent_quantity_lbs: 1850.0,
        design_concentration_percent: 5.8,
        discharge_time_seconds: 10.0,
        hold_time_minutes: 10.0,
        number_of_nozzles: 24,
        abort_switch_present: true,
        pre_discharge_alarm_seconds: 30,
        enclosure_integrity_tested: true,
    };
    encode_to_file(&original, &path).expect("encode clean agent config to file");
    let decoded: SuppressionSystemConfig =
        decode_from_file(&path).expect("decode clean agent config from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 10: Fire investigation report vec roundtrip ────────────────────────

#[test]
fn test_fire_investigation_report_vec_roundtrip() {
    let original = FireInvestigationReport {
        case_number: "FIR-2026-00142".into(),
        incident_date_epoch: 1738281600,
        address: "4521 Industrial Parkway, Unit 7B".into(),
        cause_category: FireCauseCategory::Electrical,
        area_of_origin: "Electrical panel room, northeast corner".into(),
        estimated_damage_dollars: 2_350_000,
        civilian_injuries: 0,
        civilian_fatalities: 0,
        firefighter_injuries: 1,
        sprinkler_present: true,
        sprinkler_operated: true,
        alarm_present: true,
        alarm_operated: true,
        narrative: "Fire originated at overloaded 200A breaker panel. \
            Arc flash ignited cable insulation. Sprinkler activation \
            contained fire to room of origin. Building evacuated via \
            voice evacuation system prior to FD arrival."
            .into(),
    };
    let bytes = encode_to_vec(&original).expect("encode investigation report");
    let (decoded, _): (FireInvestigationReport, _) =
        decode_from_slice(&bytes).expect("decode investigation report");
    assert_eq!(original, decoded);
}

// ── Test 11: Code compliance checklist file roundtrip ────────────────────────

#[test]
fn test_code_compliance_checklist_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_compliance_{}.bin",
        std::process::id()
    ));
    let original = CodeComplianceChecklist {
        checklist_id: "CCI-2026-0088".into(),
        building_id: "BLD-HOSP-MAIN".into(),
        inspector_id: "INSP-447".into(),
        inspection_date_epoch: 1740000000,
        code_edition: "NFPA 101, 2024 Edition".into(),
        items: vec![
            CodeComplianceItem {
                code_section: "9.6.1".into(),
                description: "Fire alarm system installed per NFPA 72".into(),
                result: InspectionResult::Pass,
                corrective_action: None,
                due_date_epoch: None,
            },
            CodeComplianceItem {
                code_section: "9.7.1".into(),
                description: "Automatic sprinkler system per NFPA 13".into(),
                result: InspectionResult::Pass,
                corrective_action: None,
                due_date_epoch: None,
            },
            CodeComplianceItem {
                code_section: "7.2.1.2".into(),
                description: "Door width meets minimum egress width".into(),
                result: InspectionResult::Fail {
                    deficiency_code: "LS-7.2.1.2-A".into(),
                    description: "Suite 302 door only 30 inches wide".into(),
                },
                corrective_action: Some("Replace door with 36-inch unit".into()),
                due_date_epoch: Some(1742592000),
            },
            CodeComplianceItem {
                code_section: "7.10.1.2".into(),
                description: "Exit signs illuminated and visible".into(),
                result: InspectionResult::PassWithNotes {
                    notes: "Two signs on emergency backup, verified functional".into(),
                },
                corrective_action: None,
                due_date_epoch: None,
            },
        ],
        overall_compliant: false,
    };
    encode_to_file(&original, &path).expect("encode compliance checklist to file");
    let decoded: CodeComplianceChecklist =
        decode_from_file(&path).expect("decode compliance checklist from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 12: Multiple suppression agent types vec roundtrip ─────────────────

#[test]
fn test_multiple_suppression_agents_vec_roundtrip() {
    let agents: Vec<SuppressionSystemConfig> = vec![
        SuppressionSystemConfig {
            system_id: "SUP-KIT-001".into(),
            protected_area_name: "Commercial Kitchen Hood".into(),
            agent: SuppressionAgent::WetChemical,
            agent_quantity_lbs: 40.0,
            design_concentration_percent: 0.0,
            discharge_time_seconds: 45.0,
            hold_time_minutes: 0.0,
            number_of_nozzles: 6,
            abort_switch_present: false,
            pre_discharge_alarm_seconds: 0,
            enclosure_integrity_tested: false,
        },
        SuppressionSystemConfig {
            system_id: "SUP-GEN-001".into(),
            protected_area_name: "Generator Room".into(),
            agent: SuppressionAgent::Co2,
            agent_quantity_lbs: 500.0,
            design_concentration_percent: 34.0,
            discharge_time_seconds: 60.0,
            hold_time_minutes: 20.0,
            number_of_nozzles: 8,
            abort_switch_present: true,
            pre_discharge_alarm_seconds: 60,
            enclosure_integrity_tested: true,
        },
        SuppressionSystemConfig {
            system_id: "SUP-HANG-001".into(),
            protected_area_name: "Aircraft Hangar Bay 3".into(),
            agent: SuppressionAgent::FoamAfff,
            agent_quantity_lbs: 5000.0,
            design_concentration_percent: 3.0,
            discharge_time_seconds: 120.0,
            hold_time_minutes: 15.0,
            number_of_nozzles: 32,
            abort_switch_present: true,
            pre_discharge_alarm_seconds: 15,
            enclosure_integrity_tested: false,
        },
    ];
    let bytes = encode_to_vec(&agents).expect("encode multiple suppression agents");
    let (decoded, _): (Vec<SuppressionSystemConfig>, _) =
        decode_from_slice(&bytes).expect("decode multiple suppression agents");
    assert_eq!(agents, decoded);
}

// ── Test 13: Hydrant flow test file roundtrip ───────────────────────────────

#[test]
fn test_hydrant_flow_test_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_hydrant_{}.bin",
        std::process::id()
    ));
    let original = HydrantFlowTest {
        hydrant_id: "HYD-MV-0247".into(),
        test_date_epoch: 1738368000,
        static_pressure_psi: 68.0,
        residual_pressure_psi: 52.0,
        pitot_pressure_psi: 28.0,
        flow_gpm: 1490.0,
        coefficient: 0.9,
    };
    encode_to_file(&original, &path).expect("encode hydrant flow test to file");
    let decoded: HydrantFlowTest =
        decode_from_file(&path).expect("decode hydrant flow test from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 14: Emergency lighting fixture batch vec roundtrip ─────────────────

#[test]
fn test_emergency_lighting_batch_vec_roundtrip() {
    let fixtures: Vec<EmergencyLightingFixture> = (0..20)
        .map(|i| EmergencyLightingFixture {
            fixture_id: format!("EL-F2-{:03}", i),
            location: format!(
                "Floor 2, Corridor {}, Station {}",
                if i < 10 { "A" } else { "B" },
                i % 10
            ),
            battery_type: if i % 3 == 0 {
                "Sealed Lead Acid".into()
            } else {
                "Nickel Cadmium".into()
            },
            illumination_fc: 1.0 + (i as f32) * 0.05,
            duration_test_passed: i != 7,
            functional_test_passed: true,
            last_90day_test_epoch: 1735689600 + (i as u64) * 86400,
            last_annual_test_epoch: 1704067200,
        })
        .collect();
    let bytes = encode_to_vec(&fixtures).expect("encode emergency lighting batch");
    let (decoded, _): (Vec<EmergencyLightingFixture>, _) =
        decode_from_slice(&bytes).expect("decode emergency lighting batch");
    assert_eq!(fixtures, decoded);
}

// ── Test 15: Fire extinguisher inventory file roundtrip ─────────────────────

#[test]
fn test_fire_extinguisher_inventory_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_extinguisher_{}.bin",
        std::process::id()
    ));
    let inventory: Vec<FireExtinguisher> = vec![
        FireExtinguisher {
            extinguisher_id: "FE-L1-001".into(),
            location: "Lobby, east wall near elevator".into(),
            agent_type: "ABC Dry Chemical".into(),
            size_lbs: 10.0,
            rating: "4-A:80-B:C".into(),
            manufacture_year: 2019,
            last_inspection_epoch: 1735689600,
            last_hydrostatic_test_epoch: 1672531200,
            pressure_gauge_ok: true,
            tamper_seal_intact: true,
        },
        FireExtinguisher {
            extinguisher_id: "FE-K1-001".into(),
            location: "Kitchen, next to hood system".into(),
            agent_type: "Class K Wet Chemical".into(),
            size_lbs: 6.0,
            rating: "K".into(),
            manufacture_year: 2022,
            last_inspection_epoch: 1735689600,
            last_hydrostatic_test_epoch: 0,
            pressure_gauge_ok: true,
            tamper_seal_intact: false,
        },
        FireExtinguisher {
            extinguisher_id: "FE-SR-001".into(),
            location: "Server room entrance".into(),
            agent_type: "CO2".into(),
            size_lbs: 15.0,
            rating: "10-B:C".into(),
            manufacture_year: 2020,
            last_inspection_epoch: 1735689600,
            last_hydrostatic_test_epoch: 1672531200,
            pressure_gauge_ok: true,
            tamper_seal_intact: true,
        },
    ];
    encode_to_file(&inventory, &path).expect("encode extinguisher inventory to file");
    let decoded: Vec<FireExtinguisher> =
        decode_from_file(&path).expect("decode extinguisher inventory from file");
    assert_eq!(inventory, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 16: Smoke control system file roundtrip ────────────────────────────

#[test]
fn test_smoke_control_system_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_smoke_control_{}.bin",
        std::process::id()
    ));
    let original = SmokeControlSystem {
        system_id: "SMK-CTRL-001".into(),
        building_id: "BLD-ATRIUM-MAIN".into(),
        system_type: "Atrium Exhaust".into(),
        zones: vec![
            SmokeControlZone {
                zone_id: "SCZ-AT-01".into(),
                zone_name: "Atrium Main Volume".into(),
                floor: 1,
                supply_cfm: 0.0,
                exhaust_cfm: 45000.0,
                pressure_differential_inches_wg: -0.05,
                damper_count: 8,
                fan_operational: true,
                mode: "Exhaust".into(),
            },
            SmokeControlZone {
                zone_id: "SCZ-AT-02".into(),
                zone_name: "Atrium Adjacent Corridor L2".into(),
                floor: 2,
                supply_cfm: 12000.0,
                exhaust_cfm: 0.0,
                pressure_differential_inches_wg: 0.05,
                damper_count: 4,
                fan_operational: true,
                mode: "Pressurization".into(),
            },
        ],
        activation_method: "Beam Detector + FACP Signal".into(),
        backup_power: true,
        last_test_epoch: 1738368000,
    };
    encode_to_file(&original, &path).expect("encode smoke control system to file");
    let decoded: SmokeControlSystem =
        decode_from_file(&path).expect("decode smoke control system from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 17: Sprinkler head types enum exhaustive ───────────────────────────

#[test]
fn test_sprinkler_head_types_exhaustive_vec_roundtrip() {
    let all_types = vec![
        SprinklerHeadType::Pendant,
        SprinklerHeadType::Upright,
        SprinklerHeadType::Sidewall,
        SprinklerHeadType::Concealed,
        SprinklerHeadType::ExtendedCoverage,
        SprinklerHeadType::EarlySuppressionFastResponse,
    ];
    let bytes = encode_to_vec(&all_types).expect("encode sprinkler head types");
    let (decoded, _): (Vec<SprinklerHeadType>, _) =
        decode_from_slice(&bytes).expect("decode sprinkler head types");
    assert_eq!(all_types, decoded);
}

// ── Test 18: All suppression agents enum exhaustive ─────────────────────────

#[test]
fn test_all_suppression_agents_exhaustive_vec_roundtrip() {
    let all_agents = vec![
        SuppressionAgent::Water,
        SuppressionAgent::WetChemical,
        SuppressionAgent::DryChemical,
        SuppressionAgent::CleanAgentFm200,
        SuppressionAgent::CleanAgentNovec1230,
        SuppressionAgent::Co2,
        SuppressionAgent::FoamAfff,
        SuppressionAgent::FoamArAfff,
        SuppressionAgent::InertGasInergen,
        SuppressionAgent::InertGasArgonite,
        SuppressionAgent::WaterMist,
    ];
    let bytes = encode_to_vec(&all_agents).expect("encode all suppression agents");
    let (decoded, _): (Vec<SuppressionAgent>, _) =
        decode_from_slice(&bytes).expect("decode all suppression agents");
    assert_eq!(all_agents, decoded);
}

// ── Test 19: Fire resistance ratings file roundtrip ─────────────────────────

#[test]
fn test_fire_resistance_ratings_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_ratings_{}.bin",
        std::process::id()
    ));
    let assemblies: Vec<FireResistanceAssembly> = vec![
        FireResistanceAssembly {
            assembly_id: "FRA-F-001".into(),
            ul_design_number: "D916".into(),
            rating: FireResistanceRating::Rating60Min,
            assembly_type: "Floor/Ceiling".into(),
            thickness_inches: 8.0,
            components: vec![
                "6\" Lightweight Concrete Slab".into(),
                "Metal Deck".into(),
                "Spray-applied Fireproofing".into(),
            ],
            hourly_rating_achieved: 1.5,
            tested_per_astm_e119: true,
        },
        FireResistanceAssembly {
            assembly_id: "FRA-C-001".into(),
            ul_design_number: "X528".into(),
            rating: FireResistanceRating::Rating240Min,
            assembly_type: "Column".into(),
            thickness_inches: 2.5,
            components: vec![
                "W14x90 Steel Column".into(),
                "2.5\" Spray-applied Cementitious Fireproofing".into(),
            ],
            hourly_rating_achieved: 4.0,
            tested_per_astm_e119: true,
        },
    ];
    encode_to_file(&assemblies, &path).expect("encode fire resistance assemblies to file");
    let decoded: Vec<FireResistanceAssembly> =
        decode_from_file(&path).expect("decode fire resistance assemblies from file");
    assert_eq!(assemblies, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 20: Complex investigation with arson cause vec roundtrip ────────────

#[test]
fn test_investigation_arson_complex_vec_roundtrip() {
    let original = FireInvestigationReport {
        case_number: "FIR-2026-00201".into(),
        incident_date_epoch: 1740000000,
        address: "1200 Warehouse District Blvd".into(),
        cause_category: FireCauseCategory::Arson,
        area_of_origin: "Loading dock, southeast corner, behind pallets".into(),
        estimated_damage_dollars: 8_750_000,
        civilian_injuries: 2,
        civilian_fatalities: 0,
        firefighter_injuries: 3,
        sprinkler_present: true,
        sprinkler_operated: false,
        alarm_present: true,
        alarm_operated: true,
        narrative: "Multiple points of origin identified. \
            Accelerant (gasoline) detected via GC/MS analysis at three locations. \
            Sprinkler system had been tampered with — main control valve was closed. \
            Security camera footage shows unidentified individual entering through \
            loading dock at 02:17. Alarm activated at 02:23 from waterflow switch \
            in adjacent zone. Structure suffered partial collapse of roof trusses \
            in south bay. Case referred to ATF."
            .into(),
    };
    let bytes = encode_to_vec(&original).expect("encode arson investigation");
    let (decoded, consumed): (FireInvestigationReport, _) =
        decode_from_slice(&bytes).expect("decode arson investigation");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ── Test 21: Nested alarm panel with many devices file roundtrip ────────────

#[test]
fn test_large_alarm_panel_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_large_panel_{}.bin",
        std::process::id()
    ));
    let device_types = [
        AlarmDeviceType::PhotoelectricSmoke,
        AlarmDeviceType::HeatFixedTemp,
        AlarmDeviceType::HeatRateOfRise,
        AlarmDeviceType::MultiSensor,
        AlarmDeviceType::DuctDetector,
        AlarmDeviceType::ManualPullStation,
        AlarmDeviceType::FlameIr,
        AlarmDeviceType::WaterflowSwitch,
    ];
    let zones: Vec<AlarmZone> = (1..=8)
        .map(|z| {
            let devices: Vec<AlarmDevice> = (0..12)
                .map(|d| AlarmDevice {
                    device_address: format!("Z{:02}-D{:03}", z, d),
                    device_type: device_types[(z as usize + d as usize) % device_types.len()]
                        .clone(),
                    location_description: format!(
                        "Floor {}, Grid {}-{}",
                        z,
                        (b'A' + (d % 6)) as char,
                        d / 6 + 1
                    ),
                    sensitivity_percent: 1.0 + (d as f32) * 0.3,
                    last_test_epoch: 1735689600 + (z as u64) * 86400,
                    is_active: d != 5,
                })
                .collect();
            AlarmZone {
                zone_id: z,
                zone_name: format!("Floor {} - Full Coverage", z),
                floor: z as i8,
                device_count: 12,
                devices,
                is_cross_zoned: z <= 2,
            }
        })
        .collect();
    let original = FireAlarmPanel {
        panel_id: "FAP-CAMPUS-MAIN".into(),
        manufacturer: "Siemens".into(),
        model: "Cerberus PRO FC726".into(),
        firmware_version: "4.11.0".into(),
        total_zones: 8,
        zones,
        has_voice_evac: true,
        has_mass_notification: true,
        battery_backup_hours: 48.0,
    };
    encode_to_file(&original, &path).expect("encode large alarm panel to file");
    let decoded: FireAlarmPanel =
        decode_from_file(&path).expect("decode large alarm panel from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 22: Combined building fire protection survey file roundtrip ────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct BuildingFireProtectionSurvey {
    building_id: String,
    building_name: String,
    address: String,
    stories_above_grade: u8,
    stories_below_grade: u8,
    total_area_sqft: u64,
    construction_type: String,
    occupancy_class: OccupancyHazardClass,
    sprinkler_system: Option<SprinklerSystemLayout>,
    alarm_panel: Option<FireAlarmPanel>,
    standpipe_system: Option<StandpipeSystem>,
    suppression_systems: Vec<SuppressionSystemConfig>,
    fire_door_inspections: Vec<FireDoorInspection>,
    egress_analysis: Vec<EgressAnalysis>,
    hydrant_tests: Vec<HydrantFlowTest>,
    extinguishers: Vec<FireExtinguisher>,
    compliance_checklist: Option<CodeComplianceChecklist>,
    surveyor_name: String,
    survey_date_epoch: u64,
}

#[test]
fn test_full_building_survey_file_roundtrip() {
    let path = temp_dir().join(format!(
        "oxicode_test_fire_full_survey_{}.bin",
        std::process::id()
    ));
    let survey = BuildingFireProtectionSurvey {
        building_id: "BLD-2026-HQ".into(),
        building_name: "Corporate Headquarters".into(),
        address: "100 Innovation Drive".into(),
        stories_above_grade: 12,
        stories_below_grade: 2,
        total_area_sqft: 250_000,
        construction_type: "Type I-A (443)".into(),
        occupancy_class: OccupancyHazardClass::LightHazard,
        sprinkler_system: Some(SprinklerSystemLayout {
            system_id: "SPK-HQ-01".into(),
            building_name: "Corporate HQ".into(),
            hazard_class: OccupancyHazardClass::LightHazard,
            design_density_gpm_sqft: 0.10,
            remote_area_sqft: 1500.0,
            number_of_heads: 1,
            heads: vec![SprinklerHead {
                head_id: "H-SAMPLE".into(),
                head_type: SprinklerHeadType::Concealed,
                temperature_rating_f: 155,
                k_factor: 5.6,
                coverage_area_sqft: 225.0,
                floor_number: 1,
                installed_year: 2023,
            }],
            water_supply_psi: 95.0,
            main_pipe_diameter_inches: 8.0,
            is_antifreeze_system: false,
        }),
        alarm_panel: Some(FireAlarmPanel {
            panel_id: "FAP-HQ-MAIN".into(),
            manufacturer: "EST".into(),
            model: "EST4".into(),
            firmware_version: "3.6.0".into(),
            total_zones: 1,
            zones: vec![AlarmZone {
                zone_id: 1,
                zone_name: "Ground Floor".into(),
                floor: 1,
                device_count: 1,
                devices: vec![AlarmDevice {
                    device_address: "GF-SD-001".into(),
                    device_type: AlarmDeviceType::MultiSensor,
                    location_description: "Main lobby ceiling".into(),
                    sensitivity_percent: 1.5,
                    last_test_epoch: 1740000000,
                    is_active: true,
                }],
                is_cross_zoned: false,
            }],
            has_voice_evac: true,
            has_mass_notification: true,
            battery_backup_hours: 72.0,
        }),
        standpipe_system: Some(StandpipeSystem {
            system_id: "STP-HQ-01".into(),
            class: "Class III".into(),
            system_type: "Automatic Wet".into(),
            risers: vec![StandpipeRiser {
                riser_id: "R-01".into(),
                floor_served: 12,
                static_pressure_psi: 150.0,
                residual_pressure_psi: 100.0,
                hose_connection_count: 2,
            }],
            fire_department_connection_count: 2,
            roof_manifold_present: true,
            pressure_reducing_valves: true,
            max_pressure_psi: 175.0,
        }),
        suppression_systems: vec![SuppressionSystemConfig {
            system_id: "SUP-HQ-DC".into(),
            protected_area_name: "12th Floor Data Center".into(),
            agent: SuppressionAgent::CleanAgentFm200,
            agent_quantity_lbs: 900.0,
            design_concentration_percent: 7.0,
            discharge_time_seconds: 10.0,
            hold_time_minutes: 10.0,
            number_of_nozzles: 12,
            abort_switch_present: true,
            pre_discharge_alarm_seconds: 30,
            enclosure_integrity_tested: true,
        }],
        fire_door_inspections: vec![FireDoorInspection {
            door_id: "FD-1-001".into(),
            location: "Floor 1, Stairwell A".into(),
            fire_rating: FireResistanceRating::Rating120Min,
            inspection_date_epoch: 1740000000,
            inspector_name: "A. Chen".into(),
            self_closing_functional: InspectionResult::Pass,
            latching_functional: InspectionResult::Pass,
            no_visible_damage: InspectionResult::Pass,
            gap_clearance_ok: InspectionResult::Pass,
            signage_present: InspectionResult::Pass,
            glazing_intact: InspectionResult::Pass,
            overall_result: InspectionResult::Pass,
        }],
        egress_analysis: vec![EgressAnalysis {
            building_id: "BLD-2026-HQ".into(),
            floor: 1,
            total_occupant_load: 500,
            required_exits: 3,
            provided_exits: 4,
            max_common_path_ft: 75.0,
            max_dead_end_ft: 20.0,
            max_travel_distance_ft: 200.0,
            segments: vec![EgressPathSegment {
                segment_id: "SEG-1-MAIN".into(),
                description: "Main lobby to front entrance".into(),
                travel_distance_ft: 40.0,
                width_inches: 72.0,
                occupant_load: 200,
                is_accessible: true,
                has_emergency_lighting: true,
                has_exit_signage: true,
            }],
            compliant: true,
        }],
        hydrant_tests: vec![HydrantFlowTest {
            hydrant_id: "HYD-HQ-001".into(),
            test_date_epoch: 1738368000,
            static_pressure_psi: 72.0,
            residual_pressure_psi: 58.0,
            pitot_pressure_psi: 32.0,
            flow_gpm: 1600.0,
            coefficient: 0.9,
        }],
        extinguishers: vec![FireExtinguisher {
            extinguisher_id: "FE-HQ-L1-001".into(),
            location: "Lobby, south wall".into(),
            agent_type: "ABC Dry Chemical".into(),
            size_lbs: 10.0,
            rating: "4-A:80-B:C".into(),
            manufacture_year: 2021,
            last_inspection_epoch: 1740000000,
            last_hydrostatic_test_epoch: 1672531200,
            pressure_gauge_ok: true,
            tamper_seal_intact: true,
        }],
        compliance_checklist: Some(CodeComplianceChecklist {
            checklist_id: "CCI-HQ-2026".into(),
            building_id: "BLD-2026-HQ".into(),
            inspector_id: "INSP-100".into(),
            inspection_date_epoch: 1740000000,
            code_edition: "IBC 2024 / NFPA 101 2024".into(),
            items: vec![CodeComplianceItem {
                code_section: "903.2".into(),
                description: "Automatic sprinkler systems required".into(),
                result: InspectionResult::Pass,
                corrective_action: None,
                due_date_epoch: None,
            }],
            overall_compliant: true,
        }),
        surveyor_name: "Licensed Fire Protection Engineer #4821".into(),
        survey_date_epoch: 1740000000,
    };
    encode_to_file(&survey, &path).expect("encode full building survey to file");
    let decoded: BuildingFireProtectionSurvey =
        decode_from_file(&path).expect("decode full building survey from file");
    assert_eq!(survey, decoded);
    std::fs::remove_file(&path).ok();
}
