//! Advanced Zstd compression tests for OxiCode — Hospitality & Hotel Management domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world hotel operations: room inventory, reservations, guest profiles,
//! housekeeping queues, PMS folio charges, revenue management metrics, banquet
//! bookings, minibar logs, key card events, maintenance work orders, guest
//! satisfaction surveys, channel manager rate parity, group block allocations,
//! concierge requests, and night audit reconciliation.

#![cfg(feature = "compression-zstd")]
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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RoomType {
    Standard,
    Superior,
    Deluxe,
    JuniorSuite,
    ExecutiveSuite,
    PresidentialSuite,
    Penthouse,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ViewType {
    CityView,
    GardenView,
    PoolView,
    OceanView,
    MountainView,
    Courtyard,
    NoView,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BedConfig {
    SingleKing,
    DoubleQueen,
    TwinDouble,
    SingleQueen,
    KingAndSofa,
    TwinSingle,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LoyaltyTier {
    Classic,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Ambassador,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HousekeepingStatus {
    Dirty,
    InProgress,
    Inspected,
    Clean,
    OutOfOrder,
    OutOfService,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChargeCategory {
    RoomRevenue,
    FoodAndBeverage,
    Minibar,
    Spa,
    Parking,
    Telephone,
    Laundry,
    RoomService,
    BusinessCenter,
    Miscellaneous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EventSetup {
    Theater,
    Classroom,
    Ushape,
    Boardroom,
    Banquet,
    Reception,
    Hollow,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaintenancePriority {
    Emergency,
    Urgent,
    High,
    Medium,
    Low,
    Scheduled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaintenanceCategory {
    Plumbing,
    Electrical,
    Hvac,
    Carpentry,
    Painting,
    Appliance,
    Elevator,
    FireSafety,
    General,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChannelSource {
    DirectWeb,
    BookingDotCom,
    Expedia,
    Agoda,
    Travelport,
    Amadeus,
    Sabre,
    PhoneReservation,
    WalkIn,
    GroupBlock,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConciergeServiceType {
    RestaurantReservation,
    AirportTransfer,
    CityTour,
    TheaterTickets,
    FlowerArrangement,
    SpecialAmenity,
    LuggageStorage,
    CourierService,
    BabySitting,
    PersonalShopping,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SurveyRating {
    Terrible,
    Poor,
    Average,
    Good,
    Excellent,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AccessEventType {
    RoomEntry,
    RoomExit,
    ElevatorAccess,
    GymAccess,
    PoolAccess,
    ParkingGarage,
    BusinessLounge,
    SpaArea,
    Denied,
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

/// A single room in the hotel inventory.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RoomInventory {
    room_number: u16,
    floor: u8,
    room_type: RoomType,
    view: ViewType,
    bed_config: BedConfig,
    max_occupancy: u8,
    square_meters: u16,
    amenities: Vec<String>,
    is_accessible: bool,
    is_smoking: bool,
    connecting_room: Option<u16>,
}

/// A reservation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Reservation {
    confirmation_number: u64,
    guest_name: String,
    room_number: u16,
    checkin_epoch: u64,
    checkout_epoch: u64,
    nightly_rate_cents: u32,
    guest_count_adults: u8,
    guest_count_children: u8,
    channel: ChannelSource,
    special_requests: Vec<String>,
    is_guaranteed: bool,
    deposit_cents: u32,
}

/// A guest loyalty profile.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GuestProfile {
    guest_id: u64,
    first_name: String,
    last_name: String,
    email: String,
    loyalty_tier: LoyaltyTier,
    lifetime_nights: u32,
    lifetime_spend_cents: u64,
    preferred_room_type: RoomType,
    preferred_floor_min: u8,
    preferred_floor_max: u8,
    dietary_restrictions: Vec<String>,
    allergies: Vec<String>,
    preferred_pillow: String,
    preferred_newspaper: String,
}

/// A single housekeeping task.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HousekeepingTask {
    task_id: u32,
    room_number: u16,
    status: HousekeepingStatus,
    assigned_staff_id: u32,
    priority_score: u8,
    is_checkout_clean: bool,
    is_vip: bool,
    special_instructions: Vec<String>,
    estimated_minutes: u16,
    linen_sets_needed: u8,
}

/// A PMS folio charge line.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FolioCharge {
    folio_id: u64,
    charge_id: u32,
    room_number: u16,
    category: ChargeCategory,
    description: String,
    amount_cents: i64,
    tax_cents: i64,
    posting_epoch: u64,
    is_adjustment: bool,
    reference_number: String,
}

/// Revenue management daily snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RevenueSnapshot {
    date_ordinal: u32,
    total_rooms: u16,
    rooms_sold: u16,
    rooms_out_of_order: u16,
    room_revenue_cents: u64,
    fb_revenue_cents: u64,
    other_revenue_cents: u64,
    adr_cents: u32,
    revpar_cents: u32,
    occupancy_bps: u16, // basis points (0–10000)
    comp_rooms: u16,
    house_use_rooms: u16,
    no_shows: u16,
    cancellations: u16,
}

/// A banquet / event booking.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BanquetBooking {
    event_id: u64,
    event_name: String,
    venue_name: String,
    setup: EventSetup,
    start_epoch: u64,
    end_epoch: u64,
    guaranteed_covers: u16,
    expected_covers: u16,
    per_person_cents: u32,
    av_rental_cents: u32,
    room_rental_cents: u32,
    menu_items: Vec<String>,
    notes: String,
}

/// A minibar consumption log entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MinibarConsumption {
    room_number: u16,
    item_code: u16,
    item_name: String,
    quantity: u8,
    unit_price_cents: u32,
    detected_epoch: u64,
    posted_to_folio: bool,
}

/// A key card access event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KeyCardEvent {
    event_id: u64,
    card_uid: u64,
    room_number: u16,
    event_type: AccessEventType,
    timestamp_epoch: u64,
    is_master_key: bool,
    door_controller_id: u16,
}

/// A maintenance work order.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceWorkOrder {
    order_id: u32,
    room_number: u16,
    category: MaintenanceCategory,
    priority: MaintenancePriority,
    description: String,
    reported_epoch: u64,
    assigned_technician_id: u32,
    completed_epoch: Option<u64>,
    parts_used: Vec<String>,
    labor_minutes: u16,
    parts_cost_cents: u32,
}

/// A guest satisfaction survey response.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GuestSurvey {
    survey_id: u64,
    confirmation_number: u64,
    overall_rating: SurveyRating,
    checkin_rating: SurveyRating,
    room_rating: SurveyRating,
    cleanliness_rating: SurveyRating,
    fb_rating: SurveyRating,
    staff_rating: SurveyRating,
    value_rating: SurveyRating,
    nps_score: i8,
    comments: String,
    would_recommend: bool,
}

/// Channel manager rate parity record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RateParityRecord {
    date_ordinal: u32,
    room_type: RoomType,
    channel: ChannelSource,
    published_rate_cents: u32,
    bar_rate_cents: u32,
    is_parity_violation: bool,
    last_update_epoch: u64,
    availability_count: u16,
    min_los: u8,
    max_los: u8,
    closed_to_arrival: bool,
    closed_to_departure: bool,
}

/// A group block allocation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroupBlock {
    block_code: String,
    group_name: String,
    contact_name: String,
    contact_email: String,
    start_date_ordinal: u32,
    end_date_ordinal: u32,
    rooms_blocked: Vec<(RoomType, u16)>,
    rooms_picked_up: Vec<(RoomType, u16)>,
    negotiated_rate_cents: u32,
    cutoff_date_ordinal: u32,
    is_elastic: bool,
    wash_factor_bps: u16,
}

