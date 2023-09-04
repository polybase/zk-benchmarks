use miden::{Assembler, ProofOptions};
use miden_processor::{AdviceInputs, MemAdviceProvider, StackInputs, VmStateIterator};

pub fn fib(n: u32) -> (impl Fn(), VmStateIterator) {
    let code = format!(
        r#"
        begin
            push.0
            push.1
            repeat.{n}
                swap dup.1 add
            end
        end
    "#
    );

    let assembler = Assembler::default()
        .with_library(&miden_stdlib::StdLibrary::default())
        .unwrap();
    let program = assembler.compile(code).unwrap();
    let advice_provider = MemAdviceProvider::from(
        AdviceInputs::default()
            .with_stack_values(vec![n as u64])
            .unwrap(),
    );
    let vm: miden_processor::VmStateIterator =
        miden_processor::execute_iter(&program, StackInputs::default(), advice_provider.clone());

    (
        move || {
            let (_stack, _proof) = miden::prove(
                &program,
                StackInputs::default(),
                advice_provider.clone(),
                ProofOptions::default(),
            )
            .unwrap();
        },
        vm,
    )
}
