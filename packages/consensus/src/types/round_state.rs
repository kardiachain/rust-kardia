use kai_types::block::PartSetHeader;
use std::fmt::Debug;

pub trait RoundState: Debug + Sync + Send + 'static {}

#[derive(Debug, Clone)]
pub struct RoundStateImpl {
    pub proposal_block_parts_header: Option<PartSetHeader>,
}

impl RoundState for RoundStateImpl {}

impl RoundStateImpl {
    pub fn new() -> Self {
        Self {
            proposal_block_parts_header: None,
        }
    }
}
