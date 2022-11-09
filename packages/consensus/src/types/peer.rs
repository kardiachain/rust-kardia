use super::messages::{
    BlockPartMessage, NewRoundStepMessage, NewValidBlockMessage, ProposalMessage,
    ProposalPOLMessage, VoteSetBitsMessage,
};
use crate::utils::compare_hrs;
use async_trait::async_trait;
use core::fmt::Debug;
use kai_types::misc::ChannelId;
use kai_types::part_set::PartSetHeader;
use kai_types::peer::PeerId;
use kai_types::round::RoundStep;
use kai_types::types::SignedMsgType;
use kai_types::vote_set::VoteSetReader;
use kai_types::{bit_array::BitArray, vote::is_valid_vote_type};
use mockall::automock;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;

pub fn internal_peerid() -> PeerId {
    String::from("")
}

#[automock]
#[async_trait]
pub trait Peer: Debug + Send + Sync + 'static {
    fn get_id(&self) -> PeerId;
    async fn get_ps(&self) -> Arc<Box<dyn PeerState>>;
    async fn get_prs(&self) -> PeerRoundState;
    fn send(&self, ch_id: ChannelId, _msg: Vec<u8>) -> bool;
    fn try_send(&self, ch_id: ChannelId, _msg: Vec<u8>) -> bool;
    fn pick_send_vote(&self, votes: Box<dyn VoteSetReader>) -> bool;
}

#[derive(Debug)]
pub struct PeerImpl {
    pub id: PeerId,
    /**
       peer state
    */
    pub ps: Arc<Box<dyn PeerState>>,
}

#[async_trait]
impl Peer for PeerImpl {
    fn get_id(&self) -> PeerId {
        self.id.clone()
    }

    async fn get_ps(&self) -> Arc<Box<dyn PeerState>> {
        self.ps.clone()
    }

    async fn get_prs(&self) -> PeerRoundState {
        self.ps.get_prs().await
    }

    fn send(&self, ch_id: ChannelId, _msg: Vec<u8>) -> bool {
        todo!()
    }

    fn try_send(&self, ch_id: ChannelId, _msg: Vec<u8>) -> bool {
        todo!()
    }

    fn pick_send_vote(&self, votes: Box<dyn VoteSetReader>) -> bool {
        todo!()
    }
}

impl PeerImpl {
    pub fn new(id: PeerId) -> Arc<dyn Peer> {
        Arc::new(Self {
            id,
            ps: Arc::new(Box::new(PeerStateImpl::new())),
        })
    }
}

#[automock]
#[async_trait]
pub trait PeerState: Debug + Sync + Send + 'static {
    async fn get_prs(&self) -> PeerRoundState;
    async fn set_has_vote(
        &self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
        index: usize,
    ) -> ();
    async fn set_has_proposal(&self, msg: ProposalMessage);
    async fn set_has_proposal_block_part(&self, msg: BlockPartMessage);
    async fn apply_new_valid_block_message(&self, msg: NewValidBlockMessage);
    async fn apply_new_round_step_message(&self, msg: NewRoundStepMessage);
    async fn apply_proposal_pol_message(&self, msg: ProposalPOLMessage);
    async fn apply_vote_set_bits_message(
        &self,
        msg: VoteSetBitsMessage,
        our_votes: Option<BitArray>,
    );
}

#[derive(Debug, Clone)]
pub struct PeerStateImpl {
    /// peer round state
    pub prs: Arc<Mutex<PeerRoundState>>,
}

impl PeerStateImpl {
    pub fn new() -> Self {
        Self {
            prs: Arc::new(Mutex::new(PeerRoundState::new())),
        }
    }
}

#[async_trait]
impl PeerState for PeerStateImpl {
    async fn get_prs(&self) -> PeerRoundState {
        let prs_guard = self.prs.lock().await;
        prs_guard.clone()
    }

