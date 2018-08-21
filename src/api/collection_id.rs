use super::*;

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

impl FromProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;
  type Error = Error;

  fn from_protobuf(pb_collection_id: svc::CollectionId) -> Result<CollectionId, Error> {
    let mut pb_collection_id = pb_collection_id;
    let pubkey = Pubkey::from_protobuf(pb_collection_id.take_pubkey())?;
    Ok(CollectionId { pubkey })
  }
}

impl api::Message for CollectionId {
  type Protobuf = svc::CollectionId;
  type Error = Error;

  fn from_protobuf_message(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
    FromProtobuf::from_protobuf(protobuf)
  }

  fn into_protobuf_message(self) -> Self::Protobuf {
    let mut protobuf = svc::CollectionId::new();
    protobuf.set_pubkey(self.pubkey.into_protobuf_message());
    protobuf
  }

  #[cfg(test)]
  fn required_fields_message() -> Self {
    CollectionId {
      pubkey: Pubkey::required_fields_message(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  #[test]
  fn collection_id_required_fields() {
    test_required_fields::<CollectionId, svc::CollectionId>(&[|p| {
      p.set_pubkey(Pubkey::required_fields_message().into_protobuf_message())
    }])
  }
}
