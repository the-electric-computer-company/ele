use common::*;

use api::Message;

use svc;

impl Message for Hash {
  type Protobuf = svc::Hash;

  fn from_protobuf(mut protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    let bytes = protobuf.take_hash();
    if bytes.len() != 32 {
      return Err(api::ErrorKind::Parse.into_error(format!("invalid hash length: {}", bytes.len())));
    }
    let mut hash: [u8; 32] = [0; 32];
    hash.copy_from_slice(bytes.as_slice());
    Ok(Hash::from_array(hash))
  }

  fn into_protobuf(self) -> Self::Protobuf {
    let mut protobuf = svc::Hash::new();
    protobuf.set_hash(self.bytes.to_vec());
    protobuf
  }

  #[cfg(test)]
  fn new_valid_test_instance() -> Self {
    Hash::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn hash_new_valid_test_instance() {
    test_required_fields::<Hash, svc::Hash>(&[|p| {
      let id = random::<[u8; 32]>();
      p.set_hash(id.to_vec());
    }])
  }
}