/// A concierge service request.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConciergeRequest {
    request_id: u64,
    guest_id: u64,
    room_number: u16,
    service_type: ConciergeServiceType,
    description: String,
    requested_epoch: u64,
    fulfilled_epoch: Option<u64>,
    charge_cents: u32,
    notes: String,
}

/// Night audit reconciliation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NightAuditRecord {
    audit_date_ordinal: u32,
    total_room_revenue_cents: u64,
    total_tax_cents: u64,
    total_fb_revenue_cents: u64,
    total_other_revenue_cents: u64,
    total_payments_cents: u64,
    total_adjustments_cents: i64,
    arrivals_count: u16,
    departures_count: u16,
    stayovers_count: u16,
    no_show_count: u16,
    day_use_count: u16,
    walk_in_count: u16,
    ar_balance_cents: i64,
    guest_ledger_cents: i64,
    deposit_ledger_cents: i64,
    variance_cents: i64,
    is_balanced: bool,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_room(num: u16, floor: u8) -> RoomInventory {
    let room_type = match num % 7 {
        0 => RoomType::Standard,
        1 => RoomType::Superior,
        2 => RoomType::Deluxe,
        3 => RoomType::JuniorSuite,
        4 => RoomType::ExecutiveSuite,
        5 => RoomType::PresidentialSuite,
        _ => RoomType::Penthouse,
    };
    let view = match floor % 6 {
        0 => ViewType::Courtyard,
        1 => ViewType::GardenView,
        2 => ViewType::PoolView,
        3 => ViewType::CityView,
        4 => ViewType::OceanView,
        _ => ViewType::MountainView,
    };
    let bed = match num % 5 {
        0 => BedConfig::SingleKing,
        1 => BedConfig::DoubleQueen,
        2 => BedConfig::TwinDouble,
        3 => BedConfig::KingAndSofa,
        _ => BedConfig::SingleQueen,
    };
    RoomInventory {
        room_number: num,
        floor,
        room_type,
        view,
        bed_config: bed,
        max_occupancy: 2 + (num % 3) as u8,
        square_meters: 25 + (num % 50) * 2,
        amenities: vec![
            "WiFi".into(),
            "MiniBar".into(),
            "SafeBox".into(),
            "IronBoard".into(),
        ],
        is_accessible: num % 10 == 0,
        is_smoking: false,
        connecting_room: if num % 8 == 0 { Some(num + 1) } else { None },
    }
}

