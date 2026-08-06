# OxiCode Development TODO

> **Trimmed 2026-07-17 (per user request).** Completed audit items, wave-by-wave
> execution logs, and pre-0.2.5 historical records were removed from this file.
> The full 0.2.5 audit record (119 findings with Issue / Evidence / Fix detail)
> and the hardening execution logs live in git history — see this file as of
> commit `2c766b1` (hardening landed in `f0d0196`). Release history is in
> [CHANGELOG.md](CHANGELOG.md); build/test commands are in
> [CONTRIBUTING.md](CONTRIBUTING.md).

## Current status (2026-08-06)

- **Version 0.2.7** on branch `0.2.7` — the release this document describes.
- The 0.2.5 production-hardening program is **complete**: 107 of 119
  adversarially-verified audit findings fixed — decode-time DoS mitigation
  (container claim accounting, decompression caps, recursion guard, checked
  u64→usize), soundness ([T; N] drop-guard, overlong-UTF-8 char rejection,
  OsStr non-UTF-8 error, async cancellation safety, stream truncation
  poisoning), real SIMD (AVX2/SSE2/NEON with runtime dispatch and
  byte-equivalence proofs), derive/serde/streaming/compression hardening,
  documentation truth pass, and a CI workflow definition (clippy `-D warnings`,
  nextest, MSRV, Miri, cargo-deny, cargo-semver-checks, BE/32-bit legs). The
  workflow ships as `.github/workflows/ci.yml.disabled` and is **not** currently
  enabled in GitHub Actions; those gates are run locally. Re-enabling it in CI
  is tracked as remaining work.
- The 0.2.6 follow-on hardening waves are **complete**: the serde bridge (which
  has its own decode path and so missed several 0.2.5 protections) gained the
  recursion guard, checked length prefixes, container claims, and — the one
  wire-relevant change — `is_human_readable() == false`, restoring
  `bincode::serde` byte parity for `IpAddr`/`uuid`/`chrono`-style types;
  allocation bounds moved from "claimed length" to "bytes actually received"
  for streaming payloads, `#[oxicode(bytes)]` derive fields and IO decoding
  (new `*_limited` entry points and budgeted readers); `AlignedVec<ZST>` UB
  fixed; two derive mis-decode hazards became compile errors; decode contexts
  became usable from the derive. See CHANGELOG.md §0.2.6.
- The remaining 12 findings are the deliberately **deferred** bincode
  wire-format items below (user decision, 2026-07-17); they are documented as
  known issues in README §"Known compatibility caveats".
- **Final verification gates (all GREEN, re-verified 2026-08-06):**
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
  `cargo nextest run --workspace --all-features` = **20,198 passed / 0 failed /
  9 skipped** (15,601 under default features); `cargo test --doc
  --all-features` = 53 passed; `RUSTDOCFLAGS="-D warnings" cargo doc` clean;
  single-compression-feature check/clippy legs pass; `cargo fmt --check` clean.
- **MSRV:** declared 1.81.0; compression features need Rust ≥ 1.85
  (oxiarc-core is edition2024) — documented in README.

## Open items

### [DECISION] Deferred: bincode wire-format compatibility (12 items)

Fixing any item below changes the serialized byte layout for currently-valid
inputs → breaking for existing oxicode-serialized data → requires a **0.3.0**
decision. Executable specification: `#[ignore]`d cross-library tests +
self-roundtrip golden vectors in `compatibility/` and `tests/hardening_m1_*`.

