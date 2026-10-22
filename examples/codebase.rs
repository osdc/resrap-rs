//! Describe and generate a small multi-file project.
//!
//! ```text
//! cargo run --example codebase -- /tmp/resrap-codebase
//! ```

use resrap_rs::{Resrap, templates};

const SPEC: &str = "\
src/
  c[2x120 hello_*.c]
  sql[1x60 schema_*.sql]
";

fn main() -> Result<(), String> {
    let target = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());

    let mut resrap = Resrap::new();
    templates::load(&mut resrap, templates::Lang::C)?;
    templates::load(&mut resrap, templates::Lang::Sql)?;

    // Parse the DSL (validates it) and write the files under `target`.
    let root = resrap_rs::parse_dsl(SPEC)?;
    root.generate_structure(&resrap, &target)?;
    println!("Generated project in {target}");
    Ok(())
}
