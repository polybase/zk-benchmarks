
use dep::std;

fn main(x: [Field; 33]) -> pub [Field; 2] {
    std::hash::pedersen(x)
}
