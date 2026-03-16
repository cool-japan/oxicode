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

// --- Domain types: Taxi & Rideshare Fleet Management ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TripRequest {
    request_id: u64,
    rider_id: u64,
    pickup_lat: f64,
    pickup_lng: f64,
    dropoff_lat: f64,
    dropoff_lng: f64,
    requested_at_epoch_ms: u64,
    passenger_count: u8,
    luggage_pieces: u8,
    accessibility_needed: bool,
    preferred_vehicle: VehicleClass,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum VehicleClass {
    Economy,
    Comfort,
    Premium,
    Xl,
    Wheelchair,
    GreenEv,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DriverProfile {
    driver_id: u64,
    license_number: String,
    first_name: String,
    last_name: String,
    phone_hash: String,
    years_experience: u8,
    languages: Vec<String>,
    is_active: bool,
    onboarded_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VehicleSpec {
    vehicle_id: u64,
    driver_id: u64,
    make: String,
    model: String,
    year: u16,
    color: String,
    plate_number: String,
    seat_capacity: u8,
    fuel_type: FuelType,
    odometer_km: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FuelType {
    Gasoline,
    Diesel,
    Hybrid,
    Electric,
    Cng,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SurgePricingZone {
    zone_id: u32,
    zone_name: String,
    center_lat: f64,
    center_lng: f64,
    radius_m: f64,
    multiplier_x100: u32,
    demand_score: u16,
    supply_count: u16,
    computed_at_epoch_ms: u64,
    ttl_seconds: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RouteWaypoint {
    waypoint_idx: u16,
    lat: f64,
    lng: f64,
    street_name: String,
    instruction: NavigationInstruction,
    distance_to_next_m: f64,
    eta_to_next_s: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum NavigationInstruction {
    Straight,
    TurnLeft,
    TurnRight,
    SlightLeft,
    SlightRight,
    UTurn,
    MergeHighway,
    ExitHighway,
    Arrive,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PaymentTransaction {
    transaction_id: String,
    trip_id: u64,
    rider_id: u64,
    driver_id: u64,
    fare_cents: u64,
    tip_cents: u64,
    toll_cents: u32,
    tax_cents: u32,
    platform_fee_cents: u32,
    payment_method: PaymentMethod,
    currency_code: String,
    status: PaymentStatus,
    processed_at_epoch_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PaymentMethod {
    CreditCard,
    DebitCard,
    DigitalWallet,
    Cash,
    CorporateAccount,
    Prepaid,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PaymentStatus {
    Pending,
    Authorized,
    Captured,
    Refunded,
    Failed,
    Disputed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DriverRatingAggregation {
    driver_id: u64,
    total_trips: u32,
    total_stars: u64,
    five_star_count: u32,
    four_star_count: u32,
    three_star_count: u32,
    two_star_count: u32,
    one_star_count: u32,
    average_rating_x100: u32,
    acceptance_rate_x100: u16,
    cancellation_rate_x100: u16,
    compliments: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DispatchState {
    Searching,
    DriverAssigned {
        driver_id: u64,
        eta_seconds: u32,
    },
    EnRouteToPickup {
        driver_id: u64,
        remaining_m: f64,
    },
    WaitingAtPickup {
        driver_id: u64,
        wait_start_epoch: u64,
    },
    InProgress {
        driver_id: u64,
        elapsed_s: u32,
    },
    Completed {
        driver_id: u64,
        total_fare_cents: u64,
    },
    CancelledByRider {
        reason: String,
    },
    CancelledByDriver {
        driver_id: u64,
        reason: String,
    },
    NoDriversAvailable,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DispatchAssignment {
    dispatch_id: u64,
    trip_request_id: u64,
    state: DispatchState,
    created_at_epoch_ms: u64,
    updated_at_epoch_ms: u64,
    retry_count: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VehicleTelematicsSnapshot {
    vehicle_id: u64,
    timestamp_epoch_ms: u64,
    lat: f64,
    lng: f64,
    speed_kmh: f32,
    heading_degrees: f32,
    engine_rpm: u16,
    fuel_level_pct: u8,
    battery_voltage: f32,
    tire_pressure_kpa: [u16; 4],
    abs_active: bool,
    check_engine: bool,
    odometer_km: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RegulatoryComplianceRecord {
    record_id: u64,
    driver_id: u64,
    vehicle_id: u64,
    license_type: LicenseType,
    license_number: String,
    issuing_authority: String,
    issued_epoch: u64,
    expiry_epoch: u64,
    inspection_passed: bool,
    insurance_policy_number: String,
    insurance_expiry_epoch: u64,
    background_check_clear: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum LicenseType {
    TlcDriver,
    PucPermit,
    TaxiMedallion,
    PhvLicense,
    LimoLicense,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CarpoolMatch {
    match_id: u64,
    anchor_trip_id: u64,
    matched_trip_ids: Vec<u64>,
    combined_route_distance_m: f64,
    individual_distances_m: Vec<f64>,
    savings_pct_x100: u16,
    detour_minutes: Vec<u16>,
    max_passengers: u8,
    current_passengers: u8,
    compatibility_score_x100: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AirportQueueEntry {
    queue_id: u64,
    airport_code: String,
    terminal: String,
    driver_id: u64,
    vehicle_class: VehicleClass,
    entered_queue_epoch_ms: u64,
    position: u32,
    estimated_wait_minutes: u16,
    geofence_verified: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FareEstimateBreakdown {
    estimate_id: u64,
    base_fare_cents: u32,
    per_km_rate_cents: u32,
    per_minute_rate_cents: u32,
    estimated_distance_m: f64,
    estimated_duration_s: u32,
    distance_charge_cents: u32,
    time_charge_cents: u32,
    surge_multiplier_x100: u32,
    surge_extra_cents: u32,
    booking_fee_cents: u32,
    toll_estimate_cents: u32,
    airport_surcharge_cents: u32,
    total_estimate_low_cents: u64,
    total_estimate_high_cents: u64,
    currency: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DriverShift {
    shift_id: u64,
    driver_id: u64,
    started_epoch_ms: u64,
    ended_epoch_ms: Option<u64>,
    total_trips: u16,
    total_online_minutes: u32,
    total_driving_minutes: u32,
    total_idle_minutes: u32,
    total_earnings_cents: u64,
    regions_served: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct IncidentReport {
    incident_id: u64,
    trip_id: u64,
    reporter_role: ReporterRole,
    reporter_id: u64,
    category: IncidentCategory,
    description: String,
    severity: u8,
    reported_at_epoch_ms: u64,
    location_lat: f64,
    location_lng: f64,
    resolved: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ReporterRole {
    Rider,
    Driver,
    ThirdParty,
    System,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum IncidentCategory {
    Accident,
    Harassment,
    RouteDeviation,
    UnsafeVehicle,
    PaymentDispute,
    ItemLeftBehind,
    Other,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PromoCode {
    code: String,
    discount_type: DiscountType,
    max_uses: u32,
    current_uses: u32,
    min_fare_cents: u32,
    valid_from_epoch: u64,
    valid_until_epoch: u64,
    valid_regions: Vec<String>,
    first_ride_only: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DiscountType {
    FixedCents(u32),
    PercentageX100(u16),
    FreeRide,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HeatmapCell {
    cell_row: u32,
    cell_col: u32,
    demand_count: u32,
    supply_count: u32,
    avg_wait_seconds: u32,
    avg_surge_x100: u32,
    period_start_epoch: u64,
    period_end_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DriverIncentive {
    incentive_id: u64,
    driver_id: u64,
    incentive_type: IncentiveType,
    target_trips: u16,
    completed_trips: u16,
    bonus_cents: u32,
    zone_restriction: Option<String>,
    start_epoch: u64,
    end_epoch: u64,
    claimed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum IncentiveType {
    QuestBonus,
    ConsecutiveTripBonus,
    PeakHourGuarantee,
    NewDriverBonus,
    ReferralReward,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RiderPreferences {
    rider_id: u64,
    preferred_temperature_c: Option<i8>,
    quiet_ride: bool,
    preferred_music: Option<String>,
    preferred_route: RoutePreference,
    default_tip_pct_x100: u16,
    saved_locations: Vec<SavedLocation>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RoutePreference {
    Fastest,
    Shortest,
    AvoidHighways,
    AvoidTolls,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SavedLocation {
    label: String,
    lat: f64,
    lng: f64,
    address: String,
}

// --- Tests ---

#[test]
fn test_trip_request_roundtrip() {
    let cfg = config::standard();
    let req = TripRequest {
        request_id: 9_000_001,
        rider_id: 42_000,
        pickup_lat: 40.748817,
        pickup_lng: -73.985428,
        dropoff_lat: 40.758896,
        dropoff_lng: -73.985130,
        requested_at_epoch_ms: 1_710_500_000_000,
        passenger_count: 2,
        luggage_pieces: 1,
        accessibility_needed: false,
        preferred_vehicle: VehicleClass::Comfort,
    };
    let bytes = encode_to_vec(&req, cfg).expect("encode TripRequest");
    let (decoded, _): (TripRequest, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TripRequest");
    assert_eq!(req, decoded);
}

#[test]
fn test_driver_profile_roundtrip() {
    let cfg = config::standard();
    let profile = DriverProfile {
        driver_id: 88_100,
        license_number: "TLC-5012847".to_string(),
        first_name: "Alejandro".to_string(),
        last_name: "Reyes".to_string(),
        phone_hash: "a3f9c8b2e1d7".to_string(),
        years_experience: 7,
        languages: vec![
            "English".to_string(),
            "Spanish".to_string(),
            "Portuguese".to_string(),
        ],
        is_active: true,
        onboarded_epoch: 1_600_000_000,
    };
    let bytes = encode_to_vec(&profile, cfg).expect("encode DriverProfile");
    let (decoded, _): (DriverProfile, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DriverProfile");
    assert_eq!(profile, decoded);
}

#[test]
fn test_vehicle_spec_electric_roundtrip() {
    let cfg = config::standard();
    let spec = VehicleSpec {
        vehicle_id: 55_200,
        driver_id: 88_100,
        make: "Tesla".to_string(),
        model: "Model Y".to_string(),
        year: 2024,
        color: "Midnight Silver".to_string(),
        plate_number: "T9X-1234".to_string(),
        seat_capacity: 5,
        fuel_type: FuelType::Electric,
        odometer_km: 23_450,
    };
    let bytes = encode_to_vec(&spec, cfg).expect("encode VehicleSpec");
    let (decoded, _): (VehicleSpec, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VehicleSpec");
    assert_eq!(spec, decoded);
}

#[test]
fn test_surge_pricing_zone_roundtrip() {
    let cfg = config::standard();
    let zone = SurgePricingZone {
        zone_id: 301,
        zone_name: "Midtown Manhattan".to_string(),
        center_lat: 40.7549,
        center_lng: -73.9840,
        radius_m: 1500.0,
        multiplier_x100: 275,
        demand_score: 890,
        supply_count: 34,
        computed_at_epoch_ms: 1_710_501_000_000,
        ttl_seconds: 120,
    };
    let bytes = encode_to_vec(&zone, cfg).expect("encode SurgePricingZone");
    let (decoded, _): (SurgePricingZone, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SurgePricingZone");
    assert_eq!(zone, decoded);
}

#[test]
fn test_route_waypoint_roundtrip() {
    let cfg = config::standard();
    let wp = RouteWaypoint {
        waypoint_idx: 3,
        lat: 40.7527,
        lng: -73.9772,
        street_name: "E 42nd St".to_string(),
        instruction: NavigationInstruction::TurnRight,
        distance_to_next_m: 320.5,
        eta_to_next_s: 45,
    };
    let bytes = encode_to_vec(&wp, cfg).expect("encode RouteWaypoint");
    let (decoded, _): (RouteWaypoint, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RouteWaypoint");
    assert_eq!(wp, decoded);
}

#[test]
fn test_payment_transaction_roundtrip() {
    let cfg = config::standard();
    let tx = PaymentTransaction {
        transaction_id: "txn_abc123def456".to_string(),
        trip_id: 9_000_001,
        rider_id: 42_000,
        driver_id: 88_100,
        fare_cents: 3_450,
        tip_cents: 500,
        toll_cents: 675,
        tax_cents: 310,
        platform_fee_cents: 520,
        payment_method: PaymentMethod::DigitalWallet,
        currency_code: "USD".to_string(),
        status: PaymentStatus::Captured,
        processed_at_epoch_ms: 1_710_502_000_000,
    };
    let bytes = encode_to_vec(&tx, cfg).expect("encode PaymentTransaction");
    let (decoded, _): (PaymentTransaction, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PaymentTransaction");
    assert_eq!(tx, decoded);
}

#[test]
fn test_driver_rating_aggregation_roundtrip() {
    let cfg = config::standard();
    let rating = DriverRatingAggregation {
        driver_id: 88_100,
        total_trips: 4_312,
        total_stars: 20_847,
        five_star_count: 3_500,
        four_star_count: 600,
        three_star_count: 150,
        two_star_count: 40,
        one_star_count: 22,
        average_rating_x100: 483,
        acceptance_rate_x100: 9200,
        cancellation_rate_x100: 150,
        compliments: vec![
            "Great conversation".to_string(),
            "Smooth driver".to_string(),
            "Clean car".to_string(),
        ],
    };
    let bytes = encode_to_vec(&rating, cfg).expect("encode DriverRatingAggregation");
    let (decoded, _): (DriverRatingAggregation, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DriverRatingAggregation");
    assert_eq!(rating, decoded);
}

#[test]
fn test_dispatch_assigned_state_roundtrip() {
    let cfg = config::standard();
    let assignment = DispatchAssignment {
        dispatch_id: 770_001,
        trip_request_id: 9_000_001,
        state: DispatchState::DriverAssigned {
            driver_id: 88_100,
            eta_seconds: 240,
        },
        created_at_epoch_ms: 1_710_500_100_000,
        updated_at_epoch_ms: 1_710_500_105_000,
        retry_count: 0,
    };
    let bytes = encode_to_vec(&assignment, cfg).expect("encode DispatchAssignment assigned");
    let (decoded, _): (DispatchAssignment, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DispatchAssignment assigned");
    assert_eq!(assignment, decoded);
}

#[test]
fn test_dispatch_cancelled_by_driver_roundtrip() {
    let cfg = config::standard();
    let assignment = DispatchAssignment {
        dispatch_id: 770_002,
        trip_request_id: 9_000_002,
        state: DispatchState::CancelledByDriver {
            driver_id: 88_200,
            reason: "Passenger not at pickup location after 5 min wait".to_string(),
        },
        created_at_epoch_ms: 1_710_500_200_000,
        updated_at_epoch_ms: 1_710_500_510_000,
        retry_count: 1,
    };
    let bytes = encode_to_vec(&assignment, cfg).expect("encode DispatchAssignment cancelled");
    let (decoded, _): (DispatchAssignment, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DispatchAssignment cancelled");
    assert_eq!(assignment, decoded);
}

#[test]
fn test_vehicle_telematics_snapshot_roundtrip() {
    let cfg = config::standard();
    let snap = VehicleTelematicsSnapshot {
        vehicle_id: 55_200,
        timestamp_epoch_ms: 1_710_503_000_000,
        lat: 40.7580,
        lng: -73.9855,
        speed_kmh: 32.5,
        heading_degrees: 178.3,
        engine_rpm: 2100,
        fuel_level_pct: 72,
        battery_voltage: 12.6,
        tire_pressure_kpa: [230, 228, 232, 231],
        abs_active: false,
        check_engine: false,
        odometer_km: 23_452,
    };
    let bytes = encode_to_vec(&snap, cfg).expect("encode VehicleTelematicsSnapshot");
    let (decoded, _): (VehicleTelematicsSnapshot, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VehicleTelematicsSnapshot");
    assert_eq!(snap, decoded);
}

#[test]
fn test_regulatory_compliance_record_roundtrip() {
    let cfg = config::standard();
    let rec = RegulatoryComplianceRecord {
        record_id: 100_500,
        driver_id: 88_100,
        vehicle_id: 55_200,
        license_type: LicenseType::TlcDriver,
        license_number: "TLC-5012847".to_string(),
        issuing_authority: "NYC TLC".to_string(),
        issued_epoch: 1_672_531_200,
        expiry_epoch: 1_735_689_600,
        inspection_passed: true,
        insurance_policy_number: "INS-NYC-2024-88100".to_string(),
        insurance_expiry_epoch: 1_735_689_600,
        background_check_clear: true,
    };
    let bytes = encode_to_vec(&rec, cfg).expect("encode RegulatoryComplianceRecord");
    let (decoded, _): (RegulatoryComplianceRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RegulatoryComplianceRecord");
    assert_eq!(rec, decoded);
}

#[test]
fn test_carpool_match_roundtrip() {
    let cfg = config::standard();
    let carpool = CarpoolMatch {
        match_id: 660_001,
        anchor_trip_id: 9_000_010,
        matched_trip_ids: vec![9_000_011, 9_000_012],
        combined_route_distance_m: 12_450.0,
        individual_distances_m: vec![8_200.0, 6_300.0, 7_100.0],
        savings_pct_x100: 3500,
        detour_minutes: vec![3, 5, 2],
        max_passengers: 4,
        current_passengers: 3,
        compatibility_score_x100: 8700,
    };
    let bytes = encode_to_vec(&carpool, cfg).expect("encode CarpoolMatch");
    let (decoded, _): (CarpoolMatch, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CarpoolMatch");
    assert_eq!(carpool, decoded);
}

#[test]
fn test_airport_queue_entry_roundtrip() {
    let cfg = config::standard();
    let entry = AirportQueueEntry {
        queue_id: 440_001,
        airport_code: "JFK".to_string(),
        terminal: "Terminal 4".to_string(),
        driver_id: 88_300,
        vehicle_class: VehicleClass::Premium,
        entered_queue_epoch_ms: 1_710_504_000_000,
        position: 17,
        estimated_wait_minutes: 35,
        geofence_verified: true,
    };
    let bytes = encode_to_vec(&entry, cfg).expect("encode AirportQueueEntry");
    let (decoded, _): (AirportQueueEntry, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AirportQueueEntry");
    assert_eq!(entry, decoded);
}

#[test]
fn test_fare_estimate_breakdown_roundtrip() {
    let cfg = config::standard();
    let estimate = FareEstimateBreakdown {
        estimate_id: 550_001,
        base_fare_cents: 275,
        per_km_rate_cents: 155,
        per_minute_rate_cents: 35,
        estimated_distance_m: 8_500.0,
        estimated_duration_s: 1_200,
        distance_charge_cents: 1_318,
        time_charge_cents: 700,
        surge_multiplier_x100: 175,
        surge_extra_cents: 1_147,
        booking_fee_cents: 250,
        toll_estimate_cents: 675,
        airport_surcharge_cents: 0,
        total_estimate_low_cents: 4_000,
        total_estimate_high_cents: 5_200,
        currency: "USD".to_string(),
    };
    let bytes = encode_to_vec(&estimate, cfg).expect("encode FareEstimateBreakdown");
    let (decoded, _): (FareEstimateBreakdown, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FareEstimateBreakdown");
    assert_eq!(estimate, decoded);
}

#[test]
fn test_driver_shift_active_roundtrip() {
    let cfg = config::standard();
    let shift = DriverShift {
        shift_id: 330_001,
        driver_id: 88_100,
        started_epoch_ms: 1_710_480_000_000,
        ended_epoch_ms: None,
        total_trips: 8,
        total_online_minutes: 210,
        total_driving_minutes: 145,
        total_idle_minutes: 65,
        total_earnings_cents: 28_750,
        regions_served: vec!["Manhattan".to_string(), "Brooklyn".to_string()],
    };
    let bytes = encode_to_vec(&shift, cfg).expect("encode DriverShift active");
    let (decoded, _): (DriverShift, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DriverShift active");
    assert_eq!(shift, decoded);
}

#[test]
fn test_driver_shift_ended_roundtrip() {
    let cfg = config::standard();
    let shift = DriverShift {
        shift_id: 330_002,
        driver_id: 88_100,
        started_epoch_ms: 1_710_440_000_000,
        ended_epoch_ms: Some(1_710_480_000_000),
        total_trips: 14,
        total_online_minutes: 660,
        total_driving_minutes: 480,
        total_idle_minutes: 180,
        total_earnings_cents: 52_300,
        regions_served: vec![
            "Manhattan".to_string(),
            "Queens".to_string(),
            "Bronx".to_string(),
        ],
    };
    let bytes = encode_to_vec(&shift, cfg).expect("encode DriverShift ended");
    let (decoded, _): (DriverShift, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DriverShift ended");
    assert_eq!(shift, decoded);
}

#[test]
fn test_incident_report_roundtrip() {
    let cfg = config::standard();
    let incident = IncidentReport {
        incident_id: 220_001,
        trip_id: 9_000_050,
        reporter_role: ReporterRole::Rider,
        reporter_id: 42_100,
        category: IncidentCategory::ItemLeftBehind,
        description: "Left a black laptop bag on the back seat".to_string(),
        severity: 2,
        reported_at_epoch_ms: 1_710_506_000_000,
        location_lat: 40.7614,
        location_lng: -73.9776,
        resolved: false,
    };
    let bytes = encode_to_vec(&incident, cfg).expect("encode IncidentReport");
    let (decoded, _): (IncidentReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode IncidentReport");
    assert_eq!(incident, decoded);
}

#[test]
fn test_promo_code_fixed_discount_roundtrip() {
    let cfg = config::standard();
    let promo = PromoCode {
        code: "WELCOME2024".to_string(),
        discount_type: DiscountType::FixedCents(500),
        max_uses: 100_000,
        current_uses: 47_320,
        min_fare_cents: 1_000,
        valid_from_epoch: 1_704_067_200,
        valid_until_epoch: 1_735_689_600,
        valid_regions: vec!["NYC".to_string(), "LAX".to_string(), "CHI".to_string()],
        first_ride_only: true,
    };
    let bytes = encode_to_vec(&promo, cfg).expect("encode PromoCode fixed");
    let (decoded, _): (PromoCode, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PromoCode fixed");
    assert_eq!(promo, decoded);
}

#[test]
fn test_promo_code_percentage_discount_roundtrip() {
    let cfg = config::standard();
    let promo = PromoCode {
        code: "HOLIDAY20".to_string(),
        discount_type: DiscountType::PercentageX100(2000),
        max_uses: 50_000,
        current_uses: 12_500,
        min_fare_cents: 800,
        valid_from_epoch: 1_703_980_800,
        valid_until_epoch: 1_704_585_600,
        valid_regions: vec!["NYC".to_string()],
        first_ride_only: false,
    };
    let bytes = encode_to_vec(&promo, cfg).expect("encode PromoCode pct");
    let (decoded, _): (PromoCode, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PromoCode pct");
    assert_eq!(promo, decoded);
}

#[test]
fn test_heatmap_cell_roundtrip() {
    let cfg = config::standard();
    let cell = HeatmapCell {
        cell_row: 145,
        cell_col: 203,
        demand_count: 87,
        supply_count: 12,
        avg_wait_seconds: 480,
        avg_surge_x100: 220,
        period_start_epoch: 1_710_500_000,
        period_end_epoch: 1_710_500_900,
    };
    let bytes = encode_to_vec(&cell, cfg).expect("encode HeatmapCell");
    let (decoded, _): (HeatmapCell, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HeatmapCell");
    assert_eq!(cell, decoded);
}

#[test]
fn test_driver_incentive_roundtrip() {
    let cfg = config::standard();
    let incentive = DriverIncentive {
        incentive_id: 880_001,
        driver_id: 88_100,
        incentive_type: IncentiveType::QuestBonus,
        target_trips: 20,
        completed_trips: 14,
        bonus_cents: 5_000,
        zone_restriction: Some("Manhattan".to_string()),
        start_epoch: 1_710_460_800,
        end_epoch: 1_710_547_200,
        claimed: false,
    };
    let bytes = encode_to_vec(&incentive, cfg).expect("encode DriverIncentive");
    let (decoded, _): (DriverIncentive, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DriverIncentive");
    assert_eq!(incentive, decoded);
}

#[test]
fn test_rider_preferences_roundtrip() {
    let cfg = config::standard();
    let prefs = RiderPreferences {
        rider_id: 42_000,
        preferred_temperature_c: Some(21),
        quiet_ride: true,
        preferred_music: Some("Jazz".to_string()),
        preferred_route: RoutePreference::AvoidTolls,
        default_tip_pct_x100: 1800,
        saved_locations: vec![
            SavedLocation {
                label: "Home".to_string(),
                lat: 40.7282,
                lng: -73.7949,
                address: "123 Elm Street, Queens, NY 11375".to_string(),
            },
            SavedLocation {
                label: "Work".to_string(),
                lat: 40.7580,
                lng: -73.9855,
                address: "1 Times Square, Manhattan, NY 10036".to_string(),
            },
        ],
    };
    let bytes = encode_to_vec(&prefs, cfg).expect("encode RiderPreferences");
    let (decoded, _): (RiderPreferences, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode RiderPreferences");
    assert_eq!(prefs, decoded);
}
