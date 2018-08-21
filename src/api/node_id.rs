use common::*;

use api::Message;

use svc;

impl Message for NodeId {
  type Protobuf = svc::NodeId;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    let mut protobuf = protobuf;
    let pubkey = Pubkey::from_protobuf(protobuf.take_pubkey())?;
    Ok(NodeId::from_pubkey(pubkey))
  }

  fn into_protobuf(self) -> Self::Protobuf {
    let mut protobuf = svc::NodeId::new();
    protobuf.set_pubkey(self.key().into_protobuf());
    protobuf
  }

  #[cfg(test)]
  fn new_valid_test_instance() -> Self {
    NodeId::from_pubkey(Pubkey::new_valid_test_instance())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn node_id_new_valid_test_instance() {
    test_required_fields::<NodeId, svc::NodeId>(&[|p| {
      p.set_pubkey(random::<Pubkey>().into_protobuf())
    }])
  }
}
