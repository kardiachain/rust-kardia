use crate::{block::BlockId, types::SignedMsgType};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(uint32, tag = "3")]
    pub round: u32,
    /// zero if vote is nil.
    #[prost(message, optional, tag = "4")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "6")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "7")]
    pub validator_index: u32,
    #[prost(bytes = "vec", tag = "8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}

impl From<kai_proto::types::Vote> for Vote {
    fn from(m: kai_proto::types::Vote) -> Self {
        Self {
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            }.into(),
            height: m.height,
            round: m.round,
            block_id: m.block_id.map(|x| x.into()),
            timestamp: m.timestamp,
            validator_address: m.validator_address,
            validator_index: m.validator_index,
            signature: m.signature,
        }
    }
}

impl Into<kai_proto::types::Vote> for Vote {
    fn into(self) -> kai_proto::types::Vote {
        kai_proto::types::Vote {
            r#type: self.r#type.into(),
            height: self.height,
            round: self.round,
            block_id: self.block_id.map(|x| x.into()),
            timestamp: self.timestamp,
            validator_address: self.validator_address,
            validator_index: self.validator_index,
            signature: self.signature,
        }
    }
}

pub fn is_valid_vote_type(t: SignedMsgType) -> bool {
    match t {
        SignedMsgType::Precommit => true,
        SignedMsgType::Prevote => true,
        _ => false,
    }
}
