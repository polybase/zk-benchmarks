use miden::{Assembler, ProofOptions};
use miden_processor::{MemAdviceProvider, StackInputs};

fn main() {
    rpo(10000);
    rpo(100000);
    rpo(1000000);
}

fn rpo(n_bytes: usize) {
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
    let program = assembler.compile(&code).unwrap();
    let vm = miden_processor::execute_iter(
        &program,
        StackInputs::default(),
        MemAdviceProvider::default(),
    );

    let last_vm_state = vm.last().unwrap().unwrap();

    let start = std::time::Instant::now();
    let (_stack, _proof) = miden::prove(
        &program,
        StackInputs::default(),
        MemAdviceProvider::default(),
        ProofOptions::default(),
    )
    .unwrap();
    let end = std::time::Instant::now();

    println!(
        "RPO of {n_bytes} bytes took {:?} for {} cycles",
        end - start,
        last_vm_state.clk
    );
}
