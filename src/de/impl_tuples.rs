//! Decode implementations for tuples (1-16 elements)

use super::{BorrowDecode, BorrowDecoder, Decode, Decoder};
use crate::error::Error;

// Implement Decode for tuples up to 16 elements
// Following bincode's pattern of direct implementations (not macros)

impl<T0: Decode> Decode for (T0,) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::decode(decoder)?,))
    }
}

impl<T0: Decode, T1: Decode> Decode for (T0, T1) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::decode(decoder)?, T1::decode(decoder)?))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode> Decode for (T0, T1, T2) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
        ))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode> Decode for (T0, T1, T2, T3) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
        ))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode, T4: Decode> Decode for (T0, T1, T2, T3, T4) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
        ))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode, T4: Decode, T5: Decode> Decode
    for (T0, T1, T2, T3, T4, T5)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode, T4: Decode, T5: Decode, T6: Decode> Decode
    for (T0, T1, T2, T3, T4, T5, T6)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
        T13: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
        T13: Decode,
        T14: Decode,
    > Decode
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
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
        T13: Decode,
        T14: Decode,
        T15: Decode,
    > Decode
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
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<'de, T0: BorrowDecode<'de>> BorrowDecode<'de> for (T0,) {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::borrow_decode(decoder)?,))
    }
}

impl<'de, T0: BorrowDecode<'de>, T1: BorrowDecode<'de>> BorrowDecode<'de> for (T0, T1) {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::borrow_decode(decoder)?, T1::borrow_decode(decoder)?))
    }
}

impl<'de, T0: BorrowDecode<'de>, T1: BorrowDecode<'de>, T2: BorrowDecode<'de>> BorrowDecode<'de>
    for (T0, T1, T2)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::borrow_decode(decoder)?,
            T1::borrow_decode(decoder)?,
            T2::borrow_decode(decoder)?,
        ))
    }
}

impl<
        'de,
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6, T7)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
        T9: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
        T9: BorrowDecode<'de>,
        T10: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
        T9: BorrowDecode<'de>,
        T10: BorrowDecode<'de>,
        T11: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
        T9: BorrowDecode<'de>,
        T10: BorrowDecode<'de>,
        T11: BorrowDecode<'de>,
        T12: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
        T9: BorrowDecode<'de>,
        T10: BorrowDecode<'de>,
        T11: BorrowDecode<'de>,
        T12: BorrowDecode<'de>,
        T13: BorrowDecode<'de>,
    > BorrowDecode<'de> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
        T9: BorrowDecode<'de>,
        T10: BorrowDecode<'de>,
        T11: BorrowDecode<'de>,
        T12: BorrowDecode<'de>,
        T13: BorrowDecode<'de>,
        T14: BorrowDecode<'de>,
    > BorrowDecode<'de>
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
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
        T0: BorrowDecode<'de>,
        T1: BorrowDecode<'de>,
        T2: BorrowDecode<'de>,
        T3: BorrowDecode<'de>,
        T4: BorrowDecode<'de>,
        T5: BorrowDecode<'de>,
        T6: BorrowDecode<'de>,
        T7: BorrowDecode<'de>,
        T8: BorrowDecode<'de>,
        T9: BorrowDecode<'de>,
        T10: BorrowDecode<'de>,
        T11: BorrowDecode<'de>,
        T12: BorrowDecode<'de>,
        T13: BorrowDecode<'de>,
        T14: BorrowDecode<'de>,
        T15: BorrowDecode<'de>,
    > BorrowDecode<'de>
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
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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
