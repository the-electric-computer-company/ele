use common::*;

pub trait FromProtobuf: Sized {
  type Protobuf;
  type Error: Debug;
  fn from_protobuf(p: Self::Protobuf) -> Result<Self, Self::Error>;
}
