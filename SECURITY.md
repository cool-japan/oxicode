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
  under `fuzz/` (`fuzz_decode_slice`, `fuzz_roundtrip`, `fuzz_streaming`,
  `fuzz_versioned`); new decode-path features should add or extend a
  fuzz target.
- `unsafe` usage is confined to a small number of modules
  (`src/de/impls.rs`, `src/de/borrow_slice.rs`,
  `src/features/impl_alloc.rs`, `src/simd/aligned.rs`) and is exercised
  under Miri in CI.
- Length-prefixed collections and buffers are bounds-checked against the
  remaining input size before allocation, to avoid trivially triggering
  out-of-memory conditions from a short malicious input.

If you are unsure whether something qualifies, please report it anyway
through one of the private channels above — we would rather triage a
false positive than miss a real issue.
