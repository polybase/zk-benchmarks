use dep::std::option::Option;
use dep::std;

fn main(leaves: [Hash; 1024], root_hash: pub Hash, node: Hash) -> pub Hash {
  let tree = parse_tree(leaves);
  assert(tree.hash == root_hash);

  let success = tree.insert(node);

  assert(success);

  tree.hash
}

type Hash = [u8; 32];
type Leaf = Hash;

global NULL: Hash = [0; 32];

struct Tree1 {
  left: Leaf,
  right: Leaf,
  hash: Hash,
}

struct Tree2 {
  left: Tree1,
  right: Tree1,
  hash: Hash,
}

struct Tree3 {
  left: Tree2,
  right: Tree2,
  hash: Hash,
}

struct Tree4 {
  left: Tree3,
  right: Tree3,
  hash: Hash,
}

struct Tree5 {
  left: Tree4,
  right: Tree4,
  hash: Hash,
}

struct Tree6 {
  left: Tree5,
  right: Tree5,
  hash: Hash,
}

struct Tree7 {
  left: Tree6,
  right: Tree6,
  hash: Hash,
}

struct Tree8 {
  left: Tree7,
  right: Tree7,
  hash: Hash,
}

struct Tree9 {
  left: Tree8,
  right: Tree8,
  hash: Hash,
}

struct Tree10 {
  left: Tree9,
  right: Tree9,
  hash: Hash,
}

impl Tree10 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree9 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree8 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree7 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree6 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree5 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree4 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}


impl Tree3 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree2 {
  fn insert(mut self, hash: Hash) -> bool {
    let success = self.left.insert(hash);
    if success {
      true
    } else {
      self.right.insert(hash)
    }
  }
}

impl Tree1 {
  fn insert(mut self, hash: Hash) -> bool {
    if self.left == NULL {
      self.left = hash;
      true
    } else if self.right == NULL {
      self.right = hash;
      true
    } else {
      false
    }
  } 
}



fn merge_hash(a: Hash, b: Hash) -> Hash {
  let mut buf = [0; 64];

  for i in 0..32 {
    buf[i] = a[i];
    buf[i + 32] = b[i];
  }

  std::hash::sha256(buf)
}

fn parse_tree(leaves: [Hash; 1024]) -> Tree10 {

  // tree1s

  let mut tree1s = [Option::none(); 512];

  for i in 0..512 {
    let left = leaves[i * 2];
    let right = leaves[( i * 2 ) + 1];
    let hash = merge_hash(left, right);
     
    tree1s[i] = Option::some(Tree1 { left, right, hash });
  }

  // tree2s

  let mut tree2s = [Option::none(); 256];

  for i in 0..256 {
    let left = tree1s[i * 2].unwrap();
    let right = tree1s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree2s[i] = Option::some(Tree2 { left, right, hash });
  }

  // tree3s

  let mut tree3s = [Option::none(); 128];

  for i in 0..128 {
    let left = tree2s[i * 2].unwrap();
    let right = tree2s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree3s[i] = Option::some(Tree3 { left, right, hash });
  }

  // tree4s

  let mut tree4s = [Option::none(); 64];

  for i in 0..64 {
    let left = tree3s[i * 2].unwrap();
    let right = tree3s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree4s[i] = Option::some(Tree4 { left, right, hash });
  }

  // tree5s

  let mut tree5s = [Option::none(); 32];

  for i in 0..32 {
    let left = tree4s[i * 2].unwrap();
    let right = tree4s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree5s[i] = Option::some(Tree5 { left, right, hash });
  }

  // tree6s

  let mut tree6s = [Option::none(); 16];

  for i in 0..16 {
    let left = tree5s[i * 2].unwrap();
    let right = tree5s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree6s[i] = Option::some(Tree6 { left, right, hash });
  }

  // tree7s

  let mut tree7s = [Option::none(); 8];

  for i in 0..8 {
    let left = tree6s[i * 2].unwrap();
    let right = tree6s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree7s[i] = Option::some(Tree7 { left, right, hash });
  }

  // tree8s

  let mut tree8s = [Option::none(); 4];

  for i in 0..4 {
    let left = tree7s[i * 2].unwrap();
    let right = tree7s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree8s[i] = Option::some(Tree8 { left, right, hash });
  }

  // tree9s

  let mut tree9s = [Option::none(); 2];

  for i in 0..2 {
    let left = tree8s[i * 2].unwrap();
    let right = tree8s[( i * 2 ) + 1].unwrap();
    let hash = merge_hash(left.hash, right.hash);
     
    tree9s[i] = Option::some(Tree9 { left, right, hash });
  }

  let left = tree9s[0].unwrap();
  let right = tree9s[1].unwrap();
  let hash = merge_hash(left.hash, right.hash);
    
  Tree10 { left, right, hash }
}







