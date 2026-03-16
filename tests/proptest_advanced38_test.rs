//! Advanced property-based tests (set 38) using proptest.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Theme: Calendar / scheduling — DayOfWeek, TimeSlot, Event, Schedule.

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
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimeSlot {
    hour: u8,
    minute: u8,
    duration_min: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Event {
    id: u64,
    title: String,
    day: DayOfWeek,
    slot: TimeSlot,
    recurring: bool,
    attendee_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Schedule {
    owner_id: u64,
    events: Vec<Event>,
    timezone_offset_min: i16,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn arb_day_of_week() -> impl Strategy<Value = DayOfWeek> {
    prop_oneof![
        Just(DayOfWeek::Monday),
        Just(DayOfWeek::Tuesday),
        Just(DayOfWeek::Wednesday),
        Just(DayOfWeek::Thursday),
        Just(DayOfWeek::Friday),
        Just(DayOfWeek::Saturday),
        Just(DayOfWeek::Sunday),
    ]
}

fn arb_timeslot() -> impl Strategy<Value = TimeSlot> {
    (0u8..24u8, 0u8..60u8, any::<u16>()).prop_map(|(hour, minute, duration_min)| TimeSlot {
        hour,
        minute,
        duration_min,
    })
}

fn arb_event() -> impl Strategy<Value = Event> {
    (
        any::<u64>(),
        "[a-zA-Z0-9 ]{0,40}",
        arb_day_of_week(),
        arb_timeslot(),
        any::<bool>(),
        any::<u16>(),
    )
        .prop_map(|(id, title, day, slot, recurring, attendee_count)| Event {
            id,
            title,
            day,
            slot,
            recurring,
            attendee_count,
        })
}

fn arb_schedule() -> impl Strategy<Value = Schedule> {
    (
        any::<u64>(),
        prop::collection::vec(arb_event(), 0..8usize),
        any::<i16>(),
    )
        .prop_map(|(owner_id, events, timezone_offset_min)| Schedule {
            owner_id,
            events,
            timezone_offset_min,
        })
}

// ── 1. TimeSlot roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_timeslot_roundtrip() {
    proptest!(|(slot in arb_timeslot())| {
        let enc = encode_to_vec(&slot).expect("encode TimeSlot failed");
        let (decoded, _): (TimeSlot, usize) =
            decode_from_slice(&enc).expect("decode TimeSlot failed");
        prop_assert_eq!(slot, decoded);
    });
}

// ── 2. DayOfWeek roundtrip ───────────────────────────────────────────────────

#[test]
fn test_day_of_week_roundtrip() {
    proptest!(|(day in arb_day_of_week())| {
        let enc = encode_to_vec(&day).expect("encode DayOfWeek failed");
        let (decoded, _): (DayOfWeek, usize) =
            decode_from_slice(&enc).expect("decode DayOfWeek failed");
        prop_assert_eq!(day, decoded);
    });
}

// ── 3. Event roundtrip ───────────────────────────────────────────────────────

#[test]
fn test_event_roundtrip() {
    proptest!(|(event in arb_event())| {
        let enc = encode_to_vec(&event).expect("encode Event failed");
        let (decoded, _): (Event, usize) =
            decode_from_slice(&enc).expect("decode Event failed");
        prop_assert_eq!(event, decoded);
    });
}

// ── 4. Schedule roundtrip ────────────────────────────────────────────────────

#[test]
fn test_schedule_roundtrip() {
    proptest!(|(schedule in arb_schedule())| {
        let enc = encode_to_vec(&schedule).expect("encode Schedule failed");
        let (decoded, _): (Schedule, usize) =
            decode_from_slice(&enc).expect("decode Schedule failed");
        prop_assert_eq!(schedule, decoded);
    });
}

// ── 5. consumed bytes equal encoded length ───────────────────────────────────

#[test]
fn test_event_consumed_bytes_equals_encoded_length() {
    proptest!(|(event in arb_event())| {
        let enc = encode_to_vec(&event).expect("encode Event failed");
        let (_, consumed): (Event, usize) =
            decode_from_slice(&enc).expect("decode Event failed");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal encoded length");
    });
}

// ── 6. Schedule consumed bytes equal encoded length ──────────────────────────

#[test]
fn test_schedule_consumed_bytes_equals_encoded_length() {
    proptest!(|(schedule in arb_schedule())| {
        let enc = encode_to_vec(&schedule).expect("encode Schedule failed");
        let (_, consumed): (Schedule, usize) =
            decode_from_slice(&enc).expect("decode Schedule failed");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal encoded length");
    });
}

// ── 7. encoding is deterministic for Event ───────────────────────────────────

#[test]
fn test_event_encoding_deterministic() {
    proptest!(|(event in arb_event())| {
        let enc1 = encode_to_vec(&event).expect("first encode Event failed");
        let enc2 = encode_to_vec(&event).expect("second encode Event failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 8. encoding is deterministic for Schedule ────────────────────────────────

#[test]
fn test_schedule_encoding_deterministic() {
    proptest!(|(schedule in arb_schedule())| {
        let enc1 = encode_to_vec(&schedule).expect("first encode Schedule failed");
        let enc2 = encode_to_vec(&schedule).expect("second encode Schedule failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 9. re-encoding decoded Event yields identical bytes ──────────────────────

#[test]
fn test_event_reencode_idempotent() {
    proptest!(|(event in arb_event())| {
        let enc1 = encode_to_vec(&event).expect("first encode Event failed");
        let (decoded, _): (Event, usize) =
            decode_from_slice(&enc1).expect("decode Event failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Event failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 10. re-encoding decoded Schedule yields identical bytes ──────────────────

#[test]
fn test_schedule_reencode_idempotent() {
    proptest!(|(schedule in arb_schedule())| {
        let enc1 = encode_to_vec(&schedule).expect("first encode Schedule failed");
        let (decoded, _): (Schedule, usize) =
            decode_from_slice(&enc1).expect("decode Schedule failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Schedule failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 11. Vec<Event> roundtrip ─────────────────────────────────────────────────

#[test]
fn test_vec_event_roundtrip() {
    proptest!(|(events in prop::collection::vec(arb_event(), 0..10usize))| {
        let enc = encode_to_vec(&events).expect("encode Vec<Event> failed");
        let (decoded, _): (Vec<Event>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Event> failed");
        prop_assert_eq!(events, decoded);
    });
}

// ── 12. Vec<Schedule> roundtrip ──────────────────────────────────────────────

#[test]
fn test_vec_schedule_roundtrip() {
    proptest!(|(schedules in prop::collection::vec(arb_schedule(), 0..5usize))| {
        let enc = encode_to_vec(&schedules).expect("encode Vec<Schedule> failed");
        let (decoded, _): (Vec<Schedule>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Schedule> failed");
        prop_assert_eq!(schedules, decoded);
    });
}

// ── 13. all DayOfWeek variants encode to distinct bytes ──────────────────────

#[test]
fn test_all_day_of_week_variants_distinct() {
    let variants = vec![
        DayOfWeek::Monday,
        DayOfWeek::Tuesday,
        DayOfWeek::Wednesday,
        DayOfWeek::Thursday,
        DayOfWeek::Friday,
        DayOfWeek::Saturday,
        DayOfWeek::Sunday,
    ];
    let encoded: Vec<Vec<u8>> = variants
        .iter()
        .map(|d| encode_to_vec(d).expect("encode DayOfWeek variant failed"))
        .collect();
    // Each variant must produce a unique byte sequence.
    for i in 0..encoded.len() {
        for j in (i + 1)..encoded.len() {
            assert_ne!(
                encoded[i], encoded[j],
                "DayOfWeek variants {i} and {j} must have distinct encodings"
            );
        }
    }
}

// ── 14. Option<Event> Some roundtrip ─────────────────────────────────────────

#[test]
fn test_option_event_some_roundtrip() {
    proptest!(|(event in arb_event())| {
        let opt: Option<Event> = Some(event);
        let enc = encode_to_vec(&opt).expect("encode Option<Event> Some failed");
        let (decoded, _): (Option<Event>, usize) =
            decode_from_slice(&enc).expect("decode Option<Event> Some failed");
        prop_assert_eq!(opt, decoded);
    });
}

// ── 15. Option<Event> None roundtrip ─────────────────────────────────────────

#[test]
fn test_option_event_none_roundtrip() {
    let opt: Option<Event> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<Event> None failed");
    let (decoded, _): (Option<Event>, usize) =
        decode_from_slice(&enc).expect("decode Option<Event> None failed");
    assert_eq!(opt, decoded, "Option<Event> None must roundtrip");
}

// ── 16. Option<Schedule> roundtrip ───────────────────────────────────────────

#[test]
fn test_option_schedule_roundtrip() {
    proptest!(|(schedule in prop::option::of(arb_schedule()))| {
        let enc = encode_to_vec(&schedule).expect("encode Option<Schedule> failed");
        let (decoded, _): (Option<Schedule>, usize) =
            decode_from_slice(&enc).expect("decode Option<Schedule> failed");
        prop_assert_eq!(schedule, decoded);
    });
}

// ── 17. TimeSlot boundary values: hour=0, minute=0, duration=0 ───────────────

#[test]
fn test_timeslot_boundary_zero() {
    let slot = TimeSlot {
        hour: 0,
        minute: 0,
        duration_min: 0,
    };
    let enc = encode_to_vec(&slot).expect("encode boundary TimeSlot zero failed");
    let (decoded, consumed): (TimeSlot, usize) =
        decode_from_slice(&enc).expect("decode boundary TimeSlot zero failed");
    assert_eq!(slot, decoded, "boundary zero TimeSlot must roundtrip");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must match encoded length"
    );
}

// ── 18. TimeSlot boundary values: hour=23, minute=59, duration=u16::MAX ──────

#[test]
fn test_timeslot_boundary_max() {
    let slot = TimeSlot {
        hour: 23,
        minute: 59,
        duration_min: u16::MAX,
    };
    let enc = encode_to_vec(&slot).expect("encode boundary TimeSlot max failed");
    let (decoded, consumed): (TimeSlot, usize) =
        decode_from_slice(&enc).expect("decode boundary TimeSlot max failed");
    assert_eq!(slot, decoded, "boundary max TimeSlot must roundtrip");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must match encoded length"
    );
}

// ── 19. Schedule with empty events vec roundtrip ─────────────────────────────

#[test]
fn test_schedule_empty_events_roundtrip() {
    proptest!(|(owner_id: u64, timezone_offset_min: i16)| {
        let schedule = Schedule {
            owner_id,
            events: vec![],
            timezone_offset_min,
        };
        let enc = encode_to_vec(&schedule).expect("encode empty-events Schedule failed");
        let (decoded, consumed): (Schedule, usize) =
            decode_from_slice(&enc).expect("decode empty-events Schedule failed");
        prop_assert_eq!(schedule, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 20. timezone_offset_min boundary values: i16::MIN, 0, i16::MAX ───────────

#[test]
fn test_schedule_timezone_offset_boundaries() {
    for tz in [i16::MIN, 0i16, i16::MAX] {
        let schedule = Schedule {
            owner_id: 0,
            events: vec![],
            timezone_offset_min: tz,
        };
        let enc = encode_to_vec(&schedule).expect("encode timezone boundary Schedule failed");
        let (decoded, consumed): (Schedule, usize) =
            decode_from_slice(&enc).expect("decode timezone boundary Schedule failed");
        assert_eq!(schedule, decoded, "timezone boundary {tz} must roundtrip");
        assert_eq!(consumed, enc.len(), "consumed bytes must match for tz={tz}");
    }
}

// ── 21. Event with recurring=true vs recurring=false have different bytes ─────

#[test]
fn test_event_recurring_flag_affects_encoding() {
    proptest!(|(
        id: u64,
        title in "[a-zA-Z0-9]{1,20}",
        day in arb_day_of_week(),
        slot in arb_timeslot(),
        attendee_count: u16,
    )| {
        let ev_true = Event {
            id,
            title: title.clone(),
            day: day.clone(),
            slot: slot.clone(),
            recurring: true,
            attendee_count,
        };
        let ev_false = Event {
            id,
            title,
            day,
            slot,
            recurring: false,
            attendee_count,
        };
        let enc_true = encode_to_vec(&ev_true).expect("encode recurring=true failed");
        let enc_false = encode_to_vec(&ev_false).expect("encode recurring=false failed");
        prop_assert_ne!(enc_true, enc_false, "recurring flag must affect encoding");
    });
}

// ── 22. Schedule event count preserved after roundtrip ───────────────────────

#[test]
fn test_schedule_event_count_preserved() {
    proptest!(|(schedule in arb_schedule())| {
        let original_count = schedule.events.len();
        let enc = encode_to_vec(&schedule).expect("encode Schedule failed");
        let (decoded, _): (Schedule, usize) =
            decode_from_slice(&enc).expect("decode Schedule failed");
        prop_assert_eq!(
            original_count,
            decoded.events.len(),
            "event count must be preserved after roundtrip"
        );
    });
}
