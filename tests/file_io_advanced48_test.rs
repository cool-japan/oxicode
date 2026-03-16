//! Advanced file I/O tests for OxiCode — domain: mining operations and mineral processing

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OreType {
    Iron,
    Copper,
    Gold,
    Platinum,
    Bauxite,
    Lithium,
    Nickel,
    Zinc,
    Uranium,
    RareEarth,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BlastHolePattern {
    Square { spacing_mm: u32 },
    Staggered { burden_mm: u32, spacing_mm: u32 },
    Echelon { angle_deg_x10: u16, spacing_mm: u32 },
    FanDrill { arc_deg_x10: u16, hole_count: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CrusherType {
    Jaw,
    Cone,
    Gyratory,
    Impact,
    HammerMill,
    RollCrusher,
    SemiAutogenous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FlotationReagent {
    Xanthate,
    Dithiophosphate,
    MethylIsobutylCarbinol,
    PineOil,
    SodiumCyanide,
    LimeSlurry,
    SulfuricAcid,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaintenancePriority {
    Routine,
    Preventive,
    Corrective,
    Emergency,
    Shutdown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VentilationMode {
    NaturalDraft,
    ForcedInlet,
    ExhaustOnly,
    BalancedPressure,
    BoosterAssisted,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TailingsDamStatus {
    Operational,
    UnderConstruction,
    RaisingInProgress,
    Decommissioned,
    AlertMonitoring,
    EmergencyDrawdown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnvironmentalParam {
    Dust { pm10_ug_m3: u32, pm25_ug_m3: u32 },
    Noise { db_level_x10: u16 },
    WaterPh { ph_x100: u16 },
    HeavyMetal { element: String, ppb: u32 },
    GroundVibration { ppv_mm_s_x100: u32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OreGradeAssay {
    sample_id: u64,
    drill_hole_id: u32,
    depth_from_mm: u32,
    depth_to_mm: u32,
    ore_type: OreType,
    grade_ppm: u32,
    moisture_percent_x100: u16,
    specific_gravity_x1000: u16,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BlastDesign {
    blast_id: u64,
    bench_level: i16,
    pattern: BlastHolePattern,
    hole_count: u16,
    hole_diameter_mm: u16,
    hole_depth_mm: u32,
    charge_mass_grams: u32,
    stemming_length_mm: u32,
    delay_interval_ms: u16,
    expected_tonnage_kg: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HaulTruckTelemetry {
    truck_id: u32,
    fleet_id: u8,
    gps_lat_micro: i64,
    gps_lon_micro: i64,
    elevation_mm: i32,
    speed_kmh_x10: u16,
    payload_kg: u32,
    fuel_level_percent_x10: u16,
    engine_rpm: u16,
    oil_pressure_kpa: u16,
    coolant_temp_c_x10: i16,
    tire_pressure_front_kpa: u16,
    tire_pressure_rear_kpa: u16,
    odometer_m: u64,
    engine_hours_x10: u32,
    is_loaded: bool,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrusherThroughput {
    crusher_id: u32,
    crusher_type: CrusherType,
    feed_rate_tph_x10: u32,
    power_draw_kw: u32,
    css_mm_x10: u16,
    oss_mm_x10: u16,
    product_p80_microns: u32,
    liner_wear_percent_x10: u16,
    bearing_temp_c_x10: i16,
    oil_flow_lpm_x10: u16,
    runtime_seconds: u64,
    downtime_seconds: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlotationCell {
    cell_id: u32,
    bank_id: u8,
    cell_volume_liters: u32,
    air_flow_lpm_x10: u32,
    froth_depth_mm: u16,
    pulp_level_percent_x10: u16,
    reagent: FlotationReagent,
    reagent_dose_gpt_x100: u32,
    feed_grade_ppm: u32,
    concentrate_grade_ppm: u32,
    recovery_percent_x100: u16,
    ph_x100: u16,
    pulp_density_percent_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TailingsDamMonitor {
    dam_id: u32,
    status: TailingsDamStatus,
    water_level_mm: u32,
    freeboard_mm: u32,
    piezometer_readings_kpa: Vec<u32>,
    inclinometer_readings_micro_rad: Vec<i32>,
    seepage_flow_lpm_x100: u32,
    pond_ph_x100: u16,
    embankment_crest_elevation_mm: i64,
    last_inspection_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrillHoleLog {
    hole_id: u32,
    collar_easting_mm: i64,
    collar_northing_mm: i64,
    collar_elevation_mm: i64,
    azimuth_deg_x100: u32,
    dip_deg_x100: i32,
    total_depth_mm: u32,
    intervals: Vec<GeologicalInterval>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeologicalInterval {
    from_mm: u32,
    to_mm: u32,
    lithology: String,
    ore_type: Option<OreType>,
    grade_ppm: u32,
    rqd_percent: u8,
    core_recovery_percent: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VentilationReading {
    station_id: u32,
    mode: VentilationMode,
    airflow_m3_per_s_x100: u32,
    air_temp_c_x10: i16,
    wet_bulb_temp_c_x10: i16,
    barometric_pressure_pa: u32,
    co_ppm_x10: u16,
    no2_ppm_x10: u16,
    methane_percent_x1000: u16,
    dust_mg_m3_x100: u16,
    fan_rpm: u16,
    fan_power_kw: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceSchedule {
    equipment_id: u32,
    equipment_name: String,
    priority: MaintenancePriority,
    scheduled_epoch: u64,
    estimated_duration_min: u32,
    parts_required: Vec<String>,
    last_service_epoch: u64,
    operating_hours_since_service: u32,
    assigned_technician_id: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StockpileInventory {
    stockpile_id: u32,
    ore_type: OreType,
    tonnage_kg: u64,
    average_grade_ppm: u32,
    moisture_percent_x100: u16,
    survey_volume_m3_x100: u64,
    last_survey_epoch: u64,
    reclaim_rate_tph_x10: u32,
    stacker_active: bool,
    reclaimer_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnvironmentalCompliance {
    monitoring_station_id: u32,
    location_name: String,
    measurements: Vec<EnvironmentalParam>,
    wind_speed_kmh_x10: u16,
    wind_direction_deg: u16,
    temperature_c_x10: i16,
    humidity_percent_x10: u16,
    timestamp_epoch: u64,
    compliant: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GrindingMillStatus {
    mill_id: u32,
    mill_type: String,
    power_draw_kw: u32,
    speed_percent_x10: u16,
    feed_rate_tph_x10: u32,
    discharge_density_percent_x10: u16,
    ball_charge_percent_x10: u16,
    product_p80_microns: u32,
    bearing_temp_drive_c_x10: i16,
    bearing_temp_non_drive_c_x10: i16,
    motor_current_amps_x10: u32,
    cyclone_overflow_density_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DewarerUnit {
    unit_id: u32,
    unit_type: String,
    feed_solids_percent_x10: u16,
    underflow_solids_percent_x10: u16,
    overflow_clarity_ntu_x10: u16,
    flocculant_dose_gpt_x100: u32,
    rake_torque_percent_x10: u16,
    bed_depth_mm: u32,
    throughput_m3_h_x10: u32,
}

// ---------------------------------------------------------------------------
// Test 1: Ore grade assay roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_ore_grade_assay_memory_roundtrip() {
    let assay = OreGradeAssay {
        sample_id: 1_000_001,
        drill_hole_id: 4523,
        depth_from_mm: 45_000,
        depth_to_mm: 46_500,
        ore_type: OreType::Gold,
        grade_ppm: 3_200,
        moisture_percent_x100: 845,
        specific_gravity_x1000: 2_710,
        timestamp_epoch: 1_710_000_000,
    };
    let bytes = encode_to_vec(&assay).expect("encode ore grade assay to vec");
    let (decoded, _): (OreGradeAssay, _) =
        decode_from_slice(&bytes).expect("decode ore grade assay from slice");
    assert_eq!(assay, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Blast design roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_blast_design_file_roundtrip() {
    let design = BlastDesign {
        blast_id: 20240315_001,
        bench_level: -145,
        pattern: BlastHolePattern::Staggered {
            burden_mm: 4500,
            spacing_mm: 5200,
        },
        hole_count: 87,
        hole_diameter_mm: 311,
        hole_depth_mm: 15_000,
        charge_mass_grams: 285_000,
        stemming_length_mm: 4_000,
        delay_interval_ms: 25,
        expected_tonnage_kg: 125_000_000,
    };
    let path = temp_dir().join("oxicode_test_mining_blast_design.bin");
    encode_to_file(&design, &path).expect("encode blast design to file");
    let decoded: BlastDesign = decode_from_file(&path).expect("decode blast design from file");
    assert_eq!(design, decoded);
    std::fs::remove_file(&path).expect("cleanup blast design temp file");
}

// ---------------------------------------------------------------------------
// Test 3: Haul truck telemetry roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_haul_truck_telemetry_memory_roundtrip() {
    let telemetry = HaulTruckTelemetry {
        truck_id: 301,
        fleet_id: 2,
        gps_lat_micro: -31_950_000,
        gps_lon_micro: 115_860_000,
        elevation_mm: -245_000,
        speed_kmh_x10: 325,
        payload_kg: 220_000,
        fuel_level_percent_x10: 672,
        engine_rpm: 1850,
        oil_pressure_kpa: 450,
        coolant_temp_c_x10: 920,
        tire_pressure_front_kpa: 690,
        tire_pressure_rear_kpa: 710,
        odometer_m: 1_450_000,
        engine_hours_x10: 185_000,
        is_loaded: true,
        timestamp_epoch: 1_710_000_100,
    };
    let bytes = encode_to_vec(&telemetry).expect("encode truck telemetry to vec");
    let (decoded, _): (HaulTruckTelemetry, _) =
        decode_from_slice(&bytes).expect("decode truck telemetry from slice");
    assert_eq!(telemetry, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Crusher throughput roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_crusher_throughput_file_roundtrip() {
    let crusher = CrusherThroughput {
        crusher_id: 12,
        crusher_type: CrusherType::Gyratory,
        feed_rate_tph_x10: 42_000,
        power_draw_kw: 2_800,
        css_mm_x10: 1250,
        oss_mm_x10: 2100,
        product_p80_microns: 150_000,
        liner_wear_percent_x10: 340,
        bearing_temp_c_x10: 625,
        oil_flow_lpm_x10: 1800,
        runtime_seconds: 3_600_000,
        downtime_seconds: 86_400,
    };
    let path = temp_dir().join("oxicode_test_mining_crusher_throughput.bin");
    encode_to_file(&crusher, &path).expect("encode crusher throughput to file");
    let decoded: CrusherThroughput =
        decode_from_file(&path).expect("decode crusher throughput from file");
    assert_eq!(crusher, decoded);
    std::fs::remove_file(&path).expect("cleanup crusher throughput temp file");
}

// ---------------------------------------------------------------------------
// Test 5: Flotation cell parameters roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_flotation_cell_memory_roundtrip() {
    let cell = FlotationCell {
        cell_id: 7,
        bank_id: 2,
        cell_volume_liters: 160_000,
        air_flow_lpm_x10: 45_000,
        froth_depth_mm: 350,
        pulp_level_percent_x10: 752,
        reagent: FlotationReagent::Xanthate,
        reagent_dose_gpt_x100: 2500,
        feed_grade_ppm: 4_200,
        concentrate_grade_ppm: 120_000,
        recovery_percent_x100: 9250,
        ph_x100: 1050,
        pulp_density_percent_x10: 350,
    };
    let bytes = encode_to_vec(&cell).expect("encode flotation cell to vec");
    let (decoded, _): (FlotationCell, _) =
        decode_from_slice(&bytes).expect("decode flotation cell from slice");
    assert_eq!(cell, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Tailings dam monitoring roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_tailings_dam_monitor_file_roundtrip() {
    let monitor = TailingsDamMonitor {
        dam_id: 3,
        status: TailingsDamStatus::Operational,
        water_level_mm: 12_500,
        freeboard_mm: 3_500,
        piezometer_readings_kpa: vec![145, 152, 148, 155, 160, 142],
        inclinometer_readings_micro_rad: vec![-120, 85, -45, 200, -180, 95],
        seepage_flow_lpm_x100: 2300,
        pond_ph_x100: 780,
        embankment_crest_elevation_mm: 1_234_567_890,
        last_inspection_epoch: 1_709_900_000,
    };
    let path = temp_dir().join("oxicode_test_mining_tailings_dam.bin");
    encode_to_file(&monitor, &path).expect("encode tailings dam to file");
    let decoded: TailingsDamMonitor =
        decode_from_file(&path).expect("decode tailings dam from file");
    assert_eq!(monitor, decoded);
    std::fs::remove_file(&path).expect("cleanup tailings dam temp file");
}

// ---------------------------------------------------------------------------
// Test 7: Drill hole geological log roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_drill_hole_log_memory_roundtrip() {
    let log = DrillHoleLog {
        hole_id: 8842,
        collar_easting_mm: 456_789_000,
        collar_northing_mm: 6_543_210_000,
        collar_elevation_mm: 325_000,
        azimuth_deg_x100: 27_500,
        dip_deg_x100: -6_000,
        total_depth_mm: 200_000,
        intervals: vec![
            GeologicalInterval {
                from_mm: 0,
                to_mm: 15_000,
                lithology: "Laterite overburden".to_string(),
                ore_type: None,
                grade_ppm: 0,
                rqd_percent: 0,
                core_recovery_percent: 45,
            },
            GeologicalInterval {
                from_mm: 15_000,
                to_mm: 85_000,
                lithology: "Banded iron formation".to_string(),
                ore_type: Some(OreType::Iron),
                grade_ppm: 620_000,
                rqd_percent: 82,
                core_recovery_percent: 95,
            },
            GeologicalInterval {
                from_mm: 85_000,
                to_mm: 200_000,
                lithology: "Dolerite intrusion".to_string(),
                ore_type: None,
                grade_ppm: 0,
                rqd_percent: 95,
                core_recovery_percent: 99,
            },
        ],
    };
    let bytes = encode_to_vec(&log).expect("encode drill hole log to vec");
    let (decoded, _): (DrillHoleLog, _) =
        decode_from_slice(&bytes).expect("decode drill hole log from slice");
    assert_eq!(log, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Mine ventilation readings roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_ventilation_reading_file_roundtrip() {
    let reading = VentilationReading {
        station_id: 42,
        mode: VentilationMode::BalancedPressure,
        airflow_m3_per_s_x100: 15_000,
        air_temp_c_x10: 285,
        wet_bulb_temp_c_x10: 240,
        barometric_pressure_pa: 101_325,
        co_ppm_x10: 35,
        no2_ppm_x10: 12,
        methane_percent_x1000: 250,
        dust_mg_m3_x100: 180,
        fan_rpm: 890,
        fan_power_kw: 450,
    };
    let path = temp_dir().join("oxicode_test_mining_ventilation.bin");
    encode_to_file(&reading, &path).expect("encode ventilation reading to file");
    let decoded: VentilationReading =
        decode_from_file(&path).expect("decode ventilation reading from file");
    assert_eq!(reading, decoded);
    std::fs::remove_file(&path).expect("cleanup ventilation temp file");
}

// ---------------------------------------------------------------------------
// Test 9: Equipment maintenance schedule roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_maintenance_schedule_memory_roundtrip() {
    let schedule = MaintenanceSchedule {
        equipment_id: 301,
        equipment_name: "CAT 797F Haul Truck".to_string(),
        priority: MaintenancePriority::Preventive,
        scheduled_epoch: 1_710_100_000,
        estimated_duration_min: 480,
        parts_required: vec![
            "Engine oil filter (2x)".to_string(),
            "Hydraulic hose assembly".to_string(),
            "Brake pads front axle".to_string(),
            "Transmission fluid 200L".to_string(),
        ],
        last_service_epoch: 1_708_500_000,
        operating_hours_since_service: 5000,
        assigned_technician_id: 1042,
    };
    let bytes = encode_to_vec(&schedule).expect("encode maintenance schedule to vec");
    let (decoded, _): (MaintenanceSchedule, _) =
        decode_from_slice(&bytes).expect("decode maintenance schedule from slice");
    assert_eq!(schedule, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Stockpile inventory tracking roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_stockpile_inventory_file_roundtrip() {
    let stockpile = StockpileInventory {
        stockpile_id: 5,
        ore_type: OreType::Copper,
        tonnage_kg: 2_500_000_000,
        average_grade_ppm: 8_500,
        moisture_percent_x100: 1120,
        survey_volume_m3_x100: 96_000_000,
        last_survey_epoch: 1_709_800_000,
        reclaim_rate_tph_x10: 35_000,
        stacker_active: false,
        reclaimer_active: true,
    };
    let path = temp_dir().join("oxicode_test_mining_stockpile.bin");
    encode_to_file(&stockpile, &path).expect("encode stockpile to file");
    let decoded: StockpileInventory = decode_from_file(&path).expect("decode stockpile from file");
    assert_eq!(stockpile, decoded);
    std::fs::remove_file(&path).expect("cleanup stockpile temp file");
}

// ---------------------------------------------------------------------------
// Test 11: Environmental compliance roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_environmental_compliance_memory_roundtrip() {
    let compliance = EnvironmentalCompliance {
        monitoring_station_id: 14,
        location_name: "North pit boundary fence line".to_string(),
        measurements: vec![
            EnvironmentalParam::Dust {
                pm10_ug_m3: 45,
                pm25_ug_m3: 18,
            },
            EnvironmentalParam::Noise { db_level_x10: 725 },
            EnvironmentalParam::WaterPh { ph_x100: 720 },
            EnvironmentalParam::HeavyMetal {
                element: "Arsenic".to_string(),
                ppb: 8,
            },
            EnvironmentalParam::GroundVibration { ppv_mm_s_x100: 350 },
        ],
        wind_speed_kmh_x10: 185,
        wind_direction_deg: 225,
        temperature_c_x10: 342,
        humidity_percent_x10: 450,
        timestamp_epoch: 1_710_050_000,
        compliant: true,
    };
    let bytes = encode_to_vec(&compliance).expect("encode environmental compliance to vec");
    let (decoded, _): (EnvironmentalCompliance, _) =
        decode_from_slice(&bytes).expect("decode environmental compliance from slice");
    assert_eq!(compliance, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Grinding mill status roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_grinding_mill_file_roundtrip() {
    let mill = GrindingMillStatus {
        mill_id: 2,
        mill_type: "SAG Mill 36ft".to_string(),
        power_draw_kw: 16_500,
        speed_percent_x10: 745,
        feed_rate_tph_x10: 22_000,
        discharge_density_percent_x10: 720,
        ball_charge_percent_x10: 280,
        product_p80_microns: 2_100,
        bearing_temp_drive_c_x10: 585,
        bearing_temp_non_drive_c_x10: 520,
        motor_current_amps_x10: 42_000,
        cyclone_overflow_density_x10: 380,
    };
    let path = temp_dir().join("oxicode_test_mining_grinding_mill.bin");
    encode_to_file(&mill, &path).expect("encode grinding mill to file");
    let decoded: GrindingMillStatus =
        decode_from_file(&path).expect("decode grinding mill from file");
    assert_eq!(mill, decoded);
    std::fs::remove_file(&path).expect("cleanup grinding mill temp file");
}

// ---------------------------------------------------------------------------
// Test 13: Dewaterer unit roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_dewaterer_unit_memory_roundtrip() {
    let unit = DewarerUnit {
        unit_id: 4,
        unit_type: "High-rate thickener 30m".to_string(),
        feed_solids_percent_x10: 250,
        underflow_solids_percent_x10: 650,
        overflow_clarity_ntu_x10: 120,
        flocculant_dose_gpt_x100: 3500,
        rake_torque_percent_x10: 420,
        bed_depth_mm: 2_800,
        throughput_m3_h_x10: 8_500,
    };
    let bytes = encode_to_vec(&unit).expect("encode dewaterer unit to vec");
    let (decoded, _): (DewarerUnit, _) =
        decode_from_slice(&bytes).expect("decode dewaterer unit from slice");
    assert_eq!(unit, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Multiple assay samples in a vec roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_assay_samples_file_roundtrip() {
    let samples: Vec<OreGradeAssay> = (0..50)
        .map(|i| OreGradeAssay {
            sample_id: 500_000 + i,
            drill_hole_id: 1000 + (i as u32 % 10),
            depth_from_mm: (i as u32) * 1_500,
            depth_to_mm: (i as u32) * 1_500 + 1_500,
            ore_type: match i % 5 {
                0 => OreType::Gold,
                1 => OreType::Copper,
                2 => OreType::Iron,
                3 => OreType::Lithium,
                _ => OreType::Nickel,
            },
            grade_ppm: 100 + (i as u32 * 37) % 5000,
            moisture_percent_x100: 300 + (i as u16 * 13) % 800,
            specific_gravity_x1000: 2500 + (i as u16 * 7) % 500,
            timestamp_epoch: 1_710_000_000 + i * 60,
        })
        .collect();
    let path = temp_dir().join("oxicode_test_mining_multi_assay.bin");
    encode_to_file(&samples, &path).expect("encode multiple assay samples to file");
    let decoded: Vec<OreGradeAssay> =
        decode_from_file(&path).expect("decode multiple assay samples from file");
    assert_eq!(samples, decoded);
    std::fs::remove_file(&path).expect("cleanup multi assay temp file");
}

// ---------------------------------------------------------------------------
// Test 15: Haul truck fleet batch telemetry via memory
// ---------------------------------------------------------------------------

#[test]
fn test_haul_truck_fleet_memory_roundtrip() {
    let fleet: Vec<HaulTruckTelemetry> = (0..20)
        .map(|i| HaulTruckTelemetry {
            truck_id: 300 + i,
            fleet_id: (i as u8) / 5 + 1,
            gps_lat_micro: -31_950_000 + (i as i64 * 100),
            gps_lon_micro: 115_860_000 + (i as i64 * 150),
            elevation_mm: -245_000 + (i as i32 * 500),
            speed_kmh_x10: if i % 3 == 0 { 0 } else { 250 + (i as u16 * 10) },
            payload_kg: if i % 2 == 0 { 220_000 } else { 0 },
            fuel_level_percent_x10: 300 + (i as u16 * 30),
            engine_rpm: if i % 3 == 0 { 700 } else { 1800 },
            oil_pressure_kpa: 400 + (i as u16 * 5),
            coolant_temp_c_x10: 850 + (i as i16 * 3),
            tire_pressure_front_kpa: 680 + (i as u16 * 2),
            tire_pressure_rear_kpa: 700 + (i as u16 * 2),
            odometer_m: 1_000_000 + (i as u64 * 50_000),
            engine_hours_x10: 150_000 + (i as u32 * 1000),
            is_loaded: i % 2 == 0,
            timestamp_epoch: 1_710_000_000 + (i as u64 * 5),
        })
        .collect();
    let bytes = encode_to_vec(&fleet).expect("encode truck fleet to vec");
    let (decoded, _): (Vec<HaulTruckTelemetry>, _) =
        decode_from_slice(&bytes).expect("decode truck fleet from slice");
    assert_eq!(fleet, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Blast pattern enum variants roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_blast_pattern_variants_file_roundtrip() {
    let patterns: Vec<BlastHolePattern> = vec![
        BlastHolePattern::Square { spacing_mm: 5000 },
        BlastHolePattern::Staggered {
            burden_mm: 4200,
            spacing_mm: 4800,
        },
        BlastHolePattern::Echelon {
            angle_deg_x10: 450,
            spacing_mm: 5500,
        },
        BlastHolePattern::FanDrill {
            arc_deg_x10: 1200,
            hole_count: 12,
        },
    ];
    let path = temp_dir().join("oxicode_test_mining_blast_patterns.bin");
    encode_to_file(&patterns, &path).expect("encode blast patterns to file");
    let decoded: Vec<BlastHolePattern> =
        decode_from_file(&path).expect("decode blast patterns from file");
    assert_eq!(patterns, decoded);
    std::fs::remove_file(&path).expect("cleanup blast patterns temp file");
}

// ---------------------------------------------------------------------------
// Test 17: Flotation circuit multi-cell bank roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_flotation_circuit_bank_memory_roundtrip() {
    let reagents = [
        FlotationReagent::Xanthate,
        FlotationReagent::Dithiophosphate,
        FlotationReagent::MethylIsobutylCarbinol,
        FlotationReagent::PineOil,
        FlotationReagent::SodiumCyanide,
        FlotationReagent::LimeSlurry,
        FlotationReagent::SulfuricAcid,
    ];
    let bank: Vec<FlotationCell> = (0..7)
        .map(|i| FlotationCell {
            cell_id: 100 + i,
            bank_id: 3,
            cell_volume_liters: 160_000,
            air_flow_lpm_x10: 40_000 + (i as u32 * 1000),
            froth_depth_mm: 300 + (i as u16 * 10),
            pulp_level_percent_x10: 720 + (i as u16 * 5),
            reagent: reagents[i as usize].clone(),
            reagent_dose_gpt_x100: 2000 + (i as u32 * 200),
            feed_grade_ppm: 5_000 - (i as u32 * 400),
            concentrate_grade_ppm: 100_000 + (i as u32 * 5000),
            recovery_percent_x100: 9500 - (i as u16 * 100),
            ph_x100: 1020 + (i as u16 * 10),
            pulp_density_percent_x10: 340 + (i as u16 * 5),
        })
        .collect();
    let bytes = encode_to_vec(&bank).expect("encode flotation bank to vec");
    let (decoded, _): (Vec<FlotationCell>, _) =
        decode_from_slice(&bytes).expect("decode flotation bank from slice");
    assert_eq!(bank, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Complex nested drill campaign via file
// ---------------------------------------------------------------------------

#[test]
fn test_drill_campaign_file_roundtrip() {
    let campaign: Vec<DrillHoleLog> = vec![
        DrillHoleLog {
            hole_id: 9001,
            collar_easting_mm: 500_000_000,
            collar_northing_mm: 7_200_000_000,
            collar_elevation_mm: 450_000,
            azimuth_deg_x100: 0,
            dip_deg_x100: -9_000,
            total_depth_mm: 300_000,
            intervals: vec![
                GeologicalInterval {
                    from_mm: 0,
                    to_mm: 20_000,
                    lithology: "Transported cover".to_string(),
                    ore_type: None,
                    grade_ppm: 0,
                    rqd_percent: 0,
                    core_recovery_percent: 30,
                },
                GeologicalInterval {
                    from_mm: 20_000,
                    to_mm: 150_000,
                    lithology: "Massive sulfide".to_string(),
                    ore_type: Some(OreType::Copper),
                    grade_ppm: 15_000,
                    rqd_percent: 75,
                    core_recovery_percent: 92,
                },
                GeologicalInterval {
                    from_mm: 150_000,
                    to_mm: 300_000,
                    lithology: "Footwall granite".to_string(),
                    ore_type: None,
                    grade_ppm: 50,
                    rqd_percent: 90,
                    core_recovery_percent: 98,
                },
            ],
        },
        DrillHoleLog {
            hole_id: 9002,
            collar_easting_mm: 500_050_000,
            collar_northing_mm: 7_200_025_000,
            collar_elevation_mm: 448_500,
            azimuth_deg_x100: 18_000,
            dip_deg_x100: -7_000,
            total_depth_mm: 250_000,
            intervals: vec![
                GeologicalInterval {
                    from_mm: 0,
                    to_mm: 8_000,
                    lithology: "Colluvium".to_string(),
                    ore_type: None,
                    grade_ppm: 0,
                    rqd_percent: 0,
                    core_recovery_percent: 25,
                },
                GeologicalInterval {
                    from_mm: 8_000,
                    to_mm: 180_000,
                    lithology: "Disseminated porphyry".to_string(),
                    ore_type: Some(OreType::Copper),
                    grade_ppm: 6_800,
                    rqd_percent: 60,
                    core_recovery_percent: 88,
                },
                GeologicalInterval {
                    from_mm: 180_000,
                    to_mm: 220_000,
                    lithology: "Quartz vein stockwork".to_string(),
                    ore_type: Some(OreType::Gold),
                    grade_ppm: 2_400,
                    rqd_percent: 45,
                    core_recovery_percent: 80,
                },
                GeologicalInterval {
                    from_mm: 220_000,
                    to_mm: 250_000,
                    lithology: "Basalt flow".to_string(),
                    ore_type: None,
                    grade_ppm: 0,
                    rqd_percent: 88,
                    core_recovery_percent: 97,
                },
            ],
        },
    ];
    let path = temp_dir().join("oxicode_test_mining_drill_campaign.bin");
    encode_to_file(&campaign, &path).expect("encode drill campaign to file");
    let decoded: Vec<DrillHoleLog> =
        decode_from_file(&path).expect("decode drill campaign from file");
    assert_eq!(campaign, decoded);
    std::fs::remove_file(&path).expect("cleanup drill campaign temp file");
}

// ---------------------------------------------------------------------------
// Test 19: Ventilation network survey roundtrip via memory
// ---------------------------------------------------------------------------

#[test]
fn test_ventilation_network_memory_roundtrip() {
    let modes = [
        VentilationMode::NaturalDraft,
        VentilationMode::ForcedInlet,
        VentilationMode::ExhaustOnly,
        VentilationMode::BalancedPressure,
        VentilationMode::BoosterAssisted,
    ];
    let survey: Vec<VentilationReading> = (0..15)
        .map(|i| VentilationReading {
            station_id: 100 + i,
            mode: modes[(i as usize) % modes.len()].clone(),
            airflow_m3_per_s_x100: 5_000 + (i as u32 * 800),
            air_temp_c_x10: 220 + (i as i16 * 8),
            wet_bulb_temp_c_x10: 180 + (i as i16 * 6),
            barometric_pressure_pa: 99_000 + (i as u32 * 150),
            co_ppm_x10: 10 + (i as u16 * 3),
            no2_ppm_x10: 5 + (i as u16 * 2),
            methane_percent_x1000: 50 + (i as u16 * 20),
            dust_mg_m3_x100: 80 + (i as u16 * 15),
            fan_rpm: 600 + (i as u16 * 30),
            fan_power_kw: 200 + (i as u16 * 25),
        })
        .collect();
    let bytes = encode_to_vec(&survey).expect("encode ventilation survey to vec");
    let (decoded, _): (Vec<VentilationReading>, _) =
        decode_from_slice(&bytes).expect("decode ventilation survey from slice");
    assert_eq!(survey, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Maintenance backlog batch roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_maintenance_backlog_file_roundtrip() {
    let priorities = [
        MaintenancePriority::Routine,
        MaintenancePriority::Preventive,
        MaintenancePriority::Corrective,
        MaintenancePriority::Emergency,
        MaintenancePriority::Shutdown,
    ];
    let backlog: Vec<MaintenanceSchedule> = vec![
        MaintenanceSchedule {
            equipment_id: 101,
            equipment_name: "Primary jaw crusher".to_string(),
            priority: priorities[3].clone(),
            scheduled_epoch: 1_710_010_000,
            estimated_duration_min: 720,
            parts_required: vec![
                "Toggle plate assembly".to_string(),
                "Jaw die set".to_string(),
            ],
            last_service_epoch: 1_707_000_000,
            operating_hours_since_service: 8500,
            assigned_technician_id: 2001,
        },
        MaintenanceSchedule {
            equipment_id: 205,
            equipment_name: "Ball mill #3 gearbox".to_string(),
            priority: priorities[1].clone(),
            scheduled_epoch: 1_710_200_000,
            estimated_duration_min: 1440,
            parts_required: vec![
                "Pinion gear".to_string(),
                "Ring gear inspection".to_string(),
                "Lubricant 500L drum".to_string(),
            ],
            last_service_epoch: 1_705_000_000,
            operating_hours_since_service: 12000,
            assigned_technician_id: 2005,
        },
        MaintenanceSchedule {
            equipment_id: 410,
            equipment_name: "Conveyor belt CV-04".to_string(),
            priority: priorities[0].clone(),
            scheduled_epoch: 1_710_300_000,
            estimated_duration_min: 240,
            parts_required: vec![
                "Idler rollers (12x)".to_string(),
                "Belt splice kit".to_string(),
            ],
            last_service_epoch: 1_709_000_000,
            operating_hours_since_service: 3000,
            assigned_technician_id: 2003,
        },
    ];
    let path = temp_dir().join("oxicode_test_mining_maintenance_backlog.bin");
    encode_to_file(&backlog, &path).expect("encode maintenance backlog to file");
    let decoded: Vec<MaintenanceSchedule> =
        decode_from_file(&path).expect("decode maintenance backlog from file");
    assert_eq!(backlog, decoded);
    std::fs::remove_file(&path).expect("cleanup maintenance backlog temp file");
}

// ---------------------------------------------------------------------------
// Test 21: Mixed ore types and tailings dam status enum coverage via memory
// ---------------------------------------------------------------------------

#[test]
fn test_ore_type_and_dam_status_enum_coverage_memory() {
    let ore_types = vec![
        OreType::Iron,
        OreType::Copper,
        OreType::Gold,
        OreType::Platinum,
        OreType::Bauxite,
        OreType::Lithium,
        OreType::Nickel,
        OreType::Zinc,
        OreType::Uranium,
        OreType::RareEarth,
    ];
    let bytes = encode_to_vec(&ore_types).expect("encode all ore types to vec");
    let (decoded, _): (Vec<OreType>, _) =
        decode_from_slice(&bytes).expect("decode all ore types from slice");
    assert_eq!(ore_types, decoded);

    let dam_statuses = vec![
        TailingsDamStatus::Operational,
        TailingsDamStatus::UnderConstruction,
        TailingsDamStatus::RaisingInProgress,
        TailingsDamStatus::Decommissioned,
        TailingsDamStatus::AlertMonitoring,
        TailingsDamStatus::EmergencyDrawdown,
    ];
    let bytes2 = encode_to_vec(&dam_statuses).expect("encode all dam statuses to vec");
    let (decoded2, _): (Vec<TailingsDamStatus>, _) =
        decode_from_slice(&bytes2).expect("decode all dam statuses from slice");
    assert_eq!(dam_statuses, decoded2);
}

// ---------------------------------------------------------------------------
// Test 22: Full mine site snapshot combining multiple domains via file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MineSiteSnapshot {
    site_name: String,
    timestamp_epoch: u64,
    assays: Vec<OreGradeAssay>,
    truck_fleet: Vec<HaulTruckTelemetry>,
    crusher: CrusherThroughput,
    flotation_cells: Vec<FlotationCell>,
    tailings: TailingsDamMonitor,
    ventilation: Vec<VentilationReading>,
    stockpiles: Vec<StockpileInventory>,
    environmental: EnvironmentalCompliance,
}

#[test]
fn test_full_mine_site_snapshot_file_roundtrip() {
    let snapshot = MineSiteSnapshot {
        site_name: "Pilbara Iron Ore Complex - Site Alpha".to_string(),
        timestamp_epoch: 1_710_000_500,
        assays: vec![
            OreGradeAssay {
                sample_id: 900_001,
                drill_hole_id: 7700,
                depth_from_mm: 30_000,
                depth_to_mm: 31_500,
                ore_type: OreType::Iron,
                grade_ppm: 640_000,
                moisture_percent_x100: 520,
                specific_gravity_x1000: 3_800,
                timestamp_epoch: 1_709_999_000,
            },
            OreGradeAssay {
                sample_id: 900_002,
                drill_hole_id: 7701,
                depth_from_mm: 50_000,
                depth_to_mm: 51_500,
                ore_type: OreType::Iron,
                grade_ppm: 580_000,
                moisture_percent_x100: 480,
                specific_gravity_x1000: 3_650,
                timestamp_epoch: 1_709_999_100,
            },
        ],
        truck_fleet: vec![HaulTruckTelemetry {
            truck_id: 501,
            fleet_id: 1,
            gps_lat_micro: -22_300_000,
            gps_lon_micro: 118_500_000,
            elevation_mm: 680_000,
            speed_kmh_x10: 400,
            payload_kg: 350_000,
            fuel_level_percent_x10: 550,
            engine_rpm: 2100,
            oil_pressure_kpa: 500,
            coolant_temp_c_x10: 880,
            tire_pressure_front_kpa: 720,
            tire_pressure_rear_kpa: 740,
            odometer_m: 2_800_000,
            engine_hours_x10: 320_000,
            is_loaded: true,
            timestamp_epoch: 1_710_000_500,
        }],
        crusher: CrusherThroughput {
            crusher_id: 1,
            crusher_type: CrusherType::Gyratory,
            feed_rate_tph_x10: 80_000,
            power_draw_kw: 5_200,
            css_mm_x10: 1800,
            oss_mm_x10: 2800,
            product_p80_microns: 200_000,
            liner_wear_percent_x10: 450,
            bearing_temp_c_x10: 610,
            oil_flow_lpm_x10: 2200,
            runtime_seconds: 7_200_000,
            downtime_seconds: 172_800,
        },
        flotation_cells: vec![FlotationCell {
            cell_id: 1,
            bank_id: 1,
            cell_volume_liters: 300_000,
            air_flow_lpm_x10: 80_000,
            froth_depth_mm: 400,
            pulp_level_percent_x10: 780,
            reagent: FlotationReagent::LimeSlurry,
            reagent_dose_gpt_x100: 5000,
            feed_grade_ppm: 640_000,
            concentrate_grade_ppm: 680_000,
            recovery_percent_x100: 9800,
            ph_x100: 950,
            pulp_density_percent_x10: 420,
        }],
        tailings: TailingsDamMonitor {
            dam_id: 1,
            status: TailingsDamStatus::Operational,
            water_level_mm: 8_500,
            freeboard_mm: 5_000,
            piezometer_readings_kpa: vec![120, 125, 118, 130],
            inclinometer_readings_micro_rad: vec![-50, 30, -20, 45],
            seepage_flow_lpm_x100: 1500,
            pond_ph_x100: 810,
            embankment_crest_elevation_mm: 695_000_000,
            last_inspection_epoch: 1_709_500_000,
        },
        ventilation: vec![VentilationReading {
            station_id: 1,
            mode: VentilationMode::ForcedInlet,
            airflow_m3_per_s_x100: 25_000,
            air_temp_c_x10: 380,
            wet_bulb_temp_c_x10: 300,
            barometric_pressure_pa: 100_500,
            co_ppm_x10: 15,
            no2_ppm_x10: 8,
            methane_percent_x1000: 100,
            dust_mg_m3_x100: 250,
            fan_rpm: 720,
            fan_power_kw: 550,
        }],
        stockpiles: vec![
            StockpileInventory {
                stockpile_id: 1,
                ore_type: OreType::Iron,
                tonnage_kg: 8_000_000_000,
                average_grade_ppm: 620_000,
                moisture_percent_x100: 600,
                survey_volume_m3_x100: 250_000_000,
                last_survey_epoch: 1_709_900_000,
                reclaim_rate_tph_x10: 60_000,
                stacker_active: true,
                reclaimer_active: true,
            },
            StockpileInventory {
                stockpile_id: 2,
                ore_type: OreType::Iron,
                tonnage_kg: 3_200_000_000,
                average_grade_ppm: 580_000,
                moisture_percent_x100: 550,
                survey_volume_m3_x100: 100_000_000,
                last_survey_epoch: 1_709_850_000,
                reclaim_rate_tph_x10: 45_000,
                stacker_active: false,
                reclaimer_active: true,
            },
        ],
        environmental: EnvironmentalCompliance {
            monitoring_station_id: 1,
            location_name: "Main processing plant perimeter".to_string(),
            measurements: vec![
                EnvironmentalParam::Dust {
                    pm10_ug_m3: 55,
                    pm25_ug_m3: 22,
                },
                EnvironmentalParam::Noise { db_level_x10: 680 },
            ],
            wind_speed_kmh_x10: 250,
            wind_direction_deg: 180,
            temperature_c_x10: 410,
            humidity_percent_x10: 280,
            timestamp_epoch: 1_710_000_500,
            compliant: true,
        },
    };
    let path = temp_dir().join("oxicode_test_mining_full_snapshot.bin");
    encode_to_file(&snapshot, &path).expect("encode mine site snapshot to file");
    let decoded: MineSiteSnapshot =
        decode_from_file(&path).expect("decode mine site snapshot from file");
    assert_eq!(snapshot, decoded);
    std::fs::remove_file(&path).expect("cleanup mine site snapshot temp file");
}
