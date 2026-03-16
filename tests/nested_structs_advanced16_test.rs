//! Advanced nested structs test — cemetery and memorial park management theme, 22 tests.

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
// Domain types — Cemetery & Memorial Park Management
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

#[derive(Debug, PartialEq, Encode, Decode)]
enum PaymentFrequency {
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
    LumpSum,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MaintenanceTaskKind {
    Mowing,
    Trimming,
    Irrigation,
    FlowerPlacement,
    MonumentCleaning,
    SnowRemoval,
    TreeCare,
    PathRepair,
    FenceRepair,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DeedTransferReason {
    Sale,
    Inheritance,
    Donation,
    CourtOrder,
    ExchangeSwap,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum NicheSize {
    Single,
    Companion,
    Family,
    OssuaryVault,
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
struct ContactInfo {
    name: PersonName,
    phone: String,
    email: Option<String>,
    address: MailingAddress,
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
struct EquipmentItem {
    item_id: u32,
    name: String,
    serial_number: Option<String>,
    last_service_date: Option<DateRecord>,
    hours_used: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MaintenanceTask {
    task_id: u64,
    kind: MaintenanceTaskKind,
    section_coordinates: Vec<SectionCoordinate>,
    scheduled_date: DateRecord,
    completed_date: Option<DateRecord>,
    assigned_crew: Vec<String>,
    equipment_used: Vec<EquipmentItem>,
    notes: Option<String>,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MaintenanceSchedule {
    schedule_id: u64,
    year: u16,
    quarter: u8,
    tasks: Vec<MaintenanceTask>,
    total_budget_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColumbNicheAssignment {
    niche_id: u64,
    wall_name: String,
    row: u16,
    column: u16,
    size: NicheSize,
    urn: Option<UrnSpec>,
    decedent: Option<DecedentInfo>,
    face_plate_engraving: Vec<EngravingLine>,
    assignment_date: Option<DateRecord>,
    cost_cents: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Columbarium {
    name: String,
    gps: GpsCoordinate,
    total_niches: u32,
    niches: Vec<ColumbNicheAssignment>,
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

#[derive(Debug, PartialEq, Encode, Decode)]
struct CemeterySection {
    section_name: String,
    plot_count: u32,
    plots: Vec<Plot>,
    gps_boundary: Vec<GpsCoordinate>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CemeteryInventory {
    cemetery_name: String,
    sections: Vec<CemeterySection>,
    columbaria: Vec<Columbarium>,
    total_capacity: u64,
    current_occupancy: u64,
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

fn make_equipment(id: u32, name: &str) -> EquipmentItem {
    EquipmentItem {
        item_id: id,
        name: name.to_string(),
        serial_number: Some(format!("SN-{:05}", id)),
        last_service_date: Some(make_date(2025, 6, 1)),
        hours_used: 1200.5,
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
// Test 9: Grounds maintenance schedule with equipment
// ---------------------------------------------------------------------------
#[test]
fn test_maintenance_schedule_with_equipment() {
    let schedule = MaintenanceSchedule {
        schedule_id: 9001,
        year: 2025,
        quarter: 2,
        tasks: vec![
            MaintenanceTask {
                task_id: 9101,
                kind: MaintenanceTaskKind::Mowing,
                section_coordinates: vec![
                    make_coord("A", 1, 1),
                    make_coord("A", 2, 1),
                    make_coord("B", 1, 1),
                ],
                scheduled_date: make_date(2025, 4, 7),
                completed_date: Some(make_date(2025, 4, 7)),
                assigned_crew: vec!["Mike Torres".to_string(), "Jake Wilson".to_string()],
                equipment_used: vec![
                    make_equipment(101, "John Deere Z930M Zero-Turn"),
                    make_equipment(102, "Stihl FS 131 Trimmer"),
                ],
                notes: Some("Heavy spring growth, double pass required".to_string()),
                cost_cents: 45_000,
            },
            MaintenanceTask {
                task_id: 9102,
                kind: MaintenanceTaskKind::MonumentCleaning,
                section_coordinates: vec![make_coord("C", 10, 1)],
                scheduled_date: make_date(2025, 4, 14),
                completed_date: None,
                assigned_crew: vec!["Sarah Chen".to_string()],
                equipment_used: vec![
                    make_equipment(201, "Pressure Washer 2000 PSI"),
                    make_equipment(202, "D/2 Biological Solution Sprayer"),
                ],
                notes: None,
                cost_cents: 12_000,
            },
        ],
        total_budget_cents: 500_000,
    };
    roundtrip(&schedule, "maintenance schedule with equipment");
}

// ---------------------------------------------------------------------------
// Test 10: Columbarium with niche assignments
// ---------------------------------------------------------------------------
#[test]
fn test_columbarium_niche_assignments() {
    let columbarium = Columbarium {
        name: "Garden of Remembrance Columbarium".to_string(),
        gps: make_gps(39.7830, -89.6480),
        total_niches: 256,
        niches: vec![
            ColumbNicheAssignment {
                niche_id: 4001,
                wall_name: "East Wall".to_string(),
                row: 3,
                column: 7,
                size: NicheSize::Single,
                urn: Some(UrnSpec {
                    material: "Ceramic".to_string(),
                    model: "Peaceful Garden".to_string(),
                    capacity_cubic_inches: 180,
                    cost_cents: 22_000,
                }),
                decedent: Some(make_decedent("Alice", "Pemberton", 1932, 2024)),
                face_plate_engraving: vec![
                    make_engraving_line("Alice Mae Pemberton"),
                    make_engraving_line("1932 - 2024"),
                ],
                assignment_date: Some(make_date(2024, 8, 15)),
                cost_cents: 350_000,
            },
            ColumbNicheAssignment {
                niche_id: 4002,
                wall_name: "East Wall".to_string(),
                row: 3,
                column: 8,
                size: NicheSize::Companion,
                urn: None,
                decedent: None,
                face_plate_engraving: vec![],
                assignment_date: None,
                cost_cents: 500_000,
            },
            ColumbNicheAssignment {
                niche_id: 4003,
                wall_name: "South Wall".to_string(),
                row: 1,
                column: 2,
                size: NicheSize::Family,
                urn: Some(UrnSpec {
                    material: "Walnut Wood".to_string(),
                    model: "Heritage Urn".to_string(),
                    capacity_cubic_inches: 220,
                    cost_cents: 45_000,
                }),
                decedent: Some(make_decedent("George", "Fairbanks", 1928, 2023)),
                face_plate_engraving: vec![
                    make_engraving_line("The Fairbanks Family"),
                    make_engraving_line("George W. 1928-2023"),
                    make_engraving_line("Reserved: Edith M."),
                ],
                assignment_date: Some(make_date(2023, 11, 5)),
                cost_cents: 750_000,
            },
        ],
    };
    roundtrip(&columbarium, "columbarium with niche assignments");
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
// Test 14: Cemetery inventory with multiple sections
// ---------------------------------------------------------------------------
#[test]
fn test_cemetery_inventory_multiple_sections() {
    let inventory = CemeteryInventory {
        cemetery_name: "Oak Hill Memorial Park".to_string(),
        sections: vec![
            CemeterySection {
                section_name: "Heritage".to_string(),
                plot_count: 3,
                plots: vec![
                    make_plot(100, PlotType::SingleDepth, "Heritage", 1, 1),
                    make_plot(101, PlotType::DoubleDepth, "Heritage", 1, 2),
                    make_plot(102, PlotType::FamilyEstate, "Heritage", 2, 1),
                ],
                gps_boundary: vec![
                    make_gps(39.7810, -89.6510),
                    make_gps(39.7815, -89.6510),
                    make_gps(39.7815, -89.6500),
                    make_gps(39.7810, -89.6500),
                ],
            },
            CemeterySection {
                section_name: "Garden of Peace".to_string(),
                plot_count: 2,
                plots: vec![
                    make_plot(200, PlotType::Cremation, "Garden", 1, 1),
                    make_plot(201, PlotType::GreenBurial, "Garden", 1, 2),
                ],
                gps_boundary: vec![
                    make_gps(39.7820, -89.6510),
                    make_gps(39.7825, -89.6510),
                    make_gps(39.7825, -89.6500),
                    make_gps(39.7820, -89.6500),
                ],
            },
        ],
        columbaria: vec![Columbarium {
            name: "Sunset Columbarium".to_string(),
            gps: make_gps(39.7828, -89.6490),
            total_niches: 128,
            niches: vec![],
        }],
        total_capacity: 5000,
        current_occupancy: 3200,
    };
    roundtrip(&inventory, "cemetery inventory with sections");
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
// Test 17: Maintenance task with tree care and path repair
// ---------------------------------------------------------------------------
#[test]
fn test_maintenance_tasks_tree_care_and_path() {
    let tasks: Vec<MaintenanceTask> = vec![
        MaintenanceTask {
            task_id: 9200,
            kind: MaintenanceTaskKind::TreeCare,
            section_coordinates: vec![
                make_coord("Heritage", 1, 1),
                make_coord("Heritage", 2, 1),
                make_coord("Heritage", 3, 1),
            ],
            scheduled_date: make_date(2025, 10, 1),
            completed_date: None,
            assigned_crew: vec![
                "Tom Arbor".to_string(),
                "Jim Sawyer".to_string(),
                "Linda Canopy".to_string(),
            ],
            equipment_used: vec![
                make_equipment(301, "Vermeer BC1000XL Chipper"),
                make_equipment(302, "Husqvarna 572XP Chainsaw"),
                make_equipment(303, "Altec AT37G Bucket Truck"),
            ],
            notes: Some("Remove three dead oaks, trim 12 maples".to_string()),
            cost_cents: 350_000,
        },
        MaintenanceTask {
            task_id: 9201,
            kind: MaintenanceTaskKind::PathRepair,
            section_coordinates: vec![make_coord("Garden", 1, 1)],
            scheduled_date: make_date(2025, 10, 15),
            completed_date: None,
            assigned_crew: vec!["Carlos Pave".to_string()],
            equipment_used: vec![
                make_equipment(401, "Bobcat S650 Skid-Steer"),
                make_equipment(402, "Wacker Neuson VP1340 Plate Compactor"),
            ],
            notes: Some("Resurface 200 ft of walkway near South entrance".to_string()),
            cost_cents: 180_000,
        },
    ];
    for (i, task) in tasks.iter().enumerate() {
        roundtrip(task, &format!("maintenance task #{}", i));
    }
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

// ---------------------------------------------------------------------------
// Test 20: Snow removal and irrigation maintenance combo
// ---------------------------------------------------------------------------
#[test]
fn test_winter_maintenance_schedule() {
    let schedule = MaintenanceSchedule {
        schedule_id: 9300,
        year: 2025,
        quarter: 4,
        tasks: vec![
            MaintenanceTask {
                task_id: 9301,
                kind: MaintenanceTaskKind::SnowRemoval,
                section_coordinates: vec![
                    make_coord("Main", 0, 0),
                    make_coord("Heritage", 0, 0),
                    make_coord("Garden", 0, 0),
                    make_coord("Founders Row", 0, 0),
                ],
                scheduled_date: make_date(2025, 12, 15),
                completed_date: None,
                assigned_crew: vec!["Dan Frost".to_string(), "Kevin Plow".to_string()],
                equipment_used: vec![
                    make_equipment(501, "Western Wideout Snowplow"),
                    make_equipment(502, "SnowEx SP-7550 Salt Spreader"),
                    make_equipment(503, "Honda HS928 Snowblower"),
                ],
                notes: Some("Priority: main roads and chapel access first".to_string()),
                cost_cents: 85_000,
            },
            MaintenanceTask {
                task_id: 9302,
                kind: MaintenanceTaskKind::Irrigation,
                section_coordinates: vec![make_coord("Garden", 1, 1), make_coord("Garden", 2, 1)],
                scheduled_date: make_date(2025, 11, 1),
                completed_date: Some(make_date(2025, 11, 1)),
                assigned_crew: vec!["Phil Sprinkler".to_string()],
                equipment_used: vec![make_equipment(601, "Rain Bird ESP-TM2 Controller")],
                notes: Some("Winterize irrigation: drain lines, insulate backflow".to_string()),
                cost_cents: 22_000,
            },
            MaintenanceTask {
                task_id: 9303,
                kind: MaintenanceTaskKind::FenceRepair,
                section_coordinates: vec![make_coord("Perimeter", 0, 0)],
                scheduled_date: make_date(2025, 11, 15),
                completed_date: None,
                assigned_crew: vec!["Bill Ironwork".to_string(), "Sam Welder".to_string()],
                equipment_used: vec![
                    make_equipment(701, "Lincoln Electric MIG Welder"),
                    make_equipment(702, "DeWalt Angle Grinder"),
                ],
                notes: Some("Replace 40 ft of wrought-iron fencing, south side".to_string()),
                cost_cents: 150_000,
            },
        ],
        total_budget_cents: 600_000,
    };
    roundtrip(&schedule, "winter maintenance schedule");
}

// ---------------------------------------------------------------------------
// Test 21: Ossuary vault niche with family columbarium
// ---------------------------------------------------------------------------
#[test]
fn test_ossuary_vault_niche_family() {
    let columbarium = Columbarium {
        name: "Eternal Light Family Columbarium".to_string(),
        gps: make_gps(39.7835, -89.6475),
        total_niches: 64,
        niches: vec![
            ColumbNicheAssignment {
                niche_id: 4100,
                wall_name: "North Atrium".to_string(),
                row: 2,
                column: 4,
                size: NicheSize::OssuaryVault,
                urn: Some(UrnSpec {
                    material: "Polished Onyx".to_string(),
                    model: "Eternal Vault".to_string(),
                    capacity_cubic_inches: 400,
                    cost_cents: 95_000,
                }),
                decedent: Some(DecedentInfo {
                    name: make_person_full("Theodore", "Winston", "Blackwood", Some("Jr."), None),
                    date_of_birth: make_date(1942, 1, 5),
                    date_of_death: make_date(2024, 7, 19),
                    veteran_status: true,
                    branch_of_service: Some("US Navy".to_string()),
                    social_security_last4: Some("5678".to_string()),
                }),
                face_plate_engraving: vec![
                    EngravingLine {
                        text: "BLACKWOOD FAMILY VAULT".to_string(),
                        font_name: "Copperplate Gothic Bold".to_string(),
                        font_size_pt: 28,
                        is_gilded: true,
                    },
                    make_engraving_line("Theodore W. Jr. 1942-2024"),
                    make_engraving_line("USN - Vietnam Veteran"),
                    make_engraving_line("Reserved: Margaret R."),
                ],
                assignment_date: Some(make_date(2024, 7, 25)),
                cost_cents: 1_100_000,
            },
            ColumbNicheAssignment {
                niche_id: 4101,
                wall_name: "North Atrium".to_string(),
                row: 2,
                column: 5,
                size: NicheSize::Single,
                urn: None,
                decedent: None,
                face_plate_engraving: vec![],
                assignment_date: None,
                cost_cents: 400_000,
            },
        ],
    };
    roundtrip(&columbarium, "ossuary vault niche family columbarium");
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
