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

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionSearchResponse {
  pub result: Result<Vec<CollectionId>, Error>,
}

impl FromProtobuf for CollectionSearchResponse {
  type Protobuf = svc::CollectionSearchResponse;
  type Error = Error;

  fn from_protobuf(
    pb_resp: svc::CollectionSearchResponse,
  ) -> Result<CollectionSearchResponse, Error> {
    let mut pb_resp = pb_resp;
    let result = if pb_resp.has_error() {
      Err(Error::from_protobuf(pb_resp.take_error()))
    } else {
      Ok(CollectionId::from_protobuf(
        pb_resp.take_collection_ids().into_vec().,
      )?)
    };

    Ok(CollectionSearchResponse { result })
  }
}

impl IntoProtobuf for CollectionSearchResponse {
  type Protobuf = svc::CollectionSearchResponse;

  fn into_protobuf(self) -> svc::CollectionSearchResponse {
    let mut pb_resp = svc::CollectionSearchResponse::new();
    match self.result {
      Ok(id) => pb_resp.set_collection_id(id.into_protobuf()),
      Err(err) => pb_resp.set_error(err.into_protobuf()),
    }
    pb_resp
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   use super::super::tests::*;

//   impl RequiredFields for CollectionCreateRequest {
//     fn required_fields() -> CollectionCreateRequest {
//       let node_id = NodeId::required_fields();
//       CollectionCreateRequest { node_id }
//     }
//   }

//   impl RequiredFields for CollectionCreateResponse {
//     fn required_fields() -> CollectionCreateResponse {
//       let id = CollectionId::required_fields();
//       CollectionCreateResponse { result: Ok(id) }
//     }
//   }

//   #[test]
//   fn collection_create_request_required_fields() {
//     test_required_fields::<CollectionCreateRequest, svc::CollectionCreateRequest>(&[|p| {
//       p.set_node_id(NodeId::required_fields().into_protobuf())
//     }])
//   }

//   #[test]
//   fn collection_create_response_required_fields() {
//     test_required_fields::<CollectionCreateResponse, svc::CollectionCreateResponse>(&[|p| {
//       p.set_collection_id(CollectionId::required_fields().into_protobuf())
//     }])
//   }
// }
