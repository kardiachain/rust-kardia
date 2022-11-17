use crate::{
    state::ConsensusState,
    types::{
        errors::ConsensusReactorError,
        messages::{
            msg_from_proto, BlockPartMessage, ConsensusMessage, ConsensusMessageType, MessageInfo,
            ProposalMessage, ProposalPOLMessage, VoteSetBitsMessage, VoteSetMaj23Message,
        },
        peer::Peer,
        round_state::RoundState,
    },
};
use async_trait::async_trait;
use kai_proto::consensus::Message as ConsensusMessageProto;
use kai_types::{
    consensus::state::LatestBlockState,
    misc::{ChannelId, Message as PeerMessage},
    round::RoundStep,
    types::SignedMsgType,
};
use prost::Message;
use std::fmt::Debug;
use std::sync::Arc;
use std::{result::Result::Ok, thread};

pub const STATE_CHANNEL: u8 = 0x20;
pub const DATA_CHANNEL: u8 = 0x21;
pub const VOTE_CHANNEL: u8 = 0x22;
pub const VOTE_SET_BITS_CHANNEL: u8 = 0x23;

#[async_trait]
pub trait ConsensusReactor: Debug + Send + Sync + 'static {
    fn switch_to_consensus(
        self: Arc<Self>,
        state: Arc<Box<dyn LatestBlockState>>,
        skip_wal: bool,
    ) -> Result<(), Box<ConsensusReactorError>>;
    fn set_priv_validator(self: Arc<Self>) -> ();
    fn get_priv_validator(self: Arc<Self>) -> ();
    fn get_validators(self: Arc<Self>) -> ();
    async fn add_peer(self: Arc<Self>, peer: Arc<dyn Peer>) -> Result<(), ConsensusReactorError>;
    async fn remove_peer(
        self: Arc<Self>,
        peer: Arc<dyn Peer>,
    ) -> Result<(), Box<ConsensusReactorError>>;
    async fn receive(
        self: Arc<Self>,
        ch_id: ChannelId,
        src: Arc<dyn Peer>,
        msg: PeerMessage,
    ) -> Result<(), Box<ConsensusReactorError>>;
    fn get_cs(self: Arc<Self>) -> Arc<Box<dyn ConsensusState>>;
}

#[derive(Debug)]
pub struct ConsensusReactorImpl {
    cs: Arc<Box<dyn ConsensusState>>,
}

#[async_trait]
impl ConsensusReactor for ConsensusReactorImpl {
    fn switch_to_consensus(
        self: Arc<Self>,
        state: Arc<Box<dyn LatestBlockState>>,
        _skip_wal: bool,
    ) -> Result<(), Box<ConsensusReactorError>> {
        let cs = self.clone().get_cs().clone();
        let _block_state = cs.get_state();

        if state.get_last_block_height() > 0 {
            // TODO: add a function below
            // cs.reconstruct_from_last_commit(state);
        }

        cs.update_to_state(state);

        // TODO: implement waitSync and skipWAL?
        // conR.mtx.Lock()
        // conR.waitSync = false
        // conR.mtx.Unlock()

        // if skipWAL {
        //     conR.conS.doWALCatchup = false
        // }

        // err := conR.conS.Start()
        // if err != nil {
        //     panic(fmt.Sprintf(`Failed to start consensus state: %v

        // conS:
        // %+v

        // conR:
        // %+v`, err, conR.conS, conR))
        //     }
        //     conR.Logger.Info("Switched to consensus", "skipWAL", skipWAL)

        Ok(())
    }

    fn set_priv_validator(self: Arc<Self>) -> () {
        todo!()
    }

    fn get_priv_validator(self: Arc<Self>) -> () {
        todo!()
    }

    fn get_validators(self: Arc<Self>) -> () {
        todo!()
    }

    async fn add_peer(self: Arc<Self>, peer: Arc<dyn Peer>) -> Result<(), ConsensusReactorError> {
        peer.clone().start().await;

        self.clone().gossip_data(peer.clone());
        self.clone().gossip_votes(peer.clone());
        self.clone().query_maj23(peer.clone());
        Ok(())
    }

    async fn remove_peer(
        self: Arc<Self>,
        peer: Arc<dyn Peer>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        peer.clone().stop().await;
        drop(peer);

        Ok(())
    }

    async fn receive(
        self: Arc<Self>,
        ch_id: ChannelId,
        src: Arc<dyn Peer>,
        msg: PeerMessage,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match Self::decode_msg(msg.as_slice()) {
            Ok(decoded_msg) => match ch_id {
                STATE_CHANNEL => self.handle_state_message(src, decoded_msg).await,
                DATA_CHANNEL => self.handle_data_message(src, decoded_msg).await,
                VOTE_CHANNEL => self.handle_vote_message(src, decoded_msg).await,
                VOTE_SET_BITS_CHANNEL => self.handle_vote_set_bits_message(src, decoded_msg).await,
                _ => Err(Box::new(ConsensusReactorError::UnknownChannelIdError(
                    ch_id,
                ))),
            },
            Err(err) => Err(err),
        }
    }

    fn get_cs(self: Arc<Self>) -> Arc<Box<dyn ConsensusState>> {
        self.cs.clone()
    }
}

impl ConsensusReactorImpl {
    fn new(_cs: Box<dyn ConsensusState>) -> Self {
        Self { cs: Arc::new(_cs) }
    }

