use common::*;

#[derive(PartialEq, Debug, Clone)]
pub struct NodeId {
  pubkey: Pubkey,
}

impl NodeId {
  pub fn from_pubkey(pubkey: Pubkey) -> NodeId {
    NodeId { pubkey }
  }

  pub fn key(&self) -> Pubkey {
    self.pubkey
  }
}
