use crate::block::BlockId;
use prost_types::Timestamp;

pub struct Proposal {
    pub r#type: i32,
    pub height: u64,
    pub round: u32,
    pub pol_round: u32,
    pub block_id: Option<BlockId>,
    pub timestamp: Option<Timestamp>,
    pub signature: Vec<u8>,
}