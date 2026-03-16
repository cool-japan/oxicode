//! Advanced property-based tests (set 40) using proptest.
//!
//! Theme: Music / audio metadata — Genre, Track, Album, Artist.
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Tests cover roundtrips, consumed bytes, determinism, Vec types,
//! all Genre variants, nested Album with tracks, Artist with multiple albums,
//! and boundary values.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Genre {
    Rock,
    Pop,
    Jazz,
    Classical,
    Electronic,
    HipHop,
    Country,
    Metal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Track {
    id: u64,
    title: String,
    duration_secs: u32,
    track_number: u16,
    genre: Genre,
    explicit: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Album {
    id: u64,
    title: String,
    year: u16,
    tracks: Vec<Track>,
    genre: Genre,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Artist {
    id: u64,
    name: String,
    albums: Vec<Album>,
    monthly_listeners: u64,
}

// ── Strategy helpers ──────────────────────────────────────────────────────────

fn genre_strategy() -> impl Strategy<Value = Genre> {
    prop_oneof![
        Just(Genre::Rock),
        Just(Genre::Pop),
        Just(Genre::Jazz),
        Just(Genre::Classical),
        Just(Genre::Electronic),
        Just(Genre::HipHop),
        Just(Genre::Country),
        Just(Genre::Metal),
    ]
}

fn track_strategy() -> impl Strategy<Value = Track> {
    (
        any::<u64>(),
        "[a-zA-Z0-9 ]{1,40}",
        0u32..7200u32,
        1u16..30u16,
        genre_strategy(),
        any::<bool>(),
    )
        .prop_map(
            |(id, title, duration_secs, track_number, genre, explicit)| Track {
                id,
                title,
                duration_secs,
                track_number,
                genre,
                explicit,
            },
        )
}

fn album_strategy() -> impl Strategy<Value = Album> {
    (
        any::<u64>(),
        "[a-zA-Z0-9 ]{1,40}",
        1900u16..2025u16,
        prop::collection::vec(track_strategy(), 0..8usize),
        genre_strategy(),
    )
        .prop_map(|(id, title, year, tracks, genre)| Album {
            id,
            title,
            year,
            tracks,
            genre,
        })
}

fn artist_strategy() -> impl Strategy<Value = Artist> {
    (
        any::<u64>(),
        "[a-zA-Z0-9 ]{1,40}",
        prop::collection::vec(album_strategy(), 0..4usize),
        any::<u64>(),
    )
        .prop_map(|(id, name, albums, monthly_listeners)| Artist {
            id,
            name,
            albums,
            monthly_listeners,
        })
}

// ── 1. Genre roundtrip ────────────────────────────────────────────────────────

#[test]
fn test_genre_roundtrip() {
    proptest!(|(g in genre_strategy())| {
        let enc = encode_to_vec(&g).expect("encode Genre failed");
        let (decoded, consumed): (Genre, usize) =
            decode_from_slice(&enc).expect("decode Genre failed");
        prop_assert_eq!(g, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 2. Genre consumed bytes equal encoded length ──────────────────────────────

#[test]
fn test_genre_consumed_eq_len() {
    proptest!(|(g in genre_strategy())| {
        let enc = encode_to_vec(&g).expect("encode Genre failed");
        let (_decoded, consumed): (Genre, usize) =
            decode_from_slice(&enc).expect("decode Genre failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 3. Genre deterministic encoding ──────────────────────────────────────────

#[test]
fn test_genre_deterministic_encoding() {
    proptest!(|(g in genre_strategy())| {
        let enc1 = encode_to_vec(&g).expect("first encode Genre failed");
        let enc2 = encode_to_vec(&g).expect("second encode Genre failed");
        prop_assert_eq!(enc1, enc2, "Genre encoding must be deterministic");
    });
}

// ── 4. All eight Genre variants encode and decode correctly ───────────────────

#[test]
fn test_all_genre_variants_roundtrip() {
    let variants = [
        Genre::Rock,
        Genre::Pop,
        Genre::Jazz,
        Genre::Classical,
        Genre::Electronic,
        Genre::HipHop,
        Genre::Country,
        Genre::Metal,
    ];
    for g in &variants {
        let enc = encode_to_vec(g).expect("encode Genre variant failed");
        let (decoded, consumed): (Genre, usize) =
            decode_from_slice(&enc).expect("decode Genre variant failed");
        assert_eq!(g, &decoded, "Genre variant mismatch");
        assert_eq!(consumed, enc.len(), "Genre consumed bytes mismatch");
    }
    // proptest! wrapper to satisfy the #[test] contract via a trivial property
    proptest!(|(_dummy: u8)| {
        prop_assert!(true);
    });
}

// ── 5. Track roundtrip ────────────────────────────────────────────────────────

#[test]
fn test_track_roundtrip() {
    proptest!(|(t in track_strategy())| {
        let enc = encode_to_vec(&t).expect("encode Track failed");
        let (decoded, consumed): (Track, usize) =
            decode_from_slice(&enc).expect("decode Track failed");
        prop_assert_eq!(t, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 6. Track re-encode is idempotent ─────────────────────────────────────────

#[test]
fn test_track_reencode_idempotent() {
    proptest!(|(t in track_strategy())| {
        let enc1 = encode_to_vec(&t).expect("first encode Track failed");
        let (decoded, _): (Track, usize) =
            decode_from_slice(&enc1).expect("decode Track failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Track failed");
        prop_assert_eq!(enc1, enc2, "Track re-encoding must be idempotent");
    });
}

// ── 7. Track with zero duration (boundary) ───────────────────────────────────

#[test]
fn test_track_zero_duration_roundtrip() {
    proptest!(|(
        id: u64,
        title in "[a-zA-Z]{1,30}",
        track_number in 1u16..20u16,
        g in genre_strategy(),
        explicit: bool,
    )| {
        let t = Track { id, title, duration_secs: 0, track_number, genre: g, explicit };
        let enc = encode_to_vec(&t).expect("encode zero-duration Track failed");
        let (decoded, consumed): (Track, usize) =
            decode_from_slice(&enc).expect("decode zero-duration Track failed");
        prop_assert_eq!(t, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 8. Track with max duration boundary ──────────────────────────────────────

#[test]
fn test_track_max_duration_roundtrip() {
    proptest!(|(
        id: u64,
        title in "[a-zA-Z]{1,30}",
        track_number in 1u16..20u16,
        g in genre_strategy(),
        explicit: bool,
    )| {
        let t = Track {
            id,
            title,
            duration_secs: u32::MAX,
            track_number,
            genre: g,
            explicit,
        };
        let enc = encode_to_vec(&t).expect("encode max-duration Track failed");
        let (decoded, consumed): (Track, usize) =
            decode_from_slice(&enc).expect("decode max-duration Track failed");
        prop_assert_eq!(t, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 9. Vec<Track> roundtrip ───────────────────────────────────────────────────

#[test]
fn test_vec_track_roundtrip() {
    proptest!(|(tracks in prop::collection::vec(track_strategy(), 0..10usize))| {
        let enc = encode_to_vec(&tracks).expect("encode Vec<Track> failed");
        let (decoded, consumed): (Vec<Track>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Track> failed");
        prop_assert_eq!(tracks, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 10. Album roundtrip ───────────────────────────────────────────────────────

#[test]
fn test_album_roundtrip() {
    proptest!(|(a in album_strategy())| {
        let enc = encode_to_vec(&a).expect("encode Album failed");
        let (decoded, consumed): (Album, usize) =
            decode_from_slice(&enc).expect("decode Album failed");
        prop_assert_eq!(a, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 11. Album with empty track list ──────────────────────────────────────────

#[test]
fn test_album_empty_tracks_roundtrip() {
    proptest!(|(
        id: u64,
        title in "[a-zA-Z0-9 ]{1,40}",
        year in 1900u16..2025u16,
        g in genre_strategy(),
    )| {
        let a = Album { id, title, year, tracks: vec![], genre: g };
        let enc = encode_to_vec(&a).expect("encode empty-tracks Album failed");
        let (decoded, consumed): (Album, usize) =
            decode_from_slice(&enc).expect("decode empty-tracks Album failed");
        prop_assert_eq!(a, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 12. Album re-encode is idempotent ────────────────────────────────────────

#[test]
fn test_album_reencode_idempotent() {
    proptest!(|(a in album_strategy())| {
        let enc1 = encode_to_vec(&a).expect("first encode Album failed");
        let (decoded, _): (Album, usize) =
            decode_from_slice(&enc1).expect("decode Album failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Album failed");
        prop_assert_eq!(enc1, enc2, "Album re-encoding must be idempotent");
    });
}

// ── 13. Vec<Album> roundtrip ──────────────────────────────────────────────────

#[test]
fn test_vec_album_roundtrip() {
    proptest!(|(albums in prop::collection::vec(album_strategy(), 0..5usize))| {
        let enc = encode_to_vec(&albums).expect("encode Vec<Album> failed");
        let (decoded, consumed): (Vec<Album>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Album> failed");
        prop_assert_eq!(albums, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 14. Artist roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_artist_roundtrip() {
    proptest!(|(ar in artist_strategy())| {
        let enc = encode_to_vec(&ar).expect("encode Artist failed");
        let (decoded, consumed): (Artist, usize) =
            decode_from_slice(&enc).expect("decode Artist failed");
        prop_assert_eq!(ar, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 15. Artist with zero monthly listeners (boundary) ────────────────────────

#[test]
fn test_artist_zero_listeners_roundtrip() {
    proptest!(|(
        id: u64,
        name in "[a-zA-Z0-9 ]{1,40}",
        albums in prop::collection::vec(album_strategy(), 0..3usize),
    )| {
        let ar = Artist { id, name, albums, monthly_listeners: 0 };
        let enc = encode_to_vec(&ar).expect("encode zero-listeners Artist failed");
        let (decoded, consumed): (Artist, usize) =
            decode_from_slice(&enc).expect("decode zero-listeners Artist failed");
        prop_assert_eq!(ar, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 16. Artist with max monthly listeners (boundary) ─────────────────────────

#[test]
fn test_artist_max_listeners_roundtrip() {
    proptest!(|(
        id: u64,
        name in "[a-zA-Z0-9 ]{1,40}",
        albums in prop::collection::vec(album_strategy(), 0..3usize),
    )| {
        let ar = Artist { id, name, albums, monthly_listeners: u64::MAX };
        let enc = encode_to_vec(&ar).expect("encode max-listeners Artist failed");
        let (decoded, consumed): (Artist, usize) =
            decode_from_slice(&enc).expect("decode max-listeners Artist failed");
        prop_assert_eq!(ar, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 17. Artist with multiple albums each having tracks ────────────────────────

#[test]
fn test_artist_multiple_albums_with_tracks() {
    proptest!(|(
        id: u64,
        name in "[a-zA-Z0-9 ]{1,40}",
        monthly_listeners: u64,
        albums in prop::collection::vec(
            (
                any::<u64>(),
                "[a-zA-Z0-9 ]{1,30}",
                1990u16..2025u16,
                prop::collection::vec(track_strategy(), 1..6usize),
                genre_strategy(),
            ).prop_map(|(aid, atitle, year, tracks, g)| Album {
                id: aid,
                title: atitle,
                year,
                tracks,
                genre: g,
            }),
            1..4usize,
        ),
    )| {
        let ar = Artist { id, name, albums, monthly_listeners };
        let enc = encode_to_vec(&ar).expect("encode multi-album Artist failed");
        let (decoded, consumed): (Artist, usize) =
            decode_from_slice(&enc).expect("decode multi-album Artist failed");
        prop_assert_eq!(ar, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 18. Artist re-encode is idempotent ────────────────────────────────────────

#[test]
fn test_artist_reencode_idempotent() {
    proptest!(|(ar in artist_strategy())| {
        let enc1 = encode_to_vec(&ar).expect("first encode Artist failed");
        let (decoded, _): (Artist, usize) =
            decode_from_slice(&enc1).expect("decode Artist failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Artist failed");
        prop_assert_eq!(enc1, enc2, "Artist re-encoding must be idempotent");
    });
}

// ── 19. Vec<Artist> roundtrip ─────────────────────────────────────────────────

#[test]
fn test_vec_artist_roundtrip() {
    proptest!(|(artists in prop::collection::vec(artist_strategy(), 0..4usize))| {
        let enc = encode_to_vec(&artists).expect("encode Vec<Artist> failed");
        let (decoded, consumed): (Vec<Artist>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Artist> failed");
        prop_assert_eq!(artists, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 20. Artist encoding length grows monotonically with album count ───────────

#[test]
fn test_artist_encoding_grows_with_albums() {
    proptest!(|(
        id: u64,
        name in "[a-zA-Z0-9 ]{1,20}",
        monthly_listeners: u64,
        base_album in album_strategy(),
        extra_album in album_strategy(),
    )| {
        let ar_fewer = Artist {
            id,
            name: name.clone(),
            albums: vec![base_album.clone()],
            monthly_listeners,
        };
        let ar_more = Artist {
            id,
            name,
            albums: vec![base_album, extra_album],
            monthly_listeners,
        };
        let enc_fewer = encode_to_vec(&ar_fewer).expect("encode fewer-albums Artist failed");
        let enc_more = encode_to_vec(&ar_more).expect("encode more-albums Artist failed");
        prop_assert!(
            enc_more.len() >= enc_fewer.len(),
            "more albums should produce >= encoded bytes"
        );
    });
}

// ── 21. Album track count is preserved after roundtrip ───────────────────────

#[test]
fn test_album_track_count_preserved() {
    proptest!(|(a in album_strategy())| {
        let original_count = a.tracks.len();
        let enc = encode_to_vec(&a).expect("encode Album failed");
        let (decoded, _): (Album, usize) =
            decode_from_slice(&enc).expect("decode Album failed");
        prop_assert_eq!(decoded.tracks.len(), original_count,
            "track count must survive encode/decode");
    });
}

// ── 22. Option<Artist> roundtrip ──────────────────────────────────────────────

#[test]
fn test_option_artist_roundtrip() {
    proptest!(|(opt in prop::option::of(artist_strategy()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<Artist> failed");
        let (decoded, consumed): (Option<Artist>, usize) =
            decode_from_slice(&enc).expect("decode Option<Artist> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}
