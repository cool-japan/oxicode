//! Validation middleware for oxicode.
//!
//! This module provides validation constraints for deserialization,
//! ensuring data integrity and security during decoding.
//!
//! ## Features
//!
//! - **Size Limits**: Limit string/collection lengths via [`Constraints::max_len`] /
//!   [`Constraints::min_len`]
//! - **Range Constraints**: Validate numeric values with [`Constraints::range`]
//! - **Non-empty**: Reject empty strings or collections via [`Constraints::non_empty`]
//! - **ASCII enforcement**: Require ASCII-only content with [`Constraints::ascii_only`]
//! - **Custom Validators**: User-defined logic via [`Constraints::custom`]
//! - **Collect or fail-fast**: Control error accumulation through [`ValidationConfig`]
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
//! // "hi" is too short (min_len 10) AND below max_len 5 is satisfied, but
//! // actually "hi".len() < 10 fails the first, and "hi".len() <= 5 passes the second.
//! // Use a value that fails both: "hello world" is > 5 and < 10 is false (len 11 >= 10).
//! // Simplest: "ab" fails min_len(10).
//! let result = validator.validate(&"ab".to_string());
//! assert!(result.is_err());
//! ```

pub mod constraints;
mod validator;

pub use constraints::{Constraint, Constraints, Range, ValidationResult};
pub use validator::{CollectionValidator, NumericValidator, ValidationError};

#[cfg(feature = "alloc")]
pub use validator::{FieldValidation, StringValidator, Validator};

/// Configuration for validation behavior.
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Whether to fail fast on the first validation error.
    pub fail_fast: bool,

    /// Maximum depth for nested structure validation.
    pub max_depth: usize,

    /// Whether to enable checksum verification.
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
