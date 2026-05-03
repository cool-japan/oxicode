# OxiCode Benchmarks

This document describes the benchmark suite shipped with OxiCode, explains how to
run and interpret results, and provides guidance for adding new benchmarks.

---

## 1. Overview

### `benches/encoding_bench.rs`

Covers the oxicode-only encode path for scalar types (u32, i64), collections
(Vec<u32>, String), and small/medium/large structs.  Also benchmarks a
selection of standard-library types that OxiCode supports natively: Duration,
IpAddr (v4 and v6), Range<u32>, and Option<Vec<u8>>.  Approximately 30 bench
functions spread across 7 criterion groups.  No bincode comparison in most
groups — the focus is absolute throughput of the oxicode encoder.

### `benches/decoding_bench.rs`

Mirror of the encoding bench but for the decode direction.  About 15 bench
functions comparing oxicode and bincode side-by-side on the same byte payloads:
primitive decode, string decode (both allocating and zero-copy borrow), Vec<u32>,
large strings, structs with multiple fields, Vec<u64>, and HashMap<String, u64>.
Each group uses `Throughput::Bytes` keyed to the raw payload size.

### `benches/comparison_bench.rs`

The most comprehensive cross-library comparison: approximately 50 bench functions
across ~785 lines.  Puts oxicode, bincode 2.0.1, rkyv, postcard, and borsh
side-by-side on the same shared payload sizes (16 B, 256 B, 1 KB, and larger).
Payloads include structs with primitive-heavy fields, string-heavy fields, and
deeply nested data.  Uses `BenchmarkId` so criterion groups results by input
size and library for easy visual comparison.

### `benches/config_matrix_bench.rs`

Tests five distinct configuration combinations on encode, decode, and byte-size
sub-groups, each parameterised over three payload sizes (16, 128, and 1024
elements):

| Label      | Integer encoding | Endian        |
|------------|-----------------|---------------|
| oxi_std    | varint          | little-endian |
| oxi_legacy | fixed-int       | little-endian |
| oxi_be     | varint          | big-endian    |
| bin_std    | varint          | little-endian |
| bin_legacy | fixed-int       | little-endian |

Throughput is reported via `criterion::Throughput::Bytes` based on an estimate
of the raw data size.  The byte-size sub-group measures the combined allocation
and encoding cost and captures the `Vec::len()` of the result, making it easy
to spot wire-format size regressions at a glance.

### `benches/serde_bench.rs`

Compares `oxicode::serde` against `bincode::serde` (the optional serde
compatibility layers of each library) on primitives (u64, String), small
structs, medium structs of varying element counts, Vec<u32>, and
HashMap<String, u64>.  Both sides use `serde::Serialize` / `serde::Deserialize`
only — no native Encode/Decode derive macros — so results isolate the
serde-dispatch overhead from the core encoder.  Gated behind `--features serde`
because `required-features = ["serde"]` is set in Cargo.toml.

### `benches/api_surface_bench.rs`

Exercises public APIs not covered by the other files:

- `encode_to_fixed_array` — stack allocation, no heap, for u32, a small struct, and a byte slice.
- `encode_seq_to_vec` — exact-size iterator path (no intermediate Vec) vs the `encode_to_vec` baseline.
- `decode_iter_from_slice` — lazy iterator that sums elements without materialising a Vec, compared to the eager slice decode.
- File I/O round-trip — `encode_to_file` + `decode_from_file` on 64 B, 512 B, and 4 KB payloads. Results depend on disk speed; use for relative comparisons only.
- Streaming write/read — `StreamingEncoder` + `StreamingDecoder` on varying item counts.
- `encoded_size` — `SizeWriter` cost compared to `encode_to_vec().len()` (the allocation cost is the signal, not raw throughput).
- Checked round-trip — `encode_to_vec_checked` + `decode_from_slice_checked` (only registered when compiled with `--features checksum`).
- Container breadth — BTreeMap, HashSet, Result, nested Vec, enum variants (unit, tuple, struct), and a combined BreadthPayload struct.

---

## 2. How to run

```bash
# All benches, all features
cargo bench --all-features

# Single file
cargo bench --bench config_matrix_bench
cargo bench --bench serde_bench --features serde
cargo bench --bench api_surface_bench

# Compile-only check (no execution)
cargo bench --no-run --all-features
```

---

## 3. Comparing baselines

```bash
# Record a baseline
cargo bench --all-features --save-baseline before

# After a code change, compare
cargo bench --all-features --baseline before
```

Criterion HTML reports land at `target/criterion/report/index.html`.  Open that
file in a browser to browse interactive charts grouped by benchmark name and
input size.

---

## 4. Interpreting results

**oxi_std vs bin_std**: oxicode uses varint encoding for small integers; expect
oxicode to produce smaller output on typical structs with u32/u64 fields whose
values are below 128.  Throughput may be slightly lower on fixed-size primitives
where bincode skips varint overhead.

**oxi_legacy vs bin_legacy**: should produce identical byte output (bincode 1.x
wire-format compatibility invariant).  Any throughput difference is purely an
implementation detail; byte sizes should match exactly.

**oxi_be**: big-endian varint encoding; expect slightly lower throughput than
oxi_std (same algorithm, plus byte-swap overhead).  Wire format is NOT compatible
with bincode.

**oxicode::serde vs oxicode derive**: the serde path goes through an extra
trait-dispatch layer; expect 5-15% lower throughput than the derive path for the
same payload.

**decode_from_reader API note**: oxicode's `decode_from_reader` uses
`config::standard()` internally (no config generic); bincode's
`decode_from_std_read` accepts an explicit config.  The bench uses
`config::standard()` on both sides for an apples-to-apples comparison.

**File I/O numbers depend on disk speed** — treat `file_io_round_trip` results
as relative (same machine, same run), not absolute.  Use small payloads (64 B –
4 KB) for stable baselines.

**encoded_size vs bincode**: bincode 2.0.1 has no direct equivalent to
`encoded_size`; the bench compares against `bincode::encode_to_vec().len()`.
The allocation cost is the signal, not throughput.

**checked_round_trip benches** are only registered when compiled with
`--features checksum`.  Without that feature the bench group is absent (not an
error).

---

## 5. Adding a new bench

1. Copy the closest existing bench file as a starting point.
2. Add a `[[bench]]` entry to the root `Cargo.toml` with `harness = false`:
   ```toml
   [[bench]]
   name = "my_new_bench"
   harness = false
   ```
3. Register the bench function with `criterion_group!` and `criterion_main!`.
4. Use `group.throughput(Throughput::Bytes(n))` for size-keyed benches;
   `group.throughput(Throughput::Elements(n))` for element-count-keyed benches
   (iterators, collections).
5. Wrap all bench inputs and outputs in `std::hint::black_box(...)` to prevent
   the compiler from optimising away the work.

---

## 6. CI note

Benchmarks are **not run in CI** (per project policy — only `pypi-publish.yml` /
`npm-publish.yml` workflows are permitted in `.github/workflows/`).  All bench
runs are on-demand local tools.  Do not add bench invocations to CI yaml files.
