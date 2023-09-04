#![no_main]
#![no_std]

use core::hint::black_box;

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

enum N {
    N1,
    N10,
    N100,
    N1000,
    N10000,
    N100000,
}

macro_rules! fib {
    ($n:ident) => {
        match $n {
            N::N1 => fib!(1),
            N::N10 => fib!(10),
            N::N100 => fib!(100),
            N::N1000 => fib!(1000),
            N::N10000 => fib!(10000),
            N::N100000 => fib!(100000),
        }
    };
    ($n:literal) => {{
        let mut a = black_box(0);
        let mut b = black_box(1);
        for _ in 0..$n {
            let c = a + b;
            a = b;
            b = c;
        }

        b
    }};
}

fn fib(n: N) {
    // Prevent the compiler from optimizing away the computation.
    black_box(fib!(n));
}

pub fn main() {
    let n: u32 = env::read();
    let n = match n {
        1 => N::N1,
        10 => N::N10,
        100 => N::N100,
        1000 => N::N1000,
        10000 => N::N10000,
        100000 => N::N100000,
        _ => panic!("invalid input"),
    };

    fib(n);
}
