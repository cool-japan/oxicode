# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.3] - 2026-05-06

### Fixed
- Clippy `needless_borrows_for_generic_args` lint in the `compatibility` crate — removed redundant `&` borrow at 10 `bincode::encode_to_vec` call sites in `compatibility/src/lib.rs`. Restores `cargo clippy --all-features --workspace -- -D warnings` to a clean run, satisfying the no-warnings policy.
- Temp-file collisions across concurrent test invocations in `tests/async_advanced7_test.rs`, `tests/file_io_advanced15_test.rs`, `tests/file_io_advanced17_test.rs`, `tests/file_io_advanced29_test.rs`, `tests/file_io_advanced30_test.rs`, `tests/file_io_advanced31_test.rs`, and `tests/file_io_advanced32_test.rs`. Each file now uses the canonical `std::process::id()`-suffixed temp-path helper that was introduced for `file_io_advanced13_test.rs` in 0.2.2.

### Changed
- Bumped workspace and crate versions from 0.2.2 to 0.2.3 (branch-name-drives-version policy).

## [0.2.2] - 2026-05-03

### Added
- Shared domain types for `nested_structs_advanced17` test suite
- `deny.toml` configuration file for dependency policy enforcement (banning unsafe or incompatible crates)

### Changed
- Pinned `bincode` dev-dependency to `=2.0.1` to prevent incompatible API changes from 3.x
- Bumped `oxiarc` dependencies to version 0.2.7 (pure Rust compression upgrade)
- Updated CI configuration

### Fixed
- Improved temp file uniqueness in `file_io_advanced13` tests using process ID suffix
- Format issues across benchmark and test files (`cargo fmt --all`)

## [0.2.1] - 2026-03-16

### Changed
- Replaced `lz4_flex` with `oxiarc-lz4` (pure Rust) for LZ4 compression
- Replaced `zstd` (C FFI) with `oxiarc-zstd` (pure Rust) for Zstd compression/decompression
- Removed `compression-zstd-pure` feature and `ruzstd` dependency; `compression-zstd` is now fully pure Rust via `oxiarc-zstd`
- LZ4 compression now uses frame format instead of block format with prepended size
- Added `MAX_DECOMPRESSED_SIZE` (256 MB) safety limit for LZ4 decompression to prevent decompression bombs
- Upgraded `criterion` dev-dependency from 0.8.1 to 0.8.2
- Upgraded `proptest` dev-dependency from 1.8 to 1.10
- Updated MSRV to 1.74.0

### Removed
- `compression-zstd-pure` feature flag (no longer needed; `compression-zstd` is pure Rust)
- `ruzstd` dependency
- `lz4_flex` dependency
- `zstd` (C FFI) dependency
- `src/compression/ruzstd_impl.rs` module

### Quality
- All compression backends are now 100% pure Rust (COOLJAPAN Pure Rust Policy)
- No C/Fortran toolchain required for any feature

## [0.2.0] - 2026-03-16

