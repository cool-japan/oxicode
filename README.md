# OxiCode

A modern binary serialization library for Rust - the successor to bincode.

[![CI](https://github.com/cool-japan/oxicode/workflows/CI/badge.svg)](https://github.com/cool-japan/oxicode/actions)
[![Crates.io](https://img.shields.io/crates/v/oxicode.svg)](https://crates.io/crates/oxicode)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![MSRV](https://img.shields.io/badge/MSRV-1.81.0-blue.svg)](https://github.com/cool-japan/oxicode)

Requires Rust **1.81.0** or later.

> **MSRV note:** 1.81.0 is the MSRV for the core library surface (default features, plus `alloc`/`std`/`derive`/`serde`/`checksum`/`simd`/`async-tokio` individually). The optional `compression-lz4` and `compression-zstd` features transitively depend on `oxiarc-core`, which requires Cargo's `edition2024` support and therefore needs Rust **1.85** or later to build; the same applies to some heavier dev-dependencies used only for benches/tests (not the published library). See the CI workflow's `msrv` job for the exact feature combinations verified at 1.81.0.

## About

OxiCode is a compact encoder/decoder pair that uses a binary zero-fluff encoding scheme. The size of the encoded object will be the same or smaller than the size that the object takes up in memory in a running Rust program.

This project serves as the spiritual successor to [bincode](https://github.com/bincode-org/bincode), maintaining binary compatibility for its verified core types (see [Known compatibility caveats](#known-compatibility-caveats)) while introducing modern improvements and advanced features that make it 150% better.

## Features

### Core Features (Bincode Compatible — see [caveats](#known-compatibility-caveats))

- **Compact encoding**: Efficient binary serialization with compact varint encoding
- **Fast**: Optimized for performance with zero-copy operations where possible
- **Flexible**: Support for various encoding configurations
- **Safe**: Strict no-unwrap policy, comprehensive error handling
- **Modern**: Built with latest Rust practices and 2021 edition features
- **no_std support**: Works in embedded and resource-constrained environments (with `alloc` feature)
- **Bincode compatibility**: Wire-format compatible with bincode 1.x default via `config::legacy()` (equivalent to bincode 2.0's `config::legacy()` preset)
- **BorrowDecode**: Zero-copy decoding via the `BorrowDecode` trait — decode into borrowed slices without allocation; generic `BorrowDecode<'de> for &'de [T]` supported for `u16`, `u32`, `u64`, `i16`, `i32`, `i64`, `f32`, `f64` via `BorrowableSliceElement`
- **encoded_size API**: Pre-calculate exact encoded byte length without allocating via `encoded_size` / `encoded_size_with_config`
- **Fixed-array encoding**: `encode_to_fixed_array::<N, _>(&value)` — encode directly into a stack-allocated `[u8; N]`
- **Sequence API**: `encode_seq_to_vec` / `decode_iter_from_slice` for streaming multi-item buffers
- **Checksum API**: `encode_with_checksum` / `decode_with_checksum` — CRC32 integrity protection (optional feature)
- **Hex display**: `encode_to_display` / `EncodedBytes` — display encoded bytes as hex without allocating a `String`

### 150% Enhancement Features (Beyond Bincode)

- **⚡ SIMD Array Codec**: Opt-in, hardware-accelerated bulk array encode/decode (`oxicode::simd`) — real AVX2/SSE2/NEON kernels, its own framing, *not* used by `encode_to_vec`/derive (see [SIMD-Accelerated Arrays](#simd-accelerated-arrays))
- **🗜️ Compression**: LZ4 (fast) and Zstd (better ratio) support
- **📦 Schema Evolution**: Version tracking and automatic migration
- **🌊 Streaming**: Chunked encoding/decoding for large datasets
- **⏱️ Async Streaming**: Non-blocking async I/O with tokio
- **✅ Validation**: Constraint-based validation middleware

See [Feature Comparison](#feature-comparison) below for detailed breakdown.

## Why OxiCode?

While bincode has served the Rust community well, OxiCode brings:

1. **Binary Compatibility**: Drop-in replacement with an identical binary format for verified core types (see [Known compatibility caveats](#known-compatibility-caveats))
2. **Modern Rust practices**: Built from the ground up with Rust 2021 edition
3. **Safety first**: Strict no-unwrap policy throughout the codebase
4. **Better error handling**: More informative error messages and comprehensive error types
5. **Advanced features**: an opt-in SIMD array codec, compression, streaming, async, validation - features bincode lacks
6. **Active maintenance**: Dedicated to long-term support and evolution

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
oxicode = "0.2"

# With serde support (for serde::Serialize/Deserialize types)
oxicode = { version = "0.2", features = ["serde"] }

# Optional features
oxicode = { version = "0.2", features = ["simd", "compression", "async-tokio"] }
```

### Feature Flags

This block matches `Cargo.toml` exactly:

```toml
default = ["std", "derive", "validation", "versioning"]
std = ["alloc", "serde?/std"]           # Standard library support
alloc = ["serde?/alloc"]                # Heap allocations (for no_std + alloc)
derive = ["oxicode_derive"]             # Derive macros for Encode/Decode/BorrowDecode
serde = ["dep:serde", "alloc"]          # Serde integration (optional)
simd = []                               # Opt-in vectorized array codec (oxicode::simd) — see below
compression-lz4 = ["alloc", "oxiarc-lz4"]   # LZ4 compression (pure Rust, fast)
compression-zstd = ["alloc", "oxiarc-zstd"] # Zstd compression (pure Rust via oxiarc-zstd)
compression = ["compression-lz4"]       # Convenience alias enabling LZ4 by default
async-tokio = ["std", "tokio"]          # Async streaming with Tokio
checksum = ["dep:crc32fast"]            # CRC32 integrity checking for encoded data
validation = []                         # Gates the oxicode::validation post-decode constraint module (on by default)
versioning = []                         # Gates the oxicode::versioning schema-version-header module (on by default)
```

**Note on `validation`/`versioning`:** these flags gate the real `oxicode::validation`
(post-decode constraint checking) and `oxicode::versioning` (schema-version-header) public
modules — both are on by default. Building with `--no-default-features` and not re-enabling
them removes access to `oxicode::validation`/`oxicode::versioning`.

## Quick Start

```rust
use oxicode::{Encode, Decode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let point = Point { x: 1.0, y: 2.0 };

    // Encode to bytes
    let encoded = oxicode::encode_to_vec(&point)?;

    // Decode from bytes
    let (decoded, _): (Point, _) = oxicode::decode_from_slice(&encoded)?;

    assert_eq!(point, decoded);
    Ok(())
}
```

## Derive Macros

OxiCode provides first-class derive macro support for `Encode`, `Decode`, and `BorrowDecode`.

```rust
use oxicode::{BorrowDecode, Encode};

// A struct with a borrowed field can only derive `BorrowDecode`, not `Decode` —
// there is no way for an owned `Decode::decode` call to hand back data borrowed
// from the input buffer with an arbitrary caller-chosen lifetime `'a`.
#[derive(Encode, BorrowDecode, Debug, PartialEq)]
struct Packet<'a> {
    id: u32,
    payload: &'a [u8],  // zero-copy via BorrowDecode
}
```

### Field Attributes

| Attribute | Description |
|-----------|-------------|
| `#[oxicode(skip)]` | Skip this field during encode/decode (uses `Default::default()` on decode) |
| `#[oxicode(default)]` | Use `Default::default()` if field is missing during decode |
| `#[oxicode(flatten)]` | Inline the fields of a nested struct |
| `#[oxicode(bytes)]` | Encode `Vec<u8>` or `&[u8]` as raw bytes without a length prefix |
| `#[oxicode(with = "module")]` | Use custom encode/decode functions from `module` |
| `#[oxicode(encode_with = "fn")]` | Use a custom encode function |
| `#[oxicode(decode_with = "fn")]` | Use a custom decode function |
| `#[oxicode(rename = "name")]` | Accepted for serde-migration source compatibility; **no-op on the wire** — oxicode's binary format is positional (fields carry no names), so this never changes the encoded bytes |
| `#[oxicode(seq_len = "u8"\|"u16"\|"u32"\|"u64")]` | Use a fixed-width length prefix for a `Vec<T>` field instead of the default `u64` length. **Wire-incompatible with bincode** |

### Container Attributes

| Attribute | Description |
|-----------|-------------|
| `#[oxicode(bound = "T: Trait")]` | Override the trait bounds on the generated impl |
| `#[oxicode(rename_all = "camelCase")]` | Accepted without error for serde-migration compatibility; **no-op on the wire** (fields are positional, so no naming convention affects the encoded bytes) |
| `#[oxicode(crate = "path")]` | Specify a custom path to the oxicode crate |
| `#[oxicode(transparent)]` | Treat a single-field struct as its inner type (no wrapper) |
| `#[oxicode(tag_type = "u8"\|"u16"\|"u32"\|"u64")]` | Set the integer type used for enum discriminants (default `u32`, bincode-compatible). Non-default widths are **wire-incompatible with bincode** |

### Variant Attributes

| Attribute | Description |
|-----------|-------------|
| `#[oxicode(variant = 5)]` | Assign a custom discriminant value to this variant. Native Rust explicit discriminants (`enum E { A = 5 }`) are **ignored** by the derive — use this attribute instead |
| `#[oxicode(rename = "name")]` | Accepted for serde-migration source compatibility; **no-op on the wire** (variants are positional in the binary format) |
| `#[oxicode(skip)]` (variant-level) | Exclude the variant from the discriminant space; on encode it aliases the next non-skipped variant's discriminant. A skipped variant with no following non-skipped variant is a compile error |

## Supported Types (120+)

OxiCode provides built-in `Encode`/`Decode` implementations for 120+ types:

### Primitives & Core
`bool`, `u8`–`u128`, `i8`–`i128`, `f32`, `f64`, `usize`, `isize`, `char`, `str`, `String`

### Option & Result
`Option<T>`, `Result<T, E>`

### Collections
`Vec<T>`, `HashMap<K,V>`, `HashSet<T>`, `BTreeMap<K,V>`, `BTreeSet<T>`, `BinaryHeap<T>`, `LinkedList<T>`, `VecDeque<T>`

### Smart Pointers & Slices
`Box<T>`, `Arc<T>`, `Rc<T>`, `Box<[T]>`, `Arc<[T]>`, `Arc<str>`, `Cow<'_, T>`

### Network & Time
`IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddr`, `SocketAddrV4`, `SocketAddrV6`, `Duration`, `SystemTime`

### Core Types
`Range<T>`, `RangeInclusive<T>`, `Bound<T>`, `Cell<T>`, `RefCell<T>`, `Wrapping<T>`

### Atomic Types
`AtomicBool`, `AtomicI8`–`AtomicI64`, `AtomicU8`–`AtomicU64`, `AtomicIsize`, `AtomicUsize`

### OS & Path
`OsStr`, `OsString`, `Path`, `PathBuf`

### Miscellaneous
`Ordering`, `Infallible`, `ControlFlow<B,C>`, `NonZeroU8`–`NonZeroU128`, `NonZeroI8`–`NonZeroI128`, `ManuallyDrop<T>`, `PhantomData<T>`, tuples (up to 12 elements), arrays `[T; N]`

## API Highlights

```rust
use oxicode::{Encode, Decode};

// Basic encode/decode
let bytes: Vec<u8> = oxicode::encode_to_vec(&value)?;
let (decoded, bytes_read): (T, usize) = oxicode::decode_from_slice(&bytes)?;

// File I/O
oxicode::encode_to_file(&value, "data.bin")?;
let decoded: T = oxicode::decode_from_file("data.bin")?;

// Pre-calculate size without allocating
let size: usize = oxicode::encoded_size(&value)?;

// Encode into a fixed-size stack array
let (arr, n): ([u8; 32], usize) = oxicode::encode_to_fixed_array(&value)?; // N inferred from the `[u8; 32]` annotation

// Sequence encoding — encode multiple items into one buffer
let bytes = oxicode::encode_seq_to_vec([item1, item2, item3].into_iter())?;
let items: Vec<T> = oxicode::decode_iter_from_slice::<T>(&bytes)?.collect::<Result<Vec<T>, _>>()?;

// Hex display without allocating a String
use oxicode::EncodedBytes;
println!("{}", EncodedBytes(&bytes)); // prints hex (space-separated bytes)
println!("{:x}", EncodedBytes(&bytes)); // prints a compact hex run
```

## Using with Serde

OxiCode provides optional serde integration for types that implement `serde::Serialize` and `serde::Deserialize`:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Encode using serde integration
    let encoded = oxicode::serde::encode_to_vec(&person, oxicode::config::standard())?;

    // Decode using serde integration
    let (decoded, _): (Person, _) = oxicode::serde::decode_from_slice(&encoded, oxicode::config::standard())?;

    assert_eq!(person.name, decoded.name);
    assert_eq!(person.age, decoded.age);
    Ok(())
}
```

**Enable serde feature in Cargo.toml:**
```toml
[dependencies]
oxicode = { version = "0.2", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

## Configuration

OxiCode supports various encoding configurations:

```rust
use oxicode::config;

// Standard configuration (default): little-endian + varint
let cfg = config::standard();

// Legacy bincode 1.0-compatible: little-endian + fixed-int
let cfg = config::legacy();

// Custom configuration
let cfg = config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .with_limit::<1048576>(); // 1MB limit

// Use with encoding/decoding
let bytes = oxicode::encode_to_vec_with_config(&value, cfg)?;
let (decoded, _) = oxicode::decode_from_slice_with_config(&bytes, cfg)?;
```

## Advanced Features

### Checksum (CRC32 Integrity)

Protect data against corruption with built-in CRC32 checksums:

```rust
use oxicode::checksum::{encode_with_checksum, decode_with_checksum};

let data = MyStruct { /* ... */ };

// Encode with CRC32 checksum appended
let bytes = encode_with_checksum(&data)?;

// Decode and verify checksum automatically — returns Err if checksum does not match
let (decoded, _): (MyStruct, _) = decode_with_checksum(&bytes)?;
```

Enable with `features = ["checksum"]`.

### SIMD-Accelerated Arrays

`oxicode::simd` is a small, **explicit opt-in** codec for contiguous arrays of
`f32`/`f64`/`i32`/`i64`/`u8`. It is **not** wired into `encode_to_vec`, the
derive macro, or the `Encode`/`Decode` traits — a `Vec<f64>` field in a
derived struct is encoded by the ordinary varint path whether or not the
`simd` feature is enabled. To use the vectorized path, call the module's
functions directly; the result has its own framing (an 8-byte little-endian
element count followed by little-endian element bytes) and is a **different
byte layout** from `encode_to_vec`'s output for the same data — the two are
not interchangeable.

```rust
use oxicode::simd::{encode_simd_array, decode_simd_array};

let readings: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
let encoded = encode_simd_array(&readings)?;      // oxicode::simd's own framing
let decoded: Vec<f64> = decode_simd_array(&encoded)?;
assert_eq!(readings, decoded);
```

On little-endian targets this dispatches, at runtime, to a real vectorized
bulk-copy kernel (AVX2 or the SSE2 baseline on x86_64, NEON on aarch64) via
`oxicode::simd::detect_capability()`; on big-endian targets it falls back to
a scalar per-element byte swap. Because the operation is a memory copy, it
is bandwidth-bound: measured on one x86_64/AVX2 machine in a release build,
the vectorized path was **~1.26x** faster than a naive per-element loop for
`Vec<f64>` encoding and **~1.44x** for the allocation-free into-buffer path —
useful, but nowhere near a fixed multiplier, and results vary by CPU and
array size. Enable with `features = ["simd"]`; see `examples/simd_arrays.rs`,
which measures the ratio on your own machine instead of printing a canned
number.

### Compression

Reduce size with LZ4 or Zstd compression. This is a standalone byte-level
API — it is not wired into `encode_to_vec`/`decode_from_slice` or any
`Config`; you call it explicitly as a second pass over already-encoded
bytes:

```rust
use oxicode::compression::{compress, decompress, Compression};

let encoded = oxicode::encode_to_vec(&value)?;

// LZ4 - fast compression
let compressed = compress(&encoded, Compression::Lz4)?;

// Zstd - better compression ratio (also: Compression::ZstdLevel(n) for 1-22)
let compressed = compress(&encoded, Compression::Zstd)?;

// decompress() caps the regenerated size at 256 MiB by default (bomb
// protection); use decompress_with_limit(data, max_output) to override it.
let decompressed = decompress(&compressed)?;
let (decoded, _): (MyStruct, _) = oxicode::decode_from_slice(&decompressed)?;
```

Enable with `features = ["compression-lz4"]` and/or `features =
["compression-zstd"]` (`features = ["compression"]` is a convenience alias
for `compression-lz4`). See `examples/compression.rs` for detailed usage,
including `compress_with_stats` for ratio/savings reporting.

### Streaming Serialization

Process large datasets incrementally. Streaming output uses oxicode's own
chunked container framing (a small header per chunk) — it is **not** the
same byte layout as `encode_to_vec`/`decode_from_slice` and is not
decodable by them (or by bincode); always read a streamed payload back with
a matching `StreamingDecoder`/`BufferStreamingDecoder`.

```rust
use oxicode::streaming::{StreamingEncoder, StreamingDecoder};

// Encode items one at a time (constructors are infallible; no `?` needed)
let mut encoder = StreamingEncoder::new(writer);
for item in large_dataset {
    encoder.write_item(&item)?;
}
encoder.finish()?;

// Decode items incrementally
let mut decoder = StreamingDecoder::new(reader);
while let Some(item) = decoder.read_item::<MyType>()? {
    process(item);
}
```

Use `StreamingEncoder::with_config(writer, streaming_config)` to customize
chunk/buffer sizing, or `new_with_config`/`new_with_configs` to also select a
non-default codec `Config` (it must match on both ends). See
`examples/streaming.rs` for detailed usage, including the in-memory
`BufferStreamingEncoder`/`BufferStreamingDecoder` variants.

### Async Streaming

Non-blocking async I/O with tokio, mirroring the sync streaming API and
sharing the same chunked framing (also not `encode_to_vec`-compatible):

```rust
use oxicode::streaming::AsyncStreamingEncoder;

// Async encoding (constructor is infallible; no `?` needed)
let mut encoder = AsyncStreamingEncoder::new(writer);
for item in dataset {
    encoder.write_item(&item).await?;
}
let writer = encoder.finish().await?;
```

`AsyncStreamingDecoder::new(reader)` is the matching decoder; `read_item`
returns `Ok(None)` only on a clean end-of-stream, so truncated input is
reported as an error rather than silently stopping. Cooperative cancellation
is available via `CancellationToken` + `CancellableAsyncEncoder`/
`CancellableAsyncDecoder`. Enable with `features = ["async-tokio"]`; see
`examples/async_streaming.rs` for a full round-trip and cancellation demo.

### Validation Middleware

Validate data during decoding. `Validator<T>` applies one or more
constraints to values of a single type `T` — construct one `Validator` per
field type you want to check, rather than mixing field names of different
types on a single validator:

```rust
use oxicode::validation::{Validator, Constraints};

// A validator over `String`, used to check the `name` field
let mut name_validator: Validator<String> = Validator::new();
name_validator.add_constraint("name", Constraints::max_len(100));

// A separate validator over `u8`, used to check the `age` field
let mut age_validator: Validator<u8> = Validator::new();
age_validator.add_constraint("age", Constraints::range(Some(0), Some(120)));

// Each returns Result<(), Vec<ValidationError>> — collects every failing
// constraint rather than stopping at the first (unless `fail_fast` is set
// via `ValidationConfig`).
name_validator.validate(&person.name)?;
age_validator.validate(&person.age)?;
```

See `examples/validation.rs` for detailed usage, including
`StringValidator`/`NumericValidator`/`CollectionValidator` convenience
wrappers and `ValidationConfig` (fail-fast mode, max recursion depth).

### Schema Evolution

Version your data formats and check compatibility on decode:

```rust
use oxicode::versioning::{Version, VersionedEncoder, VersionedDecoder};

let version = Version::new(1, 0, 0);
let payload = oxicode::encode_to_vec(&value)?;

// Stamp a version header onto already-encoded bytes
let encoder = VersionedEncoder::new(version);
let versioned_bytes = encoder.encode(&payload)?;

// Decode and (optionally) enforce a minimum compatible version
let decoder = VersionedDecoder::new().expect_version(version);
let (decoded_payload, decoded_version, compatibility) = decoder.decode(&versioned_bytes)?;
let (value, _): (MyStruct, _) = oxicode::decode_from_slice(&decoded_payload)?;
```

`encode_versioned`/`decode_versioned`/`decode_versioned_with_check` (and the
top-level `encode_versioned_value`/`decode_versioned_value` convenience
functions) are the underlying free-function API that `VersionedEncoder`/
`VersionedDecoder` wrap. See `examples/versioning.rs` for detailed usage.

## Migration from bincode

For matching configurations, oxicode and bincode 2.x produce byte-for-byte
identical output for the types verified by the `oxicode_compatibility` test
suite (primitives, strings, collections, tuples, options, derived structs
and enums, and the varint/zigzag boundary cases) — see
[Known compatibility caveats](#known-compatibility-caveats) below for the
specific standard-library types that currently diverge. Migration is
otherwise straightforward:

```rust
// Before (bincode 2.0)
use bincode::{Encode, Decode, config};
let bytes = bincode::encode_to_vec(&value, config::standard())?;
let (decoded, _) = bincode::decode_from_slice(&bytes, config::standard())?;

// After (oxicode) — same shape; the 2-arg config form is *_with_config
use oxicode::{Encode, Decode, config};
let bytes = oxicode::encode_to_vec_with_config(&value, config::standard())?;
let (decoded, _) = oxicode::decode_from_slice_with_config(&bytes, config::standard())?;
```

**Binary data is compatible when both sides use matching configs and avoid
the caveat types below** — you can mix libraries:
- Data encoded with bincode can be decoded with oxicode ✓ (see caveats)
- Data encoded with oxicode can be decoded with bincode ✓ (see caveats)

For detailed migration guide, see [MIGRATION.md](MIGRATION.md).

## Comparison with bincode

OxiCode is the spiritual successor to bincode. In **legacy mode** (`config::legacy()`), oxicode produces byte-for-byte identical output to the bincode 1.x default wire format (little-endian, fixed-int) — the same format targeted by bincode 2.0's `config::legacy()` preset — for the types verified in the compatibility test suite, making it a true drop-in replacement for those types. `config::standard()` is likewise verified byte-identical to bincode 2.x's `config::standard()` for the same set of types.

### Wire Format Compatibility

| Mode | Endianness | Int Encoding | Compatible with bincode? |
|------|-----------|--------------|--------------------------|
| `config::legacy()` | Little-endian | Fixed-width | Yes — byte-identical for verified types (see caveats) |
| `config::standard()` | Little-endian | Varint | Yes — byte-identical for verified types (see caveats); more compact than `legacy()` |

### Known compatibility caveats

A handful of standard-library types are **known** to encode differently from
bincode 2.0.1 (the pinned reference implementation). Each divergence below was
confirmed byte-for-byte against bincode's registry sources in the 2026-07
compatibility audit. They are deliberately left as-is for now: closing them
would change the wire bytes oxicode already produces for existing (valid)
data, which is a breaking change reserved for a future wire-format-breaking
release (0.3.0) with its own migration notes. Until then, treat this table as
the authoritative list of known bincode wire-format divergences. An executable
specification of each divergence exists as `#[ignore]`d cross-library tests
plus self-roundtrip golden vectors in the `oxicode_compatibility` crate and
`tests/hardening_m1_*`; the non-ignored parts of the `oxicode_compatibility`
suite are the executable definition of what is verified compatible today.
(A versioned `SPEC.md` covering every primitive's byte layout for both
configs remains a deferred follow-up.)

| Type | Known divergence from bincode 2.0.1 |
|------|-------------------------------------|
| `SystemTime` | oxicode encodes signed (zigzag) `i64` seconds + `u32` nanos relative to `UNIX_EPOCH` (pre-epoch values allowed); bincode encodes it as a `Duration` since `UNIX_EPOCH` (`u64` seconds + `u32` nanos) and errors on pre-epoch times. Diverges in **every** config |
| `SocketAddrV6` | oxicode always encodes `ip + port + flowinfo(u32) + scope_id(u32)`; bincode encodes only `ip + port`. Diverges in **every** config |
| `IpAddr`, `SocketAddr` | oxicode tags the enum variant with a `u8`; bincode uses a `u32` tag. Byte-level divergence manifests in fixed-int configs (`legacy()`); varint `standard()` happens to coincide for these small tag values |
| `Bound<T>` | oxicode tags `Unbounded`/`Included`/`Excluded` with a `u8`; bincode uses a `u32` tag — same fixed-int-config divergence as above |
| `Path` / `PathBuf` | oxicode encodes raw platform bytes on Unix and UTF-16 code units on Windows; bincode encodes a UTF-8 string (erroring on non-UTF-8 paths). Incompatible with bincode on both platforms, and the two oxicode platforms are not cross-compatible with each other |
| `Ordering` | oxicode's native codec encodes a signed `i8` (`-1`/`0`/`1`); bincode's derive-style enum uses a `u32` tag (`0`/`1`/`2`) — which is also what oxicode's own serde path emits, so the native and serde paths disagree with each other |
| `Duration` | decode-leniency difference (not byte layout): oxicode's decoder rejects `subsec_nanos >= 1_000_000_000`; bincode normalizes such values |

One further **API-level** (not wire-level) parity gap is tracked under the
same deferral: bincode 2's context API surface (`*_with_context` entry
points, derive support for a non-`()` `Context`, and serde-module
borrow/writer/reader entry points) is not yet mirrored.

If your data crosses the bincode/oxicode boundary and contains any of the
types above, pin both sides to the same library (or add your own
byte-level regression test) until these are formally reconciled in 0.3.0.
The full item-by-item deferral record lives in `TODO.md` (tagged
`⏸ DEFERRED 2026-07-17`).

### Feature Delta

| Capability | bincode | oxicode |
|-----------|---------|---------|
| Supported types | ~60 | **120+** |
| `BorrowDecode` / zero-copy | No | **Yes** |
| Derive field attributes | Limited | **9 attributes** |
| Container/variant attributes | No | **Yes** |
| Checksums (CRC32) | No | **Yes** (`checksum` feature) |
| Compression | No | **Yes** (LZ4, Zstd, pure-Rust Zstd) |
| Async streaming | No | **Yes** (`async-tokio` feature) |
| Validation middleware | No | **Yes** |
| Schema versioning | No | **Yes** |
| `encoded_size` | No | **Yes** |
| `encode_to_fixed_array` | No | **Yes** |
| `encode_seq_to_vec` / `decode_iter_from_slice` | No | **Yes** |
| `no_std` | Yes | **Yes** |
| Opt-in SIMD array codec (separate framing, not used by derive) | No | **Yes** (`simd` feature) |

## Feature Comparison

| Feature | bincode | rkyv | postcard | borsh | **oxicode** |
|---------|---------|------|----------|-------|-------------|
| Binary Compatibility | ✓ | ✗ | ✗ | ✗ | ✓\* |
| Zero-copy | ✗ | ✓ | ✗ | ✗ | ✓ |
| no_std | ✓ | ✓ | ✓ | ✓ | ✓ |
| Opt-in SIMD array codec | ✗ | ✗ | ✗ | ✗ | ✓ |
| Compression | ✗ | ✗ | ✗ | ✗ | ✓ |
| Async Streaming | ✗ | ✗ | ✗ | ✗ | ✓ |
| Validation | ✗ | ✗ | ✗ | ✗ | ✓ |
| Schema Evolution | ✗ | ✗ | ✗ | ✗ | ✓ |
| Varint Encoding | ✓ | ✗ | ✓ | ✗ | ✓ |

\* For the verified type set; known divergences are listed in
[Known compatibility caveats](#known-compatibility-caveats).

## Project Status

**Version 0.2.5 - Production Ready**

All core features and enhancements complete. See [CHANGELOG.md](CHANGELOG.md) for details.

**Statistics** (as of the 0.2.5 release; see [CHANGELOG.md](CHANGELOG.md) for details):
- **Lines of Code**: 523,697 (Rust source lines across 1,043 files)
- **Files**: 1,043 Rust files
- **Test Coverage**: 20,126 tests passing under `--all-features` (100% pass rate, 0 failed, 9 skipped); 15,539 under default features
  - `oxicode_compatibility` crate: 29 dedicated cross-library tests verifying byte-for-byte identical output against bincode 2.0.1 for the covered type set
  - 20,097+ feature, integration, property-based, and stress tests
- **Type Coverage**: 120+ types with full Encode/Decode support
- **Binary Compatibility**: verified through cross-library testing for the covered type set — known divergences are documented in [Known compatibility caveats](#known-compatibility-caveats)
- **Code Quality**: ✓ Zero unwrap(), ✓ Zero warnings, ✓ All files < 2000 lines

## Project Structure

This is a workspace with the following crates:

- `oxicode`: Main library crate
- `oxicode_derive`: Procedural macros for deriving Encode/Decode
- `oxicode_compatibility`: Compatibility tests and bincode interop

## Development Principles

OxiCode follows strict development principles:

- **No warnings policy**: All code must compile without warnings
- **No unwrap policy**: All error cases must be properly handled
- **Latest crates policy**: Use latest versions of dependencies
- **Workspace policy**: Proper workspace structure with shared dependencies
- **Refactoring policy**: Keep individual files under 2000 lines

## Performance

OxiCode is designed for performance:

- **Opt-in SIMD array codec**: `oxicode::simd` (with the `simd` feature) uses real vectorized bulk-copy kernels; measured ~1.2-1.4x over a naive scalar loop in one release-build benchmark — see [SIMD-Accelerated Arrays](#simd-accelerated-arrays) for the honest scope of this claim. It is not used by `encode_to_vec` or derive.
- **Zero-copy deserialization**: Where possible
- **Efficient varint encoding**: For integers
- **Minimal allocations**: During encoding/decoding
- **Benchmark suite**: Included in `benches/`

Run benchmarks:

```bash
cargo bench
```

## Testing

```bash
# Run all tests
cargo nextest run --all-features

# Run specific feature tests
cargo test --features simd
cargo test --features compression
cargo test --features async-tokio

# Run with no-std
cargo test --no-default-features --features alloc
```

## Examples

The `examples/` directory contains comprehensive examples (all 11 files):

- `basic_usage.rs` - Simple encoding/decoding
- `binary_format.rs` - Inspecting the binary wire format via hex dumps (`EncodedBytes`)
- `configuration.rs` - Configuration options
- `derive_attrs.rs` - Derive field/container/variant attributes
- `zero_copy.rs` - Zero-copy deserialization
- `simd_arrays.rs` - Opt-in SIMD array codec (measures its own speedup on your machine)
- `compression.rs` - LZ4 and Zstd compression
- `streaming.rs` - Chunked streaming
- `async_streaming.rs` - Async tokio streaming, including cancellation
- `validation.rs` - Validation middleware
- `versioning.rs` - Schema evolution

Run examples:

```bash
cargo run --example basic_usage
cargo run --example simd_arrays --release --features simd
cargo run --example compression --features compression-lz4
cargo run --example async_streaming --features async-tokio
```

## Fuzzing

OxiCode ships with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) targets in the `fuzz/` directory.

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Run the decode-slice fuzzer
cargo fuzz run fuzz_decode_slice

# Run roundtrip fuzzer
cargo fuzz run fuzz_roundtrip
```

Targets:
- `fuzz_decode_slice` — decode arbitrary bytes as multiple types (no panics)
- `fuzz_roundtrip` — encode + decode must be identity on structured data
- `fuzz_streaming` — streaming decoder on arbitrary bytes
- `fuzz_versioned` — versioned decode on arbitrary bytes

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Sponsorship

OxiCode is developed and maintained by **COOLJAPAN OU (Team Kitasan)**.

If you find OxiCode useful, please consider sponsoring the project to support continued development of the Pure Rust ecosystem.

[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-red?logo=github)](https://github.com/sponsors/cool-japan)

**[https://github.com/sponsors/cool-japan](https://github.com/sponsors/cool-japan)**

Your sponsorship helps us:
- Maintain and improve the COOLJAPAN ecosystem
- Keep the entire ecosystem (OxiBLAS, OxiFFT, SciRS2, etc.) 100% Pure Rust
- Provide long-term support and security updates

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

This project builds upon the excellent work done by the bincode team and community. We're grateful for their contributions to the Rust ecosystem.

## Related Projects

- [SciRS2](https://github.com/cool-japan/scirs) - Scientific computing library
- [NumRS2](https://github.com/cool-japan/numrs) - Numerical computing library
- [ToRSh](https://github.com/cool-japan/torsh) - PyTorch-like tensor library
- [OxiRS](https://github.com/cool-japan/oxirs) - RDF and SPARQL library
- [QuantRS2](https://github.com/cool-japan/quantrs) - Quantum computing library
