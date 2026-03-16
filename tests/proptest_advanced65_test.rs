//! Advanced property-based tests for OxiCode binary serialization.
//!
//! Domain: astrophysics / telescope observations / astronomical catalogs.
//!
//! All 22 proptest functions are contained inside a single proptest! block.

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

/// Right ascension and declination encoded in milliarcseconds (integer coords).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SkyCoord {
    ra_mas: i64,
    dec_mas: i64,
}

/// Enumeration of astronomical object classifications.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ObjectType {
    Star,
    Galaxy,
    Quasar,
    Nebula,
    Cluster,
    Pulsar,
    BlackHole,
    Unknown,
}

/// A catalogued celestial object with positional and photometric metadata.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CelestialObject {
    object_id: u64,
    coord: SkyCoord,
    apparent_mag_x100: i32,
    redshift_x1e6: i32,
    object_type: ObjectType,
}

/// Photometric filter band used during an observation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ObsFilter {
    U,
    B,
    V,
    R,
    I,
    J,
    H,
    K,
}

/// A single telescope observation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ObservationRecord {
    obs_id: u64,
    object_id: u64,
    telescope_id: u32,
    exposure_ms: u32,
    filter: ObsFilter,
    flux_njy: u64,
}

/// Spectral line classification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LineType {
    Emission,
    Absorption,
    Broad,
    Narrow,
}

/// A spectral emission or absorption feature.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectralLine {
    wavelength_pm: u32,
    flux_density: u32,
    line_type: LineType,
}

/// Morphological parameters of a galaxy (Sérsic profile fit).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GalaxyMorphology {
    galaxy_id: u64,
    sersic_index_x100: u32,
    half_light_radius_arcsec_x100: u32,
    axis_ratio_x1000: u32,
    pa_deg_x10: u32,
}

// ---------------------------------------------------------------------------
// Helper constructors for enum variants from small integers
// ---------------------------------------------------------------------------

fn make_object_type(n: u8) -> ObjectType {
    match n % 8 {
        0 => ObjectType::Star,
        1 => ObjectType::Galaxy,
        2 => ObjectType::Quasar,
        3 => ObjectType::Nebula,
        4 => ObjectType::Cluster,
        5 => ObjectType::Pulsar,
        6 => ObjectType::BlackHole,
        _ => ObjectType::Unknown,
    }
}

fn make_obs_filter(n: u8) -> ObsFilter {
    match n % 8 {
        0 => ObsFilter::U,
        1 => ObsFilter::B,
        2 => ObsFilter::V,
        3 => ObsFilter::R,
        4 => ObsFilter::I,
        5 => ObsFilter::J,
        6 => ObsFilter::H,
        _ => ObsFilter::K,
    }
}

fn make_line_type(n: u8) -> LineType {
    match n % 4 {
        0 => LineType::Emission,
        1 => LineType::Absorption,
        2 => LineType::Broad,
        _ => LineType::Narrow,
    }
}

// ---------------------------------------------------------------------------
// Single proptest! block with exactly 22 #[test] functions
// ---------------------------------------------------------------------------

