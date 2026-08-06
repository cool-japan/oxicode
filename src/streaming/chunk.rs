//! Chunk format for streaming data.

use crate::{Error, Result};

/// Magic bytes for chunk header: "OXIS" (OXIcode Stream)
pub const CHUNK_MAGIC: [u8; 4] = [0x4F, 0x58, 0x49, 0x53];

/// Chunk type indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChunkType {
    /// Data chunk containing encoded items.
    Data = 0,
    /// Final chunk indicating end of stream.
    End = 1,
    /// Metadata chunk (version info, etc.).
    Metadata = 2,
}

impl ChunkType {
    /// Parse from byte.
    fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(ChunkType::Data),
            1 => Some(ChunkType::End),
            2 => Some(ChunkType::Metadata),
            _ => None,
        }
    }
}

/// Header for each chunk in the stream.
///
/// Format (13 bytes):
/// - Bytes 0-3: Magic "OXIS"
/// - Byte 4: Chunk type
/// - Bytes 5-8: Payload length (u32, little-endian)
/// - Bytes 9-12: Item count in this chunk (u32, little-endian)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkHeader {
    /// Type of this chunk.
    pub chunk_type: ChunkType,

    /// Length of payload in bytes.
    pub payload_len: u32,

    /// Number of items in this chunk.
    pub item_count: u32,
}

impl ChunkHeader {
    /// Header size in bytes.
    pub const SIZE: usize = 13;

    /// Create a new data chunk header.
    #[inline]
    pub fn data(payload_len: u32, item_count: u32) -> Self {
        Self {
            chunk_type: ChunkType::Data,
            payload_len,
            item_count,
        }
    }

    /// Create an end-of-stream chunk header.
    #[inline]
    pub fn end() -> Self {
        Self {
            chunk_type: ChunkType::End,
            payload_len: 0,
            item_count: 0,
        }
    }

    /// Create a metadata chunk header.
    #[inline]
    pub fn metadata(payload_len: u32) -> Self {
        Self {
            chunk_type: ChunkType::Metadata,
            payload_len,
            item_count: 0,
        }
    }

    /// Check if this is an end chunk.
    #[inline]
    pub fn is_end(&self) -> bool {
        matches!(self.chunk_type, ChunkType::End)
    }

    /// Check if this is a data chunk.
    #[inline]
    pub fn is_data(&self) -> bool {
        matches!(self.chunk_type, ChunkType::Data)
    }

    /// Convert to bytes.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];

        bytes[0..4].copy_from_slice(&CHUNK_MAGIC);
        bytes[4] = self.chunk_type as u8;
        bytes[5..9].copy_from_slice(&self.payload_len.to_le_bytes());
        bytes[9..13].copy_from_slice(&self.item_count.to_le_bytes());