### Added
- `SizeWriter` struct for pre-computing encoded size without allocating
- `encoded_size()` and `encoded_size_with_config()` top-level functions
- `encode_to_file()` / `decode_from_file()` file I/O functions (std feature)
- `encode_to_fixed_array::<N>()` for stack-allocated encoding
- `decode_value::<D>()` without size tracking
- `encode_bytes()` alias for encode_to_vec
- `EncodedBytes` / `EncodedBytesOwned` display wrappers with hex_dump()
- `BufferedIoReader<R>` wrapping BufReader for efficient streaming decode
- `DecodeIter<T>` lazy decoding iterator via `decode_iter_from_slice()`
- `encode_iter_to_vec()` / `encode_seq_to_vec()` / `encode_seq_into_slice()`
- Checksum feature: CRC32 integrity checking via `crc32fast`
- `compression-zstd-pure` feature using pure Rust `ruzstd` for decompression
- GitHub Actions CI with matrix (stable + MSRV 1.70.0, ubuntu + macos)
- Miri CI job for undefined behavior detection
- 4 cargo-fuzz targets for fuzzing
- BorrowDecode derive macro for zero-copy decoding
- Derive attributes: `skip`, `default`, `flatten`, `bytes`, `with`, `rename`, `seq_len`
- Container attributes: `bound`, `rename_all`, `crate`, `transparent`
- Variant attributes: `variant` (custom discriminant), `rename`
- Encode/Decode for `core::cmp::Ordering`, `core::convert::Infallible`, `core::ops::ControlFlow`
- BorrowDecode for `ControlFlow<B, C>`
- Encode/Decode for `LinkedList<T>`, `BTreeMap<K,V>`, `BTreeSet<T>`, `BinaryHeap<T>`
- BorrowDecode for `Box<T>`, `Box<[T]>`, `Box<str>`, `Arc<[T]>`, `Arc<str>`, `Rc<[T]>`, `Rc<str>`
- BorrowDecode for `String`, `char`, `&[i8]`
- Encode/Decode for `OsStr` / `OsString`
- Wrapping property-based tests via proptest
- `src/display.rs` module with hex formatting utilities
- Non-zero integer types BorrowDecode impls
- `encode_with`/`decode_with` individual field transformation function attributes
- `tag_type` container attribute: control enum discriminant width (u8/u16/u32/u64)
- `default_value` attribute: inline expression defaults for skipped fields
- `ManuallyDrop<T>` Encode/Decode/BorrowDecode implementations
- `PhantomData<T: ?Sized>` with unsized type bounds
- BorrowDecode for all atomic types, `Wrapping<T>`, `Reverse<T>`
- `encode_serde`/`decode_serde` convenience functions for serde integration
- i128/u128 support in serde serializer/deserializer
- `encode_versioned_value` / `decode_versioned_value` top-level convenience functions for versioned encoding
- `Cow<str>` and `Cow<[u8]>` BorrowDecode implementations for zero-copy borrowed decoding
- `encode_to_hex`/`decode_from_hex` for hex string encoding
- `encode_to_writer`/`decode_from_reader` std::io convenience functions
- `encode_to_vec_with_size_hint` for pre-allocated encoding
- `encoded_size`/`encoded_size_with_config` functions via `SizeWriter`
- `BorrowDecode` for `Box<[T]>`, `Box<str>`, `Arc<[T]>`, `Arc<str>`
- `Encode`/`Decode`/`BorrowDecode` for `RangeFull`, `RangeFrom<T>`, `RangeTo<T>`, `RangeToInclusive<T>`
- `encode_to_writer_with_config`/`decode_from_reader_with_config` convenience functions
- `encode_to_vec_checked`/`decode_from_slice_checked` checksum-verified encoding shortcuts
- `encode_copy` for Copy types (by-value convenience)
- Binary format specification tests (`tests/format_spec_test.rs`)
- Validation tests, SIMD large-array tests, config endianness tests

### Changed
- `rust-version` set to 1.70.0 (aligned with SciRS2 ecosystem MSRV)
- Improved `DecodeError::UnexpectedVariant` display with type name
- Improved `DecodeError::LimitExceeded` display message
- `DecodeError::LimitExceeded` display now shows "limit: N, found: M" for actionable diagnostics
- `DecodeError::Utf8Error` display now includes byte offset of invalid sequence
- `DecodeError::ChecksumMismatch` variant added

### Fixed
- Miri `format!` in no_std context (validation and versioning tests)
- Upgraded tokio to 1.50 to resolve RUSTSEC-2026-0007 (bytes integer overflow)
- Removed unused `futures-io` dependency

### Quality
- 19,929 tests passing (0 regressions, 0 warnings, 0 clippy errors)
- 42 tests pass under Miri `--no-default-features` (0 undefined behavior errors)
- `cargo publish --dry-run` passes for `oxicode_derive` and `oxicode`
- All files under 2000 lines (refactoring policy maintained)

