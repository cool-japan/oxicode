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
enum PropertyType {
    Apartment,
    House,
    Condo,
    Commercial,
    Land,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ListingStatus {
    Active,
    Pending,
    Sold,
    Withdrawn,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Property {
    property_id: u64,
    address: String,
    property_type: PropertyType,
    price: u64,
    area_sqft: f64,
    bedrooms: u8,
    bathrooms: u8,
    status: ListingStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Portfolio {
    owner_id: u64,
    properties: Vec<Property>,
    total_value: u64,
}

fn make_property(
    property_id: u64,
    address: &str,
    property_type: PropertyType,
    price: u64,
    area_sqft: f64,
    bedrooms: u8,
    bathrooms: u8,
    status: ListingStatus,
) -> Property {
    Property {
        property_id,
        address: address.to_string(),
        property_type,
        price,
        area_sqft,
        bedrooms,
        bathrooms,
        status,
    }
}

fn make_portfolio(owner_id: u64, properties: Vec<Property>, total_value: u64) -> Portfolio {
    Portfolio {
        owner_id,
        properties,
        total_value,
    }
}

// Test 1: PropertyType::Apartment roundtrip standard config
#[test]
fn test_property_type_apartment_roundtrip() {
    let cfg = config::standard();
    let pt = PropertyType::Apartment;
    let bytes = encode_to_vec(&pt, cfg).expect("encode PropertyType::Apartment");
    let (decoded, _): (PropertyType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PropertyType::Apartment");
    assert_eq!(pt, decoded);
}

// Test 2: PropertyType::House roundtrip standard config
#[test]
fn test_property_type_house_roundtrip() {
    let cfg = config::standard();
    let pt = PropertyType::House;
    let bytes = encode_to_vec(&pt, cfg).expect("encode PropertyType::House");
    let (decoded, _): (PropertyType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PropertyType::House");
    assert_eq!(pt, decoded);
}

// Test 3: PropertyType::Condo roundtrip standard config
#[test]
fn test_property_type_condo_roundtrip() {
    let cfg = config::standard();
    let pt = PropertyType::Condo;
    let bytes = encode_to_vec(&pt, cfg).expect("encode PropertyType::Condo");
    let (decoded, _): (PropertyType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PropertyType::Condo");
    assert_eq!(pt, decoded);
}

// Test 4: PropertyType::Commercial roundtrip standard config
#[test]
fn test_property_type_commercial_roundtrip() {
    let cfg = config::standard();
    let pt = PropertyType::Commercial;
    let bytes = encode_to_vec(&pt, cfg).expect("encode PropertyType::Commercial");
    let (decoded, _): (PropertyType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PropertyType::Commercial");
    assert_eq!(pt, decoded);
}

// Test 5: PropertyType::Land roundtrip standard config
#[test]
fn test_property_type_land_roundtrip() {
    let cfg = config::standard();
    let pt = PropertyType::Land;
    let bytes = encode_to_vec(&pt, cfg).expect("encode PropertyType::Land");
    let (decoded, _): (PropertyType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PropertyType::Land");
    assert_eq!(pt, decoded);
}

// Test 6: ListingStatus::Active roundtrip standard config
#[test]
fn test_listing_status_active_roundtrip() {
    let cfg = config::standard();
    let status = ListingStatus::Active;
    let bytes = encode_to_vec(&status, cfg).expect("encode ListingStatus::Active");
    let (decoded, _): (ListingStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ListingStatus::Active");
    assert_eq!(status, decoded);
}

// Test 7: ListingStatus::Pending roundtrip standard config
#[test]
fn test_listing_status_pending_roundtrip() {
    let cfg = config::standard();
    let status = ListingStatus::Pending;
    let bytes = encode_to_vec(&status, cfg).expect("encode ListingStatus::Pending");
    let (decoded, _): (ListingStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ListingStatus::Pending");
    assert_eq!(status, decoded);
}

// Test 8: ListingStatus::Sold roundtrip standard config
#[test]
fn test_listing_status_sold_roundtrip() {
    let cfg = config::standard();
    let status = ListingStatus::Sold;
    let bytes = encode_to_vec(&status, cfg).expect("encode ListingStatus::Sold");
    let (decoded, _): (ListingStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ListingStatus::Sold");
    assert_eq!(status, decoded);
}

// Test 9: ListingStatus::Withdrawn roundtrip standard config
#[test]
fn test_listing_status_withdrawn_roundtrip() {
    let cfg = config::standard();
    let status = ListingStatus::Withdrawn;
    let bytes = encode_to_vec(&status, cfg).expect("encode ListingStatus::Withdrawn");
    let (decoded, _): (ListingStatus, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ListingStatus::Withdrawn");
    assert_eq!(status, decoded);
}

// Test 10: Property roundtrip with standard config
#[test]
fn test_property_roundtrip_standard() {
    let cfg = config::standard();
    let prop = make_property(
        1001,
        "123 Maple Street, Springfield, IL 62701",
        PropertyType::House,
        350_000,
        2100.0,
        4,
        2,
        ListingStatus::Active,
    );
    let bytes = encode_to_vec(&prop, cfg).expect("encode Property standard");
    let (decoded, _): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Property standard");
    assert_eq!(prop, decoded);
}

// Test 11: Property roundtrip with big_endian config
#[test]
fn test_property_roundtrip_big_endian() {
    let cfg = config::standard().with_big_endian();
    let prop = make_property(
        2002,
        "456 Oak Avenue, Austin, TX 78701",
        PropertyType::Condo,
        525_000,
        1450.5,
        2,
        2,
        ListingStatus::Pending,
    );
    let bytes = encode_to_vec(&prop, cfg).expect("encode Property big_endian");
    let (decoded, _): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Property big_endian");
    assert_eq!(prop, decoded);
}

// Test 12: Property roundtrip with fixed_int_encoding config
#[test]
fn test_property_roundtrip_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let prop = make_property(
        3003,
        "789 Commerce Blvd, Chicago, IL 60601",
        PropertyType::Commercial,
        4_200_000,
        18_500.75,
        0,
        0,
        ListingStatus::Sold,
    );
    let bytes = encode_to_vec(&prop, cfg).expect("encode Property fixed_int");
    let (decoded, _): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Property fixed_int");
    assert_eq!(prop, decoded);
}

// Test 13: Portfolio with empty properties list
#[test]
fn test_portfolio_empty_properties() {
    let cfg = config::standard();
    let portfolio = make_portfolio(9001, vec![], 0);
    let bytes = encode_to_vec(&portfolio, cfg).expect("encode Portfolio empty properties");
    let (decoded, _): (Portfolio, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Portfolio empty properties");
    assert_eq!(portfolio, decoded);
    assert_eq!(decoded.properties.len(), 0);
    assert_eq!(decoded.total_value, 0);
}

// Test 14: Portfolio with multiple properties
#[test]
fn test_portfolio_multiple_properties() {
    let cfg = config::standard();
    let properties = vec![
        make_property(
            101,
            "10 Main St, Denver, CO 80203",
            PropertyType::Apartment,
            200_000,
            850.0,
            1,
            1,
            ListingStatus::Active,
        ),
        make_property(
            102,
            "20 Pine Rd, Denver, CO 80204",
            PropertyType::House,
            480_000,
            2400.0,
            3,
            2,
            ListingStatus::Pending,
        ),
        make_property(
            103,
            "30 Industrial Way, Denver, CO 80205",
            PropertyType::Commercial,
            1_500_000,
            6000.0,
            0,
            0,
            ListingStatus::Sold,
        ),
    ];
    let total_value: u64 = properties.iter().map(|p| p.price).sum();
    let portfolio = make_portfolio(9002, properties, total_value);
    let bytes = encode_to_vec(&portfolio, cfg).expect("encode Portfolio multiple properties");
    let (decoded, _): (Portfolio, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Portfolio multiple properties");
    assert_eq!(portfolio, decoded);
    assert_eq!(decoded.properties.len(), 3);
    assert_eq!(decoded.total_value, 2_180_000);
}

// Test 15: Consumed bytes equals encoded length for Property
#[test]
fn test_property_consumed_bytes_equals_len() {
    let cfg = config::standard();
    let prop = make_property(
        5005,
        "55 Elm Street, Boston, MA 02101",
        PropertyType::Apartment,
        310_000,
        920.0,
        1,
        1,
        ListingStatus::Active,
    );
    let bytes = encode_to_vec(&prop, cfg).expect("encode Property for size check");
    let (_decoded, consumed): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Property for size check");
    assert_eq!(consumed, bytes.len());
}

// Test 16: Consumed bytes equals encoded length for Portfolio
#[test]
fn test_portfolio_consumed_bytes_equals_len() {
    let cfg = config::standard();
    let properties = vec![
        make_property(
            201,
            "1 River Lane, Seattle, WA 98101",
            PropertyType::Land,
            750_000,
            43_560.0,
            0,
            0,
            ListingStatus::Active,
        ),
        make_property(
            202,
            "2 Harbor View, Seattle, WA 98102",
            PropertyType::House,
            890_000,
            3200.0,
            5,
            3,
            ListingStatus::Active,
        ),
    ];
    let total = properties.iter().map(|p| p.price).sum();
    let portfolio = make_portfolio(9003, properties, total);
    let bytes = encode_to_vec(&portfolio, cfg).expect("encode Portfolio for size check");
    let (_decoded, consumed): (Portfolio, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Portfolio for size check");
    assert_eq!(consumed, bytes.len());
}

// Test 17: Vec<Property> roundtrip standard config
#[test]
fn test_vec_property_roundtrip() {
    let cfg = config::standard();
    let properties = vec![
        make_property(
            301,
            "100 Sunset Blvd, Los Angeles, CA 90028",
            PropertyType::Condo,
            650_000,
            1200.0,
            2,
            2,
            ListingStatus::Active,
        ),
        make_property(
            302,
            "200 Hollywood Hills Dr, Los Angeles, CA 90068",
            PropertyType::House,
            2_500_000,
            4500.0,
            6,
            5,
            ListingStatus::Pending,
        ),
        make_property(
            303,
            "300 Downtown Ave, Los Angeles, CA 90012",
            PropertyType::Commercial,
            8_000_000,
            22_000.0,
            0,
            0,
            ListingStatus::Withdrawn,
        ),
    ];
    let bytes = encode_to_vec(&properties, cfg).expect("encode Vec<Property>");
    let (decoded, _): (Vec<Property>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Property>");
    assert_eq!(properties, decoded);
    assert_eq!(decoded.len(), 3);
}

// Test 18: All PropertyType variants in a single Vec roundtrip
#[test]
fn test_all_property_types_vec_roundtrip() {
    let cfg = config::standard();
    let types = vec![
        PropertyType::Apartment,
        PropertyType::House,
        PropertyType::Condo,
        PropertyType::Commercial,
        PropertyType::Land,
    ];
    let bytes = encode_to_vec(&types, cfg).expect("encode Vec<PropertyType> all variants");
    let (decoded, _): (Vec<PropertyType>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<PropertyType> all variants");
    assert_eq!(types, decoded);
    assert_eq!(decoded.len(), 5);
}

// Test 19: All ListingStatus variants in a single Vec roundtrip
#[test]
fn test_all_listing_statuses_vec_roundtrip() {
    let cfg = config::standard();
    let statuses = vec![
        ListingStatus::Active,
        ListingStatus::Pending,
        ListingStatus::Sold,
        ListingStatus::Withdrawn,
    ];
    let bytes = encode_to_vec(&statuses, cfg).expect("encode Vec<ListingStatus> all variants");
    let (decoded, _): (Vec<ListingStatus>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ListingStatus> all variants");
    assert_eq!(statuses, decoded);
    assert_eq!(decoded.len(), 4);
}

// Test 20: Property with extreme price and area values
#[test]
fn test_property_extreme_values() {
    let cfg = config::standard();
    let prop_max_price = make_property(
        u64::MAX,
        "1 Penthouse Plaza, New York, NY 10001",
        PropertyType::Condo,
        u64::MAX,
        f64::MAX,
        u8::MAX,
        u8::MAX,
        ListingStatus::Active,
    );
    let bytes = encode_to_vec(&prop_max_price, cfg).expect("encode Property extreme values");
    let (decoded, _): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Property extreme values");
    assert_eq!(prop_max_price.property_id, decoded.property_id);
    assert_eq!(prop_max_price.price, decoded.price);
    assert_eq!(prop_max_price.bedrooms, decoded.bedrooms);
    assert_eq!(prop_max_price.bathrooms, decoded.bathrooms);
    assert_eq!(prop_max_price.area_sqft, decoded.area_sqft);
}

// Test 21: Portfolio with big_endian and fixed_int combined config
#[test]
fn test_portfolio_big_endian_fixed_int_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let properties = vec![
        make_property(
            401,
            "40 Canal Street, New Orleans, LA 70130",
            PropertyType::House,
            275_000,
            1875.5,
            3,
            2,
            ListingStatus::Active,
        ),
        make_property(
            402,
            "41 Bourbon Street, New Orleans, LA 70116",
            PropertyType::Commercial,
            1_100_000,
            5500.0,
            0,
            0,
            ListingStatus::Sold,
        ),
    ];
    let total = properties.iter().map(|p| p.price).sum();
    let portfolio = make_portfolio(8888, properties, total);
    let bytes = encode_to_vec(&portfolio, cfg).expect("encode Portfolio big_endian + fixed_int");
    let (decoded, _): (Portfolio, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Portfolio big_endian + fixed_int");
    assert_eq!(portfolio, decoded);
    assert_eq!(decoded.total_value, 1_375_000);
}

// Test 22: Cross-config non-interoperability for Property (standard vs big_endian produce different bytes)
#[test]
fn test_property_cross_config_non_interoperability() {
    let cfg_std = config::standard();
    let cfg_be = config::standard().with_big_endian();
    let prop = make_property(
        9999,
        "99 Test Drive, Miami, FL 33101",
        PropertyType::Land,
        500_000,
        87_120.0,
        0,
        0,
        ListingStatus::Withdrawn,
    );

    let bytes_std = encode_to_vec(&prop, cfg_std).expect("encode Property std for cross-config");
    let bytes_be = encode_to_vec(&prop, cfg_be).expect("encode Property be for cross-config");

    let (decoded_std, consumed_std): (Property, usize) =
        decode_owned_from_slice(&bytes_std, cfg_std).expect("decode Property std");
    let (decoded_be, consumed_be): (Property, usize) =
        decode_owned_from_slice(&bytes_be, cfg_be).expect("decode Property be");

    assert_eq!(prop, decoded_std);
    assert_eq!(prop, decoded_be);
    assert_eq!(consumed_std, bytes_std.len());
    assert_eq!(consumed_be, bytes_be.len());

    // big_endian encoding produces a different byte layout for multi-byte integers
    assert_ne!(
        bytes_std, bytes_be,
        "standard and big_endian configs must produce different byte representations"
    );
}
