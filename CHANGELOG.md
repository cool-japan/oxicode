# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.7] - Unreleased

## [0.2.6] - 2026-08-06

A follow-on hardening release to 0.2.5, concentrated in three areas: the serde
bridge (which had its own decode path and therefore missed several of 0.2.5's
protections), allocation bounds for length-prefixed and streaming input, and
decode contexts (previously usable only from hand-written impls). One change is
**wire-format relevant** — serde's `is_human_readable` now returns `false`,
restoring byte compatibility with `bincode::serde` for types that branch on it
(`IpAddr`, `uuid`, `chrono`, …). See **Fixed** for the migration note; nothing
else in this release changes the bytes produced for input that already encoded
correctly.

### Added

- **Bounded-length decode entry points**: `decode_from_buffered_read_limited`,
  `decode_from_std_read_limited`, and the serde counterpart
  `serde::decode_from_std_read_limited`. Hand the decoder the real length of
  the payload — a file size, an HTTP `Content-Length`, the frame length of a
  length-delimited protocol — and a field claiming more bytes than the stream
  can physically hold is rejected with `Error::UnexpectedEnd` *before* an
  allocation of that size is attempted, exactly as it already was for slice
  input. Reads past the budget also stop at the budget instead of running into
  the next message.
- **Byte-budgeted readers**: `de::IoReader::with_limit` / `set_limit` /
  `remaining_limit`, and `de::read::BufferedIoReader::with_limit`. These are
  what make the bounded entry points above composable with a hand-built reader.
- `de::Reader::remaining_bytes` and `de::Decoder::remaining_reader_bytes` — an
  exact upper bound on the input still obtainable, when it is cheaply knowable.
  Slice-backed readers answer exactly, budgeted IO readers answer with their
  remaining budget, and unbounded streams keep the default `None` (in which case
  length-prefixed decoding falls back to materializing incrementally).
- **Context-aware decode entry points**: `borrow_decode_from_slice_with_context`
  (zero-copy counterpart of `decode_from_slice_with_context`),
  `decode_from_std_read_with_context`, and `decode_from_de_reader_with_context`
  (the most general form — takes a reader the caller built, so a budgeted
  `IoReader::with_limit` can be combined with a context).
- **Decode contexts in the derive macros**. `#[oxicode(decode_context = "Ctx")]`
  and `#[oxicode(borrow_decode_context = "Ctx")]` generate `Decode<Ctx>` /
  `BorrowDecode<'de, Ctx>` instead of the unit context (`#[oxicode(context =
  "Ctx")]` sets both), and `#[oxicode(decode_context_generic)]` /
  `#[oxicode(borrow_decode_context_generic)]` / `#[oxicode(context_generic)]`
  make the generated impl generic over the context so the type decodes under
  *any* of them. The auto-generated bounds name the same context, and the
  context parameter itself is never given a `Decode` bound of its own.
- **Two new compile-time rejections in `#[derive(Encode)]`**, both replacing a
  silent mis-decode with a named error:
  - duplicate discriminants between decodable variants — `Decode`'s `match`
    takes the first arm, so the second variant was unreachable and its values
    silently decoded as the first;
  - a `#[oxicode(skip)]` variant whose field *types* differ from those of the
    successor whose discriminant it aliases — the skipped variant's payload
    would be read back through the successor's fields, corrupting it and
    desynchronizing everything after it in the stream.
- New fuzz target `fuzz_serde` covering the serde bridge end to end.

### Changed

- **All built-in `Decode` / `BorrowDecode` impls are now generic over the decode
  context** (`impl<Context> Decode<Context> for T`, where they previously
  implemented only `Decode<()>`). Primitives, arrays, tuples, `Option`,
  `Result`, `PhantomData`, the `core` / `alloc` / `std` types and the atomics
  can therefore be fields of a context-carrying or context-generic derived type.
  `Decode<Context = ()>` still defaults, so existing code compiles unchanged.
  `impl_borrow_decode!` gained matching forms: `(Ty)` forwards under any
  context, `(Ty, Ctx)` under one named context, `(Ty, unit_context)` pins to the
  historical unit context.
- `decode_from_file_with_config` now uses the file's size as the reader's byte
  budget, so a length prefix claiming more bytes than the file physically
  contains is rejected before that much memory is reserved. Sources with no
  determinable length (a named pipe, a `/proc` entry, a file longer than
  `usize::MAX` on a 32-bit target) decode unbounded, as before.
