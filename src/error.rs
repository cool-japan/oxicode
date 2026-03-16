//! Error types for OxiCode

use core::fmt;

/// Result type alias for OxiCode operations
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for encoding and decoding operations
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Unexpected end of input during decoding
    UnexpectedEnd {
        /// Estimate of additional bytes needed
        additional: usize,
    },

    /// Invalid data encountered during decoding
    InvalidData {
        /// Description of what went wrong
        message: &'static str,
    },

    /// Invalid integer type during varint decoding
    InvalidIntegerType {
        /// The type that was expected
        expected: IntegerType,
        /// The type that was found
        found: IntegerType,
    },

    /// Invalid boolean value
    InvalidBooleanValue(u8),

    /// Invalid character encoding
    InvalidCharEncoding([u8; 4]),

    /// UTF-8 decoding error
    #[cfg(feature = "alloc")]
    Utf8 {
        /// The inner UTF-8 error
        inner: core::str::Utf8Error,
    },

    /// Configuration limit exceeded
    LimitExceeded {
        /// The limit that was configured
        limit: u64,
        /// The value that exceeded the limit
        found: u64,
    },

    /// IO error during encoding or decoding
    #[cfg(feature = "std")]
    Io {
        /// Error kind
        kind: std::io::ErrorKind,
        /// Error message
        message: String,
    },

    /// Custom error message (static string)
    Custom {
        /// Error message
        message: &'static str,
    },

    /// Custom error message (owned string, from serde or dynamic sources)
    #[cfg(feature = "alloc")]
    OwnedCustom {
        /// Error message
        message: alloc::string::String,
    },

    /// Value outside usize range (for platforms with smaller usize)
    OutsideUsizeRange(u64),

    /// NonZero type decoded as zero
    NonZeroTypeIsZero {
        /// The NonZero type that was zero
        non_zero_type: IntegerType,
    },

    /// Invalid enum variant
    UnexpectedVariant {
        /// The variant index found
        found: u32,
        /// Type name for context
        type_name: &'static str,
    },

    /// Invalid duration (nanos >= 1_000_000_000)
    #[cfg(feature = "std")]
    InvalidDuration {
        /// Seconds component
        secs: u64,
        /// Nanoseconds component (should be < 1_000_000_000)
        nanos: u32,
    },

    /// Invalid SystemTime (before UNIX_EPOCH)
    #[cfg(feature = "std")]
    InvalidSystemTime {
        /// Duration before UNIX_EPOCH
        duration: std::time::Duration,
    },

    /// Checksum mismatch during data verification
    #[cfg(feature = "checksum")]
    ChecksumMismatch {
        /// Expected CRC32 checksum
        expected: u32,
        /// Found CRC32 checksum
        found: u32,
    },
}

/// Integer type enumeration for better error messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntegerType {
    /// u8 type
    U8,
    /// u16 type
    U16,
    /// u32 type
    U32,
    /// u64 type
    U64,
    /// u128 type
    U128,
    /// usize type
    Usize,
    /// i8 type
    I8,
    /// i16 type
    I16,
    /// i32 type
    I32,
    /// i64 type
    I64,
    /// i128 type
    I128,
    /// isize type
    Isize,
    /// Reserved/unknown type
    Reserved,
}

impl IntegerType {
    /// Convert unsigned type to corresponding signed type
    #[allow(dead_code)]
    pub(crate) const fn into_signed(self) -> Self {
        match self {
            Self::U8 => Self::I8,
            Self::U16 => Self::I16,
            Self::U32 => Self::I32,
            Self::U64 => Self::I64,
            Self::U128 => Self::I128,
            Self::Usize => Self::Isize,
            other => other,
        }
    }
}

impl fmt::Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::U128 => write!(f, "u128"),
            Self::Usize => write!(f, "usize"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::I128 => write!(f, "i128"),
            Self::Isize => write!(f, "isize"),
            Self::Reserved => write!(f, "reserved"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEnd { additional } => {
                write!(
                    f,
                    "Unexpected end of input (need {} more bytes)",
                    additional
                )
            }
            Error::InvalidData { message } => write!(f, "Invalid data: {}", message),
            Error::InvalidIntegerType { expected, found } => {
                write!(
                    f,
                    "Invalid integer type: expected {}, found {}",
                    expected, found
                )
            }
            Error::InvalidBooleanValue(v) => write!(f, "Invalid boolean value: {}", v),
            Error::InvalidCharEncoding(bytes) => {
                write!(f, "Invalid char encoding: {:?}", bytes)
            }
            #[cfg(feature = "alloc")]
            Error::Utf8 { inner } => write!(
                f,
                "UTF-8 error at byte offset {}: {}",
                inner.valid_up_to(),
                inner
            ),
            Error::LimitExceeded { limit, found } => {
                write!(f, "limit exceeded: found {} but limit is {}", found, limit)
            }
            #[cfg(feature = "std")]
            Error::Io { kind, message } => write!(f, "IO error ({:?}): {}", kind, message),
            Error::Custom { message } => write!(f, "{}", message),
            #[cfg(feature = "alloc")]
            Error::OwnedCustom { message } => write!(f, "{}", message),
            Error::OutsideUsizeRange(v) => {
                write!(f, "Value {} outside usize range", v)
            }
            Error::NonZeroTypeIsZero { non_zero_type } => {
                write!(f, "NonZero{} type decoded as zero", non_zero_type)
            }
            Error::UnexpectedVariant { found, type_name } => {
                write!(
                    f,
                    "unexpected variant for type `{}`: found discriminant {}",
                    type_name, found
                )
            }
            #[cfg(feature = "std")]
            Error::InvalidDuration { secs, nanos } => {
                write!(
                    f,
                    "Invalid duration: {} seconds, {} nanoseconds (nanos must be < 1,000,000,000)",
                    secs, nanos
                )
            }
            #[cfg(feature = "std")]
            Error::InvalidSystemTime { duration } => {
                write!(f, "Invalid SystemTime: {:?} before UNIX_EPOCH", duration)
            }
            #[cfg(feature = "checksum")]
            Error::ChecksumMismatch { expected, found } => {
                write!(
                    f,
                    "Checksum mismatch: expected 0x{:08x}, found 0x{:08x}",
                    expected, found
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io {
            kind: err.kind(),
            message: err.to_string(),
        }
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(inner: core::str::Utf8Error) -> Self {
        #[cfg(feature = "alloc")]
        {
            Error::Utf8 { inner }
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = inner;
            Error::InvalidData {
                message: "UTF-8 decoding error",
            }
        }
    }
}