fn make_reservation(id: u64) -> Reservation {
    let channel = match id % 6 {
        0 => ChannelSource::DirectWeb,
        1 => ChannelSource::BookingDotCom,
        2 => ChannelSource::Expedia,
        3 => ChannelSource::Agoda,
        4 => ChannelSource::PhoneReservation,
        _ => ChannelSource::WalkIn,
    };
    Reservation {
        confirmation_number: 900_000 + id,
        guest_name: format!("Guest_{id}"),
        room_number: (100 + id % 300) as u16,
        checkin_epoch: 1_700_000_000 + id * 86400,
        checkout_epoch: 1_700_000_000 + id * 86400 + 3 * 86400,
        nightly_rate_cents: 15_000 + (id as u32 % 50) * 500,
        guest_count_adults: 1 + (id % 3) as u8,
        guest_count_children: (id % 2) as u8,
        channel,
        special_requests: vec!["Late check-in".into(), "High floor".into()],
        is_guaranteed: id % 4 != 0,
        deposit_cents: if id % 4 != 0 { 10_000 } else { 0 },
    }
}

fn make_guest_profile(id: u64) -> GuestProfile {
    let tier = match id % 6 {
        0 => LoyaltyTier::Classic,
        1 => LoyaltyTier::Silver,
        2 => LoyaltyTier::Gold,
        3 => LoyaltyTier::Platinum,
        4 => LoyaltyTier::Diamond,
        _ => LoyaltyTier::Ambassador,
    };
    GuestProfile {
        guest_id: id,
        first_name: format!("First{id}"),
        last_name: format!("Last{id}"),
        email: format!("guest{id}@example.com"),
        loyalty_tier: tier,
        lifetime_nights: 10 + id as u32 * 5,
        lifetime_spend_cents: 500_000 + id * 25_000,
        preferred_room_type: RoomType::Deluxe,
        preferred_floor_min: 5,
        preferred_floor_max: 20,
        dietary_restrictions: vec!["Gluten-free".into()],
        allergies: vec!["Feather pillows".into()],
        preferred_pillow: "Memory foam".into(),
        preferred_newspaper: "Financial Times".into(),
    }
}

fn make_folio_charge(id: u32, room: u16) -> FolioCharge {
    let cat = match id % 8 {
        0 => ChargeCategory::RoomRevenue,
        1 => ChargeCategory::FoodAndBeverage,
        2 => ChargeCategory::Minibar,
        3 => ChargeCategory::Spa,
        4 => ChargeCategory::Parking,
        5 => ChargeCategory::Laundry,
        6 => ChargeCategory::RoomService,
        _ => ChargeCategory::Miscellaneous,
    };
    FolioCharge {
        folio_id: room as u64 * 1000 + id as u64,
        charge_id: id,
        room_number: room,
        category: cat,
        description: format!("Charge item #{id}"),
        amount_cents: 1_200 + (id as i64 % 100) * 50,
        tax_cents: 120 + (id as i64 % 10) * 5,
        posting_epoch: 1_700_100_000 + id as u64 * 3600,
        is_adjustment: id % 15 == 0,
        reference_number: format!("REF-{id:06}"),
    }
}

