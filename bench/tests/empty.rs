#[test]
fn empty() {
    mod inner {
        benchy::main!();

        pub fn call_main() {
            main();
        }
    }

    inner::call_main();
}

#[test]
fn empty_with_name() {
    mod inner {
        benchy::main!("empty_with_name");

        pub fn call_main() {
            main();
        }
    }

    inner::call_main();
}
