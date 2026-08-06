//! Decode implementations for tuples (1-16 elements)

use super::{BorrowDecode, BorrowDecoder, Decode, Decoder};
use crate::error::Error;

// Implement Decode for tuples up to 16 elements, generic over the decode context
// Following bincode's pattern of direct implementations (not macros)

impl<Context, T0: Decode<Context>> Decode<Context> for (T0,) {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::decode(decoder)?,))
    }
}

impl<Context, T0: Decode<Context>, T1: Decode<Context>> Decode<Context> for (T0, T1) {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::decode(decoder)?, T1::decode(decoder)?))
    }
}

impl<Context, T0: Decode<Context>, T1: Decode<Context>, T2: Decode<Context>> Decode<Context>
    for (T0, T1, T2)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6, T7)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
        T9: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
        T9: Decode<Context>,
        T10: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
        T9: Decode<Context>,
        T10: Decode<Context>,
        T11: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
        T9: Decode<Context>,
        T10: Decode<Context>,
        T11: Decode<Context>,
        T12: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
        T9: Decode<Context>,
        T10: Decode<Context>,
        T11: Decode<Context>,
        T12: Decode<Context>,
        T13: Decode<Context>,
    > Decode<Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
            T13::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
        T9: Decode<Context>,
        T10: Decode<Context>,
        T11: Decode<Context>,
        T12: Decode<Context>,
        T13: Decode<Context>,
        T14: Decode<Context>,
    > Decode<Context>
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
    )
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
            T13::decode(decoder)?,
            T14::decode(decoder)?,
        ))
    }
}

impl<
        Context,
        T0: Decode<Context>,
        T1: Decode<Context>,
        T2: Decode<Context>,
        T3: Decode<Context>,
        T4: Decode<Context>,
        T5: Decode<Context>,
        T6: Decode<Context>,
        T7: Decode<Context>,
        T8: Decode<Context>,
        T9: Decode<Context>,
        T10: Decode<Context>,
        T11: Decode<Context>,
        T12: Decode<Context>,
        T13: Decode<Context>,
        T14: Decode<Context>,
        T15: Decode<Context>,
    > Decode<Context>
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    )
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
            T13::decode(decoder)?,
            T14::decode(decoder)?,
            T15::decode(decoder)?,
        ))
    }
}

// ===== BorrowDecode for tuples (1-16 elements) =====

impl<'de, Context, T0: BorrowDecode<'de, Context>> BorrowDecode<'de, Context> for (T0,) {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((T0::borrow_decode(decoder)?,))
    }
}

impl<'de, Context, T0: BorrowDecode<'de, Context>, T1: BorrowDecode<'de, Context>>
    BorrowDecode<'de, Context> for (T0, T1)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((T0::borrow_decode(decoder)?, T1::borrow_decode(decoder)?))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6, T7)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
        T9: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
            T9::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
        T9: BorrowDecode<'de, Context>,
        T10: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
            T9::borrow_decode(decoder)?,
            T10::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
        T9: BorrowDecode<'de, Context>,
        T10: BorrowDecode<'de, Context>,
        T11: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
            T9::borrow_decode(decoder)?,
            T10::borrow_decode(decoder)?,
            T11::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
        T9: BorrowDecode<'de, Context>,
        T10: BorrowDecode<'de, Context>,
        T11: BorrowDecode<'de, Context>,
        T12: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
            T9::borrow_decode(decoder)?,
            T10::borrow_decode(decoder)?,
            T11::borrow_decode(decoder)?,
            T12::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
        T9: BorrowDecode<'de, Context>,
        T10: BorrowDecode<'de, Context>,
        T11: BorrowDecode<'de, Context>,
        T12: BorrowDecode<'de, Context>,
        T13: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
            T9::borrow_decode(decoder)?,
            T10::borrow_decode(decoder)?,
            T11::borrow_decode(decoder)?,
            T12::borrow_decode(decoder)?,
            T13::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
        T9: BorrowDecode<'de, Context>,
        T10: BorrowDecode<'de, Context>,
        T11: BorrowDecode<'de, Context>,
        T12: BorrowDecode<'de, Context>,
        T13: BorrowDecode<'de, Context>,
        T14: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context>
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
    )
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
            T9::borrow_decode(decoder)?,
            T10::borrow_decode(decoder)?,
            T11::borrow_decode(decoder)?,
            T12::borrow_decode(decoder)?,
            T13::borrow_decode(decoder)?,
            T14::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        Context,
        T0: BorrowDecode<'de, Context>,
        T1: BorrowDecode<'de, Context>,
        T2: BorrowDecode<'de, Context>,
        T3: BorrowDecode<'de, Context>,
        T4: BorrowDecode<'de, Context>,
        T5: BorrowDecode<'de, Context>,
        T6: BorrowDecode<'de, Context>,
        T7: BorrowDecode<'de, Context>,
        T8: BorrowDecode<'de, Context>,
        T9: BorrowDecode<'de, Context>,
        T10: BorrowDecode<'de, Context>,
        T11: BorrowDecode<'de, Context>,
        T12: BorrowDecode<'de, Context>,
        T13: BorrowDecode<'de, Context>,
        T14: BorrowDecode<'de, Context>,
        T15: BorrowDecode<'de, Context>,
    > BorrowDecode<'de, Context>
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    )
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
            T3::borrow_decode(decoder)?,
            T4::borrow_decode(decoder)?,
            T5::borrow_decode(decoder)?,
            T6::borrow_decode(decoder)?,
            T7::borrow_decode(decoder)?,
            T8::borrow_decode(decoder)?,
            T9::borrow_decode(decoder)?,
            T10::borrow_decode(decoder)?,
            T11::borrow_decode(decoder)?,
            T12::borrow_decode(decoder)?,
            T13::borrow_decode(decoder)?,
            T14::borrow_decode(decoder)?,
            T15::borrow_decode(decoder)?,
        ))
    }
}
