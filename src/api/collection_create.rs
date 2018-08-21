use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionCreateRequest {
  pub node_id: NodeId,
}

impl FromProtobuf for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;
  type Error = api::Error;

  fn from_protobuf(pb_req: svc::CollectionCreateRequest) -> Result<CollectionCreateRequest, Error> {
    let mut pb_req = pb_req;
    let node_id = NodeId::from_protobuf(pb_req.take_node_id())?;
    Ok(CollectionCreateRequest { node_id })
  }
}

impl IntoProtobuf for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;

  fn into_protobuf(self) -> svc::CollectionCreateRequest {
    let mut pb_req = svc::CollectionCreateRequest::new();
    pb_req.set_node_id(self.node_id.into_protobuf());
    pb_req
  }
}

impl Message for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;
  type Error = api::Error;

  fn from_protobuf_message(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
    FromProtobuf::from_protobuf(protobuf)
  }

  fn into_protobuf_message(self) -> Self::Protobuf {
    self.into_protobuf()
  }

  #[cfg(test)]
  fn required_fields_message() -> Self {
    api::tests::RequiredFields::required_fields()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  impl RequiredFields for CollectionCreateRequest {
    fn required_fields() -> CollectionCreateRequest {
      let node_id = NodeId::required_fields();
      CollectionCreateRequest { node_id }
    }
  }
  #[test]
  fn collection_create_request_required_fields() {
    test_required_fields::<CollectionCreateRequest, svc::CollectionCreateRequest>(&[|p| {
      p.set_node_id(NodeId::required_fields().into_protobuf())
    }])
  }
}
