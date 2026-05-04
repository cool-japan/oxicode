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

// ── Domain types: Stage production, lighting & sound ─────────────────────────

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

// ── Tests ────────────────────────────────────────────────────────────────────

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
