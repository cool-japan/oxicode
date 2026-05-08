//! Deeds/contracts-focused tests for nested_structs_advanced16 (split from nested_structs_advanced16_test.rs).

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
// Domain types — Cemetery & Memorial Park Management (deeds/contracts subset)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum PlotType {
    SingleDepth,
    DoubleDepth,
    Cremation,
    Mausoleum,
    ColumbNiche,
    GreenBurial,
    FamilyEstate,
}

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
enum PaymentFrequency {
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
    LumpSum,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DeedTransferReason {
    Sale,
    Inheritance,
    Donation,
    CourtOrder,
    ExchangeSwap,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct GpsCoordinate {
    latitude: f64,
    longitude: f64,
    elevation_ft: f32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct SectionCoordinate {
    section: String,
    row: u32,
    space: u32,
    tier: Option<u8>,
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

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Plot {
    plot_id: u64,
    plot_type: PlotType,
    coordinate: SectionCoordinate,
    gps: GpsCoordinate,
    dimensions_inches: (u32, u32, u32),
    is_occupied: bool,
    owner_deed_id: Option<u64>,
    purchase_price_cents: u64,
    perpetual_care_included: bool,
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
struct DeedHolder {
    holder: ContactInfo,
    ownership_pct_bps: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeedRecord {
    deed_id: u64,
    plot_id: u64,
    holders: Vec<DeedHolder>,
    issue_date: DateRecord,
    is_active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeedTransfer {
    transfer_id: u64,
    original_deed: DeedRecord,
    new_holders: Vec<DeedHolder>,
    reason: DeedTransferReason,
    transfer_date: DateRecord,
    transfer_fee_cents: u64,
    notarized: bool,
    legal_doc_ref: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PaymentSchedule {
    total_amount_cents: u64,
    down_payment_cents: u64,
    frequency: PaymentFrequency,
    installment_amount_cents: u64,
    num_installments: u16,
    interest_rate_bps: u16,
    start_date: DateRecord,
    payments_made: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PreNeedContractItem {
    description: String,
    item_cost_cents: u64,
    locked_price: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PreNeedContract {
    contract_id: u64,
    buyer: ContactInfo,
    beneficiary: PersonName,
    items: Vec<PreNeedContractItem>,
    payment_plan: PaymentSchedule,
    plot_reservation: Option<Plot>,
    monument_selection: Option<MonumentSpec>,
    contract_date: DateRecord,
    is_irrevocable: bool,
    notes: Vec<String>,
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

fn make_gps(lat: f64, lon: f64) -> GpsCoordinate {
    GpsCoordinate {
        latitude: lat,
        longitude: lon,
        elevation_ft: 450.0,
    }
}

fn make_coord(section: &str, row: u32, space: u32) -> SectionCoordinate {
    SectionCoordinate {
        section: section.to_string(),
        row,
        space,
        tier: None,
    }
}

fn make_plot(id: u64, plot_type: PlotType, section: &str, row: u32, space: u32) -> Plot {
    Plot {
        plot_id: id,
        plot_type,
        coordinate: make_coord(section, row, space),
        gps: make_gps(39.7817 + id as f64 * 0.0001, -89.6501 - id as f64 * 0.0001),
        dimensions_inches: (48, 96, 72),
        is_occupied: false,
        owner_deed_id: None,
        purchase_price_cents: 250_000,
        perpetual_care_included: true,
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

fn make_monument(id: u64, style: MonumentStyle) -> MonumentSpec {
    MonumentSpec {
        monument_id: id,
        style,
        material: "Georgia Gray Granite".to_string(),
        color: "Gray".to_string(),
        width_inches: 36,
        height_inches: 24,
        depth_inches: 6,
        weight_lbs: 800,
        engraving: EngravingSpec {
            front_lines: vec![
                make_engraving_line("In Loving Memory"),
                make_engraving_line("1940 - 2025"),
            ],
            back_lines: vec![],
            emblem_code: None,
            portrait_etching: false,
            custom_artwork_desc: None,
        },
        foundation_required: true,
        vendor: "Heritage Monuments".to_string(),
        cost_cents: 450_000,
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
// Test 12: Deed transfer record with multiple holders
// ---------------------------------------------------------------------------
#[test]
fn test_deed_transfer_with_holders() {
    let transfer = DeedTransfer {
        transfer_id: 11001,
        original_deed: DeedRecord {
            deed_id: 9501,
            plot_id: 2001,
            holders: vec![DeedHolder {
                holder: make_contact("Harold", "Pemberton", "555-0200"),
                ownership_pct_bps: 10000,
            }],
            issue_date: make_date(1990, 6, 15),
            is_active: false,
        },
        new_holders: vec![
            DeedHolder {
                holder: make_contact("Richard", "Pemberton", "555-0201"),
                ownership_pct_bps: 5000,
            },
            DeedHolder {
                holder: make_contact("Catherine", "Pemberton-Hall", "555-0202"),
                ownership_pct_bps: 5000,
            },
        ],
        reason: DeedTransferReason::Inheritance,
        transfer_date: make_date(2025, 3, 1),
        transfer_fee_cents: 15_000,
        notarized: true,
        legal_doc_ref: Some("Probate Case #2025-PC-4421".to_string()),
    };
    roundtrip(&transfer, "deed transfer with multiple holders");
}

// ---------------------------------------------------------------------------
// Test 13: Pre-need contract with full payment plan
// ---------------------------------------------------------------------------
#[test]
fn test_pre_need_contract_payment_plan() {
    let contract = PreNeedContract {
        contract_id: 12001,
        buyer: make_contact("William", "Stanton", "555-0300"),
        beneficiary: make_person_full("William", "George", "Stanton", Some("III"), None),
        items: vec![
            PreNeedContractItem {
                description: "Single-depth burial plot, Section D Row 14".to_string(),
                item_cost_cents: 300_000,
                locked_price: true,
            },
            PreNeedContractItem {
                description: "Upright granite monument with engraving".to_string(),
                item_cost_cents: 550_000,
                locked_price: true,
            },
            PreNeedContractItem {
                description: "Opening and closing services".to_string(),
                item_cost_cents: 125_000,
                locked_price: false,
            },
            PreNeedContractItem {
                description: "Concrete vault liner".to_string(),
                item_cost_cents: 95_000,
                locked_price: true,
            },
            PreNeedContractItem {
                description: "Perpetual care endowment".to_string(),
                item_cost_cents: 200_000,
                locked_price: true,
            },
        ],
        payment_plan: PaymentSchedule {
            total_amount_cents: 1_270_000,
            down_payment_cents: 270_000,
            frequency: PaymentFrequency::Monthly,
            installment_amount_cents: 20_000,
            num_installments: 50,
            interest_rate_bps: 0,
            start_date: make_date(2025, 4, 1),
            payments_made: 0,
        },
        plot_reservation: Some(make_plot(3050, PlotType::SingleDepth, "D", 14, 9)),
        monument_selection: Some(make_monument(7050, MonumentStyle::Upright)),
        contract_date: make_date(2025, 3, 15),
        is_irrevocable: false,
        notes: vec![
            "Buyer requested price lock guarantee".to_string(),
            "Annual review scheduled".to_string(),
        ],
    };
    roundtrip(&contract, "pre-need contract with payment plan");
}

// ---------------------------------------------------------------------------
// Test 16: Complex deed record with co-owners
// ---------------------------------------------------------------------------
#[test]
fn test_deed_record_co_owners() {
    let deed = DeedRecord {
        deed_id: 9700,
        plot_id: 4500,
        holders: vec![
            DeedHolder {
                holder: ContactInfo {
                    name: make_person("Franklin", "Morse"),
                    phone: "555-0401".to_string(),
                    email: Some("f.morse@example.com".to_string()),
                    address: make_address("42 Elm Street", "Oakville", "OH", "45858"),
                },
                ownership_pct_bps: 3334,
            },
            DeedHolder {
                holder: ContactInfo {
                    name: make_person("Diana", "Morse-Klein"),
                    phone: "555-0402".to_string(),
                    email: None,
                    address: make_address("88 Pine Lane", "Oakville", "OH", "45858"),
                },
                ownership_pct_bps: 3333,
            },
            DeedHolder {
                holder: ContactInfo {
                    name: make_person("Gerald", "Morse"),
                    phone: "555-0403".to_string(),
                    email: Some("gerald.m@example.com".to_string()),
                    address: MailingAddress {
                        line1: "1200 Oak Boulevard".to_string(),
                        line2: Some("Suite 305".to_string()),
                        city: "Columbus".to_string(),
                        state: "OH".to_string(),
                        zip: "43215".to_string(),
                        country: "US".to_string(),
                    },
                },
                ownership_pct_bps: 3333,
            },
        ],
        issue_date: make_date(2005, 8, 22),
        is_active: true,
    };
    roundtrip(&deed, "deed record with co-owners");
}

// ---------------------------------------------------------------------------
// Test 18: Pre-need contract with annual payments
// ---------------------------------------------------------------------------
#[test]
fn test_pre_need_annual_payment_contract() {
    let contract = PreNeedContract {
        contract_id: 12050,
        buyer: ContactInfo {
            name: make_person_full("Beatrice", "Ann", "Langford", None, None),
            phone: "555-0500".to_string(),
            email: Some("b.langford@example.com".to_string()),
            address: MailingAddress {
                line1: "750 Willow Creek Dr".to_string(),
                line2: Some("Apt 4B".to_string()),
                city: "Champaign".to_string(),
                state: "IL".to_string(),
                zip: "61820".to_string(),
                country: "US".to_string(),
            },
        },
        beneficiary: make_person_full("Beatrice", "Ann", "Langford", None, None),
        items: vec![
            PreNeedContractItem {
                description: "Cremation niche, Companion size, East Wall".to_string(),
                item_cost_cents: 500_000,
                locked_price: true,
            },
            PreNeedContractItem {
                description: "Companion urn, matching set".to_string(),
                item_cost_cents: 60_000,
                locked_price: true,
            },
            PreNeedContractItem {
                description: "Face plate engraving, two names".to_string(),
                item_cost_cents: 25_000,
                locked_price: false,
            },
        ],
        payment_plan: PaymentSchedule {
            total_amount_cents: 585_000,
            down_payment_cents: 85_000,
            frequency: PaymentFrequency::Annual,
            installment_amount_cents: 100_000,
            num_installments: 5,
            interest_rate_bps: 0,
            start_date: make_date(2025, 1, 15),
            payments_made: 1,
        },
        plot_reservation: None,
        monument_selection: None,
        contract_date: make_date(2025, 1, 15),
        is_irrevocable: true,
        notes: vec!["Companion plan with husband".to_string()],
    };
    roundtrip(&contract, "pre-need annual payment contract");
}

// ---------------------------------------------------------------------------
// Test 19: Deed transfer via court order
// ---------------------------------------------------------------------------
#[test]
fn test_deed_transfer_court_order() {
    let transfer = DeedTransfer {
        transfer_id: 11050,
        original_deed: DeedRecord {
            deed_id: 9550,
            plot_id: 2050,
            holders: vec![
                DeedHolder {
                    holder: make_contact("Vincent", "Carmichael", "555-0601"),
                    ownership_pct_bps: 5000,
                },
                DeedHolder {
                    holder: make_contact("Lorraine", "Carmichael", "555-0602"),
                    ownership_pct_bps: 5000,
                },
            ],
            issue_date: make_date(2000, 3, 20),
            is_active: false,
        },
        new_holders: vec![DeedHolder {
            holder: make_contact("Lorraine", "Carmichael", "555-0602"),
            ownership_pct_bps: 10000,
        }],
        reason: DeedTransferReason::CourtOrder,
        transfer_date: make_date(2025, 1, 10),
        transfer_fee_cents: 0,
        notarized: true,
        legal_doc_ref: Some("Family Court Order #FC-2024-8812, Div. Decree".to_string()),
    };
    roundtrip(&transfer, "deed transfer court order");
}
