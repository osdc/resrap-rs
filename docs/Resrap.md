# Single-threaded API (`Resrap`)

`Resrap` owns one or more parsed grammars and generates content on the calling
thread. Parsed grammar graphs are immutable afterwards, so references can be
shared with worker threads (as `ResrapMT` does).

```rust
use resrap_rs::Resrap;

let mut resrap = Resrap::new();
```

## Loading grammars

### `parse_grammar(name, grammar) -> Result<(), String>`

Parses a grammar string and stores it under `name`. The name is an arbitrary
identifier used later for generation. A previously loaded grammar with the same
name is replaced only if parsing succeeds; failed parses leave the old grammar
untouched.

```rust
use resrap_rs::Resrap;
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
resrap.parse_grammar("Greeting", "program:'hello ' [a-z] '!';")?;
Ok(())
}
```

### `parse_grammar_file(name, path) -> Result<(), String>`

Reads and parses a grammar file. File-system errors are returned as strings.

```rust
use resrap_rs::Resrap;
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
resrap.parse_grammar_file("C", "grammars/c.g4")?;
Ok(())
}
```

### Embedded templates

```rust
use resrap_rs::{templates, Resrap};

let mut resrap = Resrap::new();
templates::load(&mut resrap, "c")?;
templates::load(&mut resrap, templates::Lang::Sql)?;
templates::load_as(&mut resrap, "python", "py")?;
```

See [templates.rs](../src/templates.rs) for `names`, `langs`, `list`, `get`,
`must_get` and template metadata.

## Generation

All methods take a grammar name, a start rule and a token budget. The token
budget counts emitted terminals (characters, regex characters and words), not
bytes or grammar nodes.

### `generate_random(name, starting_node, tokens) -> String`

Non-deterministic generation. A random seed is chosen for every call.

### `generate_with_seeded(name, starting_node, seed, tokens) -> String`

Deterministic generation. The same grammar, start rule, seed and token budget
always produce the same output. A seed of `0` means "pick a random seed", so use
a non-zero seed when you need reproducibility.

`generate_with_seed` is an alias for this method.

### `generate_complete(name, starting_node, tokens) -> String`

Like `generate_random`, but the token budget is a soft limit. Generation stops
at the end of the starting rule instead of cutting output in the middle of an
expression. This is the mode used by codebase generation.

### `generate_complete_with_seeded(name, starting_node, seed, tokens) -> String`

Deterministic variant of `generate_complete`.

```rust
use resrap_rs::{templates, Resrap};
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
templates::load(&mut resrap, "c")?;

let a = resrap.generate_with_seeded("c", "program", 42, 200);
let b = resrap.generate_with_seeded("c", "program", 42, 200);
assert_eq!(a, b);
Ok(())
}
```

If the grammar name or start rule is unknown, generation returns an empty
string. It does not panic.

## Word lists

### `add_words(name, rule, words) -> Result<(), String>`

Attaches a word list to a rule in a loaded grammar. References to that rule
emit one word uniformly instead of expanding its grammar.

```rust
use resrap_rs::Resrap;
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
resrap.parse_grammar("C", "program:identifier;\nidentifier:[a-z];")?;
resrap.add_words("C", "identifier", &["count".into(), "total".into()])?;
Ok(())
}
```

An unknown grammar or rule returns an error. `add_keywords` is an alias for
`add_words`.

## Codebase generation

### `generate_codebase(config_path, target) -> Result<(), String>`

Parses a directory DSL and writes generated files below `target`. See
[Codebase_gen.md](Codebase_gen.md).

```rust
use resrap_rs::{templates, Resrap};
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
templates::load(&mut resrap, templates::Lang::C)?;
resrap.generate_codebase("project.dsl", "output/")?;
Ok(())
}
```

## Notes

- `Resrap` is single-threaded. Use [`ResrapMT`](ResrapMT.md) for parallel work.
- Grammar graphs are immutable after parsing, so generation only needs shared
  access.
- The Go compatibility contract covers parsing, generation order, probability
  handling and seeded output. See the parity tests in
  [../tests/go_parity.rs](../tests/go_parity.rs).
