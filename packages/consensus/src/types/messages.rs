use super::error::{DecodeProtoError, EncodeProtoError};
use kai_proto::consensus::{message::Sum, Message as ConsensusMessageProto};
use std::error::Error;

/**
   Message is a message that can be sent and received on the `ConsensusReactor`
*/
pub trait Message {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>>;
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError>;
}

pub fn msg_from_proto(
    msg_proto: ConsensusMessageProto,
) -> Result<Box<dyn Message>, DecodeProtoError> {
    if let Some(sum) = msg_proto.sum {
        match sum {
            Sum::NewRoundStep(m) => Ok(Box::new(NewRoundStepMessage::from(m))),
            Sum::NewValidBlock(m) => Ok(Box::new(NewValidBlockMessage::from(m))),
            Sum::Proposal(m) => Ok(Box::new(ProposalMessage::from(m))),
            Sum::ProposalPol(m) => Ok(Box::new(ProposalPOLMessage::from(m))),
            Sum::BlockPart(m) => Ok(Box::new(BlockPartMessage::from(m))),
            Sum::Vote(m) => Ok(Box::new(VoteMessage::from(m))),
            Sum::HasVote(m) => Ok(Box::new(HasVoteMessage::from(m))),
            Sum::VoteSetMaj23(m) => Ok(Box::new(VoteSetMaj23Message::from(m))),
            Sum::VoteSetBits(m) => Ok(Box::new(VoteSetBitsMessage::from(m))),
            _ => Err(DecodeProtoError.into()),
        }
    } else {
        Err(DecodeProtoError.into())
    }
}

#[derive(Debug, Clone)]
pub struct NewRoundStepMessage {
    pub height: u64,
    pub round: u32,
    pub step: u32,
    pub seconds_since_start_time: u64,
    pub last_commit_round: u32,
}

impl Message for NewRoundStepMessage {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::NewRoundStep(NewRoundStepMessage::into(self.clone()))),
        })
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

impl Into<kai_proto::consensus::NewRoundStep> for NewRoundStepMessage {
    fn into(self: Self) -> kai_proto::consensus::NewRoundStep {
        kai_proto::consensus::NewRoundStep {
            height: self.height,
            round: self.round,
            step: self.step,
            seconds_since_start_time: self.seconds_since_start_time,
            last_commit_round: self.last_commit_round,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewValidBlockMessage {
    pub height: u64,
    pub round: u32,
    pub block_part_set_header: Option<kai_proto::types::PartSetHeader>,
    pub block_parts: Option<kai_proto::types::BitArray>,
    pub is_commit: bool,
}

impl Message for NewValidBlockMessage {
    fn validate_basic(&self) -> Result<(), Box<(dyn Error)>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::NewValidBlock(NewValidBlockMessage::into(self.clone()))),
        })
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

impl Into<kai_proto::consensus::NewValidBlock> for NewValidBlockMessage {
    fn into(self: Self) -> kai_proto::consensus::NewValidBlock {
        kai_proto::consensus::NewValidBlock {
            height: self.height,
            round: self.round,
            block_part_set_header: self.block_part_set_header,
            block_parts: self.block_parts,
            is_commit: self.is_commit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasVoteMessage {
    pub height: u64,
    pub round: u32,
    pub r#type: i32,
    pub index: u32,
}

impl Message for HasVoteMessage {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::HasVote(HasVoteMessage::into(self.clone()))),
        })
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

impl Into<kai_proto::consensus::HasVote> for HasVoteMessage {
    fn into(self: Self) -> kai_proto::consensus::HasVote {
        kai_proto::consensus::HasVote {
            height: self.height,
            round: self.round,
            r#type: self.r#type,
            index: self.index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoteSetMaj23Message {
    pub height: u64,
    pub round: u32,
    pub r#type: i32,
    pub block_id: Option<kai_proto::types::BlockId>,
}

impl Message for VoteSetMaj23Message {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::VoteSetMaj23(VoteSetMaj23Message::into(self.clone()))),
        })
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

impl Into<kai_proto::consensus::VoteSetMaj23> for VoteSetMaj23Message {
    fn into(self: Self) -> kai_proto::consensus::VoteSetMaj23 {
        kai_proto::consensus::VoteSetMaj23 {
            height: self.height,
            round: self.round,
            r#type: self.r#type,
            block_id: self.block_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProposalMessage {
    pub proposal: Option<kai_proto::types::Proposal>,
}

impl Message for ProposalMessage {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::Proposal(ProposalMessage::into(self.clone()))),
        })
    }
}

impl From<kai_proto::consensus::Proposal> for ProposalMessage {
    fn from(m: kai_proto::consensus::Proposal) -> Self {
        Self {
            proposal: m.proposal,
        }
    }
}

impl Into<kai_proto::consensus::Proposal> for ProposalMessage {
    fn into(self: Self) -> kai_proto::consensus::Proposal {
        kai_proto::consensus::Proposal {
            proposal: self.proposal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProposalPOLMessage {
    pub height: u64,
    pub proposal_pol_round: u32,
    pub proposal_pol: Option<kai_proto::types::BitArray>,
}

impl Message for ProposalPOLMessage {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::ProposalPol(ProposalPOLMessage::into(self.clone()))),
        })
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

impl Into<kai_proto::consensus::ProposalPol> for ProposalPOLMessage {
    fn into(self: Self) -> kai_proto::consensus::ProposalPol {
        kai_proto::consensus::ProposalPol {
            height: self.height,
            proposal_pol_round: self.proposal_pol_round,
            proposal_pol: self.proposal_pol,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockPartMessage {
    pub height: u64,
    pub round: u32,
    pub part: Option<kai_proto::types::Part>,
}

impl Message for BlockPartMessage {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::BlockPart(BlockPartMessage::into(self.clone()))),
        })
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
impl Into<kai_proto::consensus::BlockPart> for BlockPartMessage {
    fn into(self: Self) -> kai_proto::consensus::BlockPart {
        kai_proto::consensus::BlockPart {
            height: self.height,
            round: self.round,
            part: self.part,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoteMessage {
    pub vote: Option<kai_proto::types::Vote>,
}

impl Message for VoteMessage {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::Vote(VoteMessage::into(self.clone()))),
        })
    }
}

impl From<kai_proto::consensus::Vote> for VoteMessage {
    fn from(m: kai_proto::consensus::Vote) -> Self {
        Self { vote: m.vote }
    }
}
impl Into<kai_proto::consensus::Vote> for VoteMessage {
    fn into(self: Self) -> kai_proto::consensus::Vote {
        kai_proto::consensus::Vote { vote: self.vote }
    }
}

#[derive(Debug, Clone)]
pub struct VoteSetBitsMessage {
    pub height: u64,
    pub round: u32,
    pub r#type: i32,
    pub block_id: Option<kai_proto::types::BlockId>,
    pub votes: Option<kai_proto::types::BitArray>,
}

impl Message for VoteSetBitsMessage {
    fn validate_basic(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, EncodeProtoError> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::VoteSetBits(VoteSetBitsMessage::into(self.clone()))),
        })
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

impl Into<kai_proto::consensus::VoteSetBits> for VoteSetBitsMessage {
    fn into(self: Self) -> kai_proto::consensus::VoteSetBits {
        kai_proto::consensus::VoteSetBits {
            height: self.height,
            round: self.round,
            r#type: self.r#type,
            block_id: self.block_id,
            votes: self.votes,
        }
    }
}
