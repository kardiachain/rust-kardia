use bytes::Bytes;
use prost::Message;

use crate::{
    block::BlockId,
    canonical_types::{create_canonical_proposal, CanonicalProposal},
    consensus::state::ChainId,
};
use kai_proto::types::SignedMsgType;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(uint32, tag = "3")]
    pub round: u32,
    #[prost(uint32, tag = "4")]
    pub pol_round: u32,
    #[prost(message, optional, tag = "5")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}

impl From<kai_proto::types::Proposal> for Proposal {
    fn from(m: kai_proto::types::Proposal) -> Self {
        Self {
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            }
            .into(),
            height: m.height,
            round: m.round,
            pol_round: m.pol_round,
            block_id: m.block_id.map(|x| x.into()),
            timestamp: m.timestamp,
            signature: m.signature,
        }
    }
}

impl Into<kai_proto::types::Proposal> for Proposal {
    fn into(self: Self) -> kai_proto::types::Proposal {
        kai_proto::types::Proposal {
            r#type: self.r#type.into(),
            height: self.height,
            round: self.round,
            pol_round: self.pol_round,
            block_id: self.block_id.map(|x| x.into()),
            timestamp: self.timestamp,
            signature: self.signature,
        }
    }
}

pub fn proposal_sign_bytes(chain_id: ChainId, proposal: Proposal) -> Option<Bytes> {
    create_canonical_proposal(chain_id, proposal)
        .map(|cp| Bytes::copy_from_slice(&cp.encode_length_delimited_to_vec()))
}
