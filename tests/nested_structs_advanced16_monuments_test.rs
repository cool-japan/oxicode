//! Monuments/genealogy-focused tests for nested_structs_advanced16 (split from nested_structs_advanced16_test.rs).

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

// ---------------------------------------------------------------------------
// Domain types — Cemetery & Memorial Park Management (monuments/genealogy subset)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum MonumentStyle {
    FlatMarker,
    BevelMarker,
    SlantMarker,
    Upright,
    Ledger,
    Bench,
    Obelisk,
    CustomSculpture,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ServiceType {
    Traditional,
    Graveside,
    Memorial,
    Celebration,
    Military,
    Green,
    DirectBurial,
    Scattering,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DateRecord {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PersonName {
    first: String,
    middle: Option<String>,
    last: String,
    suffix: Option<String>,
    maiden: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MailingAddress {
    line1: String,
    line2: Option<String>,
    city: String,
    state: String,
    zip: String,
    country: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ContactInfo {
    name: PersonName,
    phone: String,
    email: Option<String>,
    address: MailingAddress,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EngravingLine {
    text: String,
    font_name: String,
    font_size_pt: u8,
    is_gilded: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EngravingSpec {
    front_lines: Vec<EngravingLine>,
    back_lines: Vec<EngravingLine>,
    emblem_code: Option<String>,
    portrait_etching: bool,
    custom_artwork_desc: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MonumentSpec {
    monument_id: u64,
    style: MonumentStyle,
    material: String,
    color: String,
    width_inches: u16,
    height_inches: u16,
    depth_inches: u16,
    weight_lbs: u32,
    engraving: EngravingSpec,
    foundation_required: bool,
    vendor: String,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PerpetualCareFund {
    fund_id: u64,
    plot_id: u64,
    principal_cents: u64,
    interest_rate_bps: u16,
    annual_disbursement_cents: u64,
    inception_date: DateRecord,
    last_audit_date: Option<DateRecord>,
    trustee_name: String,
    is_fully_funded: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenealogyLink {
    person: PersonName,
    relationship: String,
    linked_plot_ids: Vec<u64>,
    birth_date: Option<DateRecord>,
    death_date: Option<DateRecord>,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenealogyRecord {
    primary_decedent_plot_id: u64,
    links: Vec<GenealogyLink>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DecedentInfo {
    name: PersonName,
    date_of_birth: DateRecord,
    date_of_death: DateRecord,
    veteran_status: bool,
    branch_of_service: Option<String>,
    social_security_last4: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ServiceVendor {
    name: String,
    role_description: String,
    contact: ContactInfo,
    fee_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MemorialServiceEvent {
    event_id: u64,
    service_type: ServiceType,
    decedent: DecedentInfo,
    date: DateRecord,
    start_time_minutes: u16,
    duration_minutes: u16,
    location_name: String,
    officiant_name: String,
    vendors: Vec<ServiceVendor>,
    attendee_estimate: u32,
    floral_arrangements: Vec<String>,
    music_selections: Vec<String>,
    readings: Vec<String>,
    special_requests: Vec<String>,
    total_cost_cents: u64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_date(year: u16, month: u8, day: u8) -> DateRecord {
    DateRecord { year, month, day }
}

fn make_person(first: &str, last: &str) -> PersonName {
    PersonName {
        first: first.to_string(),
        middle: None,
        last: last.to_string(),
        suffix: None,
        maiden: None,
    }
}

fn make_person_full(
    first: &str,
    middle: &str,
    last: &str,
    suffix: Option<&str>,
    maiden: Option<&str>,
) -> PersonName {
    PersonName {
        first: first.to_string(),
        middle: Some(middle.to_string()),
        last: last.to_string(),
        suffix: suffix.map(|s| s.to_string()),
        maiden: maiden.map(|s| s.to_string()),
    }
}

fn make_address(line1: &str, city: &str, state: &str, zip: &str) -> MailingAddress {
    MailingAddress {
        line1: line1.to_string(),
        line2: None,
        city: city.to_string(),
        state: state.to_string(),
        zip: zip.to_string(),
        country: "US".to_string(),
    }
}

fn make_contact(first: &str, last: &str, phone: &str) -> ContactInfo {
    ContactInfo {
        name: make_person(first, last),
        phone: phone.to_string(),
        email: Some(format!(
            "{}.{}@example.com",
            first.to_lowercase(),
            last.to_lowercase()
        )),
        address: make_address("100 Main St", "Springfield", "IL", "62701"),
    }
}

fn make_engraving_line(text: &str) -> EngravingLine {
    EngravingLine {
        text: text.to_string(),
        font_name: "Times New Roman".to_string(),
        font_size_pt: 24,
        is_gilded: false,
    }
}

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {}", ctx));
    let (decoded, consumed): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {}", ctx));
    assert_eq!(val, &decoded, "roundtrip mismatch for {}", ctx);
    assert_eq!(consumed, bytes.len(), "consumed mismatch for {}", ctx);
}

// ---------------------------------------------------------------------------
// Test 6: Monument specification with full engraving
// ---------------------------------------------------------------------------
#[test]
fn test_monument_full_engraving() {
    let monument = MonumentSpec {
        monument_id: 7010,
        style: MonumentStyle::Bench,
        material: "Barre Gray Granite".to_string(),
        color: "Medium Gray".to_string(),
        width_inches: 48,
        height_inches: 18,
        depth_inches: 16,
        weight_lbs: 2200,
        engraving: EngravingSpec {
            front_lines: vec![
                EngravingLine {
                    text: "THE JOHNSON FAMILY".to_string(),
                    font_name: "Helvetica Bold".to_string(),
                    font_size_pt: 36,
                    is_gilded: true,
                },
                EngravingLine {
                    text: "Together Forever".to_string(),
                    font_name: "Script MT".to_string(),
                    font_size_pt: 18,
                    is_gilded: false,
                },
            ],
            back_lines: vec![
                make_engraving_line("John 3:16"),
                make_engraving_line("For God so loved the world"),
            ],
            emblem_code: Some("PRAYING-HANDS-02".to_string()),
            portrait_etching: false,
            custom_artwork_desc: Some("Weeping willow tree etching".to_string()),
        },
        foundation_required: true,
        vendor: "Mastercraft Memorials".to_string(),
        cost_cents: 1_200_000,
    };
    roundtrip(&monument, "monument with full engraving");
}

// ---------------------------------------------------------------------------
// Test 7: Perpetual care fund record
// ---------------------------------------------------------------------------
#[test]
fn test_perpetual_care_fund() {
    let fund = PerpetualCareFund {
        fund_id: 8001,
        plot_id: 1001,
        principal_cents: 500_000,
        interest_rate_bps: 350,
        annual_disbursement_cents: 17_500,
        inception_date: make_date(2010, 4, 1),
        last_audit_date: Some(make_date(2024, 12, 31)),
        trustee_name: "First National Trust".to_string(),
        is_fully_funded: true,
    };
    roundtrip(&fund, "perpetual care fund");
}

// ---------------------------------------------------------------------------
// Test 8: Genealogy record with multiple family links
// ---------------------------------------------------------------------------
#[test]
fn test_genealogy_record_family_links() {
    let record = GenealogyRecord {
        primary_decedent_plot_id: 3001,
        links: vec![
            GenealogyLink {
                person: make_person("Martha", "Henderson"),
                relationship: "Spouse".to_string(),
                linked_plot_ids: vec![3010],
                birth_date: Some(make_date(1940, 5, 12)),
                death_date: None,
                notes: Some("Pre-need plot reserved adjacent".to_string()),
            },
            GenealogyLink {
                person: make_person("Robert Jr.", "Henderson"),
                relationship: "Son".to_string(),
                linked_plot_ids: vec![],
                birth_date: Some(make_date(1965, 8, 30)),
                death_date: None,
                notes: None,
            },
            GenealogyLink {
                person: make_person("Patricia", "Henderson-Morris"),
                relationship: "Daughter".to_string(),
                linked_plot_ids: vec![],
                birth_date: Some(make_date(1968, 2, 14)),
                death_date: None,
                notes: None,
            },
            GenealogyLink {
                person: make_person("Harold", "Henderson"),
                relationship: "Father".to_string(),
                linked_plot_ids: vec![1050, 1051],
                birth_date: Some(make_date(1910, 11, 3)),
                death_date: Some(make_date(1985, 6, 20)),
                notes: Some("Interred in family estate plot".to_string()),
            },
        ],
    };
    roundtrip(&record, "genealogy record with family links");
}

// ---------------------------------------------------------------------------
// Test 11: Memorial service event with vendors and details
// ---------------------------------------------------------------------------
#[test]
fn test_memorial_service_event_planning() {
    let event = MemorialServiceEvent {
        event_id: 6001,
        service_type: ServiceType::Celebration,
        decedent: DecedentInfo {
            name: make_person_full("Josephine", "Marie", "Delacroix", None, Some("Beaumont")),
            date_of_birth: make_date(1935, 12, 25),
            date_of_death: make_date(2025, 2, 14),
            veteran_status: false,
            branch_of_service: None,
            social_security_last4: Some("7890".to_string()),
        },
        date: make_date(2025, 2, 20),
        start_time_minutes: 600,
        duration_minutes: 120,
        location_name: "Rose Garden Chapel".to_string(),
        officiant_name: "Rev. Michael Fontaine".to_string(),
        vendors: vec![
            ServiceVendor {
                name: "Harmony Florists".to_string(),
                role_description: "Floral arrangements and altar decoration".to_string(),
                contact: make_contact("Lisa", "Park", "555-0101"),
                fee_cents: 180_000,
            },
            ServiceVendor {
                name: "Eternal Melodies Quartet".to_string(),
                role_description: "Live string quartet performance".to_string(),
                contact: make_contact("James", "Cho", "555-0102"),
                fee_cents: 120_000,
            },
            ServiceVendor {
                name: "Remembrance Catering".to_string(),
                role_description: "Reception luncheon for 150 guests".to_string(),
                contact: make_contact("Maria", "Gonzalez", "555-0103"),
                fee_cents: 450_000,
            },
        ],
        attendee_estimate: 150,
        floral_arrangements: vec![
            "Casket spray - white roses".to_string(),
            "Standing spray - lilies".to_string(),
            "Altar pieces x2".to_string(),
            "Memorial wreath".to_string(),
        ],
        music_selections: vec![
            "Ave Maria".to_string(),
            "Amazing Grace".to_string(),
            "Clair de Lune".to_string(),
        ],
        readings: vec![
            "Psalm 23".to_string(),
            "John 14:1-3".to_string(),
            "Personal eulogy by granddaughter".to_string(),
        ],
        special_requests: vec![
            "Dove release after service".to_string(),
            "Memorial slideshow projection".to_string(),
        ],
        total_cost_cents: 1_250_000,
    };
    roundtrip(&event, "memorial service event planning");
}
