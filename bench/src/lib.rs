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

    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(
        &mut self,
        name: &str,
        iterations: Option<usize>,
        func: F,
    ) {
        let group = self.group(name);
        group.benchmark(name, iterations, func);
    }

    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P), P: Debug>(
        &mut self,
        name: &str,
        params: impl IntoIterator<Item = impl Into<BenchmarkParameter<P>>>,
        func: F,
    ) {
        let group = self.group(name);
        group.benchmark_with(params, func);
    }

    pub fn output(&self) {
        let output = json!(self);
        let output_str = serde_json::to_string_pretty(&output).expect("failed to serialize");
        if let Some(path) = &self.config.output_dir {
            let path = std::path::Path::new(path);
            std::fs::create_dir_all(path).expect("failed to create output dir");
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

    pub fn benchmark<F: Fn(&mut BenchmarkRun)>(
        &mut self,
        name: &str,
        iterations: Option<usize>,
        func: F,
    ) {
        let mut runs = Vec::new();
        // time_start.elapsed() is different from sum(runs.time),
        // as run.time only reports the time of b.run(|| {}),
        // not the function's setup code.
        let time_start = Instant::now();
        // Run 10 times or until we have ran for 10 seconds
        for _times_ran in 1..=iterations.unwrap_or(10) {
            let run = fork(|| {
                let mut run = BenchmarkRun::new(name.to_owned());
                func(&mut run);
                run
            })
            .unwrap();

            runs.push(run);

            if iterations.is_none() && time_start.elapsed() > Duration::from_secs(10) {
                break;
            }
        }

        let avg_run = BenchmarkRun {
            name: name.to_owned(),
            time: runs.iter().map(|r| r.time).sum::<Duration>() / runs.len() as u32,
            metrics: {
                let mut metrics = HashMap::new();

                for run in runs {
                    for (metric, value) in run.metrics {
                        let metric = metrics.entry(metric).or_insert_with(Vec::new);
                        metric.push(value);
                    }
                }

                metrics
                    .into_iter()
                    .map(|(k, v)| (k, v.iter().sum::<usize>() / v.len()))
                    .collect()
            },
        };

        self.results.push(BenchmarkResult::Run(avg_run));
    }

    pub fn benchmark_with<F: Fn(&mut BenchmarkRun, &P), P: Debug>(
        &mut self,
        params: impl IntoIterator<Item = impl Into<BenchmarkParameter<P>>>,
        func: F,
    ) {
        let params = params
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<BenchmarkParameter<P>>>();

        let quick = self.config.quick;
        for p in params.iter().take(if quick { 1 } else { usize::MAX }) {
            self.benchmark(&p.name, p.iterations, |b| func(b, &p.value));
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
    pub time: Duration,
    pub metrics: HashMap<String, usize>,
}

impl BenchmarkRun {
    fn new(name: String) -> Self {
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

pub struct BenchmarkParameter<T> {
    name: String,
    value: T,
    iterations: Option<usize>,
}

impl<T> BenchmarkParameter<T> {
    pub fn new(name: String, value: T, iterations: Option<usize>) -> Self {
        BenchmarkParameter {
            name,
            value,
            iterations,
        }
    }
}

impl<T> From<(&str, T)> for BenchmarkParameter<T> {
    fn from((name, value): (&str, T)) -> BenchmarkParameter<T> {
        BenchmarkParameter {
            name: name.to_owned(),
            value,
            iterations: None,
        }
    }
}

impl<T> From<(usize, &str, T)> for BenchmarkParameter<T> {
    fn from((iterations, name, value): (usize, &str, T)) -> BenchmarkParameter<T> {
        BenchmarkParameter {
            name: name.to_owned(),
            value,
            iterations: Some(iterations),
        }
    }
}
