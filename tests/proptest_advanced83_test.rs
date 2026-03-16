//! Advanced property-based tests (set 83) — Waste Management & Recycling Operations domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers waste stream classifications (MSW/C&D/hazardous/e-waste/organic),
//! collection route optimization, bin fill-level IoT sensors, MRF sorting lines,
//! contamination rate tracking, landfill cell monitoring, composting facility
//! parameters, recycling commodity pricing, transfer station throughput,
//! hazardous waste manifests, EPR metrics, circular economy indicators,
//! anaerobic digestion outputs, public education campaign results, and
//! fleet vehicle maintenance.

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
use proptest::prelude::*;

// ── Domain types ──────────────────────────────────────────────────────────────

/// Classification of a waste stream entering the system.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WasteStreamClass {
    /// Municipal solid waste.
    Msw { tonnage_kg: f32, residential: bool },
    /// Construction and demolition debris.
    ConstructionDemolition {
        tonnage_kg: f32,
        recyclable_fraction: f32,
    },
    /// Hazardous waste with UN class code.
    Hazardous {
        un_class: u8,
        quantity_kg: f32,
        manifest_id: u64,
    },
    /// Electronic waste.
    EWaste {
        device_count: u32,
        avg_weight_kg: f32,
        precious_metal_ppm: f32,
    },
    /// Organic / food waste.
    Organic {
        tonnage_kg: f32,
        moisture_pct: f32,
        methane_potential_m3: f32,
    },
}

/// IoT bin fill-level sensor reading.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BinFillSensor {
    /// Sensor device identifier.
    sensor_id: u64,
    /// Fill level as fraction (0.0 = empty, 1.0 = full).
    fill_fraction: f32,
    /// Internal temperature in °C.
    internal_temp_c: f32,
    /// Battery voltage in volts.
    battery_v: f32,
    /// GPS latitude.
    latitude: f64,
    /// GPS longitude.
    longitude: f64,
    /// Timestamp (Unix seconds).
    timestamp_s: u64,
}

/// Optimised collection route for waste trucks.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollectionRoute {
    /// Route identifier.
    route_id: u32,
    /// Total distance in kilometres.
    total_distance_km: f32,
    /// Estimated fuel consumption in litres.
    fuel_litres: f32,
    /// Number of stops.
    stop_count: u16,
    /// Estimated completion time in minutes.
    eta_minutes: u16,
    /// Truck licence plate tag.
    truck_tag: String,
}

/// MRF (Materials Recovery Facility) sorting line status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MrfSortingLine {
    /// Sorting line identifier.
    line_id: u32,
    /// Throughput in tonnes per hour.
    throughput_tph: f32,
    /// Contamination rate (0.0 – 1.0).
    contamination_rate: f32,
    /// Recovery rate (0.0 – 1.0).
    recovery_rate: f32,
    /// Currently operational.
    operational: bool,
    /// Material type being sorted.
    material_type: String,
}

/// Contamination tracking record for a recycling batch.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContaminationRecord {
    /// Batch identifier.
    batch_id: u64,
    /// Total weight of batch in kg.
    total_weight_kg: f32,
    /// Weight of contaminants in kg.
    contaminant_weight_kg: f32,
    /// Contamination type code.
    contaminant_type_code: u16,
    /// Rejected flag.
    rejected: bool,
}

/// Landfill cell monitoring data.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LandfillCellMonitor {
    /// Cell identifier.
    cell_id: u32,
    /// Compaction ratio (tonnes/m³).
    compaction_ratio: f32,
    /// Leachate volume in m³.
    leachate_volume_m3: f32,
    /// Landfill gas capture rate in m³/hr.
    gas_capture_m3_per_hr: f32,
    /// Methane concentration (0.0 – 1.0).
    methane_fraction: f32,
    /// Remaining capacity in m³.
    remaining_capacity_m3: f64,
}

/// Composting facility parameter snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CompostingParams {
    /// Facility identifier.
    facility_id: u32,
    /// Core temperature in °C.
    temperature_c: f32,
    /// Moisture content (0.0 – 1.0).
    moisture_fraction: f32,
    /// Carbon-to-nitrogen ratio.
    cn_ratio: f32,
    /// Oxygen level (0.0 – 0.21).
    oxygen_fraction: f32,
    /// Windrow age in days.
    age_days: u16,
}

