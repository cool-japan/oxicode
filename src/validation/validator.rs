//! Validator for applying constraints to decoded data.

#[cfg(feature = "alloc")]
use super::constraints::{Constraint, ValidationResult};
#[cfg(feature = "alloc")]
use super::ValidationConfig;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Error type for validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// The field that failed validation.
    pub field: &'static str,
    /// The error message.
    pub message: &'static str,
}

impl ValidationError {
    /// Create a new validation error.
    pub const fn new(field: &'static str, message: &'static str) -> Self {
        Self { field, message }
    }
}

impl core::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "validation failed for '{}': {}",
            self.field, self.message
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ValidationError {}

/// Type-erased constraint wrapper.
#[cfg(feature = "alloc")]
trait ConstraintWrapper<T: ?Sized>: Send + Sync {
    fn validate(&self, value: &T) -> ValidationResult;
    #[allow(dead_code)]
    fn description(&self) -> &'static str;
}

#[cfg(feature = "alloc")]
impl<T: ?Sized, C: Constraint<T> + Send + Sync + 'static> ConstraintWrapper<T> for C {
    fn validate(&self, value: &T) -> ValidationResult {
        Constraint::validate(self, value)
    }

    fn description(&self) -> &'static str {
        Constraint::description(self)
    }
}

/// Field validation entry for the validator.
#[cfg(feature = "alloc")]
pub struct FieldValidation<T> {
    /// The field name.
    pub field: &'static str,
    /// The constraint to apply.
    constraint: Box<dyn ConstraintWrapper<T>>,
}

#[cfg(feature = "alloc")]
impl<T> FieldValidation<T> {
    /// Create a new field validation.
    pub fn new<C: Constraint<T> + Send + Sync + 'static>(
        field: &'static str,
        constraint: C,
    ) -> Self {
        Self {
            field,
            constraint: Box::new(constraint),
        }
    }

    /// Validate the given value.
    pub fn validate(&self, value: &T) -> Result<(), ValidationError> {
        match self.constraint.validate(value) {
            ValidationResult::Valid => Ok(()),
            ValidationResult::Invalid(msg) => Err(ValidationError::new(self.field, msg)),
        }
    }
}

/// A validator that applies constraints to fields.
#[cfg(feature = "alloc")]
pub struct Validator<T> {
    validations: Vec<FieldValidation<T>>,
    config: ValidationConfig,
}

#[cfg(feature = "alloc")]
impl<T> Default for Validator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl<T> Validator<T> {
    /// Create a new validator.
    pub fn new() -> Self {
        Self {
            validations: Vec::new(),
            config: ValidationConfig::default(),
        }
    }

    /// Create a validator with custom configuration.
    pub fn with_config(config: ValidationConfig) -> Self {
        Self {
            validations: Vec::new(),
            config,
        }
    }

    /// Add a constraint for a field.
    pub fn add_constraint<C: Constraint<T> + Send + Sync + 'static>(
        &mut self,
        field: &'static str,
        constraint: C,
    ) {
        self.validations
            .push(FieldValidation::new(field, constraint));
    }

    /// Add a constraint and return self for chaining.
    pub fn constraint<C: Constraint<T> + Send + Sync + 'static>(
        mut self,
        field: &'static str,
        constraint: C,
    ) -> Self {
        self.add_constraint(field, constraint);
        self
    }

    /// Validate a single value against all constraints.
    pub fn validate(&self, value: &T) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        for validation in &self.validations {
            if let Err(e) = validation.validate(value) {
                if self.config.fail_fast {
                    return Err(alloc::vec![e]);
                }
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate and return the first error only.
    pub fn validate_first(&self, value: &T) -> Result<(), ValidationError> {
        for validation in &self.validations {
            validation.validate(value)?;
        }
        Ok(())
    }

    /// Get the number of constraints.
    pub fn constraint_count(&self) -> usize {
        self.validations.len()
    }

    /// Validate a value, returning it if valid or a `default` if any constraint fails.
    ///
    /// All constraints are evaluated against `value`. If every constraint passes the
    /// value is returned as-is; otherwise `default` is returned without allocating an
    /// error vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxicode::validation::{Validator, Constraints};
    ///
    /// let validator: Validator<i32> = Validator::new()
    ///     .constraint("value", Constraints::range(Some(0i32), Some(100i32)));
    ///
    /// assert_eq!(validator.validate_or_default(50, 0), 50);
    /// assert_eq!(validator.validate_or_default(-1, 0), 0);
    /// assert_eq!(validator.validate_or_default(101, 0), 0);
    /// ```
    pub fn validate_or_default(&self, value: T, default: T) -> T
    where
        T: Clone,
    {
        match self.validate_first(&value) {
            Ok(()) => value,
            Err(_) => default,
        }
    }

    /// Validate a value, returning a clone if valid or computing a default via `default_fn`
    /// if any constraint fails.
    ///
    /// This avoids evaluating the default expression when validation succeeds.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxicode::validation::{Validator, Constraints};
    ///
    /// let validator: Validator<i32> = Validator::new()
    ///     .constraint("value", Constraints::range(Some(0i32), Some(100i32)));
    ///
    /// assert_eq!(validator.validate_or_default_with(&50, || 42), 50);
    /// assert_eq!(validator.validate_or_default_with(&-1, || 42), 42);
    /// ```
    pub fn validate_or_default_with<F>(&self, value: &T, default_fn: F) -> T
    where
        T: Clone,
        F: FnOnce() -> T,
    {
        match self.validate_first(value) {
            Ok(()) => value.clone(),
            Err(_) => default_fn(),
        }
    }
}

/// Validator for string fields with common constraints.
#[cfg(feature = "alloc")]
pub struct StringValidator {
    max_len: Option<usize>,
    min_len: Option<usize>,
    non_empty: bool,
    ascii_only: bool,
}

#[cfg(feature = "alloc")]
impl Default for StringValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl StringValidator {
    /// Create a new string validator.
    pub const fn new() -> Self {
        Self {
            max_len: None,
            min_len: None,
            non_empty: false,
            ascii_only: false,
        }
    }

    /// Set maximum length.
    pub const fn max_len(mut self, max: usize) -> Self {
        self.max_len = Some(max);
        self
    }

    /// Set minimum length.
    pub const fn min_len(mut self, min: usize) -> Self {
        self.min_len = Some(min);
        self
    }

    /// Require non-empty string.
    pub const fn non_empty(mut self) -> Self {
        self.non_empty = true;
        self
    }

    /// Require ASCII-only characters.
    pub const fn ascii_only(mut self) -> Self {
        self.ascii_only = true;
        self
    }

    /// Validate a string.
    pub fn validate(&self, value: &str) -> Result<(), &'static str> {
        if self.non_empty && value.is_empty() {
            return Err("string must not be empty");
        }

        if let Some(min) = self.min_len {
            if value.len() < min {
                return Err("string below minimum length");
            }
        }

        if let Some(max) = self.max_len {
            if value.len() > max {
                return Err("string exceeds maximum length");
            }
        }

        if self.ascii_only && !value.is_ascii() {
            return Err("string must contain only ASCII characters");
        }

        Ok(())
    }
}

