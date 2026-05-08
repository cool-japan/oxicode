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

// ── Domain types: multi-domain bundle (rider + security + settlement) ─────────

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
