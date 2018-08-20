use common::*;

pub trait Message: Sized {
  type Protobuf;
  type Error: Debug;

  fn from_protobuf_message(protobuf: Self::Protobuf) -> Result<Self, Self::Error>;

  fn into_protobuf_message(self) -> Self::Protobuf;

  #[cfg(test)]
  fn required_fields_message() -> Self;
}
