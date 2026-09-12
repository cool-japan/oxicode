# oxicode-formal

`cargo-formal` harnesses over `oxicode`'s public fixed-array varint codec API
(`encode_to_fixed_array`, `decode_from_slice`, and their `_with_config`
variants). Contributed per cargo-formal Phase 2b, design v1.1 §4 / §7.3
item 5, harness inventory `I0-d.md` §5.4 (package "E4"). This package calls
straight into the real `oxicode` crate through an ordinary path dependency —
nothing here is vendored or copied.

It replaces, for the real public API, what
`examples/ecosystem/oxicode-varint` in the `cargo-formal` repository did for
five *vendored, private* files of `oxicode/src/varint/`. That trial stays in
place as an in-repository regression fixture; this package is the first
measurement of `oxicode`'s actual dispatch path (see "Why this package's
numbers differ from the vendored trial" below).

## What is verified

Nine harnesses over five public entry points:

| entry point | harnesses |
|---|---|
| `encode_to_fixed_array::<N, u16/u32/u64>` + `decode_from_slice::<..>` | round trip, for each width |
| `encode_to_fixed_array::<{N-1}, u64>` | the nine-byte bound is tight (fails exactly above `u32::MAX`) |
| `encode_to_fixed_array_with_config`/`decode_from_slice_with_config` with `config::standard().with_fixed_int_encoding()` | fixed-width `u64` round trip, always 8 bytes |
| `encode_to_fixed_array::<N, i32/i64>` + `decode_from_slice::<..>` | zigzag round trip, both signed widths |
| `decode_from_slice::<u16>` | rejects a wide tag byte (252/253/254) |
| `decode_from_slice::<u64>` | never panics on 12 arbitrary bytes |

See `src/harness.rs` for the full doc comment on every harness (property
intended, measured verdict, and the exact blocking file:line).

## The three builds

1. **`cargo build` / `cargo test`** (plain, stable). Harnesses vanish
   (neither `#[cfg(formal)]` nor `#[cfg(all(test, oxiformal_runtime_checks))]`
   applies); only the `plain_tests` module runs, with concrete witnesses.
   Measured: 0 warnings, 10/10 tests pass.
2. **`RUSTFLAGS="--cfg oxiformal_runtime_checks" cargo test`**. Every
   harness becomes a randomized `#[test]`. Measured: 0 warnings, 9/9 tests
   pass, **none** marked `#[should_panic]` (none needed one — every property
   this package states is true of every random draw the runtime-checks
   build tried).
3. **`cargo +nightly-2026-06-20 check --cfg formal
   -Zcrate-attr=feature(register_tool) -Zcrate-attr=register_tool(formal_tool)`**
   with a separate `--target-dir`. Type-checks the `#[cfg(formal)]` copy of
   every harness; does not run the driver or the solver. Measured: 0
   warnings, exit 0.

```bash
cd oxicode/formal
cargo build
cargo test
RUSTFLAGS="--cfg oxiformal_runtime_checks" cargo test
RUSTFLAGS="--cfg formal -Zcrate-attr=feature(register_tool) -Zcrate-attr=register_tool(formal_tool)" \
  cargo +nightly-2026-06-20 check --target-dir target/formal-check
```

## Running `cargo formal check`

Until `oxiformal`/`cargo-formal` are published, use the release CLI and
driver directly:

```bash
cd oxicode/formal
FORMAL_DRIVER=/path/to/formal-driver /path/to/cargo-formal formal check --target-dir <dir>
FORMAL_DRIVER=/path/to/formal-driver /path/to/cargo-formal formal check \
  --no-extract --format json --target-dir <dir>   # same --target-dir, writes report.json
FORMAL_DRIVER=/path/to/formal-driver /path/to/cargo-formal formal status --target-dir <dir>
```

## Measured verdict table (2026-09-08)

Release CLI + release driver with dependency-body lowering (D1) and
on-demand monomorphic instance lowering (D2), OxiZ 0.3.3.

| harness | `harness` verdict |
|---|---|
| `varint_u16_roundtrip_harness` | unsupported (no-body) |
| `varint_u32_roundtrip_harness` | unsupported (no-body) |
| `varint_u64_roundtrip_harness` | unsupported (no-body) |
| `varint_u64_bound_is_tight_harness` | unsupported (no-body) |
| `fixed_int_roundtrip_harness` | unsupported (no-body) |
| `zigzag_i32_roundtrip_harness` | unsupported (no-body) |
| `zigzag_i64_roundtrip_harness` | unsupported (no-body) |
| `decode_rejects_a_wide_tag_harness` | unsupported (no-body) |
| `decode_never_panics_on_arbitrary_bytes_harness` | unsupported (no-body) |

**All nine, 0 proved / 0 refuted.** Full detail and the "worst-of-sites"
keying rule are in `EXPECTED.toml`; the exact reasoning is in each harness's
doc comment in `src/harness.rs`.

