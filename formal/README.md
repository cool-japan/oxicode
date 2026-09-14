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
measurement of `oxicode`'s actual `Encoder`/`Decoder` dispatch path (see
"How this package's numbers relate to the vendored twin" below).

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
stated, measured verdict, and the exact site of anything not proved).

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

## Measured verdict table (2026-09-14)

Release `cargo-formal` 0.1.0 CLI + release `formal-driver` with
dependency-body lowering (D1) and on-demand monomorphic instance lowering
(D2), OxiZ 0.3.3, rustc nightly-2026-06-20. 224 verification conditions,
0 cached, 6 s wall, exit 0.

| harness | obligations | measured verdict |
|---|---|---|
| `varint_u16_roundtrip_harness` | 23 (22 proved, 1 unknown) | `assert` **unknown** — 3 sites: 2 proved, 1 unknown (`src/harness.rs:151:23`) |
| `varint_u32_roundtrip_harness` | 30 (29 proved, 1 unknown) | `assert` **unknown** — 3 sites: 2 proved, 1 unknown (`src/harness.rs:173:23`) |
| `varint_u64_roundtrip_harness` | 41 (40 proved, 1 unknown) | `assert` **unknown** — 3 sites: 2 proved, 1 unknown (`src/harness.rs:195:23`) |
| `varint_u64_bound_is_tight_harness` | 20 (all proved) | `assert` **proved** — 1 site |
| `fixed_int_roundtrip_harness` | 13 (all proved) | `assert` **proved** — 3 sites |
| `zigzag_i32_roundtrip_harness` | 33 (32 proved, 1 unknown) | `assert` **unknown** — 3 sites: 2 proved, 1 unknown (`src/harness.rs:270:23`) |
| `zigzag_i64_roundtrip_harness` | 44 (43 proved, 1 unknown) | `assert` **unknown** — 3 sites: 2 proved, 1 unknown (`src/harness.rs:290:23`) |
| `decode_rejects_a_wide_tag_harness` | 12 (11 proved, 1 unknown) | `assert` **unknown** — its only site (`src/harness.rs:315:5`) |
| `decode_never_panics_on_arbitrary_bytes_harness` | 8 (all proved) | `harness` **proved** — no assertion of its own |

**218 proved / 0 refuted / 6 unknown / 0 timeout / 0 unsupported over 224
obligations.** All six `unknown`s are `solver-model-rejected` on the OxiZ
0.3.3 pin (below); nothing in this package is refuted, and nothing is
`unsupported`. Full detail and the "worst-of-sites" keying rule are in
`EXPECTED.toml`; the exact reasoning is in each harness's doc comment in
`src/harness.rs`.

### Layer counters (from the measured run)

`cargo formal check`'s own summary block, verbatim except that the two
absolute scratch paths are written as `<target-dir>`:

```text
package oxicode-formal 0.1.0               self_host: false
  functions              0   annotated 0    harnesses 9
  unsafe                 0   covered 0      uncovered 0
  hygiene            PASS
  bmc                218 proved / 0 refuted / 6 unknown
                     evidence  reproduction 224
  contract            0 proved / 0 refuted
  theorem            not run
  coverage           annotated/public 0.0%   unsafe covered 0.0%
  solver-model-rejected: 6
  dependency bodies  42 lowered (6 reachable)
  instances          68 lowered (68 reachable)
  inventory          <target-dir>/formal/inventory.json (2026-09-13T22:27:53Z)
  assumptions        MIR debug arithmetic; contract checks on; UB checks off; heap invisible; unwind=12/16; seq bound=12/16
```

and the matching `cargo formal status` block:

```text
package oxicode-formal 0.1.0               self_host: false
  functions              0   annotated 0    harnesses 9
  unsafe                 0   covered 0      uncovered 0
  hygiene            PASS
  bmc                not run in this invocation
  contract           not run in this invocation
  theorem            not run
  coverage           annotated/public 0.0%   unsafe covered 0.0%
  dependency bodies  42 lowered (6 reachable)
  instances          68 lowered (68 reachable)
  inventory          <target-dir>/formal/inventory.json (2026-09-13T22:27:53Z)
  last check         <target-dir>/formal/report.json (2026-09-13T22:27:53Z)
                     recorded  bmc 218 proved / 0 refuted / 6 unknown, contract 0 proved / 0 refuted
```

