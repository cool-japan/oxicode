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
    SingleFamily,
    Condo,
    Townhouse,
    MultiFamily,
    Commercial,
    Land,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ListingStatus {
    Active,
    Pending,
    Sold,
    Withdrawn,
    Expired,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MortgageType {
    Conventional,
    FHA,
    VA,
    USDA,
    Jumbo,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AppraisalStatus {
    Ordered,
    InProgress,
    Completed,
    UnderReview,
    Approved,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Property {
    property_id: u64,
    address: String,
    property_type: PropertyType,
    bedrooms: u8,
    bathrooms_x2: u8,
    sqft: u32,
    lot_sqft: u32,
    year_built: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Listing {
    listing_id: u64,
    property_id: u64,
    list_price_cents: u64,
    status: ListingStatus,
    listed_date: u64,
    agent_id: u32,
    description: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Mortgage {
    mortgage_id: u64,
    property_id: u64,
    borrower_id: u64,
    mortgage_type: MortgageType,
    principal_cents: u64,
    interest_rate_x10000: u32,
    term_months: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Appraisal {
    appraisal_id: u64,
    property_id: u64,
    appraiser_id: u32,
    appraised_value_cents: u64,
    status: AppraisalStatus,
    completed_date: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PropertyTax {
    property_id: u64,
    year: u16,
    assessed_value_cents: u64,
    tax_rate_x10000: u32,
    amount_due_cents: u64,
}

// --- Test 1: Property single_family round-trip with standard config ---
#[test]
fn test_property_single_family_standard() {
    let property = Property {
        property_id: 1001,
        address: "123 Maple Street, Springfield, IL 62701".to_string(),
        property_type: PropertyType::SingleFamily,
        bedrooms: 4,
        bathrooms_x2: 5,
        sqft: 2450,
        lot_sqft: 8700,
        year_built: 1998,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&property, cfg).expect("encode property single family");
    let (decoded, consumed): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode property single family");
    assert_eq!(property, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 2: Property condo round-trip with big_endian config ---
#[test]
fn test_property_condo_big_endian() {
    let property = Property {
        property_id: 2002,
        address: "Unit 14B, 500 Lakeview Drive, Chicago, IL 60601".to_string(),
        property_type: PropertyType::Condo,
        bedrooms: 2,
        bathrooms_x2: 2,
        sqft: 1100,
        lot_sqft: 0,
        year_built: 2010,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&property, cfg).expect("encode property condo big endian");
    let (decoded, consumed): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode property condo big endian");
    assert_eq!(property, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 3: Property commercial with fixed_int config ---
#[test]
fn test_property_commercial_fixed_int() {
    let property = Property {
        property_id: 3003,
        address: "789 Commerce Blvd, Dallas, TX 75201".to_string(),
        property_type: PropertyType::Commercial,
        bedrooms: 0,
        bathrooms_x2: 4,
        sqft: 15000,
        lot_sqft: 45000,
        year_built: 1985,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&property, cfg).expect("encode property commercial fixed int");
    let (decoded, consumed): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode property commercial fixed int");
    assert_eq!(property, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 4: Property land type round-trip ---
#[test]
fn test_property_land_standard() {
    let property = Property {
        property_id: 4004,
        address: "Rural Route 7, Farmington, MO 63640".to_string(),
        property_type: PropertyType::Land,
        bedrooms: 0,
        bathrooms_x2: 0,
        sqft: 0,
        lot_sqft: 217800,
        year_built: 0,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&property, cfg).expect("encode property land");
    let (decoded, consumed): (Property, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode property land");
    assert_eq!(property, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 5: Listing active with Some(description) standard config ---
#[test]
fn test_listing_active_with_description_standard() {
    let listing = Listing {
        listing_id: 5001,
        property_id: 1001,
        list_price_cents: 32500000,
        status: ListingStatus::Active,
        listed_date: 1700000000,
        agent_id: 88001,
        description: Some(
            "Charming 4-bedroom home with updated kitchen and hardwood floors throughout."
                .to_string(),
        ),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&listing, cfg).expect("encode listing active with description");
    let (decoded, consumed): (Listing, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode listing active with description");
    assert_eq!(listing, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 6: Listing pending with None description big_endian ---
#[test]
fn test_listing_pending_no_description_big_endian() {
    let listing = Listing {
        listing_id: 6001,
        property_id: 2002,
        list_price_cents: 45000000,
        status: ListingStatus::Pending,
        listed_date: 1701000000,
        agent_id: 88002,
        description: None,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&listing, cfg).expect("encode listing pending no description");
    let (decoded, consumed): (Listing, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode listing pending no description");
    assert_eq!(listing, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 7: Listing sold fixed_int config ---
#[test]
fn test_listing_sold_fixed_int() {
    let listing = Listing {
        listing_id: 7001,
        property_id: 3003,
        list_price_cents: 87500000,
        status: ListingStatus::Sold,
        listed_date: 1699000000,
        agent_id: 88003,
        description: Some(
            "Prime commercial location near highway interchange, high visibility.".to_string(),
        ),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&listing, cfg).expect("encode listing sold fixed int");
    let (decoded, consumed): (Listing, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode listing sold fixed int");
    assert_eq!(listing, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 8: Listing withdrawn status standard config ---
#[test]
fn test_listing_withdrawn_standard() {
    let listing = Listing {
        listing_id: 8001,
        property_id: 4004,
        list_price_cents: 12000000,
        status: ListingStatus::Withdrawn,
        listed_date: 1698000000,
        agent_id: 88004,
        description: None,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&listing, cfg).expect("encode listing withdrawn");
    let (decoded, consumed): (Listing, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode listing withdrawn");
    assert_eq!(listing, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 9: Mortgage conventional round-trip standard config ---
#[test]
fn test_mortgage_conventional_standard() {
    let mortgage = Mortgage {
        mortgage_id: 9001,
        property_id: 1001,
        borrower_id: 200001,
        mortgage_type: MortgageType::Conventional,
        principal_cents: 26000000,
        interest_rate_x10000: 675,
        term_months: 360,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&mortgage, cfg).expect("encode mortgage conventional");
    let (decoded, consumed): (Mortgage, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode mortgage conventional");
    assert_eq!(mortgage, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 10: Mortgage FHA big_endian config ---
#[test]
fn test_mortgage_fha_big_endian() {
    let mortgage = Mortgage {
        mortgage_id: 10001,
        property_id: 2002,
        borrower_id: 200002,
        mortgage_type: MortgageType::FHA,
        principal_cents: 38000000,
        interest_rate_x10000: 625,
        term_months: 360,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&mortgage, cfg).expect("encode mortgage FHA big endian");
    let (decoded, consumed): (Mortgage, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode mortgage FHA big endian");
    assert_eq!(mortgage, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 11: Mortgage VA fixed_int config ---
#[test]
fn test_mortgage_va_fixed_int() {
    let mortgage = Mortgage {
        mortgage_id: 11001,
        property_id: 1001,
        borrower_id: 200003,
        mortgage_type: MortgageType::VA,
        principal_cents: 28500000,
        interest_rate_x10000: 600,
        term_months: 240,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&mortgage, cfg).expect("encode mortgage VA fixed int");
    let (decoded, consumed): (Mortgage, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode mortgage VA fixed int");
    assert_eq!(mortgage, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 12: Mortgage jumbo standard config ---
#[test]
fn test_mortgage_jumbo_standard() {
    let mortgage = Mortgage {
        mortgage_id: 12001,
        property_id: 3003,
        borrower_id: 200004,
        mortgage_type: MortgageType::Jumbo,
        principal_cents: 150000000,
        interest_rate_x10000: 725,
        term_months: 360,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&mortgage, cfg).expect("encode mortgage jumbo");
    let (decoded, consumed): (Mortgage, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode mortgage jumbo");
    assert_eq!(mortgage, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 13: Appraisal completed with Some(completed_date) standard config ---
#[test]
fn test_appraisal_completed_with_date_standard() {
    let appraisal = Appraisal {
        appraisal_id: 13001,
        property_id: 1001,
        appraiser_id: 300001,
        appraised_value_cents: 33500000,
        status: AppraisalStatus::Completed,
        completed_date: Some(1700500000),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&appraisal, cfg).expect("encode appraisal completed with date");
    let (decoded, consumed): (Appraisal, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode appraisal completed with date");
    assert_eq!(appraisal, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 14: Appraisal ordered with None completed_date big_endian ---
#[test]
fn test_appraisal_ordered_no_date_big_endian() {
    let appraisal = Appraisal {
        appraisal_id: 14001,
        property_id: 2002,
        appraiser_id: 300002,
        appraised_value_cents: 0,
        status: AppraisalStatus::Ordered,
        completed_date: None,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&appraisal, cfg).expect("encode appraisal ordered no date");
    let (decoded, consumed): (Appraisal, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode appraisal ordered no date");
    assert_eq!(appraisal, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 15: Appraisal in_progress fixed_int config ---
#[test]
fn test_appraisal_in_progress_fixed_int() {
    let appraisal = Appraisal {
        appraisal_id: 15001,
        property_id: 3003,
        appraiser_id: 300003,
        appraised_value_cents: 0,
        status: AppraisalStatus::InProgress,
        completed_date: None,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&appraisal, cfg).expect("encode appraisal in progress fixed int");
    let (decoded, consumed): (Appraisal, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode appraisal in progress fixed int");
    assert_eq!(appraisal, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 16: Appraisal approved with completed_date standard config ---
#[test]
fn test_appraisal_approved_standard() {
    let appraisal = Appraisal {
        appraisal_id: 16001,
        property_id: 4004,
        appraiser_id: 300004,
        appraised_value_cents: 12500000,
        status: AppraisalStatus::Approved,
        completed_date: Some(1701200000),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&appraisal, cfg).expect("encode appraisal approved");
    let (decoded, consumed): (Appraisal, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode appraisal approved");
    assert_eq!(appraisal, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 17: PropertyTax standard config ---
#[test]
fn test_property_tax_standard() {
    let tax = PropertyTax {
        property_id: 1001,
        year: 2024,
        assessed_value_cents: 30000000,
        tax_rate_x10000: 215,
        amount_due_cents: 645000,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&tax, cfg).expect("encode property tax standard");
    let (decoded, consumed): (PropertyTax, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode property tax standard");
    assert_eq!(tax, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 18: PropertyTax big_endian config ---
#[test]
fn test_property_tax_big_endian() {
    let tax = PropertyTax {
        property_id: 2002,
        year: 2023,
        assessed_value_cents: 42000000,
        tax_rate_x10000: 189,
        amount_due_cents: 793800,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&tax, cfg).expect("encode property tax big endian");
    let (decoded, consumed): (PropertyTax, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode property tax big endian");
    assert_eq!(tax, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 19: PropertyTax fixed_int config ---
#[test]
fn test_property_tax_fixed_int() {
    let tax = PropertyTax {
        property_id: 3003,
        year: 2024,
        assessed_value_cents: 80000000,
        tax_rate_x10000: 310,
        amount_due_cents: 2480000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&tax, cfg).expect("encode property tax fixed int");
    let (decoded, consumed): (PropertyTax, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode property tax fixed int");
    assert_eq!(tax, decoded);
    assert_eq!(consumed, bytes.len());
}

// --- Test 20: Vec of listings round-trip standard config ---
#[test]
fn test_vec_of_listings_standard() {
    let listings = vec![
        Listing {
            listing_id: 20001,
            property_id: 1001,
            list_price_cents: 32500000,
            status: ListingStatus::Active,
            listed_date: 1700000000,
            agent_id: 88001,
            description: Some("Move-in ready family home with large backyard.".to_string()),
        },
        Listing {
            listing_id: 20002,
            property_id: 2002,
            list_price_cents: 45000000,
            status: ListingStatus::Pending,
            listed_date: 1701000000,
            agent_id: 88002,
            description: None,
        },
        Listing {
            listing_id: 20003,
            property_id: 4004,
            list_price_cents: 12000000,
            status: ListingStatus::Expired,
            listed_date: 1695000000,
            agent_id: 88005,
            description: Some("Raw land parcel, utilities available at road.".to_string()),
        },
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&listings, cfg).expect("encode vec of listings");
    let (decoded, consumed): (Vec<Listing>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec of listings");
    assert_eq!(listings, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 3);
}

// --- Test 21: Vec of mortgages round-trip big_endian config ---
#[test]
fn test_vec_of_mortgages_big_endian() {
    let mortgages = vec![
        Mortgage {
            mortgage_id: 21001,
            property_id: 1001,
            borrower_id: 200001,
            mortgage_type: MortgageType::Conventional,
            principal_cents: 26000000,
            interest_rate_x10000: 675,
            term_months: 360,
        },
        Mortgage {
            mortgage_id: 21002,
            property_id: 2002,
            borrower_id: 200002,
            mortgage_type: MortgageType::USDA,
            principal_cents: 18000000,
            interest_rate_x10000: 590,
            term_months: 360,
        },
        Mortgage {
            mortgage_id: 21003,
            property_id: 3003,
            borrower_id: 200004,
            mortgage_type: MortgageType::Jumbo,
            principal_cents: 150000000,
            interest_rate_x10000: 725,
            term_months: 180,
        },
    ];
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&mortgages, cfg).expect("encode vec of mortgages big endian");
    let (decoded, consumed): (Vec<Mortgage>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec of mortgages big endian");
    assert_eq!(mortgages, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 3);
}

// --- Test 22: Consumed bytes check for full property pipeline fixed_int ---
#[test]
fn test_consumed_bytes_full_pipeline_fixed_int() {
    let property = Property {
        property_id: 22001,
        address: "55 Riverfront Way, Nashville, TN 37201".to_string(),
        property_type: PropertyType::Townhouse,
        bedrooms: 3,
        bathrooms_x2: 4,
        sqft: 1850,
        lot_sqft: 2200,
        year_built: 2005,
    };
    let listing = Listing {
        listing_id: 22100,
        property_id: 22001,
        list_price_cents: 41500000,
        status: ListingStatus::Active,
        listed_date: 1702000000,
        agent_id: 88010,
        description: Some("Stunning townhouse with river views and rooftop deck.".to_string()),
    };
    let mortgage = Mortgage {
        mortgage_id: 22200,
        property_id: 22001,
        borrower_id: 200010,
        mortgage_type: MortgageType::Conventional,
        principal_cents: 33200000,
        interest_rate_x10000: 700,
        term_months: 360,
    };
    let appraisal = Appraisal {
        appraisal_id: 22300,
        property_id: 22001,
        appraiser_id: 300010,
        appraised_value_cents: 42000000,
        status: AppraisalStatus::Approved,
        completed_date: Some(1702500000),
    };
    let tax = PropertyTax {
        property_id: 22001,
        year: 2024,
        assessed_value_cents: 38000000,
        tax_rate_x10000: 230,
        amount_due_cents: 874000,
    };

    let cfg = config::standard().with_fixed_int_encoding();

    let prop_bytes = encode_to_vec(&property, cfg).expect("encode property pipeline");
    let list_bytes = encode_to_vec(&listing, cfg).expect("encode listing pipeline");
    let mort_bytes = encode_to_vec(&mortgage, cfg).expect("encode mortgage pipeline");
    let appr_bytes = encode_to_vec(&appraisal, cfg).expect("encode appraisal pipeline");
    let tax_bytes = encode_to_vec(&tax, cfg).expect("encode tax pipeline");

    let (decoded_prop, prop_consumed): (Property, usize) =
        decode_owned_from_slice(&prop_bytes, cfg).expect("decode property pipeline");
    let (decoded_list, list_consumed): (Listing, usize) =
        decode_owned_from_slice(&list_bytes, cfg).expect("decode listing pipeline");
    let (decoded_mort, mort_consumed): (Mortgage, usize) =
        decode_owned_from_slice(&mort_bytes, cfg).expect("decode mortgage pipeline");
    let (decoded_appr, appr_consumed): (Appraisal, usize) =
        decode_owned_from_slice(&appr_bytes, cfg).expect("decode appraisal pipeline");
    let (decoded_tax, tax_consumed): (PropertyTax, usize) =
        decode_owned_from_slice(&tax_bytes, cfg).expect("decode tax pipeline");

    assert_eq!(property, decoded_prop);
    assert_eq!(listing, decoded_list);
    assert_eq!(mortgage, decoded_mort);
    assert_eq!(appraisal, decoded_appr);
    assert_eq!(tax, decoded_tax);

    assert_eq!(prop_consumed, prop_bytes.len());
    assert_eq!(list_consumed, list_bytes.len());
    assert_eq!(mort_consumed, mort_bytes.len());
    assert_eq!(appr_consumed, appr_bytes.len());
    assert_eq!(tax_consumed, tax_bytes.len());

    // Verify all consumed bytes are non-zero (data was actually serialized)
    assert!(prop_consumed > 0);
    assert!(list_consumed > 0);
    assert!(mort_consumed > 0);
    assert!(appr_consumed > 0);
    assert!(tax_consumed > 0);
}
