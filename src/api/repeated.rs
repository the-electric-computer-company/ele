use common::*;

use api::Message;
use protobuf::RepeatedField;
use svc;

impl Message for Vec<CollectionId> {
  type Protobuf = RepeatedField<svc::CollectionId>;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    protobuf
      .into_iter()
      .map(CollectionId::from_protobuf)
      .collect::<Result<Vec<CollectionId>, api::Error>>()
  }

  fn into_protobuf(self) -> Self::Protobuf {
    self.into_iter().map(CollectionId::into_protobuf).collect()
  }

  #[cfg(test)]
  fn new_valid_test_instance() -> Self {
    Vec::new()
  }
}
