use common::*;
use rand;
use svc;

const ERROR_PARSE: u32 = 1;

pub trait FromProtobuf: Sized {
  type Protobuf;
  type Error: Debug;
  fn from_protobuf(p: Self::Protobuf) -> Result<Self, Self::Error>;
}

pub trait ToProtobuf: Sized {
  type Protobuf;
  fn to_protobuf(self) -> Self::Protobuf;
}

pub trait RequiredFields {
  fn required_fields() -> Self;
}

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
  pub message: String,
  pub kind: ErrorKind,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorKind {
  Unknown { code: u32 },
  Parse,
}

impl ErrorKind {
  fn to_error(self, message: impl AsRef<str>) -> Error {
    Error {
      message: message.as_ref().to_string(),
      kind: self,
    }
  }
}

impl Error {
  fn code(&self) -> u32 {
    use self::ErrorKind::*;
    match self.kind {
      Unknown { code } => code,
      Parse => ERROR_PARSE,
    }
  }

  fn to_protobuf(self) -> svc::Error {
    let mut proto_error = svc::Error::new();
    proto_error.set_code(self.code());
    proto_error.set_message(self.message);
    proto_error
  }

  fn from_protobuf(mut proto_error: svc::Error) -> Error {
    let code = proto_error.get_code();
    let message = proto_error.take_message();

    use self::ErrorKind::*;
    let kind = match code {
      ERROR_PARSE => Parse,
      _ => Unknown { code },
    };
    Error { message, kind }
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Pubkey {
  pub key: [u8; 16],
}

impl Pubkey {
  pub fn new() -> Pubkey {
    Pubkey {
      key: rand::random(),
    }
  }
}

impl ToProtobuf for Pubkey {
  type Protobuf = svc::Pubkey;
  fn to_protobuf(self) -> svc::Pubkey {
    let mut proto_pubkey = svc::Pubkey::new();
    proto_pubkey.set_key((&self.key[..]).to_vec());
    proto_pubkey
  }
}

impl FromProtobuf for Pubkey {
  type Protobuf = svc::Pubkey;
  type Error = Error;

  fn from_protobuf(proto_pubkey: Self::Protobuf) -> Result<Pubkey, Error> {
    let bytes = proto_pubkey.get_key().to_vec();
    if bytes.len() != 16 {
      return Err(ErrorKind::Parse.to_error(format!("invalid pubkey length: {}", bytes.len())));
    }
    let mut pubkey = Pubkey { key: [0; 16] };
    pubkey.key.copy_from_slice(bytes.as_slice());
    Ok(pubkey)
  }
}

impl RequiredFields for Pubkey {
  fn required_fields() -> Pubkey {
    Pubkey::new()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct NodeId {
  node_pubkey: Pubkey,
}

impl NodeId {
  pub fn new() -> NodeId {
    let node_pubkey = Pubkey::new();
    NodeId { node_pubkey }
  }
}

impl FromProtobuf for NodeId {
  type Protobuf = svc::NodeId;
  type Error = Error;

  fn from_protobuf(proto_node_id: svc::NodeId) -> Result<NodeId, Error> {
    let mut proto_node_id = proto_node_id;
    let node_pubkey = Pubkey::from_protobuf(proto_node_id.take_node_pubkey())?;
    Ok(NodeId { node_pubkey })
  }
}

impl ToProtobuf for NodeId {
  type Protobuf = svc::NodeId;

  fn to_protobuf(self) -> svc::NodeId {
    let mut proto_node_id = svc::NodeId::new();
    proto_node_id.set_node_pubkey(self.node_pubkey.to_protobuf());
    proto_node_id
  }
}

impl RequiredFields for NodeId {
  fn required_fields() -> NodeId {
    NodeId::new()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionId {
  pub node_id: NodeId,
  pub collection_pubkey: Pubkey,
}

// TODO change proto to pb

impl CollectionId {
  pub fn new(node_id: NodeId) -> CollectionId {
    let collection_pubkey = Pubkey::new();
    CollectionId {
      node_id,
      collection_pubkey,
    }
  }
}

impl FromProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;
  type Error = Error;

  fn from_protobuf(proto_collection_id: svc::CollectionId) -> Result<CollectionId, Error> {
    let mut proto_collection_id = proto_collection_id;
    let node_id = NodeId::from_protobuf(proto_collection_id.take_node_id())?;
    let collection_pubkey = Pubkey::from_protobuf(proto_collection_id.take_collection_pubkey())?;
    Ok(CollectionId {
      node_id,
      collection_pubkey,
    })
  }
}

impl ToProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;

  fn to_protobuf(self) -> svc::CollectionId {
    let mut proto_collection_id = svc::CollectionId::new();
    proto_collection_id.set_node_id(self.node_id.to_protobuf());
    proto_collection_id.set_collection_pubkey(self.collection_pubkey.to_protobuf());
    proto_collection_id
  }
}

impl RequiredFields for CollectionId {
  fn required_fields() -> CollectionId {
    let node_id = NodeId::required_fields();
    CollectionId::new(node_id)
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionCreateRequest {
  pub node_id: NodeId,
}

impl FromProtobuf for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;
  type Error = Error;

  fn from_protobuf(
    proto_req: svc::CollectionCreateRequest,
  ) -> Result<CollectionCreateRequest, Error> {
    let mut proto_req = proto_req;
    let node_id = NodeId::from_protobuf(proto_req.take_node_id())?;
    Ok(CollectionCreateRequest { node_id })
  }
}

impl ToProtobuf for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;

  fn to_protobuf(self) -> svc::CollectionCreateRequest {
    let mut proto_req = svc::CollectionCreateRequest::new();
    proto_req.set_node_id(self.node_id.to_protobuf());
    proto_req
  }
}

impl RequiredFields for CollectionCreateRequest {
  fn required_fields() -> CollectionCreateRequest {
    let node_id = NodeId::required_fields();
    CollectionCreateRequest { node_id }
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
    proto_resp: svc::CollectionCreateResponse,
  ) -> Result<CollectionCreateResponse, Error> {
    let mut proto_resp = proto_resp;
    let error = if proto_resp.has_error() {
      Some(Error::from_protobuf(proto_resp.take_error()))
    } else {
      None
    };

    let collection_id = CollectionId::from_protobuf(proto_resp.take_collection_id())?;
    Ok(CollectionCreateResponse {
      error,
      collection_id,
    })
  }
}

impl ToProtobuf for CollectionCreateResponse {
  type Protobuf = svc::CollectionCreateResponse;

  fn to_protobuf(self) -> svc::CollectionCreateResponse {
    let mut proto_resp = svc::CollectionCreateResponse::new();
    if let Some(error) = self.error {
      proto_resp.set_error(error.to_protobuf());
    }
    proto_resp.set_collection_id(self.collection_id.to_protobuf());
    proto_resp
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

#[cfg(test)]
mod tests {
  use super::*;

  fn test_required_fields<T: FromProtobuf<Protobuf = P> + Debug, P: Default>(
    setters: &[fn(&mut P)],
  ) {
    for i in 0..setters.len() {
      let mut victim = Default::default();
      for (j, setter) in setters.iter().enumerate() {
        if j == i {
          continue;
        }
        setter(&mut victim);
      }
      T::from_protobuf(victim).expect_err("it worked");
    }

    let mut p = Default::default();
    for setter in setters {
      setter(&mut p)
    }
    T::from_protobuf(p).expect("parsing failed when all required fields were present");
  }

  fn test_round_trip<
    T: RequiredFields
      + FromProtobuf<Protobuf = P>
      + ToProtobuf<Protobuf = P>
      + Clone
      + Debug
      + PartialEq,
    P,
  >() {
    let obj = T::required_fields();
    let pb = obj.clone().to_protobuf();
    let obj2 = T::from_protobuf(pb).unwrap();
    assert_eq!(obj2, obj);
  }

  #[test]
  fn error() {
    let first_error = ErrorKind::Parse.to_error("foo");
    let proto_err = first_error.clone().to_protobuf();
    let second_error = Error::from_protobuf(proto_err);
    assert_eq!(second_error, first_error);
  }

  #[test]
  fn round_trips() {
    test_round_trip::<Pubkey, svc::Pubkey>();
    test_round_trip::<NodeId, svc::NodeId>();
    test_round_trip::<CollectionId, svc::CollectionId>();
    test_round_trip::<CollectionCreateRequest, svc::CollectionCreateRequest>();
    test_round_trip::<CollectionCreateResponse, svc::CollectionCreateResponse>();
  }

  #[test]
  fn pubkey_required_fields() {
    test_required_fields::<Pubkey, svc::Pubkey>(&[|p| {
      p.set_key(rand::random::<[u8; 16]>().iter().cloned().collect())
    }])
  }

  #[test]
  fn bad_pubkey() {
    let mut bad_pubkey = svc::Pubkey::new();
    let bad_uuid: Vec<u8> = vec![1, 2, 4];
    bad_pubkey.set_key(bad_uuid);
    assert_eq!(
      Pubkey::from_protobuf(bad_pubkey)
        .expect_err("bad uuid should have caused an error")
        .kind,
      ErrorKind::Parse
    )
  }

  #[test]
  fn node_id_required_fields() {
    test_required_fields::<NodeId, svc::NodeId>(&[|p| {
      p.set_node_pubkey(Pubkey::new().to_protobuf())
    }])
  }

  #[test]
  fn collection_id_required_fields() {
    test_required_fields::<CollectionId, svc::CollectionId>(&[
      |p| p.set_node_id(NodeId::required_fields().to_protobuf()),
      |p| p.set_collection_pubkey(Pubkey::required_fields().to_protobuf()),
    ])
  }

  #[test]
  fn collection_create_request_required_fields() {
    test_required_fields::<CollectionCreateRequest, svc::CollectionCreateRequest>(&[|p| {
      p.set_node_id(NodeId::required_fields().to_protobuf())
    }])
  }

  #[test]
  fn collection_create_response_required_fields() {
    test_required_fields::<CollectionCreateResponse, svc::CollectionCreateResponse>(&[|p| {
      p.set_collection_id(CollectionId::required_fields().to_protobuf())
    }])
  }

  // #[test]
  // fn test_collection_create_response() {
  //   let first_req_id = RequestId::new();
  //   let first_error = Error {
  //     code: 42,
  //     message: "foo".to_string(),
  //   };
  //   let first_node_id = NodeId::new();
  //   let first_col_id = CollectionId::new(first_node_id);

  //   let first_col_create_resp = CollectionCreateResponse {
  //     request_id: first_req_id,
  //     error: Some(first_error),
  //     collection_id: first_col_id,
  //   };

  //   let proto_resp = first_col_create_resp.to_protobuf();
  //   let second_col_create_resp = CollectionCreateResponse::from_protobuf(&proto_resp).unwrap();
  //   assert_eq!(second_col_create_resp, first_col_create_resp);
  // }
}
