use dep::std;

type Hash = [u8; 32];

fn main(tree1: [Hash; 1024], tree2: [Hash; 1024]) -> pub Hash {
    root_hash_of_merged(tree1, tree2)
}


fn root_hash_of_merged(tree1: [Hash; 1024], tree2: [Hash; 1024]) -> Hash {
  let mut result = [[0; 32]; 2048];
  for i in 0..1024 {
    result[i] = tree1[i];
    result[1024 + i] = tree2[i];
  }
  let mut len = 2048;

  for _ in 0..10 {
    for i in 0..(len / 2) {
      let index = i * 2;
      let mut bytes = [0; 64];
      for i in 0..32 {
        bytes[i] = result[index][i];
        bytes[i + 32] = result[index + 1][i];
      }
      let new_hash = std::hash::sha256(bytes);
      result[i] = new_hash;
    }

    len /= 2;
  }

   result[0]
}



#[test]
fn test_main() {
    let tree = [[0; 32]; 1024];
    root_hash_of_merged(tree, tree);
}
