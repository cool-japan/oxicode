//! `#[oxicode(decode_context = "...")]` / `#[oxicode(context_generic)]` —
//! derived types under a non-unit decode context.
//!
//! Before this existed the derive always emitted `fn decode<D: Decoder<Context = ()>>`,
//! so `decode_from_slice_with_context` was dead API for every derived type and a
//! derived struct could never be a field of a hand-written context-using type.
//!
//! The wire format is not affected by any of this: a context is a decode-time
//! side channel, never a byte on the wire. The final test in this file pins
//! that down by comparing a context-generic decode against the plain one.

use oxicode::{config, BorrowDecode, Decode, Encode};

/// A stand-in for the real reason contexts exist (an arena, an interner, a
/// resource table): decoding records into it, and it is not `()`.
#[derive(Default, Debug, PartialEq, Eq)]
struct Tracker {
    decoded_values: Vec<u64>,
}

// ---------------------------------------------------------------------------
// A hand-written context-using type, to prove derived types compose with one
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
struct Recorded(u64);

impl Encode for Recorded {
    fn encode<E: oxicode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), oxicode::Error> {
        self.0.encode(encoder)
    }
}

impl Decode<Tracker> for Recorded {
    fn decode<D: oxicode::de::Decoder<Context = Tracker>>(
        decoder: &mut D,
    ) -> Result<Self, oxicode::Error> {
        let value = u64::decode(decoder)?;
        decoder.context().decoded_values.push(value);
        Ok(Recorded(value))
    }
}

impl<'de> BorrowDecode<'de, Tracker> for Recorded {
    fn borrow_decode<D: oxicode::de::BorrowDecoder<'de, Context = Tracker>>(
        decoder: &mut D,
    ) -> Result<Self, oxicode::Error> {
        <Recorded as Decode<Tracker>>::decode(decoder)
    }
}

// ---------------------------------------------------------------------------
// Derived types opting into a context
// ---------------------------------------------------------------------------

/// A concrete context: this type decodes only under `Tracker`.
#[derive(Encode, Decode, Debug, PartialEq, Eq)]
#[oxicode(decode_context = "Tracker")]
struct Envelope {
    id: u32,
    // A hand-written `Decode<Tracker>` field — impossible before this change.
    payload: Recorded,
    // Primitives and std containers must still resolve under a non-unit
    // context, which is what the blanket `impl<__Ctx> Decode<__Ctx>` impls buy.
    label: String,
    flags: Vec<u8>,
    nested: Option<(u8, i64)>,
}

/// A *generic* context: the derive introduces the context parameter itself, so
/// the impl reads `impl<__Ctx> Decode<__Ctx> for Anywhere` and the type decodes
/// under every context.
#[derive(Encode, Decode, Debug, PartialEq, Eq)]
#[oxicode(context_generic)]
struct Anywhere {
    seq: u32,
    name: String,
}

/// A context that is also a generic parameter of the container itself.
///
/// `Encode` is hand-written here only because the encode derive bounds every
/// type parameter by `Encode`, which a context type has no reason to satisfy.
#[derive(Decode, Debug, PartialEq, Eq)]
#[oxicode(decode_context = "Ctx")]
struct WithOwnParam<Ctx, T> {
    value: T,
    #[oxicode(skip)]
    marker: core::marker::PhantomData<Ctx>,
}

impl<Ctx, T: Encode> Encode for WithOwnParam<Ctx, T> {
    fn encode<E: oxicode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), oxicode::Error> {
        self.value.encode(encoder)
    }
}

/// Enums take the attribute too.
#[derive(Encode, Decode, Debug, PartialEq, Eq)]
#[oxicode(decode_context = "Tracker")]
enum Event {
    Nothing,
    One(Recorded),
    Named { at: u64, what: String },
}

/// Borrow-decoding under a context.
#[derive(Encode, BorrowDecode, Debug, PartialEq, Eq)]
#[oxicode(borrow_decode_context = "Tracker")]
struct BorrowedEnvelope<'a> {
    tag: u32,
    text: &'a str,
    payload: Recorded,
}

