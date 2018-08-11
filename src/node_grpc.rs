// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Node {
    fn archive_hash_get(&self, o: ::grpc::RequestOptions, p: super::node::ArchiveHashGetRequest) -> ::grpc::SingleResponse<super::node::ArchiveHashGetResponse>;

    fn archive_stream(&self, o: ::grpc::RequestOptions, p: super::node::ArchiveStreamRequest) -> ::grpc::StreamingResponse<super::node::ArchiveStreamResponse>;

    fn bundle_create(&self, o: ::grpc::RequestOptions, p: super::node::BundleCreateRequest) -> ::grpc::SingleResponse<super::node::BundleCreateResponse>;

    fn bundle_delete(&self, o: ::grpc::RequestOptions, p: super::node::BundleDeleteRequest) -> ::grpc::SingleResponse<super::node::BundleDeleteResponse>;

    fn bundle_pin(&self, o: ::grpc::RequestOptions, p: super::node::BundlePinRequest) -> ::grpc::SingleResponse<super::node::BundlePinResponse>;

    fn bundle_search(&self, o: ::grpc::RequestOptions, p: super::node::BundleSearchRequest) -> ::grpc::SingleResponse<super::node::BundleSearchResponse>;

    fn bundle_unpin(&self, o: ::grpc::RequestOptions, p: super::node::BundleUnpinRequest) -> ::grpc::SingleResponse<super::node::BundleUnpinResponse>;

    fn collection_create(&self, o: ::grpc::RequestOptions, p: super::node::CollectionCreateRequest) -> ::grpc::SingleResponse<super::node::CollectionCreateResponse>;

    fn collection_search(&self, o: ::grpc::RequestOptions, p: super::node::CollectionSearchRequest) -> ::grpc::SingleResponse<super::node::CollectionSearchResponse>;

    fn collection_update(&self, o: ::grpc::RequestOptions, p: super::node::CollectionUpdateRequest) -> ::grpc::SingleResponse<super::node::CollectionUpdateResponse>;

    fn record_get(&self, o: ::grpc::RequestOptions, p: super::node::RecordGetRequest) -> ::grpc::SingleResponse<super::node::RecordGetResponse>;

    fn syndicate(&self, o: ::grpc::RequestOptions, p: super::node::SyndicateRequest) -> ::grpc::SingleResponse<super::node::SyndicateResponse>;
}

// client

pub struct NodeClient {
    grpc_client: ::grpc::Client,
    method_ArchiveHashGet: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::ArchiveHashGetRequest, super::node::ArchiveHashGetResponse>>,
    method_ArchiveStream: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::ArchiveStreamRequest, super::node::ArchiveStreamResponse>>,
    method_BundleCreate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::BundleCreateRequest, super::node::BundleCreateResponse>>,
    method_BundleDelete: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::BundleDeleteRequest, super::node::BundleDeleteResponse>>,
    method_BundlePin: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::BundlePinRequest, super::node::BundlePinResponse>>,
    method_BundleSearch: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::BundleSearchRequest, super::node::BundleSearchResponse>>,
    method_BundleUnpin: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::BundleUnpinRequest, super::node::BundleUnpinResponse>>,
    method_CollectionCreate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::CollectionCreateRequest, super::node::CollectionCreateResponse>>,
    method_CollectionSearch: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::CollectionSearchRequest, super::node::CollectionSearchResponse>>,
    method_CollectionUpdate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::CollectionUpdateRequest, super::node::CollectionUpdateResponse>>,
    method_RecordGet: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::RecordGetRequest, super::node::RecordGetResponse>>,
    method_Syndicate: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node::SyndicateRequest, super::node::SyndicateResponse>>,
}

impl NodeClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        NodeClient {
            grpc_client: grpc_client,
            method_ArchiveHashGet: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/ArchiveHashGet".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ArchiveStream: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/ArchiveStream".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BundleCreate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/BundleCreate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BundleDelete: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/BundleDelete".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BundlePin: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/BundlePin".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BundleSearch: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/BundleSearch".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BundleUnpin: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/BundleUnpin".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CollectionCreate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/CollectionCreate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CollectionSearch: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/CollectionSearch".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CollectionUpdate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/CollectionUpdate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RecordGet: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/RecordGet".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Syndicate: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Node/Syndicate".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            NodeClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            NodeClient::with_client(c)
        })
    }
}

