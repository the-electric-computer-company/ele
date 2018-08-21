use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionSearchRequest {
  pub node_id: NodeId,
}

impl FromProtobuf for CollectionSearchRequest {
  type Protobuf = svc::CollectionSearchRequest;
  type Error = api::Error;

  fn from_protobuf(pb_req: svc::CollectionSearchRequest) -> Result<CollectionSearchRequest, Error> {
    let mut pb_req = pb_req;
    let node_id = NodeId::from_protobuf(pb_req.take_node_id())?;
    Ok(CollectionSearchRequest { node_id })
  }
}

impl IntoProtobuf for CollectionSearchRequest {
  type Protobuf = svc::CollectionSearchRequest;

  fn into_protobuf(self) -> svc::CollectionSearchRequest {
    let mut pb_req = svc::CollectionSearchRequest::new();
    pb_req.set_node_id(self.node_id.into_protobuf());
    pb_req
  }
}

impl api::Message for CollectionSearchRequest {
  type Protobuf = svc::CollectionSearchRequest;
  type Error = Error;

  fn from_protobuf_message(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
    FromProtobuf::from_protobuf(protobuf)
  }

  fn into_protobuf_message(self) -> Self::Protobuf {
    self.into_protobuf()
  }

  #[cfg(test)]
  fn required_fields_message() -> Self {
    use api::tests::RequiredFields;

    let node_id = NodeId::required_fields();
    CollectionSearchRequest { node_id: node_id }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  impl RequiredFields for CollectionSearchRequest {
    fn required_fields() -> CollectionSearchRequest {
      let node_id = NodeId::required_fields();
      CollectionSearchRequest { node_id }
    }
  }

  #[test]
  fn collection_search_request_required_fields() {
    test_required_fields::<CollectionSearchRequest, svc::CollectionSearchRequest>(&[|p| {
      p.set_node_id(NodeId::required_fields().into_protobuf())
    }]);
  }
}
