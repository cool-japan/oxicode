#![cfg(feature = "versioning")]

//! Veterinary medicine / animal health — versioning feature tests.
//!
//! 22 #[test] functions covering animal patients, diagnoses, treatments,
//! and vaccines using encode_versioned_value / decode_versioned_value.

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
use oxicode::versioning::Version;
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum AnimalSpecies {
    Dog,
    Cat,
    Horse,
    Cow,
    Pig,
    Sheep,
    Rabbit,
    Bird,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum HealthStatus {
    Healthy,
    Sick,
    Recovering,
    Critical,
    Deceased,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AnimalPatientV1 {
    patient_id: u64,
    species: AnimalSpecies,
    age_months: u32,
    weight_kg_micro: u32,
    status: HealthStatus,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AnimalPatientV2 {
    patient_id: u64,
    species: AnimalSpecies,
    age_months: u32,
    weight_kg_micro: u32,
    status: HealthStatus,
    microchip_id: Option<u64>,
    breed: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VaccineRecord {
    vaccine_id: u32,
    name: String,
    administered_at: u64,
    booster_due_at: u64,
}

// ── Test 1: AnimalPatientV1 at version 1.0.0 roundtrip ───────────────────────
#[test]
fn test_animal_patient_v1_version_1_0_0_roundtrip() {
    let version = Version::new(1, 0, 0);
    let patient = AnimalPatientV1 {
        patient_id: 10001,
        species: AnimalSpecies::Dog,
        age_months: 36,
        weight_kg_micro: 25_000_000,
        status: HealthStatus::Healthy,
    };
    let bytes = encode_versioned_value(&patient, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode_versioned_value failed");
    assert_eq!(decoded, patient);
    assert_eq!(ver, version);
}

// ── Test 2: AnimalPatientV2 at version 2.0.0 roundtrip ───────────────────────
#[test]
fn test_animal_patient_v2_version_2_0_0_roundtrip() {
    let version = Version::new(2, 0, 0);
    let patient = AnimalPatientV2 {
        patient_id: 20002,
        species: AnimalSpecies::Cat,
        age_months: 24,
        weight_kg_micro: 4_500_000,
        status: HealthStatus::Healthy,
        microchip_id: Some(9_876_543_210_u64),
        breed: String::from("Siamese"),
    };
    let bytes = encode_versioned_value(&patient, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (AnimalPatientV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode_versioned_value failed");
    assert_eq!(decoded, patient);
    assert_eq!(ver, version);
}

// ── Test 3: each AnimalSpecies variant versioned ──────────────────────────────
#[test]
fn test_each_animal_species_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let species_list = [
        AnimalSpecies::Dog,
        AnimalSpecies::Cat,
        AnimalSpecies::Horse,
        AnimalSpecies::Cow,
        AnimalSpecies::Pig,
        AnimalSpecies::Sheep,
        AnimalSpecies::Rabbit,
        AnimalSpecies::Bird,
    ];
    for species in &species_list {
        let bytes = encode_versioned_value(species, version).expect("encode AnimalSpecies failed");
        let (decoded, ver, _consumed): (AnimalSpecies, Version, usize) =
            decode_versioned_value(&bytes).expect("decode AnimalSpecies failed");
        assert_eq!(&decoded, species);
        assert_eq!(ver, version);
    }
}

// ── Test 4: each HealthStatus variant versioned ───────────────────────────────
#[test]
fn test_each_health_status_variant_versioned() {
    let version = Version::new(1, 0, 0);
    let statuses = [
        HealthStatus::Healthy,
        HealthStatus::Sick,
        HealthStatus::Recovering,
        HealthStatus::Critical,
        HealthStatus::Deceased,
    ];
    for status in &statuses {
        let bytes = encode_versioned_value(status, version).expect("encode HealthStatus failed");
        let (decoded, ver, _consumed): (HealthStatus, Version, usize) =
            decode_versioned_value(&bytes).expect("decode HealthStatus failed");
        assert_eq!(&decoded, status);
        assert_eq!(ver, version);
    }
}

// ── Test 5: VaccineRecord versioned roundtrip ─────────────────────────────────
#[test]
fn test_vaccine_record_versioned_roundtrip() {
    let version = Version::new(1, 1, 0);
    let record = VaccineRecord {
        vaccine_id: 5001,
        name: String::from("Rabies"),
        administered_at: 1_700_000_000,
        booster_due_at: 1_700_000_000 + 365 * 86400,
    };
    let bytes = encode_versioned_value(&record, version).expect("encode VaccineRecord failed");
    let (decoded, ver, _consumed): (VaccineRecord, Version, usize) =
        decode_versioned_value(&bytes).expect("decode VaccineRecord failed");
    assert_eq!(decoded, record);
    assert_eq!(ver, version);
}

// ── Test 6: version triple major/minor/patch preserved ───────────────────────
#[test]
fn test_version_triple_major_minor_patch_preserved() {
    let version = Version::new(4, 9, 17);
    let patient = AnimalPatientV1 {
        patient_id: 77777,
        species: AnimalSpecies::Sheep,
        age_months: 18,
        weight_kg_micro: 55_000_000,
        status: HealthStatus::Healthy,
    };
    let bytes = encode_versioned_value(&patient, version).expect("encode failed");
    let (_decoded, ver, _consumed): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode failed");
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 9);
    assert_eq!(ver.patch, 17);
    assert_eq!(ver, version);
}

// ── Test 7: v1.0.0 < v2.0.0 ordering ────────────────────────────────────────
#[test]
fn test_v1_less_than_v2_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v1 < v2);
    assert!(v2 > v1);
    assert!(v2.is_breaking_change_from(&v1));
    assert!(!v1.is_compatible_with(&v2));
}

// ── Test 8: Vec<AnimalPatientV1> versioned roundtrip ─────────────────────────
#[test]
fn test_vec_animal_patient_v1_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let patients = vec![
        AnimalPatientV1 {
            patient_id: 100,
            species: AnimalSpecies::Dog,
            age_months: 48,
            weight_kg_micro: 30_000_000,
            status: HealthStatus::Healthy,
        },
        AnimalPatientV1 {
            patient_id: 101,
            species: AnimalSpecies::Cat,
            age_months: 12,
            weight_kg_micro: 3_800_000,
            status: HealthStatus::Sick,
        },
        AnimalPatientV1 {
            patient_id: 102,
            species: AnimalSpecies::Rabbit,
            age_months: 6,
            weight_kg_micro: 2_100_000,
            status: HealthStatus::Recovering,
        },
    ];
    let bytes =
        encode_versioned_value(&patients, version).expect("encode Vec<AnimalPatientV1> failed");
    let (decoded, ver, _consumed): (Vec<AnimalPatientV1>, Version, usize) =
        decode_versioned_value(&bytes).expect("decode Vec<AnimalPatientV1> failed");
    assert_eq!(decoded, patients);
    assert_eq!(ver, version);
    assert_eq!(decoded.len(), 3);
}

// ── Test 9: microchip upgrade v1 → v2 ────────────────────────────────────────
#[test]
fn test_microchip_upgrade_v1_to_v2() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    // Original record without microchip (V1 schema)
    let patient_v1 = AnimalPatientV1 {
        patient_id: 5000,
        species: AnimalSpecies::Dog,
        age_months: 60,
        weight_kg_micro: 20_000_000,
        status: HealthStatus::Healthy,
    };
    let bytes_v1 = encode_versioned_value(&patient_v1, v1).expect("encode v1 patient failed");
    let (decoded_v1, ver1, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes_v1).expect("decode v1 patient failed");
    assert_eq!(decoded_v1.microchip_id_absent(), true);
    assert_eq!(ver1, v1);

    // Upgraded record with microchip (V2 schema)
    let patient_v2 = AnimalPatientV2 {
        patient_id: 5000,
        species: AnimalSpecies::Dog,
        age_months: 60,
        weight_kg_micro: 20_000_000,
        status: HealthStatus::Healthy,
        microchip_id: Some(123_456_789_u64),
        breed: String::from("Labrador"),
    };
    let bytes_v2 = encode_versioned_value(&patient_v2, v2).expect("encode v2 patient failed");
    let (decoded_v2, ver2, _): (AnimalPatientV2, Version, usize) =
        decode_versioned_value(&bytes_v2).expect("decode v2 patient failed");
    assert_eq!(decoded_v2.microchip_id, Some(123_456_789_u64));
    assert_eq!(decoded_v2.patient_id, decoded_v1.patient_id);
    assert!(ver2 > ver1);
}

// ── Test 10: critical condition patient ──────────────────────────────────────
#[test]
fn test_critical_condition_patient_versioned() {
    let version = Version::new(1, 0, 0);
    let patient = AnimalPatientV1 {
        patient_id: 9_999,
        species: AnimalSpecies::Horse,
        age_months: 120,
        weight_kg_micro: 450_000_000,
        status: HealthStatus::Critical,
    };
    let bytes = encode_versioned_value(&patient, version).expect("encode critical patient failed");
    let (decoded, ver, _consumed): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode critical patient failed");
    assert_eq!(decoded.status, HealthStatus::Critical);
    assert_eq!(decoded.patient_id, 9_999);
    assert_eq!(ver, version);
}

// ── Test 11: recovering patient timeline ─────────────────────────────────────
#[test]
fn test_recovering_patient_timeline_versioned() {
    let version = Version::new(1, 0, 0);

    // Sick snapshot
    let sick = AnimalPatientV1 {
        patient_id: 3030,
        species: AnimalSpecies::Cat,
        age_months: 30,
        weight_kg_micro: 4_200_000,
        status: HealthStatus::Sick,
    };
    let bytes_sick = encode_versioned_value(&sick, version).expect("encode sick patient failed");
    let (decoded_sick, ver_sick, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes_sick).expect("decode sick patient failed");
    assert_eq!(decoded_sick.status, HealthStatus::Sick);
    assert_eq!(ver_sick, version);

    // Recovering snapshot
    let recovering = AnimalPatientV1 {
        patient_id: 3030,
        species: AnimalSpecies::Cat,
        age_months: 30,
        weight_kg_micro: 4_350_000, // weight partially recovered
        status: HealthStatus::Recovering,
    };
    let bytes_recovering =
        encode_versioned_value(&recovering, version).expect("encode recovering patient failed");
    let (decoded_recovering, ver_recovering, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes_recovering).expect("decode recovering patient failed");
    assert_eq!(decoded_recovering.status, HealthStatus::Recovering);
    assert_eq!(ver_recovering, version);
    // Same patient tracked across snapshots
    assert_eq!(decoded_sick.patient_id, decoded_recovering.patient_id);
    assert!(decoded_recovering.weight_kg_micro > decoded_sick.weight_kg_micro);
}

// ── Test 12: dog annual checkup ──────────────────────────────────────────────
#[test]
fn test_dog_annual_checkup_v2_versioned() {
    let version = Version::new(2, 0, 0);
    let patient = AnimalPatientV2 {
        patient_id: 12_000,
        species: AnimalSpecies::Dog,
        age_months: 48,
        weight_kg_micro: 22_300_000,
        status: HealthStatus::Healthy,
        microchip_id: Some(111_222_333_u64),
        breed: String::from("Golden Retriever"),
    };
    let bytes = encode_versioned_value(&patient, version).expect("encode annual checkup failed");
    let (decoded, ver, _consumed): (AnimalPatientV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode annual checkup failed");
    assert_eq!(decoded.species, AnimalSpecies::Dog);
    assert_eq!(decoded.breed, "Golden Retriever");
    assert_eq!(decoded.status, HealthStatus::Healthy);
    assert_eq!(ver.major, 2);
}

// ── Test 13: horse weight extremes ───────────────────────────────────────────
#[test]
fn test_horse_weight_extremes_versioned() {
    let version = Version::new(1, 0, 0);

    // Lightweight pony
    let pony = AnimalPatientV1 {
        patient_id: 6001,
        species: AnimalSpecies::Horse,
        age_months: 18,
        weight_kg_micro: 200_000_000, // 200 kg
        status: HealthStatus::Healthy,
    };
    // Heavyweight draft horse
    let draft = AnimalPatientV1 {
        patient_id: 6002,
        species: AnimalSpecies::Horse,
        age_months: 96,
        weight_kg_micro: 900_000_000, // 900 kg
        status: HealthStatus::Healthy,
    };

    let bytes_pony = encode_versioned_value(&pony, version).expect("encode pony failed");
    let bytes_draft = encode_versioned_value(&draft, version).expect("encode draft horse failed");

    let (dec_pony, ver_p, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes_pony).expect("decode pony failed");
    let (dec_draft, ver_d, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes_draft).expect("decode draft horse failed");

    assert!(dec_draft.weight_kg_micro > dec_pony.weight_kg_micro);
    assert_eq!(dec_pony.weight_kg_micro, 200_000_000);
    assert_eq!(dec_draft.weight_kg_micro, 900_000_000);
    assert_eq!(ver_p, version);
    assert_eq!(ver_d, version);
}

// ── Test 14: bird minimal weight ─────────────────────────────────────────────
#[test]
fn test_bird_minimal_weight_versioned() {
    let version = Version::new(1, 0, 0);
    // A small parrot ~90 grams = 90_000 micrograms-of-kg
    let bird = AnimalPatientV1 {
        patient_id: 7001,
        species: AnimalSpecies::Bird,
        age_months: 24,
        weight_kg_micro: 90_000,
        status: HealthStatus::Healthy,
    };
    let bytes = encode_versioned_value(&bird, version).expect("encode bird failed");
    let (decoded, ver, _consumed): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode bird failed");
    assert_eq!(decoded.species, AnimalSpecies::Bird);
    assert_eq!(decoded.weight_kg_micro, 90_000);
    assert_eq!(ver, version);
}

// ── Test 15: cat with no microchip (None) ────────────────────────────────────
#[test]
fn test_cat_with_no_microchip_none_v2() {
    let version = Version::new(2, 0, 0);
    let patient = AnimalPatientV2 {
        patient_id: 8001,
        species: AnimalSpecies::Cat,
        age_months: 7,
        weight_kg_micro: 2_700_000,
        status: HealthStatus::Healthy,
        microchip_id: None,
        breed: String::from("Domestic Shorthair"),
    };
    let bytes =
        encode_versioned_value(&patient, version).expect("encode cat without microchip failed");
    let (decoded, ver, _consumed): (AnimalPatientV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode cat without microchip failed");
    assert_eq!(decoded.microchip_id, None);
    assert_eq!(decoded.species, AnimalSpecies::Cat);
    assert_eq!(decoded.breed, "Domestic Shorthair");
    assert_eq!(ver, version);
}

// ── Test 16: cow with microchip (Some) ───────────────────────────────────────
#[test]
fn test_cow_with_microchip_some_v2() {
    let version = Version::new(2, 0, 0);
    let patient = AnimalPatientV2 {
        patient_id: 11_100,
        species: AnimalSpecies::Cow,
        age_months: 36,
        weight_kg_micro: 550_000_000,
        status: HealthStatus::Healthy,
        microchip_id: Some(999_888_777_u64),
        breed: String::from("Holstein"),
    };
    let bytes =
        encode_versioned_value(&patient, version).expect("encode cow with microchip failed");
    let (decoded, ver, _consumed): (AnimalPatientV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode cow with microchip failed");
    assert_eq!(decoded.microchip_id, Some(999_888_777_u64));
    assert_eq!(decoded.species, AnimalSpecies::Cow);
    assert_eq!(decoded.breed, "Holstein");
    assert_eq!(ver, version);
}

// ── Test 17: patient history chain (v1.0.0 through v1.2.0) ───────────────────
#[test]
fn test_patient_history_chain_v1_0_0_through_v1_2_0() {
    let v100 = Version::new(1, 0, 0);
    let v110 = Version::new(1, 1, 0);
    let v120 = Version::new(1, 2, 0);

    // Initial intake at v1.0.0
    let intake = AnimalPatientV1 {
        patient_id: 4444,
        species: AnimalSpecies::Pig,
        age_months: 6,
        weight_kg_micro: 80_000_000,
        status: HealthStatus::Sick,
    };
    // Follow-up at v1.1.0 (schema added age resolution)
    let followup = AnimalPatientV1 {
        patient_id: 4444,
        species: AnimalSpecies::Pig,
        age_months: 7,
        weight_kg_micro: 85_000_000,
        status: HealthStatus::Recovering,
    };
    // Discharge at v1.2.0
    let discharge = AnimalPatientV1 {
        patient_id: 4444,
        species: AnimalSpecies::Pig,
        age_months: 8,
        weight_kg_micro: 92_000_000,
        status: HealthStatus::Healthy,
    };

    let b_intake = encode_versioned_value(&intake, v100).expect("encode intake failed");
    let b_followup = encode_versioned_value(&followup, v110).expect("encode followup failed");
    let b_discharge = encode_versioned_value(&discharge, v120).expect("encode discharge failed");

    let (d_intake, ver_in, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&b_intake).expect("decode intake failed");
    let (d_followup, ver_fo, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&b_followup).expect("decode followup failed");
    let (d_discharge, ver_di, _): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&b_discharge).expect("decode discharge failed");

    assert_eq!(d_intake.status, HealthStatus::Sick);
    assert_eq!(d_followup.status, HealthStatus::Recovering);
    assert_eq!(d_discharge.status, HealthStatus::Healthy);
    // Version progression
    assert!(ver_fo > ver_in);
    assert!(ver_di > ver_fo);
    assert_eq!(ver_in, v100);
    assert_eq!(ver_fo, v110);
    assert_eq!(ver_di, v120);
    // Weight gain across chain
    assert!(d_followup.weight_kg_micro > d_intake.weight_kg_micro);
    assert!(d_discharge.weight_kg_micro > d_followup.weight_kg_micro);
}

// ── Test 18: breed diversity (5 breeds) ──────────────────────────────────────
#[test]
fn test_breed_diversity_5_breeds_v2_versioned() {
    let version = Version::new(2, 0, 0);
    let breeds = [
        ("German Shepherd", AnimalSpecies::Dog),
        ("Persian", AnimalSpecies::Cat),
        ("Thoroughbred", AnimalSpecies::Horse),
        ("Angus", AnimalSpecies::Cow),
        ("Merino", AnimalSpecies::Sheep),
    ];
    for (idx, (breed_name, species)) in breeds.iter().enumerate() {
        let patient = AnimalPatientV2 {
            patient_id: (2000 + idx) as u64,
            species: match species {
                AnimalSpecies::Dog => AnimalSpecies::Dog,
                AnimalSpecies::Cat => AnimalSpecies::Cat,
                AnimalSpecies::Horse => AnimalSpecies::Horse,
                AnimalSpecies::Cow => AnimalSpecies::Cow,
                AnimalSpecies::Sheep => AnimalSpecies::Sheep,
                _ => AnimalSpecies::Dog,
            },
            age_months: 24,
            weight_kg_micro: 10_000_000,
            status: HealthStatus::Healthy,
            microchip_id: None,
            breed: String::from(*breed_name),
        };
        let bytes = encode_versioned_value(&patient, version).expect("encode breed patient failed");
        let (decoded, ver, _consumed): (AnimalPatientV2, Version, usize) =
            decode_versioned_value(&bytes).expect("decode breed patient failed");
        assert_eq!(&decoded.breed, breed_name);
        assert_eq!(ver, version);
    }
}

// ── Test 19: vaccine booster schedule ────────────────────────────────────────
#[test]
fn test_vaccine_booster_schedule_versioned() {
    let version = Version::new(1, 0, 0);

    // Three vaccines with distinct booster windows
    let distemper = VaccineRecord {
        vaccine_id: 1,
        name: String::from("Distemper-Parvo"),
        administered_at: 1_700_000_000,
        booster_due_at: 1_700_000_000 + 365 * 86400,
    };
    let rabies = VaccineRecord {
        vaccine_id: 2,
        name: String::from("Rabies"),
        administered_at: 1_700_000_100,
        booster_due_at: 1_700_000_100 + 3 * 365 * 86400,
    };
    let bordetella = VaccineRecord {
        vaccine_id: 3,
        name: String::from("Bordetella"),
        administered_at: 1_700_000_200,
        booster_due_at: 1_700_000_200 + 180 * 86400,
    };

    let b_distemper = encode_versioned_value(&distemper, version).expect("encode distemper failed");
    let b_rabies = encode_versioned_value(&rabies, version).expect("encode rabies failed");
    let b_bordetella =
        encode_versioned_value(&bordetella, version).expect("encode bordetella failed");

    let (d_distemper, v1, _): (VaccineRecord, Version, usize) =
        decode_versioned_value(&b_distemper).expect("decode distemper failed");
    let (d_rabies, v2, _): (VaccineRecord, Version, usize) =
        decode_versioned_value(&b_rabies).expect("decode rabies failed");
    let (d_bordetella, v3, _): (VaccineRecord, Version, usize) =
        decode_versioned_value(&b_bordetella).expect("decode bordetella failed");

    assert_eq!(d_distemper.name, "Distemper-Parvo");
    assert_eq!(d_rabies.name, "Rabies");
    assert_eq!(d_bordetella.name, "Bordetella");
    // Booster ordering: bordetella soonest, then distemper, then rabies (3yr)
    assert!(d_bordetella.booster_due_at < d_distemper.booster_due_at);
    assert!(d_distemper.booster_due_at < d_rabies.booster_due_at);
    assert_eq!(v1, version);
    assert_eq!(v2, version);
    assert_eq!(v3, version);
}

// ── Test 20: zero weight edge case ───────────────────────────────────────────
#[test]
fn test_zero_weight_edge_case_versioned() {
    let version = Version::new(1, 0, 0);
    // Unusual edge: a newly born animal with no measurable weight yet
    let patient = AnimalPatientV1 {
        patient_id: 55_555,
        species: AnimalSpecies::Rabbit,
        age_months: 0,
        weight_kg_micro: 0,
        status: HealthStatus::Healthy,
    };
    let bytes =
        encode_versioned_value(&patient, version).expect("encode zero-weight patient failed");
    let (decoded, ver, _consumed): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode zero-weight patient failed");
    assert_eq!(decoded.weight_kg_micro, 0);
    assert_eq!(decoded.age_months, 0);
    assert_eq!(ver, version);
}

// ── Test 21: max age (months) ─────────────────────────────────────────────────
#[test]
fn test_max_age_months_versioned() {
    let version = Version::new(1, 0, 0);
    // u32::MAX months — extreme edge case for record-keeping systems
    let patient = AnimalPatientV1 {
        patient_id: 1,
        species: AnimalSpecies::Horse,
        age_months: u32::MAX,
        weight_kg_micro: 1,
        status: HealthStatus::Deceased,
    };
    let bytes = encode_versioned_value(&patient, version).expect("encode max-age patient failed");
    let (decoded, ver, _consumed): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode max-age patient failed");
    assert_eq!(decoded.age_months, u32::MAX);
    assert_eq!(decoded.status, HealthStatus::Deceased);
    assert_eq!(ver, version);
}

// ── Test 22: consumed bytes check ────────────────────────────────────────────
#[test]
fn test_consumed_bytes_check_versioned_patient() {
    let version = Version::new(1, 0, 0);
    let patient = AnimalPatientV1 {
        patient_id: 99_001,
        species: AnimalSpecies::Dog,
        age_months: 60,
        weight_kg_micro: 28_000_000,
        status: HealthStatus::Healthy,
    };
    let bytes =
        encode_versioned_value(&patient, version).expect("encode for consumed-bytes test failed");
    let total_len = bytes.len();
    let (_decoded, _ver, consumed): (AnimalPatientV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode for consumed-bytes test failed");
    // consumed now includes the full versioned envelope (header + payload).
    assert_eq!(
        consumed, total_len,
        "consumed must equal the full encoded length"
    );
    // Cross-check: plain encode_to_vec payload size + header equals consumed
    let plain_bytes = encode_to_vec(&patient).expect("encode_to_vec failed");
    let (_plain_decoded, plain_consumed): (AnimalPatientV1, usize) =
        decode_from_slice(&plain_bytes).expect("decode_from_slice failed");
    let versioned_header_size: usize = 11;
    assert_eq!(consumed, plain_consumed + versioned_header_size);
}

// ── Helper trait to make test 9 ergonomic without unwrap ─────────────────────
trait MicrochipAbsent {
    fn microchip_id_absent(&self) -> bool;
}

impl MicrochipAbsent for AnimalPatientV1 {
    fn microchip_id_absent(&self) -> bool {
        // V1 has no microchip field — absence is structural
        true
    }
}