## [0.1.0] - 2025-12-28

### Added

#### Core Features - 100% Bincode Compatibility
- **Binary Serialization**: Compact, efficient binary encoding/decoding
- **Derive Macros**: Automatic `Encode` and `Decode` trait derivation via `#[derive(Encode, Decode)]`
- **Configuration System**: Flexible encoding configurations (endianness, int encoding, size limits)
  - `config::standard()` - Little-endian + varint (default)
  - `config::legacy()` - Bincode 1.0 compatible (little-endian + fixed-int)
  - Custom configurations with builder pattern
- **no_std Support**: Works in embedded and resource-constrained environments
  - `default = ["std", "derive"]`
  - `alloc` feature for heap allocations without full std
  - Core functionality works in `no_std` environments
- **Zero-Copy Deserialization**: Efficient deserialization where possible
- **Comprehensive Type Support**: 112+ types with full Encode/Decode support
  - Primitives: all integer types, floats, bool, char
  - Collections: Vec, HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList, BinaryHeap
  - Special types: Option, Result, Box, Rc, Arc, Cow, PhantomData
  - Tuples up to 16 elements
  - Arrays of any size
  - NonZero types (NonZeroU8, NonZeroI32, etc.)
  - Duration, SystemTime (with `std` feature)
  - Ipv4Addr, Ipv6Addr, SocketAddr (with `std` feature)
- **100% Binary Compatibility**: Full binary format compatibility with bincode 2.0
  - Data encoded with bincode can be decoded with oxicode
  - Data encoded with oxicode can be decoded with bincode
  - 18/18 binary compatibility tests passing

#### 150% Enhancement Features - Beyond Bincode

##### Phase A: SIMD Optimization
- **SIMD-Accelerated Array Encoding**: Hardware-accelerated encoding for large arrays
  - Auto-detection of CPU capabilities (SSE2, AVX2, AVX-512)
  - Optimized for i32, u32, i64, u64, f32, f64 arrays
  - 64-byte aligned memory operations for optimal SIMD performance
  - Graceful fallback to scalar operations on unsupported platforms
  - Enable with `features = ["simd"]`
  - Significant performance improvements for bulk data (2-4x speedup on supported arrays)

##### Phase B: Compression
- **LZ4 Compression**: Fast compression with good ratios
  - `compression-lz4` feature for LZ4 support
  - `CompressedEncoder`/`CompressedDecoder` types
  - Automatic compression level selection
  - Magic bytes for format detection
  - Ideal for network transmission and storage
- **Zstd Compression**: Better compression ratios
  - `compression-zstd` feature for Zstandard support
  - Configurable compression levels (1-21)
  - Better compression than LZ4 at the cost of speed
  - `compression` feature enables LZ4 by default

##### Phase C: Schema Evolution & Versioning
- **Semantic Versioning**: Built-in version tracking
  - `Version` struct with major.minor.patch components
  - Automatic version embedding in encoded data
  - Version validation during decoding
- **Compatibility Checking**: Automatic migration detection
  - Forward/backward compatibility checking
  - Breaking change detection
  - Migration path validation
- **Schema Migration**: Graceful data evolution
  - Field additions with defaults
  - Field removals with fallbacks
  - Type changes with conversion functions
  - `VersionedEncoder`/`VersionedDecoder` types

##### Phase D: Streaming Serialization
- **Chunked Streaming (Sync)**: Incremental encoding/decoding
  - `StreamingEncoder`/`StreamingDecoder` for I/O streams
  - `BufferStreamingEncoder`/`BufferStreamingDecoder` for in-memory
  - Configurable chunk sizes
  - Progress tracking with callbacks
  - Automatic chunk headers with magic bytes
  - Memory-efficient for large datasets
