use common::*;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionId {
  pub pubkey: Pubkey,
}

impl CollectionId {
  pub fn from_pubkey(pubkey: Pubkey) -> CollectionId {
    CollectionId { pubkey }
  }

  pub fn key(&self) -> Pubkey {
    self.pubkey
  }
}