### Layer counters (from the measured run)

* `hygiene`: pass (2 files scanned, 0 unsafe sites, 0 warnings, 0 notes).
* `bmc`: 0 proved / 0 refuted / 0 timeout / 0 unknown / **9 unsupported** /
  0 unverifiable.
* `contract`: 0 proved / 0 refuted (this package has no `#[requires]`/
  `#[ensures]`, only `#[harness]`).
* `theorem`: not run.
* `dependency bodies`: 42 lowered (6 reachable).
* `instances`: 68 lowered (68 reachable).
* `coverage`: 0 harnesses annotated, 9 harnesses total.

### Why all nine are `unsupported`, and why that is not a harness defect

Every harness calls `encode_to_fixed_array`/`decode_from_slice` (directly or
through the `_with_config` variant). Every `Encode`/`Decode` impl `oxicode`
ships reaches the varint codec through `encoder.writer()` /
`decoder.reader()` — `<EncoderImpl<W, C> as Encoder>::writer`
(`oxicode/src/enc/encoder.rs:46`) and `<DecoderImpl<R, C> as
Decoder>::reader` (`oxicode/src/de/decoder.rs`, the symmetric accessor).
Both impls set their associated type to exactly their own generic parameter
(`type W = W;` / `type R = R;`). At the monomorphic instance
(`W = SliceWriter<'_>`, `C = Configuration`) the driver reports:

```text
the driver lowered no body for `oxicode::enc::encoder::{impl-1}::writer::{inst-...}`:
unsupported-type(<oxicode::enc::EncoderImpl<oxicode::enc::SliceWriter<'_>,
oxicode::config::Configuration> as oxicode::enc::Encoder>::W)
```

(symmetrically for `{impl-2}::reader` and `::R`). `FORMAL_LOG=debug` shows
the driver otherwise performing D1/D2 correctly for this crate — **68
monomorphic instances lowered, all reachable**, including every
`varint_encode_*`/`varint_decode_*` function and every per-type
`Encode::encode`/`Decode::decode` impl for `u16`/`u32`/`u64`/`i32`/`i64` —
and even shows the **generic definition** of `writer`/`reader` lowering
fine as a dependency body (`&mut self.writer`, trivial). Only the two
accessor methods' *monomorphic instances* fail to lower a body, because the
instance's projected return type is left as an un-normalized associated-type
projection instead of the concrete `SliceWriter<'_>`/`SliceReader<'_>` the
substitution implies.

This blocks every harness in this package identically. It is not a
harness-writing choice: `writer()`/`reader()` are `oxicode`'s own dispatch
mechanism, called from inside every `Encode`/`Decode` impl, and there is no
public entry point into the varint codec that avoids them. No rewrite of
any harness in this file changes the outcome.

**`solver-model-rejected` count: 0.** No verification condition from this
package ever reached OxiZ (`report.json`'s own summary is "oxicode-formal
(0 VCs, 0 cached)"), so cargo-formal's pin to OxiZ 0.3.3 (upstream U-Z10,
fixed in the OxiZ working tree during Phase 2b, not yet released) is **not**
what limits this package — moving the pin changes nothing here. That is
unlike the vendored twin, where 2 rows were genuinely `solver-model-rejected`
and will move to `proved` once the pin moves; this package's own blocker is
the driver's associated-type-projection gap described above, one level
before any obligation would have reached a solver.

### Why this package's numbers differ from the vendored trial

`examples/ecosystem/oxicode-varint` (in the `cargo-formal` repository)
measured **176 proved / 1 refuted / 2 unknown** on the *same* D1/D2 driver,
which is what the harness inventory (`I0-d.md` §5.4) predicted for this
package too ("proved (post-D2)"). The difference: the vendored trial's
harnesses call the raw `varint_encode_*`/`varint_decode_*` functions through
its **own** hand-rolled `Writer`/`Reader` generics (`ArrayWriter`,
`SliceReader` defined inside the trial itself), because `oxicode::varint` is
`pub(crate)` and the trial vendored it rather than calling it. That path
never goes through `oxicode`'s real `Encoder`/`Decoder`/`EncoderImpl`/
`DecoderImpl` dispatch, so it never exercises the `writer()`/`reader()`
accessors above. This package calls the **real public API**
(`encode_to_fixed_array`, `decode_from_slice`), which is exactly what
routes every harness through that dispatch layer — an indirection the
vendored trial structurally could not exercise. This package is the first
measurement of that layer, and the reason its tally looks worse is that it
found a real, narrowly-localized driver gap the vendored trial's design made
invisible.

**Open question for whoever fixes the driver gap** (not resolvable from the
release binary alone): design v1.1 §3.2 item 2 says a call site's
`Callee::path` becomes the instance path "only after the driver has
queued/lowered that instance". The debug log is consistent with either
reading of "lowered" — "attempted" (current behaviour is as designed; the
fix is purely normalizing the `Self::W`/`Self::R` projection) or "succeeded"
(the redirect onto a bodyless instance would itself be a deviation, though a
fallback to the definition path would still have no encodable layout for an
abstract `W`/`R`). This package states both readings with the evidence and
does not guess between them.

