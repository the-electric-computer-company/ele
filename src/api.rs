use ::svc;
use failure;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(PartialEq, Debug)]
struct Error {
  code: u32,
  message: String,
}

impl Error {
  fn from_protobuf(proto_error: &svc::Error) -> Result<Error, failure::Error> {
    let code = proto_error.get_code();
    let message = proto_error.get_message().to_string();
    Ok(Error { code, message })
  }

  fn to_protobuf(&self) -> svc::Error {
    let mut proto_error = svc::Error::new();
    proto_error.set_code(self.code);
    proto_error.set_message(self.message.clone());
    proto_error
  }
}

#[derive(PartialEq, Debug)]
struct RequestId {
  id: Uuid,
}

impl RequestId {
  fn new() -> RequestId {
    RequestId { id: Uuid::new_v4() }
  }

  fn from_protobuf(proto_req_id: &svc::RequestId) -> Result<RequestId, failure::Error> {
    let vec = proto_req_id.get_request_id().to_vec();
    Ok(RequestId {
      id: Uuid::from_bytes(vec.as_slice())?,
    })
  }

  fn to_protobuf(&self) -> svc::RequestId {
    let buf = self.id.as_bytes().to_vec();
    let mut protobuf_req_id = svc::RequestId::new();
    protobuf_req_id.set_request_id(buf);
    protobuf_req_id
  }
}

#[derive(PartialEq, Debug)]
struct Pubkey {
  key: Vec<u8>,
}

impl Pubkey {
  fn new() -> Pubkey {
    let uuid = Uuid::new_v4();
    Pubkey::from_bytes(uuid.as_bytes())
  }

  fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Pubkey {
    Pubkey {
      key: bytes.as_ref().to_vec(),
    }
  }

  fn from_protobuf(proto_pubkey: &svc::Pubkey) -> Result<Pubkey, failure::Error> {
    let vec = proto_pubkey.get_key().to_vec();
    // make sure it's a valid uuid
    let uuid = Uuid::from_bytes(vec.as_slice())?;
    Ok(Pubkey::from_bytes(uuid.as_bytes()))
  }

  fn to_protobuf(&self) -> svc::Pubkey {
    let mut proto_pubkey = svc::Pubkey::new();
    proto_pubkey.set_key(self.key.clone());
    proto_pubkey
  }
}

#[derive(PartialEq, Debug)]
struct NodeId {
  node_pubkey: Pubkey,
}

impl NodeId {
  fn new() -> NodeId {
    let node_pubkey = Pubkey::new();
    NodeId { node_pubkey }
  }

  fn from_protobuf(proto_node_id: &svc::NodeId) -> Result<NodeId, failure::Error> {
    let node_pubkey = Pubkey::from_protobuf(proto_node_id.get_node_pubkey())?;
    Ok(NodeId { node_pubkey })
  }

  fn to_protobuf(&self) -> svc::NodeId {
    let mut proto_node_id = svc::NodeId::new();
    proto_node_id.set_node_pubkey(self.node_pubkey.to_protobuf());
    proto_node_id
  }
}

#[derive(PartialEq, Debug)]
struct CollectionId {
  node_id: NodeId,
  collection_pubkey: Pubkey,
}

impl CollectionId {
  fn new(node_id: NodeId) -> CollectionId {
    let collection_pubkey = Pubkey::new();
    CollectionId {
      node_id,
      collection_pubkey,
    }
  }

  fn from_protobuf(
    proto_collection_id: &svc::CollectionId,
  ) -> Result<CollectionId, failure::Error> {
    let node_id = NodeId::from_protobuf(proto_collection_id.get_node_id())?;
    let collection_pubkey = Pubkey::from_protobuf(proto_collection_id.get_collection_pubkey())?;
    Ok(CollectionId {
      node_id,
      collection_pubkey,
    })
  }

  fn to_protobuf(&self) -> svc::CollectionId {
    let mut proto_collection_id = svc::CollectionId::new();
    proto_collection_id.set_node_id(self.node_id.to_protobuf());
    proto_collection_id.set_collection_pubkey(self.collection_pubkey.to_protobuf());
    proto_collection_id
  }
}