impl Node for NodeClient {
    fn archive_hash_get(&self, o: ::grpc::RequestOptions, p: super::node::ArchiveHashGetRequest) -> ::grpc::SingleResponse<super::node::ArchiveHashGetResponse> {
        self.grpc_client.call_unary(o, p, self.method_ArchiveHashGet.clone())
    }

    fn archive_stream(&self, o: ::grpc::RequestOptions, p: super::node::ArchiveStreamRequest) -> ::grpc::StreamingResponse<super::node::ArchiveStreamResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_ArchiveStream.clone())
    }

    fn bundle_create(&self, o: ::grpc::RequestOptions, p: super::node::BundleCreateRequest) -> ::grpc::SingleResponse<super::node::BundleCreateResponse> {
        self.grpc_client.call_unary(o, p, self.method_BundleCreate.clone())
    }

    fn bundle_delete(&self, o: ::grpc::RequestOptions, p: super::node::BundleDeleteRequest) -> ::grpc::SingleResponse<super::node::BundleDeleteResponse> {
        self.grpc_client.call_unary(o, p, self.method_BundleDelete.clone())
    }

    fn bundle_pin(&self, o: ::grpc::RequestOptions, p: super::node::BundlePinRequest) -> ::grpc::SingleResponse<super::node::BundlePinResponse> {
        self.grpc_client.call_unary(o, p, self.method_BundlePin.clone())
    }

    fn bundle_search(&self, o: ::grpc::RequestOptions, p: super::node::BundleSearchRequest) -> ::grpc::SingleResponse<super::node::BundleSearchResponse> {
        self.grpc_client.call_unary(o, p, self.method_BundleSearch.clone())
    }

    fn bundle_unpin(&self, o: ::grpc::RequestOptions, p: super::node::BundleUnpinRequest) -> ::grpc::SingleResponse<super::node::BundleUnpinResponse> {
        self.grpc_client.call_unary(o, p, self.method_BundleUnpin.clone())
    }

    fn collection_create(&self, o: ::grpc::RequestOptions, p: super::node::CollectionCreateRequest) -> ::grpc::SingleResponse<super::node::CollectionCreateResponse> {
        self.grpc_client.call_unary(o, p, self.method_CollectionCreate.clone())
    }

    fn collection_search(&self, o: ::grpc::RequestOptions, p: super::node::CollectionSearchRequest) -> ::grpc::SingleResponse<super::node::CollectionSearchResponse> {
        self.grpc_client.call_unary(o, p, self.method_CollectionSearch.clone())
    }

    fn collection_update(&self, o: ::grpc::RequestOptions, p: super::node::CollectionUpdateRequest) -> ::grpc::SingleResponse<super::node::CollectionUpdateResponse> {
        self.grpc_client.call_unary(o, p, self.method_CollectionUpdate.clone())
    }

    fn record_get(&self, o: ::grpc::RequestOptions, p: super::node::RecordGetRequest) -> ::grpc::SingleResponse<super::node::RecordGetResponse> {
        self.grpc_client.call_unary(o, p, self.method_RecordGet.clone())
    }

    fn syndicate(&self, o: ::grpc::RequestOptions, p: super::node::SyndicateRequest) -> ::grpc::SingleResponse<super::node::SyndicateResponse> {
        self.grpc_client.call_unary(o, p, self.method_Syndicate.clone())
    }
}

// server

pub struct NodeServer;


impl NodeServer {
    pub fn new_service_def<H : Node + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Node",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/ArchiveHashGet".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.archive_hash_get(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/ArchiveStream".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.archive_stream(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/BundleCreate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.bundle_create(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/BundleDelete".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.bundle_delete(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/BundlePin".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.bundle_pin(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/BundleSearch".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.bundle_search(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/BundleUnpin".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.bundle_unpin(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/CollectionCreate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.collection_create(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/CollectionSearch".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.collection_search(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/CollectionUpdate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.collection_update(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/RecordGet".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.record_get(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Node/Syndicate".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.syndicate(o, p))
                    },
                ),
            ],
        )
    }
}
