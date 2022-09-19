use kai_types::{
    block::Block, consensus::height_vote_set::HeightVoteSet, part_set::PartSet, proposal::Proposal,
    round::RoundStep, validator_set::ValidatorSet, vote_set::VoteSet,
};

use std::{collections::{HashMap, HashSet}, fmt::Debug};

pub type RULE_NUMBER = u8;
pub const RULE_4: RULE_NUMBER = 4;

#[derive(Debug, Clone)]
pub struct RoundState {
    pub height: u64,
    pub round: u32,
    pub step: RoundStep,
    pub start_time: u64,

    pub commit_time: u64,
    pub validators: Option<ValidatorSet>,
    pub proposal: Option<Proposal>,
    pub proposal_block: Option<Block>,
    pub proposal_block_parts: Option<PartSet>,
    pub locked_round: u32,
    pub locked_block: Option<Block>,
    pub locked_block_parts: Option<PartSet>,
    pub valid_round: u32,
    pub valid_block: Option<Block>,
    pub valid_block_parts: Option<PartSet>,
    pub votes: Option<HeightVoteSet>,
    pub commit_round: u32,
    pub last_commit: Option<VoteSet>,

    /// a map from rule number to boolean.
    /// true if rule has been trigger or else.
    pub triggered_rules: HashSet<RULE_NUMBER>,

    // pub last_validators: Option<ValidatorSet>,
}

impl RoundState {
    pub fn new_default() -> Self {
        Self {
            height: 1,
            round: 1,
            step: RoundStep::Propose,
            start_time: 0,
            commit_time: 0,
            validators: None,
            proposal: None,
            proposal_block: None,
            proposal_block_parts: None,
            locked_round: 0,
            locked_block: None,
            locked_block_parts: None,
            valid_round: 0,
            valid_block: None,
            valid_block_parts: None,
            commit_round: 0,
            last_commit: None,
            votes: None,
            triggered_rules: HashSet::new(),
        }
    }
}
