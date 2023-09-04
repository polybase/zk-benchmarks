use acvm::acir::native_types::WitnessMap;
use acvm::Backend;
use acvm::{acir::circuit::Circuit, compiler::AcirTransformationMap, CommonReferenceString};
use nargo::{
    artifacts::program::PreprocessedProgram,
    ops::{execute_circuit, preprocess_program},
    package::{Package, PackageType},
    prepare_package,
};
pub use noirc_abi::{input_parser::InputValue, Abi, InputMap};
use noirc_driver::{compile_main, CompileOptions};
use std::collections::BTreeMap;
use std::path::Path;

pub mod backends;

pub struct Proof<'a, B: Backend> {
    backend: &'a B,
    common_reference_string: Vec<u8>,
    circuit: Circuit,
    proving_key: Vec<u8>,
    abi: Abi,
}

impl<'a, B: Backend> Proof<'a, B> {
    pub fn new(
        backend: &'a B,
        name: &str,
        path: impl AsRef<Path> + std::fmt::Debug,
    ) -> Proof<'a, B> {
        let (mut context, crate_id) = prepare_package(&Package {
            name: name.parse().expect("name"),
            entry_path: path.as_ref().join("src/main.nr"),
            root_dir: path.as_ref().to_path_buf(),
            package_type: PackageType::Binary,
            dependencies: BTreeMap::new(),
        });

        let (compiled_program, _) =
            compile_main(&mut context, crate_id, &CompileOptions::default()).expect("compile main");

        let common_reference_string =
            generate_common_reference_string(backend, &compiled_program.circuit)
                .expect("common_reference_string");

        let (program, _) =
            preprocess_program(backend, false, &common_reference_string, compiled_program)
                .expect("unable to preprocess");

        // let common_reference_string = read_cached_common_reference_string(&path)
        //     .expect("common_reference_string should exist");
        // let program = read_program_from_file(name, &path).expect("could not read program");

        let PreprocessedProgram { abi, bytecode, .. } = program;

        let (optimized_circuit, _) = optimize_circuit(backend, bytecode.clone())
            .expect("Backend does not support an opcode that is in the IR");

        let (proving_key, _) = backend
            .preprocess(&common_reference_string, &optimized_circuit)
            .expect("preprocess failed");

        Proof {
            backend,
            common_reference_string,
            circuit: optimized_circuit,
            proving_key,
            abi,
        }
        // return_value,
    }

    pub fn run_and_prove(self, inputs_map: &InputMap) {
        let Proof {
            backend,
            common_reference_string,
            circuit,
            proving_key,
            abi,
        } = self;

        let initial_witness: WitnessMap = abi
            .encode(inputs_map, None)
            .expect("unable to encode inputs");

        let witness = execute_circuit(backend, circuit.clone(), initial_witness, true)
            .expect("solved witness");

        // Write public inputs into Verifier.toml
        // let public_abi = abi.clone().public_abi();
        // let (_, return_value) = public_abi
        //     .decode(&solved_witness)
        //     .expect("unable to decode public abi");

        backend
            .prove_with_pk(
                &common_reference_string,
                &circuit,
                witness,
                &proving_key,
                false,
            )
            .expect("proof to be generated");
    }
}

fn optimize_circuit<B: Backend>(
    backend: &B,
    circuit: Circuit,
) -> Result<(Circuit, AcirTransformationMap), acvm::compiler::CompileError> {
    let result = acvm::compiler::compile(circuit, backend.np_language(), |opcode| {
        backend.supports_opcode(opcode)
    })?;

    Ok(result)
}

fn generate_common_reference_string<B: CommonReferenceString>(
    backend: &B,
    circuit: &Circuit,
) -> Result<Vec<u8>, B::Error> {
    use tokio::runtime::Builder;

    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    let fut = backend.generate_common_reference_string(circuit);
    runtime.block_on(fut)
}
