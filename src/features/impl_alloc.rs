//! Encode/Decode implementations for alloc-dependent types

use crate::{
    de::{read::Reader, BorrowDecode, BorrowDecoder, Decode, Decoder},
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

/// Largest buffer materialized in one step when the amount of remaining input
/// is unknown (streaming readers).
///
/// Bounding the step means a forged length prefix can only ever commit this
/// much memory before the reader has to actually produce bytes, so the
/// allocation grows in step with the data that really arrives.
const INCREMENTAL_READ_STEP: usize = 16 * 1024;

/// Largest number of elements pre-reserved for a length-prefixed container.
///
/// The elements themselves are decoded one at a time, so the vector still grows
/// to whatever the input legitimately contains; this only stops an
/// attacker-controlled count from being turned into a single huge reservation.
/// Matches the ceiling the `#[derive(Decode)]` sequence path already uses.
const MAX_PREALLOC_ELEMENTS: usize = 4096;

/// Number of elements to pre-reserve for a container claiming `len` elements.
#[inline]
pub(crate) fn prealloc_elements(len: usize) -> usize {
    core::cmp::min(len, MAX_PREALLOC_ELEMENTS)
}

/// Largest buffer materialized in one allocation even when the reader claims
/// that many bytes remain.
///
/// A reader's `remaining_bytes` is only ever an *upper* bound. It is exact for
/// slice input, but an IO reader given a deliberately generous budget (say
/// "1 GiB, to be safe") would otherwise let nine bytes of forged length prefix
/// commit a gigabyte in one `alloc`. Capping the first allocation and growing
/// from there — each step filled from the reader before the next is reserved —
/// keeps the peak proportional to the data that actually arrives, whatever the
/// bound's provenance. Large enough that realistic payloads still take a single
/// allocation.
const MAX_EAGER_ALLOC: usize = 16 * 1024 * 1024;

/// Read exactly `len` bytes without letting an attacker-controlled length
/// commit the whole allocation before the payload is known to exist.
///
/// Two independent protections:
///
/// * **Reject early.** When the reader knows how much input is left (any
///   slice-backed decoder, or an IO reader given a budget), a `len` larger than
///   that can never be satisfied, so it is rejected with
///   [`Error::UnexpectedEnd`] before a single byte is allocated.
/// * **Grow with the data.** The buffer is materialized in bounded steps —
///   [`MAX_EAGER_ALLOC`] for the first one when the length is known to fit the
///   remaining input, [`INCREMENTAL_READ_STEP`] when nothing is known — and
///   each step must actually be filled from the reader before the next one is
///   reserved. A forged length therefore fails on a short read rather than at
///   `alloc` time, no matter how loose the reported bound was.
///
/// This is the mechanism behind SECURITY.md's promise that length-prefixed
/// buffers are bounds-checked against the remaining input before allocation.
/// It is independent of `claim_bytes_read`, which enforces the *configured*
/// decode limit and is a no-op under the default `NoLimit` configuration.
pub(crate) fn read_bytes_bounded<D: Decoder>(
    decoder: &mut D,
    len: usize,
) -> Result<Vec<u8>, Error> {
    // Step size for the first reservation: a known-good bound buys a big step,
    // an unknown one buys a small one.
    let step_size = match decoder.remaining_reader_bytes() {
        Some(remaining) => {
            if len > remaining {
                return Err(Error::UnexpectedEnd {
                    additional: len - remaining,
                });
            }
            MAX_EAGER_ALLOC
        }
        None => INCREMENTAL_READ_STEP,
    };

    if len <= step_size {
        let mut bytes = alloc::vec![0u8; len];
        decoder.reader().read(&mut bytes)?;
        return Ok(bytes);
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut filled = 0usize;
    while filled < len {
        let step = core::cmp::min(step_size, len - filled);
        bytes.resize(filled + step, 0u8);
        decoder.reader().read(&mut bytes[filled..])?;
        filled += step;
    }
    Ok(bytes)
}

// ===== Vec<T> =====

impl<T: Encode> Encode for Vec<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Delegate to the slice impl, which writes the same `len` prefix and
        // elements but also carries the `T == u8` bulk-write fast path.
        self.as_slice().encode(encoder)
    }
}