In `report.json`'s own vocabulary: `hygiene` pass (2 files scanned, 0 errors,
0 warnings, 0 notes, 0 unsafe sites); `bmc` 218 proved / 0 refuted /
6 unknown / 0 timeout / 0 unsupported / 0 unverifiable, every one of the 224
obligations backed by a `reproduction` evidence record; `contract` 0 proved /
0 refuted (this package has no `#[requires]`/`#[ensures]`, only `#[harness]`);
`theorem` not run; `coverage` 0 of 0 public functions annotated, 9 harnesses.
The extraction line reads `Checking oxicode-formal (224 VCs, 0 cached)`.

### Dependency bodies and instances

* **`dependency bodies`: 42 lowered (6 reachable).** `[package.metadata.formal]
  dep-crates = ["oxicode"]`, so D1 walks `oxicode`'s own functions —
  `encode_to_fixed_array`, `decode_from_slice`, `SliceWriter::write`,
  `SliceReader::read`, the `Encode`/`Decode` impls for the integer types, and
  the `pub(crate)` `varint_encode_*`/`varint_decode_*` functions they call —
  into this package's `.fir`. Most of the 42 are generic *definitions* whose
  concrete work is done by an instance instead, which is why only 6 are
  reachable as definitions.
* **`instances`: 68 lowered (68 reachable).** Every one of the 68 monomorphic
  instances D2 queued is reachable from a harness: `encode_to_fixed_array::<3,
  u16>` and its siblings, `<SliceReader<'_> as Reader>::read`, every per-type
  `Encode::encode`/`Decode::decode` for `u16`/`u32`/`u64`/`i32`/`i64`,
  `EncoderImpl::new`/`into_writer`, `DecoderImpl::new`/`claim_bytes_read`,
  `SliceWriter::new`/`bytes_written`, `SliceReader::new`, the `config::*`
  helpers, and — the ones that used to be the whole story — the
  `<EncoderImpl<SliceWriter<'_>, Configuration> as Encoder>::writer` and
  `<DecoderImpl<SliceReader<'_>, Configuration> as Decoder>::reader`
  accessors.

### Why the six `unknown`s are a solver pin, not a harness or driver defect

All six carry the same `report.json` message:

```text
solver-model-rejected: OxiZ 0.3.3 returned a model that does not satisfy the
verification condition
```

cargo-formal is pinned to OxiZ `=0.3.3`, whose model gate (upstream item
**U-Z10**) turns a model that fails the driver's *mandatory* model check into
`unknown` rather than an invented counterexample — the conservative outcome,
and the reason none of these six is reported as a refutation. U-Z10 is fixed
in the `../oxiz` working tree but is **unreleased**, so the pin stays at
`=0.3.3`; these six rows are *expected* to move to `proved` once that release
ships. No 0.3.4 run has been measured, so that is an expectation, not a
measurement.

Where the six sit matters, and it differs between the round-trip harnesses
and the wide-tag one:

* In the five round-trip harnesses (`u16`, `u32`, `u64`, `i32`, `i64`) the
  `unknown` is the `Err(_) => assert(false)` arm of the **inner**
  `decode_from_slice` match — the statement "decoding the bytes the encoder
  just produced cannot fail". The headline round-trip equalities,
  `decoded == value` and `consumed == written`, are **proved** at every width,
  including the zigzag extremes (`i32::MIN`, `i64::MIN`). The *outer*
  `Err(_) => assert(false)` arm (encode failure into a worst-case-size buffer)
  has no item in `report.json` at all.
* In `decode_rejects_a_wide_tag_harness` the `unknown` is the harness's
  **only** assertion and its whole point, so the stated property — a `u16`
  decode refuses a 252/253/254 tag byte — is *not established on this pin*.
  Its 11 incidental obligations are all proved.

