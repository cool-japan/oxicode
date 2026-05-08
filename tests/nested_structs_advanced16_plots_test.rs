//! Plots/interments-focused tests for nested_structs_advanced16 (split from nested_structs_advanced16_test.rs).

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
// Domain types — Cemetery & Memorial Park Management (plots/interments subset)
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
struct DecedentInfo {
    name: PersonName,
    date_of_birth: DateRecord,
    date_of_death: DateRecord,
    veteran_status: bool,
    branch_of_service: Option<String>,
    social_security_last4: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CasketSpec {
    material: String,
    model: String,
    color: String,
    interior_fabric: String,
    is_sealed: bool,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UrnSpec {
    material: String,
    model: String,
    capacity_cubic_inches: u16,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ContainerSpec {
    Casket(CasketSpec),
    Urn(UrnSpec),
    Shroud { material: String, cost_cents: u64 },
    None,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IntermentRecord {
    record_id: u64,
    decedent: DecedentInfo,
    plot: Plot,
    interment_date: DateRecord,
    service_type: ServiceType,
    container: ContainerSpec,
    monument: Option<MonumentSpec>,
    funeral_home: String,
    director_name: String,
    vault_liner_used: bool,
    opening_closing_fee_cents: u64,
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

fn make_decedent(first: &str, last: &str, birth_y: u16, death_y: u16) -> DecedentInfo {
    DecedentInfo {
        name: make_person(first, last),
        date_of_birth: make_date(birth_y, 3, 15),
        date_of_death: make_date(death_y, 11, 2),
        veteran_status: false,
        branch_of_service: None,
        social_security_last4: None,
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
// Test 1: Single plot with section coordinate
// ---------------------------------------------------------------------------
#[test]
fn test_single_plot_coordinate_roundtrip() {
    let plot = make_plot(1001, PlotType::SingleDepth, "A", 5, 12);
    roundtrip(&plot, "single plot with coordinate");
}

// ---------------------------------------------------------------------------
// Test 2: Double-depth occupied plot with deed reference
// ---------------------------------------------------------------------------
#[test]
fn test_double_depth_occupied_plot() {
    let mut plot = make_plot(2001, PlotType::DoubleDepth, "B", 3, 7);
    plot.is_occupied = true;
    plot.owner_deed_id = Some(9001);
    plot.purchase_price_cents = 500_000;
    roundtrip(&plot, "double-depth occupied plot");
}

// ---------------------------------------------------------------------------
// Test 3: Full interment record with casket
// ---------------------------------------------------------------------------
#[test]
fn test_interment_record_with_casket() {
    let record = IntermentRecord {
        record_id: 5001,
        decedent: DecedentInfo {
            name: make_person_full("Robert", "James", "Henderson", Some("Sr."), None),
            date_of_birth: make_date(1938, 7, 22),
            date_of_death: make_date(2025, 1, 15),
            veteran_status: true,
            branch_of_service: Some("US Army".to_string()),
            social_security_last4: Some("4321".to_string()),
        },
        plot: make_plot(3001, PlotType::SingleDepth, "C", 10, 3),
        interment_date: make_date(2025, 1, 20),
        service_type: ServiceType::Military,
        container: ContainerSpec::Casket(CasketSpec {
            material: "Bronze".to_string(),
            model: "Patriot".to_string(),
            color: "Flag Draped".to_string(),
            interior_fabric: "White Velvet".to_string(),
            is_sealed: true,
            cost_cents: 850_000,
        }),
        monument: Some(make_monument(7001, MonumentStyle::Upright)),
        funeral_home: "Evergreen Funeral Services".to_string(),
        director_name: "Thomas Blackwell".to_string(),
        vault_liner_used: true,
        opening_closing_fee_cents: 125_000,
        notes: vec![
            "Military honors requested".to_string(),
            "Flag presented to spouse".to_string(),
        ],
    };
    roundtrip(&record, "interment record with casket");
}

// ---------------------------------------------------------------------------
// Test 4: Interment record with urn (cremation)
// ---------------------------------------------------------------------------
#[test]
fn test_interment_record_cremation_urn() {
    let record = IntermentRecord {
        record_id: 5002,
        decedent: make_decedent("Margaret", "Whitfield", 1945, 2025),
        plot: make_plot(3002, PlotType::Cremation, "Garden", 1, 5),
        interment_date: make_date(2025, 3, 10),
        service_type: ServiceType::Memorial,
        container: ContainerSpec::Urn(UrnSpec {
            material: "Marble".to_string(),
            model: "Serenity Classic".to_string(),
            capacity_cubic_inches: 200,
            cost_cents: 35_000,
        }),
        monument: Some(MonumentSpec {
            monument_id: 7002,
            style: MonumentStyle::FlatMarker,
            material: "Bronze on Granite".to_string(),
            color: "Dark Bronze".to_string(),
            width_inches: 24,
            height_inches: 12,
            depth_inches: 4,
            weight_lbs: 150,
            engraving: EngravingSpec {
                front_lines: vec![
                    make_engraving_line("Margaret Whitfield"),
                    make_engraving_line("Beloved Mother & Grandmother"),
                    make_engraving_line("1945 - 2025"),
                ],
                back_lines: vec![],
                emblem_code: Some("CROSS-01".to_string()),
                portrait_etching: true,
                custom_artwork_desc: Some("Rose border pattern".to_string()),
            },
            foundation_required: false,
            vendor: "Legacy Bronze Works".to_string(),
            cost_cents: 280_000,
        }),
        funeral_home: "Peaceful Rest Chapel".to_string(),
        director_name: "Anne Sullivan".to_string(),
        vault_liner_used: false,
        opening_closing_fee_cents: 75_000,
        notes: vec!["Butterfly release after service".to_string()],
    };
    roundtrip(&record, "interment record cremation with urn");
}

// ---------------------------------------------------------------------------
// Test 5: Green burial with shroud container
// ---------------------------------------------------------------------------
#[test]
fn test_green_burial_shroud() {
    let record = IntermentRecord {
        record_id: 5003,
        decedent: make_decedent("Eleanor", "Thorne", 1950, 2025),
        plot: make_plot(3003, PlotType::GreenBurial, "Meadow", 2, 8),
        interment_date: make_date(2025, 5, 22),
        service_type: ServiceType::Green,
        container: ContainerSpec::Shroud {
            material: "Organic Cotton".to_string(),
            cost_cents: 15_000,
        },
        monument: None,
        funeral_home: "Natural Passages".to_string(),
        director_name: "David Greenway".to_string(),
        vault_liner_used: false,
        opening_closing_fee_cents: 90_000,
        notes: vec![
            "Native wildflower seeds planted".to_string(),
            "GPS marker only, no monument".to_string(),
        ],
    };
    roundtrip(&record, "green burial with shroud");
}

// ---------------------------------------------------------------------------
// Test 15: Family estate plot with multiple interments
// ---------------------------------------------------------------------------
#[test]
fn test_family_estate_multiple_interments() {
    let estate_plot = Plot {
        plot_id: 6000,
        plot_type: PlotType::FamilyEstate,
        coordinate: SectionCoordinate {
            section: "Founders Row".to_string(),
            row: 1,
            space: 1,
            tier: Some(0),
        },
        gps: make_gps(39.7800, -89.6520),
        dimensions_inches: (240, 240, 72),
        is_occupied: true,
        owner_deed_id: Some(9600),
        purchase_price_cents: 2_500_000,
        perpetual_care_included: true,
    };

    let interments: Vec<IntermentRecord> = vec![
        IntermentRecord {
            record_id: 5100,
            decedent: make_decedent("Charles", "Worthington", 1920, 1995),
            plot: estate_plot.clone(),
            interment_date: make_date(1995, 9, 10),
            service_type: ServiceType::Traditional,
            container: ContainerSpec::Casket(CasketSpec {
                material: "Mahogany".to_string(),
                model: "Presidential".to_string(),
                color: "Dark Cherry".to_string(),
                interior_fabric: "Champagne Satin".to_string(),
                is_sealed: true,
                cost_cents: 1_200_000,
            }),
            monument: None,
            funeral_home: "Worthington & Sons".to_string(),
            director_name: "James Worthington".to_string(),
            vault_liner_used: true,
            opening_closing_fee_cents: 100_000,
            notes: vec!["Family patriarch".to_string()],
        },
        IntermentRecord {
            record_id: 5101,
            decedent: DecedentInfo {
                name: make_person_full("Evelyn", "Rose", "Worthington", None, Some("Ashford")),
                date_of_birth: make_date(1925, 4, 18),
                date_of_death: make_date(2010, 12, 3),
                veteran_status: false,
                branch_of_service: None,
                social_security_last4: None,
            },
            plot: estate_plot,
            interment_date: make_date(2010, 12, 8),
            service_type: ServiceType::Traditional,
            container: ContainerSpec::Casket(CasketSpec {
                material: "Copper".to_string(),
                model: "Serenity".to_string(),
                color: "Rose Gold".to_string(),
                interior_fabric: "Pink Crepe".to_string(),
                is_sealed: true,
                cost_cents: 950_000,
            }),
            monument: None,
            funeral_home: "Worthington & Sons".to_string(),
            director_name: "James Worthington Jr.".to_string(),
            vault_liner_used: true,
            opening_closing_fee_cents: 110_000,
            notes: vec!["Interred beside husband Charles".to_string()],
        },
    ];

    for (i, rec) in interments.iter().enumerate() {
        roundtrip(rec, &format!("family estate interment #{}", i));
    }
}

// ---------------------------------------------------------------------------
// Test 22: Direct burial with no monument and empty notes
// ---------------------------------------------------------------------------
#[test]
fn test_direct_burial_minimal() {
    let record = IntermentRecord {
        record_id: 5200,
        decedent: DecedentInfo {
            name: PersonName {
                first: "John".to_string(),
                middle: None,
                last: "Doe".to_string(),
                suffix: None,
                maiden: None,
            },
            date_of_birth: make_date(1960, 1, 1),
            date_of_death: make_date(2025, 6, 30),
            veteran_status: false,
            branch_of_service: None,
            social_security_last4: None,
        },
        plot: Plot {
            plot_id: 3100,
            plot_type: PlotType::SingleDepth,
            coordinate: SectionCoordinate {
                section: "F".to_string(),
                row: 20,
                space: 15,
                tier: None,
            },
            gps: make_gps(39.7850, -89.6460),
            dimensions_inches: (48, 96, 72),
            is_occupied: true,
            owner_deed_id: Some(9800),
            purchase_price_cents: 180_000,
            perpetual_care_included: false,
        },
        interment_date: make_date(2025, 7, 2),
        service_type: ServiceType::DirectBurial,
        container: ContainerSpec::None,
        monument: None,
        funeral_home: "County Services".to_string(),
        director_name: "Administrative Office".to_string(),
        vault_liner_used: false,
        opening_closing_fee_cents: 80_000,
        notes: vec![],
    };
    roundtrip(&record, "direct burial minimal record");
}
