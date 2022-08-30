use prost_types::Timestamp;

use crate::{block::BlockId, vote_set::VoteSetReader};

#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    pub height: u64,
    pub round: u32,
    pub block_id: Option<BlockId>,
    pub signatures: Vec<CommitSig>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommitSig {
    pub block_id_flag: i32,
    pub validator_address: Vec<u8>,
    pub timestamp: Option<Timestamp>,
    pub signature: Vec<u8>,
}

impl VoteSetReader for Commit {}