//! Encode/Decode implementations for atomic types
//!
//! These implementations read/write the inner values using relaxed ordering.

use crate::{
    de::{BorrowDecode, BorrowDecoder, Decode, Decoder},
    enc::{Encode, Encoder},
    error::Error,
};
use core::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering};

// AtomicBool is available on all platforms with target_has_atomic = "8"
#[cfg(target_has_atomic = "8")]
impl Encode for AtomicBool {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.load(Ordering::Relaxed).encode(encoder)
    }
}

#[cfg(target_has_atomic = "8")]
impl Decode for AtomicBool {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(AtomicBool::new(bool::decode(decoder)?))
    }
}

#[cfg(target_has_atomic = "8")]
impl<'__de> BorrowDecode<'__de> for AtomicBool {
    fn borrow_decode<D: BorrowDecoder<'__de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(AtomicBool::new(bool::decode(decoder)?))
    }
}

// Macro to implement Encode/Decode/BorrowDecode for atomic integer types
macro_rules! impl_atomic_int {
    ($atomic:ty, $inner:ty, $cfg:meta) => {
        #[cfg($cfg)]
        impl Encode for $atomic {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
                self.load(Ordering::Relaxed).encode(encoder)
            }
        }

        #[cfg($cfg)]
        impl Decode for $atomic {
            fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
                Ok(<$atomic>::new(<$inner>::decode(decoder)?))
            }
        }

        #[cfg($cfg)]
        impl<'__de> BorrowDecode<'__de> for $atomic {
            fn borrow_decode<D: BorrowDecoder<'__de, Context = ()>>(
                decoder: &mut D,
            ) -> Result<Self, Error> {
                Ok(<$atomic>::new(<$inner>::decode(decoder)?))
            }
        }
    };
}

// AtomicU8 and AtomicI8 require target_has_atomic = "8"
#[cfg(target_has_atomic = "8")]
use core::sync::atomic::{AtomicI8, AtomicU8};
impl_atomic_int!(AtomicU8, u8, target_has_atomic = "8");
impl_atomic_int!(AtomicI8, i8, target_has_atomic = "8");

// AtomicU16 and AtomicI16 require target_has_atomic = "16"
#[cfg(target_has_atomic = "16")]
use core::sync::atomic::{AtomicI16, AtomicU16};
impl_atomic_int!(AtomicU16, u16, target_has_atomic = "16");
impl_atomic_int!(AtomicI16, i16, target_has_atomic = "16");

// AtomicU32 and AtomicI32 require target_has_atomic = "32"
#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicI32, AtomicU32};
impl_atomic_int!(AtomicU32, u32, target_has_atomic = "32");
impl_atomic_int!(AtomicI32, i32, target_has_atomic = "32");

// AtomicU64 and AtomicI64 require target_has_atomic = "64"
#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};
impl_atomic_int!(AtomicU64, u64, target_has_atomic = "64");
impl_atomic_int!(AtomicI64, i64, target_has_atomic = "64");

// AtomicUsize and AtomicIsize require target_has_atomic = "ptr"
#[cfg(target_has_atomic = "ptr")]
impl Encode for AtomicUsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.load(Ordering::Relaxed).encode(encoder)
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Decode for AtomicUsize {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(AtomicUsize::new(usize::decode(decoder)?))
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<'__de> BorrowDecode<'__de> for AtomicUsize {
    fn borrow_decode<D: BorrowDecoder<'__de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(AtomicUsize::new(usize::decode(decoder)?))
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Encode for AtomicIsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.load(Ordering::Relaxed).encode(encoder)
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Decode for AtomicIsize {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(AtomicIsize::new(isize::decode(decoder)?))
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<'__de> BorrowDecode<'__de> for AtomicIsize {
    fn borrow_decode<D: BorrowDecoder<'__de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(AtomicIsize::new(isize::decode(decoder)?))
    }
}

// ===== ManuallyDrop =====

impl<T: Encode> Encode for core::mem::ManuallyDrop<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for core::mem::ManuallyDrop<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(core::mem::ManuallyDrop::new(T::decode(decoder)?))
    }
}

impl<'__de, T: BorrowDecode<'__de>> BorrowDecode<'__de> for core::mem::ManuallyDrop<T> {
    fn borrow_decode<D: BorrowDecoder<'__de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(core::mem::ManuallyDrop::new(T::borrow_decode(decoder)?))
    }
}
