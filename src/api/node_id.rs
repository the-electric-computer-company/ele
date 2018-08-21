use super::*;

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

impl api::message::Message for NodeId {
  type Protobuf = svc::NodeId;
  type Error = Error;

  fn from_protobuf_message(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
    let mut protobuf = protobuf;
    let pubkey = Pubkey::from_protobuf_message(protobuf.take_pubkey())?;
    Ok(NodeId { pubkey })
  }

  fn into_protobuf_message(self) -> Self::Protobuf {
    let mut protobuf = svc::NodeId::new();
    protobuf.set_pubkey(self.key().into_protobuf_message());
    protobuf
  }

  #[cfg(test)]
  fn required_fields_message() -> Self {
    NodeId {
      pubkey: Pubkey::required_fields_message(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  #[test]
  fn node_id_required_fields() {
    test_required_fields::<NodeId, svc::NodeId>(&[|p| {
      p.set_pubkey(random::<Pubkey>().into_protobuf_message())
    }])
  }
}
