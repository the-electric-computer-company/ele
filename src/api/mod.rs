use common::*;
use svc;

mod collection_create;
mod collection_id;
mod collection_search;
mod error;
mod from_protobuf;
mod into_protobuf;
mod node_id;

pub use self::{
  collection_create::{CollectionCreateRequest, CollectionCreateResponse},
  collection_id::CollectionId,
  collection_search::{CollectionSearchRequest, CollectionSearchResponse},
  error::{Error, ErrorKind},
  from_protobuf::FromProtobuf,
  into_protobuf::IntoProtobuf,
  node_id::NodeId,
};

macro_rules! response_to_protobuf {
  ($result:expr, $response:ty) => {{
    let result: Result<_, api::Error> = $result;

    let mut response: $response = Default::default();

    match result {
      Ok(payload) => response.set_payload(payload.into_protobuf()),
      Err(error) => response.set_error(error.into_protobuf()),
    }

    response
  }};
}

macro_rules! response_from_protobuf {
  ($protobuf:expr, $payload:ty) => {{
    let protobuf = $protobuf;

    if protbuf.has_error() && protobuf.has_payload() {
      unimplemeted!()
    }

    if !protobuf.has_error() && !protobuf.has_payload() {
      unimplemeted!()
    }

    if protobuf.has_error() {
      api::Error::from_protobuf(protobuf.take_error())
    } else {
      $payload::from_protobuf(protobuf.take_payload())
    }
  }};
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub trait RequiredFields {
    fn required_fields() -> Self;
  }

  pub fn test_required_fields<T: FromProtobuf<Protobuf = P> + Debug, P: Default>(
    setters: &[fn(&mut P)],
  ) {
    for i in 0..setters.len() {
      let mut victim = Default::default();
      for (j, setter) in setters.iter().enumerate() {
        if j == i {
          continue;
        }
        setter(&mut victim);
      }
      T::from_protobuf(victim).expect_err("it worked");
    }

    let mut p = Default::default();
    for setter in setters {
      setter(&mut p)
    }
    T::from_protobuf(p).expect("parsing failed when all required fields were present");
  }

  fn test_round_trip<
    T: RequiredFields
      + FromProtobuf<Protobuf = P>
      + IntoProtobuf<Protobuf = P>
      + Clone
      + Debug
      + PartialEq,
    P,
  >() {
    let obj = T::required_fields();
    let pb = obj.clone().into_protobuf();
    let obj2 = T::from_protobuf(pb).unwrap();
    assert_eq!(obj2, obj);
  }

  #[test]
  fn round_trips() {
    test_round_trip::<Pubkey, svc::Pubkey>();
    test_round_trip::<NodeId, svc::NodeId>();
    test_round_trip::<CollectionId, svc::CollectionId>();
    test_round_trip::<CollectionCreateRequest, svc::CollectionCreateRequest>();
    test_round_trip::<CollectionCreateResponse, svc::CollectionCreateResponse>();
    test_round_trip::<CollectionSearchResponse, svc::CollectionSearchResponse>();
  }
}