## Package hygiene: the nested-workspace / `cargo package` interaction

`../oxicode/Cargo.toml` is both `[workspace]` and `[package]`, so
`oxicode/formal/` lands physically inside the `oxicode` package directory.
`formal/Cargo.toml` declares its own `[workspace]`, which Cargo documents as
making a directory invisible to an ancestor workspace and to that ancestor's
`cargo package` file walk. Verified, not assumed:

* `cargo package --list --locked --allow-dirty` in `../oxicode`, run
  **after** this package existed, after the three plain/runtime-checks/
  nightly builds had populated `formal/target/`, and after `cargo formal
  check` had run (with its own scratch `--target-dir`, outside
  `formal/target/`), produced **1059 entries**, byte-identical to the
  pre-existing baseline (`p2b/w0/i0d/oxicode-package-list.txt`, captured
  before `formal/` existed) — zero diff lines, and no path under `formal/`
  anywhere in the list.
* `cargo publish -p oxicode --dry-run` (same scratch `CARGO_TARGET_DIR`)
  **does not currently succeed**, exit 101: it fails while resolving
  `oxicode_derive = "^0.2.7"` against the crates.io index, which only has
  `oxicode_derive` up to `0.2.6` published. This step is independent of this
  package: cargo hits it while resolving the *registry* version of a path
  dependency during publish preparation, before any package-contents file
  walk, so it cannot depend on `formal/`'s presence. That reasoning was not
  additionally confirmed against a dry-run captured *before* `formal/`
  existed (I0-d's Wave-0 baseline recorded only `cargo package --list`, not
  a dry-run), so "unrelated to this package" is a well-supported inference
  from where the failure occurs, not a measured before/after comparison —
  flagged as such for the gatekeeper. Not fixed, per this task's scope
  (only `oxicode/formal/` may be created; no other file in `../oxicode` may
  be touched).
* `../oxicode/Cargo.lock`'s SHA-256 is unchanged across every command run
  for this package (recorded before and after: both
  `d6bdd1ea585ae73ca6ceb50dc99fd462c3fcce46034b8deb2c811858bf8e22dd`).

## In-source contract candidates

The honest end state is `#[oxiformal::requires]`/`#[ensures]` on the real
`oxicode` functions, which needs `oxiformal` on crates.io. From reading the
source (`I0-d.md` §6), with file:line — **none of these is verified by this
run** (every harness above is `unsupported`, so no contract here has been
discharged; they are candidates for a future contract-and-harness pass, not
claims this package proved):

* `SliceWriter::write` (`oxicode/src/enc/write.rs:80-88`) —
  `ensures(|r| matches!(r, Ok(()) if self.bytes_written() == old(self.bytes_written()) + bytes.len()) || matches!(r, Err(Error::UnexpectedEnd { .. })))`.
  No panic path; `self.index + len` at `:82` is an `arith-overflow`
  obligation with no guard in the source.
* `encode_to_fixed_array::<N, u64>` (`oxicode/src/lib.rs:284`) —
  `ensures(|r| r.is_ok() ==> r.as_ref().unwrap().1 <= N)`, and informally
  `requires(N >= 9)` for a *total* `u64` varint encode (this package's
  `varint_u64_bound_is_tight_harness` states exactly that boundary, at `N`
  one byte short).
* `SliceReader::read` (`oxicode/src/de/read.rs:50-60`) — `ensures` "consumes
  exactly `bytes.len()` or returns `Err`"; `remaining()` (`:44`) shrinks
  monotonically.
* `varint_decode_u16` (`oxicode/src/varint/decode_unsigned.rs:19-37`,
  `pub(crate)`, reached through `decode_from_slice::<u16>`) — `ensures`
  "rejects every discriminant wider than `U16_BYTE`" (arms at `:32-35`);
  this package's `decode_rejects_a_wide_tag_harness` states it.
* No `debug_assert!` anywhere under `src/varint/`, `src/enc/write.rs`,
  `src/de/read.rs` (grep returns nothing) — `oxicode`'s preconditions on
  this path are all encoded as `Result`, not as unchecked assumptions, which
  is why this package (unlike, say, `oxiarc`'s `BitCache`) has no
  "precondition is necessary" refutation candidate: every boundary here is
  already a checked `Err`, not a panic.

## Dependencies

```toml
oxicode   = { path = "..", default-features = false }
oxiformal = { path = "../../cargo-formal/crates/oxiformal" }  # becomes a crates.io version once published
```

`default-features = false`: none of the targeted public functions is
feature-gated (measured — `cargo check` compiles clean with no features at
all), so this package needs neither `std` nor `alloc`.
