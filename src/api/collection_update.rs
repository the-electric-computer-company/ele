use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionUpdateRequest {
  pub collection_id: CollectionId,
  pub name: String,
}

impl FromProtobuf for CollectionUpdateRequest {
  type Protobuf = svc::CollectionUpdateRequest;
  type Error = api::Error;

  fn from_protobuf(pb_req: svc::CollectionUpdateRequest) -> Result<CollectionUpdateRequest, Error> {
    let mut pb_req = pb_req;
    let collection_id = CollectionId::from_protobuf(pb_req.take_collection_id())?;
    let name = pb_req.take_name();

    if name.len() == 0 {
      return Err(ErrorKind::InvalidCollectionName.into_error("collection name cannot be empty"));
    }

    Ok(CollectionUpdateRequest {
      collection_id,
      name,
    })
  }
}

impl IntoProtobuf for CollectionUpdateRequest {
  type Protobuf = svc::CollectionUpdateRequest;

  fn into_protobuf(self) -> svc::CollectionUpdateRequest {
    let mut pb_req = svc::CollectionUpdateRequest::new();
    pb_req.set_collection_id(self.collection_id.into_protobuf());
    pb_req.set_name(self.name);
    pb_req
  }
}

// TODO: cannot alias the same Result signature twice
// The solution may be to move to the Result<Payload, Error> model
pub type CollectionUpdateResponse = Result<(), Error>;

impl FromProtobuf for CollectionUpdateResponse {
  type Protobuf = svc::CollectionUpdateResponse;
  type Error = Error;

  fn from_protobuf(
    pb_resp: svc::CollectionUpdateResponse,
  ) -> Result<CollectionUpdateResponse, Error> {
    let mut pb_resp = pb_resp;
    let result = if pb_resp.has_error() {
      Err(Error::from_protobuf(pb_resp.take_error()))
    } else {
      Ok(())
    };

    Ok(result)
  }
}

impl IntoProtobuf for CollectionUpdateResponse {
  type Protobuf = svc::CollectionUpdateResponse;

  fn into_protobuf(self) -> svc::CollectionUpdateResponse {
    let mut pb_resp = svc::CollectionUpdateResponse::new();
    match self {
      Ok(()) => {}
      Err(err) => pb_resp.set_error(err.into_protobuf()),
    }
    pb_resp
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  impl RequiredFields for CollectionUpdateRequest {
    fn required_fields() -> CollectionUpdateRequest {
      let collection_id = CollectionId::required_fields();
      let name = "foo".to_string();
      CollectionUpdateRequest {
        collection_id,
        name,
      }
    }
  }

  impl RequiredFields for CollectionUpdateResponse {
    fn required_fields() -> CollectionUpdateResponse {
      Ok(())
    }
  }

  #[test]
  fn collection_update_request_required_fields() {
    test_required_fields::<CollectionUpdateRequest, svc::CollectionUpdateRequest>(&[
      |p| p.set_collection_id(CollectionId::required_fields().into_protobuf()),
      |p| p.set_name("foo".to_string()),
    ])
  }

  #[test]
  fn collection_update_response_required_fields() {
    test_required_fields::<CollectionUpdateResponse, svc::CollectionUpdateResponse>(&[])
  }
}
