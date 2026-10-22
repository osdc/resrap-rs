//! Generate C code with the built-in template.
//!
//! ```text
//! cargo run --example generate
//! ```

use resrap_rs::{Resrap, templates};

fn main() -> Result<(), String> {
    let mut resrap = Resrap::new();
    templates::load(&mut resrap, templates::Lang::C)?;

    // A fixed seed always produces the same output.
    let seeded = resrap.generate_with_seeded("c", "program", 7, 200);
    println!("--- seeded (200 tokens) ---\n{seeded}");

    // Complete generation finishes the starting rule instead of cutting a
    // statement in half.
    let complete = resrap.generate_complete_with_seeded("c", "program", 7, 500);
    println!("--- complete (500 tokens) ---\n{complete}");

    Ok(())
}
