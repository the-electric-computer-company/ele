use common::*;

use api::Message;
use svc;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionCreateRequest {
  pub node_id: NodeId,
}

impl Message for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    let mut protobuf = protobuf;
    let node_id = NodeId::from_protobuf(protobuf.take_node_id())?;
    Ok(CollectionCreateRequest { node_id })
  }

  fn into_protobuf(self) -> Self::Protobuf {
    let mut protobuf = svc::CollectionCreateRequest::new();
    protobuf.set_node_id(self.node_id.into_protobuf());
    protobuf
  }

  #[cfg(test)]
  fn new_valid_test_instance() -> Self {
    let node_id = NodeId::new_valid_test_instance();
    CollectionCreateRequest { node_id }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn collection_create_request_new_valid_test_instance() {
    test_required_fields::<CollectionCreateRequest, svc::CollectionCreateRequest>(&[|p| {
      p.set_node_id(NodeId::new_valid_test_instance().into_protobuf())
    }])
  }
}