impl<Context, T: Decode<Context>> Decode<Context> for Vec<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;

            // Claim memory for the container BEFORE allocating.
            decoder.claim_container_read::<T>(len)?;

            if unty::type_equal::<T, u8>() {
                // Fast path for `Vec<u8>`: read the whole buffer in one call,
                // producing byte-identical results to the per-element path.
                let bytes = read_bytes_bounded(decoder, len)?;
                // SAFETY: `unty::type_equal::<T, u8>()` proved `T == u8`, so
                // `Vec<u8>` and `Vec<T>` have identical layout.
                return Ok(unsafe { core::mem::transmute::<Vec<u8>, Vec<T>>(bytes) });
            }

            let mut vec = Vec::with_capacity(prealloc_elements(len));
            for _ in 0..len {
                // Reclaim one element's reservation before decoding it, so the
                // element's own `claim_bytes_read` calls do not double-count.
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                vec.push(T::decode(decoder)?);
            }
            Ok(vec)
        })
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

// NOTE: `Encode for &str` and `Encode for &[u8]` are intentionally NOT defined
// here. They are covered by the blanket `impl<T: Encode + ?Sized> Encode for &T`
// in `src/enc/impls.rs` (via the `str` and `[u8]` value impls), which produces
// byte-identical output while also covering every other `&T`.

impl<Context> Decode<Context> for String {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        let len = crate::de::decode_slice_len(decoder)?;

        // Claim bytes against the configured decode limit (a no-op under the
        // default `NoLimit` config), then materialize the buffer under the
        // remaining-input bound so a forged length cannot allocate up front.
        decoder.claim_bytes_read(len)?;

        let bytes = read_bytes_bounded(decoder, len)?;

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

impl<Context, T: Decode<Context>> Decode<Context> for Box<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| Ok(Box::new(T::decode(decoder)?)))
    }
}

// ===== Box<[T]> =====

impl<T: Encode> Encode for Box<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<Context, T: Decode<Context>> Decode<Context> for Box<[T]> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<Context> Decode<Context> for Box<str> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
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

// The `?Sized` bound lets this single impl cover `Cow<'a, str>` and
// `Cow<'a, [T]>` (whose `Owned` types `String` / `Vec<T>` implement `Decode`)
// as well as every sized `T`, matching bincode 2. No separate concrete impls
// for `Cow<str>` / `Cow<[u8]>` are needed (they would overlap this one).
impl<'a, Context, T> Decode<Context> for Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: Decode<Context>,
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Owned(T::Owned::decode(decoder)?))
    }
}

// ===== BorrowDecode for Cow<'de, str> (zero-copy) =====

impl<'de, Context> BorrowDecode<'de, Context> for Cow<'de, str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(Cow::Borrowed(<&'de str>::borrow_decode(decoder)?))
    }
}

// ===== BorrowDecode for Cow<'de, [u8]> (zero-copy) =====

impl<'de, Context> BorrowDecode<'de, Context> for Cow<'de, [u8]> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(Cow::Borrowed(<&'de [u8]>::borrow_decode(decoder)?))
    }
}

// ===== Rc<T> =====

impl<T: Encode> Encode for Rc<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<Context, T: Decode<Context>> Decode<Context> for Rc<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| Ok(Rc::new(T::decode(decoder)?)))
    }
}

// ===== Rc<[T]> =====

impl<T: Encode> Encode for Rc<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<Context, T: Decode<Context>> Decode<Context> for Rc<[T]> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<Context> Decode<Context> for Rc<str> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<Context, T: Decode<Context>> Decode<Context> for Arc<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| Ok(Arc::new(T::decode(decoder)?)))
    }
}

// ===== Arc<[T]> =====

impl<T: Encode> Encode for Arc<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<Context, T: Decode<Context>> Decode<Context> for Arc<[T]> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<Context> Decode<Context> for Arc<str> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<Context, K, V> Decode<Context> for BTreeMap<K, V>
where
    K: Decode<Context> + Ord,
    V: Decode<Context>,
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<(K, V)>(len)?;

            let mut map = BTreeMap::new();
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());
                let key = K::decode(decoder)?;
                let value = V::decode(decoder)?;
                map.insert(key, value);
            }
            Ok(map)
        })
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

impl<Context, T> Decode<Context> for BTreeSet<T>
where
    T: Decode<Context> + Ord,
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut set = BTreeSet::new();
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                set.insert(T::decode(decoder)?);
            }
            Ok(set)
        })
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

impl<Context, T> Decode<Context> for BinaryHeap<T>
where
    T: Decode<Context> + Ord,
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut heap = BinaryHeap::with_capacity(prealloc_elements(len));
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                heap.push(T::decode(decoder)?);
            }
            Ok(heap)
        })
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

