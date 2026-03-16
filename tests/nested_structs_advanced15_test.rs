//! Advanced nested struct encoding tests for OxiCode (set 15)
//! Theme: Theme park operations and ride management systems

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

// ─── Ride configuration with vehicle specs and track segments ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct TrackSegment {
    segment_id: u32,
    segment_type: String,
    length_meters: f64,
    incline_degrees: f32,
    max_speed_kmh: f32,
    has_inversion: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RideVehicle {
    vehicle_id: u32,
    name: String,
    capacity: u16,
    weight_kg: f64,
    safety_harness_type: String,
    last_inspection_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RideConfiguration {
    ride_id: u64,
    ride_name: String,
    category: String,
    height_requirement_cm: u16,
    vehicles: Vec<RideVehicle>,
    track: Vec<TrackSegment>,
    total_track_length_meters: f64,
    max_g_force: f32,
    ride_duration_seconds: u32,
    opened_year: u16,
}

// ─── Queue management with virtual queue slots ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct VirtualQueueSlot {
    slot_id: u64,
    guest_id: u64,
    estimated_wait_minutes: u16,
    window_start_epoch: u64,
    window_end_epoch: u64,
    redeemed: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct QueueLane {
    lane_id: u32,
    lane_type: String,
    current_wait_minutes: u16,
    capacity: u32,
    slots: Vec<VirtualQueueSlot>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct QueueManagement {
    ride_id: u64,
    ride_name: String,
    lanes: Vec<QueueLane>,
    total_guests_in_queue: u32,
    is_operational: bool,
}

// ─── Show schedule hierarchies ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct Performer {
    performer_id: u32,
    name: String,
    role: String,
    is_understudy: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Performance {
    performance_id: u64,
    start_epoch: u64,
    duration_minutes: u16,
    venue: String,
    performers: Vec<Performer>,
    sold_out: bool,
    attendance: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DaySchedule {
    date_str: String,
    performances: Vec<Performance>,
    park_open_epoch: u64,
    park_close_epoch: u64,
    weather_forecast: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SeasonSchedule {
    season_name: String,
    start_date: String,
    end_date: String,
    days: Vec<DaySchedule>,
    total_performances: u32,
}

// ─── Food & beverage outlet menus with allergen data ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct Allergen {
    code: String,
    name: String,
    severity: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NutritionInfo {
    calories: u32,
    protein_grams: f32,
    carbs_grams: f32,
    fat_grams: f32,
    sodium_mg: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MenuItem {
    item_id: u32,
    name: String,
    description: String,
    price_cents: u32,
    allergens: Vec<Allergen>,
    nutrition: NutritionInfo,
    available: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MenuCategory {
    category_name: String,
    items: Vec<MenuItem>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FoodOutlet {
    outlet_id: u32,
    name: String,
    location_zone: String,
    categories: Vec<MenuCategory>,
    open_epoch: u64,
    close_epoch: u64,
}

// ─── Hotel room inventories with rate calendars ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct NightlyRate {
    date_str: String,
    base_rate_cents: u32,
    tax_cents: u32,
    resort_fee_cents: u32,
    available_rooms: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RoomAmenity {
    amenity_name: String,
    included: bool,
    surcharge_cents: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RoomType {
    type_id: u32,
    name: String,
    max_occupancy: u8,
    bed_config: String,
    square_meters: f32,
    amenities: Vec<RoomAmenity>,
    rate_calendar: Vec<NightlyRate>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct HotelInventory {
    hotel_id: u32,
    hotel_name: String,
    star_rating: u8,
    room_types: Vec<RoomType>,
    total_rooms: u16,
}

// ─── Guest experience scores with ride-by-ride breakdowns ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct RideScore {
    ride_id: u64,
    ride_name: String,
    overall_score: f32,
    thrill_score: f32,
    queue_experience_score: f32,
    cleanliness_score: f32,
    staff_friendliness_score: f32,
    comment: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DiningScore {
    outlet_id: u32,
    outlet_name: String,
    food_quality_score: f32,
    service_score: f32,
    value_score: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GuestExperienceReport {
    guest_id: u64,
    visit_date: String,
    overall_satisfaction: f32,
    ride_scores: Vec<RideScore>,
    dining_scores: Vec<DiningScore>,
    would_recommend: bool,
    net_promoter_score: i8,
}

// ─── Maintenance work orders with parts lists ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct SparePart {
    part_number: String,
    description: String,
    quantity_needed: u16,
    unit_cost_cents: u32,
    lead_time_days: u16,
    in_stock: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MaintenanceTask {
    task_id: u32,
    description: String,
    estimated_hours: f32,
    priority: u8,
    parts: Vec<SparePart>,
    requires_ride_shutdown: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WorkOrder {
    order_id: u64,
    ride_id: u64,
    ride_name: String,
    created_epoch: u64,
    due_epoch: u64,
    tasks: Vec<MaintenanceTask>,
    assigned_technician: String,
    status: String,
    total_estimated_cost_cents: u64,
}

// ─── Fireworks choreography sequences ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct PyroEffect {
    effect_type: String,
    color_primary: String,
    color_secondary: Option<String>,
    altitude_meters: f32,
    spread_degrees: f32,
    duration_ms: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LaunchCue {
    cue_id: u32,
    timestamp_ms: u64,
    launcher_id: u16,
    angle_degrees: f32,
    effects: Vec<PyroEffect>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MusicSync {
    track_name: String,
    bpm: f32,
    beat_markers_ms: Vec<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FireworksShow {
    show_id: u64,
    show_name: String,
    total_duration_ms: u64,
    cues: Vec<LaunchCue>,
    music: MusicSync,
    total_shells: u32,
    safety_radius_meters: f32,
}

// ─── Character meet-and-greet schedules ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct CharacterCostume {
    costume_id: u32,
    variant: String,
    seasonal: bool,
    condition_rating: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MeetGreetSession {
    session_id: u64,
    start_epoch: u64,
    end_epoch: u64,
    max_guests: u16,
    photo_pass_available: bool,
    autograph_available: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CharacterSchedule {
    character_name: String,
    zone: String,
    costumes: Vec<CharacterCostume>,
    sessions: Vec<MeetGreetSession>,
    popularity_rank: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MeetGreetPlan {
    date_str: String,
    park_section: String,
    characters: Vec<CharacterSchedule>,
    total_sessions: u32,
}

// ─── Ticket pricing tiers with date-based dynamic pricing ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct DatePricePoint {
    date_str: String,
    demand_tier: String,
    adult_price_cents: u32,
    child_price_cents: u32,
    senior_price_cents: u32,
    capacity_percentage: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AddOn {
    addon_id: u32,
    name: String,
    price_cents: u32,
    description: String,
    limited_quantity: Option<u32>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TicketTier {
    tier_id: u32,
    tier_name: String,
    includes_parks: Vec<String>,
    date_prices: Vec<DatePricePoint>,
    available_addons: Vec<AddOn>,
    max_days: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PricingPlan {
    plan_id: u64,
    season: String,
    tiers: Vec<TicketTier>,
    currency: String,
    tax_rate_percent: f32,
}

// ─── Parade float configurations ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct FloatLighting {
    zone_name: String,
    led_count: u32,
    color_sequence: Vec<String>,
    animation_pattern: String,
    power_watts: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FloatAudio {
    speaker_count: u8,
    track_name: String,
    volume_db: f32,
    sync_offset_ms: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ParadeFloat {
    float_id: u32,
    theme: String,
    length_meters: f32,
    weight_kg: f64,
    performer_count: u8,
    lighting: Vec<FloatLighting>,
    audio: FloatAudio,
    max_speed_kmh: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ParadeConfiguration {
    parade_id: u64,
    parade_name: String,
    route_name: String,
    floats: Vec<ParadeFloat>,
    total_duration_minutes: u16,
    start_epoch: u64,
}

// ─── Water ride splash zone modeling ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct SplashZone {
    zone_id: u32,
    distance_from_ride_meters: f32,
    splash_probability: f32,
    avg_water_volume_liters: f32,
    spectator_area: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WaterChannel {
    channel_id: u32,
    width_meters: f32,
    depth_meters: f32,
    flow_rate_lps: f32,
    temperature_celsius: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DropProfile {
    drop_id: u32,
    height_meters: f32,
    angle_degrees: f32,
    entry_speed_kmh: f32,
    splash_height_meters: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WaterRide {
    ride_id: u64,
    ride_name: String,
    channels: Vec<WaterChannel>,
    drops: Vec<DropProfile>,
    splash_zones: Vec<SplashZone>,
    total_water_volume_liters: f64,
    recirculation_rate_percent: f32,
}

// ─── Park-wide energy management ───

#[derive(Debug, PartialEq, Encode, Decode)]
struct PowerConsumer {
    asset_id: u64,
    asset_name: String,
    asset_type: String,
    peak_watts: f64,
    avg_watts: f64,
    zone: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EnergyReading {
    timestamp_epoch: u64,
    kwh_consumed: f64,
    peak_demand_kw: f64,
    solar_generated_kwh: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EnergyZone {
    zone_name: String,
    consumers: Vec<PowerConsumer>,
    readings: Vec<EnergyReading>,
    transformer_capacity_kva: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ParkEnergyPlan {
    plan_id: u64,
    date_str: String,
    zones: Vec<EnergyZone>,
    total_budget_kwh: f64,
    renewable_target_percent: f32,
}

// ═══════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════

// Test 1: Ride configuration with track segments and vehicles
#[test]
fn test_ride_configuration_full_coaster() {
    let config = RideConfiguration {
        ride_id: 1001,
        ride_name: "Thunder Mountain Express".to_string(),
        category: "roller_coaster".to_string(),
        height_requirement_cm: 120,
        vehicles: vec![
            RideVehicle {
                vehicle_id: 1,
                name: "Train Alpha".to_string(),
                capacity: 24,
                weight_kg: 3200.0,
                safety_harness_type: "over_shoulder_lap".to_string(),
                last_inspection_epoch: 1710000000,
            },
            RideVehicle {
                vehicle_id: 2,
                name: "Train Beta".to_string(),
                capacity: 24,
                weight_kg: 3180.0,
                safety_harness_type: "over_shoulder_lap".to_string(),
                last_inspection_epoch: 1710050000,
            },
        ],
        track: vec![
            TrackSegment {
                segment_id: 1,
                segment_type: "lift_hill".to_string(),
                length_meters: 85.0,
                incline_degrees: 45.0,
                max_speed_kmh: 12.0,
                has_inversion: false,
            },
            TrackSegment {
                segment_id: 2,
                segment_type: "first_drop".to_string(),
                length_meters: 92.0,
                incline_degrees: -78.0,
                max_speed_kmh: 105.0,
                has_inversion: false,
            },
            TrackSegment {
                segment_id: 3,
                segment_type: "corkscrew".to_string(),
                length_meters: 40.0,
                incline_degrees: 0.0,
                max_speed_kmh: 85.0,
                has_inversion: true,
            },
        ],
        total_track_length_meters: 1250.0,
        max_g_force: 4.2,
        ride_duration_seconds: 150,
        opened_year: 2019,
    };

    let bytes = encode_to_vec(&config).expect("encode ride configuration");
    let (decoded, _): (RideConfiguration, usize) =
        decode_from_slice(&bytes).expect("decode ride configuration");
    assert_eq!(config, decoded);
}

// Test 2: Virtual queue management with multiple lanes
#[test]
fn test_queue_management_virtual_slots() {
    let queue = QueueManagement {
        ride_id: 2001,
        ride_name: "Galaxy Voyager".to_string(),
        lanes: vec![
            QueueLane {
                lane_id: 1,
                lane_type: "standby".to_string(),
                current_wait_minutes: 65,
                capacity: 800,
                slots: vec![],
            },
            QueueLane {
                lane_id: 2,
                lane_type: "virtual_queue".to_string(),
                current_wait_minutes: 15,
                capacity: 200,
                slots: vec![
                    VirtualQueueSlot {
                        slot_id: 50001,
                        guest_id: 990001,
                        estimated_wait_minutes: 10,
                        window_start_epoch: 1710072000,
                        window_end_epoch: 1710075600,
                        redeemed: false,
                    },
                    VirtualQueueSlot {
                        slot_id: 50002,
                        guest_id: 990002,
                        estimated_wait_minutes: 25,
                        window_start_epoch: 1710075600,
                        window_end_epoch: 1710079200,
                        redeemed: true,
                    },
                ],
            },
            QueueLane {
                lane_id: 3,
                lane_type: "disability_access".to_string(),
                current_wait_minutes: 20,
                capacity: 50,
                slots: vec![VirtualQueueSlot {
                    slot_id: 60001,
                    guest_id: 880001,
                    estimated_wait_minutes: 5,
                    window_start_epoch: 1710070000,
                    window_end_epoch: 1710073600,
                    redeemed: false,
                }],
            },
        ],
        total_guests_in_queue: 412,
        is_operational: true,
    };

    let bytes = encode_to_vec(&queue).expect("encode queue management");
    let (decoded, _): (QueueManagement, usize) =
        decode_from_slice(&bytes).expect("decode queue management");
    assert_eq!(queue, decoded);
}

// Test 3: Season → day → performance show schedule
#[test]
fn test_show_schedule_season_hierarchy() {
    let season = SeasonSchedule {
        season_name: "Halloween Spectacular".to_string(),
        start_date: "2025-09-15".to_string(),
        end_date: "2025-11-02".to_string(),
        days: vec![
            DaySchedule {
                date_str: "2025-10-31".to_string(),
                performances: vec![
                    Performance {
                        performance_id: 7001,
                        start_epoch: 1730380800,
                        duration_minutes: 45,
                        venue: "Castle Stage".to_string(),
                        performers: vec![
                            Performer {
                                performer_id: 101,
                                name: "Elena Vasquez".to_string(),
                                role: "Witch Queen".to_string(),
                                is_understudy: false,
                            },
                            Performer {
                                performer_id: 102,
                                name: "Marcus Chen".to_string(),
                                role: "Ghost Knight".to_string(),
                                is_understudy: false,
                            },
                        ],
                        sold_out: true,
                        attendance: 1200,
                    },
                    Performance {
                        performance_id: 7002,
                        start_epoch: 1730390600,
                        duration_minutes: 30,
                        venue: "Lakeside Amphitheater".to_string(),
                        performers: vec![Performer {
                            performer_id: 201,
                            name: "Yuki Tanaka".to_string(),
                            role: "Pumpkin Spirit".to_string(),
                            is_understudy: true,
                        }],
                        sold_out: false,
                        attendance: 750,
                    },
                ],
                park_open_epoch: 1730340000,
                park_close_epoch: 1730404800,
                weather_forecast: "partly_cloudy_15c".to_string(),
            },
            DaySchedule {
                date_str: "2025-11-01".to_string(),
                performances: vec![Performance {
                    performance_id: 7003,
                    start_epoch: 1730467200,
                    duration_minutes: 60,
                    venue: "Grand Theater".to_string(),
                    performers: vec![
                        Performer {
                            performer_id: 101,
                            name: "Elena Vasquez".to_string(),
                            role: "Witch Queen".to_string(),
                            is_understudy: false,
                        },
                        Performer {
                            performer_id: 301,
                            name: "Rajesh Patel".to_string(),
                            role: "Skeleton King".to_string(),
                            is_understudy: false,
                        },
                    ],
                    sold_out: true,
                    attendance: 2000,
                }],
                park_open_epoch: 1730426400,
                park_close_epoch: 1730491200,
                weather_forecast: "sunny_18c".to_string(),
            },
        ],
        total_performances: 3,
    };

    let bytes = encode_to_vec(&season).expect("encode season schedule");
    let (decoded, _): (SeasonSchedule, usize) =
        decode_from_slice(&bytes).expect("decode season schedule");
    assert_eq!(season, decoded);
}

// Test 4: Food outlet menu with allergens and nutrition
#[test]
fn test_food_outlet_full_menu() {
    let outlet = FoodOutlet {
        outlet_id: 401,
        name: "Dragon's Feast Hall".to_string(),
        location_zone: "Fantasy Kingdom".to_string(),
        categories: vec![
            MenuCategory {
                category_name: "Entrees".to_string(),
                items: vec![
                    MenuItem {
                        item_id: 1001,
                        name: "Dragon Fire Burger".to_string(),
                        description: "Smoked beef patty with ghost pepper aioli".to_string(),
                        price_cents: 1895,
                        allergens: vec![
                            Allergen {
                                code: "GLU".to_string(),
                                name: "Gluten".to_string(),
                                severity: "contains".to_string(),
                            },
                            Allergen {
                                code: "DAI".to_string(),
                                name: "Dairy".to_string(),
                                severity: "contains".to_string(),
                            },
                        ],
                        nutrition: NutritionInfo {
                            calories: 850,
                            protein_grams: 42.0,
                            carbs_grams: 55.0,
                            fat_grams: 48.0,
                            sodium_mg: 1200,
                        },
                        available: true,
                    },
                    MenuItem {
                        item_id: 1002,
                        name: "Enchanted Forest Salad".to_string(),
                        description: "Mixed greens with candied walnuts and feta".to_string(),
                        price_cents: 1495,
                        allergens: vec![
                            Allergen {
                                code: "NUT".to_string(),
                                name: "Tree Nuts".to_string(),
                                severity: "contains".to_string(),
                            },
                            Allergen {
                                code: "DAI".to_string(),
                                name: "Dairy".to_string(),
                                severity: "may_contain".to_string(),
                            },
                        ],
                        nutrition: NutritionInfo {
                            calories: 320,
                            protein_grams: 12.0,
                            carbs_grams: 28.0,
                            fat_grams: 18.0,
                            sodium_mg: 450,
                        },
                        available: true,
                    },
                ],
            },
            MenuCategory {
                category_name: "Desserts".to_string(),
                items: vec![MenuItem {
                    item_id: 2001,
                    name: "Lava Cake".to_string(),
                    description: "Molten chocolate center with vanilla ice cream".to_string(),
                    price_cents: 995,
                    allergens: vec![
                        Allergen {
                            code: "GLU".to_string(),
                            name: "Gluten".to_string(),
                            severity: "contains".to_string(),
                        },
                        Allergen {
                            code: "EGG".to_string(),
                            name: "Eggs".to_string(),
                            severity: "contains".to_string(),
                        },
                        Allergen {
                            code: "DAI".to_string(),
                            name: "Dairy".to_string(),
                            severity: "contains".to_string(),
                        },
                    ],
                    nutrition: NutritionInfo {
                        calories: 620,
                        protein_grams: 8.0,
                        carbs_grams: 72.0,
                        fat_grams: 34.0,
                        sodium_mg: 280,
                    },
                    available: false,
                }],
            },
        ],
        open_epoch: 1710064800,
        close_epoch: 1710100800,
    };

    let bytes = encode_to_vec(&outlet).expect("encode food outlet");
    let (decoded, _): (FoodOutlet, usize) = decode_from_slice(&bytes).expect("decode food outlet");
    assert_eq!(outlet, decoded);
}

// Test 5: Hotel room inventory with rate calendar
#[test]
fn test_hotel_inventory_rate_calendar() {
    let hotel = HotelInventory {
        hotel_id: 501,
        hotel_name: "Enchanted Towers Resort".to_string(),
        star_rating: 4,
        room_types: vec![
            RoomType {
                type_id: 1,
                name: "Standard Kingdom View".to_string(),
                max_occupancy: 4,
                bed_config: "1_king_1_sofa".to_string(),
                square_meters: 32.5,
                amenities: vec![
                    RoomAmenity {
                        amenity_name: "Mini Fridge".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                    RoomAmenity {
                        amenity_name: "Balcony".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                ],
                rate_calendar: vec![
                    NightlyRate {
                        date_str: "2025-12-24".to_string(),
                        base_rate_cents: 45000,
                        tax_cents: 5850,
                        resort_fee_cents: 3500,
                        available_rooms: 3,
                    },
                    NightlyRate {
                        date_str: "2025-12-25".to_string(),
                        base_rate_cents: 55000,
                        tax_cents: 7150,
                        resort_fee_cents: 3500,
                        available_rooms: 0,
                    },
                ],
            },
            RoomType {
                type_id: 2,
                name: "Royal Suite".to_string(),
                max_occupancy: 6,
                bed_config: "2_king_1_pullout".to_string(),
                square_meters: 85.0,
                amenities: vec![
                    RoomAmenity {
                        amenity_name: "Jacuzzi".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                    RoomAmenity {
                        amenity_name: "Butler Service".to_string(),
                        included: false,
                        surcharge_cents: 15000,
                    },
                    RoomAmenity {
                        amenity_name: "Park View Terrace".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                ],
                rate_calendar: vec![NightlyRate {
                    date_str: "2025-12-24".to_string(),
                    base_rate_cents: 120000,
                    tax_cents: 15600,
                    resort_fee_cents: 3500,
                    available_rooms: 1,
                }],
            },
        ],
        total_rooms: 650,
    };

    let bytes = encode_to_vec(&hotel).expect("encode hotel inventory");
    let (decoded, _): (HotelInventory, usize) =
        decode_from_slice(&bytes).expect("decode hotel inventory");
    assert_eq!(hotel, decoded);
}

// Test 6: Guest experience report with ride-by-ride scores
#[test]
fn test_guest_experience_report_detailed() {
    let report = GuestExperienceReport {
        guest_id: 770001,
        visit_date: "2025-07-14".to_string(),
        overall_satisfaction: 8.7,
        ride_scores: vec![
            RideScore {
                ride_id: 1001,
                ride_name: "Thunder Mountain Express".to_string(),
                overall_score: 9.2,
                thrill_score: 9.5,
                queue_experience_score: 6.0,
                cleanliness_score: 8.5,
                staff_friendliness_score: 9.0,
                comment: Some("Best coaster in the park, but queue was long".to_string()),
            },
            RideScore {
                ride_id: 2001,
                ride_name: "Galaxy Voyager".to_string(),
                overall_score: 8.8,
                thrill_score: 7.5,
                queue_experience_score: 9.0,
                cleanliness_score: 9.5,
                staff_friendliness_score: 8.0,
                comment: None,
            },
            RideScore {
                ride_id: 3001,
                ride_name: "Tiny Town Carousel".to_string(),
                overall_score: 7.0,
                thrill_score: 2.0,
                queue_experience_score: 8.5,
                cleanliness_score: 9.0,
                staff_friendliness_score: 9.5,
                comment: Some("Great for kids".to_string()),
            },
        ],
        dining_scores: vec![
            DiningScore {
                outlet_id: 401,
                outlet_name: "Dragon's Feast Hall".to_string(),
                food_quality_score: 8.0,
                service_score: 7.5,
                value_score: 5.5,
            },
            DiningScore {
                outlet_id: 402,
                outlet_name: "Cosmic Cantina".to_string(),
                food_quality_score: 7.0,
                service_score: 8.0,
                value_score: 6.0,
            },
        ],
        would_recommend: true,
        net_promoter_score: 9,
    };

    let bytes = encode_to_vec(&report).expect("encode guest experience report");
    let (decoded, _): (GuestExperienceReport, usize) =
        decode_from_slice(&bytes).expect("decode guest experience report");
    assert_eq!(report, decoded);
}

// Test 7: Maintenance work order with parts lists
#[test]
fn test_maintenance_work_order_complex() {
    let order = WorkOrder {
        order_id: 88001,
        ride_id: 1001,
        ride_name: "Thunder Mountain Express".to_string(),
        created_epoch: 1710000000,
        due_epoch: 1710172800,
        tasks: vec![
            MaintenanceTask {
                task_id: 1,
                description: "Replace worn brake pads on sections 3-5".to_string(),
                estimated_hours: 6.0,
                priority: 1,
                parts: vec![
                    SparePart {
                        part_number: "BP-4420-HT".to_string(),
                        description: "High-temp ceramic brake pad".to_string(),
                        quantity_needed: 12,
                        unit_cost_cents: 34500,
                        lead_time_days: 3,
                        in_stock: true,
                    },
                    SparePart {
                        part_number: "BP-BOLT-M12".to_string(),
                        description: "M12 mounting bolt stainless".to_string(),
                        quantity_needed: 48,
                        unit_cost_cents: 250,
                        lead_time_days: 1,
                        in_stock: true,
                    },
                ],
                requires_ride_shutdown: true,
            },
            MaintenanceTask {
                task_id: 2,
                description: "Lubricate chain lift mechanism".to_string(),
                estimated_hours: 2.5,
                priority: 2,
                parts: vec![SparePart {
                    part_number: "LUB-SYNTH-20L".to_string(),
                    description: "Synthetic chain lubricant 20L drum".to_string(),
                    quantity_needed: 2,
                    unit_cost_cents: 18900,
                    lead_time_days: 0,
                    in_stock: true,
                }],
                requires_ride_shutdown: true,
            },
            MaintenanceTask {
                task_id: 3,
                description: "Inspect and calibrate proximity sensors".to_string(),
                estimated_hours: 4.0,
                priority: 1,
                parts: vec![],
                requires_ride_shutdown: true,
            },
        ],
        assigned_technician: "James O'Sullivan".to_string(),
        status: "in_progress".to_string(),
        total_estimated_cost_cents: 463800,
    };

    let bytes = encode_to_vec(&order).expect("encode work order");
    let (decoded, _): (WorkOrder, usize) = decode_from_slice(&bytes).expect("decode work order");
    assert_eq!(order, decoded);
}

// Test 8: Fireworks choreography with music sync
#[test]
fn test_fireworks_choreography_full_show() {
    let show = FireworksShow {
        show_id: 9001,
        show_name: "Starlight Symphony".to_string(),
        total_duration_ms: 720000,
        cues: vec![
            LaunchCue {
                cue_id: 1,
                timestamp_ms: 0,
                launcher_id: 1,
                angle_degrees: 85.0,
                effects: vec![
                    PyroEffect {
                        effect_type: "peony".to_string(),
                        color_primary: "gold".to_string(),
                        color_secondary: Some("red".to_string()),
                        altitude_meters: 120.0,
                        spread_degrees: 360.0,
                        duration_ms: 3000,
                    },
                    PyroEffect {
                        effect_type: "crackle".to_string(),
                        color_primary: "silver".to_string(),
                        color_secondary: None,
                        altitude_meters: 100.0,
                        spread_degrees: 180.0,
                        duration_ms: 2000,
                    },
                ],
            },
            LaunchCue {
                cue_id: 2,
                timestamp_ms: 4500,
                launcher_id: 3,
                angle_degrees: 75.0,
                effects: vec![PyroEffect {
                    effect_type: "chrysanthemum".to_string(),
                    color_primary: "blue".to_string(),
                    color_secondary: Some("white".to_string()),
                    altitude_meters: 150.0,
                    spread_degrees: 360.0,
                    duration_ms: 4000,
                }],
            },
            LaunchCue {
                cue_id: 3,
                timestamp_ms: 10000,
                launcher_id: 2,
                angle_degrees: 90.0,
                effects: vec![
                    PyroEffect {
                        effect_type: "willow".to_string(),
                        color_primary: "green".to_string(),
                        color_secondary: Some("gold".to_string()),
                        altitude_meters: 130.0,
                        spread_degrees: 270.0,
                        duration_ms: 5000,
                    },
                    PyroEffect {
                        effect_type: "strobe".to_string(),
                        color_primary: "white".to_string(),
                        color_secondary: None,
                        altitude_meters: 80.0,
                        spread_degrees: 90.0,
                        duration_ms: 1500,
                    },
                    PyroEffect {
                        effect_type: "palm".to_string(),
                        color_primary: "orange".to_string(),
                        color_secondary: Some("yellow".to_string()),
                        altitude_meters: 110.0,
                        spread_degrees: 200.0,
                        duration_ms: 3500,
                    },
                ],
            },
        ],
        music: MusicSync {
            track_name: "Starlight Overture - Park Orchestra".to_string(),
            bpm: 128.0,
            beat_markers_ms: vec![0, 468, 937, 1406, 1875, 2343, 2812, 3281, 3750],
        },
        total_shells: 450,
        safety_radius_meters: 250.0,
    };

    let bytes = encode_to_vec(&show).expect("encode fireworks show");
    let (decoded, _): (FireworksShow, usize) =
        decode_from_slice(&bytes).expect("decode fireworks show");
    assert_eq!(show, decoded);
}

// Test 9: Character meet-and-greet daily plan
#[test]
fn test_character_meet_greet_plan() {
    let plan = MeetGreetPlan {
        date_str: "2025-08-10".to_string(),
        park_section: "Fairytale Village".to_string(),
        characters: vec![
            CharacterSchedule {
                character_name: "Princess Aurora".to_string(),
                zone: "Royal Courtyard".to_string(),
                costumes: vec![
                    CharacterCostume {
                        costume_id: 101,
                        variant: "classic_pink".to_string(),
                        seasonal: false,
                        condition_rating: 9,
                    },
                    CharacterCostume {
                        costume_id: 102,
                        variant: "summer_garden".to_string(),
                        seasonal: true,
                        condition_rating: 10,
                    },
                ],
                sessions: vec![
                    MeetGreetSession {
                        session_id: 30001,
                        start_epoch: 1723276800,
                        end_epoch: 1723280400,
                        max_guests: 60,
                        photo_pass_available: true,
                        autograph_available: true,
                    },
                    MeetGreetSession {
                        session_id: 30002,
                        start_epoch: 1723291200,
                        end_epoch: 1723294800,
                        max_guests: 60,
                        photo_pass_available: true,
                        autograph_available: false,
                    },
                ],
                popularity_rank: 2,
            },
            CharacterSchedule {
                character_name: "Dragon Buddy".to_string(),
                zone: "Dragon's Lair Entrance".to_string(),
                costumes: vec![CharacterCostume {
                    costume_id: 201,
                    variant: "friendly_green".to_string(),
                    seasonal: false,
                    condition_rating: 7,
                }],
                sessions: vec![MeetGreetSession {
                    session_id: 30003,
                    start_epoch: 1723284000,
                    end_epoch: 1723287600,
                    max_guests: 40,
                    photo_pass_available: true,
                    autograph_available: false,
                }],
                popularity_rank: 8,
            },
        ],
        total_sessions: 3,
    };

    let bytes = encode_to_vec(&plan).expect("encode meet greet plan");
    let (decoded, _): (MeetGreetPlan, usize) =
        decode_from_slice(&bytes).expect("decode meet greet plan");
    assert_eq!(plan, decoded);
}

// Test 10: Dynamic ticket pricing plan
#[test]
fn test_ticket_pricing_dynamic_plan() {
    let pricing = PricingPlan {
        plan_id: 600,
        season: "peak_summer_2025".to_string(),
        tiers: vec![
            TicketTier {
                tier_id: 1,
                tier_name: "Single Park Basic".to_string(),
                includes_parks: vec!["Adventure World".to_string()],
                date_prices: vec![
                    DatePricePoint {
                        date_str: "2025-07-04".to_string(),
                        demand_tier: "peak".to_string(),
                        adult_price_cents: 16900,
                        child_price_cents: 13900,
                        senior_price_cents: 14900,
                        capacity_percentage: 95.0,
                    },
                    DatePricePoint {
                        date_str: "2025-07-07".to_string(),
                        demand_tier: "regular".to_string(),
                        adult_price_cents: 12900,
                        child_price_cents: 10900,
                        senior_price_cents: 11900,
                        capacity_percentage: 60.0,
                    },
                ],
                available_addons: vec![
                    AddOn {
                        addon_id: 1,
                        name: "Lightning Lane Access".to_string(),
                        price_cents: 2500,
                        description: "Skip the line on select attractions".to_string(),
                        limited_quantity: Some(500),
                    },
                    AddOn {
                        addon_id: 2,
                        name: "Meal Plan Standard".to_string(),
                        price_cents: 5500,
                        description: "1 entree + 1 drink at participating locations".to_string(),
                        limited_quantity: None,
                    },
                ],
                max_days: 1,
            },
            TicketTier {
                tier_id: 2,
                tier_name: "Multi-Park Premium".to_string(),
                includes_parks: vec![
                    "Adventure World".to_string(),
                    "Water Kingdom".to_string(),
                    "Discovery Zone".to_string(),
                ],
                date_prices: vec![DatePricePoint {
                    date_str: "2025-07-04".to_string(),
                    demand_tier: "peak".to_string(),
                    adult_price_cents: 32900,
                    child_price_cents: 27900,
                    senior_price_cents: 29900,
                    capacity_percentage: 88.0,
                }],
                available_addons: vec![AddOn {
                    addon_id: 3,
                    name: "VIP Tour Guide".to_string(),
                    price_cents: 45000,
                    description: "Private guide for up to 10 guests".to_string(),
                    limited_quantity: Some(20),
                }],
                max_days: 5,
            },
        ],
        currency: "USD".to_string(),
        tax_rate_percent: 7.5,
    };

    let bytes = encode_to_vec(&pricing).expect("encode pricing plan");
    let (decoded, _): (PricingPlan, usize) =
        decode_from_slice(&bytes).expect("decode pricing plan");
    assert_eq!(pricing, decoded);
}

// Test 11: Parade float configuration with lighting and audio
#[test]
fn test_parade_float_configuration() {
    let parade = ParadeConfiguration {
        parade_id: 11001,
        parade_name: "Festival of Lights Grand Parade".to_string(),
        route_name: "Main Street Loop".to_string(),
        floats: vec![
            ParadeFloat {
                float_id: 1,
                theme: "Enchanted Forest".to_string(),
                length_meters: 12.5,
                weight_kg: 8500.0,
                performer_count: 8,
                lighting: vec![
                    FloatLighting {
                        zone_name: "canopy".to_string(),
                        led_count: 5000,
                        color_sequence: vec![
                            "emerald".to_string(),
                            "gold".to_string(),
                            "white".to_string(),
                        ],
                        animation_pattern: "twinkle_cascade".to_string(),
                        power_watts: 1200.0,
                    },
                    FloatLighting {
                        zone_name: "base_trim".to_string(),
                        led_count: 2000,
                        color_sequence: vec!["amber".to_string(), "warm_white".to_string()],
                        animation_pattern: "slow_pulse".to_string(),
                        power_watts: 450.0,
                    },
                ],
                audio: FloatAudio {
                    speaker_count: 6,
                    track_name: "Whispers of the Forest".to_string(),
                    volume_db: 85.0,
                    sync_offset_ms: -200,
                },
                max_speed_kmh: 5.0,
            },
            ParadeFloat {
                float_id: 2,
                theme: "Ocean Odyssey".to_string(),
                length_meters: 15.0,
                weight_kg: 11000.0,
                performer_count: 12,
                lighting: vec![FloatLighting {
                    zone_name: "wave_panels".to_string(),
                    led_count: 8000,
                    color_sequence: vec![
                        "deep_blue".to_string(),
                        "cyan".to_string(),
                        "turquoise".to_string(),
                        "white".to_string(),
                    ],
                    animation_pattern: "wave_motion".to_string(),
                    power_watts: 2000.0,
                }],
                audio: FloatAudio {
                    speaker_count: 8,
                    track_name: "Depths of Wonder".to_string(),
                    volume_db: 88.0,
                    sync_offset_ms: 0,
                },
                max_speed_kmh: 4.5,
            },
        ],
        total_duration_minutes: 35,
        start_epoch: 1710097200,
    };

    let bytes = encode_to_vec(&parade).expect("encode parade configuration");
    let (decoded, _): (ParadeConfiguration, usize) =
        decode_from_slice(&bytes).expect("decode parade configuration");
    assert_eq!(parade, decoded);
}

// Test 12: Water ride with splash zones and drop profiles
#[test]
fn test_water_ride_splash_modeling() {
    let ride = WaterRide {
        ride_id: 5001,
        ride_name: "Tsunami Plunge".to_string(),
        channels: vec![
            WaterChannel {
                channel_id: 1,
                width_meters: 3.5,
                depth_meters: 0.8,
                flow_rate_lps: 250.0,
                temperature_celsius: 22.5,
            },
            WaterChannel {
                channel_id: 2,
                width_meters: 5.0,
                depth_meters: 1.2,
                flow_rate_lps: 400.0,
                temperature_celsius: 22.0,
            },
        ],
        drops: vec![
            DropProfile {
                drop_id: 1,
                height_meters: 18.0,
                angle_degrees: 55.0,
                entry_speed_kmh: 60.0,
                splash_height_meters: 8.0,
            },
            DropProfile {
                drop_id: 2,
                height_meters: 8.0,
                angle_degrees: 40.0,
                entry_speed_kmh: 35.0,
                splash_height_meters: 4.0,
            },
        ],
        splash_zones: vec![
            SplashZone {
                zone_id: 1,
                distance_from_ride_meters: 2.0,
                splash_probability: 0.95,
                avg_water_volume_liters: 15.0,
                spectator_area: false,
            },
            SplashZone {
                zone_id: 2,
                distance_from_ride_meters: 8.0,
                splash_probability: 0.60,
                avg_water_volume_liters: 3.0,
                spectator_area: true,
            },
            SplashZone {
                zone_id: 3,
                distance_from_ride_meters: 15.0,
                splash_probability: 0.15,
                avg_water_volume_liters: 0.5,
                spectator_area: true,
            },
        ],
        total_water_volume_liters: 750000.0,
        recirculation_rate_percent: 98.5,
    };

    let bytes = encode_to_vec(&ride).expect("encode water ride");
    let (decoded, _): (WaterRide, usize) = decode_from_slice(&bytes).expect("decode water ride");
    assert_eq!(ride, decoded);
}

// Test 13: Park energy management zones
#[test]
fn test_park_energy_management_plan() {
    let plan = ParkEnergyPlan {
        plan_id: 13001,
        date_str: "2025-08-15".to_string(),
        zones: vec![
            EnergyZone {
                zone_name: "Thrill Zone".to_string(),
                consumers: vec![
                    PowerConsumer {
                        asset_id: 1001,
                        asset_name: "Thunder Mountain Express".to_string(),
                        asset_type: "coaster".to_string(),
                        peak_watts: 450000.0,
                        avg_watts: 280000.0,
                        zone: "thrill".to_string(),
                    },
                    PowerConsumer {
                        asset_id: 1002,
                        asset_name: "Zone Lighting Array".to_string(),
                        asset_type: "lighting".to_string(),
                        peak_watts: 75000.0,
                        avg_watts: 45000.0,
                        zone: "thrill".to_string(),
                    },
                ],
                readings: vec![
                    EnergyReading {
                        timestamp_epoch: 1723680000,
                        kwh_consumed: 2800.0,
                        peak_demand_kw: 525.0,
                        solar_generated_kwh: 120.0,
                    },
                    EnergyReading {
                        timestamp_epoch: 1723683600,
                        kwh_consumed: 3100.0,
                        peak_demand_kw: 580.0,
                        solar_generated_kwh: 180.0,
                    },
                ],
                transformer_capacity_kva: 800.0,
            },
            EnergyZone {
                zone_name: "Family Zone".to_string(),
                consumers: vec![PowerConsumer {
                    asset_id: 2001,
                    asset_name: "Carousel of Dreams".to_string(),
                    asset_type: "flat_ride".to_string(),
                    peak_watts: 35000.0,
                    avg_watts: 22000.0,
                    zone: "family".to_string(),
                }],
                readings: vec![EnergyReading {
                    timestamp_epoch: 1723680000,
                    kwh_consumed: 850.0,
                    peak_demand_kw: 120.0,
                    solar_generated_kwh: 95.0,
                }],
                transformer_capacity_kva: 300.0,
            },
        ],
        total_budget_kwh: 50000.0,
        renewable_target_percent: 25.0,
    };

    let bytes = encode_to_vec(&plan).expect("encode energy plan");
    let (decoded, _): (ParkEnergyPlan, usize) =
        decode_from_slice(&bytes).expect("decode energy plan");
    assert_eq!(plan, decoded);
}

// Test 14: Empty ride configuration (no vehicles, no track)
#[test]
fn test_ride_configuration_empty_under_construction() {
    let config = RideConfiguration {
        ride_id: 9999,
        ride_name: "Project Nebula".to_string(),
        category: "dark_ride".to_string(),
        height_requirement_cm: 0,
        vehicles: vec![],
        track: vec![],
        total_track_length_meters: 0.0,
        max_g_force: 0.0,
        ride_duration_seconds: 0,
        opened_year: 0,
    };

    let bytes = encode_to_vec(&config).expect("encode empty ride config");
    let (decoded, _): (RideConfiguration, usize) =
        decode_from_slice(&bytes).expect("decode empty ride config");
    assert_eq!(config, decoded);
}

// Test 15: Queue management with all lanes empty
#[test]
fn test_queue_management_early_morning_empty() {
    let queue = QueueManagement {
        ride_id: 1001,
        ride_name: "Thunder Mountain Express".to_string(),
        lanes: vec![
            QueueLane {
                lane_id: 1,
                lane_type: "standby".to_string(),
                current_wait_minutes: 0,
                capacity: 800,
                slots: vec![],
            },
            QueueLane {
                lane_id: 2,
                lane_type: "virtual_queue".to_string(),
                current_wait_minutes: 0,
                capacity: 200,
                slots: vec![],
            },
        ],
        total_guests_in_queue: 0,
        is_operational: false,
    };

    let bytes = encode_to_vec(&queue).expect("encode empty queue");
    let (decoded, _): (QueueManagement, usize) =
        decode_from_slice(&bytes).expect("decode empty queue");
    assert_eq!(queue, decoded);
}

// Test 16: Multi-outlet food court with allergen-heavy menus
#[test]
fn test_food_court_multiple_outlets() {
    let outlets = vec![
        FoodOutlet {
            outlet_id: 501,
            name: "Galactic Grill".to_string(),
            location_zone: "Space Station".to_string(),
            categories: vec![MenuCategory {
                category_name: "Burgers".to_string(),
                items: vec![MenuItem {
                    item_id: 5001,
                    name: "Meteor Burger".to_string(),
                    description: "Wagyu beef with truffle sauce".to_string(),
                    price_cents: 2495,
                    allergens: vec![
                        Allergen {
                            code: "GLU".to_string(),
                            name: "Gluten".to_string(),
                            severity: "contains".to_string(),
                        },
                        Allergen {
                            code: "DAI".to_string(),
                            name: "Dairy".to_string(),
                            severity: "contains".to_string(),
                        },
                        Allergen {
                            code: "SOY".to_string(),
                            name: "Soy".to_string(),
                            severity: "may_contain".to_string(),
                        },
                        Allergen {
                            code: "SES".to_string(),
                            name: "Sesame".to_string(),
                            severity: "contains".to_string(),
                        },
                    ],
                    nutrition: NutritionInfo {
                        calories: 1100,
                        protein_grams: 55.0,
                        carbs_grams: 62.0,
                        fat_grams: 68.0,
                        sodium_mg: 1800,
                    },
                    available: true,
                }],
            }],
            open_epoch: 1710064800,
            close_epoch: 1710100800,
        },
        FoodOutlet {
            outlet_id: 502,
            name: "Pirate's Catch".to_string(),
            location_zone: "Adventure Bay".to_string(),
            categories: vec![
                MenuCategory {
                    category_name: "Seafood".to_string(),
                    items: vec![MenuItem {
                        item_id: 5101,
                        name: "Captain's Fish & Chips".to_string(),
                        description: "Beer-battered cod with seasoned fries".to_string(),
                        price_cents: 1795,
                        allergens: vec![
                            Allergen {
                                code: "FISH".to_string(),
                                name: "Fish".to_string(),
                                severity: "contains".to_string(),
                            },
                            Allergen {
                                code: "GLU".to_string(),
                                name: "Gluten".to_string(),
                                severity: "contains".to_string(),
                            },
                        ],
                        nutrition: NutritionInfo {
                            calories: 780,
                            protein_grams: 35.0,
                            carbs_grams: 65.0,
                            fat_grams: 40.0,
                            sodium_mg: 950,
                        },
                        available: true,
                    }],
                },
                MenuCategory {
                    category_name: "Drinks".to_string(),
                    items: vec![MenuItem {
                        item_id: 5201,
                        name: "Tropical Storm Smoothie".to_string(),
                        description: "Mango, pineapple, coconut milk blend".to_string(),
                        price_cents: 795,
                        allergens: vec![Allergen {
                            code: "COCO".to_string(),
                            name: "Coconut".to_string(),
                            severity: "contains".to_string(),
                        }],
                        nutrition: NutritionInfo {
                            calories: 280,
                            protein_grams: 3.0,
                            carbs_grams: 52.0,
                            fat_grams: 8.0,
                            sodium_mg: 35,
                        },
                        available: true,
                    }],
                },
            ],
            open_epoch: 1710068400,
            close_epoch: 1710097200,
        },
    ];

    for outlet in &outlets {
        let bytes = encode_to_vec(outlet).expect("encode food court outlet");
        let (decoded, _): (FoodOutlet, usize) =
            decode_from_slice(&bytes).expect("decode food court outlet");
        assert_eq!(outlet, &decoded);
    }
}

// Test 17: Fireworks show with single cue (minimal)
#[test]
fn test_fireworks_minimal_single_cue() {
    let show = FireworksShow {
        show_id: 9010,
        show_name: "Midnight Sparkle".to_string(),
        total_duration_ms: 5000,
        cues: vec![LaunchCue {
            cue_id: 1,
            timestamp_ms: 0,
            launcher_id: 1,
            angle_degrees: 90.0,
            effects: vec![PyroEffect {
                effect_type: "single_shot".to_string(),
                color_primary: "white".to_string(),
                color_secondary: None,
                altitude_meters: 50.0,
                spread_degrees: 90.0,
                duration_ms: 2000,
            }],
        }],
        music: MusicSync {
            track_name: "Silent Night Ambient".to_string(),
            bpm: 60.0,
            beat_markers_ms: vec![0, 1000, 2000, 3000, 4000],
        },
        total_shells: 1,
        safety_radius_meters: 100.0,
    };

    let bytes = encode_to_vec(&show).expect("encode minimal fireworks");
    let (decoded, _): (FireworksShow, usize) =
        decode_from_slice(&bytes).expect("decode minimal fireworks");
    assert_eq!(show, decoded);
}

// Test 18: Hotel inventory with no availability (sold out)
#[test]
fn test_hotel_fully_booked_new_years() {
    let hotel = HotelInventory {
        hotel_id: 601,
        hotel_name: "Stardust Grand Hotel".to_string(),
        star_rating: 5,
        room_types: vec![
            RoomType {
                type_id: 10,
                name: "Deluxe Fireworks View".to_string(),
                max_occupancy: 4,
                bed_config: "2_queen".to_string(),
                square_meters: 45.0,
                amenities: vec![
                    RoomAmenity {
                        amenity_name: "Private Fireworks Viewing".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                    RoomAmenity {
                        amenity_name: "Champagne Welcome".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                    RoomAmenity {
                        amenity_name: "Late Checkout".to_string(),
                        included: false,
                        surcharge_cents: 5000,
                    },
                ],
                rate_calendar: vec![
                    NightlyRate {
                        date_str: "2025-12-31".to_string(),
                        base_rate_cents: 89900,
                        tax_cents: 11687,
                        resort_fee_cents: 4500,
                        available_rooms: 0,
                    },
                    NightlyRate {
                        date_str: "2026-01-01".to_string(),
                        base_rate_cents: 65000,
                        tax_cents: 8450,
                        resort_fee_cents: 4500,
                        available_rooms: 0,
                    },
                ],
            },
            RoomType {
                type_id: 11,
                name: "Presidential Suite".to_string(),
                max_occupancy: 8,
                bed_config: "3_king_living_room".to_string(),
                square_meters: 180.0,
                amenities: vec![
                    RoomAmenity {
                        amenity_name: "Personal Concierge".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                    RoomAmenity {
                        amenity_name: "Private Pool".to_string(),
                        included: true,
                        surcharge_cents: 0,
                    },
                    RoomAmenity {
                        amenity_name: "Helicopter Transfer".to_string(),
                        included: false,
                        surcharge_cents: 250000,
                    },
                ],
                rate_calendar: vec![NightlyRate {
                    date_str: "2025-12-31".to_string(),
                    base_rate_cents: 450000,
                    tax_cents: 58500,
                    resort_fee_cents: 0,
                    available_rooms: 0,
                }],
            },
        ],
        total_rooms: 420,
    };

    let bytes = encode_to_vec(&hotel).expect("encode fully booked hotel");
    let (decoded, _): (HotelInventory, usize) =
        decode_from_slice(&bytes).expect("decode fully booked hotel");
    assert_eq!(hotel, decoded);
}

// Test 19: Guest with only dining scores, no rides
#[test]
fn test_guest_experience_dining_only() {
    let report = GuestExperienceReport {
        guest_id: 880042,
        visit_date: "2025-03-22".to_string(),
        overall_satisfaction: 6.5,
        ride_scores: vec![],
        dining_scores: vec![
            DiningScore {
                outlet_id: 401,
                outlet_name: "Dragon's Feast Hall".to_string(),
                food_quality_score: 9.0,
                service_score: 8.5,
                value_score: 4.0,
            },
            DiningScore {
                outlet_id: 502,
                outlet_name: "Pirate's Catch".to_string(),
                food_quality_score: 7.5,
                service_score: 6.0,
                value_score: 5.5,
            },
            DiningScore {
                outlet_id: 503,
                outlet_name: "Noodle Nebula".to_string(),
                food_quality_score: 8.0,
                service_score: 9.0,
                value_score: 7.0,
            },
        ],
        would_recommend: false,
        net_promoter_score: 5,
    };

    let bytes = encode_to_vec(&report).expect("encode dining-only report");
    let (decoded, _): (GuestExperienceReport, usize) =
        decode_from_slice(&bytes).expect("decode dining-only report");
    assert_eq!(report, decoded);
}

// Test 20: Work order with zero-parts inspection-only tasks
#[test]
fn test_maintenance_inspection_only_order() {
    let order = WorkOrder {
        order_id: 88050,
        ride_id: 5001,
        ride_name: "Tsunami Plunge".to_string(),
        created_epoch: 1710200000,
        due_epoch: 1710286400,
        tasks: vec![
            MaintenanceTask {
                task_id: 10,
                description: "Annual structural integrity inspection of drop tower".to_string(),
                estimated_hours: 8.0,
                priority: 1,
                parts: vec![],
                requires_ride_shutdown: true,
            },
            MaintenanceTask {
                task_id: 11,
                description: "Non-destructive testing of weld joints".to_string(),
                estimated_hours: 12.0,
                priority: 1,
                parts: vec![],
                requires_ride_shutdown: true,
            },
            MaintenanceTask {
                task_id: 12,
                description: "Water quality and pH level verification".to_string(),
                estimated_hours: 2.0,
                priority: 3,
                parts: vec![],
                requires_ride_shutdown: false,
            },
            MaintenanceTask {
                task_id: 13,
                description: "Emergency stop system functional test".to_string(),
                estimated_hours: 3.0,
                priority: 1,
                parts: vec![],
                requires_ride_shutdown: true,
            },
        ],
        assigned_technician: "Dr. Ingrid Hoffmann".to_string(),
        status: "scheduled".to_string(),
        total_estimated_cost_cents: 0,
    };

    let bytes = encode_to_vec(&order).expect("encode inspection work order");
    let (decoded, _): (WorkOrder, usize) =
        decode_from_slice(&bytes).expect("decode inspection work order");
    assert_eq!(order, decoded);
}

// Test 21: Massive character meet-greet plan for holiday event
#[test]
fn test_character_meet_greet_holiday_extravaganza() {
    let plan = MeetGreetPlan {
        date_str: "2025-12-25".to_string(),
        park_section: "All Zones".to_string(),
        characters: vec![
            CharacterSchedule {
                character_name: "Santa Claus".to_string(),
                zone: "Winter Wonderland Grotto".to_string(),
                costumes: vec![
                    CharacterCostume {
                        costume_id: 500,
                        variant: "traditional_red".to_string(),
                        seasonal: true,
                        condition_rating: 10,
                    },
                    CharacterCostume {
                        costume_id: 501,
                        variant: "arctic_expedition".to_string(),
                        seasonal: true,
                        condition_rating: 9,
                    },
                ],
                sessions: vec![
                    MeetGreetSession {
                        session_id: 80001,
                        start_epoch: 1735113600,
                        end_epoch: 1735120800,
                        max_guests: 200,
                        photo_pass_available: true,
                        autograph_available: true,
                    },
                    MeetGreetSession {
                        session_id: 80002,
                        start_epoch: 1735124400,
                        end_epoch: 1735131600,
                        max_guests: 200,
                        photo_pass_available: true,
                        autograph_available: true,
                    },
                    MeetGreetSession {
                        session_id: 80003,
                        start_epoch: 1735135200,
                        end_epoch: 1735142400,
                        max_guests: 150,
                        photo_pass_available: true,
                        autograph_available: false,
                    },
                ],
                popularity_rank: 1,
            },
            CharacterSchedule {
                character_name: "Snowflake the Reindeer".to_string(),
                zone: "Reindeer Stables".to_string(),
                costumes: vec![CharacterCostume {
                    costume_id: 510,
                    variant: "sparkle_antlers".to_string(),
                    seasonal: true,
                    condition_rating: 8,
                }],
                sessions: vec![
                    MeetGreetSession {
                        session_id: 80010,
                        start_epoch: 1735117200,
                        end_epoch: 1735124400,
                        max_guests: 100,
                        photo_pass_available: true,
                        autograph_available: false,
                    },
                    MeetGreetSession {
                        session_id: 80011,
                        start_epoch: 1735128000,
                        end_epoch: 1735135200,
                        max_guests: 100,
                        photo_pass_available: true,
                        autograph_available: false,
                    },
                ],
                popularity_rank: 3,
            },
            CharacterSchedule {
                character_name: "Gingerbread Man".to_string(),
                zone: "Candy Cane Lane".to_string(),
                costumes: vec![
                    CharacterCostume {
                        costume_id: 520,
                        variant: "frosted_classic".to_string(),
                        seasonal: true,
                        condition_rating: 6,
                    },
                    CharacterCostume {
                        costume_id: 521,
                        variant: "chocolate_drizzle".to_string(),
                        seasonal: true,
                        condition_rating: 10,
                    },
                ],
                sessions: vec![MeetGreetSession {
                    session_id: 80020,
                    start_epoch: 1735120800,
                    end_epoch: 1735131600,
                    max_guests: 80,
                    photo_pass_available: false,
                    autograph_available: true,
                }],
                popularity_rank: 5,
            },
        ],
        total_sessions: 6,
    };

    let bytes = encode_to_vec(&plan).expect("encode holiday meet greet plan");
    let (decoded, _): (MeetGreetPlan, usize) =
        decode_from_slice(&bytes).expect("decode holiday meet greet plan");
    assert_eq!(plan, decoded);
}

// Test 22: Ride configuration with extreme track (many inversions, long track)
#[test]
fn test_ride_configuration_extreme_coaster() {
    let config = RideConfiguration {
        ride_id: 7777,
        ride_name: "Vortex Infinity".to_string(),
        category: "hypercoaster".to_string(),
        height_requirement_cm: 140,
        vehicles: vec![
            RideVehicle {
                vehicle_id: 10,
                name: "Phantom".to_string(),
                capacity: 32,
                weight_kg: 4800.0,
                safety_harness_type: "vest_restraint".to_string(),
                last_inspection_epoch: 1710100000,
            },
            RideVehicle {
                vehicle_id: 11,
                name: "Wraith".to_string(),
                capacity: 32,
                weight_kg: 4750.0,
                safety_harness_type: "vest_restraint".to_string(),
                last_inspection_epoch: 1710100500,
            },
            RideVehicle {
                vehicle_id: 12,
                name: "Spectre".to_string(),
                capacity: 32,
                weight_kg: 4820.0,
                safety_harness_type: "vest_restraint".to_string(),
                last_inspection_epoch: 1710101000,
            },
        ],
        track: vec![
            TrackSegment {
                segment_id: 1,
                segment_type: "launch".to_string(),
                length_meters: 50.0,
                incline_degrees: 0.0,
                max_speed_kmh: 160.0,
                has_inversion: false,
            },
            TrackSegment {
                segment_id: 2,
                segment_type: "top_hat".to_string(),
                length_meters: 120.0,
                incline_degrees: 90.0,
                max_speed_kmh: 140.0,
                has_inversion: true,
            },
            TrackSegment {
                segment_id: 3,
                segment_type: "heartline_roll".to_string(),
                length_meters: 35.0,
                incline_degrees: 0.0,
                max_speed_kmh: 130.0,
                has_inversion: true,
            },
            TrackSegment {
                segment_id: 4,
                segment_type: "zero_g_stall".to_string(),
                length_meters: 60.0,
                incline_degrees: 180.0,
                max_speed_kmh: 110.0,
                has_inversion: true,
            },
            TrackSegment {
                segment_id: 5,
                segment_type: "cobra_roll".to_string(),
                length_meters: 80.0,
                incline_degrees: 0.0,
                max_speed_kmh: 105.0,
                has_inversion: true,
            },
            TrackSegment {
                segment_id: 6,
                segment_type: "wave_turn".to_string(),
                length_meters: 55.0,
                incline_degrees: 15.0,
                max_speed_kmh: 95.0,
                has_inversion: false,
            },
            TrackSegment {
                segment_id: 7,
                segment_type: "brake_run".to_string(),
                length_meters: 100.0,
                incline_degrees: 0.0,
                max_speed_kmh: 20.0,
                has_inversion: false,
            },
        ],
        total_track_length_meters: 2100.0,
        max_g_force: 5.1,
        ride_duration_seconds: 95,
        opened_year: 2025,
    };

    let bytes = encode_to_vec(&config).expect("encode extreme coaster config");
    let (decoded, _): (RideConfiguration, usize) =
        decode_from_slice(&bytes).expect("decode extreme coaster config");
    assert_eq!(config, decoded);
    assert_eq!(decoded.track.len(), 7);
    assert_eq!(decoded.track.iter().filter(|s| s.has_inversion).count(), 4);
    assert_eq!(decoded.vehicles.len(), 3);
}
