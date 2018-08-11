use grpc;
use ::node::*;
use ::node_grpc::*;

pub struct NodeImpl;

impl Node for NodeImpl {
  fn archive_hash_get(&self, o: ::grpc::RequestOptions, p: super::node::ArchiveHashGetRequest) -> ::grpc::SingleResponse<super::node::ArchiveHashGetResponse> {
    let mut req = p.clone();
    let mut r = ArchiveHashGetResponse::new();
    let req_id = req.take_request_id();
    r.set_request_id(req_id);
    grpc::SingleResponse::completed(r)
  }

  fn archive_stream(&self, o: ::grpc::RequestOptions, p: super::node::ArchiveStreamRequest) -> ::grpc::StreamingResponse<super::node::ArchiveStreamResponse> {
    let mut r = ArchiveStreamResponse::new();
    grpc::StreamingResponse::completed(vec![r])
  }

  fn bundle_create(&self, o: ::grpc::RequestOptions, p: super::node::BundleCreateRequest) -> ::grpc::SingleResponse<super::node::BundleCreateResponse> {
    let mut r = BundleCreateResponse::new();
    grpc::SingleResponse::completed(r)

  }

  fn bundle_delete(&self, o: ::grpc::RequestOptions, p: super::node::BundleDeleteRequest) -> ::grpc::SingleResponse<super::node::BundleDeleteResponse> {
    let mut r = BundleDeleteResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn bundle_pin(&self, o: ::grpc::RequestOptions, p: super::node::BundlePinRequest) -> ::grpc::SingleResponse<super::node::BundlePinResponse> {
    let mut r = BundlePinResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn bundle_search(&self, o: ::grpc::RequestOptions, p: super::node::BundleSearchRequest) -> ::grpc::SingleResponse<super::node::BundleSearchResponse> {
    let mut r = BundleSearchResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn bundle_unpin(&self, o: ::grpc::RequestOptions, p: super::node::BundleUnpinRequest) -> ::grpc::SingleResponse<super::node::BundleUnpinResponse> {
    let mut r = BundleUnpinResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn collection_create(&self, o: ::grpc::RequestOptions, p: super::node::CollectionCreateRequest) -> ::grpc::SingleResponse<super::node::CollectionCreateResponse> {
    let mut r = CollectionCreateResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn collection_search(&self, o: ::grpc::RequestOptions, p: super::node::CollectionSearchRequest) -> ::grpc::SingleResponse<super::node::CollectionSearchResponse> {
    let mut r = CollectionSearchResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn collection_update(&self, o: ::grpc::RequestOptions, p: super::node::CollectionUpdateRequest) -> ::grpc::SingleResponse<super::node::CollectionUpdateResponse> {
    let mut r = CollectionUpdateResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn record_get(&self, o: ::grpc::RequestOptions, p: super::node::RecordGetRequest) -> ::grpc::SingleResponse<super::node::RecordGetResponse> {
    let mut r = RecordGetResponse::new();
    grpc::SingleResponse::completed(r)
  }

  fn syndicate(&self, o: ::grpc::RequestOptions, p: super::node::SyndicateRequest) -> ::grpc::SingleResponse<super::node::SyndicateResponse> {
    let mut r = SyndicateResponse::new();
    grpc::SingleResponse::completed(r)
  }
}