- Streaming chunk payloads are materialized incrementally in bounded 64 KiB
  steps instead of a single `vec![0u8; payload_len]` up front, in both
  `StreamingDecoder` and `AsyncStreamingDecoder`. A chunk header is 13 bytes; a
  peer that sends one and then stalls previously still cost the receiver a full
  bound-sized allocation (up to `MAX_CHUNK_SIZE`, 16 MiB by default) before a
  single payload byte had actually arrived. The allocation now tracks bytes
  really received. The async path keeps this cancellation-safe: the buffer grows
  before the `.await`, and `filled` — advanced only after a completed `read` —
  stays the sole record of what was confirmed received.
- Length-prefixed allocation no longer trusts the claimed length in one step.
  Element containers (`Vec`, `BinaryHeap`, `VecDeque`, …) pre-reserve at most
  4096 elements rather than the full decoded count, and still grow to whatever
  the input legitimately contains. Byte buffers (`String`, `Vec<u8>`, and the
  `#[oxicode(bytes)]` derive path) go through a shared bounded reader: the
  length is rejected outright when the reader knows its remaining input is
  shorter, and otherwise the buffer caps its first allocation at 16 MiB and
  grows in 16 KiB steps, each filled from the reader before the next is
  reserved. A reader's `remaining_bytes` is only an *upper* bound, so a
  deliberately generous budget ("1 GiB, to be safe") could otherwise still let
  nine bytes of forged length prefix commit a gigabyte at once.
- Derive-generated code no longer resolves items through the *invoking* crate's
  prelude: `Vec` and the `#[oxicode(bytes)]` read helper now route through a new
  `#[doc(hidden)]` `oxicode::__private` module, `Encode` calls are emitted as
  `#crate_path::Encode::encode(&value, encoder)`, and `Result` is written
  `::core::result::Result`. This fixes `#![no_std] + alloc` call sites (where
  `Vec` is never implicitly in scope) and removes the possibility of an
  unrelated same-named item in the calling crate shadowing the intended one.
- `AsyncStreamingEncoder::finish` is now documented as **not** cancellation-safe
  — a correction, as the module docs previously grouped it with the cancel-safe
  methods. It takes `self` by value, so a `finish` future dropped mid-await
  drops the encoder together with any buffered chunk data and the unwritten
  `End` marker, leaving the stream unterminated. Drive it to completion rather
  than racing it in a cancellable `select!`.
- `SimdCapability`'s derived `Ord` / `PartialOrd` is documented as meaningful
  only *within* one target-architecture family: `Neon` sorts numerically above
  `Avx512` although the two are never detected on the same build and are not
  comparable in strength. `matches!` is the safe idiom for any comparison not
  already confined to one architecture by `#[cfg]`.

### Fixed

- **serde: `is_human_readable` now returns `false`** on the serializer and both
  deserializers, matching `bincode` 2.0.1. It had been left at serde's `true`
  default, so every `Serialize` / `Deserialize` impl that branches on
  human-readability took the opposite branch from `bincode::serde`: `IpAddr`
  encoded as a length-prefixed ASCII string rather than a one-byte tag plus raw
  octets, and likewise for `uuid`, `chrono`, and any third-party type using the
  same idiom. It was also internally inconsistent — the human-readable branch is
  entitled to call `deserialize_any`, which this format rejects.
  **Migration:** bytes written by 0.2.5 or earlier for such types will not
  decode under 0.2.6, and vice versa; re-encode any persisted data containing
  them. Plain structs, enums, collections and primitives are entirely
  unaffected, and 0.2.6 is byte-compatible with `bincode::serde` again.
- **serde: infinite decode loop from nine bytes of input.** `SeqAccess` used the
  in-band sentinel `usize::MAX` to mean "length driven by the visitor, not by
  the wire". A wire length of `u64::MAX` collided with it, after which the
  counter never decremented and `next_element_seed` returned `Some(..)` forever.
  The sentinel is replaced by `Option<usize>`, with explicit `from_wire` /
  `from_schema` / `unbounded` constructors.
- **serde: uncatchable stack overflow on deeply nested input.** The bridge
  descends through visitor callbacks rather than through `Decode` impls, so it
  never passed through the recursion-depth guard added for the native path — a
  crafted payload for something like `enum Tree { Leaf, Node(Box<Tree>) }`
  recursed roughly one stack frame per input byte and aborted the process. The
  `Option`, newtype-struct, seq, map and enum descents are now wrapped in the
  guard, which stays balanced on the error path.
