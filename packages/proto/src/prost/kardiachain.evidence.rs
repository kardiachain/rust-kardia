#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    #[prost(message, repeated, tag="1")]
    pub evidence: ::prost::alloc::vec::Vec<super::types::Evidence>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    #[prost(message, optional, tag="1")]
    pub evidence: ::core::option::Option<super::types::Evidence>,
    #[prost(message, optional, tag="2")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="3")]
    pub validators: ::prost::alloc::vec::Vec<super::types::Validator>,
    #[prost(int64, tag="4")]
    pub total_voting_power: i64,
}
