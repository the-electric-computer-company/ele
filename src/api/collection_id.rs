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

impl IntoProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;

  fn into_protobuf(self) -> svc::CollectionId {
    let mut pb_collection_id = svc::CollectionId::new();
    pb_collection_id.set_pubkey(self.pubkey.into_protobuf());
    pb_collection_id
  }
}

impl api::Message for CollectionId {
  type Protobuf = svc::CollectionId;
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

    CollectionId {
      pubkey: Pubkey::required_fields(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  impl RequiredFields for CollectionId {
    fn required_fields() -> CollectionId {
      CollectionId {
        pubkey: Pubkey::required_fields(),
      }
    }
  }

  #[test]
  fn collection_id_required_fields() {
    test_required_fields::<CollectionId, svc::CollectionId>(&[|p| {
      p.set_pubkey(Pubkey::required_fields().into_protobuf())
    }])
  }
}