#[derive(PartialEq, Debug)]
struct BundleId {
  collection_id: CollectionId,
  bundle_id: Uuid,
}

impl BundleId {
  fn new(collection_id: CollectionId) -> BundleId {
    let bundle_id = Uuid::new_v4();
    BundleId {
      collection_id,
      bundle_id,
    }
  }

  fn from_protobuf(proto_bundle_id: &svc::BundleId) -> Result<BundleId, failure::Error> {
    let collection_id = CollectionId::from_protobuf(proto_bundle_id.get_collection_id())?;
    let bundle_id = Uuid::from_bytes(proto_bundle_id.get_bundle_id())?;
    Ok(BundleId {
      collection_id,
      bundle_id,
    })
  }

  fn to_protobuf(&self) -> svc::BundleId {
    let mut bundle_id = svc::BundleId::new();
    bundle_id.set_collection_id(self.collection_id.to_protobuf());
    bundle_id.set_bundle_id(self.bundle_id.as_bytes().to_vec());
    bundle_id
  }
}

#[derive(PartialEq, Debug)]
struct Hash {
  sha256: Vec<u8>,
}

impl Hash {
  fn sha256_digest(buf: &[u8]) -> Hash {
    let mut hasher = Sha256::default();
    hasher.input(buf);
    let result = hasher.result();
    Hash {
      sha256: result.to_vec(),
    }
  }

  fn from_protobuf(hash: &svc::Hash) -> Result<Hash, failure::Error> {
    let bytes = hash.get_hash();
    if bytes.len() == 32 {
      Ok(Hash {
        sha256: bytes.to_vec(),
      })
    } else {
      Err(format_err!(
        "invalid sha256 hash input of length: {}",
        bytes.len()
      ))
    }
  }

  fn to_protobuf(&self) -> svc::Hash {
    let mut proto_hash = svc::Hash::new();
    proto_hash.set_hash(self.sha256.clone());
    proto_hash
  }
}

#[derive(PartialEq, Debug)]
struct CollectionCreateRequest {
  request_id: RequestId,
  node_id: NodeId,
}

impl CollectionCreateRequest {
  fn from_protobuf(
    proto_req: &svc::CollectionCreateRequest,
  ) -> Result<CollectionCreateRequest, failure::Error> {
    let request_id = RequestId::from_protobuf(proto_req.get_request_id())?;
    let node_id = NodeId::from_protobuf(proto_req.get_node_id())?;
    Ok(CollectionCreateRequest {
      request_id,
      node_id,
    })
  }

  fn to_protobuf(&self) -> svc::CollectionCreateRequest {
    let mut proto_req = svc::CollectionCreateRequest::new();
    proto_req.set_request_id(self.request_id.to_protobuf());
    proto_req.set_node_id(self.node_id.to_protobuf());
    proto_req
  }
}

#[derive(PartialEq, Debug)]
struct CollectionCreateResponse {
  request_id: RequestId,
  error: Error,
  collection_id: CollectionId,
}

impl CollectionCreateResponse {
  fn from_protobuf(
    proto_resp: &svc::CollectionCreateResponse,
  ) -> Result<CollectionCreateResponse, failure::Error> {
    let request_id = RequestId::from_protobuf(proto_resp.get_request_id())?;
    let error = Error::from_protobuf(proto_resp.get_error())?;
    let collection_id = CollectionId::from_protobuf(proto_resp.get_collection_id())?;
    Ok(CollectionCreateResponse {
      request_id,
      error,
      collection_id,
    })
  }

