use common::*;

use svc;

use rand::{
  distributions::{Distribution, Standard},
  Rng,
};

#[derive(Debug, PartialEq)]
pub enum Error {
  Length { length: usize },
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Pubkey {
  pub bytes: [u8; 16],
}

impl Pubkey {
  pub fn from_slice(slice: &[u8]) -> Result<Pubkey, Error> {
    if slice.len() != 16 {
      Err(self::Error::Length {
        length: slice.len(),
      })
    } else {
      let mut bytes = [0; 16];
      bytes.copy_from_slice(slice);
      Ok(Pubkey { bytes })
    }
  }
}

impl Distribution<Pubkey> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pubkey {
    Pubkey {
      bytes: self.sample(rng),
    }
  }
}

impl api::Message for Pubkey {
  type Protobuf = svc::Pubkey;
  type Error = api::Error;

  fn from_protobuf_message(protobuf: Self::Protobuf) -> Result<Self, Self::Error> {
    let bytes = protobuf.get_key().to_vec();
    if bytes.len() != 16 {
      return Err(
        api::ErrorKind::Parse.into_error(format!("invalid pubkey length: {}", bytes.len())),
      );
    }
    let mut pubkey = Pubkey { bytes: [0; 16] };
    pubkey.bytes.copy_from_slice(bytes.as_slice());
    Ok(pubkey)
  }

  fn into_protobuf_message(self) -> Self::Protobuf {
    let mut protobuf = svc::Pubkey::new();
    protobuf.set_key((&self.bytes[..]).to_vec());
    protobuf
  }

  #[cfg(test)]
  fn required_fields_message() -> Self {
    random()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use api::tests::*;

  #[test]
  fn bad_pubkey() {
    use api::Message;

    let mut bad_pubkey = svc::Pubkey::new();
    let bad_uuid: Vec<u8> = vec![1, 2, 4];
    bad_pubkey.set_key(bad_uuid);
    assert_eq!(
      Pubkey::from_protobuf_message(bad_pubkey)
        .expect_err("bad uuid should have caused an error")
        .kind,
      api::ErrorKind::Parse
    )
  }

  #[test]
  fn pubkey_required_fields() {
    test_required_fields::<Pubkey, svc::Pubkey>(&[|p| p.set_key(random::<Pubkey>().bytes.to_vec())])
  }
}
