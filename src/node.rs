use api::{self, FromProtobuf, ToProtobuf};
use common::*;
use env_logger;
use grpc;
use log::LevelFilter;
use std::default;
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
    proto_create_req: svc::CollectionCreateRequest,
  ) -> ::grpc::SingleResponse<svc::CollectionCreateResponse> {
    let create_req = api::CollectionCreateRequest::from_protobuf(proto_create_req).unwrap();
    let create_resp = api::CollectionCreateResponse {
      error: None,
      collection_id: api::CollectionId::new(create_req.node_id),
    };

    grpc::SingleResponse::completed(create_resp.to_protobuf())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_client() -> svc::NodeClient {
    let conf = grpc::ClientConf::new();
    svc::NodeClient::new_plain("127.0.0.1", 2018, conf).unwrap()
  }

  fn test_server() -> grpc::Server {
    let lib_path = Library::default_path();
    let library = Library::with_path(lib_path).unwrap();
    let node = Node::new(library);

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_addr("127.0.0.1:2018").unwrap();
    server.add_service(svc::NodeServer::new_service_def(node));
    server.http.set_cpu_pool_threads(1);
    server.build().unwrap()
  }

  #[test]
  fn test_collection_create() {
    env_logger::Builder::from_default_env()
      .filter_level(LevelFilter::Error)
      .init();

    let _server = test_server();
    let client = test_client();

    let create_req = api::CollectionCreateRequest {
      node_id: api::NodeId::new(),
    };

    let (_, resp, _) = client
      .collection_create(default::Default::default(), create_req.to_protobuf())
      .wait()
      .unwrap();

    println!("resp: {:?}", resp);
  }
}
