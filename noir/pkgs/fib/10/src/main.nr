fn main(a_start: Field, b_start: Field) -> pub Field {
    let mut a = a_start;
    let mut b = b_start;
    for _ in 0..10 {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}
