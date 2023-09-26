
use dep::std;

fn main(x: [Field; 323]) -> pub [Field; 2] {
    std::hash::pedersen(x)
}
