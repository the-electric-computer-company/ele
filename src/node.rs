use common::*;
use grpc;
#[allow(unused_imports)]
use svc::{self, Node as _Node};

use api::Message;

pub struct Node {
  library: Library,
}

fn unwrap_internal_error<T, E: Debug>(result: Result<T, E>) -> T {
  match result {
    Ok(ok) => ok,
    Err(err) => {
      panic!("internal error: {:?}", err);
    }
  }
}

impl Node {
  pub fn new(library: Library) -> Node {
    Node { library }
  }

  pub fn run(self) -> Result<(), Error> {
    Ok(())
  }

  // The purpose of inner functions is to allow for idiomatic rust error
  // handling, which is not possible within the service definition due to the
  // grpc response types.

  fn collection_create_inner(
    &self,
    request: svc::CollectionCreateRequest,
  ) -> Result<CollectionId, api::Error> {
    let request = api::CollectionCreateRequest::from_protobuf(request)?;

    let node_id = unwrap_internal_error(self.library.node_id());

    if request.node_id == node_id {
      Ok(unwrap_internal_error(self.library.collection_create()))
    } else {
      Err(api::ErrorKind::WouldProxy.into_error("proxy not implemented"))
    }
  }

  fn collection_search_inner(
    &self,
    request: svc::CollectionSearchRequest,
  ) -> Result<Vec<CollectionId>, api::Error> {
    let request = api::CollectionSearchRequest::from_protobuf(request)?;

    let node_id = unwrap_internal_error(self.library.node_id());
    if request.node_id == node_id {
      Ok(unwrap_internal_error(self.library.collection_search()))
    } else {
      Err(api::ErrorKind::WouldProxy.into_error("proxy not implemented"))
    }
  }

  fn bundle_create_inner(&self, request: svc::BundleCreateRequest) -> Result<BundleId, api::Error> {
    let api::BundleCreateRequest {
      collection_id,
      record_hash,
      record_data,
      archive_hash,
      archive_data,
    } = api::BundleCreateRequest::from_protobuf(request)?;

    let verify_record_hash = Hash::from_content(record_data.as_slice());
    if record_hash != verify_record_hash {
      return Err(api::ErrorKind::Hash.into_error("invalid record hash"));
    }

    let verify_archive_hash = Hash::from_content(archive_data.as_slice());
    if archive_hash != verify_archive_hash {
      return Err(api::ErrorKind::Hash.into_error("invalid archive hash"));
    }

    let bundle_id = BundleId::from_collection_id(collection_id);
    Ok(bundle_id)
  }
}

