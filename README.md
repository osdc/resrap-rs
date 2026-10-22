# Resrap for Rust

[![Crates.io](https://img.shields.io/crates/v/resrap-rs.svg)](https://crates.io/crates/resrap-rs)
[![Rust Docs](https://img.shields.io/badge/docs.rs-resrap--rs-orange?logo=rust&logoColor=white)](https://docs.rs/resrap-rs)
[![CI](https://github.com/osdc/resrap-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/osdc/resrap-rs/actions/workflows/ci.yml)
[![Landing](https://img.shields.io/badge/docs-resrap.osdc.dev-blue)](https://resrap.osdc.dev)

*Just a parser… in reverse.*

Resrap is a **seedable, grammar-based code and text generator**. Instead of
parsing source code, it walks a grammar graph in reverse and emits source that
follows the grammar's structure. It is a Rust port of
[`github.com/osdc/resrap`](https://github.com/osdc/resrap) and is designed to
be API- and behaviour-compatible with the Go package, including **identical
seeded output**.

Use it for:

- realistic test fixtures and parser/highlighter stress tests
- typing practice with generated code
- fuzzing and property testing
- procedural content generation
- multi-file sample project generation

The crate has **no runtime dependencies**.

## Installation

```bash
cargo add resrap-rs
```

Or add it to `Cargo.toml`:

```toml
[dependencies]
resrap-rs = "1.0"
```

The minimum supported Rust version is **1.85** (Rust 2024 edition).

## Quick start

```rust
use resrap_rs::{templates, Resrap};

fn main() -> Result<(), String> {
    let mut resrap = Resrap::new();

    // Ten grammars are embedded in the crate.
    templates::load(&mut resrap, templates::Lang::C)?;

    // Random output.
    println!("{}", resrap.generate_random("c", "program", 200));

    // Deterministic output: the same seed always produces the same bytes.
    println!("{}", resrap.generate_with_seeded("c", "program", 7, 200));

    // Finish the starting rule instead of cutting a statement in half.
    println!("{}", resrap.generate_complete_with_seeded("c", "program", 7, 500));

    Ok(())
}
```

A grammar can also be loaded from a string or a file:

```rust
use resrap_rs::Resrap;
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
resrap.parse_grammar("Greeting", "program:'hello ' [a-z] '!';")?;
resrap.parse_grammar_file("C", "grammars/c.g4")?;
Ok(())
}
```

Unknown grammar names and unknown start rules return an empty string, matching
the Go implementation.

## Embedded templates

The following grammars are compiled into the binary:

| Name       | Language   | Template value        |
| ---------- | ---------- | --------------------- |
| `c`        | C          | `templates::Lang::C`  |
| `cpp`      | C++        | `templates::Lang::Cpp` |
| `go`       | Go         | `templates::Lang::Go` |
| `haskell`  | Haskell    | `templates::Lang::Haskell` |
| `html`     | HTML       | `templates::Lang::Html` |
| `java`     | Java       | `templates::Lang::Java` |
| `js`       | JavaScript | `templates::Lang::JavaScript` |
| `python`   | Python     | `templates::Lang::Python` |
| `rust`     | Rust       | `templates::Lang::Rust` |
| `sql`      | SQL        | `templates::Lang::Sql` |

```rust
use resrap_rs::{templates, Resrap};
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
templates::load(&mut resrap, "cpp")?;          // by short name
templates::load(&mut resrap, templates::Lang::Python)?;
templates::load_as(&mut resrap, "sql", "queries")?; // custom grammar name
Ok(())
}
```

`templates::names()`, `templates::langs()`, `templates::list()` and
`templates::get()` expose the same data for pickers and tooling.

## Generation modes

All generation methods take a grammar name, a start rule and a token budget.

| Method | Deterministic | Stops at rule boundary |
| ------ | ------------- | ---------------------- |
| `generate_random` | No | No |
| `generate_with_seeded` | Yes | No |
| `generate_complete` | No | Yes |
| `generate_complete_with_seeded` | Yes | Yes |

A seed of `0` means "choose a random seed", exactly as in the Go package. For a
deterministic result, use any non-zero seed. `generate_with_seed` is provided as
an alias for `generate_with_seeded`.

The normal methods stop exactly at the token budget, so output can end in the
middle of an expression. The `complete` variants treat the budget as a soft
limit and stop when the starting rule reaches its end node, producing output
that is usually syntactically complete.

## Grammar format

Resrap uses **ABNF (Awesome BNF)**, a small EBNF dialect with weighted choices
and infinite loops:

```abnf
// A tiny C-like grammar.
program : (header+<0.4>) function^;
header  : '#include <' identifier '.h>\n';
function: datatype ' ' identifier '(' ')' '{' '\n' statement* '}';
statement: identifier ' = ' [0-9]+ ';\n';

@words identifier = count, total, index, value
@words datatype   = int, long, double
```

Key features:

- **Sequences and rules:** `name : ... ;`
- **Standard repetition:** `+`, `*`, `?`, and grouping with `()`
- **Infinite generation:** `^` loops the preceding construct
- **Weighted choices:** `<0.2>` after a branch or repetition
- **Word lists:** `@words` / `@keywords` (see below)
- **Comments and escapes:** `//`, `#`, `\n`, `\t`, `\\`, `\'`, `\"`
- **Character classes and ranges:** `[a-z0-9_]`

See [docs/ABNF.md](docs/ABNF.md) for the complete language reference.

## Word lists

Word lists replace a rule's expansion with a readable token. They can be
declared in the grammar:

```abnf
@words identifier = count, total, index, value, result
@keywords datatype = int, long, double

program: datatype ' ' identifier ';\n';
```

or attached at runtime:

```rust
use resrap_rs::Resrap;
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
resrap.parse_grammar("C", "program:identifier;\nidentifier:[a-z];")?;
resrap.add_words("C", "identifier", &["count".into(), "total".into()])?;
resrap.add_keywords("C", "identifier", &["index".into()])?; // alias
Ok(())
}
```

`@keywords` and `add_keywords` are aliases for `@words` and `add_words`.
Word lists apply to every reference to the rule, including the starting rule.

## Codebase generation

A small DSL describes a directory tree and the files to generate:

```dsl
src/
  code/
    c[10x200 code_*.c]
    core/
      c[5x1000 *.c]
  more/
    sql[10x100 *.sql]
```

`c[10x200 code_*.c]` means: use the grammar named `c`, generate **10** files
with **200** tokens each, named `code_1.c`, `code_2.c`, and so on. Files are
generated with `generate_complete`, so each file ends at a rule boundary.

```rust
use resrap_rs::{templates, Resrap};

fn main() -> Result<(), String> {
    let mut resrap = Resrap::new();
    templates::load(&mut resrap, templates::Lang::C)?;
    templates::load(&mut resrap, templates::Lang::Sql)?;

    resrap.generate_codebase("project.dsl", "output/")?;
    Ok(())
}
```

The DSL can also be parsed and inspected without writing anything:

```rust
use resrap_rs::parse_dsl;
fn demo() -> Result<(), String> {
let root = parse_dsl("src/\n  c[2x100 hello_*.c]\n")?;
assert_eq!(root.children.len(), 1);
Ok(())
}
```

See [docs/Codebase_gen.md](docs/Codebase_gen.md) for details.

## Multithreaded generation

`ResrapMT` runs a worker pool over an immutable grammar graph:

```rust
use resrap_rs::{templates, ResrapMT};

fn main() -> Result<(), String> {
    let mut resrap = ResrapMT::new(8, 4096); // workers, pending-job queue size
    templates::load(&mut resrap, templates::Lang::C)?;
    resrap.start_resrap();

    let channel = resrap.get_code_channel();
    for id in 0..100u64 {
        resrap.generate_with_seeded(
            &format!("job-{id}"),
            "c",
            "program",
            id + 1,
            200,
        )?;
    }

    for _ in 0..100 {
        let result = channel.recv().map_err(|err| err.to_string())?;
        println!("{}: {} bytes", result.id, result.code.len());
    }

    resrap.shutdown_resrap();
    Ok(())
}
```

Each job returns a `CodeGenRes { code, id }`. IDs are caller-supplied and must
be unique if you need to map results back to requests. Generation results are
buffered in an unbounded channel, so workers never block on a slow result
consumer. Submitting after shutdown returns `ERR_SHUT_DOWN`.

See [docs/ResrapMT.md](docs/ResrapMT.md) for the full API.

## Go seed parity

Seeded generation for the grammar engine is tested **byte-for-byte** against the
Go reference package. `tests/data/go_parity_data.rs` contains outputs generated
by Go for inline grammars exercising probabilities, words, escapes, regex
classes, infinite loops and complete-mode boundaries.

The integration test in `tests/go_parity.rs` regenerates the same output with
Rust and compares it exactly. This includes reference quirks such as `*`
consuming two probability values, so output stays compatible rather than merely
similar.

The same fixture also pins all ten embedded templates at seeds 1, 7 and 42 as
Rust regression snapshots. The templates use expanded word lists for more
realistic output, so their seeded bytes intentionally differ from the Go
package. Regenerate those snapshots after an intentional template change.

## Benchmarks

Measured on the development machine used for this release (release profile,
8-worker MT runs). Numbers are indicative; run the benchmarks on your own
hardware for comparable figures. The interesting workload for sustained
generation is long output, not 200-token snippets.

### Long outputs

`cargo bench --bench long_generation`:

| Workload | Rust | Go reference |
| -------- | ---: | -----------: |
| Seeded, 100K tokens | 8.48M tok/s (166 MiB/s) | 6.22M tok/s (122 MiB/s) |
| Complete, 100K tokens | 8.50M tok/s (167 MiB/s) | 6.69M tok/s (131 MiB/s) |
| Seeded, 1M tokens | **8.33M tok/s** (0.120 s, 163 MiB/s) | 8.07M tok/s (0.124 s, 158 MiB/s) |
| Complete, 1M tokens | **8.46M tok/s** (0.118 s, 165 MiB/s) | 8.12M tok/s (0.123 s, 159 MiB/s) |
| 8 workers, 32 x 100K tokens | **40.0M tok/s** (784 MiB/s) | 33.0M tok/s (645 MiB/s) |
| 8 workers, 8 x 1M tokens | **38.3M tok/s** (748 MiB/s) | 30.9M tok/s (605 MiB/s) |

A 1M-token C output is about 19.6 MiB. Per-token throughput stays almost flat
from 10K to 1M tokens, so time scales linearly with the token budget. Go's
shorter long-output runs vary more between samples because of GC; both
implementations settle near 8M tokens/s at 1M tokens.

These runs exposed the main long-output bottleneck: every graph transition was
cloning and dropping an `Arc` around the target node, which turned millions of
transitions into atomic refcount traffic across worker threads. Frozen graph
nodes are now stored directly and walks borrow them, eliminating that traffic.

### Short outputs

`cargo bench --bench generation`:

| Benchmark | Rust | Go reference |
| --------- | ---: | -----------: |
| Parse embedded C template | 0.117 ms/iter (8,524 ops/s) | 0.169 ms/iter (5,901 ops/s) |
| Seeded generation, 200 tokens | 0.025 ms/iter (40,306 ops/s) | 0.027 ms/iter (36,993 ops/s) |
| Complete generation, 500 tokens | 0.062 ms/iter (16,004 ops/s) | 0.076 ms/iter (13,241 ops/s) |
| Random generation, 200 tokens | 0.025 ms/iter (40,078 ops/s) | 0.026 ms/iter (39,104 ops/s) |
| Generation throughput | 165 MB/s (4,368 B/output) | 120 MB/s (4,368 B/output) |
| Worker pool (8), 200 tokens | 188,053 jobs/s | 118,396 jobs/s |
| Worker pool (8), complete 500 | 76,232 jobs/s | 36,099 jobs/s |

Worker-pool scaling for 200-token jobs:

| Workers | Rust | Go |
| ------: | ---: | ---: |
| 1 | 36,062 jobs/s | — |
| 2 | 70,832 jobs/s | — |
| 4 | 123,170 jobs/s | — |
| 8 | 188,053 jobs/s | 118,396 jobs/s |

Worker-pool scaling for complete 500-token jobs:

| Workers | Rust | Go |
| ------: | ---: | ---: |
| 1 | 15,208 jobs/s | — |
| 2 | 29,609 jobs/s | — |
| 4 | 51,487 jobs/s | — |
| 8 | 76,232 jobs/s | 36,099 jobs/s |

### Memory

`cargo run --release --example memory` reports `VmRSS` and peak RSS on Linux.
Typical figures on the same machine:

| Scenario | Rust RSS | Go reference |
| -------- | -------: | -----------: |
| All ten templates loaded | 3.4 MiB | 4.4 MiB |
| 10,000 outputs retained (40.8 MiB payload) | 53 MiB | 81 MiB |
| One 1M-token output retained (19.6 MiB payload) | 26 MiB | 50 MiB |
| 10,000 MT jobs submitted, consumer idle | 34 MiB | 59 MiB |
| After dropping/draining outputs | 6 MiB | 60 MiB |

The RSS after dropping is allocator-dependent: Rust returns most of the memory
to the OS on this workload, while the Go runtime keeps its heap reserved. Both
implementations buffer MT results without bound, matching the reference
behaviour; a slow consumer can therefore grow memory by the size of the
generated output it has not read yet.

The benchmark harnesses are dependency-free and live in
[`benches/generation.rs`](benches/generation.rs) and
[`benches/long_generation.rs`](benches/long_generation.rs).

## Documentation

- [ABNF grammar reference](docs/ABNF.md)
- [Single-threaded API](docs/Resrap.md)
- [Multithreaded API](docs/ResrapMT.md)
- [Codebase generation](docs/Codebase_gen.md)
- [API documentation on docs.rs](https://docs.rs/resrap-rs)

## Development

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo bench --bench generation
cargo bench --bench long_generation
cargo run --release --example memory
cargo doc --no-deps
```

## License

MIT. See [LICENSE](LICENSE).
