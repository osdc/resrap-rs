# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
