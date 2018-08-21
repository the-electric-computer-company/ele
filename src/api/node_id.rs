use super::*;

impl Message for NodeId {
  type Protobuf = svc::NodeId;
  type Error = Error;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
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
  fn required_fields() -> Self {
    NodeId::from_pubkey(Pubkey::required_fields())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn node_id_required_fields() {
    test_required_fields::<NodeId, svc::NodeId>(&[|p| {
      p.set_pubkey(random::<Pubkey>().into_protobuf())
    }])
  }
}