- **Async Streaming**: Non-blocking async I/O support
  - `async-tokio` feature for tokio integration
  - `async-io` feature for generic async traits
  - `AsyncStreamingEncoder`/`AsyncStreamingDecoder` types
  - Cooperative cancellation via `CancellationToken`
  - `CancellableAsyncEncoder`/`CancellableAsyncDecoder` for interruptible operations
  - Full async/await support with tokio

##### Phase E: Validation Middleware
- **Constraint-Based Validation**: Type-safe validation at decode time
  - `Validator<T>` for applying constraints to decoded data
  - Built-in constraints: `MaxLength`, `MinLength`, `Range`, `NonEmpty`, `AsciiOnly`
  - Custom validators via `CustomValidator<T, F>`
  - Fail-fast or collect-all-errors modes
- **Specialized Validators**: Domain-specific validation helpers
  - `StringValidator` for string constraints (length, ASCII, non-empty)
  - `NumericValidator<T>` for range validation
  - `CollectionValidator` for collection size constraints
- **Validation Configuration**: Flexible validation behavior
  - `ValidationConfig` with fail-fast option
  - Max depth for nested structures
  - Optional checksum verification

#### Error Handling
- **Comprehensive Error Types**: Detailed error information
  - `Error::UnexpectedEnd` with bytes needed estimate
  - `Error::InvalidData` with descriptive messages
  - `Error::InvalidIntegerType` with expected/found types
  - `Error::LimitExceeded` for configuration violations
  - IO error integration (with `std` feature)
  - UTF-8 error integration
- **No Unwrap Policy**: All error cases properly handled
  - Zero `unwrap()` calls in codebase
  - All Results properly propagated
  - Safe error recovery paths

#### Development Quality
- **Code Statistics**:
  - 10,860 lines of Rust code
  - 61 Rust files
  - 211 tests passing (100% pass rate)
    - 18 binary compatibility tests
    - 193+ feature and integration tests
- **Code Quality**:
  - Zero compiler warnings
  - Zero clippy warnings
  - All files under 2000 lines (refactoring policy)
  - Comprehensive documentation
  - Extensive inline examples

### Changed
- Improved error messages with more context
- Optimized varint encoding/decoding performance
- Enhanced derive macro error reporting
- Better type inference in decode functions

### Fixed
- Edge cases in varint encoding for large integers
- Proper handling of zero-sized types
- Correct alignment for SIMD operations
- UTF-8 validation in string decoding

### Migration from Bincode
OxiCode is designed as a drop-in replacement for bincode 2.0:

```rust
// Before (bincode 2.0)
use bincode::{Encode, Decode, config};
let bytes = bincode::encode_to_vec(&value, config::standard())?;

// After (oxicode) - same API!
use oxicode::{Encode, Decode, config};
let bytes = oxicode::encode_to_vec(&value, config::standard())?;
```

Binary data is 100% compatible - you can mix libraries during migration.

See [MIGRATION.md](MIGRATION.md) for detailed migration guide.

### Security
- Configurable size limits to prevent DoS attacks
- Validation middleware for untrusted data
- Checksum verification option
- Max depth limits for nested structures
- Safe error handling with no panics

### Performance
- SIMD acceleration for array operations (2-4x speedup)
- Zero-copy deserialization where possible
- Efficient varint encoding for integers
- Minimal allocations during encoding/decoding
- Optimized for common usage patterns

### Documentation
- Comprehensive README with examples
- Migration guide from bincode
- API documentation with rustdoc
- Examples for all major features
- Inline code examples in documentation

## [Unreleased]

### Planned Features
- Additional compression algorithms (Brotli, Snappy)
- Schema registry for centralized version management
- Custom derive attributes for field-level validation
- Incremental deserialization for extremely large datasets
- Specialized encoders for scientific data (NumRS2 integration)
- Network protocol helpers for client-server communication

---

[0.2.3]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.3
[0.2.2]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.2
[0.2.1]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.1
[0.2.0]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.0
[0.1.0]: https://github.com/cool-japan/oxicode/releases/tag/v0.1.0
