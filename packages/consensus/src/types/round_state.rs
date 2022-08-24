use kai_proto::types::Block;
use kai_types::{part_set::PartSet, proposal::Proposal, round::RoundStep};
use std::fmt::Debug;

pub trait RoundState: Debug + Sync + Send + 'static {}

#[derive(Debug, Clone)]
pub struct RoundStateImpl {
    pub height: u64,
    pub round: u32,
    pub step: RoundStep,
    pub start_time: u64,

    pub commit_time: u64,
    // pub validators: ValidatorSet
    pub proposal: Option<Proposal>,
    pub proposal_block: Option<Block>,
    pub proposal_block_parts: Option<PartSet>,
    pub locked_round: u32,
    pub locked_block: Option<Block>,
    // pub locked_block_parts: Option<PartSet>,
    pub valid_round: u32,
    pub valid_block: Option<Block>,
    // pub valid_block_parts: Option<PartSet>,
    // pub votes: Option<HeightVoteSet>,
    pub commit_round: u32,
    // pub last_commit: Option<VoteSet>,
    // pub last_validators: Option<ValidatorSet>,
}

impl RoundState for RoundStateImpl {}

impl RoundStateImpl {
    pub fn new() -> Self {
        Self {
            height: todo!(),
            round: todo!(),
            step: todo!(),
            start_time: todo!(),
            commit_time: todo!(),
            proposal: todo!(),
            proposal_block_parts: None,
            proposal_block: todo!(),
            locked_round: todo!(),
            locked_block: todo!(),
            valid_round: todo!(),
            valid_block: todo!(),
            commit_round: todo!(),
        }
    }
}