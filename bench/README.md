# Benchy

Benchy is a Rust crate for benchmarking long-running tasks.
Unlike other benchmarking libraries such as Criterion, which are optimized for high-frequency, nanosecond-level performance, Benchy is designed for tasks that take a significant amount of time to execute.
It provides a flexible and customizable environment, allowing you to set custom iteration counts and even measure memory usage metrics.

## Features

- **Custom Iteration Counts**: Run your benchmarks as many or as few times as you need.
- **Memory Usage Metrics**: Get insights into how much memory your code is using.
- **Environment Variable Configuration**: Customize your benchmarks on the fly using environment variables.
- **JSON Output**: Easily export your benchmark results to JSON for further analysis.


## Installation

```sh
cargo add benchy
```

## Quick Start

`benches/bench.rs`:

```rust
use benchy::{benchmark, BenchmarkRun};

#[benchmark]
fn fibonacci_single(run: &mut BenchmarkRun) {
    let mut x = 0;
    let mut y = 1;
    run.run(|| {
        for _ in 0..1_000_000 {
            let temp = x;
            x = y;
            y = temp + y;
        }
    });
}

#[benchmark("Fibonacci", [("1 million iterations", 1_000_000), ("2 million iterations", 2_000_000)])]
fn fibonacci_parametrized(run: &mut BenchmarkRun, iterations: usize) {
    let mut x = 0;
    let mut y = 1;
    run.run(|| {
        for _ in 0..iterations {
            let temp = x;
            x = y;
            y = temp + y;
        }
    });
}

benchy::main!(fibonacci_single, fibonacci_parametrized);
```

`Cargo.toml`:

```toml
[[bench]]
name = "bench"
harness = false
```

For more advanced usage, check the [zk-bench](https://github.com/polybase/zk-benchmarks) repository that utilizes this crate, or refer to the [documentation](https://docs.rs/benchy).

## Environment variables

- `BENCHY_QUICK` (default: false) - if true, runs only the first parameter of each benchmark.
- `BENCHY_OUTPUT_DIR` (default: None) - directory to output the JSON benchmark results to.
- `BENCHY_MAX_DEFAULT_ITERATIONS_DURATION` (default: 10s) - the maximum total duration for the default (10) iterations of a single benchmark.
