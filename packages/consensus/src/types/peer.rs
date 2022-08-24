use super::messages::{
    BlockPartMessage, NewRoundStepMessage, NewValidBlockMessage, ProposalMessage,
    ProposalPOLMessage,
};
use crate::utils::compare_hrs;
use core::fmt::Debug;
use kai_proto::types::SignedMsgType;
use kai_types::part_set::PartSetHeader;
use kai_types::round::RoundStep;
use kai_types::{bit_array::BitArray, vote::is_valid_vote_type};
use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

pub type ChannelId = u8;
pub type Message = Vec<u8>;
pub type PeerId = String;

pub fn internal_peerid() -> PeerId {
    "".to_string()
}

pub struct Peer {
    pub id: PeerId,
    /**
       peer state
    */
    pub ps: Arc<Mutex<dyn PeerState>>,
}

impl Peer {
    pub fn new(id: PeerId) -> Self {
        Self {
            id,
            ps: Arc::new(Mutex::new(PeerStateImpl::new())),
        }
    }
}

pub trait PeerState: Debug + Sync + Send + 'static {
    fn set_prs(&mut self, new_prs: PeerRoundState);
    fn get_prs(&self) -> PeerRoundState;
    fn set_has_proposal(&mut self, msg: ProposalMessage);
    fn set_has_proposal_block_part(&mut self, msg: BlockPartMessage);
    fn apply_new_valid_block_message(&mut self, msg: NewValidBlockMessage);
    fn set_has_vote(
        &mut self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
        index: usize,
    );
    fn apply_new_round_step_message(&mut self, msg: NewRoundStepMessage);
    fn apply_proposal_pol_message(&mut self, msg: ProposalPOLMessage);
}

#[derive(Debug, Clone)]
pub struct PeerStateImpl {
    /**
    peer round state
    */
    pub prs: PeerRoundState,
}

impl PeerStateImpl {
    pub fn new() -> Self {
        Self {
            prs: PeerRoundState::new(),
        }
    }

    fn get_vote_bit_array(
        &mut self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
    ) -> Option<BitArray> {
        if !is_valid_vote_type(signed_msg_type) {
            return None;
        }

        if self.prs.height == height {
            if self.prs.round == round {
                return match signed_msg_type {
                    SignedMsgType::Prevote => self.prs.prevotes.clone(),
                    SignedMsgType::Precommit => self.prs.precommits.clone(),
                    _ => None,
                };
            }
            if self.prs.catchup_commit_round == round {
                return match signed_msg_type {
                    SignedMsgType::Precommit => self.prs.catchup_commit.clone(),
                    _ => None,
                };
            }
            if self.prs.proposal_pol_round == round {
                return match signed_msg_type {
                    SignedMsgType::Prevote => self.prs.proposal_pol.clone(),
                    _ => None,
                };
            }
        }

        if self.prs.height == height + 1 {
            if self.prs.last_commit_round == round {
                return match signed_msg_type {
                    SignedMsgType::Precommit => self.prs.last_commit.clone(),
                    _ => None,
                };
            }
        }

        return None;
    }

    fn _set_has_vote(
        &mut self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
        index: u32,
    ) {
        if let Some(ps_votes) = self.get_vote_bit_array(height, round, signed_msg_type) {
            ps_votes.set_index(index.try_into().unwrap(), true);
        }
    }
}

impl PeerState for PeerStateImpl {
    fn set_prs(&mut self, new_prs: PeerRoundState) {
        self.prs = new_prs;
    }

    fn get_prs(&self) -> PeerRoundState {
        self.prs.clone()
    }

