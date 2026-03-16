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

// --- Domain types: Theme Park Operations & Guest Experience ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RideCapacityMetrics {
    ride_id: u64,
    ride_name: String,
    hourly_throughput: u32,
    vehicle_count: u16,
    seats_per_vehicle: u8,
    dispatch_interval_secs: u16,
    daily_capacity: u64,
    operational_efficiency_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct QueueWaitPrediction {
    ride_id: u64,
    timestamp_epoch: u64,
    current_wait_mins: u16,
    predicted_30min: u16,
    predicted_60min: u16,
    predicted_90min: u16,
    queue_length: u32,
    weather_factor: f32,
    special_event_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VirtualQueueReservation {
    reservation_id: u64,
    guest_id: u64,
    ride_id: u64,
    boarding_group: u16,
    window_start_epoch: u64,
    window_end_epoch: u64,
    status: String,
    party_size: u8,
    is_disability_access: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CharacterMeetGreetSlot {
    slot_id: u64,
    character_name: String,
    location_name: String,
    start_epoch: u64,
    end_epoch: u64,
    max_guests: u16,
    current_booked: u16,
    requires_reservation: bool,
    performer_id: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ShowPerformance {
    show_id: u64,
    show_name: String,
    venue: String,
    start_epoch: u64,
    duration_mins: u16,
    capacity: u32,
    tickets_sold: u32,
    cast_size: u16,
    is_seasonal: bool,
    language: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FoodBeverageTransaction {
    transaction_id: u64,
    register_id: u32,
    outlet_name: String,
    items: Vec<PosLineItem>,
    subtotal_cents: u64,
    tax_cents: u64,
    total_cents: u64,
    payment_method: String,
    timestamp_epoch: u64,
    guest_id: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PosLineItem {
    item_name: String,
    sku: String,
    quantity: u16,
    unit_price_cents: u32,
    is_meal_deal: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MerchandiseInventory {
    item_id: u64,
    name: String,
    category: String,
    sku: String,
    location: String,
    quantity_on_hand: u32,
    reorder_point: u32,
    wholesale_cost_cents: u32,
    retail_price_cents: u32,
    is_exclusive: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AnnualPassHolder {
    pass_id: u64,
    guest_name: String,
    tier: String,
    purchase_date_epoch: u64,
    expiry_date_epoch: u64,
    visits_this_year: u32,
    home_park: String,
    blackout_dates: Vec<u64>,
    parking_included: bool,
    photopass_included: bool,
    discount_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HotelRoomOccupancy {
    room_number: u32,
    hotel_name: String,
    room_type: String,
    floor: u8,
    max_occupancy: u8,
    current_guests: u8,
    check_in_epoch: u64,
    check_out_epoch: u64,
    rate_cents_per_night: u32,
    is_accessible: bool,
    view_type: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TransportSchedule {
    route_id: u32,
    vehicle_type: String,
    departure_point: String,
    arrival_point: String,
    departure_epoch: u64,
    estimated_arrival_epoch: u64,
    capacity: u16,
    passengers_aboard: u16,
    frequency_mins: u16,
    is_express: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WeatherRideClosure {
    closure_id: u64,
    ride_id: u64,
    ride_name: String,
    closure_reason: String,
    wind_speed_mph: f32,
    temperature_f: f32,
    lightning_detected: bool,
    closure_start_epoch: u64,
    estimated_reopen_epoch: Option<u64>,
    affected_guest_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FireworksShowControl {
    show_id: u64,
    show_name: String,
    launch_zones: Vec<LaunchZone>,
    total_shells: u32,
    duration_secs: u16,
    start_epoch: u64,
    wind_check_passed: bool,
    pyro_lead: String,
    music_track: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LaunchZone {
    zone_id: u8,
    zone_name: String,
    shell_count: u16,
    caliber_inches: f32,
    angle_degrees: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ParadeShowControl {
    parade_id: u64,
    parade_name: String,
    floats: Vec<ParadeFloat>,
    start_epoch: u64,
    route_length_meters: u32,
    estimated_duration_mins: u16,
    performer_count: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ParadeFloat {
    float_id: u16,
    float_name: String,
    performers: u8,
    height_meters: f32,
    has_water_effects: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GuestSatisfactionSurvey {
    survey_id: u64,
    guest_id: u64,
    visit_date_epoch: u64,
    overall_rating: u8,
    ride_rating: u8,
    food_rating: u8,
    cleanliness_rating: u8,
    staff_rating: u8,
    value_rating: u8,
    would_recommend: bool,
    comments: String,
    favorite_ride: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SafetyInspectionRecord {
    inspection_id: u64,
    ride_id: u64,
    ride_name: String,
    inspector_name: String,
    inspection_date_epoch: u64,
    category: String,
    checks: Vec<InspectionCheck>,
    overall_pass: bool,
    next_inspection_epoch: u64,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InspectionCheck {
    check_name: String,
    passed: bool,
    measurement: Option<f64>,
    tolerance: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DynamicPricingModel {
    ticket_type: String,
    base_price_cents: u32,
    date_epoch: u64,
    demand_multiplier: f32,
    holiday_surcharge_cents: u32,
    final_price_cents: u32,
    tier: String,
    capacity_remaining_pct: f32,
    early_bird_discount_pct: u8,
    group_discount_threshold: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ParkZoneHeatmap {
    zone_id: u32,
    zone_name: String,
    current_guests: u32,
    max_capacity: u32,
    density_per_sqm: f32,
    avg_dwell_time_mins: u16,
    entry_rate_per_min: u16,
    exit_rate_per_min: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RideMaintenanceLog {
    log_id: u64,
    ride_id: u64,
    ride_name: String,
    maintenance_type: String,
    technician: String,
    start_epoch: u64,
    end_epoch: Option<u64>,
    parts_replaced: Vec<String>,
    cost_cents: u64,
    downtime_mins: u32,
    is_emergency: bool,
}

// --- Tests ---

#[test]
fn test_ride_capacity_metrics_roundtrip() {
    let cfg = config::standard();
    let data = RideCapacityMetrics {
        ride_id: 1001,
        ride_name: "Thunder Mountain Express".to_string(),
        hourly_throughput: 2400,
        vehicle_count: 12,
        seats_per_vehicle: 20,
        dispatch_interval_secs: 30,
        daily_capacity: 28800,
        operational_efficiency_pct: 92.5,
    };
    let encoded = encode_to_vec(&data, cfg).expect("encode ride capacity");
    let (decoded, _): (RideCapacityMetrics, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ride capacity");
    assert_eq!(data, decoded);
}

#[test]
fn test_queue_wait_prediction_roundtrip() {
    let cfg = config::standard();
    let data = QueueWaitPrediction {
        ride_id: 2001,
        timestamp_epoch: 1700000000,
        current_wait_mins: 75,
        predicted_30min: 85,
        predicted_60min: 65,
        predicted_90min: 45,
        queue_length: 1200,
        weather_factor: 1.15,
        special_event_active: true,
    };
    let encoded = encode_to_vec(&data, cfg).expect("encode queue wait prediction");
    let (decoded, _): (QueueWaitPrediction, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode queue wait prediction");
    assert_eq!(data, decoded);
}

#[test]
fn test_virtual_queue_reservation_roundtrip() {
    let cfg = config::standard();
    let reservations = vec![
        VirtualQueueReservation {
            reservation_id: 500001,
            guest_id: 88001,
            ride_id: 3001,
            boarding_group: 42,
            window_start_epoch: 1700010000,
            window_end_epoch: 1700013600,
            status: "confirmed".to_string(),
            party_size: 4,
            is_disability_access: false,
        },
        VirtualQueueReservation {
            reservation_id: 500002,
            guest_id: 88002,
            ride_id: 3001,
            boarding_group: 43,
            window_start_epoch: 1700013600,
            window_end_epoch: 1700017200,
            status: "pending".to_string(),
            party_size: 2,
            is_disability_access: true,
        },
    ];
    let encoded = encode_to_vec(&reservations, cfg).expect("encode virtual queue reservations");
    let (decoded, _): (Vec<VirtualQueueReservation>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode virtual queue reservations");
    assert_eq!(reservations, decoded);
}

#[test]
fn test_character_meet_greet_scheduling_roundtrip() {
    let cfg = config::standard();
    let slots = vec![
        CharacterMeetGreetSlot {
            slot_id: 7001,
            character_name: "Princess Aurora".to_string(),
            location_name: "Enchanted Garden Pavilion".to_string(),
            start_epoch: 1700020000,
            end_epoch: 1700023600,
            max_guests: 60,
            current_booked: 45,
            requires_reservation: true,
            performer_id: 301,
        },
        CharacterMeetGreetSlot {
            slot_id: 7002,
            character_name: "Captain Cosmos".to_string(),
            location_name: "Tomorrowland Stage".to_string(),
            start_epoch: 1700025000,
            end_epoch: 1700028600,
            max_guests: 80,
            current_booked: 12,
            requires_reservation: false,
            performer_id: 302,
        },
    ];
    let encoded = encode_to_vec(&slots, cfg).expect("encode character meet greet slots");
    let (decoded, _): (Vec<CharacterMeetGreetSlot>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode character meet greet slots");
    assert_eq!(slots, decoded);
}

#[test]
fn test_show_performance_timetable_roundtrip() {
    let cfg = config::standard();
    let shows = vec![
        ShowPerformance {
            show_id: 4001,
            show_name: "Enchanted Tales: A Musical Journey".to_string(),
            venue: "Grand Theatre".to_string(),
            start_epoch: 1700030000,
            duration_mins: 35,
            capacity: 1500,
            tickets_sold: 1423,
            cast_size: 28,
            is_seasonal: false,
            language: "en".to_string(),
        },
        ShowPerformance {
            show_id: 4002,
            show_name: "Winter Wonderland on Ice".to_string(),
            venue: "Arena of Dreams".to_string(),
            start_epoch: 1700040000,
            duration_mins: 45,
            capacity: 3000,
            tickets_sold: 2876,
            cast_size: 42,
            is_seasonal: true,
            language: "multilingual".to_string(),
        },
    ];
    let encoded = encode_to_vec(&shows, cfg).expect("encode show performances");
    let (decoded, _): (Vec<ShowPerformance>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode show performances");
    assert_eq!(shows, decoded);
}

#[test]
fn test_food_beverage_transaction_roundtrip() {
    let cfg = config::standard();
    let transaction = FoodBeverageTransaction {
        transaction_id: 9900001,
        register_id: 15,
        outlet_name: "Cosmic Cantina".to_string(),
        items: vec![
            PosLineItem {
                item_name: "Galaxy Burger Combo".to_string(),
                sku: "FB-BRG-001".to_string(),
                quantity: 2,
                unit_price_cents: 1899,
                is_meal_deal: true,
            },
            PosLineItem {
                item_name: "Nebula Milkshake".to_string(),
                sku: "FB-BEV-042".to_string(),
                quantity: 1,
                unit_price_cents: 799,
                is_meal_deal: false,
            },
            PosLineItem {
                item_name: "Starlight Churros".to_string(),
                sku: "FB-SNK-017".to_string(),
                quantity: 1,
                unit_price_cents: 649,
                is_meal_deal: false,
            },
        ],
        subtotal_cents: 5246,
        tax_cents: 420,
        total_cents: 5666,
        payment_method: "mobile_pay".to_string(),
        timestamp_epoch: 1700050000,
        guest_id: Some(88001),
    };
    let encoded = encode_to_vec(&transaction, cfg).expect("encode food beverage transaction");
    let (decoded, _): (FoodBeverageTransaction, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode food beverage transaction");
    assert_eq!(transaction, decoded);
}

#[test]
fn test_merchandise_inventory_roundtrip() {
    let cfg = config::standard();
    let items = vec![
        MerchandiseInventory {
            item_id: 60001,
            name: "Enchanted Castle Snow Globe".to_string(),
            category: "Collectibles".to_string(),
            sku: "MRC-COL-0088".to_string(),
            location: "Main Street Emporium".to_string(),
            quantity_on_hand: 342,
            reorder_point: 100,
            wholesale_cost_cents: 1200,
            retail_price_cents: 3499,
            is_exclusive: true,
        },
        MerchandiseInventory {
            item_id: 60002,
            name: "Theme Park Logo T-Shirt".to_string(),
            category: "Apparel".to_string(),
            sku: "MRC-APP-0201".to_string(),
            location: "Adventure Outfitters".to_string(),
            quantity_on_hand: 1580,
            reorder_point: 500,
            wholesale_cost_cents: 450,
            retail_price_cents: 2999,
            is_exclusive: false,
        },
    ];
    let encoded = encode_to_vec(&items, cfg).expect("encode merchandise inventory");
    let (decoded, _): (Vec<MerchandiseInventory>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode merchandise inventory");
    assert_eq!(items, decoded);
}

#[test]
fn test_annual_pass_holder_profile_roundtrip() {
    let cfg = config::standard();
    let holder = AnnualPassHolder {
        pass_id: 770001,
        guest_name: "Sakura Tanaka".to_string(),
        tier: "Platinum".to_string(),
        purchase_date_epoch: 1680000000,
        expiry_date_epoch: 1711536000,
        visits_this_year: 47,
        home_park: "Dreamland Resort".to_string(),
        blackout_dates: vec![1700000000, 1703000000, 1706000000],
        parking_included: true,
        photopass_included: true,
        discount_pct: 20,
    };
    let encoded = encode_to_vec(&holder, cfg).expect("encode annual pass holder");
    let (decoded, _): (AnnualPassHolder, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode annual pass holder");
    assert_eq!(holder, decoded);
}

#[test]
fn test_hotel_room_occupancy_roundtrip() {
    let cfg = config::standard();
    let rooms = vec![
        HotelRoomOccupancy {
            room_number: 4215,
            hotel_name: "Grand Fantasia Resort".to_string(),
            room_type: "Deluxe Suite".to_string(),
            floor: 42,
            max_occupancy: 5,
            current_guests: 4,
            check_in_epoch: 1700050000,
            check_out_epoch: 1700309200,
            rate_cents_per_night: 45900,
            is_accessible: false,
            view_type: "Fireworks View".to_string(),
        },
        HotelRoomOccupancy {
            room_number: 1102,
            hotel_name: "Lakeside Lodge".to_string(),
            room_type: "Standard Double".to_string(),
            floor: 11,
            max_occupancy: 4,
            current_guests: 2,
            check_in_epoch: 1700060000,
            check_out_epoch: 1700222800,
            rate_cents_per_night: 21900,
            is_accessible: true,
            view_type: "Garden View".to_string(),
        },
    ];
    let encoded = encode_to_vec(&rooms, cfg).expect("encode hotel room occupancy");
    let (decoded, _): (Vec<HotelRoomOccupancy>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode hotel room occupancy");
    assert_eq!(rooms, decoded);
}

#[test]
fn test_monorail_schedule_roundtrip() {
    let cfg = config::standard();
    let schedules = vec![
        TransportSchedule {
            route_id: 101,
            vehicle_type: "Monorail".to_string(),
            departure_point: "Main Gate Station".to_string(),
            arrival_point: "Resort Hub Station".to_string(),
            departure_epoch: 1700070000,
            estimated_arrival_epoch: 1700070900,
            capacity: 364,
            passengers_aboard: 280,
            frequency_mins: 5,
            is_express: false,
        },
        TransportSchedule {
            route_id: 102,
            vehicle_type: "Monorail".to_string(),
            departure_point: "Resort Hub Station".to_string(),
            arrival_point: "Waterfront Station".to_string(),
            departure_epoch: 1700071000,
            estimated_arrival_epoch: 1700071600,
            capacity: 364,
            passengers_aboard: 195,
            frequency_mins: 5,
            is_express: true,
        },
    ];
    let encoded = encode_to_vec(&schedules, cfg).expect("encode monorail schedule");
    let (decoded, _): (Vec<TransportSchedule>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode monorail schedule");
    assert_eq!(schedules, decoded);
}

#[test]
fn test_bus_boat_transport_roundtrip() {
    let cfg = config::standard();
    let mixed_transport = vec![
        TransportSchedule {
            route_id: 201,
            vehicle_type: "Bus".to_string(),
            departure_point: "Parking Lot G".to_string(),
            arrival_point: "Theme Park West Entrance".to_string(),
            departure_epoch: 1700080000,
            estimated_arrival_epoch: 1700080600,
            capacity: 72,
            passengers_aboard: 55,
            frequency_mins: 10,
            is_express: false,
        },
        TransportSchedule {
            route_id: 301,
            vehicle_type: "Ferryboat".to_string(),
            departure_point: "Marina Dock A".to_string(),
            arrival_point: "Resort Island Pier".to_string(),
            departure_epoch: 1700085000,
            estimated_arrival_epoch: 1700086200,
            capacity: 600,
            passengers_aboard: 412,
            frequency_mins: 20,
            is_express: false,
        },
    ];
    let encoded = encode_to_vec(&mixed_transport, cfg).expect("encode bus/boat transport");
    let (decoded, _): (Vec<TransportSchedule>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode bus/boat transport");
    assert_eq!(mixed_transport, decoded);
}

#[test]
fn test_weather_ride_closure_roundtrip() {
    let cfg = config::standard();
    let closures = vec![
        WeatherRideClosure {
            closure_id: 11001,
            ride_id: 1001,
            ride_name: "Thunder Mountain Express".to_string(),
            closure_reason: "High winds exceeding safety threshold".to_string(),
            wind_speed_mph: 38.5,
            temperature_f: 72.0,
            lightning_detected: false,
            closure_start_epoch: 1700090000,
            estimated_reopen_epoch: Some(1700097200),
            affected_guest_count: 340,
        },
        WeatherRideClosure {
            closure_id: 11002,
            ride_id: 2005,
            ride_name: "Splash Canyon Rapids".to_string(),
            closure_reason: "Lightning within 10-mile radius".to_string(),
            wind_speed_mph: 22.0,
            temperature_f: 85.3,
            lightning_detected: true,
            closure_start_epoch: 1700091000,
            estimated_reopen_epoch: None,
            affected_guest_count: 580,
        },
    ];
    let encoded = encode_to_vec(&closures, cfg).expect("encode weather ride closures");
    let (decoded, _): (Vec<WeatherRideClosure>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode weather ride closures");
    assert_eq!(closures, decoded);
}

#[test]
fn test_fireworks_show_control_roundtrip() {
    let cfg = config::standard();
    let show = FireworksShowControl {
        show_id: 8001,
        show_name: "Symphony of Stars".to_string(),
        launch_zones: vec![
            LaunchZone {
                zone_id: 1,
                zone_name: "Castle Roof".to_string(),
                shell_count: 120,
                caliber_inches: 3.0,
                angle_degrees: 85.0,
            },
            LaunchZone {
                zone_id: 2,
                zone_name: "Lakeside Battery A".to_string(),
                shell_count: 250,
                caliber_inches: 6.0,
                angle_degrees: 78.5,
            },
            LaunchZone {
                zone_id: 3,
                zone_name: "Lakeside Battery B".to_string(),
                shell_count: 180,
                caliber_inches: 8.0,
                angle_degrees: 72.0,
            },
        ],
        total_shells: 550,
        duration_secs: 1200,
        start_epoch: 1700100000,
        wind_check_passed: true,
        pyro_lead: "Marco DeLuca".to_string(),
        music_track: "symphony_of_stars_v3_master.wav".to_string(),
    };
    let encoded = encode_to_vec(&show, cfg).expect("encode fireworks show control");
    let (decoded, _): (FireworksShowControl, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode fireworks show control");
    assert_eq!(show, decoded);
}

#[test]
fn test_parade_show_control_roundtrip() {
    let cfg = config::standard();
    let parade = ParadeShowControl {
        parade_id: 9001,
        parade_name: "Festival of Fantasy Parade".to_string(),
        floats: vec![
            ParadeFloat {
                float_id: 1,
                float_name: "Enchanted Garden".to_string(),
                performers: 8,
                height_meters: 5.2,
                has_water_effects: true,
            },
            ParadeFloat {
                float_id: 2,
                float_name: "Dragon's Lair".to_string(),
                performers: 6,
                height_meters: 7.8,
                has_water_effects: false,
            },
            ParadeFloat {
                float_id: 3,
                float_name: "Stardust Finale".to_string(),
                performers: 12,
                height_meters: 6.5,
                has_water_effects: true,
            },
        ],
        start_epoch: 1700110000,
        route_length_meters: 1850,
        estimated_duration_mins: 22,
        performer_count: 86,
    };
    let encoded = encode_to_vec(&parade, cfg).expect("encode parade show control");
    let (decoded, _): (ParadeShowControl, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode parade show control");
    assert_eq!(parade, decoded);
}

#[test]
fn test_guest_satisfaction_survey_roundtrip() {
    let cfg = config::standard();
    let surveys = vec![
        GuestSatisfactionSurvey {
            survey_id: 330001,
            guest_id: 88001,
            visit_date_epoch: 1700000000,
            overall_rating: 9,
            ride_rating: 10,
            food_rating: 7,
            cleanliness_rating: 9,
            staff_rating: 10,
            value_rating: 6,
            would_recommend: true,
            comments: "Amazing day! The new coaster was incredible. Food prices were steep though."
                .to_string(),
            favorite_ride: "Thunder Mountain Express".to_string(),
        },
        GuestSatisfactionSurvey {
            survey_id: 330002,
            guest_id: 88045,
            visit_date_epoch: 1700086400,
            overall_rating: 7,
            ride_rating: 8,
            food_rating: 6,
            cleanliness_rating: 8,
            staff_rating: 9,
            value_rating: 5,
            would_recommend: true,
            comments: "Fun but very crowded. Wait times were longer than expected.".to_string(),
            favorite_ride: "Splash Canyon Rapids".to_string(),
        },
    ];
    let encoded = encode_to_vec(&surveys, cfg).expect("encode guest satisfaction surveys");
    let (decoded, _): (Vec<GuestSatisfactionSurvey>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode guest satisfaction surveys");
    assert_eq!(surveys, decoded);
}

#[test]
fn test_safety_inspection_record_roundtrip() {
    let cfg = config::standard();
    let record = SafetyInspectionRecord {
        inspection_id: 44001,
        ride_id: 1001,
        ride_name: "Thunder Mountain Express".to_string(),
        inspector_name: "Elena Vasquez".to_string(),
        inspection_date_epoch: 1699900000,
        category: "Annual Structural".to_string(),
        checks: vec![
            InspectionCheck {
                check_name: "Track bolt torque".to_string(),
                passed: true,
                measurement: Some(145.0),
                tolerance: Some(5.0),
            },
            InspectionCheck {
                check_name: "Brake system response time".to_string(),
                passed: true,
                measurement: Some(0.82),
                tolerance: Some(0.1),
            },
            InspectionCheck {
                check_name: "Restraint latch integrity".to_string(),
                passed: true,
                measurement: None,
                tolerance: None,
            },
            InspectionCheck {
                check_name: "Emergency stop function".to_string(),
                passed: true,
                measurement: Some(1.1),
                tolerance: Some(0.5),
            },
        ],
        overall_pass: true,
        next_inspection_epoch: 1731436000,
        notes: "All systems nominal. Minor cosmetic wear on car 7 paint noted.".to_string(),
    };
    let encoded = encode_to_vec(&record, cfg).expect("encode safety inspection record");
    let (decoded, _): (SafetyInspectionRecord, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode safety inspection record");
    assert_eq!(record, decoded);
}

#[test]
fn test_dynamic_pricing_model_roundtrip() {
    let cfg = config::standard();
    let pricing = vec![
        DynamicPricingModel {
            ticket_type: "1-Day Park Hopper".to_string(),
            base_price_cents: 15900,
            date_epoch: 1700000000,
            demand_multiplier: 1.35,
            holiday_surcharge_cents: 2000,
            final_price_cents: 23465,
            tier: "Peak".to_string(),
            capacity_remaining_pct: 12.5,
            early_bird_discount_pct: 0,
            group_discount_threshold: 10,
        },
        DynamicPricingModel {
            ticket_type: "1-Day Single Park".to_string(),
            base_price_cents: 10900,
            date_epoch: 1700604800,
            demand_multiplier: 0.85,
            holiday_surcharge_cents: 0,
            final_price_cents: 9265,
            tier: "Value".to_string(),
            capacity_remaining_pct: 68.0,
            early_bird_discount_pct: 10,
            group_discount_threshold: 15,
        },
    ];
    let encoded = encode_to_vec(&pricing, cfg).expect("encode dynamic pricing models");
    let (decoded, _): (Vec<DynamicPricingModel>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode dynamic pricing models");
    assert_eq!(pricing, decoded);
}

#[test]
fn test_park_zone_heatmap_roundtrip() {
    let cfg = config::standard();
    let zones = vec![
        ParkZoneHeatmap {
            zone_id: 1,
            zone_name: "Main Street USA".to_string(),
            current_guests: 3200,
            max_capacity: 5000,
            density_per_sqm: 0.42,
            avg_dwell_time_mins: 18,
            entry_rate_per_min: 85,
            exit_rate_per_min: 72,
        },
        ParkZoneHeatmap {
            zone_id: 2,
            zone_name: "Adventureland".to_string(),
            current_guests: 4800,
            max_capacity: 8000,
            density_per_sqm: 0.31,
            avg_dwell_time_mins: 45,
            entry_rate_per_min: 60,
            exit_rate_per_min: 55,
        },
        ParkZoneHeatmap {
            zone_id: 3,
            zone_name: "Tomorrowland".to_string(),
            current_guests: 6100,
            max_capacity: 7500,
            density_per_sqm: 0.58,
            avg_dwell_time_mins: 52,
            entry_rate_per_min: 90,
            exit_rate_per_min: 65,
        },
    ];
    let encoded = encode_to_vec(&zones, cfg).expect("encode park zone heatmap");
    let (decoded, _): (Vec<ParkZoneHeatmap>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode park zone heatmap");
    assert_eq!(zones, decoded);
}

#[test]
fn test_ride_maintenance_log_roundtrip() {
    let cfg = config::standard();
    let log = RideMaintenanceLog {
        log_id: 55001,
        ride_id: 2005,
        ride_name: "Splash Canyon Rapids".to_string(),
        maintenance_type: "Scheduled Preventive".to_string(),
        technician: "James Okoro".to_string(),
        start_epoch: 1699850000,
        end_epoch: Some(1699857200),
        parts_replaced: vec![
            "Water pump seal assembly".to_string(),
            "Conveyor belt segment #12".to_string(),
            "Proximity sensor unit B4".to_string(),
        ],
        cost_cents: 1250000,
        downtime_mins: 120,
        is_emergency: false,
    };
    let encoded = encode_to_vec(&log, cfg).expect("encode ride maintenance log");
    let (decoded, _): (RideMaintenanceLog, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ride maintenance log");
    assert_eq!(log, decoded);
}

#[test]
fn test_food_transaction_no_guest_id_roundtrip() {
    let cfg = config::standard();
    let transaction = FoodBeverageTransaction {
        transaction_id: 9900055,
        register_id: 3,
        outlet_name: "Enchanted Bakery".to_string(),
        items: vec![PosLineItem {
            item_name: "Royal Cinnamon Roll".to_string(),
            sku: "FB-BKR-011".to_string(),
            quantity: 1,
            unit_price_cents: 599,
            is_meal_deal: false,
        }],
        subtotal_cents: 599,
        tax_cents: 48,
        total_cents: 647,
        payment_method: "cash".to_string(),
        timestamp_epoch: 1700055000,
        guest_id: None,
    };
    let encoded = encode_to_vec(&transaction, cfg).expect("encode anonymous food transaction");
    let (decoded, _): (FoodBeverageTransaction, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode anonymous food transaction");
    assert_eq!(transaction, decoded);
}

#[test]
fn test_annual_pass_no_blackout_dates_roundtrip() {
    let cfg = config::standard();
    let holder = AnnualPassHolder {
        pass_id: 770099,
        guest_name: "Maximilian Richter".to_string(),
        tier: "Diamond".to_string(),
        purchase_date_epoch: 1680000000,
        expiry_date_epoch: 1711536000,
        visits_this_year: 112,
        home_park: "Dreamland Resort".to_string(),
        blackout_dates: vec![],
        parking_included: true,
        photopass_included: true,
        discount_pct: 30,
    };
    let encoded = encode_to_vec(&holder, cfg).expect("encode diamond pass holder");
    let (decoded, _): (AnnualPassHolder, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode diamond pass holder");
    assert_eq!(holder, decoded);
}

#[test]
fn test_emergency_maintenance_open_ended_roundtrip() {
    let cfg = config::standard();
    let log = RideMaintenanceLog {
        log_id: 55099,
        ride_id: 1001,
        ride_name: "Thunder Mountain Express".to_string(),
        maintenance_type: "Emergency".to_string(),
        technician: "Priya Sharma".to_string(),
        start_epoch: 1700095000,
        end_epoch: None,
        parts_replaced: vec![],
        cost_cents: 0,
        downtime_mins: 0,
        is_emergency: true,
    };
    let encoded = encode_to_vec(&log, cfg).expect("encode emergency maintenance log");
    let (decoded, _): (RideMaintenanceLog, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode emergency maintenance log");
    assert_eq!(log, decoded);
}
