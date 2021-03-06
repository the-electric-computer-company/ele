use common::*;

pub trait Message: Sized {
  type Protobuf;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error>;

  fn into_protobuf(self) -> Self::Protobuf;

  #[cfg(test)]
  fn new_valid_test_instance() -> Self;
}
