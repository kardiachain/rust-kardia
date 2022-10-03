use std::collections::HashMap;

use crate::{
    peer::PeerId, types::SignedMsgType, validator_set::ValidatorSet, vote::Vote, vote_set::VoteSet,
};

#[derive(Debug, Clone)]
pub struct HeightVoteSet {
    pub chain_id: String,
    pub height: u64,
    pub round: u32,

    pub validator_set: Option<ValidatorSet>,
    pub round_vote_sets: HashMap<u32, RoundVoteSet>,
    pub peer_catchup_rounds: HashMap<PeerId, Vec<u32>>,
}

impl HeightVoteSet {
    pub fn prevotes(&self, round: u32) -> Option<VoteSet> {
        self.get_vote_set(round, SignedMsgType::Prevote)
    }

    pub fn precommits(&self, round: u32) -> Option<VoteSet> {
        self.get_vote_set(round, SignedMsgType::Precommit)
    }

    pub fn get_vote_set(&self, round: u32, signed_msg_type: SignedMsgType) -> Option<VoteSet> {
        if let Some(rvs) = self.round_vote_sets.get(&round) {
            match signed_msg_type {
                SignedMsgType::Prevote => rvs.prevotes.clone(),
                SignedMsgType::Precommit => rvs.precommits.clone(),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn set_peer_maj23(&self) -> Result<(), String> {
        todo!()
    }

    pub fn add_vote(&mut self, vote: Vote) -> Result<(), String> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct RoundVoteSet {
    pub prevotes: Option<VoteSet>,
    pub precommits: Option<VoteSet>,
}
