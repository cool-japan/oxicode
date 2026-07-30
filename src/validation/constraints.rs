//! Constraint definitions for field validation.

use core::ops::RangeBounds;

#[cfg(feature = "alloc")]
extern crate alloc;

/// Result of a validation check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// Validation passed.
    Valid,
    /// Validation failed with a message.
    Invalid(&'static str),
}

impl ValidationResult {
    /// Check if the result is valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Check if the result is invalid.
    #[inline]
    pub fn is_invalid(&self) -> bool {
        matches!(self, ValidationResult::Invalid(_))
    }

    /// Get the error message if invalid.
    pub fn error_message(&self) -> Option<&'static str> {
        match self {
            ValidationResult::Invalid(msg) => Some(msg),
            ValidationResult::Valid => None,
        }
    }
}

/// A constraint that can validate a value.
pub trait Constraint<T: ?Sized> {
    /// Validate the given value.
    fn validate(&self, value: &T) -> ValidationResult;

    /// Get a description of this constraint.
    fn description(&self) -> &'static str;
}

/// Maximum length constraint for strings and collections.
///
/// # Units
///
/// For `str`/`String`, the length is measured in **UTF-8 bytes**
/// (`str::len()`), not Unicode scalar values ("chars") or grapheme
/// clusters. This mirrors oxicode's own byte-oriented wire-size limits
/// elsewhere in the crate (e.g. [`crate::Error::LimitExceeded`]), so a
/// `MaxLength` constraint gives an accurate bound on the number of bytes a
/// validated string will occupy once encoded. It also means multi-byte
/// (e.g. CJK, emoji) strings are limited more tightly in character count
/// than single-byte-per-character (ASCII) strings for the same numeric
/// limit. Callers who need a character-count limit instead should compare
/// `value.chars().count()` directly rather than using this constraint.
#[derive(Debug, Clone, Copy)]
pub struct MaxLength {
    max: usize,
}

impl MaxLength {
    /// Create a new maximum length constraint.
    pub const fn new(max: usize) -> Self {
        Self { max }
    }
}

impl Constraint<str> for MaxLength {
    fn validate(&self, value: &str) -> ValidationResult {
        if value.len() <= self.max {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string exceeds maximum length")
        }
    }

    fn description(&self) -> &'static str {
        "maximum length constraint"
    }
}

#[cfg(feature = "alloc")]
impl Constraint<alloc::string::String> for MaxLength {
    fn validate(&self, value: &alloc::string::String) -> ValidationResult {
        if value.len() <= self.max {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string exceeds maximum length")
        }
    }

    fn description(&self) -> &'static str {
        "maximum length constraint"
    }
}

impl<T> Constraint<[T]> for MaxLength {
    fn validate(&self, value: &[T]) -> ValidationResult {
        if value.len() <= self.max {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("collection exceeds maximum length")
        }
    }

    fn description(&self) -> &'static str {
        "maximum length constraint"
    }
}

#[cfg(feature = "alloc")]
impl<T> Constraint<alloc::vec::Vec<T>> for MaxLength {
    fn validate(&self, value: &alloc::vec::Vec<T>) -> ValidationResult {
        if value.len() <= self.max {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("collection exceeds maximum length")
        }
    }

    fn description(&self) -> &'static str {
        "maximum length constraint"
    }
}

/// Minimum length constraint for strings and collections.
///
/// # Units
///
/// As with [`MaxLength`], the length of a `str`/`String` is measured in
/// **UTF-8 bytes**, not chars. See [`MaxLength`]'s documentation for
/// details and rationale.
#[derive(Debug, Clone, Copy)]
pub struct MinLength {
    min: usize,
}

impl MinLength {
    /// Create a new minimum length constraint.
    pub const fn new(min: usize) -> Self {
        Self { min }
    }
}

impl Constraint<str> for MinLength {
    fn validate(&self, value: &str) -> ValidationResult {
        if value.len() >= self.min {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string below minimum length")
        }
    }

    fn description(&self) -> &'static str {
        "minimum length constraint"
    }
}

#[cfg(feature = "alloc")]
impl Constraint<alloc::string::String> for MinLength {
    fn validate(&self, value: &alloc::string::String) -> ValidationResult {
        if value.len() >= self.min {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string below minimum length")
        }
    }

    fn description(&self) -> &'static str {
        "minimum length constraint"
    }
}

impl<T> Constraint<[T]> for MinLength {
    fn validate(&self, value: &[T]) -> ValidationResult {
        if value.len() >= self.min {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("collection below minimum length")
        }
    }

    fn description(&self) -> &'static str {
        "minimum length constraint"
    }
}

