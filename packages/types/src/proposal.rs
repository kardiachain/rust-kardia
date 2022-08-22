use crate::block::BlockId;
use kai_proto::types::SignedMsgType;
use prost_types::Timestamp;

#[derive(Debug, Clone)]
pub struct Proposal {
    pub r#type: SignedMsgType,
    pub height: u64,
    pub round: u32,
    pub pol_round: u32,
    pub block_id: Option<BlockId>,
    pub timestamp: Option<Timestamp>,
    pub signature: Vec<u8>,
}

impl From<kai_proto::types::Proposal> for Proposal {
    fn from(m: kai_proto::types::Proposal) -> Self {
        Self {
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            },
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
