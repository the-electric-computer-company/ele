use common::*;

#[derive(PartialEq, Debug, Clone)]
pub struct BundleId {
  collection_id: CollectionId,
  id: [u8; 16],
}

impl BundleId {
  pub fn new(collection_id: CollectionId, id: [u8; 16]) -> BundleId {
    BundleId { collection_id, id }
  }

  pub fn from_collection_id(collection_id: CollectionId) -> BundleId {
    let id = random::<[u8; 16]>();
    BundleId { collection_id, id }
  }

  pub fn collection_id(&self) -> CollectionId {
    self.collection_id.clone()
  }

  pub fn id(&self) -> [u8; 16] {
    self.id
  }
}