proptest! {
    // 1. SkyCoord roundtrip
    #[test]
    fn prop_sky_coord_roundtrip(ra_mas: i64, dec_mas: i64) {
        let coord = SkyCoord { ra_mas, dec_mas };
        let encoded = encode_to_vec(&coord).expect("encode SkyCoord failed");
        let (decoded, consumed): (SkyCoord, usize) =
            decode_from_slice(&encoded).expect("decode SkyCoord failed");
        prop_assert_eq!(coord, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 2. ObjectType roundtrip (all 8 variants)
    #[test]
    fn prop_object_type_roundtrip(n in 0u8..8u8) {
        let ot = make_object_type(n);
        let encoded = encode_to_vec(&ot).expect("encode ObjectType failed");
        let (decoded, consumed): (ObjectType, usize) =
            decode_from_slice(&encoded).expect("decode ObjectType failed");
        prop_assert_eq!(ot, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 3. CelestialObject roundtrip
    #[test]
    fn prop_celestial_object_roundtrip(
        object_id: u64,
        ra_mas: i64,
        dec_mas: i64,
        apparent_mag_x100: i32,
        redshift_x1e6: i32,
        ot_n in 0u8..8u8,
    ) {
        let obj = CelestialObject {
            object_id,
            coord: SkyCoord { ra_mas, dec_mas },
            apparent_mag_x100,
            redshift_x1e6,
            object_type: make_object_type(ot_n),
        };
        let encoded = encode_to_vec(&obj).expect("encode CelestialObject failed");
        let (decoded, consumed): (CelestialObject, usize) =
            decode_from_slice(&encoded).expect("decode CelestialObject failed");
        prop_assert_eq!(obj, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 4. ObsFilter roundtrip (all 8 bands)
    #[test]
    fn prop_obs_filter_roundtrip(n in 0u8..8u8) {
        let filter = make_obs_filter(n);
        let encoded = encode_to_vec(&filter).expect("encode ObsFilter failed");
        let (decoded, consumed): (ObsFilter, usize) =
            decode_from_slice(&encoded).expect("decode ObsFilter failed");
        prop_assert_eq!(filter, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 5. ObservationRecord roundtrip
    #[test]
    fn prop_observation_record_roundtrip(
        obs_id: u64,
        object_id: u64,
        telescope_id: u32,
        exposure_ms: u32,
        filter_n in 0u8..8u8,
        flux_njy: u64,
    ) {
        let rec = ObservationRecord {
            obs_id,
            object_id,
            telescope_id,
            exposure_ms,
            filter: make_obs_filter(filter_n),
            flux_njy,
        };
        let encoded = encode_to_vec(&rec).expect("encode ObservationRecord failed");
        let (decoded, consumed): (ObservationRecord, usize) =
            decode_from_slice(&encoded).expect("decode ObservationRecord failed");
        prop_assert_eq!(rec, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 6. LineType roundtrip (all 4 variants)
    #[test]
    fn prop_line_type_roundtrip(n in 0u8..4u8) {
        let lt = make_line_type(n);
        let encoded = encode_to_vec(&lt).expect("encode LineType failed");
        let (decoded, consumed): (LineType, usize) =
            decode_from_slice(&encoded).expect("decode LineType failed");
        prop_assert_eq!(lt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 7. SpectralLine roundtrip
    #[test]
    fn prop_spectral_line_roundtrip(
        wavelength_pm: u32,
        flux_density: u32,
        lt_n in 0u8..4u8,
    ) {
        let line = SpectralLine {
            wavelength_pm,
            flux_density,
            line_type: make_line_type(lt_n),
        };
        let encoded = encode_to_vec(&line).expect("encode SpectralLine failed");
        let (decoded, consumed): (SpectralLine, usize) =
            decode_from_slice(&encoded).expect("decode SpectralLine failed");
        prop_assert_eq!(line, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 8. GalaxyMorphology roundtrip
    #[test]
    fn prop_galaxy_morphology_roundtrip(
        galaxy_id: u64,
        sersic_index_x100: u32,
        half_light_radius_arcsec_x100: u32,
        axis_ratio_x1000: u32,
        pa_deg_x10: u32,
    ) {
        let morph = GalaxyMorphology {
            galaxy_id,
            sersic_index_x100,
            half_light_radius_arcsec_x100,
            axis_ratio_x1000,
            pa_deg_x10,
        };
        let encoded = encode_to_vec(&morph).expect("encode GalaxyMorphology failed");
        let (decoded, consumed): (GalaxyMorphology, usize) =
            decode_from_slice(&encoded).expect("decode GalaxyMorphology failed");
        prop_assert_eq!(morph, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 9. Deterministic encoding: same SkyCoord always produces identical bytes
    #[test]
    fn prop_sky_coord_deterministic(ra_mas: i64, dec_mas: i64) {
        let coord = SkyCoord { ra_mas, dec_mas };
        let enc1 = encode_to_vec(&coord).expect("encode SkyCoord det 1 failed");
        let enc2 = encode_to_vec(&coord).expect("encode SkyCoord det 2 failed");
        prop_assert_eq!(enc1, enc2);
    }

    // 10. Deterministic encoding: same CelestialObject always produces identical bytes
    #[test]
    fn prop_celestial_object_deterministic(
        object_id: u64,
        ra_mas: i64,
        dec_mas: i64,
        apparent_mag_x100: i32,
        redshift_x1e6: i32,
        ot_n in 0u8..8u8,
    ) {
        let obj = CelestialObject {
            object_id,
            coord: SkyCoord { ra_mas, dec_mas },
            apparent_mag_x100,
            redshift_x1e6,
            object_type: make_object_type(ot_n),
        };
        let enc1 = encode_to_vec(&obj).expect("encode CelestialObject det 1 failed");
        let enc2 = encode_to_vec(&obj).expect("encode CelestialObject det 2 failed");
        prop_assert_eq!(enc1, enc2);
    }

    // 11. Consumed bytes == encoded length for ObservationRecord
    #[test]
    fn prop_observation_record_consumed_eq_len(
        obs_id: u64,
        object_id: u64,
        telescope_id: u32,
        exposure_ms: u32,
        filter_n in 0u8..8u8,
        flux_njy: u64,
    ) {
        let rec = ObservationRecord {
            obs_id,
            object_id,
            telescope_id,
            exposure_ms,
            filter: make_obs_filter(filter_n),
            flux_njy,
        };
        let encoded = encode_to_vec(&rec).expect("encode ObservationRecord consumed failed");
        let (_, consumed): (ObservationRecord, usize) =
            decode_from_slice(&encoded).expect("decode ObservationRecord consumed failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    // 12. Consumed bytes == encoded length for GalaxyMorphology
    #[test]
    fn prop_galaxy_morphology_consumed_eq_len(
        galaxy_id: u64,
        sersic_index_x100: u32,
        half_light_radius_arcsec_x100: u32,
        axis_ratio_x1000: u32,
        pa_deg_x10: u32,
    ) {
        let morph = GalaxyMorphology {
            galaxy_id,
            sersic_index_x100,
            half_light_radius_arcsec_x100,
            axis_ratio_x1000,
            pa_deg_x10,
        };
        let encoded = encode_to_vec(&morph).expect("encode GalaxyMorphology consumed failed");
        let (_, consumed): (GalaxyMorphology, usize) =
            decode_from_slice(&encoded).expect("decode GalaxyMorphology consumed failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    // 13. Vec<SkyCoord> (0..10 elements) roundtrip
    #[test]
    fn prop_vec_sky_coord_roundtrip(
        coords in prop::collection::vec(
            (any::<i64>(), any::<i64>()).prop_map(|(ra_mas, dec_mas)| SkyCoord { ra_mas, dec_mas }),
            0..10,
        )
    ) {
        let encoded = encode_to_vec(&coords).expect("encode Vec<SkyCoord> failed");
        let (decoded, consumed): (Vec<SkyCoord>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<SkyCoord> failed");
        prop_assert_eq!(coords, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 14. Vec<ObservationRecord> (0..10 elements) roundtrip
    #[test]
    fn prop_vec_observation_record_roundtrip(
        recs in prop::collection::vec(
            (any::<u64>(), any::<u64>(), any::<u32>(), any::<u32>(), 0u8..8u8, any::<u64>())
                .prop_map(|(obs_id, object_id, telescope_id, exposure_ms, fn_, flux_njy)| {
                    ObservationRecord {
                        obs_id,
                        object_id,
                        telescope_id,
                        exposure_ms,
                        filter: make_obs_filter(fn_),
                        flux_njy,
                    }
                }),
            0..10,
        )
    ) {
        let encoded = encode_to_vec(&recs).expect("encode Vec<ObservationRecord> failed");
        let (decoded, consumed): (Vec<ObservationRecord>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<ObservationRecord> failed");
        prop_assert_eq!(recs, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 15. Vec<SpectralLine> (0..10 elements) roundtrip
    #[test]
    fn prop_vec_spectral_line_roundtrip(
        lines in prop::collection::vec(
            (any::<u32>(), any::<u32>(), 0u8..4u8).prop_map(|(wp, fd, ln)| SpectralLine {
                wavelength_pm: wp,
                flux_density: fd,
                line_type: make_line_type(ln),
            }),
            0..10,
        )
    ) {
        let encoded = encode_to_vec(&lines).expect("encode Vec<SpectralLine> failed");
        let (decoded, consumed): (Vec<SpectralLine>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<SpectralLine> failed");
        prop_assert_eq!(lines, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 16. Option<SkyCoord> roundtrip
    #[test]
    fn prop_option_sky_coord_roundtrip(ra_mas: i64, dec_mas: i64, present: bool) {
        let opt: Option<SkyCoord> = if present {
            Some(SkyCoord { ra_mas, dec_mas })
        } else {
            None
        };
        let encoded = encode_to_vec(&opt).expect("encode Option<SkyCoord> failed");
        let (decoded, consumed): (Option<SkyCoord>, usize) =
            decode_from_slice(&encoded).expect("decode Option<SkyCoord> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 17. Option<CelestialObject> roundtrip
    #[test]
    fn prop_option_celestial_object_roundtrip(
        object_id: u64,
        ra_mas: i64,
        dec_mas: i64,
        apparent_mag_x100: i32,
        redshift_x1e6: i32,
        ot_n in 0u8..8u8,
        present: bool,
    ) {
        let opt: Option<CelestialObject> = if present {
            Some(CelestialObject {
                object_id,
                coord: SkyCoord { ra_mas, dec_mas },
                apparent_mag_x100,
                redshift_x1e6,
                object_type: make_object_type(ot_n),
            })
        } else {
            None
        };
        let encoded = encode_to_vec(&opt).expect("encode Option<CelestialObject> failed");
        let (decoded, consumed): (Option<CelestialObject>, usize) =
            decode_from_slice(&encoded).expect("decode Option<CelestialObject> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 18. i64 primitive roundtrip (right-ascension / declination raw values)
    #[test]
    fn prop_i64_astro_roundtrip(value: i64) {
        let encoded = encode_to_vec(&value).expect("encode i64 astro failed");
        let (decoded, consumed): (i64, usize) =
            decode_from_slice(&encoded).expect("decode i64 astro failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 19. u64 primitive roundtrip (object identifiers, flux values)
    #[test]
    fn prop_u64_astro_roundtrip(value: u64) {
        let encoded = encode_to_vec(&value).expect("encode u64 astro failed");
        let (decoded, consumed): (u64, usize) =
            decode_from_slice(&encoded).expect("decode u64 astro failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 20. i32 primitive roundtrip (apparent magnitude x100, redshift x1e6)
    #[test]
    fn prop_i32_astro_roundtrip(value: i32) {
        let encoded = encode_to_vec(&value).expect("encode i32 astro failed");
        let (decoded, consumed): (i32, usize) =
            decode_from_slice(&encoded).expect("decode i32 astro failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 21. f32 primitive roundtrip (e.g. photometric calibration coefficients)
    #[test]
    fn prop_f32_astro_roundtrip(value in proptest::num::f32::NORMAL) {
        let encoded = encode_to_vec(&value).expect("encode f32 astro failed");
        let (decoded, consumed): (f32, usize) =
            decode_from_slice(&encoded).expect("decode f32 astro failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 22. f64 primitive roundtrip (e.g. precise astrometric coordinates in radians)
    #[test]
    fn prop_f64_astro_roundtrip(value in proptest::num::f64::NORMAL) {
        let encoded = encode_to_vec(&value).expect("encode f64 astro failed");
        let (decoded, consumed): (f64, usize) =
            decode_from_slice(&encoded).expect("decode f64 astro failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}