- **serde: unchecked length prefixes and unbudgeted containers.** Collection
  lengths are decoded with a checked `usize::try_from` (returning
  `Error::OutsideUsizeRange`) instead of a truncating `as usize` cast, and
  sequences and maps now claim their length against the configured decode limit
  before any element is decoded, each element unclaiming its share as it goes —
  the same convergence `Vec<T>::decode` already performed.
- **Undefined behaviour in `AlignedVec<T>` for a zero-sized `T`.** The
  allocating paths computed a zero-size `Layout` and passed it to
  `alloc::alloc::alloc` / `realloc` / `dealloc`, which the `GlobalAlloc`
  contract forbids — reachable from entirely safe downstream code, since
  `with_capacity` is a safe public constructor and `T` is unconstrained. ZSTs
  now keep `NonNull::dangling()` and never reach the allocator, while live
  elements are still dropped (a ZST may have a meaningful `Drop`).
- **Streaming decoders now poison on item-level errors**, not only chunk-level
  ones. `chunk.offset` was not advanced when an item failed to decode, so a
  caller looping past the error re-decoded the same bytes forever instead of
  receiving a deterministic failed-state error, contradicting the documented
  poisoning contract.
- **Forged chunk `item_count` is rejected up front.** A data chunk with
  `payload_len > 0` and `item_count > payload_len` is unrepresentable output —
  every non-zero-sized item consumes at least one payload byte — and is now
  refused before the decoder trusts `item_count` as its per-item loop bound.
  Zero-sized elements legitimately produce `payload_len == 0` with a non-zero
  `item_count` and are still accepted.
- **`#[oxicode(bytes)]` derive fields no longer reserve the claimed length up
  front.** The generated code did `Vec::with_capacity(len)` + `resize` from an
  attacker-controlled `u64`; it now goes through the same bounded
  `read_bytes_bounded` helper as the built-in `String` / `Vec<u8>` impls, so the
  length is either rejected against the reader's known remainder or the buffer
  is filled in bounded steps and fails on the first short read.
- Derived `Decode` / `BorrowDecode` impls call field decoders through an
  explicit `<Ty as Decode<_>>::decode` (rather than inherent-method syntax),
  which is what lets a derived type mix context-generic fields with concrete
  ones without inference failures.

### Documentation

- The serde module now documents which serde attributes this non-self-describing
  format cannot support, and why: `#[serde(flatten)]`, internally/adjacently
  tagged enums, `#[serde(untagged)]`, and anything reaching `deserialize_any` /
  `deserialize_ignored_any` fail at runtime; `#[serde(skip_serializing_if)]` is
  worse — it compiles, appears to work, and silently desynchronizes every field
  after the omitted one, so it must not be used with oxicode. Also notes that
  `collect_str` is deliberately *not* rejected (unlike `bincode::serde`), so
  `Display`-based types serialize as length-prefixed UTF-8 strings.
- `BorrowableSliceElement`'s safety invariants are stated precisely: implementors
  must have **no padding bytes**, and alignment (not just size) must be
  compatible with reading directly out of an arbitrary buffer at an
  attacker-influenced offset — the built-in impl enforces that with a runtime
  address check, and a custom implementor must provide an equivalent.
- `Decoder::remaining_reader_bytes` documents why `config.with_limit::<N>()`
  cannot substitute for a reader-supplied bound in either direction (claims do
  not track consumed bytes exactly under variable-length encodings, and the
  remaining budget is already debited by the time a bound would be consulted).

### Dependencies
- `oxiarc-lz4` / `oxiarc-zstd` 0.4.0 → 0.4.1 (upstream release).

## [0.2.5] - 2026-07-30

A hardening-focused release: a coordinated internal audit found and fixed a
number of denial-of-service, panic, integer-overflow, and silent-corruption
issues across decoding, streaming, compression, checksum, the derive macros,
and serde integration; replaced a fabricated "SIMD" code path with real
hardware kernels; and overhauled the crate documentation for accuracy. No
serialized wire-format bytes changed for any input that was already valid —
every behavior change below is a **new rejection of previously-invalid,
malicious, or overflow-prone input**, or a bookkeeping/allocation-timing fix
on the error path. This is called out explicitly per item below because,
unlike a pure bug fix, "an input that used to decode now returns an error"
is a compatibility-relevant change for callers who were unknowingly relying
on the old (unsafe) leniency.

