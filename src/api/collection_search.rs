use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionSearchRequest {
  pub node_id: NodeId,
}

impl api::Message for CollectionSearchRequest {
  type Protobuf = svc::CollectionSearchRequest;
  type Error = Error;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
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
  fn required_fields() -> Self {
    let node_id = NodeId::required_fields();
    CollectionSearchRequest { node_id: node_id }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  #[test]
  fn collection_search_request_required_fields() {
    test_required_fields::<CollectionSearchRequest, svc::CollectionSearchRequest>(&[|p| {
      p.set_node_id(NodeId::required_fields().into_protobuf())
    }]);
  }
}