/// Validator for numeric values with range constraints.
#[derive(Debug, Clone, Copy)]
pub struct NumericValidator<T> {
    min: Option<T>,
    max: Option<T>,
}

impl<T> Default for NumericValidator<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> NumericValidator<T> {
    /// Create a new numeric validator.
    pub const fn new() -> Self {
        Self {
            min: None,
            max: None,
        }
    }

    /// Set minimum value.
    pub fn min(mut self, min: T) -> Self {
        self.min = Some(min);
        self
    }

    /// Set maximum value.
    pub fn max(mut self, max: T) -> Self {
        self.max = Some(max);
        self
    }

    /// Set both minimum and maximum.
    pub fn range(mut self, min: T, max: T) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}

impl<T: PartialOrd> NumericValidator<T> {
    /// Validate a value.
    pub fn validate(&self, value: &T) -> Result<(), &'static str> {
        if let Some(ref min) = self.min {
            if value < min {
                return Err("value below minimum");
            }
        }

        if let Some(ref max) = self.max {
            if value > max {
                return Err("value above maximum");
            }
        }

        Ok(())
    }
}

/// Validator for collections with size constraints.
#[derive(Debug, Clone, Copy, Default)]
pub struct CollectionValidator {
    max_len: Option<usize>,
    min_len: Option<usize>,
    non_empty: bool,
}

impl CollectionValidator {
    /// Create a new collection validator.
    pub const fn new() -> Self {
        Self {
            max_len: None,
            min_len: None,
            non_empty: false,
        }
    }

    /// Set maximum length.
    pub const fn max_len(mut self, max: usize) -> Self {
        self.max_len = Some(max);
        self
    }

    /// Set minimum length.
    pub const fn min_len(mut self, min: usize) -> Self {
        self.min_len = Some(min);
        self
    }

    /// Require non-empty collection.
    pub const fn non_empty(mut self) -> Self {
        self.non_empty = true;
        self
    }

    /// Validate a collection length.
    pub fn validate_len(&self, len: usize) -> Result<(), &'static str> {
        if self.non_empty && len == 0 {
            return Err("collection must not be empty");
        }

        if let Some(min) = self.min_len {
            if len < min {
                return Err("collection below minimum length");
            }
        }

        if let Some(max) = self.max_len {
            if len > max {
                return Err("collection exceeds maximum length");
            }
        }

