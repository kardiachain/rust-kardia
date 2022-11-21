use kai_proto::types::SignedMsgType;

use crate::{
    block::BlockId, consensus::state::ChainId, part_set::PartSetHeader, proposal::Proposal,
    vote::Vote,
};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalProposal {
    /// type alias for byte
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// canonicalization requires fixed size encoding here
    #[prost(uint32, tag = "3")]
    pub round: u32,
    #[prost(uint32, tag = "4")]
    pub pol_round: u32,
    #[prost(message, optional, tag = "5")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "7")]
    pub chain_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalBlockId {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub part_set_header: ::core::option::Option<CanonicalPartSetHeader>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalPartSetHeader {
    #[prost(uint32, tag = "1")]
    pub total: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalVote {
    /// type alias for byte
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// canonicalization requires fixed size encoding here
    #[prost(uint32, tag = "3")]
    pub round: u32,
    #[prost(message, optional, tag = "4")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "6")]
    pub chain_id: ::prost::alloc::string::String,
}

pub fn canonicalize_block_id(bid: BlockId) -> Option<CanonicalBlockId> {
    if bid.is_zero() {
        None
    } else {
        Some(CanonicalBlockId {
            hash: bid.hash,
            part_set_header: bid
                .part_set_header
                .and_then(|psh| canonicalize_part_set_header(psh)),
        })
    }
}

pub fn canonicalize_part_set_header(psh: PartSetHeader) -> Option<CanonicalPartSetHeader> {
    Some(CanonicalPartSetHeader {
        total: psh.total,
        hash: psh.hash,
    })
}

pub fn create_canonical_proposal(chain_id: ChainId, proposal: Proposal) -> Option<CanonicalProposal> {
    Some(CanonicalProposal {
        r#type: SignedMsgType::Proposal.into(),
        height: proposal.height,
        round: proposal.round,
        pol_round: proposal.pol_round,
        block_id: proposal.block_id.and_then(|bid| canonicalize_block_id(bid)),
        timestamp: proposal.timestamp,
        chain_id: chain_id,
    })
}

pub fn create_canonical_vote(chain_id: ChainId, vote: Vote) -> Option<CanonicalVote> {
    Some(CanonicalVote {
        r#type: SignedMsgType::Prevote.into(),
        height: vote.height,
        round: vote.round,
        block_id: vote.block_id.and_then(|bid| canonicalize_block_id(bid)),
        timestamp: vote.timestamp,
        chain_id: chain_id,
    })
}
