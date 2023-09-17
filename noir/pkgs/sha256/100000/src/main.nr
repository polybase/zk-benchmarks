use dep::std;

fn main(x: [u8; 100000]) -> pub [u8; 32] {
    std::hash::sha256(x)
}

#[test]
fn test_main() {
    let x = [0 as u8; 100000];
    let _ = main(x);
}
