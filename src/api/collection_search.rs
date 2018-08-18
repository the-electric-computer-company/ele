use super::*;

use protobuf::RepeatedField;

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

pub type CollectionSearchResponse = Result<Vec<CollectionId>, Error>;

impl FromProtobuf for CollectionSearchResponse {
  type Protobuf = svc::CollectionSearchResponse;
  type Error = api::Error;

  fn from_protobuf(
    mut response: svc::CollectionSearchResponse,
  ) -> Result<CollectionSearchResponse, Error> {
    let result = if response.has_error() {
      Err(Error::from_protobuf(response.take_error()))
    } else {
      response
        .take_collection_ids()
        .into_iter()
        .map(CollectionId::from_protobuf)
        .collect::<Result<Vec<CollectionId>, api::Error>>()
    };

    Ok(result)
  }
}

impl IntoProtobuf for CollectionSearchResponse {
  type Protobuf = svc::CollectionSearchResponse;

  fn into_protobuf(self) -> svc::CollectionSearchResponse {
    let mut response = svc::CollectionSearchResponse::new();
    match self {
      Ok(ids) => response.set_collection_ids(RepeatedField::from_vec(
        ids
          .into_iter()
          .map(|collection_id| collection_id.into_protobuf())
          .collect(),
      )),
      Err(err) => response.set_error(err.into_protobuf()),
    }
    response
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

  impl RequiredFields for CollectionSearchResponse {
    fn required_fields() -> CollectionSearchResponse {
      Ok(vec![])
    }
  }

  #[test]
  fn collection_search_request_required_fields() {
    test_required_fields::<CollectionSearchRequest, svc::CollectionSearchRequest>(&[|p| {
      p.set_node_id(NodeId::required_fields().into_protobuf())
    }]);
  }

  #[test]
  fn collection_search_response_required_fields() {
    test_required_fields::<CollectionSearchResponse, svc::CollectionSearchResponse>(&[]);
  }
}
