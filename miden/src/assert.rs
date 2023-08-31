use miden::{Assembler, ProofOptions};
use miden_processor::{AdviceInputs, MemAdviceProvider, StackInputs, VmStateIterator};

pub fn assert(a: u32, b: u32) -> (impl Fn(), VmStateIterator) {
    let code = format!(
        r#"
        begin
            adv_push.2
            u32checked_neq
            assert
        end
    "#
    );

    let assembler = Assembler::default()
        .with_library(&miden_stdlib::StdLibrary::default())
        .unwrap();
    let program = assembler.compile(code).unwrap();
    let advice_provider = MemAdviceProvider::from(
        AdviceInputs::default()
            .with_stack_values(vec![a as u64, b as u64])
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