### Security

- **Container/collection allocation-DoS**: `HashMap`, `HashSet`, `BinaryHeap`,
  `VecDeque`, `BTreeMap`, `BTreeSet`, `LinkedList`, and `PathBuf` (Windows
  UTF-16 path) decoding now call `decoder.claim_container_read` against the
  configured decode-size limit *before* allocating, matching `Vec`'s existing
  behavior. A forged huge length prefix under the configured limit is now
  rejected up front instead of driving an unbounded allocation.
- **Streaming allocation-DoS**: `StreamingDecoder`/`AsyncStreamingDecoder`/
  `BufferStreamingDecoder` now reject any chunk header whose declared payload
  length exceeds `min(max_chunk_size, configured limit)` *before* allocating a
  buffer for it. A forged `0xFFFFFFFF` chunk header no longer forces a ~4 GiB
  allocation attempt.
- **Streaming truncation detection**: a stream that ends (EOF, or the buffer
  is exhausted) without a terminal `End` chunk now returns
  `Error::UnexpectedEnd` instead of being silently treated as a clean,
  complete stream.
- **Streaming cancellation safety**: `AsyncStreamingEncoder`/
  `AsyncStreamingDecoder` now drive I/O through a persisted, resumable
  fill-cursor using the cancel-safe `AsyncReadExt::read`/`AsyncWriteExt::write`
  primitives (rather than `read_exact`/`write_all`), so a dropped/cancelled
  `read_item`/`write_item`/`finish` future no longer loses or duplicates
  bytes.
- **Streaming poison-on-error**: once a streaming decoder hits a load,
  truncation, or limit error, it now latches a poisoned state and returns a
  deterministic `Error::InvalidData` on any further read, instead of
  potentially misinterpreting stale payload bytes as a new chunk header.
- **Zstd decompression-bomb protection**: `compression::decompress`/
  `decompress_with_limit` now pre-scan the frame header and block table
  before decompressing, rejecting frames that omit `Frame_Content_Size`,
  declare a `content_size` above the configured cap, or whose summed
  per-block regenerated-size upper bound exceeds the declared `content_size`
  by more than one block. This bounds peak memory to
  `content_size + ~128 KiB` regardless of what the frame's blocks actually
  decode to.
- **LZ4 unbounded-reservation protection**: LZ4 frames that clear the
  `Content_Size` flag (or use a non-standard magic) are now rejected before
  the decoder would otherwise make an unbounded up-front output reservation.
- **Cross-feature codec mismatch**: decompressing a payload tagged as LZ4
  when only `compression-zstd` is enabled (or vice versa) now returns a clear
  "codec not enabled" error naming the missing feature, instead of
  misdispatching.
- **Checksum length-overflow panic**: `verify_checksum` computed
  `HEADER_SIZE + stored_len` with an unchecked add on an attacker-controlled
  length; a forged length near `usize::MAX` could wrap (release builds) and
  defeat the bounds check, or panic outright (debug builds). Now uses
  `checked_add`, returning `Error::InvalidData` on overflow.
- **`char` decode validation**: ported bincode's `UTF8_CHAR_WIDTH` first-byte
  validation so overlong/invalid multi-byte lead bytes are rejected during
  `char` decode instead of being accepted and then failing (or succeeding
  incorrectly) deeper in UTF-8 validation.
- **Array decode leak fix**: `[T; N]` decode now uses a drop-guard
  (tracking the initialized-so-far count) so that an error partway through
  decoding an array's elements drops the already-initialized elements instead
  of leaking them.
- **`OsStr::encode` no longer silently lossy-converts**: non-UTF-8 `OsStr`
  values previously went through `to_string_lossy()` (silently mangling the
  bytes); `encode` now returns `Error::InvalidData` for non-UTF-8 input.
  Byte output for valid UTF-8 input is unchanged.
- **Checked length/index conversions**: every remaining
  `u64::decode(..)? as usize` (or `as u32`) length read in `impl_std.rs` /
  `impl_alloc.rs` / derive-generated code (`HashMap`, `HashSet`, `CString`,
  `PathBuf`, `Vec`, `String`, `BTreeMap`, `BTreeSet`, `BinaryHeap`,
  `VecDeque`, `LinkedList`, and generated seq/bytes fields) now uses a
  checked `usize::try_from`, returning `Error::OutsideUsizeRange` instead of
  silently truncating on 32-bit targets.
