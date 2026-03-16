#![cfg(feature = "compression-lz4")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// ---------------------------------------------------------------------------
// Domain types: Aquaculture and Fish Farming
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PenMonitoring {
    pen_id: u32,
    timestamp_epoch: u64,
    dissolved_oxygen_mg_l: f64,
    temperature_celsius: f64,
    salinity_ppt: f64,
    depth_meters: f32,
    current_speed_cm_s: f32,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeedingSchedule {
    pen_id: u32,
    feed_type: String,
    pellet_size_mm: f32,
    daily_ration_kg: f64,
    feeding_times: Vec<u64>,
    fcr: f64,
    cumulative_feed_kg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BiomassEstimate {
    pen_id: u32,
    survey_date_epoch: u64,
    fish_count: u64,
    average_weight_g: f64,
    std_dev_weight_g: f64,
    total_biomass_kg: f64,
    density_kg_per_m3: f64,
    method: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeaLiceRecord {
    pen_id: u32,
    sample_date_epoch: u64,
    adult_female_count: u32,
    mobile_count: u32,
    chalimus_count: u32,
    average_per_fish: f64,
    treatment_applied: Option<String>,
    treatment_date_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MortalityRecord {
    pen_id: u32,
    date_epoch: u64,
    dead_count: u32,
    moribund_count: u32,
    cause: String,
    cumulative_mortality_pct: f64,
    disposed_kg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterQuality {
    station_id: u32,
    timestamp_epoch: u64,
    ph: f64,
    ammonia_mg_l: f64,
    nitrite_mg_l: f64,
    nitrate_mg_l: f64,
    turbidity_ntu: f32,
    chlorophyll_ug_l: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestBatch {
    batch_id: String,
    pen_id: u32,
    harvest_date_epoch: u64,
    fish_count: u64,
    total_weight_kg: f64,
    average_weight_g: f64,
    grade_distribution: Vec<(String, u32)>,
    destination: String,
    welfare_score: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BroodstockGenetics {
    fish_tag: String,
    family_id: u32,
    generation: u8,
    dam_tag: String,
    sire_tag: String,
    ebv_growth: f64,
    ebv_disease_resistance: f64,
    ebv_fillet_yield: f64,
    heterozygosity: f64,
    marker_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HatcheryIncubationLog {
    tray_id: u32,
    species: String,
    egg_batch_date_epoch: u64,
    egg_count: u64,
    temperature_celsius: f64,
    degree_days: f64,
    eyed_pct: f64,
    hatch_pct: f64,
    deformity_pct: f64,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetCleaningSchedule {
    pen_id: u32,
    last_cleaned_epoch: u64,
    next_due_epoch: u64,
    biofouling_score: u8,
    method: String,
    antifouling_type: Option<String>,
    mesh_size_mm: f32,
    damage_found: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CameraBehaviorAnalysis {
    pen_id: u32,
    session_epoch: u64,
    swimming_speed_bl_s: f64,
    school_cohesion_index: f64,
    surface_activity_pct: f64,
    feed_response_score: u8,
    abnormal_behavior_flags: Vec<String>,
    visibility_meters: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnvironmentalImpact {
    site_id: u32,
    survey_date_epoch: u64,
    benthic_index: f64,
    sediment_organic_pct: f64,
    sulfide_um: f64,
    species_richness: u32,
    copper_mg_kg: f64,
    zinc_mg_kg: f64,
    compliance_status: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherCurrentForecast {
    site_id: u32,
    issued_epoch: u64,
    valid_from_epoch: u64,
    valid_to_epoch: u64,
    wind_speed_m_s: f32,
    wind_direction_deg: u16,
    wave_height_m: f32,
    wave_period_s: f32,
    current_speed_cm_s: f32,
    current_direction_deg: u16,
    air_temp_celsius: f32,
    sea_temp_celsius: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiseaseOutbreak {
    pen_id: u32,
    detected_epoch: u64,
    pathogen: String,
    diagnostic_method: String,
    prevalence_pct: f64,
    clinical_signs: Vec<String>,
    treatment_protocol: String,
    quarantine_active: bool,
    affected_pens: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestQuota {
    license_id: String,
    species: String,
    year: u16,
    allocated_tonnes: f64,
    harvested_tonnes: f64,
    remaining_tonnes: f64,
    regulatory_body: String,
    region: String,
    quota_period_start_epoch: u64,
    quota_period_end_epoch: u64,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_pen_monitoring_roundtrip() {
    let val = PenMonitoring {
        pen_id: 7,
        timestamp_epoch: 1_710_000_000,
        dissolved_oxygen_mg_l: 8.4,
        temperature_celsius: 12.3,
        salinity_ppt: 33.5,
        depth_meters: 15.0,
        current_speed_cm_s: 6.2,
        notes: String::from("Slight algal bloom at surface"),
    };
    let enc = encode_to_vec(&val).expect("encode pen monitoring");
    let compressed = compress_lz4(&enc).expect("compress pen monitoring");
    let decompressed = decompress_lz4(&compressed).expect("decompress pen monitoring");
    let (decoded, _): (PenMonitoring, usize) =
        decode_from_slice(&decompressed).expect("decode pen monitoring");
    assert_eq!(val, decoded);
}

#[test]
fn test_feeding_schedule_roundtrip() {
    let val = FeedingSchedule {
        pen_id: 3,
        feed_type: String::from("Atlantic Salmon Grower 9mm"),
        pellet_size_mm: 9.0,
        daily_ration_kg: 450.0,
        feeding_times: vec![21600, 36000, 50400, 64800],
        fcr: 1.15,
        cumulative_feed_kg: 87500.0,
    };
    let enc = encode_to_vec(&val).expect("encode feeding schedule");
    let compressed = compress_lz4(&enc).expect("compress feeding schedule");
    let decompressed = decompress_lz4(&compressed).expect("decompress feeding schedule");
    let (decoded, _): (FeedingSchedule, usize) =
        decode_from_slice(&decompressed).expect("decode feeding schedule");
    assert_eq!(val, decoded);
}

#[test]
fn test_biomass_estimate_roundtrip() {
    let val = BiomassEstimate {
        pen_id: 12,
        survey_date_epoch: 1_709_500_000,
        fish_count: 85_000,
        average_weight_g: 4250.0,
        std_dev_weight_g: 380.0,
        total_biomass_kg: 361_250.0,
        density_kg_per_m3: 22.5,
        method: String::from("Frame-based stereo camera sampling"),
    };
    let enc = encode_to_vec(&val).expect("encode biomass estimate");
    let compressed = compress_lz4(&enc).expect("compress biomass estimate");
    let decompressed = decompress_lz4(&compressed).expect("decompress biomass estimate");
    let (decoded, _): (BiomassEstimate, usize) =
        decode_from_slice(&decompressed).expect("decode biomass estimate");
    assert_eq!(val, decoded);
}

#[test]
fn test_sea_lice_record_with_treatment() {
    let val = SeaLiceRecord {
        pen_id: 5,
        sample_date_epoch: 1_709_800_000,
        adult_female_count: 12,
        mobile_count: 28,
        chalimus_count: 45,
        average_per_fish: 0.85,
        treatment_applied: Some(String::from("Hydrogen peroxide bath 1500 ppm")),
        treatment_date_epoch: Some(1_709_850_000),
    };
    let enc = encode_to_vec(&val).expect("encode sea lice record");
    let compressed = compress_lz4(&enc).expect("compress sea lice record");
    let decompressed = decompress_lz4(&compressed).expect("decompress sea lice record");
    let (decoded, _): (SeaLiceRecord, usize) =
        decode_from_slice(&decompressed).expect("decode sea lice record");
    assert_eq!(val, decoded);
}

#[test]
fn test_sea_lice_record_no_treatment() {
    let val = SeaLiceRecord {
        pen_id: 9,
        sample_date_epoch: 1_709_700_000,
        adult_female_count: 2,
        mobile_count: 5,
        chalimus_count: 8,
        average_per_fish: 0.15,
        treatment_applied: None,
        treatment_date_epoch: None,
    };
    let enc = encode_to_vec(&val).expect("encode sea lice no treatment");
    let compressed = compress_lz4(&enc).expect("compress sea lice no treatment");
    let decompressed = decompress_lz4(&compressed).expect("decompress sea lice no treatment");
    let (decoded, _): (SeaLiceRecord, usize) =
        decode_from_slice(&decompressed).expect("decode sea lice no treatment");
    assert_eq!(val, decoded);
}

#[test]
fn test_mortality_tracking_roundtrip() {
    let val = MortalityRecord {
        pen_id: 2,
        date_epoch: 1_709_600_000,
        dead_count: 14,
        moribund_count: 3,
        cause: String::from("Pancreas disease (PD) suspected"),
        cumulative_mortality_pct: 2.8,
        disposed_kg: 56.0,
    };
    let enc = encode_to_vec(&val).expect("encode mortality record");
    let compressed = compress_lz4(&enc).expect("compress mortality record");
    let decompressed = decompress_lz4(&compressed).expect("decompress mortality record");
    let (decoded, _): (MortalityRecord, usize) =
        decode_from_slice(&decompressed).expect("decode mortality record");
    assert_eq!(val, decoded);
}

#[test]
fn test_water_quality_roundtrip() {
    let val = WaterQuality {
        station_id: 101,
        timestamp_epoch: 1_710_100_000,
        ph: 7.95,
        ammonia_mg_l: 0.012,
        nitrite_mg_l: 0.003,
        nitrate_mg_l: 1.45,
        turbidity_ntu: 2.1,
        chlorophyll_ug_l: 4.8,
    };
    let enc = encode_to_vec(&val).expect("encode water quality");
    let compressed = compress_lz4(&enc).expect("compress water quality");
    let decompressed = decompress_lz4(&compressed).expect("decompress water quality");
    let (decoded, _): (WaterQuality, usize) =
        decode_from_slice(&decompressed).expect("decode water quality");
    assert_eq!(val, decoded);
}

#[test]
fn test_harvest_batch_roundtrip() {
    let val = HarvestBatch {
        batch_id: String::from("HB-2026-0315-A"),
        pen_id: 4,
        harvest_date_epoch: 1_710_500_000,
        fish_count: 22_000,
        total_weight_kg: 99_000.0,
        average_weight_g: 4500.0,
        grade_distribution: vec![
            (String::from("Superior"), 14_000),
            (String::from("Ordinary"), 6_500),
            (String::from("Production"), 1_500),
        ],
        destination: String::from("Processing plant Kristiansund"),
        welfare_score: 4,
    };
    let enc = encode_to_vec(&val).expect("encode harvest batch");
    let compressed = compress_lz4(&enc).expect("compress harvest batch");
    let decompressed = decompress_lz4(&compressed).expect("decompress harvest batch");
    let (decoded, _): (HarvestBatch, usize) =
        decode_from_slice(&decompressed).expect("decode harvest batch");
    assert_eq!(val, decoded);
}

#[test]
fn test_broodstock_genetics_roundtrip() {
    let val = BroodstockGenetics {
        fish_tag: String::from("PIT-00482917"),
        family_id: 137,
        generation: 8,
        dam_tag: String::from("PIT-00381204"),
        sire_tag: String::from("PIT-00395611"),
        ebv_growth: 12.7,
        ebv_disease_resistance: 0.85,
        ebv_fillet_yield: 1.3,
        heterozygosity: 0.42,
        marker_count: 55_000,
    };
    let enc = encode_to_vec(&val).expect("encode broodstock genetics");
    let compressed = compress_lz4(&enc).expect("compress broodstock genetics");
    let decompressed = decompress_lz4(&compressed).expect("decompress broodstock genetics");
    let (decoded, _): (BroodstockGenetics, usize) =
        decode_from_slice(&decompressed).expect("decode broodstock genetics");
    assert_eq!(val, decoded);
}

#[test]
fn test_hatchery_incubation_log_roundtrip() {
    let val = HatcheryIncubationLog {
        tray_id: 42,
        species: String::from("Salmo salar"),
        egg_batch_date_epoch: 1_700_000_000,
        egg_count: 12_500,
        temperature_celsius: 6.5,
        degree_days: 420.0,
        eyed_pct: 92.3,
        hatch_pct: 88.7,
        deformity_pct: 0.8,
        notes: String::from("Normal development; yolk-sac absorption on track"),
    };
    let enc = encode_to_vec(&val).expect("encode hatchery log");
    let compressed = compress_lz4(&enc).expect("compress hatchery log");
    let decompressed = decompress_lz4(&compressed).expect("decompress hatchery log");
    let (decoded, _): (HatcheryIncubationLog, usize) =
        decode_from_slice(&decompressed).expect("decode hatchery log");
    assert_eq!(val, decoded);
}

#[test]
fn test_net_cleaning_schedule_roundtrip() {
    let val = NetCleaningSchedule {
        pen_id: 6,
        last_cleaned_epoch: 1_708_000_000,
        next_due_epoch: 1_710_600_000,
        biofouling_score: 3,
        method: String::from("In-situ high-pressure washer"),
        antifouling_type: Some(String::from("Copper alloy coating")),
        mesh_size_mm: 25.0,
        damage_found: false,
    };
    let enc = encode_to_vec(&val).expect("encode net cleaning");
    let compressed = compress_lz4(&enc).expect("compress net cleaning");
    let decompressed = decompress_lz4(&compressed).expect("decompress net cleaning");
    let (decoded, _): (NetCleaningSchedule, usize) =
        decode_from_slice(&decompressed).expect("decode net cleaning");
    assert_eq!(val, decoded);
}

#[test]
fn test_camera_behavior_analysis_roundtrip() {
    let val = CameraBehaviorAnalysis {
        pen_id: 11,
        session_epoch: 1_710_200_000,
        swimming_speed_bl_s: 1.2,
        school_cohesion_index: 0.78,
        surface_activity_pct: 5.3,
        feed_response_score: 8,
        abnormal_behavior_flags: vec![
            String::from("Flashing observed"),
            String::from("Crowding at surface"),
        ],
        visibility_meters: 6.5,
    };
    let enc = encode_to_vec(&val).expect("encode camera analysis");
    let compressed = compress_lz4(&enc).expect("compress camera analysis");
    let decompressed = decompress_lz4(&compressed).expect("decompress camera analysis");
    let (decoded, _): (CameraBehaviorAnalysis, usize) =
        decode_from_slice(&decompressed).expect("decode camera analysis");
    assert_eq!(val, decoded);
}

#[test]
fn test_environmental_impact_roundtrip() {
    let val = EnvironmentalImpact {
        site_id: 55,
        survey_date_epoch: 1_709_400_000,
        benthic_index: 3.2,
        sediment_organic_pct: 4.8,
        sulfide_um: 1500.0,
        species_richness: 23,
        copper_mg_kg: 34.0,
        zinc_mg_kg: 120.0,
        compliance_status: String::from("Acceptable - Group 2"),
    };
    let enc = encode_to_vec(&val).expect("encode environmental impact");
    let compressed = compress_lz4(&enc).expect("compress environmental impact");
    let decompressed = decompress_lz4(&compressed).expect("decompress environmental impact");
    let (decoded, _): (EnvironmentalImpact, usize) =
        decode_from_slice(&decompressed).expect("decode environmental impact");
    assert_eq!(val, decoded);
}

#[test]
fn test_weather_current_forecast_roundtrip() {
    let val = WeatherCurrentForecast {
        site_id: 30,
        issued_epoch: 1_710_300_000,
        valid_from_epoch: 1_710_300_000,
        valid_to_epoch: 1_710_386_400,
        wind_speed_m_s: 8.5,
        wind_direction_deg: 225,
        wave_height_m: 1.8,
        wave_period_s: 7.2,
        current_speed_cm_s: 12.0,
        current_direction_deg: 180,
        air_temp_celsius: 7.5,
        sea_temp_celsius: 9.2,
    };
    let enc = encode_to_vec(&val).expect("encode weather forecast");
    let compressed = compress_lz4(&enc).expect("compress weather forecast");
    let decompressed = decompress_lz4(&compressed).expect("decompress weather forecast");
    let (decoded, _): (WeatherCurrentForecast, usize) =
        decode_from_slice(&decompressed).expect("decode weather forecast");
    assert_eq!(val, decoded);
}

#[test]
fn test_disease_outbreak_roundtrip() {
    let val = DiseaseOutbreak {
        pen_id: 8,
        detected_epoch: 1_709_900_000,
        pathogen: String::from("Infectious Salmon Anaemia Virus (ISAV)"),
        diagnostic_method: String::from("RT-qPCR heart tissue"),
        prevalence_pct: 12.5,
        clinical_signs: vec![
            String::from("Pale gills"),
            String::from("Ascites"),
            String::from("Petechiae on visceral fat"),
            String::from("Dark liver"),
        ],
        treatment_protocol: String::from("Mandatory cull and fallowing per regulatory order"),
        quarantine_active: true,
        affected_pens: vec![8, 9, 10],
    };
    let enc = encode_to_vec(&val).expect("encode disease outbreak");
    let compressed = compress_lz4(&enc).expect("compress disease outbreak");
    let decompressed = decompress_lz4(&compressed).expect("decompress disease outbreak");
    let (decoded, _): (DiseaseOutbreak, usize) =
        decode_from_slice(&decompressed).expect("decode disease outbreak");
    assert_eq!(val, decoded);
}

#[test]
fn test_harvest_quota_roundtrip() {
    let val = HarvestQuota {
        license_id: String::from("NO-AQ-2026-1042"),
        species: String::from("Salmo salar"),
        year: 2026,
        allocated_tonnes: 5_200.0,
        harvested_tonnes: 3_180.0,
        remaining_tonnes: 2_020.0,
        regulatory_body: String::from("Norwegian Directorate of Fisheries"),
        region: String::from("Nordland - Production Area 8"),
        quota_period_start_epoch: 1_704_067_200,
        quota_period_end_epoch: 1_735_689_600,
    };
    let enc = encode_to_vec(&val).expect("encode harvest quota");
    let compressed = compress_lz4(&enc).expect("compress harvest quota");
    let decompressed = decompress_lz4(&compressed).expect("decompress harvest quota");
    let (decoded, _): (HarvestQuota, usize) =
        decode_from_slice(&decompressed).expect("decode harvest quota");
    assert_eq!(val, decoded);
}

#[test]
fn test_repeated_pen_monitoring_compression_ratio() {
    let readings: Vec<PenMonitoring> = (0..200)
        .map(|i| PenMonitoring {
            pen_id: 1,
            timestamp_epoch: 1_710_000_000 + i as u64 * 3600,
            dissolved_oxygen_mg_l: 8.0 + (i as f64 % 10.0) * 0.05,
            temperature_celsius: 11.5 + (i as f64 % 5.0) * 0.1,
            salinity_ppt: 33.0,
            depth_meters: 15.0,
            current_speed_cm_s: 5.0,
            notes: String::from("Routine hourly sensor reading"),
        })
        .collect();
    let enc = encode_to_vec(&readings).expect("encode repeated pen monitoring");
    let compressed = compress_lz4(&enc).expect("compress repeated pen monitoring");
    assert!(
        compressed.len() < enc.len(),
        "LZ4 should compress repetitive pen monitoring data: compressed {} vs original {}",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress repeated pen monitoring");
    let (decoded, _): (Vec<PenMonitoring>, usize) =
        decode_from_slice(&decompressed).expect("decode repeated pen monitoring");
    assert_eq!(readings, decoded);
}

#[test]
fn test_large_water_quality_series_compression() {
    let series: Vec<WaterQuality> = (0..500)
        .map(|i| WaterQuality {
            station_id: 101,
            timestamp_epoch: 1_710_000_000 + i as u64 * 600,
            ph: 7.9 + (i as f64 % 20.0) * 0.005,
            ammonia_mg_l: 0.01 + (i as f64 % 10.0) * 0.001,
            nitrite_mg_l: 0.002,
            nitrate_mg_l: 1.4,
            turbidity_ntu: 2.0,
            chlorophyll_ug_l: 3.5,
        })
        .collect();
    let enc = encode_to_vec(&series).expect("encode water quality series");
    let compressed = compress_lz4(&enc).expect("compress water quality series");
    assert!(
        compressed.len() < enc.len(),
        "LZ4 should compress repetitive water quality series: compressed {} vs original {}",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress water quality series");
    let (decoded, _): (Vec<WaterQuality>, usize) =
        decode_from_slice(&decompressed).expect("decode water quality series");
    assert_eq!(series, decoded);
}

#[test]
fn test_multiple_harvest_batches_compression() {
    let batches: Vec<HarvestBatch> = (0..50)
        .map(|i| HarvestBatch {
            batch_id: format!("HB-2026-{:04}-{}", 100 + i, (b'A' + (i % 26) as u8) as char),
            pen_id: (i % 12) as u32 + 1,
            harvest_date_epoch: 1_710_000_000 + i as u64 * 86400,
            fish_count: 18_000 + (i as u64 % 5) * 1000,
            total_weight_kg: 81_000.0 + (i as f64 % 5.0) * 4500.0,
            average_weight_g: 4500.0,
            grade_distribution: vec![
                (String::from("Superior"), 12_000 + (i % 3) * 500),
                (String::from("Ordinary"), 5_000),
                (String::from("Production"), 1_000),
            ],
            destination: String::from("Processing plant Bergen"),
            welfare_score: 3 + (i % 3) as u8,
        })
        .collect();
    let enc = encode_to_vec(&batches).expect("encode harvest batches");
    let compressed = compress_lz4(&enc).expect("compress harvest batches");
    assert!(
        compressed.len() < enc.len(),
        "LZ4 should compress repetitive harvest batches: compressed {} vs original {}",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress harvest batches");
    let (decoded, _): (Vec<HarvestBatch>, usize) =
        decode_from_slice(&decompressed).expect("decode harvest batches");
    assert_eq!(batches, decoded);
}

#[test]
fn test_mortality_timeseries_compression_ratio() {
    let records: Vec<MortalityRecord> = (0..365)
        .map(|day| MortalityRecord {
            pen_id: 4,
            date_epoch: 1_704_067_200 + day as u64 * 86400,
            dead_count: (day % 7) as u32,
            moribund_count: (day % 3) as u32,
            cause: String::from("Natural attrition"),
            cumulative_mortality_pct: day as f64 * 0.01,
            disposed_kg: (day % 7) as f64 * 4.2,
        })
        .collect();
    let enc = encode_to_vec(&records).expect("encode mortality timeseries");
    let compressed = compress_lz4(&enc).expect("compress mortality timeseries");
    let ratio = enc.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Expected compression ratio > 1.0, got {:.2}",
        ratio
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress mortality timeseries");
    let (decoded, _): (Vec<MortalityRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode mortality timeseries");
    assert_eq!(records, decoded);
}

#[test]
fn test_mixed_site_data_roundtrip() {
    let pen = PenMonitoring {
        pen_id: 1,
        timestamp_epoch: 1_710_000_000,
        dissolved_oxygen_mg_l: 9.1,
        temperature_celsius: 10.8,
        salinity_ppt: 34.0,
        depth_meters: 20.0,
        current_speed_cm_s: 4.5,
        notes: String::from("Clear conditions"),
    };
    let feed = FeedingSchedule {
        pen_id: 1,
        feed_type: String::from("Smolt Starter 3mm"),
        pellet_size_mm: 3.0,
        daily_ration_kg: 120.0,
        feeding_times: vec![28800, 43200, 57600],
        fcr: 0.95,
        cumulative_feed_kg: 15_600.0,
    };
    let lice = SeaLiceRecord {
        pen_id: 1,
        sample_date_epoch: 1_710_000_000,
        adult_female_count: 1,
        mobile_count: 3,
        chalimus_count: 5,
        average_per_fish: 0.09,
        treatment_applied: None,
        treatment_date_epoch: None,
    };

    let combined = (pen.clone(), feed.clone(), lice.clone());
    let enc = encode_to_vec(&combined).expect("encode mixed site data");
    let compressed = compress_lz4(&enc).expect("compress mixed site data");
    let decompressed = decompress_lz4(&compressed).expect("decompress mixed site data");
    let (decoded, _): ((PenMonitoring, FeedingSchedule, SeaLiceRecord), usize) =
        decode_from_slice(&decompressed).expect("decode mixed site data");
    assert_eq!(combined, decoded);
}

#[test]
fn test_disease_outbreak_with_many_clinical_signs() {
    let val = DiseaseOutbreak {
        pen_id: 15,
        detected_epoch: 1_710_400_000,
        pathogen: String::from("Piscirickettsia salmonis (SRS)"),
        diagnostic_method: String::from("Histopathology kidney and liver biopsies"),
        prevalence_pct: 28.0,
        clinical_signs: vec![
            String::from("Skin ulcers near lateral line"),
            String::from("Swollen kidney"),
            String::from("Granulomatous hepatitis"),
            String::from("Splenomegaly"),
            String::from("Melanisation of internal organs"),
            String::from("Lethargy and loss of appetite"),
            String::from("Exophthalmia unilateral"),
            String::from("Gill pallor"),
        ],
        treatment_protocol: String::from(
            "Florfenicol medicated feed 10 mg/kg BW/day for 10 days; \
             oxytetracycline bath 50 ppm backup protocol",
        ),
        quarantine_active: true,
        affected_pens: vec![14, 15, 16, 17],
    };
    let enc = encode_to_vec(&val).expect("encode complex disease outbreak");
    let compressed = compress_lz4(&enc).expect("compress complex disease outbreak");
    let decompressed = decompress_lz4(&compressed).expect("decompress complex disease outbreak");
    let (decoded, _): (DiseaseOutbreak, usize) =
        decode_from_slice(&decompressed).expect("decode complex disease outbreak");
    assert_eq!(val, decoded);

    // Verify compressed data differs from encoded data
    assert_ne!(
        enc, compressed,
        "compressed bytes should differ from uncompressed for disease outbreak"
    );
}
