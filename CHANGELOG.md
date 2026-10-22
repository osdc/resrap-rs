# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-10-22

### Added

- First stable release of the Rust port, covering the single-threaded and
  multithreaded APIs, embedded templates, codebase generation and Go seed
  compatibility for inline grammars.

### Changed

- Expanded the embedded templates' `@words` / `@keywords` lists with status,
  message, action and domain vocabulary, and replaced random four-letter
  identifier suffixes with word-based names such as `validate_order` and
  `TypeInvoice`. Generated code is more varied and readable.
- Wired previously unused word lists into template output (labels, messages and
  SQL status values) and added an HTML row-status list.
- Embedded template fixtures in `tests/data/go_parity_data.rs` are now Rust
  regression snapshots rather than Go byte-parity cases. Inline grammar parity
  with the Go package is unchanged.

## [0.2.1] - 2026-10-22

### Changed

- Replaced the shared mutex-protected job receiver with one bounded queue per
  worker, eliminating a serialised OS-thread handoff for every job.
- Resolve the grammar and start rule at submission time and pass a shared graph
  snapshot to workers, removing per-job grammar locks and name lookups.
- Snapshot word lists once per walk instead of taking a read lock on every
  pointer node.
- Removed the redundant result dispatcher thread; workers now send directly to
  the unbounded result channel.
- Build the frozen graph in a single allocation pass instead of constructing
  every node twice.
- Store frozen graph nodes directly and borrow them during walks, removing the
  `Arc` clone/drop pair that previously ran on every graph transition. This
  eliminates atomic refcount contention between worker threads.
- Preallocate long generation output up to 64 MiB and append decoded literals
  directly, avoiding repeated buffer growth and a temporary allocation per
  terminal.
- Borrow parser tokens instead of cloning them on every lookup.
- Added worker-count scaling to the benchmark harness, a long-output
  (10K/100K/1M token) benchmark and a Linux memory example that reports RSS and
  peak RSS, including a 1M-token output.

### Performance

- Worker-pool throughput for 200-token jobs improved from about 50k to about
  190k jobs/s at 8 workers; complete 500-token jobs reach about 76k jobs/s.
- Single-threaded 1M-token generation runs at about 8.4M tokens/s (0.119 s,
  164 MiB/s), up from about 7.1M tokens/s.
- Eight workers generate 8 x 1M tokens at about 38M tokens/s, up from about
  21M tokens/s.
- Parsing the embedded C template improved from about 0.158 ms to 0.117 ms per
  iteration.

## [0.2.0] - 2026-10-22

### Added

- Complete Rust port of the Go `resrap` package's single-threaded API:
  grammar parsing from strings and files, random generation, seeded generation,
  complete generation and codebase generation.
- Multithreaded `ResrapMT` worker pool with a cloneable `CodeChannel`,
  unbounded result dispatch, graceful shutdown and shutdown-safe submission.
- Embedded templates for C, C++, Go, Haskell, HTML, Java, JavaScript, Python,
  Rust and SQL, plus `templates::load`, `load_as`, `get`, `must_get`, `list`,
  `names` and `langs`.
- `@words` / `@keywords` grammar directives and runtime `add_words` /
  `add_keywords` support, including word lists on starting rules.
- ABNF support for probabilities, infinite `^` loops, groups, `+`, `*`, `?`,
  character classes, comments and escapes.
- Byte-for-byte Go seed-parity fixtures for inline grammars and all embedded
  templates.
- Dependency-free benchmark harness in `benches/generation.rs`.
- API documentation, grammar reference, codebase DSL documentation, examples,
  CI workflow and MIT license.

### Changed

- Replaced the previous experimental graph walker with a frozen graph that
  stores edge targets by node ID, fixing self-edges and deep template traversal.
- Per-rule end nodes make `^` terminate at the correct rule boundary.
- Generation now matches the reference implementation's probability handling,
  including the `*` operator's two probability reads.
- Unknown grammars and start rules return an empty string instead of panicking.

### Fixed

- Failed grammar parses no longer replace a previously loaded grammar.
- Seeded generation now matches Go for every embedded template at multiple
  seeds.
- Complete generation no longer truncates output mid-statement.
- Multithreaded shutdown drains pending results before closing the channel.