#[cfg(feature = "alloc")]
impl<T> Constraint<alloc::vec::Vec<T>> for MinLength {
    fn validate(&self, value: &alloc::vec::Vec<T>) -> ValidationResult {
        if value.len() >= self.min {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("collection below minimum length")
        }
    }

    fn description(&self) -> &'static str {
        "minimum length constraint"
    }
}

/// Range constraint for numeric values.
///
/// Internally a range remembers whether each bound is *inclusive* or
/// *exclusive*, so that [`Range::from_bounds`] can faithfully represent any
/// [`RangeBounds`] — including half-open ranges such as `0..100`, whose end
/// bound is [`core::ops::Bound::Excluded`] — without silently discarding the
/// exclusivity of the bound. This works uniformly for any `T: PartialOrd`,
/// not just integer types, and avoids the overflow pitfalls that a
/// "decrement the excluded bound by one" approach would hit at the type's
/// extremes (e.g. an excluded lower bound of `u8::MAX`).
#[derive(Debug, Clone)]
pub struct Range<T> {
    min: Option<T>,
    min_exclusive: bool,
    max: Option<T>,
    max_exclusive: bool,
}

impl<T: PartialOrd + Clone> Range<T> {
    /// Create a new range constraint from two *inclusive* bounds.
    ///
    /// Both `min` and `max`, when present, are treated as inclusive limits
    /// (matching the historical behavior of this constructor). Use
    /// [`Range::from_bounds`] to build a range that preserves exclusivity
    /// from a Rust [`RangeBounds`] value (e.g. `0..100`).
    pub fn new(min: Option<T>, max: Option<T>) -> Self {
        Self {
            min,
            min_exclusive: false,
            max,
            max_exclusive: false,
        }
    }

    /// Create a range from a [`RangeBounds`], preserving exclusivity.
    ///
    /// Unlike a naive conversion, an [`core::ops::Bound::Excluded`] end bound
    /// (as produced by e.g. `0..100`) is **not** silently dropped: the
    /// resulting constraint rejects the excluded boundary value itself while
    /// still constraining everything beyond it. For example:
    ///
    /// ```
    /// use oxicode::validation::{Constraint, Range};
    ///
    /// let half_open: Range<i32> = Range::from_bounds(&(0..100));
    /// assert!(half_open.validate(&0).is_valid());
    /// assert!(half_open.validate(&99).is_valid());
    /// assert!(half_open.validate(&100).is_invalid(), "100 is excluded");
    /// assert!(half_open.validate(&1000).is_invalid());
    /// ```
    pub fn from_bounds<R: RangeBounds<T>>(bounds: &R) -> Self
    where
        T: Clone,
    {
        use core::ops::Bound;

        let (min, min_exclusive) = match bounds.start_bound() {
            Bound::Included(v) => (Some(v.clone()), false),
            Bound::Excluded(v) => (Some(v.clone()), true),
            Bound::Unbounded => (None, false),
        };

        let (max, max_exclusive) = match bounds.end_bound() {
            Bound::Included(v) => (Some(v.clone()), false),
            Bound::Excluded(v) => (Some(v.clone()), true),
            Bound::Unbounded => (None, false),
        };

        Self {
            min,
            min_exclusive,
            max,
            max_exclusive,
        }
    }
}

impl<T: PartialOrd> Constraint<T> for Range<T> {
    fn validate(&self, value: &T) -> ValidationResult {
        if let Some(ref min) = self.min {
            let below = if self.min_exclusive {
                value <= min
            } else {
                value < min
            };
            if below {
                return ValidationResult::Invalid("value below minimum");
            }
        }

        if let Some(ref max) = self.max {
            let above = if self.max_exclusive {
                value >= max
            } else {
                value > max
            };
            if above {
                return ValidationResult::Invalid("value above maximum");
            }
        }

        ValidationResult::Valid
    }

    fn description(&self) -> &'static str {
        "range constraint"
    }
}

/// Non-empty constraint for strings and collections.
#[derive(Debug, Clone, Copy, Default)]
pub struct NonEmpty;

impl NonEmpty {
    /// Create a new non-empty constraint.
    pub const fn new() -> Self {
        Self
    }
}

impl Constraint<str> for NonEmpty {
    fn validate(&self, value: &str) -> ValidationResult {
        if !value.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string must not be empty")
        }
    }

    fn description(&self) -> &'static str {
        "non-empty constraint"
    }
}

#[cfg(feature = "alloc")]
impl Constraint<alloc::string::String> for NonEmpty {
    fn validate(&self, value: &alloc::string::String) -> ValidationResult {
        if !value.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string must not be empty")
        }
    }

    fn description(&self) -> &'static str {
        "non-empty constraint"
    }
}

