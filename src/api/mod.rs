use common::*;
use svc;

mod collection_create;
mod collection_id;
mod collection_search;
mod error;
mod message;
mod node_id;

pub use self::{
  collection_create::CollectionCreateRequest,
  collection_id::CollectionId,
  collection_search::CollectionSearchRequest,
  error::{Error, ErrorKind},
  message::Message,
};

macro_rules! response_to_protobuf {
  ($result:expr, $response:ty) => {{
    let result: Result<_, api::Error> = $result;

    let mut response: $response = default();

    match result {
      Ok(payload) => response.set_payload(payload.into_protobuf()),
      Err(error) => response.set_error(error.into_protobuf()),
    }

    response
  }};
}

macro_rules! response_from_protobuf {
  ($protobuf:expr, $payload:ty) => {{
    let mut protobuf = $protobuf;

    if protobuf.has_error() {
      Err(api::Error::from_protobuf(protobuf.take_error()))
    } else {
      <$payload>::from_protobuf(protobuf.take_payload())
    }
  }};
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub fn test_required_fields<T: Message<Protobuf = P> + Debug, P: Default>(
    setters: &[fn(&mut P)],
  ) {
    for i in 0..setters.len() {
      let mut victim = default();
      for (j, setter) in setters.iter().enumerate() {
        if j == i {
          continue;
        }
        setter(&mut victim);
      }
      T::from_protobuf(victim).expect_err("it worked");
    }

    let mut p = default();
    for setter in setters {
      setter(&mut p)
    }
    T::from_protobuf(p).expect("parsing failed when all required fields were present");
  }

  fn test_round_trip_message<T: Message<Protobuf = P> + Clone + Debug + PartialEq, P>() {
    let obj = T::required_fields();
    let pb = obj.clone().into_protobuf();
    let obj2 = T::from_protobuf(pb).unwrap();
    assert_eq!(obj2, obj);
  }

  #[test]
  fn round_trips_message() {
    test_round_trip_message::<Pubkey, svc::Pubkey>();
    test_round_trip_message::<NodeId, svc::NodeId>();
    test_round_trip_message::<CollectionId, svc::CollectionId>();
    test_round_trip_message::<CollectionSearchRequest, svc::CollectionSearchRequest>();
    test_round_trip_message::<CollectionCreateRequest, svc::CollectionCreateRequest>();
  }
}
