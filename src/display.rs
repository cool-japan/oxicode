//! Display utilities for encoded binary data.

use core::fmt;

/// A wrapper around a byte slice that implements human-readable Display traits.
pub struct EncodedBytes<'a>(pub &'a [u8]);

/// An owned version of [`EncodedBytes`] wrapping a `Vec<u8>`.
#[cfg(feature = "alloc")]
pub struct EncodedBytesOwned(pub alloc::vec::Vec<u8>);

impl<'a> EncodedBytes<'a> {
    /// Returns a reference to the underlying byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }

    /// Returns an xxd-style hex dump.
    #[cfg(feature = "alloc")]
    pub fn hex_dump(&self) -> alloc::string::String {
        hex_dump_bytes(self.0)
    }
}

/// `Display` — space-separated hex bytes: `01 2f ff ...`
impl<'a> fmt::Display for EncodedBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, byte) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// `LowerHex` — compact hex run: `012fff...`
impl<'a> fmt::LowerHex for EncodedBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// `UpperHex` — compact uppercase hex run: `012FFF...`
impl<'a> fmt::UpperHex for EncodedBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl EncodedBytesOwned {
    /// Returns a reference to the underlying byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns an xxd-style hex dump string:
    /// ```text
    /// 00000000: 4865 6c6c 6f2c 2057 6f72 6c64 21        Hello, World!
    /// ```
    pub fn hex_dump(&self) -> alloc::string::String {
        hex_dump_bytes(&self.0)
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for EncodedBytesOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        EncodedBytes(&self.0).fmt(f)
    }
}

#[cfg(feature = "alloc")]
impl fmt::LowerHex for EncodedBytesOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&EncodedBytes(&self.0), f)
    }
}

#[cfg(feature = "alloc")]
impl fmt::UpperHex for EncodedBytesOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&EncodedBytes(&self.0), f)
    }
}

/// Produce an xxd-style hex dump of a byte slice.
///
/// Each line shows the address, hex bytes in 2-byte groups, and an ASCII sidebar.
/// Example output:
/// ```text
/// 00000000: 4865 6c6c 6f2c 2057 6f72 6c64 21        Hello, World!
/// ```
#[cfg(feature = "alloc")]
pub fn hex_dump_bytes(data: &[u8]) -> alloc::string::String {
    use alloc::string::String;

    let mut out = String::new();
    for (chunk_idx, chunk) in data.chunks(16).enumerate() {
        let addr = chunk_idx * 16;

        // Build hex part: groups of 2 bytes separated by spaces
        let mut hex_part = String::new();
        for (i, byte) in chunk.iter().enumerate() {
            if i > 0 && i % 2 == 0 {
                hex_part.push(' ');
            }
            let hi = (byte >> 4) as usize;
            let lo = (byte & 0xf) as usize;
            const HEX: &[u8] = b"0123456789abcdef";
            hex_part.push(HEX[hi] as char);
            hex_part.push(HEX[lo] as char);
        }

        // Pad hex part to 39 chars (8 groups of 4 chars + 7 spaces = 39)
        while hex_part.len() < 39 {
            hex_part.push(' ');
        }

        // ASCII sidebar: printable chars or '.'
        let ascii: String = chunk
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();

        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&alloc::format!("{:08x}: {}  {}", addr, hex_part, ascii));
    }
    out
}
