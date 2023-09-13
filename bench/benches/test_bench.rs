#[bench::bench([
    ("test", 5),
    param_from_fn(),
])]
fn add(b: &mut bench::BenchmarkRun, p: u32) {
    let result = b.run(|| p + 1);
    assert_eq!(result, 6);
}

fn param_from_fn() -> (&'static str, u32) {
    ("param from fn", 5)
}

mod adder {
    #[bench::bench(params_from_fn())]
    pub fn add(b: &mut bench::BenchmarkRun, p: u32) {
        let result = b.run(|| p + 1);
        assert_eq!(result, 6);
    }

    fn params_from_fn() -> [(&'static str, u32); 1] {
        [("inside adder", 5)]
    }
}

#[bench::bench]
fn assert(b: &mut bench::BenchmarkRun) {
    b.run(|| assert_eq!(1, 1));
}

#[bench::bench("assert2")]
fn assert_renamed(b: &mut bench::BenchmarkRun) {
    b.run(|| assert_eq!(1, 1));
}

// bench::main!("adders", add, adder::add);
bench::main!(add, adder::add, assert, assert_renamed);
