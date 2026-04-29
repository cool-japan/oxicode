use std::io::{Read, Seek, SeekFrom};

use crate::{Decode, Error, config::{self, Config, Configuration, Fixint, LittleEndian, NoLimit}};

/// Reads fixed-size decoded items from a flat byte stream.
///
/// `FlatDecoder` expects the input to be laid out as a plain sequence of
/// `ITEM_SIZE`-byte records. There are no headers or chunk markers, so the
/// caller can seek directly to `ITEM_SIZE * n` to load item `n`.
pub struct FlatDecoder<R: Read + Seek, const ITEM_SIZE: usize, C = Configuration<LittleEndian, Fixint, NoLimit>> {
    reader: R,
    config: C,
}

impl<R: Read + Seek, const ITEM_SIZE: usize> FlatDecoder<R, ITEM_SIZE> {
    /// Create a flat decoder using the crate's standard fixed-int configuration.
    pub fn new(reader: R) -> Self {
        Self::new_with(reader, config::standard().with_fixed_int_encoding())
    }
}

impl<R: Read + Seek, const ITEM_SIZE: usize, C: Config> FlatDecoder<R, ITEM_SIZE, C> {
    /// Create a flat decoder with a custom decoding configuration.
    pub fn new_with(reader: R, config: C) -> Self {
        Self { reader, config }
    }
    
    /// Read and decode the next fixed-size item from the stream.
    ///
    /// Returns `Ok(None)` on EOF. Partial trailing records are reported as an
    /// error because the stream length must be a multiple of `ITEM_SIZE`.
    pub fn read_item<T: Decode>(&mut self) -> Result<Option<T>, Error> {
        let mut buf = vec![0u8; ITEM_SIZE];
        let count = self.reader.read(&mut buf)?;
        if count == 0 {
            return Ok(None); // EOF
        } else if count < ITEM_SIZE {
            let msg = format!("Unexpected end of file: read {} bytes, expected {}", count, ITEM_SIZE);
            return Err(Error::OwnedCustom { message: msg });
        }
        let (item, _) = crate::decode_from_slice_with_config(&buf, self.config)?;
        Ok(Some(item))
    }
    
    /// Read and decode all remaining fixed-size items.
    pub fn read_all<T: Decode>(&mut self) -> Result<Vec<T>, Error> {
        let mut items = Vec::new();
        let mut buf = vec![0u8; ITEM_SIZE];
        while let Ok(amount) = self.reader.read(&mut buf) {
            if amount == 0 {
                break; // EOF
            } else if amount < ITEM_SIZE {
                let msg = format!("Unexpected end of file: read {} bytes, expected {}", amount, ITEM_SIZE);
                return Err(Error::OwnedCustom { message: msg });
            }
            let (item, _) = crate::decode_from_slice_with_config(&buf, self.config)?;
            items.push(item);
        }
        Ok(items)
    }
    
    /// Borrow the wrapped reader.
    pub fn get_ref(&self) -> &R {
        &self.reader
    }
    
    /// Seek the underlying reader.
    ///
    /// For random access, callers can seek to `SeekFrom::Start((ITEM_SIZE * n) as u64)`
    /// to position the decoder at item `n`.
    pub fn seek(&mut self, from: SeekFrom) -> Result<u64, Error> {
        Ok(self.reader.seek(from)?)
    }

    /// To not get errors
    pub fn get<T: Decode>(&mut self, idx: usize) -> Result<T, Error> {
        let current = self.seek(SeekFrom::Current(0))?;
        
        self.seek(SeekFrom::Start((idx * ITEM_SIZE) as u64))?;
        self.read_item()?.ok_or_else(|| Error::OwnedCustom {message: format!("Unexpected end of file at index {}", idx)})

        // seek back to original position
        .and_then(|item| {
            self.seek(SeekFrom::Start(current))?;
            Ok(item)
        })
    }

}