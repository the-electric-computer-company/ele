use common::*;
use grpc;
#[allow(unused_imports)]
use svc::{self, Node as _Node};

pub struct Node {
  _library: Library,
}

impl Node {
  pub fn new(library: Library) -> Node {
    Node { _library: library }
  }

  pub fn run(self) -> Result<(), Error> {
    Ok(())
  }
}

impl svc::Node for Node {
  fn collection_create(
    &self,
    _o: ::grpc::RequestOptions,
    _pb_create_req: svc::CollectionCreateRequest,
  ) -> ::grpc::SingleResponse<svc::CollectionCreateResponse> {
    unimplemented!();

    // let create_resp = api::CollectionCreateResponse {
    //   error: None,
    //   collection_id: api::CollectionId::new(),
    // };

    // grpc::SingleResponse::completed(create_resp.into_protobuf())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // use env_logger;
  // use log::LevelFilter;

  fn test_client() -> svc::NodeClient {
    let conf = grpc::ClientConf::new();
    svc::NodeClient::new_plain("127.0.0.1", 2018, conf).unwrap()
  }

  fn test_node() -> Node {
    let tempdir = TempDir::new().unwrap();

    let library_child = tempdir.child("library.db");
    let library_path = library_child.path();

    let library = Library::with_path(library_path).unwrap();

    Node::new(library)
  }

  fn test_server() -> grpc::Server {
    let node = test_node();
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_addr("127.0.0.1:2018").unwrap();
    server.add_service(svc::NodeServer::new_service_def(node));
    server.http.set_cpu_pool_threads(1);
    server.build().unwrap()
  }

  #[test]
  fn collection_create() {
    test_init();

    let _server = test_server();
    let _client = test_client();

    // let create_req = api::CollectionCreateRequest {
    //   node_id: api::NodeId::new(),
    // };

    // let (_, resp, _) = client
    //   .collection_create(default::Default::default(), create_req.into_protobuf())
    //   .wait()
    //   .unwrap();

    // println!("resp: {:?}", resp);
  }
}
