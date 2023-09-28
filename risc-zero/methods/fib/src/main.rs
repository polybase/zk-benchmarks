#![no_main]
#![no_std]

use core::hint::black_box;

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let n: u32 = env::read();
    // Prevent the compiler from optimizing away the computation.
    black_box(fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    let mut a = 0u32;
    let mut b = 1u32;
    if n <= 1 {
        return n;
    }
    let mut i = 2;
    while i <= n {
        if i + 10 <= n {
            let c = a + b;
            let d = b + c;
            let e = c + d;
            let f = d + e;
            let g = e + f;
            let h = f + g;
            let j = g + h;
            let k = h + j;
            let l = j + k;
            let m = k + l;
            a = l;
            b = m;
            i += 10;
        } else {
            let c = a + b;
            a = b;
            b = c;
            i += 1;
        }
    }

    b
}