impl<T> Constraint<[T]> for NonEmpty {
    fn validate(&self, value: &[T]) -> ValidationResult {
        if !value.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("collection must not be empty")
        }
    }

    fn description(&self) -> &'static str {
        "non-empty constraint"
    }
}

#[cfg(feature = "alloc")]
impl<T> Constraint<alloc::vec::Vec<T>> for NonEmpty {
    fn validate(&self, value: &alloc::vec::Vec<T>) -> ValidationResult {
        if !value.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("collection must not be empty")
        }
    }

    fn description(&self) -> &'static str {
        "non-empty constraint"
    }
}

/// Pattern constraint for ASCII strings.
#[derive(Debug, Clone, Copy)]
pub struct AsciiOnly;

impl AsciiOnly {
    /// Create a new ASCII-only constraint.
    pub const fn new() -> Self {
        Self
    }
}

impl Default for AsciiOnly {
    fn default() -> Self {
        Self::new()
    }
}

impl Constraint<str> for AsciiOnly {
    fn validate(&self, value: &str) -> ValidationResult {
        if value.is_ascii() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string must contain only ASCII characters")
        }
    }

    fn description(&self) -> &'static str {
        "ASCII-only constraint"
    }
}

#[cfg(feature = "alloc")]
impl Constraint<alloc::string::String> for AsciiOnly {
    fn validate(&self, value: &alloc::string::String) -> ValidationResult {
        if value.is_ascii() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid("string must contain only ASCII characters")
        }
    }

    fn description(&self) -> &'static str {
        "ASCII-only constraint"
    }
}

/// Constraint that validates using a custom function.
pub struct CustomValidator<T, F>
where
    F: Fn(&T) -> bool,
{
    validator: F,
    error_message: &'static str,
    description: &'static str,
    _phantom: core::marker::PhantomData<T>,
}

