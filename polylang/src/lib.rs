use std::collections::HashMap;

use polylang_prover::RunOutput;

pub fn compile(code: &str) -> impl FnOnce() -> (RunOutput, Vec<u8>) {
    let program = polylang::parse_program(code).unwrap();
    let (miden_code, mut abi) = polylang::compiler::compile(program, None, "main").unwrap();
    let program = polylang_prover::compile_program(&abi, &miden_code).unwrap();

    abi.this_type = Some(abi::Type::Struct(abi::Struct {
        name: "Empty".to_string(),
        fields: Vec::new(),
    }));

    move || {
        let (output, prove) = polylang_prover::run(
            &program,
            &polylang_prover::Inputs::new(
                abi.clone(),
                None,
                vec![],
                abi.this_type
                    .map(|t| t.default_value())
                    .unwrap()
                    .try_into()
                    .unwrap(),
                vec![],
                HashMap::new(),
            )
            .unwrap(),
        )
        .unwrap();

        let proof = prove().unwrap();

        (output, proof.to_bytes())
    }
}