  fn to_protobuf(&self) -> svc::CollectionCreateResponse {
    let mut proto_resp = svc::CollectionCreateResponse::new();
    proto_resp.set_request_id(self.request_id.to_protobuf());
    proto_resp.set_error(self.error.to_protobuf());
    proto_resp.set_collection_id(self.collection_id.to_protobuf());
    proto_resp
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error() {
    let first_error = Error {
      code: 42,
      message: "foo".to_string(),
    };
    let proto_err = first_error.to_protobuf();
    let second_error = Error::from_protobuf(&proto_err).unwrap();
    assert_eq!(second_error, first_error);
  }

  #[test]
  fn test_request_id() {
    let first_req_id = RequestId::new();
    let buf = first_req_id.id.as_bytes().to_vec();
    let mut protobuf_req_id_out = first_req_id.to_protobuf();
    assert_eq!(protobuf_req_id_out.take_request_id(), buf);

    let mut protobuf_req_id_in = svc::RequestId::new();
    protobuf_req_id_in.set_request_id(buf.clone());
    let second_req_id = RequestId::from_protobuf(&protobuf_req_id_in).unwrap();
    assert_eq!(second_req_id, first_req_id);
  }

  #[test]
  fn test_pubkey() {
    let first_pubkey = Pubkey::new();
    let proto_pubkey = first_pubkey.to_protobuf();
    assert_eq!(proto_pubkey.get_key(), first_pubkey.key.as_slice());

    let second_pubkey = Pubkey::from_protobuf(&proto_pubkey).unwrap();
    assert_eq!(first_pubkey, second_pubkey);
  }

  #[test]
  fn test_node_id() {
    let first_node_id = NodeId::new();
    let proto_node_id = first_node_id.to_protobuf();

    let second_node_id = NodeId::from_protobuf(&proto_node_id).unwrap();
    assert_eq!(second_node_id, first_node_id);
  }

  #[test]
  fn test_collection_id() {
    let first_node_id = NodeId::new();
    let mut first_collection_id = CollectionId::new(first_node_id);
    let proto_collection_id = first_collection_id.to_protobuf();
    let second_collection_id = CollectionId::from_protobuf(&proto_collection_id).unwrap();
    assert_eq!(second_collection_id, first_collection_id);

    // sanity check to make sure sub-objects are also compared
    let second_node_id = NodeId::new();
    first_collection_id.node_id = second_node_id;
    assert_ne!(second_collection_id, first_collection_id);
  }

  #[test]
  fn test_bundle_id() {
    let first_node_id = NodeId::new();
    let first_collection_id = CollectionId::new(first_node_id);
    let first_bundle_id = BundleId::new(first_collection_id);

    let proto_bundle_id = first_bundle_id.to_protobuf();
    let second_bundle_id = BundleId::from_protobuf(&proto_bundle_id).unwrap();
    assert_eq!(second_bundle_id, first_bundle_id);
  }

  #[test]
  fn test_hash() {
    let first_hash = Hash::sha256_digest(b"Hey kid, I'm a computa!");
    let proto_hash = first_hash.to_protobuf();
    let second_hash = Hash::from_protobuf(&proto_hash).unwrap();
    assert_eq!(second_hash, first_hash);
    let different_hash = Hash::sha256_digest(b"Stop all the downloadin'");
    assert_ne!(different_hash, second_hash);
  }

  #[test]
  fn test_collection_create_request() {
    let first_req_id = RequestId::new();
    let first_node_id = NodeId::new();
    let first_col_create_req = CollectionCreateRequest {
      request_id: first_req_id,
      node_id: first_node_id,
    };

    let proto_col_create_req = first_col_create_req.to_protobuf();
    let second_col_create_req =
      CollectionCreateRequest::from_protobuf(&proto_col_create_req).unwrap();
    assert_eq!(second_col_create_req, first_col_create_req);
  }

  #[test]
  fn test_collection_create_response() {
    let first_req_id = RequestId::new();
    let first_error = Error{code: 42, message: "foo".to_string()};
    let first_node_id = NodeId::new();
    let first_col_id = CollectionId::new(first_node_id);

    let first_col_create_resp = CollectionCreateResponse{
      request_id: first_req_id,
      error: first_error,
      collection_id: first_col_id,
    };

    let proto_resp = first_col_create_resp.to_protobuf();
    let second_col_create_resp = CollectionCreateResponse::from_protobuf(&proto_resp).unwrap();
    assert_eq!(second_col_create_resp, first_col_create_resp);
  }
}
