attr: TokenStream [Group { delimiter: Bracket, stream: TokenStream [Group { delimiter: Parenthesis, stream: TokenStream [Literal { kind: Str, symbol: "test", suffix: None, span: #0 bytes(17..23) }, Punct { ch: ',', spacing: Alone, span: #0 bytes(23..24) }, Literal { kind: Integer, symbol: "5", suffix: None, span: #0 bytes(25..26) }], span: #0 bytes(16..27) }], span: #0 bytes(15..28) }]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct Benchmark_add;
#[allow(non_upper_case_globals)]
const _: () = {
    extern crate bench as _bench;
    #[automatically_derived]
    impl _bench::BenchmarkFn for Benchmark_add {
        type ParamType = u32;
        fn name() -> String {
            "add".to_owned()
        }
        fn params() -> Vec<(String, Self::ParamType)> {
            [("test", 5)].into_iter().map(|(name, value)| (name.into(), value)).collect()
        }
    }
};
fn add(b: &mut bench::BenchmarkRun, p: u32) {
    let result = b.run(|| 1 + 1);
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