        bytes
    }

    /// Parse from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(Error::UnexpectedEnd {
                additional: Self::SIZE - data.len(),
            });
        }

        // Check magic
        if data[0..4] != CHUNK_MAGIC {
            return Err(Error::InvalidData {
                message: "invalid chunk magic",
            });
        }

        // Parse chunk type
        let chunk_type = ChunkType::from_byte(data[4]).ok_or(Error::InvalidData {
            message: "invalid chunk type",
        })?;

        // Parse lengths
        let payload_len = u32::from_le_bytes([data[5], data[6], data[7], data[8]]);
        let item_count = u32::from_le_bytes([data[9], data[10], data[11], data[12]]);

        // Validate item_count against payload_len for data chunks so a forged
        // header cannot drive the decoder's per-item loop far beyond what the
        // payload can actually contain.
        //
        // The chunks a homogeneous stream produces come in exactly two shapes:
        //   * zero-sized elements — every item encodes to 0 bytes, so a chunk
        //     legitimately carries `payload_len == 0` with any `item_count`
        //     (see `StreamingEncoder::flush_chunk`, which keys emission on the
        //     item count precisely to preserve these). We must not reject those.
        //   * non-zero-sized elements — every item consumes at least one payload
        //     byte, so `item_count` can never exceed `payload_len`.
        //
        // A data chunk with `payload_len > 0` but `item_count > payload_len` is
        // therefore unrepresentable output and is rejected up front, before the
        // decoder trusts `item_count` as its loop bound.
        if matches!(chunk_type, ChunkType::Data) && payload_len > 0 && item_count > payload_len {
            return Err(Error::InvalidData {
                message: "chunk item_count exceeds payload_len",
            });
        }

        Ok(Self {
            chunk_type,
            payload_len,
            item_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_header_roundtrip() {
        let header = ChunkHeader::data(1024, 10);
        let bytes = header.to_bytes();
        let parsed = ChunkHeader::from_bytes(&bytes).expect("parse failed");

        assert_eq!(header, parsed);
    }

    #[test]
    fn test_end_chunk() {
        let header = ChunkHeader::end();
        assert!(header.is_end());
        assert!(!header.is_data());

        let bytes = header.to_bytes();
        let parsed = ChunkHeader::from_bytes(&bytes).expect("parse failed");
        assert!(parsed.is_end());
    }

    #[test]
    fn test_data_chunk() {
        let header = ChunkHeader::data(500, 5);
        assert!(header.is_data());
        assert!(!header.is_end());
        assert_eq!(header.payload_len, 500);
        assert_eq!(header.item_count, 5);
    }

    #[test]
    fn test_metadata_chunk() {
        let header = ChunkHeader::metadata(256);
        assert!(!header.is_data());
        assert!(!header.is_end());
        assert_eq!(header.payload_len, 256);
    }

    #[test]
    fn test_invalid_magic() {
        let mut bytes = ChunkHeader::data(100, 1).to_bytes();
        bytes[0] = 0xFF; // Corrupt magic

        let result = ChunkHeader::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_header_size() {
        assert_eq!(ChunkHeader::SIZE, 13);
    }

    #[test]
    fn test_reject_item_count_exceeding_payload_len() {
        // A forged data header claiming more items than the payload could
        // possibly hold (each non-zero-sized item needs >= 1 byte) must be
        // rejected before the decoder trusts item_count as a loop bound.
        let mut bytes = ChunkHeader::data(1, 5).to_bytes();
        // Sanity: this is a data chunk with payload_len = 1, item_count = 5.
        let result = ChunkHeader::from_bytes(&bytes);
        assert!(
            result.is_err(),
            "item_count (5) > payload_len (1) must be rejected for data chunks"
        );

        // The classic amplification case: 0-length payload, huge item_count, is
        // the malformed non-ZST shape once payload_len is forced non-zero.
        bytes[5..9].copy_from_slice(&2u32.to_le_bytes()); // payload_len = 2
        bytes[9..13].copy_from_slice(&u32::MAX.to_le_bytes()); // item_count = u32::MAX
        assert!(
            ChunkHeader::from_bytes(&bytes).is_err(),
            "payload_len = 2, item_count = u32::MAX must be rejected"
        );
    }

    #[test]
    fn test_zero_sized_item_chunk_is_accepted() {
        // Zero-sized elements encode to 0 bytes, so the encoder legitimately
        // emits payload_len = 0 with item_count > 0. Parsing must accept these.
        let header = ChunkHeader::data(0, 10);
        let parsed = ChunkHeader::from_bytes(&header.to_bytes()).expect("zero-sized chunk parse");
        assert_eq!(parsed.payload_len, 0);
        assert_eq!(parsed.item_count, 10);

        // item_count == payload_len is the boundary of the non-ZST case and is
        // valid (a stream of one-byte items).
        let boundary = ChunkHeader::data(4, 4);
        assert_eq!(
            ChunkHeader::from_bytes(&boundary.to_bytes()).expect("boundary parse"),
            boundary
        );
    }
}
