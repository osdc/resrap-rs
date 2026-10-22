//! Report heap/RSS usage for grammars, generated output and the worker pool.
//!
//! This example reads `/proc/self/status`, so it is intended for Linux. Run it
//! in release mode for realistic allocator behaviour:
//!
//! ```text
//! cargo run --release --example memory
//! ```

use std::fs;

use resrap_rs::{Resrap, ResrapMT, templates};

fn status(field: &str) -> u64 {
    let Ok(text) = fs::read_to_string("/proc/self/status") else {
        return 0;
    };
    for line in text.lines() {
        if let Some(value) = line.strip_prefix(field) {
            return value
                .trim()
                .trim_end_matches("kB")
                .trim()
                .parse()
                .unwrap_or(0);
        }
    }
    0
}

fn report(label: &str) {
    println!(
        "{label:<42} RSS {:>8} KiB   peak {:>8} KiB",
        status("VmRSS:"),
        status("VmHWM:")
    );
}

fn main() {
    report("baseline");

    let mut resrap = Resrap::new();
    for lang in templates::Lang::ALL {
        templates::load(&mut resrap, lang).unwrap();
    }
    report("all 10 templates loaded");

    let outputs: Vec<String> = (0..10_000)
        .map(|i| resrap.generate_with_seeded("c", "program", i % 97 + 1, 200))
        .collect();
    let output_bytes: usize = outputs.iter().map(String::len).sum();
    report("10,000 outputs retained (C, 200 tokens)");
    println!(
        "  retained output: {:.1} MiB, average {:.0} B/output",
        output_bytes as f64 / (1024.0 * 1024.0),
        output_bytes as f64 / outputs.len() as f64
    );

    drop(outputs);
    report("outputs dropped");

    let mut mt = ResrapMT::new(8, 4096);
    templates::load(&mut mt, "c").unwrap();
    mt.start_resrap();
    let channel = mt.get_code_channel();

    let jobs = 10_000;
    for i in 0..jobs {
        mt.generate_with_seeded(
            &format!("job-{i}"),
            "c",
            "program",
            (i as u64) % 97 + 1,
            200,
        )
        .unwrap();
    }
    report("MT results buffered, consumer idle");

    let mut received = 0usize;
    for _ in 0..jobs {
        received += channel.recv().unwrap().code.len();
    }
    report("MT results drained");
    println!(
        "  received output: {:.1} MiB",
        received as f64 / (1024.0 * 1024.0)
    );

    drop(channel);
    mt.shutdown_resrap();
    drop(mt);
    report("MT instance dropped");
    // Run the long-output case last: large allocations can change glibc's
    // mmap threshold and make later RSS figures depend on allocator history.
    let long = resrap.generate_with_seeded("c", "program", 7, 1_000_000);
    report("one 1M-token output retained");
    println!(
        "  retained output: {:.1} MiB",
        long.len() as f64 / (1024.0 * 1024.0)
    );
    drop(long);
    report("1M-token output dropped");
}
