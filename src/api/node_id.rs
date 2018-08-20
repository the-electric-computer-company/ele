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

impl FromProtobuf for NodeId {
  type Protobuf = svc::NodeId;
  type Error = Error;

  fn from_protobuf(pb_node_id: svc::NodeId) -> Result<NodeId, Error> {
    let mut pb_node_id = pb_node_id;
    let pubkey = Pubkey::from_protobuf(pb_node_id.take_pubkey())?;
    Ok(NodeId { pubkey })
  }
}

impl IntoProtobuf for NodeId {
  type Protobuf = svc::NodeId;

  fn into_protobuf(self) -> svc::NodeId {
    let mut pb_node_id = svc::NodeId::new();
    pb_node_id.set_pubkey(self.key().into_protobuf());
    pb_node_id
  }
}

impl api::message::Message for NodeId {
  type Protobuf = svc::NodeId;
  type Error = Error;

  fn from_protobuf_message(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
    FromProtobuf::from_protobuf(protobuf)
  }

  fn into_protobuf_message(self) -> Self::Protobuf {
    self.into_protobuf()
  }

  #[cfg(test)]
  fn required_fields_message() -> Self {
    use api::tests::RequiredFields;

    NodeId {
      pubkey: Pubkey::required_fields(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  impl RequiredFields for NodeId {
    fn required_fields() -> NodeId {
      NodeId {
        pubkey: Pubkey::required_fields(),
      }
    }
  }

  #[test]
  fn node_id_required_fields() {
    test_required_fields::<NodeId, svc::NodeId>(&[|p| {
      p.set_pubkey(random::<Pubkey>().into_protobuf())
    }])
  }
}
