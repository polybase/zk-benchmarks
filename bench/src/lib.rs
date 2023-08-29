// use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Benchmark<'a> {
    name: &'a str,
    config: BenchmarkConfig,
    timings: Vec<(&'a str, BenchmarkResult<'a>)>,
}

impl<'a> Benchmark<'a> {
    pub fn new(name: &'a str) -> Self {
        Benchmark {
            name,
            config: BenchmarkConfig::default(),
            timings: Vec::new(),
        }
    }

    pub fn with_config(name: &'a str, config: BenchmarkConfig) -> Self {
        Benchmark {
            name,
            config,
            timings: Vec::new(),
        }
    }

    pub fn from_env(name: &'a str) -> Self {
        Benchmark {
            name,
            config: BenchmarkConfig::from_env(),
            timings: Vec::new(),
        }
    }

    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(&mut self, name: &'a str, func: F) {
        let mut run = BenchmarkRun::new(name);
        func(&mut run);
        self.timings.push((name, BenchmarkResult { name, run }));
    }

    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P) -> T, T, P: Debug>(
        &mut self,
        name: &'a str,
        params: &[P],
        func: F,
    ) {
        for p in params
            .iter()
            .take(if self.config.quick { 1 } else { usize::MAX })
        {
            let mut run = BenchmarkRun::new(name);
            func(&mut run, p);
            self.timings.push((name, BenchmarkResult { name, run }));
        }
    }

    pub fn output(&self) {
        let output = json!({ "name": self.name, "timings": json!(self.timings) });
        let output_str = serde_json::to_string_pretty(&output).expect("failed to serialize");
        if let Some(path) = &self.config.output_dir {
            let path = std::path::Path::new(path);
            std::fs::write(path.join(self.name).with_extension("json"), &output_str)
                .expect("failed to write output");
        }
        println!("{}", &output_str);
    }
}

#[derive(Debug, Default)]
pub struct BenchmarkConfig {
    pub quick: bool,
    pub output_dir: Option<String>,
}

impl BenchmarkConfig {
    pub fn from_env() -> Self {
        let quick = env::var("BENCH_QUICK").unwrap_or("false".to_string());
        BenchmarkConfig {
            quick: quick == "true" || quick == "1",
            output_dir: env::var("BENCH_OUTPUT_DIR").ok(),
        }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BenchmarkRun<'a> {
    pub name: &'a str,
    pub time: Duration,
    pub metrics: HashMap<&'a str, usize>,
}

impl<'a> BenchmarkRun<'a> {
    fn new(name: &'a str) -> Self {
        BenchmarkRun {
            name,
            time: Duration::new(0, 0),
            metrics: HashMap::new(),
        }
    }

    pub fn run<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start_time = Instant::now();
        let out = func();
        let elapsed_time = start_time.elapsed();
        self.time = elapsed_time;
        out
    }

    pub fn log(&mut self, metric: &'a str, value: usize) {
        self.metrics.insert(metric, value);
    }
}
