use std::{fmt::{format, Debug}, collections::HashMap};

use ethereum_types::Address;

use crate::{
    bit_array::BitArray, block::BlockId, commit::Commit, consensus::state::ChainId,
    errors::AddVoteError, evidence::DuplicateVoteEvidence, types::SignedMsgType,
    validator_set::ValidatorSet, vote::Vote, peer::PeerId,
};

/*
	Votes for a particular block
	There are two ways a *blockVotes gets created for a blockKey.
	1. first (non-conflicting) vote of a validator w/ blockKey (peerMaj23=false)
	2. A peer claims to have a 2/3 majority w/ blockKey (peerMaj23=true)
*/
#[derive(Debug, Clone, PartialEq)]
struct BlockVotes {
    peer_maj23: bool, // peer claims to have maj23
    bit_array: BitArray, // valIndex -> hasVote?
    votes: Vec<Option<Vote>>, // valIndex -> *Vote
    sum: u64, // vote sum
}

impl BlockVotes {
    pub(super) fn new(peer_maj23: bool, num_validators: usize) -> Self {
        Self {
            peer_maj23,
            bit_array: BitArray::new_bit_array(num_validators),
            votes: vec![None; num_validators],
            sum: 0,
        }
    }

    fn add_verified_vote(&mut self, vote: Vote, voting_power: u64) {
        let validator_index = vote.validator_index as usize;
        if let existing = self.votes.get_mut(validator_index).unwrap() {
            self.bit_array.set_index(validator_index, true);
            *existing = Some(vote.clone());
            self.sum += voting_power
        }
    }
    
    fn get_by_index(&self, index: usize) -> Option<Vote> {
        self.votes.get(index).unwrap().clone()
    }
}

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
    pub votes_by_block:  HashMap<String, BlockVotes>, // string(blockHash|blockParts) -> blockVotes
    pub peer_maj23s: HashMap<PeerId, BlockId>, // maj23 for each peer
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
            votes_by_block: HashMap::new(),
            peer_maj23s: HashMap::new(),
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

    pub fn get_vote(&self, validator_index: u32, block_key: String) -> Option<Vote> {
        todo!()
    }

    pub fn add_vote(&mut self, vote: Vote) -> Result<(), AddVoteError> {
        if vote.height != self.height
            || vote.round != self.round
            || vote.r#type != self.r#type.into()
        {
            return Err(AddVoteError::UnexpectedMismatch(
                self.height,
                self.round,
                self.r#type.as_str_name(),
                vote.height,
                vote.round,
                SignedMsgType::from(vote.r#type).as_str_name(),
            ));
        }

        // ensure that signer is a validator
        let iv = self
            .validator_set
            .get_by_address(Address::from_slice(vote.validator_address.as_slice()));

        if iv.is_none() {
            return Err(AddVoteError::ValidatorNotFound(
                String::from_utf8(vote.validator_address).unwrap(),
            ));
        }

        // if we already know of this vote, return.
        if let Some(existing_vote) =
            self.get_vote(vote.validator_index, vote.clone().block_id.unwrap().key())
        {
            if existing_vote.signature.eq(&vote.signature) {
                return Ok(());
            }
            return Err(AddVoteError::NonDeterministicSignature {
                existing: existing_vote,
                new_vote: vote,
            });
        }

        // check signature
        if let Err(e) = vote.verify(self.chain_id.clone(), iv.clone().unwrap().1.address) {
            return Err(AddVoteError::InvalidVote(e));
        }

        // add vote and get conflicting vote if any
        match self.add_verified_vote(
            vote.clone(),
            vote.clone().block_id.unwrap().key(),
            iv.clone().unwrap().1.voting_power,
        ) {
            Ok(_) => return Ok(()),
            Err(conflicting) => {
                return Err(AddVoteError::NewConflictingVoteError(
                    DuplicateVoteEvidence {
                        vote_a: Some(conflicting),
                        vote_b: Some(vote),
                        total_voting_power: self.validator_set.total_voting_power,
                        validator_power: iv.clone().unwrap().1.voting_power,
                        timestamp: None,
                    },
                ))
            }
        }
    }

    fn add_verified_vote(&mut self, vote: Vote, block_key: String, voting_power: i64) -> Result<(), AddVoteError> {
        // TODO: implement
        
        let mut conflicting: Option<Vote> = None;
        let validator_index = vote.validator_index;
    
        let Some(existing) = self.votes.get_mut(validator_index as usize);

        // already exists?
        if existing.is_some() {
            if existing.unwrap().block_id.eq(&vote.clone().block_id) {
                panic!("add_verified_vote does not expect duplicate votes")
            } else {
                conflicting = Some(existing.clone().unwrap());
            }
            // Replace vote if blockKey matches voteSet.maj23.
            if self.maj23.is_some() && self.maj23.unwrap().key() == block_key {
                *existing = Some(vote.clone());
                self.votes_bit_array.set_index(validator_index as usize, true);
            }
            // otherwise don't add it to voteSet.votes
        } else {
            // add to voteSet.votes and incr .sum
            *existing = Some(vote.clone());
            self.votes_bit_array.set_index(validator_index as usize, true);
            self.sum += voting_power as u64;
        }
    
        if let Some(block_votes) = self.votes_by_block.get(&block_key) {
            if conflicting.is_some() && block_votes.peer_maj23 {
			    // There's a conflict and no peer claims that this block is special.
                return Err(AddVoteError::ConflictingVote(conflicting.unwrap()));
            }
		    // We'll add the vote in a bit.
        } else {    
            // .votesByBlock doesn't exist...
            if conflicting != nil {
                // ... and there's a conflicting vote.
                // We're not even tracking this blockKey, so just forget it.
                return false, conflicting
            }
            // ... and there's no conflicting vote.
            // Start tracking this blockKey
            votesByBlock = newBlockVotes(false, voteSet.valSet.Size())
            voteSet.votesByBlock[blockKey] = votesByBlock
            // We'll add the vote in a bit.
        }
    
        // Before adding to votesByBlock, see if we'll exceed quorum
        origSum := votesByBlock.sum
        quorum := voteSet.valSet.TotalVotingPower()*2/3 + 1
    
        // Add vote to votesByBlock
        votesByBlock.addVerifiedVote(vote, votingPower)
    
        // If we just crossed the quorum threshold and have 2/3 majority...
        if origSum < quorum && quorum <= votesByBlock.sum {
            // Only consider the first quorum reached
            if voteSet.maj23 == nil {
                maj23BlockID := vote.BlockID
                voteSet.maj23 = &maj23BlockID
                // And also copy votes over to voteSet.votes
                for i, vote := range votesByBlock.votes {
                    if vote != nil {
                        voteSet.votes[i] = vote
                    }
                }
            }
        }
    
        return true, conflicting

        Ok(())
    }
}

impl VoteSetReader for VoteSet {}
