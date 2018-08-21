use common::*;

use api::Message;
use svc;

impl Message for CollectionId {
  type Protobuf = svc::CollectionId;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    let mut protobuf = protobuf;
    let pubkey = Pubkey::from_protobuf(protobuf.take_pubkey())?;
    Ok(CollectionId { pubkey })
  }

  fn into_protobuf(self) -> Self::Protobuf {
    let mut protobuf = svc::CollectionId::new();
    protobuf.set_pubkey(self.pubkey.into_protobuf());
    protobuf
  }

  #[cfg(test)]
  fn required_fields() -> Self {
    CollectionId {
      pubkey: Pubkey::required_fields(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn collection_id_required_fields() {
    test_required_fields::<CollectionId, svc::CollectionId>(&[|p| {
      p.set_pubkey(Pubkey::required_fields().into_protobuf())
    }])
  }
}
