//! WAVE 2 hardening: trybuild UI tests for the two new `#[derive(Encode)]`
//! compile-error paths added in this wave (see `derive/src/encode_impl.rs`):
//!
//!   - `wave2_skip_variant_shape_mismatch.rs` — a variant-level
//!     `#[oxicode(skip)]` that aliases a successor with a different field
//!     shape (silent decode corruption / stream desync).
//!   - `wave2_duplicate_discriminant.rs` — two decodable variants resolving to
//!     the same discriminant (the second is unreachable on decode).
//!
//! Bless/refresh the expected `.stderr` files with:
//!
//! ```text
//! TRYBUILD=overwrite cargo test --features derive --test hardening_wave2_ui_test
//! ```
//!
//! Miri does not exercise proc-macro expansion / rustc subprocess spawning
//! (trybuild shells out to `rustc`), so this is skipped under Miri.

#![cfg(feature = "derive")]

#[test]
#[cfg_attr(miri, ignore)]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/wave2_skip_variant_shape_mismatch.rs");
    t.compile_fail("tests/ui/wave2_duplicate_discriminant.rs");
}
