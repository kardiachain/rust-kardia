use std::collections::HashMap;

use crate::{validator_set::ValidatorSet, vote_set::VoteSet, peer::PeerId};

#[derive(Debug, Clone)]
pub struct HeightVoteSet {
    chain_id: String,
    height: u64,
    validator_set: Option<ValidatorSet>,

    round: u32,
    round_vote_sets: HashMap<u32, RoundVoteSet>,
    peer_catchup_rounds: HashMap<PeerId, Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct RoundVoteSet {
    prevotes  : Option<VoteSet>,
	precommits: Option<VoteSet>,
}