- **Fixed-width integer decode**: `usize`/`isize` decode now uses checked
  `try_from` against the platform width instead of an unchecked cast.
- **Decode recursion-depth guard**: `Box`, `Rc`, `Arc`, and the standard
  collection decoders now enforce a recursion-depth limit (default 128,
  configurable via `DecoderImpl::set_recursion_limit`), returning
  `Error::LimitExceeded` for adversarially deep nesting instead of risking
  stack exhaustion.
- **Derive: seq_len claim-before-allocate**: the generated `Vec` decode path
  for `#[oxicode(seq_len = ...)]` fields now calls `claim_container_read`
  (accounting for the element type) before allocating, and reserves only a
  capped amount up front, closing an allocation-DoS gap in generated code.
- **Derive: mutually-exclusive attribute combinations rejected at compile
  time**: `#[oxicode(bytes)]` combined with `skip`/`default`/`seq_len`/`with`/
  `encode_with`/`decode_with` (and `skip`/`default` combined with the `with`
  family) are now compile errors instead of generating confusing or broken
  code.
- **Derive: unrepresentable skipped-variant encode fixed**: a skipped enum
  variant with no following non-skipped variant to alias onto is now a
  compile error, instead of silently generating an encode arm whose output
  could never be decoded back.
- **SIMD fabrication fixed**: `oxicode::simd` previously only exercised its
  vectorized code paths for some inputs while silently falling through to a
  scalar path for others without saying so (and shipped a fabricated
  "2-4x speedup" claim not backed by any measurement). It now always routes
  through genuine AVX2/SSE2 (x86_64) or NEON (aarch64) hardware kernels on
  little-endian targets, with runtime CPU-capability detection under `std`
  and compile-time dispatch under `no_std`. A production `.expect("invalid
  layout")` in `AlignedVec::layout_for_capacity` was replaced with the
  sanctioned `alloc::alloc::handle_alloc_error` path.
- **Serde error preservation**: serde-path decode/encode errors previously
  collapsed to a generic `"Failed to ..."` string; the real underlying
  `oxicode::Error` (e.g. `Error::UnexpectedEnd` for truncated input) is now
  preserved end-to-end.

### Added

- `compression::decompress_with_limit` and `DEFAULT_MAX_DECOMPRESSED_SIZE`
  for callers who need a decompression-bomb cap other than the 256 MiB
  default.
- `new_with_config`/`new_with_configs` constructors (selecting a non-default
  codec `Config`) across `StreamingEncoder`/`StreamingDecoder`/
  `BufferStreamingEncoder`/`BufferStreamingDecoder`/`AsyncStreamingEncoder`/
  `AsyncStreamingDecoder`/`CancellableAsyncEncoder`/`CancellableAsyncDecoder`,
  plus `end_marker_seen()` observability accessors on the sync and async
  decoders.
- Zero-copy borrowed-deserialization support for the `serde` integration
  (`&'de str`/`&'de [u8]`/`#[serde(borrow)]` fields now actually borrow
  instead of erroring at runtime).
- Generic `BorrowDecode` for `Rc<T>`/`Arc<T>`, and relaxed several collection
  `BorrowDecode` bounds from `T: Decode + 'static` to element-wise
  `T: BorrowDecode` (`Box<T>`, `Box<[T]>`, `HashMap`, `HashSet`, `BTreeMap`,
  `BTreeSet`, `VecDeque`, `LinkedList`, `BinaryHeap`, `Rc<[T]>`, `Arc<[T]>`).
- Blanket `impl<T: Encode + ?Sized> Encode for &T`.
- `u8`-specialized bulk-copy fast paths for `Vec<u8>`/`[u8; N]`/`[u8]`
  encode/decode (byte-identical output, faster).
- Real hardware SIMD kernels in `src/simd/copy.rs` backing the existing
  opt-in `oxicode::simd` array codec.
- `examples/async_streaming.rs` (round-trip + cooperative cancellation demo).
- Enum discriminants wider than `u32` are now supported when
  `#[oxicode(tag_type = "u64")]` is set, decoded/encoded at native width with
  no truncating cast.
- Roughly 40 new `hardening_*` regression tests covering the items above
  (container claim-before-allocate, streaming DoS/truncation/cancellation,
  compression bomb/codec-mismatch, checksum overflow, derive attribute
  conflicts, enum tag width, SIMD equivalence, serde borrowed decode, and
  more).

