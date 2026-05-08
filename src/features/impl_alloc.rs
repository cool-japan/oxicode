//! Encode/Decode implementations for alloc-dependent types

use crate::{
    de::{read::Reader, BorrowDecode, BorrowDecoder, BorrowReader, Decode, Decoder},
    enc::{write::Writer, Encode, Encoder},
    error::Error,
};
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
};

// ===== Vec<T> =====

impl<T: Encode> Encode for Vec<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode length first
        (self.len() as u64).encode(encoder)?;
        // Encode each element
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        // Claim memory for the container
        decoder.claim_container_read::<T>(len)?;

        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(decoder)?);
        }
        Ok(vec)
    }
}

// ===== String =====

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.as_str().encode(encoder)
    }
}

impl Encode for str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode byte length first
        (self.len() as u64).encode(encoder)?;
        // Encode UTF-8 bytes
        encoder.writer().write(self.as_bytes())
    }
}

impl Encode for &str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (*self).encode(encoder)
    }
}

impl Encode for &[u8] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode length first
        (self.len() as u64).encode(encoder)?;
        // Encode bytes
        encoder.writer().write(self)
    }
}

impl Decode for String {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        // Claim bytes
        decoder.claim_bytes_read(len)?;

        let mut bytes = alloc::vec![0u8; len];
        decoder.reader().read(&mut bytes)?;

        String::from_utf8(bytes).map_err(|e| Error::Utf8 {
            inner: e.utf8_error(),
        })
    }
}

// ===== Box<T> =====

impl<T: Encode> Encode for Box<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Box<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Box::new(T::decode(decoder)?))
    }
}

// ===== Box<[T]> =====

impl<T: Encode> Encode for Box<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Box<[T]> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}

// ===== Box<str> =====

impl Encode for Box<str> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl Decode for Box<str> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let string = String::decode(decoder)?;
        Ok(string.into_boxed_str())
    }
}

// ===== Cow<'a, T> =====

impl<T: Encode + ToOwned + ?Sized> Encode for Cow<'_, T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<'a, T> Decode for Cow<'a, T>
where
    T: ToOwned,
    T::Owned: Decode,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Owned(T::Owned::decode(decoder)?))
    }
}

// ===== Decode for Cow<'_, str> and Cow<'_, [u8]> (unsized T specializations) =====
// The generic `Decode for Cow<'a, T>` requires T: Sized, which excludes str and [u8].
// These concrete impls cover those unsized cases.

impl Decode for Cow<'_, str> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Owned(String::decode(decoder)?))
    }
}

impl Decode for Cow<'_, [u8]> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Owned(Vec::<u8>::decode(decoder)?))
    }
}

// ===== BorrowDecode for Cow<'de, str> (zero-copy) =====

impl<'de> BorrowDecode<'de> for Cow<'de, str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Borrowed(<&'de str>::borrow_decode(decoder)?))
    }
}

// ===== BorrowDecode for Cow<'de, [u8]> (zero-copy) =====

impl<'de> BorrowDecode<'de> for Cow<'de, [u8]> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Borrowed(<&'de [u8]>::borrow_decode(decoder)?))
    }
}

// ===== Rc<T> =====

impl<T: Encode> Encode for Rc<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Rc<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Rc::new(T::decode(decoder)?))
    }
}

// ===== Rc<[T]> =====

impl<T: Encode> Encode for Rc<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Rc<[T]> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(Rc::from(vec.into_boxed_slice()))
    }
}

// ===== Rc<str> =====

impl Encode for Rc<str> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl Decode for Rc<str> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let string = String::decode(decoder)?;
        Ok(Rc::from(string.into_boxed_str()))
    }
}

// ===== Arc<T> =====

impl<T: Encode> Encode for Arc<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Arc<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Arc::new(T::decode(decoder)?))
    }
}

// ===== Arc<[T]> =====

impl<T: Encode> Encode for Arc<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Arc<[T]> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(Arc::from(vec.into_boxed_slice()))
    }
}

// ===== Arc<str> =====

impl Encode for Arc<str> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl Decode for Arc<str> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let string = String::decode(decoder)?;
        Ok(Arc::from(string.into_boxed_str()))
    }
}

// ===== BTreeMap<K, V> =====

