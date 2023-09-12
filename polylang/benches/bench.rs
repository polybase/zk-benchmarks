use bench::{Benchmark, BenchmarkRun};
use polylang_bench::compile;
use polylang_prover::RunOutput;

fn main() {
    #[allow(unused_variables)]
    let bench_name = "polylang-single-cpu";
    #[cfg(feature = "multi-cpu")]
    let bench_name = "polylang-multi-cpu";
    #[cfg(feature = "metal")]
    let bench_name = "polylang-metal";
    let mut bench = Benchmark::from_env(bench_name);

    bench.benchmark("assert", |b| {
        let run_and_prove = compile(
            r#"
            function main() {
                let x = 1;
                let y = 2;

                if (x + y != 3) {
                    error("x + y != 3");
                }
            }
        "#,
        );

        let output = b.run(|| run_and_prove());
        report(b, output);
    });

    bench.benchmark_with(
        "Fibonacci",
        &[("1", 1), ("10", 10), ("100", 100)],
        |b, p| {
            let run_and_prove = compile(&format!(
                r#"
                function main() {{
                    let a = 0;
                    let b = 1;

                    for (let i = 0; i < {p}; i++) {{
                        let c = a + b;
                        a = b;
                        b = c;
                    }}
                }}
            "#
            ));

            let output = b.run(|| run_and_prove());
            report(b, output);
        },
    );

    bench.benchmark_with(
        "SHA256",
        &[
            ("1 byte", 1),
            ("10 bytes", 10),
            ("100 bytes", 100),
            ("1000 bytes", 1000),
        ],
        |b, p| {
            let bytes_per_element = 4.;
            let arr_size = f64::ceil(*p as f64 / bytes_per_element) as usize;
            let run_and_prove = compile(&format!(
                r#"
                function main() {{
                    let arr = [{zeros}];
                    let _ = hashSHA256(arr);
                }}
            "#,
                zeros = (0..arr_size).map(|_| "0").collect::<Vec<_>>().join(", "),
            ));

            let output = b.run(|| run_and_prove());
            report(b, output);
        },
    );

    bench.benchmark_with(
        "Blake3",
        &[
            ("1 byte", 1),
            ("10 bytes", 10),
            ("100 bytes", 100),
            ("1000 bytes", 1000),
        ],
        |b, p| {
            let bytes_per_element = 4.;
            let arr_size = f64::ceil(*p as f64 / bytes_per_element) as usize;
            let run_and_prove = compile(&format!(
                r#"
                function main() {{
                    let arr = [{zeros}];
                    let _ = hashBlake3(arr);
                }}
            "#,
                zeros = (0..arr_size).map(|_| "0").collect::<Vec<_>>().join(", "),
            ));

            let output = b.run(|| run_and_prove());
            report(b, output);
        },
    );

    bench.benchmark_with(
        "RPO",
        &[
            ("1 byte", 1),
            ("10 bytes", 10),
            ("100 bytes", 100),
            ("1000 bytes", 1000),
            ("10000 bytes", 10000),
        ],
        |b, p| {
            let bytes_per_element = 4.;
            let arr_size = f64::ceil(*p as f64 / bytes_per_element) as usize;
            let run_and_prove = compile(&format!(
                r#"
                function main() {{
                    let arr = [{zeros}];
                    hashRPO(arr);
                }}
            "#,
                zeros = (0..arr_size).map(|_| "0").collect::<Vec<_>>().join(", "),
            ));

            let output = b.run(|| run_and_prove());
            report(b, output);
        },
    );

    bench.output();
}

fn report(b: &mut BenchmarkRun, (run_output, proof): (RunOutput, Vec<u8>)) {
    b.log("cycles", run_output.cycle_count as usize);
    b.log("proof_size_bytes", proof.len());

    let compressed_proof = zstd::encode_all(&proof[..], 21).unwrap();
    b.log("compressed_proof_size_bytes", compressed_proof.len());
}
