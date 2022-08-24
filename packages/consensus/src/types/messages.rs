use super::{error::ConsensusReactorError, peer::PeerId};
use kai_proto::{
    consensus::{message::Sum, Message as ConsensusMessageProto},
    types::SignedMsgType,
};
use kai_types::{round::RoundStep, part_set::Part};
use std::{any::Any, sync::Arc};
use std::fmt::Debug;

/**
   Message is a message that can be sent and received on the `ConsensusReactor`
*/
pub trait ConsensusMessage: Debug + Send + Sync + 'static {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>>;
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone)]
pub enum ConsensusMessageType {
    NewRoundStepMessage(NewRoundStepMessage),
    NewValidBlockMessage(NewValidBlockMessage),
    ProposalMessage(ProposalMessage),
    ProposalPOLMessage(ProposalPOLMessage),
    BlockPartMessage(BlockPartMessage),
    VoteMessage(VoteMessage),
    HasVoteMessage(HasVoteMessage),
    VoteSetMaj23Message(VoteSetMaj23Message),
    VoteSetBitsMessage(VoteSetBitsMessage),
}

pub struct MessageInfo {
    pub msg: Arc<ConsensusMessageType>,
    pub peer_id: PeerId,
}

pub fn msg_from_proto(
    msg_proto: ConsensusMessageProto,
) -> Result<Arc<ConsensusMessageType>, Box<ConsensusReactorError>> {
    if let Some(sum) = msg_proto.sum {
        match sum {
            Sum::NewRoundStep(m) => Ok(Arc::new(ConsensusMessageType::NewRoundStepMessage(
                NewRoundStepMessage::from(m),
            ))),
            Sum::NewValidBlock(m) => Ok(Arc::new(ConsensusMessageType::NewValidBlockMessage(
                NewValidBlockMessage::from(m),
            ))),
            Sum::Proposal(m) => Ok(Arc::new(ConsensusMessageType::ProposalMessage(
                ProposalMessage::from(m),
            ))),
            Sum::ProposalPol(m) => Ok(Arc::new(ConsensusMessageType::ProposalPOLMessage(
                ProposalPOLMessage::from(m),
            ))),
            Sum::BlockPart(m) => Ok(Arc::new(ConsensusMessageType::BlockPartMessage(
                BlockPartMessage::from(m),
            ))),
            Sum::Vote(m) => Ok(Arc::new(ConsensusMessageType::VoteMessage(
                VoteMessage::from(m),
            ))),
            Sum::HasVote(m) => Ok(Arc::new(ConsensusMessageType::HasVoteMessage(
                HasVoteMessage::from(m),
            ))),
            Sum::VoteSetMaj23(m) => Ok(Arc::new(ConsensusMessageType::VoteSetMaj23Message(
                VoteSetMaj23Message::from(m),
            ))),
            Sum::VoteSetBits(m) => Ok(Arc::new(ConsensusMessageType::VoteSetBitsMessage(
                VoteSetBitsMessage::from(m),
            ))),
            _ => Err(Box::new(ConsensusReactorError::DecodeProtoError)),
        }
    } else {
        Err(Box::new(ConsensusReactorError::DecodeProtoError))
    }
}

#[derive(Debug, Clone)]
pub struct NewRoundStepMessage {
    pub height: u64,
    pub round: u32,
    pub step: RoundStep,
    pub seconds_since_start_time: u64,
    pub last_commit_round: u32,
}

