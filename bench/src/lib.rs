mod fork;
mod memory;

use fork::fork;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize)]
pub struct Benchmark<'a> {
    name: &'a str,
    results: Vec<BenchmarkGroup>,
    #[serde(skip)]
    config: BenchmarkConfig,
}

impl<'a> Benchmark<'a> {
    pub fn new(name: &'a str) -> Self {
        Benchmark {
            name,
            config: BenchmarkConfig::default(),
            results: Vec::new(),
        }
    }

    pub fn with_config(name: &'a str, config: BenchmarkConfig) -> Self {
        Benchmark {
            name,
            config,
            results: Vec::new(),
        }
    }

    pub fn from_env(name: &'a str) -> Self {
        Benchmark {
            name,
            config: BenchmarkConfig::from_env(),
            results: Vec::new(),
        }
    }

    pub fn group(&mut self, name: &str) -> &mut BenchmarkGroup {
        let group = BenchmarkGroup::new(name.to_string(), &self.config);
        self.results.push(group);
        // We can unwrap as we just added the item
        self.results.last_mut().unwrap()
    }

    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(&mut self, name: &str, func: F) {
        let group = self.group(name);
        group.benchmark(name, func);
    }

    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P) -> T, T, P: Debug>(
        &mut self,
        name: &str,
        params: &[(&str, P)],
        func: F,
    ) {
        let group = self.group(name);
        group.benchmark_with(name, params, func);
    }

    pub fn output(&self) {
        let output = json!(self);
        let output_str = serde_json::to_string_pretty(&output).expect("failed to serialize");
        if let Some(path) = &self.config.output_dir {
            let path = std::path::Path::new(path);
            std::fs::write(path.join(self.name).with_extension("json"), &output_str)
                .expect("failed to write output");
        }
        println!("{}", &output_str);
    }
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkGroup {
    name: String,
    results: Vec<BenchmarkResult>,
    #[serde(skip)]
    config: BenchmarkConfig,
}

impl BenchmarkGroup {
    pub fn new(name: String, config: &BenchmarkConfig) -> Self {
        BenchmarkGroup {
            name,
            results: Vec::new(),
            config: config.clone(),
        }
    }

    pub fn group(&mut self, name: &str) -> &mut BenchmarkGroup {
        let group = BenchmarkGroup::new(name.to_string(), &self.config);
        self.results.push(BenchmarkResult::Group(group));
        // We can unwrap as we just added the item
        match self.results.last_mut().unwrap() {
            BenchmarkResult::Group(ref mut group) => group,
            _ => unreachable!(),
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

        self.results.push(BenchmarkResult::Run(run));
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
                let mut run = BenchmarkRun::new(name.to_owned(), p.0.to_owned());
                func(&mut run, &p.1);
                run
            })
            .unwrap();

            self.results.push(BenchmarkResult::Run(run));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BenchmarkResult {
    Group(BenchmarkGroup),
    Run(BenchmarkRun),
}

#[derive(Debug, Serialize, Deserialize)]
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
        let stop_monitoring_memory = memory::monitor();
        let start_time = Instant::now();

        let out = func();

        let elapsed_time = start_time.elapsed();
        self.time = elapsed_time;

        if let Some(memory_usage_bytes) = stop_monitoring_memory() {
            self.log("memory_usage_bytes", memory_usage_bytes);
        }

        out
    }

    pub fn log(&mut self, metric: &str, value: usize) {
        self.metrics.insert(metric.to_owned(), value);
    }
}