fn make_revenue_snapshot(day: u32) -> RevenueSnapshot {
    let sold = 180 + (day % 40) as u16;
    let total: u16 = 250;
    RevenueSnapshot {
        date_ordinal: 738_500 + day,
        total_rooms: total,
        rooms_sold: sold,
        rooms_out_of_order: 3,
        room_revenue_cents: sold as u64 * 22_000,
        fb_revenue_cents: sold as u64 * 5_500,
        other_revenue_cents: sold as u64 * 1_200,
        adr_cents: 22_000,
        revpar_cents: (sold as u32 * 22_000) / total as u32,
        occupancy_bps: ((sold as u32 * 10_000) / total as u32) as u16,
        comp_rooms: 2,
        house_use_rooms: 1,
        no_shows: (day % 5) as u16,
        cancellations: (day % 8) as u16,
    }
}

fn make_banquet(id: u64) -> BanquetBooking {
    let setup = match id % 5 {
        0 => EventSetup::Banquet,
        1 => EventSetup::Theater,
        2 => EventSetup::Classroom,
        3 => EventSetup::Ushape,
        _ => EventSetup::Reception,
    };
    BanquetBooking {
        event_id: 5000 + id,
        event_name: format!("Gala Dinner #{id}"),
        venue_name: "Grand Ballroom".into(),
        setup,
        start_epoch: 1_700_200_000 + id * 7200,
        end_epoch: 1_700_200_000 + id * 7200 + 14400,
        guaranteed_covers: 80 + (id as u16 % 120),
        expected_covers: 100 + (id as u16 % 150),
        per_person_cents: 8_500,
        av_rental_cents: 250_000,
        room_rental_cents: 500_000,
        menu_items: vec![
            "Amuse-bouche".into(),
            "Soup du jour".into(),
            "Filet mignon".into(),
            "Crème brûlée".into(),
        ],
        notes: "Head table for 10 with floral centerpiece".into(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Room inventory round-trip for a single room.
#[test]
fn test_zstd_single_room_roundtrip() {
    let room = make_room(301, 3);
    let encoded = encode_to_vec(&room).expect("encode RoomInventory failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (RoomInventory, usize) =
        decode_from_slice(&decompressed).expect("decode RoomInventory failed");
    assert_eq!(room, decoded);
}

/// 2. Full-floor room inventory — compression should shrink repetitive amenity lists.
#[test]
fn test_zstd_floor_inventory_compression_ratio() {
    let rooms: Vec<RoomInventory> = (501..=530).map(|n| make_room(n, 5)).collect();
    let encoded = encode_to_vec(&rooms).expect("encode floor inventory failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<RoomInventory>, usize) =
        decode_from_slice(&decompressed).expect("decode floor inventory failed");
    assert_eq!(rooms, decoded);
}

/// 3. Reservation record round-trip.
#[test]
fn test_zstd_reservation_roundtrip() {
    let res = make_reservation(42);
    let encoded = encode_to_vec(&res).expect("encode Reservation failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Reservation, usize) =
        decode_from_slice(&decompressed).expect("decode Reservation failed");
    assert_eq!(res, decoded);
}

/// 4. Batch of 200 reservations — verify round-trip and compression.
#[test]
fn test_zstd_batch_reservations_compression() {
    let reservations: Vec<Reservation> = (0..200).map(make_reservation).collect();
    let encoded = encode_to_vec(&reservations).expect("encode batch reservations failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<Reservation>, usize) =
        decode_from_slice(&decompressed).expect("decode batch reservations failed");
    assert_eq!(reservations, decoded);
}

/// 5. Guest loyalty profiles round-trip.
#[test]
fn test_zstd_guest_profiles_roundtrip() {
    let profiles: Vec<GuestProfile> = (1..=50).map(make_guest_profile).collect();
    let encoded = encode_to_vec(&profiles).expect("encode guest profiles failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<GuestProfile>, usize) =
        decode_from_slice(&decompressed).expect("decode guest profiles failed");
    assert_eq!(profiles, decoded);
}

/// 6. Housekeeping task queue round-trip.
#[test]
fn test_zstd_housekeeping_queue_roundtrip() {
    let tasks: Vec<HousekeepingTask> = (0..60)
        .map(|i| {
            let status = match i % 5 {
                0 => HousekeepingStatus::Dirty,
                1 => HousekeepingStatus::InProgress,
                2 => HousekeepingStatus::Clean,
                3 => HousekeepingStatus::Inspected,
                _ => HousekeepingStatus::OutOfOrder,
            };
            HousekeepingTask {
                task_id: 10_000 + i,
                room_number: (200 + i % 100) as u16,
                status,
                assigned_staff_id: 500 + i % 12,
                priority_score: (i % 10) as u8,
                is_checkout_clean: i % 3 == 0,
                is_vip: i % 7 == 0,
                special_instructions: if i % 4 == 0 {
                    vec!["Extra towels".into(), "Hypoallergenic bedding".into()]
                } else {
                    vec![]
                },
                estimated_minutes: 20 + (i % 30) as u16,
                linen_sets_needed: 1 + (i % 3) as u8,
            }
        })
        .collect();
    let encoded = encode_to_vec(&tasks).expect("encode housekeeping tasks failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<HousekeepingTask>, usize) =
        decode_from_slice(&decompressed).expect("decode housekeeping tasks failed");
    assert_eq!(tasks, decoded);
}

/// 7. PMS folio charges round-trip.
#[test]
fn test_zstd_folio_charges_roundtrip() {
    let charges: Vec<FolioCharge> = (1..=40).map(|i| make_folio_charge(i, 405)).collect();
    let encoded = encode_to_vec(&charges).expect("encode folio charges failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<FolioCharge>, usize) =
        decode_from_slice(&decompressed).expect("decode folio charges failed");
    assert_eq!(charges, decoded);
}

/// 8. Revenue management 365-day snapshot — compression ratio check.
#[test]
fn test_zstd_revenue_365day_compression() {
    let snapshots: Vec<RevenueSnapshot> = (0..365).map(make_revenue_snapshot).collect();
    let encoded = encode_to_vec(&snapshots).expect("encode revenue snapshots failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len() / 2,
        "365-day revenue data should compress well: compressed={}, raw={}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<RevenueSnapshot>, usize) =
        decode_from_slice(&decompressed).expect("decode revenue snapshots failed");
    assert_eq!(snapshots, decoded);
}

/// 9. Banquet/event bookings round-trip.
#[test]
fn test_zstd_banquet_bookings_roundtrip() {
    let bookings: Vec<BanquetBooking> = (1..=15).map(make_banquet).collect();
    let encoded = encode_to_vec(&bookings).expect("encode banquet bookings failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<BanquetBooking>, usize) =
        decode_from_slice(&decompressed).expect("decode banquet bookings failed");
    assert_eq!(bookings, decoded);
}

/// 10. Minibar consumption logs round-trip.
#[test]
fn test_zstd_minibar_logs_roundtrip() {
    let logs: Vec<MinibarConsumption> = (0..80)
        .map(|i| {
            let (code, name, price) = match i % 6 {
                0 => (101u16, "Still Water 500ml", 500u32),
                1 => (102, "Sparkling Water 330ml", 600),
                2 => (201, "Premium Lager 330ml", 1_200),
                3 => (301, "Salted Cashews 80g", 900),
                4 => (401, "Chocolate Bar 50g", 700),
                _ => (501, "Orange Juice 250ml", 800),
            };
            MinibarConsumption {
                room_number: (300 + i % 50) as u16,
                item_code: code,
                item_name: name.into(),
                quantity: 1 + (i % 3) as u8,
                unit_price_cents: price,
                detected_epoch: 1_700_300_000 + i as u64 * 1800,
                posted_to_folio: i % 5 != 0,
            }
        })
        .collect();
    let encoded = encode_to_vec(&logs).expect("encode minibar logs failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<MinibarConsumption>, usize) =
        decode_from_slice(&decompressed).expect("decode minibar logs failed");
    assert_eq!(logs, decoded);
}

/// 11. Key card access events round-trip.
#[test]
fn test_zstd_keycard_events_roundtrip() {
    let events: Vec<KeyCardEvent> = (0..150)
        .map(|i| {
            let etype = match i % 7 {
                0 => AccessEventType::RoomEntry,
                1 => AccessEventType::RoomExit,
                2 => AccessEventType::ElevatorAccess,
                3 => AccessEventType::GymAccess,
                4 => AccessEventType::PoolAccess,
                5 => AccessEventType::ParkingGarage,
                _ => AccessEventType::BusinessLounge,
            };
            KeyCardEvent {
                event_id: 100_000 + i as u64,
                card_uid: 0xDEAD_0000 + (i as u64 % 80),
                room_number: (200 + i % 100) as u16,
                event_type: etype,
                timestamp_epoch: 1_700_400_000 + i as u64 * 120,
                is_master_key: i % 50 == 0,
                door_controller_id: (i % 200) as u16,
            }
        })
        .collect();
    let encoded = encode_to_vec(&events).expect("encode keycard events failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<KeyCardEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode keycard events failed");
    assert_eq!(events, decoded);
}

/// 12. Maintenance work orders round-trip.
#[test]
fn test_zstd_maintenance_orders_roundtrip() {
    let orders: Vec<MaintenanceWorkOrder> = (0..30)
        .map(|i| {
            let cat = match i % 7 {
                0 => MaintenanceCategory::Plumbing,
                1 => MaintenanceCategory::Electrical,
                2 => MaintenanceCategory::Hvac,
                3 => MaintenanceCategory::Carpentry,
                4 => MaintenanceCategory::Painting,
                5 => MaintenanceCategory::Appliance,
                _ => MaintenanceCategory::General,
            };
            let prio = match i % 4 {
                0 => MaintenancePriority::Low,
                1 => MaintenancePriority::Medium,
                2 => MaintenancePriority::High,
                _ => MaintenancePriority::Urgent,
            };
            MaintenanceWorkOrder {
                order_id: 20_000 + i,
                room_number: (100 + i % 250) as u16,
                category: cat,
                priority: prio,
                description: format!("Work order item #{i} — repair required"),
                reported_epoch: 1_700_500_000 + i as u64 * 3600,
                assigned_technician_id: 800 + i % 8,
                completed_epoch: if i % 3 == 0 {
                    Some(1_700_500_000 + i as u64 * 3600 + 7200)
                } else {
                    None
                },
                parts_used: if i % 4 == 0 {
                    vec!["Washer ring".into(), "Pipe sealant".into()]
                } else {
                    vec![]
                },
                labor_minutes: 30 + (i % 90) as u16,
                parts_cost_cents: (i % 20) * 250,
            }
        })
        .collect();
    let encoded = encode_to_vec(&orders).expect("encode maintenance orders failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<MaintenanceWorkOrder>, usize) =
        decode_from_slice(&decompressed).expect("decode maintenance orders failed");
    assert_eq!(orders, decoded);
}

/// 13. Guest satisfaction surveys round-trip.
#[test]
fn test_zstd_guest_surveys_roundtrip() {
    let surveys: Vec<GuestSurvey> = (0..40)
        .map(|i| {
            let rating = |v: u64| match v % 5 {
                0 => SurveyRating::Excellent,
                1 => SurveyRating::Good,
                2 => SurveyRating::Average,
                3 => SurveyRating::Poor,
                _ => SurveyRating::Terrible,
            };
            GuestSurvey {
                survey_id: 50_000 + i,
                confirmation_number: 900_000 + i,
                overall_rating: rating(i),
                checkin_rating: rating(i + 1),
                room_rating: rating(i + 2),
                cleanliness_rating: rating(i + 3),
                fb_rating: rating(i + 4),
                staff_rating: rating(i),
                value_rating: rating(i + 2),
                nps_score: ((i % 11) as i8) - 5,
                comments: format!("Stay #{i} was memorable. The staff went above and beyond."),
                would_recommend: i % 3 != 0,
            }
        })
        .collect();
    let encoded = encode_to_vec(&surveys).expect("encode surveys failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<GuestSurvey>, usize) =
        decode_from_slice(&decompressed).expect("decode surveys failed");
    assert_eq!(surveys, decoded);
}

/// 14. Channel manager rate parity records — compression of tabular rate data.
#[test]
fn test_zstd_rate_parity_compression() {
    let records: Vec<RateParityRecord> = (0..180)
        .flat_map(|day| {
            [
                RoomType::Standard,
                RoomType::Deluxe,
                RoomType::ExecutiveSuite,
            ]
            .into_iter()
            .map(move |rt| {
                let base = 15_000u32 + day * 100;
                RateParityRecord {
                    date_ordinal: 738_600 + day,
                    room_type: rt,
                    channel: ChannelSource::BookingDotCom,
                    published_rate_cents: base + 500,
                    bar_rate_cents: base,
                    is_parity_violation: day % 15 == 0,
                    last_update_epoch: 1_700_600_000 + day as u64 * 86400,
                    availability_count: (10 + day % 20) as u16,
                    min_los: 1,
                    max_los: 14,
                    closed_to_arrival: day % 30 == 0,
                    closed_to_departure: false,
                }
            })
        })
        .collect();
    let encoded = encode_to_vec(&records).expect("encode rate parity failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "rate parity data should compress: compressed={}, raw={}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<RateParityRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode rate parity failed");
    assert_eq!(records, decoded);
}

/// 15. Group block allocations round-trip.
#[test]
fn test_zstd_group_blocks_roundtrip() {
    let blocks: Vec<GroupBlock> = (0..10)
        .map(|i| GroupBlock {
            block_code: format!("GRP-2025-{i:03}"),
            group_name: format!("International Conference #{i}"),
            contact_name: format!("Coordinator {i}"),
            contact_email: format!("coord{i}@events.example.com"),
            start_date_ordinal: 738_700 + i * 3,
            end_date_ordinal: 738_700 + i * 3 + 4,
            rooms_blocked: vec![
                (RoomType::Standard, 20 + (i as u16 % 10)),
                (RoomType::Deluxe, 10 + (i as u16 % 5)),
                (RoomType::ExecutiveSuite, 2),
            ],
            rooms_picked_up: vec![
                (RoomType::Standard, 15 + (i as u16 % 8)),
                (RoomType::Deluxe, 7 + (i as u16 % 4)),
                (RoomType::ExecutiveSuite, 1),
            ],
            negotiated_rate_cents: 18_000 + i * 500,
            cutoff_date_ordinal: 738_700 + i * 3 - 14,
            is_elastic: i % 3 == 0,
            wash_factor_bps: 1_000 + (i as u16 % 5) * 200,
        })
        .collect();
    let encoded = encode_to_vec(&blocks).expect("encode group blocks failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<GroupBlock>, usize) =
        decode_from_slice(&decompressed).expect("decode group blocks failed");
    assert_eq!(blocks, decoded);
}

/// 16. Concierge service requests round-trip.
#[test]
fn test_zstd_concierge_requests_roundtrip() {
    let requests: Vec<ConciergeRequest> = (0..25)
        .map(|i| {
            let stype = match i % 8 {
                0 => ConciergeServiceType::RestaurantReservation,
                1 => ConciergeServiceType::AirportTransfer,
                2 => ConciergeServiceType::CityTour,
                3 => ConciergeServiceType::TheaterTickets,
                4 => ConciergeServiceType::FlowerArrangement,
                5 => ConciergeServiceType::SpecialAmenity,
                6 => ConciergeServiceType::LuggageStorage,
                _ => ConciergeServiceType::CourierService,
            };
            ConciergeRequest {
                request_id: 70_000 + i as u64,
                guest_id: 1000 + i as u64,
                room_number: (400 + i % 50) as u16,
                service_type: stype,
                description: format!("Service request #{i} for VIP guest"),
                requested_epoch: 1_700_700_000 + i as u64 * 1800,
                fulfilled_epoch: if i % 3 != 0 {
                    Some(1_700_700_000 + i as u64 * 1800 + 3600)
                } else {
                    None
                },
                charge_cents: 5_000 + (i as u32 % 20) * 1_000,
                notes: "Guest expressed preference for local artisan vendors".into(),
            }
        })
        .collect();
    let encoded = encode_to_vec(&requests).expect("encode concierge requests failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ConciergeRequest>, usize) =
        decode_from_slice(&decompressed).expect("decode concierge requests failed");
    assert_eq!(requests, decoded);
}

/// 17. Night audit reconciliation round-trip.
#[test]
fn test_zstd_night_audit_roundtrip() {
    let audits: Vec<NightAuditRecord> = (0..30)
        .map(|i| NightAuditRecord {
            audit_date_ordinal: 738_800 + i,
            total_room_revenue_cents: 4_200_000 + i as u64 * 10_000,
            total_tax_cents: 630_000 + i as u64 * 1_500,
            total_fb_revenue_cents: 1_100_000 + i as u64 * 3_000,
            total_other_revenue_cents: 300_000 + i as u64 * 800,
            total_payments_cents: 5_800_000 + i as u64 * 12_000,
            total_adjustments_cents: -(i as i64 % 7) * 2_500,
            arrivals_count: (40 + i % 20) as u16,
            departures_count: (38 + i % 18) as u16,
            stayovers_count: (140 + i % 30) as u16,
            no_show_count: (i % 4) as u16,
            day_use_count: (i % 3) as u16,
            walk_in_count: (i % 6) as u16,
            ar_balance_cents: 120_000 + i as i64 * 500,
            guest_ledger_cents: 3_500_000 - i as i64 * 1_000,
            deposit_ledger_cents: 800_000 + i as i64 * 200,
            variance_cents: if i % 10 == 0 {
                (i as i64 - 15) * 100
            } else {
                0
            },
            is_balanced: i % 10 != 0,
        })
        .collect();
    let encoded = encode_to_vec(&audits).expect("encode night audits failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<NightAuditRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode night audits failed");
    assert_eq!(audits, decoded);
}

/// 18. Large key card event log — verify significant compression.
#[test]
fn test_zstd_large_keycard_log_compression() {
    let events: Vec<KeyCardEvent> = (0..1_000)
        .map(|i| KeyCardEvent {
            event_id: i as u64,
            card_uid: 0xCAFE_0000 + (i as u64 % 120),
            room_number: (100 + i % 400) as u16,
            event_type: if i % 2 == 0 {
                AccessEventType::RoomEntry
            } else {
                AccessEventType::RoomExit
            },
            timestamp_epoch: 1_701_000_000 + i as u64 * 60,
            is_master_key: false,
            door_controller_id: (i % 300) as u16,
        })
        .collect();
    let encoded = encode_to_vec(&events).expect("encode large keycard log failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len() * 3 / 4,
        "1000-event keycard log should compress >25%: compressed={}, raw={}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<KeyCardEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode large keycard log failed");
    assert_eq!(events, decoded);
}

/// 19. Mixed folio charges across multiple rooms.
#[test]
fn test_zstd_multi_room_folio_roundtrip() {
    let charges: Vec<FolioCharge> = (0..100)
        .map(|i| make_folio_charge(i, (200 + i % 50) as u16))
        .collect();
    let encoded = encode_to_vec(&charges).expect("encode multi-room folio failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<FolioCharge>, usize) =
        decode_from_slice(&decompressed).expect("decode multi-room folio failed");
    assert_eq!(charges, decoded);
}

/// 20. Nested hotel snapshot: rooms + reservations + revenue in one struct.
#[test]
fn test_zstd_hotel_daily_snapshot_roundtrip() {
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct HotelDailySnapshot {
        hotel_code: String,
        date_ordinal: u32,
        rooms: Vec<RoomInventory>,
        reservations: Vec<Reservation>,
        revenue: RevenueSnapshot,
        audit: NightAuditRecord,
    }

    let snapshot = HotelDailySnapshot {
        hotel_code: "HTL-TOKYO-001".into(),
        date_ordinal: 738_900,
        rooms: (101..=120).map(|n| make_room(n, (n / 100) as u8)).collect(),
        reservations: (0..15).map(make_reservation).collect(),
        revenue: make_revenue_snapshot(100),
        audit: NightAuditRecord {
            audit_date_ordinal: 738_900,
            total_room_revenue_cents: 3_800_000,
            total_tax_cents: 570_000,
            total_fb_revenue_cents: 950_000,
            total_other_revenue_cents: 220_000,
            total_payments_cents: 5_200_000,
            total_adjustments_cents: -3_500,
            arrivals_count: 35,
            departures_count: 32,
            stayovers_count: 155,
            no_show_count: 2,
            day_use_count: 1,
            walk_in_count: 4,
            ar_balance_cents: 95_000,
            guest_ledger_cents: 3_200_000,
            deposit_ledger_cents: 700_000,
            variance_cents: 0,
            is_balanced: true,
        },
    };
    let encoded = encode_to_vec(&snapshot).expect("encode hotel snapshot failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "hotel snapshot should compress: compressed={}, raw={}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (HotelDailySnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode hotel snapshot failed");
    assert_eq!(snapshot, decoded);
}

/// 21. Empty collections — edge case for all list-bearing types.
#[test]
fn test_zstd_empty_collections_roundtrip() {
    let empty_rooms: Vec<RoomInventory> = vec![];
    let empty_reservations: Vec<Reservation> = vec![];
    let empty_surveys: Vec<GuestSurvey> = vec![];

    for (label, encoded) in [
        (
            "rooms",
            encode_to_vec(&empty_rooms).expect("encode empty rooms failed"),
        ),
        (
            "reservations",
            encode_to_vec(&empty_reservations).expect("encode empty reservations failed"),
        ),
        (
            "surveys",
            encode_to_vec(&empty_surveys).expect("encode empty surveys failed"),
        ),
    ] {
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress empty failed");
        let decompressed = decompress(&compressed).expect("zstd decompress empty failed");
        let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&decompressed)
            .unwrap_or_else(|_| panic!("decode empty {label} failed"));
        assert!(decoded.is_empty(), "expected empty vec for {label}");
    }
}

/// 22. Denied key card access events — security-relevant subset.
#[test]
fn test_zstd_denied_access_events_roundtrip() {
    let denied: Vec<KeyCardEvent> = (0..50)
        .map(|i| KeyCardEvent {
            event_id: 200_000 + i as u64,
            card_uid: 0xBAD0_0000 + i as u64,
            room_number: (100 + i % 300) as u16,
            event_type: AccessEventType::Denied,
            timestamp_epoch: 1_701_100_000 + i as u64 * 300,
            is_master_key: false,
            door_controller_id: (i % 150) as u16,
        })
        .collect();
    let encoded = encode_to_vec(&denied).expect("encode denied events failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<KeyCardEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode denied events failed");
    assert_eq!(denied, decoded);
}
