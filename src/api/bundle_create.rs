use common::*;

use api::Message;
use svc;

#[derive(PartialEq, Debug, Clone)]
pub struct BundleCreateRequest {
  pub collection_id: CollectionId,
  pub record_hash: Hash,
  pub record_data: Vec<u8>,
  pub archive_hash: Hash,
  pub archive_data: Vec<u8>,
}

impl Message for BundleCreateRequest {
  type Protobuf = svc::BundleCreateRequest;

  fn from_protobuf(mut protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    let collection_id = CollectionId::from_protobuf(protobuf.take_collection_id())?;
    let record_hash = Hash::from_protobuf(protobuf.take_record_hash())?;

    let record_data = protobuf.take_record_data();
    if record_data.len() == 0 {
      return Err(api::ErrorKind::Parse.into_error("non-empty record required"));
    }

    let archive_hash = Hash::from_protobuf(protobuf.take_archive_hash())?;

    let archive_data = protobuf.take_archive_data();
    if archive_data.len() == 0 {
      return Err(api::ErrorKind::Parse.into_error("non-empty archive required"));
    }

    Ok(BundleCreateRequest {
      collection_id,
      record_hash,
      record_data,
      archive_hash,
      archive_data,
    })
  }

  fn into_protobuf(self) -> Self::Protobuf {
    let mut protobuf = svc::BundleCreateRequest::new();
    protobuf.set_collection_id(self.collection_id.into_protobuf());
    protobuf.set_record_hash(self.record_hash.into_protobuf());
    protobuf.set_record_data(self.record_data);
    protobuf.set_archive_hash(self.archive_hash.into_protobuf());
    protobuf.set_archive_data(self.archive_data);
    protobuf
  }

  #[cfg(test)]
  fn new_valid_test_instance() -> Self {
    let collection_id = CollectionId::new_valid_test_instance();
    let record_data = random::<[u8; 32]>().to_vec();
    let record_hash = Hash::from_content(record_data.as_slice());
    let archive_data = random::<[u8; 32]>().to_vec();
    let archive_hash = Hash::from_content(archive_data.as_slice());
    BundleCreateRequest {
      collection_id,
      record_hash,
      record_data,
      archive_hash,
      archive_data,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn bundle_create_request_new_valid_test_instance() {
    test_required_fields::<BundleCreateRequest, svc::BundleCreateRequest>(&[
      |p| p.set_collection_id(CollectionId::new_valid_test_instance().into_protobuf()),
      |p| p.set_record_data(b"record data".to_vec()),
      |p| p.set_record_hash(Hash::from_content(b"record data").into_protobuf()),
      |p| p.set_archive_data(b"archive data".to_vec()),
      |p| p.set_archive_hash(Hash::from_content(b"archive data").into_protobuf()),
    ])
  }
}
