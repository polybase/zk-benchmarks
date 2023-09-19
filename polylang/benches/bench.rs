use benchy::{benchmark, BenchmarkRun};
use polylang_bench::compile;
use polylang_prover::RunOutput;

#[benchmark]
fn assert(b: &mut BenchmarkRun) {
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

    let output = b.run(run_and_prove);
    report(b, output);
}

#[benchmark("Fibonacci", [
    ("1", 1),
    // ("10", 10), // Miden prover panics with "index out of bounds: the len is 259 but the index is 259"
    ("100", 100),
    ("1000", 1000),
    ("10000", 10000),
])]
fn fibonacci(b: &mut BenchmarkRun, p: usize) {
    let run_and_prove = compile(&format!(
        r#"
        function main() {{
            let p: u32 = {p};
            let a: u32 = 0;
            let b: u32 = 1;

            for (let i: u32 = 0; i < p; i++) {{
                let c = a.wrappingAdd(b);
                a = b;
                b = c;
            }}
        }}
    "#
    ));

    let output = b.run(run_and_prove);
    report(b, output);
}

#[benchmark("SHA256", [
    ("1k bytes", 1000),
    // 10k bytes needs more than 32GB of ram
    ("10k bytes", 10000)
])]
fn sha256(b: &mut BenchmarkRun, p: usize) {
    let bytes_per_element = 4.;
    let arr_size = f64::ceil(p as f64 / bytes_per_element) as usize;
    let run_and_prove = compile(&format!(
        r#"
        function main() {{
            let arr = [{zeros}];
            let _ = hashSHA256(arr);
        }}
    "#,
        zeros = (0..arr_size).map(|_| "0").collect::<Vec<_>>().join(", "),
    ));

    let output = b.run(run_and_prove);
    report(b, output);
}

#[benchmark("Blake3", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
])]
fn blake3(b: &mut BenchmarkRun, p: usize) {
    let bytes_per_element = 4.;
    let arr_size = f64::ceil(p as f64 / bytes_per_element) as usize;
    let run_and_prove = compile(&format!(
        r#"
        function main() {{
            let arr = [{zeros}];
            let _ = hashBlake3(arr);
        }}
    "#,
        zeros = (0..arr_size).map(|_| "0").collect::<Vec<_>>().join(", "),
    ));

    let output = b.run(run_and_prove);
    report(b, output);
}

#[benchmark("RPO", [
    ("1k bytes", 1000),
    ("10k bytes", 10000),
])]
fn rpo(b: &mut BenchmarkRun, p: usize) {
    let bytes_per_element = 4.;
    let arr_size = f64::ceil(p as f64 / bytes_per_element) as usize;
    let run_and_prove = compile(&format!(
        r#"
        function main() {{
            let arr = [{zeros}];
            hashRPO(arr);
        }}
    "#,
        zeros = (0..arr_size).map(|_| "0").collect::<Vec<_>>().join(", "),
    ));

    let output = b.run(run_and_prove);
    report(b, output);
}

fn report(b: &mut BenchmarkRun, (run_output, proof): (RunOutput, Vec<u8>)) {
    b.log("cycles", run_output.cycle_count as usize);
    b.log("proof_size_bytes", proof.len());

    let compressed_proof = zstd::encode_all(&proof[..], 21).unwrap();
    b.log("compressed_proof_size_bytes", compressed_proof.len());
}

benchy::main!("polylang", assert, fibonacci, sha256, blake3, rpo);
