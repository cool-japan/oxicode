//! Advanced property-based roundtrip tests (set 34) using proptest.
//!
//! Theme: Audio/media metadata.
//! Each proptest! block contains exactly one #[test] function.
//! Tests verify that encode → decode is a perfect roundtrip for all tested types.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AudioFormat {
    Mp3,
    Aac,
    Flac,
    Ogg,
    Wav,
    Opus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioTrack {
    track_id: u64,
    title: String,
    duration_ms: u64,
    sample_rate: u32,
    channels: u8,
    bitrate_kbps: u32,
    format: AudioFormat,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Playlist {
    name: String,
    track_ids: Vec<u64>,
    total_duration_ms: u64,
    is_public: bool,
}

fn audio_format_from_u8(n: u8) -> AudioFormat {
    match n {
        0 => AudioFormat::Mp3,
        1 => AudioFormat::Aac,
        2 => AudioFormat::Flac,
        3 => AudioFormat::Ogg,
        4 => AudioFormat::Wav,
        _ => AudioFormat::Opus,
    }
}

fn audio_track_strategy() -> impl Strategy<Value = AudioTrack> {
    (
        any::<u64>(),
        any::<String>(),
        any::<u64>(),
        any::<u32>(),
        any::<u8>(),
        any::<u32>(),
        (0u8..6u8).prop_map(audio_format_from_u8),
    )
        .prop_map(
            |(track_id, title, duration_ms, sample_rate, channels, bitrate_kbps, format)| {
                AudioTrack {
                    track_id,
                    title,
                    duration_ms,
                    sample_rate,
                    channels,
                    bitrate_kbps,
                    format,
                }
            },
        )
}

fn playlist_strategy() -> impl Strategy<Value = Playlist> {
    (
        any::<String>(),
        proptest::collection::vec(any::<u64>(), 0..20),
        any::<u64>(),
        any::<bool>(),
    )
        .prop_map(|(name, track_ids, total_duration_ms, is_public)| Playlist {
            name,
            track_ids,
            total_duration_ms,
            is_public,
        })
}

// Test 1: AudioTrack roundtrip with various fields
proptest! {
    #[test]
    fn prop_audio_track_roundtrip(track in audio_track_strategy()) {
        let encoded = encode_to_vec(&track).expect("encode failed");
        let (decoded, _): (AudioTrack, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(track, decoded);
    }
}

// Test 2: AudioFormat variant roundtrip (0u8..6u8 → each variant)
proptest! {
    #[test]
    fn prop_audio_format_variant_roundtrip(n in 0u8..6u8) {
        let fmt = audio_format_from_u8(n);
        let encoded = encode_to_vec(&fmt).expect("encode failed");
        let (decoded, _): (AudioFormat, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(fmt, decoded);
    }
}

// Test 3: Playlist roundtrip
proptest! {
    #[test]
    fn prop_playlist_roundtrip(playlist in playlist_strategy()) {
        let encoded = encode_to_vec(&playlist).expect("encode failed");
        let (decoded, _): (Playlist, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(playlist, decoded);
    }
}

// Test 4: Vec<AudioTrack> roundtrip (0..6 items)
proptest! {
    #[test]
    fn prop_vec_audio_track_roundtrip(
        tracks in proptest::collection::vec(audio_track_strategy(), 0..6)
    ) {
        let encoded = encode_to_vec(&tracks).expect("encode failed");
        let (decoded, _): (Vec<AudioTrack>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(tracks, decoded);
    }
}

// Test 5: Vec<Playlist> roundtrip (0..4 items)
proptest! {
    #[test]
    fn prop_vec_playlist_roundtrip(
        playlists in proptest::collection::vec(playlist_strategy(), 0..4)
    ) {
        let encoded = encode_to_vec(&playlists).expect("encode failed");
        let (decoded, _): (Vec<Playlist>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(playlists, decoded);
    }
}

// Test 6: Option<AudioTrack> roundtrip
proptest! {
    #[test]
    fn prop_option_audio_track_roundtrip(
        opt in proptest::option::of(audio_track_strategy())
    ) {
        let encoded = encode_to_vec(&opt).expect("encode failed");
        let (decoded, _): (Option<AudioTrack>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(opt, decoded);
    }
}

// Test 7: Option<Playlist> roundtrip
proptest! {
    #[test]
    fn prop_option_playlist_roundtrip(
        opt in proptest::option::of(playlist_strategy())
    ) {
        let encoded = encode_to_vec(&opt).expect("encode failed");
        let (decoded, _): (Option<Playlist>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(opt, decoded);
    }
}

// Test 8: Deterministic encoding for AudioTrack
proptest! {
    #[test]
    fn prop_audio_track_deterministic_encoding(track in audio_track_strategy()) {
        let encoded_a = encode_to_vec(&track).expect("encode failed");
        let encoded_b = encode_to_vec(&track).expect("encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// Test 9: Deterministic encoding for Playlist
proptest! {
    #[test]
    fn prop_playlist_deterministic_encoding(playlist in playlist_strategy()) {
        let encoded_a = encode_to_vec(&playlist).expect("encode failed");
        let encoded_b = encode_to_vec(&playlist).expect("encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// Test 10: Consumed bytes == encoded length for AudioTrack
proptest! {
    #[test]
    fn prop_audio_track_consumed_eq_len(track in audio_track_strategy()) {
        let encoded = encode_to_vec(&track).expect("encode failed");
        let (_, consumed): (AudioTrack, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 11: Consumed bytes == encoded length for Playlist
proptest! {
    #[test]
    fn prop_playlist_consumed_eq_len(playlist in playlist_strategy()) {
        let encoded = encode_to_vec(&playlist).expect("encode failed");
        let (_, consumed): (Playlist, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 12: u64 track_id full range roundtrip
proptest! {
    #[test]
    fn prop_track_id_full_range_roundtrip(track_id: u64) {
        let encoded = encode_to_vec(&track_id).expect("encode failed");
        let (decoded, consumed): (u64, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(decoded, track_id);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 13: u64 duration_ms full range roundtrip
proptest! {
    #[test]
    fn prop_duration_ms_full_range_roundtrip(duration_ms: u64) {
        let encoded = encode_to_vec(&duration_ms).expect("encode failed");
        let (decoded, consumed): (u64, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(decoded, duration_ms);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 14: u32 sample_rate roundtrip
proptest! {
    #[test]
    fn prop_sample_rate_roundtrip(sample_rate: u32) {
        let encoded = encode_to_vec(&sample_rate).expect("encode failed");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(decoded, sample_rate);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 15: u8 channels roundtrip (0..255)
proptest! {
    #[test]
    fn prop_channels_u8_roundtrip(channels in 0u8..=255u8) {
        let encoded = encode_to_vec(&channels).expect("encode failed");
        let (decoded, consumed): (u8, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(decoded, channels);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 16: bool is_public roundtrip
proptest! {
    #[test]
    fn prop_is_public_bool_roundtrip(is_public: bool) {
        let encoded = encode_to_vec(&is_public).expect("encode failed");
        let (decoded, consumed): (bool, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(decoded, is_public);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 17: Empty track_ids in Playlist roundtrip
proptest! {
    #[test]
    fn prop_playlist_empty_track_ids_roundtrip(
        name in any::<String>(),
        total_duration_ms: u64,
        is_public: bool,
    ) {
        let playlist = Playlist {
            name,
            track_ids: Vec::new(),
            total_duration_ms,
            is_public,
        };
        let encoded = encode_to_vec(&playlist).expect("encode failed");
        let (decoded, consumed): (Playlist, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(playlist, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 18: Large track_ids (0..20 items)
proptest! {
    #[test]
    fn prop_playlist_large_track_ids_roundtrip(
        name in any::<String>(),
        track_ids in proptest::collection::vec(any::<u64>(), 0..20),
        total_duration_ms: u64,
        is_public: bool,
    ) {
        let playlist = Playlist {
            name,
            track_ids,
            total_duration_ms,
            is_public,
        };
        let encoded = encode_to_vec(&playlist).expect("encode failed");
        let (decoded, _): (Playlist, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(playlist, decoded);
    }
}

// Test 19: Distinct AudioTracks with same title but different duration → different bytes
proptest! {
    #[test]
    fn prop_audio_tracks_same_title_different_duration_differ(
        track_id: u64,
        title in any::<String>(),
        duration_a: u64,
        duration_b: u64,
        sample_rate: u32,
        channels: u8,
        bitrate_kbps: u32,
        n in 0u8..6u8,
    ) {
        prop_assume!(duration_a != duration_b);
        let format_a = audio_format_from_u8(n);
        let format_b = audio_format_from_u8(n);
        let track_a = AudioTrack {
            track_id,
            title: title.clone(),
            duration_ms: duration_a,
            sample_rate,
            channels,
            bitrate_kbps,
            format: format_a,
        };
        let track_b = AudioTrack {
            track_id,
            title,
            duration_ms: duration_b,
            sample_rate,
            channels,
            bitrate_kbps,
            format: format_b,
        };
        let encoded_a = encode_to_vec(&track_a).expect("encode failed");
        let encoded_b = encode_to_vec(&track_b).expect("encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// Test 20: String title with various content roundtrip
proptest! {
    #[test]
    fn prop_audio_track_title_string_roundtrip(
        title in any::<String>(),
        track_id: u64,
        duration_ms: u64,
        sample_rate: u32,
        channels: u8,
        bitrate_kbps: u32,
        n in 0u8..6u8,
    ) {
        let track = AudioTrack {
            track_id,
            title,
            duration_ms,
            sample_rate,
            channels,
            bitrate_kbps,
            format: audio_format_from_u8(n),
        };
        let encoded = encode_to_vec(&track).expect("encode failed");
        let (decoded, consumed): (AudioTrack, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        prop_assert_eq!(track, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// Test 21: Double encode/decode identity for Playlist
proptest! {
    #[test]
    fn prop_playlist_double_encode_decode_identity(playlist in playlist_strategy()) {
        let encoded_once = encode_to_vec(&playlist).expect("encode failed");
        let (decoded_once, _): (Playlist, usize) =
            decode_from_slice(&encoded_once).expect("decode failed");
        let encoded_twice = encode_to_vec(&decoded_once).expect("encode failed");
        let (decoded_twice, consumed): (Playlist, usize) =
            decode_from_slice(&encoded_twice).expect("decode failed");
        prop_assert_eq!(playlist, decoded_twice);
        prop_assert_eq!(consumed, encoded_twice.len());
    }
}

// Test 22: Non-empty encoded bytes assertion
proptest! {
    #[test]
    fn prop_audio_track_encoded_bytes_non_empty(track in audio_track_strategy()) {
        let encoded = encode_to_vec(&track).expect("encode failed");
        prop_assert!(!encoded.is_empty(), "encoded bytes must be non-empty");
    }
}