    fn set_has_proposal(&mut self, msg: ProposalMessage) {
        let proposal = msg.proposal.unwrap();

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

    fn set_has_proposal_block_part(&mut self, msg: BlockPartMessage) {
        if (self.prs.height != msg.height) || (self.prs.round != msg.round) {
            return;
        }

        if let Some(pbp) = self.prs.proposal_block_parts.clone() {
            // TODO: implement BitArray.set_index for Proposal Block Parts
            pbp.set_index(msg.part.unwrap().index.try_into().unwrap(), true);
        }
    }

    fn apply_new_valid_block_message(&mut self, msg: NewValidBlockMessage) {
        if self.prs.height != msg.height {
            return;
        }

        if self.prs.round != msg.round && !msg.is_commit {
            return;
        }

        self.prs.proposal_block_parts_header = msg.block_parts_header.map(|m| m.into());
        self.prs.proposal_block_parts = msg.block_parts.map(|m| m.into());
    }

    fn set_has_vote(
        &mut self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
        index: usize,
    ) {
        if let Some(ps_votes) = self.get_vote_bit_array(height, round, signed_msg_type) {
            ps_votes.set_index(index.try_into().unwrap(), true);
        }
    }

    fn apply_new_round_step_message(&mut self, msg: NewRoundStepMessage) {
        if compare_hrs(
            msg.height,
            msg.round,
            msg.step,
            self.prs.height,
            self.prs.round,
            self.prs.step,
        ) <= 0
        {
            return;
        }

        // temp values
        let ps_height = self.prs.height;
        let ps_round = self.prs.round;
        let ps_catchup_commit_round = self.prs.catchup_commit_round;
        let ps_catchup_commit = self.prs.catchup_commit.clone();

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - msg.seconds_since_start_time;
        self.prs.height = msg.height;
        self.prs.round = msg.round;
        self.prs.step = msg.step;
        self.prs.start_time = start_time;

        if (ps_height != msg.height) || (ps_round != msg.round) {
            self.prs.proposal = false;
            self.prs.proposal_block_parts_header = None;
            self.prs.proposal_block_parts = None;
            self.prs.proposal_pol_round = 0;
            self.prs.proposal_pol = None;
            self.prs.prevotes = None;
            self.prs.precommits = None;
        }
        if (ps_height == msg.height)
            && (ps_round != msg.round)
            && (msg.round == ps_catchup_commit_round)
        {
            self.prs.precommits = ps_catchup_commit;
        }
        if ps_height != msg.height {
            // shift precommits to lastcommit.
            if (ps_height + 1 == msg.height) && (ps_round == msg.last_commit_round) {
                self.prs.last_commit_round = msg.last_commit_round;
                self.prs.last_commit = self.prs.precommits.clone();
            } else {
                self.prs.last_commit_round = msg.last_commit_round;
                self.prs.last_commit = None
            }
            self.prs.catchup_commit_round = 0;
            self.prs.catchup_commit = None;
        }
    }

    // pub fn apply_vote_set_bits_message(
    //     mut self,
    //     msg: VoteSetBitsMessage,
    //     our_votes: Option<BitArray>,
    // ) {
    //     if let Some(votes) = self.get_vote_bit_array(msg.height, msg.round, msg.r#type) {
    //         if let Some(_our_votes) = our_votes {
    //             // TODO: implement sub(), or(), update() for BitArray
    //             // let other_votes = votes.sub(_our_votes);
    //             // let has_votes = other_votes.or(msg.votes);
    //             // votes.update(has_votes);
    //         } else {
    //             // TODO:
    //             // votes.Update(msg.votes)
    //         }
    //     }
    // }

    fn apply_proposal_pol_message(&mut self, msg: ProposalPOLMessage) {
        if self.prs.height != msg.height {
            return;
        }
        if self.prs.proposal_pol_round != msg.proposal_pol_round {
            return;
        }

        self.prs.proposal_pol = msg.proposal_pol.map(|p| p.into());
    }
}

/**
PeerRoundState contains the known state of a peer.
*/
#[derive(Debug, Clone)]
pub struct PeerRoundState {
    pub height: u64,
    pub round: u32,
    pub step: RoundStep,
    pub start_time: u64,
    pub proposal: bool,
    pub proposal_block_parts_header: Option<PartSetHeader>,
    pub proposal_block_parts: Option<BitArray>,
    pub proposal_pol_round: u32,
    pub proposal_pol: Option<BitArray>,
    pub prevotes: Option<BitArray>,
    pub precommits: Option<BitArray>,
    pub last_commit_round: u32,
    pub last_commit: Option<BitArray>,
    pub catchup_commit_round: u32,
    pub catchup_commit: Option<BitArray>,
}

impl PeerRoundState {
    pub fn new() -> Self {
        Self {
            height: 0,
            round: 0,
            step: RoundStep::Propose,
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

#[cfg(test)]
mod tests {
    use core::time;
    use std::{sync::Arc, thread};

    use crate::types::peer::Peer;

    use super::PeerRoundState;

    #[test]
    fn get_round_state_ok() {
        // arrange
        let peer_id = String::from("peer1");
        let peer = Peer::new(peer_id);

        // act
        if let Ok(ps_guard) = Arc::clone(&peer.ps).lock() {
            // assert
            assert_eq!(ps_guard.get_prs().height, 0);
        }
    }

    #[test]
    fn get_round_state_failed() {
        // arrange
        let peer_id = String::from("peer1");
        let peer = Peer::new(peer_id);
        let ps_1 = Arc::clone(&peer.ps);
        let ps_2 = Arc::clone(&peer.ps);
        let ps_3 = Arc::clone(&peer.ps);

        // this thread locks peer state for 500ms
        thread::spawn(move || {
            if let Ok(mut ps_guard) = ps_1.lock() {
                let mut new_prs = PeerRoundState::new();
                new_prs.height = 10;
                ps_guard.set_prs(new_prs);
                drop(ps_guard);
            }
            thread::sleep(time::Duration::from_millis(500));
        });
        // this thread try lock failed
        thread::spawn(move || {
            let ps_guard = ps_2.try_lock();
            assert!(ps_guard.is_err());
        });
        // this thread wait until thread #1 release the lock and read values
        thread::spawn(move || {
            if let Ok(ps_guard) = ps_3.lock() {
                assert_eq!(ps_guard.get_prs().height, 10);
            }
        });
    }
}
