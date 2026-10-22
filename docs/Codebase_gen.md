# Codebase generation

Resrap can generate a whole directory tree of files from a small DSL. Each file
is produced by a loaded grammar using complete generation, so it normally ends
at a rule boundary instead of being cut mid-statement.

## Directory DSL

```dsl
src/
  code/
    c[10x200 code_*.c]
    core/
      c[5x1000 *.c]
  more/
    sql[10x100 *.sql]
```

The format is indentation-based:

- A line without brackets is a **directory**.
- A line with brackets is a **file specification**:
  `grammar[count x tokens pattern]`.
- Indentation appends the entry to the current directory. The root of the
  generated tree is the `target` passed to the API (not the top-level name in
  the DSL).

### File specification

```text
c[10x200 code_*.c]
```

| Part | Meaning |
| ---- | ------- |
| `c` | Grammar name, as loaded with `parse_grammar` or `templates::load` |
| `10` | Number of files |
| `200` | Token budget per file |
| `code_*.c` | File name pattern; every `*` is replaced by the 1-based file number |

This produces `code_1.c` through `code_10.c`. The grammar is always generated
from the `program` start rule. At least one `*` is not required, but patterns
without one would overwrite the same file, so use a placeholder.

The `x` between count and token budget is case-sensitive. Both numbers must be
positive integers. Invalid specs produce a descriptive line-numbered error.

## API

### `Resrap::generate_codebase(config_path, target) -> Result<(), String>`

Reads the DSL file, validates it, creates directories and writes the files.

```rust
use resrap_rs::{templates, Resrap};

fn main() -> Result<(), String> {
    let mut resrap = Resrap::new();

    // Load every grammar referenced by the DSL.
    templates::load(&mut resrap, templates::Lang::C)?;
    templates::load(&mut resrap, templates::Lang::Sql)?;

    resrap.generate_codebase("project.dsl", "output/")?;
    Ok(())
}
```

For each file, progress is printed to stdout:

```text
Generated: output/src/code/code_1.c (200 tokens)
```

### Parsing without writing

`parse_dsl` exposes the parsed tree:

```rust
use resrap_rs::{FileType, parse_dsl};

let root = parse_dsl("src/\n  c[2x100 hello_*.c]\n")?;
let src = &root.children[0];
assert_eq!(src.name, "src");
assert_eq!(src.typ, FileType::Dir);

let file = &src.children[0];
assert_eq!(file.name, "c");
assert_eq!(file.count, 2);
assert_eq!(file.token_count, 100);
assert_eq!(file.pattern, "hello_*.c");
assert_eq!(file.file_type, "c");
Ok::<(), String>(())
```

`Node::generate_structure(&resrap, target)` writes an already-parsed tree:

```rust
use resrap_rs::{templates, parse_dsl, Resrap};
fn demo() -> Result<(), String> {
let mut resrap = Resrap::new();
templates::load(&mut resrap, templates::Lang::C)?;
let root = parse_dsl("src/\n  c[2x100 hello_*.c]\n")?;
root.generate_structure(&resrap, "output/")?;
Ok(())
}
```

## Complete generation

Codebase files use `generate_complete`, which treats the token budget as a soft
limit and stops at the starting rule's end boundary. This avoids truncated
`if` blocks, incomplete functions and dangling syntax. If the requested grammar
is not loaded, the generated file is empty rather than an error.
