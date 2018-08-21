use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionCreateRequest {
  pub node_id: NodeId,
}

impl Message for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;
  type Error = api::Error;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
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
  fn required_fields() -> Self {
    let node_id = NodeId::required_fields();
    CollectionCreateRequest { node_id }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  #[test]
  fn collection_create_request_required_fields() {
    test_required_fields::<CollectionCreateRequest, svc::CollectionCreateRequest>(&[|p| {
      p.set_node_id(NodeId::required_fields().into_protobuf())
    }])
  }
}
