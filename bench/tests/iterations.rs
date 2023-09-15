use std::{
    io::{Read, Write},
    time::Duration,
};

use benchy::{Benchmark, BenchmarkConfig, BenchmarkParameter};

enum BenchmarkType {
    Benchmark {
        iterations: Option<usize>,
    },
    BenchmarkWith {
        params: Vec<BenchmarkParameter<usize>>,
    },
}

const MAX_BENCH_DURATION: Duration = Duration::from_millis(100);

fn iterations(bench_type: BenchmarkType, f: impl Fn()) -> usize {
    let mut benchmark = Benchmark::with_config(
        "iterations",
        BenchmarkConfig {
            max_default_iterations_duration: MAX_BENCH_DURATION,
            ..Default::default()
        },
    );

    let (mut reader, writer) = std::os::unix::net::UnixStream::pair().unwrap();

    let counter_handle = std::thread::spawn(move || {
        let mut count = 0;

        let mut buf = [0; 1];
        while reader.read(&mut buf).unwrap() > 0 {
            count += 1;
        }

        count
    });

    let writer = std::sync::Mutex::new(writer);
    match bench_type {
        BenchmarkType::Benchmark { iterations } => {
            benchmark.benchmark("assert", iterations, |_b| {
                writer.lock().unwrap().write_all(&[1]).unwrap();
                f();
            });
        }
        BenchmarkType::BenchmarkWith { params } => {
            benchmark.benchmark_with("assert", params, |_b, _i| {
                writer.lock().unwrap().write_all(&[1]).unwrap();
                f();
            });
        }
    }
    drop(writer);

    counter_handle.join().unwrap()
}

#[test]
fn test_iterations_benchmark_with() {
    assert_eq!(
        iterations(
            BenchmarkType::BenchmarkWith {
                params: vec![BenchmarkParameter::new(String::new(), 0, None)]
            },
            || {}
        ),
        10
    );
}

#[test]
fn test_iterations_benchmark() {
    assert_eq!(
        iterations(BenchmarkType::Benchmark { iterations: None }, || {}),
        10
    );
}

#[test]
fn test_iterations_benchmark_5() {
    assert_eq!(
        iterations(
            BenchmarkType::Benchmark {
                iterations: Some(5)
            },
            || {}
        ),
        5
    );
}

#[test]
fn test_iterations_benchmark_with_5() {
    assert_eq!(
        iterations(
            BenchmarkType::BenchmarkWith {
                params: vec![BenchmarkParameter::new(String::new(), 0, Some(5))]
            },
            || {}
        ),
        5
    );
}

#[test]
fn test_iterations_sleep_10() {
    assert_eq!(
        iterations(BenchmarkType::Benchmark { iterations: None }, || {
            std::thread::sleep(MAX_BENCH_DURATION)
        }),
        1
    );
}

#[test]
fn test_iterations_sleep_5() {
    assert_eq!(
        iterations(BenchmarkType::Benchmark { iterations: None }, || {
            std::thread::sleep(MAX_BENCH_DURATION / 2)
        }),
        2
    );
}
