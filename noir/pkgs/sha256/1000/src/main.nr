use dep::std;

fn main(x: [u8; 1000]) -> pub [u8; 32] {
    std::hash::sha256(x)
}

#[test]
fn test_main() {
    let x = [0 as u8; 1000];
    let _ = main(x);
}
