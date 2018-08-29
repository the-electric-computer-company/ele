use common::*;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq)]
pub struct Hash {
  pub bytes: [u8; 32],
}

impl Hash {
  pub fn new() -> Hash {
    Hash { bytes: random() }
  }

  pub fn from_array(bytes: [u8; 32]) -> Hash {
    Hash { bytes }
  }

  pub fn from_content(content: &[u8]) -> Hash {
    let digest = Sha256::digest(content);
    assert_eq!(digest.len(), 32);
    let mut bytes: [u8; 32] = [0; 32];
    bytes.copy_from_slice(digest.as_slice());
    Hash { bytes }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_content() {
    // same preimage => same hash
    let foo1_hash = Hash::from_content(b"foo");
    let foo2_hash = Hash::from_content(b"foo");
    assert_eq!(foo1_hash, foo2_hash);

    // different preimage => different hash
    let foo_hash = Hash::from_content(b"foo");
    let bar_hash = Hash::from_content(b"bar");
    assert_ne!(foo_hash, bar_hash);
  }
}
