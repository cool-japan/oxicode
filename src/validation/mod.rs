//! Post-decode validation constraints for oxicode.
//!
//! This module provides a standalone constraint/validator API for checking
//! already-decoded values. **It is not decode-time middleware**: nothing in
//! this module hooks into [`crate::Decode`] or the [`crate::de::Decoder`]
//! trait, and constructing a [`Validator`] does not change how encoding or
//! decoding behaves. Instead, the typical flow is:
//!
//! 1. Decode a value normally (e.g. with [`crate::decode_from_slice`], or,
//!    when the `checksum` feature is enabled, [`crate::checksum::decode_with_checksum`]).
//! 2. Explicitly call [`Validator::validate`] (or one of its variants) on the
//!    decoded value, at a point of the caller's choosing.
//!
//! [`Validator::decode_and_validate`] (available with the `checksum` feature)
//! bundles both steps for convenience, but it is still an explicit call the
//! application makes after obtaining the raw bytes — not an automatic hook
//! invoked by the core encode/decode path.
//!
//! ## Features
//!
//! - **Size Limits**: Limit string/collection lengths via [`Constraints::max_len`] /
//!   [`Constraints::min_len`]. For `str`/`String`, lengths are counted in
//!   **UTF-8 bytes**, not chars — see [`constraints::MaxLength`] for details.
//! - **Range Constraints**: Validate numeric values with [`Constraints::range`],
//!   including half-open Rust ranges via [`Range::from_bounds`] (exclusive
//!   bounds are preserved faithfully, not silently dropped).
//! - **Non-empty**: Reject empty strings or collections via [`Constraints::non_empty`]
//! - **ASCII enforcement**: Require ASCII-only content with [`Constraints::ascii_only`]
//! - **Custom Validators**: User-defined logic via [`Constraints::custom`]
//! - **Collect or fail-fast**: Control error accumulation through [`ValidationConfig`]
//! - **Depth-limited recursive validation**: Guard hand-rolled recursive
//!   validators against excessive nesting with
//!   [`Validator::validate_at_depth`] and [`ValidationConfig::max_depth`]
//! - **Default fallbacks**: Recover gracefully with [`Validator::validate_or_default`]
//!
//! ## Examples
//!
//! ### Basic field validation
//!
//! ```rust
//! use oxicode::validation::{Validator, Constraints};
//!
//! // Build a validator for i32 values in [0, 120].
//! let validator: Validator<i32> = Validator::new()
//!     .constraint("age", Constraints::range(Some(0i32), Some(120i32)));
//!
//! assert!(validator.validate(&50).is_ok());
//! assert!(validator.validate(&-1).is_err());
//! assert!(validator.validate(&200).is_err());
//! ```
//!
//! ### String constraints
//!
//! ```rust
//! use oxicode::validation::{Validator, Constraints};
//!
//! let mut validator: Validator<String> = Validator::new();
//! validator.add_constraint("username", Constraints::min_len(3));
//! validator.add_constraint("username", Constraints::max_len(32));
//! validator.add_constraint("username", Constraints::ascii_only());
//!
//! assert!(validator.validate(&"alice".to_string()).is_ok());
//! assert!(validator.validate(&"".to_string()).is_err());
//! assert!(validator.validate(&"x".to_string()).is_err());
//! ```
//!
//! ### Returning a default on failure
//!
//! ```rust
//! use oxicode::validation::{Validator, Constraints};
//!
//! let validator: Validator<i32> = Validator::new()
//!     .constraint("score", Constraints::range(Some(0i32), Some(100i32)));
//!
//! // Returns the value unchanged when valid.
//! assert_eq!(validator.validate_or_default(75, 0), 75);
//!
//! // Returns the default when validation fails.
//! assert_eq!(validator.validate_or_default(-5, 0), 0);
//!
//! // Lazy default via closure — only evaluated on failure.
//! assert_eq!(validator.validate_or_default_with(&200, || 100), 100);
//! ```
//!
//! ### Collecting all errors (non-fail-fast)
//!
//! ```rust
//! use oxicode::validation::{Validator, Constraints, ValidationConfig};
//!
//! let config = ValidationConfig::new().with_fail_fast(false);
//! let mut validator: Validator<String> = Validator::with_config(config);
//! validator.add_constraint("field", Constraints::min_len(10));
//! validator.add_constraint("field", Constraints::max_len(5));
//!
//! // "ab" is 2 bytes, so it fails the min_len(10) constraint. With
//! // fail-fast disabled the validator reports every failing constraint at
//! // once rather than stopping at the first.
//! let result = validator.validate(&"ab".to_string());
//! assert!(result.is_err());
//! ```

pub mod constraints;
mod validator;

pub use constraints::{Constraint, Constraints, Range, ValidationResult};
pub use validator::{CollectionValidator, NumericValidator, ValidationError};

#[cfg(feature = "alloc")]
pub use validator::{FieldValidation, StringValidator, Validator};

#[cfg(all(feature = "alloc", feature = "checksum"))]
pub use validator::ValidatedDecodeError;

/// Configuration for validation behavior.
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Whether to fail fast on the first validation error.
    ///
    /// Enforced by [`Validator::validate`].
    pub fail_fast: bool,

    /// Maximum recursion depth allowed when validating nested structures.
    ///
    /// [`Validator`] itself only validates a single flat value against its
    /// registered constraints — it has no built-in notion of "nesting".
    /// This limit is therefore enforced by
    /// [`Validator::validate_at_depth`], which callers use as the
    /// recursive entry point when they hand-write validation over
    /// nested/tree-shaped data (e.g. a validator for `Vec<Vec<T>>` that
    /// recurses one level per `Vec` layer). Passing a `depth` greater than
    /// `max_depth` fails immediately with a dedicated error instead of
    /// recursing further, guarding against stack exhaustion on deeply
    /// nested or adversarially crafted input.
    pub max_depth: usize,

    /// Whether to verify a CRC32 checksum before decoding.
    ///
    /// Only consulted by [`Validator::decode_and_validate`], which is
    /// available when the crate's `checksum` feature is enabled. When
    /// `true`, the input bytes passed to `decode_and_validate` are expected
    /// to be wrapped with [`crate::checksum::wrap_with_checksum`]; the
    /// checksum is verified via [`crate::checksum::decode_with_checksum`]
    /// before the payload is decoded. When `false`, the bytes are decoded
    /// directly with no checksum framing.
    pub verify_checksum: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            fail_fast: true,
            max_depth: 64,
            verify_checksum: false,
        }
    }
}

impl ValidationConfig {
    /// Create a new validation configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set fail-fast behavior.
    #[inline]
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    /// Set maximum validation depth.
    #[inline]
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Enable or disable checksum verification.
    #[inline]
    pub fn with_checksum(mut self, verify: bool) -> Self {
        self.verify_checksum = verify;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = ValidationConfig::default();
        assert!(config.fail_fast);
        assert_eq!(config.max_depth, 64);
        assert!(!config.verify_checksum);
    }

    #[test]
    fn test_config_builder() {
        let config = ValidationConfig::new()
            .with_fail_fast(false)
            .with_max_depth(128)
            .with_checksum(true);

        assert!(!config.fail_fast);
        assert_eq!(config.max_depth, 128);
        assert!(config.verify_checksum);
    }
}