impl ConsensusMessage for NewRoundStepMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        if self.step.is_valid() {
            Ok(())
        } else {
            Err(Box::new(ConsensusReactorError::ErrInvalidStep))
        }
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::NewRoundStep(NewRoundStepMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::NewRoundStep> for NewRoundStepMessage {
    fn from(m: kai_proto::consensus::NewRoundStep) -> Self {
        Self {
            height: m.height,
            round: m.round,
            step: match m.step {
                11 | 1 | 2 | 3 => RoundStep::Propose,
                12 | 4 | 5 => RoundStep::Prevote,
                13 | 6 | 7 | 8 => RoundStep::Precommit,
                _ => RoundStep::Unknown,
            },
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
            step: self.step as u32,
            seconds_since_start_time: self.seconds_since_start_time,
            last_commit_round: self.last_commit_round,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewValidBlockMessage {
    pub height: u64,
    pub round: u32,
    pub block_parts_header: Option<kai_proto::types::PartSetHeader>,
    pub block_parts: Option<kai_proto::types::BitArray>,
    pub is_commit: bool,
}

impl ConsensusMessage for NewValidBlockMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::NewValidBlock(NewValidBlockMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::NewValidBlock> for NewValidBlockMessage {
    fn from(m: kai_proto::consensus::NewValidBlock) -> Self {
        Self {
            height: m.height,
            round: m.round,
            block_parts_header: m.block_part_set_header,
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
            block_part_set_header: self.block_parts_header,
            block_parts: self.block_parts,
            is_commit: self.is_commit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasVoteMessage {
    pub height: u64,
    pub round: u32,
    pub r#type: SignedMsgType,
    pub index: u32,
}

impl ConsensusMessage for HasVoteMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::HasVote(HasVoteMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::HasVote> for HasVoteMessage {
    fn from(m: kai_proto::consensus::HasVote) -> Self {
        Self {
            height: m.height,
            round: m.round,
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            },
            index: m.index,
        }
    }
}

impl Into<kai_proto::consensus::HasVote> for HasVoteMessage {
    fn into(self: Self) -> kai_proto::consensus::HasVote {
        kai_proto::consensus::HasVote {
            height: self.height,
            round: self.round,
            r#type: self.r#type.into(),
            index: self.index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoteSetMaj23Message {
    pub height: u64,
    pub round: u32,
    pub r#type: SignedMsgType,
    pub block_id: Option<kai_proto::types::BlockId>,
}

impl ConsensusMessage for VoteSetMaj23Message {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::VoteSetMaj23(VoteSetMaj23Message::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::VoteSetMaj23> for VoteSetMaj23Message {
    fn from(m: kai_proto::consensus::VoteSetMaj23) -> Self {
        Self {
            height: m.height,
            round: m.round,
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            },
            block_id: m.block_id,
        }
    }
}

impl Into<kai_proto::consensus::VoteSetMaj23> for VoteSetMaj23Message {
    fn into(self: Self) -> kai_proto::consensus::VoteSetMaj23 {
        kai_proto::consensus::VoteSetMaj23 {
            height: self.height,
            round: self.round,
            r#type: self.r#type.into(),
            block_id: self.block_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProposalMessage {
    pub proposal: Option<kai_types::proposal::Proposal>,
}

impl ConsensusMessage for ProposalMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::Proposal(ProposalMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::Proposal> for ProposalMessage {
    fn from(m: kai_proto::consensus::Proposal) -> Self {
        Self {
            proposal: m.proposal.map(|p| p.into()),
        }
    }
}

impl Into<kai_proto::consensus::Proposal> for ProposalMessage {
    fn into(self: Self) -> kai_proto::consensus::Proposal {
        kai_proto::consensus::Proposal {
            proposal: self.proposal.map(|p| p.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProposalPOLMessage {
    pub height: u64,
    pub proposal_pol_round: u32,
    pub proposal_pol: Option<kai_proto::types::BitArray>,
}

impl ConsensusMessage for ProposalPOLMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::ProposalPol(ProposalPOLMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
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
    pub part: Option<Part>,
}

impl ConsensusMessage for BlockPartMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::BlockPart(BlockPartMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::BlockPart> for BlockPartMessage {
    fn from(m: kai_proto::consensus::BlockPart) -> Self {
        Self {
            height: m.height,
            round: m.round,
            part: m.part.map(|p| p.into()),
        }
    }
}
impl Into<kai_proto::consensus::BlockPart> for BlockPartMessage {
    fn into(self: Self) -> kai_proto::consensus::BlockPart {
        kai_proto::consensus::BlockPart {
            height: self.height,
            round: self.round,
            part: self.part.map(|p| p.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoteMessage {
    pub vote: Option<kai_types::vote::Vote>,
}

impl ConsensusMessage for VoteMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::Vote(VoteMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::Vote> for VoteMessage {
    fn from(m: kai_proto::consensus::Vote) -> Self {
        Self {
            vote: m.vote.map(|v| v.into()),
        }
    }
}
impl Into<kai_proto::consensus::Vote> for VoteMessage {
    fn into(self: Self) -> kai_proto::consensus::Vote {
        kai_proto::consensus::Vote {
            vote: self.vote.map(|v| v.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VoteSetBitsMessage {
    pub height: u64,
    pub round: u32,
    pub r#type: SignedMsgType,
    pub block_id: Option<kai_proto::types::BlockId>,
    pub votes: Option<kai_proto::types::BitArray>,
}

impl ConsensusMessage for VoteSetBitsMessage {
    fn validate_basic(&self) -> Result<(), Box<ConsensusReactorError>> {
        todo!()
    }
    fn msg_to_proto(&self) -> Result<ConsensusMessageProto, Box<ConsensusReactorError>> {
        Ok(ConsensusMessageProto {
            sum: Some(Sum::VoteSetBits(VoteSetBitsMessage::into(self.clone()))),
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<kai_proto::consensus::VoteSetBits> for VoteSetBitsMessage {
    fn from(m: kai_proto::consensus::VoteSetBits) -> Self {
        Self {
            height: m.height,
            round: m.round,
            r#type: match m.r#type {
                1 => SignedMsgType::Prevote,
                2 => SignedMsgType::Precommit,
                32 => SignedMsgType::Proposal,
                _ => SignedMsgType::Unknown,
            },
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
            r#type: self.r#type.into(),
            block_id: self.block_id,
            votes: self.votes,
        }
    }
}