        Ok(())
    }

    /// Validate a slice.
    pub fn validate<T>(&self, value: &[T]) -> Result<(), &'static str> {
        self.validate_len(value.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use super::super::constraints::{MaxLength, MinLength, Range};

    #[cfg(feature = "alloc")]
    use alloc::{format, string::String};

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validation_error() {
        let error = ValidationError::new("name", "too long");
        assert_eq!(error.field, "name");
        assert_eq!(error.message, "too long");
        assert_eq!(
            format!("{}", error),
            "validation failed for 'name': too long"
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validator_basic() {
        let validator: Validator<i32> =
            Validator::new().constraint("value", Range::new(Some(0), Some(100)));

        assert!(validator.validate(&50).is_ok());
        assert!(validator.validate(&-1).is_err());
        assert!(validator.validate(&101).is_err());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validator_multiple_constraints() {
        let mut validator: Validator<String> = Validator::new();
        validator.add_constraint("name", MaxLength::new(10));
        validator.add_constraint("name", MinLength::new(1));

        assert!(validator.validate(&String::from("hello")).is_ok());
        assert!(validator.validate(&String::from("")).is_err());
        assert!(validator
            .validate(&String::from("this is too long"))
            .is_err());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validator_fail_fast() {
        let config = ValidationConfig::new().with_fail_fast(true);
        let mut validator: Validator<String> = Validator::with_config(config);
        validator.add_constraint("name", MaxLength::new(5));
        validator.add_constraint("name", MinLength::new(10));

        let result = validator.validate(&String::from("hello world"));
        assert!(result.is_err());
        // With fail_fast, we only get one error
        assert_eq!(result.err().map(|e| e.len()), Some(1));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validator_collect_all_errors() {
        let config = ValidationConfig::new().with_fail_fast(false);
        let mut validator: Validator<String> = Validator::with_config(config);
        validator.add_constraint("name", MaxLength::new(5));
        validator.add_constraint("name", MinLength::new(20));

        let result = validator.validate(&String::from("hello world"));
        assert!(result.is_err());
        // Without fail_fast, we get all errors
        assert_eq!(result.err().map(|e| e.len()), Some(2));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_string_validator() {
        let validator = StringValidator::new()
            .min_len(1)
            .max_len(100)
            .non_empty()
            .ascii_only();

        assert!(validator.validate("hello").is_ok());
        assert!(validator.validate("").is_err());
        assert!(validator.validate("世界").is_err());
    }

    #[test]
    fn test_numeric_validator() {
        let validator = NumericValidator::new().min(0i32).max(100);

        assert!(validator.validate(&50).is_ok());
        assert!(validator.validate(&0).is_ok());
        assert!(validator.validate(&100).is_ok());
        assert!(validator.validate(&-1).is_err());
        assert!(validator.validate(&101).is_err());
    }

    #[test]
    fn test_collection_validator() {
        let validator = CollectionValidator::new()
            .min_len(1)
            .max_len(10)
            .non_empty();

        let short: [i32; 3] = [1, 2, 3];
        let long: [i32; 11] = [0; 11];
        let empty: [i32; 0] = [];

        assert!(validator.validate(&short).is_ok());
        assert!(validator.validate(&long).is_err());
        assert!(validator.validate(&empty).is_err());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validate_or_default_returns_value_when_valid() {
        let validator: Validator<i32> =
            Validator::new().constraint("n", Range::new(Some(0), Some(100)));

        assert_eq!(validator.validate_or_default(50, -1), 50);
        assert_eq!(validator.validate_or_default(0, -1), 0);
        assert_eq!(validator.validate_or_default(100, -1), 100);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validate_or_default_returns_default_when_invalid() {
        let validator: Validator<i32> =
            Validator::new().constraint("n", Range::new(Some(0), Some(100)));

        assert_eq!(validator.validate_or_default(-1, 42), 42);
        assert_eq!(validator.validate_or_default(200, 42), 42);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validate_or_default_no_constraints_always_returns_value() {
        let validator: Validator<i32> = Validator::new();
        assert_eq!(validator.validate_or_default(999, 0), 999);
        assert_eq!(validator.validate_or_default(-999, 0), -999);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validate_or_default_with_returns_value_when_valid() {
        let validator: Validator<i32> =
            Validator::new().constraint("n", Range::new(Some(0), Some(100)));

        let mut default_called = false;
        let result = validator.validate_or_default_with(&75, || {
            default_called = true;
            -1
        });
        assert_eq!(result, 75);
        assert!(
            !default_called,
            "default closure must not be called when valid"
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validate_or_default_with_invokes_closure_when_invalid() {
        let validator: Validator<i32> =
            Validator::new().constraint("n", Range::new(Some(0), Some(100)));

        let mut default_called = false;
        let result = validator.validate_or_default_with(&-5, || {
            default_called = true;
            99
        });
        assert_eq!(result, 99);
        assert!(
            default_called,
            "default closure must be called when invalid"
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_validate_or_default_with_string() {
        let validator: Validator<String> = Validator::new()
            .constraint("s", MaxLength::new(10))
            .constraint("s", MinLength::new(1));

        assert_eq!(
            validator.validate_or_default_with(&String::from("hello"), || String::from("default")),
            "hello"
        );
        assert_eq!(
            validator.validate_or_default_with(
                &String::from("this is way too long for the constraint"),
                || String::from("default")
            ),
            "default"
        );
        assert_eq!(
            validator.validate_or_default_with(&String::new(), || String::from("default")),
            "default"
        );
    }
}
