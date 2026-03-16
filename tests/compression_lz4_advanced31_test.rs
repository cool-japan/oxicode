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

// ── Domain types: Concert venues & live event production ─────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SeatStatus {
    Available,
    Sold,
    Held,
    Blocked,
    Accessible,
    CompedArtist,
    CompedPromoter,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Seat {
    seat_number: u16,
    row_label: String,
    status: SeatStatus,
    price_tier_id: u32,
    obstructed_view: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SectionRow {
    row_label: String,
    seats: Vec<Seat>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VenueSection {
    section_id: u32,
    name: String,
    level: String,
    rows: Vec<SectionRow>,
    accessible_entry: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VenueSeatingMap {
    venue_name: String,
    total_capacity: u32,
    sections: Vec<VenueSection>,
    last_updated_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PriceTierCategory {
    GeneralAdmission,
    Reserved,
    PremiumReserved,
    FloorSeating,
    BoxSeat,
    Pit,
    Balcony,
    Mezzanine,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PriceTier {
    tier_id: u32,
    category: PriceTierCategory,
    face_value_cents: u32,
    service_fee_cents: u32,
    facility_fee_cents: u32,
    total_allocated: u32,
    total_sold: u32,
    total_held: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TicketInventory {
    event_id: u64,
    event_name: String,
    tiers: Vec<PriceTier>,
    on_sale_epoch: u64,
    off_sale_epoch: u64,
    gross_potential_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MonitorType {
    Wedge,
    SideFill,
    DrumFill,
    InEar,
    HotSpot,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MonitorMix {
    mix_id: u8,
    monitor_type: MonitorType,
    assigned_to: String,
    channels: Vec<(u16, f32)>,
    eq_preset: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BacklineItem {
    GuitarAmp,
    BassAmp,
    DrumKit,
    KeyboardStand,
    DiBOx,
    MicStand,
    MusicStand,
    RiserSection,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BacklineEntry {
    item: BacklineItem,
    brand_model: String,
    quantity: u8,
    stage_position_x: f32,
    stage_position_y: f32,
    provided_by_venue: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StagePlotLayout {
    artist_name: String,
    stage_width_ft: f32,
    stage_depth_ft: f32,
    monitor_mixes: Vec<MonitorMix>,
    backline: Vec<BacklineEntry>,
    notes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GoboPattern {
    Breakup,
    Stars,
    CityScape,
    Flames,
    WaterRipple,
    AbstractSwirl,
    BrandLogo,
    Dots,
    Lines,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DmxChannel {
    channel: u16,
    fixture_name: String,
    dimmer_value: u8,
    color_temp_kelvin: u16,
    gobo: Option<GoboPattern>,
    pan_degrees: f32,
    tilt_degrees: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LightingCue {
    cue_number: f32,
    cue_name: String,
    fade_up_ms: u32,
    fade_down_ms: u32,
    hold_ms: u32,
    channels: Vec<DmxChannel>,
    follow_cue: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LightingCueList {
    show_name: String,
    designer: String,
    cues: Vec<LightingCue>,
    universe_count: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpeakerType {
    MainPaLeft,
    MainPaRight,
    OutFill,
    FrontFill,
    DelayTower,
    SubCardioid,
    SubOmni,
    LipFill,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpeakerCluster {
    cluster_id: u16,
    speaker_type: SpeakerType,
    cabinet_model: String,
    cabinet_count: u8,
    hang_angle_degrees: f32,
    splay_degrees: f32,
    delay_ms: f32,
    level_db: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SoundSystemConfig {
    venue_name: String,
    foh_console: String,
    monitor_console: String,
    clusters: Vec<SpeakerCluster>,
    total_amplifier_channels: u16,
    max_spl_db: f32,
}

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
enum MerchItemType {
    TShirt,
    Hoodie,
    Poster,
    Vinyl,
    Cd,
    Hat,
    Pin,
    ToteBag,
    Sticker,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MerchSaleRecord {
    item_type: MerchItemType,
    item_description: String,
    size: Option<String>,
    price_cents: u32,
    quantity_sold: u32,
    quantity_remaining: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MerchSalesTracking {
    event_id: u64,
    artist_name: String,
    records: Vec<MerchSaleRecord>,
    total_revenue_cents: u64,
    settlement_split_artist_pct: u8,
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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrowdDensitySample {
    zone_name: String,
    timestamp_epoch: u64,
    estimated_count: u32,
    capacity: u32,
    temperature_celsius: f32,
    alert_triggered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrowdFlowMonitor {
    event_id: u64,
    samples: Vec<CrowdDensitySample>,
    ingress_rate_per_min: Vec<(u64, u32)>,
    egress_rate_per_min: Vec<(u64, u32)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VipExperiencePackage {
    package_id: u32,
    package_name: String,
    price_cents: u32,
    includes_meet_greet: bool,
    includes_soundcheck: bool,
    includes_merch_bundle: bool,
    exclusive_entrance: bool,
    lounge_access: bool,
    signed_item_description: Option<String>,
    total_sold: u32,
    max_available: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StreamCodec {
    H264,
    H265,
    Vp9,
    Av1,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AudioCodec {
    Aac,
    Opus,
    Flac,
    PcmUncompressed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CameraFeed {
    camera_id: u8,
    position_name: String,
    resolution_width: u16,
    resolution_height: u16,
    frame_rate: u8,
    codec: StreamCodec,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BroadcastTechSpec {
    event_id: u64,
    platform: String,
    cameras: Vec<CameraFeed>,
    audio_codec: AudioCodec,
    audio_sample_rate_hz: u32,
    audio_bit_depth: u8,
    video_bitrate_kbps: u32,
    redundant_uplink: bool,
    delay_seconds: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SettlementLineItem {
    description: String,
    amount_cents: i64,
    is_expense: bool,
    category: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PostShowSettlement {
    event_id: u64,
    event_name: String,
    event_date_epoch: u64,
    gross_ticket_revenue_cents: i64,
    line_items: Vec<SettlementLineItem>,
    artist_guarantee_cents: i64,
    artist_overage_pct: u8,
    net_to_artist_cents: i64,
    net_to_promoter_cents: i64,
    signed_off: bool,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_venue_seating_map_lz4() {
    let map = VenueSeatingMap {
        venue_name: "Madison Square Garden".to_string(),
        total_capacity: 20_789,
        sections: vec![
            VenueSection {
                section_id: 1,
                name: "Floor A".to_string(),
                level: "Floor".to_string(),
                rows: vec![SectionRow {
                    row_label: "AA".to_string(),
                    seats: vec![
                        Seat {
                            seat_number: 1,
                            row_label: "AA".to_string(),
                            status: SeatStatus::Sold,
                            price_tier_id: 1,
                            obstructed_view: false,
                        },
                        Seat {
                            seat_number: 2,
                            row_label: "AA".to_string(),
                            status: SeatStatus::Available,
                            price_tier_id: 1,
                            obstructed_view: false,
                        },
                        Seat {
                            seat_number: 3,
                            row_label: "AA".to_string(),
                            status: SeatStatus::Held,
                            price_tier_id: 1,
                            obstructed_view: false,
                        },
                        Seat {
                            seat_number: 4,
                            row_label: "AA".to_string(),
                            status: SeatStatus::CompedArtist,
                            price_tier_id: 1,
                            obstructed_view: false,
                        },
                    ],
                }],
                accessible_entry: true,
            },
            VenueSection {
                section_id: 200,
                name: "Upper Balcony 200".to_string(),
                level: "300 Level".to_string(),
                rows: vec![SectionRow {
                    row_label: "A".to_string(),
                    seats: vec![
                        Seat {
                            seat_number: 1,
                            row_label: "A".to_string(),
                            status: SeatStatus::Blocked,
                            price_tier_id: 5,
                            obstructed_view: true,
                        },
                        Seat {
                            seat_number: 2,
                            row_label: "A".to_string(),
                            status: SeatStatus::Available,
                            price_tier_id: 5,
                            obstructed_view: false,
                        },
                    ],
                }],
                accessible_entry: false,
            },
        ],
        last_updated_epoch: 1_700_000_000,
    };

    let encoded = encode_to_vec(&map).expect("encode seating map");
    let compressed = compress_lz4(&encoded).expect("compress seating map");
    let decompressed = decompress_lz4(&compressed).expect("decompress seating map");
    let (decoded, _): (VenueSeatingMap, _) =
        decode_from_slice(&decompressed).expect("decode seating map");
    assert_eq!(map, decoded);
}

#[test]
fn test_ticket_inventory_price_tiers_lz4() {
    let inventory = TicketInventory {
        event_id: 90001,
        event_name: "Rock Festival 2026 - Night One".to_string(),
        tiers: vec![
            PriceTier {
                tier_id: 1,
                category: PriceTierCategory::Pit,
                face_value_cents: 25000,
                service_fee_cents: 3500,
                facility_fee_cents: 500,
                total_allocated: 500,
                total_sold: 498,
                total_held: 2,
            },
            PriceTier {
                tier_id: 2,
                category: PriceTierCategory::FloorSeating,
                face_value_cents: 17500,
                service_fee_cents: 2800,
                facility_fee_cents: 500,
                total_allocated: 2000,
                total_sold: 1800,
                total_held: 50,
            },
            PriceTier {
                tier_id: 3,
                category: PriceTierCategory::Reserved,
                face_value_cents: 9500,
                service_fee_cents: 1500,
                facility_fee_cents: 500,
                total_allocated: 8000,
                total_sold: 7200,
                total_held: 100,
            },
            PriceTier {
                tier_id: 4,
                category: PriceTierCategory::GeneralAdmission,
                face_value_cents: 5500,
                service_fee_cents: 800,
                facility_fee_cents: 500,
                total_allocated: 5000,
                total_sold: 4500,
                total_held: 0,
            },
            PriceTier {
                tier_id: 5,
                category: PriceTierCategory::Balcony,
                face_value_cents: 4000,
                service_fee_cents: 600,
                facility_fee_cents: 500,
                total_allocated: 3000,
                total_sold: 2100,
                total_held: 0,
            },
        ],
        on_sale_epoch: 1_690_000_000,
        off_sale_epoch: 1_700_500_000,
        gross_potential_cents: 185_000_000,
    };

    let encoded = encode_to_vec(&inventory).expect("encode ticket inventory");
    let compressed = compress_lz4(&encoded).expect("compress ticket inventory");
    let decompressed = decompress_lz4(&compressed).expect("decompress ticket inventory");
    let (decoded, _): (TicketInventory, _) =
        decode_from_slice(&decompressed).expect("decode ticket inventory");
    assert_eq!(inventory, decoded);
}

#[test]
fn test_stage_plot_layout_lz4() {
    let plot = StagePlotLayout {
        artist_name: "The Resonance Collective".to_string(),
        stage_width_ft: 60.0,
        stage_depth_ft: 40.0,
        monitor_mixes: vec![
            MonitorMix {
                mix_id: 1,
                monitor_type: MonitorType::InEar,
                assigned_to: "Lead Vocals".to_string(),
                channels: vec![(1, 0.8), (5, 0.6), (9, 0.3)],
                eq_preset: "vocal_bright".to_string(),
            },
            MonitorMix {
                mix_id: 2,
                monitor_type: MonitorType::Wedge,
                assigned_to: "Guitar Stage Left".to_string(),
                channels: vec![(2, 0.9), (5, 0.4), (10, 0.5)],
                eq_preset: "guitar_warm".to_string(),
            },
            MonitorMix {
                mix_id: 3,
                monitor_type: MonitorType::DrumFill,
                assigned_to: "Drums".to_string(),
                channels: vec![(6, 0.7), (7, 0.7), (8, 0.6), (1, 0.3)],
                eq_preset: "drum_punch".to_string(),
            },
            MonitorMix {
                mix_id: 4,
                monitor_type: MonitorType::SideFill,
                assigned_to: "Bass".to_string(),
                channels: vec![(3, 0.8), (6, 0.5)],
                eq_preset: "bass_deep".to_string(),
            },
        ],
        backline: vec![
            BacklineEntry {
                item: BacklineItem::GuitarAmp,
                brand_model: "Fender Twin Reverb '65".to_string(),
                quantity: 2,
                stage_position_x: 10.0,
                stage_position_y: 25.0,
                provided_by_venue: false,
            },
            BacklineEntry {
                item: BacklineItem::BassAmp,
                brand_model: "Ampeg SVT-CL + 8x10".to_string(),
                quantity: 1,
                stage_position_x: 50.0,
                stage_position_y: 25.0,
                provided_by_venue: false,
            },
            BacklineEntry {
                item: BacklineItem::DrumKit,
                brand_model: "DW Collector's Series".to_string(),
                quantity: 1,
                stage_position_x: 30.0,
                stage_position_y: 30.0,
                provided_by_venue: true,
            },
            BacklineEntry {
                item: BacklineItem::DiBOx,
                brand_model: "Radial J48".to_string(),
                quantity: 4,
                stage_position_x: 0.0,
                stage_position_y: 0.0,
                provided_by_venue: true,
            },
        ],
        notes: vec![
            "Drum riser 8x8 center stage".to_string(),
            "Piano stage right on 4x8 riser".to_string(),
        ],
    };

    let encoded = encode_to_vec(&plot).expect("encode stage plot");
    let compressed = compress_lz4(&encoded).expect("compress stage plot");
    let decompressed = decompress_lz4(&compressed).expect("decompress stage plot");
    let (decoded, _): (StagePlotLayout, _) =
        decode_from_slice(&decompressed).expect("decode stage plot");
    assert_eq!(plot, decoded);
}

#[test]
fn test_lighting_cue_list_lz4() {
    let cue_list = LightingCueList {
        show_name: "Neon Dreams Tour 2026".to_string(),
        designer: "Alex Lightfoot".to_string(),
        cues: vec![
            LightingCue {
                cue_number: 1.0,
                cue_name: "House to Half".to_string(),
                fade_up_ms: 0,
                fade_down_ms: 5000,
                hold_ms: 0,
                channels: vec![DmxChannel {
                    channel: 1,
                    fixture_name: "House Warm".to_string(),
                    dimmer_value: 128,
                    color_temp_kelvin: 3200,
                    gobo: None,
                    pan_degrees: 0.0,
                    tilt_degrees: 0.0,
                }],
                follow_cue: false,
            },
            LightingCue {
                cue_number: 2.0,
                cue_name: "Blackout".to_string(),
                fade_up_ms: 0,
                fade_down_ms: 2000,
                hold_ms: 3000,
                channels: vec![DmxChannel {
                    channel: 1,
                    fixture_name: "House Warm".to_string(),
                    dimmer_value: 0,
                    color_temp_kelvin: 3200,
                    gobo: None,
                    pan_degrees: 0.0,
                    tilt_degrees: 0.0,
                }],
                follow_cue: true,
            },
            LightingCue {
                cue_number: 3.0,
                cue_name: "Intro - Blue Wash".to_string(),
                fade_up_ms: 500,
                fade_down_ms: 0,
                hold_ms: 0,
                channels: vec![
                    DmxChannel {
                        channel: 10,
                        fixture_name: "Front Wash L".to_string(),
                        dimmer_value: 200,
                        color_temp_kelvin: 6500,
                        gobo: Some(GoboPattern::WaterRipple),
                        pan_degrees: 45.0,
                        tilt_degrees: -20.0,
                    },
                    DmxChannel {
                        channel: 11,
                        fixture_name: "Front Wash R".to_string(),
                        dimmer_value: 200,
                        color_temp_kelvin: 6500,
                        gobo: Some(GoboPattern::WaterRipple),
                        pan_degrees: -45.0,
                        tilt_degrees: -20.0,
                    },
                    DmxChannel {
                        channel: 20,
                        fixture_name: "Back Truss Spot".to_string(),
                        dimmer_value: 255,
                        color_temp_kelvin: 5600,
                        gobo: Some(GoboPattern::Stars),
                        pan_degrees: 0.0,
                        tilt_degrees: -35.0,
                    },
                ],
                follow_cue: true,
            },
        ],
        universe_count: 4,
    };

    let encoded = encode_to_vec(&cue_list).expect("encode lighting cue list");
    let compressed = compress_lz4(&encoded).expect("compress lighting cue list");
    let decompressed = decompress_lz4(&compressed).expect("decompress lighting cue list");
    let (decoded, _): (LightingCueList, _) =
        decode_from_slice(&decompressed).expect("decode lighting cue list");
    assert_eq!(cue_list, decoded);
}

#[test]
fn test_sound_system_config_lz4() {
    let config = SoundSystemConfig {
        venue_name: "Red Rocks Amphitheatre".to_string(),
        foh_console: "Avid Venue S6L-32D".to_string(),
        monitor_console: "Yamaha CL5".to_string(),
        clusters: vec![
            SpeakerCluster {
                cluster_id: 1,
                speaker_type: SpeakerType::MainPaLeft,
                cabinet_model: "d&b audiotechnik J-Series".to_string(),
                cabinet_count: 12,
                hang_angle_degrees: 0.0,
                splay_degrees: 1.5,
                delay_ms: 0.0,
                level_db: 0.0,
            },
            SpeakerCluster {
                cluster_id: 2,
                speaker_type: SpeakerType::MainPaRight,
                cabinet_model: "d&b audiotechnik J-Series".to_string(),
                cabinet_count: 12,
                hang_angle_degrees: 0.0,
                splay_degrees: 1.5,
                delay_ms: 0.0,
                level_db: 0.0,
            },
            SpeakerCluster {
                cluster_id: 3,
                speaker_type: SpeakerType::SubCardioid,
                cabinet_model: "d&b SL-Sub".to_string(),
                cabinet_count: 16,
                hang_angle_degrees: 0.0,
                splay_degrees: 0.0,
                delay_ms: 2.5,
                level_db: 3.0,
            },
            SpeakerCluster {
                cluster_id: 4,
                speaker_type: SpeakerType::DelayTower,
                cabinet_model: "d&b Y-Series".to_string(),
                cabinet_count: 6,
                hang_angle_degrees: -5.0,
                splay_degrees: 2.0,
                delay_ms: 85.0,
                level_db: -6.0,
            },
            SpeakerCluster {
                cluster_id: 5,
                speaker_type: SpeakerType::FrontFill,
                cabinet_model: "d&b E8".to_string(),
                cabinet_count: 8,
                hang_angle_degrees: 0.0,
                splay_degrees: 0.0,
                delay_ms: 1.0,
                level_db: -12.0,
            },
        ],
        total_amplifier_channels: 96,
        max_spl_db: 110.0,
    };

    let encoded = encode_to_vec(&config).expect("encode sound system config");
    let compressed = compress_lz4(&encoded).expect("compress sound system config");
    let decompressed = decompress_lz4(&compressed).expect("decompress sound system config");
    let (decoded, _): (SoundSystemConfig, _) =
        decode_from_slice(&decompressed).expect("decode sound system config");
    assert_eq!(config, decoded);
}

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
fn test_merch_sales_tracking_lz4() {
    let tracking = MerchSalesTracking {
        event_id: 88001,
        artist_name: "Crimson Tide Band".to_string(),
        records: vec![
            MerchSaleRecord {
                item_type: MerchItemType::TShirt,
                item_description: "Tour 2026 Black Tee".to_string(),
                size: Some("M".to_string()),
                price_cents: 3500,
                quantity_sold: 120,
                quantity_remaining: 30,
            },
            MerchSaleRecord {
                item_type: MerchItemType::TShirt,
                item_description: "Tour 2026 Black Tee".to_string(),
                size: Some("L".to_string()),
                price_cents: 3500,
                quantity_sold: 180,
                quantity_remaining: 20,
            },
            MerchSaleRecord {
                item_type: MerchItemType::Hoodie,
                item_description: "Logo Hoodie Charcoal".to_string(),
                size: Some("XL".to_string()),
                price_cents: 6500,
                quantity_sold: 45,
                quantity_remaining: 55,
            },
            MerchSaleRecord {
                item_type: MerchItemType::Poster,
                item_description: "Limited Edition Venue Poster".to_string(),
                size: None,
                price_cents: 2500,
                quantity_sold: 200,
                quantity_remaining: 0,
            },
            MerchSaleRecord {
                item_type: MerchItemType::Vinyl,
                item_description: "Latest Album - Signed".to_string(),
                size: None,
                price_cents: 4500,
                quantity_sold: 80,
                quantity_remaining: 20,
            },
            MerchSaleRecord {
                item_type: MerchItemType::Pin,
                item_description: "Enamel Logo Pin".to_string(),
                size: None,
                price_cents: 1200,
                quantity_sold: 300,
                quantity_remaining: 200,
            },
        ],
        total_revenue_cents: 2_847_500,
        settlement_split_artist_pct: 80,
    };

    let encoded = encode_to_vec(&tracking).expect("encode merch tracking");
    let compressed = compress_lz4(&encoded).expect("compress merch tracking");
    let decompressed = decompress_lz4(&compressed).expect("decompress merch tracking");
    let (decoded, _): (MerchSalesTracking, _) =
        decode_from_slice(&decompressed).expect("decode merch tracking");
    assert_eq!(tracking, decoded);
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

#[test]
fn test_crowd_flow_monitoring_lz4() {
    let monitor = CrowdFlowMonitor {
        event_id: 66001,
        samples: vec![
            CrowdDensitySample {
                zone_name: "Main Floor".to_string(),
                timestamp_epoch: 1_700_030_000,
                estimated_count: 3500,
                capacity: 5000,
                temperature_celsius: 24.5,
                alert_triggered: false,
            },
            CrowdDensitySample {
                zone_name: "Main Floor".to_string(),
                timestamp_epoch: 1_700_031_800,
                estimated_count: 4800,
                capacity: 5000,
                temperature_celsius: 26.2,
                alert_triggered: true,
            },
            CrowdDensitySample {
                zone_name: "Balcony East".to_string(),
                timestamp_epoch: 1_700_030_000,
                estimated_count: 800,
                capacity: 1200,
                temperature_celsius: 22.0,
                alert_triggered: false,
            },
            CrowdDensitySample {
                zone_name: "VIP Area".to_string(),
                timestamp_epoch: 1_700_030_000,
                estimated_count: 150,
                capacity: 200,
                temperature_celsius: 21.5,
                alert_triggered: false,
            },
        ],
        ingress_rate_per_min: vec![
            (1_700_025_000, 120),
            (1_700_025_060, 145),
            (1_700_025_120, 200),
            (1_700_025_180, 180),
        ],
        egress_rate_per_min: vec![
            (1_700_040_000, 50),
            (1_700_040_060, 80),
            (1_700_040_120, 300),
        ],
    };

    let encoded = encode_to_vec(&monitor).expect("encode crowd flow");
    let compressed = compress_lz4(&encoded).expect("compress crowd flow");
    let decompressed = decompress_lz4(&compressed).expect("decompress crowd flow");
    let (decoded, _): (CrowdFlowMonitor, _) =
        decode_from_slice(&decompressed).expect("decode crowd flow");
    assert_eq!(monitor, decoded);
}

#[test]
fn test_vip_experience_packages_lz4() {
    let packages: Vec<VipExperiencePackage> = vec![
        VipExperiencePackage {
            package_id: 1,
            package_name: "Ultimate VIP".to_string(),
            price_cents: 50000,
            includes_meet_greet: true,
            includes_soundcheck: true,
            includes_merch_bundle: true,
            exclusive_entrance: true,
            lounge_access: true,
            signed_item_description: Some("Signed setlist".to_string()),
            total_sold: 25,
            max_available: 30,
        },
        VipExperiencePackage {
            package_id: 2,
            package_name: "Premium Experience".to_string(),
            price_cents: 30000,
            includes_meet_greet: false,
            includes_soundcheck: true,
            includes_merch_bundle: true,
            exclusive_entrance: true,
            lounge_access: true,
            signed_item_description: None,
            total_sold: 50,
            max_available: 75,
        },
        VipExperiencePackage {
            package_id: 3,
            package_name: "Early Entry".to_string(),
            price_cents: 10000,
            includes_meet_greet: false,
            includes_soundcheck: false,
            includes_merch_bundle: false,
            exclusive_entrance: true,
            lounge_access: false,
            signed_item_description: None,
            total_sold: 100,
            max_available: 150,
        },
    ];

    let encoded = encode_to_vec(&packages).expect("encode vip packages");
    let compressed = compress_lz4(&encoded).expect("compress vip packages");
    let decompressed = decompress_lz4(&compressed).expect("decompress vip packages");
    let (decoded, _): (Vec<VipExperiencePackage>, _) =
        decode_from_slice(&decompressed).expect("decode vip packages");
    assert_eq!(packages, decoded);
}

#[test]
fn test_broadcast_tech_spec_lz4() {
    let spec = BroadcastTechSpec {
        event_id: 44001,
        platform: "LiveStreamPro".to_string(),
        cameras: vec![
            CameraFeed {
                camera_id: 1,
                position_name: "Wide Shot FOH".to_string(),
                resolution_width: 3840,
                resolution_height: 2160,
                frame_rate: 30,
                codec: StreamCodec::H265,
            },
            CameraFeed {
                camera_id: 2,
                position_name: "Close-up Stage Left".to_string(),
                resolution_width: 1920,
                resolution_height: 1080,
                frame_rate: 60,
                codec: StreamCodec::H264,
            },
            CameraFeed {
                camera_id: 3,
                position_name: "Jib Crane Center".to_string(),
                resolution_width: 3840,
                resolution_height: 2160,
                frame_rate: 30,
                codec: StreamCodec::H265,
            },
            CameraFeed {
                camera_id: 4,
                position_name: "Handheld Pit".to_string(),
                resolution_width: 1920,
                resolution_height: 1080,
                frame_rate: 60,
                codec: StreamCodec::H264,
            },
        ],
        audio_codec: AudioCodec::Aac,
        audio_sample_rate_hz: 48000,
        audio_bit_depth: 24,
        video_bitrate_kbps: 15000,
        redundant_uplink: true,
        delay_seconds: 30,
    };

    let encoded = encode_to_vec(&spec).expect("encode broadcast spec");
    let compressed = compress_lz4(&encoded).expect("compress broadcast spec");
    let decompressed = decompress_lz4(&compressed).expect("decompress broadcast spec");
    let (decoded, _): (BroadcastTechSpec, _) =
        decode_from_slice(&decompressed).expect("decode broadcast spec");
    assert_eq!(spec, decoded);
}

#[test]
fn test_post_show_settlement_lz4() {
    let settlement = PostShowSettlement {
        event_id: 99001,
        event_name: "Stellar Nova - The Forum - Oct 15".to_string(),
        event_date_epoch: 1_700_035_000,
        gross_ticket_revenue_cents: 1_250_000_00,
        line_items: vec![
            SettlementLineItem {
                description: "Venue rent".to_string(),
                amount_cents: -2_500_000,
                is_expense: true,
                category: "Venue".to_string(),
            },
            SettlementLineItem {
                description: "Production costs".to_string(),
                amount_cents: -1_800_000,
                is_expense: true,
                category: "Production".to_string(),
            },
            SettlementLineItem {
                description: "Marketing/Advertising".to_string(),
                amount_cents: -500_000,
                is_expense: true,
                category: "Marketing".to_string(),
            },
            SettlementLineItem {
                description: "Insurance".to_string(),
                amount_cents: -150_000,
                is_expense: true,
                category: "Insurance".to_string(),
            },
            SettlementLineItem {
                description: "ASCAP/BMI licensing".to_string(),
                amount_cents: -75_000,
                is_expense: true,
                category: "Licensing".to_string(),
            },
            SettlementLineItem {
                description: "Catering buyout".to_string(),
                amount_cents: -150_000,
                is_expense: true,
                category: "Hospitality".to_string(),
            },
            SettlementLineItem {
                description: "Security".to_string(),
                amount_cents: -350_000,
                is_expense: true,
                category: "Security".to_string(),
            },
            SettlementLineItem {
                description: "Ticket service fees (promoter share)".to_string(),
                amount_cents: 2_000_000,
                is_expense: false,
                category: "Fees".to_string(),
            },
        ],
        artist_guarantee_cents: 5_000_000,
        artist_overage_pct: 85,
        net_to_artist_cents: 5_000_000,
        net_to_promoter_cents: 3_475_000,
        signed_off: false,
    };

    let encoded = encode_to_vec(&settlement).expect("encode settlement");
    let compressed = compress_lz4(&encoded).expect("compress settlement");
    let decompressed = decompress_lz4(&compressed).expect("decompress settlement");
    let (decoded, _): (PostShowSettlement, _) =
        decode_from_slice(&decompressed).expect("decode settlement");
    assert_eq!(settlement, decoded);
}

#[test]
fn test_seating_map_compression_ratio_lz4() {
    let mut sections = Vec::new();
    for s in 0..10 {
        let mut rows = Vec::new();
        for r in 0..20 {
            let row_label = format!("Row-{}", r);
            let seats: Vec<Seat> = (0..30)
                .map(|seat_num| Seat {
                    seat_number: seat_num,
                    row_label: row_label.clone(),
                    status: if seat_num % 3 == 0 {
                        SeatStatus::Sold
                    } else {
                        SeatStatus::Available
                    },
                    price_tier_id: (s % 5) + 1,
                    obstructed_view: seat_num > 25,
                })
                .collect();
            rows.push(SectionRow { row_label, seats });
        }
        sections.push(VenueSection {
            section_id: s,
            name: format!("Section-{}", s),
            level: format!("Level-{}", s / 3),
            rows,
            accessible_entry: s % 4 == 0,
        });
    }
    let map = VenueSeatingMap {
        venue_name: "Mega Arena".to_string(),
        total_capacity: 60_000,
        sections,
        last_updated_epoch: 1_700_100_000,
    };

    let encoded = encode_to_vec(&map).expect("encode large seating map");
    let compressed = compress_lz4(&encoded).expect("compress large seating map");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive seating data"
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress large seating map");
    let (decoded, _): (VenueSeatingMap, _) =
        decode_from_slice(&decompressed).expect("decode large seating map");
    assert_eq!(map, decoded);
}

#[test]
fn test_lighting_cue_list_compression_ratio_lz4() {
    let cues: Vec<LightingCue> = (0..50)
        .map(|i| LightingCue {
            cue_number: i as f32 + 1.0,
            cue_name: format!("Cue {}", i + 1),
            fade_up_ms: 500,
            fade_down_ms: 500,
            hold_ms: if i % 5 == 0 { 2000 } else { 0 },
            channels: (0..8)
                .map(|ch| DmxChannel {
                    channel: ch * 10 + 1,
                    fixture_name: format!("Fixture-{}", ch),
                    dimmer_value: ((i * 5 + ch as u32) % 256) as u8,
                    color_temp_kelvin: 3200 + (ch * 400),
                    gobo: if ch % 3 == 0 {
                        Some(GoboPattern::Breakup)
                    } else {
                        None
                    },
                    pan_degrees: (ch as f32) * 15.0,
                    tilt_degrees: -10.0 - (ch as f32) * 5.0,
                })
                .collect(),
            follow_cue: i % 3 == 0,
        })
        .collect();

    let cue_list = LightingCueList {
        show_name: "Compression Test Show".to_string(),
        designer: "Test Designer".to_string(),
        cues,
        universe_count: 8,
    };

    let encoded = encode_to_vec(&cue_list).expect("encode large cue list");
    let compressed = compress_lz4(&encoded).expect("compress large cue list");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive lighting cue data"
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress large cue list");
    let (decoded, _): (LightingCueList, _) =
        decode_from_slice(&decompressed).expect("decode large cue list");
    assert_eq!(cue_list, decoded);
}

#[test]
fn test_multiple_monitor_mixes_lz4() {
    let mixes: Vec<MonitorMix> = vec![
        MonitorMix {
            mix_id: 1,
            monitor_type: MonitorType::InEar,
            assigned_to: "Vocals".to_string(),
            channels: vec![(1, 1.0), (2, 0.5), (10, 0.3), (11, 0.3)],
            eq_preset: "vocal_present".to_string(),
        },
        MonitorMix {
            mix_id: 2,
            monitor_type: MonitorType::InEar,
            assigned_to: "Keys".to_string(),
            channels: vec![(4, 0.9), (5, 0.4), (1, 0.2)],
            eq_preset: "keys_clean".to_string(),
        },
        MonitorMix {
            mix_id: 3,
            monitor_type: MonitorType::Wedge,
            assigned_to: "Guitar 1".to_string(),
            channels: vec![(2, 1.0), (6, 0.6), (10, 0.4)],
            eq_preset: "guitar_crunch".to_string(),
        },
        MonitorMix {
            mix_id: 4,
            monitor_type: MonitorType::Wedge,
            assigned_to: "Guitar 2".to_string(),
            channels: vec![(3, 1.0), (6, 0.5), (10, 0.4)],
            eq_preset: "guitar_crunch".to_string(),
        },
        MonitorMix {
            mix_id: 5,
            monitor_type: MonitorType::DrumFill,
            assigned_to: "Drums".to_string(),
            channels: vec![(6, 0.8), (7, 0.8), (8, 0.7), (9, 0.6), (1, 0.2)],
            eq_preset: "drum_thump".to_string(),
        },
        MonitorMix {
            mix_id: 6,
            monitor_type: MonitorType::SideFill,
            assigned_to: "Bass".to_string(),
            channels: vec![(5, 0.9), (6, 0.7), (1, 0.3)],
            eq_preset: "bass_round".to_string(),
        },
        MonitorMix {
            mix_id: 7,
            monitor_type: MonitorType::HotSpot,
            assigned_to: "Violin".to_string(),
            channels: vec![(12, 0.9), (1, 0.4), (4, 0.3)],
            eq_preset: "strings_airy".to_string(),
        },
    ];

    let encoded = encode_to_vec(&mixes).expect("encode monitor mixes");
    let compressed = compress_lz4(&encoded).expect("compress monitor mixes");
    let decompressed = decompress_lz4(&compressed).expect("decompress monitor mixes");
    let (decoded, _): (Vec<MonitorMix>, _) =
        decode_from_slice(&decompressed).expect("decode monitor mixes");
    assert_eq!(mixes, decoded);
}

#[test]
fn test_dmx_gobo_patterns_all_variants_lz4() {
    let channels: Vec<DmxChannel> = vec![
        DmxChannel {
            channel: 1,
            fixture_name: "Spot 1".to_string(),
            dimmer_value: 255,
            color_temp_kelvin: 5600,
            gobo: Some(GoboPattern::Breakup),
            pan_degrees: 0.0,
            tilt_degrees: -30.0,
        },
        DmxChannel {
            channel: 2,
            fixture_name: "Spot 2".to_string(),
            dimmer_value: 200,
            color_temp_kelvin: 3200,
            gobo: Some(GoboPattern::Stars),
            pan_degrees: 45.0,
            tilt_degrees: -25.0,
        },
        DmxChannel {
            channel: 3,
            fixture_name: "Spot 3".to_string(),
            dimmer_value: 180,
            color_temp_kelvin: 4500,
            gobo: Some(GoboPattern::CityScape),
            pan_degrees: -45.0,
            tilt_degrees: -20.0,
        },
        DmxChannel {
            channel: 4,
            fixture_name: "Spot 4".to_string(),
            dimmer_value: 220,
            color_temp_kelvin: 6500,
            gobo: Some(GoboPattern::Flames),
            pan_degrees: 90.0,
            tilt_degrees: -15.0,
        },
        DmxChannel {
            channel: 5,
            fixture_name: "Spot 5".to_string(),
            dimmer_value: 190,
            color_temp_kelvin: 2700,
            gobo: Some(GoboPattern::WaterRipple),
            pan_degrees: -90.0,
            tilt_degrees: -10.0,
        },
        DmxChannel {
            channel: 6,
            fixture_name: "Spot 6".to_string(),
            dimmer_value: 240,
            color_temp_kelvin: 5000,
            gobo: Some(GoboPattern::AbstractSwirl),
            pan_degrees: 30.0,
            tilt_degrees: -35.0,
        },
        DmxChannel {
            channel: 7,
            fixture_name: "Spot 7".to_string(),
            dimmer_value: 210,
            color_temp_kelvin: 4000,
            gobo: Some(GoboPattern::BrandLogo),
            pan_degrees: -30.0,
            tilt_degrees: -40.0,
        },
        DmxChannel {
            channel: 8,
            fixture_name: "Spot 8".to_string(),
            dimmer_value: 170,
            color_temp_kelvin: 3800,
            gobo: Some(GoboPattern::Dots),
            pan_degrees: 60.0,
            tilt_degrees: -45.0,
        },
        DmxChannel {
            channel: 9,
            fixture_name: "Spot 9".to_string(),
            dimmer_value: 230,
            color_temp_kelvin: 5200,
            gobo: Some(GoboPattern::Lines),
            pan_degrees: -60.0,
            tilt_degrees: -50.0,
        },
        DmxChannel {
            channel: 10,
            fixture_name: "Wash 1".to_string(),
            dimmer_value: 255,
            color_temp_kelvin: 3200,
            gobo: None,
            pan_degrees: 0.0,
            tilt_degrees: 0.0,
        },
    ];

    let encoded = encode_to_vec(&channels).expect("encode dmx channels");
    let compressed = compress_lz4(&encoded).expect("compress dmx channels");
    let decompressed = decompress_lz4(&compressed).expect("decompress dmx channels");
    let (decoded, _): (Vec<DmxChannel>, _) =
        decode_from_slice(&decompressed).expect("decode dmx channels");
    assert_eq!(channels, decoded);
}

#[test]
fn test_merch_sales_compression_ratio_lz4() {
    let records: Vec<MerchSaleRecord> = (0..100)
        .map(|i| MerchSaleRecord {
            item_type: match i % 9 {
                0 => MerchItemType::TShirt,
                1 => MerchItemType::Hoodie,
                2 => MerchItemType::Poster,
                3 => MerchItemType::Vinyl,
                4 => MerchItemType::Cd,
                5 => MerchItemType::Hat,
                6 => MerchItemType::Pin,
                7 => MerchItemType::ToteBag,
                _ => MerchItemType::Sticker,
            },
            item_description: format!("Merch item variant {}", i),
            size: if i % 3 == 0 {
                Some("L".to_string())
            } else {
                None
            },
            price_cents: 1000 + (i * 250),
            quantity_sold: 50 + i,
            quantity_remaining: 100 - (i % 100),
        })
        .collect();

    let tracking = MerchSalesTracking {
        event_id: 88888,
        artist_name: "Test Artist".to_string(),
        records,
        total_revenue_cents: 15_000_000,
        settlement_split_artist_pct: 75,
    };

    let encoded = encode_to_vec(&tracking).expect("encode large merch data");
    let compressed = compress_lz4(&encoded).expect("compress large merch data");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive merch data"
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress large merch data");
    let (decoded, _): (MerchSalesTracking, _) =
        decode_from_slice(&decompressed).expect("decode large merch data");
    assert_eq!(tracking, decoded);
}

#[test]
fn test_crowd_flow_large_dataset_compression_lz4() {
    let samples: Vec<CrowdDensitySample> = (0..200)
        .map(|i| CrowdDensitySample {
            zone_name: format!("Zone-{}", i % 10),
            timestamp_epoch: 1_700_030_000 + (i as u64 * 60),
            estimated_count: 500 + (i as u32 * 10) % 4000,
            capacity: 5000,
            temperature_celsius: 22.0 + (i as f32 * 0.05),
            alert_triggered: (500 + (i as u32 * 10) % 4000) > 4500,
        })
        .collect();

    let monitor = CrowdFlowMonitor {
        event_id: 66002,
        samples,
        ingress_rate_per_min: (0..60)
            .map(|i| (1_700_025_000 + i * 60, 100 + (i as u32 % 200)))
            .collect(),
        egress_rate_per_min: (0..30)
            .map(|i| (1_700_040_000 + i * 60, 50 + (i as u32 % 400)))
            .collect(),
    };

    let encoded = encode_to_vec(&monitor).expect("encode large crowd flow");
    let compressed = compress_lz4(&encoded).expect("compress large crowd flow");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive crowd flow data"
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress large crowd flow");
    let (decoded, _): (CrowdFlowMonitor, _) =
        decode_from_slice(&decompressed).expect("decode large crowd flow");
    assert_eq!(monitor, decoded);
}

#[test]
fn test_full_event_production_bundle_lz4() {
    let rider = ArtistRider {
        artist_name: "Bundle Test Act".to_string(),
        dressing_room_count: 2,
        hospitality_items: vec![RiderItem {
            category: "Beverages".to_string(),
            description: "Water".to_string(),
            quantity: 24,
            is_critical: true,
        }],
        dietary_restrictions: vec![DietaryRestriction::None],
        technical_items: vec![RiderItem {
            category: "Audio".to_string(),
            description: "SM58".to_string(),
            quantity: 4,
            is_critical: true,
        }],
        buyout_amount_cents: None,
        towel_count: 12,
    };

    let security = SecurityStaffingPlan {
        event_id: 11001,
        total_staff: 20,
        assignments: vec![SecurityStaffAssignment {
            staff_id: 1,
            name: "Guard A".to_string(),
            zone: SecurityZone::FrontOfHouse,
            shift_start_epoch: 1_700_020_000,
            shift_end_epoch: 1_700_045_000,
            is_supervisor: true,
            radio_channel: 1,
        }],
        emergency_protocol_version: "v2.0".to_string(),
    };

    let settlement = PostShowSettlement {
        event_id: 11001,
        event_name: "Bundle Test Show".to_string(),
        event_date_epoch: 1_700_035_000,
        gross_ticket_revenue_cents: 500_000_00,
        line_items: vec![SettlementLineItem {
            description: "Venue rent".to_string(),
            amount_cents: -1_000_000,
            is_expense: true,
            category: "Venue".to_string(),
        }],
        artist_guarantee_cents: 2_000_000,
        artist_overage_pct: 85,
        net_to_artist_cents: 2_000_000,
        net_to_promoter_cents: 1_000_000,
        signed_off: true,
    };

    let bundle = (rider.clone(), security.clone(), settlement.clone());

    let encoded = encode_to_vec(&bundle).expect("encode production bundle");
    let compressed = compress_lz4(&encoded).expect("compress production bundle");
    let decompressed = decompress_lz4(&compressed).expect("decompress production bundle");
    let (decoded, _): ((ArtistRider, SecurityStaffingPlan, PostShowSettlement), _) =
        decode_from_slice(&decompressed).expect("decode production bundle");
    assert_eq!(bundle, decoded);
}
