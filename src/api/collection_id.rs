use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct CollectionId {
  pub collection_pubkey: Pubkey,
}

impl FromProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;
  type Error = Error;

  fn from_protobuf(pb_collection_id: svc::CollectionId) -> Result<CollectionId, Error> {
    let mut pb_collection_id = pb_collection_id;
    let collection_pubkey = Pubkey::from_protobuf(pb_collection_id.take_collection_pubkey())?;
    Ok(CollectionId { collection_pubkey })
  }
}

impl IntoProtobuf for CollectionId {
  type Protobuf = svc::CollectionId;

  fn into_protobuf(self) -> svc::CollectionId {
    let mut pb_collection_id = svc::CollectionId::new();
    pb_collection_id.set_collection_pubkey(self.collection_pubkey.into_protobuf());
    pb_collection_id
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::super::tests::*;

  impl RequiredFields for CollectionId {
    fn required_fields() -> CollectionId {
      CollectionId {
        collection_pubkey: Pubkey::required_fields(),
      }
    }
  }

  #[test]
  fn collection_id_required_fields() {
    test_required_fields::<CollectionId, svc::CollectionId>(&[|p| {
      p.set_collection_pubkey(Pubkey::required_fields().into_protobuf())
    }])
  }
}
