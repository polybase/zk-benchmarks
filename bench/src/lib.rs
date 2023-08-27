// use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Debug;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize, Serialize)]
pub struct Benchmark<'a> {
    name: &'a str,
    timings: Vec<BenchmarkResult<'a>>,
}

impl<'a> Benchmark<'a> {
    pub fn new(name: &'a str) -> Self {
        Benchmark {
            name,
            timings: Vec::new(),
        }
    }

    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(&mut self, name: &'a str, func: F) {
        let mut run = BenchmarkRun::new(name);
        func(&mut run);
        self.timings.push(BenchmarkResult { name, run });
    }

    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P) -> T, T, P: Debug>(
        &mut self,
        name: &'a str,
        params: &[P],
        func: F,
    ) {
        for p in params {
            let mut run = BenchmarkRun::new(name);
            func(&mut run, p);
            self.timings.push(BenchmarkResult { name, run });
        }
    }

    pub fn output(&self) {
        let output = json!(self);
        println!("{}", output);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BenchmarkRun<'a> {
    pub name: &'a str,
    pub time: Duration,
    pub metrics: std::collections::HashMap<&'a str, usize>,
}

impl<'a> BenchmarkRun<'a> {
    fn new(name: &'a str) -> Self {
        BenchmarkRun {
            name,
            time: Duration::new(0, 0),
            metrics: std::collections::HashMap::new(),
        }
    }

    pub fn run<F: Fn()>(&mut self, func: F) {
        let start_time = Instant::now();
        func();
        let elapsed_time = start_time.elapsed();
        self.time = elapsed_time;
    }

    pub fn log(&mut self, metric: &'a str, value: usize) {
        self.metrics.insert(metric, value);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BenchmarkResult<'a> {
    pub name: &'a str,
    pub run: BenchmarkRun<'a>,
}

impl<'a> BenchmarkResult<'a> {
    pub fn new(name: &'a str, run: BenchmarkRun<'a>) -> Self {
        BenchmarkResult { name, run }
    }
}
