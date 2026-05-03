# Migration Guide: From bincode to OxiCode

This guide helps you migrate your project from bincode to OxiCode.

## Why Migrate?

OxiCode is the successor to bincode, offering:

- Modern Rust practices (2021 edition)
- Strict safety guarantees (no-unwrap policy)
- Better error handling
- Active maintenance and long-term support
- Performance improvements
- Backward compatibility options

## Quick Migration

### Step 1: Update Dependencies

Replace in your `Cargo.toml`:

```toml
# Before
[dependencies]
bincode = "2.0"

# After
[dependencies]
oxicode = "0.2"
```

### Step 2: Update Imports

```rust
// Before
use bincode::{Encode, Decode};

// After
use oxicode::{Encode, Decode};
```

### Step 3: Update Function Calls

Most bincode functions have direct equivalents in OxiCode:

```rust
// Before (bincode)
let encoded = bincode::serialize(&value)?;
let decoded: T = bincode::deserialize(&encoded)?;

// After (oxicode)
let encoded = oxicode::encode_to_vec_with_config(&value, oxicode::config::standard())?;
let (decoded, _len): (T, usize) = oxicode::decode_from_slice_with_config(&encoded, oxicode::config::standard())?;
```

## Configuration Migration

### Standard Configuration

```rust
// Before (bincode)
use bincode::config;
let config = config::standard();
let encoded = bincode::encode_to_vec(&value, config)?;

// After (oxicode)
let config = oxicode::config::standard();
let encoded = oxicode::encode_to_vec_with_config(&value, config)?;
```

### Legacy/Bincode-Compatible Configuration

If you need exact bincode compatibility:

```rust
let config = oxicode::config::legacy();  // Wire-format compatible with bincode 1.x default
```

## Feature Flags

OxiCode maintains similar feature flags to bincode:

```toml
[dependencies]
oxicode = { version = "0.2", features = ["derive"] }

# For no_std environments
oxicode = { version = "0.2", default-features = false, features = ["alloc"] }
```

## Using Serde Integration

OxiCode's serde support is optional. If you're using serde types:

### Step 1: Enable serde feature

```toml
[dependencies]
oxicode = { version = "0.2", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

### Step 2: Use serde module

```rust
// Before (bincode)
use bincode::serde::{encode_to_vec, decode_from_slice};
let bytes = encode_to_vec(&value, config::standard())?;
let (decoded, _) = decode_from_slice(&bytes, config::standard())?;

// After (oxicode) - almost identical!
use oxicode::serde::{encode_to_vec, decode_from_slice};
let bytes = encode_to_vec(&value, oxicode::config::standard())?;
let (decoded, _) = decode_from_slice(&bytes, oxicode::config::standard())?;
```

**Important**: Unlike bincode, oxicode requires explicit `features = ["serde"]` in Cargo.toml.

### Why is serde optional?

- **Smaller binary size**: Projects not using serde don't pay for it
- **no_std compatibility**: Serde-free usage in embedded environments
- **Flexible**: Use native `Encode`/`Decode` traits or serde, your choice

## Breaking Changes

### Error Types

OxiCode uses its own error types:

```rust
// Before (bincode)
use bincode::error::DecodeError;

// After (oxicode)
use oxicode::Error;
```

### Result Types

```rust
// Before (bincode)
fn process() -> Result<T, bincode::error::EncodeError> { ... }

// After (oxicode)
fn process() -> oxicode::Result<T> { ... }
```

## Compatibility Layer

OxiCode provides a compatibility crate for gradual migration:

```toml
[dependencies]
oxicode = "0.2"
oxicode_compatibility = "0.2"
```

This allows you to:
1. Read data encoded with bincode
2. Gradually migrate your codebase
3. Ensure data format compatibility

## Common Patterns

### Encoding to Vec<u8>

```rust
// Before
let bytes = bincode::serialize(&data)?;

// After
let bytes = oxicode::encode_to_vec_with_config(&data, oxicode::config::standard())?;
```

### Decoding from &[u8]

```rust
// Before
let data: MyStruct = bincode::deserialize(&bytes)?;

// After
let (data, _len): (MyStruct, usize) = oxicode::decode_from_slice_with_config(&bytes, oxicode::config::standard())?;
```

### Derive Macros

```rust
// Before
use bincode::{Encode, Decode};

#[derive(Encode, Decode)]
struct MyStruct {
    field: String,
}

// After
use oxicode::{Encode, Decode};

#[derive(Encode, Decode)]
struct MyStruct {
    field: String,
}
```

## Data Format Compatibility

By default, OxiCode uses a slightly different encoding format optimized for modern use cases. For exact bincode compatibility, use:

```rust
let config = oxicode::config::legacy();
```

This ensures:
- Same fixed-int encoding
- Same byte ordering (little-endian)
- Wire-format compatible with bincode 1.x default (equivalent to bincode 2.0's `config::legacy()` preset)

## Testing Your Migration

1. Keep both dependencies temporarily:

```toml
[dev-dependencies]
bincode = "2.0"
oxicode = "0.2"
```

2. Write compatibility tests:

```rust
#[test]
fn test_bincode_compatibility() {
    let data = MyStruct { field: "test".into() };

    // Encode with bincode (legacy mode = bincode 1.x wire format)
    let bincode_bytes = bincode::encode_to_vec(&data, bincode::config::legacy())
        .expect("bincode encode");

    // Decode with oxicode using matching legacy config
    let (decoded, _len): (MyStruct, usize) =
        oxicode::decode_from_slice_with_config(&bincode_bytes, oxicode::config::legacy())
            .expect("oxicode decode");

    assert_eq!(data, decoded);
}
```

## Performance Considerations

OxiCode is designed to be as fast or faster than bincode:

- Run benchmarks before and after migration
- Use `cargo bench` to compare performance
- Report any performance regressions as issues

## Getting Help

If you encounter issues during migration:

1. Check the [documentation](https://docs.rs/oxicode)
2. Look at [examples](examples/)
3. Open an issue on [GitHub](https://github.com/cool-japan/oxicode/issues)

## Timeline

We recommend:

1. **Week 1**: Add oxicode as a dev-dependency and test
2. **Week 2-3**: Gradually migrate code modules
3. **Week 4**: Remove bincode dependency
4. **Ongoing**: Monitor and optimize

## Rollback Plan

If you need to rollback:

1. Keep both dependencies during migration
2. Use feature flags to switch between implementations
3. Test thoroughly before removing bincode

```toml
[features]
default = ["use-oxicode"]
use-bincode = ["bincode"]
use-oxicode = ["oxicode"]
```

## Future-Proofing

OxiCode is committed to:

- Semantic versioning
- Long-term support (LTS) releases
- Clear migration paths for major versions
- Backward compatibility options

## Questions?

For questions or concerns about migration, please:

- Open a discussion on GitHub
- Check existing issues and PRs
- Reach out to the community

We're here to help make your migration smooth and successful!
