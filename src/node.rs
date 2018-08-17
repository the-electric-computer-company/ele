use common::*;
use grpc;
#[allow(unused_imports)]
use svc::{self, Node as _Node};

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

  fn collection_create_inner(
    &self,
    req: svc::CollectionCreateRequest,
  ) -> Result<api::CollectionId, api::Error> {
    let req = api::CollectionCreateRequest::from_protobuf(req)?;

    let node_id = unwrap_internal_error(self.library.node_id());

    if req.node_id == node_id {
      Ok(unwrap_internal_error(self.library.collection_create()))
    } else {
      Err(api::ErrorKind::WouldProxy.into_error("proxy not implemented"))
    }
  }
}

impl svc::Node for Node {
  fn collection_create(
    &self,
    _o: ::grpc::RequestOptions,
    req: svc::CollectionCreateRequest,
  ) -> ::grpc::SingleResponse<svc::CollectionCreateResponse> {
    let result = self.collection_create_inner(req);
    let resp = api::CollectionCreateResponse { result };
    grpc::SingleResponse::completed(resp.into_protobuf())
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

  #[test]
  fn collection_create_success() {
    test_init();

    let node = test_node();
    let node_id = node.library.node_id().unwrap();

    let server = test_server(node);
    let client = test_client(server.local_addr().port().unwrap());

    let create_req = api::CollectionCreateRequest { node_id };

    let (_, resp, _) = client
      .collection_create(Default::default(), create_req.into_protobuf())
      .wait()
      .unwrap();

    let create_resp = api::CollectionCreateResponse::from_protobuf(resp).unwrap();
    assert_eq!(create_resp.result.is_ok(), true);
  }

  #[test]
  fn collection_create_failure() {
    test_init();

    let node = test_node();

    let server = test_server(node);
    let client = test_client(server.local_addr().port().unwrap());

    let node_id = NodeId::from_pubkey(random());

    let create_req = api::CollectionCreateRequest { node_id };

    let (_, resp, _) = client
      .collection_create(Default::default(), create_req.into_protobuf())
      .wait()
      .unwrap();

    let create_resp = api::CollectionCreateResponse::from_protobuf(resp).unwrap();

    match create_resp.result {
      Ok(value) => panic!("expected error: {:?}", value),
      Err(api::Error { kind, .. }) => assert_eq!(kind, api::ErrorKind::WouldProxy),
    }
  }
}
