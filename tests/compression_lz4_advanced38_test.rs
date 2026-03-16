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

// ---------------------------------------------------------------------------
// Domain types: Concert Venue & Live Event Production Management
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct StagePlotConfig {
    stage_name: String,
    width_meters: f64,
    depth_meters: f64,
    riser_positions: Vec<RiserPosition>,
    monitor_wedge_ids: Vec<u16>,
    drum_riser_present: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RiserPosition {
    id: u16,
    x_offset: f64,
    y_offset: f64,
    height_cm: u32,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PaSystemRoute {
    route_id: u32,
    source_channel: u16,
    destination_speaker_group: String,
    gain_db: f32,
    delay_ms: f32,
    eq_band_gains: Vec<f32>,
    muted: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PaRoutingMatrix {
    venue: String,
    routes: Vec<PaSystemRoute>,
    main_left_right_linked: bool,
    sub_crossover_hz: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LightingFixture {
    fixture_id: u32,
    fixture_type: String,
    dmx_universe: u8,
    dmx_start_address: u16,
    dmx_channel_count: u8,
    position_label: String,
    color_temperature_k: u32,
    wattage: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LightingRig {
    show_name: String,
    fixtures: Vec<LightingFixture>,
    total_universes: u8,
    artnet_nodes: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RiggingPoint {
    point_id: String,
    x_meters: f64,
    y_meters: f64,
    rated_capacity_kg: f64,
    current_load_kg: f64,
    safety_factor: f32,
    bridle_legs: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RiggingLoadPlan {
    venue: String,
    points: Vec<RiggingPoint>,
    total_load_kg: f64,
    max_venue_capacity_kg: f64,
    engineer_approval: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum RiderItemCategory {
    Beverage,
    Food,
    Equipment,
    Amenity,
    Transport,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RiderItem {
    item: String,
    quantity: u32,
    category: RiderItemCategory,
    mandatory: bool,
    notes: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ArtistHospitalityRider {
    artist_name: String,
    dressing_room_count: u8,
    items: Vec<RiderItem>,
    buyout_offered: bool,
    buyout_amount_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PricingTier {
    GeneralAdmission,
    Reserved,
    Vip,
    Platinum,
    ArtistGuest,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TicketBlock {
    tier: PricingTier,
    section: String,
    total_seats: u32,
    sold: u32,
    held: u32,
    price_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TicketInventory {
    event_id: String,
    event_date_epoch: u64,
    blocks: Vec<TicketBlock>,
    doors_open_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SoundCheckParam {
    channel: u16,
    channel_label: String,
    gain_db: f32,
    phantom_power: bool,
    high_pass_hz: f32,
    compressor_threshold_db: f32,
    compressor_ratio: f32,
    fader_db: f32,
    pan_percent: i8,
    muted: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SoundCheckLog {
    artist: String,
    timestamp_epoch: u64,
    engineer: String,
    channels: Vec<SoundCheckParam>,
    notes: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VideoScreen {
    screen_id: u8,
    width_px: u32,
    height_px: u32,
    position_label: String,
    input_source: String,
    brightness_nits: u32,
    led_pitch_mm: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VideoScreenLayout {
    show_name: String,
    screens: Vec<VideoScreen>,
    media_server: String,
    total_output_ports: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PyroEffectType {
    Gerb,
    Comet,
    Mine,
    Flame,
    Confetti,
    Cryo,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PyroCue {
    cue_number: u32,
    effect: PyroEffectType,
    timecode_ms: u64,
    position_label: String,
    safe_distance_meters: f32,
    fallback_height_meters: f32,
    armed: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PyroSafetyProtocol {
    show_id: String,
    licensed_operator: String,
    license_number: String,
    cues: Vec<PyroCue>,
    fire_marshal_approved: bool,
    wind_speed_limit_kmh: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CrowdZone {
    zone_id: String,
    capacity: u32,
    current_count: u32,
    density_per_sqm: f32,
    alert_threshold: f32,
    exits: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CrowdDensitySnapshot {
    event_id: String,
    timestamp_epoch: u64,
    zones: Vec<CrowdZone>,
    total_attendance: u32,
    venue_max_capacity: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixBusSnapshot {
    bus_name: String,
    fader_db: f32,
    muted: bool,
    eq_low_db: f32,
    eq_mid_db: f32,
    eq_high_db: f32,
    insert_active: bool,
    insert_plugin: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConsoleSnapshot {
    console_model: String,
    scene_name: String,
    position: String,
    buses: Vec<MixBusSnapshot>,
    master_fader_db: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ChangeoverTask {
    task_id: u16,
    description: String,
    department: String,
    start_offset_min: i16,
    duration_min: u16,
    completed: bool,
    crew_required: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ArtistChangeover {
    from_artist: String,
    to_artist: String,
    total_window_min: u16,
    tasks: Vec<ChangeoverTask>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PowerDistribution {
    distro_id: String,
    phase_a_amps: f32,
    phase_b_amps: f32,
    phase_c_amps: f32,
    voltage: u16,
    total_kw: f32,
    circuits: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VenueCommsChannel {
    channel_number: u8,
    label: String,
    department: String,
    frequency_mhz: f64,
    encrypted: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CommsMatrix {
    event_name: String,
    channels: Vec<VenueCommsChannel>,
    base_station_count: u8,
    beltpack_count: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BarrierSegment {
    segment_id: String,
    length_meters: f32,
    barrier_type: String,
    crowd_side_padding_meters: f32,
    anchored: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FrontOfStagePlan {
    pit_depth_meters: f32,
    barrier_segments: Vec<BarrierSegment>,
    pit_capacity: u16,
    photo_positions: u8,
    security_count: u16,
}

// ---------------------------------------------------------------------------
// Test 1: Stage plot configuration
// ---------------------------------------------------------------------------
#[test]
fn test_stage_plot_config_lz4_roundtrip() {
    let val = StagePlotConfig {
        stage_name: "Main Stage Alpha".to_string(),
        width_meters: 18.0,
        depth_meters: 12.5,
        riser_positions: vec![
            RiserPosition {
                id: 1,
                x_offset: 3.0,
                y_offset: 8.0,
                height_cm: 60,
                label: "Drum Riser".to_string(),
            },
            RiserPosition {
                id: 2,
                x_offset: 0.0,
                y_offset: 6.0,
                height_cm: 30,
                label: "Keys Riser".to_string(),
            },
        ],
        monitor_wedge_ids: vec![101, 102, 103, 104, 105, 106],
        drum_riser_present: true,
    };
    let enc = encode_to_vec(&val).expect("encode stage plot config");
    let compressed = compress_lz4(&enc).expect("compress stage plot config");
    let decompressed = decompress_lz4(&compressed).expect("decompress stage plot config");
    let (decoded, _): (StagePlotConfig, usize) =
        decode_from_slice(&decompressed).expect("decode stage plot config");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: PA system routing matrix
// ---------------------------------------------------------------------------
#[test]
fn test_pa_routing_matrix_lz4_roundtrip() {
    let val = PaRoutingMatrix {
        venue: "Riverside Arena".to_string(),
        routes: vec![
            PaSystemRoute {
                route_id: 1,
                source_channel: 1,
                destination_speaker_group: "Main L".to_string(),
                gain_db: -3.5,
                delay_ms: 0.0,
                eq_band_gains: vec![0.0, -1.5, 2.0, -0.5, 1.0],
                muted: false,
            },
            PaSystemRoute {
                route_id: 2,
                source_channel: 1,
                destination_speaker_group: "Main R".to_string(),
                gain_db: -3.5,
                delay_ms: 0.0,
                eq_band_gains: vec![0.0, -1.5, 2.0, -0.5, 1.0],
                muted: false,
            },
            PaSystemRoute {
                route_id: 3,
                source_channel: 2,
                destination_speaker_group: "Delay Tower A".to_string(),
                gain_db: -6.0,
                delay_ms: 42.5,
                eq_band_gains: vec![-2.0, -1.0, 0.0, 0.5, -1.0],
                muted: false,
            },
        ],
        main_left_right_linked: true,
        sub_crossover_hz: 100,
    };
    let enc = encode_to_vec(&val).expect("encode PA routing matrix");
    let compressed = compress_lz4(&enc).expect("compress PA routing matrix");
    let decompressed = decompress_lz4(&compressed).expect("decompress PA routing matrix");
    let (decoded, _): (PaRoutingMatrix, usize) =
        decode_from_slice(&decompressed).expect("decode PA routing matrix");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Lighting rig with DMX addresses
// ---------------------------------------------------------------------------
#[test]
fn test_lighting_rig_dmx_lz4_roundtrip() {
    let val = LightingRig {
        show_name: "Summer Solstice Festival 2026".to_string(),
        fixtures: vec![
            LightingFixture {
                fixture_id: 1,
                fixture_type: "Moving Head Spot".to_string(),
                dmx_universe: 1,
                dmx_start_address: 1,
                dmx_channel_count: 24,
                position_label: "Truss A - Pos 1".to_string(),
                color_temperature_k: 6500,
                wattage: 1200,
            },
            LightingFixture {
                fixture_id: 2,
                fixture_type: "Moving Head Wash".to_string(),
                dmx_universe: 1,
                dmx_start_address: 25,
                dmx_channel_count: 18,
                position_label: "Truss A - Pos 2".to_string(),
                color_temperature_k: 3200,
                wattage: 800,
            },
            LightingFixture {
                fixture_id: 3,
                fixture_type: "LED Strobe".to_string(),
                dmx_universe: 2,
                dmx_start_address: 1,
                dmx_channel_count: 12,
                position_label: "Floor Package - DSL".to_string(),
                color_temperature_k: 5600,
                wattage: 400,
            },
        ],
        total_universes: 4,
        artnet_nodes: vec![
            "Node-A 10.0.0.10".to_string(),
            "Node-B 10.0.0.11".to_string(),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode lighting rig");
    let compressed = compress_lz4(&enc).expect("compress lighting rig");
    let decompressed = decompress_lz4(&compressed).expect("decompress lighting rig");
    let (decoded, _): (LightingRig, usize) =
        decode_from_slice(&decompressed).expect("decode lighting rig");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Rigging load calculations
// ---------------------------------------------------------------------------
#[test]
fn test_rigging_load_plan_lz4_roundtrip() {
    let val = RiggingLoadPlan {
        venue: "Grand Ballroom Convention Center".to_string(),
        points: vec![
            RiggingPoint {
                point_id: "RP-01".to_string(),
                x_meters: 5.0,
                y_meters: 3.0,
                rated_capacity_kg: 2000.0,
                current_load_kg: 850.0,
                safety_factor: 8.0,
                bridle_legs: 4,
            },
            RiggingPoint {
                point_id: "RP-02".to_string(),
                x_meters: 10.0,
                y_meters: 3.0,
                rated_capacity_kg: 2000.0,
                current_load_kg: 920.0,
                safety_factor: 8.0,
                bridle_legs: 4,
            },
            RiggingPoint {
                point_id: "RP-03".to_string(),
                x_meters: 15.0,
                y_meters: 3.0,
                rated_capacity_kg: 1500.0,
                current_load_kg: 640.0,
                safety_factor: 10.0,
                bridle_legs: 2,
            },
        ],
        total_load_kg: 2410.0,
        max_venue_capacity_kg: 15000.0,
        engineer_approval: true,
    };
    let enc = encode_to_vec(&val).expect("encode rigging load plan");
    let compressed = compress_lz4(&enc).expect("compress rigging load plan");
    let decompressed = decompress_lz4(&compressed).expect("decompress rigging load plan");
    let (decoded, _): (RiggingLoadPlan, usize) =
        decode_from_slice(&decompressed).expect("decode rigging load plan");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Artist hospitality rider
// ---------------------------------------------------------------------------
#[test]
fn test_artist_hospitality_rider_lz4_roundtrip() {
    let val = ArtistHospitalityRider {
        artist_name: "The Midnight Echoes".to_string(),
        dressing_room_count: 3,
        items: vec![
            RiderItem {
                item: "Still water (room temp)".to_string(),
                quantity: 24,
                category: RiderItemCategory::Beverage,
                mandatory: true,
                notes: "No plastic bottles".to_string(),
            },
            RiderItem {
                item: "Hummus and crudites platter".to_string(),
                quantity: 2,
                category: RiderItemCategory::Food,
                mandatory: false,
                notes: "Vegan only".to_string(),
            },
            RiderItem {
                item: "Full-length mirror".to_string(),
                quantity: 1,
                category: RiderItemCategory::Amenity,
                mandatory: true,
                notes: "In main dressing room".to_string(),
            },
            RiderItem {
                item: "15-passenger van".to_string(),
                quantity: 2,
                category: RiderItemCategory::Transport,
                mandatory: true,
                notes: "Hotel to venue shuttle".to_string(),
            },
        ],
        buyout_offered: true,
        buyout_amount_cents: 75000,
    };
    let enc = encode_to_vec(&val).expect("encode hospitality rider");
    let compressed = compress_lz4(&enc).expect("compress hospitality rider");
    let decompressed = decompress_lz4(&compressed).expect("decompress hospitality rider");
    let (decoded, _): (ArtistHospitalityRider, usize) =
        decode_from_slice(&decompressed).expect("decode hospitality rider");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Ticket inventory and pricing tiers
// ---------------------------------------------------------------------------
#[test]
fn test_ticket_inventory_lz4_roundtrip() {
    let val = TicketInventory {
        event_id: "EVT-2026-0315".to_string(),
        event_date_epoch: 1773820800,
        blocks: vec![
            TicketBlock {
                tier: PricingTier::GeneralAdmission,
                section: "Floor".to_string(),
                total_seats: 5000,
                sold: 4200,
                held: 150,
                price_cents: 8500,
            },
            TicketBlock {
                tier: PricingTier::Reserved,
                section: "Lower Bowl".to_string(),
                total_seats: 8000,
                sold: 6500,
                held: 200,
                price_cents: 12500,
            },
            TicketBlock {
                tier: PricingTier::Vip,
                section: "Golden Circle".to_string(),
                total_seats: 500,
                sold: 500,
                held: 0,
                price_cents: 25000,
            },
            TicketBlock {
                tier: PricingTier::Platinum,
                section: "Skybox".to_string(),
                total_seats: 80,
                sold: 72,
                held: 4,
                price_cents: 50000,
            },
            TicketBlock {
                tier: PricingTier::ArtistGuest,
                section: "Artist Comp".to_string(),
                total_seats: 100,
                sold: 0,
                held: 100,
                price_cents: 0,
            },
        ],
        doors_open_epoch: 1773810000,
    };
    let enc = encode_to_vec(&val).expect("encode ticket inventory");
    let compressed = compress_lz4(&enc).expect("compress ticket inventory");
    let decompressed = decompress_lz4(&compressed).expect("decompress ticket inventory");
    let (decoded, _): (TicketInventory, usize) =
        decode_from_slice(&decompressed).expect("decode ticket inventory");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Sound check parameter log
// ---------------------------------------------------------------------------
#[test]
fn test_sound_check_log_lz4_roundtrip() {
    let val = SoundCheckLog {
        artist: "Nova Cascade".to_string(),
        timestamp_epoch: 1773800000,
        engineer: "Chris Whitfield".to_string(),
        channels: vec![
            SoundCheckParam {
                channel: 1,
                channel_label: "Kick In".to_string(),
                gain_db: 32.0,
                phantom_power: false,
                high_pass_hz: 40.0,
                compressor_threshold_db: -12.0,
                compressor_ratio: 4.0,
                fader_db: -3.0,
                pan_percent: 0,
                muted: false,
            },
            SoundCheckParam {
                channel: 2,
                channel_label: "Kick Out".to_string(),
                gain_db: 28.0,
                phantom_power: false,
                high_pass_hz: 30.0,
                compressor_threshold_db: -15.0,
                compressor_ratio: 3.0,
                fader_db: -5.0,
                pan_percent: 0,
                muted: false,
            },
            SoundCheckParam {
                channel: 15,
                channel_label: "Lead Vocal".to_string(),
                gain_db: 38.0,
                phantom_power: true,
                high_pass_hz: 100.0,
                compressor_threshold_db: -8.0,
                compressor_ratio: 3.5,
                fader_db: 0.0,
                pan_percent: 0,
                muted: false,
            },
        ],
        notes: "Singer prefers bright vocal tone, reduce 400Hz notch".to_string(),
    };
    let enc = encode_to_vec(&val).expect("encode sound check log");
    let compressed = compress_lz4(&enc).expect("compress sound check log");
    let decompressed = decompress_lz4(&compressed).expect("decompress sound check log");
    let (decoded, _): (SoundCheckLog, usize) =
        decode_from_slice(&decompressed).expect("decode sound check log");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Video screen layout
// ---------------------------------------------------------------------------
#[test]
fn test_video_screen_layout_lz4_roundtrip() {
    let val = VideoScreenLayout {
        show_name: "Luminance World Tour".to_string(),
        screens: vec![
            VideoScreen {
                screen_id: 1,
                width_px: 3840,
                height_px: 2160,
                position_label: "Upstage Center".to_string(),
                input_source: "Media Server Out 1".to_string(),
                brightness_nits: 5500,
                led_pitch_mm: 2.8,
            },
            VideoScreen {
                screen_id: 2,
                width_px: 1920,
                height_px: 1080,
                position_label: "Stage Left IMAG".to_string(),
                input_source: "Camera 1 - ISO".to_string(),
                brightness_nits: 6000,
                led_pitch_mm: 3.9,
            },
            VideoScreen {
                screen_id: 3,
                width_px: 1920,
                height_px: 1080,
                position_label: "Stage Right IMAG".to_string(),
                input_source: "Camera 2 - ISO".to_string(),
                brightness_nits: 6000,
                led_pitch_mm: 3.9,
            },
        ],
        media_server: "Disguise GX3".to_string(),
        total_output_ports: 8,
    };
    let enc = encode_to_vec(&val).expect("encode video screen layout");
    let compressed = compress_lz4(&enc).expect("compress video screen layout");
    let decompressed = decompress_lz4(&compressed).expect("decompress video screen layout");
    let (decoded, _): (VideoScreenLayout, usize) =
        decode_from_slice(&decompressed).expect("decode video screen layout");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Pyrotechnics safety protocol
// ---------------------------------------------------------------------------
#[test]
fn test_pyro_safety_protocol_lz4_roundtrip() {
    let val = PyroSafetyProtocol {
        show_id: "SHOW-2026-0315-PYRO".to_string(),
        licensed_operator: "Elena Vasquez".to_string(),
        license_number: "PYRO-EU-44821".to_string(),
        cues: vec![
            PyroCue {
                cue_number: 1,
                effect: PyroEffectType::Gerb,
                timecode_ms: 3600000,
                position_label: "DSC - Center Line".to_string(),
                safe_distance_meters: 5.0,
                fallback_height_meters: 4.0,
                armed: true,
            },
            PyroCue {
                cue_number: 2,
                effect: PyroEffectType::Flame,
                timecode_ms: 3605000,
                position_label: "DSL - Stage Left Wing".to_string(),
                safe_distance_meters: 8.0,
                fallback_height_meters: 3.0,
                armed: true,
            },
            PyroCue {
                cue_number: 3,
                effect: PyroEffectType::Confetti,
                timecode_ms: 7200000,
                position_label: "FOH Truss".to_string(),
                safe_distance_meters: 2.0,
                fallback_height_meters: 12.0,
                armed: false,
            },
            PyroCue {
                cue_number: 4,
                effect: PyroEffectType::Cryo,
                timecode_ms: 5400000,
                position_label: "DSR - Stage Right Wing".to_string(),
                safe_distance_meters: 3.0,
                fallback_height_meters: 6.0,
                armed: true,
            },
        ],
        fire_marshal_approved: true,
        wind_speed_limit_kmh: 25.0,
    };
    let enc = encode_to_vec(&val).expect("encode pyro safety protocol");
    let compressed = compress_lz4(&enc).expect("compress pyro safety protocol");
    let decompressed = decompress_lz4(&compressed).expect("decompress pyro safety protocol");
    let (decoded, _): (PyroSafetyProtocol, usize) =
        decode_from_slice(&decompressed).expect("decode pyro safety protocol");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Crowd density monitoring snapshot
// ---------------------------------------------------------------------------
#[test]
fn test_crowd_density_snapshot_lz4_roundtrip() {
    let val = CrowdDensitySnapshot {
        event_id: "EVT-2026-0315".to_string(),
        timestamp_epoch: 1773825600,
        zones: vec![
            CrowdZone {
                zone_id: "PIT-A".to_string(),
                capacity: 1200,
                current_count: 1100,
                density_per_sqm: 4.2,
                alert_threshold: 4.5,
                exits: vec!["Exit-PIT-L".to_string(), "Exit-PIT-R".to_string()],
            },
            CrowdZone {
                zone_id: "GA-FLOOR".to_string(),
                capacity: 4000,
                current_count: 3200,
                density_per_sqm: 2.8,
                alert_threshold: 3.5,
                exits: vec![
                    "Exit-FL-1".to_string(),
                    "Exit-FL-2".to_string(),
                    "Exit-FL-3".to_string(),
                    "Exit-FL-4".to_string(),
                ],
            },
            CrowdZone {
                zone_id: "BOWL-200".to_string(),
                capacity: 6000,
                current_count: 4800,
                density_per_sqm: 1.5,
                alert_threshold: 2.5,
                exits: vec![
                    "Vomitory-201".to_string(),
                    "Vomitory-205".to_string(),
                    "Vomitory-210".to_string(),
                ],
            },
        ],
        total_attendance: 9100,
        venue_max_capacity: 15000,
    };
    let enc = encode_to_vec(&val).expect("encode crowd density snapshot");
    let compressed = compress_lz4(&enc).expect("compress crowd density snapshot");
    let decompressed = decompress_lz4(&compressed).expect("decompress crowd density snapshot");
    let (decoded, _): (CrowdDensitySnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode crowd density snapshot");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: FOH mixing console snapshot
// ---------------------------------------------------------------------------
#[test]
fn test_foh_console_snapshot_lz4_roundtrip() {
    let val = ConsoleSnapshot {
        console_model: "DiGiCo SD7".to_string(),
        scene_name: "Headliner - Song 1 Intro".to_string(),
        position: "FOH".to_string(),
        buses: vec![
            MixBusSnapshot {
                bus_name: "Main L".to_string(),
                fader_db: 0.0,
                muted: false,
                eq_low_db: -1.5,
                eq_mid_db: 0.5,
                eq_high_db: 1.0,
                insert_active: true,
                insert_plugin: "Waves L2 Ultramaximizer".to_string(),
            },
            MixBusSnapshot {
                bus_name: "Main R".to_string(),
                fader_db: 0.0,
                muted: false,
                eq_low_db: -1.5,
                eq_mid_db: 0.5,
                eq_high_db: 1.0,
                insert_active: true,
                insert_plugin: "Waves L2 Ultramaximizer".to_string(),
            },
            MixBusSnapshot {
                bus_name: "Sub Group".to_string(),
                fader_db: -2.0,
                muted: false,
                eq_low_db: 3.0,
                eq_mid_db: -2.0,
                eq_high_db: -6.0,
                insert_active: false,
                insert_plugin: String::new(),
            },
            MixBusSnapshot {
                bus_name: "FX Send 1 - Reverb".to_string(),
                fader_db: -10.0,
                muted: false,
                eq_low_db: -3.0,
                eq_mid_db: 0.0,
                eq_high_db: -1.0,
                insert_active: true,
                insert_plugin: "TC Electronic VSS3".to_string(),
            },
        ],
        master_fader_db: 0.0,
    };
    let enc = encode_to_vec(&val).expect("encode FOH console snapshot");
    let compressed = compress_lz4(&enc).expect("compress FOH console snapshot");
    let decompressed = decompress_lz4(&compressed).expect("decompress FOH console snapshot");
    let (decoded, _): (ConsoleSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode FOH console snapshot");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Monitor console snapshot
// ---------------------------------------------------------------------------
#[test]
fn test_monitor_console_snapshot_lz4_roundtrip() {
    let val = ConsoleSnapshot {
        console_model: "Yamaha PM5D".to_string(),
        scene_name: "Support Act - Full Band".to_string(),
        position: "Monitor World".to_string(),
        buses: vec![
            MixBusSnapshot {
                bus_name: "IEM Mix 1 - Vocalist".to_string(),
                fader_db: -5.0,
                muted: false,
                eq_low_db: -2.0,
                eq_mid_db: 1.0,
                eq_high_db: 2.0,
                insert_active: true,
                insert_plugin: "Limiter".to_string(),
            },
            MixBusSnapshot {
                bus_name: "IEM Mix 2 - Drummer".to_string(),
                fader_db: -3.0,
                muted: false,
                eq_low_db: 2.0,
                eq_mid_db: -1.0,
                eq_high_db: 0.0,
                insert_active: true,
                insert_plugin: "Limiter".to_string(),
            },
            MixBusSnapshot {
                bus_name: "Wedge Mix 3 - Guitarist".to_string(),
                fader_db: -6.0,
                muted: false,
                eq_low_db: -4.0,
                eq_mid_db: 0.0,
                eq_high_db: 1.5,
                insert_active: false,
                insert_plugin: String::new(),
            },
        ],
        master_fader_db: 0.0,
    };
    let enc = encode_to_vec(&val).expect("encode monitor console snapshot");
    let compressed = compress_lz4(&enc).expect("compress monitor console snapshot");
    let decompressed = decompress_lz4(&compressed).expect("decompress monitor console snapshot");
    let (decoded, _): (ConsoleSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode monitor console snapshot");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Artist changeover timeline
// ---------------------------------------------------------------------------
#[test]
fn test_artist_changeover_lz4_roundtrip() {
    let val = ArtistChangeover {
        from_artist: "Nova Cascade".to_string(),
        to_artist: "The Midnight Echoes".to_string(),
        total_window_min: 30,
        tasks: vec![
            ChangeoverTask {
                task_id: 1,
                description: "Strike support backline".to_string(),
                department: "Stage Crew".to_string(),
                start_offset_min: 0,
                duration_min: 8,
                completed: false,
                crew_required: 6,
            },
            ChangeoverTask {
                task_id: 2,
                description: "Fly headliner truss to trim".to_string(),
                department: "Rigging".to_string(),
                start_offset_min: 5,
                duration_min: 10,
                completed: false,
                crew_required: 4,
            },
            ChangeoverTask {
                task_id: 3,
                description: "Patch headliner inputs at stage box".to_string(),
                department: "Audio".to_string(),
                start_offset_min: 8,
                duration_min: 7,
                completed: false,
                crew_required: 2,
            },
            ChangeoverTask {
                task_id: 4,
                description: "Focus moving lights for headliner".to_string(),
                department: "Lighting".to_string(),
                start_offset_min: 10,
                duration_min: 12,
                completed: false,
                crew_required: 2,
            },
            ChangeoverTask {
                task_id: 5,
                description: "Recall headliner console scene".to_string(),
                department: "Audio".to_string(),
                start_offset_min: 15,
                duration_min: 3,
                completed: false,
                crew_required: 1,
            },
            ChangeoverTask {
                task_id: 6,
                description: "Line check all channels".to_string(),
                department: "Audio".to_string(),
                start_offset_min: 18,
                duration_min: 10,
                completed: false,
                crew_required: 3,
            },
        ],
    };
    let enc = encode_to_vec(&val).expect("encode artist changeover");
    let compressed = compress_lz4(&enc).expect("compress artist changeover");
    let decompressed = decompress_lz4(&compressed).expect("decompress artist changeover");
    let (decoded, _): (ArtistChangeover, usize) =
        decode_from_slice(&decompressed).expect("decode artist changeover");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Power distribution
// ---------------------------------------------------------------------------
#[test]
fn test_power_distribution_lz4_roundtrip() {
    let val = PowerDistribution {
        distro_id: "PWR-STAGE-MAIN".to_string(),
        phase_a_amps: 185.0,
        phase_b_amps: 192.0,
        phase_c_amps: 178.0,
        voltage: 400,
        total_kw: 132.5,
        circuits: vec![
            "CKT-01: Lighting Dimmer Rack A".to_string(),
            "CKT-02: Lighting Dimmer Rack B".to_string(),
            "CKT-03: Audio Amplifier Rack 1".to_string(),
            "CKT-04: Audio Amplifier Rack 2".to_string(),
            "CKT-05: Video Processing".to_string(),
            "CKT-06: Stage Power - USR".to_string(),
            "CKT-07: Stage Power - USL".to_string(),
            "CKT-08: Backline Power".to_string(),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode power distribution");
    let compressed = compress_lz4(&enc).expect("compress power distribution");
    let decompressed = decompress_lz4(&compressed).expect("decompress power distribution");
    let (decoded, _): (PowerDistribution, usize) =
        decode_from_slice(&decompressed).expect("decode power distribution");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Communications matrix
// ---------------------------------------------------------------------------
#[test]
fn test_comms_matrix_lz4_roundtrip() {
    let val = CommsMatrix {
        event_name: "Summer Solstice Festival Day 2".to_string(),
        channels: vec![
            VenueCommsChannel {
                channel_number: 1,
                label: "Production".to_string(),
                department: "Production Management".to_string(),
                frequency_mhz: 470.125,
                encrypted: true,
            },
            VenueCommsChannel {
                channel_number: 2,
                label: "Stage".to_string(),
                department: "Stage Management".to_string(),
                frequency_mhz: 470.375,
                encrypted: true,
            },
            VenueCommsChannel {
                channel_number: 3,
                label: "Audio".to_string(),
                department: "Sound".to_string(),
                frequency_mhz: 470.625,
                encrypted: false,
            },
            VenueCommsChannel {
                channel_number: 4,
                label: "Lighting/Video".to_string(),
                department: "Visuals".to_string(),
                frequency_mhz: 470.875,
                encrypted: false,
            },
            VenueCommsChannel {
                channel_number: 5,
                label: "Security".to_string(),
                department: "Security Operations".to_string(),
                frequency_mhz: 471.125,
                encrypted: true,
            },
        ],
        base_station_count: 3,
        beltpack_count: 48,
    };
    let enc = encode_to_vec(&val).expect("encode comms matrix");
    let compressed = compress_lz4(&enc).expect("compress comms matrix");
    let decompressed = decompress_lz4(&compressed).expect("decompress comms matrix");
    let (decoded, _): (CommsMatrix, usize) =
        decode_from_slice(&decompressed).expect("decode comms matrix");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Front of stage barrier plan
// ---------------------------------------------------------------------------
#[test]
fn test_front_of_stage_plan_lz4_roundtrip() {
    let val = FrontOfStagePlan {
        pit_depth_meters: 2.5,
        barrier_segments: vec![
            BarrierSegment {
                segment_id: "BAR-01".to_string(),
                length_meters: 6.0,
                barrier_type: "Steel A-Frame".to_string(),
                crowd_side_padding_meters: 0.5,
                anchored: true,
            },
            BarrierSegment {
                segment_id: "BAR-02".to_string(),
                length_meters: 6.0,
                barrier_type: "Steel A-Frame".to_string(),
                crowd_side_padding_meters: 0.5,
                anchored: true,
            },
            BarrierSegment {
                segment_id: "BAR-GATE-L".to_string(),
                length_meters: 1.2,
                barrier_type: "Gate Section".to_string(),
                crowd_side_padding_meters: 0.0,
                anchored: false,
            },
            BarrierSegment {
                segment_id: "BAR-GATE-R".to_string(),
                length_meters: 1.2,
                barrier_type: "Gate Section".to_string(),
                crowd_side_padding_meters: 0.0,
                anchored: false,
            },
        ],
        pit_capacity: 40,
        photo_positions: 6,
        security_count: 24,
    };
    let enc = encode_to_vec(&val).expect("encode front of stage plan");
    let compressed = compress_lz4(&enc).expect("compress front of stage plan");
    let decompressed = decompress_lz4(&compressed).expect("decompress front of stage plan");
    let (decoded, _): (FrontOfStagePlan, usize) =
        decode_from_slice(&decompressed).expect("decode front of stage plan");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Large lighting rig compresses smaller than raw
// ---------------------------------------------------------------------------
#[test]
fn test_large_lighting_rig_compresses_smaller() {
    let fixtures: Vec<LightingFixture> = (0..200)
        .map(|i| LightingFixture {
            fixture_id: i,
            fixture_type: "Generic Par LED RGBW".to_string(),
            dmx_universe: (i / 32) as u8 + 1,
            dmx_start_address: ((i % 32) * 16 + 1) as u16,
            dmx_channel_count: 16,
            position_label: format!("Truss {}-Pos {}", (i / 20) + 1, (i % 20) + 1),
            color_temperature_k: 5600,
            wattage: 300,
        })
        .collect();
    let val = LightingRig {
        show_name: "Mega Festival - 200 Fixture Rig".to_string(),
        fixtures,
        total_universes: 8,
        artnet_nodes: vec![
            "Node-1".to_string(),
            "Node-2".to_string(),
            "Node-3".to_string(),
            "Node-4".to_string(),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode large lighting rig");
    let compressed = compress_lz4(&enc).expect("compress large lighting rig");
    assert!(
        compressed.len() < enc.len(),
        "compressed size {} should be less than raw size {}",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress large lighting rig");
    let (decoded, _): (LightingRig, usize) =
        decode_from_slice(&decompressed).expect("decode large lighting rig");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Multiple pyro effect types roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_all_pyro_effect_types_lz4_roundtrip() {
    let effects = vec![
        PyroEffectType::Gerb,
        PyroEffectType::Comet,
        PyroEffectType::Mine,
        PyroEffectType::Flame,
        PyroEffectType::Confetti,
        PyroEffectType::Cryo,
    ];
    let enc = encode_to_vec(&effects).expect("encode pyro effect types");
    let compressed = compress_lz4(&enc).expect("compress pyro effect types");
    let decompressed = decompress_lz4(&compressed).expect("decompress pyro effect types");
    let (decoded, _): (Vec<PyroEffectType>, usize) =
        decode_from_slice(&decompressed).expect("decode pyro effect types");
    assert_eq!(effects, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: All pricing tiers roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_all_pricing_tiers_lz4_roundtrip() {
    let tiers = vec![
        PricingTier::GeneralAdmission,
        PricingTier::Reserved,
        PricingTier::Vip,
        PricingTier::Platinum,
        PricingTier::ArtistGuest,
    ];
    let enc = encode_to_vec(&tiers).expect("encode pricing tiers");
    let compressed = compress_lz4(&enc).expect("compress pricing tiers");
    let decompressed = decompress_lz4(&compressed).expect("decompress pricing tiers");
    let (decoded, _): (Vec<PricingTier>, usize) =
        decode_from_slice(&decompressed).expect("decode pricing tiers");
    assert_eq!(tiers, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: All rider item categories roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_all_rider_categories_lz4_roundtrip() {
    let categories = vec![
        RiderItemCategory::Beverage,
        RiderItemCategory::Food,
        RiderItemCategory::Equipment,
        RiderItemCategory::Amenity,
        RiderItemCategory::Transport,
    ];
    let enc = encode_to_vec(&categories).expect("encode rider categories");
    let compressed = compress_lz4(&enc).expect("compress rider categories");
    let decompressed = decompress_lz4(&compressed).expect("decompress rider categories");
    let (decoded, _): (Vec<RiderItemCategory>, usize) =
        decode_from_slice(&decompressed).expect("decode rider categories");
    assert_eq!(categories, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Large crowd density snapshot with many zones
// ---------------------------------------------------------------------------
#[test]
fn test_large_crowd_density_many_zones_lz4_roundtrip() {
    let zones: Vec<CrowdZone> = (1..=30)
        .map(|i| CrowdZone {
            zone_id: format!("ZONE-{:03}", i),
            capacity: 500 + (i * 100),
            current_count: 400 + (i * 80),
            density_per_sqm: 1.0 + (i as f32 * 0.1),
            alert_threshold: 3.0 + (i as f32 * 0.05),
            exits: vec![format!("Exit-{}-A", i), format!("Exit-{}-B", i)],
        })
        .collect();
    let val = CrowdDensitySnapshot {
        event_id: "MEGA-FEST-2026".to_string(),
        timestamp_epoch: 1773830000,
        zones,
        total_attendance: 45000,
        venue_max_capacity: 60000,
    };
    let enc = encode_to_vec(&val).expect("encode large crowd density");
    let compressed = compress_lz4(&enc).expect("compress large crowd density");
    let decompressed = decompress_lz4(&compressed).expect("decompress large crowd density");
    let (decoded, _): (CrowdDensitySnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode large crowd density");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Complex nested changeover with completed and incomplete tasks
// ---------------------------------------------------------------------------
#[test]
fn test_complex_changeover_mixed_status_lz4_roundtrip() {
    let val = ArtistChangeover {
        from_artist: "DJ Spectrum".to_string(),
        to_artist: "Sonic Architecture".to_string(),
        total_window_min: 45,
        tasks: vec![
            ChangeoverTask {
                task_id: 1,
                description: "Remove DJ booth and cabling".to_string(),
                department: "Stage Crew".to_string(),
                start_offset_min: 0,
                duration_min: 10,
                completed: true,
                crew_required: 4,
            },
            ChangeoverTask {
                task_id: 2,
                description: "Roll out drum riser and backline".to_string(),
                department: "Stage Crew".to_string(),
                start_offset_min: 5,
                duration_min: 12,
                completed: true,
                crew_required: 6,
            },
            ChangeoverTask {
                task_id: 3,
                description: "Patch 48-channel stage box".to_string(),
                department: "Audio".to_string(),
                start_offset_min: 10,
                duration_min: 8,
                completed: true,
                crew_required: 2,
            },
            ChangeoverTask {
                task_id: 4,
                description: "Focus follow spots and key light".to_string(),
                department: "Lighting".to_string(),
                start_offset_min: 12,
                duration_min: 10,
                completed: false,
                crew_required: 3,
            },
            ChangeoverTask {
                task_id: 5,
                description: "Deploy video content to media server".to_string(),
                department: "Video".to_string(),
                start_offset_min: 10,
                duration_min: 5,
                completed: false,
                crew_required: 1,
            },
            ChangeoverTask {
                task_id: 6,
                description: "IEM frequency coordination check".to_string(),
                department: "RF Tech".to_string(),
                start_offset_min: 15,
                duration_min: 10,
                completed: false,
                crew_required: 1,
            },
            ChangeoverTask {
                task_id: 7,
                description: "Full line check with monitor engineer".to_string(),
                department: "Audio".to_string(),
                start_offset_min: 25,
                duration_min: 12,
                completed: false,
                crew_required: 3,
            },
            ChangeoverTask {
                task_id: 8,
                description: "Arm pyro cues for opening sequence".to_string(),
                department: "Pyrotechnics".to_string(),
                start_offset_min: 38,
                duration_min: 5,
                completed: false,
                crew_required: 2,
            },
        ],
    };
    let enc = encode_to_vec(&val).expect("encode complex changeover");
    let compressed = compress_lz4(&enc).expect("compress complex changeover");
    let decompressed = decompress_lz4(&compressed).expect("decompress complex changeover");
    let (decoded, _): (ArtistChangeover, usize) =
        decode_from_slice(&decompressed).expect("decode complex changeover");
    assert_eq!(val, decoded);
}
