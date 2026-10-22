# Multithreaded API (`ResrapMT`)

`ResrapMT` is the worker-pool version of [`Resrap`](Resrap.md). It owns its
grammars, hands jobs to a fixed pool of threads and returns results over a
blocking channel.

Use it when many independent snippets are needed: fixtures, batch generation,
previews or server-side request handling.

## Construction

### `ResrapMT::new(pool_size, wait_queue_size)`

```rust
use resrap_rs::ResrapMT;

let mut resrap = ResrapMT::new(8, 4096);
```

- `pool_size` is the number of worker threads. Values below 1 are clamped to 1.
- `wait_queue_size` is the capacity of the pending-job queue. Values below 1
  are clamped to 1.

Workers are not started until `start_resrap()` is called.

## Loading grammars

`ResrapMT` implements the same grammar-loading API as `Resrap`:

```rust
use resrap_rs::{templates, ResrapMT};
fn demo() -> Result<(), String> {
let mut resrap = ResrapMT::new(8, 4096);
templates::load(&mut resrap, "c")?;
resrap.parse_grammar("Greeting", "program:'hello ' [a-z] '!';")?;
resrap.parse_grammar_file("C", "grammars/c.g4")?;
Ok(())
}
```

`parse_grammar`, `parse_grammar_file`, `add_words` and `add_keywords` have the
same semantics as the single-threaded API. Load all grammars before starting the
pool; grammar mutations are not intended to race with generation.

## Lifecycle

### `start_resrap()`

Spawns `pool_size` worker threads. Calling it more than once is a no-op.

### `shutdown_resrap()` / `shut_down_resrap()`

Gracefully stops the pool. Workers finish the job they are running, remaining
queued jobs are drained, the result channel is closed and the call joins every
thread. It is safe to call more than once. `shut_down_resrap` is an alias using
the Go method's capitalisation.

## Submitting work

Every job takes a caller-supplied `id`, a grammar name, a start rule and a token
budget. The ID is returned with the generated code and must be unique if you
need to map results back to requests.

| Method | Deterministic | Complete |
| ------ | ------------- | -------- |
| `generate_random(id, name, start, tokens)` | No | No |
| `generate_with_seeded(id, name, start, seed, tokens)` | Yes | No |
| `generate_complete(id, name, start, tokens)` | No | Yes |
| `generate_complete_with_seeded(id, name, start, seed, tokens)` | Yes | Yes |

Each returns `Result<(), String>` and reports `ERR_SHUT_DOWN` if the instance
has already been shut down.

```rust
use resrap_rs::{templates, ResrapMT};
fn demo() -> Result<(), String> {
let mut resrap = ResrapMT::new(4, 256);
templates::load(&mut resrap, "python")?;
resrap.start_resrap();

for id in 0..16u64 {
    resrap.generate_with_seeded(&format!("job-{id}"), "python", "program", id + 1, 120)?;
}
Ok(())
}
```

## Receiving results

`get_code_channel()` returns a cloneable `CodeChannel`. Each `CodeGenRes`
contains `code` and `id`.

```rust
use resrap_rs::{templates, ResrapMT};
fn demo() -> Result<(), String> {
let mut resrap = ResrapMT::new(4, 256);
templates::load(&mut resrap, "c")?;
resrap.start_resrap();
let channel = resrap.get_code_channel();
resrap.generate_with_seeded("job-0", "c", "program", 1, 100)?;
let result = channel.recv().map_err(|err| err.to_string())?;
println!("{}: {}", result.id, result.code);
resrap.shutdown_resrap();
Ok(())
}
```

`CodeChannel` supports the usual blocking and non-blocking operations:

- `recv()` blocks until a result is available.
- `try_recv()` returns immediately.
- `recv_timeout(duration)` waits up to a timeout.

The channel is backed by an **unbounded result buffer**, so a slow consumer does
not block workers. `recv()` returns an error once the instance is shut down and
every dispatched result has been drained.

## Performance notes

Grammar graphs are immutable after parsing. Workers share `Arc<RwLock<_>>`
access and each job creates its own PRNG, so generation is independent and
deterministic per seed. The pending-job queue provides backpressure; the result
buffer does not.

See the root [README](../README.md#benchmarks) for measured single-threaded and
worker-pool throughput.
