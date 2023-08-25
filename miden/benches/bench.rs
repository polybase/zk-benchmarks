extern crate miden_bench;

use miden_bench::rpo::rpo;
use std::time::{Duration, Instant};

fn main() {
    let mut bench = Benchmark::new();

    bench.benchmark_with("RPO", &[10000, 100000, 1000000], |b, p| {
        let (setup, vm) = rpo(*p);
        let last_vm_state = vm.last().unwrap().unwrap();
        b.run(setup);
        b.log("cycles", last_vm_state.clk as usize);
    });

    bench.output();
}

pub struct Benchmark<'a> {
    timings: Vec<BenchmarkResult<'a>>,
}

impl<'a> Benchmark<'a> {
    pub fn new() -> Self {
        Benchmark {
            timings: Vec::new(),
        }
    }

    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(&mut self, name: &'a str, func: F) {
        let mut run = BenchmarkRun::new();
        func(&mut run);
        self.timings.push(BenchmarkResult { name, run });
    }

    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P) -> T, T, P>(
        &mut self,
        name: &'a str,
        params: &[P],
        func: F,
    ) {
        for p in params {
            let mut run = BenchmarkRun::new();
            func(&mut run, p);
            self.timings.push(BenchmarkResult { name, run });
        }
    }

    pub fn output(&self) {
        for res in &self.timings {
            println!("{}: {:?}", res.name, res.run.time);
            if !res.run.metrics.is_empty() {
                println!("  Metrics:");
                for (metric, value) in &res.run.metrics {
                    println!("    {}: {}", metric, value);
                }
            }
        }
    }
}

impl<'a> Default for Benchmark<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BenchmarkRun<'a> {
    pub time: Duration,
    pub metrics: std::collections::HashMap<&'a str, usize>,
}

impl<'a> BenchmarkRun<'a> {
    fn new() -> Self {
        BenchmarkRun {
            time: Duration::new(0, 0),
            metrics: std::collections::HashMap::new(),
        }
    }

    fn run<F: Fn()>(&mut self, func: F) {
        let start_time = Instant::now();
        func();
        let elapsed_time = start_time.elapsed();
        self.time = elapsed_time;
    }

    fn log(&mut self, metric: &'a str, value: usize) {
        self.metrics.insert(metric, value);
    }
}

pub struct BenchmarkResult<'a> {
    pub name: &'a str,
    pub run: BenchmarkRun<'a>,
}

impl<'a> BenchmarkResult<'a> {
    pub fn new(name: &'a str, run: BenchmarkRun<'a>) -> Self {
        BenchmarkResult { name, run }
    }
}
