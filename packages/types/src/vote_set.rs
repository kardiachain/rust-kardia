use std::fmt::Debug;

use crate::{bit_array::BitArray, block::BlockId, commit::Commit, vote::Vote};

pub trait VoteSetReader: Debug + Sync + Send + 'static {}

#[derive(Debug, Clone, PartialEq)]
pub struct VoteSet {
    pub maj23: Option<BlockId>,
}

impl VoteSet {
    pub fn bit_array(&self) -> Option<BitArray> {
        todo!()
    }

    pub fn bit_array_by_block_id(&self, block_id: BlockId) -> Option<BitArray> {
        todo!()
    }

    /// returns block id that has +2/3 votes
    pub fn two_thirds_majority(&self) -> Option<BlockId> {
        return self.maj23.clone();
    }

    pub fn make_commit(&self) -> Option<Commit> {
        todo!()
    }

    pub fn add_vote(&mut self, vote: Vote) -> Result<(), String> {
        todo!()
    }
}

impl VoteSetReader for VoteSet {}
