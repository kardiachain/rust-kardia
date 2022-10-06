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

    pub fn add_vote(&mut self, vote: Vote, peer_id: PeerId) -> Result<(), String> {
        let vote_set = match self.get_mut_vote_set(vote.round, vote.r#type()) {
            Some(vs) => vs,
            None => {
                let rounds = self.peer_catchup_rounds.get_mut(&peer_id);
                if rounds.is_none() || rounds.is_some_and(|r| r.len() < 2) {
                    if rounds.is_none() {
                        self.peer_catchup_rounds.insert(peer_id, vec![vote.round]);
                    } else {
                        rounds.unwrap().push(vote.round);
                    }
                    // init vote set for round
                    self.add_round(vote.round);
                    // return newly created vote set
                    self.get_mut_vote_set(vote.round, vote.r#type()).unwrap()
                } else {
                    return Err("ErrGotVoteFromUnwantedRound".to_owned());
                }
            }
        };

        return vote_set.add_vote(vote);
    }

    fn get_mut_vote_set(
        &mut self,
        round: u32,
        signed_msg_type: SignedMsgType,
    ) -> Option<&mut VoteSet> {
        if let Some(rvs) = self.round_vote_sets.get_mut(&round) {
            match signed_msg_type {
                SignedMsgType::Prevote => rvs.prevotes.as_mut(),
                SignedMsgType::Precommit => rvs.precommits.as_mut(),
                _ => None,
            }
        } else {
            None
        }
    }

    fn add_round(&mut self, round: u32) {
        let prevotes = VoteSet::new(
            self.chain_id.clone(),
            self.height,
            round,
            SignedMsgType::Prevote,
            self.validator_set.clone().unwrap(),
        );
        let precommits = VoteSet::new(
            self.chain_id.clone(),
            self.height,
            round,
            SignedMsgType::Precommit,
            self.validator_set.clone().unwrap(),
        );
        self.round_vote_sets.insert(
            round,
            RoundVoteSet {
                prevotes: Some(prevotes),
                precommits: Some(precommits),
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct RoundVoteSet {
    pub prevotes: Option<VoteSet>,
    pub precommits: Option<VoteSet>,
}
