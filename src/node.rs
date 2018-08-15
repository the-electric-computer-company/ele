use common::*;
use grpc;
use svc;

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
    _p: svc::CollectionCreateRequest,
  ) -> ::grpc::SingleResponse<svc::CollectionCreateResponse> {
    let r = svc::CollectionCreateResponse::new();
    grpc::SingleResponse::completed(r)
  }
}
