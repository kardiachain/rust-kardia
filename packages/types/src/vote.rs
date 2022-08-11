use crate::block::BlockId;
use prost_types::Timestamp;

pub struct Vote {
    pub r#type: i32,
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
            r#type: m.r#type,
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
