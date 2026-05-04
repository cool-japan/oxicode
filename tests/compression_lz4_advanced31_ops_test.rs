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

// ── Domain types: Operations — rider, schedule, security, hospitality, parking

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DietaryRestriction {
    None,
    Vegetarian,
    Vegan,
    GlutenFree,
    Halal,
    Kosher,
    NutAllergy,
    DairyFree,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RiderItem {
    category: String,
    description: String,
    quantity: u16,
    is_critical: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ArtistRider {
    artist_name: String,
    dressing_room_count: u8,
    hospitality_items: Vec<RiderItem>,
    dietary_restrictions: Vec<DietaryRestriction>,
    technical_items: Vec<RiderItem>,
    buyout_amount_cents: Option<u32>,
    towel_count: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScheduleBlock {
    start_epoch: u64,
    end_epoch: u64,
    activity: String,
    responsible_crew: String,
    truck_number: Option<u8>,
    requires_forklift: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoadSchedule {
    event_id: u64,
    venue_name: String,
    load_in_blocks: Vec<ScheduleBlock>,
    load_out_blocks: Vec<ScheduleBlock>,
    dock_count: u8,
    elevator_available: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SecurityZone {
    FrontOfHouse,
    Backstage,
    Pit,
    VipLounge,
    Entrance,
    Parking,
    Perimeter,
    Roof,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SecurityStaffAssignment {
    staff_id: u32,
    name: String,
    zone: SecurityZone,
    shift_start_epoch: u64,
    shift_end_epoch: u64,
    is_supervisor: bool,
    radio_channel: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SecurityStaffingPlan {
    event_id: u64,
    total_staff: u32,
    assignments: Vec<SecurityStaffAssignment>,
    emergency_protocol_version: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CateringItem {
    item_name: String,
    quantity: u16,
    unit_cost_cents: u32,
    dietary_tags: Vec<DietaryRestriction>,
    delivered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HospitalityOrder {
    order_id: u32,
    recipient: String,
    items: Vec<CateringItem>,
    delivery_epoch: u64,
    room_designation: String,
    total_cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ParkingZone {
    General,
    Vip,
    Handicapped,
    BusCoach,
    ProductionTruck,
    ArtistBus,
    MediaVan,
    StaffOnly,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParkingSlot {
    zone: ParkingZone,
    slot_id: u32,
    occupied: bool,
    vehicle_plate: Option<String>,
    entry_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParkingLotStatus {
    lot_name: String,
    total_spaces: u32,
    slots: Vec<ParkingSlot>,
    revenue_collected_cents: u64,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_artist_rider_requirements_lz4() {
    let rider = ArtistRider {
        artist_name: "Stellar Nova".to_string(),
        dressing_room_count: 3,
        hospitality_items: vec![
            RiderItem {
                category: "Beverages".to_string(),
                description: "Still water 500ml".to_string(),
                quantity: 24,
                is_critical: true,
            },
            RiderItem {
                category: "Beverages".to_string(),
                description: "Sparkling water 500ml".to_string(),
                quantity: 12,
                is_critical: false,
            },
            RiderItem {
                category: "Food".to_string(),
                description: "Fresh fruit platter".to_string(),
                quantity: 2,
                is_critical: false,
            },
            RiderItem {
                category: "Food".to_string(),
                description: "Hummus with pita bread".to_string(),
                quantity: 1,
                is_critical: false,
            },
            RiderItem {
                category: "Supplies".to_string(),
                description: "White bath towels".to_string(),
                quantity: 20,
                is_critical: true,
            },
        ],
        dietary_restrictions: vec![
            DietaryRestriction::Vegetarian,
            DietaryRestriction::GlutenFree,
        ],
        technical_items: vec![
            RiderItem {
                category: "Audio".to_string(),
                description: "Shure SM58 microphone".to_string(),
                quantity: 6,
                is_critical: true,
            },
            RiderItem {
                category: "Audio".to_string(),
                description: "Shure SM57 microphone".to_string(),
                quantity: 4,
                is_critical: true,
            },
        ],
        buyout_amount_cents: Some(150_000),
        towel_count: 20,
    };

    let encoded = encode_to_vec(&rider).expect("encode artist rider");
    let compressed = compress_lz4(&encoded).expect("compress artist rider");
    let decompressed = decompress_lz4(&compressed).expect("decompress artist rider");
    let (decoded, _): (ArtistRider, _) =
        decode_from_slice(&decompressed).expect("decode artist rider");
    assert_eq!(rider, decoded);
}

#[test]
fn test_load_in_load_out_schedule_lz4() {
    let schedule = LoadSchedule {
        event_id: 77001,
        venue_name: "The Forum".to_string(),
        load_in_blocks: vec![
            ScheduleBlock {
                start_epoch: 1_700_000_000,
                end_epoch: 1_700_003_600,
                activity: "Truck dock - unload rigging".to_string(),
                responsible_crew: "Rigging Team A".to_string(),
                truck_number: Some(1),
                requires_forklift: true,
            },
            ScheduleBlock {
                start_epoch: 1_700_003_600,
                end_epoch: 1_700_007_200,
                activity: "Hang PA and lighting truss".to_string(),
                responsible_crew: "Audio/LX Crew".to_string(),
                truck_number: Some(2),
                requires_forklift: false,
            },
            ScheduleBlock {
                start_epoch: 1_700_007_200,
                end_epoch: 1_700_010_800,
                activity: "Stage build and backline setup".to_string(),
                responsible_crew: "Stage Crew".to_string(),
                truck_number: Some(3),
                requires_forklift: true,
            },
        ],
        load_out_blocks: vec![
            ScheduleBlock {
                start_epoch: 1_700_050_000,
                end_epoch: 1_700_053_600,
                activity: "Strike backline and stage".to_string(),
                responsible_crew: "Stage Crew".to_string(),
                truck_number: Some(3),
                requires_forklift: true,
            },
            ScheduleBlock {
                start_epoch: 1_700_053_600,
                end_epoch: 1_700_057_200,
                activity: "Lower and pack PA/LX".to_string(),
                responsible_crew: "Audio/LX Crew".to_string(),
                truck_number: Some(2),
                requires_forklift: false,
            },
        ],
        dock_count: 4,
        elevator_available: true,
    };

    let encoded = encode_to_vec(&schedule).expect("encode load schedule");
    let compressed = compress_lz4(&encoded).expect("compress load schedule");
    let decompressed = decompress_lz4(&compressed).expect("decompress load schedule");
    let (decoded, _): (LoadSchedule, _) =
        decode_from_slice(&decompressed).expect("decode load schedule");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_security_staffing_plan_lz4() {
    let plan = SecurityStaffingPlan {
        event_id: 55001,
        total_staff: 45,
        assignments: vec![
            SecurityStaffAssignment {
                staff_id: 1,
                name: "Davis, Marcus".to_string(),
                zone: SecurityZone::Entrance,
                shift_start_epoch: 1_700_020_000,
                shift_end_epoch: 1_700_045_000,
                is_supervisor: true,
                radio_channel: 1,
            },
            SecurityStaffAssignment {
                staff_id: 2,
                name: "Chen, Lin".to_string(),
                zone: SecurityZone::Pit,
                shift_start_epoch: 1_700_025_000,
                shift_end_epoch: 1_700_045_000,
                is_supervisor: false,
                radio_channel: 2,
            },
            SecurityStaffAssignment {
                staff_id: 3,
                name: "Okafor, Emeka".to_string(),
                zone: SecurityZone::Backstage,
                shift_start_epoch: 1_700_015_000,
                shift_end_epoch: 1_700_050_000,
                is_supervisor: true,
                radio_channel: 3,
            },
            SecurityStaffAssignment {
                staff_id: 4,
                name: "Hernandez, Sofia".to_string(),
                zone: SecurityZone::VipLounge,
                shift_start_epoch: 1_700_020_000,
                shift_end_epoch: 1_700_045_000,
                is_supervisor: false,
                radio_channel: 4,
            },
            SecurityStaffAssignment {
                staff_id: 5,
                name: "Tanaka, Yuki".to_string(),
                zone: SecurityZone::Parking,
                shift_start_epoch: 1_700_018_000,
                shift_end_epoch: 1_700_048_000,
                is_supervisor: false,
                radio_channel: 5,
            },
        ],
        emergency_protocol_version: "v3.2.1".to_string(),
    };

    let encoded = encode_to_vec(&plan).expect("encode security plan");
    let compressed = compress_lz4(&encoded).expect("compress security plan");
    let decompressed = decompress_lz4(&compressed).expect("decompress security plan");
    let (decoded, _): (SecurityStaffingPlan, _) =
        decode_from_slice(&decompressed).expect("decode security plan");
    assert_eq!(plan, decoded);
}

#[test]
fn test_hospitality_catering_order_lz4() {
    let order = HospitalityOrder {
        order_id: 4001,
        recipient: "Artist Dressing Room 1".to_string(),
        items: vec![
            CateringItem {
                item_name: "Grilled vegetable platter".to_string(),
                quantity: 2,
                unit_cost_cents: 4500,
                dietary_tags: vec![DietaryRestriction::Vegan, DietaryRestriction::GlutenFree],
                delivered: true,
            },
            CateringItem {
                item_name: "Artisan cheese board".to_string(),
                quantity: 1,
                unit_cost_cents: 6000,
                dietary_tags: vec![DietaryRestriction::Vegetarian],
                delivered: true,
            },
            CateringItem {
                item_name: "Sparkling water case".to_string(),
                quantity: 2,
                unit_cost_cents: 2000,
                dietary_tags: vec![],
                delivered: false,
            },
            CateringItem {
                item_name: "Organic green tea assortment".to_string(),
                quantity: 1,
                unit_cost_cents: 1500,
                dietary_tags: vec![DietaryRestriction::Vegan],
                delivered: false,
            },
        ],
        delivery_epoch: 1_700_020_000,
        room_designation: "DR-1 Main".to_string(),
        total_cost_cents: 20_500,
    };

    let encoded = encode_to_vec(&order).expect("encode hospitality order");
    let compressed = compress_lz4(&encoded).expect("compress hospitality order");
    let decompressed = decompress_lz4(&compressed).expect("decompress hospitality order");
    let (decoded, _): (HospitalityOrder, _) =
        decode_from_slice(&decompressed).expect("decode hospitality order");
    assert_eq!(order, decoded);
}

#[test]
fn test_parking_lot_management_lz4() {
    let lot = ParkingLotStatus {
        lot_name: "Venue North Lot".to_string(),
        total_spaces: 2500,
        slots: vec![
            ParkingSlot {
                zone: ParkingZone::General,
                slot_id: 1,
                occupied: true,
                vehicle_plate: Some("ABC-1234".to_string()),
                entry_epoch: Some(1_700_025_000),
            },
            ParkingSlot {
                zone: ParkingZone::General,
                slot_id: 2,
                occupied: false,
                vehicle_plate: None,
                entry_epoch: None,
            },
            ParkingSlot {
                zone: ParkingZone::Vip,
                slot_id: 100,
                occupied: true,
                vehicle_plate: Some("VIP-0001".to_string()),
                entry_epoch: Some(1_700_024_000),
            },
            ParkingSlot {
                zone: ParkingZone::Handicapped,
                slot_id: 200,
                occupied: true,
                vehicle_plate: Some("HC-5678".to_string()),
                entry_epoch: Some(1_700_024_500),
            },
            ParkingSlot {
                zone: ParkingZone::ProductionTruck,
                slot_id: 300,
                occupied: true,
                vehicle_plate: Some("TRUCK-01".to_string()),
                entry_epoch: Some(1_700_010_000),
            },
            ParkingSlot {
                zone: ParkingZone::ArtistBus,
                slot_id: 400,
                occupied: true,
                vehicle_plate: Some("BUS-STELLAR".to_string()),
                entry_epoch: Some(1_700_015_000),
            },
        ],
        revenue_collected_cents: 125_000,
    };

    let encoded = encode_to_vec(&lot).expect("encode parking lot");
    let compressed = compress_lz4(&encoded).expect("compress parking lot");
    let decompressed = decompress_lz4(&compressed).expect("decompress parking lot");
    let (decoded, _): (ParkingLotStatus, _) =
        decode_from_slice(&decompressed).expect("decode parking lot");
    assert_eq!(lot, decoded);
}
