use crate::block::BlockId;
use kai_proto::types::SignedMsgType;
use prost_types::Timestamp;

#[derive(Debug, Clone)]
pub struct Vote {
    pub r#type: SignedMsgType,
    pub height: u64,
    pub round: u32,
    /// zero if vote is nil.
    pub block_id: Option<BlockId>,
    pub timestamp: Option<Timestamp>,
    pub validator_address: Vec<u8>,
    pub validator_index: u32,
    pub signature: Vec<u8>,
}

impl From<kai_proto::types::Vote> for Vote {
    fn from(m: kai_proto::types::Vote) -> Self {
        Self {
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            },
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
