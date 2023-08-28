use miden::{Assembler, ProofOptions};
use miden_processor::{MemAdviceProvider, StackInputs, VmStateIterator};

pub fn rpo(n_bytes: usize) -> (impl Fn(), VmStateIterator) {
    // We can pack 7 bytes into each field element and hash 4 field elements at a time.
    let hmerges = f64::floor(n_bytes as f64 / 4. / f64::floor(63. / 8.));
    let code = format!(
        r#"
        begin
            push.0.0.0.0
            repeat.{hmerges}
                push.1.1.1.1
                hmerge
            end
        end
    "#
    );

    let assembler = Assembler::default();
    let program = assembler.compile(code).unwrap();
    let vm: miden_processor::VmStateIterator = miden_processor::execute_iter(
        &program,
        StackInputs::default(),
        MemAdviceProvider::default(),
    );

    (
        move || {
            let (_stack, _proof) = miden_prover::prove(
                &program,
                StackInputs::default(),
                MemAdviceProvider::default(),
                ProofOptions::default(),
            )
            .unwrap();
        },
        vm,
    )
}