This run raised **no** `unwinding-assertion` obligation anywhere, so it gives
no evidence either way about the `unwind` bounds (12 for the package, 16 for
`decode_never_panics_on_arbitrary_bytes_harness`).

### History: why this package used to report nine `unsupported(no-body)`

Before Phase 2b's **`P2-14`** landed, every harness here was a whole-harness
`unsupported(no-body)` — 0 VCs reached OxiZ, and `EXPECTED.toml` recorded nine
`harness = "unsupported"` rows. The cause was narrow and specific, and it is
history now, not the current state:

`oxicode` reaches its varint codec only through `encoder.writer()` /
`decoder.reader()` — `<EncoderImpl<W, C> as Encoder>::writer` and
`<DecoderImpl<R, C> as Decoder>::reader`, called from inside every
`Encode`/`Decode` impl the crate ships. Both impls set their associated type
to exactly their own generic parameter (`type W = W;` / `type R = R;`). The
driver did **not** normalize associated-type projections in a *monomorphic
instance* signature, so at `W = SliceWriter<'_>`, `C = Configuration` the
signature still carried the un-normalized projection
`<EncoderImpl<SliceWriter<'_>, Configuration> as Encoder>::W` (symmetrically
`::R`), and the instance was refused before its body was ever requested. As
the accessors are `oxicode`'s only dispatch path into the codec, that one gap
blocked all nine harnesses identically and no harness rewrite could avoid it.

`P2-14` fixes it by normalizing an instance signature with the **fallible**
normalizer under `TypingEnv::fully_monomorphized()` — see the `P2-14` bullet
in `cargo-formal`'s `CHANGELOG.md`, which names this exact projection shape.
All nine harnesses now encode; the 68 instances above include the two
accessors that used to fail. The design v1.1 §3.2 open question this README
used to raise (whether "queued/lowered" meant attempted or succeeded) is
answered by the same bullet: `Callee::path` is rewritten at **queue** time,
so every callee path always names a `Function` entry.

### How this package's numbers relate to the vendored twin

`examples/ecosystem/oxicode-varint` (in the `cargo-formal` repository)
measured **176 proved / 1 refuted / 2 unknown**. It exists because, before
Phase 2b, the driver could not lower a path dependency's bodies at all, so
the trial vendored five `pub(crate)` files of `oxicode/src/varint/` and drove
them through its **own** hand-rolled `Writer`/`Reader` generics (`ArrayWriter`,
`SliceReader`, defined inside the trial). That bypasses `oxicode`'s real
`Encoder`/`Decoder`/`EncoderImpl`/`DecoderImpl` dispatch entirely. This
package calls the real public API and therefore goes *through* that dispatch —
68 monomorphic instances, the `writer()`/`reader()` accessors among them.

**Both now encode.** The difference in the tallies is scope and statement, not
capability:

* The twin's `varint_u64_bound_is_tight_harness` is `refuted` (counterexample
  `u64::MAX`) while this package's same-named harness is `proved`. They state
  different propositions: the twin asserts that an 8-byte writer holds a
  `u64`, which is false; this package asserts the biconditional
  `result.is_err() == (value > u32::MAX as u64)`, which is true.
* The twin's two `unknown`s sit on the same inner-decode
  `Err(_) => assert(false)` arm as five of this package's six — in the twin
  at its `src/harness.rs:200:19` and `:220:19`, here at
  `src/harness.rs:151:23`, `:173:23`, `:195:23`, `:270:23` and `:290:23` —
  and the twin's `decoded == value` equalities (its `:199:24`, `:219:24`) are
  proved too. Same root cause (OxiZ 0.3.3, U-Z10) and the same shape of site;
  this package additionally carries the wide-tag `unknown` (`:315:5`), which
  the twin has no counterpart for.
* The obligation counts differ (179 vs 224) because this package's call
  graph includes the real dispatch and configuration layers the twin's
  vendored copy does not have.

## Package hygiene: the nested-workspace / `cargo package` interaction

