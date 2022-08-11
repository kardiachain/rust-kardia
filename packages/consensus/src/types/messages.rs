use std::error::Error;

/**
   Message is a message that can be sent and received on the `ConsensusReactor`
*/
pub trait Message {
    fn validate_basic() -> Result<(), Box<dyn Error>>;
}

pub struct NewRoundStepMessage {
    pub height: u64,
    pub round: u32,
    pub step: u32,
    pub seconds_since_start_time: u64,
    pub last_commit_round: u32,
}

impl Message for NewRoundStepMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::NewRoundStep> for NewRoundStepMessage {
    fn from(m: kai_proto::consensus::NewRoundStep) -> Self {
        Self {
            height: m.height,
            round: m.round,
            step: m.step,
            seconds_since_start_time: m.seconds_since_start_time,
            last_commit_round: m.last_commit_round,
        }
    }
}

pub struct NewValidBlockMessage {
    pub height: u64,
    pub round: u32,
    pub block_part_set_header: Option<kai_proto::types::PartSetHeader>,
    pub block_parts: Option<kai_proto::types::BitArray>,
    pub is_commit: bool,
}

impl Message for NewValidBlockMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::NewValidBlock> for NewValidBlockMessage {
    fn from(m: kai_proto::consensus::NewValidBlock) -> Self {
        Self {
            height: m.height,
            round: m.round,
            block_part_set_header: m.block_part_set_header,
            block_parts: m.block_parts,
            is_commit: m.is_commit,
        }
    }
}

pub struct HasVoteMessage {
    pub height: u64,
    pub round: u32,
    pub r#type: i32,
    pub index: u32,
}

impl Message for HasVoteMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::HasVote> for HasVoteMessage {
    fn from(m: kai_proto::consensus::HasVote) -> Self {
        Self {
            height: m.height,
            round: m.round,
            r#type: m.r#type,
            index: m.index,
        }
    }
}

pub struct VoteSetMaj23Message {
    pub height: u64,
    pub round: u32,
    pub r#type: i32,
    pub block_id: Option<kai_proto::types::BlockId>,
}

impl Message for VoteSetMaj23Message {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::VoteSetMaj23> for VoteSetMaj23Message {
    fn from(m: kai_proto::consensus::VoteSetMaj23) -> Self {
        Self {
            height: m.height,
            round: m.round,
            r#type: m.r#type,
            block_id: m.block_id,
        }
    }
}

pub struct ProposalMessage {
    pub proposal: Option<kai_proto::types::Proposal>,
}

impl Message for ProposalMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::Proposal> for ProposalMessage {
    fn from(m: kai_proto::consensus::Proposal) -> Self {
        Self {
            proposal: m.proposal,
        }
    }
}

pub struct ProposalPOLMessage {
    pub height: u64,
    pub proposal_pol_round: u32,
    pub proposal_pol: Option<kai_proto::types::BitArray>,
}

impl Message for ProposalPOLMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::ProposalPol> for ProposalPOLMessage {
    fn from(m: kai_proto::consensus::ProposalPol) -> Self {
        Self {
            height: m.height,
            proposal_pol_round: m.proposal_pol_round,
            proposal_pol: m.proposal_pol,
        }
    }
}

pub struct BlockPartMessage {
    pub height: u64,
    pub round: u32,
    pub part: Option<kai_proto::types::Part>,
}

impl Message for BlockPartMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::BlockPart> for BlockPartMessage {
    fn from(m: kai_proto::consensus::BlockPart) -> Self {
        Self {
            height: m.height,
            round: m.round,
            part: m.part,
        }
    }
}

pub struct VoteMessage {
    pub vote: Option<kai_proto::types::Vote>,
}

impl Message for VoteMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::Vote> for VoteMessage {
    fn from(m: kai_proto::consensus::Vote) -> Self {
        Self {
            vote: m.vote,
        }
    }
}

pub struct VoteSetBitsMessage {
    pub height: u64,
    pub round: u32,
    pub r#type: i32,
    pub block_id: Option<kai_proto::types::BlockId>,
    pub votes: Option<kai_proto::types::BitArray>,
}

impl Message for VoteSetBitsMessage {
    fn validate_basic() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl From<kai_proto::consensus::VoteSetBits> for VoteSetBitsMessage {
    fn from(m: kai_proto::consensus::VoteSetBits) -> Self {
        Self {
            height: m.height,
            round: m.round,
            r#type: m.r#type,
            block_id: m.block_id,
            votes: m.votes,
        }
    }
}
