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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ShipmentStatus {
    Created,
    InTransit,
    OutForDelivery,
    Delivered,
    Returned,
    Lost,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Carrier {
    FedEx,
    UPS,
    DHL,
    USPS,
    Amazon,
    Custom(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TrackingEvent {
    event_id: u64,
    timestamp: u64,
    location: String,
    status: ShipmentStatus,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Shipment {
    tracking_number: String,
    carrier: Carrier,
    origin: String,
    destination: String,
    events: Vec<TrackingEvent>,
    weight_kg: f64,
}

fn make_event(
    event_id: u64,
    timestamp: u64,
    location: &str,
    status: ShipmentStatus,
    notes: Option<&str>,
) -> TrackingEvent {
    TrackingEvent {
        event_id,
        timestamp,
        location: location.to_string(),
        status,
        notes: notes.map(|s| s.to_string()),
    }
}

fn make_shipment(
    tracking_number: &str,
    carrier: Carrier,
    origin: &str,
    destination: &str,
    events: Vec<TrackingEvent>,
    weight_kg: f64,
) -> Shipment {
    Shipment {
        tracking_number: tracking_number.to_string(),
        carrier,
        origin: origin.to_string(),
        destination: destination.to_string(),
        events,
        weight_kg,
    }
}

// Test 1: ShipmentStatus::Created roundtrip standard config
#[test]
fn test_shipment_status_created_roundtrip() {
    let cfg = config::standard();
    let status = ShipmentStatus::Created;
    let bytes = encode_to_vec(&status, cfg).expect("encode ShipmentStatus::Created");
    let (decoded, _): (ShipmentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ShipmentStatus::Created");
    assert_eq!(status, decoded);
}

// Test 2: ShipmentStatus::InTransit roundtrip standard config
#[test]
fn test_shipment_status_in_transit_roundtrip() {
    let cfg = config::standard();
    let status = ShipmentStatus::InTransit;
    let bytes = encode_to_vec(&status, cfg).expect("encode ShipmentStatus::InTransit");
    let (decoded, _): (ShipmentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ShipmentStatus::InTransit");
    assert_eq!(status, decoded);
}

// Test 3: ShipmentStatus::OutForDelivery roundtrip standard config
#[test]
fn test_shipment_status_out_for_delivery_roundtrip() {
    let cfg = config::standard();
    let status = ShipmentStatus::OutForDelivery;
    let bytes = encode_to_vec(&status, cfg).expect("encode ShipmentStatus::OutForDelivery");
    let (decoded, _): (ShipmentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ShipmentStatus::OutForDelivery");
    assert_eq!(status, decoded);
}

// Test 4: ShipmentStatus::Delivered roundtrip standard config
#[test]
fn test_shipment_status_delivered_roundtrip() {
    let cfg = config::standard();
    let status = ShipmentStatus::Delivered;
    let bytes = encode_to_vec(&status, cfg).expect("encode ShipmentStatus::Delivered");
    let (decoded, _): (ShipmentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ShipmentStatus::Delivered");
    assert_eq!(status, decoded);
}

// Test 5: ShipmentStatus::Returned roundtrip standard config
#[test]
fn test_shipment_status_returned_roundtrip() {
    let cfg = config::standard();
    let status = ShipmentStatus::Returned;
    let bytes = encode_to_vec(&status, cfg).expect("encode ShipmentStatus::Returned");
    let (decoded, _): (ShipmentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ShipmentStatus::Returned");
    assert_eq!(status, decoded);
}

// Test 6: ShipmentStatus::Lost roundtrip standard config
#[test]
fn test_shipment_status_lost_roundtrip() {
    let cfg = config::standard();
    let status = ShipmentStatus::Lost;
    let bytes = encode_to_vec(&status, cfg).expect("encode ShipmentStatus::Lost");
    let (decoded, _): (ShipmentStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ShipmentStatus::Lost");
    assert_eq!(status, decoded);
}

// Test 7: All Carrier unit variants and Custom variant roundtrip
#[test]
fn test_all_carrier_variants_roundtrip() {
    let cfg = config::standard();
    let carriers = vec![
        Carrier::FedEx,
        Carrier::UPS,
        Carrier::DHL,
        Carrier::USPS,
        Carrier::Amazon,
        Carrier::Custom("FreightLine Express".to_string()),
    ];
    let bytes = encode_to_vec(&carriers, cfg).expect("encode Vec<Carrier> all variants");
    let (decoded, _): (Vec<Carrier>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Carrier> all variants");
    assert_eq!(carriers, decoded);
    assert_eq!(decoded.len(), 6);
    assert_eq!(
        decoded[5],
        Carrier::Custom("FreightLine Express".to_string())
    );
}

// Test 8: Carrier::Custom with unicode string roundtrip
#[test]
fn test_carrier_custom_unicode_roundtrip() {
    let cfg = config::standard();
    let carrier = Carrier::Custom("速達便サービス株式会社".to_string());
    let bytes = encode_to_vec(&carrier, cfg).expect("encode Carrier::Custom unicode");
    let (decoded, _): (Carrier, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Carrier::Custom unicode");
    assert_eq!(carrier, decoded);
    if let Carrier::Custom(ref name) = decoded {
        assert_eq!(name, "速達便サービス株式会社");
    } else {
        panic!("expected Carrier::Custom");
    }
}

// Test 9: TrackingEvent with Some(notes) roundtrip standard config
#[test]
fn test_tracking_event_with_notes_roundtrip() {
    let cfg = config::standard();
    let event = make_event(
        1001,
        1_700_000_000,
        "Chicago O'Hare Sort Facility, IL",
        ShipmentStatus::InTransit,
        Some("Package scanned at hub — on schedule"),
    );
    let bytes = encode_to_vec(&event, cfg).expect("encode TrackingEvent with notes");
    let (decoded, _): (TrackingEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TrackingEvent with notes");
    assert_eq!(event, decoded);
    assert!(decoded.notes.is_some());
}

// Test 10: TrackingEvent with None notes roundtrip standard config
#[test]
fn test_tracking_event_without_notes_roundtrip() {
    let cfg = config::standard();
    let event = make_event(
        1002,
        1_700_001_000,
        "Memphis Distribution Center, TN",
        ShipmentStatus::InTransit,
        None,
    );
    let bytes = encode_to_vec(&event, cfg).expect("encode TrackingEvent without notes");
    let (decoded, _): (TrackingEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TrackingEvent without notes");
    assert_eq!(event, decoded);
    assert!(decoded.notes.is_none());
}

// Test 11: Shipment roundtrip with standard config
#[test]
fn test_shipment_roundtrip_standard() {
    let cfg = config::standard();
    let events = vec![
        make_event(
            1,
            1_700_000_000,
            "Los Angeles, CA",
            ShipmentStatus::Created,
            Some("Package label created"),
        ),
        make_event(
            2,
            1_700_003_600,
            "Los Angeles Hub, CA",
            ShipmentStatus::InTransit,
            None,
        ),
        make_event(
            3,
            1_700_090_000,
            "Phoenix Sort Facility, AZ",
            ShipmentStatus::InTransit,
            None,
        ),
        make_event(
            4,
            1_700_180_000,
            "Dallas Distribution Center, TX",
            ShipmentStatus::OutForDelivery,
            Some("Out with driver"),
        ),
        make_event(
            5,
            1_700_195_000,
            "Dallas, TX 75201",
            ShipmentStatus::Delivered,
            Some("Left at front door"),
        ),
    ];
    let shipment = make_shipment(
        "1Z999AA10123456784",
        Carrier::UPS,
        "Los Angeles, CA 90001",
        "Dallas, TX 75201",
        events,
        2.35,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment standard");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment standard");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.events.len(), 5);
    assert_eq!(decoded.carrier, Carrier::UPS);
}

// Test 12: Shipment roundtrip with big_endian config
#[test]
fn test_shipment_roundtrip_big_endian() {
    let cfg = config::standard().with_big_endian();
    let events = vec![
        make_event(
            10,
            1_710_000_000,
            "Frankfurt Airport, Germany",
            ShipmentStatus::Created,
            None,
        ),
        make_event(
            11,
            1_710_043_200,
            "Leipzig Hub, Germany",
            ShipmentStatus::InTransit,
            Some("Customs cleared"),
        ),
        make_event(
            12,
            1_710_129_600,
            "New York JFK, NY",
            ShipmentStatus::InTransit,
            None,
        ),
        make_event(
            13,
            1_710_216_000,
            "New York, NY 10001",
            ShipmentStatus::Delivered,
            Some("Signature obtained"),
        ),
    ];
    let shipment = make_shipment(
        "1234567890DE",
        Carrier::DHL,
        "Frankfurt, Germany",
        "New York, NY 10001",
        events,
        5.72,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment big_endian");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment big_endian");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.carrier, Carrier::DHL);
    assert_eq!(decoded.weight_kg, 5.72);
}

// Test 13: Shipment roundtrip with fixed_int_encoding config
#[test]
fn test_shipment_roundtrip_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let events = vec![
        make_event(
            20,
            1_720_000_000,
            "Seattle, WA 98101",
            ShipmentStatus::Created,
            None,
        ),
        make_event(
            21,
            1_720_086_400,
            "Portland Distribution, OR",
            ShipmentStatus::InTransit,
            None,
        ),
        make_event(
            22,
            1_720_172_800,
            "San Francisco, CA 94102",
            ShipmentStatus::Delivered,
            Some("Package delivered to mailroom"),
        ),
    ];
    let shipment = make_shipment(
        "9400111899223462505846",
        Carrier::USPS,
        "Seattle, WA 98101",
        "San Francisco, CA 94102",
        events,
        0.45,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment fixed_int");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment fixed_int");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.carrier, Carrier::USPS);
    assert_eq!(decoded.events.len(), 3);
}

// Test 14: Shipment with empty events roundtrip
#[test]
fn test_shipment_empty_events_roundtrip() {
    let cfg = config::standard();
    let shipment = make_shipment(
        "PENDING-0000",
        Carrier::FedEx,
        "Miami, FL 33101",
        "Boston, MA 02101",
        vec![],
        0.0,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment empty events");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment empty events");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.events.len(), 0);
    assert_eq!(decoded.weight_kg, 0.0);
}

// Test 15: Shipment with 100 large events roundtrip
#[test]
fn test_shipment_large_events_roundtrip() {
    let cfg = config::standard();
    let events: Vec<TrackingEvent> = (0u64..100)
        .map(|i| {
            let location = format!("Waypoint Hub {i}, Distribution Center, Sector {}", i % 10);
            let notes_str = if i % 3 == 0 {
                Some(format!(
                    "Automated scan at checkpoint {i} — batch ref: BATCH-{:06}",
                    i * 17
                ))
            } else {
                None
            };
            TrackingEvent {
                event_id: i,
                timestamp: 1_700_000_000 + i * 3_600,
                location,
                status: if i % 6 == 0 {
                    ShipmentStatus::Created
                } else if i % 6 == 1 {
                    ShipmentStatus::InTransit
                } else if i % 6 == 2 {
                    ShipmentStatus::OutForDelivery
                } else if i % 6 == 3 {
                    ShipmentStatus::Delivered
                } else if i % 6 == 4 {
                    ShipmentStatus::Returned
                } else {
                    ShipmentStatus::Lost
                },
                notes: notes_str,
            }
        })
        .collect();
    let shipment = make_shipment(
        "LARGE-ROUTE-100-HOPS",
        Carrier::Amazon,
        "Seattle, WA 98108",
        "New York, NY 10001",
        events,
        12.80,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment 100 events");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment 100 events");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.events.len(), 100);
    assert_eq!(decoded.carrier, Carrier::Amazon);
}

// Test 16: Shipment with Custom carrier roundtrip standard config
#[test]
fn test_shipment_custom_carrier_roundtrip() {
    let cfg = config::standard();
    let events = vec![
        make_event(
            30,
            1_730_000_000,
            "Tokyo Narita Airport, Japan",
            ShipmentStatus::Created,
            Some("International express shipment"),
        ),
        make_event(
            31,
            1_730_172_800,
            "Los Angeles, CA 90045",
            ShipmentStatus::InTransit,
            None,
        ),
        make_event(
            32,
            1_730_259_200,
            "Phoenix, AZ 85001",
            ShipmentStatus::Returned,
            Some("Recipient refused delivery"),
        ),
    ];
    let shipment = make_shipment(
        "JP-EX-20240315-7743",
        Carrier::Custom("Japan Express Logistics Co.".to_string()),
        "Tokyo, Japan",
        "Phoenix, AZ 85001",
        events,
        8.14,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment custom carrier");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment custom carrier");
    assert_eq!(shipment, decoded);
    assert_eq!(
        decoded.carrier,
        Carrier::Custom("Japan Express Logistics Co.".to_string())
    );
    assert_eq!(
        decoded.events.last().expect("last event").status,
        ShipmentStatus::Returned
    );
}

// Test 17: Consumed bytes equals encoded length for Shipment
#[test]
fn test_shipment_consumed_bytes_equals_len() {
    let cfg = config::standard();
    let events = vec![
        make_event(
            40,
            1_740_000_000,
            "Houston, TX 77001",
            ShipmentStatus::Created,
            None,
        ),
        make_event(
            41,
            1_740_043_200,
            "San Antonio, TX 78201",
            ShipmentStatus::InTransit,
            Some("On track"),
        ),
        make_event(
            42,
            1_740_086_400,
            "Austin, TX 78701",
            ShipmentStatus::Delivered,
            None,
        ),
    ];
    let shipment = make_shipment(
        "BYTES-CHECK-7890",
        Carrier::FedEx,
        "Houston, TX 77001",
        "Austin, TX 78701",
        events,
        1.75,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment for consumed bytes check");
    let (_decoded, consumed): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment for consumed bytes check");
    assert_eq!(consumed, bytes.len());
}

// Test 18: Vec<Shipment> roundtrip standard config
#[test]
fn test_vec_shipment_roundtrip() {
    let cfg = config::standard();
    let shipments = vec![
        make_shipment(
            "SHIP-001",
            Carrier::FedEx,
            "Atlanta, GA 30301",
            "Charlotte, NC 28201",
            vec![make_event(
                1,
                1_750_000_000,
                "Atlanta, GA",
                ShipmentStatus::Delivered,
                None,
            )],
            0.90,
        ),
        make_shipment(
            "SHIP-002",
            Carrier::UPS,
            "Denver, CO 80201",
            "Salt Lake City, UT 84101",
            vec![
                make_event(
                    2,
                    1_750_010_000,
                    "Denver, CO",
                    ShipmentStatus::Created,
                    Some("Label created"),
                ),
                make_event(
                    3,
                    1_750_096_400,
                    "Salt Lake City, UT",
                    ShipmentStatus::InTransit,
                    None,
                ),
            ],
            3.20,
        ),
        make_shipment(
            "SHIP-003",
            Carrier::Custom("RegionalFreight LLC".to_string()),
            "Portland, OR 97201",
            "Vancouver, BC",
            vec![make_event(
                4,
                1_750_200_000,
                "Border Crossing, WA",
                ShipmentStatus::Lost,
                Some("Customs hold — item not found"),
            )],
            7.55,
        ),
    ];
    let bytes = encode_to_vec(&shipments, cfg).expect("encode Vec<Shipment>");
    let (decoded, _): (Vec<Shipment>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Shipment>");
    assert_eq!(shipments, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(
        decoded[2].carrier,
        Carrier::Custom("RegionalFreight LLC".to_string())
    );
}

// Test 19: All ShipmentStatus variants in a single Vec roundtrip
#[test]
fn test_all_shipment_statuses_vec_roundtrip() {
    let cfg = config::standard();
    let statuses = vec![
        ShipmentStatus::Created,
        ShipmentStatus::InTransit,
        ShipmentStatus::OutForDelivery,
        ShipmentStatus::Delivered,
        ShipmentStatus::Returned,
        ShipmentStatus::Lost,
    ];
    let bytes = encode_to_vec(&statuses, cfg).expect("encode Vec<ShipmentStatus> all variants");
    let (decoded, _): (Vec<ShipmentStatus>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ShipmentStatus> all variants");
    assert_eq!(statuses, decoded);
    assert_eq!(decoded.len(), 6);
}

// Test 20: Shipment with big_endian and fixed_int combined config
#[test]
fn test_shipment_big_endian_fixed_int_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let events = vec![
        make_event(
            50,
            1_760_000_000,
            "London Heathrow, UK",
            ShipmentStatus::Created,
            Some("Picked up from sender"),
        ),
        make_event(
            51,
            1_760_086_400,
            "Dubai International, UAE",
            ShipmentStatus::InTransit,
            None,
        ),
        make_event(
            52,
            1_760_172_800,
            "Singapore Changi, SG",
            ShipmentStatus::InTransit,
            Some("Transit hub scan"),
        ),
        make_event(
            53,
            1_760_259_200,
            "Sydney Kingsford Smith, AU",
            ShipmentStatus::OutForDelivery,
            None,
        ),
        make_event(
            54,
            1_760_281_600,
            "Sydney, NSW 2000, AU",
            ShipmentStatus::Delivered,
            Some("Delivered to reception"),
        ),
    ];
    let shipment = make_shipment(
        "GB-AU-EXPRESS-20240315",
        Carrier::Custom("GlobalAir Freight Solutions".to_string()),
        "London, UK",
        "Sydney, NSW 2000, AU",
        events,
        22.50,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment big_endian + fixed_int");
    let (decoded, _): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment big_endian + fixed_int");
    assert_eq!(shipment, decoded);
    assert_eq!(decoded.events.len(), 5);
    assert_eq!(decoded.weight_kg, 22.50);
}

// Test 21: Cross-config non-interoperability for Shipment (standard vs big_endian produce different bytes)
#[test]
fn test_shipment_cross_config_non_interoperability() {
    let cfg_std = config::standard();
    let cfg_be = config::standard().with_big_endian();
    let events = vec![
        make_event(
            60,
            1_770_000_000,
            "Chicago, IL 60601",
            ShipmentStatus::Created,
            None,
        ),
        make_event(
            61,
            1_770_086_400,
            "Detroit, MI 48201",
            ShipmentStatus::InTransit,
            Some("On time"),
        ),
        make_event(
            62,
            1_770_172_800,
            "Toronto, ON M5H1A1",
            ShipmentStatus::Delivered,
            None,
        ),
    ];
    let shipment = make_shipment(
        "CROSS-CFG-2024",
        Carrier::FedEx,
        "Chicago, IL 60601",
        "Toronto, ON M5H1A1",
        events,
        4.00,
    );
    let bytes_std =
        encode_to_vec(&shipment, cfg_std).expect("encode Shipment std for cross-config");
    let bytes_be = encode_to_vec(&shipment, cfg_be).expect("encode Shipment be for cross-config");

    let (decoded_std, consumed_std): (Shipment, usize) =
        decode_owned_from_slice(&bytes_std, cfg_std).expect("decode Shipment std");
    let (decoded_be, consumed_be): (Shipment, usize) =
        decode_owned_from_slice(&bytes_be, cfg_be).expect("decode Shipment be");

    assert_eq!(shipment, decoded_std);
    assert_eq!(shipment, decoded_be);
    assert_eq!(consumed_std, bytes_std.len());
    assert_eq!(consumed_be, bytes_be.len());

    // big_endian encoding produces a different byte layout for multi-byte integers
    assert_ne!(
        bytes_std, bytes_be,
        "standard and big_endian configs must produce different byte representations"
    );
}

// Test 22: TrackingEvent sequence preserves order and mixed notes across all ShipmentStatus transitions
#[test]
fn test_tracking_event_full_lifecycle_sequence() {
    let cfg = config::standard();
    let events = vec![
        make_event(
            100,
            1_780_000_000,
            "Kansas City, MO 64101",
            ShipmentStatus::Created,
            Some("Order confirmed — label printed"),
        ),
        make_event(
            101,
            1_780_010_800,
            "Kansas City Warehouse, MO",
            ShipmentStatus::InTransit,
            None,
        ),
        make_event(
            102,
            1_780_054_000,
            "St. Louis Hub, MO 63101",
            ShipmentStatus::InTransit,
            Some("Transferred to regional hub"),
        ),
        make_event(
            103,
            1_780_097_200,
            "Indianapolis, IN 46201",
            ShipmentStatus::OutForDelivery,
            None,
        ),
        make_event(
            104,
            1_780_108_000,
            "Indianapolis, IN 46201",
            ShipmentStatus::Delivered,
            Some("Left with neighbor — unit 4B"),
        ),
        make_event(
            105,
            1_780_194_400,
            "Indianapolis, IN 46201",
            ShipmentStatus::Returned,
            Some("Recipient requested return"),
        ),
        make_event(
            106,
            1_780_280_800,
            "St. Louis Hub, MO 63101",
            ShipmentStatus::Lost,
            None,
        ),
    ];
    let shipment = make_shipment(
        "LIFECYCLE-FULL-7654321",
        Carrier::UPS,
        "Kansas City, MO 64101",
        "Indianapolis, IN 46201",
        events,
        3.68,
    );
    let bytes = encode_to_vec(&shipment, cfg).expect("encode Shipment full lifecycle");
    let (decoded, consumed): (Shipment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Shipment full lifecycle");
    assert_eq!(shipment, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.events.len(), 7);
    assert_eq!(decoded.events[0].status, ShipmentStatus::Created);
    assert_eq!(decoded.events[1].status, ShipmentStatus::InTransit);
    assert_eq!(decoded.events[2].status, ShipmentStatus::InTransit);
    assert_eq!(decoded.events[3].status, ShipmentStatus::OutForDelivery);
    assert_eq!(decoded.events[4].status, ShipmentStatus::Delivered);
    assert_eq!(decoded.events[5].status, ShipmentStatus::Returned);
    assert_eq!(decoded.events[6].status, ShipmentStatus::Lost);
    assert!(decoded.events[0].notes.is_some());
    assert!(decoded.events[1].notes.is_none());
    assert!(decoded.events[2].notes.is_some());
    assert!(decoded.events[3].notes.is_none());
    assert!(decoded.events[4].notes.is_some());
    assert!(decoded.events[5].notes.is_some());
    assert!(decoded.events[6].notes.is_none());
}
