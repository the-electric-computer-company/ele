use common::*;

use api::Message;

use svc;

impl Message for BundleId {
  type Protobuf = svc::BundleId;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    let mut protobuf = protobuf;
    let collection_id = CollectionId::from_protobuf(protobuf.take_collection_id())?;
    let bytes = protobuf.take_bundle_id();
    if bytes.len() != 16 {
      return Err(
        api::ErrorKind::Parse.into_error(format!("invalid bundle id length: {}", bytes.len())),
      );
    }
    let mut id: [u8; 16] = [0; 16];
    id.copy_from_slice(bytes.as_slice());
    Ok(BundleId::new(collection_id, id))
  }

  fn into_protobuf(self) -> Self::Protobuf {
    let mut protobuf = svc::BundleId::new();
    protobuf.set_collection_id(self.collection_id().into_protobuf());
    protobuf.set_bundle_id(self.id().to_vec());
    protobuf
  }

  #[cfg(test)]
  fn new_valid_test_instance() -> Self {
    let collection_id = CollectionId::new_valid_test_instance();
    BundleId::from_collection_id(collection_id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn bundle_id_new_valid_test_instance() {
    test_required_fields::<BundleId, svc::BundleId>(&[
      |p| p.set_collection_id(CollectionId::new_valid_test_instance().into_protobuf()),
      |p| {
        let id = random::<[u8; 16]>();
        p.set_bundle_id(id.to_vec());
      },
    ])
  }
}