### Changed

- `Cow` `Decode` is now generic over `T: ToOwned + ?Sized where T::Owned:
  Decode` (previously separate concrete `Cow<str>`/`Cow<[u8]>` impls).
- `rename`/`rename_all` derive attributes are now explicitly documented as
  no-ops on oxicode's positional binary wire format (accepted only for
  source compatibility with serde-annotated types); this was already their
  runtime behavior, only the documentation was misleading before this
  release.
- The derive macro's `#[oxicode(bound = "...")]` predicate splitter now
  parses with `syn`'s `Punctuated` parser (tracking `<>`/`()`/`[]` nesting)
  instead of a hand-rolled string split, so bounds containing `Fn`-trait
  syntax (e.g. `T: Fn(u8, u8) -> u8`) parse correctly.
- `BorrowDecode` derive now skips variants identically to `Decode` (previously
  could accept a byte sequence that `Decode` would reject, or vice versa).
- Compile-error spans for union/enum-level derive errors now point at the
  `union`/`enum` keyword and the specific offending attribute/field/variant,
  rather than a generic call-site span.
- Default features now include `validation` and `versioning` (previously
  `default = ["std", "derive"]`; now `default = ["std", "derive",
  "validation", "versioning"]`). Both features were mislabeled as
  test-only opt-ins but actually gate the real `oxicode::validation`
  (post-decode constraint checking) and `oxicode::versioning`
  (schema-version-header) public modules; they now ship by default like
  the rest of the public API.
- `oxiarc-lz4` and `oxiarc-zstd` optional dependencies updated from 0.3.2 to
  0.4.0 (six incremental bumps) and moved to `[workspace.dependencies]`.
- `tokio` optional/dev-dependency updated from 1.52 to 1.53 and moved to
  `[workspace.dependencies]`.

### Fixed

- `compression-lz4`/`compression-zstd` features now also enable `alloc`,
  and `async-tokio` now also enables `std` — both were previously
  under-declared: enabling either without its now-explicit transitive
  requirement could fail to compile since the streaming/compression code
  they gate actually needs `alloc`/`std`.
- Preserved `AsyncEncoder`/`AsyncDecoder` as backward-compatible type
  aliases for `AsyncStreamingEncoder`/`AsyncStreamingDecoder`, reachable
  from all three historical import paths; added a regression test
  (`tests/async_backcompat_names_test.rs`) guarding this.
- `compression`'s module documentation no longer references a nonexistent
  `decompress_auto` function or claims compression is "applied globally or
  per-message"; it now documents the real explicit byte-level API.
- Removed a production `unreachable!()` and a 4x-duplicated 5-byte-header
  write in `compression::compress` (output bytes unchanged).
- Fixed a previously dead/unused `decode_slice_len` helper in `src/de/mod.rs`
  so it is now the single checked implementation used by length-prefix call
  sites.
- `oxicode::compression::is_compressed` now also validates the codec-id byte,
  shrinking (but not eliminating — this remains a heuristic, documented as
  such) its false-positive rate against arbitrary payloads that happen to
  start with the same 5 bytes.
- All five `map_err(|_| <static message>)` sites in the LZ4/Zstd compression
  wrappers now preserve the underlying `oxiarc` error text via
  `Error::OwnedCustom` instead of discarding it.

### Documentation

- Rewrote the README's "Advanced Features" section: replaced the nonexistent
  `CompressedEncoder`/`CompressedDecoder`/`CompressionType`,
  `VersionedEncoder::new(writer, version, config)`, and
  `EncodedBytes::new(&bytes)` API examples with the real
  `compress`/`decompress`/`Compression`, `VersionedEncoder::new(version)
  .encode(&bytes)`, and `EncodedBytes(&bytes)` APIs; fixed the
  `StreamingEncoder`/`StreamingDecoder`/`AsyncStreamingEncoder` constructor
  signatures (no `config` argument, infallible `new`); rewrote the
  `Validator<T>` example to match its actual single-type-per-validator
  design.
- Rewrote the SIMD documentation and `examples/simd_arrays.rs` to state
  plainly that `oxicode::simd` is a separate, opt-in codec not used by
  `encode_to_vec`/derive, and to measure its own speedup at run time instead
  of printing a fixed, unverified "2-4x" claim.
