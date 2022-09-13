use crate::{
    block::{BlockId, BlockIdFlag},
    vote_set::VoteSetReader,
};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub round: u32,
    #[prost(message, optional, tag = "3")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, repeated, tag = "4")]
    pub signatures: ::prost::alloc::vec::Vec<CommitSig>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitSig {
    #[prost(enumeration = "BlockIdFlag", tag = "1")]
    pub block_id_flag: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}

impl VoteSetReader for Commit {}