impl<T, F> CustomValidator<T, F>
where
    F: Fn(&T) -> bool,
{
    /// Create a new custom validator.
    pub const fn new(validator: F, error_message: &'static str, description: &'static str) -> Self {
        Self {
            validator,
            error_message,
            description,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<T, F> Constraint<T> for CustomValidator<T, F>
where
    F: Fn(&T) -> bool,
{
    fn validate(&self, value: &T) -> ValidationResult {
        if (self.validator)(value) {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(self.error_message)
        }
    }

    fn description(&self) -> &'static str {
        self.description
    }
}

/// Builder for common constraints.
pub struct Constraints;

impl Constraints {
    /// Create a maximum length constraint.
    pub const fn max_len(max: usize) -> MaxLength {
        MaxLength::new(max)
    }

    /// Create a minimum length constraint.
    pub const fn min_len(min: usize) -> MinLength {
        MinLength::new(min)
    }

    /// Create a non-empty constraint.
    pub const fn non_empty() -> NonEmpty {
        NonEmpty::new()
    }

    /// Create an ASCII-only constraint.
    pub const fn ascii_only() -> AsciiOnly {
        AsciiOnly::new()
    }

    /// Create a range constraint.
    pub fn range<T: PartialOrd + Clone>(min: Option<T>, max: Option<T>) -> Range<T> {
        Range::new(min, max)
    }

    /// Create a custom validator.
    pub const fn custom<T, F: Fn(&T) -> bool>(
        validator: F,
        error_message: &'static str,
        description: &'static str,
    ) -> CustomValidator<T, F> {
        CustomValidator::new(validator, error_message, description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_length_str() {
        let constraint = MaxLength::new(10);

        assert!(constraint.validate("hello").is_valid());
        assert!(constraint.validate("0123456789").is_valid());
        assert!(constraint.validate("01234567890").is_invalid());
    }

    #[test]
    fn test_min_length_str() {
        let constraint = MinLength::new(3);

        assert!(constraint.validate("abc").is_valid());
        assert!(constraint.validate("abcd").is_valid());
        assert!(constraint.validate("ab").is_invalid());
    }

    #[test]
    fn test_range_i32() {
        let constraint = Range::new(Some(0i32), Some(100i32));

        assert!(constraint.validate(&0).is_valid());
        assert!(constraint.validate(&50).is_valid());
        assert!(constraint.validate(&100).is_valid());
        assert!(constraint.validate(&-1).is_invalid());
        assert!(constraint.validate(&101).is_invalid());
    }

    #[test]
    fn test_range_from_bounds_half_open_excludes_end() {
        // `0..100` has an Excluded end bound; from_bounds must preserve the
        // exclusivity rather than silently dropping the upper limit.
        let constraint: Range<i32> = Range::from_bounds(&(0..100));

        assert!(constraint.validate(&0).is_valid());
        assert!(constraint.validate(&99).is_valid());
        assert!(constraint.validate(&100).is_invalid(), "100 is excluded");
        assert!(constraint.validate(&1000).is_invalid());
        assert!(constraint.validate(&-1).is_invalid());
    }

    #[test]
    fn test_range_from_bounds_inclusive_end() {
        // `0..=100` has an Included end bound.
        let constraint: Range<i32> = Range::from_bounds(&(0..=100));

        assert!(constraint.validate(&100).is_valid(), "100 is included");
        assert!(constraint.validate(&101).is_invalid());
    }

    #[test]
    fn test_range_from_bounds_exclusive_start() {
        use core::ops::Bound;
        // (0, 10] — excluded lower bound, included upper bound.
        let constraint: Range<i32> =
            Range::from_bounds(&(Bound::Excluded(0i32), Bound::Included(10i32)));

        assert!(constraint.validate(&0).is_invalid(), "0 is excluded");
        assert!(constraint.validate(&1).is_valid());
        assert!(constraint.validate(&10).is_valid());
        assert!(constraint.validate(&11).is_invalid());
    }

    #[test]
    fn test_range_from_bounds_unbounded() {
        // `..` places no constraint at all.
        let constraint: Range<i32> = Range::from_bounds(&(..));
        assert!(constraint.validate(&i32::MIN).is_valid());
        assert!(constraint.validate(&i32::MAX).is_valid());

        // `10..` constrains only the lower (inclusive) end.
        let lower: Range<i32> = Range::from_bounds(&(10..));
        assert!(lower.validate(&10).is_valid());
        assert!(lower.validate(&9).is_invalid());
        assert!(lower.validate(&i32::MAX).is_valid());
    }

    #[test]
    fn test_max_length_counts_bytes_not_chars() {
        // A 2-char multi-byte string occupies 6 UTF-8 bytes; MaxLength counts
        // bytes, so a limit of 5 rejects it even though it is only 2 chars.
        let constraint = MaxLength::new(5);
        assert_eq!("世界".chars().count(), 2);
        assert_eq!("世界".len(), 6);
        assert!(
            constraint.validate("世界").is_invalid(),
            "byte length (6) exceeds max 5"
        );
        // An equivalent-char-count ASCII string fits.
        assert!(constraint.validate("ab").is_valid());
    }

    #[test]
    fn test_non_empty() {
        let constraint = NonEmpty::new();

        assert!(constraint.validate("hello").is_valid());
        assert!(constraint.validate("").is_invalid());
    }

    #[test]
    fn test_ascii_only() {
        let constraint = AsciiOnly::new();

        assert!(constraint.validate("hello world").is_valid());
        assert!(constraint.validate("hello 世界").is_invalid());
    }

    #[test]
    fn test_custom_validator() {
        let is_even = Constraints::custom(
            |x: &i32| x % 2 == 0,
            "value must be even",
            "even number constraint",
        );

        assert!(is_even.validate(&2).is_valid());
        assert!(is_even.validate(&4).is_valid());
        assert!(is_even.validate(&3).is_invalid());
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::Valid;
        let invalid = ValidationResult::Invalid("test error");

        assert!(valid.is_valid());
        assert!(!valid.is_invalid());
        assert!(valid.error_message().is_none());

        assert!(!invalid.is_valid());
        assert!(invalid.is_invalid());
        assert_eq!(invalid.error_message(), Some("test error"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_vec_constraints() {
        let max_len = MaxLength::new(5);
        let min_len = MinLength::new(2);
        let non_empty = NonEmpty::new();

        let vec_short: alloc::vec::Vec<i32> = alloc::vec![1, 2];
        let vec_long: alloc::vec::Vec<i32> = alloc::vec![1, 2, 3, 4, 5, 6];
        let vec_empty: alloc::vec::Vec<i32> = alloc::vec![];

        assert!(max_len.validate(&vec_short).is_valid());
        assert!(max_len.validate(&vec_long).is_invalid());

        assert!(min_len.validate(&vec_short).is_valid());
        assert!(min_len.validate(&vec_empty).is_invalid());

        assert!(non_empty.validate(&vec_short).is_valid());
        assert!(non_empty.validate(&vec_empty).is_invalid());
    }

    #[test]
    fn test_constraints_builder() {
        let max = Constraints::max_len(100);
        let min = Constraints::min_len(1);
        let non_empty = Constraints::non_empty();
        let ascii = Constraints::ascii_only();
        let range = Constraints::range(Some(0u8), Some(255u8));

        assert!(max.validate("hello").is_valid());
        assert!(min.validate("h").is_valid());
        assert!(non_empty.validate("x").is_valid());
        assert!(ascii.validate("hello").is_valid());
        assert!(range.validate(&100u8).is_valid());
    }
}
