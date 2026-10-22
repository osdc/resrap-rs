//! Benchmarks for long outputs, where per-token overhead matters most.
//!
//! Run with:
//!
//! ```text
//! cargo bench --bench long_generation
//! ```

use std::hint::black_box;
use std::time::{Duration, Instant};

use resrap_rs::{Resrap, ResrapMT, templates};

fn measure<F>(iters: usize, mut run: F) -> (Duration, usize)
where
    F: FnMut() -> usize,
{
    let sample_bytes = run();
    let start = Instant::now();
    for _ in 0..iters {
        black_box(run());
    }
    (start.elapsed(), sample_bytes)
}

fn report(label: &str, tokens: usize, iters: usize, elapsed: Duration, bytes: usize) {
    let seconds = elapsed.as_secs_f64();
    let per_iter = seconds / iters as f64;
    let mib = bytes as f64 / (1024.0 * 1024.0);
    println!(
        "{label:<30} {tokens:>10} tok {iters:>4} iters {per_iter:>9.3} s/iter \
         {:>9.1} MiB/s {:>12.0} tok/s",
        mib / per_iter,
        tokens as f64 / per_iter
    );
}

fn main() {
    println!("\nresrap-rs long-generation benchmarks (release profile)\n");
    println!(
        "{:<30} {:>13} {:>9} {:>12} {:>12} {:>16}",
        "benchmark", "tokens", "iters", "time", "throughput", "token rate"
    );
    println!("{}", "-".repeat(100));

    let mut resrap = Resrap::new();
    templates::load(&mut resrap, "c").unwrap();

    for tokens in [10_000usize, 100_000, 1_000_000] {
        let iters = match tokens {
            0..=10_000 => 100,
            10_001..=100_000 => 20,
            _ => 5,
        };
        let (elapsed, bytes) = measure(iters, || {
            resrap.generate_with_seeded("c", "program", 7, tokens).len()
        });
        report("seeded", tokens, iters, elapsed, bytes);

        let (elapsed, bytes) = measure(iters, || {
            resrap
                .generate_complete_with_seeded("c", "program", 7, tokens)
                .len()
        });
        report("complete seeded", tokens, iters, elapsed, bytes);
    }

    // Long MT jobs: keep the total work similar across token counts while
    // ensuring every worker gets several jobs.
    for (tokens, jobs) in [(10_000usize, 128usize), (100_000, 32), (1_000_000, 8)] {
        let mut mt = ResrapMT::new(8, 4096);
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
                tokens,
            )
            .unwrap();
        }
        let mut bytes = 0usize;
        for _ in 0..jobs {
            bytes += channel.recv().unwrap().code.len();
        }
        let elapsed = start.elapsed();
        mt.shutdown_resrap();

        let seconds = elapsed.as_secs_f64();
        let total_tokens = tokens * jobs;
        println!(
            "{:<30} {:>13} {:>9} {:>10.3} s {:>12.1} MiB/s {:>12.0} tok/s",
            "MT pool=8",
            format!("{tokens} x{jobs}"),
            jobs,
            seconds,
            bytes as f64 / (1024.0 * 1024.0) / seconds,
            total_tokens as f64 / seconds
        );
    }

    println!();
}
