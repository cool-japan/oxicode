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

// --- Domain types: Commercial Laundry & Dry Cleaning Operations ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FabricType {
    Cotton,
    Polyester,
    Silk,
    Wool,
    Linen,
    CottonPolyBlend { cotton_pct: u8, poly_pct: u8 },
    Cashmere,
    Rayon,
    Denim,
    Leather,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StainClassification {
    Protein,
    Tannin,
    Grease,
    Dye,
    Ink,
    Oxidizable,
    Combination { primary: String, secondary: String },
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GarmentIntakeRecord {
    ticket_id: u64,
    customer_id: u32,
    garment_description: String,
    fabric: FabricType,
    color: String,
    stain: Option<StainClassification>,
    special_instructions: String,
    estimated_pickup_epoch: u64,
    is_rush: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WashProgramParameters {
    program_code: String,
    temperature_celsius: u16,
    detergent_type: String,
    detergent_ml: u32,
    softener_ml: u32,
    cycle_duration_minutes: u16,
    spin_rpm: u16,
    pre_soak: bool,
    extra_rinse: bool,
    water_level_liters: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PressingStationSettings {
    station_id: u16,
    steam_pressure_bar: f64,
    plate_temperature_celsius: f64,
    vacuum_suction_enabled: bool,
    fabric_preset: String,
    press_duration_seconds: u16,
    finishing_spray: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SolventDistillationLog {
    batch_id: u64,
    solvent_type: String,
    input_volume_liters: f64,
    recovered_volume_liters: f64,
    residue_weight_grams: f64,
    distillation_temperature_celsius: f64,
    start_epoch: u64,
    end_epoch: u64,
    operator_id: u32,
    passed_purity_check: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ConveyorGarmentTracking {
    conveyor_id: u16,
    rail_position: u32,
    ticket_id: u64,
    bag_tag: String,
    zone: String,
    loaded_epoch: u64,
    is_ready_for_pickup: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CustomerAccountBalance {
    customer_id: u32,
    full_name: String,
    phone: String,
    email: Option<String>,
    balance_cents: i64,
    loyalty_points: u32,
    preferred_starch: String,
    credit_on_file: bool,
    last_visit_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RouteDeliverySchedule {
    route_id: u32,
    driver_name: String,
    vehicle_plate: String,
    stops: Vec<DeliveryStop>,
    departure_epoch: u64,
    estimated_return_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DeliveryStop {
    stop_order: u16,
    address: String,
    customer_id: u32,
    ticket_ids: Vec<u64>,
    is_pickup: bool,
    window_start_epoch: u64,
    window_end_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpotTreatmentFormulation {
    formula_id: u32,
    name: String,
    target_stain: StainClassification,
    chemicals: Vec<ChemicalComponent>,
    dwell_time_seconds: u32,
    requires_heat: bool,
    safe_fabrics: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ChemicalComponent {
    chemical_name: String,
    concentration_pct: f64,
    volume_ml: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MachineMaintenanceCycle {
    machine_id: u32,
    machine_type: String,
    last_service_epoch: u64,
    next_service_epoch: u64,
    total_cycles_run: u64,
    cycles_since_service: u32,
    parts_replaced: Vec<String>,
    technician_notes: String,
    is_operational: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LinenRentalInventoryItem {
    item_code: String,
    description: String,
    total_stock: u32,
    in_circulation: u32,
    in_wash: u32,
    damaged: u32,
    cost_per_unit_cents: u32,
    rental_rate_cents: u32,
    category: LinenCategory,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum LinenCategory {
    Tablecloth,
    Napkin,
    BedSheet,
    PillowCase,
    Towel,
    Apron,
    ChefCoat,
    BarMop,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UniformProgramContract {
    contract_id: u64,
    client_company: String,
    start_epoch: u64,
    end_epoch: u64,
    employee_count: u32,
    garments_per_employee: u16,
    weekly_pickup_day: String,
    monthly_fee_cents: u64,
    includes_repairs: bool,
    embroidery_included: bool,
    uniform_types: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum InspectionGrade {
    Premium,
    Acceptable,
    NeedsRework,
    Rejected { reason: String },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct QualityInspectionReport {
    inspection_id: u64,
    ticket_id: u64,
    inspector_id: u32,
    grade: InspectionGrade,
    stain_removed: bool,
    pressing_quality_score: u8,
    button_check: bool,
    tear_check: bool,
    odor_free: bool,
    inspection_epoch: u64,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DryCleaningBatchLog {
    batch_id: u64,
    machine_id: u32,
    solvent_type: String,
    load_weight_kg: f64,
    ticket_ids: Vec<u64>,
    program_name: String,
    cycle_start_epoch: u64,
    cycle_end_epoch: u64,
    solvent_consumption_liters: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PricingRule {
    rule_id: u32,
    garment_type: String,
    service_type: String,
    base_price_cents: u32,
    rush_surcharge_pct: u8,
    bulk_discount_pct: u8,
    stain_treatment_fee_cents: u32,
    effective_from_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EmployeeShiftRecord {
    employee_id: u32,
    name: String,
    role: String,
    shift_start_epoch: u64,
    shift_end_epoch: u64,
    station_assignments: Vec<String>,
    garments_processed: u32,
    overtime_minutes: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WaterUsageReport {
    report_date: String,
    total_liters: u64,
    hot_water_liters: u64,
    cold_water_liters: u64,
    recycled_liters: u64,
    machine_breakdown: Vec<MachineWaterUsage>,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MachineWaterUsage {
    machine_id: u32,
    liters_used: u64,
    cycles_run: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PerchloroethyleneComplianceRecord {
    facility_id: u32,
    measurement_epoch: u64,
    air_concentration_ppm: f64,
    wastewater_concentration_ppb: f64,
    below_osha_limit: bool,
    below_epa_limit: bool,
    monitor_serial: String,
    calibration_date: String,
    inspector_badge: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GarmentDamageReport {
    report_id: u64,
    ticket_id: u64,
    damage_type: String,
    description: String,
    pre_existing: bool,
    customer_notified: bool,
    compensation_cents: u32,
    photo_reference: Option<String>,
    staff_responsible_id: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HotelLinenContract {
    contract_id: u64,
    hotel_name: String,
    room_count: u32,
    daily_sheet_sets: u32,
    daily_towel_sets: u32,
    daily_pillowcases: u32,
    weekly_banquet_linens: u32,
    pickup_time: String,
    delivery_time: String,
    monthly_rate_cents: u64,
    par_level_multiplier: f64,
}

// --- Tests ---

#[test]
fn test_garment_intake_record_with_stain() {
    let cfg = config::standard();
    let record = GarmentIntakeRecord {
        ticket_id: 100234,
        customer_id: 5501,
        garment_description: "Men's navy wool blazer, 42R".to_string(),
        fabric: FabricType::Wool,
        color: "Navy".to_string(),
        stain: Some(StainClassification::Grease),
        special_instructions: "Button loose on left cuff".to_string(),
        estimated_pickup_epoch: 1710600000,
        is_rush: false,
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode GarmentIntakeRecord with stain");
    let (decoded, _): (GarmentIntakeRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GarmentIntakeRecord with stain");
    assert_eq!(record, decoded);
}

#[test]
fn test_garment_intake_blend_fabric_no_stain() {
    let cfg = config::standard();
    let record = GarmentIntakeRecord {
        ticket_id: 100235,
        customer_id: 5502,
        garment_description: "Women's blouse, size M".to_string(),
        fabric: FabricType::CottonPolyBlend {
            cotton_pct: 65,
            poly_pct: 35,
        },
        color: "White".to_string(),
        stain: None,
        special_instructions: "Light starch on collar".to_string(),
        estimated_pickup_epoch: 1710686400,
        is_rush: true,
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode GarmentIntakeRecord blend fabric");
    let (decoded, _): (GarmentIntakeRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GarmentIntakeRecord blend fabric");
    assert_eq!(record, decoded);
}

#[test]
fn test_wash_program_parameters_heavy_duty() {
    let cfg = config::standard();
    let program = WashProgramParameters {
        program_code: "HD-90-ENZYME".to_string(),
        temperature_celsius: 90,
        detergent_type: "Enzymatic Heavy Duty".to_string(),
        detergent_ml: 120,
        softener_ml: 0,
        cycle_duration_minutes: 55,
        spin_rpm: 1200,
        pre_soak: true,
        extra_rinse: true,
        water_level_liters: 85,
    };
    let bytes = encode_to_vec(&program, cfg).expect("encode WashProgramParameters heavy duty");
    let (decoded, _): (WashProgramParameters, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WashProgramParameters heavy duty");
    assert_eq!(program, decoded);
}

#[test]
fn test_pressing_station_settings_silk() {
    let cfg = config::standard();
    let settings = PressingStationSettings {
        station_id: 7,
        steam_pressure_bar: 2.5,
        plate_temperature_celsius: 120.0,
        vacuum_suction_enabled: true,
        fabric_preset: "Silk/Delicate".to_string(),
        press_duration_seconds: 8,
        finishing_spray: Some("Anti-static silk finish".to_string()),
    };
    let bytes = encode_to_vec(&settings, cfg).expect("encode PressingStationSettings silk");
    let (decoded, _): (PressingStationSettings, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PressingStationSettings silk");
    assert_eq!(settings, decoded);
}

#[test]
fn test_pressing_station_no_finishing_spray() {
    let cfg = config::standard();
    let settings = PressingStationSettings {
        station_id: 12,
        steam_pressure_bar: 4.0,
        plate_temperature_celsius: 200.0,
        vacuum_suction_enabled: false,
        fabric_preset: "Cotton Heavy".to_string(),
        press_duration_seconds: 15,
        finishing_spray: None,
    };
    let bytes = encode_to_vec(&settings, cfg).expect("encode PressingStationSettings no spray");
    let (decoded, _): (PressingStationSettings, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PressingStationSettings no spray");
    assert_eq!(settings, decoded);
}

#[test]
fn test_solvent_distillation_log_roundtrip() {
    let cfg = config::standard();
    let log = SolventDistillationLog {
        batch_id: 88120,
        solvent_type: "Perchloroethylene".to_string(),
        input_volume_liters: 45.0,
        recovered_volume_liters: 42.3,
        residue_weight_grams: 680.5,
        distillation_temperature_celsius: 121.0,
        start_epoch: 1710500000,
        end_epoch: 1710507200,
        operator_id: 301,
        passed_purity_check: true,
    };
    let bytes = encode_to_vec(&log, cfg).expect("encode SolventDistillationLog");
    let (decoded, _): (SolventDistillationLog, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SolventDistillationLog");
    assert_eq!(log, decoded);
}

#[test]
fn test_conveyor_garment_tracking_ready() {
    let cfg = config::standard();
    let tracking = ConveyorGarmentTracking {
        conveyor_id: 3,
        rail_position: 1247,
        ticket_id: 100234,
        bag_tag: "B-100234-A".to_string(),
        zone: "Pickup-Front".to_string(),
        loaded_epoch: 1710590000,
        is_ready_for_pickup: true,
    };
    let bytes = encode_to_vec(&tracking, cfg).expect("encode ConveyorGarmentTracking ready");
    let (decoded, _): (ConveyorGarmentTracking, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ConveyorGarmentTracking ready");
    assert_eq!(tracking, decoded);
}

#[test]
fn test_customer_account_balance_with_credit() {
    let cfg = config::standard();
    let account = CustomerAccountBalance {
        customer_id: 5501,
        full_name: "Margaret Chen".to_string(),
        phone: "+1-555-0142".to_string(),
        email: Some("m.chen@email.com".to_string()),
        balance_cents: -1250,
        loyalty_points: 3400,
        preferred_starch: "Medium".to_string(),
        credit_on_file: true,
        last_visit_epoch: 1710500000,
    };
    let bytes = encode_to_vec(&account, cfg).expect("encode CustomerAccountBalance with credit");
    let (decoded, _): (CustomerAccountBalance, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CustomerAccountBalance with credit");
    assert_eq!(account, decoded);
}

#[test]
fn test_route_delivery_schedule_multiple_stops() {
    let cfg = config::standard();
    let schedule = RouteDeliverySchedule {
        route_id: 14,
        driver_name: "James Kowalski".to_string(),
        vehicle_plate: "LND-4872".to_string(),
        stops: vec![
            DeliveryStop {
                stop_order: 1,
                address: "200 Main St, Suite 400".to_string(),
                customer_id: 8801,
                ticket_ids: vec![200100, 200101, 200102],
                is_pickup: false,
                window_start_epoch: 1710580000,
                window_end_epoch: 1710583600,
            },
            DeliveryStop {
                stop_order: 2,
                address: "450 Oak Avenue".to_string(),
                customer_id: 8802,
                ticket_ids: vec![200200],
                is_pickup: true,
                window_start_epoch: 1710585000,
                window_end_epoch: 1710588600,
            },
            DeliveryStop {
                stop_order: 3,
                address: "12 Industrial Blvd".to_string(),
                customer_id: 8803,
                ticket_ids: vec![200300, 200301, 200302, 200303],
                is_pickup: false,
                window_start_epoch: 1710590000,
                window_end_epoch: 1710593600,
            },
        ],
        departure_epoch: 1710576000,
        estimated_return_epoch: 1710604800,
    };
    let bytes = encode_to_vec(&schedule, cfg).expect("encode RouteDeliverySchedule");
    let (decoded, _): (RouteDeliverySchedule, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RouteDeliverySchedule");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_spot_treatment_formulation_combination_stain() {
    let cfg = config::standard();
    let formula = SpotTreatmentFormulation {
        formula_id: 42,
        name: "Red Wine Emergency Protocol".to_string(),
        target_stain: StainClassification::Combination {
            primary: "Tannin".to_string(),
            secondary: "Dye".to_string(),
        },
        chemicals: vec![
            ChemicalComponent {
                chemical_name: "Sodium percarbonate".to_string(),
                concentration_pct: 12.0,
                volume_ml: 30.0,
            },
            ChemicalComponent {
                chemical_name: "Citric acid solution".to_string(),
                concentration_pct: 5.0,
                volume_ml: 15.0,
            },
            ChemicalComponent {
                chemical_name: "Surfactant blend".to_string(),
                concentration_pct: 8.5,
                volume_ml: 10.0,
            },
        ],
        dwell_time_seconds: 300,
        requires_heat: false,
        safe_fabrics: vec![
            "Cotton".to_string(),
            "Polyester".to_string(),
            "Linen".to_string(),
        ],
    };
    let bytes = encode_to_vec(&formula, cfg).expect("encode SpotTreatmentFormulation");
    let (decoded, _): (SpotTreatmentFormulation, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SpotTreatmentFormulation");
    assert_eq!(formula, decoded);
}

#[test]
fn test_machine_maintenance_cycle_roundtrip() {
    let cfg = config::standard();
    let maintenance = MachineMaintenanceCycle {
        machine_id: 15,
        machine_type: "Washer Extractor 60kg".to_string(),
        last_service_epoch: 1709000000,
        next_service_epoch: 1711600000,
        total_cycles_run: 45230,
        cycles_since_service: 312,
        parts_replaced: vec![
            "Door seal gasket".to_string(),
            "Drain pump impeller".to_string(),
            "Bearing set".to_string(),
        ],
        technician_notes: "Slight vibration at high spin, monitor next visit".to_string(),
        is_operational: true,
    };
    let bytes = encode_to_vec(&maintenance, cfg).expect("encode MachineMaintenanceCycle");
    let (decoded, _): (MachineMaintenanceCycle, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MachineMaintenanceCycle");
    assert_eq!(maintenance, decoded);
}

#[test]
fn test_linen_rental_inventory_tablecloth() {
    let cfg = config::standard();
    let item = LinenRentalInventoryItem {
        item_code: "LR-TBL-120R-WHT".to_string(),
        description: "120-inch round tablecloth, white polyester".to_string(),
        total_stock: 500,
        in_circulation: 320,
        in_wash: 85,
        damaged: 12,
        cost_per_unit_cents: 1450,
        rental_rate_cents: 325,
        category: LinenCategory::Tablecloth,
    };
    let bytes = encode_to_vec(&item, cfg).expect("encode LinenRentalInventoryItem tablecloth");
    let (decoded, _): (LinenRentalInventoryItem, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode LinenRentalInventoryItem tablecloth");
    assert_eq!(item, decoded);
}

#[test]
fn test_uniform_program_contract_full() {
    let cfg = config::standard();
    let contract = UniformProgramContract {
        contract_id: 770001,
        client_company: "Midwest Automotive Group".to_string(),
        start_epoch: 1704067200,
        end_epoch: 1735689600,
        employee_count: 85,
        garments_per_employee: 11,
        weekly_pickup_day: "Wednesday".to_string(),
        monthly_fee_cents: 425000,
        includes_repairs: true,
        embroidery_included: true,
        uniform_types: vec![
            "Mechanic coverall".to_string(),
            "Shop polo shirt".to_string(),
            "Safety vest".to_string(),
            "Work trousers".to_string(),
        ],
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode UniformProgramContract");
    let (decoded, _): (UniformProgramContract, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode UniformProgramContract");
    assert_eq!(contract, decoded);
}

#[test]
fn test_quality_inspection_premium_grade() {
    let cfg = config::standard();
    let report = QualityInspectionReport {
        inspection_id: 990001,
        ticket_id: 100234,
        inspector_id: 44,
        grade: InspectionGrade::Premium,
        stain_removed: true,
        pressing_quality_score: 95,
        button_check: true,
        tear_check: true,
        odor_free: true,
        inspection_epoch: 1710595000,
        notes: "Excellent finish on lapels".to_string(),
    };
    let bytes = encode_to_vec(&report, cfg).expect("encode QualityInspectionReport premium");
    let (decoded, _): (QualityInspectionReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode QualityInspectionReport premium");
    assert_eq!(report, decoded);
}

#[test]
fn test_quality_inspection_rejected_grade() {
    let cfg = config::standard();
    let report = QualityInspectionReport {
        inspection_id: 990002,
        ticket_id: 100300,
        inspector_id: 44,
        grade: InspectionGrade::Rejected {
            reason: "Persistent grease stain on front panel".to_string(),
        },
        stain_removed: false,
        pressing_quality_score: 40,
        button_check: true,
        tear_check: true,
        odor_free: true,
        inspection_epoch: 1710596000,
        notes: "Return to spot treatment station for second attempt".to_string(),
    };
    let bytes = encode_to_vec(&report, cfg).expect("encode QualityInspectionReport rejected");
    let (decoded, _): (QualityInspectionReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode QualityInspectionReport rejected");
    assert_eq!(report, decoded);
}

#[test]
fn test_dry_cleaning_batch_log_roundtrip() {
    let cfg = config::standard();
    let batch = DryCleaningBatchLog {
        batch_id: 55001,
        machine_id: 8,
        solvent_type: "GreenEarth Silicone D5".to_string(),
        load_weight_kg: 18.7,
        ticket_ids: vec![100500, 100501, 100502, 100503, 100504, 100505],
        program_name: "Delicate Dry Clean 35min".to_string(),
        cycle_start_epoch: 1710560000,
        cycle_end_epoch: 1710562100,
        solvent_consumption_liters: 2.3,
    };
    let bytes = encode_to_vec(&batch, cfg).expect("encode DryCleaningBatchLog");
    let (decoded, _): (DryCleaningBatchLog, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DryCleaningBatchLog");
    assert_eq!(batch, decoded);
}

#[test]
fn test_pricing_rule_with_rush_surcharge() {
    let cfg = config::standard();
    let rule = PricingRule {
        rule_id: 201,
        garment_type: "Suit (2-piece)".to_string(),
        service_type: "Dry Clean + Press".to_string(),
        base_price_cents: 1895,
        rush_surcharge_pct: 50,
        bulk_discount_pct: 10,
        stain_treatment_fee_cents: 500,
        effective_from_epoch: 1704067200,
    };
    let bytes = encode_to_vec(&rule, cfg).expect("encode PricingRule with rush");
    let (decoded, _): (PricingRule, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PricingRule with rush");
    assert_eq!(rule, decoded);
}

#[test]
fn test_employee_shift_record_overtime() {
    let cfg = config::standard();
    let shift = EmployeeShiftRecord {
        employee_id: 1007,
        name: "Fatima Al-Rashid".to_string(),
        role: "Lead Presser".to_string(),
        shift_start_epoch: 1710504000,
        shift_end_epoch: 1710536400,
        station_assignments: vec![
            "Press-Station-3".to_string(),
            "Press-Station-4".to_string(),
            "Quality-Check-1".to_string(),
        ],
        garments_processed: 147,
        overtime_minutes: 45,
    };
    let bytes = encode_to_vec(&shift, cfg).expect("encode EmployeeShiftRecord overtime");
    let (decoded, _): (EmployeeShiftRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EmployeeShiftRecord overtime");
    assert_eq!(shift, decoded);
}

#[test]
fn test_water_usage_report_with_machine_breakdown() {
    let cfg = config::standard();
    let report = WaterUsageReport {
        report_date: "2026-03-15".to_string(),
        total_liters: 12500,
        hot_water_liters: 7800,
        cold_water_liters: 4700,
        recycled_liters: 1900,
        machine_breakdown: vec![
            MachineWaterUsage {
                machine_id: 15,
                liters_used: 4200,
                cycles_run: 12,
            },
            MachineWaterUsage {
                machine_id: 16,
                liters_used: 3800,
                cycles_run: 11,
            },
            MachineWaterUsage {
                machine_id: 17,
                liters_used: 2600,
                cycles_run: 8,
            },
            MachineWaterUsage {
                machine_id: 18,
                liters_used: 1900,
                cycles_run: 6,
            },
        ],
        cost_cents: 3750,
    };
    let bytes = encode_to_vec(&report, cfg).expect("encode WaterUsageReport");
    let (decoded, _): (WaterUsageReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WaterUsageReport");
    assert_eq!(report, decoded);
}

#[test]
fn test_perchloroethylene_compliance_record_passing() {
    let cfg = config::standard();
    let record = PerchloroethyleneComplianceRecord {
        facility_id: 1001,
        measurement_epoch: 1710550000,
        air_concentration_ppm: 12.4,
        wastewater_concentration_ppb: 3.7,
        below_osha_limit: true,
        below_epa_limit: true,
        monitor_serial: "PCE-MON-2024-0087".to_string(),
        calibration_date: "2026-02-01".to_string(),
        inspector_badge: "ENV-4412".to_string(),
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode PerchloroethyleneComplianceRecord");
    let (decoded, _): (PerchloroethyleneComplianceRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PerchloroethyleneComplianceRecord");
    assert_eq!(record, decoded);
}

#[test]
fn test_garment_damage_report_with_photo() {
    let cfg = config::standard();
    let report = GarmentDamageReport {
        report_id: 7700,
        ticket_id: 100400,
        damage_type: "Discoloration".to_string(),
        description: "Bleach spot on left sleeve, approximately 2cm diameter".to_string(),
        pre_existing: false,
        customer_notified: true,
        compensation_cents: 4500,
        photo_reference: Some("DMG-7700-IMG001.jpg".to_string()),
        staff_responsible_id: 1003,
    };
    let bytes = encode_to_vec(&report, cfg).expect("encode GarmentDamageReport with photo");
    let (decoded, _): (GarmentDamageReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GarmentDamageReport with photo");
    assert_eq!(report, decoded);
}

#[test]
fn test_hotel_linen_contract_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let contract = HotelLinenContract {
        contract_id: 330001,
        hotel_name: "Grand Meridian Hotel & Convention Center".to_string(),
        room_count: 420,
        daily_sheet_sets: 504,
        daily_towel_sets: 630,
        daily_pillowcases: 1008,
        weekly_banquet_linens: 200,
        pickup_time: "06:00".to_string(),
        delivery_time: "14:00".to_string(),
        monthly_rate_cents: 8750000,
        par_level_multiplier: 3.5,
    };
    let bytes = encode_to_vec(&contract, cfg).expect("encode HotelLinenContract fixed int");
    let (decoded, _): (HotelLinenContract, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HotelLinenContract fixed int");
    assert_eq!(contract, decoded);
}
