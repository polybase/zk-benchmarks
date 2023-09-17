use benchy::benchmark;

#[benchmark([
    ("test", 5),
    param_from_fn(),
])]
fn add(b: &mut benchy::BenchmarkRun, p: u32) {
    let result = b.run(|| p + 1);
    assert_eq!(result, 6);
}

fn param_from_fn() -> (&'static str, u32) {
    ("param from fn", 5)
}

mod adder {
    use benchy::benchmark;

    #[benchmark(params_from_fn())]
    pub fn add(b: &mut benchy::BenchmarkRun, p: u32) {
        let result = b.run(|| p + 1);
        assert_eq!(result, 6);
    }

    fn params_from_fn() -> [(&'static str, u32); 1] {
        [("inside adder", 5)]
    }
}

#[benchmark]
fn assert(b: &mut benchy::BenchmarkRun) {
    b.run(|| assert_eq!(1, 1));
}

#[benchmark("assert2")]
fn assert_renamed(b: &mut benchy::BenchmarkRun) {
    b.run(|| assert_eq!(1, 1));
}

// benchy::main!("adders", add, adder::add);
benchy::main!(add, adder::add, assert, assert_renamed);
