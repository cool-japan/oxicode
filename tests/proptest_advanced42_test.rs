//! Property-based tests (set 42) with an astronomy / astrophysics domain theme.
//!
//! All 22 test functions live inside `proptest! { }` macro blocks.
//! Tests cover roundtrip encoding, consumed == bytes.len(), deterministic
//! encoding, Vec of bodies, Option types, and all enum variants for
//! CelestialType and SpectralClass with arbitrary inputs.

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
use proptest::prelude::*;

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum CelestialType {
    Star,
    Planet,
    Moon,
    Asteroid,
    Comet,
    Galaxy,
    Nebula,
    BlackHole,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum SpectralClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct CelestialBody {
    id: u64,
    name: String,
    body_type: CelestialType,
    mass_kg: f64,
    radius_m: f64,
    distance_ly: f64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct StarSystem {
    primary_star_id: u64,
    bodies: Vec<CelestialBody>,
    spectral_class: SpectralClass,
    age_gyr: f32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Observation {
    body_id: u64,
    timestamp_unix: u64,
    ra_deg: f64,
    dec_deg: f64,
    magnitude: f32,
}

// ── Strategies ────────────────────────────────────────────────────────────────

fn arb_celestial_type() -> impl Strategy<Value = CelestialType> {
    prop_oneof![
        Just(CelestialType::Star),
        Just(CelestialType::Planet),
        Just(CelestialType::Moon),
        Just(CelestialType::Asteroid),
        Just(CelestialType::Comet),
        Just(CelestialType::Galaxy),
        Just(CelestialType::Nebula),
        Just(CelestialType::BlackHole),
    ]
}

fn arb_spectral_class() -> impl Strategy<Value = SpectralClass> {
    prop_oneof![
        Just(SpectralClass::O),
        Just(SpectralClass::B),
        Just(SpectralClass::A),
        Just(SpectralClass::F),
        Just(SpectralClass::G),
        Just(SpectralClass::K),
        Just(SpectralClass::M),
    ]
}

fn arb_celestial_body() -> impl Strategy<Value = CelestialBody> {
    (
        any::<u64>(),
        any::<String>(),
        arb_celestial_type(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
    )
        .prop_map(
            |(id, name, body_type, mass_kg, radius_m, distance_ly)| CelestialBody {
                id,
                name,
                body_type,
                mass_kg,
                radius_m,
                distance_ly,
            },
        )
}

fn arb_star_system() -> impl Strategy<Value = StarSystem> {
    (
        any::<u64>(),
        proptest::collection::vec(arb_celestial_body(), 0..8),
        arb_spectral_class(),
        any::<f32>(),
    )
        .prop_map(
            |(primary_star_id, bodies, spectral_class, age_gyr)| StarSystem {
                primary_star_id,
                bodies,
                spectral_class,
                age_gyr,
            },
        )
}

fn arb_observation() -> impl Strategy<Value = Observation> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f32>(),
    )
        .prop_map(
            |(body_id, timestamp_unix, ra_deg, dec_deg, magnitude)| Observation {
                body_id,
                timestamp_unix,
                ra_deg,
                dec_deg,
                magnitude,
            },
        )
}

// ── Test 1: CelestialBody roundtrip ──────────────────────────────────────────

proptest! {
    #[test]
    fn prop_celestial_body_roundtrip(body in arb_celestial_body()) {
        let encoded = encode_to_vec(&body).expect("encode CelestialBody failed");
        let (decoded, consumed): (CelestialBody, usize) =
            decode_from_slice(&encoded).expect("decode CelestialBody failed");
        prop_assert_eq!(&decoded, &body);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 2: StarSystem roundtrip ─────────────────────────────────────────────

proptest! {
    #[test]
    fn prop_star_system_roundtrip(system in arb_star_system()) {
        let encoded = encode_to_vec(&system).expect("encode StarSystem failed");
        let (decoded, consumed): (StarSystem, usize) =
            decode_from_slice(&encoded).expect("decode StarSystem failed");
        prop_assert_eq!(&decoded, &system);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 3: Observation roundtrip ────────────────────────────────────────────

proptest! {
    #[test]
    fn prop_observation_roundtrip(obs in arb_observation()) {
        let encoded = encode_to_vec(&obs).expect("encode Observation failed");
        let (decoded, consumed): (Observation, usize) =
            decode_from_slice(&encoded).expect("decode Observation failed");
        prop_assert_eq!(&decoded, &obs);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 4: CelestialType roundtrip (all variants via strategy) ──────────────

proptest! {
    #[test]
    fn prop_celestial_type_roundtrip(ct in arb_celestial_type()) {
        let encoded = encode_to_vec(&ct).expect("encode CelestialType failed");
        let (decoded, consumed): (CelestialType, usize) =
            decode_from_slice(&encoded).expect("decode CelestialType failed");
        prop_assert_eq!(&decoded, &ct);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 5: SpectralClass roundtrip (all variants via strategy) ──────────────

proptest! {
    #[test]
    fn prop_spectral_class_roundtrip(sc in arb_spectral_class()) {
        let encoded = encode_to_vec(&sc).expect("encode SpectralClass failed");
        let (decoded, consumed): (SpectralClass, usize) =
            decode_from_slice(&encoded).expect("decode SpectralClass failed");
        prop_assert_eq!(&decoded, &sc);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 6: consumed == bytes.len() for CelestialBody ────────────────────────

proptest! {
    #[test]
    fn prop_celestial_body_consumed_eq_len(body in arb_celestial_body()) {
        let encoded = encode_to_vec(&body).expect("encode CelestialBody (consumed) failed");
        let (_decoded, consumed): (CelestialBody, usize) =
            decode_from_slice(&encoded).expect("decode CelestialBody (consumed) failed");
        prop_assert_eq!(
            consumed,
            encoded.len(),
            "consumed bytes must equal encoded length for CelestialBody"
        );
    }
}

// ── Test 7: consumed == bytes.len() for StarSystem ───────────────────────────

proptest! {
    #[test]
    fn prop_star_system_consumed_eq_len(system in arb_star_system()) {
        let encoded = encode_to_vec(&system).expect("encode StarSystem (consumed) failed");
        let (_decoded, consumed): (StarSystem, usize) =
            decode_from_slice(&encoded).expect("decode StarSystem (consumed) failed");
        prop_assert_eq!(
            consumed,
            encoded.len(),
            "consumed bytes must equal encoded length for StarSystem"
        );
    }
}

// ── Test 8: Deterministic encoding for CelestialBody ─────────────────────────

proptest! {
    #[test]
    fn prop_celestial_body_deterministic(body in arb_celestial_body()) {
        let enc1 = encode_to_vec(&body).expect("encode CelestialBody (det1) failed");
        let enc2 = encode_to_vec(&body).expect("encode CelestialBody (det2) failed");
        prop_assert_eq!(
            &enc1,
            &enc2,
            "two encodes of identical CelestialBody must produce identical bytes"
        );
    }
}

// ── Test 9: Deterministic encoding for StarSystem ────────────────────────────

proptest! {
    #[test]
    fn prop_star_system_deterministic(system in arb_star_system()) {
        let enc1 = encode_to_vec(&system).expect("encode StarSystem (det1) failed");
        let enc2 = encode_to_vec(&system).expect("encode StarSystem (det2) failed");
        prop_assert_eq!(
            &enc1,
            &enc2,
            "two encodes of identical StarSystem must produce identical bytes"
        );
    }
}

// ── Test 10: Vec<CelestialBody> roundtrip ────────────────────────────────────

proptest! {
    #[test]
    fn prop_vec_celestial_bodies_roundtrip(
        bodies in proptest::collection::vec(arb_celestial_body(), 0..10)
    ) {
        let encoded = encode_to_vec(&bodies).expect("encode Vec<CelestialBody> failed");
        let (decoded, consumed): (Vec<CelestialBody>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<CelestialBody> failed");
        prop_assert_eq!(&decoded, &bodies);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 11: Vec<Observation> roundtrip ──────────────────────────────────────

proptest! {
    #[test]
    fn prop_vec_observations_roundtrip(
        observations in proptest::collection::vec(arb_observation(), 0..10)
    ) {
        let encoded = encode_to_vec(&observations).expect("encode Vec<Observation> failed");
        let (decoded, consumed): (Vec<Observation>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Observation> failed");
        prop_assert_eq!(&decoded, &observations);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 12: Option<CelestialBody> roundtrip ─────────────────────────────────

proptest! {
    #[test]
    fn prop_option_celestial_body_roundtrip(
        opt in proptest::option::of(arb_celestial_body())
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<CelestialBody> failed");
        let (decoded, consumed): (Option<CelestialBody>, usize) =
            decode_from_slice(&encoded).expect("decode Option<CelestialBody> failed");
        prop_assert_eq!(&decoded, &opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 13: Option<StarSystem> roundtrip ────────────────────────────────────

proptest! {
    #[test]
    fn prop_option_star_system_roundtrip(
        opt in proptest::option::of(arb_star_system())
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<StarSystem> failed");
        let (decoded, consumed): (Option<StarSystem>, usize) =
            decode_from_slice(&encoded).expect("decode Option<StarSystem> failed");
        prop_assert_eq!(&decoded, &opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 14: Option<Observation> roundtrip ───────────────────────────────────

proptest! {
    #[test]
    fn prop_option_observation_roundtrip(
        opt in proptest::option::of(arb_observation())
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Observation> failed");
        let (decoded, consumed): (Option<Observation>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Observation> failed");
        prop_assert_eq!(&decoded, &opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 15: CelestialType::Star with arbitrary CelestialBody fields ─────────

proptest! {
    #[test]
    fn prop_celestial_body_star_variant(
        id: u64,
        name: String,
        mass_kg: f64,
        radius_m: f64,
        distance_ly: f64,
    ) {
        let body = CelestialBody {
            id,
            name,
            body_type: CelestialType::Star,
            mass_kg,
            radius_m,
            distance_ly,
        };
        let encoded = encode_to_vec(&body).expect("encode Star body failed");
        let (decoded, consumed): (CelestialBody, usize) =
            decode_from_slice(&encoded).expect("decode Star body failed");
        prop_assert_eq!(&decoded.body_type, &CelestialType::Star);
        prop_assert_eq!(&decoded, &body);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 16: CelestialType::BlackHole with arbitrary CelestialBody fields ────

proptest! {
    #[test]
    fn prop_celestial_body_black_hole_variant(
        id: u64,
        name: String,
        mass_kg: f64,
        radius_m: f64,
        distance_ly: f64,
    ) {
        let body = CelestialBody {
            id,
            name,
            body_type: CelestialType::BlackHole,
            mass_kg,
            radius_m,
            distance_ly,
        };
        let encoded = encode_to_vec(&body).expect("encode BlackHole body failed");
        let (decoded, consumed): (CelestialBody, usize) =
            decode_from_slice(&encoded).expect("decode BlackHole body failed");
        prop_assert_eq!(&decoded.body_type, &CelestialType::BlackHole);
        prop_assert_eq!(&decoded, &body);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 17: SpectralClass::G (Sun-like) StarSystem roundtrip ────────────────

proptest! {
    #[test]
    fn prop_star_system_spectral_g_roundtrip(
        primary_star_id: u64,
        age_gyr: f32,
        bodies in proptest::collection::vec(arb_celestial_body(), 0..5),
    ) {
        let system = StarSystem {
            primary_star_id,
            bodies,
            spectral_class: SpectralClass::G,
            age_gyr,
        };
        let encoded = encode_to_vec(&system).expect("encode SpectralClass::G system failed");
        let (decoded, consumed): (StarSystem, usize) =
            decode_from_slice(&encoded).expect("decode SpectralClass::G system failed");
        prop_assert_eq!(&decoded.spectral_class, &SpectralClass::G);
        prop_assert_eq!(&decoded, &system);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 18: SpectralClass::O (hot blue) StarSystem roundtrip ────────────────

proptest! {
    #[test]
    fn prop_star_system_spectral_o_roundtrip(
        primary_star_id: u64,
        age_gyr: f32,
        bodies in proptest::collection::vec(arb_celestial_body(), 0..5),
    ) {
        let system = StarSystem {
            primary_star_id,
            bodies,
            spectral_class: SpectralClass::O,
            age_gyr,
        };
        let encoded = encode_to_vec(&system).expect("encode SpectralClass::O system failed");
        let (decoded, consumed): (StarSystem, usize) =
            decode_from_slice(&encoded).expect("decode SpectralClass::O system failed");
        prop_assert_eq!(&decoded.spectral_class, &SpectralClass::O);
        prop_assert_eq!(&decoded, &system);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 19: Observation with extreme coordinate values roundtrip ─────────────

proptest! {
    #[test]
    fn prop_observation_extreme_coords_roundtrip(
        body_id: u64,
        timestamp_unix: u64,
        ra_deg in (-360.0f64..=360.0f64),
        dec_deg in (-90.0f64..=90.0f64),
        magnitude: f32,
    ) {
        let obs = Observation {
            body_id,
            timestamp_unix,
            ra_deg,
            dec_deg,
            magnitude,
        };
        let encoded = encode_to_vec(&obs).expect("encode extreme-coord Observation failed");
        let (decoded, consumed): (Observation, usize) =
            decode_from_slice(&encoded).expect("decode extreme-coord Observation failed");
        prop_assert_eq!(&decoded, &obs);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 20: Vec<StarSystem> roundtrip (up to 4 systems) ─────────────────────

proptest! {
    #[test]
    fn prop_vec_star_systems_roundtrip(
        systems in proptest::collection::vec(arb_star_system(), 0..4)
    ) {
        let encoded = encode_to_vec(&systems).expect("encode Vec<StarSystem> failed");
        let (decoded, consumed): (Vec<StarSystem>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<StarSystem> failed");
        prop_assert_eq!(&decoded, &systems);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 21: CelestialBody with Galaxy / Nebula / Asteroid / Comet variants ──

proptest! {
    #[test]
    fn prop_celestial_body_diffuse_variants(
        id: u64,
        name: String,
        mass_kg: f64,
        radius_m: f64,
        distance_ly: f64,
        variant_idx in 0u8..4u8,
    ) {
        let body_type = match variant_idx {
            0 => CelestialType::Galaxy,
            1 => CelestialType::Nebula,
            2 => CelestialType::Asteroid,
            _ => CelestialType::Comet,
        };
        let body = CelestialBody {
            id,
            name,
            body_type,
            mass_kg,
            radius_m,
            distance_ly,
        };
        let encoded = encode_to_vec(&body).expect("encode diffuse-variant CelestialBody failed");
        let (decoded, consumed): (CelestialBody, usize) =
            decode_from_slice(&encoded).expect("decode diffuse-variant CelestialBody failed");
        prop_assert_eq!(&decoded, &body);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ── Test 22: Deterministic encoding for Observation ──────────────────────────

proptest! {
    #[test]
    fn prop_observation_deterministic(obs in arb_observation()) {
        let enc1 = encode_to_vec(&obs).expect("encode Observation (det1) failed");
        let enc2 = encode_to_vec(&obs).expect("encode Observation (det2) failed");
        prop_assert_eq!(
            &enc1,
            &enc2,
            "two encodes of identical Observation must produce identical bytes"
        );
    }
}
