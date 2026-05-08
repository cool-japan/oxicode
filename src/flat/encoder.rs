use std::io::Write;

use crate::{Encode, Error, config::{self, Config, Configuration, Fixint, LittleEndian, NoLimit}};


/// Writes fixed-size encoded items as a flat byte stream.
///
/// `FlatEncoder` is intended for data types whose binary representation is
/// always exactly `ITEM_SIZE` bytes. It writes each encoded value directly to
/// the underlying writer without chunking, headers, or extra framing.
///
/// Because every item has the same size, the resulting file length is always
/// `ITEM_SIZE * n_items`, which makes direct record access straightforward.
pub struct FlatEncoder<W: Write, const ITEM_SIZE: usize, C = Configuration<LittleEndian, Fixint, NoLimit>> {
    writer: W,
    config: C,    
}

impl<W: Write, const ITEM_SIZE: usize> FlatEncoder<W, ITEM_SIZE> {
    /// Create a flat encoder using the crate's standard fixed-int configuration.
    pub fn new(writer: W) -> Self {
        Self::new_with(writer, config::standard().with_fixed_int_encoding())
    }
}

impl<W: Write, const ITEM_SIZE: usize, C: Config> FlatEncoder<W, ITEM_SIZE, C> {
    /// Create a flat encoder with a custom encoding configuration.
    pub fn new_with(writer: W, config: C) -> Self {
        Self { writer, config }
    }
    
    /// Encode one item and append it to the underlying writer.
    ///
    /// The encoded byte length is validated in debug builds to match
    /// `ITEM_SIZE`.
    pub fn write_item<T: Encode>(&mut self, item: &T) -> Result<(), Error> {
        let buf = crate::encode_to_vec_with_config(item, self.config)?;
        self.writer.write_all(buf.as_slice())?;
        debug_assert_eq!(
            buf.len(), ITEM_SIZE,
            "encoded {} bytes, expected {}. Check impl_flat_encodable!",
            buf.len(), ITEM_SIZE
        );

        Ok(())
    }
    
    /// Encode and write all items from an iterator.
    pub fn write_all<T: Encode, I: IntoIterator<Item = T>>(&mut self, items: I) -> Result<(), Error> {
        for item in items {
            self.write_item(&item)?;
        }
        Ok(())
    }
    
    /// Flush the underlying writer.
    pub fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()?;
        Ok(())
    }
    
    /// Flush pending output and return the wrapped writer.
    pub fn finish(mut self) -> Result<W, Error> {
        self.flush()?;
        Ok(self.writer)
    }
    
    /// Borrow the wrapped writer.
    pub fn get_ref(&self) -> &W {
        &self.writer
    }
}

