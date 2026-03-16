# oxicode-compatibility

Internal binary format compatibility test suite for the [oxicode](https://crates.io/crates/oxicode) crate.

## Purpose

This crate verifies that **oxicode** produces byte-for-byte identical binary output to **bincode** (v2) for all supported types and configurations. It ensures that data serialized with one library can be deserialized by the other — and vice versa — without any modification.

## What Is Tested

- Primitive types: `u32`, `i64`, `bool`, `char`
- Heap types: `String`, `Vec<T>`, `Option<T>`
- Tuples and complex nested structures
- Derived `Encode`/`Decode` on structs and enums (unit, tuple, and struct variants)
- Configuration variants: standard, legacy, big-endian, fixed-int encoding
- Varint boundary values and zigzag-encoded signed integers
- Cross-decoding: encoding with oxicode and decoding with bincode, and vice versa

## Running the Tests

```bash
# From the repository root:
cargo test -p oxicode_compatibility

# Or from this directory:
cargo test
```

## Important Notes

- `publish = false` — this crate is **internal only** and will never be published to crates.io.
- It depends on both `oxicode` (path dependency) and `bincode` v2 directly.
- All tests live in `src/lib.rs` under `#[cfg(test)]`.