impl<K: Encode, V: Encode> Encode for BTreeMap<K, V> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for (key, value) in self.iter() {
            key.encode(encoder)?;
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut map = BTreeMap::new();
        for _ in 0..len {
            let key = K::decode(decoder)?;
            let value = V::decode(decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

// ===== BTreeSet<T> =====

impl<T: Encode> Encode for BTreeSet<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> Decode for BTreeSet<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut set = BTreeSet::new();
        for _ in 0..len {
            set.insert(T::decode(decoder)?);
        }
        Ok(set)
    }
}

// ===== BinaryHeap<T> =====

impl<T: Encode> Encode for BinaryHeap<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> Decode for BinaryHeap<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut heap = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            heap.push(T::decode(decoder)?);
        }
        Ok(heap)
    }
}

// ===== VecDeque<T> =====

impl<T: Encode> Encode for VecDeque<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: Decode> Decode for VecDeque<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut deque = VecDeque::with_capacity(len);
        for _ in 0..len {
            deque.push_back(T::decode(decoder)?);
        }
        Ok(deque)
    }
}

// ===== LinkedList<T> =====

impl<T: Encode> Encode for LinkedList<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: Decode> Decode for LinkedList<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut list = LinkedList::new();
        for _ in 0..len {
            list.push_back(T::decode(decoder)?);
        }
        Ok(list)
    }
}

// ===== BorrowDecode for owned alloc types (delegate to Decode) =====
// These allow BorrowDecode derive to work with owned fields like String, Vec<T>, etc.

crate::impl_borrow_decode!(String);

impl<'de, T: Decode + Ord + 'static> BorrowDecode<'de> for BinaryHeap<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        BinaryHeap::<T>::decode(decoder)
    }
}

impl<'de, K: Decode + Ord + 'static, V: Decode + 'static> BorrowDecode<'de> for BTreeMap<K, V> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        BTreeMap::<K, V>::decode(decoder)
    }
}

impl<'de, T: Decode + Ord + 'static> BorrowDecode<'de> for BTreeSet<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        BTreeSet::<T>::decode(decoder)
    }
}

impl<'de, T: Decode + 'static> BorrowDecode<'de> for VecDeque<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        VecDeque::<T>::decode(decoder)
    }
}

impl<'de, T: Decode + 'static> BorrowDecode<'de> for LinkedList<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        LinkedList::<T>::decode(decoder)
    }
}

impl<'de, T> BorrowDecode<'de> for alloc::vec::Vec<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;
        decoder.claim_container_read::<T>(len)?;
        let mut vec = alloc::vec::Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::borrow_decode(decoder)?);
        }
        Ok(vec)
    }
}

impl<'de, T> BorrowDecode<'de> for Option<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(None),
            1 => Ok(Some(T::borrow_decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Option variant",
            }),
        }
    }
}

impl<'de, T: Decode + 'static> BorrowDecode<'de> for alloc::boxed::Box<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Box::<T>::decode(decoder)
    }
}

impl<'de, T: Decode + 'static> BorrowDecode<'de> for alloc::boxed::Box<[T]> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Box::<[T]>::decode(decoder)
    }
}

impl<'de> BorrowDecode<'de> for alloc::boxed::Box<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Box::<str>::decode(decoder)
    }
}

impl<'de, T: Decode + 'static> BorrowDecode<'de> for alloc::sync::Arc<[T]> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Arc::<[T]>::decode(decoder)
    }
}

impl<'de> BorrowDecode<'de> for alloc::sync::Arc<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Arc::<str>::decode(decoder)
    }
}

impl<'de, T: Decode + 'static> BorrowDecode<'de> for alloc::rc::Rc<[T]> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Rc::<[T]>::decode(decoder)
    }
}

impl<'de> BorrowDecode<'de> for alloc::rc::Rc<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Rc::<str>::decode(decoder)
    }
}

// ===== BorrowDecode for &[u8] (zero-copy) =====

impl<'de> BorrowDecode<'de> for &'de [u8] {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        use crate::de::Decode;
        let len = u64::decode(decoder)? as usize;
        decoder.claim_bytes_read(len)?;

        let bytes = decoder.borrow_reader().take_bytes(len)?;
        Ok(bytes)
    }
}

// ===== BorrowDecode for &str (zero-copy) =====

