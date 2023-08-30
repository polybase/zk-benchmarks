use dep::std;

fn main(x: [u8; 1]) -> pub [u8; 32] {
    std::hash::sha256(x)
}

#[test]
fn test_main() {
    let x = [163];
    let _ = main(x);
}