/// Recycling commodity market price.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RecyclingCommodityPrice {
    /// Commodity code.
    commodity_code: String,
    /// Price per tonne in USD.
    price_usd_per_tonne: f32,
    /// Available quantity in tonnes.
    available_tonnes: f32,
    /// Grade quality (1 = highest, 5 = lowest).
    grade: u8,
    /// Price date (Unix seconds).
    price_date_s: u64,
}

/// Transfer station throughput record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransferStationThroughput {
    /// Station identifier.
    station_id: u32,
    /// Inbound tonnes for the period.
    inbound_tonnes: f32,
    /// Outbound tonnes for the period.
    outbound_tonnes: f32,
    /// Number of truck arrivals.
    truck_arrivals: u16,
    /// Average dwell time in minutes.
    avg_dwell_minutes: f32,
    /// Capacity utilisation (0.0 – 1.0).
    utilisation: f32,
}

/// Hazardous waste manifest entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HazardousManifest {
    /// Manifest tracking number.
    tracking_number: u64,
    /// Generator EPA identifier.
    generator_id: String,
    /// Transporter identifier.
    transporter_id: String,
    /// Designated facility identifier.
    facility_id: String,
    /// Waste quantity in kg.
    quantity_kg: f32,
    /// Proper DOT shipping name code.
    dot_code: u16,
    /// Emergency response guide number.
    erg_number: u16,
}

/// Extended Producer Responsibility (EPR) metric.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EprMetric {
    /// Producer identifier.
    producer_id: u32,
    /// Product category code.
    category_code: u16,
    /// Units placed on market.
    units_placed: u64,
    /// Units collected for recycling.
    units_collected: u64,
    /// Collection rate (0.0 – 1.0).
    collection_rate: f32,
    /// Financial contribution in USD.
    contribution_usd: f32,
}

/// Circular economy indicator.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CircularEconomyMetric {
    /// Jurisdiction code.
    jurisdiction_code: String,
    /// Material circularity index (0.0 – 1.0).
    circularity_index: f32,
    /// Recycled content fraction (0.0 – 1.0).
    recycled_content: f32,
    /// Waste-to-landfill diversion rate (0.0 – 1.0).
    diversion_rate: f32,
    /// Year of measurement.
    year: u16,
}

/// Anaerobic digestion facility output.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnaerobicDigestionOutput {
    /// Digester identifier.
    digester_id: u32,
    /// Biogas output in m³/day.
    biogas_m3_per_day: f32,
    /// Methane fraction in biogas (0.0 – 1.0).
    methane_fraction: f32,
    /// Digestate output in tonnes/day.
    digestate_tonnes_per_day: f32,
    /// Feedstock input in tonnes/day.
    feedstock_tonnes_per_day: f32,
    /// Hydraulic retention time in days.
    hrt_days: f32,
    /// pH value of digestate.
    digestate_ph: f32,
}

/// Public education campaign result.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EducationCampaignResult {
    /// Campaign identifier.
    campaign_id: u32,
    /// Target population size.
    target_population: u64,
    /// Participation count.
    participants: u64,
    /// Pre-campaign contamination rate (0.0 – 1.0).
    pre_contamination: f32,
    /// Post-campaign contamination rate (0.0 – 1.0).
    post_contamination: f32,
    /// Campaign cost in USD.
    cost_usd: f32,
}

/// Fleet vehicle maintenance record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FleetVehicleMaintenance {
    /// Vehicle identifier.
    vehicle_id: u32,
    /// Odometer reading in km.
    odometer_km: f64,
    /// Fuel consumed in litres since last service.
    fuel_consumed_litres: f32,
    /// Engine hours since last service.
    engine_hours: f32,
    /// Maintenance cost in USD.
    maintenance_cost_usd: f32,
    /// Next service due (Unix seconds).
    next_service_due_s: u64,
    /// Vehicle currently in service.
    in_service: bool,
}

// ── prop_compose! strategies ──────────────────────────────────────────────────

