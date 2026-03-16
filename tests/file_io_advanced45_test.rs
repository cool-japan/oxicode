//! Advanced file I/O tests for OxiCode — domain: food truck and mobile restaurant operations

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
enum AllergenCategory {
    Gluten,
    Dairy,
    TreeNuts,
    Peanuts,
    Shellfish,
    Soy,
    Eggs,
    Sesame,
    Fish,
    Sulfites,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PrepComplexity {
    Quick,
    Moderate,
    Complex,
    ChefSpecial,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PermitType {
    Street,
    PrivateLot,
    Festival,
    CateringVenue,
    FarmersMarket,
    SpecialEvent,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PaymentMethod {
    Cash,
    CreditCard,
    DebitCard,
    MobileWallet,
    PrepaidVoucher,
    LoyaltyRedemption,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InspectionGrade {
    Pass,
    ConditionalPass,
    ReInspectionRequired,
    Fail,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FuelType {
    Propane,
    Diesel,
    Gasoline,
    Biodiesel,
    Electric,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeatherCondition {
    Sunny,
    Cloudy,
    LightRain,
    HeavyRain,
    Snow,
    Windy,
    Extreme,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ShiftRole {
    HeadChef,
    LineCook,
    CashierWindow,
    Prep,
    Driver,
    Manager,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EquipmentKind {
    DeepFryer,
    FlatTopGrill,
    CharGrill,
    SteamTable,
    Refrigerator,
    Generator,
    PropaneTank,
    PosTerminal,
    Exhaust,
    WaterPump,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaintenanceUrgency {
    Routine,
    Soon,
    Urgent,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BookingStatus {
    Inquiry,
    Tentative,
    Confirmed,
    DepositPaid,
    Completed,
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BroadcastPlatform {
    Twitter,
    Instagram,
    Facebook,
    TikTok,
    GoogleMaps,
    CustomApp,
}

// ---------------------------------------------------------------------------
// Struct types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Ingredient {
    name: String,
    quantity_grams: u32,
    cost_cents: u32,
    is_perishable: bool,
    shelf_life_hours: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MenuItem {
    item_id: u32,
    name: String,
    description: String,
    price_cents: u32,
    ingredients: Vec<Ingredient>,
    allergens: Vec<AllergenCategory>,
    prep_time_seconds: u16,
    complexity: PrepComplexity,
    is_vegan: bool,
    is_gluten_free: bool,
    calories: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsCoordinate {
    latitude_microdeg: i64,
    longitude_microdeg: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RouteStop {
    stop_id: u16,
    location: GpsCoordinate,
    location_name: String,
    arrival_timestamp: u64,
    departure_timestamp: u64,
    permit: PermitType,
    permit_number: String,
    expected_foot_traffic: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DailyRouteSchedule {
    schedule_id: u32,
    truck_id: String,
    date_ymd: String,
    stops: Vec<RouteStop>,
    total_distance_meters: u64,
    driver_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InventoryItem {
    ingredient_name: String,
    on_hand_grams: u32,
    par_level_grams: u32,
    reorder_point_grams: u32,
    supplier_name: String,
    last_delivery_timestamp: u64,
    unit_cost_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PosTransaction {
    transaction_id: u64,
    timestamp: u64,
    items_sold: Vec<(u32, u8)>,
    subtotal_cents: u32,
    tax_cents: u32,
    tip_cents: u32,
    total_cents: u32,
    payment_method: PaymentMethod,
    customer_loyalty_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InspectionViolation {
    code: String,
    description: String,
    is_critical: bool,
    corrected_on_site: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HealthInspection {
    inspection_id: u32,
    inspector_name: String,
    inspection_date: String,
    truck_id: String,
    grade: InspectionGrade,
    violations: Vec<InspectionViolation>,
    follow_up_date: Option<String>,
    overall_score: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommissaryLog {
    log_id: u64,
    kitchen_name: String,
    arrival_timestamp: u64,
    departure_timestamp: u64,
    items_prepped: Vec<(String, u32)>,
    items_loaded: Vec<(String, u32)>,
    waste_disposed_kg: u16,
    cleaning_completed: bool,
    inspector_sign_off: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuelReading {
    reading_id: u32,
    fuel_type: FuelType,
    tank_capacity_liters_x10: u32,
    current_level_liters_x10: u32,
    consumption_rate_liters_per_hour_x10: u16,
    last_refill_timestamp: u64,
    cost_per_liter_cents: u32,
    estimated_hours_remaining_x10: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LocationBroadcast {
    broadcast_id: u64,
    platform: BroadcastPlatform,
    timestamp: u64,
    location: GpsCoordinate,
    message: String,
    menu_specials: Vec<String>,
    estimated_wait_minutes: u8,
    likes_count: u32,
    shares_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CateringBooking {
    booking_id: u64,
    client_name: String,
    event_date: String,
    event_location: String,
    guest_count: u16,
    menu_items: Vec<u32>,
    status: BookingStatus,
    deposit_cents: u32,
    total_quote_cents: u32,
    dietary_notes: Vec<String>,
    setup_time_minutes: u16,
    teardown_time_minutes: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherForecastEntry {
    date: String,
    condition: WeatherCondition,
    high_temp_c_x10: i16,
    low_temp_c_x10: i16,
    precipitation_pct: u8,
    wind_speed_kph_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DemandForecast {
    forecast_id: u32,
    truck_id: String,
    date: String,
    weather: WeatherForecastEntry,
    predicted_customers: u32,
    predicted_revenue_cents: u64,
    recommended_par_items: Vec<(String, u32)>,
    confidence_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StaffShift {
    employee_id: u32,
    employee_name: String,
    role: ShiftRole,
    shift_date: String,
    start_timestamp: u64,
    end_timestamp: u64,
    break_minutes: u16,
    hourly_rate_cents: u32,
    overtime_minutes: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StaffSchedule {
    schedule_id: u32,
    week_start_date: String,
    truck_id: String,
    shifts: Vec<StaffShift>,
    total_labor_cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FoodCostRecord {
    record_id: u32,
    menu_item_id: u32,
    menu_item_name: String,
    ingredient_costs_cents: Vec<(String, u32)>,
    total_food_cost_cents: u32,
    selling_price_cents: u32,
    margin_bps: u16,
    portions_sold_today: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoyaltyStampCard {
    card_id: u64,
    customer_name: String,
    phone_hash: u64,
    stamps_collected: u8,
    stamps_for_reward: u8,
    rewards_redeemed: u16,
    total_visits: u32,
    total_spent_cents: u64,
    last_visit_timestamp: u64,
    favorite_item_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FestivalApplication {
    application_id: u64,
    festival_name: String,
    festival_dates: Vec<String>,
    location_name: String,
    booth_fee_cents: u32,
    electricity_provided: bool,
    water_hookup: bool,
    expected_attendance: u32,
    menu_submitted: Vec<String>,
    insurance_policy_number: String,
    status: BookingStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquipmentMaintenanceLog {
    log_id: u64,
    equipment: EquipmentKind,
    equipment_serial: String,
    truck_id: String,
    last_service_date: String,
    next_service_date: String,
    urgency: MaintenanceUrgency,
    notes: String,
    service_cost_cents: u32,
    parts_replaced: Vec<String>,
    technician_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_menu_item_config_roundtrip() {
    let item = MenuItem {
        item_id: 101,
        name: "Korean BBQ Tacos".into(),
        description: "Bulgogi beef with kimchi slaw, sriracha aioli, on corn tortillas".into(),
        price_cents: 1250,
        ingredients: vec![
            Ingredient {
                name: "Bulgogi beef".into(),
                quantity_grams: 150,
                cost_cents: 320,
                is_perishable: true,
                shelf_life_hours: 48,
            },
            Ingredient {
                name: "Kimchi".into(),
                quantity_grams: 40,
                cost_cents: 55,
                is_perishable: true,
                shelf_life_hours: 168,
            },
            Ingredient {
                name: "Corn tortilla".into(),
                quantity_grams: 60,
                cost_cents: 30,
                is_perishable: false,
                shelf_life_hours: 720,
            },
            Ingredient {
                name: "Sriracha aioli".into(),
                quantity_grams: 20,
                cost_cents: 18,
                is_perishable: true,
                shelf_life_hours: 72,
            },
        ],
        allergens: vec![
            AllergenCategory::Soy,
            AllergenCategory::Gluten,
            AllergenCategory::Eggs,
        ],
        prep_time_seconds: 240,
        complexity: PrepComplexity::Moderate,
        is_vegan: false,
        is_gluten_free: false,
        calories: 480,
    };

    let encoded = encode_to_vec(&item).expect("encode MenuItem");
    let (decoded, _): (MenuItem, _) = decode_from_slice(&encoded).expect("decode MenuItem");
    assert_eq!(item, decoded);
}

#[test]
fn test_menu_item_vegan_gluten_free_file() {
    let item = MenuItem {
        item_id: 202,
        name: "Jackfruit Carnitas Bowl".into(),
        description: "Slow-braised jackfruit with cilantro lime rice and black beans".into(),
        price_cents: 1400,
        ingredients: vec![
            Ingredient {
                name: "Jackfruit".into(),
                quantity_grams: 180,
                cost_cents: 210,
                is_perishable: true,
                shelf_life_hours: 96,
            },
            Ingredient {
                name: "Jasmine rice".into(),
                quantity_grams: 120,
                cost_cents: 22,
                is_perishable: false,
                shelf_life_hours: 8760,
            },
            Ingredient {
                name: "Black beans".into(),
                quantity_grams: 80,
                cost_cents: 18,
                is_perishable: false,
                shelf_life_hours: 4380,
            },
        ],
        allergens: vec![],
        prep_time_seconds: 180,
        complexity: PrepComplexity::Quick,
        is_vegan: true,
        is_gluten_free: true,
        calories: 390,
    };

    let path = temp_dir().join("oxicode_fio45_menu_vegan.bin");
    encode_to_file(&item, &path).expect("encode_to_file vegan MenuItem");
    let decoded: MenuItem = decode_from_file(&path).expect("decode_from_file vegan MenuItem");
    assert_eq!(item, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_daily_route_schedule_roundtrip() {
    let schedule = DailyRouteSchedule {
        schedule_id: 5001,
        truck_id: "TACO-FURY-01".into(),
        date_ymd: "2026-03-15".into(),
        stops: vec![
            RouteStop {
                stop_id: 1,
                location: GpsCoordinate {
                    latitude_microdeg: 37_774_900,
                    longitude_microdeg: -122_419_400,
                },
                location_name: "Financial District".into(),
                arrival_timestamp: 1710489600,
                departure_timestamp: 1710500400,
                permit: PermitType::Street,
                permit_number: "SF-2026-4412".into(),
                expected_foot_traffic: 8500,
            },
            RouteStop {
                stop_id: 2,
                location: GpsCoordinate {
                    latitude_microdeg: 37_785_800,
                    longitude_microdeg: -122_409_200,
                },
                location_name: "SOMA Tech Park".into(),
                arrival_timestamp: 1710504000,
                departure_timestamp: 1710514800,
                permit: PermitType::PrivateLot,
                permit_number: "PVT-TP-889".into(),
                expected_foot_traffic: 3200,
            },
        ],
        total_distance_meters: 18_400,
        driver_name: "Maria Gonzalez".into(),
    };

    let path = temp_dir().join("oxicode_fio45_route.bin");
    encode_to_file(&schedule, &path).expect("encode_to_file DailyRouteSchedule");
    let decoded: DailyRouteSchedule =
        decode_from_file(&path).expect("decode_from_file DailyRouteSchedule");
    assert_eq!(schedule, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_inventory_par_levels_batch() {
    let inventory = vec![
        InventoryItem {
            ingredient_name: "Ground beef 80/20".into(),
            on_hand_grams: 15_000,
            par_level_grams: 20_000,
            reorder_point_grams: 8_000,
            supplier_name: "Metro Meats".into(),
            last_delivery_timestamp: 1710403200,
            unit_cost_cents: 1150,
        },
        InventoryItem {
            ingredient_name: "Brioche buns".into(),
            on_hand_grams: 4_800,
            par_level_grams: 6_000,
            reorder_point_grams: 2_400,
            supplier_name: "Golden Bakery".into(),
            last_delivery_timestamp: 1710316800,
            unit_cost_cents: 280,
        },
        InventoryItem {
            ingredient_name: "Avocados".into(),
            on_hand_grams: 3_200,
            par_level_grams: 5_000,
            reorder_point_grams: 2_000,
            supplier_name: "Fresh Fields".into(),
            last_delivery_timestamp: 1710403200,
            unit_cost_cents: 190,
        },
    ];

    let encoded = encode_to_vec(&inventory).expect("encode inventory batch");
    let (decoded, _): (Vec<InventoryItem>, _) =
        decode_from_slice(&encoded).expect("decode inventory batch");
    assert_eq!(inventory, decoded);

    for item in &decoded {
        if item.on_hand_grams < item.reorder_point_grams {
            assert!(item.par_level_grams > item.on_hand_grams);
        }
    }
}

#[test]
fn test_pos_transaction_cash() {
    let txn = PosTransaction {
        transaction_id: 900_001,
        timestamp: 1710510000,
        items_sold: vec![(101, 2), (202, 1), (305, 3)],
        subtotal_cents: 5_650,
        tax_cents: 492,
        tip_cents: 0,
        total_cents: 6_142,
        payment_method: PaymentMethod::Cash,
        customer_loyalty_id: None,
    };

    let encoded = encode_to_vec(&txn).expect("encode POS cash txn");
    let (decoded, _): (PosTransaction, _) =
        decode_from_slice(&encoded).expect("decode POS cash txn");
    assert_eq!(txn, decoded);
    assert_eq!(
        decoded.total_cents,
        decoded.subtotal_cents + decoded.tax_cents + decoded.tip_cents
    );
}

#[test]
fn test_pos_transaction_with_tip_and_loyalty_file() {
    let txn = PosTransaction {
        transaction_id: 900_042,
        timestamp: 1710513600,
        items_sold: vec![(101, 1), (410, 2)],
        subtotal_cents: 3_150,
        tax_cents: 274,
        tip_cents: 500,
        total_cents: 3_924,
        payment_method: PaymentMethod::CreditCard,
        customer_loyalty_id: Some("LOYAL-8832".into()),
    };

    let path = temp_dir().join("oxicode_fio45_pos_tip.bin");
    encode_to_file(&txn, &path).expect("encode_to_file POS with tip");
    let decoded: PosTransaction = decode_from_file(&path).expect("decode_from_file POS with tip");
    assert_eq!(txn, decoded);
    assert!(decoded.customer_loyalty_id.is_some());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_health_inspection_pass() {
    let inspection = HealthInspection {
        inspection_id: 7701,
        inspector_name: "Sandra Chen".into(),
        inspection_date: "2026-02-20".into(),
        truck_id: "TACO-FURY-01".into(),
        grade: InspectionGrade::Pass,
        violations: vec![],
        follow_up_date: None,
        overall_score: 96,
    };

    let encoded = encode_to_vec(&inspection).expect("encode inspection pass");
    let (decoded, _): (HealthInspection, _) =
        decode_from_slice(&encoded).expect("decode inspection pass");
    assert_eq!(inspection, decoded);
    assert!(decoded.violations.is_empty());
}

#[test]
fn test_health_inspection_with_violations_file() {
    let inspection = HealthInspection {
        inspection_id: 7788,
        inspector_name: "James Rivera".into(),
        inspection_date: "2026-03-10".into(),
        truck_id: "BURGER-BUS-03".into(),
        grade: InspectionGrade::ConditionalPass,
        violations: vec![
            InspectionViolation {
                code: "3-501.16".into(),
                description: "Cold holding temp above 41F for diced tomatoes".into(),
                is_critical: true,
                corrected_on_site: true,
            },
            InspectionViolation {
                code: "6-301.14".into(),
                description: "Hand wash signage missing at service window".into(),
                is_critical: false,
                corrected_on_site: true,
            },
        ],
        follow_up_date: Some("2026-03-24".into()),
        overall_score: 78,
    };

    let path = temp_dir().join("oxicode_fio45_inspection.bin");
    encode_to_file(&inspection, &path).expect("encode_to_file HealthInspection");
    let decoded: HealthInspection =
        decode_from_file(&path).expect("decode_from_file HealthInspection");
    assert_eq!(inspection, decoded);
    assert_eq!(decoded.violations.len(), 2);
    assert!(decoded.follow_up_date.is_some());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_commissary_kitchen_log() {
    let log = CommissaryLog {
        log_id: 44_001,
        kitchen_name: "Central Commissary West".into(),
        arrival_timestamp: 1710396000,
        departure_timestamp: 1710410400,
        items_prepped: vec![
            ("Bulgogi marinade batch".into(), 5_000),
            ("Pickled onions".into(), 2_000),
            ("Chimichurri sauce".into(), 1_500),
        ],
        items_loaded: vec![
            ("Marinated beef portions".into(), 40),
            ("Prepped tortilla packs".into(), 200),
            ("Sauce bottles".into(), 12),
        ],
        waste_disposed_kg: 8,
        cleaning_completed: true,
        inspector_sign_off: Some("K. Tanaka".into()),
    };

    let path = temp_dir().join("oxicode_fio45_commissary.bin");
    encode_to_file(&log, &path).expect("encode_to_file CommissaryLog");
    let decoded: CommissaryLog = decode_from_file(&path).expect("decode_from_file CommissaryLog");
    assert_eq!(log, decoded);
    assert!(decoded.cleaning_completed);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_propane_fuel_tracking() {
    let reading = FuelReading {
        reading_id: 601,
        fuel_type: FuelType::Propane,
        tank_capacity_liters_x10: 400,
        current_level_liters_x10: 185,
        consumption_rate_liters_per_hour_x10: 12,
        last_refill_timestamp: 1710316800,
        cost_per_liter_cents: 175,
        estimated_hours_remaining_x10: 154,
    };

    let encoded = encode_to_vec(&reading).expect("encode propane FuelReading");
    let (decoded, _): (FuelReading, _) =
        decode_from_slice(&encoded).expect("decode propane FuelReading");
    assert_eq!(reading, decoded);
    assert!(decoded.current_level_liters_x10 < decoded.tank_capacity_liters_x10);
}

#[test]
fn test_generator_diesel_fuel_file() {
    let reading = FuelReading {
        reading_id: 602,
        fuel_type: FuelType::Diesel,
        tank_capacity_liters_x10: 800,
        current_level_liters_x10: 620,
        consumption_rate_liters_per_hour_x10: 25,
        last_refill_timestamp: 1710403200,
        cost_per_liter_cents: 198,
        estimated_hours_remaining_x10: 248,
    };

    let path = temp_dir().join("oxicode_fio45_diesel.bin");
    encode_to_file(&reading, &path).expect("encode_to_file diesel FuelReading");
    let decoded: FuelReading =
        decode_from_file(&path).expect("decode_from_file diesel FuelReading");
    assert_eq!(reading, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_social_media_location_broadcast() {
    let broadcast = LocationBroadcast {
        broadcast_id: 330_001,
        platform: BroadcastPlatform::Instagram,
        timestamp: 1710504000,
        location: GpsCoordinate {
            latitude_microdeg: 37_774_900,
            longitude_microdeg: -122_419_400,
        },
        message: "We're parked at Financial District today! Try our new Korean BBQ Tacos!".into(),
        menu_specials: vec!["Korean BBQ Tacos".into(), "Spicy Pork Belly Fries".into()],
        estimated_wait_minutes: 12,
        likes_count: 247,
        shares_count: 38,
    };

    let encoded = encode_to_vec(&broadcast).expect("encode LocationBroadcast");
    let (decoded, _): (LocationBroadcast, _) =
        decode_from_slice(&encoded).expect("decode LocationBroadcast");
    assert_eq!(broadcast, decoded);
    assert!(!decoded.menu_specials.is_empty());
}

#[test]
fn test_catering_event_booking_file() {
    let booking = CateringBooking {
        booking_id: 88_001,
        client_name: "TechCorp Annual Picnic".into(),
        event_date: "2026-04-15".into(),
        event_location: "Golden Gate Park - Hellman Hollow".into(),
        guest_count: 250,
        menu_items: vec![101, 202, 305, 410, 512],
        status: BookingStatus::DepositPaid,
        deposit_cents: 250_000,
        total_quote_cents: 875_000,
        dietary_notes: vec![
            "15 guests vegetarian".into(),
            "5 guests nut allergy".into(),
            "3 guests gluten-free".into(),
        ],
        setup_time_minutes: 90,
        teardown_time_minutes: 60,
    };

    let path = temp_dir().join("oxicode_fio45_catering.bin");
    encode_to_file(&booking, &path).expect("encode_to_file CateringBooking");
    let decoded: CateringBooking =
        decode_from_file(&path).expect("decode_from_file CateringBooking");
    assert_eq!(booking, decoded);
    assert_eq!(decoded.dietary_notes.len(), 3);
    assert!(decoded.deposit_cents < decoded.total_quote_cents);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_weather_demand_forecast() {
    let forecast = DemandForecast {
        forecast_id: 1501,
        truck_id: "TACO-FURY-01".into(),
        date: "2026-03-16".into(),
        weather: WeatherForecastEntry {
            date: "2026-03-16".into(),
            condition: WeatherCondition::Sunny,
            high_temp_c_x10: 220,
            low_temp_c_x10: 140,
            precipitation_pct: 5,
            wind_speed_kph_x10: 85,
        },
        predicted_customers: 320,
        predicted_revenue_cents: 448_000,
        recommended_par_items: vec![
            ("Ground beef 80/20".into(), 24_000),
            ("Corn tortillas".into(), 640),
            ("Avocados".into(), 5_500),
        ],
        confidence_pct: 82,
    };

    let encoded = encode_to_vec(&forecast).expect("encode DemandForecast");
    let (decoded, _): (DemandForecast, _) =
        decode_from_slice(&encoded).expect("decode DemandForecast");
    assert_eq!(forecast, decoded);
    assert!(decoded.confidence_pct <= 100);
}

#[test]
fn test_rainy_day_demand_forecast_file() {
    let forecast = DemandForecast {
        forecast_id: 1502,
        truck_id: "TACO-FURY-01".into(),
        date: "2026-03-18".into(),
        weather: WeatherForecastEntry {
            date: "2026-03-18".into(),
            condition: WeatherCondition::HeavyRain,
            high_temp_c_x10: 110,
            low_temp_c_x10: 70,
            precipitation_pct: 90,
            wind_speed_kph_x10: 320,
        },
        predicted_customers: 85,
        predicted_revenue_cents: 119_000,
        recommended_par_items: vec![
            ("Ground beef 80/20".into(), 8_000),
            ("Corn tortillas".into(), 200),
        ],
        confidence_pct: 68,
    };

    let path = temp_dir().join("oxicode_fio45_forecast_rain.bin");
    encode_to_file(&forecast, &path).expect("encode_to_file rainy DemandForecast");
    let decoded: DemandForecast =
        decode_from_file(&path).expect("decode_from_file rainy DemandForecast");
    assert_eq!(forecast, decoded);
    assert!(decoded.predicted_customers < 100);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_staff_schedule_weekly() {
    let schedule = StaffSchedule {
        schedule_id: 2201,
        week_start_date: "2026-03-16".into(),
        truck_id: "TACO-FURY-01".into(),
        shifts: vec![
            StaffShift {
                employee_id: 1001,
                employee_name: "Maria Gonzalez".into(),
                role: ShiftRole::HeadChef,
                shift_date: "2026-03-16".into(),
                start_timestamp: 1710576000,
                end_timestamp: 1710612000,
                break_minutes: 30,
                hourly_rate_cents: 2800,
                overtime_minutes: 0,
            },
            StaffShift {
                employee_id: 1002,
                employee_name: "Jake Thompson".into(),
                role: ShiftRole::LineCook,
                shift_date: "2026-03-16".into(),
                start_timestamp: 1710576000,
                end_timestamp: 1710612000,
                break_minutes: 30,
                hourly_rate_cents: 2000,
                overtime_minutes: 60,
            },
            StaffShift {
                employee_id: 1003,
                employee_name: "Aisha Patel".into(),
                role: ShiftRole::CashierWindow,
                shift_date: "2026-03-16".into(),
                start_timestamp: 1710583200,
                end_timestamp: 1710612000,
                break_minutes: 15,
                hourly_rate_cents: 1800,
                overtime_minutes: 0,
            },
        ],
        total_labor_cost_cents: 58_800,
    };

    let path = temp_dir().join("oxicode_fio45_staff.bin");
    encode_to_file(&schedule, &path).expect("encode_to_file StaffSchedule");
    let decoded: StaffSchedule = decode_from_file(&path).expect("decode_from_file StaffSchedule");
    assert_eq!(schedule, decoded);
    assert_eq!(decoded.shifts.len(), 3);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_food_cost_calculations() {
    let records = vec![
        FoodCostRecord {
            record_id: 3001,
            menu_item_id: 101,
            menu_item_name: "Korean BBQ Tacos".into(),
            ingredient_costs_cents: vec![
                ("Bulgogi beef".into(), 320),
                ("Kimchi".into(), 55),
                ("Corn tortilla".into(), 30),
                ("Sriracha aioli".into(), 18),
            ],
            total_food_cost_cents: 423,
            selling_price_cents: 1250,
            margin_bps: 6616,
            portions_sold_today: 85,
        },
        FoodCostRecord {
            record_id: 3002,
            menu_item_id: 202,
            menu_item_name: "Jackfruit Carnitas Bowl".into(),
            ingredient_costs_cents: vec![
                ("Jackfruit".into(), 210),
                ("Jasmine rice".into(), 22),
                ("Black beans".into(), 18),
            ],
            total_food_cost_cents: 250,
            selling_price_cents: 1400,
            margin_bps: 8214,
            portions_sold_today: 42,
        },
    ];

    let encoded = encode_to_vec(&records).expect("encode FoodCostRecord batch");
    let (decoded, _): (Vec<FoodCostRecord>, _) =
        decode_from_slice(&encoded).expect("decode FoodCostRecord batch");
    assert_eq!(records, decoded);

    for record in &decoded {
        assert!(record.total_food_cost_cents < record.selling_price_cents);
    }
}

#[test]
fn test_customer_loyalty_stamp_card_file() {
    let card = LoyaltyStampCard {
        card_id: 55_001,
        customer_name: "Alex Kim".into(),
        phone_hash: 0xDEAD_BEEF_CAFE_1234,
        stamps_collected: 7,
        stamps_for_reward: 10,
        rewards_redeemed: 3,
        total_visits: 37,
        total_spent_cents: 518_400,
        last_visit_timestamp: 1710510000,
        favorite_item_ids: vec![101, 305],
    };

    let path = temp_dir().join("oxicode_fio45_loyalty.bin");
    encode_to_file(&card, &path).expect("encode_to_file LoyaltyStampCard");
    let decoded: LoyaltyStampCard =
        decode_from_file(&path).expect("decode_from_file LoyaltyStampCard");
    assert_eq!(card, decoded);
    assert!(decoded.stamps_collected < decoded.stamps_for_reward);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_festival_application() {
    let application = FestivalApplication {
        application_id: 66_001,
        festival_name: "Bay Area Street Food Festival 2026".into(),
        festival_dates: vec![
            "2026-05-23".into(),
            "2026-05-24".into(),
            "2026-05-25".into(),
        ],
        location_name: "Jack London Square, Oakland".into(),
        booth_fee_cents: 350_000,
        electricity_provided: true,
        water_hookup: true,
        expected_attendance: 45_000,
        menu_submitted: vec![
            "Korean BBQ Tacos".into(),
            "Jackfruit Carnitas Bowl".into(),
            "Spicy Pork Belly Fries".into(),
            "Mango Chili Paletas".into(),
        ],
        insurance_policy_number: "CGL-2026-FT-991123".into(),
        status: BookingStatus::Tentative,
    };

    let encoded = encode_to_vec(&application).expect("encode FestivalApplication");
    let (decoded, _): (FestivalApplication, _) =
        decode_from_slice(&encoded).expect("decode FestivalApplication");
    assert_eq!(application, decoded);
    assert_eq!(decoded.festival_dates.len(), 3);
    assert!(decoded.electricity_provided);
}

#[test]
fn test_equipment_maintenance_fryer_file() {
    let log = EquipmentMaintenanceLog {
        log_id: 77_001,
        equipment: EquipmentKind::DeepFryer,
        equipment_serial: "FRY-PF-4422".into(),
        truck_id: "TACO-FURY-01".into(),
        last_service_date: "2026-02-01".into(),
        next_service_date: "2026-04-01".into(),
        urgency: MaintenanceUrgency::Routine,
        notes: "Oil changed, thermostat calibrated, basket replaced".into(),
        service_cost_cents: 28_500,
        parts_replaced: vec!["Fry basket".into(), "Thermostat probe".into()],
        technician_name: Some("Robert Diaz".into()),
    };

    let path = temp_dir().join("oxicode_fio45_maint_fryer.bin");
    encode_to_file(&log, &path).expect("encode_to_file fryer EquipmentMaintenanceLog");
    let decoded: EquipmentMaintenanceLog =
        decode_from_file(&path).expect("decode_from_file fryer EquipmentMaintenanceLog");
    assert_eq!(log, decoded);
    assert_eq!(decoded.parts_replaced.len(), 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_equipment_maintenance_generator_urgent() {
    let log = EquipmentMaintenanceLog {
        log_id: 77_042,
        equipment: EquipmentKind::Generator,
        equipment_serial: "GEN-HND-EU7000".into(),
        truck_id: "BURGER-BUS-03".into(),
        last_service_date: "2025-11-15".into(),
        next_service_date: "2026-02-15".into(),
        urgency: MaintenanceUrgency::Urgent,
        notes: "Overdue for service, intermittent voltage fluctuations reported by POS terminal"
            .into(),
        service_cost_cents: 0,
        parts_replaced: vec![],
        technician_name: None,
    };

    let encoded = encode_to_vec(&log).expect("encode urgent generator maintenance");
    let (decoded, _): (EquipmentMaintenanceLog, _) =
        decode_from_slice(&encoded).expect("decode urgent generator maintenance");
    assert_eq!(log, decoded);
    assert!(decoded.technician_name.is_none());
}

#[test]
fn test_full_day_operations_combined_file() {
    let menu = vec![MenuItem {
        item_id: 101,
        name: "Korean BBQ Tacos".into(),
        description: "Bulgogi tacos".into(),
        price_cents: 1250,
        ingredients: vec![Ingredient {
            name: "Beef".into(),
            quantity_grams: 150,
            cost_cents: 320,
            is_perishable: true,
            shelf_life_hours: 48,
        }],
        allergens: vec![AllergenCategory::Soy],
        prep_time_seconds: 240,
        complexity: PrepComplexity::Moderate,
        is_vegan: false,
        is_gluten_free: false,
        calories: 480,
    }];

    let transactions = vec![
        PosTransaction {
            transaction_id: 900_100,
            timestamp: 1710504000,
            items_sold: vec![(101, 2)],
            subtotal_cents: 2500,
            tax_cents: 218,
            tip_cents: 400,
            total_cents: 3118,
            payment_method: PaymentMethod::MobileWallet,
            customer_loyalty_id: Some("LOYAL-1122".into()),
        },
        PosTransaction {
            transaction_id: 900_101,
            timestamp: 1710504300,
            items_sold: vec![(101, 1)],
            subtotal_cents: 1250,
            tax_cents: 109,
            tip_cents: 0,
            total_cents: 1359,
            payment_method: PaymentMethod::Cash,
            customer_loyalty_id: None,
        },
    ];

    let fuel = FuelReading {
        reading_id: 603,
        fuel_type: FuelType::Propane,
        tank_capacity_liters_x10: 400,
        current_level_liters_x10: 310,
        consumption_rate_liters_per_hour_x10: 12,
        last_refill_timestamp: 1710403200,
        cost_per_liter_cents: 175,
        estimated_hours_remaining_x10: 258,
    };

    let combined: (Vec<MenuItem>, Vec<PosTransaction>, FuelReading) =
        (menu.clone(), transactions.clone(), fuel.clone());

    let path = temp_dir().join("oxicode_fio45_full_day.bin");
    encode_to_file(&combined, &path).expect("encode_to_file full day operations");
    let decoded: (Vec<MenuItem>, Vec<PosTransaction>, FuelReading) =
        decode_from_file(&path).expect("decode_from_file full day operations");

    assert_eq!(decoded.0, menu);
    assert_eq!(decoded.1, transactions);
    assert_eq!(decoded.2, fuel);
    assert_eq!(decoded.1.len(), 2);

    let total_revenue: u32 = decoded.1.iter().map(|t| t.total_cents).sum();
    assert_eq!(total_revenue, 4477);
    let _ = std::fs::remove_file(&path);
}