impl<'de> BorrowDecode<'de> for &'de str {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        use crate::de::Decode;
        let len = u64::decode(decoder)? as usize;
        decoder.claim_bytes_read(len)?;

        let bytes = decoder.borrow_reader().take_bytes(len)?;

        // Validate UTF-8
        core::str::from_utf8(bytes).map_err(|e| Error::Utf8 { inner: e })
    }
}

// ===== BorrowDecode for &[i8] (zero-copy signed bytes) =====

/// Zero-copy decode for `&[i8]`.
///
/// Since `i8` and `u8` have identical bit patterns, we can reinterpret
/// the `&[u8]` taken from the input buffer as `&[i8]` using a safe transmute.
/// This is valid because:
/// 1. `i8` has the same size and alignment as `u8`
/// 2. All bit patterns are valid for `i8`
impl<'de> BorrowDecode<'de> for &'de [i8] {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        use crate::de::Decode;
        let len = u64::decode(decoder)? as usize;
        decoder.claim_bytes_read(len)?;

        let bytes: &'de [u8] = decoder.borrow_reader().take_bytes(len)?;

        // Safety: &[u8] and &[i8] have the same size, alignment, and all bit patterns are valid.
        // The lifetime 'de is preserved correctly.
        let signed_bytes: &'de [i8] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len()) };
        Ok(signed_bytes)
    }
}

// ===== BorrowDecode for &[T] where T: BorrowableSliceElement (zero-copy Pod-primitive) =====

/// Zero-copy borrow-decode for `&'de [T]` where `T` is a Pod-like primitive.
///
/// Reinterprets a contiguous byte slice from the input buffer as `&'de [T]`
/// without copying. Three runtime guards ensure soundness:
///
/// 1. **Encoding gate** — `IntEncoding::Fixed` only; Varint compresses
///    elements and breaks the in-memory layout identity.
/// 2. **Endianness gate** — decoder endianness must match the host's native
///    byte order for any `T` wider than one byte.
/// 3. **Alignment gate** — `take_bytes`'s result must be aligned to
///    `align_of::<T>()`.
///
/// All gates produce `Error::InvalidData` with a descriptive message on
/// failure, making the error actionable (switch to `Vec<T>`, fix config, etc.).
impl<'de, T> BorrowDecode<'de> for &'de [T]
where
    T: crate::de::BorrowableSliceElement,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        use crate::config::{Config, IntEncoding};
        use crate::de::Decode;

        let len = u64::decode(decoder)? as usize;
        decoder.claim_container_read::<T>(len)?;

        // Encoding gate: only Fixint produces verbatim in-memory layout per element.
        let cfg = decoder.config();
        if cfg.int_encoding() != IntEncoding::Fixed {
            return Err(Error::InvalidData {
                message: "borrow-decode of &[T] requires IntEncoding::Fixed; \
                          varint-encoded payloads do not match in-memory layout",
            });
        }

        // Endianness gate: the buffer bytes must match the host's native byte order.
        if !T::endianness_compatible(cfg.endianness()) {
            return Err(Error::InvalidData {
                message: "borrow-decode of &[T] requires native endianness; \
                          use Vec<T> for cross-endian decoding",
            });
        }

        let elem_size = core::mem::size_of::<T>();
        let byte_count = len.checked_mul(elem_size).ok_or(Error::InvalidData {
            message: "borrow-decode of &[T]: length × element-size overflows usize",
        })?;

        let bytes = decoder.borrow_reader().take_bytes(byte_count)?;

        // Alignment gate: pointer must satisfy align_of::<T>().
        let align = core::mem::align_of::<T>();
        if (bytes.as_ptr() as usize) % align != 0 {
            return Err(Error::InvalidData {
                message: "borrow-decode of &[T]: buffer is not aligned to align_of::<T>(); \
                          use Vec<T> instead",
            });
        }

        // SAFETY:
        // - `bytes` is `&'de [u8]` with length exactly `len * size_of::<T>()`.
        // - `bytes.as_ptr()` is aligned to `align_of::<T>()` (checked above).
        // - Encoding is Fixed and endianness is native (checked above), so
        //   the bytes are a verbatim copy of `[T; len]` in host memory.
        // - `T: BorrowableSliceElement` guarantees: `T: Copy + Sized + 'static`,
        //   all bit patterns are valid, layout is fixed and known.
        // - Lifetime `'de` of `bytes` is correctly propagated.
        let slice: &'de [T] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const T, len) };
        Ok(slice)
    }
}