prop_compose! {
    fn arb_bin_fill_sensor()(
        sensor_id: u64,
        fill_fraction in 0.0f32..1.0f32,
        internal_temp_c in (-20.0f32)..80.0f32,
        battery_v in 2.0f32..4.5f32,
        latitude in (-90.0f64)..90.0f64,
        longitude in (-180.0f64)..180.0f64,
        timestamp_s: u64,
    ) -> BinFillSensor {
        BinFillSensor {
            sensor_id, fill_fraction, internal_temp_c, battery_v,
            latitude, longitude, timestamp_s,
        }
    }
}

prop_compose! {
    fn arb_composting_params()(
        facility_id: u32,
        temperature_c in 20.0f32..80.0f32,
        moisture_fraction in 0.3f32..0.7f32,
        cn_ratio in 15.0f32..40.0f32,
        oxygen_fraction in 0.05f32..0.21f32,
        age_days in 0u16..365u16,
    ) -> CompostingParams {
        CompostingParams {
            facility_id, temperature_c, moisture_fraction,
            cn_ratio, oxygen_fraction, age_days,
        }
    }
}

prop_compose! {
    fn arb_anaerobic_digestion()(
        digester_id: u32,
        biogas_m3_per_day in 100.0f32..10000.0f32,
        methane_fraction in 0.5f32..0.75f32,
        digestate_tonnes_per_day in 1.0f32..500.0f32,
        feedstock_tonnes_per_day in 1.0f32..500.0f32,
        hrt_days in 15.0f32..60.0f32,
        digestate_ph in 6.5f32..8.5f32,
    ) -> AnaerobicDigestionOutput {
        AnaerobicDigestionOutput {
            digester_id, biogas_m3_per_day, methane_fraction,
            digestate_tonnes_per_day, feedstock_tonnes_per_day,
            hrt_days, digestate_ph,
        }
    }
}

prop_compose! {
    fn arb_landfill_cell()(
        cell_id: u32,
        compaction_ratio in 0.5f32..1.5f32,
        leachate_volume_m3 in 0.0f32..5000.0f32,
        gas_capture_m3_per_hr in 0.0f32..500.0f32,
        methane_fraction in 0.3f32..0.7f32,
        remaining_capacity_m3 in 0.0f64..1_000_000.0f64,
    ) -> LandfillCellMonitor {
        LandfillCellMonitor {
            cell_id, compaction_ratio, leachate_volume_m3,
            gas_capture_m3_per_hr, methane_fraction, remaining_capacity_m3,
        }
    }
}

