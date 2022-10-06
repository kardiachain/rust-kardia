use std::fmt::Debug;

use crate::{
    bit_array::BitArray, block::BlockId, commit::Commit, consensus::state::ChainId,
    types::SignedMsgType, validator_set::ValidatorSet, vote::Vote,
};

pub trait VoteSetReader: Debug + Sync + Send + 'static {}

#[derive(Debug, Clone, PartialEq)]
pub struct VoteSet {
    pub chain_id: ChainId,
    pub height: u64,
    pub round: u32,
    pub validator_set: ValidatorSet,
    pub votes_bit_array: BitArray,
    pub votes: Vec<Option<Vote>>,
    pub sum: u64,
    pub r#type: SignedMsgType,
    pub maj23: Option<BlockId>,
    // votesByBlock  map[string]*blockVotes // string(blockHash|blockParts) -> blockVotes
    // peerMaj23s    map[p2p.ID]BlockID     // Maj23 for each peer
}

impl VoteSet {
    pub fn new(
        chain_id: ChainId,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
        validator_set: ValidatorSet,
    ) -> Self {
        Self {
            chain_id: chain_id,
            height: height,
            round: round,
            r#type: signed_msg_type,
            validator_set: validator_set.clone(),
            votes_bit_array: BitArray::new_bit_array(validator_set.clone().validators.len()),
            votes: vec![None; validator_set.validators.clone().len()],
            sum: 0,
            maj23: None,
        }
    }

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
