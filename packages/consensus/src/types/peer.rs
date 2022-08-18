use super::round::RoundStep;
use kai_types::{bit_array::BitArray, block::PartSetHeader, proposal::Proposal};
use std::sync::{Arc, Mutex};

pub type ChannelId = u8;
pub type Message = Vec<u8>;
pub type PeerId = String;

pub struct Peer {
    pub id: PeerId,
    /**
       peer state
    */
    pub ps: Arc<Mutex<PeerState>>,
}

impl Peer {
    pub fn new(id: PeerId) -> Self {
        Self {
            id,
            ps: Arc::new(Mutex::new(PeerState::new())),
        }
    }
}

pub struct PeerState {
    /**
       peer round state
    */
    pub prs: PeerRoundState,
}

impl PeerState {
    pub fn new() -> Self {
        Self {
            prs: PeerRoundState::new(),
        }
    }

    pub fn get_round_state(self) -> Box<PeerRoundState> {
        Box::new(self.prs)
    }

    pub fn set_has_proposal(mut self, proposal: Proposal) {
        if (self.prs.height != proposal.height) || (self.prs.round != proposal.round) {
            return;
        }

        if self.prs.proposal {
            return;
        }

        self.prs.proposal = true;

        if self.prs.proposal_block_parts.is_some() {
            return;
        }

        self.prs.proposal_block_parts_header = proposal.block_id.unwrap().part_set_header;
        self.prs.proposal_block_parts = None; // None until ProposalBlockPartMessage received.
        self.prs.proposal_pol_round = proposal.pol_round;
        self.prs.proposal_pol = None; // None until ProposalPOLMessage received.
    }

    pub fn set_has_proposal_block_part(self, height: u64, round: u32, index: usize) {
        if (self.prs.height != height) || (self.prs.round != round) {
            return;
        }

        if let Some(pbp) = self.prs.proposal_block_parts {
            // TODO: implement BitArray.set_index for Proposal Block Parts
            pbp.set_index(index, true);
        }
    }

    // pub fn apply_new_round_step_message(msg)
}

/**
PeerRoundState contains the known state of a peer.
*/
pub struct PeerRoundState {
    height: u64,
    round: u32,
    step: RoundStep,
    start_time: u64,
    proposal: bool,
    proposal_block_parts_header: Option<PartSetHeader>,
    proposal_block_parts: Option<BitArray>,
    proposal_pol_round: u32,
    proposal_pol: Option<BitArray>,
    prevotes: Option<BitArray>,
    precommits: Option<BitArray>,
    last_commit_round: u32,
    last_commit: Option<BitArray>,
    catchup_commit_round: u32,
    catchup_commit: Option<BitArray>,
}

impl PeerRoundState {
    pub fn new() -> Self {
        Self {
            height: 0,
            round: 0,
            step: RoundStep::RoundStepPropose,
            start_time: 0,
            proposal: false,
            proposal_block_parts_header: None,
            proposal_block_parts: None,
            proposal_pol_round: 0,
            proposal_pol: None,
            prevotes: None,
            precommits: None,
            last_commit_round: 0,
            last_commit: None,
            catchup_commit_round: 0,
            catchup_commit: None,
        }
    }
}
