#[test]
fn empty() {
    mod inner {
        bench::main!();

        pub fn call_main() {
            main();
        }
    }

    inner::call_main();
}

#[test]
fn empty_with_name() {
    mod inner {
        bench::main!("empty_with_name");

        pub fn call_main() {
            main();
        }
    }

    inner::call_main();
}
