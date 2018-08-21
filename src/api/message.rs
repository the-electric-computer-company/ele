use common::*;

pub trait Message: Sized {
  type Protobuf;
  type Error: Debug;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, Self::Error>;

  fn into_protobuf(self) -> Self::Protobuf;

  #[cfg(test)]
  fn required_fields() -> Self;
}
