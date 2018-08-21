mod bundle_id;
mod collection_create;
mod collection_id;
mod collection_search;
mod error;
mod message;
mod node_id;
mod repeated;

pub use self::{
  collection_create::CollectionCreateRequest,
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

#[cfg(test)]
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

  use common::*;
  use svc;

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

  fn test_round_trip<T: Message<Protobuf = P> + Clone + Debug + PartialEq, P>() {
    let obj = T::new_valid_test_instance();
    let pb = obj.clone().into_protobuf();
    let obj2 = T::from_protobuf(pb).unwrap();
    assert_eq!(obj2, obj);
  }

  #[test]
  fn round_trips() {
    test_round_trip::<Pubkey, svc::Pubkey>();
    test_round_trip::<NodeId, svc::NodeId>();
    test_round_trip::<CollectionId, svc::CollectionId>();
    test_round_trip::<BundleId, svc::BundleId>();
    test_round_trip::<CollectionSearchRequest, svc::CollectionSearchRequest>();
    test_round_trip::<CollectionCreateRequest, svc::CollectionCreateRequest>();
  }

  macro_rules! round_trip_response_test {
    (name: $name:ident,payload: $payload:ty,response: $response:ty,) => {
      #[test]
      fn $name() {
        let input = Ok(<$payload>::new_valid_test_instance());
        let protobuf = response_to_protobuf!(input.clone(), $response);
        let output = response_from_protobuf!(protobuf, $payload);
        assert_eq!(input, output);

        let input: Result<$payload, _> = Err(ErrorKind::Parse.into_error("bad message"));
        let protobuf = response_to_protobuf!(input.clone(), $response);
        let output = response_from_protobuf!(protobuf, $payload);
        assert_eq!(input, output);
      }
    };
  }

  round_trip_response_test! {
    name:     collection_create_response,
    payload:  CollectionId,
    response: svc::CollectionCreateResponse,
  }

  round_trip_response_test! {
    name:     collection_search_response,
    payload:  Vec<CollectionId>,
    response: svc::CollectionSearchResponse,
  }
}
