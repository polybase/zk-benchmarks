use dep::std;

type Hash = [u8; 32];

fn main(path: [Hash; 10], mut hash: Hash) -> pub Hash {
  for new_hash in path {
    let mut bytes = [0; 64];

    for i in 0..32 {
      bytes[i] = hash[i];
      bytes[i + 32] = new_hash[i];
    }

    hash = std::hash::sha256(bytes);
  } 

  hash
}

#[test]
fn main_test() {
  let path = [[0; 32]; 10];
  let hash = [0; 32];

  main(path, hash);
}