    fn decode_msg(bz: &[u8]) -> Result<Arc<ConsensusMessageType>, Box<ConsensusReactorError>> {
        if let Ok(proto_msg) = ConsensusMessageProto::decode(bz) {
            msg_from_proto(proto_msg)
        } else {
            Err(Box::new(ConsensusReactorError::DecodeProtoError))
        }
    }

    async fn handle_state_message(
        self: Arc<Self>,
        src: Arc<dyn Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::NewRoundStepMessage(_msg) => {
                if let Err(e) = _msg.validate_basic() {
                    return Err(e);
                }

                let ps = src.get_ps().await;
                ps.apply_new_round_step_message(_msg.clone()).await;
                return Ok(());
            }
            ConsensusMessageType::NewValidBlockMessage(_msg) => {
                let ps = src.get_ps().await;
                ps.apply_new_valid_block_message(_msg.clone()).await;
                Ok(())
            }
            ConsensusMessageType::HasVoteMessage(_msg) => {
                let ps = src.get_ps().await;
                ps.set_has_vote(
                    _msg.height,
                    _msg.round,
                    _msg.r#type,
                    _msg.index.try_into().unwrap(),
                )
                .await;
                Ok(())
            }
            ConsensusMessageType::VoteSetMaj23Message(_msg) => {
                let rs = self.get_cs().get_rs().await;
                let height = rs.height.clone();
                let votes = rs.votes.clone();

                if height != _msg.height {
                    return Ok(());
                }

                // TODO:
                // Peer claims to have a maj23 for some BlockID at H,R,S,
                // err := votes.SetPeerMaj23(msg.Round, msg.Type, ps.peer.ID(), msg.BlockID)
                // if err != nil {
                // 	conR.Switch.StopPeerForError(src, err)
                // 	return
                // }

                let our_votes = match _msg.r#type {
                    SignedMsgType::Prevote => votes
                        .and_then(|vts| vts.prevotes(_msg.round))
                        .and_then(|pv| pv.bit_array_by_block_id(_msg.block_id.clone().unwrap())),
                    SignedMsgType::Precommit => votes
                        .and_then(|vts| vts.precommits(_msg.round))
                        .and_then(|pv| pv.bit_array_by_block_id(_msg.block_id.clone().unwrap())),
                    _ => {
                        log::warn!("bad VoteSetBitsMessage field type, forgot to add a check in validate_basic?");
                        return Err(Box::new(
                                ConsensusReactorError::UnexpectedMessageTypeError(
                                    "bad VoteSetBitsMessage field type, forgot to add a check in validate_basic?".to_string()
                                )
                            ));
                    }
                };

                src.clone().try_send(
                    DATA_CHANNEL,
                    VoteSetBitsMessage {
                        height: _msg.height,
                        round: _msg.round,
                        r#type: _msg.r#type,
                        block_id: _msg.block_id.clone(),
                        votes: our_votes,
                    }
                    .msg_to_proto()
                    .unwrap()
                    .encode_to_vec(),
                );

                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    async fn handle_data_message(
        self: Arc<Self>,
        src: Arc<dyn Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::ProposalMessage(_msg) => {
                _ = self.cs.send(MessageInfo::IncomingMessage {
                    peer_id: src.get_id(),
                    msg: msg.clone(),
                });
                let ps = src.get_ps().await;
                ps.set_has_proposal(_msg.clone()).await;
                Ok(())
            }
            ConsensusMessageType::ProposalPOLMessage(_msg) => {
                let ps = src.get_ps().await;
                ps.apply_proposal_pol_message(_msg.clone()).await;
                Ok(())
            }
            ConsensusMessageType::BlockPartMessage(_msg) => {
                _ = self.cs.send(MessageInfo::IncomingMessage {
                    peer_id: src.get_id(),
                    msg: msg.clone(),
                });
                let ps = src.get_ps().await;
                ps.set_has_proposal_block_part(_msg.clone()).await;
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    async fn handle_vote_message(
        self: Arc<Self>,
        src: Arc<dyn Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::VoteMessage(_msg) => {
                if let Some(vote) = _msg.vote.clone() {
                    _ = self.cs.send(MessageInfo::IncomingMessage {
                        peer_id: src.get_id().clone(),
                        msg: msg.clone(),
                    });
                    let ps = src.get_ps().await;
                    ps.set_has_vote(
                        vote.height,
                        vote.round,
                        vote.r#type.into(),
                        vote.validator_index.try_into().unwrap(),
                    )
                    .await;
                    Ok(())
                } else {
                    Ok(())
                }
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    async fn handle_vote_set_bits_message(
        self: Arc<Self>,
        src: Arc<dyn Peer>,
        msg: Arc<ConsensusMessageType>,
    ) -> Result<(), Box<ConsensusReactorError>> {
        match msg.as_ref() {
            ConsensusMessageType::VoteSetBitsMessage(_msg) => {
                let rs = self.get_cs().get_rs().await;

                if rs.height == _msg.height {
                    let our_votes = match _msg.r#type {
                        SignedMsgType::Prevote => rs
                            .votes
                            .and_then(|vts| vts.prevotes(_msg.round))
                            .and_then(|pv| {
                                pv.bit_array_by_block_id(_msg.block_id.clone().unwrap())
                            }),
                        SignedMsgType::Precommit => rs
                            .votes
                            .and_then(|vts| vts.precommits(_msg.round))
                            .and_then(|pv| {
                                pv.bit_array_by_block_id(_msg.block_id.clone().unwrap())
                            }),
                        _ => {
                            log::warn!("bad VoteSetBitsMessage field type, forgot to add a check in validate_basic?");
                            return Err(Box::new(
                                    ConsensusReactorError::UnexpectedMessageTypeError(
                                        "bad VoteSetBitsMessage field type, forgot to add a check in validate_basic?".to_string()
                                    )
                                ));
                        }
                    };

                    let ps = src.get_ps().await;
                    ps.apply_vote_set_bits_message(_msg.clone(), our_votes)
                        .await;
                } else {
                    let ps = src.get_ps().await;
                    ps.apply_vote_set_bits_message(_msg.clone(), None).await;
                }
                Ok(())
            }
            _ => Err(Box::new(ConsensusReactorError::UnknownMessageTypeError)),
        }
    }

    fn gossip_data(self: Arc<Self>, peer: Arc<dyn Peer>) {
        log::trace!("start gossip data for peer_id: {:?}", peer.get_id());

        tokio::spawn(async move {
            loop {
                if peer.is_removed().await {
                    log::trace!("stop gossip data for peer_id: {:?}", peer.get_id());
                    return;
                }

                let cs = self.clone().get_cs();
                let rs = cs.get_rs().await;
                let prs = peer.get_prs().await;

                // send proposal block parts if any
                if rs
                    .proposal_block_parts
                    .clone()
                    .map(|pbp| pbp.header())
                    .eq(&prs.proposal_block_parts_header)
                {
                    if let Some(index) = rs.proposal_block_parts.clone().and_then(|pbp| {
                        pbp.parts_bit_array
                            .sub(prs.proposal_block_parts.clone().unwrap())
                            .pick_random()
                    }) {
                        let part = rs.proposal_block_parts.clone().unwrap().get_part(index);
                        let msg = BlockPartMessage {
                            height: rs.height,
                            round: rs.round,
                            part: part,
                        };

                        log::debug!(
                            "sending block part: height={} round={}",
                            prs.height,
                            prs.round
                        );
                        if peer.send(DATA_CHANNEL, msg.msg_to_proto().unwrap().encode_to_vec()) {
                            let ps = peer.get_ps().await;
                            ps.set_has_proposal_block_part(msg).await;
                        }

                        continue;
                    }
                }

                // if the peer is on a previous height, help catch up.
                if prs.height > 0
                    && prs.height < rs.height
                    && prs.height >= cs.get_block_operations().base()
                {
                    self.clone()
                        .gossip_data_for_catch_up(rs, peer.clone())
                        .await;
                    continue;
                }

                // if "our and their" height and round don't match, sleep.
                if rs.height != prs.height || rs.round != prs.round {
                    log::trace!(
                        "peer height|round mismatch, sleeping. peer: id={} height={} round={}",
                        peer.get_id(),
                        prs.height,
                        prs.round,
                    );
                    thread::sleep(cs.get_config().peer_gossip_sleep_duration);
                    continue;
                }

                // send proposal or proposal POL
                if rs.proposal.is_some() && !prs.proposal {
                    // proposal: share the proposal metadata with peer
                    {
                        let msg = ProposalMessage {
                            proposal: rs.proposal.clone(),
                        };
                        log::debug!(
                            "sending proposal: height={} round={}",
                            prs.height,
                            prs.round
                        );
                        if peer.send(DATA_CHANNEL, msg.msg_to_proto().unwrap().encode_to_vec()) {
                            let ps = peer.get_ps().await;
                            ps.set_has_proposal(msg).await;
                        }
                    }

                    if rs.proposal.clone().is_some_and(|p| p.pol_round > 0) {
                        let msg = ProposalPOLMessage {
                            height: rs.height,
                            proposal_pol_round: rs.proposal.clone().unwrap().pol_round,
                            proposal_pol: rs
                                .votes
                                .clone()
                                .and_then(|vts| {
                                    vts.prevotes(rs.proposal.clone().unwrap().pol_round)
                                })
                                .and_then(|vts| vts.bit_array()),
                        };
                        log::debug!("sending POL: height={} round={}", prs.height, prs.round);
                        peer.send(DATA_CHANNEL, msg.msg_to_proto().unwrap().encode_to_vec());
                    }
                    continue;
                }

                // nothing to do, sleep.
                thread::sleep(cs.get_config().peer_gossip_sleep_duration);
                continue;
            }
        });
    }

    fn gossip_votes(self: Arc<Self>, peer: Arc<dyn Peer>) {
        log::trace!("start gossip votes for peer_id: {:?}", peer.get_id());

        tokio::spawn(async move {
            loop {
                if peer.is_removed().await {
                    log::trace!("stop gossip votes for peer_id: {:?}", peer.get_id());
                    return;
                }

                let cs = self.clone().get_cs();
                let rs = cs.get_rs().await;
                let prs = peer.get_prs().await;

                // if height matches, then send LastCommit, Prevotes, Precommits.
                if rs.height == prs.height {
                    if self
                        .clone()
                        .gossip_votes_for_height(rs.clone(), peer.clone())
                        .await
                    {
                        continue;
                    }
                }

                // special catchup logic.
                // if peer is lagging by height 1, send LastCommit.
                if (prs.height != 0) && (rs.height == prs.height + 1) {
                    if let Some(last_commit) = rs.last_commit.clone() {
                        if peer.pick_send_vote(Box::new(last_commit)) {
                            log::debug!("Picked rs.LastCommit to send: height={}", prs.height);
                            continue;
                        }
                    }
                }

                // Catchup logic
                // If peer is lagging by more than 1, send Commit.
                if prs.height != 0 && rs.height >= prs.height + 2 {
                    // load the block commit for prs.height,
                    // which contains precommit signatures for prs.height.
                    if let Some(commit) = cs.get_block_operations().load_block_commit(prs.height) {
                        if peer.pick_send_vote(Box::new(commit)) {
                            log::debug!("Picked Catchup commit to send: height={}", prs.height);
                            continue;
                        }
                    }
                }

                thread::sleep(cs.get_config().peer_gossip_sleep_duration);
                continue;
            }
        });
    }

    fn query_maj23(self: Arc<Self>, peer: Arc<dyn Peer>) {
        log::trace!("start query major 2/3 for peer_id: {:?}", peer.get_id());

        tokio::spawn(async move {
            loop {
                if peer.is_removed().await {
                    log::trace!("stop query major 2/3 for peer_id: {:?}", peer.get_id());
                    return;
                }

                let cs = self.clone().get_cs();

                // Send Height/Round/Prevotes
                {
                    let rs = cs.get_rs().await;
                    let prs = peer.get_prs().await;
                    if rs.height == prs.height {
                        if let Some(maj23_blockid) = rs
                            .votes
                            .clone()
                            .and_then(|vts| vts.prevotes(prs.round))
                            .and_then(|pvts| pvts.two_thirds_majority())
                        {
                            peer.try_send(
                                STATE_CHANNEL,
                                VoteSetMaj23Message {
                                    height: prs.height,
                                    round: prs.round,
                                    r#type: SignedMsgType::Prevote,
                                    block_id: Some(maj23_blockid),
                                }
                                .msg_to_proto()
                                .unwrap()
                                .encode_to_vec(),
                            );
                            thread::sleep(cs.get_config().peer_query_maj23_sleep_duration)
                        }
                    }
                }

                // Send Height/Round/Precommits
                {
                    let rs = cs.get_rs().await;
                    let prs = peer.get_prs().await;
                    if rs.height == prs.height {
                        if let Some(maj23_blockid) = rs
                            .votes
                            .clone()
                            .and_then(|vts| vts.precommits(prs.round))
                            .and_then(|pvts| pvts.two_thirds_majority())
                        {
                            peer.try_send(
                                STATE_CHANNEL,
                                VoteSetMaj23Message {
                                    height: prs.height,
                                    round: prs.round,
                                    r#type: SignedMsgType::Precommit,
                                    block_id: Some(maj23_blockid),
                                }
                                .msg_to_proto()
                                .unwrap()
                                .encode_to_vec(),
                            );
                            thread::sleep(cs.get_config().peer_query_maj23_sleep_duration)
                        }
                    }
                }

                // Send Height/Round/ProposalPOL
                {
                    let rs = cs.get_rs().await;
                    let prs = peer.get_prs().await;
                    if rs.height == prs.height {
                        if let Some(maj23_blockid) = rs
                            .votes
                            .clone()
                            .and_then(|vts| vts.prevotes(prs.proposal_pol_round))
                            .and_then(|pvts| pvts.two_thirds_majority())
                        {
                            peer.try_send(
                                STATE_CHANNEL,
                                VoteSetMaj23Message {
                                    height: prs.height,
                                    round: prs.round,
                                    r#type: SignedMsgType::Prevote,
                                    block_id: Some(maj23_blockid),
                                }
                                .msg_to_proto()
                                .unwrap()
                                .encode_to_vec(),
                            );
                            thread::sleep(cs.get_config().peer_query_maj23_sleep_duration)
                        }
                    }
                }

                // Send Height/CatchupCommitRound/CatchupCommit.
                {
                    let prs = peer.get_prs().await;

                    if prs.catchup_commit_round != 0
                        && prs.height > 0
                        && prs.height <= cs.clone().get_block_operations().height()
                    {
                        if let Some(commit) =
                            cs.clone().get_block_operations().load_commit(prs.height)
                        {
                            peer.try_send(
                                STATE_CHANNEL,
                                VoteSetMaj23Message {
                                    height: prs.height,
                                    round: commit.round,
                                    r#type: SignedMsgType::Precommit,
                                    block_id: commit.block_id,
                                }
                                .msg_to_proto()
                                .unwrap()
                                .encode_to_vec(),
                            );
                            thread::sleep(cs.get_config().peer_query_maj23_sleep_duration)
                        }
                    }
                }

                thread::sleep(cs.get_config().peer_query_maj23_sleep_duration)
            }
        });
    }

    async fn gossip_data_for_catch_up(self: Arc<Self>, rs: RoundState, peer: Arc<dyn Peer>) {
        let prs = peer.get_prs().await;

        if let Some(index) = prs.proposal_block_parts.unwrap().not().pick_random() {
            let cs = self.clone().get_cs();
            let block_ops = cs.get_block_operations();

            // ensure that the peer's PartSetHeader is correct
            if let Some(block_meta) = block_ops.load_block_meta(prs.height) {
                if block_meta
                    .block_id
                    .part_set_header
                    .eq(&prs.proposal_block_parts_header)
                {
                    // load the part
                    if let Some(part) = block_ops.load_block_part(prs.height, index) {
                        let msg = BlockPartMessage {
                            height: prs.height,
                            round: prs.round,
                            part: Some(part),
                        };
                        log::debug!(
                            "sending block part for catchup: round={} index={}",
                            prs.round,
                            index
                        );
                        if peer.send(DATA_CHANNEL, msg.msg_to_proto().unwrap().encode_to_vec()) {
                            let ps = peer.get_ps().await;
                            ps.set_has_proposal_block_part(msg).await;
                        }
                    } else {
                        log::error!(
                                "could not load part: index={} blockPartsHeader={:?} peerBlockPartsHeader={:?}",
                                index,
                                block_meta.block_id.part_set_header,
                                prs.proposal_block_parts_header
                            );
                        thread::sleep(cs.get_config().peer_gossip_sleep_duration);
                        return;
                    }
                } else {
                    log::info!(
                            "peer ProposalBlockPartsHeader mismatch, sleeping: blockPartsHeader={:?}, blockMeta.BlockID.PartsHeader={:?}"
                            ,block_meta.block_id.part_set_header
                            ,prs.proposal_block_parts_header);
                    thread::sleep(cs.get_config().peer_gossip_sleep_duration);
                    return;
                }
            } else {
                log::error!(
                    "failed to load block meta: ourHeight={} blockstoreHeight={}",
                    rs.height,
                    block_ops.height()
                );
                thread::sleep(cs.get_config().peer_gossip_sleep_duration);
                return;
            }
        }
    }

    async fn gossip_votes_for_height(self: Arc<Self>, rs: RoundState, peer: Arc<dyn Peer>) -> bool {
        let prs = peer.get_prs().await;
        // If there are lastCommits to send...
        if prs.step == RoundStep::CanonicalNewHeight {
            if let Some(last_commit) = rs.last_commit {
                if peer.pick_send_vote(Box::new(last_commit)) {
                    log::debug!("Picked rs.LastCommit to send");
                    return true;
                }
            }
        }

        // If there are POL prevotes to send...
        if RoundStep::Unknown < prs.step
            && prs.step <= RoundStep::CanonicalPropose
            && prs.round != 0
            && prs.round <= rs.round
            && prs.proposal_pol_round != 0
        {
            if let Some(pol_prevotes) = rs
                .votes
                .clone()
                .and_then(|vts| vts.prevotes(prs.proposal_pol_round))
            {
                if peer.pick_send_vote(Box::new(pol_prevotes)) {
                    log::debug!(
                        "picked rs.Prevotes(prs.ProposalPOLRound) to send: round={}",
                        prs.proposal_pol_round
                    );
                    return true;
                }
            }
        }

        // If there are prevotes to send...
        if prs.step <= RoundStep::CanonicalPrevoteWait && prs.round <= rs.round {
            if let Some(prevotes) = rs.votes.clone().and_then(|vts| vts.prevotes(prs.round)) {
                if peer.pick_send_vote(Box::new(prevotes)) {
                    log::debug!(
                        "picked rs.Prevotes(prs.Round) to send: round={}",
                        prs.proposal_pol_round
                    );
                    return true;
                }
            }
        }

        // If there are precommits to send...
        if prs.step <= RoundStep::CanonicalPrecommitWait && prs.round != 0 && prs.round <= rs.round
        {
            if let Some(precommits) = rs.votes.clone().and_then(|vts| vts.precommits(prs.round)) {
                if peer.pick_send_vote(Box::new(precommits)) {
                    log::debug!(
                        "picked rs.Precommits(prs.Round) to send: round={}",
                        prs.proposal_pol_round
                    );
                    return true;
                }
            }
        }

        // If there are prevotes to send...Needed because of validBlock mechanism
        if prs.round != 0 && prs.round <= rs.round {
            if let Some(prevotes) = rs.votes.clone().and_then(|vts| vts.prevotes(prs.round)) {
                if peer.pick_send_vote(Box::new(prevotes)) {
                    log::debug!(
                        "picked rs.Prevotes(prs.Round) to send: round={}",
                        prs.proposal_pol_round
                    );
                    return true;
                }
            }
        }

        // If there are POLPrevotes to send...
        if prs.proposal_pol_round != 0 {
            if let Some(pol_prevotes) = rs
                .votes
                .clone()
                .and_then(|vts| vts.prevotes(prs.proposal_pol_round).clone())
            {
                if peer.pick_send_vote(Box::new(pol_prevotes)) {
                    log::debug!(
                        "picked rs.Prevotes(prs.ProposalPOLRound) to send: round={}",
                        prs.proposal_pol_round
                    );
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::TryInto, sync::Arc};

    use crate::{
        reactor::{ConsensusReactor, ConsensusReactorImpl, DATA_CHANNEL},
        state::MockConsensusState,
        types::{
            config::ConsensusConfig,
            errors::ConsensusReactorError,
            messages::{
                msg_from_proto, BlockPartMessage, ConsensusMessageType, HasVoteMessage,
                MessageInfo, NewRoundStepMessage, NewValidBlockMessage, ProposalMessage,
                ProposalPOLMessage, VoteMessage, VoteSetBitsMessage, VoteSetMaj23Message,
            },
            peer::{self, MockPeer, MockPeerState, Peer, PeerRoundState, PeerState},
            round_state::RoundState,
        },
    };
    use kai_proto::consensus::Message as ConsensusMessageProto;
    use kai_types::{round::RoundStep, types::SignedMsgType};
    use prost::Message as ProstMessage;
    use tokio::runtime::Builder;

    async fn init_reactor(
        m_consensus_state: Box<MockConsensusState>,
        m_peers: Vec<Arc<dyn Peer>>,
    ) -> Arc<ConsensusReactorImpl> {
        let reactor = Arc::new(ConsensusReactorImpl::new(m_consensus_state));
        for m_peer in m_peers {
            let rs = reactor.clone().add_peer(m_peer).await;
            assert!(rs.is_ok());
        }
        reactor
    }

    fn get_mock_cs() -> Box<MockConsensusState> {
        let mut mock_consensus_state = Box::new(MockConsensusState::new());
        mock_consensus_state
            .expect_get_rs()
            .returning(|| RoundState::new_default());
        mock_consensus_state
            .expect_get_config()
            .returning(|| Arc::new(ConsensusConfig::new_default()));
        return mock_consensus_state;
    }

    #[tokio::test]
    async fn add_peer() {
        let mut mock_cs = Box::new(MockConsensusState::new());
        let mut mock_peer = MockPeer::new();

        mock_cs
            .expect_get_rs()
            .returning(|| RoundState::new_default());
        mock_cs
            .expect_get_config()
            .returning(|| Arc::new(ConsensusConfig::new_default()));

        mock_peer
            .expect_get_prs()
            .return_const(PeerRoundState::new());
        mock_peer.expect_get_id().return_const(String::from("0x11"));
        mock_peer.expect_is_removed().return_const(false);
        mock_peer.expect_start().return_const(());
        mock_peer.expect_stop().return_const(());

        let c_mock_peer: Arc<dyn Peer> = Arc::new(mock_peer);

        let reactor = Arc::new(ConsensusReactorImpl::new(mock_cs));

        let rt = Builder::new_multi_thread()
            .worker_threads(3)
            .enable_time()
            .build()
            .unwrap();

        _ = rt
            .spawn(async move {
                let rs = reactor.clone().add_peer(c_mock_peer.clone()).await;
                assert!(rs.is_ok());
            })
            .await;

        rt.shutdown_background();
    }

    #[tokio::test]
    async fn remove_peer() {
        let mut mock_cs = Box::new(MockConsensusState::new());
        let mut mock_peer = MockPeer::new();

        mock_cs
            .expect_get_rs()
            .returning(|| RoundState::new_default());
        mock_cs
            .expect_get_config()
            .returning(|| Arc::new(ConsensusConfig::new_default()));

        mock_peer
            .expect_get_prs()
            .return_const(PeerRoundState::new());
        mock_peer.expect_get_id().return_const(String::from("0x11"));
        mock_peer.expect_start().return_const(());
        mock_peer.expect_stop().return_const(());
        mock_peer.expect_is_removed().return_const(true);

        let c_mock_peer: Arc<dyn Peer> = Arc::new(mock_peer);
        let c_mock_peer_2: Arc<dyn Peer> = c_mock_peer.clone();

        let reactor = Arc::new(ConsensusReactorImpl::new(mock_cs));
        let reactor_2 = reactor.clone();

        let rt = Builder::new_multi_thread()
            .worker_threads(3)
            .enable_time()
            .build()
            .unwrap();

        _ = rt
            .spawn(async move {
                let add_rs = reactor.clone().add_peer(c_mock_peer.clone()).await;
                assert!(add_rs.is_ok());
            })
            .await;

        _ = rt
            .spawn(async move {
                let remove_rs = reactor_2.clone().remove_peer(c_mock_peer_2.clone()).await;
                assert!(remove_rs.is_ok());
            })
            .await;

        rt.shutdown_background();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn handle_state_message() {
        // messages
        let nrs_msg = NewRoundStepMessage {
            height: 1,
            round: 1,
            step: RoundStep::Propose,
            seconds_since_start_time: 1000,
            last_commit_round: 0,
        };
        let nrs_cs_msg = Arc::new(ConsensusMessageType::NewRoundStepMessage(nrs_msg.clone()));

        let nvb_msg = NewValidBlockMessage {
            height: 1,
            round: 1,
            block_parts_header: None,
            block_parts: None,
            is_commit: false,
        };
        let nvb_cs_msg = Arc::new(ConsensusMessageType::NewValidBlockMessage(nvb_msg.clone()));

        let hv_msg = HasVoteMessage {
            height: 1,
            round: 1,
            r#type: SignedMsgType::Precommit,
            index: 1,
        };
        let hv_cs_msg = Arc::new(ConsensusMessageType::HasVoteMessage(hv_msg.clone()));

        let vsm_msg = VoteSetMaj23Message {
            height: 1,
            round: 1,
            r#type: SignedMsgType::Precommit,
            block_id: None,
        };
        let vsm_cs_msg = Arc::new(ConsensusMessageType::VoteSetMaj23Message(vsm_msg.clone()));

        let unknown_msg = ProposalMessage { proposal: None };
        let unknown_cs_msg = Arc::new(ConsensusMessageType::ProposalMessage(unknown_msg.clone()));

        // mocking dependencies
        let mut mock_rs = RoundState::new_default();
        mock_rs.height = 1;

        let mut mock_ps = MockPeerState::new();
        mock_ps
            .expect_apply_new_round_step_message()
            .withf(move |p_msg| {
                p_msg.height == nrs_msg.height
                    && p_msg.round == nrs_msg.round
                    && p_msg.step == nrs_msg.step
                    && p_msg.seconds_since_start_time == nrs_msg.seconds_since_start_time
                    && p_msg.last_commit_round == nrs_msg.last_commit_round
            })
            .times(1)
            .return_const(());
        mock_ps
            .expect_apply_new_valid_block_message()
            .withf(move |p_msg| {
                p_msg.height == nvb_msg.height
                    && p_msg.round == nvb_msg.round
                    && p_msg.block_parts_header == nvb_msg.block_parts_header
                    && p_msg.block_parts == nvb_msg.block_parts
                    && p_msg.is_commit == nvb_msg.is_commit
            })
            .times(1)
            .return_const(());
        mock_ps
            .expect_set_has_vote()
            .withf(move |h, r, t, i| {
                *h == hv_msg.height
                    && *r == hv_msg.round
                    && *t == hv_msg.r#type
                    && *i == hv_msg.index.try_into().unwrap()
            })
            .times(1)
            .return_const(());

        let am_mock_ps: Arc<Box<dyn PeerState>> = Arc::new(Box::new(mock_ps));

        let mut mock_peer = MockPeer::new();
        mock_peer
            .expect_get_ps()
            .returning(move || am_mock_ps.clone());
        mock_peer
            .expect_try_send()
            .withf(move |ch_id, msg_bz| {
                if let ConsensusMessageType::VoteSetBitsMessage(_msg) =
                    msg_from_proto(ConsensusMessageProto::decode(msg_bz.as_slice()).unwrap())
                        .unwrap()
                        .as_ref()
                {
                    return *ch_id == DATA_CHANNEL
                        && _msg.height == vsm_msg.height
                        && _msg.round == vsm_msg.round
                        && _msg.r#type == vsm_msg.r#type
                        && _msg.block_id == vsm_msg.block_id;
                } else {
                    return false;
                }
            })
            .times(1)
            .return_const(true);
        let a_mock_peer: Arc<dyn Peer> = Arc::new(mock_peer);

        let reactor = init_reactor(get_mock_cs(), vec![]).await;

        // act
        let mut rs = reactor
            .clone()
            .handle_state_message(a_mock_peer.clone(), nrs_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_state_message(a_mock_peer.clone(), nvb_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_state_message(a_mock_peer.clone(), vsm_cs_msg.clone())
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_state_message(a_mock_peer.clone(), hv_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_state_message(a_mock_peer.clone(), unknown_cs_msg)
            .await;
        assert!(rs
            .is_err_and(|e| matches!(*e.as_ref(), ConsensusReactorError::UnknownMessageTypeError)));
    }

    #[tokio::test]
    async fn handle_data_message() {
        // messages
        let proposal_msg = ProposalMessage { proposal: None };
        let proposal_cs_msg = Arc::new(ConsensusMessageType::ProposalMessage(proposal_msg.clone()));

        let proposal_pol_msg = ProposalPOLMessage {
            height: 1,
            proposal_pol_round: 1,
            proposal_pol: None,
        };
        let proposal_pol_cs_msg = Arc::new(ConsensusMessageType::ProposalPOLMessage(
            proposal_pol_msg.clone(),
        ));

        let block_part_msg = BlockPartMessage {
            height: 1,
            round: 1,
            part: None,
        };
        let block_part_cs_msg = Arc::new(ConsensusMessageType::BlockPartMessage(
            block_part_msg.clone(),
        ));

        let unknown_msg = HasVoteMessage {
            height: 1,
            round: 1,
            r#type: SignedMsgType::Precommit,
            index: 1,
        };
        let unknown_cs_msg = Arc::new(ConsensusMessageType::HasVoteMessage(unknown_msg.clone()));

        // mocking dependencies
        let mock_peer_id = "".to_string();
        let mut mock_cs = Box::new(MockConsensusState::new());
        mock_cs
            .expect_send()
            .withf(move |msg_info| match msg_info.clone() {
                MessageInfo::IncomingMessage { msg: _, peer_id } => mock_peer_id.clone() == peer_id,
                MessageInfo::TerminationMessage => false,
            })
            .returning(|_| Ok(()));

        let mut mock_ps = MockPeerState::new();
        mock_ps
            .expect_set_has_proposal()
            .withf(move |_msg| _msg.proposal.eq(&proposal_msg.proposal))
            .times(1)
            .return_const(());
        mock_ps
            .expect_apply_proposal_pol_message()
            .withf(move |_msg| {
                _msg.height == proposal_pol_msg.height
                    && _msg.proposal_pol_round == proposal_pol_msg.proposal_pol_round
                    && _msg.proposal_pol.eq(&proposal_pol_msg.proposal_pol)
            })
            .times(1)
            .return_const(());
        mock_ps
            .expect_set_has_proposal_block_part()
            .withf(move |_msg| {
                _msg.height == block_part_msg.height
                    && _msg.round == block_part_msg.round
                    && _msg.part.eq(&block_part_msg.part)
            })
            .times(1)
            .return_const(());
        let am_mock_ps: Arc<Box<dyn PeerState>> = Arc::new(Box::new(mock_ps));

        let mut mock_peer = MockPeer::new();
        mock_peer.expect_get_id().return_const("".to_string());
        mock_peer
            .expect_get_ps()
            .returning(move || am_mock_ps.clone());
        let a_mock_peer: Arc<dyn Peer> = Arc::new(mock_peer);

        let reactor = init_reactor(mock_cs, vec![]).await;

        // act
        let mut rs = reactor
            .clone()
            .handle_data_message(a_mock_peer.clone(), proposal_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_data_message(a_mock_peer.clone(), proposal_pol_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_data_message(a_mock_peer.clone(), block_part_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_data_message(a_mock_peer.clone(), unknown_cs_msg)
            .await;
        assert!(rs
            .is_err_and(|e| matches!(*e.as_ref(), ConsensusReactorError::UnknownMessageTypeError)));
    }

    #[tokio::test]
    async fn handle_vote_message() {
        // messages
        let vote_msg = VoteMessage {
            vote: Some(kai_types::vote::Vote {
                r#type: SignedMsgType::Precommit.into(),
                height: 1,
                round: 1,
                block_id: None,
                timestamp: None,
                validator_address: vec![],
                validator_index: 1,
                signature: vec![],
            }),
        };
        let vote_cs_msg = Arc::new(ConsensusMessageType::VoteMessage(vote_msg.clone()));

        let unknown_msg = HasVoteMessage {
            height: 1,
            round: 1,
            r#type: SignedMsgType::Precommit,
            index: 1,
        };
        let unknown_cs_msg = Arc::new(ConsensusMessageType::HasVoteMessage(unknown_msg.clone()));

        // mocking dependencies
        let mock_peer_id = "".to_string();
        let mut mock_cs = Box::new(MockConsensusState::new());
        mock_cs
            .expect_send()
            .withf(move |msg_info| match msg_info.clone() {
                MessageInfo::IncomingMessage { msg: _, peer_id } => mock_peer_id.clone() == peer_id,
                MessageInfo::TerminationMessage => false,
            })
            .returning(|_| Ok(()));

        let mut mock_ps = MockPeerState::new();
        mock_ps
            .expect_set_has_vote()
            .withf(move |h, r, t, i| {
                *h == vote_msg.clone().vote.unwrap().height
                    && *r == vote_msg.clone().vote.unwrap().round
                    && t.eq(
                        &SignedMsgType::from_i32(vote_msg.clone().vote.unwrap().r#type).unwrap(),
                    )
                    && *i
                        == vote_msg
                            .clone()
                            .vote
                            .unwrap()
                            .validator_index
                            .try_into()
                            .unwrap()
            })
            .times(1)
            .return_const(());

        let am_mock_ps: Arc<Box<dyn PeerState>> = Arc::new(Box::new(mock_ps));

        let mut mock_peer = MockPeer::new();
        mock_peer.expect_get_id().return_const("".to_string());
        mock_peer
            .expect_get_ps()
            .returning(move || am_mock_ps.clone());
        let a_mock_peer: Arc<dyn Peer> = Arc::new(mock_peer);

        let reactor = init_reactor(mock_cs, vec![]).await;

        // act
        let mut rs = reactor
            .clone()
            .handle_vote_message(a_mock_peer.clone(), vote_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_vote_message(a_mock_peer.clone(), unknown_cs_msg)
            .await;
        assert!(rs
            .is_err_and(|e| matches!(*e.as_ref(), ConsensusReactorError::UnknownMessageTypeError)));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn handle_vote_set_bits_message() {
        // messages
        let vote_msg = VoteSetBitsMessage {
            height: 1,
            round: 1,
            r#type: SignedMsgType::Precommit,
            block_id: None,
            votes: None,
        };
        let vote_cs_msg = Arc::new(ConsensusMessageType::VoteSetBitsMessage(vote_msg.clone()));

        let unknown_msg = HasVoteMessage {
            height: 1,
            round: 1,
            r#type: SignedMsgType::Precommit,
            index: 1,
        };
        let unknown_cs_msg = Arc::new(ConsensusMessageType::HasVoteMessage(unknown_msg.clone()));

        // mocking dependencies
        let mock_peer_id = "".to_string();
        let mut mock_cs = Box::new(MockConsensusState::new());
        mock_cs
            .expect_get_rs()
            .returning(|| RoundState::new_default());
        mock_cs
            .expect_send()
            .withf(move |msg_info| match msg_info.clone() {
                MessageInfo::IncomingMessage { msg: _, peer_id } => mock_peer_id.clone() == peer_id,
                MessageInfo::TerminationMessage => false,
            })
            .returning(|_| Ok(()));

        let mut mock_ps = MockPeerState::new();
        mock_ps
            .expect_apply_vote_set_bits_message()
            .withf(move |_msg, _| {
                _msg.height == vote_msg.height
                    && _msg.round == vote_msg.round
                    && _msg.r#type == vote_msg.r#type
                    && _msg.block_id == vote_msg.block_id
                    && _msg.votes == vote_msg.votes
            })
            .times(1)
            .return_const(());

        let am_mock_ps: Arc<Box<dyn PeerState>> = Arc::new(Box::new(mock_ps));

        let mut mock_peer = MockPeer::new();
        mock_peer.expect_get_id().return_const("".to_string());
        mock_peer
            .expect_get_ps()
            .returning(move || am_mock_ps.clone());
        let a_mock_peer: Arc<dyn Peer> = Arc::new(mock_peer);

        let reactor = init_reactor(mock_cs, vec![]).await;

        // act
        let mut rs = reactor
            .clone()
            .handle_vote_set_bits_message(a_mock_peer.clone(), vote_cs_msg)
            .await;
        assert!(rs.is_ok());
        rs = reactor
            .clone()
            .handle_vote_set_bits_message(a_mock_peer.clone(), unknown_cs_msg)
            .await;
        assert!(rs
            .is_err_and(|e| matches!(*e.as_ref(), ConsensusReactorError::UnknownMessageTypeError)));
    }
}
