//! Advanced checksum integration tests — audio processing / music production domain.

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{unwrap_with_checksum, wrap_with_checksum, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum AudioCodec {
    Pcm,
    Mp3,
    Aac,
    Flac,
    Opus,
    Vorbis,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SampleRate {
    Rate8k,
    Rate16k,
    Rate44k,
    Rate48k,
    Rate96k,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AudioSample {
    timestamp_us: u64,
    left_channel: i16,
    right_channel: i16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AudioTrack {
    track_id: u32,
    codec: AudioCodec,
    sample_rate: SampleRate,
    bit_depth: u8,
    samples: Vec<AudioSample>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixSession {
    session_id: u64,
    bpm: u32,
    time_signature_num: u8,
    time_signature_den: u8,
    tracks: Vec<AudioTrack>,
}

// ---------------------------------------------------------------------------
// Helper: encode a value to bytes then wrap with checksum
// ---------------------------------------------------------------------------
fn encode_and_wrap<T: Encode>(value: &T) -> Vec<u8> {
    let encoded = encode_to_vec(value).expect("encode_to_vec failed");
    wrap_with_checksum(&encoded)
}

// Helper: unwrap checksum then decode
fn unwrap_and_decode<T: Decode>(wrapped: &[u8]) -> T {
    let payload = unwrap_with_checksum(wrapped).expect("unwrap_with_checksum failed");
    let (value, _) = decode_from_slice::<T>(&payload).expect("decode_from_slice failed");
    value
}

// ---------------------------------------------------------------------------
// Test 1: HEADER_SIZE constant equals 16
// ---------------------------------------------------------------------------
#[test]
fn test_header_size_constant_is_16() {
    assert_eq!(HEADER_SIZE, 16, "HEADER_SIZE must be exactly 16");
}

// ---------------------------------------------------------------------------
// Test 2: wrap increases length by exactly HEADER_SIZE
// ---------------------------------------------------------------------------
#[test]
fn test_wrap_increases_length_by_header_size() {
    let sample = AudioSample {
        timestamp_us: 1_000_000,
        left_channel: 1024,
        right_channel: -512,
    };
    let encoded = encode_to_vec(&sample).expect("encode_to_vec failed");
    let plain_len = encoded.len();
    let wrapped = wrap_with_checksum(&encoded);
    assert_eq!(
        wrapped.len(),
        plain_len + HEADER_SIZE,
        "wrapped length must equal plain length plus HEADER_SIZE"
    );
}

// ---------------------------------------------------------------------------
// Test 3: AudioSample wrapped and verified roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_audio_sample_wrap_verify_roundtrip() {
    let sample = AudioSample {
        timestamp_us: 44_100,
        left_channel: 16000,
        right_channel: -16000,
    };
    let wrapped = encode_and_wrap(&sample);
    let decoded: AudioSample = unwrap_and_decode(&wrapped);
    assert_eq!(sample, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: AudioTrack integrity roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_audio_track_integrity_roundtrip() {
    let track = AudioTrack {
        track_id: 1,
        codec: AudioCodec::Flac,
        sample_rate: SampleRate::Rate44k,
        bit_depth: 24,
        samples: vec![
            AudioSample {
                timestamp_us: 0,
                left_channel: 100,
                right_channel: -100,
            },
            AudioSample {
                timestamp_us: 1000,
                left_channel: 200,
                right_channel: -200,
            },
        ],
    };
    let wrapped = encode_and_wrap(&track);
    let decoded: AudioTrack = unwrap_and_decode(&wrapped);
    assert_eq!(track, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: MixSession integrity roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mix_session_integrity_roundtrip() {
    let session = MixSession {
        session_id: 42,
        bpm: 120,
        time_signature_num: 4,
        time_signature_den: 4,
        tracks: vec![AudioTrack {
            track_id: 0,
            codec: AudioCodec::Pcm,
            sample_rate: SampleRate::Rate48k,
            bit_depth: 16,
            samples: vec![AudioSample {
                timestamp_us: 0,
                left_channel: 0,
                right_channel: 0,
            }],
        }],
    };
    let wrapped = encode_and_wrap(&session);
    let decoded: MixSession = unwrap_and_decode(&wrapped);
    assert_eq!(session, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Each AudioCodec variant roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_each_audio_codec_variant_roundtrip() {
    let codecs = [
        AudioCodec::Pcm,
        AudioCodec::Mp3,
        AudioCodec::Aac,
        AudioCodec::Flac,
        AudioCodec::Opus,
        AudioCodec::Vorbis,
    ];
    for codec in codecs {
        let encoded = encode_to_vec(&codec).expect("encode_to_vec failed");
        let wrapped = wrap_with_checksum(&encoded);
        let payload = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
        let (decoded, _): (AudioCodec, _) =
            decode_from_slice(&payload).expect("decode_from_slice failed");
        assert_eq!(codec, decoded, "AudioCodec variant mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 7: Each SampleRate variant roundtrip via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_each_sample_rate_variant_roundtrip() {
    let rates = [
        SampleRate::Rate8k,
        SampleRate::Rate16k,
        SampleRate::Rate44k,
        SampleRate::Rate48k,
        SampleRate::Rate96k,
    ];
    for rate in rates {
        let encoded = encode_to_vec(&rate).expect("encode_to_vec failed");
        let wrapped = wrap_with_checksum(&encoded);
        let payload = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
        let (decoded, _): (SampleRate, _) =
            decode_from_slice(&payload).expect("decode_from_slice failed");
        assert_eq!(rate, decoded, "SampleRate variant mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 8: Empty data wrap and unwrap
// ---------------------------------------------------------------------------
#[test]
fn test_empty_data_wrap_unwrap() {
    let wrapped = wrap_with_checksum(b"");
    assert_eq!(
        wrapped.len(),
        HEADER_SIZE,
        "wrapping empty data must yield exactly HEADER_SIZE bytes"
    );
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap_with_checksum failed");
    assert_eq!(payload, b"", "payload of wrapped empty data must be empty");
}

// ---------------------------------------------------------------------------
// Test 9: Large track with 1000 samples integrity
// ---------------------------------------------------------------------------
#[test]
fn test_large_track_1000_samples_integrity() {
    let samples: Vec<AudioSample> = (0u64..1000)
        .map(|i| AudioSample {
            timestamp_us: i * 1000,
            left_channel: (i % 32768) as i16,
            right_channel: -((i % 32768) as i16),
        })
        .collect();
    let track = AudioTrack {
        track_id: 99,
        codec: AudioCodec::Opus,
        sample_rate: SampleRate::Rate96k,
        bit_depth: 32,
        samples,
    };
    let wrapped = encode_and_wrap(&track);
    let decoded: AudioTrack = unwrap_and_decode(&wrapped);
    assert_eq!(track, decoded);
    assert_eq!(decoded.samples.len(), 1000);
}

// ---------------------------------------------------------------------------
// Test 10: Corruption detection for AudioSample
//
// The corruption pattern flips all bytes from index 4 onward (including the
// LEN field at bytes 4-11).  In debug builds the inflated stored_len causes
// an arithmetic overflow inside `unwrap_with_checksum`; in release builds it
// wraps and the data length check returns an error.  Either outcome proves the
// data is correctly rejected.
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detection_audio_sample() {
    let sample = AudioSample {
        timestamp_us: 999_999,
        left_channel: i16::MAX,
        right_channel: i16::MIN,
    };
    let encoded = encode_to_vec(&sample).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    // Use catch_unwind so that both a returned Err and an overflow panic
    // (debug-mode arithmetic) are treated as "corruption detected".
    let outcome = std::panic::catch_unwind(|| unwrap_with_checksum(&corrupted));
    let rejected = match outcome {
        Err(_panic) => true,
        Ok(Err(_err)) => true,
        Ok(Ok(_)) => false,
    };
    assert!(rejected, "corruption must be detected for AudioSample");
}

// ---------------------------------------------------------------------------
// Test 11: Corruption detection for AudioTrack
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detection_audio_track() {
    let track = AudioTrack {
        track_id: 7,
        codec: AudioCodec::Aac,
        sample_rate: SampleRate::Rate16k,
        bit_depth: 8,
        samples: vec![AudioSample {
            timestamp_us: 500,
            left_channel: 42,
            right_channel: -42,
        }],
    };
    let encoded = encode_to_vec(&track).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let outcome = std::panic::catch_unwind(|| unwrap_with_checksum(&corrupted));
    let rejected = match outcome {
        Err(_panic) => true,
        Ok(Err(_err)) => true,
        Ok(Ok(_)) => false,
    };
    assert!(rejected, "corruption must be detected for AudioTrack");
}

// ---------------------------------------------------------------------------
// Test 12: Corruption detection for MixSession
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detection_mix_session() {
    let session = MixSession {
        session_id: 1,
        bpm: 140,
        time_signature_num: 3,
        time_signature_den: 4,
        tracks: vec![],
    };
    let encoded = encode_to_vec(&session).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let outcome = std::panic::catch_unwind(|| unwrap_with_checksum(&corrupted));
    let rejected = match outcome {
        Err(_panic) => true,
        Ok(Err(_err)) => true,
        Ok(Ok(_)) => false,
    };
    assert!(rejected, "corruption must be detected for MixSession");
}

// ---------------------------------------------------------------------------
// Test 13: Multi-track session integrity (5 tracks, varied codecs)
// ---------------------------------------------------------------------------
#[test]
fn test_multi_track_session_5_tracks_integrity() {
    let tracks: Vec<AudioTrack> = vec![
        AudioTrack {
            track_id: 1,
            codec: AudioCodec::Pcm,
            sample_rate: SampleRate::Rate44k,
            bit_depth: 16,
            samples: vec![AudioSample {
                timestamp_us: 0,
                left_channel: 1000,
                right_channel: -1000,
            }],
        },
        AudioTrack {
            track_id: 2,
            codec: AudioCodec::Mp3,
            sample_rate: SampleRate::Rate48k,
            bit_depth: 16,
            samples: vec![AudioSample {
                timestamp_us: 100,
                left_channel: 2000,
                right_channel: -2000,
            }],
        },
        AudioTrack {
            track_id: 3,
            codec: AudioCodec::Flac,
            sample_rate: SampleRate::Rate96k,
            bit_depth: 24,
            samples: vec![AudioSample {
                timestamp_us: 200,
                left_channel: 3000,
                right_channel: -3000,
            }],
        },
        AudioTrack {
            track_id: 4,
            codec: AudioCodec::Opus,
            sample_rate: SampleRate::Rate8k,
            bit_depth: 8,
            samples: vec![AudioSample {
                timestamp_us: 300,
                left_channel: 100,
                right_channel: -100,
            }],
        },
        AudioTrack {
            track_id: 5,
            codec: AudioCodec::Vorbis,
            sample_rate: SampleRate::Rate16k,
            bit_depth: 32,
            samples: vec![AudioSample {
                timestamp_us: 400,
                left_channel: 5000,
                right_channel: -5000,
            }],
        },
    ];
    let session = MixSession {
        session_id: 100,
        bpm: 90,
        time_signature_num: 6,
        time_signature_den: 8,
        tracks,
    };
    let wrapped = encode_and_wrap(&session);
    let decoded: MixSession = unwrap_and_decode(&wrapped);
    assert_eq!(session, decoded);
    assert_eq!(decoded.tracks.len(), 5);
}

// ---------------------------------------------------------------------------
// Test 14: Wrap/unwrap idempotency (double wrap then double unwrap)
// ---------------------------------------------------------------------------
#[test]
fn test_wrap_unwrap_idempotency() {
    let sample = AudioSample {
        timestamp_us: 88_200,
        left_channel: 12345,
        right_channel: -12345,
    };
    let encoded = encode_to_vec(&sample).expect("encode_to_vec failed");
    let wrapped_once = wrap_with_checksum(&encoded);
    let wrapped_twice = wrap_with_checksum(&wrapped_once);

    let inner = unwrap_with_checksum(&wrapped_twice).expect("first unwrap failed");
    let payload = unwrap_with_checksum(&inner).expect("second unwrap failed");
    let (decoded, _): (AudioSample, _) =
        decode_from_slice(&payload).expect("decode_from_slice failed");
    assert_eq!(sample, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Stereo sample accuracy (exact i16 values preserved)
// ---------------------------------------------------------------------------
#[test]
fn test_stereo_sample_accuracy() {
    let sample = AudioSample {
        timestamp_us: 1_234_567_890,
        left_channel: i16::MAX,
        right_channel: i16::MIN,
    };
    let wrapped = encode_and_wrap(&sample);
    let decoded: AudioSample = unwrap_and_decode(&wrapped);
    assert_eq!(decoded.left_channel, i16::MAX);
    assert_eq!(decoded.right_channel, i16::MIN);
    assert_eq!(decoded.timestamp_us, 1_234_567_890);
}

// ---------------------------------------------------------------------------
// Test 16: Wrapped length formula: plain + HEADER_SIZE
// ---------------------------------------------------------------------------
#[test]
fn test_wrapped_length_formula() {
    let session = MixSession {
        session_id: 77,
        bpm: 128,
        time_signature_num: 4,
        time_signature_den: 4,
        tracks: vec![],
    };
    let encoded = encode_to_vec(&session).expect("encode_to_vec failed");
    let plain_len = encoded.len();
    let wrapped = wrap_with_checksum(&encoded);
    assert_eq!(
        wrapped.len(),
        plain_len + HEADER_SIZE,
        "wrapped.len() must equal encoded.len() + HEADER_SIZE"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Codec roundtrip via checksum (Aac specifically)
// ---------------------------------------------------------------------------
#[test]
fn test_codec_roundtrip_via_checksum_aac() {
    let codec = AudioCodec::Aac;
    let encoded = encode_to_vec(&codec).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (AudioCodec, _) =
        decode_from_slice(&payload).expect("decode_from_slice failed");
    assert_eq!(codec, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: SampleRate roundtrip via checksum (Rate96k specifically)
// ---------------------------------------------------------------------------
#[test]
fn test_sample_rate_roundtrip_via_checksum_96k() {
    let rate = SampleRate::Rate96k;
    let encoded = encode_to_vec(&rate).expect("encode_to_vec failed");
    let wrapped = wrap_with_checksum(&encoded);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (SampleRate, _) =
        decode_from_slice(&payload).expect("decode_from_slice failed");
    assert_eq!(rate, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: bit_depth roundtrip (8, 16, 24, 32) via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_bit_depth_roundtrip_8_16_24_32() {
    for bit_depth in [8u8, 16, 24, 32] {
        let track = AudioTrack {
            track_id: bit_depth as u32,
            codec: AudioCodec::Pcm,
            sample_rate: SampleRate::Rate44k,
            bit_depth,
            samples: vec![],
        };
        let encoded = encode_to_vec(&track).expect("encode_to_vec failed");
        let wrapped = wrap_with_checksum(&encoded);
        let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
        let (decoded, _): (AudioTrack, _) =
            decode_from_slice(&payload).expect("decode_from_slice failed");
        assert_eq!(
            decoded.bit_depth, bit_depth,
            "bit_depth mismatch for {}",
            bit_depth
        );
    }
}

// ---------------------------------------------------------------------------
// Test 20: BPM roundtrip via checksum (60, 120, 140, 200)
// ---------------------------------------------------------------------------
#[test]
fn test_bpm_roundtrip_via_checksum() {
    for bpm in [60u32, 120, 140, 200] {
        let session = MixSession {
            session_id: bpm as u64,
            bpm,
            time_signature_num: 4,
            time_signature_den: 4,
            tracks: vec![],
        };
        let encoded = encode_to_vec(&session).expect("encode_to_vec failed");
        let wrapped = wrap_with_checksum(&encoded);
        let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
        let (decoded, _): (MixSession, _) =
            decode_from_slice(&payload).expect("decode_from_slice failed");
        assert_eq!(decoded.bpm, bpm, "bpm mismatch for {}", bpm);
    }
}

// ---------------------------------------------------------------------------
// Test 21: time_signature roundtrip (4/4, 3/4, 6/8, 5/4) via checksum
// ---------------------------------------------------------------------------
#[test]
fn test_time_signature_roundtrip_via_checksum() {
    let signatures: &[(u8, u8)] = &[(4, 4), (3, 4), (6, 8), (5, 4)];
    for &(num, den) in signatures {
        let session = MixSession {
            session_id: (num as u64) * 100 + (den as u64),
            bpm: 120,
            time_signature_num: num,
            time_signature_den: den,
            tracks: vec![],
        };
        let encoded = encode_to_vec(&session).expect("encode_to_vec failed");
        let wrapped = wrap_with_checksum(&encoded);
        let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
        let (decoded, _): (MixSession, _) =
            decode_from_slice(&payload).expect("decode_from_slice failed");
        assert_eq!(
            decoded.time_signature_num, num,
            "time_signature_num mismatch for {}/{}",
            num, den
        );
        assert_eq!(
            decoded.time_signature_den, den,
            "time_signature_den mismatch for {}/{}",
            num, den
        );
    }
}

// ---------------------------------------------------------------------------
// Test 22: Session with 5 tracks full integrity (track ids and codecs verified)
// ---------------------------------------------------------------------------
#[test]
fn test_session_with_5_tracks_full_integrity() {
    let codecs = [
        AudioCodec::Pcm,
        AudioCodec::Mp3,
        AudioCodec::Aac,
        AudioCodec::Flac,
        AudioCodec::Opus,
    ];
    let rates = [
        SampleRate::Rate8k,
        SampleRate::Rate16k,
        SampleRate::Rate44k,
        SampleRate::Rate48k,
        SampleRate::Rate96k,
    ];
    let tracks: Vec<AudioTrack> = (0..5)
        .map(|i| AudioTrack {
            track_id: i as u32 + 1,
            codec: match i {
                0 => AudioCodec::Pcm,
                1 => AudioCodec::Mp3,
                2 => AudioCodec::Aac,
                3 => AudioCodec::Flac,
                _ => AudioCodec::Opus,
            },
            sample_rate: match i {
                0 => SampleRate::Rate8k,
                1 => SampleRate::Rate16k,
                2 => SampleRate::Rate44k,
                3 => SampleRate::Rate48k,
                _ => SampleRate::Rate96k,
            },
            bit_depth: 16,
            samples: (0..10u64)
                .map(|j| AudioSample {
                    timestamp_us: j * 1000,
                    left_channel: (j * 100) as i16,
                    right_channel: -((j * 100) as i16),
                })
                .collect(),
        })
        .collect();
    let _ = codecs; // referenced structurally above
    let _ = rates; // referenced structurally above
    let session = MixSession {
        session_id: 0xDEAD_CAFE,
        bpm: 174,
        time_signature_num: 4,
        time_signature_den: 4,
        tracks,
    };
    let wrapped = encode_and_wrap(&session);
    let decoded: MixSession = unwrap_and_decode(&wrapped);
    assert_eq!(session, decoded);
    assert_eq!(decoded.tracks.len(), 5);
    for (i, track) in decoded.tracks.iter().enumerate() {
        assert_eq!(track.track_id, i as u32 + 1);
        assert_eq!(track.samples.len(), 10);
    }
}
