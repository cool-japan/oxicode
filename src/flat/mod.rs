//! Flat, fixed-size binary serialization helpers.
//!
//! The `flat` module is the lowest-overhead binary layout in OxiCode. It is
//! designed for types that always encode to the same number of bytes, so the
//! output is just a contiguous sequence of items with no chunk headers, length
//! prefixes, or other framing data.
//!
//! That means the size of a flat file is exactly `ITEM_SIZE * n_items`, and the
//! byte offset of item `n` can be computed directly as `ITEM_SIZE * n`. This
//! makes random access and direct loading of fixed-size records effectively
//! `O(1)` once the item size is known.
//!
//! # Overview
//!
//! - [`FlatEncoder`] writes fixed-size items directly to any `Write` target.
//! - [`FlatDecoder`] reads fixed-size items directly from any `Read + Seek` target.
//!
//! # Example
//!
//! ```rust,ignore
//! use oxicode::flat::{FlatDecoder, FlatEncoder};
//! use std::fs::File;
//! use std::io::SeekFrom;
//!
//! const ITEM_SIZE: usize = 16;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let file = File::create("items.bin")?;
//! let mut encoder = FlatEncoder::<_, ITEM_SIZE>::new(file);
//! encoder.write_item(&123u64)?;
//! encoder.write_item(&456u64)?;
//! let file = encoder.finish()?;
//!
//! let mut decoder = FlatDecoder::<_, ITEM_SIZE>::new(file);
//! let first: Option<u64> = decoder.read_item()?;
//! decoder.seek(SeekFrom::Start((ITEM_SIZE * 1) as u64))?;
//! let second: Option<u64> = decoder.read_item()?;
//! # Ok(())
//! # }
//! ```

/// Flat encoder implementation.
pub mod encoder;
/// Flat decoder implementation.
pub mod decoder;

pub use encoder::FlatEncoder;
pub use decoder::FlatDecoder;