prop_compose! {
    fn arb_fleet_maintenance()(
        vehicle_id: u32,
        odometer_km in 0.0f64..500_000.0f64,
        fuel_consumed_litres in 0.0f32..5000.0f32,
        engine_hours in 0.0f32..20_000.0f32,
        maintenance_cost_usd in 0.0f32..50_000.0f32,
        next_service_due_s: u64,
        in_service: bool,
    ) -> FleetVehicleMaintenance {
        FleetVehicleMaintenance {
            vehicle_id, odometer_km, fuel_consumed_litres,
            engine_hours, maintenance_cost_usd, next_service_due_s,
            in_service,
        }
    }
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. WasteStreamClass roundtrip ─────────────────────────────────────────────

#[test]
fn test_waste_stream_class_roundtrip() {
    proptest!(|(variant in 0u8..5u8,
        tonnage_kg in 0.0f32..50000.0f32,
        residential: bool,
        recyclable_fraction in 0.0f32..1.0f32,
        un_class in 1u8..9u8,
        manifest_id: u64,
        device_count: u32,
        avg_weight_kg in 0.1f32..50.0f32,
        precious_metal_ppm in 0.0f32..1000.0f32,
        moisture_pct in 0.0f32..100.0f32,
        methane_potential_m3 in 0.0f32..500.0f32,
    )| {
        let val = match variant {
            0 => WasteStreamClass::Msw { tonnage_kg, residential },
            1 => WasteStreamClass::ConstructionDemolition { tonnage_kg, recyclable_fraction },
            2 => WasteStreamClass::Hazardous { un_class, quantity_kg: tonnage_kg, manifest_id },
            3 => WasteStreamClass::EWaste { device_count, avg_weight_kg, precious_metal_ppm },
            _ => WasteStreamClass::Organic { tonnage_kg, moisture_pct, methane_potential_m3 },
        };
        let enc = encode_to_vec(&val).expect("encode WasteStreamClass failed");
        let (dec, consumed): (WasteStreamClass, usize) =
            decode_from_slice(&enc).expect("decode WasteStreamClass failed");
        prop_assert_eq!(&val, &dec, "WasteStreamClass roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 2. BinFillSensor roundtrip ────────────────────────────────────────────────

#[test]
fn test_bin_fill_sensor_roundtrip() {
    proptest!(|(val in arb_bin_fill_sensor())| {
        let enc = encode_to_vec(&val).expect("encode BinFillSensor failed");
        let (dec, consumed): (BinFillSensor, usize) =
            decode_from_slice(&enc).expect("decode BinFillSensor failed");
        prop_assert_eq!(&val, &dec, "BinFillSensor roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 3. Vec<BinFillSensor> roundtrip ───────────────────────────────────────────

#[test]
fn test_vec_bin_fill_sensor_roundtrip() {
    proptest!(|(items in prop::collection::vec(arb_bin_fill_sensor(), 0..10))| {
        let enc = encode_to_vec(&items).expect("encode Vec<BinFillSensor> failed");
        let (dec, consumed): (Vec<BinFillSensor>, usize) =
            decode_from_slice(&enc).expect("decode Vec<BinFillSensor> failed");
        prop_assert_eq!(&items, &dec, "Vec<BinFillSensor> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 4. CollectionRoute roundtrip ──────────────────────────────────────────────

#[test]
fn test_collection_route_roundtrip() {
    proptest!(|(
        route_id: u32,
        total_distance_km in 1.0f32..500.0f32,
        fuel_litres in 5.0f32..300.0f32,
        stop_count in 1u16..200u16,
        eta_minutes in 30u16..600u16,
        truck_tag in "[A-Z]{2}[0-9]{4}",
    )| {
        let val = CollectionRoute {
            route_id, total_distance_km, fuel_litres,
            stop_count, eta_minutes, truck_tag,
        };
        let enc = encode_to_vec(&val).expect("encode CollectionRoute failed");
        let (dec, consumed): (CollectionRoute, usize) =
            decode_from_slice(&enc).expect("decode CollectionRoute failed");
        prop_assert_eq!(&val, &dec, "CollectionRoute roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 5. MrfSortingLine roundtrip ───────────────────────────────────────────────

#[test]
fn test_mrf_sorting_line_roundtrip() {
    proptest!(|(
        line_id: u32,
        throughput_tph in 0.5f32..50.0f32,
        contamination_rate in 0.0f32..0.3f32,
        recovery_rate in 0.7f32..0.99f32,
        operational: bool,
        material_type in "(PET|HDPE|Aluminium|Glass|Paper|Cardboard)",
    )| {
        let val = MrfSortingLine {
            line_id, throughput_tph, contamination_rate,
            recovery_rate, operational, material_type,
        };
        let enc = encode_to_vec(&val).expect("encode MrfSortingLine failed");
        let (dec, consumed): (MrfSortingLine, usize) =
            decode_from_slice(&enc).expect("decode MrfSortingLine failed");
        prop_assert_eq!(&val, &dec, "MrfSortingLine roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 6. MrfSortingLine determinism ─────────────────────────────────────────────

#[test]
fn test_mrf_sorting_line_determinism() {
    proptest!(|(
        line_id: u32,
        throughput_tph in 0.5f32..50.0f32,
        contamination_rate in 0.0f32..0.3f32,
        recovery_rate in 0.7f32..0.99f32,
        operational: bool,
        material_type in "(PET|HDPE|Glass)",
    )| {
        let val = MrfSortingLine {
            line_id, throughput_tph, contamination_rate,
            recovery_rate, operational, material_type,
        };
        let enc1 = encode_to_vec(&val).expect("first encode MrfSortingLine failed");
        let enc2 = encode_to_vec(&val).expect("second encode MrfSortingLine failed");
        prop_assert_eq!(enc1, enc2, "MrfSortingLine encoding must be deterministic");
    });
}

// ── 7. ContaminationRecord roundtrip ──────────────────────────────────────────

#[test]
fn test_contamination_record_roundtrip() {
    proptest!(|(
        batch_id: u64,
        total_weight_kg in 100.0f32..10000.0f32,
        contaminant_weight_kg in 0.0f32..500.0f32,
        contaminant_type_code: u16,
        rejected: bool,
    )| {
        let val = ContaminationRecord {
            batch_id, total_weight_kg, contaminant_weight_kg,
            contaminant_type_code, rejected,
        };
        let enc = encode_to_vec(&val).expect("encode ContaminationRecord failed");
        let (dec, consumed): (ContaminationRecord, usize) =
            decode_from_slice(&enc).expect("decode ContaminationRecord failed");
        prop_assert_eq!(&val, &dec, "ContaminationRecord roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 8. LandfillCellMonitor roundtrip ──────────────────────────────────────────

#[test]
fn test_landfill_cell_monitor_roundtrip() {
    proptest!(|(val in arb_landfill_cell())| {
        let enc = encode_to_vec(&val).expect("encode LandfillCellMonitor failed");
        let (dec, consumed): (LandfillCellMonitor, usize) =
            decode_from_slice(&enc).expect("decode LandfillCellMonitor failed");
        prop_assert_eq!(&val, &dec, "LandfillCellMonitor roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 9. CompostingParams roundtrip ─────────────────────────────────────────────

#[test]
fn test_composting_params_roundtrip() {
    proptest!(|(val in arb_composting_params())| {
        let enc = encode_to_vec(&val).expect("encode CompostingParams failed");
        let (dec, consumed): (CompostingParams, usize) =
            decode_from_slice(&enc).expect("decode CompostingParams failed");
        prop_assert_eq!(&val, &dec, "CompostingParams roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 10. Vec<CompostingParams> roundtrip ───────────────────────────────────────

#[test]
fn test_vec_composting_params_roundtrip() {
    proptest!(|(items in prop::collection::vec(arb_composting_params(), 0..8))| {
        let enc = encode_to_vec(&items).expect("encode Vec<CompostingParams> failed");
        let (dec, consumed): (Vec<CompostingParams>, usize) =
            decode_from_slice(&enc).expect("decode Vec<CompostingParams> failed");
        prop_assert_eq!(&items, &dec, "Vec<CompostingParams> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 11. RecyclingCommodityPrice roundtrip ─────────────────────────────────────

#[test]
fn test_recycling_commodity_price_roundtrip() {
    proptest!(|(
        commodity_code in "(OCC|ONP|PET|HDPE|ALU|FE|CU|GLASS)",
        price_usd_per_tonne in 10.0f32..2000.0f32,
        available_tonnes in 1.0f32..10000.0f32,
        grade in 1u8..6u8,
        price_date_s: u64,
    )| {
        let val = RecyclingCommodityPrice {
            commodity_code, price_usd_per_tonne,
            available_tonnes, grade, price_date_s,
        };
        let enc = encode_to_vec(&val).expect("encode RecyclingCommodityPrice failed");
        let (dec, consumed): (RecyclingCommodityPrice, usize) =
            decode_from_slice(&enc).expect("decode RecyclingCommodityPrice failed");
        prop_assert_eq!(&val, &dec, "RecyclingCommodityPrice roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 12. TransferStationThroughput roundtrip ───────────────────────────────────

#[test]
fn test_transfer_station_throughput_roundtrip() {
    proptest!(|(
        station_id: u32,
        inbound_tonnes in 0.0f32..5000.0f32,
        outbound_tonnes in 0.0f32..5000.0f32,
        truck_arrivals in 0u16..200u16,
        avg_dwell_minutes in 5.0f32..120.0f32,
        utilisation in 0.0f32..1.0f32,
    )| {
        let val = TransferStationThroughput {
            station_id, inbound_tonnes, outbound_tonnes,
            truck_arrivals, avg_dwell_minutes, utilisation,
        };
        let enc = encode_to_vec(&val).expect("encode TransferStationThroughput failed");
        let (dec, consumed): (TransferStationThroughput, usize) =
            decode_from_slice(&enc).expect("decode TransferStationThroughput failed");
        prop_assert_eq!(&val, &dec, "TransferStationThroughput roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 13. HazardousManifest roundtrip ───────────────────────────────────────────

#[test]
fn test_hazardous_manifest_roundtrip() {
    proptest!(|(
        tracking_number: u64,
        generator_id in "[A-Z]{3}[0-9]{6}",
        transporter_id in "[A-Z]{3}[0-9]{6}",
        facility_id in "[A-Z]{3}[0-9]{6}",
        quantity_kg in 0.1f32..50000.0f32,
        dot_code in 1u16..3500u16,
        erg_number in 100u16..200u16,
    )| {
        let val = HazardousManifest {
            tracking_number, generator_id, transporter_id,
            facility_id, quantity_kg, dot_code, erg_number,
        };
        let enc = encode_to_vec(&val).expect("encode HazardousManifest failed");
        let (dec, consumed): (HazardousManifest, usize) =
            decode_from_slice(&enc).expect("decode HazardousManifest failed");
        prop_assert_eq!(&val, &dec, "HazardousManifest roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 14. HazardousManifest determinism ─────────────────────────────────────────

#[test]
fn test_hazardous_manifest_determinism() {
    proptest!(|(
        tracking_number: u64,
        generator_id in "[A-Z]{3}[0-9]{6}",
        transporter_id in "[A-Z]{3}[0-9]{6}",
        facility_id in "[A-Z]{3}[0-9]{6}",
        quantity_kg in 0.1f32..50000.0f32,
        dot_code in 1u16..3500u16,
        erg_number in 100u16..200u16,
    )| {
        let val = HazardousManifest {
            tracking_number, generator_id, transporter_id,
            facility_id, quantity_kg, dot_code, erg_number,
        };
        let enc1 = encode_to_vec(&val).expect("first encode HazardousManifest failed");
        let enc2 = encode_to_vec(&val).expect("second encode HazardousManifest failed");
        prop_assert_eq!(enc1, enc2, "HazardousManifest encoding must be deterministic");
    });
}

// ── 15. EprMetric roundtrip ───────────────────────────────────────────────────

#[test]
fn test_epr_metric_roundtrip() {
    proptest!(|(
        producer_id: u32,
        category_code: u16,
        units_placed: u64,
        units_collected: u64,
        collection_rate in 0.0f32..1.0f32,
        contribution_usd in 0.0f32..1_000_000.0f32,
    )| {
        let val = EprMetric {
            producer_id, category_code, units_placed,
            units_collected, collection_rate, contribution_usd,
        };
        let enc = encode_to_vec(&val).expect("encode EprMetric failed");
        let (dec, consumed): (EprMetric, usize) =
            decode_from_slice(&enc).expect("decode EprMetric failed");
        prop_assert_eq!(&val, &dec, "EprMetric roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 16. CircularEconomyMetric roundtrip ───────────────────────────────────────

#[test]
fn test_circular_economy_metric_roundtrip() {
    proptest!(|(
        jurisdiction_code in "[A-Z]{2}",
        circularity_index in 0.0f32..1.0f32,
        recycled_content in 0.0f32..1.0f32,
        diversion_rate in 0.0f32..1.0f32,
        year in 2000u16..2050u16,
    )| {
        let val = CircularEconomyMetric {
            jurisdiction_code, circularity_index,
            recycled_content, diversion_rate, year,
        };
        let enc = encode_to_vec(&val).expect("encode CircularEconomyMetric failed");
        let (dec, consumed): (CircularEconomyMetric, usize) =
            decode_from_slice(&enc).expect("decode CircularEconomyMetric failed");
        prop_assert_eq!(&val, &dec, "CircularEconomyMetric roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 17. AnaerobicDigestionOutput roundtrip ────────────────────────────────────

#[test]
fn test_anaerobic_digestion_roundtrip() {
    proptest!(|(val in arb_anaerobic_digestion())| {
        let enc = encode_to_vec(&val).expect("encode AnaerobicDigestionOutput failed");
        let (dec, consumed): (AnaerobicDigestionOutput, usize) =
            decode_from_slice(&enc).expect("decode AnaerobicDigestionOutput failed");
        prop_assert_eq!(&val, &dec, "AnaerobicDigestionOutput roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 18. Vec<AnaerobicDigestionOutput> roundtrip ───────────────────────────────

#[test]
fn test_vec_anaerobic_digestion_roundtrip() {
    proptest!(|(items in prop::collection::vec(arb_anaerobic_digestion(), 0..6))| {
        let enc = encode_to_vec(&items).expect("encode Vec<AnaerobicDigestionOutput> failed");
        let (dec, consumed): (Vec<AnaerobicDigestionOutput>, usize) =
            decode_from_slice(&enc).expect("decode Vec<AnaerobicDigestionOutput> failed");
        prop_assert_eq!(&items, &dec, "Vec<AnaerobicDigestionOutput> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 19. EducationCampaignResult roundtrip ─────────────────────────────────────

#[test]
fn test_education_campaign_result_roundtrip() {
    proptest!(|(
        campaign_id: u32,
        target_population in 1000u64..1_000_000u64,
        participants in 0u64..500_000u64,
        pre_contamination in 0.05f32..0.5f32,
        post_contamination in 0.01f32..0.3f32,
        cost_usd in 1000.0f32..500_000.0f32,
    )| {
        let val = EducationCampaignResult {
            campaign_id, target_population, participants,
            pre_contamination, post_contamination, cost_usd,
        };
        let enc = encode_to_vec(&val).expect("encode EducationCampaignResult failed");
        let (dec, consumed): (EducationCampaignResult, usize) =
            decode_from_slice(&enc).expect("decode EducationCampaignResult failed");
        prop_assert_eq!(&val, &dec, "EducationCampaignResult roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 20. FleetVehicleMaintenance roundtrip ─────────────────────────────────────

#[test]
fn test_fleet_vehicle_maintenance_roundtrip() {
    proptest!(|(val in arb_fleet_maintenance())| {
        let enc = encode_to_vec(&val).expect("encode FleetVehicleMaintenance failed");
        let (dec, consumed): (FleetVehicleMaintenance, usize) =
            decode_from_slice(&enc).expect("decode FleetVehicleMaintenance failed");
        prop_assert_eq!(&val, &dec, "FleetVehicleMaintenance roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 21. Vec<FleetVehicleMaintenance> roundtrip ────────────────────────────────

#[test]
fn test_vec_fleet_vehicle_maintenance_roundtrip() {
    proptest!(|(items in prop::collection::vec(arb_fleet_maintenance(), 0..8))| {
        let enc = encode_to_vec(&items).expect("encode Vec<FleetVehicleMaintenance> failed");
        let (dec, consumed): (Vec<FleetVehicleMaintenance>, usize) =
            decode_from_slice(&enc).expect("decode Vec<FleetVehicleMaintenance> failed");
        prop_assert_eq!(&items, &dec, "Vec<FleetVehicleMaintenance> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 22. Mixed waste operations tuple roundtrip ────────────────────────────────

#[test]
fn test_mixed_waste_ops_tuple_roundtrip() {
    proptest!(|(
        sensor in arb_bin_fill_sensor(),
        compost in arb_composting_params(),
        landfill in arb_landfill_cell(),
        digestion in arb_anaerobic_digestion(),
        fleet in arb_fleet_maintenance(),
    )| {
        let val = (
            sensor.clone(),
            compost.clone(),
            landfill.clone(),
            digestion.clone(),
            fleet.clone(),
        );
        let enc = encode_to_vec(&val).expect("encode mixed waste ops tuple failed");
        let (dec, consumed): (
            (BinFillSensor, CompostingParams, LandfillCellMonitor,
             AnaerobicDigestionOutput, FleetVehicleMaintenance),
            usize,
        ) = decode_from_slice(&enc).expect("decode mixed waste ops tuple failed");
        prop_assert_eq!(val.0, dec.0, "BinFillSensor mismatch in tuple");
        prop_assert_eq!(val.1, dec.1, "CompostingParams mismatch in tuple");
        prop_assert_eq!(val.2, dec.2, "LandfillCellMonitor mismatch in tuple");
        prop_assert_eq!(val.3, dec.3, "AnaerobicDigestionOutput mismatch in tuple");
        prop_assert_eq!(val.4, dec.4, "FleetVehicleMaintenance mismatch in tuple");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}