impl svc::Node for &'static Node {
  fn collection_create(
    &self,
    _options: ::grpc::RequestOptions,
    request: svc::CollectionCreateRequest,
  ) -> ::grpc::SingleResponse<svc::CollectionCreateResponse> {
    let response = self.collection_create_inner(request);
    let protobuf = response_to_protobuf!(response, svc::CollectionCreateResponse);
    grpc::SingleResponse::completed(protobuf)
  }

  fn collection_search(
    &self,
    _options: ::grpc::RequestOptions,
    request: svc::CollectionSearchRequest,
  ) -> ::grpc::SingleResponse<svc::CollectionSearchResponse> {
    let response = self.collection_search_inner(request);
    let protobuf = response_to_protobuf!(response, svc::CollectionSearchResponse);
    grpc::SingleResponse::completed(protobuf)
  }

  fn bundle_create(
    &self,
    _options: ::grpc::RequestOptions,
    request: svc::BundleCreateRequest,
  ) -> ::grpc::SingleResponse<svc::BundleCreateResponse> {
    let response = self.bundle_create_inner(request);
    let protobuf = response_to_protobuf!(response, svc::BundleCreateResponse);
    grpc::SingleResponse::completed(protobuf)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Test {
    node: &'static Node,
    client: svc::NodeClient,
    _server: grpc::Server,
  }

  impl Test {
    fn new() -> Test {
      test_init();

      let node = Test::node();
      let node = Box::new(node);
      // TODO: this is way less than ideal. The server will continue running
      // after the test ends, and because grpc::Server doesn't implement drop,
      // we cant drop it. In practice it's not too bad until we run enough tests
      // to have many servers running in the background.
      let node: &'static mut Node = Box::leak(node);
      let server = Test::server(node);
      let client = Test::client(server.local_addr().port().unwrap());

      Test {
        node,
        client,
        _server: server,
      }
    }

    fn node() -> Node {
      let tempdir = TempDir::new().unwrap();

      let library_child = tempdir.child("library.db");
      let library_path = library_child.path();

      let library = Library::with_path(library_path).unwrap();

      Node::new(library)
    }

    fn client(port: u16) -> svc::NodeClient {
      let conf = grpc::ClientConf::new();
      svc::NodeClient::new_plain("127.0.0.1", port, conf).unwrap()
    }

    fn server(node: &'static Node) -> grpc::Server {
      let mut server = grpc::ServerBuilder::new_plain();
      server.http.set_addr("127.0.0.1:0").unwrap();
      server.add_service(svc::NodeServer::new_service_def(node));
      server.http.set_cpu_pool_threads(1);
      server.build().unwrap()
    }

    fn collection_create(&self, node_id: NodeId) -> CollectionId {
      let create_req = api::CollectionCreateRequest { node_id };

      let (_, protobuf, _) = self
        .client
        .collection_create(Default::default(), create_req.into_protobuf())
        .wait()
        .unwrap();

      response_from_protobuf!(protobuf, CollectionId).unwrap()
    }

    fn collection_search(&self, node_id: NodeId) -> Vec<CollectionId> {
      let req = api::CollectionSearchRequest { node_id };

      let (_, protobuf, _) = self
        .client
        .collection_search(Default::default(), req.into_protobuf())
        .wait()
        .unwrap();

      response_from_protobuf!(protobuf, Vec<CollectionId>).unwrap()
    }

    fn bundle_create(
      &self,
      collection_id: CollectionId,
      record: &[u8],
      archive: &[u8],
    ) -> BundleId {
      let record_data = record.to_vec();
      let record_hash = Hash::from_content(record);

      let archive_data = archive.to_vec();
      let archive_hash = Hash::from_content(archive);

      let request = api::BundleCreateRequest {
        collection_id,
        record_hash,
        record_data,
        archive_hash,
        archive_data,
      };

      let (_, protobuf, _) = self
        .client
        .bundle_create(Default::default(), request.into_protobuf())
        .wait()
        .unwrap();

      response_from_protobuf!(protobuf, BundleId).unwrap()
    }
  }

  #[test]
  fn collection_create_success() {
    let test = Test::new();

    let node_id = test.node.library.node_id().unwrap();

    test.collection_create(node_id);
  }

  #[test]
  fn collection_create_failure() {
    let test = Test::new();

    let request = api::CollectionCreateRequest {
      node_id: NodeId::from_pubkey(random()),
    };

    let (_, protobuf, _) = test
      .client
      .collection_create(Default::default(), request.into_protobuf())
      .wait()
      .unwrap();

    let response = response_from_protobuf!(protobuf, CollectionId);

    match response {
      Ok(value) => panic!("expected error: {:?}", value),
      Err(api::Error { kind, .. }) => assert_eq!(kind, api::ErrorKind::WouldProxy),
    }
  }

  #[test]
  fn collection_search_success() {
    let test = Test::new();

    let node_id = test.node.library.node_id().unwrap();

    let ids = test.collection_search(node_id.clone());
    assert_eq!(ids.len(), 0);

    test.collection_create(node_id.clone());

    let ids = test.collection_search(node_id.clone());
    assert_eq!(ids.len(), 1);
  }

  #[test]
  fn bundle_create_bad_record_hash() {
    // TODO supply bad hash and figure out array to slice

    let test = Test::new();
    let node_id = test.node.library.node_id().unwrap();
    let collection_id = test.collection_create(node_id);
    let record = random::<[u8; 32]>().to_vec();
    let archive = random::<[u8; 32]>().to_vec();
    test.bundle_create(collection_id, record.as_slice(), archive.as_slice());
  }

  #[test]
  fn bundle_create_bad_record_schema() {}

  #[test]
  fn bundle_create_bad_archive_hash() {}
  // TODO: are there any basic forms of validation we can do on a diskimage?
}
