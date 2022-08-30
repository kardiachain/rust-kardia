use std::collections::HashMap;

use crate::{peer::PeerId, validator_set::ValidatorSet, vote_set::VoteSet};

#[derive(Debug, Clone)]
pub struct HeightVoteSet {
    chain_id: String,
    height: u64,
    validator_set: Option<ValidatorSet>,

    round: u32,
    round_vote_sets: HashMap<u32, RoundVoteSet>,
    peer_catchup_rounds: HashMap<PeerId, Vec<u32>>,
}

impl HeightVoteSet {
    pub fn prevotes(&self, round: u32) -> Option<VoteSet> {
        todo!()
    }
    pub fn precommits(&self, round: u32) -> Option<VoteSet> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct RoundVoteSet {
    prevotes: Option<VoteSet>,
    precommits: Option<VoteSet>,
}
