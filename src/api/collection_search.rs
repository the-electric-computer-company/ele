use common::*;

use api::Message;
use svc;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionSearchRequest {
  pub node_id: NodeId,
}

impl Message for CollectionSearchRequest {
  type Protobuf = svc::CollectionSearchRequest;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    let mut protobuf = protobuf;
    let node_id = NodeId::from_protobuf(protobuf.take_node_id())?;
    Ok(CollectionSearchRequest { node_id })
  }

  fn into_protobuf(self) -> Self::Protobuf {
    let mut protobuf = svc::CollectionSearchRequest::new();
    protobuf.set_node_id(self.node_id.into_protobuf());
    protobuf
  }

  #[cfg(test)]
  fn new_valid_test_instance() -> Self {
    let node_id = NodeId::new_valid_test_instance();
    CollectionSearchRequest { node_id: node_id }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn collection_search_request_new_valid_test_instance() {
    test_required_fields::<CollectionSearchRequest, svc::CollectionSearchRequest>(&[|p| {
      p.set_node_id(NodeId::new_valid_test_instance().into_protobuf())
    }]);
  }
}
