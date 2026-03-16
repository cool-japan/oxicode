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

// ── Domain types: nuclear physics / particle detection / accelerator systems ──

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum ParticleType {
    Proton,
    Neutron,
    Electron,
    Positron,
    Muon,
    Pion,
    Kaon,
    Photon,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ParticleHit {
    pub detector_id: u32,
    pub channel: u16,
    pub adc_value: u32,
    pub timestamp_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct TrackSegment {
    pub track_id: u64,
    pub x_mm: i32,
    pub y_mm: i32,
    pub z_mm: i32,
    pub momentum_mev: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct DecayEvent {
    pub event_id: u64,
    pub decay_time_ns: u64,
    pub energy_mev: u32,
    pub particle_type: ParticleType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct BeamPulse {
    pub pulse_id: u64,
    pub intensity_ua: u32,
    pub energy_gev: u32,
    pub repetition_hz: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RadiationDose {
    pub detector_id: u32,
    pub dose_ugy: u32,
    pub dose_rate_ugy_h: u32,
    pub timestamp: u64,
}

// ── Strategy helpers ──────────────────────────────────────────────────────────

fn particle_type_strategy() -> impl Strategy<Value = ParticleType> {
    prop_oneof![
        Just(ParticleType::Proton),
        Just(ParticleType::Neutron),
        Just(ParticleType::Electron),
        Just(ParticleType::Positron),
        Just(ParticleType::Muon),
        Just(ParticleType::Pion),
        Just(ParticleType::Kaon),
        Just(ParticleType::Photon),
    ]
}

fn particle_hit_strategy() -> impl Strategy<Value = ParticleHit> {
    (any::<u32>(), any::<u16>(), any::<u32>(), any::<u64>()).prop_map(
        |(detector_id, channel, adc_value, timestamp_ns)| ParticleHit {
            detector_id,
            channel,
            adc_value,
            timestamp_ns,
        },
    )
}

fn track_segment_strategy() -> impl Strategy<Value = TrackSegment> {
    (
        any::<u64>(),
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
        any::<u32>(),
    )
        .prop_map(|(track_id, x_mm, y_mm, z_mm, momentum_mev)| TrackSegment {
            track_id,
            x_mm,
            y_mm,
            z_mm,
            momentum_mev,
        })
}

fn decay_event_strategy() -> impl Strategy<Value = DecayEvent> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<u32>(),
        particle_type_strategy(),
    )
        .prop_map(
            |(event_id, decay_time_ns, energy_mev, particle_type)| DecayEvent {
                event_id,
                decay_time_ns,
                energy_mev,
                particle_type,
            },
        )
}

fn beam_pulse_strategy() -> impl Strategy<Value = BeamPulse> {
    (any::<u64>(), any::<u32>(), any::<u32>(), any::<u16>()).prop_map(
        |(pulse_id, intensity_ua, energy_gev, repetition_hz)| BeamPulse {
            pulse_id,
            intensity_ua,
            energy_gev,
            repetition_hz,
        },
    )
}

fn radiation_dose_strategy() -> impl Strategy<Value = RadiationDose> {
    (any::<u32>(), any::<u32>(), any::<u32>(), any::<u64>()).prop_map(
        |(detector_id, dose_ugy, dose_rate_ugy_h, timestamp)| RadiationDose {
            detector_id,
            dose_ugy,
            dose_rate_ugy_h,
            timestamp,
        },
    )
}

// ── Property-based tests ──────────────────────────────────────────────────────

proptest! {
    // 1. ParticleHit roundtrip
    #[test]
    fn prop_particle_hit_roundtrip(hit in particle_hit_strategy()) {
        let encoded = encode_to_vec(&hit).expect("ParticleHit encode failed");
        let (decoded, _): (ParticleHit, usize) =
            decode_from_slice(&encoded).expect("ParticleHit decode failed");
        prop_assert_eq!(hit, decoded);
    }

    // 2. TrackSegment roundtrip
    #[test]
    fn prop_track_segment_roundtrip(seg in track_segment_strategy()) {
        let encoded = encode_to_vec(&seg).expect("TrackSegment encode failed");
        let (decoded, _): (TrackSegment, usize) =
            decode_from_slice(&encoded).expect("TrackSegment decode failed");
        prop_assert_eq!(seg, decoded);
    }

    // 3. DecayEvent roundtrip
    #[test]
    fn prop_decay_event_roundtrip(evt in decay_event_strategy()) {
        let encoded = encode_to_vec(&evt).expect("DecayEvent encode failed");
        let (decoded, _): (DecayEvent, usize) =
            decode_from_slice(&encoded).expect("DecayEvent decode failed");
        prop_assert_eq!(evt, decoded);
    }

    // 4. BeamPulse roundtrip
    #[test]
    fn prop_beam_pulse_roundtrip(pulse in beam_pulse_strategy()) {
        let encoded = encode_to_vec(&pulse).expect("BeamPulse encode failed");
        let (decoded, _): (BeamPulse, usize) =
            decode_from_slice(&encoded).expect("BeamPulse decode failed");
        prop_assert_eq!(pulse, decoded);
    }

    // 5. RadiationDose roundtrip
    #[test]
    fn prop_radiation_dose_roundtrip(dose in radiation_dose_strategy()) {
        let encoded = encode_to_vec(&dose).expect("RadiationDose encode failed");
        let (decoded, _): (RadiationDose, usize) =
            decode_from_slice(&encoded).expect("RadiationDose decode failed");
        prop_assert_eq!(dose, decoded);
    }

    // 6. ParticleType enum roundtrip (all variants)
    #[test]
    fn prop_particle_type_roundtrip(pt in particle_type_strategy()) {
        let encoded = encode_to_vec(&pt).expect("ParticleType encode failed");
        let (decoded, _): (ParticleType, usize) =
            decode_from_slice(&encoded).expect("ParticleType decode failed");
        prop_assert_eq!(pt, decoded);
    }

    // 7. ParticleHit deterministic encoding: same value → same bytes
    #[test]
    fn prop_particle_hit_deterministic(hit in particle_hit_strategy()) {
        let enc1 = encode_to_vec(&hit).expect("ParticleHit first encode failed");
        let enc2 = encode_to_vec(&hit).expect("ParticleHit second encode failed");
        prop_assert_eq!(enc1, enc2);
    }

    // 8. TrackSegment deterministic encoding
    #[test]
    fn prop_track_segment_deterministic(seg in track_segment_strategy()) {
        let enc1 = encode_to_vec(&seg).expect("TrackSegment first encode failed");
        let enc2 = encode_to_vec(&seg).expect("TrackSegment second encode failed");
        prop_assert_eq!(enc1, enc2);
    }

    // 9. DecayEvent deterministic encoding
    #[test]
    fn prop_decay_event_deterministic(evt in decay_event_strategy()) {
        let enc1 = encode_to_vec(&evt).expect("DecayEvent first encode failed");
        let enc2 = encode_to_vec(&evt).expect("DecayEvent second encode failed");
        prop_assert_eq!(enc1, enc2);
    }

    // 10. BeamPulse consumed bytes == encoded length
    #[test]
    fn prop_beam_pulse_consumed_bytes(pulse in beam_pulse_strategy()) {
        let encoded = encode_to_vec(&pulse).expect("BeamPulse encode failed");
        let len = encoded.len();
        let (_, consumed): (BeamPulse, usize) =
            decode_from_slice(&encoded).expect("BeamPulse decode failed");
        prop_assert_eq!(consumed, len);
    }

    // 11. RadiationDose consumed bytes == encoded length
    #[test]
    fn prop_radiation_dose_consumed_bytes(dose in radiation_dose_strategy()) {
        let encoded = encode_to_vec(&dose).expect("RadiationDose encode failed");
        let len = encoded.len();
        let (_, consumed): (RadiationDose, usize) =
            decode_from_slice(&encoded).expect("RadiationDose decode failed");
        prop_assert_eq!(consumed, len);
    }

    // 12. ParticleHit consumed bytes == encoded length
    #[test]
    fn prop_particle_hit_consumed_bytes(hit in particle_hit_strategy()) {
        let encoded = encode_to_vec(&hit).expect("ParticleHit encode failed");
        let len = encoded.len();
        let (_, consumed): (ParticleHit, usize) =
            decode_from_slice(&encoded).expect("ParticleHit decode failed");
        prop_assert_eq!(consumed, len);
    }

    // 13. Vec<ParticleHit> roundtrip
    #[test]
    fn prop_vec_particle_hit_roundtrip(
        hits in prop::collection::vec(particle_hit_strategy(), 0..16)
    ) {
        let encoded = encode_to_vec(&hits).expect("Vec<ParticleHit> encode failed");
        let (decoded, _): (Vec<ParticleHit>, usize) =
            decode_from_slice(&encoded).expect("Vec<ParticleHit> decode failed");
        prop_assert_eq!(hits, decoded);
    }

    // 14. Vec<TrackSegment> roundtrip
    #[test]
    fn prop_vec_track_segment_roundtrip(
        segs in prop::collection::vec(track_segment_strategy(), 0..16)
    ) {
        let encoded = encode_to_vec(&segs).expect("Vec<TrackSegment> encode failed");
        let (decoded, _): (Vec<TrackSegment>, usize) =
            decode_from_slice(&encoded).expect("Vec<TrackSegment> decode failed");
        prop_assert_eq!(segs, decoded);
    }

    // 15. Vec<DecayEvent> roundtrip
    #[test]
    fn prop_vec_decay_event_roundtrip(
        evts in prop::collection::vec(decay_event_strategy(), 0..16)
    ) {
        let encoded = encode_to_vec(&evts).expect("Vec<DecayEvent> encode failed");
        let (decoded, _): (Vec<DecayEvent>, usize) =
            decode_from_slice(&encoded).expect("Vec<DecayEvent> decode failed");
        prop_assert_eq!(evts, decoded);
    }

    // 16. Option<ParticleHit> roundtrip (Some and None paths)
    #[test]
    fn prop_option_particle_hit_roundtrip(
        opt in prop::option::of(particle_hit_strategy())
    ) {
        let encoded = encode_to_vec(&opt).expect("Option<ParticleHit> encode failed");
        let (decoded, _): (Option<ParticleHit>, usize) =
            decode_from_slice(&encoded).expect("Option<ParticleHit> decode failed");
        prop_assert_eq!(opt, decoded);
    }

    // 17. Option<BeamPulse> roundtrip
    #[test]
    fn prop_option_beam_pulse_roundtrip(
        opt in prop::option::of(beam_pulse_strategy())
    ) {
        let encoded = encode_to_vec(&opt).expect("Option<BeamPulse> encode failed");
        let (decoded, _): (Option<BeamPulse>, usize) =
            decode_from_slice(&encoded).expect("Option<BeamPulse> decode failed");
        prop_assert_eq!(opt, decoded);
    }

    // 18. Primitive u64 roundtrip (e.g. event timestamps in nanoseconds)
    #[test]
    fn prop_primitive_u64_roundtrip(val: u64) {
        let encoded = encode_to_vec(&val).expect("u64 encode failed");
        let (decoded, _): (u64, usize) =
            decode_from_slice(&encoded).expect("u64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 19. Primitive i64 roundtrip (e.g. signed detector offsets)
    #[test]
    fn prop_primitive_i64_roundtrip(val: i64) {
        let encoded = encode_to_vec(&val).expect("i64 encode failed");
        let (decoded, _): (i64, usize) =
            decode_from_slice(&encoded).expect("i64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 20. Primitive f64 roundtrip (e.g. energy measurements in MeV)
    #[test]
    fn prop_primitive_f64_roundtrip(val: f64) {
        let encoded = encode_to_vec(&val).expect("f64 encode failed");
        let (decoded, _): (f64, usize) =
            decode_from_slice(&encoded).expect("f64 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    // 21. Primitive f32 roundtrip (e.g. low-precision gain calibration values)
    #[test]
    fn prop_primitive_f32_roundtrip(val: f32) {
        let encoded = encode_to_vec(&val).expect("f32 encode failed");
        let (decoded, _): (f32, usize) =
            decode_from_slice(&encoded).expect("f32 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    // 22. String roundtrip (e.g. detector labels / subsystem names)
    #[test]
    fn prop_string_detector_label_roundtrip(
        label in "[A-Za-z0-9_\\-]{0,64}"
    ) {
        let encoded = encode_to_vec(&label).expect("String encode failed");
        let (decoded, consumed): (String, usize) =
            decode_from_slice(&encoded).expect("String decode failed");
        prop_assert_eq!(&label, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}