    async fn set_has_vote(
        &self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
        index: usize,
    ) {
        let mut prs_guard = self.prs.lock().await;
        prs_guard.set_has_vote(height, round, signed_msg_type, index);
    }

    async fn set_has_proposal(&self, msg: ProposalMessage) {
        let mut prs_guard = self.prs.lock().await;
        prs_guard.set_has_proposal(msg);
    }

    async fn set_has_proposal_block_part(&self, msg: BlockPartMessage) {
        let mut prs_guard = self.prs.lock().await;
        prs_guard.set_has_proposal_block_part(msg);
    }

    async fn apply_new_valid_block_message(&self, msg: NewValidBlockMessage) {
        let mut prs_guard = self.prs.lock().await;
        prs_guard.apply_new_valid_block_message(msg);
    }

    async fn apply_new_round_step_message(&self, msg: NewRoundStepMessage) {
        let mut prs_guard = self.prs.lock().await;
        prs_guard.apply_new_round_step_message(msg);
    }

    async fn apply_proposal_pol_message(&self, msg: ProposalPOLMessage) {
        let mut prs_guard = self.prs.lock().await;
        prs_guard.apply_proposal_pol_message(msg);
    }

    async fn apply_vote_set_bits_message(
        &self,
        msg: VoteSetBitsMessage,
        our_votes: Option<BitArray>,
    ) {
        let mut prs_guard = self.prs.lock().await;
        prs_guard.apply_vote_set_bits_message(msg, our_votes);
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

    fn get_mut_vote_bit_array(
        &mut self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
    ) -> Option<&mut BitArray> {
        if !is_valid_vote_type(signed_msg_type) {
            return None;
        }

        if self.height == height {
            if self.round == round {
                return match signed_msg_type {
                    SignedMsgType::Prevote => self.prevotes.as_mut(),
                    SignedMsgType::Precommit => self.precommits.as_mut(),
                    _ => None,
                };
            }
            if self.catchup_commit_round == round {
                return match signed_msg_type {
                    SignedMsgType::Precommit => self.catchup_commit.as_mut(),
                    _ => None,
                };
            }
            if self.proposal_pol_round == round {
                return match signed_msg_type {
                    SignedMsgType::Prevote => self.proposal_pol.as_mut(),
                    _ => None,
                };
            }
        }

        if self.height == height + 1 {
            if self.last_commit_round == round {
                return match signed_msg_type {
                    SignedMsgType::Precommit => self.last_commit.as_mut(),
                    _ => None,
                };
            }
        }

        return None;
    }

    pub fn set_has_vote(
        &mut self,
        height: u64,
        round: u32,
        signed_msg_type: SignedMsgType,
        index: usize,
    ) {
        if let Some(vote_bit_array) = self.get_mut_vote_bit_array(height, round, signed_msg_type) {
            vote_bit_array.set_index(index, true);
        }
    }

    pub fn set_has_proposal(&mut self, msg: ProposalMessage) {
        let proposal = msg.proposal.unwrap();

        if (self.height != proposal.height) || (self.round != proposal.round) {
            return;
        }

        if self.proposal {
            return;
        }

        self.proposal = true;

        if self.proposal_block_parts.is_some() {
            return;
        }

        self.proposal_block_parts_header = proposal.block_id.unwrap().part_set_header;
        self.proposal_block_parts = None; // None until ProposalBlockPartMessage received.
        self.proposal_pol_round = proposal.pol_round;
        self.proposal_pol = None; // None until ProposalPOLMessage received.
    }

    pub fn set_has_proposal_block_part(&mut self, msg: BlockPartMessage) {
        if (self.height != msg.height) || (self.round != msg.round) {
            return;
        }

        if let Some(pbp) = self.proposal_block_parts.as_mut() {
            // TODO: implement BitArray.set_index for Proposal Block Parts
            pbp.set_index(msg.part.unwrap().index.try_into().unwrap(), true);
        }
    }

    pub fn apply_new_valid_block_message(&mut self, msg: NewValidBlockMessage) {
        if self.height != msg.height {
            return;
        }

        if self.round != msg.round && !msg.is_commit {
            return;
        }

        self.proposal_block_parts_header = msg.block_parts_header.map(|m| m.into());
        self.proposal_block_parts = msg.block_parts.map(|m| m.into());
    }

    pub fn apply_new_round_step_message(&mut self, msg: NewRoundStepMessage) {
        if compare_hrs(
            msg.height,
            msg.round,
            msg.step,
            self.height,
            self.round,
            self.step,
        ) <= 0
        {
            return;
        }

        // temp values
        let ps_height = self.height;
        let ps_round = self.round;
        let ps_catchup_commit_round = self.catchup_commit_round;
        let ps_catchup_commit = self.catchup_commit.clone();

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - msg.seconds_since_start_time;
        self.height = msg.height;
        self.round = msg.round;
        self.step = msg.step;
        self.start_time = start_time;

        if (ps_height != msg.height) || (ps_round != msg.round) {
            self.proposal = false;
            self.proposal_block_parts_header = None;
            self.proposal_block_parts = None;
            self.proposal_pol_round = 0;
            self.proposal_pol = None;
            self.prevotes = None;
            self.precommits = None;
        }
        if (ps_height == msg.height)
            && (ps_round != msg.round)
            && (msg.round == ps_catchup_commit_round)
        {
            self.precommits = ps_catchup_commit;
        }
        if ps_height != msg.height {
            // shift precommits to lastcommit.
            if (ps_height + 1 == msg.height) && (ps_round == msg.last_commit_round) {
                self.last_commit_round = msg.last_commit_round;
                self.last_commit = self.precommits.clone();
            } else {
                self.last_commit_round = msg.last_commit_round;
                self.last_commit = None
            }
            self.catchup_commit_round = 0;
            self.catchup_commit = None;
        }
    }

    pub fn apply_vote_set_bits_message(
        &mut self,
        msg: VoteSetBitsMessage,
        our_votes: Option<BitArray>,
    ) {
        todo!()
        // if let Some(votes) = self.get_vote_bit_array(msg.height, msg.round, msg.r#type) {
        //     if let Some(_our_votes) = our_votes {
        //         // TODO: implement sub(), or(), update() for BitArray
        //         // let other_votes = votes.sub(_our_votes);
        //         // let has_votes = other_votes.or(msg.votes);
        //         // votes.update(has_votes);
        //     } else {
        //         // TODO:
        //         // votes.Update(msg.votes)
        //     }
        // }
    }

    pub fn apply_proposal_pol_message(&mut self, msg: ProposalPOLMessage) {
        if self.height != msg.height {
            return;
        }
        if self.proposal_pol_round != msg.proposal_pol_round {
            return;
        }

        self.proposal_pol = msg.proposal_pol.map(|p| p.into());
    }
}

#[cfg(test)]
mod tests {
    use kai_types::{bit_array::BitArray, types::SignedMsgType};

    use crate::types::peer::PeerImpl;

    use super::PeerRoundState;

    #[tokio::test]
    async fn get_round_state_ok() {
        // arrange
        let peer_id = "peer1".to_string();
        let peer = PeerImpl::new(peer_id);

        // act
        let prs = peer.get_prs().await;
        // assert
        assert_eq!(prs.height, 0);
    }

    #[tokio::test]
    async fn set_has_vote() {
        // arrange
        let mut prs = PeerRoundState::new();
        prs.height = 1;
        prs.round = 1;

        let bit_arr_size: usize = 10;
        prs.prevotes = Some(BitArray::new(bit_arr_size));

        let new_vote_index = 0;
        let new_vote_type = SignedMsgType::Prevote;

        // act
        prs.set_has_vote(1, 1, new_vote_type, new_vote_index);

        // assertions
        let vote = prs.prevotes;
        let rs = vote.unwrap().get_index(new_vote_index);
        assert!(rs.is_ok_and(|v| *v == true));
    }
}
