use leo_compiler::Compiler;
use leo_errors::emitter::Handler;
use leo_package::root::Env;
use std::fs::{self, File};

use snarkvm::{
    circuit::AleoV0,
    file::Manifest,
    package::Package,
    prelude::{
        query::Query,
        store::{helpers::memory::BlockMemory, BlockStore},
        *,
    },
};

pub fn assert() -> impl FnOnce() -> Execution<Testnet3> {
    let run_and_prove = prepare(
        "asrt",
        "
    program asrt.aleo {
        transition main(a: field, b: field) {
            assert_neq(a, b);
        }
    }
    ",
        &["1field".to_string(), "2field".to_string()],
    );

    run_and_prove
}

pub fn fib(n: u32) -> impl FnOnce() -> Execution<Testnet3> {
    let run_and_prove = prepare(
        "fib",
        &format!(
            "
    program fib.aleo {{
        transition main() {{
            let a: field = 0field;
            let b: field = 1field;

            for i:u32 in 0u32..{n}u32 {{
                let c: field = a + b;
                a = b;
                b = c;
            }}
        }}
    }}
    ",
        ),
        &[],
    );

    run_and_prove
}

pub fn sha_3_256(n_bytes: u32) -> impl FnOnce() -> Execution<Testnet3> {
    let iterations = (n_bytes as f64 / (256. / 8.)).ceil(); // 32 bytes at a time
    let run_and_prove = prepare(
        "sha",
        &format!(
            "
    program sha.aleo {{
        struct u256 {{
            a: u128,
            b: u128,
        }}

        transition main() {{
            let value: u256 = u256 {{ a: 0u128, b: 0u128 }};
            
            for i: u32 in 0u32..{iterations}u32 {{
                SHA3_256::hash_to_i8(value);
            }}
        }}
    }}
    ",
        ),
        &[],
    );

    run_and_prove
}

pub fn pedersen_128(n_bytes: u32) -> impl FnOnce() -> Execution<Testnet3> {
    let iterations = (n_bytes as f64 / (64. / 8.)).ceil(); // 8 bytes at a time
    let run_and_prove = prepare(
        "pedersen",
        &format!(
            "
    program pedersen.aleo {{
        transition main() {{
            for i: u32 in 0u32..{iterations}u32 {{
                // Max value is 64 bits
                Pedersen128::hash_to_i8(0u64);
            }}
        }}
    }}
    ",
        ),
        &[],
    );

    run_and_prove
}

pub fn prepare(name: &str, code: &str, inputs: &[String]) -> impl FnOnce() -> Execution<Testnet3> {
    let package = leo_span::symbol::create_session_if_not_set_then(|_| {
        let bytecode = compile(name, code);

        let package = build_package(&format!("{name}.aleo"), &bytecode);

        package
    });

    let mut rng = TestRng::default();

    let block_store = BlockStore::<Testnet3, BlockMemory<_>>::open(None).unwrap();
    let process = package.get_process().unwrap();
    let pk = PrivateKey::new(&mut TestRng::default()).unwrap();
    let authorization = process
        .authorize::<AleoV0, _>(
            &pk,
            package.program_id(),
            Identifier::from_str("main").unwrap(),
            inputs.into_iter(),
            &mut rng,
        )
        .unwrap();

    move || {
        let (_response, mut trace) = process.execute::<AleoV0>(authorization).unwrap();

        trace.prepare(Query::from(block_store)).unwrap();
        let execution = trace
            .prove_execution::<AleoV0, _>("testing", &mut rng)
            .unwrap();

        execution
    }
}

fn build_package(program_name: &str, bytecode: &str) -> Package<Testnet3> {
    let directory = tempdir::TempDir::new("aleo").unwrap().into_path();

    let path = directory.join("main.aleo");
    let mut file = File::create(path).unwrap();
    file.write_all(bytecode.as_bytes()).unwrap();

    let _manifest_file = Manifest::create(
        &directory,
        &ProgramID::<Testnet3>::from_str(program_name).unwrap(),
    )
    .unwrap();

    Env::<Testnet3>::new()
        .unwrap()
        .write_to(&directory)
        .unwrap();
    if Env::<Testnet3>::exists_at(&directory) {
        println!(".env file created at {:?}", &directory);
    }

    let build_directory = directory.join("build");
    fs::create_dir_all(build_directory).unwrap();

    Package::<Testnet3>::open(&directory).unwrap()
}

/// Returns bytecode
fn compile(program_name: &str, code: &str) -> String {
    let tmpdir = tempdir::TempDir::new("aleo").unwrap();
    let main_file_path = tmpdir.path().join("main.aleo");

    File::create(main_file_path.clone())
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap();

    let handler = Handler::default();
    let mut compiler = Compiler::new(
        program_name.to_owned(),
        "testnet3".to_owned(),
        &handler,
        main_file_path,
        tmpdir.into_path().join("output"),
        None,
    );

    let (_, bytecode) = compiler.compile().unwrap();

    bytecode
}
