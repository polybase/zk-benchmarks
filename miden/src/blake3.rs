use miden::{Assembler, ProofOptions};
use miden_processor::{MemAdviceProvider, StackInputs, VmStateIterator};

pub fn blake3(n_bytes: usize) -> (impl Fn(), VmStateIterator) {
    // Input: 32-bit value per element, 16 elements per hash. 8 elements will be spent on the previous hash.
    // Output: 32-byte digest stored in the first 8 elements of the stack.
    let blake3_ops = f64::ceil(n_bytes as f64 / 4. / 8.);
    let code = format!(
        r#"
        use.std::crypto::hashes::blake3

        begin
            push.1.2.3.4.5.6.7.8
            repeat.{blake3_ops}
                push.9.10.11.12.13.14.15.16
                exec.blake3::hash_2to1
            end
        end
    "#
    );

    let assembler = Assembler::default()
        .with_library(&miden_stdlib::StdLibrary::default())
        .unwrap();
    let program = assembler.compile(code).unwrap();
    let vm: miden_processor::VmStateIterator = miden_processor::execute_iter(
        &program,
        StackInputs::default(),
        MemAdviceProvider::default(),
    );

    (
        move || {
            let (_stack, _proof) = miden::prove(
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