/// The historical default: no attribute at all, so `Context = ()`.
#[derive(Encode, Decode, Debug, PartialEq, Eq)]
struct PlainOldStruct {
    id: u32,
    label: String,
    flags: Vec<u8>,
    nested: Option<(u8, i64)>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn derived_struct_decodes_under_a_concrete_context() {
    let value = Envelope {
        id: 7,
        payload: Recorded(4242),
        label: "envelope".to_string(),
        flags: vec![9, 8, 7],
        nested: Some((3, -1)),
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");

    let (decoded, read): (Envelope, usize) =
        oxicode::decode_from_slice_with_context(&bytes, config::standard(), Tracker::default())
            .expect("decode with context");

    assert_eq!(decoded, value);
    assert_eq!(read, bytes.len());
}

#[test]
fn the_context_actually_reaches_the_field_impls() {
    // Not just "it compiles": the context must be the *same* object the field
    // impls mutate, otherwise threading it through would be cosmetic.
    let value = Event::One(Recorded(1234));
    let bytes = oxicode::encode_to_vec(&value).expect("encode");

    let reader = oxicode::de::SliceReader::new(&bytes);
    let mut decoder =
        oxicode::de::DecoderImpl::with_context(reader, config::standard(), Tracker::default());
    let decoded = <Event as Decode<Tracker>>::decode(&mut decoder).expect("decode");

    assert_eq!(decoded, value);
    assert_eq!(decoder.get_context().decoded_values, vec![1234]);
}

#[test]
fn enum_variants_round_trip_under_a_context() {
    for value in [
        Event::Nothing,
        Event::One(Recorded(5)),
        Event::Named {
            at: 99,
            what: "boom".to_string(),
        },
    ] {
        let bytes = oxicode::encode_to_vec(&value).expect("encode");
        let (decoded, _): (Event, usize) =
            oxicode::decode_from_slice_with_context(&bytes, config::standard(), Tracker::default())
                .expect("decode with context");
        assert_eq!(decoded, value);
    }
}

#[test]
fn context_generic_struct_decodes_under_any_context() {
    let value = Anywhere {
        seq: 3,
        name: "any".to_string(),
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");

    // ... under the unit context,
    let (as_unit, _): (Anywhere, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode under ()");
    assert_eq!(as_unit, value);

    // ... under a foreign context,
    let (as_tracked, _): (Anywhere, usize) =
        oxicode::decode_from_slice_with_context(&bytes, config::standard(), Tracker::default())
            .expect("decode under Tracker");
    assert_eq!(as_tracked, value);

    // ... and under yet another one.
    let (as_string_ctx, _): (Anywhere, usize) =
        oxicode::decode_from_slice_with_context(&bytes, config::standard(), String::new())
            .expect("decode under String");
    assert_eq!(as_string_ctx, value);
}

#[test]
fn context_may_be_a_generic_parameter_of_the_container() {
    let value: WithOwnParam<Tracker, u32> = WithOwnParam {
        value: 77,
        marker: core::marker::PhantomData,
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");

    let (decoded, _): (WithOwnParam<Tracker, u32>, usize) =
        oxicode::decode_from_slice_with_context(&bytes, config::standard(), Tracker::default())
            .expect("decode with context");
    assert_eq!(decoded, value);
}

#[test]
fn borrow_decode_derive_honours_the_context() {
    let owned_text = "borrowed".to_string();
    let value = BorrowedEnvelope {
        tag: 11,
        text: &owned_text,
        payload: Recorded(63),
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");

    let reader = oxicode::de::SliceReader::new(&bytes);
    let mut decoder =
        oxicode::de::DecoderImpl::with_context(reader, config::standard(), Tracker::default());
    let decoded = <BorrowedEnvelope<'_> as BorrowDecode<'_, Tracker>>::borrow_decode(&mut decoder)
        .expect("borrow decode with context");

    assert_eq!(decoded, value);
    assert_eq!(decoder.get_context().decoded_values, vec![63]);
}

#[test]
fn default_derive_still_uses_the_unit_context() {
    // The no-attribute path must be untouched: `Decode` (i.e. `Decode<()>`)
    // resolves, and the plain entry points keep working.
    let value = PlainOldStruct {
        id: 1,
        label: "plain".to_string(),
        flags: vec![1, 2],
        nested: None,
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");
    let (decoded, _): (PlainOldStruct, usize) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded, value);

    fn assert_unit_decode<T: Decode<()>>() {}
    assert_unit_decode::<PlainOldStruct>();
}

#[test]
fn a_decode_context_does_not_change_the_wire_format() {
    // `Envelope` and `PlainOldStruct` have the same non-context fields in the
    // same order; encoding them must produce identical bytes, and the
    // context-carrying type must decode bytes produced without any context.
    let plain = PlainOldStruct {
        id: 7,
        label: "envelope".to_string(),
        flags: vec![9, 8, 7],
        nested: Some((3, -1)),
    };
    let with_ctx = Envelope {
        id: 7,
        payload: Recorded(4242),
        label: "envelope".to_string(),
        flags: vec![9, 8, 7],
        nested: Some((3, -1)),
    };

    let plain_bytes = oxicode::encode_to_vec(&plain).expect("encode");
    let ctx_bytes = oxicode::encode_to_vec(&with_ctx).expect("encode");

    // The only difference is the extra `payload: u64` after `id`.
    let payload_bytes = oxicode::encode_to_vec(&4242u64).expect("encode");
    let mut expected = Vec::new();
    expected.extend_from_slice(&oxicode::encode_to_vec(&7u32).expect("encode"));
    expected.extend_from_slice(&payload_bytes);
    expected
        .extend_from_slice(&plain_bytes[oxicode::encode_to_vec(&7u32).expect("encode").len()..]);
    assert_eq!(ctx_bytes, expected);

    // And the context-generic type is byte-identical to a unit-context one.
    let anywhere = Anywhere {
        seq: 3,
        name: "any".to_string(),
    };
    let anywhere_bytes = oxicode::encode_to_vec(&anywhere).expect("encode");
    let mut manual = Vec::new();
    manual.extend_from_slice(&oxicode::encode_to_vec(&3u32).expect("encode"));
    manual.extend_from_slice(&oxicode::encode_to_vec(&"any".to_string()).expect("encode"));
    assert_eq!(anywhere_bytes, manual);
}

// ---------------------------------------------------------------------------
// The context entry points that pair with the derive
// ---------------------------------------------------------------------------

#[test]
fn borrow_decode_from_slice_with_context_entry_point() {
    let owned_text = "zero copy".to_string();
    let value = BorrowedEnvelope {
        tag: 5,
        text: &owned_text,
        payload: Recorded(21),
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");

    let (decoded, read): (BorrowedEnvelope<'_>, usize) =
        oxicode::borrow_decode_from_slice_with_context(
            &bytes,
            config::standard(),
            Tracker::default(),
        )
        .expect("borrow decode with context");

    assert_eq!(decoded, value);
    assert_eq!(read, bytes.len());
}

#[test]
fn decode_from_std_read_with_context_entry_point() {
    let value = Envelope {
        id: 2,
        payload: Recorded(8),
        label: "io".to_string(),
        flags: vec![],
        nested: None,
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");

    let decoded: Envelope = oxicode::decode_from_std_read_with_context(
        std::io::Cursor::new(bytes),
        config::standard(),
        Tracker::default(),
    )
    .expect("decode from reader with context");
    assert_eq!(decoded, value);
}

#[test]
fn decode_from_de_reader_with_context_combines_a_budget_and_a_context() {
    // The general entry point: a budgeted IO reader *and* a context together.
    let value = Envelope {
        id: 3,
        payload: Recorded(9),
        label: "budgeted".to_string(),
        flags: vec![4, 5],
        nested: Some((1, 2)),
    };
    let bytes = oxicode::encode_to_vec(&value).expect("encode");
    let len = bytes.len();

    let reader = oxicode::de::IoReader::with_limit(std::io::Cursor::new(bytes), len);
    let decoded: Envelope =
        oxicode::decode_from_de_reader_with_context(reader, config::standard(), Tracker::default())
            .expect("decode with budget and context");
    assert_eq!(decoded, value);

    // And the budget still bites: a forged length inside a context decode is
    // rejected rather than pre-allocated.
    let forged = [253u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let reader = oxicode::de::IoReader::with_limit(std::io::Cursor::new(forged), forged.len());
    let err = oxicode::decode_from_de_reader_with_context::<Tracker, String, _, _>(
        reader,
        config::standard(),
        Tracker::default(),
    )
    .expect_err("forged length must be rejected under a context too");
    assert!(matches!(err, oxicode::Error::UnexpectedEnd { .. }));
}
