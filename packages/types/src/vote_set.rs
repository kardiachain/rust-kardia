use std::{collections::HashMap, fmt::Debug};

use ethereum_types::Address;

use crate::{
    bit_array::BitArray,
    block::BlockId,
    commit::{Commit, CommitSig},
    consensus::state::ChainId,
    errors::{AddVoteError, MakeCommitError},
    evidence::DuplicateVoteEvidence,
    peer::PeerId,
    types::SignedMsgType,
    validator_set::ValidatorSet,
    vote::Vote,
};

/// Votes for a particular block
/// There are two ways a BlockVotes gets created for a block key.
/// 1. first (non-conflicting) vote of a validator w/ blockKey (peerMaj23=false)
/// 2. A peer claims to have a 2/3 majority w/ blockKey (peerMaj23=true)
#[derive(Debug, Clone, PartialEq)]
pub struct BlockVotes {
    peer_maj23: bool,         // peer claims to have maj23
    bit_array: BitArray,      // valIndex -> hasVote?
    votes: Vec<Option<Vote>>, // valIndex -> *Vote
    sum: u64,                 // vote sum
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
        match self.votes.get_mut(validator_index) {
            Some(existing) => {
                self.bit_array.set_index(validator_index, true);
                *existing = Some(vote);
                self.sum += voting_power
            }
            _ => (),
        }
    }

    fn get_by_index(&self, index: usize) -> Option<Vote> {
        self.votes.get(index).unwrap().clone()
    }
}

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
    pub votes_by_block: HashMap<String, BlockVotes>, // string(blockHash|blockParts) -> blockVotes
    pub peer_maj23s: HashMap<PeerId, BlockId>,       // maj23 for each peer
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

    pub fn get_vote(&self, validator_index: u32, block_key: String) -> Option<Vote> {
        if let Some(existing_vote) = self.votes.get(validator_index as usize).unwrap() {
            if existing_vote
                .clone()
                .block_id
                .is_some_and(|bid| bid.key() == block_key)
            {
                return Some(existing_vote.clone());
            }
        }

        if let Some(existing_vote) = self
            .votes_by_block
            .get(&block_key)
            .and_then(|bv| bv.get_by_index(validator_index as usize))
        {
            return Some(existing_vote);
        }

        return None;
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
            Err(e) => match e {
                AddVoteError::ConflictingVote(conflicting_vote) => {
                    return Err(AddVoteError::NewConflictingVoteError(
                        DuplicateVoteEvidence {
                            vote_a: Some(conflicting_vote),
                            vote_b: Some(vote),
                            total_voting_power: self.validator_set.total_voting_power as i64,
                            validator_power: iv.clone().unwrap().1.voting_power as i64,
                            timestamp: None,
                        },
                    ))
                }
                _ => panic!("unexpected error"),
            },
        }
    }

    pub fn has_two_thirds_majority(&self) -> bool {
        return self.maj23.is_some();
    }

    pub fn make_commit(&self) -> Result<Commit, MakeCommitError> {
        if self.maj23.is_none() {
            return Err(MakeCommitError::NotEnoughMajority());
        }

        let mut commit_sigs: Vec<CommitSig> = vec![];
        for vote in self.votes.clone().into_iter() {
            let mut commit_sig = if vote.clone().is_some() {
                vote.clone().unwrap().commit_sig()
            } else {
                CommitSig::new_commit_sig_absent()
            };

            if commit_sig.for_block() && !vote.unwrap().block_id.eq(&self.maj23) {
                commit_sig = CommitSig::new_commit_sig_absent()
            }

            commit_sigs.push(commit_sig);
        }

        return Ok(Commit {
            height: self.height,
            round: self.round,
            block_id: self.maj23.clone(),
            signatures: commit_sigs,
        });
    }

    /// assumes signature is valid.
    /// if conflicting vote exists, returns it.
    fn add_verified_vote(
        &mut self,
        vote: Vote,
        block_key: String,
        voting_power: u64,
    ) -> Result<Option<Vote>, AddVoteError> {
        let mut conflicting_vote: Option<Vote> = None;
        let validator_index = vote.validator_index;

        // Already exists in voteSet.votes?
        if let Some(existing_vote) = self.votes.get_mut(validator_index as usize) {
            if existing_vote.is_some() {
                if existing_vote
                    .clone()
                    .unwrap()
                    .block_id
                    .eq(&vote.clone().block_id)
                {
                    panic!("add_verified_vote does not expect duplicate votes")
                } else {
                    conflicting_vote = Some(existing_vote.clone().unwrap());
                }
                // Replace vote if blockKey matches voteSet.maj23.
                if self
                    .maj23
                    .is_some_and(|maj23| maj23.key() == block_key.clone())
                {
                    *existing_vote = Some(vote.clone());
                    self.votes_bit_array
                        .set_index(validator_index as usize, true);
                }
                // otherwise don't add it to voteSet.votes
            } else {
                // add to voteSet.votes and incr .sum
                *existing_vote = Some(vote.clone());
                self.votes_bit_array
                    .set_index(validator_index as usize, true);
                self.sum += voting_power;
            }
        }

        let votes_by_block = self.votes_by_block.get(&block_key.clone());
        if votes_by_block.is_some() {
            if conflicting_vote.is_some() && votes_by_block.unwrap().peer_maj23 {
                // There's a conflict and no peer claims that this block is special.
                return Err(AddVoteError::ConflictingVote(conflicting_vote.unwrap()));
            }
            // We'll add the vote in a bit.
        } else {
            // .votesByBlock doesn't exist...
            if conflicting_vote.is_some() {
                // ... and there's a conflicting vote.
                // We're not even tracking this blockKey, so just forget it.
                return Err(AddVoteError::ConflictingVote(conflicting_vote.unwrap()));
            }
            // ... and there's no conflicting vote.
            // Start tracking this blockKey
            self.votes_by_block.insert(
                block_key.clone(),
                BlockVotes::new(false, self.validator_set.size()),
            );
            // We'll add the vote in a bit.
        }

        // votes by block must exist
        if let Some(votes_by_block) = self.votes_by_block.get_mut(&block_key.clone()) {
            // Before adding to votesByBlock, see if we'll exceed quorum
            let orig_sum = votes_by_block.sum.clone();
            let quorum = self.validator_set.total_voting_power * 2 / 3 + 1;

            // Add vote to votesByBlock
            votes_by_block.add_verified_vote(vote.clone(), voting_power);

            // If we just crossed the quorum threshold and have 2/3 majority...
            if orig_sum < quorum && quorum <= votes_by_block.sum {
                // Only consider the first quorum reached
                if self.maj23.is_none() {
                    let maj23_block_id = vote.clone().block_id;
                    self.maj23 = maj23_block_id;
                    // And also copy votes over to voteSet.votes
                    for (index, vote) in votes_by_block.votes.iter().enumerate() {
                        if vote.is_some() {
                            self.votes.insert(index, vote.clone());
                        }
                    }
                }
            }
        }

        return Ok(conflicting_vote);
    }
}

pub trait VoteSetReader: Debug + Sync + Send + 'static {}

impl VoteSetReader for VoteSet {}
