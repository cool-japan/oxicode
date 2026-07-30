//! WP-Y1 hardening: trybuild UI tests for the `#[derive(Encode/Decode/BorrowDecode)]`
//! compile-error paths hardened in earlier waves (see `derive/src/attrs.rs`,
//! `derive/src/encode_impl.rs`, `derive/src/decode_impl.rs`).
//!
//! Each fixture under `tests/ui/` is a standalone crate that must fail to
//! compile with a specific, human-readable diagnostic:
//!
//!   - `union_not_supported.rs` — `#[derive(Encode/Decode/BorrowDecode)]` on a `union`.
//!   - `transparent_on_enum.rs` — `#[oxicode(transparent)]` on an enum.
//!   - `transparent_multi_field_struct.rs` — `#[oxicode(transparent)]` on a
//!     struct with more than one field.
//!   - `variant_tag_exceeds_u32.rs` — `#[oxicode(variant = N)]` with `N` above
//!     `u32::MAX` and no `#[oxicode(tag_type = "u64")]` override.
//!   - `skip_bytes_named_enum_variant_field.rs` — `#[oxicode(skip, bytes)]` on
//!     a named field inside an enum variant.
//!
//! Bless/refresh the expected `.stderr` files with:
//!
//! ```text
//! TRYBUILD=overwrite cargo test -p oxicode --features derive --test hardening_y1_ui_test
//! ```
//!
//! then re-run without `TRYBUILD=overwrite` once to confirm the blessed
//! output is stable.
//!
//! Miri does not exercise proc-macro expansion / rustc subprocess spawning
//! (trybuild shells out to `rustc`), so this is skipped under Miri.

#![cfg(feature = "derive")]

#[test]
#[cfg_attr(miri, ignore)]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/union_not_supported.rs");
    t.compile_fail("tests/ui/transparent_on_enum.rs");
    t.compile_fail("tests/ui/transparent_multi_field_struct.rs");
    t.compile_fail("tests/ui/variant_tag_exceeds_u32.rs");
    t.compile_fail("tests/ui/skip_bytes_named_enum_variant_field.rs");
}
