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

// ===== Vec<T> =====

impl<T: Encode> Encode for Vec<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Delegate to the slice impl, which writes the same `len` prefix and
        // elements but also carries the `T == u8` bulk-write fast path.
        self.as_slice().encode(encoder)
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;

            // Claim memory for the container BEFORE allocating.
            decoder.claim_container_read::<T>(len)?;

            if unty::type_equal::<T, u8>() {
                // Fast path for `Vec<u8>`: read the whole buffer in one call,
                // producing byte-identical results to the per-element path.
                let mut bytes = alloc::vec![0u8; len];
                decoder.reader().read(&mut bytes)?;
                // SAFETY: `unty::type_equal::<T, u8>()` proved `T == u8`, so
                // `Vec<u8>` and `Vec<T>` have identical layout.
                return Ok(unsafe { core::mem::transmute::<Vec<u8>, Vec<T>>(bytes) });
            }

            let mut vec = Vec::with_capacity(len);
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

impl Decode for String {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = crate::de::decode_slice_len(decoder)?;

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
        crate::de::decode_with_depth_guard(decoder, |decoder| Ok(Box::new(T::decode(decoder)?)))
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

// The `?Sized` bound lets this single impl cover `Cow<'a, str>` and
// `Cow<'a, [T]>` (whose `Owned` types `String` / `Vec<T>` implement `Decode`)
// as well as every sized `T`, matching bincode 2. No separate concrete impls
// for `Cow<str>` / `Cow<[u8]>` are needed (they would overlap this one).
impl<'a, T> Decode for Cow<'a, T>
where
    T: ToOwned + ?Sized,
    T::Owned: Decode,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Owned(T::Owned::decode(decoder)?))
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
        crate::de::decode_with_depth_guard(decoder, |decoder| Ok(Rc::new(T::decode(decoder)?)))
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
        crate::de::decode_with_depth_guard(decoder, |decoder| Ok(Arc::new(T::decode(decoder)?)))
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

impl<T> Decode for BTreeSet<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<T> Decode for BinaryHeap<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut heap = BinaryHeap::with_capacity(len);
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

impl<T: Decode> Decode for VecDeque<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut deque = VecDeque::with_capacity(len);
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

impl<T: Decode> Decode for LinkedList<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<'de, T: BorrowDecode<'de> + Ord> BorrowDecode<'de> for BinaryHeap<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Vec::<T>::borrow_decode(decoder)?.into())
    }
}

impl<'de, K: BorrowDecode<'de> + Ord, V: BorrowDecode<'de>> BorrowDecode<'de> for BTreeMap<K, V> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<'de, T: BorrowDecode<'de> + Ord> BorrowDecode<'de> for BTreeSet<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<'de, T: BorrowDecode<'de>> BorrowDecode<'de> for VecDeque<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Vec::<T>::borrow_decode(decoder)?.into())
    }
}

impl<'de, T: BorrowDecode<'de>> BorrowDecode<'de> for LinkedList<T> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
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

impl<'de, T> BorrowDecode<'de> for alloc::vec::Vec<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;
            let mut vec = alloc::vec::Vec::with_capacity(len);
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                vec.push(T::borrow_decode(decoder)?);
            }
            Ok(vec)
        })
    }
}

impl<'de, T> BorrowDecode<'de> for alloc::boxed::Box<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            Ok(Box::new(T::borrow_decode(decoder)?))
        })
    }
}

impl<'de, T> BorrowDecode<'de> for alloc::boxed::Box<[T]>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Vec::<T>::borrow_decode(decoder)?.into_boxed_slice())
    }
}

impl<'de> BorrowDecode<'de> for alloc::boxed::Box<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Box::<str>::decode(decoder)
    }
}

impl<'de, T> BorrowDecode<'de> for alloc::rc::Rc<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            Ok(Rc::new(T::borrow_decode(decoder)?))
        })
    }
}

impl<'de, T> BorrowDecode<'de> for alloc::sync::Arc<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        crate::de::decode_with_depth_guard(decoder, |decoder| {
            Ok(Arc::new(T::borrow_decode(decoder)?))
        })
    }
}

impl<'de, T> BorrowDecode<'de> for alloc::sync::Arc<[T]>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Arc::from(
            Vec::<T>::borrow_decode(decoder)?.into_boxed_slice(),
        ))
    }
}

impl<'de> BorrowDecode<'de> for alloc::sync::Arc<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Arc::<str>::decode(decoder)
    }
}

impl<'de, T> BorrowDecode<'de> for alloc::rc::Rc<[T]>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Rc::from(
            Vec::<T>::borrow_decode(decoder)?.into_boxed_slice(),
        ))
    }
}

impl<'de> BorrowDecode<'de> for alloc::rc::Rc<str> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Rc::<str>::decode(decoder)
    }
}

// NOTE: The allocation-free zero-copy `BorrowDecode` impls for `Option<T>`,
// `&'de [u8]`, `&'de str`, `&'de [i8]` and `&'de [T]` were moved to
// `src/de/impls.rs` so they remain available in a `no_std`-without-`alloc`
// build (they perform no allocation).
