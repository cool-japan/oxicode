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

// --- Domain types: Last-mile delivery and courier operations ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GpsCoordinate {
    latitude: f64,
    longitude: f64,
    accuracy_meters: f32,
    timestamp_epoch_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DeliveryRouteStop {
    stop_index: u32,
    address: String,
    parcel_ids: Vec<String>,
    location: GpsCoordinate,
    estimated_service_time_secs: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct OptimizedRoute {
    route_id: String,
    driver_id: String,
    vehicle_id: String,
    stops: Vec<DeliveryRouteStop>,
    total_distance_meters: u64,
    total_estimated_time_secs: u64,
    optimization_algorithm: String,
    optimization_score: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PackageDimension {
    scan_id: String,
    parcel_id: String,
    length_cm: f32,
    width_cm: f32,
    height_cm: f32,
    weight_grams: u32,
    volumetric_weight_grams: u32,
    billable_weight_grams: u32,
    scanned_at_epoch_ms: u64,
    scanner_device_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DriverShiftSchedule {
    schedule_id: String,
    driver_id: String,
    driver_name: String,
    shift_date: String,
    start_epoch_ms: u64,
    end_epoch_ms: u64,
    break_windows: Vec<(u64, u64)>,
    zone_assignments: Vec<String>,
    max_parcels: u32,
    vehicle_type: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ProofOfDelivery {
    delivery_id: String,
    parcel_id: String,
    proof_type: String,
    photo_url: Option<String>,
    signature_data_base64: Option<String>,
    gps_location: GpsCoordinate,
    recipient_name: Option<String>,
    delivered_at_epoch_ms: u64,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DeliveryAttemptRecord {
    attempt_id: String,
    parcel_id: String,
    attempt_number: u32,
    attempted_at_epoch_ms: u64,
    outcome: String,
    failure_reason: Option<String>,
    driver_id: String,
    gps_location: GpsCoordinate,
    customer_contacted: bool,
    next_attempt_scheduled_epoch_ms: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LockerAllocation {
    allocation_id: String,
    locker_station_id: String,
    locker_station_name: String,
    locker_station_address: String,
    compartment_id: String,
    compartment_size: String,
    parcel_id: String,
    access_code: String,
    allocated_at_epoch_ms: u64,
    expires_at_epoch_ms: u64,
    picked_up: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PickupPointAllocation {
    allocation_id: String,
    pickup_point_id: String,
    pickup_point_name: String,
    partner_store_name: String,
    address: String,
    parcel_ids: Vec<String>,
    shelf_slot: String,
    ready_at_epoch_ms: u64,
    hold_until_epoch_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VehicleCapacityPlan {
    plan_id: String,
    vehicle_id: String,
    vehicle_type: String,
    max_weight_grams: u64,
    max_volume_cm3: u64,
    current_weight_grams: u64,
    current_volume_cm3: u64,
    parcel_count: u32,
    max_parcel_count: u32,
    utilization_pct: f32,
    overweight: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RealTimeTrackingUpdate {
    tracking_id: String,
    parcel_id: String,
    status: String,
    location: GpsCoordinate,
    speed_kmh: f32,
    heading_degrees: f32,
    eta_epoch_ms: Option<u64>,
    stops_remaining: u32,
    updated_at_epoch_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TimeWindow {
    start_epoch_ms: u64,
    end_epoch_ms: u64,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CustomerDeliveryPreference {
    customer_id: String,
    preferred_time_windows: Vec<TimeWindow>,
    safe_spot_description: Option<String>,
    leave_with_neighbor: bool,
    neighbor_name: Option<String>,
    delivery_instructions: String,
    requires_signature: bool,
    preferred_locker_station_id: Option<String>,
    sms_notifications: bool,
    email_notifications: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FailedDeliveryReattempt {
    reattempt_id: String,
    original_attempt_id: String,
    parcel_id: String,
    customer_id: String,
    reason_code: String,
    reason_detail: String,
    reattempt_date: String,
    reattempt_time_window: TimeWindow,
    updated_address: Option<String>,
    redirect_to_locker: bool,
    redirect_locker_id: Option<String>,
    priority_boost: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CrossDockSortation {
    sortation_id: String,
    facility_id: String,
    inbound_trailer_id: String,
    outbound_lane: String,
    parcels_sorted: Vec<String>,
    sort_start_epoch_ms: u64,
    sort_end_epoch_ms: u64,
    misrouted_count: u32,
    damaged_count: u32,
    total_processed: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DynamicEtaPrediction {
    prediction_id: String,
    parcel_id: String,
    driver_id: String,
    predicted_arrival_epoch_ms: u64,
    confidence_pct: f32,
    model_version: String,
    factors: Vec<String>,
    traffic_delay_secs: u32,
    weather_delay_secs: u32,
    previous_prediction_epoch_ms: Option<u64>,
    drift_secs: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DriverPerformanceScorecard {
    scorecard_id: String,
    driver_id: String,
    driver_name: String,
    period: String,
    deliveries_completed: u32,
    deliveries_attempted: u32,
    on_time_pct: f32,
    first_attempt_success_pct: f32,
    customer_complaints: u32,
    safety_incidents: u32,
    avg_delivery_time_secs: u32,
    parcels_per_hour: f32,
    overall_score: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FuelMileageReimbursement {
    reimbursement_id: String,
    driver_id: String,
    vehicle_id: String,
    period_start: String,
    period_end: String,
    total_distance_km: f64,
    fuel_consumed_liters: f64,
    fuel_cost_cents: u64,
    rate_per_km_cents: u32,
    distance_reimbursement_cents: u64,
    fuel_reimbursement_cents: u64,
    total_reimbursement_cents: u64,
    approved: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CustomerSatisfactionScore {
    survey_id: String,
    customer_id: String,
    delivery_id: String,
    nps_score: i8,
    csat_score: u8,
    effort_score: u8,
    feedback_text: Option<String>,
    tags: Vec<String>,
    submitted_at_epoch_ms: u64,
    driver_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RouteOptimizationRequest {
    request_id: String,
    depot_location: GpsCoordinate,
    stops: Vec<DeliveryRouteStop>,
    vehicle_capacity_grams: u64,
    vehicle_capacity_cm3: u64,
    shift_start_epoch_ms: u64,
    shift_end_epoch_ms: u64,
    avoid_tolls: bool,
    prefer_highways: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BatchTrackingEvent {
    batch_id: String,
    events: Vec<RealTimeTrackingUpdate>,
    source_system: String,
    ingested_at_epoch_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DeliveryZoneBoundary {
    zone_id: String,
    zone_name: String,
    polygon_vertices: Vec<(f64, f64)>,
    active: bool,
    max_drivers: u32,
    avg_density_parcels_per_km2: f32,
}

// --- Tests ---

#[test]
fn test_delivery_route_optimization() {
    let cfg = config::standard();
    let route = OptimizedRoute {
        route_id: "ROUTE-20260315-001".to_string(),
        driver_id: "DRV-4421".to_string(),
        vehicle_id: "VAN-088".to_string(),
        stops: vec![
            DeliveryRouteStop {
                stop_index: 0,
                address: "12 Sakura Lane, Shibuya".to_string(),
                parcel_ids: vec!["PKG-90001".to_string(), "PKG-90002".to_string()],
                location: GpsCoordinate {
                    latitude: 35.6580,
                    longitude: 139.7016,
                    accuracy_meters: 4.5,
                    timestamp_epoch_ms: 1710489600000,
                },
                estimated_service_time_secs: 180,
            },
            DeliveryRouteStop {
                stop_index: 1,
                address: "7-3 Minami-Aoyama".to_string(),
                parcel_ids: vec!["PKG-90003".to_string()],
                location: GpsCoordinate {
                    latitude: 35.6636,
                    longitude: 139.7146,
                    accuracy_meters: 3.2,
                    timestamp_epoch_ms: 1710489900000,
                },
                estimated_service_time_secs: 120,
            },
        ],
        total_distance_meters: 14200,
        total_estimated_time_secs: 5400,
        optimization_algorithm: "or-tools-tsp-v3".to_string(),
        optimization_score: 0.924,
    };
    let encoded = encode_to_vec(&route, cfg).expect("encode optimized route");
    let (decoded, _): (OptimizedRoute, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode optimized route");
    assert_eq!(route, decoded);
}

#[test]
fn test_package_dimension_weight_scanning() {
    let cfg = config::standard();
    let scan = PackageDimension {
        scan_id: "SCN-55001".to_string(),
        parcel_id: "PKG-90001".to_string(),
        length_cm: 40.0,
        width_cm: 30.0,
        height_cm: 20.0,
        weight_grams: 3500,
        volumetric_weight_grams: 4800,
        billable_weight_grams: 4800,
        scanned_at_epoch_ms: 1710480000000,
        scanner_device_id: "CUBISCAN-A7".to_string(),
    };
    let encoded = encode_to_vec(&scan, cfg).expect("encode package scan");
    let (decoded, _): (PackageDimension, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode package scan");
    assert_eq!(scan, decoded);
}

#[test]
fn test_driver_shift_schedules() {
    let cfg = config::standard();
    let schedule = DriverShiftSchedule {
        schedule_id: "SCHED-20260315-DRV4421".to_string(),
        driver_id: "DRV-4421".to_string(),
        driver_name: "Tanaka Yuki".to_string(),
        shift_date: "2026-03-15".to_string(),
        start_epoch_ms: 1710478800000,
        end_epoch_ms: 1710514800000,
        break_windows: vec![
            (1710496800000, 1710500400000),
            (1710507600000, 1710508500000),
        ],
        zone_assignments: vec!["ZONE-SBY-01".to_string(), "ZONE-SBY-02".to_string()],
        max_parcels: 120,
        vehicle_type: "light-van".to_string(),
    };
    let encoded = encode_to_vec(&schedule, cfg).expect("encode shift schedule");
    let (decoded, _): (DriverShiftSchedule, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode shift schedule");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_proof_of_delivery_photo() {
    let cfg = config::standard();
    let pod = ProofOfDelivery {
        delivery_id: "DEL-70001".to_string(),
        parcel_id: "PKG-90001".to_string(),
        proof_type: "photo".to_string(),
        photo_url: Some("https://cdn.courier.example/pod/DEL-70001.jpg".to_string()),
        signature_data_base64: None,
        gps_location: GpsCoordinate {
            latitude: 35.6580,
            longitude: 139.7016,
            accuracy_meters: 3.0,
            timestamp_epoch_ms: 1710492000000,
        },
        recipient_name: None,
        delivered_at_epoch_ms: 1710492000000,
        notes: "Left at front door per customer instructions".to_string(),
    };
    let encoded = encode_to_vec(&pod, cfg).expect("encode proof of delivery photo");
    let (decoded, _): (ProofOfDelivery, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode proof of delivery photo");
    assert_eq!(pod, decoded);
}

#[test]
fn test_proof_of_delivery_signature() {
    let cfg = config::standard();
    let pod = ProofOfDelivery {
        delivery_id: "DEL-70002".to_string(),
        parcel_id: "PKG-90003".to_string(),
        proof_type: "signature".to_string(),
        photo_url: None,
        signature_data_base64: Some("iVBORw0KGgoAAAANSUhEUgAAAEAAAA...truncated...".to_string()),
        gps_location: GpsCoordinate {
            latitude: 35.6636,
            longitude: 139.7146,
            accuracy_meters: 2.8,
            timestamp_epoch_ms: 1710493200000,
        },
        recipient_name: Some("Suzuki Hana".to_string()),
        delivered_at_epoch_ms: 1710493200000,
        notes: "Signed by recipient at door".to_string(),
    };
    let encoded = encode_to_vec(&pod, cfg).expect("encode proof of delivery signature");
    let (decoded, _): (ProofOfDelivery, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode proof of delivery signature");
    assert_eq!(pod, decoded);
}

#[test]
fn test_delivery_attempt_records() {
    let cfg = config::standard();
    let attempt = DeliveryAttemptRecord {
        attempt_id: "ATT-80001".to_string(),
        parcel_id: "PKG-90004".to_string(),
        attempt_number: 1,
        attempted_at_epoch_ms: 1710494400000,
        outcome: "failed".to_string(),
        failure_reason: Some("customer_not_home".to_string()),
        driver_id: "DRV-4421".to_string(),
        gps_location: GpsCoordinate {
            latitude: 35.6712,
            longitude: 139.7035,
            accuracy_meters: 5.1,
            timestamp_epoch_ms: 1710494400000,
        },
        customer_contacted: true,
        next_attempt_scheduled_epoch_ms: Some(1710576000000),
    };
    let encoded = encode_to_vec(&attempt, cfg).expect("encode delivery attempt");
    let (decoded, _): (DeliveryAttemptRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode delivery attempt");
    assert_eq!(attempt, decoded);
}

#[test]
fn test_locker_allocation() {
    let cfg = config::standard();
    let alloc = LockerAllocation {
        allocation_id: "LOCK-60001".to_string(),
        locker_station_id: "LSTA-SBY-003".to_string(),
        locker_station_name: "Shibuya Station East Lockers".to_string(),
        locker_station_address: "2-1 Shibuya, Shibuya-ku".to_string(),
        compartment_id: "C-14".to_string(),
        compartment_size: "medium".to_string(),
        parcel_id: "PKG-90004".to_string(),
        access_code: "8832-5561".to_string(),
        allocated_at_epoch_ms: 1710496800000,
        expires_at_epoch_ms: 1710756000000,
        picked_up: false,
    };
    let encoded = encode_to_vec(&alloc, cfg).expect("encode locker allocation");
    let (decoded, _): (LockerAllocation, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode locker allocation");
    assert_eq!(alloc, decoded);
}

#[test]
fn test_pickup_point_allocation() {
    let cfg = config::standard();
    let pickup = PickupPointAllocation {
        allocation_id: "PUP-40001".to_string(),
        pickup_point_id: "PP-CONV-221".to_string(),
        pickup_point_name: "FamilyMart Aoyama 1-chome".to_string(),
        partner_store_name: "FamilyMart".to_string(),
        address: "1-3-5 Minami-Aoyama, Minato-ku".to_string(),
        parcel_ids: vec!["PKG-90005".to_string(), "PKG-90006".to_string()],
        shelf_slot: "B-07".to_string(),
        ready_at_epoch_ms: 1710500400000,
        hold_until_epoch_ms: 1710842400000,
    };
    let encoded = encode_to_vec(&pickup, cfg).expect("encode pickup point allocation");
    let (decoded, _): (PickupPointAllocation, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode pickup point allocation");
    assert_eq!(pickup, decoded);
}

#[test]
fn test_vehicle_capacity_planning() {
    let cfg = config::standard();
    let plan = VehicleCapacityPlan {
        plan_id: "VCAP-20260315-VAN088".to_string(),
        vehicle_id: "VAN-088".to_string(),
        vehicle_type: "light-van-1t".to_string(),
        max_weight_grams: 1_000_000,
        max_volume_cm3: 4_500_000,
        current_weight_grams: 756_200,
        current_volume_cm3: 3_100_000,
        parcel_count: 95,
        max_parcel_count: 120,
        utilization_pct: 75.6,
        overweight: false,
    };
    let encoded = encode_to_vec(&plan, cfg).expect("encode vehicle capacity plan");
    let (decoded, _): (VehicleCapacityPlan, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode vehicle capacity plan");
    assert_eq!(plan, decoded);
}

#[test]
fn test_real_time_tracking_updates() {
    let cfg = config::standard();
    let update = RealTimeTrackingUpdate {
        tracking_id: "TRK-20260315-001".to_string(),
        parcel_id: "PKG-90001".to_string(),
        status: "out_for_delivery".to_string(),
        location: GpsCoordinate {
            latitude: 35.6601,
            longitude: 139.7086,
            accuracy_meters: 8.0,
            timestamp_epoch_ms: 1710490800000,
        },
        speed_kmh: 28.5,
        heading_degrees: 135.0,
        eta_epoch_ms: Some(1710492000000),
        stops_remaining: 3,
        updated_at_epoch_ms: 1710490800000,
    };
    let encoded = encode_to_vec(&update, cfg).expect("encode tracking update");
    let (decoded, _): (RealTimeTrackingUpdate, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode tracking update");
    assert_eq!(update, decoded);
}

#[test]
fn test_customer_delivery_preferences() {
    let cfg = config::standard();
    let prefs = CustomerDeliveryPreference {
        customer_id: "CUST-11234".to_string(),
        preferred_time_windows: vec![
            TimeWindow {
                start_epoch_ms: 1710500400000,
                end_epoch_ms: 1710507600000,
                label: "evening".to_string(),
            },
            TimeWindow {
                start_epoch_ms: 1710478800000,
                end_epoch_ms: 1710482400000,
                label: "morning".to_string(),
            },
        ],
        safe_spot_description: Some(
            "Behind the meter box on the left side of entrance".to_string(),
        ),
        leave_with_neighbor: false,
        neighbor_name: None,
        delivery_instructions: "Ring doorbell twice. Do not leave if no answer.".to_string(),
        requires_signature: true,
        preferred_locker_station_id: None,
        sms_notifications: true,
        email_notifications: true,
    };
    let encoded = encode_to_vec(&prefs, cfg).expect("encode customer prefs");
    let (decoded, _): (CustomerDeliveryPreference, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode customer prefs");
    assert_eq!(prefs, decoded);
}

#[test]
fn test_failed_delivery_reattempt_scheduling() {
    let cfg = config::standard();
    let reattempt = FailedDeliveryReattempt {
        reattempt_id: "REATTEMPT-30001".to_string(),
        original_attempt_id: "ATT-80001".to_string(),
        parcel_id: "PKG-90004".to_string(),
        customer_id: "CUST-11234".to_string(),
        reason_code: "CUST_NOT_HOME".to_string(),
        reason_detail: "No answer after 2 attempts to ring bell".to_string(),
        reattempt_date: "2026-03-16".to_string(),
        reattempt_time_window: TimeWindow {
            start_epoch_ms: 1710586800000,
            end_epoch_ms: 1710594000000,
            label: "evening-next-day".to_string(),
        },
        updated_address: None,
        redirect_to_locker: false,
        redirect_locker_id: None,
        priority_boost: true,
    };
    let encoded = encode_to_vec(&reattempt, cfg).expect("encode reattempt");
    let (decoded, _): (FailedDeliveryReattempt, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode reattempt");
    assert_eq!(reattempt, decoded);
}

#[test]
fn test_cross_dock_sortation() {
    let cfg = config::standard();
    let sortation = CrossDockSortation {
        sortation_id: "SORT-20260315-A1".to_string(),
        facility_id: "FAC-TYO-EAST".to_string(),
        inbound_trailer_id: "TRL-8842".to_string(),
        outbound_lane: "LANE-SBY-07".to_string(),
        parcels_sorted: vec![
            "PKG-90001".to_string(),
            "PKG-90002".to_string(),
            "PKG-90003".to_string(),
            "PKG-90004".to_string(),
            "PKG-90005".to_string(),
        ],
        sort_start_epoch_ms: 1710471600000,
        sort_end_epoch_ms: 1710475200000,
        misrouted_count: 2,
        damaged_count: 0,
        total_processed: 347,
    };
    let encoded = encode_to_vec(&sortation, cfg).expect("encode cross-dock sortation");
    let (decoded, _): (CrossDockSortation, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode cross-dock sortation");
    assert_eq!(sortation, decoded);
}

#[test]
fn test_dynamic_eta_predictions() {
    let cfg = config::standard();
    let prediction = DynamicEtaPrediction {
        prediction_id: "ETA-20260315-PKG90001-003".to_string(),
        parcel_id: "PKG-90001".to_string(),
        driver_id: "DRV-4421".to_string(),
        predicted_arrival_epoch_ms: 1710492300000,
        confidence_pct: 87.3,
        model_version: "eta-lstm-v2.4".to_string(),
        factors: vec![
            "traffic_moderate".to_string(),
            "weather_clear".to_string(),
            "historical_fast_zone".to_string(),
        ],
        traffic_delay_secs: 240,
        weather_delay_secs: 0,
        previous_prediction_epoch_ms: Some(1710491400000),
        drift_secs: -120,
    };
    let encoded = encode_to_vec(&prediction, cfg).expect("encode eta prediction");
    let (decoded, _): (DynamicEtaPrediction, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode eta prediction");
    assert_eq!(prediction, decoded);
}

#[test]
fn test_driver_performance_scorecards() {
    let cfg = config::standard();
    let scorecard = DriverPerformanceScorecard {
        scorecard_id: "SC-DRV4421-202603".to_string(),
        driver_id: "DRV-4421".to_string(),
        driver_name: "Tanaka Yuki".to_string(),
        period: "2026-03".to_string(),
        deliveries_completed: 1842,
        deliveries_attempted: 1900,
        on_time_pct: 96.9,
        first_attempt_success_pct: 91.2,
        customer_complaints: 3,
        safety_incidents: 0,
        avg_delivery_time_secs: 145,
        parcels_per_hour: 14.8,
        overall_score: 92.5,
    };
    let encoded = encode_to_vec(&scorecard, cfg).expect("encode driver scorecard");
    let (decoded, _): (DriverPerformanceScorecard, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode driver scorecard");
    assert_eq!(scorecard, decoded);
}

#[test]
fn test_fuel_mileage_reimbursements() {
    let cfg = config::standard();
    let reimbursement = FuelMileageReimbursement {
        reimbursement_id: "REIMB-DRV4421-202603".to_string(),
        driver_id: "DRV-4421".to_string(),
        vehicle_id: "VAN-088".to_string(),
        period_start: "2026-03-01".to_string(),
        period_end: "2026-03-15".to_string(),
        total_distance_km: 1245.8,
        fuel_consumed_liters: 124.6,
        fuel_cost_cents: 2241000,
        rate_per_km_cents: 35,
        distance_reimbursement_cents: 4360300,
        fuel_reimbursement_cents: 2241000,
        total_reimbursement_cents: 6601300,
        approved: false,
    };
    let encoded = encode_to_vec(&reimbursement, cfg).expect("encode reimbursement");
    let (decoded, _): (FuelMileageReimbursement, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode reimbursement");
    assert_eq!(reimbursement, decoded);
}

#[test]
fn test_customer_satisfaction_nps_csat() {
    let cfg = config::standard();
    let score = CustomerSatisfactionScore {
        survey_id: "SURV-99001".to_string(),
        customer_id: "CUST-11234".to_string(),
        delivery_id: "DEL-70001".to_string(),
        nps_score: 9,
        csat_score: 5,
        effort_score: 2,
        feedback_text: Some("Very quick delivery, driver was polite".to_string()),
        tags: vec![
            "fast".to_string(),
            "polite".to_string(),
            "on-time".to_string(),
        ],
        submitted_at_epoch_ms: 1710496800000,
        driver_id: "DRV-4421".to_string(),
    };
    let encoded = encode_to_vec(&score, cfg).expect("encode satisfaction score");
    let (decoded, _): (CustomerSatisfactionScore, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode satisfaction score");
    assert_eq!(score, decoded);
}

#[test]
fn test_route_optimization_request_with_constraints() {
    let cfg = config::standard();
    let request = RouteOptimizationRequest {
        request_id: "ROPT-20260315-002".to_string(),
        depot_location: GpsCoordinate {
            latitude: 35.6295,
            longitude: 139.7109,
            accuracy_meters: 2.0,
            timestamp_epoch_ms: 1710478800000,
        },
        stops: vec![
            DeliveryRouteStop {
                stop_index: 0,
                address: "3-5 Ebisu, Shibuya-ku".to_string(),
                parcel_ids: vec!["PKG-91001".to_string()],
                location: GpsCoordinate {
                    latitude: 35.6467,
                    longitude: 139.7100,
                    accuracy_meters: 3.5,
                    timestamp_epoch_ms: 0,
                },
                estimated_service_time_secs: 90,
            },
            DeliveryRouteStop {
                stop_index: 1,
                address: "1-2 Daikanyama, Shibuya-ku".to_string(),
                parcel_ids: vec!["PKG-91002".to_string(), "PKG-91003".to_string()],
                location: GpsCoordinate {
                    latitude: 35.6487,
                    longitude: 139.7030,
                    accuracy_meters: 4.0,
                    timestamp_epoch_ms: 0,
                },
                estimated_service_time_secs: 150,
            },
        ],
        vehicle_capacity_grams: 1_000_000,
        vehicle_capacity_cm3: 4_500_000,
        shift_start_epoch_ms: 1710478800000,
        shift_end_epoch_ms: 1710514800000,
        avoid_tolls: true,
        prefer_highways: false,
    };
    let encoded = encode_to_vec(&request, cfg).expect("encode route optimization request");
    let (decoded, _): (RouteOptimizationRequest, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode route optimization request");
    assert_eq!(request, decoded);
}

#[test]
fn test_batch_tracking_events() {
    let cfg = config::standard();
    let batch = BatchTrackingEvent {
        batch_id: "BATCH-TRK-20260315-014".to_string(),
        events: vec![
            RealTimeTrackingUpdate {
                tracking_id: "TRK-20260315-101".to_string(),
                parcel_id: "PKG-90001".to_string(),
                status: "in_transit".to_string(),
                location: GpsCoordinate {
                    latitude: 35.6550,
                    longitude: 139.7050,
                    accuracy_meters: 10.0,
                    timestamp_epoch_ms: 1710489000000,
                },
                speed_kmh: 42.0,
                heading_degrees: 90.0,
                eta_epoch_ms: Some(1710492000000),
                stops_remaining: 5,
                updated_at_epoch_ms: 1710489000000,
            },
            RealTimeTrackingUpdate {
                tracking_id: "TRK-20260315-102".to_string(),
                parcel_id: "PKG-90002".to_string(),
                status: "in_transit".to_string(),
                location: GpsCoordinate {
                    latitude: 35.6550,
                    longitude: 139.7050,
                    accuracy_meters: 10.0,
                    timestamp_epoch_ms: 1710489000000,
                },
                speed_kmh: 42.0,
                heading_degrees: 90.0,
                eta_epoch_ms: Some(1710492000000),
                stops_remaining: 5,
                updated_at_epoch_ms: 1710489000000,
            },
        ],
        source_system: "fleet-gps-gateway".to_string(),
        ingested_at_epoch_ms: 1710489001000,
    };
    let encoded = encode_to_vec(&batch, cfg).expect("encode batch tracking events");
    let (decoded, _): (BatchTrackingEvent, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode batch tracking events");
    assert_eq!(batch, decoded);
}

#[test]
fn test_delivery_zone_boundary() {
    let cfg = config::standard();
    let zone = DeliveryZoneBoundary {
        zone_id: "ZONE-SBY-01".to_string(),
        zone_name: "Shibuya Central".to_string(),
        polygon_vertices: vec![
            (35.6620, 139.6980),
            (35.6620, 139.7150),
            (35.6520, 139.7150),
            (35.6520, 139.6980),
        ],
        active: true,
        max_drivers: 8,
        avg_density_parcels_per_km2: 145.3,
    };
    let encoded = encode_to_vec(&zone, cfg).expect("encode delivery zone boundary");
    let (decoded, _): (DeliveryZoneBoundary, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode delivery zone boundary");
    assert_eq!(zone, decoded);
}

#[test]
fn test_locker_redirect_after_failed_delivery() {
    let cfg = config::standard();
    let reattempt = FailedDeliveryReattempt {
        reattempt_id: "REATTEMPT-30002".to_string(),
        original_attempt_id: "ATT-80002".to_string(),
        parcel_id: "PKG-90007".to_string(),
        customer_id: "CUST-11500".to_string(),
        reason_code: "CUST_REFUSED".to_string(),
        reason_detail: "Package too large for mailbox, customer not home for 3rd attempt"
            .to_string(),
        reattempt_date: "2026-03-17".to_string(),
        reattempt_time_window: TimeWindow {
            start_epoch_ms: 1710669600000,
            end_epoch_ms: 1710676800000,
            label: "redirect-window".to_string(),
        },
        updated_address: None,
        redirect_to_locker: true,
        redirect_locker_id: Some("LSTA-SBY-003".to_string()),
        priority_boost: false,
    };
    let encoded = encode_to_vec(&reattempt, cfg).expect("encode locker redirect reattempt");
    let (decoded, _): (FailedDeliveryReattempt, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode locker redirect reattempt");
    assert_eq!(reattempt, decoded);
}

#[test]
fn test_negative_nps_detractor_feedback() {
    let cfg = config::standard();
    let score = CustomerSatisfactionScore {
        survey_id: "SURV-99050".to_string(),
        customer_id: "CUST-11500".to_string(),
        delivery_id: "DEL-70050".to_string(),
        nps_score: 2,
        csat_score: 1,
        effort_score: 5,
        feedback_text: Some(
            "Three failed attempts, had to pick up from locker far from home".to_string(),
        ),
        tags: vec![
            "multiple-failures".to_string(),
            "locker-redirect".to_string(),
            "inconvenient".to_string(),
            "detractor".to_string(),
        ],
        submitted_at_epoch_ms: 1710676800000,
        driver_id: "DRV-5510".to_string(),
    };
    let encoded = encode_to_vec(&score, cfg).expect("encode detractor feedback");
    let (decoded, _): (CustomerSatisfactionScore, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode detractor feedback");
    assert_eq!(score, decoded);
}
