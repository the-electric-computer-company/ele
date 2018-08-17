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

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionCreateResponse {
  pub error: Option<Error>,
  pub collection_id: CollectionId,
}

impl FromProtobuf for CollectionCreateResponse {
  type Protobuf = svc::CollectionCreateResponse;
  type Error = Error;

  fn from_protobuf(
    pb_resp: svc::CollectionCreateResponse,
  ) -> Result<CollectionCreateResponse, Error> {
    let mut pb_resp = pb_resp;
    let error = if pb_resp.has_error() {
      Some(Error::from_protobuf(pb_resp.take_error()))
    } else {
      None
    };

    let collection_id = CollectionId::from_protobuf(pb_resp.take_collection_id())?;
    Ok(CollectionCreateResponse {
      error,
      collection_id,
    })
  }
}

impl IntoProtobuf for CollectionCreateResponse {
  type Protobuf = svc::CollectionCreateResponse;

  fn into_protobuf(self) -> svc::CollectionCreateResponse {
    let mut pb_resp = svc::CollectionCreateResponse::new();
    if let Some(error) = self.error {
      pb_resp.set_error(error.into_protobuf());
    }
    pb_resp.set_collection_id(self.collection_id.into_protobuf());
    pb_resp
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

  impl RequiredFields for CollectionCreateResponse {
    fn required_fields() -> CollectionCreateResponse {
      let collection_id = CollectionId::required_fields();
      CollectionCreateResponse {
        collection_id,
        error: None,
      }
    }
  }

  #[test]
  fn collection_create_request_required_fields() {
    test_required_fields::<CollectionCreateRequest, svc::CollectionCreateRequest>(&[|p| {
      p.set_node_id(NodeId::required_fields().into_protobuf())
    }])
  }

  #[test]
  fn collection_create_response_required_fields() {
    test_required_fields::<CollectionCreateResponse, svc::CollectionCreateResponse>(&[|p| {
      p.set_collection_id(CollectionId::required_fields().into_protobuf())
    }])
  }
}
