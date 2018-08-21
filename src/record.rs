use serde_cbor;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Arch {
  Amd64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Record {
  DiskImage {
    distro: String,
    version: String,
    arch: Arch,
  },
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn round_trip() {
    let old_record = Record::DiskImage {
      distro: "foo".to_string(),
      version: "1".to_string(),
      arch: Arch::Amd64,
    };

    let bytes = serde_cbor::to_vec(&old_record).unwrap();
    let new_record = serde_cbor::from_slice::<Record>(bytes.as_slice()).unwrap();
    assert_eq!(new_record, old_record);
  }
}
