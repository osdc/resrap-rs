//! Dependency-free benchmarks for parsing, generation and the worker pool.
//!
//! Run with:
//!
//! ```text
//! cargo bench --bench generation
//! ```
//!
//! `cargo bench` builds in release mode, so the numbers reflect optimised code.

use std::hint::black_box;
use std::time::{Duration, Instant};

use resrap_rs::{Resrap, ResrapMT, templates};

fn measure<F>(iters: usize, mut run: F) -> Duration
where
    F: FnMut(),
{
    // Warm up caches and the allocator before timing.
    for _ in 0..iters.min(100) {
        run();
    }
    let start = Instant::now();
    for _ in 0..iters {
        run();
    }
    start.elapsed()
}

fn line(label: &str, iters: usize, elapsed: Duration) {
    let per_iter = elapsed.as_secs_f64() / iters as f64;
    println!(
        "{label:<34} {iters:>8} iters  {:>10.3} ms/iter  {:>12.0} ops/s",
        per_iter * 1e3,
        1.0 / per_iter
    );
}

fn main() {
    println!("\nresrap-rs benchmarks (release profile)\n");
    println!(
        "{:<34} {:>8}        {:>10}         {:>12}",
        "benchmark", "iters", "time", "throughput"
    );
    println!("{}", "-".repeat(82));

    let grammar = templates::must_get("c").grammar;

    let iters = 300;
    let elapsed = measure(iters, || {
        let mut resrap = Resrap::new();
        resrap.parse_grammar("c", black_box(grammar)).unwrap();
        black_box(&resrap);
    });
    line("parse C template", iters, elapsed);

    let mut resrap = Resrap::new();
    templates::load(&mut resrap, "c").unwrap();
    let iters = 200_000;
    let elapsed = measure(iters, || {
        black_box(resrap.generate_with_seeded("c", "program", 7, 200));
    });
    line("generate seeded, 200 tokens", iters, elapsed);

    let iters = 50_000;
    let elapsed = measure(iters, || {
        black_box(resrap.generate_complete_with_seeded("c", "program", 7, 500));
    });
    line("generate complete, 500 tokens", iters, elapsed);

    let iters = 100_000;
    let elapsed = measure(iters, || {
        black_box(resrap.generate_random("c", "program", 200));
    });
    line("generate random, 200 tokens", iters, elapsed);

    // Byte throughput for the seeded 200-token case.
    let sample = resrap.generate_with_seeded("c", "program", 7, 200);
    let bytes = sample.len() as f64;
    let elapsed = measure(20_000, || {
        black_box(resrap.generate_with_seeded("c", "program", 7, 200));
    });
    let mb_per_sec = bytes * 20_000.0 / elapsed.as_secs_f64() / (1024.0 * 1024.0);
    println!(
        "{:<34} {:>8}        {:>10.3} MB/s (avg {} B/output)",
        "generation byte throughput",
        20_000,
        mb_per_sec,
        sample.len()
    );

    let workers = 8;
    let jobs = 50_000;
    let mut mt = ResrapMT::new(workers, 4096);
    templates::load(&mut mt, "c").unwrap();
    mt.start_resrap();
    let channel = mt.get_code_channel();

    let start = Instant::now();
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
    for _ in 0..jobs {
        black_box(channel.recv().unwrap());
    }
    let elapsed = start.elapsed();
    mt.shutdown_resrap();
    line(&format!("MT pool={workers}, 200 tokens"), jobs, elapsed);

    let mut mt = ResrapMT::new(workers, 4096);
    templates::load(&mut mt, "c").unwrap();
    mt.start_resrap();
    let channel = mt.get_code_channel();
    let jobs = 10_000;
    let start = Instant::now();
    for i in 0..jobs {
        mt.generate_complete_with_seeded(
            &format!("complete-{i}"),
            "c",
            "program",
            (i as u64) % 97 + 1,
            500,
        )
        .unwrap();
    }
    for _ in 0..jobs {
        black_box(channel.recv().unwrap());
    }
    let elapsed = start.elapsed();
    mt.shutdown_resrap();
    line(&format!("MT pool={workers}, complete 500"), jobs, elapsed);

    println!();
}
