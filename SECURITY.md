# Security Policy

oxicode is a binary serialization library that routinely decodes data from
untrusted sources (network peers, files, IPC, etc.). Memory-safety and
denial-of-service issues in the decode path are treated as security
vulnerabilities, not ordinary bugs.

## Supported Versions

Only the latest published `0.2.x` release of `oxicode` / `oxicode_derive`
receives security fixes. Older `0.2.x` releases and the `0.1.x` line are
not maintained; users should upgrade to the latest release before
reporting an issue against an older version.

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x (latest) | :white_check_mark: |
| < 0.2.x        | :x:                 |

## Reporting a Vulnerability

Please **do not** open a public GitHub issue for security reports.

Report privately using one of the following channels:

1. **GitHub Security Advisories (preferred):** open a
   [private security advisory](https://github.com/cool-japan/oxicode/security/advisories/new)
   against this repository. This lets us collaborate on a fix before any
   public disclosure and gives you a CVE if one is warranted.
2. **Email:** if you cannot use GitHub's advisory flow, email
   `info@kitasan.io` with a description of the issue, affected version(s),
   and, if possible, a minimal reproduction (a byte sequence and the
   config/type used to decode it is usually sufficient).

Please include:

- The crate(s) affected (`oxicode`, `oxicode_derive`) and version.
- Whether the issue requires trusted or untrusted input to trigger.
- A minimal reproduction (fuzz corpus entry, byte slice, or code snippet).
- The observed impact (panic/DoS, out-of-bounds read, unbounded
  allocation, undefined behavior, etc.).

We aim to acknowledge reports within 5 business days and to provide a
fix or mitigation plan within 30 days, depending on severity and
complexity. Coordinated disclosure is preferred: we will work with you
on a disclosure timeline once a fix is available.

## Scope

In scope:

- Panics, out-of-bounds reads/writes, integer overflow, or undefined
  behavior triggered by decoding **untrusted** byte input via any public
  `decode_*` / `*_from_slice` / `*_from_reader` entry point, the derive
  macros' generated code, or the `serde` bridge.
- Unbounded memory/CPU consumption ("decompression bomb" style attacks)
  from decoding attacker-controlled input, including through the
  optional `compression-lz4` / `compression-zstd` features (oxicode
  already enforces a `MAX_DECOMPRESSED_SIZE` limit to mitigate this
  class of issue; bypasses of that limit are in scope).

Out of scope:

- Issues that require a fully trusted, cooperating encoder and decoder
  agreeing on a mutually-known-unsafe configuration (i.e. you control
  both sides and choose to misuse the API).

- Denial of service from calling the public API with parameters that
  are already documented as caller-controlled resource limits (e.g.
  intentionally providing a huge in-memory buffer you allocated
  yourself).

## Hardening Posture

- The crate maintains a `no_std` + `alloc`-only build path, minimizing
  the trusted computing base for embedded/constrained targets.
- Untrusted-input decode paths are exercised by a `cargo-fuzz` suite
  under `fuzz/` (`fuzz_decode_slice`, `fuzz_decompress`, `fuzz_roundtrip`,
  `fuzz_serde`, `fuzz_streaming`, `fuzz_streaming_std_reader`,
  `fuzz_versioned`); new decode-path features should add or extend a
  fuzz target.
- `unsafe` usage is confined to a small number of modules
  (`src/de/borrow_slice.rs`, `src/de/impls.rs`, `src/enc/impls.rs`,
  `src/features/impl_alloc.rs`, `src/simd/aligned.rs`, `src/simd/array.rs`,
  `src/simd/copy.rs`) and is exercised under Miri.
- Length-prefixed collections and buffers are bounds-checked against the
  input actually available before allocation, so a short malicious input
  cannot trivially trigger an out-of-memory condition. This is two
  independent mechanisms, one per entry point:
  - The core `Decode`/`BorrowDecode` path (`String`, `Vec<u8>`, and the
    derive macros' `#[oxicode(bytes)]` / `#[oxicode(seq_len = ...)]`
    fields) uses `read_bytes_bounded` (`src/features/impl_alloc.rs`) and
    its derive-generated equivalent: a slice-backed decoder rejects a
    length that exceeds the bytes actually remaining *before* allocating
    anything, and a `std::io::Read`-backed decoder grows its buffer in
    small bounded increments that each have to be filled from the reader
    before the next one is reserved. An IO reader can be given the same
    exact bound as a slice when the length of the stream is known:
    `IoReader::with_limit` / `BufferedIoReader::with_limit`, or the
    `decode_from_std_read_limited` / `decode_from_buffered_read_limited`
    entry points; `decode_from_file` derives the budget from the file's
    size automatically. The serde bridge decodes its `String` / byte
    fields through the same impls, so it inherits the bound as soon as
    the reader has one: use `oxicode::serde::decode_from_std_read_limited`
    (the budgeted counterpart of `oxicode::serde::decode_from_std_read`).
    Note that a reader's *buffered* byte count is
    deliberately **not** used as a bound — it is a lower bound on what the
    stream can still produce, and reporting it would reject valid input.
  - The `oxicode::streaming` module (`StreamingDecoder`,
    `BufferStreamingDecoder`, `AsyncStreamingDecoder`) separately bounds
    each chunk's *declared* payload length against a configurable ceiling
    (`MAX_CHUNK_SIZE` by default) before allocating anything, and — for
    the two reader-backed decoders (`std::io::Read` and `AsyncRead`) —
    materializes the payload in the same style of bounded increments
    rather than committing the full claimed length up front.

  Both are independent of `claim_bytes_read` / `with_limit`, which enforce
  the *configured* decode byte budget and are a no-op under the default
  `NoLimit` configuration — set an explicit limit via `Config::with_limit`
  if you also want a hard cap on total bytes/elements consumed by one
  decode call, on top of the per-allocation bounds described above.

If you are unsure whether something qualifies, please report it anyway
through one of the private channels above — we would rather triage a
false positive than miss a real issue.
