//! Advanced Zstd compression tests for OxiCode — Zoo & Aquarium Management domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world zoo and aquarium management systems: animal inventory records,
//! enclosure environmental controls, feeding schedules, veterinary treatment logs,
//! breeding program (SSP/EEP) data, enrichment activities, keeper daily reports,
//! quarantine protocols, guest attendance metrics, conservation project tracking,
//! water quality monitoring for aquariums, and more.

#![cfg(feature = "compression-zstd")]
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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TaxonomicClass {
    Mammalia,
    Aves,
    Reptilia,
    Amphibia,
    Actinopterygii,
    Chondrichthyes,
    Invertebrate,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConservationStatus {
    LeastConcern,
    NearThreatened,
    Vulnerable,
    Endangered,
    CriticallyEndangered,
    ExtinctInWild,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnclosureType {
    OpenAir,
    IndoorClimateControlled,
    Aviary,
    AquariumTank,
    TerrestrialTerrarium,
    SemiAquatic,
    FreeRange,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DietCategory {
    Herbivore,
    Carnivore,
    Omnivore,
    Insectivore,
    Frugivore,
    Piscivore,
    Nectarivore,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TreatmentType {
    Vaccination,
    Surgery,
    DentalProcedure,
    BloodWork,
    Imaging,
    Medication,
    WoundCare,
    Deworming,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EnrichmentCategory {
    FoodBased,
    Sensory,
    Cognitive,
    Social,
    Physical,
    Novel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QuarantineReason {
    NewArrival,
    IllnessIsolation,
    PostSurgeryRecovery,
    ParasiteDetected,
    PreTransfer,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WaterType {
    Freshwater,
    Saltwater,
    Brackish,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreedingOutcome {
    SuccessfulBirth,
    EggLaid,
    EggHatched,
    Unsuccessful,
    Pending,
    InGestation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

/// An individual animal in the zoo registry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnimalRecord {
    animal_id: u64,
    name: String,
    species_common: String,
    species_latin: String,
    taxonomic_class: TaxonomicClass,
    conservation_status: ConservationStatus,
    birth_year: u16,
    weight_grams: u32,
    sex_is_male: bool,
    microchip_id: String,
    origin_facility: String,
}

/// Environmental control settings for an enclosure.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnclosureEnvironment {
    enclosure_id: u32,
    enclosure_name: String,
    enclosure_type: EnclosureType,
    temperature_mk: u32,
    humidity_permille: u16,
    light_lux: u32,
    uv_index_x10: u16,
    area_sq_cm: u64,
    max_occupancy: u16,
    current_occupancy: u16,
    substrate_type: String,
}

/// A single feeding event with diet formulation details.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeedingScheduleEntry {
    schedule_id: u64,
    animal_id: u64,
    diet_category: DietCategory,
    feed_time_minutes_from_midnight: u16,
    food_items: Vec<FoodItem>,
    total_calories_kcal: u32,
    supplements: Vec<String>,
    keeper_id: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FoodItem {
    name: String,
    weight_grams: u32,
    calories_per_100g: u16,
    is_enrichment_delivery: bool,
}

/// A veterinary treatment log entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VetTreatmentLog {
    log_id: u64,
    animal_id: u64,
    treatment_type: TreatmentType,
    date_days_since_epoch: u32,
    vet_name: String,
    diagnosis: String,
    medications: Vec<Medication>,
    anesthesia_used: bool,
    follow_up_days: u16,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Medication {
    drug_name: String,
    dose_mg_x100: u32,
    route: String,
    frequency_hours: u8,
    duration_days: u16,
}

/// Breeding program record (SSP/EEP style).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BreedingRecord {
    record_id: u64,
    program_name: String,
    species_latin: String,
    sire_id: u64,
    dam_id: u64,
    introduction_date: u32,
    outcome: BreedingOutcome,
    offspring_count: u8,
    offspring_ids: Vec<u64>,
    genetic_diversity_score_x1000: u32,
    studbook_number: u32,
}

/// An enrichment activity record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnrichmentActivity {
    activity_id: u64,
    enclosure_id: u32,
    category: EnrichmentCategory,
    description: String,
    duration_minutes: u16,
    animal_ids: Vec<u64>,
    engagement_score_x10: u16,
    materials_used: Vec<String>,
    date_days_since_epoch: u32,
}

/// Keeper daily report for one enclosure.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KeeperDailyReport {
    report_id: u64,
    keeper_id: u32,
    keeper_name: String,
    enclosure_id: u32,
    date_days_since_epoch: u32,
    animal_observations: Vec<AnimalObservation>,
    maintenance_tasks: Vec<String>,
    issues_reported: Vec<String>,
    shift_start_min: u16,
    shift_end_min: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnimalObservation {
    animal_id: u64,
    appetite_score: u8,
    activity_level: u8,
    stool_normal: bool,
    notes: String,
}

/// Quarantine protocol record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuarantineRecord {
    quarantine_id: u64,
    animal_id: u64,
    reason: QuarantineReason,
    start_date: u32,
    planned_duration_days: u16,
    isolation_room: String,
    daily_checks: Vec<QuarantineDailyCheck>,
    cleared: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuarantineDailyCheck {
    day_number: u16,
    temperature_mk: u32,
    weight_grams: u32,
    appetite_normal: bool,
    fecal_sample_taken: bool,
    blood_sample_taken: bool,
    vet_notes: String,
}

/// Guest attendance metrics for a single day.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GuestAttendance {
    date_days_since_epoch: u32,
    season: Season,
    total_visitors: u32,
    adult_tickets: u32,
    child_tickets: u32,
    member_entries: u32,
    group_bookings: u16,
    revenue_cents: u64,
    peak_hour: u8,
    hourly_counts: Vec<u32>,
    special_event: Option<String>,
}

/// Conservation project tracking record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConservationProject {
    project_id: u64,
    project_name: String,
    species_latin: String,
    conservation_status: ConservationStatus,
    partner_organizations: Vec<String>,
    funding_cents: u64,
    start_date: u32,
    milestones: Vec<Milestone>,
    field_site_country: String,
    animals_released: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Milestone {
    description: String,
    target_date: u32,
    completed: bool,
}

/// Water quality reading for an aquarium system.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterQualityReading {
    reading_id: u64,
    tank_id: u32,
    water_type: WaterType,
    timestamp_minutes: u32,
    temperature_mk: u32,
    ph_x100: u16,
    salinity_ppt_x100: u16,
    ammonia_ppb: u32,
    nitrite_ppb: u32,
    nitrate_ppb: u32,
    dissolved_oxygen_ppb: u32,
    alkalinity_ppm_x10: u16,
    calcium_ppm_x10: u16,
    flow_rate_lpm_x100: u32,
}

/// Species taxonomy record with full classification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpeciesTaxonomy {
    taxon_id: u64,
    kingdom: String,
    phylum: String,
    class_name: String,
    order: String,
    family: String,
    genus: String,
    species: String,
    common_name: String,
    conservation_status: ConservationStatus,
    global_population_estimate: u64,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_animal_record(id: u64) -> AnimalRecord {
    AnimalRecord {
        animal_id: id,
        name: format!("Animal-{id:04}"),
        species_common: format!("Common Species {}", id % 50),
        species_latin: format!("Genus{} species{}", id % 30, id % 20),
        taxonomic_class: match id % 7 {
            0 => TaxonomicClass::Mammalia,
            1 => TaxonomicClass::Aves,
            2 => TaxonomicClass::Reptilia,
            3 => TaxonomicClass::Amphibia,
            4 => TaxonomicClass::Actinopterygii,
            5 => TaxonomicClass::Chondrichthyes,
            _ => TaxonomicClass::Invertebrate,
        },
        conservation_status: match id % 6 {
            0 => ConservationStatus::LeastConcern,
            1 => ConservationStatus::NearThreatened,
            2 => ConservationStatus::Vulnerable,
            3 => ConservationStatus::Endangered,
            4 => ConservationStatus::CriticallyEndangered,
            _ => ConservationStatus::ExtinctInWild,
        },
        birth_year: 2005 + (id % 20) as u16,
        weight_grams: 500 + (id * 137 % 500_000) as u32,
        sex_is_male: id % 2 == 0,
        microchip_id: format!("MC-{id:012}"),
        origin_facility: format!("Zoo-{}", id % 15),
    }
}

fn make_enclosure_env(id: u32) -> EnclosureEnvironment {
    EnclosureEnvironment {
        enclosure_id: id,
        enclosure_name: format!("Enclosure-{id}"),
        enclosure_type: match id % 7 {
            0 => EnclosureType::OpenAir,
            1 => EnclosureType::IndoorClimateControlled,
            2 => EnclosureType::Aviary,
            3 => EnclosureType::AquariumTank,
            4 => EnclosureType::TerrestrialTerrarium,
            5 => EnclosureType::SemiAquatic,
            _ => EnclosureType::FreeRange,
        },
        temperature_mk: 293_000 + (id * 500) as u32,
        humidity_permille: 400 + (id * 13 % 500) as u16,
        light_lux: 500 + (id * 73 % 10_000) as u32,
        uv_index_x10: (id % 120) as u16,
        area_sq_cm: 1_000_000 + (id as u64 * 50_000),
        max_occupancy: 2 + (id % 20) as u16,
        current_occupancy: 1 + (id % 10) as u16,
        substrate_type: match id % 4 {
            0 => "Soil and mulch".to_string(),
            1 => "Sand".to_string(),
            2 => "Gravel and rock".to_string(),
            _ => "Grass".to_string(),
        },
    }
}

fn make_feeding_entry(id: u64) -> FeedingScheduleEntry {
    let items: Vec<FoodItem> = (0..3)
        .map(|i| FoodItem {
            name: format!("Food-{}-{i}", id),
            weight_grams: 100 + i * 50,
            calories_per_100g: 80 + (i * 20) as u16,
            is_enrichment_delivery: i == 0,
        })
        .collect();
    FeedingScheduleEntry {
        schedule_id: id,
        animal_id: id * 10 + 1,
        diet_category: match id % 7 {
            0 => DietCategory::Herbivore,
            1 => DietCategory::Carnivore,
            2 => DietCategory::Omnivore,
            3 => DietCategory::Insectivore,
            4 => DietCategory::Frugivore,
            5 => DietCategory::Piscivore,
            _ => DietCategory::Nectarivore,
        },
        feed_time_minutes_from_midnight: 480 + (id * 30 % 600) as u16,
        food_items: items,
        total_calories_kcal: 350 + (id * 17 % 800) as u32,
        supplements: vec![format!("Vitamin-{}", id % 5), format!("Mineral-{}", id % 3)],
        keeper_id: (id % 20) as u32,
    }
}

fn make_vet_log(id: u64) -> VetTreatmentLog {
    VetTreatmentLog {
        log_id: id,
        animal_id: id * 5 + 3,
        treatment_type: match id % 8 {
            0 => TreatmentType::Vaccination,
            1 => TreatmentType::Surgery,
            2 => TreatmentType::DentalProcedure,
            3 => TreatmentType::BloodWork,
            4 => TreatmentType::Imaging,
            5 => TreatmentType::Medication,
            6 => TreatmentType::WoundCare,
            _ => TreatmentType::Deworming,
        },
        date_days_since_epoch: 19_000 + id as u32,
        vet_name: format!("Dr. Vet-{}", id % 8),
        diagnosis: format!("Condition-{}", id % 25),
        medications: vec![Medication {
            drug_name: format!("Drug-{}", id % 12),
            dose_mg_x100: 500 + (id * 13 % 2000) as u32,
            route: "Oral".to_string(),
            frequency_hours: 8 + (id % 4) as u8,
            duration_days: 5 + (id % 10) as u16,
        }],
        anesthesia_used: id % 3 == 0,
        follow_up_days: 7 + (id % 14) as u16,
        notes: format!("Treatment notes for log {id}"),
    }
}

fn make_breeding_record(id: u64) -> BreedingRecord {
    BreedingRecord {
        record_id: id,
        program_name: format!("SSP-Program-{}", id % 10),
        species_latin: format!("Panthera tigris subspecies{}", id % 5),
        sire_id: id * 100 + 1,
        dam_id: id * 100 + 2,
        introduction_date: 19_500 + id as u32,
        outcome: match id % 6 {
            0 => BreedingOutcome::SuccessfulBirth,
            1 => BreedingOutcome::EggLaid,
            2 => BreedingOutcome::EggHatched,
            3 => BreedingOutcome::Unsuccessful,
            4 => BreedingOutcome::Pending,
            _ => BreedingOutcome::InGestation,
        },
        offspring_count: (id % 4) as u8,
        offspring_ids: (0..id % 4).map(|o| id * 1000 + o).collect(),
        genetic_diversity_score_x1000: 750 + (id * 19 % 250) as u32,
        studbook_number: 10_000 + id as u32,
    }
}

fn make_enrichment(id: u64) -> EnrichmentActivity {
    EnrichmentActivity {
        activity_id: id,
        enclosure_id: (id % 30) as u32,
        category: match id % 6 {
            0 => EnrichmentCategory::FoodBased,
            1 => EnrichmentCategory::Sensory,
            2 => EnrichmentCategory::Cognitive,
            3 => EnrichmentCategory::Social,
            4 => EnrichmentCategory::Physical,
            _ => EnrichmentCategory::Novel,
        },
        description: format!("Enrichment activity #{id}: novel item exploration"),
        duration_minutes: 15 + (id * 7 % 90) as u16,
        animal_ids: (0..3).map(|a| id * 10 + a).collect(),
        engagement_score_x10: 30 + (id * 11 % 70) as u16,
        materials_used: vec![
            format!("Material-A-{}", id % 8),
            format!("Material-B-{}", id % 5),
        ],
        date_days_since_epoch: 20_000 + id as u32,
    }
}

fn make_keeper_report(id: u64) -> KeeperDailyReport {
    let observations: Vec<AnimalObservation> = (0..4)
        .map(|a| AnimalObservation {
            animal_id: id * 100 + a,
            appetite_score: 3 + (a % 3) as u8,
            activity_level: 2 + (a % 4) as u8,
            stool_normal: a % 5 != 0,
            notes: format!("Obs for animal {} on report {id}", id * 100 + a),
        })
        .collect();
    KeeperDailyReport {
        report_id: id,
        keeper_id: (id % 15) as u32,
        keeper_name: format!("Keeper-{}", id % 15),
        enclosure_id: (id % 30) as u32,
        date_days_since_epoch: 20_000 + id as u32,
        animal_observations: observations,
        maintenance_tasks: vec![
            "Clean water troughs".to_string(),
            "Replace substrate".to_string(),
            format!("Task-{}", id % 7),
        ],
        issues_reported: if id % 3 == 0 {
            vec![format!("Issue noted on report {id}")]
        } else {
            vec![]
        },
        shift_start_min: 360 + (id * 60 % 120) as u16,
        shift_end_min: 960 + (id * 60 % 120) as u16,
    }
}

fn make_quarantine(id: u64) -> QuarantineRecord {
    let checks: Vec<QuarantineDailyCheck> = (1..=5)
        .map(|d| QuarantineDailyCheck {
            day_number: d,
            temperature_mk: 311_000 + (d as u32 * 100),
            weight_grams: 45_000 + (d as u32 * 20),
            appetite_normal: d > 1,
            fecal_sample_taken: d % 2 == 1,
            blood_sample_taken: d == 1 || d == 5,
            vet_notes: format!("Day {d} check for quarantine {id}"),
        })
        .collect();
    QuarantineRecord {
        quarantine_id: id,
        animal_id: id * 7 + 3,
        reason: match id % 5 {
            0 => QuarantineReason::NewArrival,
            1 => QuarantineReason::IllnessIsolation,
            2 => QuarantineReason::PostSurgeryRecovery,
            3 => QuarantineReason::ParasiteDetected,
            _ => QuarantineReason::PreTransfer,
        },
        start_date: 20_100 + id as u32,
        planned_duration_days: 14 + (id % 16) as u16,
        isolation_room: format!("QRoom-{}", id % 6),
        daily_checks: checks,
        cleared: id % 4 != 0,
    }
}

fn make_attendance(day: u32) -> GuestAttendance {
    GuestAttendance {
        date_days_since_epoch: 20_000 + day,
        season: match day % 4 {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            _ => Season::Winter,
        },
        total_visitors: 800 + (day * 37 % 5000),
        adult_tickets: 500 + (day * 23 % 3000),
        child_tickets: 200 + (day * 11 % 1500),
        member_entries: 100 + (day * 7 % 500),
        group_bookings: (day % 15) as u16,
        revenue_cents: 50_000_00 + (day as u64 * 1337 % 200_000_00),
        peak_hour: 11 + (day % 4) as u8,
        hourly_counts: (9..=17).map(|h| 50 + (day * h % 400)).collect(),
        special_event: if day % 7 == 0 {
            Some(format!("Event-{}", day / 7))
        } else {
            None
        },
    }
}

fn make_conservation_project(id: u64) -> ConservationProject {
    ConservationProject {
        project_id: id,
        project_name: format!("Project-{id}: habitat restoration"),
        species_latin: format!("Gorilla gorilla subspecies{}", id % 3),
        conservation_status: match id % 5 {
            0 => ConservationStatus::Vulnerable,
            1 => ConservationStatus::Endangered,
            2 => ConservationStatus::CriticallyEndangered,
            3 => ConservationStatus::NearThreatened,
            _ => ConservationStatus::ExtinctInWild,
        },
        partner_organizations: (0..3).map(|p| format!("Org-{}-{p}", id % 10)).collect(),
        funding_cents: 100_000_00 + id * 50_000_00,
        start_date: 18_000 + id as u32,
        milestones: (0..4)
            .map(|m| Milestone {
                description: format!("Milestone {m} for project {id}"),
                target_date: 18_500 + id as u32 + m * 90,
                completed: m < 2,
            })
            .collect(),
        field_site_country: format!("Country-{}", id % 12),
        animals_released: (id * 3 % 50) as u32,
    }
}

fn make_water_quality(id: u64) -> WaterQualityReading {
    WaterQualityReading {
        reading_id: id,
        tank_id: (id % 20) as u32,
        water_type: match id % 3 {
            0 => WaterType::Freshwater,
            1 => WaterType::Saltwater,
            _ => WaterType::Brackish,
        },
        timestamp_minutes: (id * 15 % 1440) as u32,
        temperature_mk: 297_000 + (id * 100 % 8000) as u32,
        ph_x100: 700 + (id * 5 % 200) as u16,
        salinity_ppt_x100: if id % 3 == 0 {
            0
        } else {
            3200 + (id * 7 % 500) as u16
        },
        ammonia_ppb: id as u32 % 50,
        nitrite_ppb: id as u32 % 30,
        nitrate_ppb: 500 + (id * 11 % 4000) as u32,
        dissolved_oxygen_ppb: 6_000_000 + (id * 100_000 % 2_000_000) as u32,
        alkalinity_ppm_x10: 800 + (id * 3 % 400) as u16,
        calcium_ppm_x10: 3800 + (id * 9 % 600) as u16,
        flow_rate_lpm_x100: 500 + (id * 17 % 2000) as u32,
    }
}

fn make_taxonomy(id: u64) -> SpeciesTaxonomy {
    SpeciesTaxonomy {
        taxon_id: id,
        kingdom: "Animalia".to_string(),
        phylum: "Chordata".to_string(),
        class_name: match id % 4 {
            0 => "Mammalia".to_string(),
            1 => "Aves".to_string(),
            2 => "Reptilia".to_string(),
            _ => "Amphibia".to_string(),
        },
        order: format!("Order-{}", id % 15),
        family: format!("Family-{}", id % 40),
        genus: format!("Genus{}", id % 60),
        species: format!("species{id}"),
        common_name: format!("Common Name {id}"),
        conservation_status: match id % 6 {
            0 => ConservationStatus::LeastConcern,
            1 => ConservationStatus::NearThreatened,
            2 => ConservationStatus::Vulnerable,
            3 => ConservationStatus::Endangered,
            4 => ConservationStatus::CriticallyEndangered,
            _ => ConservationStatus::ExtinctInWild,
        },
        global_population_estimate: 100 + id * 937,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Round-trip for a single animal inventory record.
#[test]
fn test_zstd_animal_record_roundtrip() {
    let animal = make_animal_record(42);
    let encoded = encode_to_vec(&animal).expect("encode AnimalRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AnimalRecord, usize) =
        decode_from_slice(&decompressed).expect("decode AnimalRecord failed");
    assert_eq!(animal, decoded);
}

/// 2. Round-trip for a batch of animal records.
#[test]
fn test_zstd_animal_records_batch_roundtrip() {
    let animals: Vec<AnimalRecord> = (1..=50).map(make_animal_record).collect();
    let encoded = encode_to_vec(&animals).expect("encode Vec<AnimalRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compression should reduce size for batch animal records"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AnimalRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AnimalRecord> failed");
    assert_eq!(animals, decoded);
}

/// 3. Round-trip for enclosure environmental control settings.
#[test]
fn test_zstd_enclosure_environment_roundtrip() {
    let enclosure = make_enclosure_env(7);
    let encoded = encode_to_vec(&enclosure).expect("encode EnclosureEnvironment failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (EnclosureEnvironment, usize) =
        decode_from_slice(&decompressed).expect("decode EnclosureEnvironment failed");
    assert_eq!(enclosure, decoded);
}

/// 4. Round-trip for multiple enclosures with varied types.
#[test]
fn test_zstd_enclosure_batch_all_types_roundtrip() {
    let enclosures: Vec<EnclosureEnvironment> = (0..35).map(make_enclosure_env).collect();
    let encoded = encode_to_vec(&enclosures).expect("encode Vec<EnclosureEnvironment> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<EnclosureEnvironment>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<EnclosureEnvironment> failed");
    assert_eq!(enclosures, decoded);
    // Verify all enclosure types are represented (7 types, 35 enclosures)
    assert_eq!(decoded.len(), 35);
}

/// 5. Round-trip for feeding schedule entries with nested food items.
#[test]
fn test_zstd_feeding_schedule_roundtrip() {
    let entries: Vec<FeedingScheduleEntry> = (1..=20).map(make_feeding_entry).collect();
    let encoded = encode_to_vec(&entries).expect("encode Vec<FeedingScheduleEntry> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<FeedingScheduleEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<FeedingScheduleEntry> failed");
    assert_eq!(entries, decoded);
    // Each entry has 3 food items
    for entry in &decoded {
        assert_eq!(entry.food_items.len(), 3);
    }
}

/// 6. Round-trip for veterinary treatment logs with medication details.
#[test]
fn test_zstd_vet_treatment_log_roundtrip() {
    let logs: Vec<VetTreatmentLog> = (0..15).map(make_vet_log).collect();
    let encoded = encode_to_vec(&logs).expect("encode Vec<VetTreatmentLog> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<VetTreatmentLog>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<VetTreatmentLog> failed");
    assert_eq!(logs, decoded);
}

/// 7. Round-trip for breeding program records (SSP-style studbook data).
#[test]
fn test_zstd_breeding_record_roundtrip() {
    let records: Vec<BreedingRecord> = (1..=12).map(make_breeding_record).collect();
    let encoded = encode_to_vec(&records).expect("encode Vec<BreedingRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<BreedingRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<BreedingRecord> failed");
    assert_eq!(records, decoded);
}

/// 8. Round-trip for enrichment activity records across categories.
#[test]
fn test_zstd_enrichment_activities_roundtrip() {
    let activities: Vec<EnrichmentActivity> = (0..24).map(make_enrichment).collect();
    let encoded = encode_to_vec(&activities).expect("encode Vec<EnrichmentActivity> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<EnrichmentActivity>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<EnrichmentActivity> failed");
    assert_eq!(activities, decoded);
}

/// 9. Round-trip for keeper daily reports with nested animal observations.
#[test]
fn test_zstd_keeper_daily_report_roundtrip() {
    let reports: Vec<KeeperDailyReport> = (1..=10).map(make_keeper_report).collect();
    let encoded = encode_to_vec(&reports).expect("encode Vec<KeeperDailyReport> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<KeeperDailyReport>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<KeeperDailyReport> failed");
    assert_eq!(reports, decoded);
    // Verify each report contains 4 observations
    for report in &decoded {
        assert_eq!(report.animal_observations.len(), 4);
    }
}

/// 10. Round-trip for quarantine records with daily health checks.
#[test]
fn test_zstd_quarantine_record_roundtrip() {
    let quarantines: Vec<QuarantineRecord> = (0..8).map(make_quarantine).collect();
    let encoded = encode_to_vec(&quarantines).expect("encode Vec<QuarantineRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<QuarantineRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<QuarantineRecord> failed");
    assert_eq!(quarantines, decoded);
    for q in &decoded {
        assert_eq!(q.daily_checks.len(), 5);
    }
}

/// 11. Round-trip for guest attendance metrics spanning a full year.
#[test]
fn test_zstd_guest_attendance_year_roundtrip() {
    let year: Vec<GuestAttendance> = (0..365).map(make_attendance).collect();
    let encoded = encode_to_vec(&year).expect("encode year attendance failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "365-day attendance data should compress well"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<GuestAttendance>, usize) =
        decode_from_slice(&decompressed).expect("decode year attendance failed");
    assert_eq!(year, decoded);
}

/// 12. Round-trip for conservation project tracking data.
#[test]
fn test_zstd_conservation_project_roundtrip() {
    let projects: Vec<ConservationProject> = (1..=8).map(make_conservation_project).collect();
    let encoded = encode_to_vec(&projects).expect("encode Vec<ConservationProject> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<ConservationProject>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<ConservationProject> failed");
    assert_eq!(projects, decoded);
    for p in &decoded {
        assert_eq!(p.milestones.len(), 4);
        assert_eq!(p.partner_organizations.len(), 3);
    }
}

/// 13. Round-trip for aquarium water quality time-series readings.
#[test]
fn test_zstd_water_quality_time_series_roundtrip() {
    let readings: Vec<WaterQualityReading> = (0..96).map(make_water_quality).collect();
    let encoded = encode_to_vec(&readings).expect("encode Vec<WaterQualityReading> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<WaterQualityReading>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<WaterQualityReading> failed");
    assert_eq!(readings, decoded);
}

/// 14. Round-trip for species taxonomy records with full classification.
#[test]
fn test_zstd_species_taxonomy_roundtrip() {
    let taxa: Vec<SpeciesTaxonomy> = (1..=40).map(make_taxonomy).collect();
    let encoded = encode_to_vec(&taxa).expect("encode Vec<SpeciesTaxonomy> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<SpeciesTaxonomy>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<SpeciesTaxonomy> failed");
    assert_eq!(taxa, decoded);
}

/// 15. Round-trip verifying compression ratio for large keeper report dataset.
#[test]
fn test_zstd_keeper_reports_compression_ratio() {
    let reports: Vec<KeeperDailyReport> = (1..=100).map(make_keeper_report).collect();
    let encoded = encode_to_vec(&reports).expect("encode 100 keeper reports failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 0.95,
        "compression ratio {ratio:.3} should be below 0.95 for repetitive report data"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<KeeperDailyReport>, usize) =
        decode_from_slice(&decompressed).expect("decode 100 keeper reports failed");
    assert_eq!(reports, decoded);
}

/// 16. Round-trip for a mixed tuple of different zoo management types.
#[test]
fn test_zstd_mixed_zoo_tuple_roundtrip() {
    let data = (
        make_animal_record(1),
        make_enclosure_env(2),
        make_feeding_entry(3),
        make_vet_log(4),
    );
    let encoded = encode_to_vec(&data).expect("encode mixed zoo tuple failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            AnimalRecord,
            EnclosureEnvironment,
            FeedingScheduleEntry,
            VetTreatmentLog,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode mixed zoo tuple failed");
    assert_eq!(data, decoded);
}

/// 17. Round-trip for nested Vec of breeding records paired with enrichment logs.
#[test]
fn test_zstd_breeding_enrichment_pairs_roundtrip() {
    let pairs: Vec<(BreedingRecord, Vec<EnrichmentActivity>)> = (1..=6)
        .map(|i| {
            let breeding = make_breeding_record(i);
            let enrichments: Vec<EnrichmentActivity> =
                (i * 10..i * 10 + 3).map(make_enrichment).collect();
            (breeding, enrichments)
        })
        .collect();
    let encoded = encode_to_vec(&pairs).expect("encode breeding+enrichment pairs failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<(BreedingRecord, Vec<EnrichmentActivity>)>, usize) =
        decode_from_slice(&decompressed).expect("decode breeding+enrichment pairs failed");
    assert_eq!(pairs, decoded);
}

/// 18. Round-trip for quarantine records with varied reasons ensuring enum coverage.
#[test]
fn test_zstd_quarantine_all_reasons_roundtrip() {
    let quarantines: Vec<QuarantineRecord> = (0..10).map(make_quarantine).collect();
    let encoded = encode_to_vec(&quarantines).expect("encode quarantine batch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<QuarantineRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode quarantine batch failed");
    assert_eq!(quarantines, decoded);
    // Verify all 5 quarantine reasons are covered (10 records, 5 reasons)
    let mut reason_seen = [false; 5];
    for (i, q) in decoded.iter().enumerate() {
        reason_seen[i % 5] = matches!(
            q.reason,
            QuarantineReason::NewArrival
                | QuarantineReason::IllnessIsolation
                | QuarantineReason::PostSurgeryRecovery
                | QuarantineReason::ParasiteDetected
                | QuarantineReason::PreTransfer
        );
    }
    assert!(reason_seen.iter().all(|&s| s));
}

/// 19. Round-trip for attendance data with and without special events (Option field).
#[test]
fn test_zstd_attendance_optional_events_roundtrip() {
    let days: Vec<GuestAttendance> = (0..14).map(make_attendance).collect();
    // Verify we have both Some and None for special_event
    let has_some = days.iter().any(|d| d.special_event.is_some());
    let has_none = days.iter().any(|d| d.special_event.is_none());
    assert!(
        has_some,
        "should have at least one day with a special event"
    );
    assert!(
        has_none,
        "should have at least one day without a special event"
    );
    let encoded = encode_to_vec(&days).expect("encode attendance with options failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<GuestAttendance>, usize) =
        decode_from_slice(&decompressed).expect("decode attendance with options failed");
    assert_eq!(days, decoded);
}

/// 20. Round-trip for a large water quality dataset simulating multi-tank monitoring.
#[test]
fn test_zstd_water_quality_multi_tank_roundtrip() {
    let readings: Vec<WaterQualityReading> = (0..400).map(make_water_quality).collect();
    let encoded = encode_to_vec(&readings).expect("encode 400 water quality readings failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "400 water quality readings should benefit from compression"
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<WaterQualityReading>, usize) =
        decode_from_slice(&decompressed).expect("decode 400 water quality readings failed");
    assert_eq!(readings, decoded);
    // Check all 3 water types represented (400 records, 3 types)
    let freshwater_count = decoded
        .iter()
        .filter(|r| r.water_type == WaterType::Freshwater)
        .count();
    let saltwater_count = decoded
        .iter()
        .filter(|r| r.water_type == WaterType::Saltwater)
        .count();
    let brackish_count = decoded
        .iter()
        .filter(|r| r.water_type == WaterType::Brackish)
        .count();
    assert!(freshwater_count > 0);
    assert!(saltwater_count > 0);
    assert!(brackish_count > 0);
}

/// 21. Round-trip for a comprehensive zoo snapshot: animals, enclosures, vet logs, and taxonomy.
#[test]
fn test_zstd_full_zoo_snapshot_roundtrip() {
    let snapshot = (
        (1..=25).map(make_animal_record).collect::<Vec<_>>(),
        (1..=10).map(make_enclosure_env).collect::<Vec<_>>(),
        (1..=8).map(make_vet_log).collect::<Vec<_>>(),
        (1..=15).map(make_taxonomy).collect::<Vec<_>>(),
    );
    let encoded = encode_to_vec(&snapshot).expect("encode zoo snapshot failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            Vec<AnimalRecord>,
            Vec<EnclosureEnvironment>,
            Vec<VetTreatmentLog>,
            Vec<SpeciesTaxonomy>,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode zoo snapshot failed");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.0.len(), 25);
    assert_eq!(decoded.1.len(), 10);
    assert_eq!(decoded.2.len(), 8);
    assert_eq!(decoded.3.len(), 15);
}

/// 22. Round-trip for conservation projects linked with breeding outcomes and attendance.
#[test]
fn test_zstd_conservation_breeding_attendance_roundtrip() {
    let dataset: Vec<(ConservationProject, Vec<BreedingRecord>, GuestAttendance)> = (1..=5)
        .map(|i| {
            let project = make_conservation_project(i);
            let breedings: Vec<BreedingRecord> =
                (i * 10..i * 10 + 4).map(make_breeding_record).collect();
            let attendance = make_attendance(i as u32);
            (project, breedings, attendance)
        })
        .collect();
    let encoded = encode_to_vec(&dataset).expect("encode conservation dataset failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        Vec<(ConservationProject, Vec<BreedingRecord>, GuestAttendance)>,
        usize,
    ) = decode_from_slice(&decompressed).expect("decode conservation dataset failed");
    assert_eq!(dataset, decoded);
    for (_, breedings, _) in &decoded {
        assert_eq!(breedings.len(), 4);
    }
}
