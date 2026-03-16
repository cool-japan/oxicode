//! The config module is used to change the behavior of oxicode's encoding and decoding logic.
//!
//! *Important* make sure you use the same config for encoding and decoding, or else oxicode will not work properly.
//!
//! To use a config, first create a type of [Configuration]. This type will implement trait [Config] for use with oxicode.
//!
//! ```
//! let config = oxicode::config::standard()
//!     // pick one of:
//!     .with_big_endian()
//!     .with_little_endian()
//!     // pick one of:
//!     .with_variable_int_encoding()
//!     .with_fixed_int_encoding();
//! ```
//!
//! See [Configuration] for more information on the configuration options.

pub(crate) use self::internal::*;
use core::marker::PhantomData;

/// The Configuration struct is used to build oxicode configurations. The [Config] trait is implemented
/// by this struct when a valid configuration has been constructed.
///
/// The following methods are mutually exclusive and will overwrite each other:
///
/// - [with_little_endian] and [with_big_endian]
/// - [with_fixed_int_encoding] and [with_variable_int_encoding]
///
/// [with_little_endian]: #method.with_little_endian
/// [with_big_endian]: #method.with_big_endian
/// [with_fixed_int_encoding]: #method.with_fixed_int_encoding
/// [with_variable_int_encoding]: #method.with_variable_int_encoding
#[derive(Copy, Clone, Debug)]
pub struct Configuration<E = LittleEndian, I = Varint, L = NoLimit> {
    _e: PhantomData<E>,
    _i: PhantomData<I>,
    _l: PhantomData<L>,
}

/// The default config for oxicode. By default this will be:
/// - Little endian
/// - Variable int encoding
pub const fn standard() -> Configuration {
    generate()
}

/// Creates the "legacy" default config compatible with bincode 1.0
/// - Little endian
/// - Fixed int length encoding
pub const fn legacy() -> Configuration<LittleEndian, Fixint, NoLimit> {
    generate()
}

impl<E, I, L> Default for Configuration<E, I, L> {
    fn default() -> Self {
        generate()
    }
}

const fn generate<E, I, L>() -> Configuration<E, I, L> {
    Configuration {
        _e: PhantomData,
        _i: PhantomData,
        _l: PhantomData,
    }
}

impl<E, I, L> Configuration<E, I, L> {
    /// Makes oxicode encode all integer types in big endian.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = oxicode::config::standard().with_big_endian();
    /// let bytes = oxicode::encode_to_vec_with_config(&1u32, config).expect("encode");
    /// assert!(!bytes.is_empty());
    /// ```
    pub const fn with_big_endian(self) -> Configuration<BigEndian, I, L> {
        generate()
    }

    /// Makes oxicode encode all integer types in little endian.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = oxicode::config::standard().with_little_endian();
    /// let bytes = oxicode::encode_to_vec_with_config(&1u32, config).expect("encode");
    /// assert!(!bytes.is_empty());
    /// ```
    pub const fn with_little_endian(self) -> Configuration<LittleEndian, I, L> {
        generate()
    }

    /// Makes oxicode encode all integer types with a variable integer encoding.
    ///
    /// Small values are encoded compactly — integers below 128 take a single byte.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = oxicode::config::standard().with_variable_int_encoding();
    /// let bytes = oxicode::encode_to_vec_with_config(&127u64, config).expect("encode");
    /// assert_eq!(bytes.len(), 1);
    /// ```
    pub const fn with_variable_int_encoding(self) -> Configuration<E, Varint, L> {
        generate()
    }

    /// Fixed-size integer encoding.
    ///
    /// * Fixed size integers are encoded directly
    /// * Enum discriminants are encoded as u32
    /// * Lengths and usize are encoded as u64
    ///
    /// # Examples
    ///
    /// ```
    /// let config = oxicode::config::standard().with_fixed_int_encoding();
    /// let bytes = oxicode::encode_to_vec_with_config(&1u32, config).expect("encode");
    /// assert_eq!(bytes.len(), 4); // u32 always occupies 4 bytes with fixed encoding
    /// ```
    pub const fn with_fixed_int_encoding(self) -> Configuration<E, Fixint, L> {
        generate()
    }

    /// Sets the byte limit to `limit`.
    ///
    /// Encoding or decoding more than `N` bytes will return an error.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = oxicode::config::standard().with_limit::<64>();
    /// let bytes = oxicode::encode_to_vec_with_config(&42u8, config).expect("encode within limit");
    /// assert!(!bytes.is_empty());
    /// ```
    pub const fn with_limit<const N: usize>(self) -> Configuration<E, I, Limit<N>> {
        generate()
    }

