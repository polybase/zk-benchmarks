mod fork;
mod memory;

use fork::fork;
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
    timings: Vec<(String, BenchmarkResult)>,
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

    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(&mut self, name: &str, func: F) {
        let run = fork(|| {
            let stop_monitoring_memory = memory::monitor();

            let mut run = BenchmarkRun::new(name.to_owned(), String::new());
            func(&mut run);

            if let Some(memory_usage_bytes) = stop_monitoring_memory() {
                run.log("memory_usage_bytes", memory_usage_bytes);
            }

            run
        })
        .unwrap();

        self.timings.push((
            name.to_owned(),
            BenchmarkResult {
                name: name.to_owned(),
                run,
            },
        ));
    }

    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P) -> T, T, P: Debug>(
        &mut self,
        name: &str,
        params: &[(&str, P)],
        func: F,
    ) {
        for p in params
            .iter()
            .take(if self.config.quick { 1 } else { usize::MAX })
        {
            let run = fork(|| {
                let stop_monitoring_memory = memory::monitor();

                let mut run = BenchmarkRun::new(name.to_owned(), p.0.to_owned());
                func(&mut run, &p.1);

                if let Some(memory_usage_bytes) = stop_monitoring_memory() {
                    run.log("memory_usage_bytes", memory_usage_bytes);
                }
                run
            })
            .unwrap();

            self.timings.push((
                name.to_owned(),
                BenchmarkResult {
                    name: name.to_owned(),
                    run,
                },
            ));
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
pub struct BenchmarkResult {
    pub name: String,
    pub run: BenchmarkRun,
}

impl BenchmarkResult {
    pub fn new(name: String, run: BenchmarkRun) -> Self {
        BenchmarkResult { name, run }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BenchmarkRun {
    pub name: String,
    pub param: String,
    pub time: Duration,
    pub metrics: HashMap<String, usize>,
}

impl BenchmarkRun {
    fn new(name: String, param: String) -> Self {
        BenchmarkRun {
            name,
            param,
            time: Duration::new(0, 0),
            metrics: HashMap::new(),
        }
    }

    pub fn run<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start_time = Instant::now();
        let out: R = func();
        let elapsed_time = start_time.elapsed();
        self.time = elapsed_time;
        out
    }

    pub fn log(&mut self, metric: &str, value: usize) {
        self.metrics.insert(metric.to_owned(), value);
    }
}
