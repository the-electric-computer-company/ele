use common::*;
use rand;
use svc;

const ERROR_PARSE: u32 = 1;

pub trait FromProtobuf: Sized {
  type Protobuf;
  type Error: Debug;
  fn from_protobuf(p: Self::Protobuf) -> Result<Self, Self::Error>;
}

pub trait IntoProtobuf: Sized {
  type Protobuf;
  fn into_protobuf(self) -> Self::Protobuf;
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
  fn into_error(self, message: impl AsRef<str>) -> Error {
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

  fn into_protobuf(self) -> svc::Error {
    let mut pb_error = svc::Error::new();
    pb_error.set_code(self.code());
    pb_error.set_message(self.message);
    pb_error
  }

  fn from_protobuf(mut pb_error: svc::Error) -> Error {
    let code = pb_error.get_code();
    let message = pb_error.take_message();

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

impl IntoProtobuf for Pubkey {
  type Protobuf = svc::Pubkey;
  fn into_protobuf(self) -> svc::Pubkey {
    let mut pb_pubkey = svc::Pubkey::new();
    pb_pubkey.set_key((&self.key[..]).to_vec());
    pb_pubkey
  }
}

impl FromProtobuf for Pubkey {
  type Protobuf = svc::Pubkey;
  type Error = Error;

  fn from_protobuf(pb_pubkey: Self::Protobuf) -> Result<Pubkey, Error> {
    let bytes = pb_pubkey.get_key().to_vec();
    if bytes.len() != 16 {
      return Err(ErrorKind::Parse.into_error(format!("invalid pubkey length: {}", bytes.len())));
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

  fn from_protobuf(pb_node_id: svc::NodeId) -> Result<NodeId, Error> {
    let mut pb_node_id = pb_node_id;
    let node_pubkey = Pubkey::from_protobuf(pb_node_id.take_node_pubkey())?;
    Ok(NodeId { node_pubkey })
  }
}

impl IntoProtobuf for NodeId {
  type Protobuf = svc::NodeId;

  fn into_protobuf(self) -> svc::NodeId {
    let mut pb_node_id = svc::NodeId::new();
    pb_node_id.set_node_pubkey(self.node_pubkey.into_protobuf());
    pb_node_id
  }
}

impl RequiredFields for NodeId {
  fn required_fields() -> NodeId {
    NodeId::new()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionId {
  pub collection_pubkey: Pubkey,
}

impl CollectionId {
  pub fn new() -> CollectionId {
    let collection_pubkey = Pubkey::new();
    CollectionId { collection_pubkey }
  }
}

impl FromProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;
  type Error = Error;

  fn from_protobuf(pb_collection_id: svc::CollectionId) -> Result<CollectionId, Error> {
    let mut pb_collection_id = pb_collection_id;
    let collection_pubkey = Pubkey::from_protobuf(pb_collection_id.take_collection_pubkey())?;
    Ok(CollectionId { collection_pubkey })
  }
}

impl IntoProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;

  fn into_protobuf(self) -> svc::CollectionId {
    let mut pb_collection_id = svc::CollectionId::new();
    pb_collection_id.set_collection_pubkey(self.collection_pubkey.into_protobuf());
    pb_collection_id
  }
}

impl RequiredFields for CollectionId {
  fn required_fields() -> CollectionId {
    CollectionId::new()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionCreateRequest {
  pub node_id: NodeId,
}

impl FromProtobuf for CollectionCreateRequest {
  type Protobuf = svc::CollectionCreateRequest;
  type Error = Error;

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
      + IntoProtobuf<Protobuf = P>
      + Clone
      + Debug
      + PartialEq,
    P,
  >() {
    let obj = T::required_fields();
    let pb = obj.clone().into_protobuf();
    let obj2 = T::from_protobuf(pb).unwrap();
    assert_eq!(obj2, obj);
  }

  #[test]
  fn error() {
    let first_error = ErrorKind::Parse.into_error("foo");
    let pb_err = first_error.clone().into_protobuf();
    let second_error = Error::from_protobuf(pb_err);
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
      p.set_node_pubkey(Pubkey::new().into_protobuf())
    }])
  }

  #[test]
  fn collection_id_required_fields() {
    test_required_fields::<CollectionId, svc::CollectionId>(&[|p| {
      p.set_collection_pubkey(Pubkey::required_fields().into_protobuf())
    }])
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