    /// Clear the byte limit.
    ///
    /// Removes any previously set byte limit, allowing arbitrarily large payloads.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = oxicode::config::standard().with_limit::<64>().with_no_limit();
    /// let bytes = oxicode::encode_to_vec_with_config(&42u8, config).expect("encode");
    /// assert!(!bytes.is_empty());
    /// ```
    pub const fn with_no_limit(self) -> Configuration<E, I, NoLimit> {
        generate()
    }
}

/// Indicates a type is valid for controlling the oxicode configuration
pub trait Config:
    InternalEndianConfig + InternalIntEncodingConfig + InternalLimitConfig + Copy + Clone
{
    /// This configuration's Endianness
    fn endianness(&self) -> Endianness;

    /// This configuration's Integer Encoding
    fn int_encoding(&self) -> IntEncoding;

    /// This configuration's byte limit, or `None` if no limit is configured
    fn limit(&self) -> Option<usize>;
}

impl<T> Config for T
where
    T: InternalEndianConfig + InternalIntEncodingConfig + InternalLimitConfig + Copy + Clone,
{
    fn endianness(&self) -> Endianness {
        <T as InternalEndianConfig>::ENDIAN
    }

    fn int_encoding(&self) -> IntEncoding {
        <T as InternalIntEncodingConfig>::INT_ENCODING
    }

    fn limit(&self) -> Option<usize> {
        <T as InternalLimitConfig>::LIMIT
    }
}

/// Big endian byte order
#[derive(Copy, Clone, Debug)]
pub struct BigEndian;

/// Little endian byte order
#[derive(Copy, Clone, Debug)]
pub struct LittleEndian;

/// Variable integer encoding
#[derive(Copy, Clone, Debug)]
pub struct Varint;

/// Fixed integer encoding
#[derive(Copy, Clone, Debug)]
pub struct Fixint;

/// No size limit
#[derive(Copy, Clone, Debug)]
pub struct NoLimit;

/// Size limit
#[derive(Copy, Clone, Debug)]
pub struct Limit<const N: usize>;

/// Endianness configuration
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Endianness {
    /// Little endian
    Little,
    /// Big endian
    Big,
}

/// Integer encoding configuration
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntEncoding {
    /// Variable-length encoding
    Variable,
    /// Fixed-length encoding
    Fixed,
}

mod internal {
    use super::*;

    /// Internal trait for endianness configuration
    pub trait InternalEndianConfig {
        /// The endianness setting
        const ENDIAN: Endianness;
    }

    impl InternalEndianConfig for LittleEndian {
        const ENDIAN: Endianness = Endianness::Little;
    }

    impl InternalEndianConfig for BigEndian {
        const ENDIAN: Endianness = Endianness::Big;
    }

    impl<I, L> InternalEndianConfig for Configuration<LittleEndian, I, L> {
        const ENDIAN: Endianness = Endianness::Little;
    }

    impl<I, L> InternalEndianConfig for Configuration<BigEndian, I, L> {
        const ENDIAN: Endianness = Endianness::Big;
    }

    /// Internal trait for integer encoding configuration
    pub trait InternalIntEncodingConfig {
        /// The integer encoding setting
        const INT_ENCODING: IntEncoding;
    }

    impl InternalIntEncodingConfig for Varint {
        const INT_ENCODING: IntEncoding = IntEncoding::Variable;
    }

    impl InternalIntEncodingConfig for Fixint {
        const INT_ENCODING: IntEncoding = IntEncoding::Fixed;
    }

    impl<E, L> InternalIntEncodingConfig for Configuration<E, Varint, L> {
        const INT_ENCODING: IntEncoding = IntEncoding::Variable;
    }

    impl<E, L> InternalIntEncodingConfig for Configuration<E, Fixint, L> {
        const INT_ENCODING: IntEncoding = IntEncoding::Fixed;
    }

    /// Internal trait for limit configuration
    pub trait InternalLimitConfig {
        /// The size limit, if any
        const LIMIT: Option<usize>;
    }

    impl InternalLimitConfig for NoLimit {
        const LIMIT: Option<usize> = None;
    }

    impl<const N: usize> InternalLimitConfig for Limit<N> {
        const LIMIT: Option<usize> = Some(N);
    }

    impl<E, I> InternalLimitConfig for Configuration<E, I, NoLimit> {
        const LIMIT: Option<usize> = None;
    }

    impl<E, I, const N: usize> InternalLimitConfig for Configuration<E, I, Limit<N>> {
        const LIMIT: Option<usize> = Some(N);
    }
}
