//! Maintenance/columbarium-focused tests for nested_structs_advanced16 (split from nested_structs_advanced16_test.rs).

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
// Domain types — Cemetery & Memorial Park Management (maintenance/columbarium subset)
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
struct DecedentInfo {
    name: PersonName,
    date_of_birth: DateRecord,
    date_of_death: DateRecord,
    veteran_status: bool,
    branch_of_service: Option<String>,
    social_security_last4: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UrnSpec {
    material: String,
    model: String,
    capacity_cubic_inches: u16,
    cost_cents: u64,
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
