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

// ── Domain types: Venue seating & ticket inventory ───────────────────────────

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
