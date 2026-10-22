//! Submit generation jobs to the worker pool.
//!
//! ```text
//! cargo run --example multithreaded
//! ```

use resrap_rs::{ResrapMT, templates};

fn main() -> Result<(), String> {
    let mut resrap = ResrapMT::new(4, 256);
    templates::load(&mut resrap, templates::Lang::Python)?;
    resrap.start_resrap();

    let channel = resrap.get_code_channel();
    for id in 0..8u64 {
        resrap.generate_with_seeded(&format!("job-{id}"), "python", "program", id + 1, 80)?;
    }

    for _ in 0..8 {
        let result = channel.recv().map_err(|err| err.to_string())?;
        println!("--- {} ---\n{}\n", result.id, result.code);
    }

    resrap.shutdown_resrap();
    Ok(())
}
