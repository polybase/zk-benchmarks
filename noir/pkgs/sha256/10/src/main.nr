use dep::std;

fn main(x: [u8; 10]) -> pub [u8; 32] {
    std::hash::sha256(x)
}

#[test]
fn test_main() {
    let x = [
        42, 235, 118, 29, 97, 206, 154, 167, 78, 231
    ];
    let _ = main(x);
}