- [ ] **[DECISION]** 12 bincode wire-compat items remain ⏸ DEFERRED per user decision (2026-07-17): SystemTime, Bound/IpAddr/SocketAddr tag widths, SocketAddrV6 flowinfo/scope_id, Path/PathBuf format, Ordering native-vs-serde, Duration leniency, context-API parity. Executable spec exists as `#[ignore]`d cross-tests + self-roundtrip golden vectors in `compatibility/` and `tests/hardening_m1_*`. Fixing them changes wire bytes → requires a 0.3.0 decision. **Update (2026-07-17, per user):** documented as known issues in `README.md` §"Known compatibility caveats" — each divergence stated precisely (bincode's actual layout, config scope, decode-leniency vs byte-layout distinction) plus the API-level context-parity gap; README points back to this TODO for the item-by-item record.

- [ ] **[CRITICAL]** ✅ CONFIRMED · _medium_ · `src/enc/impls.rs:501` — SystemTime wire format diverges from bincode 2 in every config (silent data corruption) — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2.0.1 encodes SystemTime as Duration since UNIX_EPOCH: u64 secs + u32 nanos, and returns EncodeError::InvalidSystemTime for pre-epoch times (bincode-2.0.1/src/features/impl_std.rs:238-248). oxicode encodes i64 secs (zigzag varint in standard mode) + u32 nanos and supports pre-epoch. In the default standard() varint config the bytes differ for every timestamp (e.g. secs=1 -> bincode 0x01, oxicode zigzag 0x02). Worse, decoding bincode-2-produced bytes with oxicode succeeds silently with a wrong value: a u64 secs varint payload is zigzag-decoded, halving even values (epoch secs ~1.75e9 decodes as ~0.87e9 — a 1997 timestamp). In legacy/fixint mode positive secs happen to match byte-wise, masking the bug in legacy tests.
  - **Evidence:** src/enc/impls.rs:506-512: `match self.duration_since(std::time::SystemTime::UNIX_EPOCH) { Ok(dur) => { let secs = dur.as_secs() as i64; ... secs.encode(encoder)?; nanos.encode(encoder) }` (i64 -> zigzag varint) vs bincode 2.0.1 impl_std.rs:238-247: `let duration = self.duration_since(SystemTime::UNIX_EPOCH).map_err(...)?; duration.encode(encoder)` (Duration = u64 secs + u32 nanos). Decode side src
  - **Fix:** Match bincode 2 exactly: encode `self.duration_since(UNIX_EPOCH)` as Duration (u64 secs + u32 nanos) and return an error (add Error::InvalidSystemTime { .. }) for pre-epoch times; decode as Duration + UNIX_EPOCH.checked_add with error on overflow (mirror bincode de). If pre-epoch support is desired, offer it as a separate opt-in wrapper type rather than changing the default wire format.
  - <sub>audit-dims: decode-core-soundness,wire-format-core · id: wire-format-core#0</sub>

- [ ] **[CRITICAL]** ✅ CONFIRMED · _easy_ · `src/enc/impls.rs:404` — Bound<T> variant tags encoded as u8 instead of u32 — diverges from bincode in fixint/legacy configs — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2.0.1 encodes Bound tags as 0u32/1u32/2u32 (enc/impls.rs:462-482). oxicode uses 0u8/1u8/2u8 for encode (src/enc/impls.rs:401-415) and u8 for decode (src/de/impls.rs:460, :734). Identical bytes in varint mode, but in fixed-int configs bincode writes 4-byte tags vs oxicode's 1 byte — wire incompatible and misframes subsequent data.
  - **Evidence:** src/enc/impls.rs:403-413: `Bound::Unbounded => 0u8.encode(encoder), Bound::Included(value) => { 1u8.encode(encoder)?; ... }` vs bincode 2.0.1 enc/impls.rs:466-479: `Self::Unbounded => { 0u32.encode(encoder)?; } Self::Included(val) => { 1u32.encode(encoder)?; ... }`.
  - **Fix:** Switch Bound<T> tags to u32 in Encode, Decode, and BorrowDecode (src/de/impls.rs:460ff and :734ff) to match bincode 2.
  - <sub>audit-dims: wire-format-core · id: wire-format-core#3</sub>

- [ ] **[CRITICAL]** ✅ CONFIRMED · _medium_ · `src/enc/impls.rs:502` — SystemTime wire format incompatible with bincode: zigzag i64 secs instead of u64 Duration — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2 encodes SystemTime as Duration-since-UNIX_EPOCH (u64 secs + u32 nanos) and returns EncodeError::InvalidSystemTime for pre-epoch times. Oxicode encodes i64 secs (negative = pre-epoch) + u32 nanos. Under the default standard() config (IntEncoding::Variable, confirmed at src/config.rs:44), signed ints are zigzag-encoded (src/varint/encode_signed.rs:35), so secs=1_700_000_000 is written as varint(3_400_000_000). A bincode decoder reads this as u64 secs = 3.4e9 → every timestamp is silently doubled/corrupted cross-library. Under Fixed config, pre-epoch times encode as negative i64 which bincode decodes as a huge u64 → far-future date. Decode side (src/de/impls.rs:585-621) has the mirrored incompatibility.
  - **Evidence:** src/enc/impls.rs:506-511: `match self.duration_since(UNIX_EPOCH) { Ok(dur) => { let secs = dur.as_secs() as i64; ... secs.encode(encoder)?; }` — vs bincode 2.0.1 impl_std.rs:238-248: `let duration = self.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| EncodeError::InvalidSystemTime {...})?; duration.encode(encoder)` (u64 secs, no zigzag)
  - **Fix:** Encode SystemTime as Duration (u64 secs + u32 nanos) exactly like bincode; return an error (add Error::InvalidSystemTime) for pre-epoch times on encode; decode via Duration + UNIX_EPOCH.checked_add with error on overflow. If pre-epoch support is desired, offer it only behind a distinct non-compat config/type wrapper. Add byte-level compat test vs bincode.
  - <sub>audit-dims: std-alloc-impls · id: std-alloc-impls#1</sub>

- [ ] **[CRITICAL]** ✅ CONFIRMED · _easy_ · `src/features/impl_std.rs:302` — SocketAddrV6 encodes flowinfo and scope_id — extra fields vs bincode 2 in ALL configs — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2.0.1 encodes SocketAddrV6 as ip + port only, and decode constructs `SocketAddrV6::new(ip, port, 0, 0)` (bincode-2.0.1/src/features/impl_std.rs:399-412). oxicode additionally writes flowinfo (u32) and scope_id (u32). This adds 2+ bytes (varint) / 8 bytes (fixint) to the stream, so any payload containing a SocketAddrV6 (including inside SocketAddr/enums/structs) is incompatible with bincode in both directions in every configuration, and all subsequent fields in the stream are misframed.
  - **Evidence:** src/features/impl_std.rs:303-308: `self.ip().encode(encoder)?; self.port().encode(encoder)?; self.flowinfo().encode(encoder)?; self.scope_id().encode(encoder)` vs bincode 2.0.1 impl_std.rs:400-403: `self.ip().encode(encoder)?; self.port().encode(encoder)`.
  - **Fix:** Drop flowinfo/scope_id from Encode and read only ip+port in Decode, constructing SocketAddrV6::new(ip, port, 0, 0), exactly mirroring bincode 2.0.1. Update src/de/impls or impl_std decode accordingly and add a cross-library compatibility test.
  - <sub>audit-dims: wire-format-core · id: wire-format-core#1</sub>

- [ ] **[CRITICAL]** ✅ CONFIRMED · _easy_ · `src/features/impl_std.rs:196` — IpAddr and SocketAddr variant tags encoded as u8 instead of u32 — diverges from bincode in fixint/legacy configs — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2.0.1 encodes the V4/V6 discriminant as 0u32/1u32 (impl_std.rs:293-306 and 353-360). oxicode writes 0u8/1u8. In varint mode the bytes coincide (single byte 0/1), but with fixed-int encoding (config::legacy(), or standard().with_fixed_int_encoding()) bincode writes a 4-byte tag while oxicode writes 1 byte — a wire incompatibility that also misframes everything after the address. Additionally, the oxicode serde path (serde's IpAddr Serialize -> serialize_newtype_variant writes u32) diverges from oxicode's own native impl in fixint mode.
  - **Evidence:** src/features/impl_std.rs:196-203: `IpAddr::V4(addr) => { 0u8.encode(encoder)?; ... }` and :258-265 same for SocketAddr; bincode 2.0.1 impl_std.rs:296-303: `IpAddr::V4(v4) => { 0u32.encode(encoder)?; ... }`, decode at :310 `match u32::decode(decoder)?`.
  - **Fix:** Change tags to 0u32/1u32 in Encode for IpAddr and SocketAddr, and decode via u32::decode, matching bincode 2. Wire-identical in varint mode, fixes fixint/legacy modes.
  - <sub>audit-dims: wire-format-core · id: wire-format-core#2</sub>

- [ ] **[CRITICAL]** ✅ CONFIRMED · _medium_ · `src/features/impl_std.rs:130` — Path/PathBuf on Windows encoded as UTF-16 code units — wire incompatible with bincode 2 and with oxicode-on-unix — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2.0.1 encodes Path via `to_str()` as a UTF-8 str on all platforms and errors with InvalidPathCharacters on non-UTF-8 (impl_std.rs:261-267). oxicode on unix writes raw OS bytes (byte-compatible with bincode only for valid UTF-8 paths), but on Windows writes a u16 code-unit count followed by each u16 individually encoded (varint per unit in standard mode). A path encoded on Windows cannot be decoded by bincode anywhere, nor by oxicode on unix, and vice versa — bincode's format is platform-independent, oxicode's is not. The unix branch also silently accepts non-UTF-8 paths that bincode peers cannot decode.
  - **Evidence:** src/features/impl_std.rs:130-139: `#[cfg(windows)] { let wide: Vec<u16> = os_str.encode_wide().collect(); (wide.len() as u64).encode(encoder)?; for code_unit in wide { code_unit.encode(encoder)?; } }` vs bincode 2.0.1 impl_std.rs:262-266: `match self.to_str() { Some(str) => str.encode(encoder), None => Err(EncodeError::InvalidPathCharacters) }`.
  - **Fix:** Encode Path/PathBuf via `to_str()` as a str on all platforms, returning a new Error variant (e.g. InvalidPathCharacters) for non-UTF-8; decode via String, matching bincode's PathBuf decode (impl_std.rs:285-290). Remove the cfg(unix)/cfg(windows) branches.
  - <sub>audit-dims: wire-format-core · id: wire-format-core#4</sub>

- [ ] **[CRITICAL]** ✅ CONFIRMED · _easy_ · `src/features/impl_std.rs:302` — SocketAddrV6 wire format incompatible with bincode: encodes flowinfo and scope_id — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** Oxicode encodes SocketAddrV6 as ip + port + flowinfo(u32) + scope_id(u32) and decodes all four fields. bincode 2.0.1 (impl_std.rs:399-412) encodes ONLY ip + port and decodes with `Self::new(ip, port, 0, 0)`. serde's SocketAddrV6 impl (bincode 1.x legacy) also omits flowinfo/scope_id. Any payload containing SocketAddrV6 is 8 bytes longer in oxicode; cross-library decode desynchronizes the stream, silently corrupting all subsequent fields or erroring. This directly falsifies the '100% binary compatibility' claim. The compat test suite (tests/bincode_compat_test.rs) never tests SocketAddrV6, so it went undetected.
  - **Evidence:** oxicode impl_std.rs:303-308: `self.ip().encode(encoder)?; self.port().encode(encoder)?; self.flowinfo().encode(encoder)?; self.scope_id().encode(encoder)` — vs bincode 2.0.1 impl_std.rs: `impl Encode for SocketAddrV6 { ... self.ip().encode(encoder)?; self.port().encode(encoder) }` and decode `Ok(Self::new(ip, port, 0, 0))`
  - **Fix:** Match bincode exactly: encode only ip and port; decode as `SocketAddrV6::new(ip, port, 0, 0)`. Update SocketAddr enum path accordingly and add a byte-for-byte cross test against bincode 2.0.1 for SocketAddr/SocketAddrV4/SocketAddrV6.
  - <sub>audit-dims: std-alloc-impls · id: std-alloc-impls#0</sub>

- [ ] **[CRITICAL]** ✅ CONFIRMED · _easy_ · `src/features/impl_std.rs:197` — IpAddr / SocketAddr enum discriminant encoded as u8, but bincode uses u32 — wire incompatibility — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** oxicode encodes the IpAddr and SocketAddr variant discriminant with `0u8`/`1u8` and decodes it with `u8::decode` (impl_std.rs: IpAddr enc lines 197/201, dec 210-213; SocketAddr enc 259/263, dec 272-275). bincode 2.0.1 encodes these discriminants as `0u32`/`1u32` (verified in registry `bincode-2.0.1/src/features/impl_std.rs` lines 296-303 and 356-363; decode reads `u32`). Under `config::legacy()` / any Fixint config a u32 discriminant is 4 bytes while oxicode writes 1 byte, so the byte streams differ and a bincode-encoded IpAddr/SocketAddr misdecodes in oxicode (and vice-versa). This directly breaks the crate's headline '100% wire compatible / byte-for-byte identical to bincode' claim and causes silent data corruption on cross-library interop. (Under Varint config the values 0/1 happen to be 1 byte for both widths, masking the bug in the default `standard()` config and likely in the test-suite.)
  - **Evidence:** oxicode impl_std.rs:197  `0u8.encode(encoder)?;` (IpAddr::V4), :201 `1u8.encode(encoder)?;` (IpAddr::V6); decode :210 `let variant = u8::decode(decoder)?;`  vs bincode-2.0.1/src/features/impl_std.rs:297 `0u32.encode(encoder)?;` / :301 `1u32.encode(encoder)?;` and decode via `u32::decode`.
  - **Fix:** Change the IpAddr, SocketAddr (and any other manually-written std enum) discriminants from u8 to u32 on both encode and decode to match bincode: `0u32.encode(...)` / `1u32.encode(...)` and `let variant = u32::decode(decoder)?;`. Add a fixed-int cross-compat test that encodes an `IpAddr::V4`/`SocketAddr` with `config::legacy()` and asserts byte-for-byte equality against a bincode-produced vector.
  - <sub>audit-dims: decode-core-soundness · id: decode-core-soundness#0</sub>

- [ ] **[HIGH]** ✅ CONFIRMED · _medium_ · `src/features/impl_std.rs:120` — Path/PathBuf wire format is platform-dependent and incompatible with bincode — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2 encodes Path via `self.to_str()` as a UTF-8 string (error InvalidPathCharacters on non-UTF8) and decodes PathBuf via String::decode. Oxicode encodes raw OS bytes on unix (`OsStrExt::as_bytes`), UTF-16 code units on windows (`encode_wide`, each u16 individually encoded, so varint-expanded under standard config), and lossy UTF-8 elsewhere. Consequences: (1) on Windows every PathBuf payload is byte-incompatible with bincode and with oxicode-on-unix — a file/stream written on Windows silently desyncs when decoded on Linux since the unix decoder interprets the u16-count as a byte-count; (2) on unix, non-UTF8 paths encode successfully into payloads bincode cannot decode (UTF-8 error), where bincode would have errored at encode time.
  - **Evidence:** impl_std.rs:130-139 (windows): `let wide: Vec<u16> = os_str.encode_wide().collect(); (wide.len() as u64).encode(encoder)?; for code_unit in wide { code_unit.encode(encoder)?; }` — vs bincode 2.0.1 impl_std.rs:261-267: `match self.to_str() { Some(str) => str.encode(encoder), None => Err(EncodeError::InvalidPathCharacters) }`
  - **Fix:** Match bincode: `impl Encode for Path` = `self.to_str().ok_or(Error::InvalidPathCharacters)?.encode(encoder)`; `impl Decode for PathBuf` = `String::decode(decoder).map(Into::into)`; add `BorrowDecode for &'de Path` from `&'de str` (bincode has it, oxicode lacks it). If lossless OS-specific paths are needed, provide a separate opt-in wrapper type instead of changing the default wire format.
  - <sub>audit-dims: std-alloc-impls · id: std-alloc-impls#2</sub>

- [ ] **[MEDIUM]** ✅ CONFIRMED · _easy_ · `src/enc/impls.rs:319` — core::cmp::Ordering encoded as i8 (-1/0/1) — inconsistent with oxicode's own serde path (u32 variant 0/1/2) — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** The native Encode impl writes Ordering as i8 -1/0/1 (zigzag byte 0x01/0x00/0x02 in varint mode), while serde's derive-based Serialize for Ordering goes through serialize_unit_variant with variant_index 0/1/2 as u32 (src/features/serde/ser.rs:186-196). The same value therefore has two different wire encodings inside one library: data written with oxicode::serde cannot be decoded by native Decode (native rejects 2 for Greater vs serde's 2 for Greater... e.g. Less: serde writes 0x00, native decode maps 0 to Equal — silent wrong value). bincode 2 has no Ordering impl, so oxicode is free to pick, but the two internal paths must agree.
  - **Evidence:** src/enc/impls.rs:321-326: `let val: i8 = match self { Less => -1, Equal => 0, Greater => 1 };` and src/de/impls.rs:373-376 decodes i8 -1/0/1; serde path src/features/serde/ser.rs:186-196 writes `variant_index` (0=Less,1=Equal,2=Greater) as u32. Serde-encoded Less (byte 0x00) decodes natively as Equal — silent corruption across paths.
  - **Fix:** Change the native impl to encode the variant index as u32 (0=Less, 1=Equal, 2=Greater) and decode accordingly, aligning with the serde path and with what serde+bincode1 legacy streams contain. Document as a wire-format fix in CHANGELOG.
  - <sub>audit-dims: wire-format-core · id: wire-format-core#8</sub>

- [ ] **[LOW]** ✅ CONFIRMED · _easy_ · `src/de/impls.rs:574` — Duration decode stricter than bincode: rejects nanos >= 1e9 that bincode normalizes — ⏸ DEFERRED 2026-07-17: bincode wire-compat on hold (user decision)
  - **Issue:** bincode 2 accepts nanos >= 1_000_000_000 in Duration payloads, normalizing via Duration::new after guarding only the secs overflow (bincode de/impls.rs:678-687: `if secs.checked_add(nanos/1e9).is_none() -> InvalidDuration`). Oxicode rejects any nanos >= 1e9. Canonical bincode-encoded Durations always have nanos < 1e9 so normal interop is unaffected, but hand-crafted or third-party payloads that bincode decodes successfully are rejected by oxicode — a documented-compat divergence in the strict sense of 'decodes everything bincode decodes'.
  - **Evidence:** de/impls.rs:574-578: `if nanos >= 1_000_000_000 { return Err(Error::InvalidData { message: "Duration subsec_nanos out of range..." }); }` — vs bincode which only errors when `secs.checked_add(u64::from(nanos) / NANOS_PER_SEC).is_none()`.
  - **Fix:** Match bincode: allow nanos >= 1e9, guard `secs.checked_add(u64::from(nanos) / 1_000_000_000)` for overflow (error), then `Duration::new(secs, nanos)`. Alternatively document the stricter validation as an intentional deviation.
  - <sub>audit-dims: std-alloc-impls · id: std-alloc-impls#13</sub>

- [ ] **[MEDIUM]** ⚠️ PARTIALLY DONE · _hard_ · `src/lib.rs` — bincode 2 API parity gaps: ~~missing *_with_context variants~~, ~~derive hardcodes Context = ()~~, serde module missing borrow/writer/reader entry points — remaining part deferred (bincode wire-compat on hold, user decision 2026-07-17)
  - **Issue:** Migrating bincode 2 users hit these gaps: (1) Native context API: oxicode has only decode_from_slice_with_context (lib.rs:1029); bincode 2 also has borrow_decode_from_slice_with_context, decode_from_std_read_with_context, and decode_from_reader_with_context (bincode-2.0.1 src/lib.rs:191,213; features/impl_std.rs:39). (2) The context API is effectively unusable with derived types: oxicode_derive always emits `fn decode<__D: Decoder<Context = ()>>` (derive/src/lib.rs:302, :369), so any `#[derive(Decode)]` type only implements Decode<()> — decode_from_slice_with_context::<MyCtx, DerivedType, _> will not compile; bincode's derive supports contexts (#[bincode(decode_context)]). (3) serde module naming/coverage: bincode has bincode::serde::{borrow_decode_from_slice, encode_into_writer, decode_from_reader, seed_decode_from_slice}; oxicode::serde lacks all four names (its borrowing decode_from_slice is broken per the separate finding). Also oxicode::decode_from_reader takes std::io::Read and returns (D, usize) whereas bincode's decode_from_reader takes its Reader trait and returns D — same name, different contract (compile error on migration, but a documented rename table would help).
  - **Evidence:** grep 'with_context' src/lib.rs -> only decode_from_slice_with_context (line 1029). derive/src/lib.rs:302: `fn decode<__D: #crate_path::de::Decoder<Context = ()>>`. bincode-2.0.1/src/lib.rs:191 `pub fn borrow_decode_from_slice_with_context`, impl_std.rs:39 `pub fn decode_from_std_read_with_context`. grep 'borrow_decode_from_slice' src/features/serde/ -> no matches.
  - **Fix:** Add borrow_decode_from_slice_with_context, decode_from_std_read_with_context, decode_from_reader_with_context mirroring the existing decode_from_slice_with_context (DecoderImpl::with_context already exists). Extend the derive to accept a container attribute (e.g. #[oxicode(decode_context = "Ctx")] or generate `impl<__Ctx> Decode<__Ctx>` when all field types are context-generic) matching bincode_derive behavior. In oxicode::serde add borrow_decode_from_slice (on the fixed borrowed deserializer), encode_into_writer (enc::Writer generic), and decode_from_reader (de::Reader generic). Document the decode_from_reader semantic difference in MIGRATION.md.
  - **Done (2026-08-03):** (1) the native context entry points now exist —
    `borrow_decode_from_slice_with_context`, `decode_from_std_read_with_context`
    and the reader-generic `decode_from_de_reader_with_context`. (2) The derive
    is no longer pinned to `Context = ()`: every built-in `Decode` /
    `BorrowDecode` impl is now generic over the context
    (`impl<__Ctx> Decode<__Ctx> for u32`, mirroring bincode 2), and the derive
    accepts `#[oxicode(decode_context = "Ctx")]`,
    `#[oxicode(borrow_decode_context = "Ctx")]`, `#[oxicode(context = "Ctx")]`
    and the fully generic `#[oxicode(context_generic)]`. Without an attribute
    the generated impl is still `Decode<()>`, so the wire format and the public
    API are unchanged (`tests/context_generic_derive_test.rs`).
  - **Still open:** part (3), the `oxicode::serde` naming/coverage gap
    (`borrow_decode_from_slice`, `encode_into_writer`, `decode_from_reader`,
    `seed_decode_from_slice`) and the `decode_from_reader` contract difference.
    The serde bridge is inherently unit-context (serde's `Deserializer` carries
    no context), so it is unaffected by the context work above.
  - **Note (2026-08-04):** unrelated to the parity gap, the serde module gained
    `oxicode::serde::decode_from_std_read_limited` — the budgeted counterpart of
    `decode_from_std_read`, so serde-decoded `String`/byte fields get the same
    remaining-input allocation bound as the native IO path
    (`tests/hardening_wave4_test.rs::serde_bounded_io`).
  - <sub>audit-dims: api-parity-serde · id: api-parity-serde#8</sub>

### [RELEASE] Tag / publish decisions (user)

- [x] `v0.2.5` tagged and published to crates.io 2026-07-30.
- [ ] Stale `git stash` entries exist on this branch: `stash@{0}` holds
  superseded mid-wave copies of `src/de/impls.rs` / `impl_alloc.rs`; the
  current tree is newer and fully gate-verified — safe to drop after review.