- Added a "Known compatibility caveats" section (README, cross-referenced
  from MIGRATION.md) documenting the standard-library types that currently
  diverge from bincode 2.0.1 regardless of config: `SystemTime`,
  `SocketAddrV6` (`flowinfo`/`scope_id`), `IpAddr`/`SocketAddr`/`Bound<T>`
  (tag width), `Path`/`PathBuf` (platform-dependent byte format), `Ordering`
  (`i8` vs. a wider enum tag), and `Duration` (strictness). Softened
  previously unqualified "100% binary compatible" / "100% Bincode Compatible"
  claims to reference this list; noted that a full versioned wire-format
  `SPEC.md` remains a deferred follow-up (the `oxicode_compatibility` test
  suite is the current source of truth).
- MIGRATION.md: replaced the fictitious `oxicode_compatibility = "0.2"`
  runtime dependency instructions (the crate is `publish = false` and
  test-only) with instructions to run its test suite directly; split the
  bincode "Before" example into bincode 1.x (`serialize`/`deserialize`) and
  bincode 2.x (`encode_to_vec`/`decode_from_slice`) cases, since the two
  don't share an API; corrected the claim that oxicode's serde-feature-gating
  is a divergence from bincode (bincode 2.x gates serde the same way).
- BENCHMARKS.md: corrected `comparison_bench.rs`'s description — it compares
  oxicode against bincode 2.0.1 only; the module doc's mention of rkyv,
  postcard, and borsh does not correspond to any dev-dependency or bench code
  in the file.
- Synced the README feature-flag block and examples list with `Cargo.toml`
  and `examples/` exactly (added `binary_format.rs`, `derive_attrs.rs`, and
  the new `async_streaming.rs`); added an MSRV badge/line (1.81.0, unchanged
  from 0.2.4).

## [0.2.4] - 2026-06-04

### Added
- `StreamingDecoder<R, C>` and `StreamingEncoder<W, C>` now accept a generic `Config` type parameter (`C: Config = config::Configuration`). The existing `new()` constructors are unchanged (default to standard config); a new `new_with_config(reader/writer, codec_config)` constructor allows the codec configuration to be matched between encoder and decoder pairs.
- Regression test `test_streaming_decoder_with_fixed_int_config` covering `StreamingDecoder::new_with_config` with fixed-width integer encoding roundtrip.
- Feature flags added to LZ4 compression test modules.

### Changed
- MSRV bumped from 1.74.0 to 1.81.0 (`rust-version` in `[workspace.package]`).
- `oxiarc-lz4` and `oxiarc-zstd` optional dependencies updated from 0.2.8 to 0.3.2.
- `DeError` and `SerError` now implement `core::error::Error` instead of `std::error::Error`, removing the `#[cfg(feature = "std")]` gate and enabling the impls in `no_std` + `alloc` contexts (Rust 1.81+ stabilised `core::error::Error`).
- Bumped workspace and crate versions from 0.2.3 to 0.2.4 (branch-name-drives-version policy).

### Fixed
- Removed spurious blank lines in several LZ4 compression test files.

## [0.2.3] - 2026-05-08

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

#### Core Features - Bincode Compatibility *(originally announced as "100%"; qualified in 0.2.5 — see README §"Known compatibility caveats")*
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
- **Binary Compatibility**: binary format compatibility with bincode 2.0 for the verified type set
  - Data encoded with bincode can be decoded with oxicode (for covered types)
  - Data encoded with oxicode can be decoded with bincode (for covered types)
  - 18/18 binary compatibility tests passing
  - *(originally claimed as "100% binary compatibility"; qualified in 0.2.5 — known divergences for `SystemTime`, `SocketAddrV6`, `IpAddr`/`SocketAddr`/`Bound` tag widths, `Path`/`PathBuf`, `Ordering`, and `Duration` leniency are documented in README §"Known compatibility caveats")*

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

Binary data is compatible for the verified type set when both sides use matching
configs — you can mix libraries during migration for those types. *(Originally
stated as "100% compatible"; qualified in 0.2.5 — see README §"Known
compatibility caveats" for the known divergences.)*

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

[0.2.6]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.6
[0.2.5]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.5
[0.2.4]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.4
[0.2.3]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.3
[0.2.2]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.2
[0.2.1]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.1
[0.2.0]: https://github.com/cool-japan/oxicode/releases/tag/v0.2.0
[0.1.0]: https://github.com/cool-japan/oxicode/releases/tag/v0.1.0
