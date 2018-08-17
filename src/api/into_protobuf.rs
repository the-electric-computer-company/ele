pub trait IntoProtobuf: Sized {
  type Protobuf;
  fn into_protobuf(self) -> Self::Protobuf;
}
