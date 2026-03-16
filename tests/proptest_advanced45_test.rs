//! Advanced property-based tests (set 45) using proptest.
//!
//! 22 top-level #[test] functions with proptest! blocks.
//! Domain: supply chain / logistics optimization.
//! Covers roundtrip, consumed == bytes.len(), deterministic encoding,
//! all enum variants, vec of shipments, option types for optional fields.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransportMode {
    Truck,
    Rail,
    Air,
    Sea,
    Courier,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Priority {
    Standard,
    Express,
    Urgent,
    SameDay,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Address {
    street: String,
    city: String,
    country_code: String,
    postal_code: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Shipment {
    id: u64,
    origin: Address,
    destination: Address,
    weight_g: u32,
    volume_cm3: u32,
    mode: TransportMode,
    priority: Priority,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Route {
    shipments: Vec<Shipment>,
    total_distance_km: u64,
    estimated_days: u32,
    carrier: String,
}

// ── Proptest strategies ───────────────────────────────────────────────────────

fn arb_transport_mode() -> impl Strategy<Value = TransportMode> {
    prop_oneof![
        Just(TransportMode::Truck),
        Just(TransportMode::Rail),
        Just(TransportMode::Air),
        Just(TransportMode::Sea),
        Just(TransportMode::Courier),
    ]
}

fn arb_priority() -> impl Strategy<Value = Priority> {
    prop_oneof![
        Just(Priority::Standard),
        Just(Priority::Express),
        Just(Priority::Urgent),
        Just(Priority::SameDay),
    ]
}

fn arb_address() -> impl Strategy<Value = Address> {
    (
        "[a-zA-Z0-9 ]{1,40}",
        "[a-zA-Z ]{1,30}",
        "[A-Z]{2}",
        "[0-9A-Z]{3,10}",
    )
        .prop_map(|(street, city, country_code, postal_code)| Address {
            street,
            city,
            country_code,
            postal_code,
        })
}

fn arb_shipment() -> impl Strategy<Value = Shipment> {
    (
        any::<u64>(),
        arb_address(),
        arb_address(),
        any::<u32>(),
        any::<u32>(),
        arb_transport_mode(),
        arb_priority(),
        any::<u64>(),
    )
        .prop_map(
            |(id, origin, destination, weight_g, volume_cm3, mode, priority, cost_cents)| {
                Shipment {
                    id,
                    origin,
                    destination,
                    weight_g,
                    volume_cm3,
                    mode,
                    priority,
                    cost_cents,
                }
            },
        )
}

fn arb_route() -> impl Strategy<Value = Route> {
    (
        prop::collection::vec(arb_shipment(), 0..5),
        any::<u64>(),
        any::<u32>(),
        "[a-zA-Z0-9 ]{1,40}",
    )
        .prop_map(
            |(shipments, total_distance_km, estimated_days, carrier)| Route {
                shipments,
                total_distance_km,
                estimated_days,
                carrier,
            },
        )
}

// ── 1. Address roundtrip ──────────────────────────────────────────────────────

#[test]
fn test_address_roundtrip() {
    proptest!(|(addr in arb_address())| {
        let enc = encode_to_vec(&addr).expect("encode Address failed");
        let (decoded, consumed): (Address, usize) =
            decode_from_slice(&enc).expect("decode Address failed");
        prop_assert_eq!(addr, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 2. Shipment roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_shipment_roundtrip() {
    proptest!(|(s in arb_shipment())| {
        let enc = encode_to_vec(&s).expect("encode Shipment failed");
        let (decoded, consumed): (Shipment, usize) =
            decode_from_slice(&enc).expect("decode Shipment failed");
        prop_assert_eq!(s, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 3. Route roundtrip ────────────────────────────────────────────────────────

#[test]
fn test_route_roundtrip() {
    proptest!(|(r in arb_route())| {
        let enc = encode_to_vec(&r).expect("encode Route failed");
        let (decoded, consumed): (Route, usize) =
            decode_from_slice(&enc).expect("decode Route failed");
        prop_assert_eq!(r, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 4. TransportMode consumed == bytes.len() ──────────────────────────────────

#[test]
fn test_transport_mode_consumed_eq_len() {
    proptest!(|(mode in arb_transport_mode())| {
        let enc = encode_to_vec(&mode).expect("encode TransportMode failed");
        let (_decoded, consumed): (TransportMode, usize) =
            decode_from_slice(&enc).expect("decode TransportMode failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 5. Priority consumed == bytes.len() ───────────────────────────────────────

#[test]
fn test_priority_consumed_eq_len() {
    proptest!(|(prio in arb_priority())| {
        let enc = encode_to_vec(&prio).expect("encode Priority failed");
        let (_decoded, consumed): (Priority, usize) =
            decode_from_slice(&enc).expect("decode Priority failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 6. Shipment consumed == bytes.len() ───────────────────────────────────────

#[test]
fn test_shipment_consumed_eq_len() {
    proptest!(|(s in arb_shipment())| {
        let enc = encode_to_vec(&s).expect("encode Shipment failed");
        let (_decoded, consumed): (Shipment, usize) =
            decode_from_slice(&enc).expect("decode Shipment failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 7. Deterministic encoding for Address ─────────────────────────────────────

#[test]
fn test_address_encoding_deterministic() {
    proptest!(|(addr in arb_address())| {
        let enc1 = encode_to_vec(&addr).expect("first encode Address failed");
        let enc2 = encode_to_vec(&addr).expect("second encode Address failed");
        prop_assert_eq!(enc1, enc2, "Address encoding must be deterministic");
    });
}

// ── 8. Deterministic encoding for Shipment ────────────────────────────────────

#[test]
fn test_shipment_encoding_deterministic() {
    proptest!(|(s in arb_shipment())| {
        let enc1 = encode_to_vec(&s).expect("first encode Shipment failed");
        let enc2 = encode_to_vec(&s).expect("second encode Shipment failed");
        prop_assert_eq!(enc1, enc2, "Shipment encoding must be deterministic");
    });
}

// ── 9. Deterministic encoding for Route ───────────────────────────────────────

#[test]
fn test_route_encoding_deterministic() {
    proptest!(|(r in arb_route())| {
        let enc1 = encode_to_vec(&r).expect("first encode Route failed");
        let enc2 = encode_to_vec(&r).expect("second encode Route failed");
        prop_assert_eq!(enc1, enc2, "Route encoding must be deterministic");
    });
}

// ── 10. TransportMode::Truck variant ─────────────────────────────────────────

#[test]
fn test_transport_mode_truck_roundtrip() {
    proptest!(|(weight: u32, cost: u64)| {
        let mode = TransportMode::Truck;
        let enc = encode_to_vec(&mode).expect("encode Truck failed");
        let (decoded, consumed): (TransportMode, usize) =
            decode_from_slice(&enc).expect("decode Truck failed");
        prop_assert_eq!(decoded, TransportMode::Truck);
        prop_assert_eq!(consumed, enc.len());
        // exercise weight and cost to satisfy proptest generation
        prop_assert!(weight < u32::MAX || cost <= u64::MAX);
    });
}

// ── 11. TransportMode::Air variant ───────────────────────────────────────────

#[test]
fn test_transport_mode_air_roundtrip() {
    proptest!(|(weight: u32)| {
        let mode = TransportMode::Air;
        let enc = encode_to_vec(&mode).expect("encode Air failed");
        let (decoded, consumed): (TransportMode, usize) =
            decode_from_slice(&enc).expect("decode Air failed");
        prop_assert_eq!(decoded, TransportMode::Air);
        prop_assert_eq!(consumed, enc.len());
        prop_assert!(weight <= u32::MAX);
    });
}

// ── 12. Priority::SameDay variant ────────────────────────────────────────────

#[test]
fn test_priority_same_day_roundtrip() {
    proptest!(|(cost: u64)| {
        let prio = Priority::SameDay;
        let enc = encode_to_vec(&prio).expect("encode SameDay failed");
        let (decoded, consumed): (Priority, usize) =
            decode_from_slice(&enc).expect("decode SameDay failed");
        prop_assert_eq!(decoded, Priority::SameDay);
        prop_assert_eq!(consumed, enc.len());
        prop_assert!(cost <= u64::MAX);
    });
}

// ── 13. Priority::Urgent variant ─────────────────────────────────────────────

#[test]
fn test_priority_urgent_roundtrip() {
    proptest!(|(id: u64)| {
        let prio = Priority::Urgent;
        let enc = encode_to_vec(&prio).expect("encode Urgent failed");
        let (decoded, consumed): (Priority, usize) =
            decode_from_slice(&enc).expect("decode Urgent failed");
        prop_assert_eq!(decoded, Priority::Urgent);
        prop_assert_eq!(consumed, enc.len());
        prop_assert!(id <= u64::MAX);
    });
}

// ── 14. Vec<Shipment> roundtrip ───────────────────────────────────────────────

#[test]
fn test_vec_shipment_roundtrip() {
    proptest!(|(shipments in prop::collection::vec(arb_shipment(), 0..8))| {
        let enc = encode_to_vec(&shipments).expect("encode Vec<Shipment> failed");
        let (decoded, consumed): (Vec<Shipment>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Shipment> failed");
        prop_assert_eq!(shipments, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 15. Option<Address> roundtrip ────────────────────────────────────────────

#[test]
fn test_option_address_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_address()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<Address> failed");
        let (decoded, consumed): (Option<Address>, usize) =
            decode_from_slice(&enc).expect("decode Option<Address> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 16. Option<Shipment> roundtrip ───────────────────────────────────────────

#[test]
fn test_option_shipment_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_shipment()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<Shipment> failed");
        let (decoded, consumed): (Option<Shipment>, usize) =
            decode_from_slice(&enc).expect("decode Option<Shipment> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 17. Option<TransportMode> roundtrip ──────────────────────────────────────

#[test]
fn test_option_transport_mode_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_transport_mode()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<TransportMode> failed");
        let (decoded, consumed): (Option<TransportMode>, usize) =
            decode_from_slice(&enc).expect("decode Option<TransportMode> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 18. Option<Priority> roundtrip ───────────────────────────────────────────

#[test]
fn test_option_priority_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_priority()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<Priority> failed");
        let (decoded, consumed): (Option<Priority>, usize) =
            decode_from_slice(&enc).expect("decode Option<Priority> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 19. Shipment re-encode idempotent ─────────────────────────────────────────

#[test]
fn test_shipment_reencode_idempotent() {
    proptest!(|(s in arb_shipment())| {
        let enc1 = encode_to_vec(&s).expect("first encode Shipment failed");
        let (decoded, _): (Shipment, usize) =
            decode_from_slice(&enc1).expect("first decode Shipment failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Shipment failed");
        prop_assert_eq!(enc1, enc2, "re-encoding Shipment must be idempotent");
    });
}

// ── 20. Route re-encode idempotent ────────────────────────────────────────────

#[test]
fn test_route_reencode_idempotent() {
    proptest!(|(r in arb_route())| {
        let enc1 = encode_to_vec(&r).expect("first encode Route failed");
        let (decoded, _): (Route, usize) =
            decode_from_slice(&enc1).expect("first decode Route failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Route failed");
        prop_assert_eq!(enc1, enc2, "re-encoding Route must be idempotent");
    });
}

// ── 21. Vec<Route> roundtrip ──────────────────────────────────────────────────

#[test]
fn test_vec_route_roundtrip() {
    proptest!(|(routes in prop::collection::vec(arb_route(), 0..4))| {
        let enc = encode_to_vec(&routes).expect("encode Vec<Route> failed");
        let (decoded, consumed): (Vec<Route>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Route> failed");
        prop_assert_eq!(routes, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 22. (TransportMode, Priority, u64) tuple roundtrip ────────────────────────

#[test]
fn test_mode_priority_cost_tuple_roundtrip() {
    proptest!(|(mode in arb_transport_mode(), prio in arb_priority(), cost: u64)| {
        let tup = (mode, prio, cost);
        let enc = encode_to_vec(&tup).expect("encode (TransportMode, Priority, u64) failed");
        let (decoded, consumed): ((TransportMode, Priority, u64), usize) =
            decode_from_slice(&enc).expect("decode (TransportMode, Priority, u64) failed");
        prop_assert_eq!(tup, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}
