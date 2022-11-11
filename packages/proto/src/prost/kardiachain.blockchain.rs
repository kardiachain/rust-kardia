/// BlockRequest requests a block for a specific height
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRequest {
    #[prost(uint64, tag="1")]
    pub height: u64,
}
/// NoBlockResponse informs the node that the peer does not have block at the requested height
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoBlockResponse {
    #[prost(uint64, tag="1")]
    pub height: u64,
}
/// BlockResponse returns block to the requested
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockResponse {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<super::types::Block>,
}
/// StatusRequest requests the status of a peer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
}
/// StatusResponse is a peer response to inform their status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(uint64, tag="2")]
    pub base: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof="message::Sum", tags="1, 2, 3, 4, 5")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag="1")]
        BlockRequest(super::BlockRequest),
        #[prost(message, tag="2")]
        NoBlockResponse(super::NoBlockResponse),
        #[prost(message, tag="3")]
        BlockResponse(super::BlockResponse),
        #[prost(message, tag="4")]
        StatusRequest(super::StatusRequest),
        #[prost(message, tag="5")]
        StatusResponse(super::StatusResponse),
    }
}