impl<Context, T: Decode<Context>> Decode<Context> for VecDeque<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut deque = VecDeque::with_capacity(prealloc_elements(len));
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                deque.push_back(T::decode(decoder)?);
            }
            Ok(deque)
        })
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

impl<Context, T: Decode<Context>> Decode<Context> for LinkedList<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut list = LinkedList::new();
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                list.push_back(T::decode(decoder)?);
            }
            Ok(list)
        })
    }
}

// ===== BorrowDecode for owned alloc types =====
// These mirror bincode 2: element-wise `T: BorrowDecode` rather than the
// stricter `T: Decode + 'static` delegation, so borrowing element types such as
// `HashMap<&'de str, u32>` or `Box<&'de str>` compile.

crate::impl_borrow_decode!(String);

impl<'de, Context, T: BorrowDecode<'de, Context> + Ord> BorrowDecode<'de, Context>
    for BinaryHeap<T>
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(Vec::<T>::borrow_decode(decoder)?.into())
    }
}

impl<'de, Context, K: BorrowDecode<'de, Context> + Ord, V: BorrowDecode<'de, Context>>
    BorrowDecode<'de, Context> for BTreeMap<K, V>
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<(K, V)>(len)?;

            let mut map = BTreeMap::new();
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());
                let key = K::borrow_decode(decoder)?;
                let value = V::borrow_decode(decoder)?;
                map.insert(key, value);
            }
            Ok(map)
        })
    }
}

impl<'de, Context, T: BorrowDecode<'de, Context> + Ord> BorrowDecode<'de, Context> for BTreeSet<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut set = BTreeSet::new();
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                set.insert(T::borrow_decode(decoder)?);
            }
            Ok(set)
        })
    }
}

impl<'de, Context, T: BorrowDecode<'de, Context>> BorrowDecode<'de, Context> for VecDeque<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(Vec::<T>::borrow_decode(decoder)?.into())
    }
}

impl<'de, Context, T: BorrowDecode<'de, Context>> BorrowDecode<'de, Context> for LinkedList<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut list = LinkedList::new();
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                list.push_back(T::borrow_decode(decoder)?);
            }
            Ok(list)
        })
    }
}

impl<'de, Context, T> BorrowDecode<'de, Context> for alloc::vec::Vec<T>
where
    T: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;
            let mut vec = alloc::vec::Vec::with_capacity(prealloc_elements(len));
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                vec.push(T::borrow_decode(decoder)?);
            }
            Ok(vec)
        })
    }
}

impl<'de, Context, T> BorrowDecode<'de, Context> for alloc::boxed::Box<T>
where
    T: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            Ok(Box::new(T::borrow_decode(decoder)?))
        })
    }
}

impl<'de, Context, T> BorrowDecode<'de, Context> for alloc::boxed::Box<[T]>
where
    T: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(Vec::<T>::borrow_decode(decoder)?.into_boxed_slice())
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for alloc::boxed::Box<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Box::<str>::decode(decoder)
    }
}

impl<'de, Context, T> BorrowDecode<'de, Context> for alloc::rc::Rc<T>
where
    T: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            Ok(Rc::new(T::borrow_decode(decoder)?))
        })
    }
}

impl<'de, Context, T> BorrowDecode<'de, Context> for alloc::sync::Arc<T>
where
    T: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            Ok(Arc::new(T::borrow_decode(decoder)?))
        })
    }
}

impl<'de, Context, T> BorrowDecode<'de, Context> for alloc::sync::Arc<[T]>
where
    T: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(Arc::from(
            Vec::<T>::borrow_decode(decoder)?.into_boxed_slice(),
        ))
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for alloc::sync::Arc<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Arc::<str>::decode(decoder)
    }
}

impl<'de, Context, T> BorrowDecode<'de, Context> for alloc::rc::Rc<[T]>
where
    T: BorrowDecode<'de, Context>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Ok(Rc::from(
            Vec::<T>::borrow_decode(decoder)?.into_boxed_slice(),
        ))
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for alloc::rc::Rc<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        Rc::<str>::decode(decoder)
    }
}

// NOTE: The allocation-free zero-copy `BorrowDecode` impls for `Option<T>`,
// `&'de [u8]`, `&'de str`, `&'de [i8]` and `&'de [T]` were moved to
// `src/de/impls.rs` so they remain available in a `no_std`-without-`alloc`
// build (they perform no allocation).
