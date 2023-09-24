use dep::std;

type Hash = [u8; 32];
type Hash10 = [u8; 320];  // nested arrays cause compiler errors

fn main(path: Hash10, mut hash: Hash) -> pub Hash {
  for hash_index in 0..10 {
    
    let mut bytes = [0; 64];

    for i in 0..32 {
      bytes[i] = hash[i];
      bytes[i + 32] = path[(32 * hash_index) + i];
    }

    hash = std::hash::sha256(bytes);
  } 

  hash
}

#[test]
fn main_test() {
  let path = [0; 320];
  let hash = [0; 32];

  let _ = main(path, hash);
}

