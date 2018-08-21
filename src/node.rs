use common::*;
use grpc;
#[allow(unused_imports)]
use svc::{self, Node as _Node};

use api::Message;
use protobuf::RepeatedField;

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
    req: svc::CollectionCreateRequest,
  ) -> Result<CollectionId, api::Error> {
    let req = api::CollectionCreateRequest::from_protobuf(req)?;

    let node_id = unwrap_internal_error(self.library.node_id());

    if req.node_id == node_id {
      Ok(unwrap_internal_error(self.library.collection_create()))
    } else {
      Err(api::ErrorKind::WouldProxy.into_error("proxy not implemented"))
    }
  }

  fn collection_search_inner(
    &self,
    req: svc::CollectionSearchRequest,
  ) -> Result<Vec<CollectionId>, api::Error> {
    let req = api::CollectionSearchRequest::from_protobuf(req)?;

    let node_id = unwrap_internal_error(self.library.node_id());
    if req.node_id == node_id {
      Ok(unwrap_internal_error(self.library.collection_search()))
    } else {
      Err(api::ErrorKind::WouldProxy.into_error("proxy not implemented"))
    }
  }
}

impl Message for Vec<CollectionId> {
  type Protobuf = RepeatedField<svc::CollectionId>;

  fn from_protobuf(protobuf: Self::Protobuf) -> Result<Self, api::Error> {
    protobuf
      .into_iter()
      .map(CollectionId::from_protobuf)
      .collect::<Result<Vec<CollectionId>, api::Error>>()
  }

  fn into_protobuf(self) -> Self::Protobuf {
    self.into_iter().map(CollectionId::into_protobuf).collect()
  }

  #[cfg(test)]
  fn required_fields() -> Self {
    Vec::new()
  }
}

impl svc::Node for Node {
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
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_client(port: u16) -> svc::NodeClient {
    let conf = grpc::ClientConf::new();
    svc::NodeClient::new_plain("127.0.0.1", port, conf).unwrap()
  }

  fn test_node() -> Node {
    let tempdir = TempDir::new().unwrap();

    let library_child = tempdir.child("library.db");
    let library_path = library_child.path();

    let library = Library::with_path(library_path).unwrap();

    Node::new(library)
  }

  fn test_server(node: Node) -> grpc::Server {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_addr("127.0.0.1:0").unwrap();
    server.add_service(svc::NodeServer::new_service_def(node));
    server.http.set_cpu_pool_threads(1);
    server.build().unwrap()
  }

  fn create_req(client: &svc::NodeClient, node_id: NodeId) -> CollectionId {
    let create_req = api::CollectionCreateRequest { node_id };

    let (_, protobuf, _) = client
      .collection_create(Default::default(), create_req.into_protobuf())
      .wait()
      .unwrap();

    response_from_protobuf!(protobuf, CollectionId).unwrap()
  }

  fn search_req(client: &svc::NodeClient, node_id: NodeId) -> Vec<CollectionId> {
    let req = api::CollectionSearchRequest { node_id };

    let (_, protobuf, _) = client
      .collection_search(Default::default(), req.into_protobuf())
      .wait()
      .unwrap();

    response_from_protobuf!(protobuf, Vec<CollectionId>).unwrap()
  }

  #[test]
  fn collection_create_success() {
    test_init();

    let node = test_node();
    let node_id = node.library.node_id().unwrap();

    let server = test_server(node);
    let client = test_client(server.local_addr().port().unwrap());

    create_req(&client, node_id);
  }

  #[test]
  fn collection_create_failure() {
    test_init();

    let node = test_node();

    let server = test_server(node);
    let client = test_client(server.local_addr().port().unwrap());

    let node_id = NodeId::from_pubkey(random());

    let req = api::CollectionCreateRequest { node_id };

    let (_, protobuf, _) = client
      .collection_create(Default::default(), req.into_protobuf())
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
    test_init();

    let node = test_node();
    let node_id = node.library.node_id().unwrap();

    let server = test_server(node);
    let client = test_client(server.local_addr().port().unwrap());

    let ids = search_req(&client, node_id.clone());
    assert_eq!(ids.len(), 0);

    create_req(&client, node_id.clone());

    let ids = search_req(&client, node_id.clone());
    assert_eq!(ids.len(), 1);
  }

}
