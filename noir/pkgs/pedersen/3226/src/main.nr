
use dep::std;

fn main(x: [Field; 3226]) -> pub [Field; 2] {
    std::hash::pedersen(x)
}
