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

// ── Domain types: Merch, crowd, VIP, broadcast, settlement, & bundle ─────────

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