`../oxicode/Cargo.toml` is both `[workspace]` and `[package]`, so
`oxicode/formal/` lands physically inside the `oxicode` package directory.
`formal/Cargo.toml` declares its own `[workspace]`, which Cargo documents as
making a directory invisible to an ancestor workspace and to that ancestor's
`cargo package` file walk. Verified, not assumed — but note that this section
alone carries numbers **measured 2026-09-08 and not re-measured for the
2026-09-14 run** above:

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
* `../oxicode/Cargo.lock`'s SHA-256 was unchanged across every command run
  for this package (recorded before and after **on 2026-09-08**: both
  `d6bdd1ea585ae73ca6ceb50dc99fd462c3fcce46034b8deb2c811858bf8e22dd`). That
  root lockfile is git-ignored, and it has since been re-resolved by a later
  `cargo` invocation in `../oxicode` unrelated to `formal/`, so its hash
  today no longer matches the one recorded here; re-record if this section
  is re-measured.

## In-source contract candidates

The honest end state is `#[oxiformal::requires]`/`#[ensures]` on the real
`oxicode` functions, which needs `oxiformal` on crates.io. From reading the
source (`I0-d.md` §6), with file:line. **None of these is a discharged
contract**: this package states its properties as `#[harness]`es and carries
no `#[requires]`/`#[ensures]` at all, so the `contract` layer measured
0 proved / 0 refuted. Where an obligation below *is* proved by this run, it is
proved as an incidental MIR-inserted check under the harnesses' reachable
inputs, which is strictly weaker than the contract it suggests.

* `SliceWriter::write` (`oxicode/src/enc/write.rs:80-88`) —
  `ensures(|r| matches!(r, Ok(()) if self.bytes_written() == old(self.bytes_written()) + bytes.len()) || matches!(r, Err(Error::UnexpectedEnd { .. })))`.
  No panic path. The `self.index + len` at `:83` (and again at `:86`/`:87`)
  has no guard in the source; it is an `arith-overflow` obligation and this
  run proves it (in every encoding harness) *for the inputs those harnesses
  reach*, not for all callers.
* `encode_to_fixed_array::<N, u64>` (`oxicode/src/lib.rs:284`) —
  `ensures(|r| r.is_ok() ==> r.as_ref().unwrap().1 <= N)`, and informally
  `requires(N >= 9)` for a *total* `u64` varint encode. This package's
  `varint_u64_bound_is_tight_harness` states exactly that boundary at `N` one
  byte short, and this run proves *that biconditional*
  (`result.is_err() == (value > u32::MAX as u64)` at `N = 8`) — not the
  `ensures`/`requires` clauses above, which remain undischarged candidates.
* `SliceReader::read` (`oxicode/src/de/read.rs:50-60`) — `ensures` "consumes
  exactly `bytes.len()` or returns `Err`"; `remaining()` (`:44`) shrinks
  monotonically. The `slice-range` obligations in that function are proved
  under every decoding harness here.
* `varint_decode_u16` (`oxicode/src/varint/decode_unsigned.rs:19-37`,
  `pub(crate)`, reached through `decode_from_slice::<u16>`) — `ensures`
  "rejects every discriminant wider than `U16_BYTE`" (arms at `:32-35`).
  This package's `decode_rejects_a_wide_tag_harness` states it, and that is
  the one harness whose own assertion is `unknown` on the OxiZ 0.3.3 pin — so
  this property is currently *stated and encoded but not established*.
* No `debug_assert!` anywhere under `src/varint/`, `src/enc/write.rs`,
  `src/de/read.rs` (grep returns nothing) — `oxicode`'s preconditions on
  this path are all encoded as `Result`, not as unchecked assumptions, which
  is why this package (unlike, say, `oxiarc`'s `BitCache`) has no
  "precondition is necessary" refutation candidate: every boundary here is
  already a checked `Err`, not a panic. That is consistent with the measured
  0 refutations.

## Dependencies

```toml
oxicode   = { path = "..", default-features = false }
oxiformal = { path = "../../cargo-formal/crates/oxiformal" }  # becomes a crates.io version once published
```

`default-features = false`: none of the targeted public functions is
feature-gated (measured — `cargo check` compiles clean with no features at
all), so this package needs neither `std` nor `alloc`.
